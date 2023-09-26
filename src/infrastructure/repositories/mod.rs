pub mod session_repository;
pub mod user_repository;

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
