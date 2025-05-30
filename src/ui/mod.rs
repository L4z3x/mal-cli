pub mod help;
mod side_menu;
mod top_three;
pub mod util;
use crate::app::*;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use util::get_color;
mod display_block;

pub fn draw_main_layout(f: &mut Frame, app: &mut App) {
    let margin = util::get_main_layout_margin(app);
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .margin(margin)
        .split(f.area());

    // Search Input and help
    draw_input_and_help_box(f, app, parent_layout[0]);

    // Draw dashboard
    let chunk = side_menu::draw_routes(f, app, parent_layout[1]);
    display_block::draw_display_layout(f, app, chunk);
}

pub fn draw_input_and_help_box(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let [search_chunk, title_chunk] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(17), Constraint::Percentage(82)])
        .flex(Flex::SpaceBetween)
        .areas(layout_chunk);
    // removing the little gap
    let [_, search_chunk] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(1), Constraint::Fill(1)])
        .areas(search_chunk);
    let current_block = app.active_block;

    let highlight_state = current_block == ActiveBlock::Input;

    let input_string: String = app.input.iter().collect();
    let lines = Span::from(input_string);
    let input = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(Span::styled(
                "Search",
                get_color(highlight_state, app.app_config.theme),
            ))
            .border_style(get_color(highlight_state, app.app_config.theme)),
    );
    f.render_widget(input, search_chunk);

    let mut title = app.display_block_title.clone();
    if title.is_empty() {
        title = "Home".to_string(); // Default title , since i couldn't initialize it in app.rs:15
    }
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.app_config.theme.inactive));

    let lines = Line::from(Span::from(title))
        .alignment(Alignment::Center)
        .style(Style::default().fg(app.app_config.theme.banner));

    let help = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(app.app_config.theme.banner));
    f.render_widget(help, title_chunk);
}

pub fn format_number_with_commas(number: u64) -> String {
    let num_str = number.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in num_str.chars().rev() {
        if count == 3 {
            result.push(',');
            count = 0;
        }
        result.push(c);
        count += 1;
    }

    result.chars().rev().collect()
}

fn get_end_index(app: &App, typ: &str) -> usize {
    match typ {
        "anime" => {
            let data_len = app.search_results.anime.as_ref().unwrap().data.len();
            if app.start_card_list_index as usize + DISPLAY_RAWS_NUMBER * DISPLAY_COLUMN_NUMBER
                > data_len - 1
            // end is bigger than last index
            {
                data_len - 1
            } else {
                // start index + 14 to get the last index
                app.start_card_list_index as usize + DISPLAY_COLUMN_NUMBER * DISPLAY_RAWS_NUMBER - 1
            }
        }
        "manga" => {
            let data_len = app.search_results.manga.as_ref().unwrap().data.len();
            if app.start_card_list_index as usize + DISPLAY_RAWS_NUMBER * DISPLAY_COLUMN_NUMBER
                > data_len - 1
            // end is bigger than last index in the data
            {
                data_len - 1
            } else {
                // start index + 14 to get the last index
                app.start_card_list_index as usize + DISPLAY_COLUMN_NUMBER * DISPLAY_RAWS_NUMBER - 1
            }
        }
        //TODO: handle these cases:
        "anime_ranking" => {
            let data_len = app.anime_ranking_data.as_ref().unwrap().data.len();
            if app.start_card_list_index as usize + DISPLAY_RAWS_NUMBER * DISPLAY_COLUMN_NUMBER
                > data_len - 1
            // end is bigger than last index in the data
            {
                data_len - 1
            } else {
                // start index + 14 to get the last index
                app.start_card_list_index as usize + DISPLAY_COLUMN_NUMBER * DISPLAY_RAWS_NUMBER - 1
            }
        }
        "manga_ranking" => {
            let data_len = app.manga_ranking_data.as_ref().unwrap().data.len();
            if app.start_card_list_index as usize + DISPLAY_RAWS_NUMBER * DISPLAY_COLUMN_NUMBER
                > data_len - 1
            // end is bigger than last index in the data
            {
                data_len - 1
            } else {
                // start index + 14 to get the last index
                app.start_card_list_index as usize + DISPLAY_COLUMN_NUMBER * DISPLAY_RAWS_NUMBER - 1
            }
        }
        _ => panic!("Unknown type: {}", typ),
    }
}

pub fn get_end_card_index(app: &App) -> usize {
    match app.active_display_block {
        ActiveDisplayBlock::SearchResultBlock => match app.search_results.selected_tab {
            SelectedSearchTab::Manga => get_end_index(app, "manga"),
            SelectedSearchTab::Anime => get_end_index(app, "anime"),
        },
        ActiveDisplayBlock::AnimeRanking => get_end_index(app, "anime_ranking"),
        ActiveDisplayBlock::MangaRanking => get_end_index(app, "manga_ranking"),
        _ => {
            // Default case, if no specific block is active
            get_end_index(app, "anime")
        }
    }
}
