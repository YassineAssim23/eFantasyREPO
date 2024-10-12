#[macro_use] extern crate rocket;

use sqlx::postgres::PgPool;
use dotenv::dotenv;
use reqwest::Client;
use mongodb::{Client as MongoClient, options::ClientOptions};

mod models;
mod handlers;
mod db;
use crate::handlers::user::{register, get_user, delete_user};
use crate::handlers::pro::{insert_pro_player};

/// Main application state
/// 
/// This struct holds all shared resources that our web server will use.
/// 
/// Fields:
/// - db: PostgreSQL connection pool for database operations
/// - supabase_client: HTTP client for making requests to Supabase
/// - supabase_api_key: API key for authenticating with Supabase
/// - mongo_db: MongoDB database connection
pub struct AppState {
    pub db: PgPool,
    pub supabase_client: Client,
    pub supabase_api_key: String,
    pub mongo_db: mongodb::Database,
}

/// Root route handler
/// 
/// This function will be called when a GET request is made to the root URL "/".
/// 
/// Returns:
/// - A static string as a simple response
#[get("/")]
fn index() -> &'static str {
    "EHLIE???"
}

/// Main function to set up and run the Rocket web server
/// 
/// This function initializes the application state and builds the Rocket server.
/// 
/// Returns:
/// - A Rocket instance configured with routes and state
#[launch]
async fn rocket() -> _ {
    let state = initialize_app_state().await.expect("Failed to initialize app state");
    rocket::build()
        .manage(state)
        .mount("/", routes![index, register, get_user, delete_user])
}

/// Establish connection to PostgreSQL
/// 
/// This function creates a connection pool to the PostgreSQL database.
/// 
/// Parameters:
/// - url: The connection URL for the PostgreSQL database
/// 
/// Returns:
/// - Ok(PgPool): A successful connection pool
/// - Err(sqlx::Error): An error if the connection fails
async fn connect_to_postgres(url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(url).await?;
    println!("Successfully connected to Postgres");
    Ok(pool)
}

/// Establish connection to MongoDB
/// 
/// This function creates a connection to the MongoDB database.
/// 
/// Parameters:
/// - uri: The connection URI for the MongoDB database
/// 
/// Returns:
/// - Ok(mongodb::Database): A successful database connection
/// - Err(mongodb::error::Error): An error if the connection fails
async fn connect_to_mongodb(uri: &str) -> Result<mongodb::Database, mongodb::error::Error> {
    let db_name = std::env::var("MONGODB_NAME").unwrap();
    let client_options = ClientOptions::parse(uri).await?;
    let client = MongoClient::with_options(client_options)?;
    let db = client.database(&db_name);

    //Test
    insert_pro_player(db).await;

    let db = client.database(&db_name);
    println!("Successfully connected to MongoDB");
    Ok(db)
}

/// Create a new HTTP client for Supabase
/// 
/// This function builds a new HTTP client for making requests to Supabase.
/// 
/// Returns:
/// - Ok(Client): A successfully created HTTP client
/// - Err(reqwest::Error): An error if client creation fails
fn create_supabase_client() -> Result<Client, reqwest::Error> {
    let client = Client::builder().build()?;
    println!("Successfully created Supabase client");
    Ok(client)
}

/// Initialize the entire application state
/// 
/// This function sets up all necessary connections and resources for the application.
/// 
/// Returns:
/// - Ok(AppState): A fully initialized application state
/// - Err(Box<dyn std::error::Error>): An error if initialization fails
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