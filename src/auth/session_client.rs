use crate::config::Expiry;
use crate::domain::entity::{Session, SessionToken};
use crate::interface::SessionRepository;
use crate::{config::DatabaseConfig, infrastructure::gateway};
use anyhow::Result;

pub struct SessionClient<S: SessionRepository> {
    pub gateway: S,
}

impl<S: SessionRepository> SessionClient<S> {
    #[cfg(feature = "mysql")]
    pub async fn new(db_configs: &DatabaseConfig) -> SessionClient<gateway::mysql::MySqlGateway> {
        let gateway = gateway::mysql::MySqlGateway::new(db_configs).await;

        SessionClient { gateway }
    }

    #[cfg(feature = "surreal")]
    pub async fn _new(
        db_configs: &DatabaseConfig,
    ) -> SessionClient<gateway::surreal::SurrealGateway> {
        let gateway = gateway::surreal::SurrealGateway::new(db_configs).await;

        SessionClient { gateway }
    }
}

impl<C: SessionRepository> SessionClient<C> {
    /// Issues a new session token to start the user session
    pub async fn start_session(&mut self, user_id: &str, duration: Expiry) -> Result<SessionToken> {
        let session = Session::new(&duration, user_id);
        self.gateway.insert_session(&session).await?;
        Ok(session.id)
    }

    /// Validates the session token
    pub async fn validate_session(&mut self, session_token: &str) -> Result<String> {
        let attempt_to_get_session = self
            .gateway
            .get_session_by_id(&session_token.to_string())
            .await;

        match attempt_to_get_session {
            Ok(session) => {
                if session.is_expired() {
                    self.gateway
                        .delete_session(&session_token.to_string())
                        .await?;
                    Err(anyhow::anyhow!("Session expired"))
                } else {
                    Ok(session.user_identity)
                }
            }
            Err(e) => Err(anyhow::anyhow!("Session validation error: {}", e)),
        }
    }
}
