use axum::{
    Router,
    routing::{get, post, put},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use crate::presentation::{handlers::*, app_state::AppState};

pub fn create_router<UC, CC, TC, TMC, BC>(app_state: AppState<UC, CC, TC, TMC, BC>) -> Router
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Setting up routes...");
    Router::new()
        // Client endpoints
        .route("/auth/register", post(register_handler))
        .route("/auth/authenticate", post(authenticate_handler))
        .route("/trips/start", post(start_trip_handler))
        .route("/trips/activate", put(activate_trip_handler))
        .route("/trips/end", put(end_trip_handler))
        .route("/trips/cancel", put(cancel_trip_handler))
        .route("/trips/active", get(get_active_trip_handler))
        .route("/cars", get(get_available_cars_handler))
        .route("/cars/:car_id/data", get(get_car_data_handler))
        .route("/cars/:car_id/commands", post(send_car_command_handler))
        // Admin endpoints
        .route("/admin/users", get(get_all_users_handler))
        .route("/admin/users/:id", get(get_user_handler))
        .route("/admin/cars", get(get_all_cars_handler))
        .route("/admin/cars/:id", get(get_car_handler))
        .route("/admin/trips", get(get_all_trips_handler))
        .route("/admin/trips/:id", get(get_trip_handler))
        .route("/admin/commands", post(send_command_handler))
        .with_state(app_state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

