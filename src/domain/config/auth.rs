use super::Expiry;

pub enum SessionType {
    JWT(Expiry),
    Session(Expiry),
    None,
}

pub struct DatabaseConfig {
    pub db_name: String,
    pub password: String,
    pub user_name: String,
    pub host: String,
}

pub enum GatewayType {
    MySql(DatabaseConfig),
    Surreal(DatabaseConfig),
    Redis(DatabaseConfig),
}

pub struct AuthConfig {
    pub credentials_gateway: Option<GatewayType>,
    pub session_gateway: Option<GatewayType>,
    pub session_type: SessionType,
}

impl AuthConfig {
    pub fn new() -> Self {
        Self {
            credentials_gateway: None,
            session_gateway: None,
            session_type: SessionType::None,
        }
    }

    pub fn set_credentials_gateway(mut self, config: GatewayType) -> Self {
        self.credentials_gateway = Some(config);
        self
    }

    pub fn set_session_gateway(mut self, config: GatewayType) -> Self {
        self.session_gateway = Some(config);
        self
    }

    pub fn set_session_type(mut self, session_type: SessionType) -> Self {
        self.session_type = session_type;
        self
    }
}
