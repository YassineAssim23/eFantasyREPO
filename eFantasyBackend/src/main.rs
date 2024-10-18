#[macro_use] extern crate rocket;

use sqlx::postgres::PgPool;
use dotenv::dotenv;
use reqwest::Client;
use mongodb::{Client as MongoClient, options::ClientOptions};

use crate::handlers::user::{register, get_user, delete_user, login, sign_out, complete_profile, get_user_profile, update_user_profile, get_user_stats};
use crate::handlers::pro::{get_pro_player};
use crate::handlers::league::create_league;

mod models;
mod handlers;
mod db;
mod errors;
mod auth;
mod guards;

use crate::handlers::user::{register, get_user, delete_user, login, sign_out, complete_profile, get_user_profile, update_user_profile, get_user_stats};
use crate::handlers::pro::{get_pro_player_by_id, insert_players_route};

/// Main application state
pub struct AppState {
    pub db: PgPool,
    pub supabase_client: Client,
    pub supabase_api_key: String,
    pub mongo_db: mongodb::Database,
}

/// Root route handler
#[get("/")]
fn index() -> &'static str {
    "Welcome to eFantasy API"
}

/// Conflict error catcher
#[catch(409)]
fn conflict_catcher() -> &'static str {
    "Username or email already exists!"
}

#[launch]
async fn rocket() -> _ {
    match dotenv() {
        Ok(_) => println!("Successfully loaded .env file"),
        Err(e) => println!("Failed to load .env file: {:?}", e),
    }
    let state = initialize_app_state().await.expect("Failed to initialize app state");
    rocket::build()
        .manage(state)
        .mount("/", routes![
            index, 
            register, 
            get_user, 
            delete_user, 
            get_pro_player_by_id, 
            login, 
            sign_out,  
            complete_profile,
            get_user_profile,
            update_user_profile,
            get_user_stats,
            insert_players_route,
        ])
        .register("/", catchers![conflict_catcher])
}

/// Establishes connection to PostgreSQL
async fn connect_to_postgres(url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(url).await?;
    println!("Successfully connected to Postgres");
    Ok(pool)
}

/// Establishes connection to MongoDB
async fn connect_to_mongodb(uri: &str) -> Result<mongodb::Database, mongodb::error::Error> {
    let db_name = std::env::var("MONGODB_NAME").unwrap();
    let client_options = ClientOptions::parse(uri).await?;
    let client = MongoClient::with_options(client_options)?;
    let db = client.database(&db_name);
    println!("Successfully connected to MongoDB");
    Ok(db)
}

/// Creates a new HTTP client for Supabase
fn create_supabase_client() -> Result<Client, reqwest::Error> {
    let client = Client::builder().build()?;
    println!("Successfully created Supabase client");
    Ok(client)
}

/// Initializes the entire application state
async fn initialize_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    dotenv().ok();
    let postgres_url = std::env::var("POSTGRES_DATABASE_URL")?;
    let supabase_api_key = std::env::var("SUPABASE_API_KEY")?;
    let mongodb_uri = std::env::var("MONGODB_URI")?;

    let db = connect_to_postgres(&postgres_url).await?;
    let mongo_db = connect_to_mongodb(&mongodb_uri).await?;
    let supabase_client = create_supabase_client()?;

    println!("All connections established successfully");

    Ok(AppState {
        db,
        supabase_client,
        supabase_api_key,
        mongo_db,
    })
}