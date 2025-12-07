use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::domain::{
    errors::CarError,
    interfaces::CarRepository,
    models::{Car, CarState},
};

pub struct PostgresCarRepository {
    pool: PgPool,
}

impl PostgresCarRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Clone for PostgresCarRepository {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

fn car_state_from_str(s: &str) -> CarState {
    match s {
        "available" => CarState::Available,
        "in_use" => CarState::InUse,
        "maintenance" => CarState::Maintenance,
        "reserved" => CarState::Reserved,
        _ => CarState::Available, // default
    }
}

#[async_trait]
impl CarRepository for PostgresCarRepository {
    async fn create(&self, car: &Car) -> Result<(), CarError> {
        sqlx::query(
            r#"
            INSERT INTO cars (id, model, license_plate, iot_serial_number, state, tariff_id, base_price)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(car.id)
        .bind(&car.model)
        .bind(&car.license_plate)
        .bind(&car.iot_serial_number)
        .bind(car.state.as_str())
        .bind(car.tariff_id)
        .bind(car.base_price)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Car>, CarError> {
        let row = sqlx::query(
            r#"
            SELECT id, model, license_plate, iot_serial_number, state, tariff_id, base_price
            FROM cars
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Car {
            id: r.get("id"),
            model: r.get("model"),
            license_plate: r.get("license_plate"),
            iot_serial_number: r.get("iot_serial_number"),
            state: car_state_from_str(r.get::<String, _>("state").as_str()),
            tariff_id: r.get("tariff_id"),
            base_price: r.get("base_price"),
        }))
    }

    async fn find_by_license_plate(&self, license_plate: &str) -> Result<Option<Car>, CarError> {
        let row = sqlx::query(
            r#"
            SELECT id, model, license_plate, iot_serial_number, state, tariff_id, base_price
            FROM cars
            WHERE license_plate = $1
            "#,
        )
        .bind(license_plate)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Car {
            id: r.get("id"),
            model: r.get("model"),
            license_plate: r.get("license_plate"),
            iot_serial_number: r.get("iot_serial_number"),
            state: car_state_from_str(r.get::<String, _>("state").as_str()),
            tariff_id: r.get("tariff_id"),
            base_price: r.get("base_price"),
        }))
    }

    async fn find_by_iot_serial(&self, iot_serial: &str) -> Result<Option<Car>, CarError> {
        let row = sqlx::query(
            r#"
            SELECT id, model, license_plate, iot_serial_number, state, tariff_id, base_price
            FROM cars
            WHERE iot_serial_number = $1
            "#,
        )
        .bind(iot_serial)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Car {
            id: r.get("id"),
            model: r.get("model"),
            license_plate: r.get("license_plate"),
            iot_serial_number: r.get("iot_serial_number"),
            state: car_state_from_str(r.get::<String, _>("state").as_str()),
            tariff_id: r.get("tariff_id"),
            base_price: r.get("base_price"),
        }))
    }

    async fn find_by_tariff_id(&self, tariff_id: Uuid) -> Result<Vec<Car>, CarError> {
        let rows = sqlx::query(
            r#"
            SELECT id, model, license_plate, iot_serial_number, state, tariff_id, base_price
            FROM cars
            WHERE tariff_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(tariff_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| Car {
            id: r.get("id"),
            model: r.get("model"),
            license_plate: r.get("license_plate"),
            iot_serial_number: r.get("iot_serial_number"),
            state: car_state_from_str(r.get::<String, _>("state").as_str()),
            tariff_id: r.get("tariff_id"),
            base_price: r.get("base_price"),
        }).collect())
    }

    async fn find_all(&self) -> Result<Vec<Car>, CarError> {
        let rows = sqlx::query(
            r#"
            SELECT id, model, license_plate, iot_serial_number, state, tariff_id, base_price
            FROM cars
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| Car {
            id: r.get("id"),
            model: r.get("model"),
            license_plate: r.get("license_plate"),
            iot_serial_number: r.get("iot_serial_number"),
            state: car_state_from_str(r.get::<String, _>("state").as_str()),
            tariff_id: r.get("tariff_id"),
            base_price: r.get("base_price"),
        }).collect())
    }

    async fn update(&self, id: Uuid, car: &Car) -> Result<(), CarError> {
        sqlx::query(
            r#"
            UPDATE cars
            SET model = $2, license_plate = $3, iot_serial_number = $4, state = $5, tariff_id = $6, base_price = $7
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(&car.model)
        .bind(&car.license_plate)
        .bind(&car.iot_serial_number)
        .bind(car.state.as_str())
        .bind(car.tariff_id)
        .bind(car.base_price)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), CarError> {
        sqlx::query(
            r#"
            DELETE FROM cars
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

