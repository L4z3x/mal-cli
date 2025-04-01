use crate::app::App;
use figlet_rs::FIGfont;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Padding};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use super::center_area;

pub fn draw_empty(f: &mut Frame, app: &App, chunk: Rect) {
    let banner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ])
        .split(chunk)[1];

    draw_figlet(f, "MAL-TUI".to_string(), banner_layout, app);
}

pub fn draw_figlet(f: &mut Frame, string: String, chunk: Rect, app: &App) {
    let standard_font = FIGfont::standard().unwrap();
    let figlet = standard_font.convert(&string);
    let fig_string = figlet.unwrap().to_string();

    let banner_lines: Vec<&str> = fig_string.lines().collect();

    let style = Style::new()
        .fg(app.app_config.theme.banner)
        .add_modifier(Modifier::BOLD);

    let spans: Vec<Line> = banner_lines
        .iter()
        .map(|line| Line::styled(*line, style))
        .collect();
    let block = Block::default().padding(Padding::new(0, 0, 1, 0));
    let paragraph = Paragraph::new(spans)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });
    let centered_chunk = center_area(chunk, 100, 80);
    f.render_widget(paragraph, centered_chunk);
}
