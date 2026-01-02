use crate::domain::{UserDomainService, UserRepository, UserId, DomainError};

pub struct DeleteUserUseCase<R: UserRepository> {
    user_domain_service: UserDomainService<R>,
}

impl<R: UserRepository> DeleteUserUseCase<R> {
    pub fn new(user_repository: R) -> Self {
        Self {
            user_domain_service: UserDomainService::new(user_repository),
        }
    }

    pub async fn execute(&self, user_id: String) -> Result<(), ApplicationError> {
        let user_id = UserId::from_string(user_id)
            .map_err(|err| ApplicationError::InvalidUserId(err.to_string()))?;
        
        self.user_domain_service
            .delete_user(user_id)
            .await
            .map_err(ApplicationError::DomainError)?;
            
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Invalid user ID: {0}")]
    InvalidUserId(String),
    
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