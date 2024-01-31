use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::api::record::RecordParam;
use crate::api::ResMessage;
use crate::api::ResMessage::{Failed, Success};
use crate::log_warn;
use crate::model::league::League;
use crate::model::user::*;

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
            insert(&this_clan_tag, result_param.this_name, league.id, is_global).await.unwrap_or(User::default())
        }
    };

    // 查询对方
    let other = match select_by_tag(&other_clan_tag, is_global).await {
        Some(user) => {
            user
        }
        None => {
            /// # 需要查各盟接口
            /// * 查询返回状态、联盟标识
            match check_state(&other_clan_tag).await {
                Success(re) => {
                    let league_id = union_to_league(&re.union);
                    if league_id == league.id {
                        return Failed("同联盟".to_string());
                    }
                    if re.tag.len() < 3 {
                        return Failed("匹配失败".to_string());
                    }
                    insert(&re.tag, Some(re.name), union_to_league(&re.union), is_global).await.unwrap_or(User::default())
                }
                Failed(err) => { return Failed(err); }
            }
        }
    };
    Success((this, other))
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
struct LeagueJson {
    tag: String,
    name: String,
    state: String,
    union: String,
}

fn union_to_league(union: &str) -> i64 {
    let binding = union.to_lowercase();
    let union = binding.as_str();
    match union {
        "o" => 1,
        "bzlm" => 2,
        "g" => 3,
        _ => 0
    }
}

async fn check_state(mut tag: &str) -> ResMessage<LeagueJson, String> {
    tag = tag.trim_start_matches("#");
    let om = get_om_api(tag).await;
    let bz = get_bz_api(tag).await;
    let gm = get_gm_api(tag).await;

    let mut league = om.clone();
    for lea in vec![&om, &bz, &gm] {
        if !lea.tag.eq(&format!("#{tag}")) || !lea.state.to_lowercase().eq("ok") || lea.tag.len() < 3 {
            continue;
        }
        if lea.tag.eq(&league.tag) {
            let str = format!("{}和{}标签重复", league.union, lea.union);

            return Failed(str);
        }
        league = lea.clone();
    };

    league.tag = format!("#{}", league.tag.trim_start_matches("#"));
    Success(league)
}

async fn get_om_api(tag: &str) -> LeagueJson {
    LeagueJson::default()
}

async fn get_bz_api(tag: &str) -> LeagueJson {
    LeagueJson::default()
}

async fn get_gm_api(tag: &str) -> LeagueJson {
    let url = format!("http://www.coc-hs.cn/tag/{tag}");
    let response = Client::new().get(url).send().await;
    match response {
        Ok(re) => {
            let json = re.json::<LeagueJson>().await.expect("格式不对");
            json
        }
        Err(err) => {
            log_warn!("{err}");
            LeagueJson::default()
        }
    }
}