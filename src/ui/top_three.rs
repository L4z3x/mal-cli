use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{
    api::model::{
        AnimeMediaType, AnimeRankingType, MangaMediaType, MangaRankingType, RankingType,
        UserReadStatus, UserWatchStatus,
    },
    app::{ActiveBlock, App, TopThreeBlock},
};

use super::{
    display_block::{get_anime_status_color, get_manga_status_color},
    format_number_with_commas,
    util::{capitalize_each_word, get_color},
};

pub fn draw_top_three(f: &mut Frame, app: &App, chunk: Rect) {
    let is_active = app.active_block == ActiveBlock::TopThree;
    let block = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Rounded)
        .title_style(get_color(is_active, app.app_config.theme))
        .title_alignment(Alignment::Left);

    match &app.active_top_three {
        TopThreeBlock::Anime(rank_type) => draw_anime_top_three(f, app, chunk, block, rank_type),
        TopThreeBlock::Manga(rank_type) => draw_manga_top_three(f, app, chunk, block, rank_type),
        TopThreeBlock::Loading(rank_type) => draw_loading_top_three(f, chunk, block, rank_type),
        TopThreeBlock::Error(_) => draw_error_top_three(f, app, chunk, block),
    };
}

fn draw_anime_top_three(
    f: &mut Frame,
    app: &App,
    chunk: Rect,
    block: Block,
    rank_type: &AnimeRankingType,
) {
    let block = block.title(format!(
        "Top {}",
        capitalize_each_word(rank_type.to_string())
    ));
    f.render_widget(block, chunk);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 3); 3])
        .split(chunk.inner(Margin::new(0, 1)));

    let list = match rank_type {
        AnimeRankingType::All => &app.top_three_anime.all,
        AnimeRankingType::Airing => &app.top_three_anime.airing,
        AnimeRankingType::Upcoming => &app.top_three_anime.upcoming,
        AnimeRankingType::TV => &app.top_three_anime.tv,
        AnimeRankingType::Movie => &app.top_three_anime.movie,
        AnimeRankingType::OVA => &app.top_three_anime.ova,
        AnimeRankingType::Special => &app.top_three_anime.special,
        AnimeRankingType::ByPopularity => &app.top_three_anime.popular,
        AnimeRankingType::Favorite => &app.top_three_anime.favourite,
        AnimeRankingType::Other(_) => {
            println!("no anime ranking type specifyed ui/top_three.rs:59");
            &app.top_three_anime.airing
        }
    };
    if list.is_some() {
        for (i, (chunk, data)) in chunks.iter().zip(list.as_ref().unwrap().iter()).enumerate() {
            let is_active: bool =
                app.active_block == ActiveBlock::TopThree && app.selected_top_three == i as u32;

            let block = Block::default()
                .title((i + 1).to_string())
                .title_style(
                    get_color(is_active, app.app_config.theme).add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(get_color(is_active, app.app_config.theme));

            let anime_title = data.get_title(&app.app_config, false)[0].clone();

            let score = data
                .mean
                .map_or("N/A".to_string(), |m| m.to_string())
                .clone();

            let media_type: &str = Into::<&str>::into(
                data.media_type
                    .as_ref()
                    .map_or(AnimeMediaType::Other("Unknown".to_string()), |media_type| {
                        media_type.clone()
                    }),
            );

            let year = data
                .start_date
                .clone()
                .map_or("".to_string(), |date| date.date.year().to_string().clone());

            let ep_num = data.num_episodes.map_or("?".to_string(), |n| n.to_string());

            let number_users = data
                .num_list_users
                .map_or("?".to_string(), |n| format_number_with_commas(n));

            // let anime_status: &str = data
            //     .status
            //     .as_ref()
            //     .map_or(AnimeStatus::Other("Unknown".to_string()), |s| s.clone())
            //     .into();

            let user_anime_status = data
                .my_list_status
                .as_ref()
                .map_or(UserWatchStatus::Other("None".to_string()), |status| {
                    status.status.clone()
                });

            let user_anime_status_color = get_anime_status_color(&user_anime_status, app);

            let user_anime_status: &str = user_anime_status.into();
            // constracting lines and spans

            let title = Line::from(vec![
                Span::raw(anime_title),
                Span::raw(" "),
                Span::styled(
                    user_anime_status,
                    Style::default().fg(user_anime_status_color),
                ),
            ])
            .style(get_color(is_active, app.app_config.theme))
            .alignment(Alignment::Left);

            let status = Line::from(vec![
                Span::raw(capitalize_each_word(media_type.to_string())),
                Span::raw(" "),
                Span::raw(format!("{} eps", ep_num)),
                Span::raw(" "),
                // Span::raw(capitalize_each_word(anime_status.to_string())),
            ]);

            let score = Line::from(vec![Span::raw("Scored "), Span::raw(score)]);

            let number = Line::from(vec![Span::raw(number_users), Span::raw(" members")]);

            let year = Line::from(Span::raw(year));

            let card = Paragraph::new(vec![title, status, score, number, year])
                .alignment(Alignment::Left)
                .wrap(ratatui::widgets::Wrap { trim: false })
                .block(block);
            f.render_widget(card, *chunk);

            //TODO: MAKE SURE THERE S DATA
        }
    } else {
        // draw error in the top three layout
    }
}

