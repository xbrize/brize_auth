use crate::{
    application::{
        login_user, register_user, start_session, CredentialsRepository, SessionRepository,
    },
    domain::{Claims, Credentials, CredentialsId, Expiry, Session, SessionRecordId},
    infrastructure::{MySqlGateway, RedisGateway, SurrealGateway},
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::error::Error;

pub struct Auth {
    credentials_gateway: Box<dyn CredentialsRepository>,
    credentials_table_name: String,
    session_duration: Expiry,
    // session_type: SessionType,
    // session_gateway: Box<dyn SessionRepository>,
    // session_table_name: String,
}

// #[derive(Clone, Copy)]
// pub enum SessionType {
//     JWT(Expiry),
//     Session(Expiry),
//     None,
// }

pub enum GatewayConfig {
    MySqlGateway(String),
    SurrealGateway((String, String, String)),
    RedisGateway(String),
}

pub struct AuthConfig {
    credentials_gateway: Option<GatewayConfig>,
    credentials_table_name: Option<String>,
    session_duration: Option<Expiry>, // session_gateway: Option<GatewayConfig>,
                                      // session_table_name: Option<String>,
                                      // session_type: SessionType,
}

impl AuthConfig {
    pub fn new() -> Self {
        Self {
            credentials_gateway: None,
            credentials_table_name: None,
            session_duration: None,
            // session_gateway: None,
            // session_table_name: None,
            // session_type: SessionType::Session(Expiry::Month(1)),
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

    pub fn set_session_duration(mut self, duration: Expiry) -> Self {
        self.session_duration = Some(duration);
        self
    }

    // pub fn set_session_gateway(mut self, config: GatewayConfig) -> Self {
    //     self.session_gateway = Some(config);
    //     self
    // }

    // pub fn set_session_table_name(mut self, name: &str) -> Self {
    //     self.session_table_name = Some(name.to_string());
    //     self
    // }

    // pub fn use_jwt_token(mut self, expiration: Expiry) -> Self {
    //     self.session_gateway = None;
    //     self.session_table_name = None;
    //     // self.session_type = SessionType::JWT(expiration);
    //     self
    // }
}

static SECRET: &'static str = "super_secret_key";
impl Auth {
    pub async fn new(auth_config: AuthConfig) -> Result<Self, Box<dyn Error>> {
        let credentials_gateway_config = auth_config
            .credentials_gateway
            .expect("Credentials Gateway Not Configured");

        let credentials_table_name = auth_config
            .credentials_table_name
            .unwrap_or("credentials".to_string());

        let session_duration = auth_config.session_duration.unwrap_or(Expiry::Month(1));

        // let session_table_name = auth_config
        //     .session_table_name
        //     .unwrap_or("sessions".to_string());

        let credentials_gateway: Box<dyn CredentialsRepository> = match credentials_gateway_config {
            GatewayConfig::SurrealGateway(params) => Box::new(SurrealGateway::new(params).await),
            GatewayConfig::MySqlGateway(url) => Box::new(MySqlGateway::new(&url).await),
            GatewayConfig::RedisGateway(_) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Credentials Gateway Config Does Not Support Reddis",
                )))
            }
        };

        // let session_gateway: Box<dyn SessionRepository> = match auth_config.session_gateway {
        //     Some(gateway_config) => match gateway_config {
        //         GatewayConfig::SurrealGateway(params) => {
        //             Box::new(SurrealGateway::new(params).await)
        //         }
        //         GatewayConfig::MySqlGateway(url) => Box::new(MySqlGateway::new(&url).await),
        //         GatewayConfig::RedisGateway(url) => Box::new(RedisGateway::new(&url).await),
        //     },
        //     None => {
        //         return Err(Box::new(std::io::Error::new(
        //             std::io::ErrorKind::InvalidData,
        //             "Session Gateway Config Missing",
        //         )))
        //     }
        // };

        // let session_type = auth_config.session_type;

        Ok(Self {
            credentials_gateway,
            credentials_table_name,
            session_duration,
            // session_type,
        })
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
                    let credentials = Credentials::new(user_identity, raw_password);
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
        password: &str,
    ) -> Result<SessionRecordId, Box<dyn Error>> {
        if self.match_credentials(user_identity, password).await {
            let session_record_id = self.start_session(user_identity).await?;
            Ok(session_record_id)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Credentials did not match",
            )))
        }
    }

    async fn match_credentials(&self, email: &str, password: &str) -> bool {
        match self
            .credentials_gateway
            .find_credentials_by_user_identity(&email)
            .await
        {
            Ok(credentials_query) => match credentials_query {
                Some(credentials) => {
                    if credentials.match_password(password) {
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
        let claims = Claims::new(user_identity, self.session_duration);
        let token = Self::encode_token(claims)?;

        Ok(token)
        // match self.session_type {
        //     SessionType::JWT(duration) => {
        //         let claims = Claims::new(user_identity, duration);
        //         let token = Self::encode_token(claims)?;

        //         Ok(token)
        //     }
        //     SessionType::Session(duration) => {
        //         let session = Session::new(duration);
        //         self.session_gateway.store_session(&session).await?;

        //         Ok(session.id)
        //     }
        // }
    }

    pub async fn validate_session(&mut self, session_token: &str) -> Result<bool, Box<dyn Error>> {
        let valid = Self::decode_token(&session_token);

        if valid.is_ok() {
            Ok(true)
        } else {
            Ok(false)
        }
        // match self.session_type {
        //     SessionType::JWT(_) => {
        //         let valid = Self::decode_token(&session_record_id);

        //         if valid.is_ok() {
        //             Ok(true)
        //         } else {
        //             Ok(false)
        //         }
        //     }
        //     SessionType::Session(_) => {
        //         let session = self
        //             .session_gateway
        //             .get_session_by_id(session_record_id)
        //             .await?;

        //         if session.is_expired() {
        //             self.session_gateway
        //                 .delete_session(session_record_id)
        //                 .await?;
        //             Ok(false)
        //         } else {
        //             Ok(true)
        //         }
        //     }
        // }
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
