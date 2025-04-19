use crate::handlers::{common, get_media_detail_page};
use crate::{
    api::model::{AnimeRankingType, MangaRankingType, RankingType},
    app::{App, TopThreeBlock},
    event::Key,
    network::IoEvent,
};

pub fn handler(key: Key, app: &mut App) {
    let mut index = app.selected_top_three;
    match key {
        k if common::up_event(k) => {
            if index > 0 {
                index = index - 1;
            } else {
                index = 2;
            }
        }

        k if common::down_event(k) => {
            if index < 2 {
                index = index + 1;
            } else {
                index = 0;
            }
        }

        k if k == app.app_config.keys.toggle => match &app.active_top_three {
            TopThreeBlock::Anime(_) => {
                let data_available = is_manga_data_available(
                    app,
                    &app.active_top_three_manga
                        .as_ref()
                        .unwrap_or(&app.available_manga_ranking_types[0]),
                );

                if !data_available {
                    app.active_top_three = TopThreeBlock::Loading(RankingType::MangaRankingType(
                        app.active_top_three_manga
                            .clone()
                            .unwrap_or(app.available_manga_ranking_types[0].clone()),
                    ));

                    app.dispatch(IoEvent::GetTopThree(TopThreeBlock::Manga(
                        app.active_top_three_manga
                            .clone()
                            .unwrap_or(app.available_manga_ranking_types[0].clone()),
                    )));
                } else {
                    app.active_top_three = TopThreeBlock::Manga(
                        app.active_top_three_manga
                            .as_ref()
                            .unwrap_or(&app.available_manga_ranking_types[0])
                            .clone(),
                    )
                }
            }

            TopThreeBlock::Manga(_) => {
                let data_available = is_anime_data_available(
                    app,
                    app.active_top_three_anime
                        .as_ref()
                        .unwrap_or(&app.available_anime_ranking_types[0]),
                );

                if !data_available {
                    app.active_top_three = TopThreeBlock::Loading(RankingType::AnimeRankingType(
                        app.active_top_three_anime
                            .clone()
                            .unwrap_or(app.available_anime_ranking_types[0].clone()),
                    ));

                    app.dispatch(IoEvent::GetTopThree(TopThreeBlock::Anime(
                        app.active_top_three_anime
                            .clone()
                            .unwrap_or(AnimeRankingType::Airing),
                    )));
                } else {
                    app.active_top_three = TopThreeBlock::Anime(
                        app.active_top_three_anime
                            .as_ref()
                            .unwrap_or(&app.available_anime_ranking_types[0])
                            .clone(),
                    );
                }
            }

            // reload the current block
            TopThreeBlock::Error(previous) => match previous {
                RankingType::AnimeRankingType(_) => {
                    app.dispatch(IoEvent::GetTopThree(TopThreeBlock::Manga(
                        app.active_top_three_manga
                            .as_ref()
                            .unwrap_or(&app.available_manga_ranking_types[0])
                            .clone(),
                    )))
                }
                RankingType::MangaRankingType(_) => {
                    app.dispatch(IoEvent::GetTopThree(TopThreeBlock::Anime(
                        app.active_top_three_anime
                            .as_ref()
                            .unwrap_or(&app.available_anime_ranking_types[0])
                            .clone(),
                    )))
                }
            },
            _ => {}
        },
        // switch between ranking types
        k if common::left_event(k) => match &app.active_top_three {
            TopThreeBlock::Anime(_) => {
                let mut index = app.active_anime_rank_index;
                let max = app.available_anime_ranking_types.len() as u32;
                decrement_index(&mut index, max);
                change_anime_top_three_block(app, index);
            }

            TopThreeBlock::Manga(_) => {
                let mut index = app.active_manga_rank_index;
                let max = app.available_manga_ranking_types.len() as u32;

                decrement_index(&mut index, max);
                change_manga_top_three_block(app, index);
            }
            TopThreeBlock::Error(previous) => match previous {
                RankingType::AnimeRankingType(_) => {
                    let mut index = app.active_anime_rank_index;
                    let max = app.available_anime_ranking_types.len() as u32;
                    decrement_index(&mut index, max);
                    change_anime_top_three_block(app, index)
                }

                RankingType::MangaRankingType(_) => {
                    let mut index = app.active_manga_rank_index;
                    let max = app.available_manga_ranking_types.len() as u32;
                    decrement_index(&mut index, max);
                    change_manga_top_three_block(app, index)
                }
            },

            _ => {}
        },
        k if common::right_event(k) => match &app.active_top_three {
            TopThreeBlock::Anime(_) => {
                let mut index = app.active_anime_rank_index;
                let max = app.available_anime_ranking_types.len() as u32;
                increment_index(&mut index, max);
                change_anime_top_three_block(app, index);
            }
            TopThreeBlock::Manga(_) => {
                let mut index = app.active_manga_rank_index;
                let max = app.available_manga_ranking_types.len() as u32;

                increment_index(&mut index, max);
                change_manga_top_three_block(app, index);
            }
            TopThreeBlock::Error(previous) => match previous {
                RankingType::AnimeRankingType(_) => {
                    let mut index = app.active_anime_rank_index;
                    let max = app.available_anime_ranking_types.len() as u32;
                    increment_index(&mut index, max);
                    change_anime_top_three_block(app, index)
                }

                RankingType::MangaRankingType(_) => {
                    let mut index = app.active_manga_rank_index;
                    let max = app.available_manga_ranking_types.len() as u32;
                    increment_index(&mut index, max);
                    change_manga_top_three_block(app, index)
                }
            },
            _ => {}
        },

        Key::Enter => get_media_detail_page(app),
        _ => {}
    }
    app.selected_top_three = index;
}

