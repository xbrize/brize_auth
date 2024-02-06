#[cfg(feature = "sessions")]
mod session_repo;
#[cfg(feature = "sessions")]
pub use session_repo::*;

mod creds_repo;
pub use creds_repo::*;

use sqlx::mysql::MySqlPool;

pub struct MySqlGateway {
    pub pool: MySqlPool,
}

impl MySqlGateway {
    pub async fn new(database_url: &str) -> Self {
        let pool = MySqlPool::connect(database_url)
            .await
            .expect("Failed connection with MySql database");

        Self { pool }
    }
}
