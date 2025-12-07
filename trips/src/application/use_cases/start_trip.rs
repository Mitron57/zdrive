use uuid::Uuid;
use chrono::Utc;
use crate::domain::{
    errors::TripError,
    interfaces::TripRepository,
    models::{Trip, StartTripRequest, TripStatus},
};

pub struct StartTripUseCase<R> 
where
    R: TripRepository,
{
    repository: R,
}

impl<R> StartTripUseCase<R>
where
    R: TripRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, request: StartTripRequest) -> Result<Uuid, TripError> {
        // Проверяем, нет ли у пользователя активной поездки
        if let Some(_) = self.repository.find_active_by_user_id(request.user_id).await? {
            return Err(TripError::UserHasActiveTrip);
        }

        // Проверяем, не занята ли машина
        if let Some(_) = self.repository.find_active_by_car_id(request.car_id).await? {
            return Err(TripError::CarAlreadyInUse);
        }

        // Создаем новую поездку со статусом Reserved
        // Поездка будет активирована отдельным действием (когда пользователь реально начал поездку)
        let now = Utc::now();
        let trip = Trip {
            id: Uuid::new_v4(),
            user_id: request.user_id,
            car_id: request.car_id,
            status: TripStatus::Reserved,
            started_at: None,
            ended_at: None,
            cancelled_at: None,
            created_at: now,
        };

        self.repository.create(&trip).await?;
        Ok(trip.id)
    }
}

