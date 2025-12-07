use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tariff {
    pub id: Uuid,
    pub price_per_minute: f64,
    pub minimal_rating: f64,
    pub minimal_experience: u32, // в годах
}

#[derive(Deserialize)]
pub struct CreateTariffRequest {
    pub price_per_minute: f64,
    pub minimal_rating: f64,
    pub minimal_experience: u32,
}

#[derive(Deserialize)]
pub struct UpdateTariffRequest {
    pub price_per_minute: Option<f64>,
    pub minimal_rating: Option<f64>,
    pub minimal_experience: Option<u32>,
}

