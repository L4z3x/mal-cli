use crate::{
    api::{
        self, model::*, GetAnimeDetailQuery, GetAnimeRankingQuery, GetMangaDetailQuery,
        GetMangaRankingQuery, GetSeasonalAnimeQuery, GetSuggestedAnimeQuery,
        GetUserInformationQuery, UpdateUserAnimeListStatusQuery, UpdateUserMangaStatus,
    },
    app::{
        ActiveBlock, ActiveDisplayBlock, App, Data, Route, SelectedSearchTab, TopThreeBlock,
        UserAnimeList, UserMangaList,
    },
    auth::OAuth,
};
use bytes::Bytes;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::warn;

#[derive(Debug)]
pub enum IoEvent {
    GetSearchResults(String),
    GetAnimeSearchResults(String),
    GetMangaSearchResults(String),
    GetAnime(u64),
    GetManga(u64),
    GetAnimeRanking(AnimeRankingType),
    GetMangaRanking(MangaRankingType),
    GetSeasonalAnime,
    GetSuggestedAnime,
    UpdateAnimeListStatus(u64, UpdateUserAnimeListStatusQuery),
    DeleteAnimeListStatus(String),
    GetAnimeList(Option<UserWatchStatus>),
    GetMangaList(Option<UserReadStatus>),
    UpdateMangaListStatus(u64, UpdateUserMangaStatus),
    DeleteMangaListStatus(String),
    GetUserInfo,
    GetTopThree(TopThreeBlock),
}

