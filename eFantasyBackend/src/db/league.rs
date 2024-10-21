use sqlx::PgPool;
use crate::models::league::{League, NewLeague};
use crate::models::league::{LeagueInvitation, NewLeagueInvitation};
use crate::models::league::UpdateLeague;
use crate::errors::LeagueError;
use chrono::Utc;
use std::collections::HashSet;


/// Creates a new league in the database
///
/// # Parameters
/// - `pool`: A reference to the database connection pool
/// - `new_league`: The data for the new league
/// - `admin_id`: The user ID of the league administrator
///
/// # Returns
/// - `Result<League, LeagueError>`: The created League if successful, or a LeagueError if the operation fails
///
/// # Errors
/// This function will return an error if there's a database error during league creation
pub async fn create_league(pool: &PgPool, new_league: NewLeague, admin_id: i64) -> Result<League, LeagueError> {
    sqlx::query_as!(
        League,
        r#"
        INSERT INTO leagues (name, admin_id, max_teams, is_public, draft_time, scoring_type, participants, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, ARRAY[$2]::bigint[], CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING *
        "#,
        new_league.name,
        admin_id,
        new_league.max_teams,
        new_league.is_public,
        new_league.draft_time,
        new_league.scoring_type
    )
    .fetch_one(pool)
    .await
    .map_err(|e| LeagueError::DatabaseError(e))
}


/// Attempts to add a user to a league
///
/// # Parameters
/// - `pool`: A reference to the database connection pool
/// - `league_id`: The ID of the league the user is trying to join
/// - `user_id`: The ID of the user trying to join the league
///
/// # Returns
/// - `Result<League, LeagueError>`: The updated League if successful, or a LeagueError if the operation fails
///
/// # Errors
/// This function will return an error if:
/// - The league is not found
/// - The user is already in the league
/// - The league is full
/// - There's a database error
pub async fn join_league(pool: &PgPool, league_id: i64, user_id: i64) -> Result<League, LeagueError> {
    let mut transaction = pool.begin().await.map_err(LeagueError::DatabaseError)?;

    let league = sqlx::query_as!(
        League,
        "SELECT * FROM leagues WHERE id = $1",
        league_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => LeagueError::NotFound,
        _ => LeagueError::DatabaseError(e),
    })?;

    if !league.is_public {
        let invitation_exists = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM league_invitations WHERE league_id = $1 AND invitee_id = $2 AND status = 'accepted') as exists",
            league_id,
            user_id
        )
        .fetch_one(&mut transaction)
        .await
        .map_err(LeagueError::DatabaseError)?
        .exists
        .unwrap_or(false);

        if !invitation_exists {
            return Err(LeagueError::NotAuthorized);
        }
    }

    if league.participants.contains(&user_id) {
        return Err(LeagueError::AlreadyJoined);
    }

    if league.participants.len() >= league.max_teams as usize {
        return Err(LeagueError::LeagueFull);
    }

    let updated_league = sqlx::query_as!(
        League,
        r#"
        UPDATE leagues
        SET participants = array_append(participants, $1)
        WHERE id = $2
        RETURNING *
        "#,
        user_id,
        league_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(LeagueError::DatabaseError)?;

    transaction.commit().await.map_err(LeagueError::DatabaseError)?;

    Ok(updated_league)
}

