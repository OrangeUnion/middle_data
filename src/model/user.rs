use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Executor;
use crate::{log_error, log_info, util};
use crate::model::get_conn;

/// # User 数据库映射
#[derive(Clone, Default, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub tag: String,
    pub name: Option<String>,
    pub score: i64,
    pub reverse_count: i64,
    pub league_id: i64,
    pub is_national: bool,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

pub async fn select_by_tag(tag: &str, is_national: bool) -> Option<User> {
    let conn = get_conn().await;
    let sql = "select * from ele_user where tag = ? and is_national = ?";
    let response = sqlx::query_as::<_, User>(sql).bind(tag).bind(is_national).fetch_one(&conn).await;
    match response {
        Ok(r) => {
            log_info!("Tag查询 {r:?}");
            Some(r)
        }
        Err(e) => {
            log_error!("Tag查询 {e}");
            None
        }
    }
}

pub async fn create(tag: &str, name: Option<String>, league_id: i64, is_national: bool) -> Option<User> {
    let now = util::now_time();
    let name = name.unwrap_or("New Clan".to_string());
    let conn = get_conn().await;
    let sql = "insert into ele_user (tag, name, league_id, is_national, create_time, update_time) values (?, ?, ?, ?, ?, ?) RETURNING id, tag, name, score, reverse_count, league_id, is_national, create_time, update_time;";
    let response = sqlx::query_as::<_, User>(sql)
        .bind(tag)
        .bind(name)
        .bind(league_id)
        .bind(is_national)
        .bind(now)
        .bind(now)
        .fetch_one(&conn).await;
    match response {
        Ok(r) => {
            log_info!("User新增 {r:?}");
            Some(r)
        }
        Err(e) => {
            log_error!("User新增 {e}");
            None
        }
    }
}