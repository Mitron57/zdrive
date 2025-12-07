use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    errors::CarError,
    models::Tariff,
};

#[async_trait]
pub trait TariffRepository {
    async fn create(&self, tariff: &Tariff) -> Result<(), CarError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tariff>, CarError>;
    async fn find_all(&self) -> Result<Vec<Tariff>, CarError>;
    async fn update(&self, id: Uuid, tariff: &Tariff) -> Result<(), CarError>;
    async fn delete(&self, id: Uuid) -> Result<(), CarError>;
}

