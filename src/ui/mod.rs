pub mod help;
pub mod util;
use crate::api::model::*;
use crate::app::*;
// use color_eyre::config::Frame;
use ratatui::{
    // buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Backend,
    style::{Color, Modifier, Style, Stylize},
    text::{Span, Text},
    widgets::{
        Block, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Widget, Wrap,
    },
    Frame, // DefaultTerminal, Frame,
};
use util::get_color;
mod display_block;
pub enum TableId {
    Anime,
    Manga,
    User,
    AnimeList,
    MangaList,
}

#[derive(PartialEq)]
pub enum ColumnId {
    None,
    Anime,
    Manga,
}

impl Default for ColumnId {
    fn default() -> Self {
        ColumnId::None
    }
}

pub struct TableHeader<'a> {
    id: TableId,
    items: Vec<TableHeaderItem<'a>>,
}

impl TableHeader<'_> {
    pub fn get_index(&self, id: ColumnId) -> Option<usize> {
        self.items.iter().position(|item| item.id == id)
    }
}

#[derive(Default)]
pub struct TableHeaderItem<'a> {
    id: ColumnId,
    text: &'a str,
    width: u16,
}

pub struct TableItem {
    id: String,
    format: Vec<String>,
}

// from chatgpt
pub fn draw_help_menu(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(2)
        .split(f.area());

    let white = Style::default().fg(app.app_config.theme.text);
    let gray = Style::default().fg(Color::Gray); //

    let header = ["Description", "Event", "Context"];
    let help_docs = help::get_help();
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
        .widths(&[
            Constraint::Length(50),
            Constraint::Length(40),
            Constraint::Length(20),
        ]);

    f.render_widget(help_menu, chunks[0]);
}

pub fn draw_error(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.area());

    let error = vec![
        Span::from(Span::from("Api response: ")),
        Span::from(Span::styled(
            &app.api_error,
            Style::default().fg(app.app_config.theme.error_text),
        )),
        Span::from(Span::styled(
            "Your api thing may be wrong idk man",
            Style::default().fg(app.app_config.theme.text),
        )),
        Span::from(Span::styled(
            "Hint: Maybe do something again",
            Style::default().fg(app.app_config.theme.hint),
        )),
        Span::from(Span::styled(
            "\n Press <Esc> to return",
            Style::default().fg(app.app_config.theme.inactive),
        )),
    ];

    let error = Text::from_iter(error);
    let error_paragraph = Paragraph::new(error)
        .style(Style::default().fg(app.app_config.theme.text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(app.app_config.theme.error_border))
                .title(Span::styled(
                    "Error",
                    Style::default().fg(app.app_config.theme.error_border),
                )),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(error_paragraph, chunks[0]);
}

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
    draw_routes(f, app, parent_layout[1]);
}

pub fn draw_input_and_help_box(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(layout_chunk);

    let current_route = app.active_block;

    let highlight_state = (
        current_route == ActiveBlock::Input,
        current_route == ActiveBlock::Input,
    );

    let input_string: String = app.input.iter().collect();
    let lines = Span::from(input_string);
    let input = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                "Search",
                get_color(highlight_state, app.app_config.theme),
            ))
            .border_style(get_color(highlight_state, app.app_config.theme)),
    );
    f.render_widget(input, chunks[0]);

    let show_loading = app.is_loading && app.app_config.behavior.show_loading_indicator;
    let help_block_text = if show_loading {
        (app.app_config.theme.hint, "Loading...")
    } else {
        (app.app_config.theme.inactive, "Type ?")
    };

    let block = Block::default()
        .title(Span::styled("Help", Style::default().fg(help_block_text.0)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(help_block_text.0));

    let lines = Span::from(help_block_text.1);
    let help = Paragraph::new(lines)
        .block(block)
        .style(Style::default().fg(help_block_text.0));
    f.render_widget(help, chunks[1]);
}

pub fn draw_routes(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    draw_user_block(f, app, chunks[0]);

    // let current_route = app.active_block;

    display_block::draw_display_layout(f, app, chunks[1]);
}

