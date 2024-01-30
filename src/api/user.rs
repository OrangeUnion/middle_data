use crate::api::record::RecordParam;
use crate::api::ResMessage;
use crate::model::league::League;
use crate::model::user::*;

/// # 获取用户
pub async fn get_users(result_param: RecordParam, league: League) -> ResMessage<(User, User), &'static str> {
    // 获取标签
    let Some(this_clan_tag) = result_param.this_tag else { return ResMessage::Failed("本方标签缺失"); };
    let Some(other_clan_tag) = result_param.other_tag else { return ResMessage::Failed("对方标签缺失"); };
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
            insert(&other_clan_tag, result_param.other_name, league.id, is_global).await.unwrap_or(User::default())
        }
    };
    ResMessage::Success((this, other))
}