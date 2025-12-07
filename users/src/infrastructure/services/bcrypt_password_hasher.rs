use bcrypt::{hash, verify, DEFAULT_COST};
use crate::domain::interfaces::PasswordHasher;

#[derive(Clone)]
pub struct BcryptPasswordHasher;

impl BcryptPasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasher for BcryptPasswordHasher {
    fn hash(&self, password: &str) -> Result<String, anyhow::Error> {
        let hashed = hash(password, DEFAULT_COST)?;
        Ok(hashed)
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool, anyhow::Error> {
        let is_valid = verify(password, hash)?;
        Ok(is_valid)
    }
}

