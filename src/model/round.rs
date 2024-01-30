use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::model::get_conn;
use crate::{log_error, log_info, util};

/// # Round 数据库映射
#[derive(Clone, Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Round {
    pub id: i64,
    pub time: NaiveDateTime,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

/// # 按Id查询Round
/// * id 主键
pub async fn select_by_id(id: i64) -> Option<Round> {
    let conn = get_conn().await;
    let sql = "select * from ele_round where id = ?";
    let response = sqlx::query_as::<_, Round>(sql).bind(id).fetch_one(&conn).await;
    match response {
        Ok(r) => {
            log_info!("ID查询 {r:?}");
            Some(r)
        }
        Err(e) => {
            log_error!("ID查询 {e}");
            None
        }
    }
}

pub async fn select_by_now() -> Option<Round> {
    let conn = get_conn().await;
    let sql = "SELECT * FROM ele_round ORDER BY id DESC LIMIT 1;";
    let response = sqlx::query_as::<_, Round>(sql).fetch_one(&conn).await;
    match response {
        Ok(r) => {
            log_info!("实时查询 {r:?}");
            Some(r)
        }
        Err(e) => {
            log_error!("实时查询 {e}");
            None
        }
    }
}

/// # 新增Round
/// * result Round数据
pub async fn insert(time_str: &str) -> Option<Round> {
    let now = util::now_time();
    let naive_data_time = util::format_time(time_str);
    let id = naive_data_time.format("%Y%m%d").to_string().parse::<i64>().unwrap_or(00000000);
    let conn = get_conn().await;
    let sql = "insert into ele_round (id, time, create_time, update_time) values (?, ?, ?, ?)";
    let response = sqlx::query(sql)
        .bind(id)
        .bind(naive_data_time)
        .bind(now)
        .bind(now)
        .execute(&conn).await;
    match response {
        Ok(r) => {
            let new = select_by_id(id).await;
            log_info!("Round新增{} {:?}", r.rows_affected(), new);
            new
        }
        Err(e) => {
            log_error!("Round新增 {e}");
            None
        }
    }
}

