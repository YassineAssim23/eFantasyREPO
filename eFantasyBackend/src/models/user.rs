use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a user in the system
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub wins: i32,
    pub losses: i32,
    pub ties: i32,
    pub total_points: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents the data required to create a new user
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Represents the data for completing a user's profile
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileCompletion {
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

/// Represents the data for updating a user's profile
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileUpdate {
    pub nickname: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

/// Represents the credentials for user login
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

/// Represents the statistics of a user
#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    pub wins: i32,
    pub losses: i32,
    pub ties: i32,
    pub total_points: f64,
    pub leagues_joined: i32, 
    pub teams_created: i32, 
}