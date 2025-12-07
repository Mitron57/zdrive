use std::sync::Arc;
use crate::{
    application::use_cases::{RegisterUserUseCase, AuthenticateUserUseCase, UpdateUserUseCase, GetUserUseCase, GetAllUsersUseCase},
    domain::interfaces::{UserRepository, PasswordHasher, TokenGenerator},
};

pub struct AppState<R, H, T>
where
    R: UserRepository + Send + Sync + 'static,
    H: PasswordHasher + Send + Sync + 'static,
    T: TokenGenerator + Send + Sync + 'static,
{
    pub register_use_case: Arc<RegisterUserUseCase<R, H>>,
    pub auth_use_case: Arc<AuthenticateUserUseCase<R, H, T>>,
    pub update_use_case: Arc<UpdateUserUseCase<R>>,
    pub get_use_case: Arc<GetUserUseCase<R>>,
    pub get_all_users_use_case: Arc<GetAllUsersUseCase<R>>,
}

impl<R, H, T> Clone for AppState<R, H, T>
where
    R: UserRepository + Send + Sync + 'static,
    H: PasswordHasher + Send + Sync + 'static,
    T: TokenGenerator + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            register_use_case: Arc::clone(&self.register_use_case),
            auth_use_case: Arc::clone(&self.auth_use_case),
            update_use_case: Arc::clone(&self.update_use_case),
            get_use_case: Arc::clone(&self.get_use_case),
            get_all_users_use_case: Arc::clone(&self.get_all_users_use_case),
        }
    }
}

