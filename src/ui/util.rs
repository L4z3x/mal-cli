use ratatui::style::{Modifier, Style};

// use crate::api::model::*;
use crate::app::App;
use crate::config::app_config::Theme;

pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

pub fn get_color(is_active: bool, theme: Theme) -> Style {
    match is_active {
        true => Style::default()
            .fg(theme.selected)
            .add_modifier(Modifier::BOLD),
        _ => Style::default().fg(theme.inactive),
    }
}

pub fn capitalize_each_word(text: String) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn get_main_layout_margin(app: &App) -> u16 {
    if app.size.height > SMALL_TERMINAL_HEIGHT {
        1
    } else {
        0
    }
}
