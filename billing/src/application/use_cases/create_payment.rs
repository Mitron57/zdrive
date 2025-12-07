use uuid::Uuid;
use chrono::Utc;
use crate::domain::{
    errors::PaymentError,
    interfaces::{PaymentRepository, QRCodeGenerator},
    models::{Payment, PaymentStatus, CreatePaymentRequest},
};

pub struct CreatePaymentUseCase<R, Q> 
where
    R: PaymentRepository,
    Q: QRCodeGenerator,
{
    repository: R,
    qr_generator: Q,
}

impl<R, Q> CreatePaymentUseCase<R, Q>
where
    R: PaymentRepository,
    Q: QRCodeGenerator,
{
    pub fn new(repository: R, qr_generator: Q) -> Self {
        Self { repository, qr_generator }
    }

    pub async fn execute(&self, request: CreatePaymentRequest) -> Result<Uuid, PaymentError> {
        // Валидация суммы
        if request.amount <= 0.0 {
            return Err(PaymentError::InvalidAmount { amount: request.amount });
        }

        // Проверяем, нет ли уже платежа для этой поездки
        if let Some(_) = self.repository.find_by_trip_id(request.trip_id).await? {
            return Err(PaymentError::PaymentAlreadyProcessed);
        }

        // Создаем платеж
        let payment_id = Uuid::new_v4();
        let qr_code_url = self.qr_generator.generate_qr_code(payment_id, request.amount).await
            .map_err(|e| PaymentError::Internal(anyhow::anyhow!("Failed to generate QR code: {}", e)))?;

        let payment = Payment {
            id: payment_id,
            trip_id: request.trip_id,
            user_id: request.user_id,
            amount: request.amount,
            status: PaymentStatus::Pending,
            bank_reference: None,
            qr_code_url: Some(qr_code_url),
            created_at: Utc::now(),
            paid_at: None,
        };

        self.repository.create(&payment).await?;
        Ok(payment_id)
    }
}

