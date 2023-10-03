#[async_trait::async_trait]
pub trait Authenticate {
    async fn register(&self, columns: Vec<(&str, &str)>) -> bool;
}
