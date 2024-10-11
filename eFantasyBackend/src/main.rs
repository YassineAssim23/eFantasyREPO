#[macro_use] extern crate rocket;

use sqlx::postgres::PgPool;
use dotenv::dotenv;
use reqwest::Client;

mod models;
mod handlers;
mod db;

pub struct AppState {
    pub db: PgPool,
    pub supabase_client: Client,
    pub supabase_api_key: String,
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
    };

    rocket::build()
        .manage(state)
        .mount("/", routes![index])
}