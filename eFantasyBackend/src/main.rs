#[macro_use] extern crate rocket;

use sqlx::postgres::PgPool;
use dotenv::dotenv;
use reqwest::Client;
use mongodb::{Client as MongoClient, options::ClientOptions};

mod models;
mod handlers;
mod db;

pub struct AppState {
    pub db: PgPool,
    pub supabase_client: Client,
    pub supabase_api_key: String,
    pub mongo_db: mongodb::Database,
}

#[get("/")]
fn index() -> &'static str {
    "EHLIE???"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    
    let database_url = std::env::var("POSTGRES_DATABASE_URL")
        .expect("POSTGRES_DATABASE_URL must be set");
    let supabase_api_key = std::env::var("SUPABASE_API_KEY")
        .expect("SUPABASE_API_KEY must be set");

    //mongoDB
    let mongo_uri = std::env::var("MONGODB_URI").expect("Set env var");

    println!("Attempting to connect to MongoDB...");
    let mongo_options = ClientOptions::parse(&mongo_uri)
        .await
        .expect("Failed to parse MongoDB options");

    let mongo_client = MongoClient::with_options(mongo_options)
        .expect("Failed to create MongoDB client");
    let mongo_db = mongo_client.database("your_database_name");
    println!("MongoDB connection successful!");


    println!("Attempting to connect to database...");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");
    println!("Database connection successful!");

    let supabase_client = Client::builder()
        .build()
        .expect("Failed to create supabase client");

    let state = AppState {
        db: pool,
        supabase_client,
        supabase_api_key,
        mongo_db,
    };

    rocket::build()
        .manage(state)
        .mount("/", routes![index])
}