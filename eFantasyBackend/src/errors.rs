use thiserror::Error;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, status};
use jsonwebtoken;

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