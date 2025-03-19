use std::{fmt::Debug, slice::Iter};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListState, Padding, Paragraph},
    Frame,
};

use crate::{
    api::model::{AnimeMediaType, MangaMediaType, PageableData, UserWatchStatus},
    app::{
        ActiveBlock, ActiveDisplayBlock, App, ANIME_RANKING_TYPES, DISPLAY_COLUMN_NUMBER,
        DISPLAY_RAWS_NUMBER, MANGA_RANKING_TYPES,
    },
    ui::util::get_color,
};

use super::{center_area, get_anime_status_color};

pub fn draw_anime_ranking(f: &mut Frame, app: &App, chunk: Rect) {
    let chunk = draw_nav_bar(f, app, chunk);
    draw_anime_ranking_results(f, app, chunk);
    if app.popup {
        draw_ranking_popup(f, app, chunk)
    }
}

pub fn draw_manga_ranking(f: &mut Frame, app: &App, chunk: Rect) {
    let chunk = draw_nav_bar(f, app, chunk);
    draw_manga_ranking_results(f, app, chunk);
    if app.popup {
        draw_ranking_popup(f, app, chunk)
    }
}

pub fn draw_nav_bar(f: &mut Frame, app: &App, chunk: Rect) -> Rect {
    let splitted_layout: [Rect; 2] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(7), Constraint::Percentage(93)])
        .margin(0)
        .areas(chunk);

    // bar:
    let bar = splitted_layout[0];

    let block = Block::default().border_style(app.app_config.theme.active);

    f.render_widget(block, bar);

    let tab: [Rect; 2] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .areas(bar);

    // anime tab:
    let anime_tab = tab[0];

    let is_active = app.active_display_block == ActiveDisplayBlock::AnimeRanking;
    let anime = Span::styled("Anime", get_color(is_active, app.app_config.theme));
    let anime_tab_paragraph = Paragraph::new(anime).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(get_color(is_active, app.app_config.theme)),
    );

    f.render_widget(anime_tab_paragraph, anime_tab);

    // manga tab:
    let manga_tab = tab[1];

    let is_active = app.active_display_block == ActiveDisplayBlock::MangaRanking;

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

pub fn draw_anime_ranking_results(f: &mut Frame, app: &App, chunk: Rect) {
    let results = app.anime_ranking_data.as_ref().unwrap();
    if results.data.is_empty() {
        // draw_no_results(f, app, chunk);
        return;
    }
    let cards_results = constract_cards_with_data(chunk, results);

    let cards = cards_results.0;
    let components = cards_results.1;

    // let's just use the search card index for now
    let selected_card_index = app.search_results.selected_display_card_index.unwrap_or(0);

    for (index, component_pair) in components.iter().enumerate() {
        let component = component_pair.node.clone();
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

        let num_user_list: String = component.num_list_users.unwrap().to_string();

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

        if index >= cards.len() {
            break;
        }

        let card = Paragraph::new(vec![title, num_ep, score, start_date, num_user_list])
            .alignment(Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(get_color(is_active, app.app_config.theme))
                    .title(component_pair.ranking.rank.to_string())
                    .title_style(get_color(is_active, app.app_config.theme)),
            );

        f.render_widget(card, cards[index]);
    }
}

