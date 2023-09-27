use crate::{
    application::{login_user, register_user, start_session, SessionRecordId},
    infrastructure::DataStore,
};

pub async fn handle_user_login(email: &str, password: &str) -> Option<SessionRecordId> {
    let repository = DataStore::new("127.0.0.1:8000", "test", "test").await;

    match login_user(&repository, email, password).await {
        Some(user_record_id) => start_session(&repository, &user_record_id).await,
        None => None,
    }
}

pub async fn handle_user_registration(
    username: &str,
    password: &str,
    email: &str,
) -> Option<SessionRecordId> {
    let repository = DataStore::new("127.0.0.1:8000", "test", "test").await;

    match register_user(&repository, username, password, email).await {
        Some(user_record_id) => start_session(&repository, &user_record_id).await,
        None => None,
    }
}
