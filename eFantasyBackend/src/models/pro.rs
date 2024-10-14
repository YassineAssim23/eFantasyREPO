use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

/// Represents a professional player in esports with their statistics and attributes.
/// All fields are optional to accommodate varying data availability across different players.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProPlayer {
    /// MongoDB's unique identifier for the document.
    #[serde(rename = "_id")]
    pub id: ObjectId,

    /// The player's in-game name or alias.
    pub name: Option<String>,

    /// The player's country of origin, typically represented by a country code.
    pub country: Option<String>,

    /// The player's role or position in the game (e.g., "TOP", "MID", "ADC").
    pub position: Option<String>,

    /// Number of games played by the player.
    pub games: Option<String>,

    /// The player's win rate, represented as a percentage string.
    #[serde(rename = "Win rate")]
    pub win_rate: Option<String>,

    /// Kill/Death/Assist ratio of the player.
    pub kda: Option<String>,

    /// Average number of kills per game.
    #[serde(rename = "Avg kills")]
    pub avg_kills: Option<String>,

    /// Average number of deaths per game.
    #[serde(rename = "Avg deaths")]
    pub avg_deaths: Option<String>,

    /// Average number of assists per game.
    #[serde(rename = "Avg assists")]
    pub avg_assists: Option<String>,

    /// Creep Score per Minute, indicating farming efficiency.
    pub csm: Option<String>,

    /// Gold Per Minute, indicating the player's economic impact.
    pub gpm: Option<String>,

    /// Kill Participation percentage, showing involvement in team fights.
    #[serde(rename = "KP%")]
    pub kp_percentage: Option<String>,

    /// Damage percentage, indicating the player's share of team's total damage.
    #[serde(rename = "DMG%")]
    pub dmg_percentage: Option<String>,

    /// Damage Per Minute, showing the player's damage output over time.
    pub dpm: Option<String>,

    /// Vision Score Per Minute, indicating the player's contribution to team vision.
    pub vspm: Option<String>,

    /// Average Wards Placed per Minute.
    #[serde(rename = "Avg WPM")]
    pub avg_wpm: Option<String>,

    /// Average Wards Cleared per Minute.
    #[serde(rename = "Avg WCPM")]
    pub avg_wcpm: Option<String>,

    /// Average Vision Wards Placed per Minute.
    #[serde(rename = "Avg VWPM")]
    pub avg_vwpm: Option<String>,

    /// Gold Difference at 15 minutes, indicating early game performance.
    #[serde(rename = "GD@15")]
    pub gd_at_15: Option<String>,

    /// Creep Score Difference at 15 minutes.
    #[serde(rename = "CSD@15")]
    pub csd_at_15: Option<String>,

    /// Experience Difference at 15 minutes.
    #[serde(rename = "XPD@15")]
    pub xpd_at_15: Option<String>,

    /// First Blood participation percentage.
    #[serde(rename = "FB %")]
    pub fb_percentage: Option<String>,

    /// Number of times the player was the victim of First Blood.
    #[serde(rename = "FB Victim")]
    pub fb_victim: Option<String>,

    /// Number of Pentakills (killing all 5 enemy champions) achieved.
    #[serde(rename = "Penta Kills")]
    pub penta_kills: Option<String>,
}