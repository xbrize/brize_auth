#[cfg(feature = "sessions")]
mod session_repo;
#[cfg(feature = "sessions")]
pub use session_repo::*;

mod creds_repo;
pub use creds_repo::*;

use crate::domain::config::DatabaseConfig;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};

#[derive(Serialize, Deserialize)]
pub struct SurrealRecord<T> {
    id: Option<Thing>,
    data: T,
}

pub struct SurrealGateway {
    pub database: Surreal<Client>,
}

impl SurrealGateway {
    pub async fn new(config: &DatabaseConfig) -> Self {
        let address = match &config.port {
            Some(port) => {
                format!("{}:{}", config.host, port)
            }
            None => {
                format!("{}", config.host)
            }
        };

        let db = Surreal::new::<Ws>(address.as_str())
            .await
            .expect("Failed connection with SurrealDB");

        db.signin(Root {
            username: config.user_name.as_str(),
            password: config.password.as_str(),
        })
        .await
        .expect("Failed to sign into SurrealDB");

        let namespace = match &config.namespace {
            Some(namespace) => namespace.as_str(),
            None => "",
        };

        db.use_ns(namespace)
            .use_db(config.db_name.as_str())
            .await
            .expect("Failed connection with SurrealDB");

        Self { database: db }
    }
}
