use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub license_id: String,
    pub driving_experience: u32,
    pub rating: f64,
    pub email: String,
}

impl From<crate::domain::models::User> for UserResponse {
    fn from(user: crate::domain::models::User) -> Self {
        Self {
            id: user.id,
            license_id: user.license_id,
            driving_experience: user.driving_experience,
            rating: user.rating,
            email: user.email,
        }
    }
}

