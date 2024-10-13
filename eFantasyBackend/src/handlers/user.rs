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
pub async fn get_user(state: &State<AppState>, id_or_name: &str) -> Result<Json<User>, Status> {
    if let Ok(id) = id_or_name.parse::<i64>() {
        crate::db::user::get_user_by_id(&state.db, id).await
    } else {
        crate::db::user::get_user_by_name(&state.db, id_or_name).await
    }.map(Json).map_err(|_| Status::NotFound)
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





