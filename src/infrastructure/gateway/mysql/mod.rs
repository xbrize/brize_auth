#[cfg(feature = "sessions")]
mod session_repo;
#[cfg(feature = "sessions")]
pub use session_repo::*;

mod creds_repo;
pub use creds_repo::*;

use crate::domain::config::DatabaseConfig;
use sqlx::mysql::MySqlPool;

pub struct MySqlGateway {
    pub pool: MySqlPool,
}

impl MySqlGateway {
    pub async fn new(config: &DatabaseConfig) -> Self {
        let addr = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.user_name, config.password, config.host, config.port, config.db_name
        );
        let pool = MySqlPool::connect(addr.as_str())
            .await
            .expect("Failed connection with MySql database");

        Self { pool }
    }
}
