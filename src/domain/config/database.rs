/// These are the database params needed for connecting to most databases.
pub struct DatabaseConfig {
    pub db_name: String,
    pub password: String,
    pub user_name: String,
    pub host: String,
    pub port: Option<String>,
    pub namespace: Option<String>,
}

impl DatabaseConfig {
    pub fn mysql_connection_string(&self) -> String {
        match &self.port {
            Some(port) => {
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    self.user_name, self.password, self.host, port, self.db_name
                )
            }
            None => {
                format!(
                    "mysql://{}:{}@{}/{}",
                    self.user_name, self.password, self.host, self.db_name
                )
            }
        }
    }
}
