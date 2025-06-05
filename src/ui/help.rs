use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub fn draw_help_menu(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(2)
        .split(f.area());

    let white = Style::default().fg(app.app_config.theme.text);
    let gray = Style::default().fg(Color::Gray); //

    let header = ["Description", "Event", "Context"];
    let help_docs = get_help();
    let help_docs: &[Vec<&str>] = &help_docs[app.help_menu_offset as usize..];

    let rows: Vec<Row> = help_docs
        .iter()
        .map(|i| -> Row {
            Row::new(
                i.iter()
                    .map(|&cell| -> Cell { Cell::from(cell).style(gray) })
                    .collect::<Vec<Cell>>(),
            )
        })
        .collect::<Vec<Row>>();

    let header = Row::new(
        header
            .iter()
            .map(|&header| Cell::from(header).style(white))
            .collect::<Vec<Cell>>(),
    );

    let help_menu = Table::default()
        .rows(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(white)
                .title(Span::styled("Help (press <Esc> to go back)", gray))
                .border_style(gray),
        )
        .style(Style::default().fg(app.app_config.theme.text))
        .widths([
            Constraint::Length(50),
            Constraint::Length(40),
            Constraint::Length(20),
        ]);

    f.render_widget(help_menu, chunks[0]);
}

pub fn get_help() -> Vec<Vec<&'static str>> {
    // TODO: Help docs
    vec![vec!["Down", "j", "Pagination"]]
}
