use uuid::Uuid;
use chrono::Utc;
use crate::domain::{
    errors::TripError,
    interfaces::TripRepository,
    models::TripStatus,
};

pub struct ActivateTripUseCase<R> 
where
    R: TripRepository,
{
    repository: R,
}

impl<R> ActivateTripUseCase<R>
where
    R: TripRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, trip_id: Uuid) -> Result<(), TripError> {
        let mut trip = self.repository.find_by_id(trip_id).await?
            .ok_or(TripError::TripNotFound)?;

        // Можно активировать только зарезервированную поездку
        if trip.status != TripStatus::Reserved {
            return Err(TripError::InvalidStatusTransition {
                from: trip.status.as_str().to_string(),
                to: TripStatus::Active.as_str().to_string(),
            });
        }

        // Обновляем статус и время начала
        trip.status = TripStatus::Active;
        trip.started_at = Some(Utc::now());

        self.repository.update(trip_id, &trip).await?;
        Ok(())
    }
}

