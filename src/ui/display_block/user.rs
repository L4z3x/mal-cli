use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

use super::{center_area, empty::draw_figlet};

pub fn draw_user_info(f: &mut Frame, app: &App, chunk: Rect) {
    let [username_chunk, info_chunk] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Fill(1)])
        .areas(chunk);

    // draw border:
    let block = Block::default().borders(Borders::BOTTOM);
    f.render_widget(block, center_area(username_chunk, 95, 100));

    let username = app.user_profile.as_ref().unwrap().name.clone();
    draw_figlet(f, username, username_chunk, app);
    // extracting data:
    let gauge_chunk = draw_info(f, app, info_chunk);
    let block = Block::default().borders(Borders::RIGHT);
    f.render_widget(block, center_area(gauge_chunk, 100, 95));
    draw_gauges(f, app, gauge_chunk);
}

fn draw_gauges(f: &mut Frame, app: &App, layout: Rect) {
    let stats = app.user_profile.as_ref().unwrap().anime_statistics.as_ref();
    if stats.is_none() {
        draw_no_stats(f, app, layout);
        return;
    }
    let stats = stats.unwrap().clone();

    let watching = stats.num_items_watching;
    let completed = stats.num_items_completed;
    let on_hold = stats.num_items_on_hold;
    let dropped = stats.num_items_dropped;
    let plan_to_watch = stats.num_items_plan_to_watch;
    let total_entries = stats.num_items;
    let layout: [Rect; 5] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3); 5])
        .flex(Flex::SpaceAround)
        .areas(center_area(layout, 70, 80));

    let block = Block::default()
        .border_type(BorderType::Plain)
        .borders(Borders::ALL);

    let watching_title = Paragraph::new(format!("Watching: {}", watching))
        .style(Style::default().fg(app.app_config.theme.text))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    let watching_gauge = Gauge::default()
        .block(block.clone())
        .gauge_style(app.app_config.theme.status_watching)
        .ratio(watching as f64 / total_entries as f64)
        .label(format!(
            "{:.0}%",
            (watching as f64 / total_entries as f64) * 100.0
        ));

    let completed_title = Paragraph::new(format!("Completed: {}", completed))
        .style(Style::default().fg(app.app_config.theme.text))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    let completed_gauge = Gauge::default()
        .block(block.clone())
        .gauge_style(app.app_config.theme.status_watching)
        .gauge_style(app.app_config.theme.status_completed)
        .ratio(completed as f64 / total_entries as f64)
        .label(format!(
            "{:.0}%",
            (completed as f64 / total_entries as f64) * 100.0
        ));

    let on_hold_title = Paragraph::new(format!("On Hold: {}", on_hold))
        .style(Style::default().fg(app.app_config.theme.text))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    let on_hold_gauge = Gauge::default()
        .block(block.clone())
        .gauge_style(app.app_config.theme.status_watching)
        .gauge_style(app.app_config.theme.status_on_hold)
        .ratio(on_hold as f64 / total_entries as f64)
        .label(format!(
            "{:.0}%",
            (on_hold as f64 / total_entries as f64) * 100.0
        ));

    let dropped_title = Paragraph::new(format!("Dropped: {}", dropped))
        .style(Style::default().fg(app.app_config.theme.text))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    let dropped_gauge = Gauge::default()
        .block(block.clone())
        .gauge_style(app.app_config.theme.status_watching)
        .gauge_style(app.app_config.theme.status_dropped)
        .ratio(dropped as f64 / total_entries as f64)
        .label(format!(
            "{:.0}%",
            (dropped as f64 / total_entries as f64) * 100.0
        ));
    let plan_to_watch_title = Paragraph::new(format!("Plan to Watch: {}", plan_to_watch))
        .style(Style::default().fg(app.app_config.theme.text))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    let plan_to_watch_gauge = Gauge::default()
        .block(block.clone())
        .gauge_style(app.app_config.theme.status_plan_to_watch)
        .ratio(plan_to_watch as f64 / total_entries as f64)
        .label(format!(
            "{:.0}%",
            (plan_to_watch as f64 / total_entries as f64) * 100.0
        ));

    let percent_y = 40;
    let percent_x = 100;
    let chunk = get_stat_chunk(layout[0]);
    f.render_widget(watching_title, center_area(chunk[0], percent_x, percent_y));
    f.render_widget(watching_gauge, chunk[1]);

    let chunk = get_stat_chunk(layout[1]);
    f.render_widget(completed_title, center_area(chunk[0], percent_x, percent_y));
    f.render_widget(completed_gauge, chunk[1]);

    let chunk = get_stat_chunk(layout[2]);
    f.render_widget(on_hold_title, center_area(chunk[0], percent_x, percent_y));
    f.render_widget(on_hold_gauge, chunk[1]);

    let chunk = get_stat_chunk(layout[3]);
    f.render_widget(dropped_title, center_area(chunk[0], percent_x, percent_y));
    f.render_widget(dropped_gauge, chunk[1]);

    let chunk = get_stat_chunk(layout[4]);
    f.render_widget(
        plan_to_watch_title,
        center_area(chunk[0], percent_x, percent_y),
    );
    f.render_widget(plan_to_watch_gauge, chunk[1]);
}

