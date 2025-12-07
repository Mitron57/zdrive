use async_trait::async_trait;
use crate::domain::{
    errors::TelematicsError,
    models::SensorData,
};

#[async_trait]
pub trait RedisRepository {
    async fn save_sensor_data(&self, vin: &str, sensor_data: &SensorData) -> Result<(), TelematicsError>;
    async fn get_sensor_data(&self, vin: &str) -> Result<Option<SensorData>, TelematicsError>;
    async fn get_all_sensor_data(&self) -> Result<Vec<(String, SensorData)>, TelematicsError>;
    async fn get_sensor_data_by_license_plate(&self, license_plate: &str) -> Result<Option<(String, SensorData)>, TelematicsError>;
}

