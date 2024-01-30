use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::{log_info, log_warn};
use crate::model::get_conn;

/// # League 数据库映射
#[derive(Clone, Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct League {
    pub id: i64,
    pub name: String,
    pub api_key: String,
    pub create_time: NaiveDateTime,
}

pub async fn select_by_id(id: i64) -> Option<League> {
    let conn = get_conn().await;
    let sql = "select * from ele_league where id = ?";
    let response = sqlx::query_as::<_, League>(sql).bind(id).fetch_one(&conn).await;
    match response {
        Ok(r) => {
            log_info!("League ID查询 {r:?}");
            Some(r)
        }
        Err(e) => {
            log_warn!("League ID查询 {e}");
            None
        }
    }
}

pub async fn select_by_key(api_key: &str) -> Option<League> {
    let conn = get_conn().await;
    let sql = "select * from ele_league where api_key = ?";
    let response = sqlx::query_as::<_, League>(sql).bind(api_key).fetch_one(&conn).await;
    match response {
        Ok(r) => {
            log_info!("League查询 {r:?}");
            Some(r)
        }
        Err(e) => {
            log_warn!("League查询 {e}");
            None
        }
    }
}