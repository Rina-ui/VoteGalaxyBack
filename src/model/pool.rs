use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pool {
    pub id: Uuid,
    pub question: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub closes_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolOption {
    pub id: Uuid,
    pub pool_id: Uuid,
    pub text: String,
    pub vote_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct CreatePoolInput{
    pub question: String,
    pub options: Vec<String>,
    pub closes_at: Option<DateTime<Utc>>,
}

//le option signifie que la valuer peut etre null
