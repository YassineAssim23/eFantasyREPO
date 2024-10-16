use rocket::State;
use crate::AppState;
use crate::models::user::NewUser;
use crate::models::User;
use crate::errors::UserError;
use rocket::serde::json::Json;
use rocket::http::Status;
use crate::models::user::LoginCredentials;
use crate::auth::verify_password;
use crate::guards::NoAuthGuard;
use crate::guards::AuthGuard;
// use rocket::request::FromParam;

#[post("/login", data = "<credentials>")]
pub async fn login(_guard: NoAuthGuard, state: &State<AppState>, credentials: Json<LoginCredentials>) -> Result<Json<String>, Status> {
    let user = crate::db::user::get_user_by_name(&state.db, &credentials.username)
        .await
        .map_err(|_| Status::Unauthorized)?;

    if verify_password(&credentials.password, &user.password) {
        match crate::auth::generate_token(user.id) {
            Ok(token) => Ok(Json(token)),
            Err(e) => {
                eprintln!("Token generation error: {}", e);
                Err(Status::InternalServerError)
            }
        }
    } else {
        Err(Status::Unauthorized)
    }
}
/// Register a new user
///
/// This function handles POST requests to create a new user.
/// 
/// Parameters:
/// - state: A reference to the application state, which includes the database connection.
/// - new_user: JSON data representing the new user to be created.
/// 
/// Returns:
/// - Ok(Json<User>): If the user is successfully created, returns the user data as JSON.
/// - Err(Status::InternalServerError): If there's an error during user creation.
#[post("/register", data = "<new_user>")]
pub async fn register(_guard: NoAuthGuard, state: &State<AppState>, new_user: Json<NewUser>) -> Result<Json<User>, UserError> {
    match crate::db::user::create_user(&state.db, new_user.into_inner()).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}

#[post("/signout")]
pub async fn sign_out(_auth: AuthGuard) -> Status {
    // We don't need to do anything server-side since we're using JWTs
    // The client will handle removing the token
    Status::Ok
}

/// Retrieve a user
///
/// This function handles GET requests to retrieve a new user based on ID or username.
/// 
/// Parameters:
/// - state: A reference to the application state, which includes the database connection.
/// - id_or_name: id or username of user that will be retrieved
/// 
/// Returns:
/// - Ok(Json<User>): If the user is successfully created, returns the user data as JSON.
/// - Err(Status::InternalServerError): If there's an error during user creation.
#[get("/user/<id_or_name>")]
pub async fn get_user(state: &State<AppState>, id_or_name: &str) -> Result<Json<User>, UserError> {
    let result = if let Ok(id) = id_or_name.parse::<i64>() {
        crate::db::user::get_user_by_id(&state.db, id).await
    } else {
        crate::db::user::get_user_by_name(&state.db, id_or_name).await
    };

    result.map(Json)
}
/// Delete a user by their ID
/// 
/// This function handles DELETE requests to delete a user by their ID.
/// 
/// Parameters:
/// - state: A reference to the application state, which includes the database connection.
/// - id: The ID of the user to delete.
/// 
/// Returns:
/// - Ok(Status::NoContent): If the user is successfully deleted.
/// - Err(Status::InternalServerError): If there's an error during the deletion.
#[delete("/user/<id>")]
pub async fn delete_user(state: &State<AppState>, id: i64) -> Status {
    match crate::db::user::delete_user(&state.db, id).await {
        Ok(true) => Status::NoContent,
        Ok(false) => Status::NotFound,
        Err(_) => Status::InternalServerError
    }
}


