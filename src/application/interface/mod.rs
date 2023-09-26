mod user_repository;
use surrealdb::sql::Thing;
pub use user_repository::*;

mod session_repository;
pub use session_repository::*;

pub type SessionRecordId = Thing;
pub type UserRecordId = Thing;
