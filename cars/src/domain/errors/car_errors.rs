use thiserror::Error;

#[derive(Error, Debug)]
pub enum CarError {
    #[error("car not found")]
    CarNotFound,
    
    #[error("tariff not found")]
    TariffNotFound,
    
    #[error("car already exists: {license_plate}")]
    CarAlreadyExists { license_plate: String },
    
    #[error("iot device already registered: {serial_number}")]
    IotDeviceAlreadyRegistered { serial_number: String },
    
    #[error("invalid state transition")]
    InvalidStateTransition,
    
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

