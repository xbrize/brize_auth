#[cfg(feature = "mysql")]
pub mod mysql;

#[cfg(feature = "surreal")]
pub mod surreal;

use crate::domain::entity::{Claims, Credentials, CredentialsId, Session, SessionToken};
use crate::{
    application::interface::{CredentialsRepository, SessionRepository},
    infrastructure::services::{
        jwt::{generate_json_web_token, verify_json_web_token},
        password_hash::{hash_raw_password, verify_password},
    },
    SessionType,
};
use anyhow::{Context, Result};

pub struct Auth<C: CredentialsRepository, S: SessionRepository> {
    pub credentials_gateway: C,
    pub session_gateway: Option<S>,
    pub session_type: SessionType,
}

impl<C: CredentialsRepository, S: SessionRepository> Auth<C, S> {
    /// Register a new user and insert them into the database if user does not already exist
    pub async fn register(
        &mut self,
        user_identity: &str,
        raw_password: &str,
    ) -> Result<CredentialsId> {
        match self
            .credentials_gateway
            .find_credentials_by_user_identity(user_identity)
            .await
        {
            Ok(_) => {
                return Err(anyhow::anyhow!(
                    "Registration failed, credentials already exist"
                ))
            }
            Err(_) => {
                let hashed_password = hash_raw_password(raw_password);

                let credentials = Credentials::new(user_identity, hashed_password.as_str());

                self.credentials_gateway
                    .insert_credentials(&credentials)
                    .await
                    .context("Registration failed, repository error")?;

                return Ok(credentials.id);
            }
        };
    }

    /// Verify user credentials and issue a session token.
    ///
    /// If sessions are not enabled, use the verify_credentials method instead.
    pub async fn login(&mut self, user_identity: &str, raw_password: &str) -> Result<SessionToken> {
        if self.verify_credentials(user_identity, raw_password).await {
            let session_record_id = self
                .start_session(user_identity)
                .await
                .context("Login failed, session creation failure")?;

            Ok(session_record_id)
        } else {
            Err(anyhow::anyhow!("Login failed, invalid credentials"))
        }
    }

    /// End the user's session.
    ///
    /// If sessions are not enabled, this will throw and error.
    pub async fn logout(&mut self, session_token: &str) -> Result<()> {
        match self.session_gateway {
            Some(ref mut gateway) => {
                gateway
                    .delete_session(&session_token.to_string())
                    .await
                    .context("Logout failed, session was not able to be deleted")?;

                Ok(())
            }
            None => Err(anyhow::anyhow!("Logout failed, sessions not enabled")),
        }
    }

    /// Matches credentials provided by the user with the what is in the database
    pub async fn verify_credentials(&self, user_identity: &str, raw_password: &str) -> bool {
        match self
            .credentials_gateway
            .find_credentials_by_user_identity(&user_identity)
            .await
        {
            Ok(credentials) => {
                if verify_password(raw_password, &credentials.hashed_password).is_ok() {
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    /// Issues a new session token to start the user session
    async fn start_session(&mut self, user_identity: &str) -> Result<SessionToken> {
        match &self.session_type {
            SessionType::JWT(duration) => {
                let claims = Claims::new(user_identity, duration);
                let token = generate_json_web_token(claims)?;

                Ok(token)
            }
            SessionType::Session(duration) => {
                let session = Session::new(duration, user_identity);
                match self.session_gateway {
                    Some(ref mut gateway) => {
                        gateway.insert_session(&session).await?;
                        Ok(session.id)
                    }
                    None => Err(anyhow::anyhow!("Sessions not enabled")),
                }
            }
            _ => Err(anyhow::anyhow!("Sessions not enabled")),
        }
    }

    /// Validates the session token
    pub async fn validate_session(&mut self, session_token: &str) -> Result<String> {
        match self.session_type {
            SessionType::JWT(_) => verify_json_web_token(session_token),
            SessionType::Session(_) => match self.session_gateway {
                Some(ref mut gateway) => {
                    let attempt_to_get_session =
                        gateway.get_session_by_id(&session_token.to_string()).await;

                    match attempt_to_get_session {
                        Ok(session) => {
                            if session.is_expired() {
                                gateway.delete_session(&session_token.to_string()).await?;
                                Err(anyhow::anyhow!("Session expired"))
                            } else {
                                Ok(session.user_identity)
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                None => Err(anyhow::anyhow!("Sessions not enabled")),
            },
            SessionType::None => Err(anyhow::anyhow!("Sessions not enabled")),
        }
    }
}
