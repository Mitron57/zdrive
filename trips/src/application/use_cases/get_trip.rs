use uuid::Uuid;
use crate::domain::{
    errors::TripError,
    interfaces::TripRepository,
    models::Trip,
};

pub struct GetTripUseCase<R> 
where
    R: TripRepository,
{
    repository: R,
}

impl<R> GetTripUseCase<R>
where
    R: TripRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, trip_id: Uuid) -> Result<Trip, TripError> {
        self.repository.find_by_id(trip_id).await?
            .ok_or(TripError::TripNotFound)
    }
}

