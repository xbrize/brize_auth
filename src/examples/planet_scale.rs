use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env;

use crate::{Auth, AuthConfig, DatabaseConfig, Expiry, GatewayType, SessionType};

fn get_env(secret_name: &str) -> String {
    let err = format!("Secret {} not found", secret_name);
    env::var(secret_name).context(err).unwrap()
}

pub async fn planet_scale_example() -> Result<()> {
    dotenv().context(".env file not found")?;

    // ** Set database params
    let db_config = DatabaseConfig {
        host: get_env("PSCALE_DB_HOST"),
        port: get_env("PSCALE_DB_PORT"),
        user_name: get_env("PSCALE_DB_USERNAME"),
        password: get_env("PSCALE_DB_PASSWORD"),
        db_name: get_env("PSCALE_DB_NAME"),
        namespace: None,
    };

    // ** Start Auth config
    let config = AuthConfig::new()
        .set_credentials_gateway(GatewayType::MySql(db_config))
        .set_session_type(SessionType::Session(Expiry::Month(1)));

    // ** Init Auth
    let mut auth = Auth::new(config).await?;

    // ** Get user credentials from a request
    let user_identity = "test@gmail.com";
    let raw_password = "plokij1234!";

    // ** Register user
    let credentials_id = auth.register(user_identity, raw_password).await?;
    dbg!(credentials_id);

    // ** Log user in and get a session token back
    let session_token: String = auth.login(user_identity, raw_password).await?;
    dbg!(&session_token);

    // ** Validate token upon request and get user identity
    let validated_user: String = auth.validate_session(session_token.as_str()).await?;
    dbg!(validated_user);

    // ** Logout
    let logout_status: Result<()> = auth.logout(&session_token).await;
    dbg!(logout_status.unwrap());

    Ok(())
}
