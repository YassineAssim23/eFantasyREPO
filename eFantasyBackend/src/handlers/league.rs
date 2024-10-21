use rocket::State;
use rocket::serde::json::Json;
use rocket::http::Status;
use crate::AppState;
use crate::models::league::{League, NewLeague};
use crate::models::league::{NewLeagueInvitation, LeagueInvitation};
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
pub async fn join_league(state: &State<AppState>, league_id: i64, auth: AuthGuard) -> Result<Json<League>, Status> {
    match crate::db::league::join_league(&state.db, league_id, auth.user_id).await {
        Ok(league) => Ok(Json(league)),
        Err(e) => match e {
            LeagueError::NotFound => Err(Status::NotFound),
            LeagueError::NotAuthorized => Err(Status::Forbidden),
            _ => Err(Status::InternalServerError),
        },
    }
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
    let updated_league = crate::db::league::leave_league(&state.db, league_id, auth.user_id).await?;
    Ok(Json(updated_league))
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

/// Handler for creating a new league invitation
///
/// # Parameters
/// - `state`: The shared application state
/// - `new_invitation`: The data for the new invitation, provided in the request body
/// - `auth`: The authenticated user information
///
/// # Returns
/// - `Result<Json<LeagueInvitation>, LeagueError>`: The created LeagueInvitation as JSON if successful, or a LeagueError if the operation fails
#[post("/leagues/invite", data = "<new_invitation>")]
pub async fn create_league_invitation(
    state: &State<AppState>,
    new_invitation: Json<NewLeagueInvitation>,
    auth: AuthGuard
) -> Result<Json<LeagueInvitation>, LeagueError> {
    let invitation = crate::db::league::create_league_invitation(
        &state.db,
        new_invitation.league_id,
        new_invitation.invitee_id,
        auth.user_id
    ).await?;
    Ok(Json(invitation))
}

/// Handler for accepting a league invitation
///
/// # Parameters
/// - `state`: The shared application state
/// - `invitation_id`: The ID of the invitation to accept
/// - `auth`: The authenticated user information
///
/// # Returns
/// - `Result<Status, LeagueError>`: 200 OK if successful, or a LeagueError if the operation fails
#[post("/leagues/invitations/<invitation_id>/accept")]
pub async fn accept_league_invitation(
    state: &State<AppState>,
    invitation_id: i64,
    auth: AuthGuard
) -> Result<Json<League>, LeagueError> {
    let updated_league = crate::db::league::accept_league_invitation(&state.db, invitation_id, auth.user_id).await?;
    Ok(Json(updated_league))
}

/// Handler for declining a league invitation
///
/// # Parameters
/// - `state`: The shared application state
/// - `invitation_id`: The ID of the invitation to decline
/// - `auth`: The authenticated user information
///
/// # Returns
/// - `Result<Status, LeagueError>`: 200 OK if successful, or a LeagueError if the operation fails
#[post("/leagues/invitations/<invitation_id>/decline")]
pub async fn decline_league_invitation(
    state: &State<AppState>,
    invitation_id: i64,
    auth: AuthGuard
) -> Result<Status, LeagueError> {
    match crate::db::league::decline_league_invitation(&state.db, invitation_id, auth.user_id).await {
        Ok(_) => Ok(Status::Ok),
        Err(LeagueError::NotFound) => Err(LeagueError::InvitationNotFound),
        Err(e) => Err(e),
    }
}

/// Handler for retrieving pending league invitations for the authenticated user
///
/// # Parameters
/// - `state`: The shared application state
/// - `auth`: The authenticated user information
///
/// # Returns
/// - `Result<Json<Vec<LeagueInvitation>>, LeagueError>`: A vector of pending LeagueInvitations as JSON if successful, or a LeagueError if the operation fails
#[get("/leagues/invitations/pending")]
pub async fn get_pending_league_invitations(state: &State<AppState>, auth: AuthGuard) -> Result<Json<Vec<LeagueInvitation>>, LeagueError> {
    let invitations = crate::db::league::get_pending_league_invitations(&state.db, auth.user_id).await?;
    Ok(Json(invitations))
}

/// Handler for retrieving all leagues a user is a member of
///
/// # Parameters
/// - `state`: The shared application state
/// - `auth`: The authenticated user information
///
/// # Returns
/// - `Result<Json<Vec<League>>, LeagueError>`: A vector of Leagues as JSON if successful, or a LeagueError if the operation fails
#[get("/leagues/my")]
pub async fn get_my_leagues(state: &State<AppState>, auth: AuthGuard) -> Result<Json<Vec<League>>, LeagueError> {
    let leagues = crate::db::league::get_user_leagues(&state.db, auth.user_id).await?;
    Ok(Json(leagues))
}