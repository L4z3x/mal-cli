use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    app::{App, SelectedSearchTab},
    ui::util::get_color,
};

use super::results::draw_results;

pub fn draw_search_result(f: &mut Frame, app: &App, chunk: Rect) {
    let chunk = draw_nav_bar(f, app, chunk);

    let chunk = super::draw_keys_bar(f, app, chunk);

    draw_results(f, app, chunk);
    /*
    we get data as pages and display page by page,navigating through the pages with
     */
}

pub fn draw_nav_bar(f: &mut Frame, app: &App, chunk: Rect) -> Rect {
    let splitted_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(7), Constraint::Percentage(93)])
        .margin(0)
        .split(chunk);

    let bar = splitted_layout[0];

    let block = Block::default().border_style(app.app_config.theme.active);

    f.render_widget(block, bar);

    let tab = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(bar);

    let anime_tab = tab[0];

    let mut is_active = app.search_results.selected_tab == SelectedSearchTab::Anime;
    // handle toggle
    let anime = Span::styled("Anime", get_color(is_active, app.app_config.theme));
    let anime_tab_paragraph = Paragraph::new(anime).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(get_color(is_active, app.app_config.theme)),
    );

    f.render_widget(anime_tab_paragraph, anime_tab);

    let manga_tab = tab[1];

    is_active = app.search_results.selected_tab == SelectedSearchTab::Manga;

    let manga = Span::styled("Manga", get_color(is_active, app.app_config.theme));
    let manga_tab_block = Paragraph::new(manga).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(get_color(is_active, app.app_config.theme)),
    );
    // .block(block);

    f.render_widget(manga_tab_block, manga_tab);
    return splitted_layout[1];
}
