use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vote{
    pub id: Uuid,
    pub pool_id: Uuid,
    pub option_id: Uuid,
    pub voter_id: Option<String>,
    pub voted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CastVoteInput {
    pub poll_id: Uuid,
    pub option_id: Uuid,
    pub voter_id: Option<String>,
}
