use anyhow::{Context, Result};
use redis::aio::Connection;
use redis::AsyncCommands;

use crate::application::interface::SessionRepository;
use crate::domain::config::DatabaseConfig;
use crate::domain::entity::{Session, SessionToken};

pub struct RedisGateway {
    conn: Connection,
}

impl RedisGateway {
    pub async fn new(config: &DatabaseConfig) -> Self {
        let addr = format!(
            "redis://:{}@{}:{}/",
            config.password, config.host, config.port
        );
        let client = redis::Client::open(addr).expect("Failed to connect to Redis client");
        let conn = client
            .get_async_connection()
            .await
            .expect("Failed to make Redis async");

        Self { conn }
    }
}

#[async_trait::async_trait]
impl SessionRepository for RedisGateway {
    async fn insert_session(&mut self, session: &Session) -> Result<()> {
        let session_json = serde_json::to_string(&session)
            .context("Failed to serialize session id before storing in Redis")?;

        self.conn
            .set(&session.id, session_json)
            .await
            .context("Failed to store session in Redis")?;

        Ok(())
    }

    async fn get_session_by_id(&mut self, session_id: &SessionToken) -> Result<Session> {
        let session_string: String = self
            .conn
            .get(session_id.to_string())
            .await
            .context("Get session from Redis failed")?;

        let session: Session = serde_json::from_str(&session_string)
            .context("Could not deserialize session id from Redis")?;

        Ok(session)
    }

    async fn delete_session(&mut self, session_id: &SessionToken) -> Result<()> {
        self.conn
            .del(session_id)
            .await
            .context("Failed to delete session from Redis")?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::config::Expiry;

    #[tokio::test]
    async fn test_redis_gateway() {
        let config = DatabaseConfig {
            host: "localhost".to_string(),
            password: "mypassword".to_string(),
            user_name: "".to_string(),
            db_name: "".to_string(),
            port: "6379".to_string(),
            namespace: None,
        };

        let mut repo = RedisGateway::new(&config).await;

        let session = Session::new(&Expiry::Day(1), "user_identity@mail.com");
        let query = repo.insert_session(&session).await;
        assert!(query.is_ok());

        let session_from_storage = repo.get_session_by_id(&session.id).await.unwrap();
        assert_eq!(session_from_storage.is_expired(), false);
        assert_eq!(session_from_storage.id, session.id);

        repo.delete_session(&session.id).await.unwrap();
        let session_from_repo = repo.get_session_by_id(&session.id).await;
        assert!(session_from_repo.is_err());
    }
}