fn is_manga_data_available(app: &App, manga_type: &MangaRankingType) -> bool {
    match manga_type {
        MangaRankingType::All => app.top_three_manga.all.is_some(),
        MangaRankingType::Manga => app.top_three_manga.manga.is_some(),
        MangaRankingType::Novels => app.top_three_manga.novels.is_some(),
        MangaRankingType::OneShots => app.top_three_manga.oneshots.is_some(),
        MangaRankingType::Doujinshi => app.top_three_manga.doujin.is_some(),
        MangaRankingType::Manhwa => app.top_three_manga.manhwa.is_some(),
        MangaRankingType::Manhua => app.top_three_manga.manhua.is_some(),
        MangaRankingType::ByPopularity => app.top_three_manga.popular.is_some(),
        MangaRankingType::Favorite => app.top_three_manga.favourite.is_some(),
        MangaRankingType::Other(_) => false,
    }
}

fn is_anime_data_available(app: &App, anime_type: &AnimeRankingType) -> bool {
    match anime_type {
        AnimeRankingType::All => app.top_three_anime.all.is_some(),
        AnimeRankingType::Airing => app.top_three_anime.airing.is_some(),
        AnimeRankingType::Upcoming => app.top_three_anime.upcoming.is_some(),
        AnimeRankingType::TV => app.top_three_anime.tv.is_some(),
        AnimeRankingType::OVA => app.top_three_anime.ova.is_some(),
        AnimeRankingType::Movie => app.top_three_anime.movie.is_some(),
        AnimeRankingType::Special => app.top_three_anime.special.is_some(),
        AnimeRankingType::ByPopularity => app.top_three_anime.popular.is_some(),
        AnimeRankingType::Favorite => app.top_three_anime.favourite.is_some(),
        AnimeRankingType::Other(_) => false,
    }
}

fn change_anime_top_three_block(app: &mut App, index: u32) {
    let data_available =
        is_anime_data_available(app, &app.available_anime_ranking_types[index as usize]);

    if !data_available {
        app.active_top_three = TopThreeBlock::Loading(RankingType::AnimeRankingType(
            app.available_anime_ranking_types[index as usize].clone(),
        ));

        app.active_top_three_anime =
            Some(app.available_anime_ranking_types[index as usize].clone());

        app.dispatch(IoEvent::GetTopThree(TopThreeBlock::Anime(
            app.available_anime_ranking_types[index as usize].clone(),
        )));
        app.active_anime_rank_index = index;
    } else {
        app.active_top_three_anime =
            Some(app.available_anime_ranking_types[index as usize].clone());

        app.active_top_three =
            TopThreeBlock::Anime(app.available_anime_ranking_types[index as usize].clone());
        app.active_anime_rank_index = index;
    }
}

fn change_manga_top_three_block(app: &mut App, index: u32) {
    let data_available =
        is_manga_data_available(app, &app.available_manga_ranking_types[index as usize]);

    if !data_available {
        app.active_top_three = TopThreeBlock::Loading(RankingType::MangaRankingType(
            app.available_manga_ranking_types[index as usize].clone(),
        ));

        app.active_top_three_manga =
            Some(app.available_manga_ranking_types[index as usize].clone());

        app.dispatch(IoEvent::GetTopThree(TopThreeBlock::Manga(
            app.available_manga_ranking_types[index as usize].clone(),
        )));
        app.active_manga_rank_index = index;
    } else {
        app.active_top_three_manga =
            Some(app.available_manga_ranking_types[index as usize].clone());

        app.active_top_three =
            TopThreeBlock::Manga(app.available_manga_ranking_types[index as usize].clone());
        app.active_manga_rank_index = index;
    }
}

fn increment_index(index: &mut u32, max: u32) {
    if *index < max - 1 {
        *index = *index + 1;
    } else {
        *index = 0;
    }
}
fn decrement_index(index: &mut u32, max: u32) {
    if *index > 0 {
        *index = *index - 1;
    } else {
        *index = max - 1;
    }
}
