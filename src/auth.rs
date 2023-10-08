use crate::{
    application::{CredentialsRepository, SessionRepository},
    infrastructure::{MySqlGateway, RedisGateway, SurrealGateway},
};
use redis::Expiry;
use std::error::Error;

pub struct Auth {
    credentials_gateway: Box<dyn CredentialsRepository>,
    credentials_table_name: String,
    session_gateway: Option<Box<dyn SessionRepository>>,
    session_table_name: String,
    jwt_token_expiry: Option<Expiry>,
}

pub enum SessionType {
    JWT(Expiry),
    Session,
}

pub enum GatewayConfig {
    MySqlGateway(String),
    SurrealGateway((String, String, String)),
    RedisGateway(String),
}

pub struct AuthConfig {
    credentials_gateway: Option<GatewayConfig>,
    credentials_table_name: Option<String>,
    session_gateway: Option<GatewayConfig>,
    session_table_name: Option<String>,
    session_type: SessionType,
}

impl AuthConfig {
    pub fn new() -> Self {
        Self {
            credentials_gateway: None,
            credentials_table_name: None,
            session_gateway: None,
            session_table_name: None,
            session_type: SessionType::Session,
        }
    }

    pub fn set_credentials_gateway(mut self, config: GatewayConfig) -> Self {
        self.credentials_gateway = Some(config);
        self
    }

    pub fn set_credentials_table_name(mut self, name: &str) -> Self {
        self.credentials_table_name = Some(name.to_string());
        self
    }

    pub fn set_session_gateway(mut self, config: GatewayConfig) -> Self {
        self.session_gateway = Some(config);
        self
    }

    pub fn set_session_table_name(mut self, name: &str) -> Self {
        self.session_table_name = Some(name.to_string());
        self
    }

    pub fn use_jwt_token(mut self, expiration: Expiry) -> Self {
        self.session_gateway = None;
        self.session_table_name = None;
        self.session_type = SessionType::JWT(expiration);
        self
    }
}

impl Auth {
    pub async fn new(auth_config: AuthConfig) -> Result<Self, Box<dyn Error>> {
        let credentials_gateway_config = auth_config
            .credentials_gateway
            .expect("Credentials Gateway Not Configured");

        let credentials_table_name = auth_config
            .credentials_table_name
            .unwrap_or("credentials".to_string());

        let session_table_name = auth_config
            .session_table_name
            .unwrap_or("sessions".to_string());

        let credentials_gateway: Box<dyn CredentialsRepository> = match credentials_gateway_config {
            GatewayConfig::SurrealGateway(params) => Box::new(SurrealGateway::new(params).await),
            GatewayConfig::MySqlGateway(url) => Box::new(MySqlGateway::new(&url).await),
            GatewayConfig::RedisGateway(url) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Gateway Config Does Not Support Reddis",
                )))
            }
        };

        let session_gateway: Option<Box<dyn SessionRepository>> = match auth_config.session_gateway
        {
            Some(gateway_config) => match gateway_config {
                GatewayConfig::SurrealGateway(params) => {
                    Some(Box::new(SurrealGateway::new(params).await))
                }
                GatewayConfig::MySqlGateway(url) => Some(Box::new(MySqlGateway::new(&url).await)),
                GatewayConfig::RedisGateway(url) => Some(Box::new(RedisGateway::new(&url).await)),
            },
            None => None,
        };

        let session_type = auth_config.session_type;

        let jwt_token_expiry = match session_type {
            SessionType::JWT(expiration) => Some(expiration),
            _ => None,
        };

        Ok(Self {
            credentials_gateway,
            credentials_table_name,
            session_gateway,
            session_table_name,
            jwt_token_expiry,
        })
    }
}
