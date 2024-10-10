mod models;
mod handlers;
mod db;

use actix_web::{HttpServer, App, web::Data};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    println!("Starting running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(|| async {HttpResponse::Ok().body("EHLIE???") }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


