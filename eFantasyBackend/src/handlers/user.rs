use rocket::State;
use crate::AppState;
use crate::models::user::NewUser;
use crate::models::User;
use rocket::serde::json::Json;
use rocket::http::Status;
// use rocket::request::FromParam;

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
pub async fn register(state: &State<AppState>, new_user: Json<NewUser>) -> Result<Json<User>, Status> {
    match crate::db::user::create_user(&state.db, new_user.into_inner()).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError)
    }
}

/// Retrieve a user by their ID
/// 
/// This function handles GET requests to retrieve a user by their ID.
/// 
/// Parameters:
/// - state: A reference to the application state, which includes the database connection.
/// - id: The ID of the user to retrieve.
/// 
/// Returns:
/// - Ok(Json<User>): If the user is successfully retrieved, returns the user data as JSON.
/// - Err(Status::InternalServerError): If there's an error during the retrieval.
#[get("/user/<id>")]
pub async fn get_user(state: &State<AppState>, id: i64) -> Result<Json<User>, Status> {
    match crate::db::user::get_user_by_id(&state.db, id).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::NotFound)
    }
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




