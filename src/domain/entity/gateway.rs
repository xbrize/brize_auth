pub struct DatabaseConfig {
    pub db_name: String,
    pub password: String,
    pub user_name: String,
    pub host: String,
}

pub enum GatewayType {
    MySql(DatabaseConfig),
    Surreal(DatabaseConfig),
    Redis(DatabaseConfig),
}
