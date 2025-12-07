use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::errors::DispatcherError;
use serde::{Serialize, Deserialize};

// Интерфейсы для HTTP клиентов к микросервисам

#[async_trait]
pub trait UsersServiceClient {
    async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, DispatcherError>;
    async fn authenticate(&self, request: AuthRequest) -> Result<AuthResponse, DispatcherError>;
    async fn get_user(&self, user_id: Uuid) -> Result<UserInfo, DispatcherError>;
    async fn get_all_users(&self) -> Result<Vec<UserInfo>, DispatcherError>;
}

#[async_trait]
pub trait CarsServiceClient {
    async fn get_car(&self, car_id: Uuid) -> Result<CarInfo, DispatcherError>;
    async fn get_all_cars(&self) -> Result<Vec<CarInfo>, DispatcherError>;
    async fn get_tariff(&self, tariff_id: Uuid) -> Result<TariffInfo, DispatcherError>;
}

#[derive(Serialize, Deserialize)]
pub struct TariffInfo {
    pub id: Uuid,
    pub price_per_minute: f64,
    pub minimal_rating: f64,
    pub minimal_experience: u32,
}

#[async_trait]
pub trait TripsServiceClient {
    async fn start_trip(&self, user_id: Uuid, car_id: Uuid) -> Result<Uuid, DispatcherError>;
    async fn activate_trip(&self, trip_id: Uuid) -> Result<(), DispatcherError>;
    async fn end_trip(&self, trip_id: Uuid) -> Result<(), DispatcherError>;
    async fn cancel_trip(&self, trip_id: Uuid) -> Result<(), DispatcherError>;
    async fn get_trip(&self, trip_id: Uuid) -> Result<TripInfo, DispatcherError>;
    async fn get_user_active_trip(&self, user_id: Uuid) -> Result<Option<TripInfo>, DispatcherError>;
    async fn get_all_trips(&self) -> Result<Vec<TripInfo>, DispatcherError>;
}

#[async_trait]
pub trait TelematicsServiceClient {
    async fn send_command(&self, car_id: Uuid, command_type: String) -> Result<Uuid, DispatcherError>;
    async fn get_sensor_data_by_car_id(&self, car_id: Uuid) -> Result<Option<SensorDataInfo>, DispatcherError>;
    async fn get_all_sensor_data(&self) -> Result<Vec<SensorDataInfo>, DispatcherError>;
}

#[async_trait]
pub trait BillingServiceClient {
    async fn create_payment(&self, trip_id: Uuid, user_id: Uuid, amount: f64) -> Result<PaymentInfo, DispatcherError>;
    async fn get_payment(&self, payment_id: Uuid) -> Result<PaymentInfo, DispatcherError>;
}

// Модели данных для взаимодействия с сервисами

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub license_id: String,
    pub driving_experience: u32,
    pub rating: f64,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterResponse {
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub license_id: String,
    pub driving_experience: u32,
    pub rating: f64,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct CarInfo {
    pub id: Uuid,
    pub model: String,
    pub license_plate: String,
    pub state: String,
    pub tariff_id: Uuid,
    pub base_price: f64,
    pub price_per_minute: Option<f64>, // Опционально, чтобы не ломать существующий код
}

#[derive(Serialize, Deserialize)]
pub struct TripInfo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub car_id: Uuid,
    pub status: String,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct SensorDataInfo {
    pub vin: String,
    pub license_plate: String,
    pub fuel_level: f64,
    pub location: LocationInfo,
    pub door_status: String,
    pub speed: f64,
    pub temperature: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct LocationInfo {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize)]
pub struct PaymentInfo {
    pub id: Uuid,
    pub trip_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub status: String,
    pub qr_code_url: Option<String>,
}

