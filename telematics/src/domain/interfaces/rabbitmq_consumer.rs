use async_trait::async_trait;
use crate::domain::models::SensorData;

#[async_trait]
pub trait RabbitMQConsumer {
    async fn consume_sensor_data<F>(&self, topic: &str, callback: F) -> Result<(), anyhow::Error>
    where
        F: Fn(SensorData) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), anyhow::Error>> + Send>> + Send + Sync + 'static;
}

