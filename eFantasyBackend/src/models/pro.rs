use serde::{ Deserialize, Serialize};

/// Represents a pro player and their basic attributes
/// Member Variables:
///     - gamertag: A Players current gamertag
///     - team: A players current team
#[derive(Serialize, Deserialize, Debug)]
pub struct ProPlayer 
{
    pub gamertag: String,
    pub team: String,
}

