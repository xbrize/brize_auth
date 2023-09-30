use brize_auth::{
    application::SessionRepository,
    domain::{Expiry, Session},
    infrastructure::{RedisGateway, SurrealGateway},
};
use redis::RedisResult;

#[tokio::main]
async fn main() -> RedisResult<()> {
    let mut redis_gateway = RedisGateway::new("redis://:mypassword@localhost/").await;
    let session = Session::new(Expiry::Day(1));

    let storage_result = redis_gateway.store_session(&session).await.unwrap();
    dbg!(&storage_result);

    let session = redis_gateway.get_session_by_id(&storage_result).await;
    dbg!(session.unwrap());

    // let mut repo = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;
    // let session = Session::new(Expiry::Day(1));
    // let session_id = repo.store_session(&session).await.unwrap();
    // dbg!(&session_id);

    // // Test get session
    // let session = repo.get_session_by_id(&session_id).await;
    // dbg!(session.unwrap());

    Ok(())
}
