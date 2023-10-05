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

    // let shit = sqlx::query(
    //     r#"
    //     SELECT * FROM users;
    //     "#,
    // )
    // .execute(&repo.pool)
    // .await
    // .unwrap();

    // dbg!(shit);

    let binding = uuid::Uuid::new_v4().to_string();
    let fields = vec![
        ("id", binding.as_str()),
        ("username", "jon"),
        ("password", "password"),
        ("email", "email@gmail.com"),
    ];
    let unique_fields = vec!["user_name", "password", "email"];

    repo.register(fields, unique_fields).await;
    let user = repo.find_user_by_email("email@gmail.com").await.unwrap();
    dbg!(user);
}
