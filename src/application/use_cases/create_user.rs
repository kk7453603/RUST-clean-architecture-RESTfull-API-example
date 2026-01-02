use crate::domain::{UserDomainService, UserRepository, Email, DomainError};

pub struct CreateUserUseCase<R: UserRepository> {
    user_domain_service: UserDomainService<R>,
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub fn new(user_repository: R) -> Self {
        Self {
            user_domain_service: UserDomainService::new(user_repository),
        }
    }

    pub async fn execute(&self, email: String, name: String) -> Result<domain::User, ApplicationError> {
        let email = Email::new(email)
            .map_err(|err| ApplicationError::InvalidEmail(err.to_string()))?;
        
        let user = self.user_domain_service
            .create_user(email, name)
            .await
            .map_err(ApplicationError::DomainError)?;
            
        Ok(user)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

impl From<ApplicationError> for String {
    fn from(error: ApplicationError) -> Self {
        error.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Email;

    struct MockUserRepository {
        users: std::collections::HashMap<String, domain::User>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: std::collections::HashMap::new(),
            }
        }
    }

    impl UserRepository for MockUserRepository {
        async fn find_by_id(&self, id: &domain::UserId) -> Result<Option<domain::User>, DomainError> {
            Ok(self.users.get(&id.to_string()).cloned())
        }

        async fn find_by_email(&self, email: &Email) -> Result<Option<domain::User>, DomainError> {
            Ok(self.users.values()
                .find(|user| user.email() == email)
                .cloned())
        }

        async fn save(&self, user: &domain::User) -> Result<(), DomainError> {
            self.users.insert(user.id().to_string(), user.clone());
            Ok(())
        }

        async fn delete(&self, id: &domain::UserId) -> Result<(), DomainError> {
            self.users.remove(&id.to_string());
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let repository = MockUserRepository::new();
        let use_case = CreateUserUseCase::new(repository);

        let result = use_case.execute("test@example.com".to_string(), "Test User".to_string()).await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name(), "Test User");
        assert_eq!(user.email().as_str(), "test@example.com");
    }

    #[tokio::test]
    async fn test_create_user_invalid_email() {
        let repository = MockUserRepository::new();
        let use_case = CreateUserUseCase::new(repository);

        let result = use_case.execute("invalid-email".to_string(), "Test User".to_string()).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApplicationError::InvalidEmail(_)));
    }
}