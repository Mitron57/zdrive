use crate::domain::{
    errors::TelematicsError,
    interfaces::RedisRepository,
    models::SensorData,
};

pub struct ProcessSensorDataUseCase<R> 
where
    R: RedisRepository,
{
    repository: R,
}

impl<R> ProcessSensorDataUseCase<R>
where
    R: RedisRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, sensor_data: SensorData) -> Result<(), TelematicsError> {
        // Сохраняем в Redis с ключом VIN
        self.repository.save_sensor_data(&sensor_data.vin, &sensor_data).await?;
        Ok(())
    }
}

