/// These are the database params needed for connecting to most databases.
pub struct DatabaseConfig {
    pub db_name: String,
    pub password: String,
    pub user_name: String,
    pub host: String,
    pub port: Option<String>,
    pub namespace: Option<String>,
}

pub fn mysql_connection_string(config: &DatabaseConfig) -> String {
    match &config.port {
        Some(port) => {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                config.user_name, config.password, config.host, port, config.db_name
            )
        }
        None => {
            format!(
                "mysql://{}:{}@{}/{}",
                config.user_name, config.password, config.host, config.db_name
            )
        }
    }
}
