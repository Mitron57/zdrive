use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, warn, error};
use crate::presentation::app_state::AppState;
use crate::domain::errors::PaymentError;

#[derive(Deserialize)]
pub struct CreatePaymentRequest {
    pub trip_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct CreatePaymentResponse {
    pub payment_id: Uuid,
    pub qr_code_url: String,
}

#[derive(Serialize)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub trip_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub status: String,
    pub bank_reference: Option<String>,
    pub qr_code_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<crate::domain::models::Payment> for PaymentResponse {
    fn from(payment: crate::domain::models::Payment) -> Self {
        Self {
            id: payment.id,
            trip_id: payment.trip_id,
            user_id: payment.user_id,
            amount: payment.amount,
            status: payment.status.as_str().to_string(),
            bank_reference: payment.bank_reference,
            qr_code_url: payment.qr_code_url,
            created_at: payment.created_at,
            paid_at: payment.paid_at,
        }
    }
}

pub async fn create_payment_handler<R, Q>(
    State(state): State<AppState<R, Q>>,
    Json(request): Json<CreatePaymentRequest>,
) -> Result<Json<CreatePaymentResponse>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::PaymentRepository + Send + Sync + 'static,
    Q: crate::domain::interfaces::QRCodeGenerator + Send + Sync + 'static,
{
    info!("Creating payment for trip {} by user {}", request.trip_id, request.user_id);
    
    let create_request = crate::domain::models::CreatePaymentRequest {
        trip_id: request.trip_id,
        user_id: request.user_id,
        amount: request.amount,
    };

    match state.create_payment_use_case.execute(create_request).await {
        Ok(payment_id) => {
            // Получаем созданный платеж для получения QR-кода
            match state.get_payment_use_case.execute(payment_id).await {
                Ok(payment) => {
                    info!("Payment created successfully: {}", payment_id);
                    Ok(Json(CreatePaymentResponse {
                        payment_id,
                        qr_code_url: payment.qr_code_url.unwrap_or_default(),
                    }))
                }
                Err(e) => {
                    error!("Error getting created payment: {:?}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": "Internal server error"})),
                    ))
                }
            }
        }
        Err(PaymentError::InvalidAmount { amount }) => {
            warn!("Invalid payment amount: {}", amount);
            Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": format!("Invalid payment amount: {}", amount)})),
            ))
        }
        Err(PaymentError::PaymentAlreadyProcessed) => {
            warn!("Payment already exists for trip {}", request.trip_id);
            Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({"error": "Payment already exists for this trip"})),
            ))
        }
        Err(e) => {
            error!("Error creating payment: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_payment_handler<R, Q>(
    State(state): State<AppState<R, Q>>,
    Path(payment_id): Path<Uuid>,
) -> Result<Json<PaymentResponse>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::PaymentRepository + Send + Sync + 'static,
    Q: crate::domain::interfaces::QRCodeGenerator + Send + Sync + 'static,
{
    info!("Getting payment: {}", payment_id);
    match state.get_payment_use_case.execute(payment_id).await {
        Ok(payment) => {
            info!("Payment retrieved successfully: {}", payment_id);
            Ok(Json(payment.into()))
        }
        Err(PaymentError::PaymentNotFound) => {
            warn!("Payment not found: {}", payment_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "Payment not found"})),
            ))
        }
        Err(e) => {
            error!("Error getting payment {}: {:?}", payment_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

pub async fn get_user_payments_handler<R, Q>(
    State(state): State<AppState<R, Q>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<PaymentResponse>>, (StatusCode, Json<serde_json::Value>)>
where
    R: crate::domain::interfaces::PaymentRepository + Send + Sync + 'static,
    Q: crate::domain::interfaces::QRCodeGenerator + Send + Sync + 'static,
{
    info!("Getting payments for user: {}", user_id);
    match state.get_user_payments_use_case.execute(user_id).await {
        Ok(payments) => {
            info!("Payments retrieved successfully: {} payments", payments.len());
            Ok(Json(payments.into_iter().map(|p| p.into()).collect()))
        }
        Err(e) => {
            error!("Error getting payments for user {}: {:?}", user_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Internal server error"})),
            ))
        }
    }
}

