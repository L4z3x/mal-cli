use super::*;
use crate::event::key::Key;
use ratatui::style::Color;

#[derive(Clone)]
pub struct AppConfig {
    pub keys: KeyBindings,
    pub theme: Theme,
    pub behavior: BehaviorConfig,
    pub nsfw: bool,
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
        }
    }
}

#[derive(Clone)]
pub struct KeyBindings {
    pub help: Key,
    pub back: Key,
    pub search: Key,
}

#[derive(Clone)]
pub struct BehaviorConfig {
    pub seek_milliseconds: u32,
    pub tick_rate_milliseconds: u64,
    pub show_loading_indicator: bool,
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
            },
            behavior: BehaviorConfig {
                seek_milliseconds: 1000,
                tick_rate_milliseconds: 250,
                show_loading_indicator: true,
            },
            nsfw: true,
        })
    }
}
