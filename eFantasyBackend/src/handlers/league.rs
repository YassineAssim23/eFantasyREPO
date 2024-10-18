use rocket::State;
use rocket::serde::json::Json;
use rocket::http::Status;
use crate::AppState;
use crate::models::league::{League, NewLeague};
use crate::errors::LeagueError;
use crate::guards::AuthGuard;
use crate::models::league::UpdateLeague;

/// Handler for creating a new league
///
/// # Parameters
/// - `state`: The shared application state
/// - `new_league`: The data for the new league, provided in the request body
/// - `auth`: The authenticated user information
///
/// # Returns
/// - `Result<Json<League>, LeagueError>`: The created League as JSON if successful, or a LeagueError if the operation fails
#[post("/leagues", data = "<new_league>")]
pub async fn create_league(state: &State<AppState>, new_league: Json<NewLeague>, auth: AuthGuard) -> Result<Json<League>, LeagueError> {
    let league = crate::db::league::create_league(&state.db, new_league.into_inner(), auth.user_id).await?;
    Ok(Json(league))
}

/// Handler for joining a league
///
/// # Parameters
/// - `state`: The shared application state
/// - `league_id`: The ID of the league to join
/// - `auth`: The authenticated user information
///
/// # Returns
/// - `Result<Json<League>, LeagueError>`: The updated League as JSON if successful, or a LeagueError if the operation fails
#[post("/leagues/<league_id>/join")]
pub async fn join_league(state: &State<AppState>, league_id: i64, auth: AuthGuard) -> Result<Json<League>, LeagueError> {
    println!("Handling join_league request: league_id={}, user_id={}", league_id, auth.user_id);
    let league = crate::db::league::join_league(&state.db, league_id, auth.user_id).await?;
    println!("Join league successful: {:?}", league);
    Ok(Json(league))
}

/// Handler for retrieving all public leagues
///
/// # Parameters
/// - `state`: The shared application state
///
/// # Returns
/// - `Result<Json<Vec<League>>, LeagueError>`: A vector of all public leagues as JSON if successful, or a LeagueError if the operation fails
#[get("/leagues/public")]
pub async fn get_public_leagues(state: &State<AppState>) -> Result<Json<Vec<League>>, LeagueError> {
    println!("Handling get_public_leagues request");
    let leagues = crate::db::league::get_public_leagues(&state.db).await?;
    println!("Returning {} public leagues", leagues.len());
    Ok(Json(leagues))
}

/// Handler for leaving a league
///
/// # Parameters
/// - `state`: The shared application state
/// - `league_id`: The ID of the league to leave
/// - `auth`: The authenticated user information
///
/// # Returns
/// - `Result<Json<League>, LeagueError>`: The updated League as JSON if successful, or a LeagueError if the operation fails
#[post("/leagues/<league_id>/leave")]
pub async fn leave_league(state: &State<AppState>, league_id: i64, auth: AuthGuard) -> Result<Json<League>, LeagueError> {
    println!("Handling leave_league request: league_id={}, user_id={}", league_id, auth.user_id);
    let league = crate::db::league::leave_league(&state.db, league_id, auth.user_id).await?;
    println!("Leave league successful: {:?}", league);
    Ok(Json(league))
}

/// Handler for deleting a league
///
/// # Parameters
/// - `state`: The shared application state
/// - `league_id`: The ID of the league to delete
/// - `auth`: The authenticated user information
///
/// # Returns
/// - `Result<Status, LeagueError>`: 204 No Content if successful, or a LeagueError if the operation fails
#[delete("/leagues/<league_id>")]
pub async fn delete_league(state: &State<AppState>, league_id: i64, auth: AuthGuard) -> Result<Status, LeagueError> {
    println!("Handling delete_league request: league_id={}, user_id={}", league_id, auth.user_id);
    crate::db::league::delete_league(&state.db, league_id, auth.user_id).await?;
    println!("Delete league successful");
    Ok(Status::NoContent)
}

/// Handler for updating league settings
///
/// # Parameters
/// - `state`: The shared application state
/// - `league_id`: The ID of the league to update
/// - `update_league`: The new settings for the league
/// - `auth`: The authenticated user information
///
/// # Returns
/// - `Result<Json<League>, LeagueError>`: The updated League as JSON if successful, or a LeagueError if the operation fails
#[put("/leagues/<league_id>", data = "<update_league>")]
pub async fn update_league_settings(state: &State<AppState>, league_id: i64, update_league: Json<UpdateLeague>, auth: AuthGuard) -> Result<Json<League>, LeagueError> {
    println!("Handling update_league_settings request: league_id={}, user_id={}", league_id, auth.user_id);
    let league = crate::db::league::update_league_settings(&state.db, league_id, auth.user_id, update_league.into_inner()).await?;
    println!("Update league settings successful: {:?}", league);
    Ok(Json(league))
}