#[derive(Clone)]
pub struct Network<'a> {
    oauth: OAuth,
    large_search_limit: u64,
    // small_search_limit: u64,
    app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
    pub fn new(oauth: OAuth, app: &'a Arc<Mutex<App>>, search_limit: u64) -> Self {
        Self {
            oauth,
            large_search_limit: search_limit,
            // small_search_limit: 3,
            app,
        }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::GetSearchResults(q) => self.get_search_results(q).await,

            IoEvent::GetSeasonalAnime => self.get_seasonal().await,

            IoEvent::GetAnimeRanking(r) => self.get_anime_ranking(r).await,

            IoEvent::GetMangaRanking(r) => self.get_manga_ranking(r).await,

            IoEvent::GetSuggestedAnime => self.get_suggested().await,

            IoEvent::GetAnimeList(s) => self.get_user_anime_list(s).await,

            IoEvent::GetMangaList(s) => self.get_user_manga_list(s).await,

            IoEvent::GetAnime(id) => self.get_anime_details(id).await,

            IoEvent::GetManga(id) => self.get_manga_details(id).await,

            // IoEvent::GetAnimeSearchResults(String) => {}
            // IoEvent::GetMangaSearchResults(String) => {}
            // IoEvent::GetSuggestedAnime(String) => {}
            // IoEvent::UpdateAnimeListStatus(String) => {}
            // IoEvent::DeleteAnimeListStatus(String) => {}
            // IoEvent::GetMangaRanking(String) => {}
            // IoEvent::UpdateMangaListStatus(String) => {}
            // IoEvent::DeleteMangaListStatus(String) => {}
            IoEvent::GetUserInfo => self.get_user_info().await,
            IoEvent::GetTopThree(r) => self.get_top_three(r).await,

            IoEvent::UpdateAnimeListStatus(anime_id, query) => {
                self.update_anime_list_status(anime_id, query).await
            }
            IoEvent::UpdateMangaListStatus(manga_id, query) => {
                self.update_manga_list_status(manga_id, query).await
            }
            _ => (),
        }

        let mut app = self.app.lock().await;
        app.is_loading = false
    }

    async fn get_anime_details(&mut self, id: u64) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;

        let query = GetAnimeDetailQuery {
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            nsfw: app.app_config.nsfw,
        };

        match api::get_anime_details(id, &query, &self.oauth).await {
            Ok(result) => {
                app.anime_details = Some(result.clone());
            }
            Err(e) => {
                app.write_error(e);
                app.active_display_block = ActiveDisplayBlock::Error;
                return;
            }
        }

        let mut image = None;
        if app.picker.is_some() {
            image = get_picture(
                app.app_config.paths.picture_cache_dir_path.clone(),
                app.anime_details.as_ref().unwrap().id,
                &app.anime_details.as_ref().unwrap().main_picture,
                app.app_config.max_cached_images,
            )
            .await;
            app.media_image = image.clone();
            app.image_state = Some(
                app.picker
                    .as_ref()
                    .unwrap()
                    .new_resize_protocol(app.get_picture_from_cache().unwrap()),
            );
        }

        let route = Route {
            data: Some(Data::Anime(app.anime_details.as_ref().unwrap().clone())),
            block: ActiveDisplayBlock::AnimeDetails,
            title: app.anime_details.as_ref().unwrap().title.clone(),
            image,
        };
        app.push_navigation_stack(route);
        app.active_block = ActiveBlock::DisplayBlock;
        app.active_display_block = ActiveDisplayBlock::AnimeDetails;
        app.display_block_title = app.anime_details.as_ref().unwrap().title.clone();
    }

    async fn get_manga_details(&mut self, id: u64) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;

        let query = GetMangaDetailQuery {
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            nsfw: app.app_config.nsfw,
        };

        match api::get_manga_details(id, &query, &self.oauth).await {
            Ok(result) => {
                app.manga_details = Some(result.clone());
            }
            Err(e) => {
                app.write_error(e);
                app.active_display_block = ActiveDisplayBlock::Error;
                return;
            }
        }

        let mut image = None;
        if app.picker.is_some() {
            image = get_picture(
                app.app_config.paths.picture_cache_dir_path.clone(),
                app.manga_details.as_ref().unwrap().id,
                &app.manga_details.as_ref().unwrap().main_picture,
                app.app_config.max_cached_images,
            )
            .await;
            app.media_image = image.clone();
            app.image_state = Some(
                app.picker
                    .as_ref()
                    .unwrap()
                    .new_resize_protocol(app.get_picture_from_cache().unwrap()),
            );
        }
        let route = Route {
            data: Some(Data::Manga(app.manga_details.as_ref().unwrap().clone())),
            block: ActiveDisplayBlock::MangaDetails,
            title: app.manga_details.as_ref().unwrap().title.clone(),
            image,
        };
        app.push_navigation_stack(route);
        app.active_block = ActiveBlock::DisplayBlock;
        app.active_display_block = ActiveDisplayBlock::MangaDetails;
        app.display_block_title = app.manga_details.as_ref().unwrap().title.clone();
    }

    async fn get_anime_ranking(&mut self, ranking_type: AnimeRankingType) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;
        let query = GetAnimeRankingQuery {
            ranking_type: ranking_type.clone(),
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            limit: self.large_search_limit,
            nsfw: app.app_config.nsfw,
            offset: 0,
        };
        let title = format!("Top Anime by {}", ranking_type);
        match api::get_anime_ranking(&query, &self.oauth).await {
            Ok(result) => {
                app.anime_ranking_data = Some(result.clone());
            }
            Err(e) => {
                app.write_error(e);
                app.active_display_block = ActiveDisplayBlock::Error;
                return;
            }
        }

        let route = Route {
            data: Some(Data::AnimeRanking(
                app.anime_ranking_data.as_ref().unwrap().clone(),
            )),
            block: ActiveDisplayBlock::AnimeRanking,
            title: title.clone(),
            image: None,
        };
        app.push_navigation_stack(route);
        app.active_block = ActiveBlock::DisplayBlock;
        app.active_display_block = ActiveDisplayBlock::AnimeRanking;
        app.display_block_title = title;
    }

    async fn get_manga_ranking(&mut self, ranking_type: MangaRankingType) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;
        let query = GetMangaRankingQuery {
            ranking_type: ranking_type.clone(),
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            limit: self.large_search_limit,
            nsfw: app.app_config.nsfw,
            offset: 0,
        };
        // better title:
        let mut rank = ranking_type.to_string();
        if rank == *"bypopularity" {
            rank = "Popular Manga".to_string();
        }
        let title = format!("Top {}", rank);
        match api::get_manga_ranking(&query, &self.oauth).await {
            Ok(result) => {
                app.manga_ranking_data = Some(result.clone());
            }
            Err(e) => {
                app.write_error(e);
                app.active_display_block = ActiveDisplayBlock::Error;
                return;
            }
        }

        let route = Route {
            data: Some(Data::MangaRanking(
                app.manga_ranking_data.as_ref().unwrap().clone(),
            )),
            block: ActiveDisplayBlock::MangaRanking,
            title: title.clone(),
            image: None,
        };
        app.push_navigation_stack(route);

        app.active_block = ActiveBlock::DisplayBlock;
        app.active_display_block = ActiveDisplayBlock::MangaRanking;
        app.display_block_title = title;
    }

    async fn get_top_three(&mut self, ranking_type: TopThreeBlock) {
        match ranking_type {
            TopThreeBlock::Anime(r) => self.get_anime_top_three(r).await,
            TopThreeBlock::Manga(r) => self.get_manga_top_three(r).await,
            _ => (),
        }
    }

    async fn get_anime_top_three(&mut self, rank_type: AnimeRankingType) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;
        let query = GetAnimeRankingQuery {
            ranking_type: rank_type.clone(),
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            limit: 3,
            nsfw: app.app_config.nsfw,
            offset: 0,
        };
        match api::get_anime_ranking(&query, &self.oauth).await {
            Ok(result) => match &rank_type {
                AnimeRankingType::Airing => {
                    app.top_three_anime.airing = Some([
                        result.data[0].node.clone(),
                        result.data[1].node.clone(),
                        result.data[2].node.clone(),
                    ]);
                }
                AnimeRankingType::All => {
                    app.top_three_anime.all = Some([
                        result.data[0].node.clone(),
                        result.data[1].node.clone(),
                        result.data[2].node.clone(),
                    ])
                }
                AnimeRankingType::Upcoming => {
                    app.top_three_anime.upcoming = Some([
                        result.data[0].node.clone(),
                        result.data[1].node.clone(),
                        result.data[2].node.clone(),
                    ]);
                }
                AnimeRankingType::ByPopularity => {
                    app.top_three_anime.popular = Some([
                        result.data[0].node.clone(),
                        result.data[1].node.clone(),
                        result.data[2].node.clone(),
                    ]);
                }
                AnimeRankingType::Favorite => {
                    app.top_three_anime.favourite = Some([
                        result.data[0].node.clone(),
                        result.data[1].node.clone(),
                        result.data[2].node.clone(),
                    ]);
                }
                AnimeRankingType::Movie => {
                    app.top_three_anime.movie = Some([
                        result.data[0].node.clone(),
                        result.data[1].node.clone(),
                        result.data[2].node.clone(),
                    ]);
                }
                AnimeRankingType::OVA => {
                    app.top_three_anime.ova = Some([
                        result.data[0].node.clone(),
                        result.data[1].node.clone(),
                        result.data[2].node.clone(),
                    ]);
                }
                AnimeRankingType::TV => {
                    app.top_three_anime.tv = Some([
                        result.data[0].node.clone(),
                        result.data[1].node.clone(),
                        result.data[2].node.clone(),
                    ]);
                }
                AnimeRankingType::Special => {
                    app.top_three_anime.special = Some([
                        result.data[0].node.clone(),
                        result.data[1].node.clone(),
                        result.data[2].node.clone(),
                    ]);
                }
                AnimeRankingType::Other(_s) => {}
            },
            Err(e) => {
                app.write_error(e);
                app.active_top_three = TopThreeBlock::Error(RankingType::AnimeRankingType(
                    app.active_top_three_anime
                        .as_ref()
                        .unwrap_or(&app.available_anime_ranking_types[0])
                        .clone(),
                ));
                return;
            }
        }
        app.active_top_three = TopThreeBlock::Anime(
            app.active_top_three_anime
                .as_ref()
                .unwrap_or(&app.available_anime_ranking_types[0])
                .clone(),
        );
    }

    async fn get_manga_top_three(&mut self, rank_type: MangaRankingType) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;
        let query = GetMangaRankingQuery {
            ranking_type: rank_type.clone(),
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            limit: 3,
            nsfw: app.app_config.nsfw,
            offset: 0,
        };

        match api::get_manga_ranking(&query, &self.oauth).await {
            Ok(results) => match &rank_type {
                MangaRankingType::All => {
                    app.top_three_manga.all = Some([
                        results.data[0].node.clone(),
                        results.data[1].node.clone(),
                        results.data[2].node.clone(),
                    ]);
                }
                MangaRankingType::Manga => {
                    app.top_three_manga.manga = Some([
                        results.data[0].node.clone(),
                        results.data[1].node.clone(),
                        results.data[2].node.clone(),
                    ]);
                }
                MangaRankingType::Novels => {
                    app.top_three_manga.novels = Some([
                        results.data[0].node.clone(),
                        results.data[1].node.clone(),
                        results.data[2].node.clone(),
                    ]);
                }
                MangaRankingType::OneShots => {
                    app.top_three_manga.oneshots = Some([
                        results.data[0].node.clone(),
                        results.data[1].node.clone(),
                        results.data[2].node.clone(),
                    ]);
                }
                MangaRankingType::Favorite => {
                    app.top_three_manga.favourite = Some([
                        results.data[0].node.clone(),
                        results.data[1].node.clone(),
                        results.data[2].node.clone(),
                    ]);
                }
                MangaRankingType::Doujinshi => {
                    app.top_three_manga.doujin = Some([
                        results.data[0].node.clone(),
                        results.data[1].node.clone(),
                        results.data[2].node.clone(),
                    ]);
                }
                MangaRankingType::Manhwa => {
                    app.top_three_manga.manhwa = Some([
                        results.data[0].node.clone(),
                        results.data[1].node.clone(),
                        results.data[2].node.clone(),
                    ]);
                }
                MangaRankingType::Manhua => {
                    app.top_three_manga.manhua = Some([
                        results.data[0].node.clone(),
                        results.data[1].node.clone(),
                        results.data[2].node.clone(),
                    ]);
                }
                MangaRankingType::ByPopularity => {
                    app.top_three_manga.popular = Some([
                        results.data[0].node.clone(),
                        results.data[1].node.clone(),
                        results.data[2].node.clone(),
                    ]);
                }
                MangaRankingType::Other(_) => {}
            },

            Err(e) => {
                app.write_error(e);
                app.active_top_three = TopThreeBlock::Error(RankingType::MangaRankingType(
                    app.active_top_three_manga
                        .as_ref()
                        .unwrap_or(&app.available_manga_ranking_types[0])
                        .clone(),
                ));
                return;
            }
        }
        app.active_top_three = TopThreeBlock::Manga(
            app.active_top_three_manga
                .as_ref()
                .unwrap_or(&app.available_manga_ranking_types[0])
                .clone(),
        );
    }

    async fn get_user_info(&mut self) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;
        let query = GetUserInformationQuery {
            fields: Some(ALL_USER_FIELDS.to_string()),
        };
        //? we can only use @me for the user in the current api version
        match api::get_my_user_information("@me".to_string(), &query, &self.oauth).await {
            Ok(result) => {
                app.user_profile = Some(result.clone());
            }
            Err(e) => {
                app.write_error(e);
                app.active_display_block = ActiveDisplayBlock::Error;
                return;
            }
        }
        let route = Route {
            data: Some(Data::UserInfo(app.user_profile.as_ref().unwrap().clone())),
            block: ActiveDisplayBlock::UserInfo,
            title: "Profile".to_string(),
            image: None,
        };
        app.push_navigation_stack(route);
        app.active_block = ActiveBlock::DisplayBlock;
        app.active_display_block = ActiveDisplayBlock::UserInfo;
        app.display_block_title = "Profile".to_string();
    }

    async fn get_suggested(&mut self) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;
        let query = GetSuggestedAnimeQuery {
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            limit: self.large_search_limit,
            nsfw: app.app_config.nsfw,
            offset: 0,
        };
        match api::get_suggested_anime(&query, &self.oauth).await {
            Ok(result) => {
                app.search_results.anime = Some(result.clone());
            }
            Err(e) => {
                app.write_error(e);
                app.active_display_block = ActiveDisplayBlock::Error;
                return;
            }
        }

        let route = Route {
            data: Some(Data::Suggestions(app.search_results.clone())),
            block: ActiveDisplayBlock::Suggestions,
            title: "Suggested Anime".to_string(),
            image: None,
        };
        app.push_navigation_stack(route);
        app.active_block = ActiveBlock::DisplayBlock;
        app.active_display_block = ActiveDisplayBlock::Suggestions;
        app.display_block_title = "Suggested Anime".to_string();
    }

    async fn get_seasonal(&mut self) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;
        let query = GetSeasonalAnimeQuery {
            sort: Some(app.anime_season.anime_sort.clone()),
            offset: 0,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            limit: self.large_search_limit,
            nsfw: app.app_config.nsfw,
        };
        match api::get_seasonal_anime(&app.anime_season.anime_season, &query, &self.oauth).await {
            Ok(result) => app.search_results.anime = Some(result),
            Err(e) => {
                app.write_error(e);
                app.active_display_block = ActiveDisplayBlock::Error;
                return;
            }
        }

        let title = format!(
            "Seasonal Anime: {} {}",
            app.anime_season.anime_season.season, app.anime_season.anime_season.year,
        );

        let route = Route {
            data: Some(Data::SearchResult(app.search_results.clone())),
            block: ActiveDisplayBlock::Seasonal,
            title: title.clone(),
            image: None,
        };
        app.push_navigation_stack(route);
        app.active_block = ActiveBlock::DisplayBlock;
        app.active_display_block = ActiveDisplayBlock::Seasonal;
        app.display_block_title = title;
    }

    async fn get_user_anime_list(&mut self, status: Option<UserWatchStatus>) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;
        let query = api::GetUserAnimeListQuery {
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            status: status.clone(),
            sort: Some(SortStyle::ListScore),
            limit: self.large_search_limit,
            offset: 0,
            nsfw: app.app_config.nsfw,
        };
        match api::get_user_anime_list("@me".to_string(), &query, &self.oauth).await {
            Ok(result) => {
                app.search_results.anime = Some(result.clone());
            }
            Err(e) => {
                app.write_error(e);
                app.active_display_block = ActiveDisplayBlock::Error;
                return;
            }
        }

        let data = UserAnimeList {
            anime_list: app.search_results.anime.as_ref().unwrap().clone(),
            status: status.clone(),
        };
        let route = Route {
            block: ActiveDisplayBlock::UserAnimeList,
            data: Some(Data::UserAnimeList(data)),
            title: format!("My Anime List: {}", get_status_string(status)),
            image: None,
        };
        app.active_block = ActiveBlock::DisplayBlock;
        app.active_display_block = ActiveDisplayBlock::UserAnimeList;
        app.display_block_title = route.title.clone();
        app.push_navigation_stack(route);
    }

    async fn get_user_manga_list(&mut self, status: Option<UserReadStatus>) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;
        let query = api::GetUserMangaListQuery {
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
            status: status.clone(),
            sort: Some(SortStyle::ListScore),
            limit: self.large_search_limit,
            offset: 0,
            nsfw: app.app_config.nsfw,
        };
        match api::get_user_manga_list("@me".to_string(), &query, &self.oauth).await {
            Ok(result) => {
                app.search_results.manga = Some(result.clone());
            }
            Err(e) => {
                app.write_error(e);
                app.active_display_block = ActiveDisplayBlock::Error;
                return;
            }
        }

        let data = UserMangaList {
            manga_list: app.search_results.manga.as_ref().unwrap().clone(),
            status: status.clone(),
        };
        let route = Route {
            block: ActiveDisplayBlock::UserMangaList,
            data: Some(Data::UserMangaList(data)),
            title: format!("My Manga List: {}", get_manga_status_string(status)),
            image: None,
        };

        app.active_block = ActiveBlock::DisplayBlock;
        app.active_display_block = ActiveDisplayBlock::UserMangaList;
        app.display_block_title = route.title.clone();
        app.push_navigation_stack(route);
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
            q: q.clone(),
            limit: self.large_search_limit,
            offset: 0,
            nsfw: app.app_config.nsfw,
            fields: Some(ALL_ANIME_AND_MANGA_FIELDS.to_string()),
        };

        match api::get_anime_list(&anime_query, &self.oauth).await {
            Ok(results) => {
                app.search_results.anime = Some(results);
            }
            Err(e) => {
                app.write_error(e);
                app.active_display_block = ActiveDisplayBlock::Error;
                return;
            }
        };

        match api::get_manga_list(&manga_query, &self.oauth).await {
            Ok(results) => {
                app.search_results.manga = Some(results);
            }
            Err(e) => {
                app.write_error(e);
                app.active_display_block = ActiveDisplayBlock::Error;
                return;
            }
        };

        let route = Route {
            data: Some(Data::SearchResult(app.search_results.clone())),
            block: ActiveDisplayBlock::SearchResultBlock,
            title: format!("Search Results: {}", q.clone()).to_string(),
            image: None,
        };
        app.push_navigation_stack(route);

        app.search_results.selected_tab = SelectedSearchTab::Anime;
        app.active_display_block = ActiveDisplayBlock::SearchResultBlock;
        app.display_block_title = format!("Search Results: {}", q).to_string()
    }

    async fn update_anime_list_status(
        &mut self,
        anime_id: u64,
        query: UpdateUserAnimeListStatusQuery,
    ) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;

        match api::update_anime_list_status(anime_id, &query, &self.oauth).await {
            Ok(result) => {
                app.anime_details.as_mut().unwrap().my_list_status = Some(result);
                app.popup_post_req_success_message = Some("updated Successfully".to_string());
                app.popup_post_req_success = true;
            }
            Err(e) => {
                app.write_error(e);
                app.popup_post_req_success = false;
            }
        }
        app.popup_is_loading = false;
    }

    async fn update_manga_list_status(&mut self, manga_id: u64, query: UpdateUserMangaStatus) {
        self.oauth.refresh().unwrap();
        let mut app = self.app.lock().await;
        match api::update_manga_list_status(manga_id, &query, &self.oauth).await {
            Ok(result) => {
                //
                app.manga_details.as_mut().unwrap().my_list_status = Some(result);
                app.popup_post_req_success = true;
                app.popup_post_req_success_message = Some("updated Successfully".to_string());
            }
            Err(e) => {
                app.write_error(e);
                app.popup_post_req_success = false;
            }
        }
        app.popup_is_loading = false;
    }
}

