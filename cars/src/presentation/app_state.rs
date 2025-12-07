use std::sync::Arc;
use crate::{
    application::use_cases::{
        CreateCarUseCase, GetCarUseCase, UpdateCarUseCase, DeleteCarUseCase, ListCarsUseCase,
        CreateTariffUseCase, GetTariffUseCase, UpdateTariffUseCase, ListTariffsUseCase,
    },
    domain::interfaces::{CarRepository, TariffRepository},
};

pub struct AppState<CR, TR>
where
    CR: CarRepository + Send + Sync + 'static,
    TR: TariffRepository + Send + Sync + 'static,
{
    pub create_car_use_case: Arc<CreateCarUseCase<CR, TR>>,
    pub get_car_use_case: Arc<GetCarUseCase<CR>>,
    pub update_car_use_case: Arc<UpdateCarUseCase<CR, TR>>,
    pub delete_car_use_case: Arc<DeleteCarUseCase<CR>>,
    pub list_cars_use_case: Arc<ListCarsUseCase<CR>>,
    pub create_tariff_use_case: Arc<CreateTariffUseCase<TR>>,
    pub get_tariff_use_case: Arc<GetTariffUseCase<TR>>,
    pub update_tariff_use_case: Arc<UpdateTariffUseCase<TR>>,
    pub list_tariffs_use_case: Arc<ListTariffsUseCase<TR>>,
}

impl<CR, TR> Clone for AppState<CR, TR>
where
    CR: CarRepository + Send + Sync + 'static,
    TR: TariffRepository + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            create_car_use_case: Arc::clone(&self.create_car_use_case),
            get_car_use_case: Arc::clone(&self.get_car_use_case),
            update_car_use_case: Arc::clone(&self.update_car_use_case),
            delete_car_use_case: Arc::clone(&self.delete_car_use_case),
            list_cars_use_case: Arc::clone(&self.list_cars_use_case),
            create_tariff_use_case: Arc::clone(&self.create_tariff_use_case),
            get_tariff_use_case: Arc::clone(&self.get_tariff_use_case),
            update_tariff_use_case: Arc::clone(&self.update_tariff_use_case),
            list_tariffs_use_case: Arc::clone(&self.list_tariffs_use_case),
        }
    }
}

