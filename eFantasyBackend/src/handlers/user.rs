use rocket::State;
use crate::AppState;
use crate::models::user::{NewUser, User, LoginCredentials, UserProfileUpdate, ProfileCompletion, UserStats};
use crate::errors::UserError;
use rocket::serde::json::Json;
use rocket::http::Status;
use crate::auth::{verify_password, generate_token};
use crate::guards::{NoAuthGuard, AuthGuard};

/// Handles user login
#[post("/login", data = "<credentials>")]
pub async fn login(_guard: NoAuthGuard, state: &State<AppState>, credentials: Json<LoginCredentials>) -> Result<Json<String>, Status> {
    let user = crate::db::user::get_user_by_name(&state.db, &credentials.username)
        .await
        .map_err(|_| Status::Unauthorized)?;

    if verify_password(&credentials.password, &user.password) {
        match generate_token(user.id) {
            Ok(token) => Ok(Json(token)),
            Err(e) => {
                eprintln!("Token generation error: {}", e);
                Err(Status::InternalServerError)
            }
        }
    } else {
        Err(Status::Unauthorized)
    }
}

/// Handles user registration
#[post("/register", data = "<new_user>")]
pub async fn register(_guard: NoAuthGuard, state: &State<AppState>, new_user: Json<NewUser>) -> Result<Json<User>, UserError> {
    let user = crate::db::user::create_user(&state.db, new_user.into_inner()).await?;
    Ok(Json(user))
}

/// Handles profile completion
#[post("/complete-profile", data = "<profile>")]
pub async fn complete_profile(auth: AuthGuard, state: &State<AppState>, profile: Json<ProfileCompletion>) -> Result<Json<User>, UserError> {
    println!("complete_profile: Handler called for user_id: {}", auth.user_id);
    let updated_user = crate::db::user::complete_profile(&state.db, auth.user_id, profile.into_inner()).await?;
    println!("complete_profile: Profile updated successfully");
    Ok(Json(updated_user))
}
/// Handles user sign out
#[post("/signout")]
pub async fn sign_out(_auth: AuthGuard) -> Status {
    Status::Ok
}

/// Retrieves a user by ID or username
#[get("/user/<id_or_name>")]
pub async fn get_user(state: &State<AppState>, id_or_name: &str) -> Result<Json<User>, UserError> {
    let result = if let Ok(id) = id_or_name.parse::<i64>() {
        crate::db::user::get_user_by_id(&state.db, id).await
    } else {
        crate::db::user::get_user_by_name(&state.db, id_or_name).await
    };

    result.map(Json)
}

/// Deletes a user
#[delete("/user/<id>")]
pub async fn delete_user(state: &State<AppState>, id: i64) -> Status {
    match crate::db::user::delete_user(&state.db, id).await {
        Ok(true) => Status::NoContent,
        Ok(false) => Status::NotFound,
        Err(_) => Status::InternalServerError
    }
}

/// Retrieves a user's profile
#[get("/user/<id>/profile")]
pub async fn get_user_profile(state: &State<AppState>, id: i64, _auth: AuthGuard) -> Result<Json<User>, UserError> {
    let user = crate::db::user::get_user_by_id(&state.db, id).await?;
    Ok(Json(user))
}

/// Updates a user's profile
#[put("/user/<id>/profile", data = "<profile_update>")]
pub async fn update_user_profile(
    state: &State<AppState>,
    id: i64, 
    profile_update: Json<UserProfileUpdate>, 
    _auth: AuthGuard
) -> Result<Json<User>, UserError> {
    let updated_user = crate::db::user::update_user_profile(&state.db, id, profile_update.into_inner()).await?;
    Ok(Json(updated_user))
}

/// Retrieves a user's statistics
#[get("/user/<id>/stats")]
pub async fn get_user_stats(state: &State<AppState>, id: i64, _auth: AuthGuard) -> Result<Json<UserStats>, UserError> {
    let stats = crate::db::user::get_user_statistics(&state.db, id).await?;
    Ok(Json(stats))
}