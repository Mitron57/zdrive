use crate::domain::{
    errors::UserError,
    interfaces::{UserRepository, PasswordHasher, TokenGenerator},
    models::AuthRequest,
};
use std::time::Duration;

pub struct AuthenticateUserUseCase<R, H, T> 
where
    R: UserRepository,
    H: PasswordHasher,
    T: TokenGenerator,
{
    repository: R,
    password_hasher: H,
    token_generator: T,
    jwt_secret: String,
    token_ttl: Duration,
}

impl<R, H, T> AuthenticateUserUseCase<R, H, T>
where
    R: UserRepository,
    H: PasswordHasher,
    T: TokenGenerator,
{
    pub fn new(
        repository: R,
        password_hasher: H,
        token_generator: T,
        jwt_secret: String,
        token_ttl: Duration,
    ) -> Self {
        Self {
            repository,
            password_hasher,
            token_generator,
            jwt_secret,
            token_ttl,
        }
    }

    pub async fn execute(&self, request: AuthRequest) -> Result<(String, uuid::Uuid), UserError> {
        // Находим пользователя по email
        let user = self.repository.find_by_email(&request.email).await?
            .ok_or(UserError::InvalidCredentials)?;

        // Проверяем пароль
        let is_valid = self.password_hasher.verify(&request.password, &user.password_hash)
            .map_err(|e| UserError::Internal(anyhow::anyhow!("Failed to verify password: {}", e)))?;

        if !is_valid {
            return Err(UserError::InvalidCredentials);
        }

        // Генерируем токен
        let token = self.token_generator.generate_token(user.id, &self.jwt_secret, self.token_ttl)
            .map_err(|e| UserError::Internal(anyhow::anyhow!("Failed to generate token: {}", e)))?;

        Ok((token, user.id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use uuid::Uuid;
    use crate::domain::models::User;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct MockUserRepository {
        users: Arc<Mutex<HashMap<String, User>>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        async fn add_user(&self, user: User) {
            let mut users = self.users.lock().await;
            users.insert(user.email.clone(), user);
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, _user: &User) -> Result<(), UserError> {
            Ok(())
        }

        async fn find_by_id(&self, _id: Uuid) -> Result<Option<User>, UserError> {
            Ok(None)
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
            let users = self.users.lock().await;
            Ok(users.get(email).cloned())
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

    struct MockTokenGenerator;

    impl TokenGenerator for MockTokenGenerator {
        fn generate_token(&self, user_id: Uuid, _secret: &str, _ttl: Duration) -> Result<String, anyhow::Error> {
            Ok(format!("token_{}", user_id))
        }

        fn validate_token(&self, _token: &str, _secret: &str) -> Result<Uuid, anyhow::Error> {
            Ok(Uuid::new_v4())
        }
    }

    #[tokio::test]
    async fn test_authenticate_user_success() {
        let repository = MockUserRepository::new();
        let password_hasher = MockPasswordHasher;
        let token_generator = MockTokenGenerator;
        
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            license_id: "DL123456".to_string(),
            driving_experience: 5,
            rating: 4.5,
            email: "test@example.com".to_string(),
            password_hash: "hashed_password123".to_string(),
        };
        repository.add_user(user).await;

        let use_case = AuthenticateUserUseCase::new(
            repository,
            password_hasher,
            token_generator,
            "secret".to_string(),
            Duration::from_secs(3600),
        );

        let request = AuthRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = use_case.execute(request).await;
        assert!(result.is_ok());
        let (token, returned_user_id) = result.unwrap();
        assert_eq!(token, format!("token_{}", user_id));
        assert_eq!(returned_user_id, user_id);
    }

    #[tokio::test]
    async fn test_authenticate_user_not_found() {
        let repository = MockUserRepository::new();
        let password_hasher = MockPasswordHasher;
        let token_generator = MockTokenGenerator;

        let use_case = AuthenticateUserUseCase::new(
            repository,
            password_hasher,
            token_generator,
            "secret".to_string(),
            Duration::from_secs(3600),
        );

        let request = AuthRequest {
            email: "nonexistent@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = use_case.execute(request).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            UserError::InvalidCredentials => {}
            _ => panic!("Expected InvalidCredentials error"),
        }
    }

    #[tokio::test]
    async fn test_authenticate_user_invalid_password() {
        let repository = MockUserRepository::new();
        let password_hasher = MockPasswordHasher;
        let token_generator = MockTokenGenerator;
        
        let user = User {
            id: Uuid::new_v4(),
            license_id: "DL123456".to_string(),
            driving_experience: 5,
            rating: 4.5,
            email: "test@example.com".to_string(),
            password_hash: "hashed_correct_password".to_string(),
        };
        repository.add_user(user).await;

        let use_case = AuthenticateUserUseCase::new(
            repository,
            password_hasher,
            token_generator,
            "secret".to_string(),
            Duration::from_secs(3600),
        );

        let request = AuthRequest {
            email: "test@example.com".to_string(),
            password: "wrong_password".to_string(),
        };

        let result = use_case.execute(request).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            UserError::InvalidCredentials => {}
            _ => panic!("Expected InvalidCredentials error"),
        }
    }
}
