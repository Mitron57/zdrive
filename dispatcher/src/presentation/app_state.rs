use std::sync::Arc;
use crate::{
    application::use_cases::{
        StartTripScenario, EndTripScenario, CancelTripScenario, GetCarDataScenario,
    },
    domain::interfaces::*,
};

pub struct AppState<UC, CC, TC, TMC, BC> 
where
    UC: UsersServiceClient + Send + Sync + 'static,
    CC: CarsServiceClient + Send + Sync + 'static,
    TC: TripsServiceClient + Send + Sync + 'static,
    TMC: TelematicsServiceClient + Send + Sync + 'static,
    BC: BillingServiceClient + Send + Sync + 'static,
{
    pub users_client: Arc<UC>,
    pub cars_client: Arc<CC>,
    pub trips_client: Arc<TC>,
    pub telematics_client: Arc<TMC>,
    pub billing_client: Arc<BC>,
    pub start_trip_scenario: Arc<StartTripScenario<TC>>,
    pub end_trip_scenario: Arc<EndTripScenario<TC, BC, CC>>,
    pub cancel_trip_scenario: Arc<CancelTripScenario<TC>>,
    pub get_car_data_scenario: Arc<GetCarDataScenario<CC, TMC>>,
}

impl<UC, CC, TC, TMC, BC> Clone for AppState<UC, CC, TC, TMC, BC>
where
    UC: UsersServiceClient + Send + Sync + 'static,
    CC: CarsServiceClient + Send + Sync + 'static,
    TC: TripsServiceClient + Send + Sync + 'static,
    TMC: TelematicsServiceClient + Send + Sync + 'static,
    BC: BillingServiceClient + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            users_client: Arc::clone(&self.users_client),
            cars_client: Arc::clone(&self.cars_client),
            trips_client: Arc::clone(&self.trips_client),
            telematics_client: Arc::clone(&self.telematics_client),
            billing_client: Arc::clone(&self.billing_client),
            start_trip_scenario: Arc::clone(&self.start_trip_scenario),
            end_trip_scenario: Arc::clone(&self.end_trip_scenario),
            cancel_trip_scenario: Arc::clone(&self.cancel_trip_scenario),
            get_car_data_scenario: Arc::clone(&self.get_car_data_scenario),
        }
    }
}

