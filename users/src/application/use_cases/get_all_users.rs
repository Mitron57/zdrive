use crate::domain::{
    errors::UserError,
    interfaces::UserRepository,
    models::User,
};

pub struct GetAllUsersUseCase<R> 
where
    R: UserRepository,
{
    repository: R,
}

impl<R> GetAllUsersUseCase<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<User>, UserError> {
        self.repository.find_all().await
    }
}

