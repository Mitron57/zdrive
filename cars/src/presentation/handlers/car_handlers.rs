use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, warn, error};
use crate::{
    presentation::app_state::AppState,
    domain::errors::CarError,
    domain::models::CarState,
};

#[derive(Deserialize)]
pub struct CreateCarRequest {
    pub model: String,
    pub license_plate: String,
    pub iot_serial_number: String,
    #[serde(with = "car_state_serde")]
    pub state: CarState,
    pub tariff_id: Uuid,
    pub base_price: f64,
}

#[derive(Serialize)]
pub struct CreateCarResponse {
    pub car_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateCarRequest {
    pub model: Option<String>,
    pub license_plate: Option<String>,
    pub iot_serial_number: Option<String>,
    #[serde(with = "option_car_state_serde")]
    pub state: Option<CarState>,
    pub tariff_id: Option<Uuid>,
    pub base_price: Option<f64>,
}

#[derive(Serialize)]
pub struct UpdateCarResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct CarResponse {
    pub id: Uuid,
    pub model: String,
    pub license_plate: String,
    pub iot_serial_number: String,
    #[serde(with = "car_state_serde")]
    pub state: CarState,
    pub tariff_id: Uuid,
    pub base_price: f64,
}

impl From<crate::domain::models::Car> for CarResponse {
    fn from(car: crate::domain::models::Car) -> Self {
        Self {
            id: car.id,
            model: car.model,
            license_plate: car.license_plate,
            iot_serial_number: car.iot_serial_number,
            state: car.state,
            tariff_id: car.tariff_id,
            base_price: car.base_price,
        }
    }
}

#[derive(Deserialize)]
pub struct ListCarsQuery {
    pub tariff_id: Option<Uuid>,
}

mod car_state_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use crate::domain::models::CarState;

