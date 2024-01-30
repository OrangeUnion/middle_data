use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::api::{ResMessage, round};
use crate::api::league::get_leagues;
use crate::api::ResMessage::*;
use crate::api::user::get_users;
use crate::log_info;
use crate::model::{league, user};
use crate::model::league::League;
use crate::model::record::*;
use crate::model::user::User;

pub struct ResInfo {
    pub this_record: Record,
    pub other_record: Record,
    pub this_user: User,
    pub other_user: User,
}

/// # Result Json映射
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RecordApi {
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
    pub old_score: i64,
    pub new_score: i64,
    pub league: ClanInfoLeague,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ClanInfoLeague {
    id: i64,
    name: String,
}

/// # result接口传参
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RecordParam {
    pub this_tag: Option<String>,
    pub this_name: Option<String>,
    pub other_tag: Option<String>,
    pub other_name: Option<String>,
    pub is_global: Option<bool>,
    pub api_key: Option<String>,
}

/// # 获取对战入口
pub async fn get(result_param: RecordParam) -> ResMessage<RecordApi, &'static str> {
    // 验证token
    let Some(api_key) = &result_param.api_key else { return Failed("无联盟信息"); };
    let Some(league) = league::select_by_key(api_key).await else { return Failed("非法接入"); };

    let res_info = match save_record(result_param, league).await {
        Success(re) => { re }
        Failed(err) => { return Failed(err); }
    };
    log_info!("{}",res_info.this_record.create_time);

    let (this_league, other_league) = get_leagues(res_info.this_user.league_id, res_info.other_user.league_id).await;

    let record_result = match res_info.this_record.result.cmp(&0) {
        Ordering::Less => { "lose".to_string() }
        Ordering::Equal => { "failed".to_string() }
        Ordering::Greater => { "win".to_string() }
    };

    let result = RecordApi {
        time_round: res_info.this_record.round_id,
        this_clan: ClanInfo {
            tag: res_info.this_user.tag,
            name: res_info.this_user.name.unwrap_or("None".to_string()),
            old_score: res_info.this_record.old_score,
            new_score: res_info.this_record.new_score,
            league: ClanInfoLeague {
                id: this_league.id,
                name: this_league.name,
            },
        },
        other_clan: ClanInfo {
            tag: res_info.other_user.tag,
            name: res_info.other_user.name.unwrap_or("None".to_string()),
            old_score: res_info.other_record.old_score,
            new_score: res_info.other_record.new_score,
            league: ClanInfoLeague {
                id: other_league.id,
                name: other_league.name,
            },
        },
        result: record_result,
        create_time: res_info.this_record.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    Success(result)
}

async fn save_record(result_param: RecordParam, league: League) -> ResMessage<ResInfo, &'static str> {
    // 获取两边用户信息
    let users = get_users(result_param, league).await;
    let (this, other) = match users {
        Success(res) => { res }
        Failed(err) => {
            return Failed(err);
        }
    };

    if let (Some(this_record), Some(other_record)) = get_record(&this, &other).await {
        Success(ResInfo {
            this_record,
            other_record,
            this_user: this,
            other_user: other,
        })
    } else {
        // 调用Recode封装，获取对战结果判断
        let record = set_record(&this, &other).await;

        // 写入对战结果
        let save_record = (insert(record.0.clone()).await, insert(record.1.clone()).await);

        Success(ResInfo {
            this_record: save_record.0.unwrap_or(Default::default()),
            other_record: save_record.1.unwrap_or(Default::default()),
            this_user: this,
            other_user: other,
        })
    }
}

/// # 查询对战结果
async fn get_record(this: &User, other: &User) -> (Option<Record>, Option<Record>) {
    let round_id = round::now_round().await.id;
    let this_record = select_by_user(round_id, this.id).await;
    let other_record = select_by_user(round_id, other.id).await;
    (this_record, other_record)
}

/// # 计算对战结果
async fn set_record(this: &User, other: &User) -> (Record, Record) {
    let round = round::now_round().await;
    let (this_id, other_id) = (this.id, other.id);
    let mut this_score = this.score;
    let mut other_score = other.score;

    let result = match this.score.cmp(&other.score) {
        Ordering::Less => {
            this_score += 1;
            other_score -= 1;
            1
        }
        Ordering::Equal => {
            this_score += 1;
            other_score -= 1;
            1
        }
        Ordering::Greater => {
            this_score -= 1;
            other_score += 1;
            -1
        }
    };

    // 更新积分
    user::update_score(this.id, this_score).await;
    user::update_score(other.id, other_score).await;

    // 封装Record记录
    let this_insert_data = Record::insert_data(round.id, this_id, other_id, this.score, this_score, result);
    let other_insert_data = Record::insert_data(round.id, other_id, this_id, other.score, other_score, 0 - result);
    (this_insert_data, other_insert_data)
}