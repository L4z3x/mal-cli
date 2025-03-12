use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

pub fn draw_loading(f: &mut Frame, app: &App, chunk: Rect) {
    let banner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // Top padding
            Constraint::Percentage(20), // Height of the paragraph block
            Constraint::Percentage(40), // Bottom padding
        ])
        .split(chunk)[1];

    let banner_lines: Vec<&str> = "Loading...".lines().collect();

    let style = Style::new()
        .fg(app.app_config.theme.banner)
        .add_modifier(Modifier::BOLD);

    let spans: Vec<Line> = banner_lines
        .iter()
        .map(|line| Line::styled(*line, style))
        .collect();

    let paragraph = Paragraph::new(spans)
        // .block(paragraph_block)
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(paragraph, banner_layout);
}
