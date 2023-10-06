use std::error::Error;

#[async_trait::async_trait]
pub trait Authenticate {
    async fn register(&self, fields: Vec<(&str, &str, bool)>) -> Result<bool, Box<dyn Error>>;
    async fn check_for_unique_fields(
        &self,
        fields: &Vec<(&str, &str, bool)>,
    ) -> Result<bool, Box<dyn Error>>;
}
