#[derive(Debug)]
pub enum RepositoryError {
    QueryFail,
    NotFound,
}

pub type RepoResult<T> = std::result::Result<T, RepositoryError>;
