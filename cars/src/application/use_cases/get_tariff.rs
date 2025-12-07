use uuid::Uuid;
use crate::domain::{
    errors::CarError,
    interfaces::TariffRepository,
    models::Tariff,
};

pub struct GetTariffUseCase<R> 
where
    R: TariffRepository,
{
    repository: R,
}

impl<R> GetTariffUseCase<R>
where
    R: TariffRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, tariff_id: Uuid) -> Result<Tariff, CarError> {
        self.repository.find_by_id(tariff_id).await?
            .ok_or(CarError::TariffNotFound)
    }
}

