use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::domain::{
    errors::TripError,
    interfaces::TripRepository,
    models::{Trip, TripStatus},
};

pub struct PostgresTripRepository {
    pool: PgPool,
}

impl PostgresTripRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Clone for PostgresTripRepository {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

fn trip_status_from_str(s: &str) -> TripStatus {
    match s {
        "reserved" => TripStatus::Reserved,
        "active" => TripStatus::Active,
        "completed" => TripStatus::Completed,
        "cancelled" => TripStatus::Cancelled,
        _ => TripStatus::Reserved, // default
    }
}

#[async_trait]
impl TripRepository for PostgresTripRepository {
    async fn create(&self, trip: &Trip) -> Result<(), TripError> {
        sqlx::query(
            r#"
            INSERT INTO trips (id, user_id, car_id, status, started_at, ended_at, cancelled_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(trip.id)
        .bind(trip.user_id)
        .bind(trip.car_id)
        .bind(trip.status.as_str())
        .bind(trip.started_at)
        .bind(trip.ended_at)
        .bind(trip.cancelled_at)
        .bind(trip.created_at)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Trip>, TripError> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, car_id, status, started_at, ended_at, cancelled_at, created_at
            FROM trips
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Trip {
            id: r.get("id"),
            user_id: r.get("user_id"),
            car_id: r.get("car_id"),
            status: trip_status_from_str(r.get::<String, _>("status").as_str()),
            started_at: r.get("started_at"),
            ended_at: r.get("ended_at"),
            cancelled_at: r.get("cancelled_at"),
            created_at: r.get("created_at"),
        }))
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Trip>, TripError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, car_id, status, started_at, ended_at, cancelled_at, created_at
            FROM trips
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| Trip {
            id: r.get("id"),
            user_id: r.get("user_id"),
            car_id: r.get("car_id"),
            status: trip_status_from_str(r.get::<String, _>("status").as_str()),
            started_at: r.get("started_at"),
            ended_at: r.get("ended_at"),
            cancelled_at: r.get("cancelled_at"),
            created_at: r.get("created_at"),
        }).collect())
    }

    async fn find_all(&self) -> Result<Vec<Trip>, TripError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, car_id, status, started_at, ended_at, cancelled_at, created_at
            FROM trips
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| Trip {
            id: r.get("id"),
            user_id: r.get("user_id"),
            car_id: r.get("car_id"),
            status: trip_status_from_str(r.get::<String, _>("status").as_str()),
            started_at: r.get("started_at"),
            ended_at: r.get("ended_at"),
            cancelled_at: r.get("cancelled_at"),
            created_at: r.get("created_at"),
        }).collect())
    }

    async fn find_active_by_user_id(&self, user_id: Uuid) -> Result<Option<Trip>, TripError> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, car_id, status, started_at, ended_at, cancelled_at, created_at
            FROM trips
            WHERE user_id = $1 AND status IN ('reserved', 'active')
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Trip {
            id: r.get("id"),
            user_id: r.get("user_id"),
            car_id: r.get("car_id"),
            status: trip_status_from_str(r.get::<String, _>("status").as_str()),
            started_at: r.get("started_at"),
            ended_at: r.get("ended_at"),
            cancelled_at: r.get("cancelled_at"),
            created_at: r.get("created_at"),
        }))
    }

    async fn find_active_by_car_id(&self, car_id: Uuid) -> Result<Option<Trip>, TripError> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, car_id, status, started_at, ended_at, cancelled_at, created_at
            FROM trips
            WHERE car_id = $1 AND status IN ('reserved', 'active')
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(car_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Trip {
            id: r.get("id"),
            user_id: r.get("user_id"),
            car_id: r.get("car_id"),
            status: trip_status_from_str(r.get::<String, _>("status").as_str()),
            started_at: r.get("started_at"),
            ended_at: r.get("ended_at"),
            cancelled_at: r.get("cancelled_at"),
            created_at: r.get("created_at"),
        }))
    }

    async fn update(&self, id: Uuid, trip: &Trip) -> Result<(), TripError> {
        sqlx::query(
            r#"
            UPDATE trips
            SET user_id = $2, car_id = $3, status = $4, started_at = $5, ended_at = $6, cancelled_at = $7
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(trip.user_id)
        .bind(trip.car_id)
        .bind(trip.status.as_str())
        .bind(trip.started_at)
        .bind(trip.ended_at)
        .bind(trip.cancelled_at)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

