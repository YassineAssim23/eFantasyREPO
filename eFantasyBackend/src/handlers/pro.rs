use rocket::State;
use crate::AppState;
use crate::models::pro::ProPlayer;
use rocket::serde::json::Json;
use rocket::http::Status;

/// Retrieve a pro player by their ID
/// 
/// This function handles GET requests to retrieve a pro player by their ID.
/// 
/// Parameters:
/// - state: A reference to the application state, which includes the database connection.
/// - id: The ID of the pro to retrieve.
/// 
/// Returns:
/// - Ok(Json<User>): If the pro is successfully retrieved, returns the pro players data as JSON.
/// - Err(Status::InternalServerError): If there's an error during the retrieval.
#[get("/pro/<id>")]
pub async fn get_pro_player(state: &State<AppState>, id: String) -> Result<Json<ProPlayer>, Status> {
    match crate::db::pro::get_pro_player_by_id(&state.mongo_db, id).await {
        Ok(pro) => Ok(Json(pro)),
        Err(_) => Err(Status::NotFound)
    }
}   