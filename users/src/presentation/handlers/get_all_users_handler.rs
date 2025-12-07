use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use tracing::{info, error};
use crate::presentation::app_state::AppState;
use crate::presentation::handlers::types::UserResponse;

pub async fn get_all_users_handler<R, H, T>(
    State(state): State<AppState<R, H, T>>,
) -> Result<Json<Vec<UserResponse>>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::UserRepository + Send + Sync + 'static,
    H: crate::domain::interfaces::PasswordHasher + Send + Sync + 'static,
    T: crate::domain::interfaces::TokenGenerator + Send + Sync + 'static,
{
    info!("Getting all users");
    match state.get_all_users_use_case.execute().await {
        Ok(users) => {
            info!("Users retrieved successfully: {} users", users.len());
            Ok(Json(users.into_iter().map(|u| u.into()).collect()))
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

