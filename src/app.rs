use crate::api::{self, model::*};
use crate::config::app_config::AppConfig;
use crate::network::IoEvent;
use chrono::Datelike;
use ratatui::layout::Rect;
use ratatui_image::picker::Picker;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::mpsc::Sender;
use strum_macros::IntoStaticStr;
use tui_scrollview::ScrollViewState;
const DEFAULT_ROUTE: Route = Route {
    data: None,
    block: ActiveDisplayBlock::Empty, //todo: change to empty
    title: String::new(),
    image: None,
};

pub const DISPLAY_RAWS_NUMBER: usize = 5;

pub const SEASONS: [&str; 4] = ["Winter", "Spring", "Summer", "Fall"];

pub const DISPLAY_COLUMN_NUMBER: usize = 3;

pub const ANIME_OPTIONS: [&str; 3] = ["Seasonal", "Ranking", "Suggested"];

pub const USER_OPTIONS: [&str; 3] = ["Stats", "AnimeList", "MangaList"];

pub const GENERAL_OPTIONS: [&str; 3] = ["Help", "About", "Quit"];

pub const ANIME_OPTIONS_RANGE: std::ops::Range<usize> = 0..3;

pub const USER_OPTIONS_RANGE: std::ops::Range<usize> = 3..6;

pub const GENERAL_OPTIONS_RANGE: std::ops::Range<usize> = 6..9;

pub const ANIME_RANKING_TYPES: [&str; 9] = [
    "All",
    "Airing",
    "Upcoming",
    "Movie",
    "Popularity",
    "Special",
    "TV",
    "OVA",
    "Favorite",
];

