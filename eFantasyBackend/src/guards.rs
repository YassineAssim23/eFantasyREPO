use rocket::request::{FromRequest, Outcome};
use rocket::http::Status;
use rocket::Request;
use crate::auth;
/// Guard for authenticated routes
pub struct AuthGuard {
    pub user_id: i64,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("AuthGuard: Checking for Authorization header");

        if let Some(auth_header) = request.headers().get_one("Authorization") {
            let token = auth_header
                .trim_start_matches("Bearer ")
                .trim()
                .trim_matches('"');  // This line removes surrounding quotes
            
            println!("AuthGuard: Token received: {}", token);

            match auth::validate_token(token) {
                Ok(user_id) => {
                    println!("AuthGuard: Token validated successfully for user_id: {}", user_id);
                    Outcome::Success(AuthGuard { user_id })
                },
                Err(e) => {
                    println!("AuthGuard: Token validation failed: {:?}", e);
                    Outcome::Error((Status::Unauthorized, ()))
                }
            }
        } else {
            println!("AuthGuard: No Authorization header found");
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

/// Guard for routes that require no authentication
pub struct NoAuthGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for NoAuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            let token = auth_header.trim_start_matches("Bearer ")
                                    .trim()
                                    .trim_matches('"');
            match crate::auth::validate_token(token) {
                Ok(_) => Outcome::Error((Status::Forbidden, ())),
                Err(_) => Outcome::Success(NoAuthGuard),
            }
        } else {
            Outcome::Success(NoAuthGuard)
        }
    }
}