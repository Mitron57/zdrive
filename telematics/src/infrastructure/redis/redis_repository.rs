use async_trait::async_trait;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};
use crate::domain::{
    errors::TelematicsError,
    interfaces::RedisRepository,
    models::SensorData,
};

pub struct RedisRepositoryImpl {
    connection: Arc<Mutex<ConnectionManager>>,
}

impl RedisRepositoryImpl {
    pub async fn new(redis_url: &str) -> Result<Self, anyhow::Error> {
        let client = redis::Client::open(redis_url)?;
        let connection = ConnectionManager::new(client).await?;
        
        Ok(Self { 
            connection: Arc::new(Mutex::new(connection)),
        })
    }
}

impl Clone for RedisRepositoryImpl {
    fn clone(&self) -> Self {
        Self {
            connection: Arc::clone(&self.connection),
        }
    }
}

impl RedisRepositoryImpl {
    // Вспомогательная функция для парсинга данных из hash
    fn parse_sensor_data_from_hash(
        data: &std::collections::HashMap<String, String>,
    ) -> Result<SensorData, TelematicsError> {
        let vin_value = data.get("vin").ok_or_else(|| TelematicsError::InvalidData("Missing vin".to_string()))?;
        let license_plate = data.get("license_plate").ok_or_else(|| TelematicsError::InvalidData("Missing license_plate".to_string()))?;
        let fuel_level: f64 = data.get("fuel_level")
            .ok_or_else(|| TelematicsError::InvalidData("Missing fuel_level".to_string()))?
            .parse()
            .map_err(|e| TelematicsError::InvalidData(format!("Invalid fuel_level: {}", e)))?;
        let latitude: f64 = data.get("location_latitude")
            .ok_or_else(|| TelematicsError::InvalidData("Missing location_latitude".to_string()))?
            .parse()
            .map_err(|e| TelematicsError::InvalidData(format!("Invalid location_latitude: {}", e)))?;
        let longitude: f64 = data.get("location_longitude")
            .ok_or_else(|| TelematicsError::InvalidData("Missing location_longitude".to_string()))?
            .parse()
            .map_err(|e| TelematicsError::InvalidData(format!("Invalid location_longitude: {}", e)))?;
        let door_status_str = data.get("door_status").ok_or_else(|| TelematicsError::InvalidData("Missing door_status".to_string()))?;
        let speed: f64 = data.get("speed")
            .ok_or_else(|| TelematicsError::InvalidData("Missing speed".to_string()))?
            .parse()
            .map_err(|e| TelematicsError::InvalidData(format!("Invalid speed: {}", e)))?;
        let temperature: f64 = data.get("temperature")
            .ok_or_else(|| TelematicsError::InvalidData("Missing temperature".to_string()))?
            .parse()
            .map_err(|e| TelematicsError::InvalidData(format!("Invalid temperature: {}", e)))?;
        let timestamp_str = data.get("timestamp").ok_or_else(|| TelematicsError::InvalidData("Missing timestamp".to_string()))?;
        
        let door_status = match door_status_str.as_str() {
            "open" => crate::domain::models::DoorStatus::Open,
            "closed" => crate::domain::models::DoorStatus::Closed,
            "locked" => crate::domain::models::DoorStatus::Locked,
            _ => return Err(TelematicsError::InvalidData(format!("Invalid door_status: {}", door_status_str))),
        };
        
        let timestamp = chrono::DateTime::parse_from_rfc3339(timestamp_str)
            .map_err(|e| TelematicsError::InvalidData(format!("Invalid timestamp: {}", e)))?
            .with_timezone(&chrono::Utc);

        Ok(SensorData {
            vin: vin_value.clone(),
            license_plate: license_plate.clone(),
            fuel_level,
            location: crate::domain::models::Location {
                latitude,
                longitude,
            },
            door_status,
            speed,
            temperature,
            timestamp,
        })
    }
}