pub const MANGA_RANKING_TYPES: [&str; 9] = [
    "All",
    "Manga",
    "Manhwa",
    "Popularity",
    "Novels",
    "Oneshots",
    "Doujin",
    "Manhua",
    "Favorite",
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Input,
    DisplayBlock,
    Anime,
    Option,
    User,
    TopThree,
    Error,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveDisplayBlock {
    SearchResultBlock,
    Help,
    UserInfo,
    UserAnimeList,
    UserMangaList,
    Suggestions,
    Seasonal,
    AnimeRanking,
    MangaRanking,
    Loading,
    Error,
    Empty,
    AnimeDetails,
    MangaDetails,
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SelectedSearchTab {
    Anime,
    Manga,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub anime: Option<Page<Anime>>,
    pub manga: Option<Page<Manga>>,
    pub selected_tab: SelectedSearchTab,
    pub selected_anime_index: Option<usize>,
    pub selected_display_card_index: Option<usize>,
    pub selected_manga_index: Option<usize>,
}

#[derive(Clone)]
pub struct ScrollablePages<T> {
    index: usize,
    pages: Vec<T>,
}

impl<T> ScrollablePages<T> {
    pub fn new() -> Self {
        Self {
            index: 0,
            pages: vec![],
        }
    }

    pub fn get_results(&self, at_index: Option<usize>) -> Option<&T> {
        self.pages.get(at_index.unwrap_or(self.index))
    }

    pub fn get_mut_results(&mut self, at_index: Option<usize>) -> Option<&mut T> {
        self.pages.get_mut(at_index.unwrap_or(self.index))
    }

    pub fn add_pages(&mut self, new_pages: T) {
        self.pages.push(new_pages);
        self.index = self.pages.len() - 1;
    }
}

pub struct Library {
    pub selected_index: usize,
    pub saved_anime: ScrollablePages<Page<Anime>>,
    pub saved_manga: ScrollablePages<Page<Manga>>,
}

#[derive(Debug)]
pub struct Navigator {
    pub history: Vec<u16>,
    pub index: u16,
    pub data: HashMap<u16, Route>,
    pub last_id: u16,
}

impl Navigator {
    pub fn new() -> Self {
        let mut data = HashMap::new();
        data.insert(0, DEFAULT_ROUTE);
        Self {
            history: vec![0],
            index: 0,
            data,
            last_id: 0,
        }
    }

    pub fn add_existing_route(&mut self, id: u16) {
        self.history.push(id);
        self.index = self.history.len() as u16 - 1;
    }

    pub fn add_route(&mut self, r: Route) {
        self.last_id += 1;
        self.data.insert(self.last_id, r);
        self.history.push(self.last_id);
        self.index = self.history.len() as u16 - 1;
    }

    pub fn remove_old_history(&mut self) {
        self.history.remove(1);
        self.clear_unused_data();
    }

    pub fn clear_unused_data(&mut self) {
        let active_routes: HashSet<u16> = self.history.iter().copied().collect();
        self.data.retain(|k, _| active_routes.contains(k));
    }

    pub fn get_current_title(&self) -> &String {
        let id = self.history[self.index as usize];
        &self.data[&id].title
    }

    pub fn get_current_block(&self) -> &ActiveDisplayBlock {
        let id = self.history[self.index as usize];
        &self.data[&id].block
    }
}

pub struct App {
    pub io_tx: Option<Sender<IoEvent>>,
    pub app_config: AppConfig,
    pub is_loading: bool,
    pub api_error: String,
    pub search_results: SearchResult,
    pub size: Rect,
    pub input: Vec<char>,
    pub input_cursor_position: u16,
    pub input_idx: usize,
    pub library: Library,
    pub help_menu_offset: u32,
    pub help_menu_page: u32,
    pub help_menu_max_lines: u32,
    pub help_docs_size: u32,
    // image:
    pub picker: Option<Picker>,
    pub media_image: Option<String>,
    // state:
    pub active_block: ActiveBlock,
    pub active_display_block: ActiveDisplayBlock,
    pub navigator: Navigator,
    pub display_block_title: String,
    pub popup: bool,
    pub details_scroll_view_state: ScrollViewState,
    // top three bar:
    pub top_three_anime: TopThreeAnime,
    pub top_three_manga: TopThreeManga,
    pub active_top_three: TopThreeBlock,
    pub active_top_three_anime: Option<AnimeRankingType>,
    pub active_top_three_manga: Option<MangaRankingType>,
    pub selected_top_three: u32,
    pub available_anime_ranking_types: Vec<AnimeRankingType>,
    pub available_manga_ranking_types: Vec<MangaRankingType>,
    pub active_anime_rank_index: u32,
    pub active_manga_rank_index: u32,
    // detail
    pub anime_details: Option<Anime>,
    pub manga_details: Option<Manga>,
    // seasonal
    pub anime_season: Seasonal,
    //ranking
    pub anime_ranking_data: Option<Ranking<RankingAnimePair>>,
    pub anime_ranking_type: AnimeRankingType,
    pub manga_ranking_data: Option<Ranking<RankingMangaPair>>,
    pub manga_ranking_type: MangaRankingType,
    pub anime_ranking_index: u8,
    pub manga_ranking_index: u8,
    //profile:
    pub user_profile: Option<UserInfo>,
    // use UserWatchStatus to determine the current tab
    pub anime_list_status: Option<UserWatchStatus>,
    // use UserReadStatus to determine the current tab
    pub manga_list_status: Option<UserReadStatus>,
}

pub struct Seasonal {
    pub anime_season: AnimeSeason,
    pub popup_season_highlight: bool,
    pub anime_sort: SortStyle,
    pub selected_season: u8,
    pub selected_year: u16,
}

#[derive(Debug, Clone, IntoStaticStr)]
pub enum TopThreeBlock {
    Anime(AnimeRankingType),
    Manga(MangaRankingType),
    Loading(RankingType),
    Error(RankingType),
}

#[derive(Debug, Clone)]
pub struct TopThreeManga {
    pub all: Option<[Manga; 3]>,
    pub manga: Option<[Manga; 3]>,
    pub novels: Option<[Manga; 3]>,
    pub oneshots: Option<[Manga; 3]>,
    pub doujin: Option<[Manga; 3]>,
    pub manhwa: Option<[Manga; 3]>,
    pub manhua: Option<[Manga; 3]>,
    pub popular: Option<[Manga; 3]>,
    pub favourite: Option<[Manga; 3]>,
}

impl TopThreeManga {
    pub fn default() -> Self {
        Self {
            all: None,
            manga: None,
            novels: None,
            oneshots: None,
            doujin: None,
            manhwa: None,
            manhua: None,
            popular: None,
            favourite: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TopThreeAnime {
    pub airing: Option<[Anime; 3]>,
    pub upcoming: Option<[Anime; 3]>,
    pub popular: Option<[Anime; 3]>,
    pub all: Option<[Anime; 3]>,
    pub tv: Option<[Anime; 3]>,
    pub ova: Option<[Anime; 3]>,
    pub movie: Option<[Anime; 3]>,
    pub special: Option<[Anime; 3]>,
    pub favourite: Option<[Anime; 3]>,
}

impl TopThreeAnime {
    pub fn default() -> Self {
        Self {
            airing: None,
            upcoming: None,
            popular: None,
            all: None,
            tv: None,
            ova: None,
            movie: None,
            special: None,
            favourite: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Data {
    SearchResult(SearchResult),
    Suggestions(SearchResult),
    UserInfo(UserInfo),
    Anime(Anime),
    Manga(Manga),
    UserAnimeList(UserAnimeList),
    UserMangaList(UserMangaList),
    AnimeRanking(Ranking<RankingAnimePair>),
    MangaRanking(Ranking<RankingMangaPair>),
}

#[derive(Debug, Clone)]
pub struct UserAnimeList {
    pub anime_list: Page<Anime>,
    pub status: Option<UserWatchStatus>,
}
#[derive(Debug, Clone)]
pub struct UserMangaList {
    pub manga_list: Page<Manga>,
    pub status: Option<UserReadStatus>,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub data: Option<Data>,
    pub block: ActiveDisplayBlock,
    pub title: String,
    pub image: Option<String>,
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>, app_config: AppConfig) -> Self {
        // let can_render =
        let year = chrono::Utc::now().year_ce();
        let season = get_season();
        let selected_season = get_selected_season(&season);
        let picker_res = Picker::from_query_stdio();
        let mut picker: Option<Picker> = None;
        if picker_res.is_ok() {
            picker = Some(picker_res.unwrap());
        }
        Self {
            io_tx: Some(io_tx),
            anime_season: Seasonal {
                anime_season: AnimeSeason {
                    year: year.1 as u64,
                    season,
                },
                anime_sort: SortStyle::ListScore,
                popup_season_highlight: true,
                selected_season,
                selected_year: year.1 as u16,
            },

            available_anime_ranking_types: app_config.top_three_anime_types.clone(),
            active_top_three: TopThreeBlock::Anime(app_config.top_three_anime_types[0].clone()),
            available_manga_ranking_types: app_config.top_three_manga_types.clone(),
            app_config,
            is_loading: false,
            api_error: String::new(),
            search_results: SearchResult {
                anime: None,
                manga: None,
                selected_anime_index: None,
                selected_manga_index: None,
                selected_display_card_index: Some(0),
                selected_tab: SelectedSearchTab::Anime,
            },
            size: Rect::default(),
            input: vec![],
            input_cursor_position: 0,
            input_idx: 0,
            library: Library {
                saved_anime: ScrollablePages::new(),
                saved_manga: ScrollablePages::new(),
                selected_index: 9, // out of range to show nothing
            },
            help_menu_offset: 0,
            help_menu_page: 0,
            help_menu_max_lines: 0,
            help_docs_size: 0,
            active_block: ActiveBlock::DisplayBlock,
            active_display_block: DEFAULT_ROUTE.block,
            navigator: Navigator::new(),
            // top three
            top_three_anime: TopThreeAnime::default(),
            top_three_manga: TopThreeManga::default(),
            selected_top_three: 3, // out of index to select nothing
            active_top_three_anime: None,
            active_top_three_manga: None,
            active_anime_rank_index: 0,
            active_manga_rank_index: 0,
            // ranking page
            anime_ranking_data: None,
            anime_ranking_type: AnimeRankingType::All,
            anime_ranking_index: 0,
            manga_ranking_data: None,
            manga_ranking_type: MangaRankingType::All,
            manga_ranking_index: 0,
            // anime list
            anime_list_status: None,
            // manga list
            manga_list_status: None,
            //
            anime_details: None,
            manga_details: None,
            user_profile: None,
            display_block_title: String::new(),
            popup: false,
            media_image: None,
            picker,
            details_scroll_view_state: ScrollViewState::default(),
        }
    }

    pub fn write_error(&mut self, e: api::Error) {
        match e {
            api::Error::NoAuth => {
                self.api_error = "Auth Error, Please reload the App".to_string();
            }
            api::Error::TimedOut => {
                self.api_error = "Conntection Timed Out, Please try again".to_string();
            }
            api::Error::Unknown => {
                self.api_error = "Check you internet connection".to_string();
            }
            api::Error::NoBody => {
                self.api_error = "there is No Body".to_string();
            }
            api::Error::ParseError(e) => {
                self.api_error = format!("Parse Error: {}", e);
            }
            api::Error::QuerySerializeError(e) => {
                self.api_error = format!("Query Serialize Error: {}", e);
            }
            api::Error::HttpError(e) => {
                self.api_error = format!("Http Error: {}", e);
            }
        }
    }

    pub fn get_top_three(&mut self) {
        let _ = &self.dispatch(IoEvent::GetTopThree(self.active_top_three.clone()));
    }

    pub fn dispatch(&mut self, event: IoEvent) {
        self.is_loading = true;
        if let Some(io_tx) = &self.io_tx {
            if let Err(e) = io_tx.send(event) {
                self.is_loading = false;
                // dbg!(e);
                println!("Error from dispatch {}", e);
            }
        };
    }

    pub fn clear_route_before_push(&mut self) {
        let index = self.navigator.index as usize;

        if index < self.navigator.history.len() - 1 {
            for _ in index + 1..self.navigator.history.len() {
                self.navigator.history.pop();
                self.navigator.clear_unused_data();
            }
        }
        self.remove_old_history();
    }

    fn push_existing_route(&mut self, id: u16) {
        self.clear_route_before_push();
        self.navigator.add_existing_route(id);
    }

    pub fn push_navigation_stack(&mut self, r: Route) {
        self.clear_route_before_push();
        self.navigator.add_route(r);
        self.remove_old_history();
    }

    fn remove_old_history(&mut self) {
        if self.navigator.history.len() - 1 > self.app_config.navigation_stack_limit as usize {
            self.navigator.remove_old_history();
        }
    }

    pub fn get_current_route(&self) -> Option<&Route> {
        let index = self.navigator.index as usize;

        // Ensure the index is within bounds
        if index >= self.navigator.history.len() {
            eprintln!("Error: Navigation index {} is out of bounds", index);
            return None;
        }

        let id = self.navigator.history[index];

        // Ensure the route ID exists in the data map
        match self.navigator.data.get(&id) {
            Some(route) => Some(route),
            None => {
                eprintln!("Error: Route ID {} not found in data map", id);
                None
            }
        }
    }

    pub fn calculate_help_menu_offset(&mut self) {
        let old_offset = self.help_menu_offset;
        if self.help_menu_max_lines < self.help_docs_size {
            self.help_menu_offset = self.help_menu_page * self.help_menu_max_lines;
        }
        if self.help_menu_offset > self.help_docs_size {
            self.help_menu_offset = old_offset;
            self.help_menu_page -= 1;
        }
    }

    pub fn load_previous_route(&mut self) {
        if self.popup {
            self.popup = false;
            return;
        }

        if self.navigator.index == 1 {
            self.active_display_block = ActiveDisplayBlock::Empty;
            self.display_block_title = "Home".to_string();
            self.navigator.index = 0;
            return;
        }

        if self.active_display_block == ActiveDisplayBlock::Loading {
            return;
        }

        if self.active_display_block == ActiveDisplayBlock::Error
            || self.active_display_block == ActiveDisplayBlock::Help
        {
            self.active_display_block = self.navigator.get_current_block().clone();
            return;
        }
        if self.navigator.index == 0 {
            return;
        }
        let i = self.navigator.index.saturating_sub(1);
        self.load_state_data(i);
    }

    pub fn load_next_route(&mut self) {
        if self.navigator.index >= self.navigator.history.len() as u16 {
            self.navigator.index = self.navigator.history.len().saturating_sub(2) as u16;
        }

        if self.navigator.index == self.navigator.history.len() as u16 - 1 {
            return;
        }

        self.load_state_data(self.navigator.index + 1);
    }

    pub fn load_route(&mut self, id: u16) {
        // todo: change to u16
        self.push_existing_route(id as u16);
        self.load_state_data(self.navigator.history.len() as u16 - 1);
    }

    fn load_state_data(&mut self, i: u16) {
        if i as usize >= self.navigator.history.len() {
            return;
        }
        self.navigator.index = i;
        let route = match self.get_current_route() {
            Some(route) => route.clone(),
            None => return,
        };

        let data = route.data.clone();
        match data {
            Some(data) => {
                match data {
                    Data::SearchResult(d) => {
                        self.search_results = d.clone();
                    }

                    Data::Suggestions(d) => {
                        self.search_results = d.clone();
                    }

                    Data::Anime(d) => {
                        // self.set_image_from_route(route.as_ref().unwrap(), Some(d.clone()));
                        self.anime_details = Some(d.clone());

                        if let Some(image) = &route.image {
                            self.media_image = Some(image.clone());
                        }
                    }

                    Data::Manga(d) => {
                        self.manga_details = Some(d.clone()); // todo: add here too.
                    }

                    Data::AnimeRanking(d) => {
                        self.anime_ranking_data = Some(d.clone());
                    }

                    Data::MangaRanking(d) => {
                        self.manga_ranking_data = Some(d.clone());
                    }

                    Data::UserInfo(d) => self.user_profile = Some(d.clone()),

                    Data::UserAnimeList(d) => {
                        self.anime_list_status = d.status.clone();
                        self.search_results.anime = Some(d.anime_list.clone());
                    }

                    Data::UserMangaList(d) => {
                        self.manga_list_status = d.status.clone();
                        self.search_results.manga = Some(d.manga_list.clone());
                    }
                }

                self.active_display_block = self.navigator.get_current_block().clone();
                self.display_block_title = self.navigator.get_current_title().clone();
                self.active_block = ActiveBlock::DisplayBlock;
            }

            None => {
                self.active_display_block = ActiveDisplayBlock::Empty;
                self.display_block_title = "No data".to_string();
            }
        }
    }

    pub fn next_anime_list_status(&self) -> Option<UserWatchStatus> {
        match &self.anime_list_status {
            Some(s) => match s {
                UserWatchStatus::Watching => Some(UserWatchStatus::Completed),
                UserWatchStatus::Completed => Some(UserWatchStatus::OnHold),
                UserWatchStatus::OnHold => Some(UserWatchStatus::Dropped),
                UserWatchStatus::Dropped => Some(UserWatchStatus::PlanToWatch),
                UserWatchStatus::PlanToWatch => None,
                UserWatchStatus::Other(_) => None,
            },
            None => Some(UserWatchStatus::Watching),
        }
    }

    pub fn previous_anime_list_status(&self) -> Option<UserWatchStatus> {
        match &self.anime_list_status {
            Some(s) => match s {
                UserWatchStatus::Watching => None,
                UserWatchStatus::Completed => Some(UserWatchStatus::Watching),
                UserWatchStatus::OnHold => Some(UserWatchStatus::Completed),
                UserWatchStatus::Dropped => Some(UserWatchStatus::OnHold),
                UserWatchStatus::PlanToWatch => Some(UserWatchStatus::Dropped),
                UserWatchStatus::Other(_) => Some(UserWatchStatus::PlanToWatch),
            },
            None => Some(UserWatchStatus::Watching),
        }
    }
}

fn get_season() -> Season {
    let month = chrono::Utc::now().month();
    match month {
        3..=5 => Season::Spring,
        6..=8 => Season::Summer,
        9..=11 => Season::Fall,
        _ => Season::Winter,
    }
}

fn get_selected_season(season: &Season) -> u8 {
    match season {
        &Season::Winter => 0,
        &Season::Spring => 1,
        &Season::Summer => 2,
        &Season::Fall => 3,
        &Season::Other(_) => panic!("no season selected"),
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::config::app_config::AppConfig;
    pub fn get_app() -> App {
        let config = AppConfig::load();
        let (sync_io_tx, _) = std::sync::mpsc::channel::<IoEvent>();

        let mut app = App::new(sync_io_tx, config.unwrap());
        let route = Route {
            data: None,
            block: ActiveDisplayBlock::Empty,
            title: "Home".to_string(),
            image: None,
        };
        app.push_navigation_stack(route.clone());
        app.push_navigation_stack(route.clone());
        app.push_navigation_stack(route.clone());
        app.push_navigation_stack(route);
        app
    }
    #[test]
    fn test_navigation_push() {
        let app = get_app();

        assert_eq!(app.navigator.history.len(), 5);
        assert_eq!(app.navigator.index, 4);
    }

    #[test]
    fn test_backward_navigation() {
        let mut app = get_app();
        assert_eq!(app.navigator.index, 4);
        app.load_previous_route();
        assert_eq!(app.navigator.index, 3);
        app.load_previous_route();
        assert_eq!(app.navigator.index, 2);
        app.load_previous_route();
        assert_eq!(app.navigator.index, 1);
        app.load_previous_route();
        assert_eq!(app.navigator.index, 0);
    }
    #[test]
    fn test_forward_navigation() {
        let mut app = get_app();
        app.navigator.index = 0;
        app.load_next_route();
        assert_eq!(app.navigator.index, 1);
        app.load_next_route();
        assert_eq!(app.navigator.index, 2);
        app.load_next_route();
        assert_eq!(app.navigator.index, 3);
        app.load_next_route();
        assert_eq!(app.navigator.index, 4);
    }
}
