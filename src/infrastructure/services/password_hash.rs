use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_raw_password(raw_password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(raw_password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

pub fn verify_password(raw_password: &str, hashed_password: &str) -> Result<()> {
    let parsed_hash = PasswordHash::new(hashed_password)
        .map_err(|e| anyhow::anyhow!("Hash parsing failed: {e}"))?;

    Argon2::default()
        .verify_password(raw_password.as_bytes(), &parsed_hash)
        .map_err(|e| anyhow::anyhow!("Verification failed: {e}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let raw_password = "test_password";
        let hashed_password = hash_raw_password(raw_password);
        let verification = verify_password(raw_password, &hashed_password);
        assert!(verification.is_ok());
    }
}
