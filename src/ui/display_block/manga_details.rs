use super::{details_utils, draw_keys_bar};
use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

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

    draw_top_info(f, app, upper_chunk);
    let [synopsis_chunk, side_info_chunk] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .areas(lower_chunk);

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
    draw_bordered_block(f, chunk);
}

fn draw_info(f: &mut Frame, app: &App, chunk: Rect) {
    draw_bordered_block(f, chunk);
}
