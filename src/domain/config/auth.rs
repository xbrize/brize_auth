use super::Expiry;

/// This config is used to set the desired session type.
/// Supports json web tokens or classic table sessions. Can also be disabled with None.
pub enum SessionType {
    JWT(Expiry),
    Session(Expiry),
    None,
}

/// These are the database params needed for connecting to most databases.
pub struct DatabaseConfig {
    pub db_name: String,
    pub password: String,
    pub user_name: String,
    pub host: String,
    pub port: String,
    pub namespace: Option<String>,
}

/// Brize auth considers a gateway to be some sort of database technology.
/// Supports MySql, SurrealDB, and Redis (sessions only).
pub enum GatewayType {
    MySql(DatabaseConfig),
    Surreal(DatabaseConfig),
    Redis(DatabaseConfig),
}

/// The main auth configuration. Once everything has been set this is passed to the main Auth.
pub struct AuthConfig {
    pub credentials_gateway: Option<GatewayType>,
    pub session_gateway: Option<GatewayType>,
    pub session_type: SessionType,
}

impl AuthConfig {
    /// Start with the blank slate and then begin building.
    pub fn new() -> Self {
        Self {
            credentials_gateway: None,
            session_gateway: None,
            session_type: SessionType::None,
        }
    }

    /// Used to set the type of gateway desired to store the credentials in.
    pub fn set_credentials_gateway(mut self, config: GatewayType) -> Self {
        self.credentials_gateway = Some(config);
        self
    }

    /// Used to set the type of gateway desired to store the sessions in.
    /// This is optional, if not set here, the credentials gateway will be used.
    /// If session type is set to None, this will not work either.
    pub fn set_session_gateway(mut self, config: GatewayType) -> Self {
        self.session_gateway = Some(config);
        self
    }

    /// Used to set the desired session type.
    pub fn set_session_type(mut self, session_type: SessionType) -> Self {
        self.session_type = session_type;
        self
    }
}