/// Retrieves all leagues from the database
///
/// # Parameters
/// - `pool`: A reference to the database connection pool
///
/// # Returns
/// - `Result<Vec<League>, LeagueError>`: A vector of all leagues if successful, or a LeagueError if the operation fails
///
/// # Errors
/// This function will return an error if there's a database error while fetching the leagues
pub async fn get_public_leagues(pool: &PgPool) -> Result<Vec<League>, LeagueError> {
    sqlx::query_as!(
        League,
        r#"
        SELECT * FROM leagues
        WHERE is_public = true
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(LeagueError::DatabaseError)
}

/// Attempts to remove a user from a league
///
/// # Parameters
/// - `pool`: A reference to the database connection pool
/// - `league_id`: The ID of the league the user is trying to leave
/// - `user_id`: The ID of the user trying to leave the league
///
/// # Returns
/// - `Result<League, LeagueError>`: The updated League if successful, or a LeagueError if the operation fails
///
/// # Errors
/// This function will return an error if:
/// - The league is not found
/// - The user is not in the league
/// - The draft has already started
/// - There's a database error
pub async fn leave_league(pool: &PgPool, league_id: i64, user_id: i64) -> Result<League, LeagueError> {
    let mut transaction = pool.begin().await.map_err(LeagueError::DatabaseError)?;

    // Check if the league exists and if the user is a participant
    let league = sqlx::query_as!(
        League,
        "SELECT * FROM leagues WHERE id = $1",
        league_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => LeagueError::NotFound,
        _ => LeagueError::DatabaseError(e),
    })?;

    if !league.participants.contains(&user_id) {
        return Err(LeagueError::NotInLeague);
    }

    // Check if the user is the last member
    if league.participants.len() == 1 {
        return Err(LeagueError::LastMember);
    }

    // Check if the draft has already started
    let now = Utc::now();
    if now > league.draft_time {
        return Err(LeagueError::DraftAlreadyStarted);
    }

    // Remove the user from the league
    let updated_league = sqlx::query_as!(
        League,
        r#"
        UPDATE leagues
        SET participants = array_remove(participants, $1)
        WHERE id = $2
        RETURNING *
        "#,
        user_id,
        league_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(LeagueError::DatabaseError)?;

    // Invalidate any existing invitations for this user to this league
    sqlx::query!(
        r#"
        UPDATE league_invitations
        SET status = 'invalidated', updated_at = CURRENT_TIMESTAMP
        WHERE league_id = $1 AND invitee_id = $2 AND status = 'accepted'
        "#,
        league_id,
        user_id
    )
    .execute(&mut transaction)
    .await
    .map_err(LeagueError::DatabaseError)?;

    // If the user was the admin, assign a new admin
    if league.admin_id == user_id {
        let new_admin_id = updated_league.participants.iter().find(|&&id| id != user_id).unwrap();
        sqlx::query!(
            r#"
            UPDATE leagues
            SET admin_id = $1
            WHERE id = $2
            "#,
            new_admin_id,
            league_id
        )
        .execute(&mut transaction)
        .await
        .map_err(LeagueError::DatabaseError)?;
    }

    transaction.commit().await.map_err(LeagueError::DatabaseError)?;

    Ok(updated_league)
}


/// Updates league settings
///
/// # Parameters
/// - `pool`: A reference to the database connection pool
/// - `league_id`: The ID of the league to update
/// - `admin_id`: The ID of the user attempting to update the league
/// - `update_league`: The new settings for the league
///
/// # Returns
/// - `Result<League, LeagueError>`: The updated League if successful, or a LeagueError if the operation fails
///
/// # Errors
/// This function will return an error if:
/// - The league is not found
/// - The user is not the admin of the league
/// - The draft has already started
/// - There's a database error
pub async fn update_league_settings(pool: &PgPool, league_id: i64, admin_id: i64, update_league: UpdateLeague) -> Result<League, LeagueError> {
    let mut transaction = pool.begin().await.map_err(LeagueError::DatabaseError)?;

    // Fetch the current league
    let current_league = sqlx::query_as!(
        League,
        "SELECT * FROM leagues WHERE id = $1",
        league_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => LeagueError::NotFound,
        _ => LeagueError::DatabaseError(e),
    })?;

    // Check if the user is the admin
    if current_league.admin_id != admin_id {
        return Err(LeagueError::NotAuthorized);
    }

    // Check if the draft has already started
    if Utc::now() > current_league.draft_time {
        return Err(LeagueError::DraftAlreadyStarted);
    }

    // Ensure we're only removing participants, not adding new ones
    let current_participants: HashSet<i64> = current_league.participants.into_iter().collect();
    let new_participants: HashSet<i64> = update_league.participants.into_iter().collect();

    if !new_participants.is_subset(&current_participants) {
        return Err(LeagueError::CannotAddParticipants);
    }

    let final_participants: Vec<i64> = new_participants.into_iter().collect();

    if final_participants.is_empty() {
        return Err(LeagueError::NoParticipantsLeft);
    }

    // Check if admin is removing themselves
    let new_admin_id = if final_participants.contains(&admin_id) {
        admin_id
    } else {
        *final_participants.first().unwrap()
    };

    // If admin is removing themselves, they can't update other settings
    let (name, max_teams, is_public, draft_time, scoring_type) = if new_admin_id != admin_id {
        (
            current_league.name,
            current_league.max_teams,
            current_league.is_public,
            current_league.draft_time,
            current_league.scoring_type,
        )
    } else {
        (
            update_league.name,
            update_league.max_teams,
            update_league.is_public,
            update_league.draft_time,
            update_league.scoring_type,
        )
    };

    // Update the league
    let updated_league = sqlx::query_as!(
        League,
        r#"
        UPDATE leagues
        SET name = $1, max_teams = $2, is_public = $3, draft_time = $4, scoring_type = $5, participants = $6, admin_id = $7
        WHERE id = $8
        RETURNING *
        "#,
        name,
        max_teams,
        is_public,
        draft_time,
        scoring_type,
        &final_participants,
        new_admin_id,
        league_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(LeagueError::DatabaseError)?;

    transaction.commit().await.map_err(LeagueError::DatabaseError)?;

    Ok(updated_league)
}


/// Deletes a league from the database
///
/// # Parameters
/// - `pool`: A reference to the database connection pool
/// - `league_id`: The ID of the league to delete
/// - `admin_id`: The ID of the user attempting to delete the league
///
/// # Returns
/// - `Result<(), LeagueError>`: Ok(()) if successful, or a LeagueError if the operation fails
///
/// # Errors
/// This function will return an error if:
/// - The league is not found
/// - The user is not the admin of the league
/// - The draft has already started
/// - There's a database error
pub async fn delete_league(pool: &PgPool, league_id: i64, admin_id: i64) -> Result<(), LeagueError> {
    let mut transaction = pool.begin().await.map_err(LeagueError::DatabaseError)?;

    // Fetch the league
    let league = sqlx::query_as!(
        League,
        "SELECT * FROM leagues WHERE id = $1",
        league_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| match e{
        sqlx::Error::RowNotFound => LeagueError::NotFound,
        _ => LeagueError::DatabaseError(e),
    })?;

    // Check if user is the admin
    if league.admin_id != admin_id{
        return Err(LeagueError::NotAuthorized);
    }

    // Check if the draft has already started
    if Utc::now() > league.draft_time {
        return Err(LeagueError::DraftAlreadyStarted);
    }

    // Delete the league
    sqlx::query!("DELETE FROM leagues WHERE id = $1", league_id)
        .execute(&mut transaction)
        .await
        .map_err(LeagueError::DatabaseError)?;
    
    transaction.commit().await.map_err(LeagueError::DatabaseError)?;

    Ok(())
}

/// Creates a new league invitation
///
/// # Parameters
/// - `pool`: A reference to the database connection pool
/// - `new_invitation`: The data for the new invitation
///
/// # Returns
/// - `Result<LeagueInvitation, LeagueError>`: The created LeagueInvitation if successful, or a LeagueError if the operation fails
///
/// # Errors
/// This function will return an error if:
/// - The league is not found
/// - The inviter is not the admin of the league
/// - The invitee is already a member of the league
/// - There's a database error
pub async fn create_league_invitation(
    pool: &PgPool,
    league_id: i64,
    invitee_id: i64,
    inviter_id: i64
) -> Result<LeagueInvitation, LeagueError> {
    let mut transaction = pool.begin().await.map_err(LeagueError::DatabaseError)?;

    // Check if the league exists and is private
    let league = sqlx::query!(
        "SELECT admin_id, is_public FROM leagues WHERE id = $1",
        league_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => LeagueError::NotFound,
        _ => LeagueError::DatabaseError(e),
    })?;

    // Check if the league is private
    if league.is_public {
        return Err(LeagueError::LeagueIsPublic);
    }

    // Check if the inviter is the admin of the league
    if league.admin_id != inviter_id {
        return Err(LeagueError::NotAuthorized);
    }

    // Check if the invitee is already a member of the league
    let is_member = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM league_participants WHERE league_id = $1 AND user_id = $2) as is_member",
        league_id,
        invitee_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(LeagueError::DatabaseError)?
    .is_member
    .unwrap_or(false);

    if is_member {
        return Err(LeagueError::AlreadyJoined);
    }

    // Create the invitation
    let invitation = sqlx::query_as!(
        LeagueInvitation,
        r#"
        INSERT INTO league_invitations (league_id, invitee_id, inviter_id, status)
        VALUES ($1, $2, $3, 'pending')
        RETURNING *
        "#,
        league_id,
        invitee_id,
        inviter_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(LeagueError::DatabaseError)?;

    transaction.commit().await.map_err(LeagueError::DatabaseError)?;

    Ok(invitation)
}

