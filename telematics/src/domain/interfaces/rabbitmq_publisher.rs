use async_trait::async_trait;
use crate::domain::{
    errors::TelematicsError,
    models::Command,
};

#[async_trait]
pub trait RabbitMQPublisher {
    async fn publish_command(&self, command: &Command, topic: &str) -> Result<(), TelematicsError>;
}

