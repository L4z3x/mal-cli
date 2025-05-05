use super::{top_three::draw_top_three, util::get_color};
use crate::app::{
    ActiveBlock, App, ANIME_OPTIONS, ANIME_OPTIONS_RANGE, GENERAL_OPTIONS, GENERAL_OPTIONS_RANGE,
    USER_OPTIONS, USER_OPTIONS_RANGE,
};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListState},
    Frame,
};

pub fn draw_routes(f: &mut Frame, app: &App, layout_chunk: Rect) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(18), Constraint::Percentage(82)])
        .split(layout_chunk);

    draw_user_block(f, app, chunks[0]);

    chunks[1]
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
    let [list_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)])
        .flex(Flex::Center)
        .areas(layout_chunk);
    draw_selectable_list(f, app, list_layout, items, index);
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
    let [list_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)])
        .flex(Flex::Center)
        .areas(layout_chunk);
    draw_selectable_list(f, app, list_layout, items, index);
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
    let [list_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)])
        .flex(Flex::Center)
        .areas(layout_chunk);
    draw_selectable_list(f, app, list_layout, items, index);
}

pub fn draw_user_block(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let [anime_routes_chunk, user_routes_chunk, option_route_chunk, top_three_chunk] =
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Fill(1),
            ])
            .areas(layout_chunk.inner(Margin::new(1, 0)));

    draw_anime_routes(f, app, anime_routes_chunk);
    draw_user_routes(f, app, user_routes_chunk);
    draw_options_routes(f, app, option_route_chunk);
    draw_top_three(f, app, top_three_chunk);
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

    // let centered_rect = display_block::center_area(layout_chunk, 80, 60);

    f.render_stateful_widget(items, layout_chunk, &mut state);
}
