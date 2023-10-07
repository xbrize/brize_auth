use brize_auth::{
    application::{login_user, register_user, CredentialsRepository, SessionRepository},
    domain::{Credentials, Expiry, Session},
    infrastructure::{MySqlGateway, RedisGateway, SurrealGateway},
};
use redis::RedisResult;
use sqlx::*;

#[tokio::main]
async fn main() {
    let url = "mysql://root:my-secret-pw@localhost:3306/mysql";
    let repo = MySqlGateway::new(url).await;
    // repo.create_credentials_table().await;

    let password = "test-pass-word";
    let email = "test_email@gmail.com";

    let creds_id = register_user(&repo, email, password).await;
    dbg!(creds_id);

    let user = login_user(&repo, email, password).await;
    dbg!(user);
}
