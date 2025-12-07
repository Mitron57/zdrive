use thiserror::Error;

#[derive(Error, Debug)]
pub enum DispatcherError {
    #[error("service unavailable: {service}")]
    ServiceUnavailable { service: String },
    
    #[error("service error: {service} - {message}")]
    ServiceError { service: String, message: String },
    
    #[error("invalid request: {message}")]
    InvalidRequest { message: String },
    
    #[error("unauthorized")]
    Unauthorized,
    
    #[error("not found: {resource}")]
    NotFound { resource: String },
    
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

