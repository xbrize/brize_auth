use crate::domain::entity::Credentials;
use anyhow::Result;

#[trait_variant::make(HttpService: Send)]
pub trait CredentialsRepository: Send + Sync {
    async fn find_credentials_by_user_name(&self, user_name: &str) -> Result<Credentials>;

    async fn find_credentials_by_id(&self, id: &str) -> Result<Credentials>;

    async fn insert_credentials(&self, credentials: &Credentials) -> Result<()>;

    async fn update_user_name(&self, user_name: &str, new_user_name: &str) -> Result<()>;

    async fn update_user_password(&self, user_name: &str, new_password: &str) -> Result<()>;

    async fn delete_credentials_by_user_name(&self, user_name: &str) -> Result<()>;

    async fn delete_credentials_by_id(&self, id: &str) -> Result<()>;
}
