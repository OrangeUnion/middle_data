use crate::model::round::*;
use serde::{Deserialize, Serialize};
use void_log::log_info;

/// # round接口传参
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RoundParam {
    pub time: Option<String>,
}

pub async fn now_round() -> Round {
    match select_by_now().await {
        Some(r) => { r }
        None => { Round::default() }
    }
}

pub async fn create_round(round_param: RoundParam) -> Round {
    log_info!("{:?}", round_param);
    let mut round = Round::default();
    if let Some(time) = round_param.time {
        round = insert(&time).await.unwrap_or(Round::default());
    }
    round
}