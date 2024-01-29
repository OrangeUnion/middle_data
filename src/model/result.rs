use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// # Result 数据库映射
#[derive(Clone, Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Result {
    pub id: i64,
    pub round_id: String,
    pub this_clan_id: String,
    pub other_clan_id: String,
    pub result: String,
    pub create_time: NaiveDateTime,
}

/// # Result Json映射
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ResultApi {
    pub time_round: i64,
    pub this_clan: ClanInfo,
    pub other_clan: ClanInfo,
    pub result: String,
    pub create_time: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ClanInfo {
    pub tag: String,
    pub name: String,
    pub score: i64,
    pub league: ClanInfoLeague,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ClanInfoLeague {
    id: i64,
    name: String,
}

/// # 获取对战入口
pub async fn get() {
    //
}