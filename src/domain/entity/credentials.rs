use serde::{Deserialize, Serialize};

pub type CredentialsId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub credentials_id: CredentialsId,
    pub user_name: String,
    pub hashed_password: String,
}

impl Credentials {
    pub fn new(user_name: &str, hashed_password: &str) -> Self {
        Self {
            credentials_id: uuid::Uuid::new_v4().to_string(),
            user_name: user_name.to_string(),
            hashed_password: hashed_password.to_string(),
        }
    }
}
