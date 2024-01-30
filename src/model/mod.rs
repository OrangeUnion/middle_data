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
    let config = Config::new().await;
    let connect = MySqlConnectOptions::from_str(&config.database_url.unwrap()).unwrap()
        .username(&config.database_username.unwrap())
        .password(&config.database_password.unwrap());
    sqlx::MySqlPool::connect_with(connect).await.expect("数据库连接错误")
}