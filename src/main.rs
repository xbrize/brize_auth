use brize_auth::{
    application::{Authenticate, CredentialsRepository, SessionRepository},
    domain::{Credentials, Expiry, Session},
    infrastructure::{MySqlGateway, RedisGateway, SurrealGateway},
};
use redis::RedisResult;
use sqlx::*;

#[tokio::main]
async fn main() {
    let url = "mysql://root:my-secret-pw@localhost:3306/mysql";
    let repo = MySqlGateway::new(url).await;
    repo.create_credentials_table().await;

    let password = "test-pass-word";
    let email = "test@email.com";

    // Create new user
    let user = Credentials::new(email, password);
    repo.insert_credentials(&user).await.unwrap();

    // Test getting user
    let user_record = repo.find_credentials_by_user_identity(email).await.unwrap();
    dbg!(&user_record);
}
