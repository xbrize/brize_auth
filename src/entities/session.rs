#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    id: Thing,
    user: Thing,
    created_at: String,
    updated_at: String,
    expires_at: String,
    is_expired: bool,
}

pub async fn create_session(database: &Surreal<Client>, user_record_link: Thing) -> Option<Thing> {
    let sql = "
    RETURN (CREATE session:uuid() CONTENT {
        user: $user,
        created_at: time::now(),
        updated_at: <future> {time::now()},
        expires_at: time::now() + 2w,
        is_expired: <future> {expires_at < time::now()}
    }).id
    ";

    match database.query(sql).bind(("user", user_record_link)).await {
        Ok(mut response) => match response.take(0) {
            Ok(session_id) => session_id,
            Err(e) => {
                println!("No session data in response:\n{}", e);
                None
            }
        },
        Err(e) => {
            println!("Failed to create session:\n{}", e);
            None
        }
    }
}

pub async fn get_session(database: &Surreal<Client>, session_record_id: Thing) -> Option<Session> {
    match database.select(session_record_id).await {
        Ok(session) => session,
        Err(e) => {
            println!("Error getting session:\n{}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use surrealdb::{engine::remote::ws::Ws, sql::Id, Surreal};

    async fn setup_db() -> Surreal<Client> {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_session_model() {
        let email = "test@email.com";

        // Start database
        let db = setup_db().await;

        // Test Create session
        let new_session = create_session(
            &db,
            Thing {
                tb: String::from("user"),
                id: Id::String(String::from(email)),
            },
        )
        .await;
        assert!(new_session.is_some());

        // Test get session
        let session = get_session(&db, new_session.unwrap()).await;
        assert!(session.is_some());
    }
}