pub fn draw_anime_routes(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let current_block = app.active_block;
    let highlight_state = (
        current_block == ActiveBlock::Anime,
        current_block == ActiveBlock::Anime,
    );

    let items: Vec<ListItem> = ANIME_OPTIONS
        .iter()
        .map(|i| ListItem::new(*i).style(Style::default().fg(app.app_config.theme.text)))
        .collect();

    let mut index = Some(app.library.selected_index);
    if !ANIME_OPTIONS_RANGE.contains(&app.library.selected_index) {
        index = None;
    }

    draw_selectable_list(f, app, layout_chunk, "Anime", items, highlight_state, index)
}

pub fn draw_manga_routes(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let current_block = app.active_block;
    let highlight_state = (
        current_block == ActiveBlock::Manga,
        current_block == ActiveBlock::Manga,
    );

    let items: Vec<ListItem> = MANGA_OPTIONS
        .iter()
        .map(|i| ListItem::new(*i).style(Style::default().fg(app.app_config.theme.text)))
        .collect();

    let mut index = Some(app.library.selected_index);
    if !MANGA_OPTIONS_RANGE.contains(&app.library.selected_index) {
        index = None;
    }

    draw_selectable_list(f, app, layout_chunk, "Manga", items, highlight_state, index);
}

pub fn draw_user_routes(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let current_block = app.active_block;
    let highlight_state = (
        current_block == ActiveBlock::User,
        current_block == ActiveBlock::User,
    );

    let items: Vec<ListItem> = USER_OPTIONS
        .iter()
        .map(|i| ListItem::new(*i).style(Style::default().fg(app.app_config.theme.text)))
        .collect();

    let mut index = Some(app.library.selected_index);
    if !USER_OPTIONS_RANGE.contains(&app.library.selected_index) {
        index = None;
    }

    draw_selectable_list(f, app, layout_chunk, "User", items, highlight_state, index);
}

pub fn draw_user_block(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(layout_chunk);

    draw_anime_routes(f, app, chunks[0]);
    draw_manga_routes(f, app, chunks[1]);
    draw_user_routes(f, app, chunks[2]);
}

pub fn draw_selectable_list(
    f: &mut Frame,
    app: &App,
    layout_chunk: Rect,
    title: &str,
    items: Vec<ListItem>,
    highlight_state: (bool, bool),
    selected_index: Option<usize>,
) {
    let mut state = ListState::default();
    if selected_index.is_some() {
        // dbg!(selected_index.unwrap() % items.len());
        state.select(Some(selected_index.unwrap() % items.len()));
    }

    // choose color based on hover state
    let highlight_color = app.app_config.theme.hovered;

    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(title, Style::default().fg(highlight_color)))
                .border_style(Style::default().fg(highlight_color)),
        )
        .highlight_style(
            Style::default()
                .fg(highlight_color)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(items, layout_chunk, &mut state);
}

// match current_route.id {
//     RouteId::Search => {
//         draw_search_results(f, app, chunks[1]);
//     }
//     RouteId::Error => {}
//     _ => {
//         draw_dashboard(f, app, chunks[1]);
//     }
// };

