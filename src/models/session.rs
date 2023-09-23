#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Id, Thing};
use surrealdb::{Error, Surreal};

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionStatus {
    PURGE,
    CHANGED,
    RENEWED,
    UNCHANGED,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    id: Thing,
    user: Thing,
    created_at: String,
    updated_at: String,
    expires_at: String,
    is_expired: bool,
}

pub async fn create_session(
    database: Surreal<Client>,
    user_record_link: Thing,
) -> surrealdb::Result<()> {
    let sql = "
    RETURN (CREATE session:uuid() CONTENT {
        user: $user,
        created_at: time::now(),
        updated_at: <future> {time::now()},
        expires_at: time::now() + 2w,
        is_expired: <future> {expires_at < time::now()}
    }).id
    ";

    let mut query = database.query(sql).bind(("user", user_record_link)).await?;
    let session: Option<Thing> = query.take(0)?;
    dbg!(session);
    Ok(())
}
