use crate::{
    api::model::{AnimeRankingType, MangaRankingType},
    app::{ActiveDisplayBlock, App},
    event::Key,
    network::IoEvent,
};

use crate::handlers::{
    anime::{get_anime_ranking, get_manga_ranking},
    common, handle_result_block,
};

pub fn handler(key: Key, app: &mut App) {
    if app.popup {
        handle_popup(key, app);
    } else {
        match key {
            k if k == app.app_config.keys.toggle => {
                if app.active_display_block == ActiveDisplayBlock::AnimeRanking {
                    get_manga_ranking(app)
                } else {
                    get_anime_ranking(app)
                }
            }

            k if k == app.app_config.keys.open_popup => {
                app.popup = true;
            }

            _ => handle_result_block(key, app),
        }
    }
}

fn handle_popup(key: Key, app: &mut App) {
    match key {
        k if common::up_event(k) => {
            if app.active_display_block == ActiveDisplayBlock::AnimeRanking {
                if app.anime_ranking_type_index > 0 {
                    app.anime_ranking_type_index -= 1;
                } else {
                    app.anime_ranking_type_index = 8; // max index
                }
            } else {
                if app.manga_ranking_type_index > 0 {
                    app.manga_ranking_type_index -= 1;
                } else {
                    app.manga_ranking_type_index = 8; // max index
                }
            }
        }

        k if common::down_event(k) => {
            if app.active_display_block == ActiveDisplayBlock::AnimeRanking {
                app.anime_ranking_type_index = (app.anime_ranking_type_index + 1) % 9;
            } else {
                app.manga_ranking_type_index = (app.manga_ranking_type_index + 1) % 9;
            }
        }
        Key::Enter => {
            if app.active_display_block == ActiveDisplayBlock::AnimeRanking {
                app.popup = false;
                app.active_display_block = ActiveDisplayBlock::Loading;
                app.anime_ranking_type = get_anime_rank(app.anime_ranking_type_index);
                app.dispatch(IoEvent::GetAnimeRanking(app.anime_ranking_type.clone()));
            } else {
                app.popup = false;
                app.active_display_block = ActiveDisplayBlock::Loading;
                app.manga_ranking_type = get_manga_rank(app.manga_ranking_type_index);
                app.dispatch(IoEvent::GetMangaRanking(app.manga_ranking_type.clone()));
            }
        }
        _ => {}
    }
}

fn get_anime_rank(i: u8) -> AnimeRankingType {
    match i {
        0 => AnimeRankingType::All,
        1 => AnimeRankingType::Airing,
        2 => AnimeRankingType::Upcoming,
        3 => AnimeRankingType::Movie,
        4 => AnimeRankingType::ByPopularity,
        5 => AnimeRankingType::Special,
        6 => AnimeRankingType::TV,
        7 => AnimeRankingType::OVA,
        8 => AnimeRankingType::Favorite,
        _ => AnimeRankingType::Airing,
    }
}

fn get_manga_rank(i: u8) -> MangaRankingType {
    match i {
        0 => MangaRankingType::All,
        1 => MangaRankingType::Manga,
        2 => MangaRankingType::Manhwa,
        3 => MangaRankingType::ByPopularity,
        4 => MangaRankingType::Novels,
        5 => MangaRankingType::OneShots,
        6 => MangaRankingType::Doujinshi,
        7 => MangaRankingType::Manhua,
        8 => MangaRankingType::Favorite,
        _ => MangaRankingType::All,
    }
}
