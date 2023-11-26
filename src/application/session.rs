use crate::config::Expiry;
use crate::domain::entity::Session;

use crate::interface::SessionRepository;
use crate::{config::DatabaseConfig, infrastructure::gateway};
use anyhow::Result;

pub struct SessionClient<S: SessionRepository> {
    pub gateway: S,
}

#[cfg(feature = "mysql")]
impl SessionClient<gateway::mysql::MySqlGateway> {
    pub async fn new(db_configs: &DatabaseConfig) -> SessionClient<gateway::mysql::MySqlGateway> {
        let gateway = gateway::mysql::MySqlGateway::new(db_configs).await;

        SessionClient { gateway }
    }
}

#[cfg(feature = "surreal")]
impl SessionClient<gateway::surreal::SurrealGateway> {
    pub async fn _new(
        db_configs: &DatabaseConfig,
    ) -> SessionClient<gateway::surreal::SurrealGateway> {
        let gateway = gateway::surreal::SurrealGateway::new(db_configs).await;

        SessionClient { gateway }
    }
}

impl<S: SessionRepository> SessionClient<S> {
    /// Issues a new session token to start the user session
    pub async fn start_session(&self, user_id: &str, duration: Expiry) -> Result<Session> {
        let session = Session::new(&duration, user_id);
        self.gateway.insert_session(&session).await?;
        Ok(session)
    }

    /// Validates the session token
    pub async fn validate_session(&self, session_token: &str) -> Result<Session> {
        let session = self
            .gateway
            .get_session_by_id(&session_token.to_string())
            .await?;

        if session.is_expired() {
            self.gateway
                .delete_session(&session_token.to_string())
                .await?;
            Err(anyhow::anyhow!("Session expired"))
        } else {
            Ok(session)
        }
    }

    /// Get the session details for a token
    pub async fn get_session(&mut self, session_token: &str) -> Result<Session> {
        self.gateway
            .get_session_by_id(&session_token.to_string())
            .await
    }

    /// Deletes the session from the table
    pub async fn destory_session(&mut self, session_token: &str) -> Result<()> {
        self.gateway
            .delete_session(&session_token.to_string())
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;
    use crate::helpers::{mysql_configs, surreal_configs};

    #[tokio::test]
    async fn test_mysql_session() {
        let db_configs = mysql_configs();
        let sesh = SessionClient::new(&db_configs).await;
        let user_id = &uuid::Uuid::new_v4().to_string();

        // Test healthy session
        let sesh_details = sesh
            .start_session(user_id, Expiry::Second(20))
            .await
            .unwrap();

        let is_valid = sesh
            .validate_session(sesh_details.session_id.as_str())
            .await;
        assert!(is_valid.is_ok());

        // Test expired session
        let user_id = &uuid::Uuid::new_v4().to_string();
        let sesh_details = sesh
            .start_session(user_id, Expiry::Second(1))
            .await
            .unwrap();

        sleep(Duration::new(2, 0));
        let is_valid = sesh
            .validate_session(sesh_details.session_id.as_str())
            .await;
        assert!(is_valid.is_err());
    }

    #[tokio::test]
    async fn test_surreal_session() {
        let db_configs = surreal_configs();
        let sesh = SessionClient::new(&db_configs).await;
        let user_id = &uuid::Uuid::new_v4().to_string();

        // Test healthy session
        let sesh_details = sesh
            .start_session(user_id, Expiry::Second(20))
            .await
            .unwrap();

        let is_valid = sesh
            .validate_session(sesh_details.session_id.as_str())
            .await;
        assert!(is_valid.is_ok());

        // Test expired session
        let user_id = &uuid::Uuid::new_v4().to_string();
        let sesh_details = sesh
            .start_session(user_id, Expiry::Second(1))
            .await
            .unwrap();

        sleep(Duration::new(2, 0));
        let is_valid = sesh
            .validate_session(sesh_details.session_id.as_str())
            .await;
        assert!(is_valid.is_err());
    }
}
