use super::common;
use crate::app::{ActiveBlock, ActiveDisplayBlock, App, Data, ANIME_OPTIONS, ANIME_OPTIONS_RANGE};

use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
    match key {
        // k if common::right_event(k) => common::handle_right_event(app),
        k if common::down_event(k) => {
            // calculate the next index in the list
            let next_index = ANIME_OPTIONS_RANGE.start
                + common::on_down_press(
                    &ANIME_OPTIONS,
                    Some(app.library.selected_index % (ANIME_OPTIONS.len())),
                );
            app.library.selected_index = next_index;
        }
        k if common::up_event(k) => {
            // calculate the next index in the list
            let next_index = ANIME_OPTIONS_RANGE.start
                + common::on_up_press(
                    &ANIME_OPTIONS,
                    Some(app.library.selected_index % (ANIME_OPTIONS.len())),
                );
            app.library.selected_index = next_index;
        }

        //? idk what this means ??
        // k if common::high_event(k) => {
        //     let next_index = common::on_high_press();
        //     app.library.selected_index = next_index;
        // }
        // k if common::middle_event(k) => {
        //     let next_index = common::on_middle_press(&ANIME_OPTIONS);
        //     app.library.selected_index = next_index;
        // }
        // k if common::low_event(k) => {
        //     let next_index = common::on_low_press(&ANIME_OPTIONS);
        //     app.library.selected_index = next_index
        // }
        // `library` should probably be an array of structs with enums rather than just using indexes
        // like this
        Key::Enter => match app.library.selected_index {
            // Seasonal
            0 => get_seasonal(app),
            // Ranking
            1 => get_anime_ranking(app),
            // Suggested
            2 => {}
            // This is required because Rust can't tell if this pattern in exhaustive
            _ => {} //# search is not neaded in the list.
                    // // Search
                    // 3 => {}
        },
        _ => (),
    };
}

fn get_seasonal(app: &mut App) {
    let is_data_availabe = is_seasonal_data_available(app);
    let is_current_route = app
        .get_current_route()
        .map_or(false, |r| r.block == ActiveDisplayBlock::Seasonal);

    if is_current_route {
        return;
    }

    if is_data_availabe.0 {
        app.search_results = match app.navigation_stack[is_data_availabe.1.unwrap()]
            .data
            .as_ref()
            .unwrap()
        {
            Data::SearchResult(d) => d.clone(),
            _ => return,
        };
        app.display_block_title = app.navigation_stack[is_data_availabe.1.unwrap()]
            .title
            .clone();
        app.active_display_block = ActiveDisplayBlock::Seasonal;
        app.active_block = ActiveBlock::DisplayBlock;
        app.navigation_index = is_data_availabe.1.unwrap() as u32;
    } else {
        app.active_display_block = ActiveDisplayBlock::Loading;

        app.dispatch(IoEvent::GetSeasonalAnime);
    }
}

fn is_seasonal_data_available(app: &mut App) -> (bool, Option<usize>) {
    for i in 0..(app.navigation_stack.len() - 1) {
        if app.navigation_stack[i].block == ActiveDisplayBlock::Seasonal
            && app.navigation_stack[i].data.is_some()
        {
            return (true, Some(i));
        }
    }
    return (false, None);
}

pub fn get_anime_ranking(app: &mut App) {
    let is_data_available = is_anime_ranking_data_available(app);

    let is_current_route = app
        .get_current_route()
        .map_or(false, |r| r.block == ActiveDisplayBlock::AnimeRanking);

    if is_current_route {
        return;
    }

    if is_data_available.0 {
        app.anime_ranking_data = match app.navigation_stack[is_data_available.1.unwrap()]
            .data
            .as_ref()
            .unwrap()
        {
            Data::AnimeRanking(d) => Some(d.clone()),
            _ => return,
        };

        app.display_block_title = app.navigation_stack[is_data_available.1.unwrap()]
            .title
            .clone();
        app.active_display_block = ActiveDisplayBlock::AnimeRanking;
        app.active_block = ActiveBlock::DisplayBlock;
        app.navigation_index = is_data_available.1.unwrap() as u32;
    } else {
        app.active_display_block = ActiveDisplayBlock::Loading;

        app.dispatch(IoEvent::GetAnimeRanking(app.anime_ranking_type.clone()));
    }
}

pub fn get_manga_ranking(app: &mut App) {
    let is_data_available = is_manga_ranking_data_available(app);

    let is_current_route = app
        .get_current_route()
        .map_or(false, |r| r.block == ActiveDisplayBlock::MangaRanking);
    if is_current_route {
        return;
    }

    if is_data_available.0 {
        app.manga_ranking_data = match app.navigation_stack[is_data_available.1.unwrap()]
            .data
            .as_ref()
            .unwrap()
        {
            Data::MangaRanking(d) => Some(d.clone()),
            _ => return,
        };

        app.display_block_title = app.navigation_stack[is_data_available.1.unwrap()]
            .title
            .clone();
        app.active_display_block = ActiveDisplayBlock::MangaRanking;
        app.active_block = ActiveBlock::DisplayBlock;
        app.navigation_index = is_data_available.1.unwrap() as u32;
    } else {
        app.active_display_block = ActiveDisplayBlock::Loading;

        app.dispatch(IoEvent::GetMangaRanking(app.manga_ranking_type.clone()));
    }
}

fn is_anime_ranking_data_available(app: &mut App) -> (bool, Option<usize>) {
    for i in 0..(app.navigation_stack.len()) {
        if app.navigation_stack[i].block == ActiveDisplayBlock::AnimeRanking
            && app.navigation_stack[i].data.is_some()
        {
            if let Data::AnimeRanking(_) = app.navigation_stack[i].data.as_ref().unwrap() {
                return (true, Some(i));
            }
        }
    }
    return (false, None);
}

fn is_manga_ranking_data_available(app: &mut App) -> (bool, Option<usize>) {
    for i in 0..(app.navigation_stack.len()) {
        if app.navigation_stack[i].block == ActiveDisplayBlock::MangaRanking
            && app.navigation_stack[i].data.is_some()
        {
            if let Data::MangaRanking(_) = app.navigation_stack[i].data.as_ref().unwrap() {
                return (true, Some(i));
            }
        }
    }
    return (false, None);
}
