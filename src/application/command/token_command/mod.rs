use crate::domain::Claims;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

static SECRET: &'static str = "super_secret_key";

pub fn encode_token(claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret(SECRET.as_ref()))
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &validation,
    )
    .map(|c| c.claims)
}

#[cfg(test)]
mod test {
    use crate::domain::ClaimsExpiry;

    use super::*;

    #[test]
    fn test_token_validation() {
        let token = encode_token(Claims::new("jon@gmail.com", ClaimsExpiry::Day(1))).unwrap();

        let claims = decode_token(&token).unwrap();
        assert_eq!(claims.sub, "jon@gmail.com")
    }
}
