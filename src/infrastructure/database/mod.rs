pub mod session_store;
pub mod user_store;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

pub type DatabaseClient = Surreal<Client>;

pub struct DataStore {
    database: DatabaseClient,
}

impl DataStore {
    pub async fn new(addr: &str, namespace: &str, database_name: &str) -> Self {
        let db = Surreal::new::<Ws>(addr)
            .await
            .expect("Could not connect to database:");
        db.use_ns(namespace)
            .use_db(database_name)
            .await
            .expect("Could not connect to database:");

        Self { database: db }
    }
}
