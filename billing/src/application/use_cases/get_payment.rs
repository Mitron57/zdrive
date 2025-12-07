use uuid::Uuid;
use crate::domain::{
    errors::PaymentError,
    interfaces::PaymentRepository,
    models::Payment,
};

pub struct GetPaymentUseCase<R> 
where
    R: PaymentRepository,
{
    repository: R,
}

impl<R> GetPaymentUseCase<R>
where
    R: PaymentRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, payment_id: Uuid) -> Result<Payment, PaymentError> {
        self.repository.find_by_id(payment_id).await?
            .ok_or(PaymentError::PaymentNotFound)
    }
}

