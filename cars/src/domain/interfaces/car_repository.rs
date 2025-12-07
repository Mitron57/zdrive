use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    errors::CarError,
    models::Car,
};

#[async_trait]
pub trait CarRepository {
    async fn create(&self, car: &Car) -> Result<(), CarError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Car>, CarError>;
    async fn find_by_license_plate(&self, license_plate: &str) -> Result<Option<Car>, CarError>;
    async fn find_by_iot_serial(&self, iot_serial: &str) -> Result<Option<Car>, CarError>;
    async fn find_by_tariff_id(&self, tariff_id: Uuid) -> Result<Vec<Car>, CarError>;
    async fn find_all(&self) -> Result<Vec<Car>, CarError>;
    async fn update(&self, id: Uuid, car: &Car) -> Result<(), CarError>;
    async fn delete(&self, id: Uuid) -> Result<(), CarError>;
}

