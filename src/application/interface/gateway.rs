use crate::DatabaseConfig;

#[async_trait::async_trait]
pub trait GatewayFactory<T> {
    async fn new(config: &DatabaseConfig) -> T;
}
