use uuid::Uuid;
use crate::domain::{
    errors::PaymentError,
    interfaces::PaymentRepository,
    models::Payment,
};

pub struct GetUserPaymentsUseCase<R> 
where
    R: PaymentRepository,
{
    repository: R,
}

impl<R> GetUserPaymentsUseCase<R>
where
    R: PaymentRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<Vec<Payment>, PaymentError> {
        self.repository.find_by_user_id(user_id).await
    }
}

