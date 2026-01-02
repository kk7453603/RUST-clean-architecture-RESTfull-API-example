use axum::{
    Router,
    routing::{get, post, put, delete},
};
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};
use crate::presentation::{user_handlers, logging};

pub fn create_app_router() -> Router {
    // Настройка CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Создаем пользовательское приложение (с in-memory репозиторием для примера)
    let user_repository = infrastructure::InMemoryUserRepository::new();
    let user_application_service = application::UserApplicationService::new(user_repository);
    
    Router::new()
        // Health check
        .route("/health", get(user_handlers::health_handler))
        
        // User routes
        .route("/api/users", post(create_user_handler))
        .route("/api/users/:id", get(get_user_handler))
        .route("/api/users/email", post(get_user_by_email_handler))
        .route("/api/users/:id", put(update_user_handler))
        .route("/api/users/:id", delete(delete_user_handler))
        
        // Добавляем состояние приложения
        .with_state(user_application_service)
        
        // Добавляем middleware
        .layer(ServiceBuilder::new().layer(cors))
        .layer(logging::build_logging_layer())
}

fn create_user_handler(
    state: axum::extract::State<application::UserApplicationService<infrastructure::InMemoryUserRepository>>,
    request: axum::extract::Json<application::dto::CreateUserRequest>,
) -> axum::response::Response {
    user_handlers::create_user_handler(state, request).await
}

fn get_user_handler(
    state: axum::extract::State<application::UserApplicationService<infrastructure::InMemoryUserRepository>>,
    request: axum::extract::Path<String>,
) -> axum::response::Response {
    user_handlers::get_user_handler(state, request).await
}

fn get_user_by_email_handler(
    state: axum::extract::State<application::UserApplicationService<infrastructure::InMemoryUserRepository>>,
    request: axum::extract::Json<serde_json::Value>,
) -> axum::response::Response {
    user_handlers::get_user_by_email_handler(state, request).await
}

fn update_user_handler(
    state: axum::extract::State<application::UserApplicationService<infrastructure::InMemoryUserRepository>>,
    path: axum::extract::Path<String>,
    request: axum::extract::Json<application::dto::UpdateUserRequest>,
) -> axum::response::Response {
    user_handlers::update_user_handler(state, path, request).await
}

fn delete_user_handler(
    state: axum::extract::State<application::UserApplicationService<infrastructure::InMemoryUserRepository>>,
    request: axum::extract::Path<String>,
) -> axum::response::Response {
    user_handlers::delete_user_handler(state, request).await
}