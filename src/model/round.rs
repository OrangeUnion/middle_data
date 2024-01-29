use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// # Round 数据库映射
#[derive(Clone, Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Round {
    pub id: i64,
    pub name: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}