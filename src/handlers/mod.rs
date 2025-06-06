mod anime;
pub mod common;
mod display_block;
mod help;
mod input;
mod option;
mod user;
use crate::api::model::{AnimeRankingType, MangaRankingType, Media};
use crate::app::{
    ActiveBlock, ActiveDisplayBlock, App, Data, SelectedSearchTab, TopThreeBlock,
    ANIME_OPTIONS_RANGE, GENERAL_OPTIONS_RANGE, USER_OPTIONS_RANGE,
};
use crate::event::Key;
use crate::network::IoEvent;

use common::get_lowercase_key;
pub use input::handler as input_handler;
use log::warn;

pub fn handle_app(key: Key, app: &mut App) {
    // First handle any global event and then move to block event
    if app.exit_confirmation_popup {
        if key == Key::Esc || key == Key::Char('n') {
            app.exit_confirmation_popup = false;
            return;
        } else if key == Key::Enter || get_lowercase_key(key) == Key::Char('y') {
            app.exit_flag = true;
            return;
        }
    }
    match key {
        Key::Esc => app.load_previous_route(),

        _ if key == app.app_config.keys.next_state => app.load_next_route(),

        _ if key == app.app_config.keys.help => {
            app.active_display_block = ActiveDisplayBlock::Help;
        }

        _ if key == app.app_config.keys.search => {
            app.input = vec![];
            app.input_idx = 0;
            app.input_cursor_position = 0;
            app.active_block = ActiveBlock::Input;
        }

        _ => handle_block_events(key, app),
    }
}

// Handler event for the current active block
fn handle_block_events(key: Key, app: &mut App) {
    let current_block = app.active_block;
    match current_block {
        ActiveBlock::Input => input::handler(key, app),

        ActiveBlock::Anime => anime::handler(key, app),

        ActiveBlock::User => user::handler(key, app),

        ActiveBlock::Option => option::handler(key, app),

        ActiveBlock::Error => {}

        ActiveBlock::TopThree => display_block::top_three::handler(key, app),

        ActiveBlock::DisplayBlock => display_block::handle_display_block(key, app),
    }
}

pub fn handle_tab(app: &mut App) {
    match app.active_block {
        ActiveBlock::Input => {
            // todo: anything else to handle ? like when exiting the input state.
            app.library.selected_index = ANIME_OPTIONS_RANGE.start;
            app.active_block = ActiveBlock::Anime;
        }

        ActiveBlock::Anime => {
            app.library.selected_index = USER_OPTIONS_RANGE.start;
            app.active_block = ActiveBlock::User;
        }

        ActiveBlock::User => {
            app.library.selected_index = GENERAL_OPTIONS_RANGE.start;
            app.active_block = ActiveBlock::Option;
        }

        ActiveBlock::Option => {
            app.library.selected_index = 10; // out of range to not display anything
            app.active_block = ActiveBlock::TopThree;
        }

        ActiveBlock::TopThree => {
            app.active_block = ActiveBlock::DisplayBlock;
        }

        ActiveBlock::DisplayBlock => {
            if !app.popup {
                app.active_block = ActiveBlock::Input;
            }
        }
        _ => {}
    }
}

pub fn handle_back_tab(app: &mut App) {
    match app.active_block {
        ActiveBlock::Input => {
            app.library.selected_index = 10; // out of range to not display anything
            app.active_block = ActiveBlock::DisplayBlock;
        }

        ActiveBlock::DisplayBlock => {
            if !app.popup {
                app.active_block = ActiveBlock::TopThree;
            }
        }

        ActiveBlock::TopThree => {
            app.library.selected_index = GENERAL_OPTIONS_RANGE.end;
            app.active_block = ActiveBlock::Option;
        }

        ActiveBlock::Option => {
            app.library.selected_index = USER_OPTIONS_RANGE.end;
            app.active_block = ActiveBlock::User;
        }

        ActiveBlock::User => {
            app.library.selected_index = ANIME_OPTIONS_RANGE.end;
            app.active_block = ActiveBlock::Anime;
        }

        ActiveBlock::Anime => {
            app.library.selected_index = 10; // out of range to not display anything
            app.active_block = ActiveBlock::Input;
        }
        _ => {}
    }
}

