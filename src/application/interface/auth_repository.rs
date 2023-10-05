#[async_trait::async_trait]
pub trait Authenticate {
    async fn register(&self, fields: Vec<(&str, &str)>, unique_fields: Vec<&str>) -> bool;
}
