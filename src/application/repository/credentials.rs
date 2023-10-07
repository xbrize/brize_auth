use crate::domain::{Credentials, CredentialsId};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[async_trait::async_trait]
pub trait CredentialsRepository {
    async fn find_credentials_by_unique_identifier(
        &self,
        unique_identifier: &str,
    ) -> Result<Credentials, Box<dyn Error>>;

    async fn find_credentials_by_id(
        &self,
        credentials_id: &str,
    ) -> Result<Credentials, Box<dyn Error>>;

    async fn insert_credentials(&self, user: &Credentials)
        -> Result<CredentialsId, Box<dyn Error>>;
}
