use image::{DynamicImage, ImageError};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use ratatui_image::StatefulImage;

use crate::{app::App, ui::display_block::anime_details};

use super::{
    anime_details::{draw_bordered_block, draw_image_place_holder},
    center_area,
};

pub fn draw_manga_detail(f: &mut Frame, app: &App, chunk: Rect) {
    let layout: [Rect; 2] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(16), Constraint::Percentage(100)])
        .areas(chunk);

    draw_top_info(f, app, layout[0]);
    let layout: [Rect; 2_] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        // .margin(1)
        .areas(layout[1]);
    draw_synopsis(f, app, layout[0]);
    draw_side_info(f, app, layout[1]);
}

fn draw_top_info(f: &mut Frame, app: &App, chunk: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(22), Constraint::Percentage(100)])
        // .margin(1)
        .split(chunk);

    let picture_chunk = layout[0];
    let top_info_chunk = layout[1];

    draw_picture(f, app, picture_chunk);
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
                    let (title, text, height) = anime_details::get_text_prop($title.to_string(), content.clone(), $app);
                    layout_items.push((title, text, height));
                    total_height += height + 6; // Add padding
                }else {
                    // handle empty string
                    let (title, text, height) = anime_details::get_text_prop($title.to_string(), format!("No {} available.",$title.to_string()).to_string(), $app);
                    layout_items.push((title, text, height));
                    total_height += height + 6  ; // Add padding
                }
            }else {
                // needed to duplicate this in case they decided a null value and  not ab empty string
                let (title, text, height) = anime_details::get_text_prop($title.to_string(), format!("No {} available.",$title.to_string()).to_string(), $app);
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
    // related

    // call the macro
    let (total_height, layout_items) = construct_synopsis_layout!(
        app,
        // "Synopsis:" =>synopsis,
        // "Synopsis:" =>synopsis,
        "Synopsis:" =>synopsis,
        "Background:" => background
    );
    anime_details::draw_synopsis_items(f, app, total_height, layout_items, chunk);
}

fn draw_side_info(f: &mut Frame, app: &App, chunk: Rect) {
    draw_bordered_block(f, chunk);
}

fn draw_picture(f: &mut Frame, app: &App, chunk: Rect) {
    let picker = app.picker.clone();
    if picker.is_none() || app.media_image.is_none() {
        draw_image_place_holder(f, chunk);
        return;
    }
    let picker = picker.unwrap();

    match get_picture_from_cache(app) {
        Ok(image) => {
            let mut image_state = picker.new_resize_protocol(image);
            let image = StatefulImage::default();
            f.render_stateful_widget(image, center_area(chunk, 90, 80), &mut image_state);
            image_state.last_encoding_result().unwrap().unwrap();
        }
        Err(e) => {
            // pass e to the place holder
            draw_image_place_holder(f, chunk);
        }
    }
}

fn draw_info(f: &mut Frame, app: &App, chunk: Rect) {
    draw_bordered_block(f, chunk);
}

pub fn get_picture_from_cache(app: &App) -> Result<DynamicImage, ImageError> {
    // all images are stored in $HOME?/.cache/mal-tui/images/
    let file_name = app.media_image.as_ref().unwrap();
    let file_path = app.app_config.paths.picture_cache_dir_path.join(file_name);
    let image = image::ImageReader::open(file_path)?.decode()?;
    Ok(image)
}
