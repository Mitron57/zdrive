// Используем модули из lib.rs
use users::*;

use std::time::Duration;
use dotenv::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::{info, error, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use infrastructure::{
    PostgresUserRepository,
    JwtTokenGenerator,
    BcryptPasswordHasher,
};
use application::use_cases::{
    RegisterUserUseCase,
    AuthenticateUserUseCase,
    UpdateUserUseCase,
    GetUserUseCase,
    GetAllUsersUseCase,
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
    
    info!("Starting Users Service...");
    
    // Загружаем переменные окружения
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| {
            error!("DATABASE_URL environment variable is not set");
            anyhow::anyhow!("DATABASE_URL must be set")
        })?;
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| {
            warn!("JWT_SECRET not set, using default (not recommended for production)");
            "your-secret-key".to_string()
        });
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .map_err(|e| {
            error!("Invalid PORT value: {}", e);
            anyhow::anyhow!("PORT must be a valid number")
        })?;

    info!("Connecting to database...");
    // Подключаемся к базе данных
    let pool = PgPool::connect(&database_url).await
        .map_err(|e| {
            error!("Failed to connect to database: {}", e);
            e
        })?;
    info!("Database connection established");
    
    // Проверяем, что миграции были выполнены
    // Если таблица users не существует, предупреждаем пользователя
    let table_exists: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'users'
        )
        "#
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(false);

    if !table_exists {
        warn!("Table 'users' does not exist. Please run migrations first:");
        warn!("  cargo run --package users --bin migrate");
        warn!("  or: sqlx migrate run");
        return Err(anyhow::anyhow!("Database schema not initialized. Run migrations first."));
    }
    
    info!("Database schema ready");

    // Инициализируем инфраструктурные сервисы
    info!("Initializing infrastructure services...");
    let repository = PostgresUserRepository::new(pool);
    let password_hasher = BcryptPasswordHasher::new();
    let token_generator = JwtTokenGenerator::new();
    
    // Создаем use cases
    info!("Initializing use cases...");
    let register_use_case = RegisterUserUseCase::new(
        repository.clone(),
        password_hasher.clone(),
    );
    
    let auth_use_case = AuthenticateUserUseCase::new(
        repository.clone(),
        password_hasher.clone(),
        token_generator.clone(),
        jwt_secret.clone(),
        Duration::from_secs(3600 * 24), // 24 часа
    );
    
    let update_use_case = UpdateUserUseCase::new(repository.clone());
    let get_use_case = GetUserUseCase::new(repository.clone());
    let get_all_users_use_case = GetAllUsersUseCase::new(repository);

    // Создаем состояние приложения
    let app_state = AppState {
        register_use_case: std::sync::Arc::new(register_use_case),
        auth_use_case: std::sync::Arc::new(auth_use_case),
        update_use_case: std::sync::Arc::new(update_use_case),
        get_use_case: std::sync::Arc::new(get_use_case),
        get_all_users_use_case: std::sync::Arc::new(get_all_users_use_case),
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
    
    info!("Users Service started successfully");
    info!("Server running on http://{}", addr);
    info!("OpenAPI specification available at http://{}/openapi.yaml", addr);
    
    axum::serve(listener, app).await
        .map_err(|e| {
            error!("Server error: {}", e);
            e
        })?;
    
    Ok(())
}
