use crate::domain::{GatewayType, SessionType};

pub struct AuthConfig {
    pub credentials_gateway: Option<GatewayType>,
    pub credentials_table_name: Option<String>,
    pub session_gateway: Option<GatewayType>,
    pub session_table_name: Option<String>,
    pub session_type: SessionType,
}

impl AuthConfig {
    pub fn new() -> Self {
        Self {
            credentials_gateway: None,
            credentials_table_name: None,
            session_gateway: None,
            session_table_name: None,
            session_type: SessionType::None,
        }
    }

    pub fn set_credentials_gateway(mut self, config: GatewayType) -> Self {
        self.credentials_gateway = Some(config);
        self
    }

    pub fn set_credentials_table_name(mut self, name: &str) -> Self {
        self.credentials_table_name = Some(name.to_string());
        self
    }

    pub fn set_session_gateway(mut self, config: GatewayType) -> Self {
        self.session_gateway = Some(config);
        self
    }

    pub fn set_session_table_name(mut self, name: &str) -> Self {
        self.session_table_name = Some(name.to_string());
        self
    }

    pub fn set_session_type(mut self, session_type: SessionType) -> Self {
        self.session_type = session_type;
        self
    }
}
