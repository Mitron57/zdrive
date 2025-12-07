use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    errors::TripError,
    models::Trip,
};

#[async_trait]
pub trait TripRepository {
    async fn create(&self, trip: &Trip) -> Result<(), TripError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Trip>, TripError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Trip>, TripError>;
    async fn find_all(&self) -> Result<Vec<Trip>, TripError>;
    async fn find_active_by_user_id(&self, user_id: Uuid) -> Result<Option<Trip>, TripError>;
    async fn find_active_by_car_id(&self, car_id: Uuid) -> Result<Option<Trip>, TripError>;
    async fn update(&self, id: Uuid, trip: &Trip) -> Result<(), TripError>;
}

