use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, warn, error};
use crate::presentation::app_state::AppState;
use crate::domain::errors::CarError;

#[derive(Deserialize)]
pub struct CreateTariffRequest {
    pub price_per_minute: f64,
    pub minimal_rating: f64,
    pub minimal_experience: u32,
}

#[derive(Serialize)]
pub struct CreateTariffResponse {
    pub tariff_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateTariffRequest {
    pub price_per_minute: Option<f64>,
    pub minimal_rating: Option<f64>,
    pub minimal_experience: Option<u32>,
}

#[derive(Serialize)]
pub struct UpdateTariffResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct TariffResponse {
    pub id: Uuid,
    pub price_per_minute: f64,
    pub minimal_rating: f64,
    pub minimal_experience: u32,
}

impl From<crate::domain::models::Tariff> for TariffResponse {
    fn from(tariff: crate::domain::models::Tariff) -> Self {
        Self {
            id: tariff.id,
            price_per_minute: tariff.price_per_minute,
            minimal_rating: tariff.minimal_rating,
            minimal_experience: tariff.minimal_experience,
        }
    }
}

pub async fn create_tariff_handler<CR, TR>(
    State(state): State<AppState<CR, TR>>,
    Json(request): Json<CreateTariffRequest>,
) -> Result<Json<CreateTariffResponse>, (StatusCode, Json<serde_json::Value>)>
where
    CR: crate::domain::interfaces::CarRepository + Send + Sync + 'static,
    TR: crate::domain::interfaces::TariffRepository + Send + Sync + 'static,
{
    info!("Creating tariff with price_per_minute: {}", request.price_per_minute);
    let create_request = crate::domain::models::CreateTariffRequest {
        price_per_minute: request.price_per_minute,
        minimal_rating: request.minimal_rating,
        minimal_experience: request.minimal_experience,
    };

    match state.create_tariff_use_case.execute(create_request).await {
        Ok(tariff_id) => {
            info!("Tariff created successfully: {}", tariff_id);
            Ok(Json(CreateTariffResponse { tariff_id }))
        }
        Err(e) => {
            error!("Error creating tariff: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_tariff_handler<CR, TR>(
    State(state): State<AppState<CR, TR>>,
    Path(tariff_id): Path<Uuid>,
) -> Result<Json<TariffResponse>, (StatusCode, Json<serde_json::Value>)>
where
    CR: crate::domain::interfaces::CarRepository + Send + Sync + 'static,
    TR: crate::domain::interfaces::TariffRepository + Send + Sync + 'static,
{
    info!("Getting tariff: {}", tariff_id);
    match state.get_tariff_use_case.execute(tariff_id).await {
        Ok(tariff) => {
            info!("Tariff retrieved successfully: {}", tariff_id);
            Ok(Json(tariff.into()))
        }
        Err(CarError::TariffNotFound) => {
            warn!("Tariff not found: {}", tariff_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Tariff not found"})),
            ))
        }
        Err(e) => {
            error!("Error getting tariff {}: {:?}", tariff_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn update_tariff_handler<CR, TR>(
    State(state): State<AppState<CR, TR>>,
    Path(tariff_id): Path<Uuid>,
    Json(request): Json<UpdateTariffRequest>,
) -> Result<Json<UpdateTariffResponse>, (StatusCode, Json<serde_json::Value>)>
where
    CR: crate::domain::interfaces::CarRepository + Send + Sync + 'static,
    TR: crate::domain::interfaces::TariffRepository + Send + Sync + 'static,
{
    info!("Updating tariff: {}", tariff_id);
    let update_request = crate::domain::models::UpdateTariffRequest {
        price_per_minute: request.price_per_minute,
        minimal_rating: request.minimal_rating,
        minimal_experience: request.minimal_experience,
    };

    match state.update_tariff_use_case.execute(tariff_id, update_request).await {
        Ok(_) => {
            info!("Tariff updated successfully: {}", tariff_id);
            Ok(Json(UpdateTariffResponse {
                message: "Tariff updated successfully".to_string(),
            }))
        }
        Err(CarError::TariffNotFound) => {
            warn!("Tariff not found for update: {}", tariff_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Tariff not found"})),
            ))
        }
        Err(e) => {
            error!("Error updating tariff {}: {:?}", tariff_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn list_tariffs_handler<CR, TR>(
    State(state): State<AppState<CR, TR>>,
) -> Result<Json<Vec<TariffResponse>>, (StatusCode, Json<serde_json::Value>)>
where
    CR: crate::domain::interfaces::CarRepository + Send + Sync + 'static,
    TR: crate::domain::interfaces::TariffRepository + Send + Sync + 'static,
{
    info!("Listing tariffs");
    match state.list_tariffs_use_case.execute().await {
        Ok(tariffs) => {
            info!("Tariffs retrieved successfully: {} tariffs", tariffs.len());
            Ok(Json(tariffs.into_iter().map(|t| t.into()).collect()))
        }
        Err(e) => {
            error!("Error listing tariffs: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

