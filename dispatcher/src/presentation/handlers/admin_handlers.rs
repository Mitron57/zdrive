use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, error};
use crate::presentation::app_state::AppState;
use crate::domain::errors::DispatcherError;

#[derive(Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub license_id: String,
    pub driving_experience: u32,
    pub rating: f64,
    pub email: String,
}

impl From<crate::domain::interfaces::UserInfo> for UserInfo {
    fn from(user: crate::domain::interfaces::UserInfo) -> Self {
        Self {
            id: user.id,
            license_id: user.license_id,
            driving_experience: user.driving_experience,
            rating: user.rating,
            email: user.email,
        }
    }
}

#[derive(Serialize)]
pub struct CarInfo {
    pub id: Uuid,
    pub model: String,
    pub license_plate: String,
    pub state: String,
    pub tariff_id: Uuid,
    pub base_price: f64,
}

impl From<crate::domain::interfaces::CarInfo> for CarInfo {
    fn from(car: crate::domain::interfaces::CarInfo) -> Self {
        Self {
            id: car.id,
            model: car.model,
            license_plate: car.license_plate,
            state: car.state,
            tariff_id: car.tariff_id,
            base_price: car.base_price,
        }
    }
}

#[derive(Serialize)]
pub struct TripInfo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub car_id: Uuid,
    pub status: String,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<crate::domain::interfaces::TripInfo> for TripInfo {
    fn from(trip: crate::domain::interfaces::TripInfo) -> Self {
        Self {
            id: trip.id,
            user_id: trip.user_id,
            car_id: trip.car_id,
            status: trip.status,
            started_at: trip.started_at,
            ended_at: trip.ended_at,
            created_at: trip.created_at,
        }
    }
}

#[derive(Deserialize)]
pub struct SendCommandRequest {
    pub car_id: Uuid,
    pub command_type: String,
}

#[derive(Serialize)]
pub struct SendCommandResponse {
    pub command_id: Uuid,
}

pub async fn get_all_users_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
) -> Result<Json<Vec<UserInfo>>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Getting all users (admin)");
    match state.users_client.get_all_users().await {
        Ok(users) => {
            info!("Users retrieved successfully: {} users", users.len());
            Ok(Json(users.into_iter().map(|u| u.into()).collect()))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error getting all users: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_user_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserInfo>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Getting user: {} (admin)", user_id);
    match state.users_client.get_user(user_id).await {
        Ok(user) => {
            info!("User retrieved successfully: {}", user_id);
            Ok(Json(user.into()))
        }
        Err(DispatcherError::NotFound { resource }) => {
            error!("User not found: {}", resource);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": format!("{} not found", resource)})),
            ))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error getting user: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_all_cars_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
) -> Result<Json<Vec<CarInfo>>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Getting all cars (admin)");
    match state.cars_client.get_all_cars().await {
        Ok(cars) => {
            info!("Cars retrieved successfully: {} cars", cars.len());
            Ok(Json(cars.into_iter().map(|c| c.into()).collect()))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error getting all cars: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_car_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    Path(car_id): Path<Uuid>,
) -> Result<Json<CarInfo>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Getting car: {} (admin)", car_id);
    match state.cars_client.get_car(car_id).await {
        Ok(car) => {
            info!("Car retrieved successfully: {}", car_id);
            Ok(Json(car.into()))
        }
        Err(DispatcherError::NotFound { resource }) => {
            error!("Car not found: {}", resource);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": format!("{} not found", resource)})),
            ))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error getting car: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_all_trips_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
) -> Result<Json<Vec<TripInfo>>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Getting all trips (admin)");
    match state.trips_client.get_all_trips().await {
        Ok(trips) => {
            info!("Trips retrieved successfully: {} trips", trips.len());
            Ok(Json(trips.into_iter().map(|t| t.into()).collect()))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error getting all trips: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_trip_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    Path(trip_id): Path<Uuid>,
) -> Result<Json<TripInfo>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Getting trip: {} (admin)", trip_id);
    match state.trips_client.get_trip(trip_id).await {
        Ok(trip) => {
            info!("Trip retrieved successfully: {}", trip_id);
            Ok(Json(trip.into()))
        }
        Err(DispatcherError::NotFound { resource }) => {
            error!("Trip not found: {}", resource);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": format!("{} not found", resource)})),
            ))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error getting trip: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn send_command_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    Json(request): Json<SendCommandRequest>,
) -> Result<Json<SendCommandResponse>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Sending command {} to car {} (admin)", request.command_type, request.car_id);
    match state.telematics_client.send_command(request.car_id, request.command_type).await {
        Ok(command_id) => {
            info!("Command sent successfully: {}", command_id);
            Ok(Json(SendCommandResponse { command_id }))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error sending command: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