    pub fn serialize<S>(state: &CarState, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(state.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<CarState, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

mod option_car_state_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use crate::domain::models::CarState;

    #[allow(dead_code)]
    pub fn serialize<S>(state: &Option<CarState>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match state {
            Some(s) => serializer.serialize_str(s.as_str()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<CarState>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => s.parse().map(Some).map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

pub async fn create_car_handler<CR, TR>(
    State(state): State<AppState<CR, TR>>,
    Json(request): Json<CreateCarRequest>,
) -> Result<Json<CreateCarResponse>, (StatusCode, Json<serde_json::Value>)>
where
    CR: crate::domain::interfaces::CarRepository + Send + Sync + 'static,
    TR: crate::domain::interfaces::TariffRepository + Send + Sync + 'static,
{
    info!("Creating car with license plate: {}", request.license_plate);
    let create_request = crate::domain::models::CreateCarRequest {
        model: request.model,
        license_plate: request.license_plate,
        iot_serial_number: request.iot_serial_number,
        state: request.state,
        tariff_id: request.tariff_id,
        base_price: request.base_price,
    };

    match state.create_car_use_case.execute(create_request).await {
        Ok(car_id) => {
            info!("Car created successfully: {}", car_id);
            Ok(Json(CreateCarResponse { car_id }))
        }
        Err(CarError::TariffNotFound) => {
            warn!("Car creation failed: tariff not found");
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Tariff not found"})),
            ))
        }
        Err(CarError::CarAlreadyExists { license_plate }) => {
            warn!("Car creation failed: license plate {} already exists", license_plate);
            Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({"error": format!("Car with license plate {} already exists", license_plate)})),
            ))
        }
        Err(CarError::IotDeviceAlreadyRegistered { serial_number }) => {
            warn!("Car creation failed: IoT device {} already registered", serial_number);
            Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({"error": format!("IoT device {} already registered", serial_number)})),
            ))
        }
        Err(e) => {
            error!("Error creating car: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_car_handler<CR, TR>(
    State(state): State<AppState<CR, TR>>,
    Path(car_id): Path<Uuid>,
) -> Result<Json<CarResponse>, (StatusCode, Json<serde_json::Value>)>
where
    CR: crate::domain::interfaces::CarRepository + Send + Sync + 'static,
    TR: crate::domain::interfaces::TariffRepository + Send + Sync + 'static,
{
    info!("Getting car: {}", car_id);
    match state.get_car_use_case.execute(car_id).await {
        Ok(car) => {
            info!("Car retrieved successfully: {}", car_id);
            Ok(Json(car.into()))
        }
        Err(CarError::CarNotFound) => {
            warn!("Car not found: {}", car_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Car not found"})),
            ))
        }
        Err(e) => {
            error!("Error getting car {}: {:?}", car_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn update_car_handler<CR, TR>(
    State(state): State<AppState<CR, TR>>,
    Path(car_id): Path<Uuid>,
    Json(request): Json<UpdateCarRequest>,
) -> Result<Json<UpdateCarResponse>, (StatusCode, Json<serde_json::Value>)>
where
    CR: crate::domain::interfaces::CarRepository + Send + Sync + 'static,
    TR: crate::domain::interfaces::TariffRepository + Send + Sync + 'static,
{
    info!("Updating car: {}", car_id);
    let update_request = crate::domain::models::UpdateCarRequest {
        model: request.model,
        license_plate: request.license_plate,
        iot_serial_number: request.iot_serial_number,
        state: request.state,
        tariff_id: request.tariff_id,
        base_price: request.base_price,
    };

    match state.update_car_use_case.execute(car_id, update_request).await {
        Ok(_) => {
            info!("Car updated successfully: {}", car_id);
            Ok(Json(UpdateCarResponse {
                message: "Car updated successfully".to_string(),
            }))
        }
        Err(CarError::CarNotFound) => {
            warn!("Car not found for update: {}", car_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Car not found"})),
            ))
        }
        Err(CarError::TariffNotFound) => {
            warn!("Update failed: tariff not found");
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Tariff not found"})),
            ))
        }
        Err(CarError::CarAlreadyExists { license_plate }) => {
            warn!("Update failed: license plate {} already exists", license_plate);
            Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({"error": format!("Car with license plate {} already exists", license_plate)})),
            ))
        }
        Err(CarError::IotDeviceAlreadyRegistered { serial_number }) => {
            warn!("Update failed: IoT device {} already registered", serial_number);
            Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({"error": format!("IoT device {} already registered", serial_number)})),
            ))
        }
        Err(e) => {
            error!("Error updating car {}: {:?}", car_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn delete_car_handler<CR, TR>(
    State(state): State<AppState<CR, TR>>,
    Path(car_id): Path<Uuid>,
) -> Result<Json<UpdateCarResponse>, (StatusCode, Json<serde_json::Value>)>
where
    CR: crate::domain::interfaces::CarRepository + Send + Sync + 'static,
    TR: crate::domain::interfaces::TariffRepository + Send + Sync + 'static,
{
    info!("Deleting car: {}", car_id);
    match state.delete_car_use_case.execute(car_id).await {
        Ok(_) => {
            info!("Car deleted successfully: {}", car_id);
            Ok(Json(UpdateCarResponse {
                message: "Car deleted successfully".to_string(),
            }))
        }
        Err(CarError::CarNotFound) => {
            warn!("Car not found for deletion: {}", car_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Car not found"})),
            ))
        }
        Err(e) => {
            error!("Error deleting car {}: {:?}", car_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn list_cars_handler<CR, TR>(
    State(state): State<AppState<CR, TR>>,
    Query(params): Query<ListCarsQuery>,
) -> Result<Json<Vec<CarResponse>>, (StatusCode, Json<serde_json::Value>)>
where
    CR: crate::domain::interfaces::CarRepository + Send + Sync + 'static,
    TR: crate::domain::interfaces::TariffRepository + Send + Sync + 'static,
{
    info!("Listing cars, tariff_id: {:?}", params.tariff_id);
    match state.list_cars_use_case.execute(params.tariff_id).await {
        Ok(cars) => {
            info!("Cars retrieved successfully: {} cars", cars.len());
            Ok(Json(cars.into_iter().map(|c| c.into()).collect()))
        }
        Err(e) => {
            error!("Error listing cars: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