fn get_stat_chunk(layout: Rect) -> [Rect; 2] {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .areas(layout)
}

fn draw_no_stats(f: &mut Frame, app: &App, chunk: Rect) {
    let text = "No statistics available !! Add some anime to your list!";

    let line = Line::from(Span::styled(
        text,
        Style::default().fg(app.app_config.theme.text),
    ));
    let paragraph = Paragraph::new(line)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    let centered_chunk = center_area(chunk, 100, 20);
    f.render_widget(paragraph, centered_chunk);
}

fn draw_info(f: &mut Frame, app: &App, chunk: Rect) -> Rect {
    let layout: [Rect; 2] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100), Constraint::Min(35)])
        .areas(chunk);

    let joined_at = app.user_profile.as_ref().unwrap().joined_at.clone();
    let mut total_items = 0;
    let mut mean_score = 0.0;
    let mut total_eps = 0;
    let mut total_days = 0.0;
    let stats = app.user_profile.as_ref().unwrap().anime_statistics.as_ref();
    if stats.is_some() {
        total_items = stats.unwrap().num_items;
        mean_score = stats.unwrap().mean_score;
        total_days = stats.unwrap().num_days;
        total_eps = stats.unwrap().num_episodes;
    }

    let location = app
        .user_profile
        .as_ref()
        .unwrap()
        .location
        .clone()
        .unwrap_or("".to_string());

    let mut list = vec![
        (
            "Joined at: ".to_string(),
            joined_at.datetime.date().to_string(),
        ),
        ("Total Entries: ".to_string(), total_items.to_string()),
    ];
    if mean_score != 0.0 {
        let mean_score = format!("{:.2}", mean_score);
        list.push(("Mean Score: ".to_string(), mean_score));
    }
    if total_eps != 0 {
        list.push(("Total Episodes: ".to_string(), format!("{} Eps", total_eps)));
    }
    if total_days != 0.0 {
        let total_days = {
            let hours = ((total_days) * 24.0).floor();
            format!(" {:.0} hours", hours)
        };
        list.push(("Total Time: ".to_string(), total_days));
    }

    if !location.is_empty() {
        list.push(("Location: ".to_string(), location));
    }
    let birthday = app
        .user_profile
        .as_ref()
        .unwrap()
        .birthday
        .clone()
        .map_or("".to_string(), |b| b.date.to_string());

    let gender = app
        .user_profile
        .as_ref()
        .unwrap()
        .gender
        .clone()
        .map_or("".to_string(), |g| g.to_string());

    let is_supperter = app.user_profile.as_ref().unwrap().is_supporter;

    if !gender.is_empty() {
        list.push(("Gender: ".to_string(), gender));
    }
    if !birthday.is_empty() {
        list.push(("Birthday: ".to_string(), birthday));
    }

    if is_supperter.is_none() && is_supperter.unwrap() {
        list.push(("Supporter: ".to_string(), "Yes".to_string()));
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            list.iter()
                .map(|_| Constraint::Percentage(100 / list.len() as u16))
                .collect::<Vec<Constraint>>(),
        )
        .split(center_area(layout[1], 90, 80));
    for (i, item) in list.iter().enumerate() {
        let attr = Span::styled(
            item.0.clone(),
            Style::default()
                .fg(app.app_config.theme.text)
                .add_modifier(Modifier::BOLD),
        );
        let item = Span::styled(
            item.1.clone(),
            Style::default().fg(app.app_config.theme.active),
        );
        let line = Line::from(vec![attr, item]);
        let paragraph = Paragraph::new(line).alignment(Alignment::Left);
        f.render_widget(paragraph, chunks[i]);
    }

    layout[0]
}
