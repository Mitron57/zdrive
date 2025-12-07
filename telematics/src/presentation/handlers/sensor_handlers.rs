use axum::{
    extract::{State, Query, Path},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, error, warn};
use crate::presentation::app_state::AppState;

#[derive(Serialize)]
pub struct SensorDataResponse {
    pub vin: String,
    pub license_plate: String,
    pub fuel_level: f64,
    pub location: LocationResponse,
    pub door_status: String,
    pub speed: f64,
    pub temperature: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct LocationResponse {
    pub latitude: f64,
    pub longitude: f64,
}

impl From<crate::domain::models::SensorData> for SensorDataResponse {
    fn from(data: crate::domain::models::SensorData) -> Self {
        Self {
            vin: data.vin,
            license_plate: data.license_plate,
            fuel_level: data.fuel_level,
            location: LocationResponse {
                latitude: data.location.latitude,
                longitude: data.location.longitude,
            },
            door_status: data.door_status.as_str().to_string(),
            speed: data.speed,
            temperature: data.temperature,
            timestamp: data.timestamp,
        }
    }
}

#[derive(Deserialize)]
pub struct GetSensorDataQuery {
    pub vin: Option<String>,
}

pub async fn get_sensor_data_handler<P, R>(
    State(state): State<AppState<P, R>>,
    Query(params): Query<GetSensorDataQuery>,
) -> Result<Json<Vec<SensorDataResponse>>, (StatusCode, Json<serde_json::Value>)>
where
    P: crate::domain::interfaces::RabbitMQPublisher + Send + Sync + 'static,
    R: crate::domain::interfaces::RedisRepository + Send + Sync + 'static,
{
    info!("Getting sensor data, VIN: {:?}", params.vin);
    match state.get_sensor_data_use_case.execute(params.vin.as_deref()).await {
        Ok(data_list) => {
            info!("Sensor data retrieved successfully: {} entries", data_list.len());
            Ok(Json(data_list.into_iter().map(|(_, data)| data.into()).collect()))
        }
        Err(e) => {
            error!("Error getting sensor data: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

#[derive(Serialize)]
pub struct SensorDataWithVin {
    pub vin: String,
    pub license_plate: String,
    pub fuel_level: f64,
    pub location: LocationResponse,
    pub door_status: String,
    pub speed: f64,
    pub temperature: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub async fn get_sensor_data_by_car_id_handler<P, R>(
    State(_state): State<AppState<P, R>>,
    Path(car_id): Path<Uuid>,
) -> Result<Json<Option<SensorDataWithVin>>, (StatusCode, Json<serde_json::Value>)>
where
    P: crate::domain::interfaces::RabbitMQPublisher + Send + Sync + 'static,
    R: crate::domain::interfaces::RedisRepository + Send + Sync + 'static,
{
    info!("Getting sensor data by car_id: {}", car_id);
    
    // Примечание: В реальном приложении здесь нужно сделать запрос к cars сервису
    // для получения license_plate по car_id, а затем искать в Redis по license_plate.
    // Для MVP пока возвращаем ошибку с подсказкой.
    
    warn!("get_sensor_data_by_car_id: car_id lookup requires integration with cars service. Use /sensors?vin=... or /sensors/all instead");
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "error": "Car ID lookup requires integration with cars service",
            "message": "To get sensor data by car_id, first get license_plate from cars service, then use /sensors?vin={vin} or /sensors/all",
            "alternative": "Use /sensors/all to get all sensor data and filter by car_id on client side"
        })),
    ))
}

pub async fn get_all_sensor_data_handler<P, R>(
    State(state): State<AppState<P, R>>,
) -> Result<Json<Vec<SensorDataWithVin>>, (StatusCode, Json<serde_json::Value>)>
where
    P: crate::domain::interfaces::RabbitMQPublisher + Send + Sync + 'static,
    R: crate::domain::interfaces::RedisRepository + Send + Sync + 'static,
{
    info!("Getting all sensor data");
    match state.get_sensor_data_use_case.execute(None).await {
        Ok(data_list) => {
            info!("All sensor data retrieved successfully: {} entries", data_list.len());
            let response: Vec<SensorDataWithVin> = data_list
                .into_iter()
                .map(|(vin, data)| {
                    let sensor_response: SensorDataResponse = data.into();
                    SensorDataWithVin {
                        vin,
                        license_plate: sensor_response.license_plate,
                        fuel_level: sensor_response.fuel_level,
                        location: sensor_response.location,
                        door_status: sensor_response.door_status,
                        speed: sensor_response.speed,
                        temperature: sensor_response.temperature,
                        timestamp: sensor_response.timestamp,
                    }
                })
                .collect();
            Ok(Json(response))
        }
        Err(e) => {
            error!("Error getting all sensor data: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

