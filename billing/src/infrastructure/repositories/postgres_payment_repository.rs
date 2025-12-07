use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::domain::{
    errors::PaymentError,
    interfaces::PaymentRepository,
    models::{Payment, PaymentStatus},
};

pub struct PostgresPaymentRepository {
    pool: PgPool,
}

impl PostgresPaymentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Clone for PostgresPaymentRepository {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

fn payment_status_from_str(s: &str) -> PaymentStatus {
    match s {
        "pending" => PaymentStatus::Pending,
        "paid" => PaymentStatus::Paid,
        "failed" => PaymentStatus::Failed,
        "cancelled" => PaymentStatus::Cancelled,
        _ => PaymentStatus::Pending,
    }
}

#[async_trait]
impl PaymentRepository for PostgresPaymentRepository {
    async fn create(&self, payment: &Payment) -> Result<(), PaymentError> {
        sqlx::query(
            r#"
            INSERT INTO payments (id, trip_id, user_id, amount, status, bank_reference, qr_code_url, created_at, paid_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(payment.id)
        .bind(payment.trip_id)
        .bind(payment.user_id)
        .bind(payment.amount)
        .bind(payment.status.as_str())
        .bind(&payment.bank_reference)
        .bind(&payment.qr_code_url)
        .bind(payment.created_at)
        .bind(payment.paid_at)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Payment>, PaymentError> {
        let row = sqlx::query(
            r#"
            SELECT id, trip_id, user_id, amount, status, bank_reference, qr_code_url, created_at, paid_at
            FROM payments
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Payment {
            id: r.get("id"),
            trip_id: r.get("trip_id"),
            user_id: r.get("user_id"),
            amount: r.get("amount"),
            status: payment_status_from_str(r.get::<String, _>("status").as_str()),
            bank_reference: r.get("bank_reference"),
            qr_code_url: r.get("qr_code_url"),
            created_at: r.get("created_at"),
            paid_at: r.get("paid_at"),
        }))
    }

    async fn find_by_trip_id(&self, trip_id: Uuid) -> Result<Option<Payment>, PaymentError> {
        let row = sqlx::query(
            r#"
            SELECT id, trip_id, user_id, amount, status, bank_reference, qr_code_url, created_at, paid_at
            FROM payments
            WHERE trip_id = $1
            LIMIT 1
            "#,
        )
        .bind(trip_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Payment {
            id: r.get("id"),
            trip_id: r.get("trip_id"),
            user_id: r.get("user_id"),
            amount: r.get("amount"),
            status: payment_status_from_str(r.get::<String, _>("status").as_str()),
            bank_reference: r.get("bank_reference"),
            qr_code_url: r.get("qr_code_url"),
            created_at: r.get("created_at"),
            paid_at: r.get("paid_at"),
        }))
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Payment>, PaymentError> {
        let rows = sqlx::query(
            r#"
            SELECT id, trip_id, user_id, amount, status, bank_reference, qr_code_url, created_at, paid_at
            FROM payments
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| Payment {
            id: r.get("id"),
            trip_id: r.get("trip_id"),
            user_id: r.get("user_id"),
            amount: r.get("amount"),
            status: payment_status_from_str(r.get::<String, _>("status").as_str()),
            bank_reference: r.get("bank_reference"),
            qr_code_url: r.get("qr_code_url"),
            created_at: r.get("created_at"),
            paid_at: r.get("paid_at"),
        }).collect())
    }

    async fn update(&self, id: Uuid, payment: &Payment) -> Result<(), PaymentError> {
        sqlx::query(
            r#"
            UPDATE payments
            SET trip_id = $2, user_id = $3, amount = $4, status = $5, bank_reference = $6, qr_code_url = $7, paid_at = $8
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(payment.trip_id)
        .bind(payment.user_id)
        .bind(payment.amount)
        .bind(payment.status.as_str())
        .bind(&payment.bank_reference)
        .bind(&payment.qr_code_url)
        .bind(payment.paid_at)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