fn get_status_string(status: Option<UserWatchStatus>) -> String {
    match status {
        Some(s) => match s {
            UserWatchStatus::Completed => "completed".to_string(),
            UserWatchStatus::Watching => "watching".to_string(),
            UserWatchStatus::OnHold => "on_hold".to_string(),
            UserWatchStatus::Dropped => "dropped".to_string(),
            UserWatchStatus::PlanToWatch => "plan_to_watch".to_string(),
            UserWatchStatus::Other(_) => "All".to_string(),
        },
        None => "All".to_string(),
    }
}

fn get_manga_status_string(status: Option<UserReadStatus>) -> String {
    match status {
        Some(s) => match s {
            UserReadStatus::Completed => "completed".to_string(),
            UserReadStatus::Reading => "reading".to_string(),
            UserReadStatus::OnHold => "on_hold".to_string(),
            UserReadStatus::Dropped => "dropped".to_string(),
            UserReadStatus::PlanToRead => "plan_to_read".to_string(),
            UserReadStatus::Other(_) => "All".to_string(),
        },
        None => "All".to_string(),
    }
}

async fn fetch_image(url: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes)
}

async fn get_picture(
    image_dir_path: PathBuf,
    id: u64,
    pictures: &Option<Picture>,
    max_limit: u16,
) -> Option<(String, u32, u32)> {
    // check if the image is already in the cache, if not we fetch it and save it
    // look for it in the cache first:
    let file_path = image_dir_path.join(format!("{}.png", id));
    if file_path.exists() {
        let image = image::open(&file_path).ok()?;
        return Some((
            file_path.to_string_lossy().to_string(),
            image.width(),
            image.height(),
        ));
    }

    // fetch the image and save it
    // after fetch we need to look into the total count of files and if it exceeds the limit we delete the last one used
    //? make the image number limit a conf var
    //? we ll implement a FIFO mechanisme
    // so the flow is:
    // when an image is fetchead we enter its name  in the array (just use array for poping and pushing)

    // checking the json file
    let cache_index_file = image_dir_path.join("cache_index.json");
    if !cache_index_file.exists() {
        // create an empty  file with emtpy array
        let cache_index = Vec::<String>::new();
        // Read all existing images in the folder and create an array
        let mut existing_images = Vec::<String>::new();
        if let Ok(entries) = std::fs::read_dir(&image_dir_path) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name != "cache_index.json" {
                        existing_images.push(file_name.to_string());
                    }
                }
            }
        }
        let cache_index_json =
            serde_json::to_string(&cache_index).unwrap_or_else(|_| "[]".to_string());
        std::fs::write(&cache_index_file, cache_index_json).ok();
    }

    if let Some(p) = pictures {
        let urls = vec![&p.large, &p.medium];
        // loop all image urls and return the first fetched one
        for url in urls.into_iter().flatten() {
            // save the image in the .cache/mal-tui/media-images folder
            let image = fetch_image(url).await;
            match image {
                Ok(bytes) => {
                    let file_name = format!("{}.png", id);
                    let file_path = image_dir_path.join(file_name.clone());
                    let image = image::load_from_memory(&bytes).ok();
                    if let Some(image) = image {
                        image
                            .save_with_format(&file_path, image::ImageFormat::Png)
                            .ok();
                        // after saving the image we need to update the index file
                        // first push to array then check if it reached the max size if yes then we remove the first element(image then array element)
                        let res = update_image_cache(
                            &cache_index_file,
                            &image_dir_path,
                            &file_name,
                            max_limit,
                        )
                        .await;
                        if let Err(e) = res {
                            warn!("error updating the cache index file: {}", e)
                        }

                        return Some((
                            file_path.to_string_lossy().to_string(),
                            image.width(),
                            image.height(),
                        ));
                    }
                }
                Err(e) => {
                    warn!("Error fetching image: {}", e);
                }
            }
        }
    }
    None
}

async fn update_image_cache(
    cache_index_file: &PathBuf,
    image_dir_path: &Path,
    file_name: &str,
    max_limit: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    // read the current cache index
    let cache_content = std::fs::read_to_string(cache_index_file)?;
    let mut cache_index: Vec<String> = serde_json::from_str(&cache_content).unwrap_or_default();

    // add the new image name to the array
    cache_index.push(file_name.to_string());

    // Check if we've reached the capacity limit
    if cache_index.len() > max_limit as usize {
        // Remove the first element and delete the corresponding image
        let oldest_image = cache_index.remove(0);
        let oldest_image_path = image_dir_path.join(&oldest_image);
        if oldest_image_path.exists() {
            std::fs::remove_file(oldest_image_path).ok();
        }
    }

    // Write the updated cache index back to the file
    let updated_cache_json = serde_json::to_string(&cache_index)?;
    std::fs::write(cache_index_file, updated_cache_json)?;

    Ok(())
}