/// Accepts a league invitation
///
/// # Parameters
/// - `pool`: A reference to the database connection pool
/// - `invitation_id`: The ID of the invitation to accept
/// - `user_id`: The ID of the user accepting the invitation
///
/// # Returns
/// - `Result<(), LeagueError>`: Ok(()) if successful, or a LeagueError if the operation fails
///
/// # Errors
/// This function will return an error if:
/// - The invitation is not found
/// - The user is not the invitee
/// - The invitation has already been accepted or declined
/// - There's a database error
pub async fn accept_league_invitation(pool: &PgPool, invitation_id: i64, user_id: i64) -> Result<League, LeagueError> {
    let mut transaction = pool.begin().await.map_err(LeagueError::DatabaseError)?;

    // Fetch the invitation
    let invitation = sqlx::query!(
        "SELECT * FROM league_invitations WHERE id = $1 AND invitee_id = $2",
        invitation_id,
        user_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => LeagueError::InvitationNotFound,
        _ => LeagueError::DatabaseError(e),
    })?;

    if invitation.status != "pending" {
        return Err(LeagueError::InvitationNotPending);
    }

    // Update invitation status
    sqlx::query!(
        "UPDATE league_invitations SET status = 'accepted', updated_at = CURRENT_TIMESTAMP WHERE id = $1",
        invitation_id
    )
    .execute(&mut transaction)
    .await
    .map_err(LeagueError::DatabaseError)?;

    // Add user to league participants
    let updated_league = sqlx::query_as!(
        League,
        r#"
        UPDATE leagues
        SET participants = array_append(participants, $1)
        WHERE id = $2
        RETURNING *
        "#,
        user_id,
        invitation.league_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(LeagueError::DatabaseError)?;

    transaction.commit().await.map_err(LeagueError::DatabaseError)?;

    Ok(updated_league)
}

