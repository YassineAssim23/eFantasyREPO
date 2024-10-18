use sqlx::PgPool;
use crate::models::league::{League, NewLeague};
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
    println!("Attempting to join league: league_id={}, user_id={}", league_id, user_id);
    
    // Fetch the league
    let league = sqlx::query_as!(
        League,
        "SELECT * FROM leagues WHERE id = $1",
        league_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        println!("Error fetching league: {:?}", e);
        match e {
            sqlx::Error::RowNotFound => LeagueError::NotFound,
            _ => LeagueError::DatabaseError(e),
        }
    })?;

    println!("Current league state: {:?}", league);

    // Check if the user is already in the league
    if league.participants.contains(&user_id) {
        println!("User is already in the league");
        return Err(LeagueError::AlreadyJoined);  // Return the league without an error
    }

    // Check if the league is full
    if league.participants.len() >= league.max_teams as usize {
        println!("League is full");
        return Err(LeagueError::LeagueFull);
    }

    // If we've made it here, we can try to join the league
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
    .fetch_one(pool)
    .await
    .map_err(|e| {
        println!("Error updating league: {:?}", e);
        LeagueError::DatabaseError(e)
    })?;

    println!("Successfully joined league: {:?}", updated_league);
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
    println!("Fetching all leagues...");
    let leagues = sqlx::query_as!(
        League,
        r#"
        SELECT * FROM leagues
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        println!("Error fetching leagues: {:?}", e);
        LeagueError::DatabaseError(e)
    })?;

    println!("Found {} leagues in total", leagues.len());
    for league in &leagues {
        println!("League: id={}, name={}, is_public={}, participants={:?}", 
                 league.id, league.name, league.is_public, league.participants);
    }
    Ok(leagues)
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

    // Fetch the league
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

    // Check if the draft has already started
    if Utc::now() > league.draft_time {
        return Err(LeagueError::DraftAlreadyStarted);
    }

    // Check if the user is in the league
    if !league.participants.contains(&user_id) {
        return Err(LeagueError::NotInLeague);
    }

    let new_participants: Vec<i64> = league.participants.iter().filter(|&&id| id != user_id).cloned().collect();
    let mut new_admin_id = league.admin_id;

    // If the leaving user is the admin, assign a new admin
    if user_id == league.admin_id {
        new_admin_id = new_participants.first().cloned().ok_or(LeagueError::LastMember)?;
    }

    // Update the league
    let updated_league = sqlx::query_as!(
        League,
        r#"
        UPDATE leagues
        SET participants = $1, admin_id = $2
        WHERE id = $3
        RETURNING *
        "#,
        &new_participants,
        new_admin_id,
        league_id
    )
    .fetch_one(&mut transaction)
    .await
    .map_err(LeagueError::DatabaseError)?;

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