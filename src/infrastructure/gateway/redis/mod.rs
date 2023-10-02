use std::error::Error;

use redis::aio::Connection;
use redis::AsyncCommands;

use crate::application::SessionRepository;
use crate::domain::{Session, SessionRecordId};

pub struct RedisGateway {
    conn: Connection,
}

impl RedisGateway {
    pub async fn new(addr: &str) -> Self {
        let client = redis::Client::open(addr).unwrap();
        let conn = client.get_async_connection().await.unwrap();

        Self { conn }
    }
}

#[async_trait::async_trait]
impl SessionRepository for RedisGateway {
    async fn store_session(&mut self, session: &Session) -> Result<(), Box<dyn Error>> {
        let session_json = serde_json::to_string(&session)?;
        self.conn.set(&session.id, session_json).await?;
        Ok(())
    }

    async fn get_session_by_id(
        &mut self,
        session_record_id: &SessionRecordId,
    ) -> Result<Session, Box<dyn Error>> {
        let session_string: String = self.conn.get(session_record_id.to_string()).await?;
        let session: Session = serde_json::from_str(&session_string)?;

        Ok(session)
    }

    async fn delete_session(
        &mut self,
        _session_record_id: &SessionRecordId,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
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
        let query = redis_gateway.store_session(&session).await;
        assert!(query.is_ok());

        let session_from_storage = redis_gateway.get_session_by_id(&session.id).await.unwrap();
        assert_eq!(session_from_storage.is_expired(), false);
        assert_eq!(session_from_storage.id, session.id);
    }
}
