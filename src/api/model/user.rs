use super::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum_macros::{EnumString, IntoStaticStr};

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum UserWatchStatus {
    Watching,
    Completed,
    OnHold,
    Dropped,
    PlanToWatch,
    #[strum(serialize = "add")]
    Other(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnimeStatistics {
    pub num_items_watching: u64,
    pub num_items_completed: u64,
    pub num_items_on_hold: u64,
    pub num_items_dropped: u64,
    pub num_items_plan_to_watch: u64,
    pub num_items: u64,

    pub num_days_watched: f64,
    pub num_days_watching: f64,
    pub num_days_completed: f64,
    pub num_days_on_hold: f64,
    pub num_days_dropped: f64,
    pub num_days: f64,

    pub num_episodes: u64,
    pub num_times_rewatched: u64,
    pub mean_score: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: u64,
    pub name: String,
    pub picture: Option<String>,
    pub gender: Option<String>,
    pub birthday: Option<DateWrapper>,
    pub location: Option<String>,
    pub joined_at: DateTimeWrapper,
    pub anime_statistics: Option<AnimeStatistics>,
    pub time_zone: Option<String>,
    pub is_supporter: Option<bool>,
}
