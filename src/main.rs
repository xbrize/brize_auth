use brize_auth::{
    application::{login_user, register_user, CredentialsRepository, SessionRepository},
    domain::{Credentials, Expiry, Session},
    infrastructure::{MySqlGateway, RedisGateway, SurrealGateway},
};
use redis::RedisResult;
use sqlx::*;

#[tokio::main]
async fn main() {
    // ---------- SQL
    // let url = "mysql://root:my-secret-pw@localhost:3306/mysql";
    // let repo = MySqlGateway::new(url).await;
    // // repo.create_credentials_table().await;

    // let email = "test_email@email.com";
    // let password = "test-pass-word";

    // register_user(&repo, email, password).await;
    // login_user(&repo, email, password).await;

    // ---------- Surreal
    let repo = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;

    let email = "t@email.com";
    let password = "test-pass-word";

    register_user(&repo, email, password).await;
    login_user(&repo, email, password).await;
}
