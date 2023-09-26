pub mod session_store;
pub mod user_store;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

pub type DatabaseClient = Surreal<Client>;

pub async fn initialize_test_database() -> DatabaseClient {
    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("Could not connect to database:");
    db.use_ns("test")
        .use_db("test")
        .await
        .expect("Could not connect to database:");

    db
}

pub struct DataStore<'a> {
    database: &'a DatabaseClient,
}

impl<'a> DataStore<'a> {
    pub fn new(database: &'a DatabaseClient) -> Self {
        Self { database }
    }
}
