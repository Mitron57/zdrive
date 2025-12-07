use uuid::Uuid;
use std::sync::Arc;
use crate::domain::{
    errors::DispatcherError,
    interfaces::TripsServiceClient,
};

pub struct CancelTripScenario<TC> 
where
    TC: TripsServiceClient + Send + Sync + 'static,
{
    trips_client: Arc<TC>,
}

impl<TC> CancelTripScenario<TC>
where
    TC: TripsServiceClient + Send + Sync + 'static,
{
    pub fn new(trips_client: Arc<TC>) -> Self {
        Self { trips_client }
    }

    pub async fn execute(&self, trip_id: Uuid) -> Result<(), DispatcherError> {
        self.trips_client.cancel_trip(trip_id).await
    }
}

