use surreal_auth::{
    entities::User,
    interface_adapters::{initialize_test_database, user_repository::UserRepository},
    use_cases::register_user,
};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let database = initialize_test_database().await;
    let user_repo = UserRepository::new(database);
    let new_user = User::new("test_name", "test_password", "test_email@gmail.com");

    register_user(&user_repo, &new_user).await;
    dbg!(user_repo.find_user_by_email("test_email@gmail.com").await);
    Ok(())
}
