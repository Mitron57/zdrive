use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    errors::UserError,
    models::User
};

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: &User) -> Result<(), UserError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError>;
    async fn find_all(&self) -> Result<Vec<User>, UserError>;
    async fn update(&self, id: Uuid, user: &User) -> Result<(), UserError>;
    async fn delete(&self, id: Uuid) -> Result<(), UserError>;
}