use thiserror::Error;

#[derive(Error, Debug)]
pub enum TripError {
    #[error("trip not found")]
    TripNotFound,
    
    #[error("car not found")]
    CarNotFound,
    
    #[error("user not found")]
    UserNotFound,
    
    #[error("car is already in use")]
    CarAlreadyInUse,
    
    #[error("user already has an active trip")]
    UserHasActiveTrip,
    
    #[error("invalid trip status transition: from {from} to {to}")]
    InvalidStatusTransition { from: String, to: String },
    
    #[error("trip is not in a valid state for this operation")]
    InvalidTripState,
    
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

