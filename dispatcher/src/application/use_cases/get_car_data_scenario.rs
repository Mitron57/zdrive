use uuid::Uuid;
use std::sync::Arc;
use crate::domain::{
    errors::DispatcherError,
    interfaces::{CarsServiceClient, TelematicsServiceClient, CarInfo, LocationInfo},
    models::scenarios::{CarDataResponse, TelematicsInfo},
};

pub struct GetCarDataScenario<CC, TC> 
where
    CC: CarsServiceClient + Send + Sync + 'static,
    TC: TelematicsServiceClient + Send + Sync + 'static,
{
    cars_client: Arc<CC>,
    telematics_client: Arc<TC>,
}

impl<CC, TC> GetCarDataScenario<CC, TC>
where
    CC: CarsServiceClient + Send + Sync + 'static,
    TC: TelematicsServiceClient + Send + Sync + 'static,
{
    pub fn new(cars_client: Arc<CC>, telematics_client: Arc<TC>) -> Self {
        Self { cars_client, telematics_client }
    }

    pub async fn execute(&self, car_id: Uuid) -> Result<CarDataResponse, DispatcherError> {
        // 1. Получаем данные машины
        let car = self.cars_client.get_car(car_id).await?;
        
        // 2. Получаем тариф для машины
        let tariff = self.cars_client.get_tariff(car.tariff_id).await?;
        
        // 3. Получаем телематические данные
        // Пока используем упрощенный подход - получаем все данные и фильтруем
        let all_sensor_data = self.telematics_client.get_all_sensor_data().await?;
        let telematics = all_sensor_data
            .iter()
            .find(|_data| {
                // В реальном приложении нужно сравнивать по license_plate
                // Для MVP пока возвращаем первое совпадение или None
                true // Упрощенная логика для MVP
            })
            .map(|data| TelematicsInfo {
                fuel_level: data.fuel_level,
                location: LocationInfo {
                    latitude: data.location.latitude,
                    longitude: data.location.longitude,
                },
                door_status: data.door_status.clone(),
                speed: data.speed,
                temperature: data.temperature,
                timestamp: data.timestamp,
            });
        
        Ok(CarDataResponse {
            car: CarInfo {
                id: car.id,
                model: car.model,
                license_plate: car.license_plate,
                state: car.state,
                tariff_id: car.tariff_id,
                base_price: car.base_price,
                price_per_minute: Some(tariff.price_per_minute),
            },
            price_per_minute: tariff.price_per_minute,
            telematics,
        })
    }
}

