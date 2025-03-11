mod anime;
mod common;
mod display_block;
mod help;
mod input;
mod manga;
mod user;

use crate::app::{
    ActiveBlock, ActiveDisplayBlock, App, SearchResultBlock, ANIME_OPTIONS_RANGE,
    MANGA_OPTIONS_RANGE, USER_OPTIONS_RANGE,
};
use crate::event::Key;

pub use input::handler as input_handler;

pub fn handle_app(key: Key, app: &mut App) {
    // First handle any global event and then move to block event
    match key {
        Key::Esc => {
            handle_escape(app);
        }
        _ if key == app.app_config.keys.help => {
            app.active_display_block = ActiveDisplayBlock::Help;
        }
        _ if key == app.app_config.keys.search => {
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

        ActiveBlock::Manga => manga::handler(key, app),

        ActiveBlock::User => user::handler(key, app),

        ActiveBlock::Error => {}

        ActiveBlock::DisplayBlock => display_block::handle_display_block(key, app),
    }
    // todo: move this to active_display_block_handler
    // help::handler(key, app);
} // todo: move this to active_display_block_handler
  // ActiveBlock::BasicView => {}

fn handle_escape(app: &mut App) {
    match app.active_block {
        ActiveBlock::DisplayBlock => {
            // todo: should i remove the display block when escape is pressed ?
            app.search_results.selected_block = SearchResultBlock::Empty;
        }
        ActiveBlock::Error => {
            app.active_display_block = ActiveDisplayBlock::Empty;
        }
        // do nothing
        _ => {}
    }
}

pub fn handle_tab(app: &mut App) {
    match app.active_block {
        ActiveBlock::Input => {
            // todo: anything else to handle ? like when exiting the input state.
            app.active_block = ActiveBlock::Anime;
        }

        ActiveBlock::Anime => {
            app.library.selected_index = MANGA_OPTIONS_RANGE.start;
            app.active_block = ActiveBlock::Manga;
        }
        ActiveBlock::Manga => {
            app.library.selected_index = USER_OPTIONS_RANGE.start;
            app.active_block = ActiveBlock::User;
        }
        ActiveBlock::User => {
            app.library.selected_index = ANIME_OPTIONS_RANGE.start;
            app.active_block = ActiveBlock::DisplayBlock;
        }

        ActiveBlock::DisplayBlock => {
            app.active_block = ActiveBlock::Input;
            // todo: handle cases when exiting the Display_block.

            // app.search_results.selected_block = match app.search_results.selected_block {
            //     SearchResultBlock::AnimeSearch => SearchResultBlock::MangaSearch,
            //     SearchResultBlock::MangaSearch => SearchResultBlock::AnimeSearch,
            //     SearchResultBlock::Empty => SearchResultBlock::Empty,
            // };
        }
        // ActiveBlock::UserStats => {
        //     app.set_current_route_state(Some(ActiveBlock::Anime), Some(ActiveBlock::Anime));
        // }
        _ => {}
    }
}
