use sqlx::PgPool;
use crate::models::user::{User, NewUser, UserProfileUpdate, ProfileCompletion, UserStats};
use crate::errors::UserError;

/// Creates a new user in the database
pub async fn create_user(pool: &PgPool, user: NewUser) -> Result<User, UserError> {
    // Check if user already exists
    let user_exists = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 OR email = $2) as exists",
        user.username,
        user.email,
    )
    .fetch_one(pool)
    .await
    .map_err(UserError::DatabaseError)?
    .exists
    .unwrap_or(false);

    if user_exists {
        return Err(UserError::AlreadyExists);
    }
    
    let hashed_password = crate::auth::hash_password(&user.password);
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password, created_at, updated_at)
        VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING *
        "#,
        user.username,
        user.email,
        hashed_password
    )
    .fetch_one(pool)
    .await
    .map_err(UserError::DatabaseError)
}

/// Retrieves a user by their ID
pub async fn get_user_by_id(pool: &PgPool, user_id: i64) -> Result<User, UserError> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => UserError::NotFound,
        _ => UserError::DatabaseError(e),
    })
}

/// Retrieves a user by their username
pub async fn get_user_by_name(pool: &PgPool, user_name: &str) -> Result<User, UserError> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        user_name
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => UserError::NotFound,
        _ => UserError::DatabaseError(e),
    })
}

/// Updates a user's profile
pub async fn update_user_profile(
    pool: &PgPool,
    user_id: i64,
    profile_update: UserProfileUpdate
) -> Result<User, UserError> {
    sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET 
            nickname = COALESCE($1, nickname),
            bio = COALESCE($2, bio),
            avatar_url = COALESCE($3, avatar_url),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $4
        RETURNING *
        "#,
        profile_update.nickname,
        profile_update.bio,
        profile_update.avatar_url,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => UserError::NotFound,
        _ => UserError::DatabaseError(e),
    })
}

/// Completes a user's profile
pub async fn complete_profile(pool: &PgPool, user_id: i64, profile: ProfileCompletion) -> Result<User, UserError> {
    println!("db::complete_profile: Updating profile for user_id: {}", user_id);
    sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET 
            nickname = COALESCE($1, nickname),
            bio = COALESCE($2, bio),
            avatar_url = COALESCE($3, avatar_url),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $4
        RETURNING *
        "#,
        profile.nickname,
        profile.bio,
        profile.avatar_url,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        println!("db::complete_profile: Error updating profile: {:?}", e);
        match e {
            sqlx::Error::RowNotFound => UserError::NotFound,
            _ => UserError::DatabaseError(e),
        }
    })
}

/// Deletes a user from the database
pub async fn delete_user(pool: &PgPool, user_id: i64) -> Result<bool, UserError> {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id
    )
    .execute(pool)
    .await
    .map_err(UserError::DatabaseError)?;

    Ok(result.rows_affected() > 0)
}

/// Updates a user's statistics
pub async fn update_user_stats(
    pool: &PgPool,
    user_id: i64,
    wins: i32,
    losses: i32,
    ties: i32,
    points: f64
) -> Result<User, UserError> {
    sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET 
            wins = wins + $1,
            losses = losses + $2,
            ties = ties + $3,
            total_points = total_points + $4
        WHERE id = $5
        RETURNING *
        "#,
        wins,
        losses,
        ties,
        points,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => UserError::NotFound,
        _ => UserError::DatabaseError(e),
    })
}

/// Retrieves a user's statistics
pub async fn get_user_statistics(pool: &PgPool, user_id: i64) -> Result<UserStats, UserError> {
    let row = sqlx::query!(
        r#"
        SELECT 
            COALESCE(wins, 0) as "wins!: i32",
            COALESCE(losses, 0) as "losses!: i32",
            COALESCE(ties, 0) as "ties!: i32",
            COALESCE(total_points, 0.0) as "total_points!: f64"
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => UserError::NotFound,
        _ => UserError::DatabaseError(e),
    })?;

    Ok(UserStats {
        wins: row.wins,
        losses: row.losses,
        ties: row.ties,
        total_points: row.total_points,
        leagues_joined: 0,  // Placeholder value
        teams_created: 0,   // Placeholder value
    })
}