use sqlx::PgPool;
use crate::models::User;
use crate::models::user::NewUser;

///  Create a new user in the database
/// 
/// This function inserts a new user into the 'users' table and returns the created user.
/// 
///  Parameters:
///  - pool: A reference to the database connection pool.
///  - user: A NewUser struct containing the user's details.
/// 
///  Returns:
///  - Result<User, sqlx::Error>: The created user or an error if the insertion fails.
pub async fn create_user(pool: &PgPool, user: NewUser) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING *",
        user.username,
        user.email,
        user.password
    )
    .fetch_one(pool)
    .await
}

///  Retrieve a user from the database by their ID
/// 
///  This function selects a user from the 'users' table based on the provided ID and returns the user.
/// 
///  Parameters:
///  - pool: A reference to the database connection pool.
///  - user_id: The ID of the user to retrieve.
/// 
///  Returns:
///  - Result<User, sqlx::Error>: The retrieved user or an error if the retrieval fails.
pub async fn get_user_by_id(pool: &PgPool, user_id: i64) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(pool)
    .await
}

///  Delete a user from the database by their ID
/// 
///  This function deletes a user from the 'users' table based on the provided ID and returns a boolean indicating success.
/// 
///  Parameters:
///  - pool: A reference to the database connection pool.
///  - user_id: The ID of the user to delete.
/// 
///  Returns:
///  - Result<bool, sqlx::Error>: A boolean indicating success or failure of the deletion.
pub async fn delete_user(pool: &PgPool, user_id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}