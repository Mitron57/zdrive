use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, warn, error};
use crate::presentation::app_state::AppState;
use crate::domain::errors::TripError;

#[derive(Deserialize)]
pub struct StartTripRequest {
    pub user_id: Uuid,
    pub car_id: Uuid,
}

#[derive(Serialize)]
pub struct StartTripResponse {
    pub trip_id: Uuid,
}

#[derive(Serialize)]
pub struct TripResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub car_id: Uuid,
    pub status: String,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub cancelled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<crate::domain::models::Trip> for TripResponse {
    fn from(trip: crate::domain::models::Trip) -> Self {
        Self {
            id: trip.id,
            user_id: trip.user_id,
            car_id: trip.car_id,
            status: trip.status.as_str().to_string(),
            started_at: trip.started_at,
            ended_at: trip.ended_at,
            cancelled_at: trip.cancelled_at,
            created_at: trip.created_at,
        }
    }
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

pub async fn start_trip_handler<R>(
    State(state): State<AppState<R>>,
    Json(request): Json<StartTripRequest>,
) -> Result<Json<StartTripResponse>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::TripRepository + Send + Sync + 'static,
{
    info!("Starting trip for user {} with car {}", request.user_id, request.car_id);
    let start_request = crate::domain::models::StartTripRequest {
        user_id: request.user_id,
        car_id: request.car_id,
    };

    match state.start_trip_use_case.execute(start_request).await {
        Ok(trip_id) => {
            info!("Trip started successfully: {}", trip_id);
            Ok(Json(StartTripResponse { trip_id }))
        }
        Err(TripError::UserHasActiveTrip) => {
            warn!("Start trip failed: user {} already has an active trip", request.user_id);
            Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({"error": "User already has an active trip"})),
            ))
        }
        Err(TripError::CarAlreadyInUse) => {
            warn!("Start trip failed: car {} is already in use", request.car_id);
            Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({"error": "Car is already in use"})),
            ))
        }
        Err(e) => {
            error!("Error starting trip: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn activate_trip_handler<R>(
    State(state): State<AppState<R>>,
    Path(trip_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::TripRepository + Send + Sync + 'static,
{
    info!("Activating trip: {}", trip_id);
    match state.activate_trip_use_case.execute(trip_id).await {
        Ok(_) => {
            info!("Trip activated successfully: {}", trip_id);
            Ok(Json(MessageResponse {
                message: "Trip activated successfully".to_string(),
            }))
        }
        Err(TripError::TripNotFound) => {
            warn!("Trip not found: {}", trip_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Trip not found"})),
            ))
        }
        Err(TripError::InvalidStatusTransition { from, to }) => {
            warn!("Invalid status transition: {} -> {}", from, to);
            Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": format!("Cannot activate trip: invalid status transition from {} to {}", from, to)})),
            ))
        }
        Err(e) => {
            error!("Error activating trip {}: {:?}", trip_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn end_trip_handler<R>(
    State(state): State<AppState<R>>,
    Path(trip_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::TripRepository + Send + Sync + 'static,
{
    info!("Ending trip: {}", trip_id);
    match state.end_trip_use_case.execute(trip_id).await {
        Ok(_) => {
            info!("Trip ended successfully: {}", trip_id);
            Ok(Json(MessageResponse {
                message: "Trip ended successfully".to_string(),
            }))
        }
        Err(TripError::TripNotFound) => {
            warn!("Trip not found: {}", trip_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Trip not found"})),
            ))
        }
        Err(TripError::InvalidStatusTransition { from, to }) => {
            warn!("Invalid status transition: {} -> {}", from, to);
            Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": format!("Cannot end trip: invalid status transition from {} to {}", from, to)})),
            ))
        }
        Err(e) => {
            error!("Error ending trip {}: {:?}", trip_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn cancel_trip_handler<R>(
    State(state): State<AppState<R>>,
    Path(trip_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::TripRepository + Send + Sync + 'static,
{
    info!("Cancelling trip: {}", trip_id);
    match state.cancel_trip_use_case.execute(trip_id).await {
        Ok(_) => {
            info!("Trip cancelled successfully: {}", trip_id);
            Ok(Json(MessageResponse {
                message: "Trip cancelled successfully".to_string(),
            }))
        }
        Err(TripError::TripNotFound) => {
            warn!("Trip not found: {}", trip_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Trip not found"})),
            ))
        }
        Err(TripError::InvalidStatusTransition { from, to }) => {
            warn!("Invalid status transition: {} -> {}", from, to);
            Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": format!("Cannot cancel trip: invalid status transition from {} to {}", from, to)})),
            ))
        }
        Err(e) => {
            error!("Error cancelling trip {}: {:?}", trip_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_trip_handler<R>(
    State(state): State<AppState<R>>,
    Path(trip_id): Path<Uuid>,
) -> Result<Json<TripResponse>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::TripRepository + Send + Sync + 'static,
{
    info!("Getting trip: {}", trip_id);
    match state.get_trip_use_case.execute(trip_id).await {
        Ok(trip) => {
            info!("Trip retrieved successfully: {}", trip_id);
            Ok(Json(trip.into()))
        }
        Err(TripError::TripNotFound) => {
            warn!("Trip not found: {}", trip_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Trip not found"})),
            ))
        }
        Err(e) => {
            error!("Error getting trip {}: {:?}", trip_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_user_trips_handler<R>(
    State(state): State<AppState<R>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<TripResponse>>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::TripRepository + Send + Sync + 'static,
{
    info!("Getting trips for user: {}", user_id);
    match state.get_user_trips_use_case.execute(user_id).await {
        Ok(trips) => {
            info!("Trips retrieved successfully: {} trips", trips.len());
            Ok(Json(trips.into_iter().map(|t| t.into()).collect()))
        }
        Err(e) => {
            error!("Error getting trips for user {}: {:?}", user_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

