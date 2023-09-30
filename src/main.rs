use brize_auth::{
    application::SessionRepository,
    domain::{Expiry, Session},
    infrastructure::{MySqlGateway, RedisGateway, SurrealGateway},
};
use redis::RedisResult;
use sqlx::*;

#[tokio::main]
async fn main() -> RedisResult<()> {
    let url = "mysql://root:my-secret-pw@localhost:3306/mysql";
    let mut repo = MySqlGateway::new(url).await;
    repo.create_session_table().await;

    let session = &Session::new(Expiry::Day(1));
    let session_id = repo.store_session(session).await.unwrap();
    assert_eq!(session_id, session.id);

    let session_from_repo = repo.get_session_by_id(&session_id).await.unwrap();
    assert_eq!(session_from_repo.is_expired(), false);
    assert_eq!(session_from_repo.id, session.id);

    Ok(())
}
