use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub type CredentialsId = String;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Credentials {
    pub id: CredentialsId,
    pub user_identity: String,

    // TODO need to enable this but still pass tests
    // #[serde(skip_deserializing)]
    pub hashed_password: String,
}

impl Credentials {
    pub fn new(user_identity: &str, hashed_password: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_identity: user_identity.to_string(),
            hashed_password: hashed_password.to_string(),
        }
    }
}
