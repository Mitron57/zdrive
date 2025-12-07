use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, error};
use crate::presentation::app_state::AppState;
use crate::domain::errors::DispatcherError;

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
pub struct ActivateTripRequest {
    pub trip_id: Uuid,
}

#[derive(Serialize)]
pub struct ActivateTripResponse {
    pub trip_id: Uuid,
    pub message: String,
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
pub struct ActiveTripResponse {
    pub trip: Option<crate::domain::interfaces::TripInfo>,
}

pub async fn start_trip_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    Json(request): Json<StartTripRequest>,
) -> Result<Json<StartTripResponse>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Starting trip for user {} with car {}", request.user_id, request.car_id);
    match state.start_trip_scenario.execute(request.user_id, request.car_id).await {
        Ok(trip_id) => {
            info!("Trip started successfully: {}", trip_id);
            Ok(Json(StartTripResponse { trip_id }))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
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

pub async fn end_trip_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    Json(request): Json<EndTripRequest>,
) -> Result<Json<EndTripResponse>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Ending trip: {}", request.trip_id);
    match state.end_trip_scenario.execute(request.trip_id).await {
        Ok((trip_id, payment_id, qr_code_url)) => {
            info!("Trip ended successfully: {}, payment: {}", trip_id, payment_id);
            Ok(Json(EndTripResponse {
                trip_id,
                payment_id,
                qr_code_url,
            }))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error ending trip: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn activate_trip_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    Json(request): Json<ActivateTripRequest>,
) -> Result<Json<ActivateTripResponse>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Activating trip: {}", request.trip_id);
    match state.trips_client.activate_trip(request.trip_id).await {
        Ok(_) => {
            info!("Trip activated successfully: {}", request.trip_id);
            Ok(Json(ActivateTripResponse {
                trip_id: request.trip_id,
                message: "Trip activated successfully".to_string(),
            }))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error activating trip: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_active_trip_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ActiveTripResponse>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    let user_id = params.get("user_id")
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing or invalid user_id parameter"})),
        ))?;
    
    info!("Getting active trip for user: {}", user_id);
    match state.trips_client.get_user_active_trip(user_id).await {
        Ok(trip) => {
            Ok(Json(ActiveTripResponse { trip }))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error getting active trip: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn cancel_trip_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    Json(request): Json<CancelTripRequest>,
) -> Result<Json<CancelTripResponse>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Cancelling trip: {}", request.trip_id);
    match state.cancel_trip_scenario.execute(request.trip_id).await {
        Ok(_) => {
            info!("Trip cancelled successfully: {}", request.trip_id);
            Ok(Json(CancelTripResponse {
                trip_id: request.trip_id,
                message: "Trip cancelled successfully".to_string(),
            }))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error cancelling trip: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

