use uuid::Uuid;
use crate::domain::{
    errors::TripError,
    interfaces::TripRepository,
    models::Trip,
};

pub struct GetUserTripsUseCase<R> 
where
    R: TripRepository,
{
    repository: R,
}

impl<R> GetUserTripsUseCase<R>
where
    R: TripRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<Vec<Trip>, TripError> {
        self.repository.find_by_user_id(user_id).await
    }
}

