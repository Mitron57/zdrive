use std::sync::Arc;
use crate::{
    application::use_cases::{
        SendCommandUseCase, GetSensorDataUseCase,
    },
    domain::interfaces::{RabbitMQPublisher, RedisRepository},
};

pub struct AppState<P, R>
where
    P: RabbitMQPublisher + Send + Sync + 'static,
    R: RedisRepository + Send + Sync + 'static,
{
    pub send_command_use_case: Arc<SendCommandUseCase<P>>,
    pub get_sensor_data_use_case: Arc<GetSensorDataUseCase<R>>,
}

impl<P, R> Clone for AppState<P, R>
where
    P: RabbitMQPublisher + Send + Sync + 'static,
    R: RedisRepository + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            send_command_use_case: Arc::clone(&self.send_command_use_case),
            get_sensor_data_use_case: Arc::clone(&self.get_sensor_data_use_case),
        }
    }
}

