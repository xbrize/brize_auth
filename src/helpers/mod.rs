use crate::config::DatabaseConfig;

#[allow(dead_code)]
pub fn mysql_configs() -> DatabaseConfig {
    DatabaseConfig {
        host: "localhost".to_string(),
        password: "my-secret-pw".to_string(),
        db_name: "mysql".to_string(),
        user_name: "root".to_string(),
        port: Some("3306".to_string()),
        namespace: None,
    }
}
