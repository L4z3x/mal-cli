use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use tui_big_text::{BigText, PixelSize};
use tui_scrollview::{ScrollView, ScrollbarVisibility};

use crate::{
    api::model::AnimeMediaType,
    app::{ActiveAnimeDetailBlock, App},
    ui::{
        display_block::{
            center_area,
            details_utils::{construct_synopsis_layout, get_score_text},
        },
        format_number_with_commas,
    },
};

use super::{
    details_utils::{self, get_anime_key_val_info},
    draw_keys_bar,
};

pub fn draw_anime_detail(f: &mut Frame, app: &mut App, chunk: Rect) {
    let chunk = draw_keys_bar(f, app, chunk);
    let [_, upper_chunk, lower_chunk] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(16),
            Constraint::Percentage(100),
        ])
        .areas(chunk);

    draw_top_info(f, app, upper_chunk.inner(Margin::new(2, 0)));
    let [synopsis_chunk, side_info_chunk] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .areas(lower_chunk.inner(Margin::new(2, 0)));

    draw_synopsis(f, app, synopsis_chunk);
    draw_side_info(f, app, side_info_chunk);
    if app.popup {
        match app.active_anime_detail_block {
            ActiveAnimeDetailBlock::AddToList => {
                details_utils::draw_user_status_popup(f, app, chunk);
            }

            ActiveAnimeDetailBlock::Episodes => {
                details_utils::draw_count_popup(f, app, chunk);
            }

            ActiveAnimeDetailBlock::Rate => {
                details_utils::draw_rate_popup(f, app, chunk);
            }
            _ => {}
        }
    }
}

fn draw_top_info(f: &mut Frame, app: &mut App, chunk: Rect) {
    let [picture_chunk, top_info_chunk] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(26), Constraint::Percentage(100)])
        .areas(chunk);
    details_utils::draw_picture(f, app, picture_chunk);
    draw_info(f, app, top_info_chunk);
}

fn draw_synopsis(f: &mut Frame, app: &App, chunk: Rect) {
    if let ActiveAnimeDetailBlock::Synopsis = app.active_anime_detail_block {
        details_utils::draw_bordered_block(f, chunk, app.app_config.theme.hovered)
    } else {
        details_utils::draw_bordered_block(f, chunk, app.app_config.theme.inactive)
    }

    let chunk = center_area(chunk, 90, 90);
    let synopsis = app.anime_details.as_ref().unwrap().synopsis.clone();
    let background = app.anime_details.as_ref().unwrap().background.clone();
    // related anime:
    let related_anime = app.anime_details.as_ref().unwrap().related_anime.clone();
    let related_anime_string;
    if let Some(related_anime) = related_anime {
        let strings = related_anime
            .iter()
            .map(|a| {
                format!(
                    "{}: {}",
                    a.relation_type_formatted,
                    a.node.get_title(&app.app_config, false)[0].clone()
                )
            })
            .collect::<Vec<String>>();

        related_anime_string = Some(strings.join("\n"));
    } else {
        related_anime_string = None;
    }

    // related manga:
    let related_manga = app.anime_details.as_ref().unwrap().related_manga.clone();
    let related_manga_string;
    if let Some(related_manga) = related_manga {
        let strings = related_manga
            .iter()
            .map(|a| {
                format!(
                    "{}: {}",
                    a.relation_type_formatted,
                    a.node.get_title(&app.app_config, false)[0].clone()
                )
            })
            .collect::<Vec<String>>();

        related_manga_string = Some(strings.join("\n"));
    } else {
        related_manga_string = None;
    }
    // recommendations:
    let recommendations = app.anime_details.as_ref().unwrap().recommendations.clone();
    let recommendations_string;
    if let Some(recommendations) = recommendations {
        let strings = recommendations
            .iter()
            .enumerate()
            .map(|(i, a)| {
                format!(
                    "{:02}. {}",
                    i + 1,
                    a.node.get_title(&app.app_config, false)[0].clone()
                )
            })
            .collect::<Vec<String>>();

        recommendations_string = Some(strings.join("\n"));
    } else {
        recommendations_string = None;
    }

    // call the macro
    let (total_height, layout_items) = construct_synopsis_layout(
        app,
        &[
            ("Synopsis:", synopsis),
            ("Background:", background),
            ("Related Anime:", related_anime_string),
            ("Related Manga:", related_manga_string),
            ("Recommendations:", recommendations_string),
        ],
    );
    details_utils::draw_synopsis_items(f, app, total_height, layout_items, chunk);
}

