use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("user not found")]
    NotFound,
    
    #[error("user already exists: {email}")]
    AlreadyExists { email: String },
    
    #[error("invalid credentials")]
    InvalidCredentials,
    
    #[error("invalid token: {reason}")]
    InvalidToken { reason: String },
    
    #[error("user's token has expired")]
    ExpiredToken,
    
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}