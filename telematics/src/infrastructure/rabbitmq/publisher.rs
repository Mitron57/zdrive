use async_trait::async_trait;
use lapin::{
    options::BasicPublishOptions,
    BasicProperties, Channel, Connection, ConnectionProperties,
};
use tracing::{info, error};
use crate::domain::{
    errors::TelematicsError,
    interfaces::RabbitMQPublisher,
    models::Command,
};

pub struct RabbitMQPublisherImpl {
    channel: Channel,
}

impl RabbitMQPublisherImpl {
    pub async fn new(amqp_url: &str) -> Result<Self, anyhow::Error> {
        let connection = Connection::connect(amqp_url, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        
        // Создаем exchange для команд
        channel
            .exchange_declare(
                "telematics_commands",
                lapin::ExchangeKind::Topic,
                lapin::options::ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                lapin::types::FieldTable::default(),
            )
            .await?;
        
        Ok(Self { channel })
    }
}

impl Clone for RabbitMQPublisherImpl {
    fn clone(&self) -> Self {
        Self {
            channel: self.channel.clone(),
        }
    }
}

#[async_trait]
impl RabbitMQPublisher for RabbitMQPublisherImpl {
    async fn publish_command(&self, command: &Command, topic: &str) -> Result<(), TelematicsError> {
        info!("Publishing command {} to topic {}", command.id, topic);
        
        // Сериализуем команду в JSON
        let payload = serde_json::to_vec(command)?;
        
        // Публикуем в exchange с routing key = topic
        // Используем topic exchange для маршрутизации по топикам
        self.channel
            .basic_publish(
                "telematics_commands", // exchange name
                topic,                // routing key (car_id)
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await
            .map_err(|e| {
                error!("Failed to publish command: {}", e);
                TelematicsError::RabbitMQPublishError(e.to_string())
            })?;
        
        info!("Command {} published successfully", command.id);
        Ok(())
    }
}

