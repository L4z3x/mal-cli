use crate::app::{ActiveBlock, ActiveDisplayBlock, App};
use crate::ui::{draw_error, draw_help_menu};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{layout::Rect, Frame};

use super::util::get_color;
mod empty;
mod loading;
mod results;
mod search;
pub fn draw_display_layout(f: &mut Frame, app: &App, chunk: Rect) {
    let current_display_block = &app.active_display_block;

    draw_main_display_layout(f, app, chunk);

    match current_display_block {
        ActiveDisplayBlock::Empty => {
            // drow mal-cli
            empty::draw_empty(f, app, chunk);
        }

        ActiveDisplayBlock::Help => {
            // draw_help_menu(f, app);
        }

        ActiveDisplayBlock::AnimeRanking => {}

        ActiveDisplayBlock::MangaRanking => {}

        ActiveDisplayBlock::UserAnimeList => {}

        ActiveDisplayBlock::UserMangaList => {}

        ActiveDisplayBlock::UserInfo => {}

        ActiveDisplayBlock::SearchResultBlock => {
            search::draw_search_result(f, app, chunk);
        }

        ActiveDisplayBlock::Seasonal => {}

        ActiveDisplayBlock::Error => {
            // draw_error(f, app);
        }

        ActiveDisplayBlock::Loading => {
            if app.is_loading {
                loading::draw_loading(f, app, chunk);
            }
        }

        _ => {}
    }
}

pub fn draw_main_display_layout(f: &mut Frame, app: &App, chunk: Rect) {
    let highlight_state = app.active_block == ActiveBlock::DisplayBlock;

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(get_color(highlight_state, app.app_config.theme));

    f.render_widget(block, chunk);
}

pub const NAVIGATION_KEYS: [(&str, &str); 5] = [
    ("s", "Switch results"),
    ("q", "Quit"),
    ("arrows", "Navigate"),
    ("n", "Next page"),
    ("p", "Previous page"),
];

pub fn draw_keys_bar(f: &mut Frame, app: &App, chunk: Rect) -> Rect {
    let splitted_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(chunk);

    let key_bar = splitted_layout[1];
    let key_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            NAVIGATION_KEYS
                .iter()
                .map(|_| Constraint::Percentage(100 / NAVIGATION_KEYS.len() as u16))
                .collect::<Vec<Constraint>>(),
        )
        .split(key_bar);

    for (i, (key, description)) in NAVIGATION_KEYS.iter().enumerate() {
        let block =
            Paragraph::new(format!("{}: {}", key, description)).alignment(Alignment::Center);
        f.render_widget(block, key_chunks[i]);
    }
    //todo: for the keys handle slpitting the bar into equal blocks and filling them with the keys

    splitted_layout[0]
}
