use std::str::FromStr;
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlConnectOptions;
use crate::util::Config;

pub mod record;
pub mod reverse;
pub mod league;
pub mod round;
pub mod user;

pub async fn get_conn() -> Pool<MySql> {
    let database = Config::new().await.database.unwrap_or_default();
    let connect = MySqlConnectOptions::from_str(&database.url.unwrap()).unwrap()
        .username(&database.username.unwrap())
        .password(&database.password.unwrap());
    sqlx::MySqlPool::connect_with(connect).await.expect("数据库连接错误")
}