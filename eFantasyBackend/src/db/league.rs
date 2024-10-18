use sqlx::PgPool;
use crate::models::league::{League, NewLeague};
use crate::errors::LeagueError;

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