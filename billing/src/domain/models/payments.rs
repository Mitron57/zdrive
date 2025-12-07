use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub trip_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub status: PaymentStatus,
    pub bank_reference: Option<String>,
    pub qr_code_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,    // Ожидает оплаты
    Paid,       // Оплачено
    Failed,     // Ошибка оплаты
    Cancelled,  // Отменено
}

impl PaymentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentStatus::Pending => "pending",
            PaymentStatus::Paid => "paid",
            PaymentStatus::Failed => "failed",
            PaymentStatus::Cancelled => "cancelled",
        }
    }
}

impl std::str::FromStr for PaymentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(PaymentStatus::Pending),
            "paid" => Ok(PaymentStatus::Paid),
            "failed" => Ok(PaymentStatus::Failed),
            "cancelled" => Ok(PaymentStatus::Cancelled),
            _ => Err(format!("Invalid payment status: {}", s)),
        }
    }
}

#[derive(Deserialize)]
pub struct CreatePaymentRequest {
    pub trip_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
}

#[derive(Deserialize)]
pub struct UpdatePaymentRequest {
    pub status: Option<String>,
    pub bank_reference: Option<String>,
}

