use axum::{
    Router,
    routing::{get, post, put},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use crate::presentation::{handlers::*, app_state::AppState};

pub fn create_router<R>(app_state: AppState<R>) -> Router
where
    R: crate::domain::interfaces::TripRepository + Send + Sync + 'static,
{
    info!("Setting up routes...");
    Router::new()
        .route("/trips", post(start_trip_handler))
        .route("/trips", get(get_all_trips_handler))
        .route("/trips/:id", get(get_trip_handler))
        .route("/trips/:id/activate", put(activate_trip_handler))
        .route("/trips/:id/end", put(end_trip_handler))
        .route("/trips/:id/cancel", put(cancel_trip_handler))
        .route("/users/:user_id/trips", get(get_user_trips_handler))
        .with_state(app_state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

