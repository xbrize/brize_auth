use redis::aio::Connection;
use redis::{AsyncCommands, RedisResult};

use crate::application::SessionRepository;
use crate::domain::{RepoResult, RepositoryError, Session, SessionRecordId};

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
    async fn store_session(&mut self, session: &Session) -> RepoResult<SessionRecordId> {
        let session_json = serde_json::to_string(&session).unwrap();
        let setter: RedisResult<()> = self.conn.set(&session.id, session_json).await;

        match setter {
            Ok(_) => Ok(session.id.to_owned()),
            Err(_) => Err(RepositoryError::QueryFail),
        }
    }

    async fn get_session_by_id(
        &mut self,
        session_record_id: &SessionRecordId,
    ) -> Result<Session, RepositoryError> {
        let session_json: RedisResult<String> = self.conn.get(session_record_id.to_string()).await;
        match session_json {
            Ok(session_string) => {
                let session: Session = serde_json::from_str(&session_string).unwrap();
                Ok(session)
            }
            Err(_) => Err(RepositoryError::QueryFail),
        }
    }

    async fn delete_session(
        &mut self,
        _session_record_id: &SessionRecordId,
    ) -> Result<(), RepositoryError> {
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

        let storage_result_id = redis_gateway.store_session(&session).await;
        assert!(storage_result_id.is_ok());

        let session_from_storage = redis_gateway
            .get_session_by_id(&storage_result_id.unwrap())
            .await
            .unwrap();
        assert_eq!(session_from_storage.is_expired(), false);
        assert_eq!(session_from_storage.id, session.id);
    }
}
