use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Car {
    pub id: Uuid,
    pub model: String,
    pub license_plate: String,
    pub iot_serial_number: String,
    pub state: CarState,
    pub tariff_id: Uuid,
    pub base_price: f64, // Базовая стоимость для машины (добавляется к тарифу)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CarState {
    Available,
    InUse,
    Maintenance,
    Reserved,
}

impl CarState {
    pub fn as_str(&self) -> &'static str {
        match self {
            CarState::Available => "available",
            CarState::InUse => "in_use",
            CarState::Maintenance => "maintenance",
            CarState::Reserved => "reserved",
        }
    }
}

impl std::str::FromStr for CarState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "available" => Ok(CarState::Available),
            "in_use" => Ok(CarState::InUse),
            "maintenance" => Ok(CarState::Maintenance),
            "reserved" => Ok(CarState::Reserved),
            _ => Err(format!("Invalid car state: {}", s)),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateCarRequest {
    pub model: String,
    pub license_plate: String,
    pub iot_serial_number: String,
    pub state: CarState,
    pub tariff_id: Uuid,
    pub base_price: f64,
}

#[derive(Deserialize)]
pub struct UpdateCarRequest {
    pub model: Option<String>,
    pub license_plate: Option<String>,
    pub iot_serial_number: Option<String>,
    pub state: Option<CarState>,
    pub tariff_id: Option<Uuid>,
    pub base_price: Option<f64>,
}

