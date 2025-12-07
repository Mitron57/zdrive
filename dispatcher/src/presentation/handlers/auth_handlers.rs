use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use crate::presentation::app_state::AppState;
use crate::domain::errors::DispatcherError;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub license_id: String,
    pub driving_experience: u32,
    pub rating: f64,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user_id: uuid::Uuid,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: uuid::Uuid,
}

pub async fn register_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Registering new user: {}", request.email);
    let register_req = crate::domain::interfaces::RegisterRequest {
        license_id: request.license_id,
        driving_experience: request.driving_experience,
        rating: request.rating,
        email: request.email,
        password: request.password,
    };

    match state.users_client.register(register_req).await {
        Ok(response) => {
            info!("User registered successfully: {}", response.user_id);
            Ok(Json(RegisterResponse {
                user_id: response.user_id,
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
            error!("Error registering user: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn authenticate_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    Json(request): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    let email = request.email.clone();
    info!("Authenticating user: {}", email);
    let auth_req = crate::domain::interfaces::AuthRequest {
        email: request.email,
        password: request.password,
    };

    match state.users_client.authenticate(auth_req).await {
        Ok(response) => {
            info!("User authenticated successfully: {}", response.user_id);
            Ok(Json(AuthResponse {
                token: response.token,
                user_id: response.user_id,
            }))
        }
        Err(DispatcherError::Unauthorized) => {
            error!("Authentication failed for user: {}", email);
            Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid credentials"})),
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
            error!("Error authenticating user: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

