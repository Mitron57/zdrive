use async_trait::async_trait;
use reqwest::Client;
use uuid::Uuid;
use tracing::{info, error, warn};
use crate::domain::{
    errors::DispatcherError,
    interfaces::*,
};

pub struct HttpUsersServiceClient {
    client: Client,
    base_url: String,
}

impl HttpUsersServiceClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[async_trait]
impl UsersServiceClient for HttpUsersServiceClient {
    async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, DispatcherError> {
        let url = format!("{}/users/register", self.base_url);
        info!("Calling users service: POST {}", url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Users service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "users".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn authenticate(&self, request: AuthRequest) -> Result<AuthResponse, DispatcherError> {
        let url = format!("{}/users/authenticate", self.base_url);
        info!("Calling users service: POST {}", url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            Err(DispatcherError::Unauthorized)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Users service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "users".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn get_user(&self, user_id: Uuid) -> Result<UserInfo, DispatcherError> {
        let url = format!("{}/users/{}", self.base_url, user_id);
        info!("Calling users service: GET {}", url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else if response.status() == reqwest::StatusCode::NOT_FOUND {
            Err(DispatcherError::NotFound {
                resource: format!("user {}", user_id),
            })
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Users service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "users".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn get_all_users(&self) -> Result<Vec<UserInfo>, DispatcherError> {
        let url = format!("{}/users", self.base_url);
        info!("Calling users service: GET {}", url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Users service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "users".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }
}

pub struct HttpCarsServiceClient {
    client: Client,
    base_url: String,
}

impl HttpCarsServiceClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[async_trait]
impl CarsServiceClient for HttpCarsServiceClient {
    async fn get_car(&self, car_id: Uuid) -> Result<CarInfo, DispatcherError> {
        let url = format!("{}/cars/{}", self.base_url, car_id);
        info!("Calling cars service: GET {}", url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else if response.status() == reqwest::StatusCode::NOT_FOUND {
            Err(DispatcherError::NotFound {
                resource: format!("car {}", car_id),
            })
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Cars service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "cars".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn get_all_cars(&self) -> Result<Vec<CarInfo>, DispatcherError> {
        let url = format!("{}/cars", self.base_url);
        info!("Calling cars service: GET {}", url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Cars service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "cars".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn get_tariff(&self, tariff_id: Uuid) -> Result<crate::domain::interfaces::TariffInfo, DispatcherError> {
        let url = format!("{}/tariffs/{}", self.base_url, tariff_id);
        info!("Calling cars service: GET {}", url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else if response.status() == reqwest::StatusCode::NOT_FOUND {
            Err(DispatcherError::NotFound {
                resource: format!("tariff {}", tariff_id),
            })
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Cars service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "cars".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }
}

pub struct HttpTripsServiceClient {
    client: Client,
    base_url: String,
}

impl HttpTripsServiceClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[async_trait]
impl TripsServiceClient for HttpTripsServiceClient {
    async fn start_trip(&self, user_id: Uuid, car_id: Uuid) -> Result<Uuid, DispatcherError> {
        let url = format!("{}/trips", self.base_url);
        info!("Calling trips service: POST {}", url);
        
        let request = serde_json::json!({
            "user_id": user_id,
            "car_id": car_id,
        });
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            Ok(result["trip_id"].as_str()
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| DispatcherError::InvalidRequest {
                    message: "Invalid response format".to_string(),
                })?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Trips service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "trips".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn activate_trip(&self, trip_id: Uuid) -> Result<(), DispatcherError> {
        let url = format!("{}/trips/{}/activate", self.base_url, trip_id);
        info!("Calling trips service: PUT {}", url);
        
        let response = self.client
            .put(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Trips service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "trips".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn end_trip(&self, trip_id: Uuid) -> Result<(), DispatcherError> {
        let url = format!("{}/trips/{}/end", self.base_url, trip_id);
        info!("Calling trips service: PUT {}", url);
        
        let response = self.client
            .put(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Trips service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "trips".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn cancel_trip(&self, trip_id: Uuid) -> Result<(), DispatcherError> {
        let url = format!("{}/trips/{}/cancel", self.base_url, trip_id);
        info!("Calling trips service: PUT {}", url);
        
        let response = self.client
            .put(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Trips service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "trips".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn get_trip(&self, trip_id: Uuid) -> Result<TripInfo, DispatcherError> {
        let url = format!("{}/trips/{}", self.base_url, trip_id);
        info!("Calling trips service: GET {}", url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else if response.status() == reqwest::StatusCode::NOT_FOUND {
            Err(DispatcherError::NotFound {
                resource: format!("trip {}", trip_id),
            })
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Trips service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "trips".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn get_user_active_trip(&self, user_id: Uuid) -> Result<Option<TripInfo>, DispatcherError> {
        let url = format!("{}/users/{}/trips", self.base_url, user_id);
        info!("Calling trips service: GET {}", url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            let trips: Vec<TripInfo> = response.json().await?;
            // Находим активную или зарезервированную поездку
            Ok(trips.into_iter()
                .find(|t| t.status == "active" || t.status == "reserved"))
        } else if response.status() == reqwest::StatusCode::NOT_FOUND {
            Ok(None)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Trips service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "trips".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn get_all_trips(&self) -> Result<Vec<TripInfo>, DispatcherError> {
        let url = format!("{}/trips", self.base_url);
        info!("Calling trips service: GET {}", url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Trips service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "trips".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }
}

pub struct HttpTelematicsServiceClient {
    client: Client,
    base_url: String,
}

impl HttpTelematicsServiceClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[async_trait]
impl TelematicsServiceClient for HttpTelematicsServiceClient {
    async fn send_command(&self, car_id: Uuid, command_type: String) -> Result<Uuid, DispatcherError> {
        let url = format!("{}/commands", self.base_url);
        info!("Calling telematics service: POST {}", url);
        
        let request = serde_json::json!({
            "car_id": car_id,
            "command_type": command_type,
        });
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            Ok(result["command_id"].as_str()
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| DispatcherError::InvalidRequest {
                    message: "Invalid response format".to_string(),
                })?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Telematics service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "telematics".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn get_sensor_data_by_car_id(&self, _car_id: Uuid) -> Result<Option<SensorDataInfo>, DispatcherError> {
        // Пока endpoint возвращает 501, используем get_all_sensor_data и фильтруем
        // В будущем можно реализовать через cars сервис для получения license_plate
        let _all_data = self.get_all_sensor_data().await?;
        
        // Получаем car из cars сервиса для получения license_plate
        // Пока используем упрощенный подход - ищем по car_id в данных
        // В реальном приложении нужно делать запрос к cars сервису
        
        // Для MVP возвращаем None, так как нужна интеграция с cars
        warn!("get_sensor_data_by_car_id: requires integration with cars service");
        Ok(None)
    }

    async fn get_all_sensor_data(&self) -> Result<Vec<SensorDataInfo>, DispatcherError> {
        let url = format!("{}/sensors/all", self.base_url);
        info!("Calling telematics service: GET {}", url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Telematics service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "telematics".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }
}

pub struct HttpBillingServiceClient {
    client: Client,
    base_url: String,
}

impl HttpBillingServiceClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[async_trait]
impl BillingServiceClient for HttpBillingServiceClient {
    async fn create_payment(&self, trip_id: Uuid, user_id: Uuid, amount: f64) -> Result<PaymentInfo, DispatcherError> {
        let url = format!("{}/payments", self.base_url);
        info!("Calling billing service: POST {}", url);
        
        let request = serde_json::json!({
            "trip_id": trip_id,
            "user_id": user_id,
            "amount": amount,
        });
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            let payment_id: Uuid = result["payment_id"].as_str()
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| DispatcherError::InvalidRequest {
                    message: "Invalid response format".to_string(),
                })?;
            
            // Получаем полную информацию о платеже
            self.get_payment(payment_id).await
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Billing service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "billing".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    async fn get_payment(&self, payment_id: Uuid) -> Result<PaymentInfo, DispatcherError> {
        let url = format!("{}/payments/{}", self.base_url, payment_id);
        info!("Calling billing service: GET {}", url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else if response.status() == reqwest::StatusCode::NOT_FOUND {
            Err(DispatcherError::NotFound {
                resource: format!("payment {}", payment_id),
            })
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Billing service error: {} - {}", status, error_text);
            Err(DispatcherError::ServiceError {
                service: "billing".to_string(),
                message: format!("{}: {}", status, error_text),
            })
        }
    }
}

