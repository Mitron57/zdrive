use std::sync::Arc;
use crate::{
    application::use_cases::{
        StartTripUseCase, ActivateTripUseCase, EndTripUseCase, CancelTripUseCase, GetTripUseCase, GetUserTripsUseCase, GetAllTripsUseCase,
    },
    domain::interfaces::TripRepository,
};

pub struct AppState<R>
where
    R: TripRepository + Send + Sync + 'static,
{
    pub start_trip_use_case: Arc<StartTripUseCase<R>>,
    pub activate_trip_use_case: Arc<ActivateTripUseCase<R>>,
    pub end_trip_use_case: Arc<EndTripUseCase<R>>,
    pub cancel_trip_use_case: Arc<CancelTripUseCase<R>>,
    pub get_trip_use_case: Arc<GetTripUseCase<R>>,
    pub get_user_trips_use_case: Arc<GetUserTripsUseCase<R>>,
    pub get_all_trips_use_case: Arc<GetAllTripsUseCase<R>>,
}

impl<R> Clone for AppState<R>
where
    R: TripRepository + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            start_trip_use_case: Arc::clone(&self.start_trip_use_case),
            activate_trip_use_case: Arc::clone(&self.activate_trip_use_case),
            end_trip_use_case: Arc::clone(&self.end_trip_use_case),
            cancel_trip_use_case: Arc::clone(&self.cancel_trip_use_case),
            get_trip_use_case: Arc::clone(&self.get_trip_use_case),
            get_user_trips_use_case: Arc::clone(&self.get_user_trips_use_case),
            get_all_trips_use_case: Arc::clone(&self.get_all_trips_use_case),
        }
    }
}

