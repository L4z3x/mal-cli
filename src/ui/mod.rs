pub mod help;
mod side_menu;
mod top_three;
pub mod util;
use crate::app::*;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use util::get_color;
mod display_block;

pub fn draw_main_layout(f: &mut Frame, app: &App) {
    let margin = util::get_main_layout_margin(app);
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .margin(margin)
        .split(f.area());

    // Search Input and help
    draw_input_and_help_box(f, app, parent_layout[0]);

    // Draw dashboard
    side_menu::draw_routes(f, app, parent_layout[1]);
}

pub fn draw_input_and_help_box(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(17), Constraint::Percentage(82)])
        .flex(Flex::SpaceBetween)
        .split(layout_chunk);

    let current_block = app.active_block;

    let highlight_state = current_block == ActiveBlock::Input;

    let input_string: String = app.input.iter().collect();
    let lines = Span::from(input_string);
    let input = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(Span::styled(
                "Search",
                get_color(highlight_state, app.app_config.theme),
            ))
            .border_style(get_color(highlight_state, app.app_config.theme)),
    );
    f.render_widget(input, chunks[0]);

    let mut title = app.display_block_title.clone();
    if title.is_empty() {
        title = "Home".to_string(); // Default title , since i couldn't initialize it in app.rs:15
    }
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.app_config.theme.inactive));

    let lines = Line::from(Span::from(title))
        .alignment(Alignment::Center)
        .style(Style::default().fg(app.app_config.theme.banner));

    let help = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(app.app_config.theme.banner));
    f.render_widget(help, chunks[1]);
}

pub fn format_number_with_commas(number: u64) -> String {
    let num_str = number.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in num_str.chars().rev() {
        if count == 3 {
            result.push(',');
            count = 0;
        }
        result.push(c);
        count += 1;
    }

    result.chars().rev().collect()
}
