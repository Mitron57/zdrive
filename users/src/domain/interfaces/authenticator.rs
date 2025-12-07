use uuid::Uuid;
use std::time::Duration;

pub trait TokenGenerator {
    fn generate_token(&self, user_id: Uuid, secret: &str, ttl: Duration) -> Result<String, anyhow::Error>;
    fn validate_token(&self, token: &str, secret: &str) -> Result<Uuid, anyhow::Error>;
}

pub trait PasswordHasher {
    fn hash(&self, password: &str) -> Result<String, anyhow::Error>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool, anyhow::Error>;
}