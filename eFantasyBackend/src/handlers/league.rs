use rocket::State;
use rocket::serde::json::Json;
use crate::AppState;
use crate::models::league::{League, NewLeague};
use crate::errors::LeagueError;
use crate::guards::AuthGuard;

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