//
//             let items: Vec<ListItem> = app
//                 .items
//                 .items
//                 .iter()
//                 .map(|i| {
//                     let mut lines = vec![Spans::from(i.0)];
//                     for _ in 0..i.1 {
//                         lines.push(Spans::from(Span::styled(
//                             "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
//                             Style::default().add_modifier(Modifier::ITALIC),
//                         )));
//                     }
//                     ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
//                 })
//                 .collect();
//             let items = List::new(items)
//                 .block(Block::default().borders(Borders::ALL).title("List"))
//                 .highlight_style(
//                     Style::default()
//                         .bg(Color::LightGreen)
//                         .add_modifier(Modifier::BOLD),
//                 )
//                 .highlight_symbol(">> ");
//             f.render_stateful_widget(items, chunks[0], &mut app.items.state);
//
//             let events: Vec<ListItem> = app
//                 .events
//                 .iter()
//                 .map(|&(evt, level)| {
//                     let s = match level {
//                         "CRITICAL" => Style::default().fg(Color::Red),
//                         "ERROR" => Style::default().fg(Color::Magenta),
//                         "WARNING" => Style::default().fg(Color::Yellow),
//                         "INFO" => Style::default().fg(Color::Blue),
//                         _ => Style::default(),
//                     };
//                     let header = Spans::from(vec![
//                         Span::styled(format!("{:<9}", level), s),
//                         Span::raw(" "),
//                         Span::styled(
//                             "2020-01-01 10:00:00",
//                             Style::default().add_modifier(Modifier::ITALIC),
//                         ),
//                     ]);
//                     let log = Spans::from(vec![Span::raw(evt)]);
//                     ListItem::new(vec![
//                         Spans::from("-".repeat(chunks[1].width as usize)),
//                         header,
//                         Spans::from(""),
//                         log,
//                     ])
//                 })
//                 .collect();
//             let events_list = List::new(events)
//                 .block(Block::default().borders(Borders::ALL).title("List"))
//                 .start_corner(Corner::BottomLeft);
//             f.render_widget(events_list, chunks[1]);
//         })?;
// pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
//     let chunks = Layout::default()
//         .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
//         .split(f.size());
//     let titles = app
//         .tabs
//         .titles
//         .iter()
//         .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
//         .collect();
//     let tabs = Tabs::new(titles)
//         .block(Block::default().borders(Borders::ALL).title(app.title))
//         .highlight_style(Style::default().fg(Color::Yellow))
//         .select(app.tabs.index);
//     f.render_widget(tabs, chunks[0]);
//     match app.tabs.index {
//         0 => draw_first_tab(f, app, chunks[1]),
//         1 => draw_second_tab(f, app, chunks[1]),
//         _ => {}
//     };
// }

// fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
// where
//     B: Backend,
// {
//     let chunks = Layout::default()
//         .constraints(
//             [
//                 Constraint::Length(7),
//                 Constraint::Min(7),
//                 Constraint::Length(7),
//             ]
//             .as_ref(),
//         )
//         .split(area);
//     draw_gauges(f, app, chunks[0]);
//     draw_charts(f, app, chunks[1]);
//     draw_text(f, chunks[2]);
// }

// fn draw_gauges<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
// where
//     B: Backend,
// {
//     let chunks = Layout::default()
//         .constraints([Constraint::Length(2), Constraint::Length(3)].as_ref())
//         .margin(1)
//         .split(area);
//     let block = Block::default().borders(Borders::ALL).title("Graphs");
//     f.render_widget(block, area);

//     let label = format!("{:.2}%", app.progress * 100.0);
//     let gauge = Gauge::default()
//         .block(Block::default().title("Gauge:"))
//         .gauge_style(
//             Style::default()
//                 .fg(Color::Magenta)
//                 .bg(Color::Black)
//                 .add_modifier(Modifier::ITALIC | Modifier::BOLD),
//         )
//         .label(label)
//         .ratio(app.progress);
//     f.render_widget(gauge, chunks[0]);

//     let sparkline = Sparkline::default()
//         .block(Block::default().title("Sparkline:"))
//         .style(Style::default().fg(Color::Green))
//         .data(&app.sparkline.points)
//         .bar_set(if app.enhanced_graphics {
//             symbols::bar::NINE_LEVELS
//         } else {
//             symbols::bar::THREE_LEVELS
//         });
//     f.render_widget(sparkline, chunks[1]);
// }

// fn draw_charts<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
// where
//     B: Backend,
// {
//     let constraints = if app.show_chart {
//         vec![Constraint::Percentage(50), Constraint::Percentage(50)]
//     } else {
//         vec![Constraint::Percentage(100)]
//     };
//     let chunks = Layout::default()
//         .constraints(constraints)
//         .direction(Direction::Horizontal)
//         .split(area);
//     {
//         let chunks = Layout::default()
//             .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
//             .split(chunks[0]);
//         {
//             let chunks = Layout::default()
//                 .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
//                 .direction(Direction::Horizontal)
//                 .split(chunks[0]);

