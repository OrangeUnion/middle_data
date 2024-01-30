use crate::model::league::*;

pub async fn get_leagues(this_league_id: i64, other_league_id: i64) -> (League, League) {
    let this_league = select_by_id(this_league_id).await;
    let this = match this_league {
        Some(league) => {
            league
        }
        None => {
            League::default()
        }
    };

    let other_league = select_by_id(other_league_id).await;
    let other = match other_league {
        Some(league) => {
            league
        }
        None => {
            League::default()
        }
    };

    (this, other)
}