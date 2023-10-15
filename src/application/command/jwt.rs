use anyhow::{Context, Result};
use dotenvy::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::env;

use crate::domain::entity::Claims;

pub fn generate_json_web_token(claims: Claims) -> Result<String> {
    // Load env and get the secret
    dotenv().context(".env file not found")?;
    let secret = env::var("JWT_SECRET").context("JWT_SECRET key not found")?;

    // Generate token
    let header = Header::new(Algorithm::HS256);
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
        .context("Token encoding failed")?;

    Ok(token)
}

pub fn verify_json_web_token(token: &str) -> Result<()> {
    // Load env and get the secret
    dotenv().context(".env file not found")?;
    let secret = env::var("JWT_SECRET").context("JWT_SECRET key not found")?;

    // Decode token
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .context("Token decoding failed")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::domain::config::Expiry;

    use super::*;

    #[test]
    fn test_jwt_token_command() {
        let claims = Claims::new("user_identity", &Expiry::Day(1));
        let token = generate_json_web_token(claims).unwrap();
        let attempt_to_verify = verify_json_web_token(token.as_str());
        assert!(attempt_to_verify.is_ok());
    }
}
