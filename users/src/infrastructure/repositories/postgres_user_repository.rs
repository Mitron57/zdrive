use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::domain::{
    errors::UserError,
    interfaces::UserRepository,
    models::User,
};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Clone for PostgresUserRepository {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> Result<(), UserError> {
        sqlx::query(
            r#"
            INSERT INTO users (id, license_id, driving_experience, rating, email, password_hash)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(user.id)
        .bind(&user.license_id)
        .bind(user.driving_experience as i32)
        .bind(user.rating)
        .bind(&user.email)
        .bind(&user.password_hash)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError> {
        let row = sqlx::query(
            r#"
            SELECT id, license_id, driving_experience, rating, email, password_hash
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            license_id: r.get("license_id"),
            driving_experience: r.get::<i32, _>("driving_experience") as u32,
            rating: r.get("rating"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
        let row = sqlx::query(
            r#"
            SELECT id, license_id, driving_experience, rating, email, password_hash
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            license_id: r.get("license_id"),
            driving_experience: r.get::<i32, _>("driving_experience") as u32,
            rating: r.get("rating"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
        }))
    }

    async fn find_all(&self) -> Result<Vec<User>, UserError> {
        let rows = sqlx::query(
            r#"
            SELECT id, license_id, driving_experience, rating, email, password_hash
            FROM users
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| User {
            id: r.get("id"),
            license_id: r.get("license_id"),
            driving_experience: r.get::<i32, _>("driving_experience") as u32,
            rating: r.get("rating"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
        }).collect())
    }

    async fn update(&self, id: Uuid, user: &User) -> Result<(), UserError> {
        sqlx::query(
            r#"
            UPDATE users
            SET license_id = $2, driving_experience = $3, rating = $4, email = $5, password_hash = $6
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(&user.license_id)
        .bind(user.driving_experience as i32)
        .bind(user.rating)
        .bind(&user.email)
        .bind(&user.password_hash)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), UserError> {
        sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

