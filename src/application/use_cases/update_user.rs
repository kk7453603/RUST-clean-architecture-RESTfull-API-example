use crate::domain::{UserDomainService, UserRepository, UserId, DomainError};
use crate::application::dto::UserResponse;

pub struct UpdateUserUseCase<R: UserRepository> {
    user_domain_service: UserDomainService<R>,
}

impl<R: UserRepository> UpdateUserUseCase<R> {
    pub fn new(user_repository: R) -> Self {
        Self {
            user_domain_service: UserDomainService::new(user_repository),
        }
    }

    pub async fn execute(&self, user_id: String, email: Option<String>, name: Option<String>) -> Result<UserResponse, ApplicationError> {
        let user_id = UserId::from_string(user_id)
            .map_err(|err| ApplicationError::InvalidUserId(err.to_string()))?;
        
        let email = if let Some(email_str) = email {
            Some(Email::new(email_str).map_err(|err| ApplicationError::InvalidEmail(err.to_string()))?)
        } else {
            None
        };
        
        let user = self.user_domain_service
            .update_user(user_id, email, name)
            .await
            .map_err(ApplicationError::DomainError)?;
            
        Ok(UserResponse::from(user))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Invalid user ID: {0}")]
    InvalidUserId(String),
    
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
}

impl From<ApplicationError> for String {
    fn from(error: ApplicationError) -> Self {
        error.to_string()
    }
}