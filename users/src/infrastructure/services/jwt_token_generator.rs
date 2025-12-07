use uuid::Uuid;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use crate::domain::interfaces::TokenGenerator;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // user_id
    exp: usize,
    iat: usize,
}

#[derive(Clone)]
pub struct JwtTokenGenerator;

impl JwtTokenGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl TokenGenerator for JwtTokenGenerator {
    fn generate_token(&self, user_id: Uuid, secret: &str, ttl: Duration) -> Result<String, anyhow::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as usize;
        
        let exp = now + ttl.as_secs() as usize;
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp,
            iat: now,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;

        Ok(token)
    }

    fn validate_token(&self, token: &str, secret: &str) -> Result<Uuid, anyhow::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )?;

        let user_id = Uuid::parse_str(&token_data.claims.sub)?;
        Ok(user_id)
    }
}

