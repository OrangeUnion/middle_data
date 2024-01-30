use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::{log_info, log_warn, util};
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
    pub is_global: bool,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

/// # 按Id查询User
/// * id 主键
pub async fn select_by_id(id: i64) -> Option<User> {
    let conn = get_conn().await;
    let sql = "select * from ele_user where id = ?";
    let response = sqlx::query_as::<_, User>(sql).bind(id).fetch_one(&conn).await;
    match response {
        Ok(r) => {
            log_info!("User ID查询 {r:?}");
            Some(r)
        }
        Err(e) => {
            log_warn!("User ID查询 {e}");
            None
        }
    }
}

/// # 按Tag查询User
/// * tag 标签
/// * is_global 是否国际服
pub async fn select_by_tag(tag: &str, is_national: bool) -> Option<User> {
    let conn = get_conn().await;
    let sql = "select * from ele_user where tag = ? and is_global = ?";
    let response = sqlx::query_as::<_, User>(sql).bind(tag).bind(is_national).fetch_one(&conn).await;
    match response {
        Ok(r) => {
            log_info!("User Tag查询 {r:?}");
            Some(r)
        }
        Err(e) => {
            log_warn!("User Tag查询 {e}");
            None
        }
    }
}

/// # 新增User
/// * tag 标签
/// * name 名称
/// * league 联盟Id
/// * is_national 是否国际服
pub async fn insert(tag: &str, name: Option<String>, league_id: i64, is_global: bool) -> Option<User> {
    let now = util::now_time();
    let name = name.unwrap_or("New Clan".to_string());
    let conn = get_conn().await;
    let sql = "insert into ele_user (tag, name, league_id, is_global, create_time, update_time) values (?, ?, ?, ?, ?, ?)";
    let response = sqlx::query(sql)
        .bind(tag)
        .bind(name)
        .bind(league_id)
        .bind(is_global)
        .bind(now)
        .bind(now)
        .execute(&conn).await;
    match response {
        Ok(r) => {
            let new_user = select_by_id(r.last_insert_id() as i64).await;
            log_info!("User新增 {:?}", new_user);
            new_user
        }
        Err(e) => {
            log_warn!("User新增 {e}");
            None
        }
    }
}

/// # UserScore更新
/// * id 主键
/// * score 新积分
pub async fn update_score(id: i64, score: i64) -> Option<User> {
    let now = util::now_time();
    let conn = get_conn().await;
    let sql = "update ele_user set score = ?, update_time = ? where id = ?";
    let response = sqlx::query(sql)
        .bind(score)
        .bind(now)
        .bind(id)
        .execute(&conn).await;
    match response {
        Ok(r) => {
            let new_user = select_by_id(id).await;
            log_info!("UserScore更新{} {:?}", r.last_insert_id(), new_user);
            new_user
        }
        Err(e) => {
            log_warn!("UserScore更新 {e}");
            None
        }
    }
}