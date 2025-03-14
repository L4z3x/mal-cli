use super::{display_block, top_three::draw_top_three, util::get_color};
use crate::app::{
    ActiveBlock, App, ANIME_OPTIONS, ANIME_OPTIONS_RANGE, GENERAL_OPTIONS, GENERAL_OPTIONS_RANGE,
    USER_OPTIONS, USER_OPTIONS_RANGE,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListState},
    Frame,
};

pub fn draw_routes(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(18), Constraint::Percentage(82)].as_ref())
        .split(layout_chunk);

    draw_user_block(f, app, chunks[0]);

    // let current_route = app.active_block;

    display_block::draw_display_layout(f, app, chunks[1]);
}

pub fn draw_anime_routes(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let current_block = app.active_block;
    let highlight_state = current_block == ActiveBlock::Anime;

    let items: Vec<Line> = ANIME_OPTIONS
        .iter()
        .map(|i| {
            Line::from(*i)
                .alignment(Alignment::Center)
                .style(Style::default().fg(app.app_config.theme.text))
        })
        .collect();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(
            "Anime",
            get_color(highlight_state, app.app_config.theme),
        ))
        .border_style(get_color(highlight_state, app.app_config.theme));

    f.render_widget(block, layout_chunk);

    let mut index = Some(app.library.selected_index);
    if !ANIME_OPTIONS_RANGE.contains(&app.library.selected_index) {
        index = None;
    }

    draw_selectable_list(f, app, layout_chunk, items, index)
}

pub fn draw_user_routes(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let current_block = app.active_block;
    let highlight_state = current_block == ActiveBlock::User;

    let items: Vec<Line> = USER_OPTIONS
        .iter()
        .map(|i| {
            Line::from(*i)
                .alignment(Alignment::Center)
                .style(Style::default().fg(app.app_config.theme.text))
        })
        .collect();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(
            "User",
            get_color(highlight_state, app.app_config.theme),
        ))
        .border_style(get_color(highlight_state, app.app_config.theme));

    f.render_widget(block, layout_chunk);

    let mut index = Some(app.library.selected_index);
    if !USER_OPTIONS_RANGE.contains(&app.library.selected_index) {
        index = None;
    }

    draw_selectable_list(f, app, layout_chunk, items, index);
}

pub fn draw_options_routes(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let current_block = app.active_block;
    let highlight_state = current_block == ActiveBlock::Option;

    let items: Vec<Line> = GENERAL_OPTIONS
        .iter()
        .map(|i| {
            Line::from(*i)
                .alignment(Alignment::Center)
                .style(Style::default().fg(app.app_config.theme.text))
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(
            "Options",
            get_color(highlight_state, app.app_config.theme),
        ))
        .border_style(get_color(highlight_state, app.app_config.theme));

    f.render_widget(block, layout_chunk);

    let mut index = Some(app.library.selected_index);
    if !GENERAL_OPTIONS_RANGE.contains(&app.library.selected_index) {
        index = None;
    }

    draw_selectable_list(f, app, layout_chunk, items, index);
}

pub fn draw_user_block(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(17),
                Constraint::Percentage(17),
                Constraint::Percentage(17),
                Constraint::Percentage(100 - 17 * 3),
            ]
            .as_ref(),
        )
        .split(layout_chunk.inner(Margin::new(1, 0)));

    draw_anime_routes(f, app, chunks[0]);
    draw_user_routes(f, app, chunks[1]);
    draw_options_routes(f, app, chunks[2]);
    draw_top_three(f, app, chunks[3]);
}

pub fn draw_selectable_list(
    f: &mut Frame,
    app: &App,
    layout_chunk: Rect,
    items: Vec<Line>,
    selected_index: Option<usize>,
) {
    let mut state = ListState::default();
    if selected_index.is_some() {
        // dbg!(selected_index.unwrap() % items.len());
        state.select(Some(selected_index.unwrap() % items.len()));
    }

    // choose color based on hover state
    let items = List::new(items).highlight_style(
        Style::default()
            .fg(app.app_config.theme.selected)
            .add_modifier(Modifier::BOLD),
    );

    let centered_rect = center_rect(layout_chunk, items.len() as u16);
    f.render_stateful_widget(items, centered_rect, &mut state);
}

pub fn center_rect(area: Rect, line_num: u16) -> Rect {
    // the split is just ughhh
    let total_height = area.height;
    let content_height = line_num; // Number of lines to center

    let top_padding = (total_height.saturating_sub(content_height)) / 2;

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(top_padding),    // Space before the text
            Constraint::Length(content_height), // The actual text
            Constraint::Min(0),                 // Space after the text (remaining area)
        ])
        .split(area)[1] // Get the centered text area
}
// Layout::default()
//     .direction(Direction::Vertical)
//     .constraints([
//         Constraint::Length(1),
//         Constraint::Percentage(10),
//         Constraint::Percentage(80),
//         Constraint::Percentage(10),
//         Constraint::Length(1),
//     ])
//     .split(area)[2];
// let block = Block::default().borders(Borders::ALL);
// // .border_type(BorderType::Rounded)
// // .style(Style::default().fg(app.app_config.theme.active));
// // f.render_widget(block.clone(), a[1]);
// // f.render_widget(block, a[3]);

//     a[2]
// }

// pub fn draw_top_3