/// Declines a league invitation
///
/// # Parameters
/// - `pool`: A reference to the database connection pool
/// - `invitation_id`: The ID of the invitation to decline
/// - `user_id`: The ID of the user declining the invitation
///
/// # Returns
/// - `Result<(), LeagueError>`: Ok(()) if successful, or a LeagueError if the operation fails
///
/// # Errors
/// This function will return an error if:
/// - The invitation is not found
/// - The user is not the invitee
/// - The invitation has already been accepted or declined
/// - There's a database error
pub async fn decline_league_invitation(pool: &PgPool, invitation_id: i64, user_id: i64) -> Result<(), LeagueError> {
    let mut transaction = pool.begin().await.map_err(LeagueError::DatabaseError)?;

    let invitation = sqlx::query!(
        "SELECT * FROM league_invitations WHERE id = $1",
        invitation_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => LeagueError::InvitationNotFound,
        _ => LeagueError::DatabaseError(e),
    })?;

    // Check if the user is the invitee
    if invitation.invitee_id != user_id {
        return Err(LeagueError::NotAuthorized);
    }

    // Check if the invitation is still pending
    if invitation.status != "pending" {
        return Err(LeagueError::InvitationNotPending);
    }

    // Update the invitation status
    sqlx::query!(
        "UPDATE league_invitations SET status = 'declined', updated_at = CURRENT_TIMESTAMP WHERE id = $1",
        invitation_id
    )
    .execute(&mut transaction)
    .await
    .map_err(LeagueError::DatabaseError)?;

    transaction.commit().await.map_err(LeagueError::DatabaseError)?;

    Ok(())
}

/// Retrieves all pending league invitations for a user
///
/// # Parameters
/// - `pool`: A reference to the database connection pool
/// - `user_id`: The ID of the user to fetch invitations for
///
/// # Returns
/// - `Result<Vec<LeagueInvitation>, LeagueError>`: A vector of pending LeagueInvitations if successful, or a LeagueError if the operation fails
///
/// # Errors
/// This function will return an error if there's a database error while fetching the invitations
pub async fn get_pending_league_invitations(pool: &PgPool, user_id: i64) -> Result<Vec<LeagueInvitation>, LeagueError> {
    sqlx::query_as!(
        LeagueInvitation,
        r#"
        SELECT * FROM league_invitations
        WHERE invitee_id = $1 AND status = 'pending'
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(LeagueError::DatabaseError)
}

/// Retrieves all leagues a user is a member of
///
/// # Parameters
/// - `pool`: A reference to the database connection pool
/// - `user_id`: The ID of the user to fetch leagues for
///
/// # Returns
/// - `Result<Vec<League>, LeagueError>`: A vector of Leagues if successful, or a LeagueError if the operation fails
///
/// # Errors
/// This function will return an error if there's a database error while fetching the leagues
pub async fn get_user_leagues(pool: &PgPool, user_id: i64) -> Result<Vec<League>, LeagueError> {
    sqlx::query_as!(
        League,
        r#"
        SELECT * FROM leagues
        WHERE $1 = ANY(participants)
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(LeagueError::DatabaseError)
}