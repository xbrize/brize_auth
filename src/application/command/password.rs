use anyhow::{Context, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_raw_password(raw_password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(raw_password.as_bytes(), &salt)
        .context("Failed to hash password")?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(raw_password: &str, hashed_password: &str) -> Result<()> {
    let parsed_hash = PasswordHash::new(hashed_password).context("Failed to parse PHC string")?;

    Argon2::default()
        .verify_password(raw_password.as_bytes(), &parsed_hash)
        .context("Password did not match")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let raw_password = "test_password";
        let hashed_password = hash_raw_password(raw_password).unwrap();
        let verification = verify_password(raw_password, &hashed_password);
        assert!(verification.is_ok());
    }
}
