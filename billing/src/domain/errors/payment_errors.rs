use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("payment not found")]
    PaymentNotFound,
    
    #[error("invalid payment amount: {amount}")]
    InvalidAmount { amount: f64 },
    
    #[error("payment already processed")]
    PaymentAlreadyProcessed,
    
    #[error("invalid payment status transition: from {from} to {to}")]
    InvalidStatusTransition { from: String, to: String },
    
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

