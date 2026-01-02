use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use crate::application::{UserApplicationService, CreateUserRequest, UpdateUserRequest};
use crate::application::dto::ApiResponse;

pub async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn create_user_handler(
    State(user_service): State<UserApplicationService<infrastructure::InMemoryUserRepository>>,
    Json(request): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let response = user_service.create_user(request).await;
    
    match response.success {
        true => (StatusCode::CREATED, Json(response)).into_response(),
        false => (StatusCode::BAD_REQUEST, Json(response)).into_response(),
    }
}

pub async fn get_user_handler(
    State(user_service): State<UserApplicationService<infrastructure::InMemoryUserRepository>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let response = user_service.get_user(user_id).await;
    
    match response.success {
        true => (StatusCode::OK, Json(response)).into_response(),
        false => (StatusCode::NOT_FOUND, Json(response)).into_response(),
    }
}

pub async fn get_user_by_email_handler(
    State(user_service): State<UserApplicationService<infrastructure::InMemoryUserRepository>>,
    Json(request): Json<serde_json::Value>,
) -> impl IntoResponse {
    let email = request["email"].as_str().unwrap_or("").to_string();
    
    if email.is_empty() {
        let error_response = ApiResponse::<application::dto::UserResponse> {
            success: false,
            data: None,
            error: Some("Email is required".to_string()),
        };
        return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
    }
    
    let response = user_service.get_user_by_email(email).await;
    
    match response.success {
        true => (StatusCode::OK, Json(response)).into_response(),
        false => (StatusCode::NOT_FOUND, Json(response)).into_response(),
    }
}

pub async fn update_user_handler(
    State(user_service): State<UserApplicationService<infrastructure::InMemoryUserRepository>>,
    Path(user_id): Path<String>,
    Json(request): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    let response = user_service.update_user(user_id, request).await;
    
    match response.success {
        true => (StatusCode::OK, Json(response)).into_response(),
        false => (StatusCode::BAD_REQUEST, Json(response)).into_response(),
    }
}

pub async fn delete_user_handler(
    State(user_service): State<UserApplicationService<infrastructure::InMemoryUserRepository>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let response = user_service.delete_user(user_id).await;
    
    match response.success {
        true => (StatusCode::NO_CONTENT, Json(response)).into_response(),
        false => (StatusCode::NOT_FOUND, Json(response)).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::body::Body;
    use axum::extract::Request;
    use axum::routing::post;
    use axum::Router;

    #[tokio::test]
    async fn test_health_handler() {
        let response = health_handler().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}