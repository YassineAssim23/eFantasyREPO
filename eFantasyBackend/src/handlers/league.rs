use rocket::State;
use rocket::serde::json::Json;
use crate::AppState;
use crate::models::league::{League, NewLeague};
use crate::errors::LeagueError;
use crate::guards::AuthGuard;

/// Handler for creating a new league
///
/// # Arguments
///
/// * `state` - The shared application state
/// * `new_league` - The data for the new league, provided in the request body
/// * `auth` - The authenticated user information
///
/// # Returns
///
/// Returns the created League as JSON on success, or a LeagueError on failure
#[post("/leagues", data = "<new_league>")]
pub async fn create_league(state: &State<AppState>, new_league: Json<NewLeague>, auth: AuthGuard) -> Result<Json<League>, LeagueError> {
    // Call the database function to create the league
    let league = crate::db::league::create_league(&state.db, new_league.into_inner(), auth.user_id).await?;
    // Return the created league as JSON
    Ok(Json(league))
}