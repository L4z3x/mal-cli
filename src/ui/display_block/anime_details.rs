use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    Frame,
};
use ratatui_image::StatefulImage;

use crate::app::App;
use ratatui::widgets::{Block, Borders, Paragraph};

use super::center_area;

pub fn draw_anime_detail(f: &mut Frame, app: &App, chunk: Rect) {
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

    let image = StatefulImage::default();
    let mut image_state = picker.new_resize_protocol(app.media_image.as_ref().unwrap().clone());

    f.render_stateful_widget(image, center_area(chunk, 90, 80), &mut image_state);
    image_state.last_encoding_result().unwrap().unwrap();
}

fn draw_info(f: &mut Frame, app: &App, chunk: Rect) {
    draw_bordered_block(f, chunk);
}

pub fn draw_bordered_block(f: &mut Frame, chunk: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ratatui::style::Color::Yellow));

    f.render_widget(block, chunk);
}

// fn get_media_image(app: &App) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
//     let media = app.get_current_media();
//     let image = match media {
//         Some(media) => {
//             let image = media.image.clone();
//             image
//         }
//         None => image::DynamicImage::new_rgb8(1, 1),
//     };
//     image
// }

fn draw_image_place_holder(f: &mut Frame, chunk: Rect) {
    draw_bordered_block(f, chunk);
    // unable to render image (use kitty)
}
