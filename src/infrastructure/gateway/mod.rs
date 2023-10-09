mod redis;
pub use redis::*;

mod sql;
pub use sql::*;

mod surreal;
pub use surreal::*;

pub struct DatabaseConfig {
    pub db_name: String,
    pub password: String,
    pub user_name: String,
    pub host: String,
}
