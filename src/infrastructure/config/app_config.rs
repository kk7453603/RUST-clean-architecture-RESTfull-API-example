use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub email_service_url: Option<String>,
    pub log_level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_host: "0.0.0.0".to_string(),
            server_port: 3000,
            database_url: "in-memory".to_string(),
            jwt_secret: "your-secret-key".to_string(),
            email_service_url: None,
            log_level: "info".to_string(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        if let Ok(host) = env::var("SERVER_HOST") {
            config.server_host = host;
        }
        
        if let Ok(port) = env::var("SERVER_PORT") {
            config.server_port = port.parse().unwrap_or(3000);
        }
        
        if let Ok(db_url) = env::var("DATABASE_URL") {
            config.database_url = db_url;
        }
        
        if let Ok(secret) = env::var("JWT_SECRET") {
            config.jwt_secret = secret;
        }
        
        if let Ok(email_url) = env::var("EMAIL_SERVICE_URL") {
            config.email_service_url = Some(email_url);
        }
        
        if let Ok(log_level) = env::var("LOG_LEVEL") {
            config.log_level = log_level;
        }
        
        config
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.server_host, "0.0.0.0");
        assert_eq!(config.server_port, 3000);
        assert_eq!(config.database_url, "in-memory");
    }

    #[test]
    fn test_config_from_env() {
        // Устанавливаем переменные окружения
        std::env::set_var("SERVER_HOST", "127.0.0.1");
        std::env::set_var("SERVER_PORT", "8080");
        
        let config = AppConfig::from_env();
        assert_eq!(config.server_host, "127.0.0.1");
        assert_eq!(config.server_port, 8080);
        
        // Очищаем переменные
        std::env::remove_var("SERVER_HOST");
        std::env::remove_var("SERVER_PORT");
    }

    #[test]
    fn test_server_address() {
        let config = AppConfig {
            server_host: "localhost".to_string(),
            server_port: 8080,
            database_url: "test".to_string(),
            jwt_secret: "test".to_string(),
            email_service_url: None,
            log_level: "test".to_string(),
        };
        
        assert_eq!(config.server_address(), "localhost:8080");
    }
}