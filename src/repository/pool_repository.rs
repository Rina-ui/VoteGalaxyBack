use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use crate::model::pool::CreatePoolInput;

pub struct PoolRepository {
    pool: Pool<Sqlite>,
}

impl PoolRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    //creation du nouveau poll
    pub async fn create_pool(
        &self,
        input: CreatePoolInput,
        admin_id: Uuid,
    ) -> Result<Pool, sqlx::Error> {
        let pool_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!()
    }

}