use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a league in the fantasy sports system
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct League {
    pub id: i64,
    pub name: String,
    pub admin_id: i64,
    pub max_teams: i32,
    pub is_public: bool,
    pub draft_time: DateTime<Utc>,
    pub scoring_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents the data required to create a new league
#[derive(Debug, Serialize, Deserialize)]
pub struct NewLeague {
    pub name: String,
    pub max_teams: i32,
    pub is_public: bool,
    pub draft_time: DateTime<Utc>,
    pub scoring_type: String,
}