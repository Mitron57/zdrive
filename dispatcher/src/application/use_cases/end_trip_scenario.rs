use uuid::Uuid;
use std::sync::Arc;
use chrono::Utc;
use crate::domain::{
    errors::DispatcherError,
    interfaces::{TripsServiceClient, BillingServiceClient, CarsServiceClient},
};

pub struct EndTripScenario<TC, BC, CC> 
where
    TC: TripsServiceClient + Send + Sync + 'static,
    BC: BillingServiceClient + Send + Sync + 'static,
    CC: CarsServiceClient + Send + Sync + 'static,
{
    trips_client: Arc<TC>,
    billing_client: Arc<BC>,
    cars_client: Arc<CC>,
}

impl<TC, BC, CC> EndTripScenario<TC, BC, CC>
where
    TC: TripsServiceClient + Send + Sync + 'static,
    BC: BillingServiceClient + Send + Sync + 'static,
    CC: CarsServiceClient + Send + Sync + 'static,
{
    pub fn new(trips_client: Arc<TC>, billing_client: Arc<BC>, cars_client: Arc<CC>) -> Self {
        Self { trips_client, billing_client, cars_client }
    }

    pub async fn execute(&self, trip_id: Uuid) -> Result<(Uuid, Uuid, String), DispatcherError> {
        // 1. Завершаем поездку
        self.trips_client.end_trip(trip_id).await?;
        
        // 2. Получаем информацию о поездке для расчета стоимости
        let trip = self.trips_client.get_trip(trip_id).await?;
        
        // 3. Рассчитываем стоимость на основе времени поездки и тарифа
        let amount = self.calculate_trip_cost(&trip).await?;
        
        // 4. Создаем платеж
        let payment = self.billing_client.create_payment(trip_id, trip.user_id, amount).await?;
        
        Ok((
            trip_id,
            payment.id,
            payment.qr_code_url.unwrap_or_default(),
        ))
    }

    async fn calculate_trip_cost(&self, trip: &crate::domain::interfaces::TripInfo) -> Result<f64, DispatcherError> {
        // 1. Получаем данные машины
        let car = self.cars_client.get_car(trip.car_id).await?;
        
        // 2. Получаем тариф
        let tariff = self.cars_client.get_tariff(car.tariff_id).await?;
        
        // 3. Рассчитываем время поездки в минутах
        let minutes = if let (Some(started_at), Some(ended_at)) = (trip.started_at, trip.ended_at) {
            let duration = ended_at.signed_duration_since(started_at);
            duration.num_minutes().max(1) as f64 // Минимум 1 минута
        } else {
            // Если нет времени начала/окончания, используем время создания
            let now = Utc::now();
            let duration = now.signed_duration_since(trip.created_at);
            duration.num_minutes().max(1) as f64
        };
        
        // 4. Применяем формулу: (tariff.price_per_minute * minutes) + car.base_price
        let amount = (tariff.price_per_minute * minutes) + car.base_price;
        
        Ok(amount)
    }
}

