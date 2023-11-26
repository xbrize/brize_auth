/// These are the database params needed for connecting to most databases.
pub struct DatabaseConfig {
    pub db_name: String,
    pub password: String,
    pub user_name: String,
    pub host: String,
    pub port: String,
    pub namespace: Option<String>,
}