pub fn is_data_available(
    app: &App,
    data: &Data,
    block: ActiveDisplayBlock,
) -> (bool, Option<usize>) {
    for (i, route) in app.navigator.data.iter().enumerate() {
        if route.1.block == block
            && route.1.data.is_some()
            && std::mem::discriminant(data)
                == std::mem::discriminant(route.1.data.as_ref().unwrap())
        {
            return (true, Some(i));
        }
    }
    (false, None)
}

pub fn get_media_detail_page(app: &mut App) {
    let index = app.search_results.selected_display_card_index.unwrap_or(0)
        + app.start_card_list_index as usize;

    match app.active_block {
        ActiveBlock::DisplayBlock => {
            match app.active_display_block {
                ActiveDisplayBlock::AnimeRanking => {
                    let data = app.anime_ranking_data.as_ref().unwrap().data.get(index);

                    if let Some(data) = data {
                        let (is_data_available, is_next, index) =
                            is_media_data_available(app, &Media::Anime(&data.node));
                        if is_next {
                            app.load_next_route();
                            return;
                        }
                        if is_data_available {
                            app.load_route(index.unwrap());
                        } else {
                            app.active_display_block = ActiveDisplayBlock::Loading;
                            app.dispatch(IoEvent::GetAnime(data.node.id));
                        }
                        app.active_block = ActiveBlock::DisplayBlock;
                    }
                }

                ActiveDisplayBlock::MangaRanking => {
                    let data = app.manga_ranking_data.as_ref().unwrap().data.get(index);

                    if let Some(data) = data {
                        let (is_data_available, is_next, index) =
                            is_media_data_available(app, &Media::Manga(&data.node));
                        if is_next {
                            app.load_next_route();
                            return;
                        }
                        if is_data_available {
                            app.load_route(index.unwrap());
                        } else {
                            app.active_display_block = ActiveDisplayBlock::Loading;
                            app.dispatch(IoEvent::GetManga(data.node.id));
                        }
                        app.active_block = ActiveBlock::DisplayBlock;
                    }
                }

                ActiveDisplayBlock::SearchResultBlock => match app.search_results.selected_tab {
                    SelectedSearchTab::Anime => {
                        let data = app.search_results.anime.as_ref().unwrap().data.get(index);

                        if let Some(data) = data {
                            let (is_data_available, is_next, index) =
                                is_media_data_available(app, &Media::Anime(&data.node));
                            if is_next {
                                app.load_next_route();
                                return;
                            }
                            if is_data_available {
                                app.load_route(index.unwrap());
                            } else {
                                app.active_display_block = ActiveDisplayBlock::Loading;
                                app.dispatch(IoEvent::GetAnime(data.node.id));
                            }
                            app.active_block = ActiveBlock::DisplayBlock;
                        }
                    }
                    SelectedSearchTab::Manga => {
                        let data = app.search_results.manga.as_ref().unwrap().data.get(index);

                        if let Some(data) = data {
                            let (is_data_available, is_next, index) =
                                is_media_data_available(app, &Media::Manga(&data.node));
                            if is_next {
                                app.load_next_route();
                                return;
                            }
                            if is_data_available {
                                app.load_route(index.unwrap());
                            } else {
                                app.active_display_block = ActiveDisplayBlock::Loading;
                                app.dispatch(IoEvent::GetManga(data.node.id));
                            }
                            app.active_block = ActiveBlock::DisplayBlock;
                        }
                    }
                },

                ActiveDisplayBlock::Suggestions => {
                    let data = app.search_results.anime.as_ref().unwrap().data.get(index);

                    if let Some(data) = data {
                        let (is_data_available, is_next, index) =
                            is_media_data_available(app, &Media::Anime(&data.node));
                        if is_next {
                            app.load_next_route();
                            return;
                        }
                        if is_data_available {
                            app.load_route(index.unwrap());
                        } else {
                            app.active_display_block = ActiveDisplayBlock::Loading;
                            app.dispatch(IoEvent::GetAnime(data.node.id));
                        }
                        app.active_block = ActiveBlock::DisplayBlock;
                    }
                }

                ActiveDisplayBlock::UserAnimeList => {
                    let data = app.search_results.anime.as_ref().unwrap().data.get(index);

                    if let Some(data) = data {
                        let (is_data_available, is_next, index) =
                            is_media_data_available(app, &Media::Anime(&data.node));
                        if is_next {
                            app.load_next_route();
                            return;
                        }
                        if is_data_available {
                            app.load_route(index.unwrap());
                        } else {
                            app.active_display_block = ActiveDisplayBlock::Loading;
                            app.dispatch(IoEvent::GetAnime(data.node.id));
                        }
                        app.active_block = ActiveBlock::DisplayBlock;
                    }
                }

                ActiveDisplayBlock::UserMangaList => {
                    let data = app.search_results.manga.as_ref().unwrap().data.get(index);

                    if let Some(data) = data {
                        let (is_data_available, is_next, index) =
                            is_media_data_available(app, &Media::Manga(&data.node));
                        if is_next {
                            app.load_next_route();
                            return;
                        }
                        if is_data_available {
                            app.load_route(index.unwrap());
                        } else {
                            app.active_display_block = ActiveDisplayBlock::Loading;
                            app.dispatch(IoEvent::GetManga(data.node.id));
                        }
                        app.active_block = ActiveBlock::DisplayBlock;
                    }
                }

                ActiveDisplayBlock::Seasonal => {
                    let data = app.search_results.anime.as_ref().unwrap().data.get(index);

                    if let Some(data) = data {
                        let (is_data_available, is_next, index) =
                            is_media_data_available(app, &Media::Anime(&data.node));
                        if is_next {
                            app.load_next_route();
                            return;
                        }
                        if is_data_available {
                            app.load_route(index.unwrap());
                        } else {
                            app.active_display_block = ActiveDisplayBlock::Loading;
                            app.dispatch(IoEvent::GetAnime(data.node.id));
                        }
                        app.active_block = ActiveBlock::DisplayBlock;
                    }
                }

                _ => {}
            };
        }
        ActiveBlock::TopThree => match &app.active_top_three {
            TopThreeBlock::Anime(anime_ranking_type) => {
                let index = app.selected_top_three as usize % 3;
                let mut anime = None;
                match anime_ranking_type {
                    AnimeRankingType::Airing => {
                        if let Some(data) = &app.top_three_anime.airing {
                            anime = Some(&data[index]);
                        }
                    }
                    AnimeRankingType::All => {
                        if let Some(data) = &app.top_three_anime.all {
                            anime = Some(&data[index]);
                        }
                    }

                    AnimeRankingType::Upcoming => {
                        if let Some(data) = &app.top_three_anime.upcoming {
                            anime = Some(&data[index]);
                        }
                    }
                    AnimeRankingType::Favorite => {
                        if let Some(data) = &app.top_three_anime.favourite {
                            anime = Some(&data[index]);
                        }
                    }
                    AnimeRankingType::Movie => {
                        if let Some(data) = &app.top_three_anime.movie {
                            anime = Some(&data[index]);
                        }
                    }

                    AnimeRankingType::OVA => {
                        if let Some(data) = &app.top_three_anime.ova {
                            anime = Some(&data[index]);
                        }
                    }
                    AnimeRankingType::TV => {
                        if let Some(data) = &app.top_three_anime.tv {
                            anime = Some(&data[index]);
                        }
                    }
                    AnimeRankingType::ByPopularity => {
                        if let Some(data) = &app.top_three_anime.popular {
                            anime = Some(&data[index]);
                        }
                    }
                    AnimeRankingType::Special => {
                        if let Some(data) = &app.top_three_anime.special {
                            anime = Some(&data[index]);
                        }
                    }
                    AnimeRankingType::Other(_) => {
                        warn!("other anime ranking type was specified")
                    }
                }
                let anime = match anime {
                    Some(data) => data,
                    None => {
                        // push error
                        app.api_error = "Error: Not Found".to_string();
                        app.active_display_block = ActiveDisplayBlock::Error;
                        return;
                    }
                };
                let (is_data_available, is_next, index) =
                    is_media_data_available(app, &Media::Anime(anime));
                if is_next {
                    app.load_next_route();
                    return;
                }
                if is_data_available {
                    app.load_route(index.unwrap());
                } else {
                    app.active_display_block = ActiveDisplayBlock::Loading;
                    app.dispatch(IoEvent::GetAnime(anime.id));
                }
            }
            TopThreeBlock::Manga(manga_ranking_type) => {
                let index = app.selected_top_three as usize % 3;
                let mut manga = None;
                match manga_ranking_type {
                    MangaRankingType::All => {
                        if let Some(data) = &app.top_three_manga.all {
                            manga = Some(&data[index]);
                        }
                    }
                    MangaRankingType::Manga => {
                        if let Some(data) = &app.top_three_manga.manga {
                            manga = Some(&data[index]);
                        }
                    }
                    MangaRankingType::OneShots => {
                        if let Some(data) = &app.top_three_manga.oneshots {
                            manga = Some(&data[index]);
                        }
                    }
                    MangaRankingType::ByPopularity => {
                        if let Some(data) = &app.top_three_manga.popular {
                            manga = Some(&data[index]);
                        }
                    }
                    MangaRankingType::Favorite => {
                        if let Some(data) = &app.top_three_manga.favourite {
                            manga = Some(&data[index]);
                        }
                    }
                    MangaRankingType::Manhua => {
                        if let Some(data) = &app.top_three_manga.manhua {
                            manga = Some(&data[index]);
                        }
                    }
                    MangaRankingType::Manhwa => {
                        if let Some(data) = &app.top_three_manga.manhwa {
                            manga = Some(&data[index]);
                        }
                    }
                    MangaRankingType::Novels => {
                        if let Some(data) = &app.top_three_manga.novels {
                            manga = Some(&data[index]);
                        }
                    }
                    MangaRankingType::Doujinshi => {
                        if let Some(data) = &app.top_three_manga.doujin {
                            manga = Some(&data[index]);
                        }
                    }
                    MangaRankingType::Other(_) => {
                        warn!("other manga ranking type was specified")
                    } // PUSH ERROR
                }

                let manga = match manga {
                    Some(data) => data,
                    None => {
                        // push error
                        app.api_error = "Error: Not Found".to_string();
                        app.active_display_block = ActiveDisplayBlock::Error;
                        return;
                    }
                };

                let (is_data_available, is_next, index) =
                    is_media_data_available(app, &Media::Manga(manga));
                if is_next {
                    app.load_next_route();
                    return;
                }
                if is_data_available {
                    app.load_route(index.unwrap());
                } else {
                    app.active_display_block = ActiveDisplayBlock::Loading;
                    app.dispatch(IoEvent::GetManga(manga.id));
                }
            }
            _ => {}
        },
        _ => {}
    }
}

fn is_media_data_available(app: &App, data: &Media) -> (bool, bool, Option<u16>) {
    match data {
        Media::Anime(data) => {
            for i in 0..(app.navigator.history.len()) {
                let page_id = app.navigator.history[i];
                if app.navigator.data[&page_id].block == ActiveDisplayBlock::AnimeDetails
                    && app.navigator.data[&page_id].data.is_some()
                {
                    if let Data::Anime(d) = app.navigator.data[&page_id].data.as_ref().unwrap() {
                        if d.id == data.id {
                            let is_next = app.navigator.index + 1 == i;
                            return (true, is_next, Some(page_id));
                        }
                    }
                }
            }
        }
        Media::Manga(data) => {
            for i in 0..(app.navigator.history.len()) {
                let id = app.navigator.history[i];
                if app.navigator.data[&id].block == ActiveDisplayBlock::MangaDetails
                    && app.navigator.data[&id].data.is_some()
                {
                    if let Data::Manga(d) = app.navigator.data[&id].data.as_ref().unwrap() {
                        if d.id == data.id {
                            let is_next = app.navigator.index + 1 == i;
                            return (true, is_next, Some(id));
                        }
                    }
                }
            }
        }
    }
    (false, false, None)
}
