use super::Auth;
use crate::{infrastructure::gateway, DatabaseConfig, SessionType};
use anyhow::Result;

pub struct AuthBuilder {
    pub credential_db_config: DatabaseConfig,
    pub session_db_config: DatabaseConfig,
    pub session_type: SessionType,
}

impl AuthBuilder {
    pub fn set_credentials_db_config(mut self, db_configs: DatabaseConfig) -> Self {
        self.credential_db_config = db_configs;
        self
    }

    pub fn set_sessions_db_config(mut self, db_configs: DatabaseConfig) -> Self {
        self.session_db_config = db_configs;
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
        let credentials_gateway =
            gateway::mysql::MySqlGateway::new(&self.credential_db_config).await;

        match self.session_type {
            SessionType::None => Ok(Auth {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::None,
            }),
            SessionType::Session(duration) => {
                let session_gateway =
                    gateway::mysql::MySqlGateway::new(&self.session_db_config).await;

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
        let credentials_gateway =
            gateway::surreal::SurrealGateway::new(&self.credential_db_config).await;

        match self.session_type {
            SessionType::None => Ok(Auth {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::None,
            }),
            SessionType::Session(duration) => {
                let session_gateway =
                    gateway::surreal::SurrealGateway::new(&self.session_db_config).await;

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

    #[cfg(feature = "surreal-redis")]
    pub async fn build(
        self,
    ) -> Result<Auth<gateway::surreal::SurrealGateway, gateway::redis::RedisGateway>> {
        let credentials_gateway =
            gateway::surreal::SurrealGateway::new(&self.credential_db_config).await;

        match self.session_type {
            SessionType::None => Ok(Auth {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::None,
            }),
            SessionType::Session(duration) => {
                let session_gateway =
                    gateway::redis::RedisGateway::new(&self.session_db_config).await;

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

    #[cfg(feature = "mysql-redis")]
    pub async fn build(
        self,
    ) -> Result<Auth<gateway::mysql::MySqlGateway, gateway::redis::RedisGateway>> {
        let credentials_gateway =
            gateway::mysql::MySqlGateway::new(&self.credential_db_config).await;

        match self.session_type {
            SessionType::None => Ok(Auth {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::None,
            }),
            SessionType::Session(duration) => {
                let session_gateway =
                    gateway::redis::RedisGateway::new(&self.session_db_config).await;

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
