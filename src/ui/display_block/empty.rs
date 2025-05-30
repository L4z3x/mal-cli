use crate::app::App;
use figlet_rs::FIGfont;
use ratatui::layout::Flex;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

pub fn draw_empty(f: &mut Frame, app: &App, chunk: Rect) {
    let [banner_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6)])
        .flex(Flex::Center)
        .areas(chunk);
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
    let block = Block::default();
    let height = spans.len() as u16;
    let paragraph = Paragraph::new(spans)
        .block(block)
        .alignment(Alignment::Center);

    let [centered_chunk] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(chunk);
    f.render_widget(paragraph, centered_chunk);
}
