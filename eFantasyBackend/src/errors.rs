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

/// Represents possible errors that can occur during league operations
#[derive(Error, Debug)]
pub enum LeagueError {
    /// The requested league was not found in the database
    #[error("League not found")]
    NotFound,
    /// An error occurred while interacting with the database
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    /// The user is already a participant in the league
    #[error("User is already in the league")]
    AlreadyJoined,
    /// The league has reached its maximum number of participants
    #[error("League is full")]
    LeagueFull,
     /// The user is already a participant in the league
     #[error("Cannot leave. Season has already begun.")]
     DraftAlreadyStarted,
     /// The league has reached its maximum number of participants
     #[error("User not in league")]
     NotInLeague,
     #[error("Cannot leave league: you are the last member")]
    LastMember,
    #[error("User is not authorized to perform this action")]
    NotAuthorized,
    #[error("Cannot add new participants through settings update")]
    CannotAddParticipants,
    #[error("Cannot remove all participants from the league")]
    NoParticipantsLeft,
    #[error("League is public")]
    LeagueIsPublic,
    #[error("Invitation is not pending")]
    InvitationNotPending,
    #[error("Invitation not found")]
    InvitationNotFound,
}

/// Implement Responder for LeagueError to allow it to be returned directly from route handlers
impl<'r> rocket::response::Responder<'r, 'static> for LeagueError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let (status, error_message) = match self {
            LeagueError::NotFound => (Status::NotFound, "League not found"),
            LeagueError::DatabaseError(_) => (Status::InternalServerError, "Database error"),
            LeagueError::AlreadyJoined => (Status::BadRequest, "User is already in the league"),
            LeagueError::LeagueFull => (Status::BadRequest, "League is full"),
            LeagueError::DraftAlreadyStarted => (Status::BadRequest, "Cannot leave. Season has already begun."),
            LeagueError::NotInLeague => (Status::BadRequest, "User not in league."),
            LeagueError::LastMember => (Status::BadRequest, "Cannot leave league: you are the last member"),
            LeagueError::NotAuthorized => (Status::Forbidden, "User is not authorized to perform this action"),
            LeagueError::CannotAddParticipants => (Status::BadRequest, "Cannot add new users without valid invitation"),
            LeagueError::NoParticipantsLeft => (Status::BadRequest, "Cannot remove all participants from the league. Please delete the league to remove all participants."),
            LeagueError::LeagueIsPublic => (Status::BadRequest, "League is public"),
            LeagueError::InvitationNotPending => (Status::BadRequest, "Invitation is not pending"),
            LeagueError::InvitationNotFound => (Status::BadRequest, "Invitation not found"),
        };
        // Return a custom error response
        status::Custom(status, Json(json!({
            "error": error_message
        }))).respond_to(request)
    }
}