use std::sync::Arc;
use crate::{
    application::use_cases::{
        CreatePaymentUseCase, GetPaymentUseCase, GetUserPaymentsUseCase,
    },
    domain::interfaces::{PaymentRepository, QRCodeGenerator},
};

pub struct AppState<R, Q>
where
    R: PaymentRepository + Send + Sync + 'static,
    Q: QRCodeGenerator + Send + Sync + 'static,
{
    pub create_payment_use_case: Arc<CreatePaymentUseCase<R, Q>>,
    pub get_payment_use_case: Arc<GetPaymentUseCase<R>>,
    pub get_user_payments_use_case: Arc<GetUserPaymentsUseCase<R>>,
}

impl<R, Q> Clone for AppState<R, Q>
where
    R: PaymentRepository + Send + Sync + 'static,
    Q: QRCodeGenerator + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            create_payment_use_case: Arc::clone(&self.create_payment_use_case),
            get_payment_use_case: Arc::clone(&self.get_payment_use_case),
            get_user_payments_use_case: Arc::clone(&self.get_user_payments_use_case),
        }
    }
}

