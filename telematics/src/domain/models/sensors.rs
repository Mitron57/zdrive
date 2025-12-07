use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub vin: String,
    pub license_plate: String,
    pub fuel_level: f64, // Процент от 0.0 до 100.0
    pub location: Location,
    pub door_status: DoorStatus,
    pub speed: f64, // км/ч
    pub temperature: f64, // Цельсий
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DoorStatus {
    Open,
    Closed,
    Locked,
}

impl DoorStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DoorStatus::Open => "open",
            DoorStatus::Closed => "closed",
            DoorStatus::Locked => "locked",
        }
    }
}

#[derive(Deserialize)]
pub struct SensorDataMessage {
    pub vin: String,
    pub license_plate: String,
    pub fuel_level: f64,
    pub location: Location,
    pub door_status: String,
    pub speed: f64,
    pub temperature: f64,
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<SensorDataMessage> for SensorData {
    type Error = String;

    fn try_from(msg: SensorDataMessage) -> Result<Self, Self::Error> {
        let door_status = match msg.door_status.to_lowercase().as_str() {
            "open" => DoorStatus::Open,
            "closed" => DoorStatus::Closed,
            "locked" => DoorStatus::Locked,
            _ => return Err(format!("Invalid door status: {}", msg.door_status)),
        };

        Ok(SensorData {
            vin: msg.vin,
            license_plate: msg.license_plate,
            fuel_level: msg.fuel_level,
            location: msg.location,
            door_status,
            speed: msg.speed,
            temperature: msg.temperature,
            timestamp: msg.timestamp,
        })
    }
}

