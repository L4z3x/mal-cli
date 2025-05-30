use std::fmt::Debug;

use crate::api::model::MangaMediaType;
use crate::api::model::Node;
use crate::api::model::PageableData;
use crate::api::model::UserReadStatus;
use crate::app::DISPLAY_COLUMN_NUMBER;
use crate::app::DISPLAY_RAWS_NUMBER;
use crate::ui::format_number_with_commas;
use crate::ui::get_end_card_index;
use crate::{
    api::model::{AnimeMediaType, UserWatchStatus},
    app::{ActiveBlock, App, SelectedSearchTab},
    ui::util::get_color,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::get_anime_status_color;

pub fn draw_results(f: &mut Frame, app: &App, chunk: Rect) {
    match app.search_results.selected_tab {
        SelectedSearchTab::Anime => {
            if app.search_results.anime.as_ref().is_some() {
                draw_anime_search_results(f, app, chunk);
            } else {
                // draw_no_results(f, app, chunk);
            }
        }
        SelectedSearchTab::Manga => {
            if app.search_results.manga.as_ref().is_some() {
                draw_manga_search_results(f, app, chunk);
            } else {
                // draw_no_results(f, app, chunk);
            }
        }
    }
}

pub fn draw_anime_search_results(f: &mut Frame, app: &App, chunk: Rect) {
    let results = app.search_results.anime.as_ref().unwrap();
    if results.data.is_empty() {
        // draw_no_results(f, app, chunk);
        return;
    }
    let start_index = app.start_card_list_index as usize;
    let end_index = get_end_card_index(app);
    // let end_index = app.end_card_list_index as usize;

    let (cards, components_result) = construct_cards_with_data(chunk, results);
    // we need to calculate the end index carefully
    let component_page = components_result[start_index..=end_index].to_vec();

    let selected_card_index = app.search_results.selected_display_card_index.unwrap_or(0);

    // let selected_card_index = 5;

    for (index, component) in component_page.iter().enumerate() {
        let is_active =
            index == selected_card_index && app.active_block == ActiveBlock::DisplayBlock;

        let anime_status = component
            .my_list_status
            .as_ref()
            .map_or(UserWatchStatus::Other("None".to_string()), |status| {
                status.status.clone()
            });

        let anime_status_color = get_anime_status_color(&anime_status, app);

        let anime_status: &str = anime_status.into();

        let title_style = get_color(is_active, app.app_config.theme);

        let anime_title = &component.get_title(&app.app_config, false)[0];

        let title: Line<'_> = Line::from(vec![
            Span::styled(anime_title, title_style.add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(anime_status, Style::default().fg(anime_status_color)),
        ]);

        let media_type: &str = Into::<&str>::into(
            component
                .media_type
                .as_ref()
                .map_or(AnimeMediaType::Other("Unknown".to_string()), |media_type| {
                    media_type.clone()
                }),
        );

        let ep_num: String = component
            .num_episodes
            .map_or("N/A".to_string(), |ep| ep.to_string());

        let start_date: String = component
            .start_date
            .as_ref()
            .map_or("unknown".to_string(), |date| date.date.year().to_string());

        let num_user_list: String = component
            .num_list_users
            .map_or("N/A".to_string(), |s| format_number_with_commas(s));

        let score = Line::from(Span::styled(
            format!(
                "Scored {}",
                component.mean.map_or("N/A".to_string(), |m| m.to_string())
            ),
            Style::default(), //? we can add a function to get color based on score
        ));

        let num_ep = Line::from(Span::styled(
            format!("{} ({} eps)", media_type, ep_num),
            app.app_config.theme.text,
        ));

        let start_date = Line::from(Span::styled(start_date, app.app_config.theme.text));

        let num_user_list = Line::from(Span::styled(
            format!("{} members", num_user_list),
            app.app_config.theme.text,
        ));

        // if index >= cards.len() {
        //     break;
        //

        let card = Paragraph::new(vec![title, num_ep, score, start_date, num_user_list])
            .alignment(Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(get_color(is_active, app.app_config.theme)),
            );

        f.render_widget(card, cards[index]);
    }
    /*
    we are gonna display these fields:
    1. title
    2. average score
    3. mean score
    4. number of episodes
    5. media type
    6. start date
    7. user status
    */
}

