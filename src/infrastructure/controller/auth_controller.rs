use crate::{
    application::{login_user, register_user, start_session},
    domain::{Expiry, SessionRecordId},
    infrastructure::SurrealGateway,
};

pub async fn handle_user_login(email: &str, password: &str) -> Option<SessionRecordId> {
    let mut repository = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;

    match login_user(&repository, email, password).await {
        Some(_) => {
            let session_id = start_session(&mut repository, Expiry::Day(1))
                .await
                .unwrap();
            Some(session_id)
        }
        None => None,
    }
}

pub async fn handle_user_registration(
    username: &str,
    password: &str,
    email: &str,
) -> Option<SessionRecordId> {
    let mut repository = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;

    match register_user(&repository, username, password, email).await {
        Some(_) => {
            let session_id = start_session(&mut repository, Expiry::Day(1))
                .await
                .unwrap();
            Some(session_id)
        }
        None => None,
    }
}

// pub async fn handle_user_validation(session_record_id: SessionRecordId) -> SessionState {
//     let mut repository = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;

//     match validate_session(&mut repository, &session_record_id).await {
//         true => SessionState::Valid,
//         false => SessionState::Invalid,
//     }
// }
