use chrono::{DateTime, Utc};
use crate::domain::{UserId, Email, DomainError};

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: UserId,
    email: Email,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: Email, name: String) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::InvalidUserData("Name cannot be empty".to_string()));
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

    pub fn from_existing(
        id: UserId,
        email: Email,
        name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::InvalidUserData("Name cannot be empty".to_string()));
        }

        Ok(Self {
            id,
            email,
            name: name.trim().to_string(),
            created_at,
            updated_at,
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

    pub fn update_name(&mut self, new_name: String) -> Result<(), DomainError> {
        if new_name.trim().is_empty() {
            return Err(DomainError::InvalidUserData("Name cannot be empty".to_string()));
        }

        self.name = new_name.trim().to_string();
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_email(&mut self, new_email: Email) -> Result<(), DomainError> {
        self.email = new_email;
        self.updated_at = Utc::now();
        Ok(())
    }
}

impl PartialEq<User> for User {
    fn eq(&self, other: &User) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_success() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        let user = User::new(email, "John Doe".to_string()).unwrap();
        
        assert_eq!(user.name(), "John Doe");
        assert_eq!(user.email().as_str(), "user@example.com");
    }

    #[test]
    fn test_create_user_with_empty_name() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        let result = User::new(email, "".to_string());
        
        assert!(result.is_err());
    }

    #[test]
    fn test_update_user_name() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        let mut user = User::new(email, "John Doe".to_string()).unwrap();
        
        let updated_at = *user.updated_at();
        user.update_name("Jane Doe".to_string()).unwrap();
        
        assert_eq!(user.name(), "Jane Doe");
        assert_ne!(*user.updated_at(), updated_at);
    }

    #[test]
    fn test_update_user_email() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        let mut user = User::new(email, "John Doe".to_string()).unwrap();
        
        let new_email = Email::new("jane@example.com".to_string()).unwrap();
        user.update_email(new_email).unwrap();
        
        assert_eq!(user.email().as_str(), "jane@example.com");
    }
}