use crate::domain::{
    errors::TelematicsError,
    interfaces::RedisRepository,
    models::SensorData,
};

pub struct GetSensorDataUseCase<R> 
where
    R: RedisRepository,
{
    repository: R,
}

impl<R> GetSensorDataUseCase<R>
where
    R: RedisRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, vin: Option<&str>) -> Result<Vec<(String, SensorData)>, TelematicsError> {
        match vin {
            Some(v) => {
                // Получаем данные для конкретной машины
                match self.repository.get_sensor_data(v).await? {
                    Some(data) => Ok(vec![(v.to_string(), data)]),
                    None => Ok(vec![]),
                }
            }
            None => {
                // Получаем все данные
                self.repository.get_all_sensor_data().await
            }
        }
    }

    pub async fn execute_by_license_plate(&self, license_plate: &str) -> Result<Option<(String, SensorData)>, TelematicsError> {
        self.repository.get_sensor_data_by_license_plate(license_plate).await
    }
}

