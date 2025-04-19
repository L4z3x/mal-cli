use super::{details_utils, draw_keys_bar};
use crate::{
    api::model::{Manga, MangaMediaType},
    app::App,
    ui::format_number_with_commas,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    Frame,
};
use tui_big_text::{BigText, PixelSize};
use tui_scrollview::{ScrollView, ScrollbarVisibility};

use super::{center_area, details_utils::draw_bordered_block};

pub fn draw_manga_detail(f: &mut Frame, app: &mut App, chunk: Rect) {
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
}

fn draw_top_info(f: &mut Frame, app: &mut App, chunk: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(26),
            Constraint::Percentage(100),
        ])
        // .margin(1)
        .split(chunk);

    let picture_chunk = layout[1];
    let top_info_chunk = layout[2];

    details_utils::draw_picture(f, app, picture_chunk);
    draw_info(f, app, top_info_chunk);
}

fn draw_synopsis(f: &mut Frame, app: &App, chunk: Rect) {
    draw_bordered_block(f, chunk);
    macro_rules!   construct_synopsis_layout {
    ($app:ident, $($title:expr => $text:expr),+ ) => {{
        let mut layout_items = Vec::new();
        let mut total_height = 0;

        $(
            if let Some(content) = &$text {
                if !content.is_empty() {
                    let (title, text, height) = details_utils::get_text_prop($title.to_string(), content.clone(), $app);
                    layout_items.push((title, text, height));
                    total_height += height + 6; // Add padding
                }else {
                    // handle empty string
                    let mut title = $title.to_string();
                    title.remove(title.len()-1);
                    let (title, text, height) = details_utils::get_text_prop($title.to_string(), format!("No {} available.",title).to_ascii_lowercase(), $app);
                    layout_items.push((title, text, height));
                    total_height += height + 6  ; // Add padding
                }
            }else {
                // needed to duplicate this in case they decided a null value and  not ab empty string
                let mut title = $title.to_string();
                title.remove(title.len()-1);
                let (title, text, height) = details_utils::get_text_prop($title.to_string(), format!("No {} available.",title).to_ascii_lowercase(), $app);
                layout_items.push((title, text, height));
                total_height += height + 6  ; // Add padding
        }

        )*
        (total_height,layout_items)
        }
        };
    }

    let chunk = center_area(chunk, 90, 90);
    let synopsis = app.manga_details.as_ref().unwrap().synopsis.clone();
    let background = app.manga_details.as_ref().unwrap().background.clone();
    // related anime:
    let related_anime = app.manga_details.as_ref().unwrap().related_anime.clone();
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
    let related_manga = app.manga_details.as_ref().unwrap().related_manga.clone();
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
    let recommendations = app.manga_details.as_ref().unwrap().recommendations.clone();
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
    let (total_height, layout_items) = construct_synopsis_layout!(
        app,
        "Synopsis:" =>synopsis,
        "Background:" => background,
        "Related Anime:" => related_anime_string,
        "Related Manga:" => related_manga_string,
        "Recommendations:" => recommendations_string
    );
    details_utils::draw_synopsis_items(f, app, total_height, layout_items, chunk);
}

fn draw_side_info(f: &mut Frame, app: &App, chunk: Rect) {
    details_utils::draw_bordered_block(f, chunk);

    let chunk = center_area(chunk, 90, 90);
    //*  info:
    let (key_values_info, alter_titles, alter_titles_title, alter_height) =
        details_utils::get_manga_key_val_info(app);
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
    let mut scroll_state = app.manga_details_info_scroll_view_state.clone();

    f.render_stateful_widget(scrol_view, chunk, &mut scroll_state);
}

