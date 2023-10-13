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

    async fn update_user_identity(
        &self,
        current_identity: &str,
        new_identity: &str,
    ) -> Result<(), Box<dyn Error>>;

    async fn update_user_password(
        &self,
        user_identity: &str,
        new_raw_password: &str,
    ) -> Result<(), Box<dyn Error>>;
}
