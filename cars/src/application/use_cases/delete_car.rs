use uuid::Uuid;
use crate::domain::{
    errors::CarError,
    interfaces::CarRepository,
};

pub struct DeleteCarUseCase<R> 
where
    R: CarRepository,
{
    repository: R,
}

impl<R> DeleteCarUseCase<R>
where
    R: CarRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, car_id: Uuid) -> Result<(), CarError> {
        // Проверяем, существует ли машина
        self.repository.find_by_id(car_id).await?
            .ok_or(CarError::CarNotFound)?;

        self.repository.delete(car_id).await?;
        Ok(())
    }
}

