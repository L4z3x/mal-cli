use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, Paragraph, Wrap},
    Frame,
};
use ratatui_image::StatefulImage;
use tui_scrollview::{ScrollView, ScrollbarVisibility};

use crate::{
    api::model::{
        AlternativeTitles, AnimeMediaType, AnimeStatus, MangaMediaType, MangaStatus, Source,
    },
    app::App,
};

use super::center_area;

pub fn get_score_text(s: u8) -> String {
    let r = match s {
        1 => "(1) Appalling",
        2 => "(2) Horrible",
        3 => "(3) Very Bad",
        4 => "(4) Bad",
        5 => "(5) Average",
        6 => "(6) Fine",
        7 => "(7) Good",
        8 => "(8) Very Good",
        9 => "(9) Great",
        10 => "(10) Masterpiece",
        _ => "rate",
    };
    r.to_string()
}

pub fn draw_picture(f: &mut Frame, app: &mut App, chunk: Rect) {
    // let chunk = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints([Constraint::Length(1), Constraint::Fill(1)])
    //     .split(chunk)[1];

    if let Some(_) = app.media_image {
        if let (Some(_), Some(_)) = (&app.picker, &app.image_state) {
            let image = StatefulImage::default();

            let image_width = app.media_image.as_ref().unwrap().1;
            let image_height = app.media_image.as_ref().unwrap().2;

            let perc_x = ((chunk.width as f32 / image_width as f32) * 100.0) as u16;
            let perc_y = ((chunk.height as f32 / image_height as f32) * 100.0) as u16;
            f.render_stateful_widget(
                image,
                center_area(chunk, 100 - perc_x, 100 - perc_y),
                &mut app.image_state.as_mut().unwrap(),
            );
        }
    } else {
        draw_image_place_holder(f, chunk);
    }
}

fn draw_image_place_holder(f: &mut Frame, chunk: Rect) {
    draw_bordered_block(f, chunk);
    let paragraph = Paragraph::new("Unable to render the image.")
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, center_area(chunk, 95, 30));
}

pub fn draw_bordered_block(f: &mut Frame, chunk: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Yellow));

    f.render_widget(block, chunk);
}

pub fn get_text_prop(
    title: String,
    text: String,
    app: &App,
) -> (Line<'static>, List<'static>, u16) {
    let text_title = Line::raw(title).style(
        Style::default()
            .fg(app.app_config.theme.text)
            .add_modifier(Modifier::BOLD),
    );
    let text_lines = text.lines();
    let mut text_height = 0;
    let mut text_lines_ui = Vec::new();

    for line in text_lines {
        let (height, line) = construct_paragraph_lines(&line, 80, 1, 1);
        line.iter().for_each(|l| text_lines_ui.push(l.clone()));
        text_height += height;
    }

    let text = List::new(text_lines_ui);
    (text_title, text, text_height)
}

pub fn draw_synopsis_items(
    f: &mut Frame,
    app: &App,
    total_height: u16,
    layout_items: Vec<(Line<'static>, List<'static>, u16)>,
    chunk: Rect,
) {
    let scroll_view_rect = Rect::new(chunk.x, chunk.y, chunk.width, total_height);
    let mut constraints = layout_items
        .iter()
        .flat_map(|(_, _, height)| {
            vec![
                Constraint::Length(1),           // title
                Constraint::Length(1),           // spacing
                Constraint::Length(*height + 1), // text
                Constraint::Length(1),           // spacing
            ]
        })
        .collect::<Vec<_>>();
    constraints.push(Constraint::Percentage(100));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .flex(Flex::Start)
        .split(scroll_view_rect)
        .to_vec();

    let mut scroll_view = ScrollView::new(scroll_view_rect.as_size())
        .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

    for (index, (title, text, _)) in layout_items.iter().enumerate() {
        let title_chunk = Rect::new(
            layout[index * 4].x - chunk.x,
            layout[index * 4].y - chunk.y,
            layout[index * 4].width,
            layout[index * 4].height,
        );
        let text_chunk = Rect::new(
            layout[index * 4 + 2].x - chunk.x,
            layout[index * 4 + 2].y - chunk.y,
            layout[index * 4 + 2].width,
            layout[index * 4 + 2].height,
        );
        // dbg!(&title_chunk.height, &text_chunk.height);
        // dbg!(&total_height);
        scroll_view.render_widget(title.clone(), title_chunk);
        scroll_view.render_widget(text.clone(), text_chunk);
    }
    let mut state = app.anime_details_synopsys_scroll_view_state.clone();
    f.render_stateful_widget(scroll_view, chunk, &mut state);
}

