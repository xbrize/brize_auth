use surreal_auth::{
    application::{login_user, register_user, start_session},
    domain::User,
    infrastructure::{
        initialize_test_database, session_repository::SessionRepository,
        user_repository::UserRepository,
    },
};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let database = initialize_test_database().await;
    let user_repo = UserRepository::new(&database);
    let session_repo = SessionRepository::new(&database);

    let username = "test_name";
    let password = "test_password";
    let email = "test_email@gmail.com";

    let new_user = User::new(username, password, email);

    register_user(&user_repo, &new_user).await;
    match login_user(&user_repo, email, password).await {
        Some(record_id) => {
            dbg!(start_session(session_repo, record_id).await);
        }
        None => {
            dbg!("No session made");
        }
    };
    Ok(())
}
