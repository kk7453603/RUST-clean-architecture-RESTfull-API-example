use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::{User, UserRepository, UserId, Email, DomainError};

pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn clear(&self) {
        let mut users = self.users.write().await;
        users.clear();
    }

    pub async fn seed(&self, users: Vec<User>) {
        let mut users_map = self.users.write().await;
        for user in users {
            users_map.insert(user.id().to_string(), user);
        }
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        let users = self.users.read().await;
        Ok(users.get(&id.to_string()).cloned())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        let users = self.users.read().await;
        Ok(users.values()
            .find(|user| user.email() == email)
            .cloned())
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let mut users = self.users.write().await;
        users.insert(user.id().to_string(), user.clone());
        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<(), DomainError> {
        let mut users = self.users.write().await;
        users.remove(&id.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Email, User};

    #[tokio::test]
    async fn test_save_and_find_user() {
        let repository = InMemoryUserRepository::new();
        
        let email = Email::new("test@example.com".to_string()).unwrap();
        let user = User::new(email, "Test User".to_string()).unwrap();
        
        repository.save(&user).await.unwrap();
        
        let found_user = repository.find_by_id(user.id()).await.unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().id(), user.id());
    }

    #[tokio::test]
    async fn test_find_by_email() {
        let repository = InMemoryUserRepository::new();
        
        let email = Email::new("test@example.com".to_string()).unwrap();
        let user = User::new(email, "Test User".to_string()).unwrap();
        
        repository.save(&user).await.unwrap();
        
        let found_user = repository.find_by_email(user.email()).await.unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().email(), user.email());
    }

    #[tokio::test]
    async fn test_delete_user() {
        let repository = InMemoryUserRepository::new();
        
        let email = Email::new("test@example.com".to_string()).unwrap();
        let user = User::new(email, "Test User".to_string()).unwrap();
        
        repository.save(&user).await.unwrap();
        assert!(repository.find_by_id(user.id()).await.unwrap().is_some());
        
        repository.delete(user.id()).await.unwrap();
        assert!(repository.find_by_id(user.id()).await.unwrap().is_none());
    }
}