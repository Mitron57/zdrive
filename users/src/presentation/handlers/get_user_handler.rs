use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use tracing::{info, warn, error};
use crate::{
    presentation::app_state::AppState,
    domain::errors::UserError,
    presentation::handlers::types::UserResponse,
};

pub async fn get_user_handler<R, H, T>(
    State(state): State<AppState<R, H, T>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::UserRepository + Send + Sync + 'static,
    H: crate::domain::interfaces::PasswordHasher + Send + Sync + 'static,
    T: crate::domain::interfaces::TokenGenerator + Send + Sync + 'static,
{
    info!("Getting user: {}", user_id);
    match state.get_use_case.execute(user_id).await {
        Ok(user) => {
            info!("User retrieved successfully: {}", user_id);
            Ok(Json(user.into()))
        }
        Err(UserError::NotFound) => {
            warn!("User not found: {}", user_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "User not found"})),
            ))
        }
        Err(e) => {
            error!("Error getting user {}: {:?}", user_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

