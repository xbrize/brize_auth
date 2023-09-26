use surreal_auth::{
    application::{login_user, register_user, start_session},
    domain::User,
    infrastructure::{database::DataStore, initialize_test_database},
};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let database = initialize_test_database().await;
    let repo = DataStore::new(&database);

    let username = "test_name";
    let password = "test_password";
    let email = "test_email@gmail.com";

    let new_user = User::new(username, password, email);

    register_user(&repo, &new_user).await;
    match login_user(&repo, email, password).await {
        Some(record_id) => {
            dbg!(start_session(&repo, record_id).await);
        }
        None => {
            dbg!("No session made");
        }
    };
    Ok(())
}
