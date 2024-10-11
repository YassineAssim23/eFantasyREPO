// This line allows us to use Rocket's macros throughout our code
#[macro_use] extern crate rocket;

// Import necessary dependencies
use sqlx::postgres::PgPool;  // For PostgreSQL database connection
use dotenv::dotenv;  // For loading environment variables
use reqwest::Client;  // For making HTTP requests (used with Supabase)
use mongodb::{Client as MongoClient, options::ClientOptions};  // For MongoDB connection

// Import local modules
mod models;
mod handlers;
mod db;

// Define the main application state
// This struct holds all shared resources that our web server will use
pub struct AppState {
    pub db: PgPool,  // PostgreSQL connection pool
    pub supabase_client: Client,  // HTTP client for Supabase
    pub supabase_api_key: String,  // API key for Supabase
    pub mongo_db: mongodb::Database,  // MongoDB database connection
}

// Define the root route of our web server
// This function will be called when a GET request is made to the root URL "/"
#[get("/")]
fn index() -> &'static str {
    "EHLIE???"
}

// The main function that sets up and runs our Rocket web server
#[launch]
async fn rocket() -> _ {
    // Initialize the application state
    let state = initialize_app_state().await.expect("Failed to initialize app state");

    // Build and launch the Rocket server
    rocket::build()
        .manage(state)  // Make the AppState available to all routes
        .mount("/", routes![index])  // Mount the index route
}

// Function to establish a connection to PostgreSQL
async fn connect_to_postgres(url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(url).await?;
    println!("Successfully connected to Postgres");
    Ok(pool)
}

// Function to establish a connection to MongoDB
async fn connect_to_mongodb(uri: &str) -> Result<mongodb::Database, mongodb::error::Error> {
    // Parse the MongoDB connection string
    let client_options = ClientOptions::parse(uri).await?;
    // Create a new client
    let client = MongoClient::with_options(client_options)?;
    // Connect to a specific database
    let db = client.database("eFantasy");  // Replace "eFantasy" with your actual database name
    println!("Successfully connected to MongoDB");
    Ok(db)
}

// Function to create a new HTTP client for Supabase
fn create_supabase_client() -> Result<Client, reqwest::Error> {
    let client = Client::builder().build()?;
    println!("Successfully created Supabase client");
    Ok(client)
}

// Function to initialize the entire application state
async fn initialize_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    // Load environment variables from a .env file if present
    dotenv().ok();

    // Retrieve necessary environment variables
    let postgres_url = std::env::var("POSTGRES_DATABASE_URL")?;
    let supabase_api_key = std::env::var("SUPABASE_API_KEY")?;
    let mongodb_uri = std::env::var("MONGODB_URI")?;

    // Attempt to connect to PostgreSQL
    let db = match connect_to_postgres(&postgres_url).await {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to connect to Postgres: {}", e);
            return Err(Box::new(e));
        }
    };

    // Attempt to connect to MongoDB
    let mongo_db = match connect_to_mongodb(&mongodb_uri).await {
        Ok(db) => db,
        Err(e) => {
            println!("Failed to connect to MongoDB: {}", e);
            return Err(Box::new(e));
        }
    };

    // Attempt to create Supabase client
    let supabase_client = match create_supabase_client() {
        Ok(client) => client,
        Err(e) => {
            println!("Failed to create Supabase client: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("All connections established successfully");

    // Return the fully initialized AppState
    Ok(AppState {
        db,
        supabase_client,
        supabase_api_key,
        mongo_db,
    })
}