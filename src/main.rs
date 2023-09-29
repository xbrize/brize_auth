use brize_auth::{
    domain::{Expiry, Session},
    infrastructure::RedisGateway,
};
use redis::RedisResult;

#[tokio::main]
async fn main() -> RedisResult<()> {
    let mut redis_gateway = RedisGateway::new("redis://:mypassword@localhost/").await;
    let session = Session::new(Expiry::Day(1));

    let storage_result = redis_gateway.store_session_in_redis(&session).await;
    assert!(storage_result.is_ok());

    Ok(())
}
