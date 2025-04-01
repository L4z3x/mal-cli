use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{
    api::model::{AnimeMediaType, UserReadStatus, UserWatchStatus},
    app::{ActiveBlock, App},
    config::app_config::Theme,
    ui::util::get_color,
};

use super::results::construct_cards_with_data;

pub fn draw_user_anime_list(f: &mut Frame, app: &App, chunk: Rect) {
    // order matters, it should be the same as the Status enum
    let statuses = vec![
        "add",
        "watching",
        "completed",
        "on_hold",
        "dropped",
        "plan_to_watch",
    ];

    let chunk = draw_user_list_nav_bar(f, app, chunk, true, statuses);

    let chunk = super::draw_keys_bar(f, app, chunk);
    draw_anime_list_results(f, app, chunk);
}

pub fn draw_user_list_nav_bar(
    f: &mut Frame,
    app: &App,
    chunk: Rect,
    is_anime: bool,
    status_list: Vec<&str>,
) -> Rect {
    let layout: [Rect; 2] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(7), Constraint::Percentage(93)])
        .margin(0)
        .areas(chunk);

    let bar = layout[0];

    let block = Block::default().border_style(app.app_config.theme.active);
    f.render_widget(block, bar);

    let tabs = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(100 / status_list.len() as u16);
            status_list.len()
        ])
        .flex(Flex::SpaceAround)
        .split(bar);

    for (i, status) in status_list.iter().enumerate() {
        let is_active = if is_anime {
            eq_anime_status(&app.anime_list_status, status)
        } else {
            eq_manga_status(&app.manga_list_status, status)
        };
        let status = get_status_title(status, is_anime);
        draw_tab(f, tabs[i], status, is_active, app.app_config.theme);
    }

    layout[1]
}

pub fn get_status_title(status: &str, is_anime: bool) -> String {
    match status {
        "watching" => "Watching".to_string(),
        "completed" => "Completed".to_string(),
        "on_hold" => "On Hold".to_string(),
        "dropped" => "Dropped".to_string(),
        "plan_to_watch" => "Plan To Watch".to_string(),
        "plan_to_read" => "Plan To Read".to_string(),
        "reading" => "Reading".to_string(),
        "add" => {
            if is_anime {
                "All Anime".to_string()
            } else {
                "All Manga".to_string()
            }
        }
        _ => status.to_string(),
    }
}

pub fn draw_tab(f: &mut Frame, tab_chunk: Rect, content: String, is_active: bool, theme: Theme) {
    let content_span = Span::styled(content, get_color(is_active, theme));
    let paragraph = Paragraph::new(content_span)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(get_color(is_active, theme)),
        );

    f.render_widget(paragraph, tab_chunk);
}

fn draw_anime_list_results(f: &mut Frame, app: &App, chunk: Rect) {
    let results = app.search_results.anime.as_ref().unwrap();
    if results.data.is_empty() {
        // draw_no_results(f, app, chunk);
        return;
    }
    let cards_results = construct_cards_with_data(chunk, results);

    let cards = cards_results.0;
    let components = cards_results.1;

    let selected_card_index = app.search_results.selected_display_card_index.unwrap_or(0);

    for (index, component) in components.iter().enumerate() {
        if index >= cards.len() {
            break;
        }

        let is_active =
            index == selected_card_index && app.active_block == ActiveBlock::DisplayBlock;

        let title_style = get_color(is_active, app.app_config.theme);

        let anime_title = &component.get_title(&app.app_config, false)[0];

        let anime_title = Line::styled(anime_title, title_style);

        let media_type: &str = Into::<&str>::into(
            component
                .media_type
                .as_ref()
                .map_or(AnimeMediaType::Other("Unknown".to_string()), |media_type| {
                    media_type.clone()
                }),
        );

        let score = component
            .my_list_status
            .as_ref()
            .map_or("-".to_string(), |status| {
                if status.score.to_string() != "0" {
                    status.score.to_string()
                } else {
                    "-".to_string()
                }
            });

        let anime_type_and_score = Line::from(format!("{} {}", media_type, score));

        let tags = Line::from(
            component
                .my_list_status
                .as_ref()
                .map_or("".to_string(), |status| {
                    status
                        .tags
                        .as_ref()
                        .map_or("".to_string(), |tags| tags.join(", "))
                }),
        );
        let anime_info = vec![anime_title, anime_type_and_score, tags];
        let paragraph = Paragraph::new(anime_info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(get_color(is_active, app.app_config.theme)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, cards[index]);
    }
}

fn eq_manga_status(status: &Option<UserReadStatus>, status_str: &str) -> bool {
    match status {
        Some(UserReadStatus::Reading) => status_str == "reading",
        Some(UserReadStatus::Completed) => status_str == "completed",
        Some(UserReadStatus::OnHold) => status_str == "on_hold",
        Some(UserReadStatus::Dropped) => status_str == "dropped",
        Some(UserReadStatus::PlanToRead) => status_str == "plan_to_read",
        None => status_str == "add",
        _ => false,
    }
}

fn eq_anime_status(status: &Option<UserWatchStatus>, status_str: &str) -> bool {
    match status {
        Some(UserWatchStatus::Watching) => status_str == "watching",
        Some(UserWatchStatus::Completed) => status_str == "completed",
        Some(UserWatchStatus::OnHold) => status_str == "on_hold",
        Some(UserWatchStatus::Dropped) => status_str == "dropped",
        Some(UserWatchStatus::PlanToWatch) => status_str == "plan_to_watch",
        None => status_str == "add",
        _ => false,
    }
}
