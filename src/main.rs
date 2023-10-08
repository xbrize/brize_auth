use brize_auth::{
    auth::{Auth, AuthConfig, GatewayConfig},
    domain::Expiry,
};

#[tokio::main]
async fn main() {
    let config = AuthConfig::new()
        .set_credentials_gateway(GatewayConfig::MySqlGateway(
            "mysql://root:my-secret-pw@localhost:3306/mysql".to_string(),
        ))
        .use_jwt_token(Expiry::Day(1));

    let auth = Auth::new(config);
}
