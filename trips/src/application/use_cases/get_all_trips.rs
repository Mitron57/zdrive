use crate::domain::{
    errors::TripError,
    interfaces::TripRepository,
    models::Trip,
};

pub struct GetAllTripsUseCase<R> 
where
    R: TripRepository,
{
    repository: R,
}

impl<R> GetAllTripsUseCase<R>
where
    R: TripRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<Trip>, TripError> {
        self.repository.find_all().await
    }
}