fn construct_paragraph_lines(
    text: &str,
    width: u16,
    word_width: u16,
    line_height: u16,
) -> (u16, Vec<Line<'static>>) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    let mut current_width = 0;
    let mut line_count = 0;

    for word in words {
        let word_length = (word.chars().count() as u16) * word_width;

        if current_width + word_length > width {
            // Push the constructed line to Vec<Line>
            lines.push(Line::from(current_line.clone()));
            current_line.clear();
            line_count += 1;
            current_width = 0;
        }

        current_line.push(Span::raw(word.to_string() + " "));
        current_width += word_length + word_width; // Include space
    }

    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
        line_count += 1;
    }

    (line_count * line_height, lines)
}

pub fn construct_synopsis_layout<T: AsRef<str>>(
    app: &App,
    items: &[(&str, Option<T>)],
) -> (u16, Vec<(Line<'static>, List<'static>, u16)>) {
    let mut total_height = 0;
    let mut layout_items = Vec::new();

    for (title, text) in items {
        let (title, text, height) = match &text {
            Some(content) => {
                if !content.as_ref().is_empty() {
                    let (title, text, height) =
                        get_text_prop(title.to_string(), content.as_ref().to_string(), app);
                    (title, text, height)
                } else {
                    let mut title = title.to_string();
                    title.remove(title.len() - 1);
                    let (title, text, height) = get_text_prop(
                        title.to_string(),
                        format!("No {} available.", title).to_ascii_lowercase(),
                        app,
                    );
                    (title, text, height)
                }
            }
            _ => {
                let mut title = title.to_string();
                title.remove(title.len() - 1);
                let (title, text, height) = get_text_prop(
                    title.to_string(),
                    format!("No {} available.", title).to_ascii_lowercase(),
                    app,
                );
                (title, text, height)
            }
        };
        layout_items.push((title, text, height));
        total_height += height + 6; // 6 for spacing
    }
    (total_height, layout_items)
}

pub fn get_alternative_titles(
    alternative_titles: Option<AlternativeTitles>,
) -> (List<'static>, u16) {
    // todo :return hight of the list
    let first_padding = " -";
    let second_padding = "   -";
    let mut height = 0;
    let mut titles = vec![];
    if let Some(alternative_titles) = alternative_titles {
        if let Some(en) = alternative_titles.en {
            let title = Span::styled(
                format!("{}En: ", first_padding),
                Style::default().add_modifier(Modifier::BOLD),
            );
            let en = Span::styled(en, Style::default());
            titles.push(Line::from(vec![title, en]));
            height += 1;
        }
        if let Some(jp) = alternative_titles.jp {
            let title = Span::styled(
                format!("{}Jp: ", first_padding),
                Style::default().add_modifier(Modifier::BOLD),
            );
            let jp = Span::styled(jp, Style::default());
            titles.push(Line::from(vec![title, jp]));
            height += 1;
        }
        if let Some(ja) = alternative_titles.synonyms {
            let is_empty_string = ja
                .iter()
                .map(|s| s.is_empty())
                .collect::<Vec<bool>>()
                .iter()
                .all(|&s| s);

            if !ja.is_empty() && !is_empty_string {
                let title = Span::styled(
                    format!("{}Synonyms: ", first_padding),
                    Style::default().add_modifier(Modifier::BOLD),
                );

                titles.push(Line::from(title));

                height += 1;

                ja.iter().for_each(|s| {
                    titles.push(Line::from(Span::raw(format!(
                        "{}{}",
                        second_padding,
                        s.clone()
                    ))));
                    height += 1;
                });
            }
        }
    }
    (List::new(titles), height)
}

