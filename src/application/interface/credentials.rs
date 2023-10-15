use crate::domain::entity::Credentials;
use anyhow::Result;

#[async_trait::async_trait]
pub trait CredentialsRepository {
    async fn find_credentials_by_user_identity(&self, user_identity: &str) -> Result<Credentials>;

    async fn find_credentials_by_id(&self, id: &str) -> Result<Credentials>;

    async fn insert_credentials(&self, credentials: &Credentials) -> Result<()>;

    async fn update_user_identity(&self, current_identity: &str, new_identity: &str) -> Result<()>;

    async fn update_user_password(&self, user_identity: &str, new_password: &str) -> Result<()>;

    async fn delete_credentials_by_user_identity(&self, user_identity: &str) -> Result<()>;
}
