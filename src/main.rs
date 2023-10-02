use brize_auth::{
    application::SessionRepository,
    domain::{Expiry, Session},
    infrastructure::{MySqlGateway, RedisGateway, SurrealGateway},
};
use redis::RedisResult;
use sqlx::*;

#[tokio::main]
async fn main() -> RedisResult<()> {
    Ok(())
}
