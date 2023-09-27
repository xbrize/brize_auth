use surreal_auth::{
    application::SessionRepository,
    infrastructure::{handle_user_registration, DataStore},
};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let repo = DataStore::new("127.0.0.1:8000", "test", "test").await;

    let username = "test_name";
    let password = "test_password";
    let email = "test_email@gmail.com";

    let session_id = handle_user_registration(username, password, email)
        .await
        .unwrap();
    dbg!(repo.get_session(&session_id).await.unwrap());
    Ok(())
}
