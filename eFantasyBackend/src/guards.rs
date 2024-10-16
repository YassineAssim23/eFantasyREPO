use rocket::request::{FromRequest, Outcome};
use rocket::http::Status;
use rocket::Request;
pub struct AuthGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(token) = request.headers().get_one("Authorization") {
            match crate::auth::validate_token(token) {
                Ok(_) => Outcome::Success(AuthGuard),
                Err(_) => Outcome::Error((Status::Unauthorized, ())),
            }
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

pub struct GuestGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuestGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if request.headers().get_one("Authorization").is_none() {
            Outcome::Success(GuestGuard)
        } else {
            Outcome::Error((Status::Forbidden, ()))
        }
    }
}


pub struct NoAuthGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for NoAuthGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("NoAuthGuard: Checking authorization");
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            println!("NoAuthGuard: Authorization header found");
            let token = auth_header.trim_start_matches("Bearer ")
                                    .trim()
                                    .trim_matches('"');
            match crate::auth::validate_token(token) {
                Ok(_) => {
                    println!("NoAuthGuard: Valid token, denying access");
                    Outcome::Error((Status::Forbidden, ()))
                },
                Err(e) => {
                    println!("NoAuthGuard: Invalid token: {:?}, allowing access", e);
                    Outcome::Success(NoAuthGuard)
                },
            }
        } else {
            println!("NoAuthGuard: No token found, allowing access");
            Outcome::Success(NoAuthGuard)
        }
    }
}