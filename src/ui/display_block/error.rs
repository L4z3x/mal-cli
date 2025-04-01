use super::center_area;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn draw_error(f: &mut Frame, app: &App, chunk: Rect) {
    let error_raw = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ])
        .split(chunk)[1];

    let error_box = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Percentage(35),
        ])
        .split(error_raw)[1];

    let error_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.app_config.theme.error_border))
        .border_type(BorderType::Double)
        .title(Span::styled(
            "ERROR",
            Style::default().fg(app.app_config.theme.error_text),
        ))
        .title_alignment(Alignment::Center);

    f.render_widget(error_block.clone(), error_box.clone());

    let error_text = Line::from(app.api_error.clone())
        .centered()
        .alignment(Alignment::Center);

    let error_paragraph = Paragraph::new(error_text)
        // .style(Style::default().fg(app.app_config.theme.text))
        // .block(error_block)
        .wrap(Wrap { trim: true });

    let centered_box = center_area(error_box, 100, 50);
    f.render_widget(error_paragraph, centered_box);
}
