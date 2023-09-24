pub mod user_repository;

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record<T> {
    pub id: Thing,
    pub data: T,
}
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
