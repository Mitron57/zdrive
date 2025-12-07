use axum::{
    Router,
    routing::{get, post},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use crate::presentation::{handlers::*, app_state::AppState};

pub fn create_router<R, Q>(app_state: AppState<R, Q>) -> Router
where
    R: crate::domain::interfaces::PaymentRepository + Send + Sync + 'static,
    Q: crate::domain::interfaces::QRCodeGenerator + Send + Sync + 'static,
{
    info!("Setting up routes...");
    Router::new()
        .route("/payments", post(create_payment_handler))
        .route("/payments/:id", get(get_payment_handler))
        .route("/users/:user_id/payments", get(get_user_payments_handler))
        .with_state(app_state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

