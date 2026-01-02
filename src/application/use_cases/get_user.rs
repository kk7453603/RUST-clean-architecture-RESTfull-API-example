use crate::domain::{UserDomainService, UserRepository, UserId, DomainError};
use crate::application::dto::UserResponse;

pub struct GetUserUseCase<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> GetUserUseCase<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, user_id: String) -> Result<UserResponse, ApplicationError> {
        let user_id = UserId::from_string(user_id)
            .map_err(|err| ApplicationError::InvalidUserId(err.to_string()))?;
        
        let user = self.user_repository
            .find_by_id(&user_id)
            .await
            .map_err(ApplicationError::DomainError)?
            .ok_or(ApplicationError::UserNotFound)?;
            
        Ok(UserResponse::from(user))
    }

    pub async fn get_by_email(&self, email: String) -> Result<UserResponse, ApplicationError> {
        let email = Email::new(email)
            .map_err(|err| ApplicationError::InvalidEmail(err.to_string()))?;
        
        let user = self.user_repository
            .find_by_email(&email)
            .await
            .map_err(ApplicationError::DomainError)?
            .ok_or(ApplicationError::UserNotFound)?;
            
        Ok(UserResponse::from(user))
    }
}

use crate::domain::Email;

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