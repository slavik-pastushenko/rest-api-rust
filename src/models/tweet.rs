use chrono::prelude::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

/// Tweet
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tweet {
    /// The id of a tweet
    pub id: Option<String>,

    /// The message of a tweet
    pub message: String,

    /// The updated date of a tweet
    pub updated_at: DateTime<Utc>,

    /// The created date of a tweet
    pub created_at: DateTime<Utc>,
}
