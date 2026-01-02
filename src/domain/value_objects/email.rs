use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email {
    value: String,
}

impl Email {
    pub fn new(email: String) -> Result<Self, DomainError> {
        if email.is_empty() {
            return Err(DomainError::InvalidEmail("Email cannot be empty".to_string()));
        }

        if !Self::is_valid_email(&email) {
            return Err(DomainError::InvalidEmail("Invalid email format".to_string()));
        }

        Ok(Self { value: email.to_lowercase() })
    }

    fn is_valid_email(email: &str) -> bool {
        // Простая валидация email
        email.contains('@') && 
        email.chars().filter(|c| *c == '@').count() == 1 &&
        email.len() > 5 &&
        email.len() < 256
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

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.value
    }
}

impl PartialEq<str> for Email {
    fn eq(&self, other: &str) -> bool {
        self.value == other.to_lowercase()
    }
}

use crate::domain::errors::DomainError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_invalid_email_empty() {
        let result = Email::new("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_email_no_at() {
        let result = Email::new("userexample.com".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_email_case_insensitive() {
        let email1 = Email::new("User@Example.COM".to_string()).unwrap();
        let email2 = Email::new("user@example.com".to_string()).unwrap();
        assert_eq!(email1, email2);
    }
}