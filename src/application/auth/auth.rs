use super::AuthConfig;
use crate::{
    application::{CredentialsRepository, SessionRepository},
    domain::{
        Claims, Credentials, CredentialsId, GatewayType, Session, SessionRecordId, SessionType,
    },
    infrastructure::{MySqlGateway, RedisGateway, SurrealGateway},
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::error::Error;

pub struct Auth {
    credentials_gateway: Box<dyn CredentialsRepository>,
    session_gateway: Option<Box<dyn SessionRepository>>,
    session_type: SessionType,
}

static SECRET: &'static str = "super_secret_key";
impl Auth {
    pub async fn new(auth_config: AuthConfig) -> Result<Self, Box<dyn Error>> {
        // ** Credentials config
        let credentials_gateway_config = auth_config
            .credentials_gateway
            .expect("Credentials Gateway Not Configured");

        let credentials_gateway: Box<dyn CredentialsRepository> = match &credentials_gateway_config
        {
            GatewayType::Surreal(config) => Box::new(SurrealGateway::new(&config).await),
            GatewayType::MySql(config) => Box::new(MySqlGateway::new(&config).await),
            GatewayType::Redis(_) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Credentials Gateway Config Does Not Support Reddis",
                )))
            }
        };

        // ** Session Config
        match auth_config.session_type {
            SessionType::None => Ok(Self {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::None,
            }),
            SessionType::Session(duration) => {
                let session_gateway: Box<dyn SessionRepository> = match auth_config.session_gateway
                {
                    Some(gateway_type) => match gateway_type {
                        // Custom gateway case
                        GatewayType::Surreal(config) => {
                            Box::new(SurrealGateway::new(&config).await)
                        }
                        GatewayType::MySql(config) => Box::new(MySqlGateway::new(&config).await),
                        GatewayType::Redis(config) => Box::new(RedisGateway::new(&config).await),
                    },
                    None => match &credentials_gateway_config {
                        // Default case, make same as credentials gateway
                        GatewayType::Surreal(config) => {
                            Box::new(SurrealGateway::new(&config).await)
                        }
                        GatewayType::MySql(config) => Box::new(MySqlGateway::new(&config).await),
                        GatewayType::Redis(config) => Box::new(RedisGateway::new(&config).await),
                    },
                };

                Ok(Self {
                    credentials_gateway,
                    session_gateway: Some(session_gateway),
                    session_type: SessionType::Session(duration),
                })
            }
            SessionType::JWT(duration) => Ok(Self {
                credentials_gateway,
                session_gateway: None,
                session_type: SessionType::JWT(duration),
            }),
        }
    }

    pub async fn register(
        &mut self,
        user_identity: &str,
        raw_password: &str,
    ) -> Option<CredentialsId> {
        match self
            .credentials_gateway
            .find_credentials_by_user_identity(user_identity)
            .await
        {
            Ok(credentials_query) => match credentials_query {
                Some(_) => {
                    println!("Credentials Already Exist, User Not Created");
                    return None;
                }
                None => {
                    println!("New User Created");
                    let credentials = Credentials::new(user_identity, raw_password).hash_password();
                    self.credentials_gateway
                        .insert_credentials(&credentials)
                        .await
                        .unwrap();

                    return Some(credentials.id);
                }
            },
            Err(e) => {
                println!("Failed to register user:{}", e);
                return None;
            }
        };
    }

    pub async fn login(
        &mut self,
        user_identity: &str,
        raw_password: &str,
    ) -> Result<SessionRecordId, Box<dyn Error>> {
        if self.match_credentials(user_identity, raw_password).await {
            let session_record_id = self.start_session(user_identity).await?;
            Ok(session_record_id)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Credentials did not match",
            )))
        }
    }

    pub async fn logout(&mut self, session_token: &str) -> Result<(), Box<dyn Error>> {
        match self.session_gateway {
            Some(ref mut gateway) => {
                gateway.delete_session(&session_token.to_string()).await?;
                Ok(())
            }
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Session gateway not configured",
            ))),
        }
    }

    async fn match_credentials(&self, user_identity: &str, raw_password: &str) -> bool {
        match self
            .credentials_gateway
            .find_credentials_by_user_identity(&user_identity)
            .await
        {
            Ok(credentials_query) => match credentials_query {
                Some(credentials) => {
                    if credentials.verify_password(raw_password) {
                        true
                    } else {
                        println!("Password Did Not Match");
                        false
                    }
                }
                None => {
                    println!("User Credentials Not Found For Login");
                    false
                }
            },
            Err(e) => {
                println!("Error Logging In User:{:#?}", e);
                false
            }
        }
    }

    pub async fn start_session(
        &mut self,
        user_identity: &str,
    ) -> Result<SessionRecordId, Box<dyn Error>> {
        match self.session_type {
            SessionType::JWT(duration) => {
                let claims = Claims::new(user_identity, duration);
                let token = Self::encode_token(claims)?;

                Ok(token)
            }
            SessionType::Session(duration) => {
                let session = Session::new(duration);
                match self.session_gateway {
                    Some(ref mut gateway) => {
                        gateway.store_session(&session).await?;
                        Ok(session.id)
                    }
                    None => Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Session not enabled",
                    ))),
                }
            }
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Session not enabled",
            ))),
        }
    }

    pub async fn validate_session(&mut self, session_token: &str) -> Result<bool, Box<dyn Error>> {
        match self.session_type {
            SessionType::JWT(_) => {
                let valid = Self::decode_token(&session_token);

                if valid.is_ok() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            SessionType::Session(_) => match self.session_gateway {
                Some(ref mut gateway) => {
                    let attempt_to_get_session =
                        gateway.get_session_by_id(&session_token.to_string()).await;

                    match attempt_to_get_session {
                        Ok(session) => {
                            if session.is_expired() {
                                gateway.delete_session(&session_token.to_string()).await?;
                                Ok(false)
                            } else {
                                Ok(true)
                            }
                        }
                        Err(_) => {
                            println!("Failed to get session during validation");
                            Ok(false)
                        }
                    }
                }
                None => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Session not enabled",
                ))),
            },
            SessionType::None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Session type set to none",
            ))),
        }
    }

    pub fn encode_token(claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
        let header = Header::new(Algorithm::HS256);
        encode(&header, &claims, &EncodingKey::from_secret(SECRET.as_ref()))
    }

    pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let validation = Validation::new(Algorithm::HS256);
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &validation,
        )
        .map(|c| c.claims)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{DatabaseConfig, Expiry};

    use super::*;

    #[tokio::test]
    async fn test_auth_mysql() {
        let db_config = DatabaseConfig {
            host: "localhost:3306".to_string(),
            db_name: "mysql".to_string(),
            user_name: "root".to_string(),
            password: "my-secret-pw".to_string(),
        };

        let repo = MySqlGateway::new(&db_config).await;
        repo.create_credentials_table().await;
        repo.create_session_table().await;

        let config = AuthConfig::new()
            .set_credentials_gateway(GatewayType::MySql(db_config))
            .set_session_type(SessionType::Session(Expiry::Month(1)));

        let mut auth = Auth::new(config).await.unwrap();

        let random_string = uuid::Uuid::new_v4().to_string();
        let user_identity = &random_string[0..10];
        let raw_password = &random_string[0..8];

        auth.register(user_identity, raw_password).await.unwrap();
        let session = auth.login(user_identity, raw_password).await.unwrap();
        let validation = auth.validate_session(session.as_str()).await.unwrap();
        assert!(validation);

        auth.logout(&session).await.unwrap();
        let validation = auth.validate_session(session.as_str()).await.unwrap();
        assert!(!validation)
    }

    #[tokio::test]
    async fn test_auth_surreal() {
        let db_config = DatabaseConfig {
            db_name: "test".to_string(),
            host: "127.0.0.1:8000".to_string(),
            user_name: "test".to_string(),
            password: "".to_string(),
        };

        let config = AuthConfig::new()
            .set_credentials_gateway(GatewayType::Surreal(db_config))
            .set_session_type(SessionType::Session(Expiry::Month(1)));

        let mut auth = Auth::new(config).await.unwrap();

        let random_string = uuid::Uuid::new_v4().to_string();
        let user_identity = &random_string[0..10];
        let raw_password = &random_string[0..8];

        auth.register(user_identity, raw_password).await.unwrap();
        let session = auth.login(user_identity, raw_password).await.unwrap();
        let validation = auth.validate_session(session.as_str()).await.unwrap();
        assert!(validation);

        auth.logout(&session).await.unwrap();
        let validation = auth.validate_session(session.as_str()).await.unwrap();
        assert!(!validation)
    }
}
