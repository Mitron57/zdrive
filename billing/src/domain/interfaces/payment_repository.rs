use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    errors::PaymentError,
    models::Payment,
};

#[async_trait]
pub trait PaymentRepository {
    async fn create(&self, payment: &Payment) -> Result<(), PaymentError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Payment>, PaymentError>;
    async fn find_by_trip_id(&self, trip_id: Uuid) -> Result<Option<Payment>, PaymentError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Payment>, PaymentError>;
    async fn update(&self, id: Uuid, payment: &Payment) -> Result<(), PaymentError>;
}

