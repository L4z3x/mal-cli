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

pub fn handle_app(key: Key, app: &mut App) {
    // First handle any global event and then move to block event
    if app.exit_confirmation_popup {
        if key == Key::Esc || get_lowercase_key(key) == Key::Char('n') {
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

    // help::handler(key, app);
} // todo: move this to active_display_block_handler
  // ActiveBlock::BasicView => {}

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
            // todo: handle cases when exiting the Display_block.
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
        if route.1.block == block && route.1.data.is_some() {
            if std::mem::discriminant(data)
                == std::mem::discriminant(route.1.data.as_ref().unwrap())
            {
                return (true, Some(i));
            }
        }
    }
    return (false, None);
}

pub fn get_media_detail_page(app: &mut App) {
    let index = app.search_results.selected_display_card_index.unwrap_or(0);
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
                let index = app.selected_top_three as usize;
                let mut anime = None;
                match anime_ranking_type {
                    AnimeRankingType::Airing => match &app.top_three_anime.airing {
                        Some(data) => {
                            anime = Some(&data[index]);
                        }
                        None => {}
                    },
                    AnimeRankingType::All => match &app.top_three_anime.all {
                        Some(data) => {
                            anime = Some(&data[index]);
                        }
                        None => {}
                    },

                    AnimeRankingType::Upcoming => match &app.top_three_anime.upcoming {
                        Some(data) => {
                            anime = Some(&data[index]);
                        }
                        None => {}
                    },
                    AnimeRankingType::Favorite => match &app.top_three_anime.favourite {
                        Some(data) => {
                            anime = Some(&data[index]);
                        }
                        None => {}
                    },
                    AnimeRankingType::Movie => match &app.top_three_anime.movie {
                        Some(data) => {
                            anime = Some(&data[index]);
                        }
                        None => {}
                    },

                    AnimeRankingType::OVA => match &app.top_three_anime.ova {
                        Some(data) => {
                            anime = Some(&data[index]);
                        }
                        None => {}
                    },
                    AnimeRankingType::TV => match &app.top_three_anime.tv {
                        Some(data) => {
                            anime = Some(&data[index]);
                        }
                        None => {}
                    },
                    AnimeRankingType::ByPopularity => match &app.top_three_anime.popular {
                        Some(data) => {
                            anime = Some(&data[index]);
                        }
                        None => {}
                    },
                    AnimeRankingType::Special => match &app.top_three_anime.special {
                        Some(data) => {
                            anime = Some(&data[index]);
                        }
                        None => {}
                    },
                    AnimeRankingType::Other(_) => {} // PUSH ERROR
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
                let index = app.selected_top_three as usize;
                let mut manga = None;
                match manga_ranking_type {
                    MangaRankingType::All => match &app.top_three_manga.all {
                        Some(data) => {
                            manga = Some(&data[index]);
                        }
                        None => {}
                    },
                    MangaRankingType::Manga => match &app.top_three_manga.manga {
                        Some(data) => {
                            manga = Some(&data[index]);
                        }
                        None => {}
                    },
                    MangaRankingType::OneShots => match &app.top_three_manga.oneshots {
                        Some(data) => {
                            manga = Some(&data[index]);
                        }
                        None => {}
                    },
                    MangaRankingType::ByPopularity => match &app.top_three_manga.popular {
                        Some(data) => {
                            manga = Some(&data[index]);
                        }
                        None => {}
                    },
                    MangaRankingType::Favorite => match &app.top_three_manga.favourite {
                        Some(data) => {
                            manga = Some(&data[index]);
                        }
                        None => {}
                    },
                    MangaRankingType::Manhua => match &app.top_three_manga.manhua {
                        Some(data) => {
                            manga = Some(&data[index]);
                        }
                        None => {}
                    },
                    MangaRankingType::Manhwa => match &app.top_three_manga.manhwa {
                        Some(data) => {
                            manga = Some(&data[index]);
                        }
                        None => {}
                    },
                    MangaRankingType::Novels => match &app.top_three_manga.novels {
                        Some(data) => {
                            manga = Some(&data[index]);
                        }
                        None => {}
                    },
                    MangaRankingType::Doujinshi => match &app.top_three_manga.doujin {
                        Some(data) => {
                            manga = Some(&data[index]);
                        }
                        None => {}
                    },
                    MangaRankingType::Other(_) => {} // PUSH ERROR
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
    return (false, false, None);
}
