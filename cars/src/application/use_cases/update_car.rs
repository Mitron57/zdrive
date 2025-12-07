use uuid::Uuid;
use crate::domain::{
    errors::CarError,
    interfaces::{CarRepository, TariffRepository},
    models::UpdateCarRequest,
};

pub struct UpdateCarUseCase<CR, TR> 
where
    CR: CarRepository,
    TR: TariffRepository,
{
    car_repository: CR,
    tariff_repository: TR,
}

impl<CR, TR> UpdateCarUseCase<CR, TR>
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

    pub async fn execute(&self, car_id: Uuid, request: UpdateCarRequest) -> Result<(), CarError> {
        // Получаем текущую машину
        let mut car = self.car_repository.find_by_id(car_id).await?
            .ok_or(CarError::CarNotFound)?;

        // Обновляем поля, если они предоставлены
        if let Some(model) = request.model {
            car.model = model;
        }
        if let Some(license_plate) = request.license_plate {
            // Проверяем, не занят ли новый номер другим автомобилем
            if let Some(existing_car) = self.car_repository.find_by_license_plate(&license_plate).await? {
                if existing_car.id != car_id {
                    return Err(CarError::CarAlreadyExists { license_plate });
                }
            }
            car.license_plate = license_plate;
        }
        if let Some(iot_serial) = request.iot_serial_number {
            // Проверяем, не зарегистрирован ли уже IoT девайс
            if let Some(existing_car) = self.car_repository.find_by_iot_serial(&iot_serial).await? {
                if existing_car.id != car_id {
                    return Err(CarError::IotDeviceAlreadyRegistered { serial_number: iot_serial });
                }
            }
            car.iot_serial_number = iot_serial;
        }
        if let Some(state) = request.state {
            car.state = state;
        }
        if let Some(tariff_id) = request.tariff_id {
            // Проверяем, существует ли тариф
            self.tariff_repository.find_by_id(tariff_id).await?
                .ok_or(CarError::TariffNotFound)?;
            car.tariff_id = tariff_id;
        }
        if let Some(base_price) = request.base_price {
            car.base_price = base_price;
        }

        self.car_repository.update(car_id, &car).await?;
        Ok(())
    }
}

