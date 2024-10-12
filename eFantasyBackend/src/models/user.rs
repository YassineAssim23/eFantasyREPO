use serde::{Deserialize, Serialize};

/// Represents a user in the system with all fields, including database-generated ID.
/// 
/// Fields:
/// - id: Unique identifier (database-generated)
/// - username: User's chosen username
/// - email: User's email address
/// - password: User's password (should be hashed in production)
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Represents a new user being created in the system.
/// 
/// Fields:
/// - username: User's chosen username
/// - email: User's email address
/// - password: User's password (should be hashed in production)
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}