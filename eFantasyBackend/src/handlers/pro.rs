extern crate serde_json;
use std::fs::File;
use std::io::BufReader;
use rocket::State;
use crate::AppState;
use crate::models::pro::ProPlayer;
use crate::models::pro::InsertResponse;
use crate::models::pro::ProPlayerVec;
use rocket::serde::json::Json;
use serde_json::{Value};
use rocket::http::Status;
use std::error::Error;
use rocket::response::status::Custom;
use rocket::serde::ser::StdError;

/// Handles GET requests to retrieve a pro player by their name.
///
/// This function is the endpoint handler for the `/pro/<name>` route. It attempts to retrieve
/// a pro player from the database using the provided ID and returns the player data as JSON
/// if successful.
///
/// # Arguments
///
/// * `state` - The application state, which includes the database connection
/// * `name` - The name of the pro player to retrieve, provided in the URL
///
/// # Returns
///
/// * `Ok(Json<ProPlayer>)` if the player is found, with a 200 OK status
/// * `Err(Status)` with an appropriate error status if the player is not found or another error occurs
#[get("/pro/<id>")]
pub async fn get_pro_player_by_id(state: &State<AppState>, id: &str) -> Result<Json<ProPlayer>, Status> {
    match crate::db::pro::get_pro_player_by_id(&state.mongo_db, id).await {
        Ok(pro) => Ok(Json(pro)),
        Err(e) => {
            eprintln!("Error in get_pro_player: {}", e);  // Log the error
            match e.as_str() {
                "Invalid ObjectId format" => Err(Status::BadRequest),
                "Pro player not found" => Err(Status::NotFound),
                _ => Err(Status::InternalServerError),
            }
        },
    }
}


#[post("/insert_pro", data="<pro_player>")]
pub async fn insert_pro_player(state: &State<AppState>, pro_player: Json<ProPlayer>) -> Result<Json<InsertResponse>, Status> {
    match crate::db::pro::insert_pro_player_by_json(&state.mongo_db, &pro_player.into_inner()).await {
        Ok(pro) => {
            let resp = InsertResponse {
                inserted_id: pro.inserted_id.to_string(),
            };
            Ok(Json(resp))
        },
        Err(e) => {
            // **CHANGE TO SHOW CORRECT ERRORS**
            eprintln!("Error in get_pro_player: {}", e);  // Log the error
            match e.as_str() {
                "Invalid ObjectId format" => Err(Status::BadRequest),
                "Pro player not found" => Err(Status::NotFound),
                _ => Err(Status::InternalServerError),
            }
        },
    }
}


pub async fn insert_all_pro_players(state: &State<AppState>, pro_players: &Json<ProPlayerVec>) -> Result<Status, Custom<String>> {
    match insert_all_pro_players_helper(state, pro_players).await {
        Ok(_) => Ok(Status::Ok),
        Err(e) => {
            eprintln!("Error inserting pro players: {}", e);
            Err(Custom(Status::InternalServerError, e.to_string()))
        }
    }
}

async fn insert_all_pro_players_helper(state: &State<AppState>, pro_players: &Json<ProPlayerVec>) -> Result<(), Box<dyn StdError>> {

    let inner_players = pro_players.clone().into_inner();

    for player in inner_players {
        insert_pro_player(state, Json(player)).await;
    }

    Ok(())
}

#[post("/insert_players", data="<pro_players>")]
pub async fn insert_players_route(state: &State<AppState>, pro_players: Json<ProPlayerVec>) -> Result<Status, Custom<String>> {
    insert_all_pro_players(state, &pro_players).await
}