use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error};
use crate::{
    presentation::app_state::AppState,
    domain::errors::UserError,
};

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

pub async fn authenticate_handler<R, H, T>(
    State(state): State<AppState<R, H, T>>,
    Json(request): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::UserRepository + Send + Sync + 'static,
    H: crate::domain::interfaces::PasswordHasher + Send + Sync + 'static,
    T: crate::domain::interfaces::TokenGenerator + Send + Sync + 'static,
{
    let email = request.email.clone();
    info!("Authenticating user with email: {}", email);
    
    let auth_request = crate::domain::models::AuthRequest {
        email: request.email,
        password: request.password,
    };
    match state.auth_use_case.execute(auth_request).await {
        Ok((token, user_id)) => {
            info!("User authenticated successfully: {}", email);
            Ok(Json(AuthResponse { token, user_id }))
        }
        Err(UserError::InvalidCredentials) => {
            warn!("Authentication failed: invalid credentials for email {}", email);
            Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid credentials"})),
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

