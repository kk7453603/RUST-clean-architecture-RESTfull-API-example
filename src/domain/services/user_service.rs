use crate::domain::{User, Email, UserId, DomainError};

pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn delete(&self, id: &UserId) -> Result<(), DomainError>;
}

pub struct UserDomainService<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> UserDomainService<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, email: Email, name: String) -> Result<User, DomainError> {
        // Проверяем, что пользователь с таким email не существует
        if let Some(existing_user) = self.user_repository.find_by_email(&email).await? {
            return Err(DomainError::UserAlreadyExists);
        }

        // Создаем нового пользователя
        User::new(email, name)
    }

    pub async fn update_user(&self, user_id: UserId, email: Option<Email>, name: Option<String>) -> Result<User, DomainError> {
        // Получаем существующего пользователя
        let mut user = self.user_repository.find_by_id(&user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        // Обновляем email если нужно
        if let Some(new_email) = email {
            // Проверяем, что новый email не занят другим пользователем
            if let Some(existing_user) = self.user_repository.find_by_email(&new_email).await? {
                if existing_user.id() != &user_id {
                    return Err(DomainError::UserAlreadyExists);
                }
            }
            user.update_email(new_email)?;
        }

        // Обновляем имя если нужно
        if let Some(new_name) = name {
            user.update_name(new_name)?;
        }

        // Сохраняем обновленного пользователя
        self.user_repository.save(&user).await?;

        Ok(user)
    }

    pub async fn delete_user(&self, user_id: UserId) -> Result<(), DomainError> {
        // Проверяем, что пользователь существует
        let user = self.user_repository.find_by_id(&user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        // Удаляем пользователя
        self.user_repository.delete(&user.id()).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

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

    impl UserRepository for MockUserRepository {
        async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
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

        async fn delete(&self, id: &UserId) -> Result<(), DomainError> {
            self.users.remove(&id.to_string());
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let repository = MockUserRepository::new();
        let service = UserDomainService::new(repository);

        let email = Email::new("test@example.com".to_string()).unwrap();
        let user = service.create_user(email, "Test User".to_string()).await.unwrap();

        assert_eq!(user.name(), "Test User");
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        let mut repository = MockUserRepository::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let existing_user = User::new(email.clone(), "Existing User".to_string()).unwrap();
        repository.users.insert(existing_user.id().to_string(), existing_user);

        let service = UserDomainService::new(repository);
        let result = service.create_user(email, "New User".to_string()).await;

        assert!(matches!(result, Err(DomainError::UserAlreadyExists)));
    }
}