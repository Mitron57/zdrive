use uuid::Uuid;
use crate::domain::{
    errors::UserError,
    interfaces::UserRepository,
    models::User,
};

pub struct GetUserUseCase<R> 
where
    R: UserRepository,
{
    repository: R,
}

impl<R> GetUserUseCase<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<User, UserError> {
        self.repository.find_by_id(user_id).await?
            .ok_or(UserError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use crate::domain::models::User;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct MockUserRepository {
        users: Arc<Mutex<HashMap<Uuid, User>>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, user: &User) -> Result<(), UserError> {
            let mut users = self.users.lock().await;
            users.insert(user.id, user.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError> {
            let users = self.users.lock().await;
            Ok(users.get(&id).cloned())
        }

        async fn find_by_email(&self, _email: &str) -> Result<Option<User>, UserError> {
            Ok(None)
        }

        async fn update(&self, _id: Uuid, _user: &User) -> Result<(), UserError> {
            Ok(())
        }

        async fn delete(&self, _id: Uuid) -> Result<(), UserError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_get_user_success() {
        let repository = MockUserRepository::new();
        let user = User {
            id: Uuid::new_v4(),
            license_id: "DL123456".to_string(),
            driving_experience: 5,
            rating: 4.5,
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
        };
        repository.create(&user).await.unwrap();

        let use_case = GetUserUseCase::new(repository);
        let result = use_case.execute(user.id).await;
        
        assert!(result.is_ok());
        let retrieved_user = result.unwrap();
        assert_eq!(retrieved_user.id, user.id);
        assert_eq!(retrieved_user.email, user.email);
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let repository = MockUserRepository::new();
        let use_case = GetUserUseCase::new(repository);
        let result = use_case.execute(Uuid::new_v4()).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            UserError::NotFound => {}
            _ => panic!("Expected NotFound error"),
        }
    }
}