fn draw_manga_top_three(
    f: &mut Frame,
    app: &App,
    chunk: Rect,
    block: Block,
    rank_type: &MangaRankingType,
) {
    // let block = block.title(format!("Top {}", rank_type));
    // f.render_widget(block, chunk);

    let block = block.title(format!(
        "Top {}",
        capitalize_each_word(rank_type.to_string())
    ));
    f.render_widget(block, chunk);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 3); 3])
        .split(chunk.inner(Margin::new(0, 1)));

    let list = match rank_type {
        MangaRankingType::All => &app.top_three_manga.all,
        MangaRankingType::Manga => &app.top_three_manga.manga,
        MangaRankingType::Novels => &app.top_three_manga.novels,
        MangaRankingType::OneShots => &app.top_three_manga.oneshots,
        MangaRankingType::Doujinshi => &app.top_three_manga.doujin,
        MangaRankingType::Manhwa => &app.top_three_manga.manhwa,
        MangaRankingType::Manhua => &app.top_three_manga.manhua,
        MangaRankingType::ByPopularity => &app.top_three_manga.popular,
        MangaRankingType::Other(_) => {
            println!("No manga ranking type specified in ui/top_three.rs");
            &app.top_three_manga.manga
        }
    };

    if let Some(list) = list {
        for (i, (chunk, data)) in chunks.iter().zip(list.iter()).enumerate() {
            let is_active =
                app.active_block == ActiveBlock::TopThree && app.selected_top_three == i as u32;

            let block = Block::default()
                .title((i + 1).to_string())
                .title_style(
                    get_color(is_active, app.app_config.theme).add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(get_color(is_active, app.app_config.theme));

            let manga_title = data.get_title(&app.app_config, false)[0].clone();
            let score = data.mean.map_or("N/A".to_string(), |m| m.to_string());
            let media_type: &str = Into::<&str>::into(
                data.media_type
                    .clone()
                    .unwrap_or(MangaMediaType::Other("Unknown".to_string())),
            );
            let year = data
                .start_date
                .clone()
                .map_or("".to_string(), |date| date.date.year().to_string());
            let chapter_num = data.num_chapters.map_or("?".to_string(), |n| n.to_string());
            let number_users = data
                .num_list_users
                .map_or("?".to_string(), |n| format_number_with_commas(n));
            let user_manga_status = data
                .my_list_status
                .as_ref()
                .map_or(UserReadStatus::Other("None".to_string()), |status| {
                    status.status.clone()
                });
            let user_manga_status_color = get_manga_status_color(&user_manga_status, app);
            let user_manga_status: &str = user_manga_status.into();

            let title = Line::from(vec![
                Span::raw(manga_title),
                Span::raw(" "),
                Span::styled(
                    user_manga_status,
                    Style::default().fg(user_manga_status_color),
                ),
            ])
            .style(get_color(is_active, app.app_config.theme))
            .alignment(Alignment::Left);

            let status = Line::from(vec![
                Span::raw(capitalize_each_word(media_type.to_string())),
                Span::raw(" "),
                Span::raw(format!("{} chapters", chapter_num)),
                Span::raw(" "),
            ]);

            let score = Line::from(vec![Span::raw("Scored "), Span::raw(score)]);
            let number = Line::from(vec![Span::raw(number_users), Span::raw(" members")]);
            let year = Line::from(Span::raw(year));

            let card = Paragraph::new(vec![title, status, score, number, year])
                .alignment(Alignment::Left)
                .wrap(ratatui::widgets::Wrap { trim: false })
                .block(block);
            f.render_widget(card, *chunk);
        }
    } else {
        // Draw error in the top three layout
    }
}

fn draw_loading_top_three(f: &mut Frame, chunk: Rect, block: Block, rank_type: &RankingType) {
    let title = match rank_type {
        RankingType::AnimeRankingType(rank) => capitalize_each_word(rank.to_string()),
        RankingType::MangaRankingType(rank) => capitalize_each_word(rank.to_string()),
    };

    let block = block.title(format!("Top {}", title));
    f.render_widget(block, chunk);

    let middle_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 5); 5])
        .split(chunk.inner(Margin::new(0, 1)))[2];

    let loading = Paragraph::new(Line::from(Span::raw("Loading...")).alignment(Alignment::Center));
    f.render_widget(loading, middle_chunk);
}

fn draw_error_top_three(f: &mut Frame, app: &App, chunk: Rect, block: Block) {
    let block = block.title("Top Three List");
    f.render_widget(block, chunk);

    let middle_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 5); 5])
        .split(chunk.inner(Margin::new(0, 1)))[2];

    let error = Paragraph::new(
        Line::from(Span::raw(format!("Error: {}", app.api_error)))
            .alignment(Alignment::Center)
            .centered(),
    )
    .wrap(Wrap { trim: true });
    f.render_widget(error, middle_chunk);
}
