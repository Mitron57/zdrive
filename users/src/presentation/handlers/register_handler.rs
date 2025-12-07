use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, warn, error};
use crate::{
    presentation::app_state::AppState,
    domain::errors::UserError,
};

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
    pub user_id: Uuid,
}

pub async fn register_handler<R, H, T>(
    State(state): State<AppState<R, H, T>>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::UserRepository + Send + Sync + 'static,
    H: crate::domain::interfaces::PasswordHasher + Send + Sync + 'static,
    T: crate::domain::interfaces::TokenGenerator + Send + Sync + 'static,
{
    let email = request.email.clone();
    info!("Registering new user with email: {}", email);
    
    let create_request = crate::domain::models::CreateUserRequest {
        license_id: request.license_id,
        driving_experience: request.driving_experience,
        rating: request.rating,
        email: request.email,
        password: request.password,
    };
    match state.register_use_case.execute(create_request).await {
        Ok(user_id) => {
            info!("User registered successfully: {}", user_id);
            Ok(Json(RegisterResponse { user_id }))
        }
        Err(UserError::AlreadyExists { email }) => {
            warn!("Registration failed: user with email {} already exists", email);
            Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({"error": format!("User with email {} already exists", email)})),
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

