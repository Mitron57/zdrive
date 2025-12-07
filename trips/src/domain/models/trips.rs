use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trip {
    pub id: Uuid,
    pub user_id: Uuid,
    pub car_id: Uuid,
    pub status: TripStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TripStatus {
    Reserved,   // Зарезервирована, но еще не начата
    Active,     // Активная поездка
    Completed, // Завершена
    Cancelled,  // Отменена
}

impl TripStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TripStatus::Reserved => "reserved",
            TripStatus::Active => "active",
            TripStatus::Completed => "completed",
            TripStatus::Cancelled => "cancelled",
        }
    }
}

impl std::str::FromStr for TripStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "reserved" => Ok(TripStatus::Reserved),
            "active" => Ok(TripStatus::Active),
            "completed" => Ok(TripStatus::Completed),
            "cancelled" => Ok(TripStatus::Cancelled),
            _ => Err(format!("Invalid trip status: {}", s)),
        }
    }
}

#[derive(Deserialize)]
pub struct StartTripRequest {
    pub user_id: Uuid,
    pub car_id: Uuid,
}

#[derive(Deserialize)]
pub struct EndTripRequest {
    // Можно добавить дополнительные поля, например, финальное местоположение
}

#[derive(Deserialize)]
pub struct CancelTripRequest {
    // Можно добавить причину отмены
}

