use mongodb::Collection;
use mongodb::bson::{doc, oid::ObjectId};
use crate::models::pro::ProPlayer;

/// Retrieves a pro player from the database by their ID.
///
/// This function connects to the MongoDB database, finds the collection
/// specified by the MONGODB_PRO_PLAYER_COLLECTION environment variable,
/// and attempts to retrieve a document with the given ID.
///
/// # Arguments
///
/// * `db` - A reference to the MongoDB database
/// * `pro_id` - A string slice containing the MongoDB ObjectId of the pro player
///
/// # Returns
///
/// * `Ok(ProPlayer)` if the player is found
/// * `Err(String)` if there's an error (e.g., player not found, database error)
pub async fn get_pro_player_by_id(db: &mongodb::Database, pro_id: &str) -> Result<ProPlayer, String> {
    // Retrieve the collection name from environment variables
    let collection_name = std::env::var("MONGODB_PRO_PLAYER_COLLECTION")
        .map_err(|_| "MONGODB_PRO_PLAYER_COLLECTION environment variable not set".to_string())?;
    
    println!("Accessing collection: {}", collection_name);

    // Get a handle to the pro players collection
    let collection: Collection<ProPlayer> = db.collection(&collection_name);

    // Parse the provided ID string into a MongoDB ObjectId
    let object_id = ObjectId::parse_str(pro_id)
        .map_err(|_| "Invalid ObjectId format".to_string())?;

    // Attempt to find the document with the given ID
    let result = collection.find_one(doc! { "_id": object_id }).await
        .map_err(|e| format!("Database error: {}", e))?;
    
    println!("Query result: {:?}", result);

    // If the document is found, return it; otherwise, return an error
    result.ok_or_else(|| "Pro player not found".to_string())
}

/// Inserts a pro player into the database from a JSON representation of them.
///
/// This function connects to the MongoDB database, finds the collection
/// specified by the MONGODB_PRO_PLAYER_COLLECTION environment variable,
/// and attempts to insert a document in the ProPlayer format.
///
/// # Arguments
///
/// * `db` - A reference to the MongoDB database
/// * `ProPlayer` - A ProPlayer object
///
/// # Returns
///
/// * `Ok(InsertOneResult)` if the player is inserted
/// * `Err(String)` if there's an error (e.g., insertion error, database error)
pub async fn insert_pro_player_by_json(db: &mongodb::Database, pro_player: &ProPlayer) -> Result<mongodb::results::InsertOneResult, String> {
    // Retrieve the collection name from environment variables
    let collection_name = std::env::var("MONGODB_PRO_PLAYER_COLLECTION")
        .map_err(|_| "MONGODB_PRO_PLAYER_COLLECTION environment variable not set".to_string())?;
    
    println!("Accessing collection: {}", collection_name);

    // Get a handle to the pro players collection
    let collection: Collection<ProPlayer> = db.collection(&collection_name);

    let result = collection.insert_one(pro_player).await
        .map_err(|e| format!("Database error: {}", e));
        
    
    result
}