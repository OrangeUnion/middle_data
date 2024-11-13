use crate::api::record::RecordParam;
use crate::api::ResMessage;
use crate::api::ResMessage::{Failed, Success};
use crate::model::league::League;
use crate::model::user::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use void_log::log_warn;

/// # 获取用户
pub async fn get_users(result_param: RecordParam, league: League) -> ResMessage<(User, User), String> {
    // 获取标签
    let Some(this_clan_tag) = result_param.this_tag else { return Failed("本方标签缺失".to_string()); };
    let Some(other_clan_tag) = result_param.other_tag else { return Failed("对方标签缺失".to_string()); };
    let is_global = result_param.is_global.unwrap_or(false);

    // 查询本方
    let this = match select_by_tag(&this_clan_tag, is_global).await {
        Some(user) => {
            user
        }
        None => {
            insert(&this_clan_tag, result_param.this_name, &league.id, is_global).await.unwrap_or(User::default())
        }
    };

    // 查询对方
    let other = match select_by_tag(&other_clan_tag, is_global).await {
        Some(user) => {
            user
        }
        None => {
            // # 需要查各盟接口
            // * 查询返回状态、联盟标识
            match check_state(&other_clan_tag).await {
                Success(re) => {
                    let league_id = LeagueJsonUnion::num(&re.union);
                    if league_id == league.id {
                        return Failed("同联盟".to_string());
                    }
                    if re.tag.len() < 3 {
                        return Failed("匹配失败".to_string());
                    }
                    insert(&re.tag, Some(re.name), &LeagueJsonUnion::num(&re.union), is_global).await.unwrap_or(User::default())
                }
                Failed(err) => { return Failed(err); }
            }
        }
    };
    Success((this, other))
}

type LeagueJsons = Vec<LeagueJson>;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
struct LeagueJson {
    tag: String,
    name: String,
    state: String,
    union: LeagueJsonUnion,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[repr(i64)]
pub enum LeagueJsonUnion {
    O = 1,
    BzLm = 2,
    G = 3,
    #[default]
    Other = 0,
}

impl LeagueJsonUnion {
    fn num(&self) -> i64 {
        match &self {
            LeagueJsonUnion::O => { 1 }
            LeagueJsonUnion::BzLm => { 2 }
            LeagueJsonUnion::G => { 3 }
            LeagueJsonUnion::Other => { 0 }
        }
    }
}

async fn check_state(mut tag: &str) -> ResMessage<LeagueJson, String> {
    tag = tag.trim_start_matches("#");
    let league_jsons = get_league_jsons(tag).await;

    let mut league = LeagueJson::default();
    for lea in league_jsons {
        if !lea.tag.eq(&format!("#{tag}")) || !lea.state.to_lowercase().eq("ok") || lea.tag.len() < 3 {
            continue;
        }
        if lea.tag.eq(&league.tag) {
            let str = format!("{:?}和{:?}标签重复", league.union, lea.union);

            return Failed(str);
        }
        league = lea.clone();
    };

    league.tag = format!("#{}", league.tag.trim_start_matches("#"));
    Success(league)
}

async fn get_league_jsons(tag: &str) -> LeagueJsons {
    // let om_url = format!("http://www.coc-hs.cn/tag/{tag}");
    let bz_url = format!("http://cocbzlm.com:8422/tag/{tag}");
    let gm_url = format!("http://www.coc-hs.cn/tag/{tag}");
    let mut league_jsons = LeagueJsons::new();
    for url in vec![bz_url,gm_url] {
        league_jsons.push(get_client(url).await)
    }
    league_jsons
}

async fn get_client(url:String) -> LeagueJson {
    let response = Client::new().get(url).send().await;
    match response {
        Ok(re) => {
            re.json::<LeagueJson>().await.expect("格式不对")
        }
        Err(err) => {
            log_warn!("{err}");
            LeagueJson::default()
        }
    }
}