pub fn get_anime_key_val_info(
    app: &App,
) -> (Paragraph<'static>, List<'static>, Span<'static>, u16) {
    let mut key_vals_paragraph: Vec<Line> = Vec::new();
    //* alternative titles:
    let alternative_title = app
        .anime_details
        .as_ref()
        .unwrap()
        .alternative_titles
        .clone();

    let (alter_titles, alter_titles_height) = get_alternative_titles(alternative_title);
    let alter_titles_title = Span::styled(
        "Alternative Titles: ",
        Style::default().add_modifier(Modifier::BOLD),
    );

    //*  type:
    let media_type = Into::<&str>::into(
        app.anime_details
            .as_ref()
            .unwrap()
            .media_type
            .as_ref()
            .map_or(AnimeMediaType::Other("Unknown".to_string()), |m| m.clone()),
    );
    let media_type_title = Span::styled("Type: ", Style::default().add_modifier(Modifier::BOLD));
    let media_type_line = Line::from(vec![media_type_title, Span::raw(media_type)]);
    key_vals_paragraph.push(media_type_line);

    //* episodes:
    let episodes = app
        .anime_details
        .as_ref()
        .unwrap()
        .num_episodes
        .map_or("?".to_string(), |n| n.to_string());
    let ep_title = Span::styled("Episodes: ", Style::default().add_modifier(Modifier::BOLD));
    let episodes_line = Line::from(vec![ep_title, Span::raw(episodes)]);
    key_vals_paragraph.push(episodes_line);

    //* status:
    let status = Into::<&str>::into(
        app.anime_details
            .as_ref()
            .unwrap()
            .status
            .as_ref()
            .map_or(AnimeStatus::Other("None".to_string()), |s| s.clone()),
    );
    let status_title = Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD));
    let status_line = Line::from(vec![status_title, Span::raw(status)]);
    key_vals_paragraph.push(status_line);

    //* Aired:
    let start_date = app
        .anime_details
        .as_ref()
        .unwrap()
        .start_date
        .as_ref()
        .map_or("?".to_string(), |d| {
            format!("{} {}, {}", d.date.month(), d.date.day(), d.date.year())
        });

    let end_date = app
        .anime_details
        .as_ref()
        .unwrap()
        .end_date
        .as_ref()
        .map_or("?".to_string(), |d| {
            format!("{} {}, {}", d.date.month(), d.date.day(), d.date.year())
        });
    let aired = format!("{} to {}", start_date, end_date);
    let aired_title = Span::styled("Aired: ", Style::default().add_modifier(Modifier::BOLD));
    let aired_line = Line::from(vec![aired_title, Span::raw(aired)]);
    key_vals_paragraph.push(aired_line);

    //* premiered:
    let premiered = app
        .anime_details
        .as_ref()
        .unwrap()
        .start_season
        .as_ref()
        .map_or("Unknown".to_string(), |s| {
            format!("{} {}", s.season.clone(), s.year)
        });
    let premiered_title =
        Span::styled("Premiered: ", Style::default().add_modifier(Modifier::BOLD));
    let premiered_line = Line::from(vec![premiered_title, Span::raw(premiered)]);
    key_vals_paragraph.push(premiered_line);

    //* broadcast:
    let broadcast = app
        .anime_details
        .as_ref()
        .unwrap()
        .broadcast
        .as_ref()
        .map_or("Unknown".to_string(), |b| {
            format!(
                "{} {}",
                b.clone()
                    .start_time
                    .map_or("?".to_string(), |t| t.time.to_string()),
                b.day_of_the_week.clone()
            )
        });

    let broadcast_title =
        Span::styled("Broadcast: ", Style::default().add_modifier(Modifier::BOLD));
    let broadcast_line = Line::from(vec![broadcast_title, Span::raw(broadcast)]);
    key_vals_paragraph.push(broadcast_line);

    //* studios:
    let studios =
        app.anime_details
            .as_ref()
            .unwrap()
            .studios
            .as_ref()
            .map_or("Unknown".to_string(), |s| {
                s.iter()
                    .map(|s| s.name.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            });
    let studios_title = Span::styled("Studios: ", Style::default().add_modifier(Modifier::BOLD));
    let studios_line = Line::from(vec![studios_title, Span::raw(studios)]);
    key_vals_paragraph.push(studios_line);

    //* source:
    let source: &str = Into::<&str>::into(
        app.anime_details
            .as_ref()
            .unwrap()
            .source
            .as_ref()
            .map_or(&Source::Other, |s| s),
    );
    let source_title = Span::styled("Source: ", Style::default().add_modifier(Modifier::BOLD));
    let source_line = Line::from(vec![source_title, Span::raw(source)]);
    key_vals_paragraph.push(source_line);

    //* genre:
    let genres =
        app.anime_details
            .as_ref()
            .unwrap()
            .genres
            .as_ref()
            .map_or("Unknown".to_string(), |g| {
                g.iter()
                    .map(|g| g.name.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            });
    let genres_title = Span::styled("Genres: ", Style::default().add_modifier(Modifier::BOLD));
    let genres_line = Line::from(vec![genres_title, Span::raw(genres)]);
    key_vals_paragraph.push(genres_line);

    //* rating:
    let rating = app
        .anime_details
        .as_ref()
        .unwrap()
        .rating
        .as_ref()
        .map_or("Unknown".to_string(), |r| r.clone());

    let rating_title = Span::styled("Rating: ", Style::default().add_modifier(Modifier::BOLD));
    let rating_line = Line::from(vec![rating_title, Span::raw(rating)]);
    key_vals_paragraph.push(rating_line);

    let key_vals = Paragraph::new(key_vals_paragraph)
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    (
        key_vals,
        alter_titles,
        alter_titles_title,
        alter_titles_height,
    )
}

pub fn get_manga_key_val_info(
    app: &App,
) -> (Paragraph<'static>, List<'static>, Span<'static>, u16) {
    let mut key_vals_paragraph: Vec<Line> = Vec::new();
    //* alternative titles:
    let alternative_title = app
        .manga_details
        .as_ref()
        .unwrap()
        .alternative_titles
        .clone();

    let (alter_titles, alter_titles_height) = get_alternative_titles(alternative_title);
    let alter_titles_title = Span::styled(
        "Alternative Titles: ",
        Style::default().add_modifier(Modifier::BOLD),
    );

    //* type:
    let media_type = Into::<&str>::into(
        app.manga_details
            .as_ref()
            .unwrap()
            .media_type
            .as_ref()
            .map_or(MangaMediaType::Other("Unknown".to_string()), |m| m.clone()),
    );
    let media_type_title = Span::styled("Type: ", Style::default().add_modifier(Modifier::BOLD));
    let media_type_line = Line::from(vec![media_type_title, Span::raw(media_type)]);
    key_vals_paragraph.push(media_type_line);

    //* volumes:
    let volumes = app
        .manga_details
        .as_ref()
        .unwrap()
        .num_volumes
        .map_or("?".to_string(), |n| n.to_string());
    let ep_title = Span::styled("Volumes: ", Style::default().add_modifier(Modifier::BOLD));
    let volumes_line = Line::from(vec![ep_title, Span::raw(volumes)]);
    key_vals_paragraph.push(volumes_line);

    //* chapters:
    let chapters = app
        .manga_details
        .as_ref()
        .unwrap()
        .num_chapters
        .map_or("?".to_string(), |n| n.to_string());
    let ep_title = Span::styled("Chapters: ", Style::default().add_modifier(Modifier::BOLD));
    let chapters_line = Line::from(vec![ep_title, Span::raw(chapters)]);
    key_vals_paragraph.push(chapters_line);

    //* status:
    let status = Into::<&str>::into(
        app.manga_details
            .as_ref()
            .unwrap()
            .status
            .as_ref()
            .map_or(MangaStatus::Other("None".to_string()), |s| s.clone()),
    );
    let status_title = Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD));
    let status_line = Line::from(vec![status_title, Span::raw(status)]);
    key_vals_paragraph.push(status_line);

    //* Published:
    let start_date = app
        .manga_details
        .as_ref()
        .unwrap()
        .start_date
        .as_ref()
        .map_or("?".to_string(), |d| {
            format!("{} {}, {}", d.date.month(), d.date.day(), d.date.year())
        });
    let end_date = app
        .manga_details
        .as_ref()
        .unwrap()
        .end_date
        .as_ref()
        .map_or("?".to_string(), |d| {
            format!("{} {}, {}", d.date.month(), d.date.day(), d.date.year())
        });
    let published = format!("{} to {}", start_date, end_date);
    let published_title =
        Span::styled("Published: ", Style::default().add_modifier(Modifier::BOLD));
    let published_line = Line::from(vec![published_title, Span::raw(published)]);
    key_vals_paragraph.push(published_line);

    //* genres:
    let genres =
        app.manga_details
            .as_ref()
            .unwrap()
            .genres
            .as_ref()
            .map_or("Unknown".to_string(), |g| {
                g.iter()
                    .map(|g| g.name.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            });
    let genres_title = Span::styled("Genres: ", Style::default().add_modifier(Modifier::BOLD));
    let genres_line = Line::from(vec![genres_title, Span::raw(genres)]);
    key_vals_paragraph.push(genres_line);

    //* serialization:
    let serialization = app
        .manga_details
        .as_ref()
        .unwrap()
        .serialization
        .as_ref()
        .map_or("Unknown".to_string(), |s| {
            s.iter()
                .map(|s| s.node.name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        });
    let serialization_title = Span::styled(
        "Serialization: ",
        Style::default().add_modifier(Modifier::BOLD),
    );
    let serialization_line = Line::from(vec![serialization_title, Span::raw(serialization)]);
    key_vals_paragraph.push(serialization_line);

    //* authors:
    let authors =
        app.manga_details
            .as_ref()
            .unwrap()
            .authors
            .as_ref()
            .map_or("unknown".to_string(), |s| {
                s.iter()
                    .map(|s| {
                        format!(
                            "{} {} ({})",
                            s.node
                                .first_name
                                .as_ref()
                                .map_or("".to_string(), |s| s.clone()),
                            s.node
                                .last_name
                                .as_ref()
                                .map_or("".to_string(), |s| s.clone()),
                            s.role.clone()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            });

    let authors_title = Span::styled("Authors: ", Style::default().add_modifier(Modifier::BOLD));
    let authors_line = Line::from(vec![authors_title, Span::raw(authors)]);
    key_vals_paragraph.push(authors_line);

    let key_vals = Paragraph::new(key_vals_paragraph)
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    (
        key_vals,
        alter_titles,
        alter_titles_title,
        alter_titles_height,
    )
}