#[async_trait]
impl RedisRepository for RedisRepositoryImpl {
    async fn save_sensor_data(&self, vin: &str, sensor_data: &SensorData) -> Result<(), TelematicsError> {
        info!("Saving sensor data for VIN: {}", vin);
        
        // Сохраняем в Redis как hash с отдельными полями
        // Структура: HSET sensors:{vin} field1 value1 field2 value2 ...
        let hash_key = format!("sensors:{}", vin);
        let mut conn = self.connection.lock().await;
        
        // Используем несколько HSET вызовов для установки всех полей
        let _: () = conn.hset(&hash_key, "vin", vin).await?;
        let _: () = conn.hset(&hash_key, "license_plate", &sensor_data.license_plate).await?;
        let _: () = conn.hset(&hash_key, "fuel_level", &sensor_data.fuel_level.to_string()).await?;
        let _: () = conn.hset(&hash_key, "location_latitude", &sensor_data.location.latitude.to_string()).await?;
        let _: () = conn.hset(&hash_key, "location_longitude", &sensor_data.location.longitude.to_string()).await?;
        let _: () = conn.hset(&hash_key, "door_status", sensor_data.door_status.as_str()).await?;
        let _: () = conn.hset(&hash_key, "speed", &sensor_data.speed.to_string()).await?;
        let _: () = conn.hset(&hash_key, "temperature", &sensor_data.temperature.to_string()).await?;
        let _: () = conn.hset(&hash_key, "timestamp", &sensor_data.timestamp.to_rfc3339()).await?;
        
        info!("Sensor data saved for VIN: {}", vin);
        Ok(())
    }

    async fn get_sensor_data(&self, vin: &str) -> Result<Option<SensorData>, TelematicsError> {
        // Получаем данные из Redis hash
        let hash_key = format!("sensors:{}", vin);
        let mut conn = self.connection.lock().await;
        
        let data: std::collections::HashMap<String, String> = conn
            .hgetall(&hash_key)
            .await?;

        if data.is_empty() {
            return Ok(None);
        }

        // Используем общую функцию парсинга
        match Self::parse_sensor_data_from_hash(&data) {
            Ok(sensor_data) => Ok(Some(sensor_data)),
            Err(e) => Err(e),
        }
    }

    async fn get_all_sensor_data(&self) -> Result<Vec<(String, SensorData)>, TelematicsError> {
        // Получаем все ключи вида sensors:*
        let mut conn = self.connection.lock().await;
        let keys: Vec<String> = conn
            .keys("sensors:*")
            .await?;

        let mut result = Vec::new();
        for key in keys {
            // Извлекаем VIN из ключа (sensors:VIN -> VIN)
            if let Some(vin) = key.strip_prefix("sensors:") {
                // Читаем данные напрямую из hash, используя уже заблокированное соединение
                let data: std::collections::HashMap<String, String> = conn
                    .hgetall(&key)
                    .await?;

                if data.is_empty() {
                    error!("No data found for VIN: {}", vin);
                    continue;
                }

                // Парсим данные из hash полей
                match Self::parse_sensor_data_from_hash(&data) {
                    Ok(sensor_data) => result.push((vin.to_string(), sensor_data)),
                    Err(e) => {
                        error!("Failed to parse sensor data for VIN {}: {:?}", vin, e);
                    }
                }
            }
        }

        Ok(result)
    }

    async fn get_sensor_data_by_license_plate(&self, license_plate: &str) -> Result<Option<(String, SensorData)>, TelematicsError> {
        // Получаем все ключи вида sensors:*
        let mut conn = self.connection.lock().await;
        let keys: Vec<String> = conn
            .keys("sensors:*")
            .await?;

        for key in keys {
            if let Some(vin) = key.strip_prefix("sensors:") {
                // Получаем license_plate из hash для быстрой проверки
                let hash_key = format!("sensors:{}", vin);
                let stored_license_plate: Option<String> = conn
                    .hget(&hash_key, "license_plate")
                    .await?;

                if let Some(stored) = stored_license_plate {
                    if stored == license_plate {
                        // Нашли совпадение, получаем полные данные
                        match self.get_sensor_data(vin).await {
                            Ok(Some(sensor_data)) => return Ok(Some((vin.to_string(), sensor_data))),
                            Ok(None) => continue,
                            Err(e) => {
                                error!("Failed to get sensor data for VIN {}: {:?}", vin, e);
                                continue;
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}

