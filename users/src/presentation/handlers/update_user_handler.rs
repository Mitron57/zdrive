use axum::{
    extract::{State, Path},
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
pub struct UpdateUserRequest {
    pub license_id: Option<String>,
    pub driving_experience: Option<u32>,
    pub rating: Option<f64>,
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateUserResponse {
    pub message: String,
}

pub async fn update_user_handler<R, H, T>(
    State(state): State<AppState<R, H, T>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<UpdateUserResponse>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::UserRepository + Send + Sync + 'static,
    H: crate::domain::interfaces::PasswordHasher + Send + Sync + 'static,
    T: crate::domain::interfaces::TokenGenerator + Send + Sync + 'static,
{
    let update_request = crate::domain::models::UpdateUserRequest {
        license_id: request.license_id,
        driving_experience: request.driving_experience,
        rating: request.rating,
        email: request.email,
    };

    info!("Updating user: {}", user_id);
    match state.update_use_case.execute(user_id, update_request).await {
        Ok(_) => {
            info!("User updated successfully: {}", user_id);
            Ok(Json(UpdateUserResponse {
                message: "User updated successfully".to_string(),
            }))
        }
        Err(UserError::NotFound) => {
            warn!("User not found for update: {}", user_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "User not found"})),
            ))
        }
        Err(UserError::AlreadyExists { email }) => {
            warn!("Update failed: email {} already exists", email);
            Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({"error": format!("User with email {} already exists", email)})),
            ))
        }
        Err(e) => {
            error!("Error updating user {}: {:?}", user_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

