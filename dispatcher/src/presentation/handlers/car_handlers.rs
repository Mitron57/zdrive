use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use tracing::{info, error};
use crate::presentation::app_state::AppState;
use crate::domain::errors::DispatcherError;
use crate::domain::models::scenarios::CarDataResponse;
use crate::domain::interfaces::CarInfo;

pub async fn get_car_data_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    Path(car_id): Path<Uuid>,
) -> Result<Json<CarDataResponse>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Getting car data: {}", car_id);
    match state.get_car_data_scenario.execute(car_id).await {
        Ok(car_data) => {
            info!("Car data retrieved successfully: {}", car_id);
            Ok(Json(car_data))
        }
        Err(DispatcherError::NotFound { resource }) => {
            error!("Car not found: {}", resource);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": format!("{} not found", resource)})),
            ))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error getting car data: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_available_cars_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
) -> Result<Json<Vec<CarInfo>>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    info!("Getting available cars");
    match state.cars_client.get_all_cars().await {
        Ok(cars) => {
            // Фильтруем только доступные машины и добавляем тарифы
            let mut available_cars: Vec<CarInfo> = cars
                .into_iter()
                .filter(|car| car.state == "available")
                .collect();
            
            // Получаем тарифы для каждой машины
            for car in &mut available_cars {
                match state.cars_client.get_tariff(car.tariff_id).await {
                    Ok(tariff) => {
                        car.price_per_minute = Some(tariff.price_per_minute);
                    }
                    Err(e) => {
                        error!("Failed to get tariff {} for car {}: {:?}", car.tariff_id, car.id, e);
                        // Продолжаем без тарифа
                    }
                }
            }
            
            info!("Available cars retrieved successfully: {} cars", available_cars.len());
            Ok(Json(available_cars))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
            ))
        }
        Err(e) => {
            error!("Error getting available cars: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct SendCarCommandRequest {
    pub command_type: String,
}

#[derive(serde::Serialize)]
pub struct SendCarCommandResponse {
    pub command_id: uuid::Uuid,
    pub message: String,
}

pub async fn send_car_command_handler<UC, CC, TC, TMC, BC>(
    State(state): State<AppState<UC, CC, TC, TMC, BC>>,
    Path(car_id): Path<Uuid>,
    Json(request): Json<SendCarCommandRequest>,
) -> Result<Json<SendCarCommandResponse>, (StatusCode, Json<serde_json::Value>)>
where
    UC: crate::domain::interfaces::UsersServiceClient + Send + Sync + 'static,
    CC: crate::domain::interfaces::CarsServiceClient + Send + Sync + 'static,
    TC: crate::domain::interfaces::TripsServiceClient + Send + Sync + 'static,
    TMC: crate::domain::interfaces::TelematicsServiceClient + Send + Sync + 'static,
    BC: crate::domain::interfaces::BillingServiceClient + Send + Sync + 'static,
{
    let command_type = request.command_type.clone();
    info!("Sending command {} to car {} (client)", command_type, car_id);
    match state.telematics_client.send_command(car_id, command_type.clone()).await {
        Ok(command_id) => {
            info!("Command sent successfully: {}", command_id);
            Ok(Json(SendCarCommandResponse {
                command_id,
                message: format!("Command {} sent successfully", command_type),
            }))
        }
        Err(DispatcherError::ServiceError { service, message }) => {
            error!("Service error from {}: {}", service, message);
            Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Service {} unavailable", service)})),
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
