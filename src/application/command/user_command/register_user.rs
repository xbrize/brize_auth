use crate::application::{UserRecordId, UserRepository};

pub async fn register_user<T: UserRepository>(
    repository: &T,
    username: &str,
    password: &str,
    email: &str,
) -> Option<UserRecordId> {
    match repository.find_user_by_email(email).await {
        Ok(user_record) => {
            println!("User {} Already Exists", user_record.user.get_email());
            return None;
        }
        Err(_) => {
            match repository.create_user(username, password, email).await {
                Ok(record_id) => {
                    println!("User Has Been Created");
                    return Some(record_id);
                }
                Err(_) => {
                    println!("Failed to create new user");
                    return None;
                }
            };
        }
    };
}
