use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, warn, error};
use crate::presentation::app_state::AppState;
use crate::domain::errors::TelematicsError;
use crate::domain::models::CommandType;

#[derive(Deserialize)]
pub struct SendCommandRequest {
    pub car_id: Uuid,
    pub command_type: String,
}

#[derive(Serialize)]
pub struct SendCommandResponse {
    pub command_id: Uuid,
}

pub async fn send_command_handler<P, R>(
    State(state): State<AppState<P, R>>,
    Json(request): Json<SendCommandRequest>,
) -> Result<Json<SendCommandResponse>, (StatusCode, Json<serde_json::Value>)>
where
    P: crate::domain::interfaces::RabbitMQPublisher + Send + Sync + 'static,
    R: crate::domain::interfaces::RedisRepository + Send + Sync + 'static,
{
    info!("Sending command {} to car {}", request.command_type, request.car_id);
    
    let command_type = match request.command_type.parse::<CommandType>() {
        Ok(cmd) => cmd,
        Err(e) => {
            warn!("Invalid command type: {}", e);
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": format!("Invalid command type: {}", e)})),
            ));
        }
    };

    match state.send_command_use_case.execute(request.car_id, command_type).await {
        Ok(command_id) => {
            info!("Command sent successfully: {}", command_id);
            Ok(Json(SendCommandResponse { command_id }))
        }
        Err(TelematicsError::RabbitMQPublishError(e)) => {
            error!("Failed to publish command: {}", e);
            Err((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error": "Failed to send command to car"})),
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

