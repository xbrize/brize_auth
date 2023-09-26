use crate::infrastructure::session_store::SessionRepository;
use surrealdb::opt::RecordId;

pub async fn start_session(
    session_repository: SessionRepository<'_>,
    user_record_link: RecordId,
) -> Option<RecordId> {
    session_repository.create_session(user_record_link).await
}
