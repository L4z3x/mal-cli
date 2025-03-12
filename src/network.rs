use crate::{
    api::{self, model::*},
    app::{ActiveDisplayBlock, App, SelectedSearchTab},
    auth::OAuth,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum IoEvent {
    GetSearchResults(String),
    GetAnimeSearchResults(String),
    GetMangaSearchResults(String),
    GetAnime(String),
    GetAnimeRanking(String),
    GetSeasonalAnime(String),
    GetSuggestedAnime(String),
    UpdateAnimeListStatus(String),
    DeleteAnimeListStatus(String),
    GetAnimeList(String),
    GetManga(String),
    GetMangaRanking(String),
    UpdateMangaListStatus(String),
    DeleteMangaListStatus(String),
    GetMangaList(String),
    GetUserInfo(String),
}

#[derive(Clone)]
pub struct Network<'a> {
    oauth: OAuth,
    large_search_limit: u64,
    small_search_limit: u64,
    app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
    pub fn new(oauth: OAuth, app: &'a Arc<Mutex<App>>) -> Self {
        Self {
            oauth,
            large_search_limit: 20,
            small_search_limit: 4,
            app,
        }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::GetSearchResults(q) => {
                self.get_search_results(q).await;
            }
            // IoEvent::GetAnimeSearchResults(String) => {}
            // IoEvent::GetMangaSearchResults(String) => {}
            // IoEvent::GetAnime(String) => {}
            // IoEvent::GetAnimeRanking(String) => {}
            // IoEvent::GetSeasonalAnime(String) => {}
            // IoEvent::GetSuggestedAnime(String) => {}
            // IoEvent::UpdateAnimeListStatus(String) => {}
            // IoEvent::DeleteAnimeListStatus(String) => {}
            // IoEvent::GetAnimeList(String) => {}
            // IoEvent::GetManga(String) => {}
            // IoEvent::GetMangaRanking(String) => {}
            // IoEvent::UpdateMangaListStatus(String) => {}
            // IoEvent::DeleteMangaListStatus(String) => {}
            // IoEvent::GetMangaList(String) => {}
            // IoEvent::GetUserInfo(String) => {}
            _ => (),
        }

        let mut app = self.app.lock().await;
        app.is_loading = false
    }

    // TODO: Add actual error handling
    async fn handle_error(&mut self) {
        let mut app = self.app.lock().await;
        app.handle_error();
    }

    async fn get_search_results(&mut self, q: String) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;

        let anime_query = api::GetAnimeListQuery {
            q: q.clone(),
            limit: self.large_search_limit,
            offset: 0,
            nsfw: app.app_config.nsfw,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };

        let manga_query = api::GetMangaListQuery {
            q,
            limit: self.large_search_limit,
            offset: 0,
            nsfw: app.app_config.nsfw,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };

        match api::get_anime_list(&anime_query, &self.oauth).await {
            Ok(results) => {
                app.search_results.anime = Some(results);
            }
            Err(_) => {
                self.handle_error().await;
                return;
            }
        };

        match api::get_manga_list(&manga_query, &self.oauth).await {
            Ok(results) => {
                app.search_results.manga = Some(results);
            }
            Err(_) => {
                self.handle_error().await;
                return;
            }
        };
        app.search_results.selected_tab = SelectedSearchTab::Anime;
        app.active_display_block = ActiveDisplayBlock::SearchResultBlock;
    }
}
