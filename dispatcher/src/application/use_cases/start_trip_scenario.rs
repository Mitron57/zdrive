use uuid::Uuid;
use std::sync::Arc;
use crate::domain::{
    errors::DispatcherError,
    interfaces::TripsServiceClient,
};

pub struct StartTripScenario<TC> 
where
    TC: TripsServiceClient + Send + Sync + 'static,
{
    trips_client: Arc<TC>,
}

impl<TC> StartTripScenario<TC>
where
    TC: TripsServiceClient + Send + Sync + 'static,
{
    pub fn new(trips_client: Arc<TC>) -> Self {
        Self { trips_client }
    }

    pub async fn execute(&self, user_id: Uuid, car_id: Uuid) -> Result<Uuid, DispatcherError> {
        self.trips_client.start_trip(user_id, car_id).await
    }
}

