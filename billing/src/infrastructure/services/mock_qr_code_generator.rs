use async_trait::async_trait;
use uuid::Uuid;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use crate::domain::interfaces::QRCodeGenerator;

#[derive(Clone)]
pub struct MockQRCodeGenerator;

impl MockQRCodeGenerator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl QRCodeGenerator for MockQRCodeGenerator {
    async fn generate_qr_code(&self, payment_id: Uuid, amount: f64) -> Result<String, anyhow::Error> {
        // Генерируем QR-код используя внешний API, который возвращает изображение
        // Используем api.qrserver.com API для генерации QR кода
        let payment_data = format!("payment_id={}&amount={:.2}", payment_id, amount);
        
        // Используем API, который возвращает изображение QR кода
        // Формат: https://api.qrserver.com/v1/create-qr-code/?size=300x300&data=...
        let encoded_data = utf8_percent_encode(&payment_data, NON_ALPHANUMERIC).to_string();
        let qr_image_url = format!(
            "https://api.qrserver.com/v1/create-qr-code/?size=300x300&data={}",
            encoded_data
        );
        
        Ok(qr_image_url)
    }
}

