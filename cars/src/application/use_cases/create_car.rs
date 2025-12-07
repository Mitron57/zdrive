use uuid::Uuid;
use crate::domain::{
    errors::CarError,
    interfaces::{CarRepository, TariffRepository},
    models::{Car, CreateCarRequest},
};

pub struct CreateCarUseCase<CR, TR> 
where
    CR: CarRepository,
    TR: TariffRepository,
{
    car_repository: CR,
    tariff_repository: TR,
}

impl<CR, TR> CreateCarUseCase<CR, TR>
where
    CR: CarRepository,
    TR: TariffRepository,
{
    pub fn new(car_repository: CR, tariff_repository: TR) -> Self {
        Self {
            car_repository,
            tariff_repository,
        }
    }

    pub async fn execute(&self, request: CreateCarRequest) -> Result<Uuid, CarError> {
        // Проверяем, существует ли тариф
        self.tariff_repository.find_by_id(request.tariff_id).await?
            .ok_or(CarError::TariffNotFound)?;

        // Проверяем, не занят ли номер
        if let Some(_) = self.car_repository.find_by_license_plate(&request.license_plate).await? {
            return Err(CarError::CarAlreadyExists {
                license_plate: request.license_plate,
            });
        }

        // Проверяем, не зарегистрирован ли уже IoT девайс
        if let Some(_) = self.car_repository.find_by_iot_serial(&request.iot_serial_number).await? {
            return Err(CarError::IotDeviceAlreadyRegistered {
                serial_number: request.iot_serial_number,
            });
        }

        // Создаем машину
        let car = Car {
            id: Uuid::new_v4(),
            model: request.model,
            license_plate: request.license_plate,
            iot_serial_number: request.iot_serial_number,
            state: request.state,
            tariff_id: request.tariff_id,
            base_price: request.base_price,
        };

        self.car_repository.create(&car).await?;
        Ok(car.id)
    }
}

