use crate::domain::{
    errors::CarError,
    interfaces::CarRepository,
    models::Car,
};
use uuid::Uuid;

pub struct ListCarsUseCase<R> 
where
    R: CarRepository,
{
    repository: R,
}

impl<R> ListCarsUseCase<R>
where
    R: CarRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, tariff_id: Option<Uuid>) -> Result<Vec<Car>, CarError> {
        match tariff_id {
            Some(id) => self.repository.find_by_tariff_id(id).await,
            None => self.repository.find_all().await,
        }
    }
}

