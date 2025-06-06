use std::{
    fs,
    path::{Path, PathBuf},
};

use super::*;
use crate::{
    api::model::{AnimeRankingType, MangaRankingType},
    event::key::Key,
};
use log::LevelFilter;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(skip_deserializing, skip_serializing)]
    pub paths: CachePaths,
    pub keys: KeyBindings,
    pub theme: Theme,
    pub behavior: BehaviorConfig,
    pub nsfw: bool,
    pub title_language: TitleLanguage,
    pub manga_display_type: MangaDisplayType,
    // pub first_top_three_block: TopThreeBlock,
    pub top_three_anime_types: Vec<AnimeRankingType>,
    pub top_three_manga_types: Vec<MangaRankingType>,
    pub navigation_stack_limit: u32,
    pub search_limit: u64,
    pub log_level: LevelFilter,
    pub max_cached_images: u16,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum TitleLanguage {
    Japanese,
    English,
}

#[derive(Copy, Deserialize, Serialize, Clone, Debug)]
pub struct Theme {
    pub mal_color: Color,
    pub active: Color,
    pub banner: Color,
    pub hovered: Color,
    pub text: Color,
    pub selected: Color,
    pub error_border: Color,
    pub error_text: Color,
    pub inactive: Color,
    pub status_completed: Color,
    pub status_dropped: Color,
    pub status_on_hold: Color,
    pub status_watching: Color,
    pub status_plan_to_watch: Color,
    pub status_other: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            mal_color: Color::Rgb(46, 81, 162),
            active: Color::Cyan,
            banner: Color::Rgb(46, 81, 162),
            hovered: Color::Magenta,
            selected: Color::LightCyan,
            text: Color::White,
            error_border: Color::Red,
            error_text: Color::LightRed,
            inactive: Color::Gray,
            status_completed: Color::Green,
            status_dropped: Color::Gray,
            status_on_hold: Color::Yellow,
            status_watching: Color::Blue,
            status_plan_to_watch: Color::LightMagenta,
            status_other: Color::DarkGray,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct KeyBindings {
    pub help: Key,
    pub back: Key,
    pub search: Key,
    pub toggle: Key,
    pub next_state: Key,
    pub open_popup: Key,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BehaviorConfig {
    // pub show_loading_indicator: bool,
    // pub seek_milliseconds: u64,
    pub tick_rate_milliseconds: u64,
    pub show_logger: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MangaDisplayType {
    Vol,
    Ch,
    Both,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let paths = get_cache_dir()?;

        Ok(Self {
            paths,
            theme: Theme::default(),
            keys: KeyBindings {
                help: Key::Char('?'),
                back: Key::Char('q'),
                search: Key::Char('/'),
                toggle: Key::Char('s'),
                open_popup: Key::Char('r'),
                next_state: Key::Ctrl('p'),
            },
            behavior: BehaviorConfig {
                tick_rate_milliseconds: 500,
                show_logger: false,
            },
            nsfw: false,
            title_language: TitleLanguage::English,
            manga_display_type: MangaDisplayType::Both,
            top_three_anime_types: vec![
                AnimeRankingType::Airing,
                AnimeRankingType::All,
                AnimeRankingType::Upcoming,
                AnimeRankingType::Movie,
                AnimeRankingType::Special,
                AnimeRankingType::OVA,
                AnimeRankingType::TV,
                AnimeRankingType::ByPopularity,
                AnimeRankingType::Favorite,
            ],
            top_three_manga_types: vec![
                MangaRankingType::All,
                MangaRankingType::Manga,
                MangaRankingType::Novels,
                MangaRankingType::OneShots,
                MangaRankingType::Doujinshi,
                MangaRankingType::Manhwa,
                MangaRankingType::Manhua,
                MangaRankingType::ByPopularity,
                MangaRankingType::Favorite,
            ],
            navigation_stack_limit: 15,
            search_limit: 30,
            max_cached_images: 15,
            log_level: LevelFilter::Debug,
        })
    }

    pub fn load() -> Result<Self, ConfigError> {
        // check file exists
        // do not get paths from config file,always use the default paths
        let config_file = dirs::home_dir()
            .ok_or(ConfigError::PathError)?
            .join(CONFIG_DIR)
            .join(APP_CONFIG_DIR)
            .join(_CONFIG_FILE);
        if !config_file.exists() {
            // if config file doesn't exist, create default config
            fs::create_dir_all(config_file.parent().unwrap())?;
            let default_config = Self::new()?;

            fs::write(&config_file, serde_yaml::to_string(&default_config)?)?;
            Ok(default_config)
        } else {
            // if config file exists, read it
            let content = fs::read_to_string(&config_file).map_err(|_| ConfigError::ReadError)?;
            let config: Self = serde_yaml::from_str(&content).map_err(ConfigError::ParseError)?;

            Ok(config)
        }
    }
}

fn get_cache_dir() -> Result<CachePaths, ConfigError> {
    match dirs::home_dir() {
        Some(home) => {
            let path = Path::new(&home);

            // cache dir:
            let home_cache_dir = path.join(CACHE_DIR);

            let cache_dir = home_cache_dir.join(APP_CACHE_DIR);

            let picture_cache_dir = cache_dir.join(PICTURE_CACHE_DIR);

            let data_file_path = cache_dir.join(DATA_FILE);

            if !home_cache_dir.exists() {
                fs::create_dir(&home_cache_dir)?;
            }
            if !cache_dir.exists() {
                fs::create_dir(&cache_dir)?;
            }

            if !picture_cache_dir.exists() {
                fs::create_dir(&picture_cache_dir)?;
            }

            let paths = CachePaths {
                picture_cache_dir_path: picture_cache_dir.to_path_buf(),
                data_file_path,
            };

            Ok(paths)
        }
        None => Err(ConfigError::PathError),
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CachePaths {
    pub picture_cache_dir_path: PathBuf,
    pub data_file_path: PathBuf,
}
impl Default for CachePaths {
    fn default() -> Self {
        get_cache_dir().ok().unwrap()
    }
}
