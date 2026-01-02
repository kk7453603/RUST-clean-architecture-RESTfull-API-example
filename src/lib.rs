// Создание единого файла для демонстрации чистой архитектуры
// В реальном проекте эти модули должны быть разделены на отдельные файлы
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// ===============================
// DOMAIN LAYER
// ===============================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_string(id: String) -> Result<Self, String> {
        match Uuid::parse_str(&id) {
            Ok(uuid) => Ok(Self(uuid)),
            Err(_) => Err("Invalid UUID format".to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email {
    value: String,
}

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }

        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }

        Ok(Self {
            value: email.to_lowercase(),
        })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq<str> for Email {
    fn eq(&self, other: &str) -> bool {
        self.value == other.to_lowercase()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: UserId,
    email: Email,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: Email, name: String) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        let now = Utc::now();
        Ok(Self {
            id: UserId::new(),
            email,
            name: name.trim().to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn update_name(&mut self, new_name: String) -> Result<(), String> {
        if new_name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        self.name = new_name.trim().to_string();
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_email(&mut self, new_email: Email) -> Result<(), String> {
        self.email = new_email;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// ===============================
// APPLICATION LAYER
// ===============================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id().to_string(),
            email: user.email().as_str().to_string(),
            name: user.name().to_string(),
            created_at: *user.created_at(),
            updated_at: *user.updated_at(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

// ===============================
// INFRASTRUCTURE LAYER
// ===============================

#[derive(Clone)]
pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn find_by_id(&self, id: &UserId) -> Option<User> {
        let users = self.users.read().await;
        users.get(&id.to_string()).cloned()
    }
    #[warn(dead_code)]
    pub async fn find_by_email(&self, email: &Email) -> Option<User> {
        let users = self.users.read().await;
        users.values().find(|user| user.email() == email).cloned()
    }

    pub async fn save(&self, user: &User) {
        let mut users = self.users.write().await;
        users.insert(user.id().to_string(), user.clone());
    }

    pub async fn delete(&self, id: &UserId) {
        let mut users = self.users.write().await;
        users.remove(&id.to_string());
    }
}

// ===============================
// APPLICATION SERVICES
// ===============================

#[derive(Clone)]
pub struct UserApplicationService {
    repository: InMemoryUserRepository,
}

impl UserApplicationService {
    pub fn new() -> Self {
        Self {
            repository: InMemoryUserRepository::new(),
        }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> ApiResponse<UserResponse> {
        match Email::new(request.email) {
            Ok(email) => match User::new(email, request.name) {
                Ok(user) => {
                    self.repository.save(&user).await;
                    ApiResponse::success(UserResponse::from(user))
                }
                Err(error) => ApiResponse::error(error),
            },
            Err(error) => ApiResponse::error(error),
        }
    }

    pub async fn get_user(&self, user_id: String) -> ApiResponse<UserResponse> {
        match UserId::from_string(user_id) {
            Ok(user_id) => match self.repository.find_by_id(&user_id).await {
                Some(user) => ApiResponse::success(UserResponse::from(user)),
                None => ApiResponse::error("User not found".to_string()),
            },
            Err(error) => ApiResponse::error(error),
        }
    }

    pub async fn update_user(
        &self,
        user_id: String,
        request: UpdateUserRequest,
    ) -> ApiResponse<UserResponse> {
        match UserId::from_string(user_id) {
            Ok(user_id) => match self.repository.find_by_id(&user_id).await {
                Some(mut user) => {
                    let mut updated = false;

                    if let Some(email_str) = request.email {
                        match Email::new(email_str) {
                            Ok(email) => {
                                if user.update_email(email).is_ok() {
                                    updated = true;
                                }
                            }
                            Err(error) => return ApiResponse::error(error),
                        }
                    }

                    if let Some(name) = request.name {
                        if user.update_name(name).is_ok() {
                            updated = true;
                        }
                    }

                    if updated {
                        self.repository.save(&user).await;
                        ApiResponse::success(UserResponse::from(user))
                    } else {
                        ApiResponse::error("No valid updates provided".to_string())
                    }
                }
                None => ApiResponse::error("User not found".to_string()),
            },
            Err(error) => ApiResponse::error(error),
        }
    }

    pub async fn delete_user(&self, user_id: String) -> ApiResponse<()> {
        match UserId::from_string(user_id) {
            Ok(user_id) => {
                if self.repository.find_by_id(&user_id).await.is_some() {
                    self.repository.delete(&user_id).await;
                    ApiResponse::success(())
                } else {
                    ApiResponse::error("User not found".to_string())
                }
            }
            Err(error) => ApiResponse::error(error),
        }
    }
}

// ===============================
// PRESENTATION LAYER
// ===============================

pub async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn create_user_handler(
    State(user_service): State<UserApplicationService>,
    Json(request): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let response = user_service.create_user(request).await;

    match response.success {
        true => (StatusCode::CREATED, Json(response)).into_response(),
        false => (StatusCode::BAD_REQUEST, Json(response)).into_response(),
    }
}

pub async fn get_user_handler(
    State(user_service): State<UserApplicationService>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let response = user_service.get_user(user_id).await;

    match response.success {
        true => (StatusCode::OK, Json(response)).into_response(),
        false => (StatusCode::NOT_FOUND, Json(response)).into_response(),
    }
}

pub async fn update_user_handler(
    State(user_service): State<UserApplicationService>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
    Json(request): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    let response = user_service.update_user(user_id, request).await;

    match response.success {
        true => (StatusCode::OK, Json(response)).into_response(),
        false => (StatusCode::BAD_REQUEST, Json(response)).into_response(),
    }
}

pub async fn delete_user_handler(
    State(user_service): State<UserApplicationService>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let response = user_service.delete_user(user_id).await;

    match response.success {
        true => (StatusCode::NO_CONTENT, Json(response)).into_response(),
        false => (StatusCode::NOT_FOUND, Json(response)).into_response(),
    }
}

// ===============================
// ROUTER
// ===============================

pub fn create_app_router() -> Router {
    let user_service = UserApplicationService::new();

    Router::new()
        .route("/health", get(health_handler))
        .route("/api/users", post(create_user_handler))
        .route("/api/users/{:id}", get(get_user_handler))
        .route("/api/users/{:id}", put(update_user_handler))
        .route("/api/users/{:id}", delete(delete_user_handler))
        .with_state(user_service)
}
