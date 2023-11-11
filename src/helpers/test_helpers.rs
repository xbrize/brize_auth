use crate::config::DatabaseConfig;

#[allow(dead_code)]
pub fn mysql_configs() -> DatabaseConfig {
    DatabaseConfig {
        host: "localhost".to_string(),
        password: "my-secret-pw".to_string(),
        db_name: "mysql".to_string(),
        user_name: "root".to_string(),
        port: "3306".to_string(),
        namespace: None,
    }
}

#[allow(dead_code)]
pub fn surreal_configs() -> DatabaseConfig {
    DatabaseConfig {
        db_name: "test".to_string(),
        host: "127.0.0.1".to_string(),
        port: "8000".to_string(),
        user_name: "root".to_string(),
        password: "root".to_string(),
        namespace: Some("test".to_string()),
    }
}