//             // Draw tasks
//             let tasks: Vec<ListItem> = app
//                 .tasks
//                 .items
//                 .iter()
//                 .map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
//                 .collect();
//             let tasks = List::new(tasks)
//                 .block(Block::default().borders(Borders::ALL).title("List"))
//                 .highlight_style(Style::default().add_modifier(Modifier::BOLD))
//                 .highlight_symbol("> ");
//             f.render_stateful_widget(tasks, chunks[0], &mut app.tasks.state);

//             // Draw logs
//             let info_style = Style::default().fg(Color::Blue);
//             let warning_style = Style::default().fg(Color::Yellow);
//             let error_style = Style::default().fg(Color::Magenta);
//             let critical_style = Style::default().fg(Color::Red);
//             let logs: Vec<ListItem> = app
//                 .logs
//                 .items
//                 .iter()
//                 .map(|&(evt, level)| {
//                     let s = match level {
//                         "ERROR" => error_style,
//                         "CRITICAL" => critical_style,
//                         "WARNING" => warning_style,
//                         _ => info_style,
//                     };
//                     let content = vec![Spans::from(vec![
//                         Span::styled(format!("{:<9}", level), s),
//                         Span::raw(evt),
//                     ])];
//                     ListItem::new(content)
//                 })
//                 .collect();
//             let logs = List::new(logs).block(Block::default().borders(Borders::ALL).title("List"));
//             f.render_stateful_widget(logs, chunks[1], &mut app.logs.state);
//         }

//         let barchart = BarChart::default()
//             .block(Block::default().borders(Borders::ALL).title("Bar chart"))
//             .data(&app.barchart)
//             .bar_width(3)
//             .bar_gap(2)
//             .bar_set(if app.enhanced_graphics {
//                 symbols::bar::NINE_LEVELS
//             } else {
//                 symbols::bar::THREE_LEVELS
//             })
//             .value_style(
//                 Style::default()
//                     .fg(Color::Black)
//                     .bg(Color::Green)
//                     .add_modifier(Modifier::ITALIC),
//             )
//             .label_style(Style::default().fg(Color::Yellow))
//             .bar_style(Style::default().fg(Color::Green));
//         f.render_widget(barchart, chunks[1]);
//     }
//     if app.show_chart {
//         let x_labels = vec![
//             Span::styled(
//                 format!("{}", app.signals.window[0]),
//                 Style::default().add_modifier(Modifier::BOLD),
//             ),
//             Span::raw(format!(
//                 "{}",
//                 (app.signals.window[0] + app.signals.window[1]) / 2.0
//             )),
//             Span::styled(
//                 format!("{}", app.signals.window[1]),
//                 Style::default().add_modifier(Modifier::BOLD),
//             ),
//         ];
//         let datasets = vec![
//             Dataset::default()
//                 .name("data2")
//                 .marker(symbols::Marker::Dot)
//                 .style(Style::default().fg(Color::Cyan))
//                 .data(&app.signals.sin1.points),
//             Dataset::default()
//                 .name("data3")
//                 .marker(if app.enhanced_graphics {
//                     symbols::Marker::Braille
//                 } else {
//                     symbols::Marker::Dot
//                 })
//                 .style(Style::default().fg(Color::Yellow))
//                 .data(&app.signals.sin2.points),
//         ];
//         let chart = Chart::new(datasets)
//             .block(
//                 Block::default()
//                     .title(Span::styled(
//                         "Chart",
//                         Style::default()
//                             .fg(Color::Cyan)
//                             .add_modifier(Modifier::BOLD),
//                     ))
//                     .borders(Borders::ALL),
//             )
//             .x_axis(
//                 Axis::default()
//                     .title("X Axis")
//                     .style(Style::default().fg(Color::Gray))
//                     .bounds(app.signals.window)
//                     .labels(x_labels),
//             )
//             .y_axis(
//                 Axis::default()
//                     .title("Y Axis")
//                     .style(Style::default().fg(Color::Gray))
//                     .bounds([-20.0, 20.0])
//                     .labels(vec![
//                         Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
//                         Span::raw("0"),
//                         Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
//                     ]),
//             );
//         f.render_widget(chart, chunks[1]);
//     }
// }

