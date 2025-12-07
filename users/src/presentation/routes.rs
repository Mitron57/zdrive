use axum::{
    Router,
    routing::{get, post, put},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use crate::{
    presentation::{handlers::*, app_state::AppState},
};

pub fn create_router<R, H, T>(app_state: AppState<R, H, T>) -> Router
where
    R: crate::domain::interfaces::UserRepository + Send + Sync + 'static,
    H: crate::domain::interfaces::PasswordHasher + Send + Sync + 'static,
    T: crate::domain::interfaces::TokenGenerator + Send + Sync + 'static,
{
    info!("Setting up routes...");
    Router::new()
        .route("/users/register", post(register_handler))
        .route("/users/authenticate", post(authenticate_handler))
        .route("/users", get(get_all_users_handler))
        .route("/users/:id", get(get_user_handler))
        .route("/users/:id", put(update_user_handler))
        .with_state(app_state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

