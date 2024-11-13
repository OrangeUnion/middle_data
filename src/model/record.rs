use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use void_log::{log_info, log_warn};
use crate::{model::get_conn, util};

/// # Record 数据库映射
#[derive(Clone, Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Record {
    pub id: i64,
    pub round_id: i64,
    pub this_clan_id: i64,
    pub other_clan_id: i64,
    pub old_score: i64,
    pub new_score: i64,
    pub result: i64,
    pub create_time: NaiveDateTime,
}

impl Record {
    pub fn insert_data(round_id: i64, this_clan_id: i64, other_clan_id: i64, old_score: i64, new_score: i64, result: i64) -> Self {
        Self {
            id: 0,
            round_id,
            this_clan_id,
            other_clan_id,
            old_score,
            new_score,
            result,
            create_time: Default::default(),
        }
    }
}

/// # 按Id查询Record
/// * id 主键
pub async fn select_by_id(id: u64) -> Option<Record> {
    let conn = get_conn().await;
    let sql = "select * from ele_record where id = ?";
    let response = sqlx::query_as::<_, Record>(sql).bind(id).fetch_one(&conn).await;
    match response {
        Ok(r) => {
            log_info!("Record ID查询 {r:?}");
            Some(r)
        }
        Err(e) => {
            log_warn!("Record ID查询 {e}");
            None
        }
    }
}

/// # 按UserId查询Record
/// * id 主键
pub async fn select_by_user(round_id: i64, this_clan_id: i64) -> Option<Record> {
    let conn = get_conn().await;
    let sql = "select * from ele_record where round_id = ? and this_clan_id = ?";
    let response = sqlx::query_as::<_, Record>(sql)
        .bind(round_id)
        .bind(this_clan_id)
        .fetch_one(&conn).await;
    match response {
        Ok(r) => {
            log_info!("Record UserId查询 {r:?}");
            Some(r)
        }
        Err(e) => {
            log_warn!("Record UserId查询 {e}");
            None
        }
    }
}

/// # 新增Record
/// * result Result数据
pub async fn insert(result: Record) -> Option<Record> {
    let now = util::now_time();
    let conn = get_conn().await;
    let sql = "insert into ele_record (round_id, this_clan_id, other_clan_id, old_score, new_score, result, create_time) values (?, ?, ?, ?, ?, ?, ?)";
    let response = sqlx::query(sql)
        .bind(result.round_id)
        .bind(result.this_clan_id)
        .bind(result.other_clan_id)
        .bind(result.old_score)
        .bind(result.new_score)
        .bind(result.result)
        .bind(now)
        .execute(&conn).await;
    match response {
        Ok(r) => {
            let new = select_by_id(r.last_insert_id()).await;
            log_info!("Record新增 {:?}", new);
            new
        }
        Err(e) => {
            let view = select_by_user(result.round_id, result.this_clan_id).await;
            log_warn!("Record查询 {view:?} {e}");
            view
        }
    }
}