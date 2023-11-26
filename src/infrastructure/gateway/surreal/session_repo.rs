use super::{SurrealGateway, SurrealRecord};
use crate::{
    entity::{Session, SessionToken},
    interface::SessionRepository,
};
use anyhow::{Context, Result};

#[async_trait::async_trait]
impl SessionRepository for SurrealGateway {
    async fn get_session_by_id(&self, session_id: &SessionToken) -> Result<Session> {
        let query_for_record: Option<SurrealRecord<Session>> = self
            .database
            .select(("user_sessions", session_id))
            .await
            .context("Failed to get session by id from Surreal")?;

        match query_for_record {
            Some(record) => Ok(record.data),
            None => Err(anyhow::anyhow!("Session not found by id in Surreal")),
        }
    }

    async fn insert_session(&self, session: &Session) -> Result<()> {
        let record = SurrealRecord {
            id: None,
            data: session,
        };

        self.database
            .create::<Option<SurrealRecord<Session>>>(("user_sessions", &session.session_id))
            .content(&record)
            .await
            .context("Failed to store session in Surreal")?;

        Ok(())
    }

    async fn delete_session(&self, session_id: &SessionToken) -> Result<()> {
        self.database
            .delete::<Option<SurrealRecord<Session>>>(("user_sessions", session_id))
            .await
            .context("Failed to delete session from Surreal")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{domain::config::Expiry, helpers::surreal_configs};

    use super::*;

    #[tokio::test]
    async fn test_surreal_session_repo() {
        let db_config = surreal_configs();
        let repo = SurrealGateway::new(&db_config).await;

        let session = Session::new(&Expiry::Day(1), "user_name@mail.com");
        let query = repo.insert_session(&session).await;
        assert!(query.is_ok());

        let session_from_storage = repo.get_session_by_id(&session.session_id).await.unwrap();
        assert!(!session_from_storage.is_expired());
        assert_eq!(session_from_storage.session_id, session.session_id);
        assert_eq!(session_from_storage.csrf_token, session.csrf_token);

        repo.delete_session(&session.session_id).await.unwrap();
        let session_from_repo = repo.get_session_by_id(&session.session_id).await;
        assert!(session_from_repo.is_err());
    }
}
