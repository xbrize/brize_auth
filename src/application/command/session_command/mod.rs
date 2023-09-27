mod start_session;
pub use start_session::*;

mod validate_session;
pub use validate_session::*;

mod destroy_session;
pub use destroy_session::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{application::UserRecordId, infrastructure::DataStore};

    #[tokio::test]
    async fn test_session_commands() {
        let session_repo = DataStore::new("127.0.0.1:8000", "test", "test").await;
        let email = "test@email.com";

        // Test starting session
        let session_id = start_session(&session_repo, &UserRecordId::from(("user", email))).await;
        assert!(session_id.is_some());

        // Test validating session
        let session_id = session_id.unwrap();
        let is_valid = validate_session(&session_repo, &session_id).await;
        assert_eq!(is_valid, true);

        // Test destroying session
        let destroyed = destroy_session(&session_repo, &session_id).await;
        assert!(destroyed.is_ok())
    }
}
