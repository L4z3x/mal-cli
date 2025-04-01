use ratatui::{
    layout::{Alignment, Rect},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{
    api::model::MangaMediaType,
    app::{ActiveBlock, App},
    ui::util::get_color,
};

use super::{results::construct_cards_with_data, user_anime_list::draw_user_list_nav_bar};

pub fn draw_user_manga_list(f: &mut Frame, app: &App, chunk: Rect) {
    let statuses = vec![
        "add",
        "reading",
        "completed",
        "on_hold",
        "dropped",
        "plan_to_read",
    ];
    let chunk = draw_user_list_nav_bar(f, app, chunk, false, statuses);
    let chunk = super::draw_keys_bar(f, app, chunk);
    draw_manga_list_results(f, app, chunk);
}

fn draw_manga_list_results(f: &mut Frame, app: &App, chunk: Rect) {
    let results = app.search_results.manga.as_ref().unwrap();
    if results.data.is_empty() {
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

        let manga_title = &component.get_title(&app.app_config, false)[0];
        let manga_title = Line::styled(manga_title, title_style);

        let media_type: &str = Into::<&str>::into(
            component
                .media_type
                .as_ref()
                .map_or(MangaMediaType::Other("Unknown".to_string()), |media_type| {
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

        let manga_type_and_score = Line::from(format!("{} {}", media_type, score));

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

        let manga_info = vec![manga_title, manga_type_and_score, tags];
        let paragraph = Paragraph::new(manga_info)
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