// fn draw_text<B>(f: &mut Frame<B>, area: Rect)
// where
//     B: Backend,
// {
//     let text = vec![
//         Spans::from("This is a paragraph with several lines. You can change style your text the way you want"),
//         Spans::from(""),
//         Spans::from(vec![
//             Span::from("For example: "),
//             Span::styled("under", Style::default().fg(Color::Red)),
//             Span::raw(" "),
//             Span::styled("the", Style::default().fg(Color::Green)),
//             Span::raw(" "),
//             Span::styled("rainbow", Style::default().fg(Color::Blue)),
//             Span::raw("."),
//         ]),
//         Spans::from(vec![
//             Span::raw("Oh and if you didn't "),
//             Span::styled("notice", Style::default().add_modifier(Modifier::ITALIC)),
//             Span::raw(" you can "),
//             Span::styled("automatically", Style::default().add_modifier(Modifier::BOLD)),
//             Span::raw(" "),
//             Span::styled("wrap", Style::default().add_modifier(Modifier::REVERSED)),
//             Span::raw(" your "),
//             Span::styled("text", Style::default().add_modifier(Modifier::UNDERLINED)),
//             Span::raw(".")
//         ]),
//         Spans::from(
//             "One more thing is that it should display unicode characters: 10€"
//         ),
//     ];
//     let block = Block::default().borders(Borders::ALL).title(Span::styled(
//         "Footer",
//         Style::default()
//             .fg(Color::Magenta)
//             .add_modifier(Modifier::BOLD),
//     ));
//     let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
//     f.render_widget(paragraph, area);
// }

// fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
// where
//     B: Backend,
// {
//     let chunks = Layout::default()
//         .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
//         .direction(Direction::Horizontal)
//         .split(area);
//     let up_style = Style::default().fg(Color::Green);
//     let failure_style = Style::default()
//         .fg(Color::Red)
//         .add_modifier(Modifier::RAPID_BLINK | Modifier::CROSSED_OUT);
//     let header = ["Server", "Location", "Status"];
//     let rows = app.servers.iter().map(|s| {
//         let style = if s.status == "Up" {
//             up_style
//         } else {
//             failure_style
//         };
//         Row::StyledData(vec![s.name, s.location, s.status].into_iter(), style)
//     });
//     let table = Table::new(header.iter(), rows)
//         .block(Block::default().title("Servers").borders(Borders::ALL))
//         .header_style(Style::default().fg(Color::Yellow))
//         .widths(&[
//             Constraint::Length(15),
//             Constraint::Length(15),
//             Constraint::Length(10),
//         ]);
//     f.render_widget(table, chunks[0]);

//     let map = Canvas::default()
//         .block(Block::default().title("World").borders(Borders::ALL))
//         .paint(|ctx| {
//             ctx.draw(&Map {
//                 color: Color::White,
//                 resolution: MapResolution::High,
//             });
//             ctx.layer();
//             ctx.draw(&Rectangle {
//                 x: 0.0,
//                 y: 30.0,
//                 width: 10.0,
//                 height: 10.0,
//                 color: Color::Yellow,
//             });
//             for (i, s1) in app.servers.iter().enumerate() {
//                 for s2 in &app.servers[i + 1..] {
//                     ctx.draw(&Line {
//                         x1: s1.coords.1,
//                         y1: s1.coords.0,
//                         y2: s2.coords.0,
//                         x2: s2.coords.1,
//                         color: Color::Yellow,
//                     });
//                 }
//             }
//             for server in &app.servers {
//                 let color = if server.status == "Up" {
//                     Color::Green
//                 } else {
//                     Color::Red
//                 };
//                 ctx.print(server.coords.1, server.coords.0, "X", color);
//             }
//         })
//         .marker(if app.enhanced_graphics {
//             symbols::Marker::Braille
//         } else {
//             symbols::Marker::Dot
//         })
//         .x_bounds([-180.0, 180.0])
//         .y_bounds([-90.0, 90.0]);
//     f.render_widget(map, chunks[1]);
