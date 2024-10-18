use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a league in the fantasy sports system
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct League {
    /// Unique identifier for the league
    pub id: i64,
    /// Name of the league
    pub name: String,
    /// User ID of the league administrator
    pub admin_id: i64,
    /// Maximum number of teams allowed in the league
    pub max_teams: i32,
    /// Whether the league is public or private
    pub is_public: bool,
    /// Scheduled time for the league's draft
    pub draft_time: DateTime<Utc>,
    /// Type of scoring system used in the league
    pub scoring_type: String,
    /// List of user IDs of league participants
    pub participants: Vec<i64>,
    /// Optional draft order, represented as a list of user IDs
    pub draft_order: Option<Vec<i64>>,
    /// Timestamp of when the league was created
    pub created_at: DateTime<Utc>,
    /// Timestamp of when the league was last updated
    pub updated_at: DateTime<Utc>,
}

/// Represents the data required to create a new league
#[derive(Debug, Serialize, Deserialize)]
pub struct NewLeague {
    /// Name of the new league
    pub name: String,
    /// Maximum number of teams allowed in the new league
    pub max_teams: i32,
    /// Whether the new league is public or private
    pub is_public: bool,
    /// Scheduled time for the new league's draft
    pub draft_time: DateTime<Utc>,
    /// Type of scoring system to be used in the new league
    pub scoring_type: String,
}

/// Represents the data required to update a league
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLeague {
    pub name: String,
    pub max_teams: i32,
    pub is_public: bool,
    pub draft_time: DateTime<Utc>,
    pub scoring_type: String,
    pub participants: Vec<i64>,
}