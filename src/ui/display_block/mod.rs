use crate::api::model::{UserReadStatus, UserWatchStatus};
use crate::app::{ActiveBlock, ActiveDisplayBlock, App};
use ratatui::layout::{Alignment, Constraint, Direction, Flex, Layout};
use ratatui::style::Color;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::{layout::Rect, Frame};
mod error;
mod seasonal;
use super::util::get_color;
mod anime_details;
mod details_utils;
mod empty;
mod loading;
mod manga_details;
mod ranking;
mod results;
mod search;
mod suggestion;
mod user;
mod user_anime_list;
mod user_manga_list;

pub fn draw_display_layout(f: &mut Frame, app: &mut App, chunk: Rect) {
    let current_display_block = &app.active_display_block;

    draw_main_display_layout(f, app, chunk);

    match current_display_block {
        ActiveDisplayBlock::Empty => empty::draw_empty(f, app, chunk),

        ActiveDisplayBlock::Help => {} // draw_help_menu(f, app);

        ActiveDisplayBlock::AnimeDetails => anime_details::draw_anime_detail(f, app, chunk),

        ActiveDisplayBlock::MangaDetails => manga_details::draw_manga_detail(f, app, chunk),

        ActiveDisplayBlock::AnimeRanking => ranking::draw_anime_ranking(f, app, chunk),

        ActiveDisplayBlock::MangaRanking => ranking::draw_manga_ranking(f, app, chunk),

        ActiveDisplayBlock::Suggestions => suggestion::draw_suggestions(f, app, chunk),

        ActiveDisplayBlock::UserAnimeList => user_anime_list::draw_user_anime_list(f, app, chunk),

        ActiveDisplayBlock::UserMangaList => user_manga_list::draw_user_manga_list(f, app, chunk),

        ActiveDisplayBlock::UserInfo => user::draw_user_info(f, app, chunk),

        ActiveDisplayBlock::SearchResultBlock => search::draw_search_result(f, app, chunk),

        ActiveDisplayBlock::Seasonal => seasonal::draw_seasonal_anime(f, app, chunk),

        ActiveDisplayBlock::Error => error::draw_error(f, app, chunk),

        ActiveDisplayBlock::Loading => {
            if app.is_loading {
                loading::draw_centered_line(f, app, chunk, "Loading...");
            }
        }
    }
}

pub fn draw_main_display_layout(f: &mut Frame, app: &App, chunk: Rect) {
    let highlight_state = app.active_block == ActiveBlock::DisplayBlock;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(get_color(highlight_state, app.app_config.theme));

    f.render_widget(block, chunk);
}

pub const NAVIGATION_KEYS: [(&str, &str); 5] = [
    ("s", "Switch Type"),
    ("q", "Quit"),
    ("arrows", "Navigate"),
    ("n", "Next page"),
    ("p", "Previous page"),
];
pub const DETAILS_NAVIGATION_KEYS: [(&str, &str); 3] =
    [("s/arrows", "Navigate"), ("q", "Quit"), ("enter", "Select")];

pub fn draw_keys_bar(f: &mut Frame, app: &App, chunk: Rect) -> Rect {
    let [display_chunk, keys_chunk] = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(95), Constraint::Length(2)])
        .areas(chunk);

    let keys = match app.active_display_block {
        ActiveDisplayBlock::AnimeDetails | ActiveDisplayBlock::MangaDetails => {
            DETAILS_NAVIGATION_KEYS.to_vec()
        }
        _ => NAVIGATION_KEYS.to_vec(),
    };
    let key_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            keys.iter()
                .map(|_| Constraint::Percentage(100 / keys.len() as u16))
                .collect::<Vec<Constraint>>(),
        )
        .split(keys_chunk);

    for (i, (key, description)) in keys.iter().enumerate() {
        let block =
            Paragraph::new(format!("{}: {}", key, description)).alignment(Alignment::Center);
        f.render_widget(block, key_chunks[i]);
    }

    display_chunk
}

pub fn get_anime_status_color(status: &UserWatchStatus, app: &App) -> Color {
    match status {
        UserWatchStatus::Completed => app.app_config.theme.status_completed,
        UserWatchStatus::Dropped => app.app_config.theme.status_dropped,
        UserWatchStatus::OnHold => app.app_config.theme.status_on_hold,
        UserWatchStatus::PlanToWatch => app.app_config.theme.status_plan_to_watch,
        UserWatchStatus::Watching => app.app_config.theme.status_watching,
        UserWatchStatus::Other(_) => app.app_config.theme.status_other,
    }
}

pub fn get_manga_status_color(status: &UserReadStatus, app: &App) -> Color {
    match status {
        UserReadStatus::Completed => app.app_config.theme.status_completed,
        UserReadStatus::Dropped => app.app_config.theme.status_dropped,
        UserReadStatus::OnHold => app.app_config.theme.status_on_hold,
        UserReadStatus::PlanToRead => app.app_config.theme.status_plan_to_watch,
        UserReadStatus::Reading => app.app_config.theme.status_watching,
        UserReadStatus::Other(_) => app.app_config.theme.status_other,
    }
}

pub fn center_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

// pub fn first_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
//     let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Start);
//     let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Start);
//     let [area] = vertical.areas(area);
//     let [area] = horizontal.areas(area);
//     area
// }

// pub fn last_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
//     let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::End);
//     let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::End);
//     let [area] = vertical.areas(area);
//     let [area] = horizontal.areas(area);
//     area
// }
