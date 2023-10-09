use crate::domain::Credentials;
use std::error::Error;

#[async_trait::async_trait]
pub trait CredentialsRepository {
    async fn find_credentials_by_user_identity(
        &self,
        user_identity: &str,
    ) -> Result<Option<Credentials>, Box<dyn Error>>;

    async fn find_credentials_by_id(&self, id: &str) -> Result<Credentials, Box<dyn Error>>;

    async fn insert_credentials(&self, credentials: &Credentials) -> Result<(), Box<dyn Error>>;
}
