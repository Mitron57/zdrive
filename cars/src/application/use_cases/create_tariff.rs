use uuid::Uuid;
use crate::domain::{
    errors::CarError,
    interfaces::TariffRepository,
    models::{Tariff, CreateTariffRequest},
};

pub struct CreateTariffUseCase<R> 
where
    R: TariffRepository,
{
    repository: R,
}

impl<R> CreateTariffUseCase<R>
where
    R: TariffRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, request: CreateTariffRequest) -> Result<Uuid, CarError> {
        let tariff = Tariff {
            id: Uuid::new_v4(),
            price_per_minute: request.price_per_minute,
            minimal_rating: request.minimal_rating,
            minimal_experience: request.minimal_experience,
        };

        self.repository.create(&tariff).await?;
        Ok(tariff.id)
    }
}