pub fn draw_manga_ranking_results(f: &mut Frame, app: &App, chunk: Rect) {
    let results = app.manga_ranking_data.as_ref().unwrap();
    if results.data.is_empty() {
        // draw_no_results(f,app,chunk);
        return;
    }
    let cards_results = constract_cards_with_data(chunk, results);
    let components = cards_results.1;
    let selected_card_index = app.search_results.selected_display_card_index.unwrap_or(0);

    for (index, component_pair) in components.iter().enumerate() {
        let component = component_pair.node.clone();
        let is_active =
            index == selected_card_index && app.active_block == ActiveBlock::DisplayBlock;

        let title_style = get_color(is_active, app.app_config.theme);

        let manga_title = &component.get_title(&app.app_config, false)[0];

        let title: Line<'_> = Line::from(vec![
            Span::styled(manga_title, title_style.add_modifier(Modifier::BOLD)),
            Span::raw(" "),
        ]);

        let media_type: &str = Into::<&str>::into(
            component
                .media_type
                .as_ref()
                .map_or(MangaMediaType::Other("Unknown".to_string()), |media_type| {
                    media_type.clone()
                }),
        );

        let chap_num: String = component
            .num_chapters
            .map_or("N/A".to_string(), |ep| ep.to_string());

        let start_date: String = component
            .start_date
            .as_ref()
            .map_or("unknown".to_string(), |date| date.date.year().to_string());

        let num_user_list: String = component.num_list_users.unwrap().to_string();

        let score = Line::from(Span::styled(
            format!(
                "Scored {}",
                component.mean.map_or("N/A".to_string(), |m| m.to_string())
            ),
            Style::default(), //? we can add a function to get color based on score
        ));

        let num_ep = Line::from(Span::styled(
            format!("{} ({} chaps)", media_type, chap_num),
            app.app_config.theme.text,
        ));

        let start_date = Line::from(Span::styled(start_date, app.app_config.theme.text));

        let num_user_list = Line::from(Span::styled(
            format!("{} members", num_user_list),
            app.app_config.theme.text,
        ));

        if index >= cards_results.0.len() {
            break;
        }

        let card = Paragraph::new(vec![title, num_ep, score, start_date, num_user_list])
            .alignment(Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(get_color(is_active, app.app_config.theme))
                    .title(component_pair.ranking.rank.to_string())
                    .title_style(get_color(is_active, app.app_config.theme)),
            );
        f.render_widget(card, cards_results.0[index]);
    }
}

fn constract_cards_with_data<T: Clone + Debug>(
    chunk: Rect,
    results: &PageableData<Vec<T>>,
) -> (Vec<Rect>, Vec<&T>) {
    let current_page = &results;

    let raw_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Percentage(20); DISPLAY_RAWS_NUMBER.into()])
        .split(chunk);

    let components: Vec<&T> = current_page.data.iter().map(|node| node).collect();

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

fn draw_ranking_popup(f: &mut Frame, app: &App, chunk: Rect) {
    let area = center_area(chunk, 20, 40);
    let popup = Block::default()
        .title("Ranking Type")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    f.render_widget(Clear, area);
    f.render_widget(popup, area);
    // trait object to hold the list of ranking types
    let list: Box<dyn Iterator<Item = Line>> = match &app.active_display_block {
        ActiveDisplayBlock::AnimeRanking => {
            let iter: Iter<_> = ANIME_RANKING_TYPES.iter();
            let mapped = iter.map(|s| {
                Line::from(*s)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(app.app_config.theme.text))
            });
            Box::new(mapped)
        }
        ActiveDisplayBlock::MangaRanking => {
            let iter: Iter<_> = MANGA_RANKING_TYPES.iter();
            let mapped = iter.map(|s| {
                Line::from(*s)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(app.app_config.theme.text))
            });
            Box::new(mapped)
        }
        _ => Box::new(std::iter::empty()),
    };
    let index = match &app.active_display_block {
        ActiveDisplayBlock::AnimeRanking => Some(app.anime_ranking_index as usize),
        ActiveDisplayBlock::MangaRanking => Some(app.manga_ranking_index as usize),
        _ => None,
    };
    let mut state = ListState::default();
    state.select(index);

    let block = Block::default().padding(Padding::symmetric(1, 2));

    let rank_list = List::new(list).block(block).highlight_style(
        Style::default()
            .fg(app.app_config.theme.selected)
            .add_modifier(Modifier::BOLD), // .add_modifier(Modifier::UNDERLINED),
    );
    let centered_list_area = center_area(area, 60, 90);
    f.render_stateful_widget(rank_list, centered_list_area, &mut state);
}

// fn draw_manga_ranking_popup(f: &mut Frame, app: &App, chunk: Rect) {
//     let area = center_area(chunk, 30, 30);
// }
