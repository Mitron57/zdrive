// Используем модули из lib.rs
use trips::*;

use dotenv::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::{info, error, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use infrastructure::PostgresTripRepository;
use application::use_cases::{
    StartTripUseCase,
    ActivateTripUseCase,
    EndTripUseCase,
    CancelTripUseCase,
    GetTripUseCase,
    GetUserTripsUseCase,
    GetAllTripsUseCase,
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
    
    info!("Starting Trips Service...");
    
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| {
            error!("DATABASE_URL environment variable is not set");
            anyhow::anyhow!("DATABASE_URL must be set")
        })?;
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3002".to_string())
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
    let trips_table_exists: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'trips'
        )
        "#
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(false);

    if !trips_table_exists {
        warn!("Table 'trips' does not exist. Please run migrations first:");
        warn!("  cargo run --package trips --bin migrate");
        warn!("  or: sqlx migrate run");
        return Err(anyhow::anyhow!("Database schema not initialized. Run migrations first."));
    }
    
    info!("Database schema ready");

    // Инициализируем репозиторий
    info!("Initializing repository...");
    let trip_repository = PostgresTripRepository::new(pool);
    
    // Создаем use cases
    info!("Initializing use cases...");
    let start_trip_use_case = StartTripUseCase::new(trip_repository.clone());
    let activate_trip_use_case = ActivateTripUseCase::new(trip_repository.clone());
    let end_trip_use_case = EndTripUseCase::new(trip_repository.clone());
    let cancel_trip_use_case = CancelTripUseCase::new(trip_repository.clone());
    let get_trip_use_case = GetTripUseCase::new(trip_repository.clone());
    let get_user_trips_use_case = GetUserTripsUseCase::new(trip_repository.clone());
    let get_all_trips_use_case = GetAllTripsUseCase::new(trip_repository);

    // Создаем состояние приложения
    let app_state = AppState {
        start_trip_use_case: std::sync::Arc::new(start_trip_use_case),
        activate_trip_use_case: std::sync::Arc::new(activate_trip_use_case),
        end_trip_use_case: std::sync::Arc::new(end_trip_use_case),
        cancel_trip_use_case: std::sync::Arc::new(cancel_trip_use_case),
        get_trip_use_case: std::sync::Arc::new(get_trip_use_case),
        get_user_trips_use_case: std::sync::Arc::new(get_user_trips_use_case),
        get_all_trips_use_case: std::sync::Arc::new(get_all_trips_use_case),
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
    
    info!("Trips Service started successfully");
    info!("Server running on http://{}", addr);
    
    axum::serve(listener, app).await
        .map_err(|e| {
            error!("Server error: {}", e);
            e
        })?;
    
    Ok(())
}