pub fn draw_manga_search_results(f: &mut Frame, app: &App, chunk: Rect) {
    let results = app.search_results.manga.as_ref().unwrap();
    if results.data.is_empty() {
        //TODO: handle no results
        // draw_no_results(f, app, chunk);
        return;
    }
    let start_index = app.start_card_list_index as usize;
    let end_index = get_end_card_index(app);
    let (cards, components_result) = construct_cards_with_data(chunk, results);
    let component_page = components_result[start_index..=end_index].to_vec();

    let selected_card_index = app.search_results.selected_display_card_index.unwrap_or(0);

    for (index, component) in component_page.iter().enumerate() {
        if index >= cards.len() {
            break;
        }
        let is_active =
            index == selected_card_index && app.active_block == ActiveBlock::DisplayBlock;

        let manga_status = component
            .my_list_status
            .as_ref()
            .map_or(UserReadStatus::Other("None".to_string()), |status| {
                status.status.clone()
            });

        let manga_status_color = get_manga_status_color(&manga_status, app);

        let title_style = get_color(is_active, app.app_config.theme);
        let title = &component.get_title(&app.app_config, false)[0];

        let title: Line<'_> = Line::from(vec![
            Span::styled(title, title_style.add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled::<&str, ratatui::prelude::Style>(
                manga_status.into(),
                Style::default().fg(manga_status_color),
            ),
        ]);

        let media_type: &str = Into::<&str>::into(
            component
                .media_type
                .as_ref()
                .map_or(MangaMediaType::Other("None".to_string()), |media_type| {
                    media_type.clone()
                }),
        );

        let vol_num: String = component.get_num(&app.app_config);
        let start_date: String = component
            .start_date
            .as_ref()
            .map_or("unknown".to_string(), |date| date.date.year().to_string());

        let score = Line::from(Span::styled(
            format!(
                "Scored {}",
                component.mean.map_or("N/A".to_string(), |m| m.to_string())
            ),
            Style::default(), //? we can add a function to get color based on score
        ));

        let num_user_list: String = component
            .num_list_users
            .map_or("N/A".to_string(), |n| format_number_with_commas(n));

        //todo: add vols and ch based on app_config
        let type_num_vol = Line::from(Span::styled(
            format!("{} ({})", media_type, vol_num),
            app.app_config.theme.text,
        ));

        let start_date = Line::from(Span::styled(start_date, app.app_config.theme.text));

        let num_user_list = Line::from(Span::styled(
            format!("{} members", num_user_list),
            app.app_config.theme.text,
        ));

        let card = Paragraph::new(vec![title, type_num_vol, score, start_date, num_user_list])
            .alignment(Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(get_color(is_active, app.app_config.theme)),
            );

        f.render_widget(card, cards[index]);
    }
}

pub fn construct_cards_with_data<T: Clone + Debug>(
    chunk: Rect,
    results: &PageableData<Vec<Node<T>>>,
) -> (Vec<Rect>, Vec<&T>) {
    let current_page = &results.data;

    let raw_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Percentage(20); DISPLAY_RAWS_NUMBER.into()])
        .split(chunk);

    let components: Vec<&T> = current_page.iter().map(|node| &node.node).collect();

    (
        raw_layout
            .iter()
            .flat_map(|raw| {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Ratio(1, DISPLAY_COLUMN_NUMBER as u32);
                        DISPLAY_COLUMN_NUMBER.into()
                    ])
                    .split(*raw)
                    .into_iter()
                    .map(|rect| rect.clone())
                    .collect::<Vec<Rect>>()
            })
            .collect(),
        components,
    )
}

fn get_manga_status_color(status: &UserReadStatus, app: &App) -> Color {
    match status {
        UserReadStatus::Completed => app.app_config.theme.status_completed,
        UserReadStatus::Dropped => app.app_config.theme.status_dropped,
        UserReadStatus::OnHold => app.app_config.theme.status_on_hold,
        UserReadStatus::PlanToRead => app.app_config.theme.status_plan_to_watch,
        UserReadStatus::Reading => app.app_config.theme.status_watching,
        UserReadStatus::Other(_) => app.app_config.theme.status_other,
    }
}
