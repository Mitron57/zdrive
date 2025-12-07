use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub license_id: String,
    // driving_experience contains count of years
    pub driving_experience: u32,
    // five-point scale 
    pub rating: f64,
    pub email: String,
    pub password_hash: String,
}

pub struct CreateUserRequest {
    pub license_id: String,
    pub driving_experience: u32,
    pub rating: f64,
    pub email: String,
    pub password: String,
}

pub struct UpdateUserRequest {
    pub license_id: Option<String>,
    pub driving_experience: Option<u32>,
    pub rating: Option<f64>,
    pub email: Option<String>,
}

pub struct AuthRequest {
    pub email: String,
    pub password: String,
}
