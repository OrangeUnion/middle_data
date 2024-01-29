use std::f32::consts::E;
use serde::{Deserialize, Serialize};
use crate::api::result::ResMessage::{Failed, Success};
use crate::log_info;
use crate::model::{league, user};
use crate::model::user::User;

enum ResMessage<T, E> {
    Success(T),
    Failed(E),
}

/// # Result Json映射
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Result {
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

/// # result接口传参
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ResultParam {
    pub this_clan_tag: Option<String>,
    pub this_clan_name: Option<String>,
    pub other_clan_tag: Option<String>,
    pub other_clan_name: Option<String>,
    pub is_national: Option<bool>,
    pub api_key: Option<String>,
}

/// # 获取对战入口
pub async fn get(result_param: ResultParam) -> String {
    let users = get_users(result_param).await;
    let (this, other) = match users {
        Success(u) => {
            u
        }
        Failed(e) => {
            return e.to_string();
        }
    };
    log_info!("{this:?}");
    log_info!("{other:?}");
    "13".to_string()
}

async fn get_users(result_param: ResultParam) -> ResMessage<(User, User), &'static str> {
    let Some(this_clan_tag) = result_param.this_clan_tag else { return Failed("本方标签缺失"); };
    let Some(other_clan_tag) = result_param.other_clan_tag else { return Failed("对方标签缺失"); };
    let Some(api_key) = result_param.api_key else { return Failed("无联盟信息"); };
    let is_national = result_param.is_national.unwrap_or(false);

    // 查询User
    let this_user = user::select_by_tag(&this_clan_tag, is_national).await;

    // 查询本方
    let Some(league) = league::select_by_key(&api_key).await else { return Failed("非法接入"); };
    let this = match this_user {
        Some(user) => {
            user
        }
        None => {
            user::create(&this_clan_tag, result_param.this_clan_name, league.id, is_national).await.unwrap_or(User::default())
        }
    };

    // 查询对方
    let other_user = user::select_by_tag(&other_clan_tag, is_national).await;
    let other = match other_user {
        Some(user) => {
            user
        }
        None => {
            user::create(&other_clan_tag, result_param.other_clan_name, league.id, is_national).await.unwrap_or(User::default())
        }
    };
    Success((this, other))
}