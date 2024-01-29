use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// # Reverse 数据库映射
#[derive(Clone, Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Reverse {
    pub id: i64,
    pub round_id: i64,
    pub this_clan_id: i64,
    pub this_old_score: i64,
    pub this_new_score: i64,
    pub other_clan_id: i64,
    pub other_old_score: i64,
    pub other_new_score: i64,
    pub old_result: i64,
    pub new_result: i64,
    pub create_time: NaiveDateTime,
}

/// # 设置逆转入口
pub async fn set() {
    //
}