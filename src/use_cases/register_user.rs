use crate::{entities::User, interface_adapters::user_repository::UserRepository};

pub async fn register_user(repository: UserRepository, new_user: &User) -> bool {
    match repository.find_user_by_email(&new_user.get_email()).await {
        Some(user) => {
            println!("User {} Already Exists", user.get_email());
            return false;
        }
        None => {
            match repository.create_user(new_user).await {
                Ok(_) => {
                    println!("User {} Has Been Created", new_user.get_email());
                    return true;
                }
                Err(e) => {
                    println!("Failed to create new user:\n {}", e);
                    return false;
                }
            };
        }
    };
}
