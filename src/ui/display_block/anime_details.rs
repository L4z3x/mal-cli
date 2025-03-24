use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::List,
    Frame,
};
use ratatui_image::StatefulImage;
use tui_scrollview::{ScrollView, ScrollbarVisibility};

use crate::app::App;
use ratatui::widgets::{Block, Borders};

use super::{center_area, manga_details::get_picture_from_cache};

pub fn draw_anime_detail(f: &mut Frame, app: &App, chunk: Rect) {
    let layout: [Rect; 2] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(16), Constraint::Percentage(100)])
        .areas(chunk);

    draw_top_info(f, app, layout[0]);
    let layout: [Rect; 2_] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
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
    //  takes the data and the chunk and the app and returns the total hight and the chunks to be rendered an the layout items
    macro_rules!   construct_synopsis_layout {
    ($app:ident, $($title:expr => $text:expr),+ ) => {{
        let mut layout_items = Vec::new();
        let mut total_height = 0;

        $(
            if let Some(content) = &$text {
                if !content.is_empty() {
                    let (title, text, height) = get_text_prop($title.to_string(), content.clone(), $app);
                    layout_items.push((title, text, height));
                    total_height += height + 6; // Add padding
                }else {
                    // handle empty string
                    let (title, text, height) = get_text_prop($title.to_string(), format!("No {} available.",$title.to_string()).to_string(), $app);
                    layout_items.push((title, text, height));
                    total_height += height + 6  ; // Add padding
                }
            }else {
                // needed to duplicate this in case they decided a null value and  not ab empty string
                let (title, text, height) = get_text_prop($title.to_string(), format!("No {} available.",$title.to_string()).to_string(), $app);
                layout_items.push((title, text, height));
                total_height += height + 6  ; // Add padding
        }

        )*
        (total_height,layout_items)
        }
        };
    }

    let chunk = center_area(chunk, 90, 90);
    let synopsis = app.anime_details.as_ref().unwrap().synopsis.clone();
    let background = app.anime_details.as_ref().unwrap().background.clone();
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
    draw_synopsis_items(f, app, total_height, layout_items, chunk);
}

fn draw_side_info(f: &mut Frame, app: &App, chunk: Rect) {
    draw_bordered_block(f, chunk);
}

fn draw_picture(f: &mut Frame, app: &App, chunk: Rect) {
    // the image is downloaded and cached, then we store the file name in the app and the route
    let picker = app.picker.clone();
    if picker.is_none() || app.media_image.is_none() {
        draw_image_place_holder(f, chunk);
        return;
    }
    let picker = picker.unwrap();

    let image = StatefulImage::default();
    let mut image_state = picker.new_resize_protocol(get_picture_from_cache(app).unwrap());

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

pub fn draw_image_place_holder(f: &mut Frame, chunk: Rect) {
    draw_bordered_block(f, chunk);
    // ==> unable to render image (use kitty)
}

fn construct_paragraph_lines(
    text: &str,
    width: u16,
    word_width: u16,
    line_height: u16,
) -> (u16, Vec<Line<'static>>) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    let mut current_width = 0;
    let mut line_count = 0;

    for word in words {
        let word_length = (word.chars().count() as u16) * word_width;

        if current_width + word_length > width {
            // Push the constructed line to Vec<Line>
            lines.push(Line::from(current_line.clone()));
            current_line.clear();
            line_count += 1;
            current_width = 0;
        }

        current_line.push(Span::raw(word.to_string() + " "));
        current_width += word_length + word_width; // Include space
    }

    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
        line_count += 1;
    }

    (line_count * line_height, lines)
}

pub fn get_text_prop(
    title: String,
    text: String,
    app: &App,
) -> (Line<'static>, List<'static>, u16) {
    let text_title = Line::raw(title).style(
        Style::default()
            .fg(app.app_config.theme.text)
            .add_modifier(Modifier::BOLD),
    );
    let text_lines = text.lines();
    let mut text_height = 0;
    let mut text_lines_ui = Vec::new();
    for line in text_lines {
        let (height, line) = construct_paragraph_lines(&line, 100, 1, 1);
        line.iter().for_each(|l| text_lines_ui.push(l.clone()));
        text_height += height;
    }

    let text = List::new(text_lines_ui);
    (text_title, text, text_height)
}

pub fn draw_synopsis_items(
    f: &mut Frame,
    app: &App,
    total_height: u16,
    layout_items: Vec<(Line<'static>, List<'static>, u16)>,
    chunk: Rect,
) {
    let scroll_view_rect = Rect::new(chunk.x, chunk.y, chunk.width, total_height);
    let mut constraints = layout_items
        .iter()
        .flat_map(|(_, _, height)| {
            vec![
                Constraint::Length(1),           // title
                Constraint::Length(1),           // spacing
                Constraint::Length(*height + 1), // text
                Constraint::Length(1),           // spacing
            ]
        })
        .collect::<Vec<_>>();
    constraints.push(Constraint::Percentage(100));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .flex(Flex::Start)
        .split(scroll_view_rect)
        .to_vec();

    let mut scroll_view = ScrollView::new(scroll_view_rect.as_size())
        .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

    for (index, (title, text, _)) in layout_items.iter().enumerate() {
        let title_chunk = Rect::new(
            layout[index * 4].x - chunk.x,
            layout[index * 4].y - chunk.y,
            layout[index * 4].width,
            layout[index * 4].height,
        );
        let text_chunk = Rect::new(
            layout[index * 4 + 2].x - chunk.x,
            layout[index * 4 + 2].y - chunk.y,
            layout[index * 4 + 2].width,
            layout[index * 4 + 2].height,
        );
        // dbg!(&title_chunk.height, &text_chunk.height);
        // dbg!(&total_height);
        scroll_view.render_widget(title.clone(), title_chunk);
        scroll_view.render_widget(text.clone(), text_chunk);
    }
    let mut state = app.details_scroll_view_state.clone();
    f.render_stateful_widget(scroll_view, chunk, &mut state);
}
