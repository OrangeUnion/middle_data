use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use void_log::{log_info, log_warn};
use crate::{model::get_conn, util};

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

impl Reverse {
    pub fn insert_data(round_id: i64,
                       this_clan_id: i64, this_old_score: i64, this_new_score: i64,
                       other_clan_id: i64, other_old_score: i64, other_new_score: i64,
                       old_result: i64, new_result: i64) -> Self {
        Self {
            id: 0,
            round_id,
            this_clan_id,
            this_old_score,
            this_new_score,
            other_clan_id,
            other_old_score,
            other_new_score,
            old_result,
            new_result,
            create_time: Default::default(),
        }
    }
}

pub async fn select_by_id(id: u64) -> Option<Reverse> {
    let conn = get_conn().await;
    let sql = "select * from ele_reverse where id = ?";
    let response = sqlx::query_as::<_, Reverse>(sql).bind(id).fetch_one(&conn).await;
    match response {
        Ok(r) => {
            log_info!("Reverse ID查询 {r:?}");
            Some(r)
        }
        Err(e) => {
            log_warn!("Reverse ID查询 {e}");
            None
        }
    }
}

pub async fn insert(reverse: Reverse) -> Option<Reverse> {
    let now = util::now_time();
    let conn = get_conn().await;
    let sql = "insert into ele_reverse values (null, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
    let response = sqlx::query(sql)
        .bind(reverse.round_id)
        .bind(reverse.this_clan_id)
        .bind(reverse.this_old_score)
        .bind(reverse.this_new_score)
        .bind(reverse.other_clan_id)
        .bind(reverse.other_old_score)
        .bind(reverse.other_new_score)
        .bind(reverse.old_result)
        .bind(reverse.new_result)
        .bind(now)
        .execute(&conn).await;
    match response {
        Ok(r) => {
            let new = select_by_id(r.last_insert_id()).await;
            log_info!("Reverse新增 {:?}", new);
            new
        }
        Err(e) => {
            log_warn!("Reverse新增 {e}");
            None
        }
    }
}