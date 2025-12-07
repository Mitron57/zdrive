use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use tracing::{info, error};
use crate::presentation::app_state::AppState;
use crate::presentation::handlers::trip_handlers::TripResponse;

pub async fn get_all_trips_handler<R>(
    State(state): State<AppState<R>>,
) -> Result<Json<Vec<TripResponse>>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::TripRepository + Send + Sync + 'static,
{
    info!("Getting all trips");
    match state.get_all_trips_use_case.execute().await {
        Ok(trips) => {
            info!("Trips retrieved successfully: {} trips", trips.len());
            Ok(Json(trips.into_iter().map(|t| t.into()).collect()))
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

