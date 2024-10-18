use thiserror::Error;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, status};
use jsonwebtoken;
use serde_json::json;
use rocket::serde::json::Json;

/// Custom error types for user-related operations
#[derive(Error, Debug)]
pub enum UserError {
    #[error("Username or email already exists")]
    AlreadyExists,
    #[error("User not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("JWT error: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),
}

impl<'r> Responder<'r, 'static> for UserError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let (status, message) = match self {
            UserError::AlreadyExists => (Status::Conflict, "Username or email already exists"),
            UserError::NotFound => (Status::NotFound, "User not found"),
            UserError::DatabaseError(_) => (Status::InternalServerError, "An internal error occurred"),
            UserError::InvalidCredentials => (Status::Unauthorized, "Invalid credentials"),
            UserError::JWTError(_) => (Status::InternalServerError, "An error occurred with authentication"),
        };
        status::Custom(status, message).respond_to(req)
    }
}

/// Represents errors that can occur during league operations
#[derive(Error, Debug)]
pub enum LeagueError {
    #[error("League not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
/// Implement Responder for LeagueError to allow it to be returned directly from route handlers
impl<'r> rocket::response::Responder<'r, 'static> for LeagueError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let (status, error_message) = match self {
            LeagueError::NotFound => (Status::NotFound, "League not found"),
            LeagueError::DatabaseError(_) => (Status::InternalServerError, "Database error"),
        };
        // Return a custom error response
        status::Custom(status, Json(json!({
            "error": error_message
        }))).respond_to(request)
    }
}