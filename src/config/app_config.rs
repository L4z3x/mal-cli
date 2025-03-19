use super::*;
use crate::{
    api::model::{AnimeRankingType, MangaRankingType},
    event::key::Key,
};
use ratatui::style::Color;

#[derive(Clone)]
pub struct AppConfig {
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
}

#[derive(Clone, Debug)]
pub enum TitleLanguage {
    Japanese,
    English,
}

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub active: Color,
    pub banner: Color,
    pub hint: Color,
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
            active: Color::Cyan,
            banner: Color::LightCyan,
            hint: Color::Yellow,
            hovered: Color::Magenta,
            text: Color::White,
            selected: Color::LightCyan,
            error_border: Color::Red,
            error_text: Color::LightRed,
            inactive: Color::Gray,
            status_completed: Color::Green,
            status_dropped: Color::Gray,
            status_on_hold: Color::Yellow,
            status_watching: Color::Blue,
            status_plan_to_watch: Color::Cyan,
            status_other: Color::White,
        }
    }
}

#[derive(Clone)]
pub struct KeyBindings {
    pub help: Key,
    pub back: Key,
    pub search: Key,
    pub toggle: Key,
    pub next_state: Key,
    pub open_popup: Key,
}

#[derive(Clone)]
pub struct BehaviorConfig {
    pub seek_milliseconds: u32,
    pub tick_rate_milliseconds: u64,
    pub show_loading_indicator: bool,
}

#[derive(Clone, Debug)]
pub enum MangaDisplayType {
    Vol,
    Ch,
    Both,
}

// TODO: get app config from file
impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        Ok(Self {
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
                seek_milliseconds: 1000,
                tick_rate_milliseconds: 250,
                show_loading_indicator: true,
            },
            nsfw: true,
            title_language: TitleLanguage::English,
            manga_display_type: MangaDisplayType::Both,
            // first_top_three_block: TopThreeBlock::Anime(AnimeRankingType::Airing),
            top_three_anime_types: vec![
                AnimeRankingType::Airing,
                AnimeRankingType::All,
                AnimeRankingType::Upcoming,
                AnimeRankingType::Movie,
            ],
            top_three_manga_types: vec![MangaRankingType::All, MangaRankingType::Manga],
            navigation_stack_limit: 6,
        })
    }
}
