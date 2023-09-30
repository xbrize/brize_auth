use redis::aio::Connection;
use redis::{AsyncCommands, RedisResult};

use crate::domain::Session;

pub struct RedisGateway {
    conn: Connection,
}

impl RedisGateway {
    pub async fn new(addr: &str) -> Self {
        let client = redis::Client::open(addr).unwrap();
        let conn = client.get_async_connection().await.unwrap();

        Self { conn }
    }

    pub async fn store_session_in_redis(&mut self, session: &Session) -> RedisResult<()> {
        let session_json = serde_json::to_string(&session).unwrap();
        self.conn.set(&session.id, session_json).await?;
        Ok(())
    }

    pub async fn get_session_from_redis(
        &mut self,
        session_id: &str,
    ) -> RedisResult<Session> {
        let session_json: String = self.conn.get(session_id).await?;
        let session: Session = serde_json::from_str(&session_json).unwrap();
        Ok(session)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::Expiry;

    #[tokio::test]
    async fn test_redis_gateway() {
        let mut redis_gateway = RedisGateway::new("redis://:mypassword@localhost/").await;
        let session = Session::new(Expiry::Day(1));

        let storage_result = redis_gateway.store_session_in_redis(&session).await;
        assert!(storage_result.is_ok());

        let session_from_storage = redis_gateway
            .get_session_from_redis(&session.id)
            .await
            .unwrap();
        assert_eq!(session_from_storage.created_at, session.created_at);
        assert_eq!(session_from_storage.is_expired(), false);
    }
}
