use mongodb::{Client as MongoClient, Collection};
use mongodb::bson::{doc, oid::ObjectId, Document};
use crate::models::pro::ProPlayer;

/// Test function to insert a pro player into mongoDB
pub async fn test_insert_pro_player(db: mongodb::Database) {

    let coll: Collection<ProPlayer> = db.collection("pro_players");

    let pro = ProPlayer {
        gamertag: "Faker".to_string(),
        team: "T1".to_string(),
    };
    println!("test");
    let res = coll.insert_one(pro).await;
    println!("Inserted a pro with _id: {}", res.unwrap().inserted_id);
}

///  Retrieve a pro player from the Mongo database by their pro_id
/// 
///  This function gets a pro from the pro_players collection based on the requested pro_id and returns the pro player
/// 
///  Parameters:
///  - pro_id: An int that refers to a pro's ObjectID
///  - client: The MongoDB Client
/// 
///  Returns:
///  - Result<ProPlayer, Error(Confirm this)>: The pulled pro or an error from MongoDB.
pub async fn get_pro_player_by_id(db: &mongodb::Database, pro_id: String) -> Result<ProPlayer, mongodb::error::Error> {
    let collection: Collection<ProPlayer> = db.collection(&std::env::var("MONGODB_PRO_PLAYER_COLLECTION").unwrap());

    let _id = ObjectId::parse_str(pro_id).unwrap();

    let result = collection.find_one(
        doc! { "_id": _id }
    ).await?;

    println!("{:#?}", result);

    Ok(result.unwrap())
}