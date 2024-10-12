use reqwest::Client;
use mongodb::{Client as MongoClient, options::ClientOptions, Collection };
use crate::AppState;
use crate::models::pro::Pro_Player;

/// Test function to insert a pro player into mongoDB
pub async fn insert_pro_player(db: mongodb::Database) {

    let coll: Collection<Pro_Player> = db.collection("pro_players");

    let pro = Pro_Player {
        gamertag: "Faker".to_string(),
        team: "T1".to_string(),
    };
    println!("test");
    let res = coll.insert_one(pro).await;
    println!("Inserted a pro with _id: {}", res.unwrap().inserted_id);
}
