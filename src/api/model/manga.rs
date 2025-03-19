use crate::config::app_config::{AppConfig, MangaDisplayType, TitleLanguage};

use super::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr, Display)]
#[strum(serialize_all = "snake_case")]
pub enum MangaRankingType {
    All,
    Manga,
    Novels,
    #[strum(serialize = "oneshots")]
    OneShots,
    Doujinshi,
    Manhwa,
    Manhua,
    #[strum(serialize = "bypopularity")]
    ByPopularity,
    Favorite,
    Other(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankingMangaPair {
    pub node: Manga,
    pub ranking: RankingInfo,
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum MangaMediaType {
    Unknown,
    Manga,
    Novel,
    OneShot,
    Doujinshi,
    Manhwa,
    Manhua,
    #[strum(serialize = "oel")]
    OEL,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum MangaStatus {
    Finished,
    CurrentlyPublishing,
    NotYetPublished,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum UserReadStatus {
    Reading,
    Completed,
    OnHold,
    Dropped,
    PlanToRead,
    #[strum(serialize = "add")]
    Other(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserMangaListStatus {
    pub status: UserReadStatus,
    pub score: u8,
    pub num_volumes_read: u64,
    pub num_chapters_read: u64,
    pub is_rereading: bool,
    pub start_date: Option<DateWrapper>,
    pub finish_date: Option<DateWrapper>,
    pub priority: Option<u8>,
    pub num_times_reread: Option<u64>,
    pub reread_value: Option<u8>,
    pub tags: Option<Vec<String>>,
    pub comments: Option<String>,
    pub updated_at: DateTimeWrapper,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manga {
    pub id: u64,
    pub title: String,
    pub main_picture: Option<Picture>,
    pub alternative_titles: Option<AlternativeTitles>,
    pub start_date: Option<DateWrapper>,
    pub end_date: Option<DateWrapper>,
    pub synopsis: Option<String>,
    pub mean: Option<f64>,
    pub rank: Option<u64>,
    pub popularity: Option<u64>,
    pub num_list_users: Option<u64>,
    pub num_scoring_users: Option<u64>,
    pub nsfw: Option<NSFW>,
    pub genres: Option<Vec<Genre>>,
    pub created_at: Option<DateTimeWrapper>,
    pub updated_at: Option<DateTimeWrapper>,
    pub media_type: Option<MangaMediaType>,
    pub status: Option<MangaStatus>,
    pub my_list_status: Option<UserMangaListStatus>,
    pub num_volumes: Option<u64>,
    pub num_chapters: Option<u64>,
    pub authors: Option<Vec<PersonRole>>,
}

impl Manga {
    pub fn get_title(&self, app_config: &AppConfig, both: bool) -> Vec<String> {
        if both {
            vec![
                self.title.clone(),
                self.alternative_titles
                    .as_ref()
                    .map_or("None".to_string(), |alt| {
                        alt.clone().en.map_or("None".to_string(), |e| e)
                    }),
            ]
        } else {
            match app_config.title_language {
                TitleLanguage::Japanese => vec![self.title.clone()],
                TitleLanguage::English => {
                    if let Some(ref alternative_titles) = self.alternative_titles {
                        if let Some(en) = &alternative_titles.en {
                            if !en.is_empty() {
                                vec![en.clone()]
                            } else {
                                vec![self.title.clone()]
                            }
                        } else {
                            vec![self.title.clone()]
                        }
                    } else {
                        vec![self.title.clone()]
                    }
                }
            }
        }
    }
    pub fn get_num(&self, app_config: &AppConfig) -> String {
        match app_config.manga_display_type {
            MangaDisplayType::Vol => self
                .num_volumes
                .map_or("N/A vol".to_string(), |n| format!("{} vol", n.to_string())),
            MangaDisplayType::Ch => self
                .num_chapters
                .map_or("N/A ch".to_string(), |n| format!("{} ch", n.to_string())),
            MangaDisplayType::Both => format!(
                "{}, {}",
                self.num_volumes
                    .map_or("N/A vol".to_string(), |n| format!("{} vol", n.to_string())),
                self.num_chapters
                    .map_or("N/A ch".to_string(), |n| format!("{} ch", n.to_string()))
            ),
        }
    }
}
