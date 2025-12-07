use axum::{
    Router,
    routing::{get, post},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use crate::presentation::{handlers::*, app_state::AppState};

pub fn create_router<P, R>(app_state: AppState<P, R>) -> Router
where
    P: crate::domain::interfaces::RabbitMQPublisher + Send + Sync + 'static,
    R: crate::domain::interfaces::RedisRepository + Send + Sync + 'static,
{
    info!("Setting up routes...");
    Router::new()
        .route("/commands", post(send_command_handler))
        .route("/sensors", get(get_sensor_data_handler))
        .route("/sensors/car/:car_id", get(get_sensor_data_by_car_id_handler))
        .route("/sensors/all", get(get_all_sensor_data_handler))
        .with_state(app_state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

