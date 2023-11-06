use super::Auth;
use crate::{
    config::{DatabaseConfig, SessionType},
    infrastructure::gateway,
};
use anyhow::Result;

pub struct AuthBuilder {
    pub credential_db_config: Option<DatabaseConfig>,
    pub session_db_config: Option<DatabaseConfig>,
    pub session_type: SessionType,
}

impl AuthBuilder {
    pub fn new() -> Self {
        AuthBuilder {
            credential_db_config: None,
            session_db_config: None,
            session_type: SessionType::None,
        }
    }

    pub fn set_credentials_db_config(mut self, db_configs: DatabaseConfig) -> Self {
        self.credential_db_config = Some(db_configs);
        self
    }

    pub fn set_sessions_db_config(mut self, db_configs: DatabaseConfig) -> Self {
        self.session_db_config = Some(db_configs);
        self
    }

    pub fn set_session_type(mut self, session_type: SessionType) -> Self {
        self.session_type = session_type;
        self
    }

    #[cfg(feature = "mysql")]
    pub async fn build(
        self,
    ) -> Result<Auth<gateway::mysql::MySqlGateway, gateway::mysql::MySqlGateway>> {
        let cred_configs = &self
            .credential_db_config
            .expect("Credentials database config not set");

        let credentials_gateway = gateway::mysql::MySqlGateway::new(cred_configs).await;

        match self.session_type {
            SessionType::None => Ok(Auth {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::None,
            }),
            SessionType::Session(duration) => {
                let sesh_configs = &self
                    .session_db_config
                    .expect("Sessions database config not set");
                let session_gateway = gateway::mysql::MySqlGateway::new(sesh_configs).await;

                Ok(Auth {
                    credentials_gateway,
                    session_gateway: Some(session_gateway),
                    session_type: SessionType::Session(duration),
                })
            }
            SessionType::JWT(duration) => Ok(Auth {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::JWT(duration),
            }),
        }
    }

    #[cfg(feature = "surreal")]
    pub async fn build(
        self,
    ) -> Result<Auth<gateway::surreal::SurrealGateway, gateway::surreal::SurrealGateway>> {
        let cred_configs = &self
            .credential_db_config
            .expect("Credentials database config not set");
        let credentials_gateway = gateway::surreal::SurrealGateway::new(cred_configs).await;

        match self.session_type {
            SessionType::None => Ok(Auth {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::None,
            }),
            SessionType::Session(duration) => {
                let sesh_configs = &self
                    .session_db_config
                    .expect("Sessions database config not set");
                let session_gateway = gateway::surreal::SurrealGateway::new(sesh_configs).await;

                Ok(Auth {
                    credentials_gateway,
                    session_gateway: Some(session_gateway),
                    session_type: SessionType::Session(duration),
                })
            }
            SessionType::JWT(duration) => Ok(Auth {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::JWT(duration),
            }),
        }
    }

    #[cfg(all(feature = "redis", feature = "surreal"))]
    pub async fn build_with_redis_sessions(
        self,
    ) -> Result<Auth<gateway::surreal::SurrealGateway, gateway::redis::RedisGateway>> {
        let cred_configs = &self
            .credential_db_config
            .expect("Credentials database config not set");
        let credentials_gateway = gateway::surreal::SurrealGateway::new(cred_configs).await;

        match self.session_type {
            SessionType::None => Ok(Auth {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::None,
            }),
            SessionType::Session(duration) => {
                let sesh_configs = &self
                    .session_db_config
                    .expect("Sessions database config not set");
                let session_gateway = gateway::redis::RedisGateway::new(sesh_configs).await;

                Ok(Auth {
                    credentials_gateway,
                    session_gateway: Some(session_gateway),
                    session_type: SessionType::Session(duration),
                })
            }
            SessionType::JWT(duration) => Ok(Auth {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::JWT(duration),
            }),
        }
    }

    #[cfg(all(feature = "redis", feature = "mysql"))]
    pub async fn build_with_redis_sessions(
        self,
    ) -> Result<Auth<gateway::mysql::MySqlGateway, gateway::redis::RedisGateway>> {
        let cred_configs = &self
            .credential_db_config
            .expect("Credentials database config not set");
        let credentials_gateway = gateway::mysql::MySqlGateway::new(cred_configs).await;

        match self.session_type {
            SessionType::None => Ok(Auth {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::None,
            }),
            SessionType::Session(duration) => {
                let sesh_configs = &self
                    .session_db_config
                    .expect("Sessions database config not set");
                let session_gateway = gateway::redis::RedisGateway::new(sesh_configs).await;

                Ok(Auth {
                    credentials_gateway,
                    session_gateway: Some(session_gateway),
                    session_type: SessionType::Session(duration),
                })
            }
            SessionType::JWT(duration) => Ok(Auth {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::JWT(duration),
            }),
        }
    }
}