fn draw_side_info(f: &mut Frame, app: &App, chunk: Rect) {
    if let ActiveAnimeDetailBlock::SideInfo = app.active_anime_detail_block {
        details_utils::draw_bordered_block(f, chunk, app.app_config.theme.hovered)
    } else {
        details_utils::draw_bordered_block(f, chunk, app.app_config.theme.inactive)
    }

    let chunk = center_area(chunk, 90, 90);
    //*  info:
    let (key_values_info, alter_titles, alter_titles_title, alter_height) =
        get_anime_key_val_info(app);
    //* alternative titles:

    let scroll_view_rect = Rect::new(chunk.x, chunk.y, chunk.width, chunk.height + 5);
    let mut scrol_view = ScrollView::new(scroll_view_rect.as_size())
        .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

    let [alternative_titles_title_chunk, alternative_title_chunk, key_val_chunk] =
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(alter_height),
                Constraint::Percentage(100),
            ])
            .areas(scroll_view_rect);
    let alternative_titles_title_chunk = Rect::new(
        alternative_titles_title_chunk.x - chunk.x,
        alternative_titles_title_chunk.y - chunk.y,
        alternative_titles_title_chunk.width,
        alternative_titles_title_chunk.height,
    );

    let alternative_title_chunk = Rect::new(
        alternative_title_chunk.x - chunk.x,
        alternative_title_chunk.y - chunk.y,
        alternative_title_chunk.width,
        alternative_title_chunk.height,
    );
    let key_val_chunk = Rect::new(
        key_val_chunk.x - chunk.x,
        key_val_chunk.y - chunk.y,
        key_val_chunk.width,
        key_val_chunk.height,
    );

    scrol_view.render_widget(alter_titles_title, alternative_titles_title_chunk);
    scrol_view.render_widget(alter_titles, alternative_title_chunk);
    scrol_view.render_widget(key_values_info, key_val_chunk);
    let mut scroll_state = app.anime_details_info_scroll_view_state;

    f.render_stateful_widget(scrol_view, chunk, &mut scroll_state);
}

