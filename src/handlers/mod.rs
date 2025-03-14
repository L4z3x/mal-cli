mod anime;
mod common;
mod display_block;
mod help;
mod input;
mod manga;
mod result;
mod top_three;
mod user;
use crate::app::{
    ActiveBlock, ActiveDisplayBlock, App, ANIME_OPTIONS_RANGE, GENERAL_OPTIONS_RANGE,
    USER_OPTIONS_RANGE,
};
use crate::event::Key;

pub use input::handler as input_handler;

pub fn handle_app(key: Key, app: &mut App) {
    // First handle any global event and then move to block event
    match key {
        Key::Esc => app.load_previous_state(),
        Key::Ctrl('p') => app.load_next_state(),
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

        ActiveBlock::User => manga::handler(key, app),

        ActiveBlock::Option => user::handler(key, app),

        ActiveBlock::Error => {}

        ActiveBlock::TopThree => {
            top_three::handler(key, app);
        }

        ActiveBlock::DisplayBlock => display_block::handle_display_block(key, app),
    }
    // todo: move this to active_display_block_handler
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
            app.active_block = ActiveBlock::Input;
            // todo: handle cases when exiting the Display_block.

            // app.search_results.selected_block = match app.search_results.selected_block {
            //     SearchResultBlock::AnimeSearch => SearchResultBlock::MangaSearch,
            //     SearchResultBlock::MangaSearch => SearchResultBlock::AnimeSearch,
            //     SearchResultBlock::Empty => SearchResultBlock::Empty,
            // };
        }
        // ActiveBlock::OptionStats => {
        //     app.set_current_route_state(Some(ActiveBlock::Anime), Some(ActiveBlock::Anime));
        // }
        _ => {}
    }
}
