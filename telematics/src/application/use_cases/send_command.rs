use uuid::Uuid;
use chrono::Utc;
use crate::domain::{
    errors::TelematicsError,
    interfaces::RabbitMQPublisher,
    models::{Command, CommandType},
};

pub struct SendCommandUseCase<P> 
where
    P: RabbitMQPublisher,
{
    publisher: P,
}

impl<P> SendCommandUseCase<P>
where
    P: RabbitMQPublisher,
{
    pub fn new(publisher: P) -> Self {
        Self { publisher }
    }

    pub async fn execute(&self, car_id: Uuid, command_type: CommandType) -> Result<Uuid, TelematicsError> {
        let command = Command {
            id: Uuid::new_v4(),
            car_id,
            command_type,
            timestamp: Utc::now(),
        };

        // Топик формируется по номеру машины (car_id)
        let topic = format!("car_{}", car_id);
        
        self.publisher.publish_command(&command, &topic).await?;
        Ok(command.id)
    }
}

