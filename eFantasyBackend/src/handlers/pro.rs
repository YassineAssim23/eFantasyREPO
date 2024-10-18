use rocket::State;
use crate::AppState;
use crate::models::pro::ProPlayer;
use rocket::serde::json::Json;
use rocket::http::Status;

/// Handles GET requests to retrieve a pro player by their ID.
///
/// This function is the endpoint handler for the `/pro/<id>` route. It attempts to retrieve
/// a pro player from the database using the provided ID and returns the player data as JSON
/// if successful.
///
/// # Arguments
///
/// * `state` - The application state, which includes the database connection
/// * `id` - The ID of the pro player to retrieve, provided in the URL
///
/// # Returns
///
/// * `Ok(Json<ProPlayer>)` if the player is found, with a 200 OK status
/// * `Err(Status)` with an appropriate error status if the player is not found or another error occurs
#[get("/pro/<id>")]
pub async fn get_pro_player(state: &State<AppState>, id: &str) -> Result<Json<ProPlayer>, Status> {
    match crate::db::pro::get_pro_player_by_id(&state.mongo_db, id).await {
        Ok(pro) => Ok(Json(pro)),
        Err(e) => {
            eprintln!("Error in get_pro_player: {}", e);  // Log the error
            match e.as_str() {
                "Invalid ObjectId format" => Err(Status::BadRequest),
                "Pro player not found" => Err(Status::NotFound),
                _ => Err(Status::InternalServerError),
            }
        },
    }
}