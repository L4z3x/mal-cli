use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

pub fn draw_centered_line(f: &mut Frame, app: &App, chunk: Rect, line: &str) {
    let [loading_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2)])
        .flex(Flex::Center)
        .areas(chunk);

    // let loading_str = "Loading...";

    let style = Style::new()
        .fg(app.app_config.theme.banner)
        .add_modifier(Modifier::BOLD);

    let loading_line = Line::styled(line, style);

    let paragraph = Paragraph::new(loading_line)
        // .block(paragraph_block)
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(paragraph, loading_layout);
}
