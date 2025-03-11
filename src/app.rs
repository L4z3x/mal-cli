use crate::api::model::*;
use crate::config::app_config::AppConfig;
use crate::network::IoEvent;
use ratatui::layout::Rect;
use std::sync::mpsc::Sender;

// const DEFAULT_ROUTE: Route = Route {
//     id: RouteId::Home,
//     active_block: ActiveBlock::Anime,
//     hovered_block: ActiveBlock::UserStats,
// };

pub const ANIME_OPTIONS: [&str; 3] = ["Seasonal", "Ranking", "Suggested"];

pub const MANGA_OPTIONS: [&str; 1] = ["Ranking"];

pub const USER_OPTIONS: [&str; 3] = ["Stats", "AnimeList", "MangaList"];

pub const ANIME_OPTIONS_RANGE: std::ops::Range<usize> = 0..3;

pub const MANGA_OPTIONS_RANGE: std::ops::Range<usize> = 3..4;

pub const USER_OPTIONS_RANGE: std::ops::Range<usize> = 4..7;

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Search,
    Home,
    Seasonal,
    Recommendations,
    Ranking,
    Error,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Input,
    DisplayBlock,
    Anime,
    Manga,
    User,
    Error,
    // SearchResultBlock,
    // UserStats,
    // Error,
    // Help,
    // BasicView,
    // Anime,
    // Manga,
    // User,
    // Empty,
}

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
}

#[derive(Debug)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

pub struct SearchResult {
    pub anime: Option<Page<Anime>>,
    pub manga: Option<Page<Manga>>,
    pub selected_anime_index: Option<usize>,
    pub selected_manga_index: Option<usize>,
    pub hovered_block: SearchResultBlock,
    pub selected_block: SearchResultBlock,
}

#[derive(PartialEq, Debug)]
pub enum SearchResultBlock {
    AnimeSearch,
    MangaSearch,
    Empty,
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
    pub active_block: ActiveBlock,
    pub active_display_block: ActiveDisplayBlock,
    // navigation_stack: Vec<Route>,
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>, app_config: AppConfig) -> Self {
        Self {
            io_tx: Some(io_tx),
            app_config,
            is_loading: false,
            api_error: String::new(),
            search_results: SearchResult {
                hovered_block: SearchResultBlock::AnimeSearch,
                selected_block: SearchResultBlock::Empty,
                anime: None,
                manga: None,
                selected_anime_index: None,
                selected_manga_index: None,
            },
            size: Rect::default(),
            input: vec![],
            input_cursor_position: 0,
            input_idx: 0,
            library: Library {
                saved_anime: ScrollablePages::new(),
                saved_manga: ScrollablePages::new(),
                selected_index: 0,
            },
            help_menu_offset: 0,
            help_menu_page: 0,
            help_menu_max_lines: 0,
            help_docs_size: 0,
            active_block: ActiveBlock::DisplayBlock,
            active_display_block: ActiveDisplayBlock::Suggestions,
            // navigation_stack: vec![DEFAULT_ROUTE],
        }
    }

    // TODO:  <05-09-20, yourname>  Handle errors in TUI //
    pub fn handle_error(&mut self) {
        self.api_error = "Error!".to_string();
    }

    // Send a network event to the network thread
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

    // pub fn push_navigation_stack(
    //     &mut self,
    //     next_route_id: RouteId,
    //     next_active_block: ActiveBlock,
    // ) {
    //     self.navigation_stack.push(Route {
    //         id: next_route_id,
    //         active_block: next_active_block,
    //         hovered_block: next_active_block,
    //     })
    // }

    // pub fn pop_navigation_stack(&mut self) -> Option<Route> {
    //     if self.navigation_stack.len() == 1 {
    //         None
    //     } else {
    //         self.navigation_stack.pop()
    //     }
    // }

    // pub fn get_current_route(&self) -> &Route {
    //     self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    // }

    // pub fn get_current_route_mut(&mut self) -> &mut Route {
    //     self.navigation_stack.last_mut().unwrap()
    // }

    // pub fn set_current_route_state(
    //     &mut self,
    //     active_block: Option<ActiveBlock>,
    //     hovered_block: Option<ActiveBlock>,
    // ) {
    //     let current_route = self.get_current_route_mut();
    //     if let Some(active_block) = active_block {
    //         current_route.active_block = active_block;
    //     }
    //     if let Some(hovered_block) = hovered_block {
    //         current_route.hovered_block = hovered_block;
    //     }
    // }
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
}
