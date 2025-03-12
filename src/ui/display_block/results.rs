use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    api::model::{Anime, AnimeMediaType, UserWatchStatus},
    app::{ActiveBlock, App, SelectedSearchTab},
    ui::util::get_color,
};

pub fn draw_results(f: &mut Frame, app: &App, chunk: Rect) {
    match app.search_results.selected_tab {
        SelectedSearchTab::Anime => {
            if let Some(results) = app.search_results.anime.as_ref() {
                draw_anime_search_results(f, app, chunk);
            } else {
                // draw_no_results(f, app, chunk);
            }
        }
        SelectedSearchTab::Manga => {
            // draw_manga_search_results(f, app, chunk);
        }
    }
}

pub fn draw_anime_search_results(f: &mut Frame, app: &App, chunk: Rect) {
    let results = app.search_results.anime.as_ref().unwrap();
    if results.data.is_empty() {
        // draw_no_results(f, app, chunk);
        return;
    }

    let current_page = &results.data;

    let raw_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Percentage(20); 5])
        .split(chunk);

    let components: Vec<&Anime> = current_page.iter().map(|node| &node.node).collect();

    let cards: Vec<Rect> = raw_layout
        .iter()
        .flat_map(|raw| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Ratio(1, 3); 3])
                .split(*raw)
                .into_iter()
                .map(|rect| rect.clone())
                .collect::<Vec<Rect>>()
        })
        .collect();

    let selected_card_index = app.search_results.selected_display_card_index.unwrap_or(0);

    for (index, component) in components.iter().enumerate() {
        let is_active =
            index == selected_card_index && app.active_block == ActiveBlock::DisplayBlock;

        let anime_status = component
            .my_list_status
            .as_ref()
            .map_or(UserWatchStatus::Other("None".to_string()), |status| {
                status.status.clone()
            });

        let anime_status_color = get_anime_status_color(&anime_status, app);

        let title_style = get_color(is_active, app.app_config.theme);

        let title: Line<'_> = Line::from(vec![
            Span::styled(
                component.title.clone(),
                title_style.add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled::<&str, ratatui::prelude::Style>(
                anime_status.into(),
                Style::default().fg(anime_status_color),
            ),
        ]);

        let media_type: &str = Into::<&str>::into(
            component
                .media_type
                .as_ref()
                .map_or(AnimeMediaType::Other("None".to_string()), |media_type| {
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

        let num_user_list: String = component.num_list_users.unwrap().to_string();

        let num_ep = Line::from(Span::styled(
            format!("{} ({} eps)", media_type, ep_num),
            app.app_config.theme.text,
        ));

        let start_date = Line::from(Span::styled(start_date, app.app_config.theme.text));

        let num_user_list = Line::from(Span::styled(
            format!("{} members", num_user_list),
            app.app_config.theme.text,
        ));

        if index >= cards.len() {
            break;
        }

        let card = Paragraph::new(vec![title, num_ep, start_date, num_user_list])
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
