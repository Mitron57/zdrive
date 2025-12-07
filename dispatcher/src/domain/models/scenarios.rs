use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::interfaces::{CarInfo, LocationInfo};

// Request/Response модели для сценариев

#[derive(Deserialize)]
pub struct StartTripRequest {
    pub user_id: Uuid,
    pub car_id: Uuid,
}

#[derive(Serialize)]
pub struct StartTripResponse {
    pub trip_id: Uuid,
}

#[derive(Deserialize)]
pub struct EndTripRequest {
    pub trip_id: Uuid,
}

#[derive(Serialize)]
pub struct EndTripResponse {
    pub trip_id: Uuid,
    pub payment_id: Uuid,
    pub qr_code_url: String,
}

#[derive(Deserialize)]
pub struct CancelTripRequest {
    pub trip_id: Uuid,
}

#[derive(Serialize)]
pub struct CancelTripResponse {
    pub trip_id: Uuid,
    pub message: String,
}

#[derive(Serialize)]
pub struct CarDataResponse {
    pub car: CarInfo,
    pub price_per_minute: f64,
    pub telematics: Option<TelematicsInfo>,
}

#[derive(Serialize)]
pub struct TelematicsInfo {
    pub fuel_level: f64,
    pub location: LocationInfo,
    pub door_status: String,
    pub speed: f64,
    pub temperature: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

