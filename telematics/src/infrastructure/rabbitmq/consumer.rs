use async_trait::async_trait;
use lapin::{
    options::BasicConsumeOptions,
    types::FieldTable,
    Channel, Connection, ConnectionProperties,
    message::Delivery,
};
use futures_util::StreamExt;
use tracing::{info, error, warn};
use crate::domain::{
    interfaces::RabbitMQConsumer,
    models::{SensorData, SensorDataMessage},
};

pub struct RabbitMQConsumerImpl {
    channel: Channel,
}

impl RabbitMQConsumerImpl {
    pub async fn new(amqp_url: &str) -> Result<Self, anyhow::Error> {
        let connection = Connection::connect(amqp_url, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        
        Ok(Self { channel })
    }

    pub async fn setup_exchange_and_queue(&self, topic: &str) -> Result<(), anyhow::Error> {
        // Создаем exchange для сенсорных данных
        self.channel
            .exchange_declare(
                "telematics_sensors",
                lapin::ExchangeKind::Topic,
                lapin::options::ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        // Создаем очередь для каждого топика
        let queue_name = format!("sensors_{}", topic);
        self.channel
            .queue_declare(
                &queue_name,
                lapin::options::QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        // Биндим очередь к exchange с routing key = topic
        self.channel
            .queue_bind(
                &queue_name,
                "telematics_sensors",
                topic,
                lapin::options::QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!("Exchange and queue setup completed for topic: {}", topic);
        Ok(())
    }

    pub async fn consume_from_topic<F>(
        &self,
        topic: &str,
        callback: F,
    ) -> Result<(), anyhow::Error>
    where
        F: Fn(SensorData) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), anyhow::Error>> + Send>> + Send + Sync + 'static,
    {
        let queue_name = format!("sensors_{}", topic);
        
        let mut consumer = self.channel
            .basic_consume(
                &queue_name,
                "telematics_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!("Started consuming from queue: {}", queue_name);

        while let Some(delivery_result) = consumer.next().await {
            let delivery: Delivery = match delivery_result {
                Ok(d) => d,
                Err(e) => {
                    error!("Error receiving delivery: {}", e);
                    continue;
                }
            };
            
            match std::str::from_utf8(&delivery.data) {
                Ok(json_str) => {
                    match serde_json::from_str::<SensorDataMessage>(json_str) {
                        Ok(msg) => {
                            match SensorData::try_from(msg) {
                                Ok(sensor_data) => {
                                    if let Err(e) = callback(sensor_data).await {
                                        error!("Error processing sensor data: {}", e);
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to convert sensor data message: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to deserialize sensor data: {}", e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Invalid UTF-8 in message: {}", e);
                }
            }

            // Подтверждаем обработку сообщения
            if let Err(e) = delivery.ack(lapin::options::BasicAckOptions::default()).await {
                error!("Failed to ack delivery: {}", e);
            }
        }

        Ok(())
    }
}

#[async_trait]
impl RabbitMQConsumer for RabbitMQConsumerImpl {
    async fn consume_sensor_data<F>(&self, topic: &str, callback: F) -> Result<(), anyhow::Error>
    where
        F: Fn(SensorData) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), anyhow::Error>> + Send>> + Send + Sync + 'static,
    {
        let topic = topic.to_string();
        self.consume_from_topic(&topic, callback).await
    }
}

