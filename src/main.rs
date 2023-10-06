use brize_auth::{
    application::{Authenticate, SessionRepository, UserRepository},
    domain::{Expiry, Session},
    infrastructure::{MySqlGateway, RedisGateway, SurrealGateway},
};
use redis::RedisResult;
use sqlx::*;

#[tokio::main]
async fn main() {
    let url = "mysql://root:my-secret-pw@localhost:3306/mysql";
    let repo = MySqlGateway::new(url).await;
    // repo.create_user_table().await;

    let binding = uuid::Uuid::new_v4().to_string();
    let fields = vec![
        ("id", binding.as_str(), true),
        ("username", "jon", true),
        ("password", "password", false),
        ("email", "email@gmail.com", true),
    ];

    repo.register(fields).await;
    let user = repo.find_user_by_email("email@gmail.com").await.unwrap();
    dbg!(user);
}
