use uuid::Uuid;
use crate::domain::{
    errors::UserError,
    interfaces::{UserRepository, PasswordHasher},
    models::{User, CreateUserRequest},
};

pub struct RegisterUserUseCase<R, H> 
where
    R: UserRepository,
    H: PasswordHasher,
{
    repository: R,
    password_hasher: H,
}

impl<R, H> RegisterUserUseCase<R, H>
where
    R: UserRepository,
    H: PasswordHasher,
{
    pub fn new(repository: R, password_hasher: H) -> Self {
        Self {
            repository,
            password_hasher,
        }
    }

    pub async fn execute(&self, request: CreateUserRequest) -> Result<Uuid, UserError> {
        // Проверяем, существует ли пользователь с таким email
        if let Some(_) = self.repository.find_by_email(&request.email).await? {
            return Err(UserError::AlreadyExists {
                email: request.email,
            });
        }

        // Хешируем пароль
        let password_hash = self.password_hasher.hash(&request.password)
            .map_err(|e| UserError::Internal(anyhow::anyhow!("Failed to hash password: {}", e)))?;

        // Создаем пользователя
        let user = User {
            id: Uuid::new_v4(),
            license_id: request.license_id,
            driving_experience: request.driving_experience,
            rating: request.rating,
            email: request.email,
            password_hash,
        };

        self.repository.create(&user).await?;
        Ok(user.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Mock implementations для тестирования
    struct MockUserRepository {
        users: Arc<Mutex<HashMap<Uuid, User>>>,
        emails: Arc<Mutex<HashMap<String, User>>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: Arc::new(Mutex::new(HashMap::new())),
                emails: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, user: &User) -> Result<(), UserError> {
            let mut users = self.users.lock().await;
            let mut emails = self.emails.lock().await;
            users.insert(user.id, user.clone());
            emails.insert(user.email.clone(), user.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError> {
            let users = self.users.lock().await;
            Ok(users.get(&id).cloned())
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
            let emails = self.emails.lock().await;
            Ok(emails.get(email).cloned())
        }

        async fn update(&self, _id: Uuid, _user: &User) -> Result<(), UserError> {
            Ok(())
        }

        async fn delete(&self, _id: Uuid) -> Result<(), UserError> {
            Ok(())
        }
    }

    struct MockPasswordHasher;

    impl PasswordHasher for MockPasswordHasher {
        fn hash(&self, password: &str) -> Result<String, anyhow::Error> {
            Ok(format!("hashed_{}", password))
        }

        fn verify(&self, password: &str, hash: &str) -> Result<bool, anyhow::Error> {
            Ok(hash == format!("hashed_{}", password))
        }
    }

    #[tokio::test]
    async fn test_register_user_success() {
        let repository = MockUserRepository::new();
        let password_hasher = MockPasswordHasher;
        let use_case = RegisterUserUseCase::new(repository, password_hasher);

        let request = CreateUserRequest {
            license_id: "DL123456".to_string(),
            driving_experience: 5,
            rating: 4.5,
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = use_case.execute(request).await;
        assert!(result.is_ok());
        let user_id = result.unwrap();
        assert!(!user_id.is_nil());
    }

    #[tokio::test]
    async fn test_register_user_already_exists() {
        let repository = MockUserRepository::new();
        let password_hasher = MockPasswordHasher;
        let use_case = RegisterUserUseCase::new(repository, password_hasher);

        let email = "existing@example.com".to_string();
        
        // Создаем первого пользователя
        let request1 = CreateUserRequest {
            license_id: "DL123456".to_string(),
            driving_experience: 5,
            rating: 4.5,
            email: email.clone(),
            password: "password123".to_string(),
        };
        use_case.execute(request1).await.unwrap();

        // Пытаемся создать второго с тем же email
        let request2 = CreateUserRequest {
            license_id: "DL789012".to_string(),
            driving_experience: 3,
            rating: 4.0,
            email,
            password: "password456".to_string(),
        };

        let result = use_case.execute(request2).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            UserError::AlreadyExists { .. } => {}
            _ => panic!("Expected AlreadyExists error"),
        }
    }
}