fn draw_info(f: &mut Frame, app: &App, chunk: Rect) {
    if app.active_anime_detail_block == ActiveAnimeDetailBlock::AddToList
        || app.active_anime_detail_block == ActiveAnimeDetailBlock::Rate
        || app.active_anime_detail_block == ActiveAnimeDetailBlock::Episodes
    {
        details_utils::draw_bordered_block(f, chunk, app.app_config.theme.hovered)
    } else {
        details_utils::draw_bordered_block(f, chunk, app.app_config.theme.inactive)
    }
    // splitting the layout
    let [upper_chunk, lower_chunk, _] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(chunk);

    let [score_chunk, rest_chunk] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(22), Constraint::Percentage(100)])
        .areas(upper_chunk);

    let [upper_rest_chunk, lower_rest_chunk] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3)])
        .flex(Flex::Center)
        .areas(rest_chunk);

    let [_, user_status_chunk, user_score_chunk, user_progress_chunk, _] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(23),
            Constraint::Min(23),
            Constraint::Min(23),
            Constraint::Percentage(100),
        ])
        .flex(Flex::Center)
        .areas(lower_chunk);
    // draw_bordered_block(f, score_chunk);
    let [score_title_chunk, big_score_chunk, num_users_chunk] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(4),
            Constraint::Length(2),
        ])
        .flex(Flex::Center)
        .areas(score_chunk);

    // gettting the data

    // let block = Block::default()
    //     .borders(Borders::NONE)
    //     .border_type(BorderType::Rounded);

    // score
    let score = Line::from(
        app.anime_details
            .as_ref()
            .unwrap()
            .mean
            .map_or("N/A".to_string(), |f| f.to_string()),
    )
    .alignment(Alignment::Right);

    let score_title = Line::from("SCORE")
        .alignment(Alignment::Center)
        .style(Style::default().bg(app.app_config.theme.mal_color));
    let big_score = BigText::builder()
        .pixel_size(PixelSize::Sextant)
        .style(Style::default().fg(app.app_config.theme.text))
        .alignment(Alignment::Center)
        .lines(vec![score])
        .build();

    let num_user = app
        .anime_details
        .as_ref()
        .unwrap()
        .num_list_users
        .map_or("N/A".to_string(), format_number_with_commas);

    let num_user_line =
        Line::from(format!("{} {}", num_user, "users")).alignment(Alignment::Center);

    // ranked
    let ranked = app
        .anime_details
        .as_ref()
        .unwrap()
        .rank
        .map_or("N/A".to_string(), format_number_with_commas);
    let rank = Span::styled(
        format!("#{}", ranked),
        Style::default().add_modifier(Modifier::BOLD),
    );
    // let ranked_line =
    //     Paragraph::new(Line::from(rank).alignment(Alignment::Center)).block(block.clone());

    //* popularity

    let popularity = app
        .anime_details
        .as_ref()
        .unwrap()
        .popularity
        .map_or("N/A".to_string(), format_number_with_commas);
    let popularity = Span::styled(
        format!("#{}", popularity),
        Style::default().add_modifier(Modifier::BOLD),
    );

    //* Members
    let members = app
        .anime_details
        .as_ref()
        .unwrap()
        .num_list_users
        .map_or("N/A".to_string(), format_number_with_commas);

    let members = Span::styled(members, Style::default().add_modifier(Modifier::BOLD));

    let first_line = Line::from(vec![
        Span::raw("Ranked "),
        rank,
        Span::raw("   Popularity "),
        popularity,
        Span::raw("   Members "),
        members,
    ])
    .alignment(Alignment::Left);
    // let members_line =
    //     Paragraph::new(Line::from(members).alignment(Alignment::Center)).block(block.clone());

    //*  season, type, studio
    let season = app
        .anime_details
        .as_ref()
        .unwrap()
        .start_season
        .as_ref()
        .map_or("".to_string(), |s| s.season.clone().to_string());

    let start_year = app
        .anime_details
        .as_ref()
        .unwrap()
        .start_season
        .as_ref()
        .map_or("".to_string(), |s| s.year.clone().to_string());

    let season_year: String = if season.is_empty() && start_year.is_empty() {
        "unknown".to_string()
    } else {
        format!("{} {}", season, start_year)
            .trim_start()
            .to_string()
    };

    let media_type: &str = Into::<&str>::into(
        app.anime_details
            .as_ref()
            .unwrap()
            .media_type
            .clone()
            .map_or(AnimeMediaType::Other("other".to_string()), |s| s),
    );

    let studio =
        app.anime_details
            .as_ref()
            .unwrap()
            .studios
            .clone()
            .map_or("unknown".to_string(), |s| {
                s.iter()
                    .map(|s| s.name.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            });

    let info_line = Line::from(format!(
        "{}   |   {}   |   {}",
        season_year, media_type, studio
    ))
    .alignment(Alignment::Left);

    let unhovered_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.app_config.theme.inactive));

    // user stats:
    let user_status_list = app.anime_details.as_ref().unwrap().my_list_status.clone();
    // user_status:
    let user_status = user_status_list
        .as_ref()
        .map_or("add to list".to_string(), |s| s.status.clone().to_string());
    let mut user_status_paragraph = Paragraph::new(user_status)
        .alignment(Alignment::Center)
        .block(unhovered_block.clone());

    // user score:
    let user_score = user_status_list
        .as_ref()
        .map_or("rate ⭐ ".to_string(), |s| {
            format!("{} ⭐", get_score_text(s.score))
        });
    let mut user_score_paragraph = Paragraph::new(user_score)
        .alignment(Alignment::Center)
        .block(unhovered_block.clone());

    // user progress:
    let watched_ep = user_status_list
        .as_ref()
        .map_or(0, |s| s.num_episodes_watched);
    let total_ep = app
        .anime_details
        .as_ref()
        .unwrap()
        .num_episodes
        .map_or("?".to_string(), |n| {
            (n > 0).then(|| n.to_string()).unwrap_or("?".to_string())
        });

    let user_progress = format!("Episodes: {}/{}", watched_ep, total_ep);
    let mut user_progress_paragraph = Paragraph::new(user_progress)
        .alignment(Alignment::Center)
        .block(unhovered_block);

    let hovered_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.app_config.theme.hovered));

    match app.active_anime_detail_block {
        ActiveAnimeDetailBlock::AddToList => {
            user_status_paragraph = user_status_paragraph.block(hovered_block)
        }
        ActiveAnimeDetailBlock::Episodes => {
            user_progress_paragraph = user_progress_paragraph.block(hovered_block);
        }
        ActiveAnimeDetailBlock::Rate => {
            user_score_paragraph = user_score_paragraph.block(hovered_block);
        }
        _ => {}
    }

    f.render_widget(user_score_paragraph, user_score_chunk);
    f.render_widget(user_progress_paragraph, user_progress_chunk);
    f.render_widget(user_status_paragraph, user_status_chunk);
    f.render_widget(first_line, upper_rest_chunk);
    f.render_widget(info_line, lower_rest_chunk);
    f.render_widget(score_title, center_area(score_title_chunk, 45, 100));
    f.render_widget(big_score, center_area(big_score_chunk, 69, 80));
    f.render_widget(num_user_line, num_users_chunk);
    // let
    // just remember the old days
    /*
    | SCORE |        |       |        |
    |_______|        |       |        |
    | Big s | ranked | popul.| member |
    | num u | season | type  | studio |
    |_______|________|_______|________|
    |       |        |       |        |

    */
}
