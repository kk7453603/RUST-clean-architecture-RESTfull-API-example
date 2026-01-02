use crate::domain::{UserRepository, DomainError};
use crate::application::{CreateUserUseCase, GetUserUseCase, UpdateUserUseCase, DeleteUserUseCase};
use crate::application::dto::{CreateUserRequest, UpdateUserRequest, UserResponse, ApiResponse};

pub struct UserApplicationService<R: UserRepository> {
    create_user_use_case: CreateUserUseCase<R>,
    get_user_use_case: GetUserUseCase<R>,
    update_user_use_case: UpdateUserUseCase<R>,
    delete_user_use_case: DeleteUserUseCase<R>,
}

impl<R: UserRepository> UserApplicationService<R> {
    pub fn new(user_repository: R) -> Self {
        Self {
            create_user_use_case: CreateUserUseCase::new(user_repository.clone()),
            get_user_use_case: GetUserUseCase::new(user_repository.clone()),
            update_user_use_case: UpdateUserUseCase::new(user_repository.clone()),
            delete_user_use_case: DeleteUserUseCase::new(user_repository),
        }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> ApiResponse<UserResponse> {
        match self.create_user_use_case.execute(request.email, request.name).await {
            Ok(user) => ApiResponse::success(UserResponse::from(user)),
            Err(error) => ApiResponse::error(error.to_string()),
        }
    }

    pub async fn get_user(&self, user_id: String) -> ApiResponse<UserResponse> {
        match self.get_user_use_case.execute(user_id).await {
            Ok(user) => ApiResponse::success(user),
            Err(error) => ApiResponse::error(error.to_string()),
        }
    }

    pub async fn get_user_by_email(&self, email: String) -> ApiResponse<UserResponse> {
        match self.get_user_use_case.get_by_email(email).await {
            Ok(user) => ApiResponse::success(user),
            Err(error) => ApiResponse::error(error.to_string()),
        }
    }

    pub async fn update_user(&self, user_id: String, request: UpdateUserRequest) -> ApiResponse<UserResponse> {
        match self.update_user_use_case.execute(user_id, request.email, request.name).await {
            Ok(user) => ApiResponse::success(user),
            Err(error) => ApiResponse::error(error.to_string()),
        }
    }

    pub async fn delete_user(&self, user_id: String) -> ApiResponse<()> {
        match self.delete_user_use_case.execute(user_id).await {
            Ok(_) => ApiResponse::success(()),
            Err(error) => ApiResponse::error(error.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{User, Email};

    struct MockUserRepository {
        users: std::collections::HashMap<String, User>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: std::collections::HashMap::new(),
            }
        }
    }

    impl Clone for MockUserRepository {
        fn clone(&self) -> Self {
            Self {
                users: self.users.clone(),
            }
        }
    }

    impl UserRepository for MockUserRepository {
        async fn find_by_id(&self, id: &domain::UserId) -> Result<Option<User>, DomainError> {
            Ok(self.users.get(&id.to_string()).cloned())
        }

        async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
            Ok(self.users.values()
                .find(|user| user.email() == email)
                .cloned())
        }

        async fn save(&self, user: &User) -> Result<(), DomainError> {
            self.users.insert(user.id().to_string(), user.clone());
            Ok(())
        }

        async fn delete(&self, id: &domain::UserId) -> Result<(), DomainError> {
            self.users.remove(&id.to_string());
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create_user_service() {
        let repository = MockUserRepository::new();
        let service = UserApplicationService::new(repository);

        let request = CreateUserRequest {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };

        let response = service.create_user(request).await;
        
        assert!(response.success);
        assert!(response.data.is_some());
        let user = response.data.unwrap();
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test User");
    }
}