use billing::*;

use dotenv::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::{info, error, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use infrastructure::{
    PostgresPaymentRepository,
    MockQRCodeGenerator,
};
use application::use_cases::{
    CreatePaymentUseCase,
    GetPaymentUseCase,
    GetUserPaymentsUseCase,
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
    
    info!("Starting Billing Service...");
    
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| {
            error!("DATABASE_URL environment variable is not set");
            anyhow::anyhow!("DATABASE_URL must be set")
        })?;
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3004".to_string())
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
    let payments_table_exists: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'payments'
        )
        "#
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(false);

    if !payments_table_exists {
        warn!("Table 'payments' does not exist. Please run migrations first:");
        warn!("  cargo run --package billing --bin migrate");
        warn!("  or: sqlx migrate run");
        return Err(anyhow::anyhow!("Database schema not initialized. Run migrations first."));
    }
    
    info!("Database schema ready");

    // Инициализируем репозиторий и сервисы
    info!("Initializing repository and services...");
    let payment_repository = PostgresPaymentRepository::new(pool);
    let qr_generator = MockQRCodeGenerator::new();
    
    // Создаем use cases
    info!("Initializing use cases...");
    let create_payment_use_case = CreatePaymentUseCase::new(
        payment_repository.clone(),
        qr_generator.clone(),
    );
    let get_payment_use_case = GetPaymentUseCase::new(payment_repository.clone());
    let get_user_payments_use_case = GetUserPaymentsUseCase::new(payment_repository);

    // Создаем состояние приложения
    let app_state = AppState {
        create_payment_use_case: std::sync::Arc::new(create_payment_use_case),
        get_payment_use_case: std::sync::Arc::new(get_payment_use_case),
        get_user_payments_use_case: std::sync::Arc::new(get_user_payments_use_case),
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
    
    info!("Billing Service started successfully");
    info!("Server running on http://{}", addr);
    
    axum::serve(listener, app).await
        .map_err(|e| {
            error!("Server error: {}", e);
            e
        })?;
    
    Ok(())
}
