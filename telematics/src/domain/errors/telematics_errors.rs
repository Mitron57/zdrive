use thiserror::Error;

#[derive(Error, Debug)]
pub enum TelematicsError {
    #[error("car not found")]
    CarNotFound,
    
    #[error("invalid command type: {command_type}")]
    InvalidCommandType { command_type: String },
    
    #[error("failed to publish message to RabbitMQ: {0}")]
    RabbitMQPublishError(String),
    
    #[error("failed to consume message from RabbitMQ: {0}")]
    RabbitMQConsumeError(String),
    
    #[error("redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
    
    #[error("invalid data: {0}")]
    InvalidData(String),
}

