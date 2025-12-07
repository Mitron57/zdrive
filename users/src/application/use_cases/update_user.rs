use uuid::Uuid;
use crate::domain::{
    errors::UserError,
    interfaces::UserRepository,
    models::UpdateUserRequest,
};

pub struct UpdateUserUseCase<R> 
where
    R: UserRepository,
{
    repository: R,
}

impl<R> UpdateUserUseCase<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: Uuid, request: UpdateUserRequest) -> Result<(), UserError> {
        // Получаем текущего пользователя
        let mut user = self.repository.find_by_id(user_id).await?
            .ok_or(UserError::NotFound)?;

        // Обновляем поля, если они предоставлены
        if let Some(license_id) = request.license_id {
            user.license_id = license_id;
        }
        if let Some(driving_experience) = request.driving_experience {
            user.driving_experience = driving_experience;
        }
        if let Some(rating) = request.rating {
            user.rating = rating;
        }
        if let Some(email) = request.email {
            // Проверяем, не занят ли email другим пользователем
            if let Some(existing_user) = self.repository.find_by_email(&email).await? {
                if existing_user.id != user_id {
                    return Err(UserError::AlreadyExists { email });
                }
            }
            user.email = email;
        }

        self.repository.update(user_id, &user).await?;
        Ok(())
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
        emails: Arc<Mutex<HashMap<String, Uuid>>>,
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
            emails.insert(user.email.clone(), user.id);
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError> {
            let users = self.users.lock().await;
            Ok(users.get(&id).cloned())
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError> {
            let users = self.users.lock().await;
            let emails = self.emails.lock().await;
            if let Some(&user_id) = emails.get(email) {
                Ok(users.get(&user_id).cloned())
            } else {
                Ok(None)
            }
        }

        async fn update(&self, id: Uuid, user: &User) -> Result<(), UserError> {
            let mut users = self.users.lock().await;
            let mut emails = self.emails.lock().await;
            if let Some(old_user) = users.get(&id) {
                emails.remove(&old_user.email);
            }
            users.insert(id, user.clone());
            emails.insert(user.email.clone(), id);
            Ok(())
        }

        async fn delete(&self, _id: Uuid) -> Result<(), UserError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_update_user_success() {
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

        let use_case = UpdateUserUseCase::new(repository);
        let request = UpdateUserRequest {
            license_id: Some("DL789012".to_string()),
            driving_experience: Some(6),
            rating: Some(4.7),
            email: None,
        };

        let result = use_case.execute(user.id, request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_user_not_found() {
        let repository = MockUserRepository::new();
        let use_case = UpdateUserUseCase::new(repository);
        let request = UpdateUserRequest {
            license_id: None,
            driving_experience: None,
            rating: None,
            email: None,
        };

        let result = use_case.execute(Uuid::new_v4(), request).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            UserError::NotFound => {}
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_update_user_email_conflict() {
        let repository = MockUserRepository::new();
        let user1 = User {
            id: Uuid::new_v4(),
            license_id: "DL123456".to_string(),
            driving_experience: 5,
            rating: 4.5,
            email: "user1@example.com".to_string(),
            password_hash: "hash".to_string(),
        };
        let user2 = User {
            id: Uuid::new_v4(),
            license_id: "DL789012".to_string(),
            driving_experience: 3,
            rating: 4.0,
            email: "user2@example.com".to_string(),
            password_hash: "hash".to_string(),
        };
        repository.create(&user1).await.unwrap();
        repository.create(&user2).await.unwrap();

        let use_case = UpdateUserUseCase::new(repository);
        let request = UpdateUserRequest {
            license_id: None,
            driving_experience: None,
            rating: None,
            email: Some("user2@example.com".to_string()),
        };

        let result = use_case.execute(user1.id, request).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            UserError::AlreadyExists { .. } => {}
            _ => panic!("Expected AlreadyExists error"),
        }
    }
}

