use uuid::Uuid;
use chrono::Utc;
use crate::domain::{
    errors::TripError,
    interfaces::TripRepository,
    models::TripStatus,
};

pub struct CancelTripUseCase<R> 
where
    R: TripRepository,
{
    repository: R,
}

impl<R> CancelTripUseCase<R>
where
    R: TripRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, trip_id: Uuid) -> Result<(), TripError> {
        let mut trip = self.repository.find_by_id(trip_id).await?
            .ok_or(TripError::TripNotFound)?;

        // Можно отменять только зарезервированные или активные поездки
        if trip.status != TripStatus::Reserved && trip.status != TripStatus::Active {
            return Err(TripError::InvalidStatusTransition {
                from: trip.status.as_str().to_string(),
                to: TripStatus::Cancelled.as_str().to_string(),
            });
        }

        // Обновляем статус и время отмены
        trip.status = TripStatus::Cancelled;
        trip.cancelled_at = Some(Utc::now());

        self.repository.update(trip_id, &trip).await?;
        Ok(())
    }
}

