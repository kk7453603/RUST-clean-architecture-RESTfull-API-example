use crate::domain::{Email, User, DomainError};

pub trait EmailService: Send + Sync {
    async fn send_welcome_email(&self, user: &User) -> Result<(), DomainError>;
    async fn send_password_reset_email(&self, email: &Email, reset_token: String) -> Result<(), DomainError>;
}

pub struct ConsoleEmailService;

impl ConsoleEmailService {
    pub fn new() -> Self {
        Self
    }
}

impl EmailService for ConsoleEmailService {
    async fn send_welcome_email(&self, user: &User) -> Result<(), DomainError> {
        println!(
            "Добро пожаловать, {}! Ваш email: {}",
            user.name(),
            user.email()
        );
        Ok(())
    }

    async fn send_password_reset_email(&self, email: &Email, reset_token: String) -> Result<(), DomainError> {
        println!(
            "Отправлен токен сброса пароля {} для email: {}",
            reset_token,
            email
        );
        Ok(())
    }
}

pub struct MockEmailService {
    sent_emails: std::sync::Mutex<Vec<String>>,
}

impl MockEmailService {
    pub fn new() -> Self {
        Self {
            sent_emails: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn get_sent_emails(&self) -> Vec<String> {
        self.sent_emails.lock().unwrap().clone()
    }
}

impl EmailService for MockEmailService {
    async fn send_welcome_email(&self, user: &User) -> Result<(), DomainError> {
        let email = format!("WELCOME: {} - {}", user.name(), user.email());
        self.sent_emails.lock().unwrap().push(email);
        Ok(())
    }

    async fn send_password_reset_email(&self, email: &Email, reset_token: String) -> Result<(), DomainError> {
        let email = format!("RESET: {} - {}", email, reset_token);
        self.sent_emails.lock().unwrap().push(email);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_console_email_service() {
        let service = ConsoleEmailService::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let user = User::new(email, "Test User".to_string()).unwrap();

        let result = service.send_welcome_email(&user).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mock_email_service() {
        let service = MockEmailService::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let user = User::new(email, "Test User".to_string()).unwrap();

        let result = service.send_welcome_email(&user).await;
        assert!(result.is_ok());

        let sent_emails = service.get_sent_emails();
        assert_eq!(sent_emails.len(), 1);
        assert!(sent_emails[0].contains("WELCOME: Test User - test@example.com"));
    }
}