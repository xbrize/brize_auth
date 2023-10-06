#[async_trait::async_trait]
pub trait Authenticate {
    async fn register(&self, fields: Vec<(&str, &str, bool)>) -> bool;
    async fn check_for_unique_fields(&self, fields: &Vec<(&str, &str, bool)>) -> bool;
}
