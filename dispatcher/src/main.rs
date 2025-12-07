use dispatcher::*;

use dotenv::dotenv;
use tokio::net::TcpListener;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use std::sync::Arc;

use infrastructure::{
    HttpUsersServiceClient,
    HttpCarsServiceClient,
    HttpTripsServiceClient,
    HttpTelematicsServiceClient,
    HttpBillingServiceClient,
};
use application::use_cases::{
    StartTripScenario,
    EndTripScenario,
    CancelTripScenario,
    GetCarDataScenario,
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
    
    info!("Starting Dispatcher Service (API Gateway)...");
    
    // Загружаем URL'ы микросервисов
    let users_url = std::env::var("USERS_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    let cars_url = std::env::var("CARS_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:3001".to_string());
    let trips_url = std::env::var("TRIPS_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:3002".to_string());
    let telematics_url = std::env::var("TELEMATICS_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:3003".to_string());
    let billing_url = std::env::var("BILLING_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:3004".to_string());
    
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .map_err(|e| {
            error!("Invalid PORT value: {}", e);
            anyhow::anyhow!("PORT must be a valid number")
        })?;

    info!("Service URLs:");
    info!("  Users: {}", users_url);
    info!("  Cars: {}", cars_url);
    info!("  Trips: {}", trips_url);
    info!("  Telematics: {}", telematics_url);
    info!("  Billing: {}", billing_url);

    // Инициализируем HTTP клиенты
    info!("Initializing service clients...");
    let users_client = Arc::new(HttpUsersServiceClient::new(users_url));
    let cars_client = Arc::new(HttpCarsServiceClient::new(cars_url));
    let trips_client = Arc::new(HttpTripsServiceClient::new(trips_url));
    let telematics_client = Arc::new(HttpTelematicsServiceClient::new(telematics_url));
    let billing_client = Arc::new(HttpBillingServiceClient::new(billing_url));
    
    // Создаем сценарии
    info!("Initializing scenarios...");
    let start_trip_scenario = Arc::new(StartTripScenario::new(trips_client.clone()));
    let end_trip_scenario = Arc::new(EndTripScenario::new(trips_client.clone(), billing_client.clone(), cars_client.clone()));
    let cancel_trip_scenario = Arc::new(CancelTripScenario::new(trips_client.clone()));
    let get_car_data_scenario = Arc::new(GetCarDataScenario::new(cars_client.clone(), telematics_client.clone()));

    // Создаем состояние приложения
    let app_state = AppState {
        users_client,
        cars_client,
        trips_client,
        telematics_client,
        billing_client,
        start_trip_scenario,
        end_trip_scenario,
        cancel_trip_scenario,
        get_car_data_scenario,
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
    
    info!("Dispatcher Service started successfully");
    info!("Server running on http://{}", addr);
    info!("API Gateway ready to route requests");
    
    axum::serve(listener, app).await
        .map_err(|e| {
            error!("Server error: {}", e);
            e
        })?;
    
    Ok(())
}

