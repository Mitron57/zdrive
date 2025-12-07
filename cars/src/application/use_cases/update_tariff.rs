use uuid::Uuid;
use crate::domain::{
    errors::CarError,
    interfaces::TariffRepository,
    models::UpdateTariffRequest,
};

pub struct UpdateTariffUseCase<R> 
where
    R: TariffRepository,
{
    repository: R,
}

impl<R> UpdateTariffUseCase<R>
where
    R: TariffRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, tariff_id: Uuid, request: UpdateTariffRequest) -> Result<(), CarError> {
        // Получаем текущий тариф
        let mut tariff = self.repository.find_by_id(tariff_id).await?
            .ok_or(CarError::TariffNotFound)?;

        // Обновляем поля, если они предоставлены
        if let Some(price_per_minute) = request.price_per_minute {
            tariff.price_per_minute = price_per_minute;
        }
        if let Some(minimal_rating) = request.minimal_rating {
            tariff.minimal_rating = minimal_rating;
        }
        if let Some(minimal_experience) = request.minimal_experience {
            tariff.minimal_experience = minimal_experience;
        }

        self.repository.update(tariff_id, &tariff).await?;
        Ok(())
    }
}

