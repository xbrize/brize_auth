use crate::{entities::User, interface_adapters::user_repository::UserRepository};

// pub async fn login_user(repository: UserRepository, email: &str, password: &str) -> Option<User> {
//     match repository.find_user_by_email(&email).await {
//         Some(user) => {
//             if user.get_password() == password {
//                 println!("Login Successful");
//                 return Some(user);
//             } else {
//                 println!("Password Did Not Match");
//                 return None;
//             }
//         }
//         None => {
//             println!("Username Not Found");
//             None
//         }
//     }
// }
