use sqlx::PgPool;
use crate::models::league::{League, NewLeague};
use crate::errors::LeagueError;

/// Creates a new league in the database
///
/// # Arguments
///
/// * `pool` - The database connection pool
/// * `new_league` - The data for the new league
/// * `admin_id` - The user ID of the league administrator
///
/// # Returns
///
/// Returns the created League on success, or a LeagueError on failure
pub async fn create_league(pool: &PgPool, new_league: NewLeague, admin_id: i64) -> Result<League, LeagueError> {
    sqlx::query_as!(
        League,
        r#"
        INSERT INTO leagues (name, admin_id, max_teams, is_public, draft_time, scoring_type, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
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