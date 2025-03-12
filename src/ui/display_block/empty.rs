use crate::app::App;
use crate::BANNER;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

pub fn draw_empty(f: &mut Frame, app: &App, chunk: Rect) {
    let banner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ])
        .split(chunk)[1];

    let banner_lines: Vec<&str> = BANNER.lines().collect();

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
