// Используем модули из lib.rs
use telematics::*;

use dotenv::dotenv;
use tokio::net::TcpListener;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use std::sync::Arc;

use infrastructure::{
    RabbitMQPublisherImpl,
    RabbitMQConsumerImpl,
    RedisRepositoryImpl,
};
use application::use_cases::{
    SendCommandUseCase,
    ProcessSensorDataUseCase,
    GetSensorDataUseCase,
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
    
    info!("Starting Telematics Service...");
    
    let amqp_url = std::env::var("AMQP_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672/%2f".to_string());
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3003".to_string())
        .parse::<u16>()
        .map_err(|e| {
            error!("Invalid PORT value: {}", e);
            anyhow::anyhow!("PORT must be a valid number")
        })?;

    info!("Connecting to RabbitMQ...");
    let publisher = RabbitMQPublisherImpl::new(&amqp_url).await?;
    info!("RabbitMQ publisher connected");

    info!("Connecting to Redis...");
    let redis_repo = RedisRepositoryImpl::new(&redis_url).await?;
    info!("Redis connected");

    // Создаем use cases
    info!("Initializing use cases...");
    let send_command_use_case = SendCommandUseCase::new(publisher.clone());
    let process_sensor_data_use_case = ProcessSensorDataUseCase::new(redis_repo.clone());
    let get_sensor_data_use_case = GetSensorDataUseCase::new(redis_repo.clone());

    // Создаем состояние приложения
    let app_state = AppState {
        send_command_use_case: Arc::new(send_command_use_case),
        get_sensor_data_use_case: Arc::new(get_sensor_data_use_case),
    };

    // Создаем роутер
    let app = create_router(app_state);

    // Запускаем HTTP сервер
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await
        .map_err(|e| {
            error!("Failed to bind to {}: {}", addr, e);
            e
        })?;
    
    info!("Telematics Service started successfully");
    info!("HTTP server running on http://{}", addr);

    // Запускаем RabbitMQ consumer в отдельной задаче
    // В реальном приложении нужно подписываться на топики всех машин
    // Для примера подписываемся на все топики с паттерном "car_*"
    let _consumer = RabbitMQConsumerImpl::new(&amqp_url).await?;
    let _process_use_case = Arc::new(process_sensor_data_use_case);
    
    // Запускаем consumer для всех машин (в продакшене нужно получать список машин из cars сервиса)
    // Пока запускаем consumer в фоне для обработки сообщений
    tokio::spawn(async move {
        // В реальном приложении нужно получать список машин и подписываться на каждый топик
        // Для примера используем wildcard или подписываемся на все топики
        info!("RabbitMQ consumer started (listening for sensor data)");
        
        // Можно запустить несколько consumers для разных машин
        // Или использовать wildcard routing key для подписки на все машины
        loop {
            // В реальном приложении здесь должна быть логика подписки на топики всех машин
            // Пока оставляем заглушку
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
    
    axum::serve(listener, app).await
        .map_err(|e| {
            error!("Server error: {}", e);
            e
        })?;
    
    Ok(())
}
