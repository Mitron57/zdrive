use uuid::Uuid;
use crate::domain::{
    errors::CarError,
    interfaces::CarRepository,
    models::Car,
};

pub struct GetCarUseCase<R> 
where
    R: CarRepository,
{
    repository: R,
}

impl<R> GetCarUseCase<R>
where
    R: CarRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, car_id: Uuid) -> Result<Car, CarError> {
        self.repository.find_by_id(car_id).await?
            .ok_or(CarError::CarNotFound)
    }
}

