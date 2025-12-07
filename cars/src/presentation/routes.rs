use axum::{
    Router,
    routing::{get, post, put, delete},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use crate::presentation::{handlers::*, app_state::AppState};

pub fn create_router<CR, TR>(app_state: AppState<CR, TR>) -> Router
where
    CR: crate::domain::interfaces::CarRepository + Send + Sync + 'static,
    TR: crate::domain::interfaces::TariffRepository + Send + Sync + 'static,
{
    info!("Setting up routes...");
    Router::new()
        // Car routes
        .route("/cars", post(create_car_handler))
        .route("/cars", get(list_cars_handler))
        .route("/cars/:id", get(get_car_handler))
        .route("/cars/:id", put(update_car_handler))
        .route("/cars/:id", delete(delete_car_handler))
        // Tariff routes
        .route("/tariffs", post(create_tariff_handler))
        .route("/tariffs", get(list_tariffs_handler))
        .route("/tariffs/:id", get(get_tariff_handler))
        .route("/tariffs/:id", put(update_tariff_handler))
        .with_state(app_state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