fn draw_info(f: &mut Frame, app: &App, chunk: Rect) {
    details_utils::draw_bordered_block(f, chunk);
    // splitting the layout
    let [upper_chunk, lower_chunk] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
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
            Constraint::Min(20),
            Constraint::Min(20),
            Constraint::Min(20),
            Constraint::Percentage(100),
        ])
        .flex(Flex::Start)
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

    // score
    let score = Line::from(
        app.manga_details
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
        .manga_details
        .as_ref()
        .unwrap()
        .num_list_users
        .map_or("N/A".to_string(), |s| format_number_with_commas(s));

    let num_user_line =
        Line::from(format!("{} {}", num_user, "users")).alignment(Alignment::Center);

    // ranked
    let ranked = app
        .manga_details
        .as_ref()
        .unwrap()
        .rank
        .map_or("N/A".to_string(), |n| format_number_with_commas(n));
    let rank = Span::styled(
        format!("#{}", ranked),
        Style::default().add_modifier(Modifier::BOLD),
    );

    //* popularity

    let popularity = app
        .manga_details
        .as_ref()
        .unwrap()
        .popularity
        .map_or("N/A".to_string(), |n| format_number_with_commas(n));
    let popularity = Span::styled(
        format!("#{}", popularity),
        Style::default().add_modifier(Modifier::BOLD),
    );

    //* Members
    let members = app
        .manga_details
        .as_ref()
        .unwrap()
        .num_list_users
        .map_or("N/A".to_string(), |n| format_number_with_commas(n));

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

    //*  type,  magasin,authors,

    let authors = get_manga_authors(app.manga_details.as_ref());

    let media_type: &str = Into::<&str>::into(
        app.manga_details
            .as_ref()
            .unwrap()
            .media_type
            .clone()
            .map_or(MangaMediaType::Other("other".to_string()), |s| s),
    );

    let magasine = app
        .manga_details
        .as_ref()
        .unwrap()
        .serialization
        .as_ref()
        .map_or("unknown".to_string(), |s| {
            s.iter()
                .map(|s| s.node.name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        });

    let info_line = Line::from(format!(
        "{}   |   {}   |   {}",
        media_type, magasine, authors
    ))
    .alignment(Alignment::Left);

    // user stats:
    let user_status_list = app.manga_details.as_ref().unwrap().my_list_status.clone();
    // user_status:
    let user_status = user_status_list
        .as_ref()
        .map_or("add to list".to_string(), |s| {
            Into::<&str>::into(s.status.clone()).to_string()
        });
    let user_status_line = Line::from(user_status).alignment(Alignment::Left);

    // user score:
    let user_score = user_status_list
        .as_ref()
        .map_or("rate ⭐ ".to_string(), |s| {
            format!("{} ⭐", details_utils::get_score_text(s.score))
        });
    let user_score_line = Line::from(user_score).alignment(Alignment::Left);

    // user progress:
    let user_progress = user_status_list
        .as_ref()
        .map_or("Epsiodes: ".to_string(), |s| {
            format!(
                "Episodes: {} / {}",
                s.num_chapters_read,
                app.manga_details
                    .as_ref()
                    .unwrap()
                    .num_chapters
                    .map_or("?".to_string(), |n| n.to_string())
            )
        });
    let user_progress_line = Line::from(user_progress).alignment(Alignment::Left);

    // todo: draw boxes arround the info
    // f.render_widget(ranked_line, first_area(ranked_chunk, 100, 50));
    // f.render_widget(popularity_line, first_area(popularity_chunk, 100, 50));
    // f.render_widget(members_line, first_area(memeber_chunk, 100, 50));
    f.render_widget(user_score_line, user_score_chunk);
    f.render_widget(user_progress_line, user_progress_chunk);
    f.render_widget(user_status_line, user_status_chunk);
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

fn get_manga_authors(manga_details: Option<&Manga>) -> String {
    match manga_details {
        Some(details) => match details.authors.as_ref() {
            Some(authors) => authors
                .iter()
                .map(|s| {
                    format!(
                        "{} {} ({})",
                        s.node
                            .first_name
                            .as_ref()
                            .map_or("?".to_string(), |s| s.clone()),
                        s.node
                            .last_name
                            .as_ref()
                            .map_or("?".to_string(), |s| s.clone()),
                        s.role.clone()
                    )
                })
                .collect::<Vec<String>>()
                .join(", "),
            None => "?".to_string(),
        },
        None => "?".to_string(),
    }
}
