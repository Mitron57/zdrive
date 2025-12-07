use crate::domain::{
    errors::CarError,
    interfaces::TariffRepository,
    models::Tariff,
};

pub struct ListTariffsUseCase<R> 
where
    R: TariffRepository,
{
    repository: R,
}

impl<R> ListTariffsUseCase<R>
where
    R: TariffRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<Tariff>, CarError> {
        self.repository.find_all().await
    }
}

