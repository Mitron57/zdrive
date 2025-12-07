// Используем модули из lib.rs
use cars::*;

use dotenv::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::{info, error, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use infrastructure::{
    PostgresCarRepository,
    PostgresTariffRepository,
};
use application::use_cases::{
    CreateCarUseCase,
    GetCarUseCase,
    UpdateCarUseCase,
    DeleteCarUseCase,
    ListCarsUseCase,
    CreateTariffUseCase,
    GetTariffUseCase,
    UpdateTariffUseCase,
    ListTariffsUseCase,
};
use presentation::{create_router, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Инициализируем логирование
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();
    
    info!("Starting Cars Service...");
    
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| {
            error!("DATABASE_URL environment variable is not set");
            anyhow::anyhow!("DATABASE_URL must be set")
        })?;
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .map_err(|e| {
            error!("Invalid PORT value: {}", e);
            anyhow::anyhow!("PORT must be a valid number")
        })?;

    info!("Connecting to database...");
    let pool = PgPool::connect(&database_url).await
        .map_err(|e| {
            error!("Failed to connect to database: {}", e);
            e
        })?;
    info!("Database connection established");
    
    // Проверяем, что миграции были выполнены
    let cars_table_exists: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'cars'
        )
        "#
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(false);

    let tariffs_table_exists: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'tariffs'
        )
        "#
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(false);

    if !cars_table_exists || !tariffs_table_exists {
        warn!("Tables 'cars' or 'tariffs' do not exist. Please run migrations first:");
        warn!("  cargo run --package cars --bin migrate");
        warn!("  or: sqlx migrate run");
        return Err(anyhow::anyhow!("Database schema not initialized. Run migrations first."));
    }
    
    info!("Database schema ready");

    // Инициализируем репозитории
    info!("Initializing repositories...");
    let car_repository = PostgresCarRepository::new(pool.clone());
    let tariff_repository = PostgresTariffRepository::new(pool);
    
    // Создаем use cases
    info!("Initializing use cases...");
    let create_car_use_case = CreateCarUseCase::new(
        car_repository.clone(),
        tariff_repository.clone(),
    );
    
    let get_car_use_case = GetCarUseCase::new(car_repository.clone());
    let update_car_use_case = UpdateCarUseCase::new(
        car_repository.clone(),
        tariff_repository.clone(),
    );
    let delete_car_use_case = DeleteCarUseCase::new(car_repository.clone());
    let list_cars_use_case = ListCarsUseCase::new(car_repository);

    let create_tariff_use_case = CreateTariffUseCase::new(tariff_repository.clone());
    let get_tariff_use_case = GetTariffUseCase::new(tariff_repository.clone());
    let update_tariff_use_case = UpdateTariffUseCase::new(tariff_repository.clone());
    let list_tariffs_use_case = ListTariffsUseCase::new(tariff_repository);

    // Создаем состояние приложения
    let app_state = AppState {
        create_car_use_case: std::sync::Arc::new(create_car_use_case),
        get_car_use_case: std::sync::Arc::new(get_car_use_case),
        update_car_use_case: std::sync::Arc::new(update_car_use_case),
        delete_car_use_case: std::sync::Arc::new(delete_car_use_case),
        list_cars_use_case: std::sync::Arc::new(list_cars_use_case),
        create_tariff_use_case: std::sync::Arc::new(create_tariff_use_case),
        get_tariff_use_case: std::sync::Arc::new(get_tariff_use_case),
        update_tariff_use_case: std::sync::Arc::new(update_tariff_use_case),
        list_tariffs_use_case: std::sync::Arc::new(list_tariffs_use_case),
    };

    // Создаем роутер
    let app = create_router(app_state);

    // Запускаем сервер
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await
        .map_err(|e| {
            error!("Failed to bind to {}: {}", addr, e);
            e
        })?;
    
    info!("Cars Service started successfully");
    info!("Server running on http://{}", addr);
    
    axum::serve(listener, app).await
        .map_err(|e| {
            error!("Server error: {}", e);
            e
        })?;
    
    Ok(())
}
