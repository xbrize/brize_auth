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
        let db = Surreal::new::<Ws>("127.0.0.1:8000")
            .await
            .expect("Could not connect to database:");
        db.use_ns("test")
            .use_db("test")
            .await
            .expect("Could not connect to database:");

        Self { database: db }
    }
}
