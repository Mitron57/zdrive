use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::domain::{
    errors::CarError,
    interfaces::TariffRepository,
    models::Tariff,
};

pub struct PostgresTariffRepository {
    pool: PgPool,
}

impl PostgresTariffRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Clone for PostgresTariffRepository {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

#[async_trait]
impl TariffRepository for PostgresTariffRepository {
    async fn create(&self, tariff: &Tariff) -> Result<(), CarError> {
        sqlx::query(
            r#"
            INSERT INTO tariffs (id, price_per_minute, minimal_rating, minimal_experience)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(tariff.id)
        .bind(tariff.price_per_minute)
        .bind(tariff.minimal_rating)
        .bind(tariff.minimal_experience as i32)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tariff>, CarError> {
        let row = sqlx::query(
            r#"
            SELECT id, price_per_minute, minimal_rating, minimal_experience
            FROM tariffs
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Tariff {
            id: r.get("id"),
            price_per_minute: r.get("price_per_minute"),
            minimal_rating: r.get("minimal_rating"),
            minimal_experience: r.get::<i32, _>("minimal_experience") as u32,
        }))
    }

    async fn find_all(&self) -> Result<Vec<Tariff>, CarError> {
        let rows = sqlx::query(
            r#"
            SELECT id, price_per_minute, minimal_rating, minimal_experience
            FROM tariffs
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| Tariff {
            id: r.get("id"),
            price_per_minute: r.get("price_per_minute"),
            minimal_rating: r.get("minimal_rating"),
            minimal_experience: r.get::<i32, _>("minimal_experience") as u32,
        }).collect())
    }

    async fn update(&self, id: Uuid, tariff: &Tariff) -> Result<(), CarError> {
        sqlx::query(
            r#"
            UPDATE tariffs
            SET price_per_minute = $2, minimal_rating = $3, minimal_experience = $4
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(tariff.price_per_minute)
        .bind(tariff.minimal_rating)
        .bind(tariff.minimal_experience as i32)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), CarError> {
        sqlx::query(
            r#"
            DELETE FROM tariffs
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

