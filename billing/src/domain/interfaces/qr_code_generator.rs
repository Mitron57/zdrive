use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait QRCodeGenerator {
    async fn generate_qr_code(&self, payment_id: Uuid, amount: f64) -> Result<String, anyhow::Error>;
}

