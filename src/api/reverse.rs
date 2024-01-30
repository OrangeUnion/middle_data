use serde::{Deserialize, Serialize};
use crate::api::ResMessage;
use crate::api::ResMessage::*;
use crate::model::{league, record, user};
use crate::model::record::Record;
use crate::model::reverse::{insert, Reverse};
use crate::model::user::{select_by_tag, User};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ReverseApi {
    pub round_id: i64,
    pub this_clan: ClanInfo,
    pub other_clan: ClanInfo,
    pub is_global: bool,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ClanInfo {
    pub id: i64,
    pub old_score: i64,
    pub new_score: i64,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ReverseParam {
    pub round_id: Option<i64>,
    pub this_tag: Option<String>,
    pub other_tag: Option<String>,
    pub is_global: Option<bool>,
    pub api_key: Option<String>,
}

pub async fn set(reverse_param: ReverseParam) -> ResMessage<ReverseApi, &'static str> {
    // 验证token
    let Some(api_key) = &reverse_param.api_key else { return Failed("无联盟信息"); };
    if let None = league::select_by_key(&api_key).await { return Failed("非法接入"); };

    // 存库
    let (this_rev, other_rev) = match save_reverse(reverse_param.clone()).await {
        Success(revs) => { revs }
        Failed(err) => { return Failed(err) }
    };

    // 组装返回值
    let api = ReverseApi {
        round_id: reverse_param.round_id.unwrap_or(0),
        this_clan: ClanInfo {
            id: this_rev.id,
            old_score: this_rev.this_old_score,
            new_score: this_rev.this_new_score,
        },
        other_clan: ClanInfo {
            id: other_rev.id,
            old_score: other_rev.other_old_score,
            new_score: other_rev.other_new_score,
        },
        is_global: reverse_param.is_global.unwrap_or(false),
    };
    Success(api)
}

/// # 设置逆转入口
pub async fn save_reverse(reverse_param: ReverseParam) -> ResMessage<(Reverse, Reverse), &'static str> {
    // 获取用户
    let users = get_users(&reverse_param).await;
    let (this, other) = match users {
        Success(users) => { users }
        Failed(err) => { return Failed(err); }
    };

    // 获取原对战信息
    let record = match get_record(&reverse_param, &this).await {
        Success(re) => { re }
        Failed(err) => { return Failed(err); }
    };

    // 逆转积分
    let this_new_score = this.score - record.result * 2;
    let other_new_score = other.score + record.result * 2;

    // 封装逆转记录
    let this_insert_data = Reverse::insert_data(reverse_param.round_id.unwrap_or(0),
                                                this.id, this.score, this_new_score,
                                                other.id, other.score, other_new_score, record.result, -record.result);
    let other_insert_data = Reverse::insert_data(reverse_param.round_id.unwrap_or(0),
                                                 other.id, other.score, other_new_score,
                                                 this.id, this.score, this_new_score, -record.result, record.result);

    // 逆转记录入库，用户积分修改
    let this_insert = match insert(this_insert_data).await {
        Some(rev) => {
            user::update_score(this.id, this_new_score).await;
            rev
        }
        None => { return Failed("本方逆转记录失败"); }
    };
    let other_insert = match insert(other_insert_data).await {
        Some(rev) => {
            user::update_score(other.id, other_new_score).await;
            rev
        }
        None => { return Failed("对方逆转记录失败"); }
    };

    Success((this_insert, other_insert))
}

async fn get_record(reverse_param: &ReverseParam, user: &User) -> ResMessage<Record, &'static str> {
    let this_record = match record::select_by_user(reverse_param.round_id.unwrap_or(0), user.id).await {
        Some(re) => { re }
        None => { return Failed("本轮无对战信息"); }
    };
    Success(this_record)
}

async fn get_users(reverse_param: &ReverseParam) -> ResMessage<(User, User), &'static str> {
    // 获取标签
    let Some(this_clan_tag) = &reverse_param.this_tag else { return Failed("本方标签缺失"); };
    let Some(other_clan_tag) = &reverse_param.other_tag else { return Failed("对方标签缺失"); };
    let is_global = reverse_param.is_global.unwrap_or(false);

    // 查询本方
    let this = match select_by_tag(this_clan_tag, is_global).await {
        Some(user) => {
            user
        }
        None => {
            User::default()
        }
    };

    // 查询对方
    let other = match select_by_tag(other_clan_tag, is_global).await {
        Some(user) => {
            user
        }
        None => {
            User::default()
        }
    };
    Success((this, other))
}