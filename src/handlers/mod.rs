mod anime;
mod common;
mod display_block;
mod help;
mod input;
mod option;
mod user;
use crate::app::{
    ActiveBlock, ActiveDisplayBlock, App, Data, ANIME_OPTIONS_RANGE, DISPLAY_COLUMN_NUMBER,
    DISPLAY_RAWS_NUMBER, GENERAL_OPTIONS_RANGE, USER_OPTIONS_RANGE,
};
use crate::event::Key;

pub use input::handler as input_handler;

pub fn handle_app(key: Key, app: &mut App) {
    // First handle any global event and then move to block event
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
            if !app.popup {
                app.active_block = ActiveBlock::Input;
            }
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

pub fn handle_result_block(key: Key, app: &mut App) {
    let max = DISPLAY_COLUMN_NUMBER * DISPLAY_RAWS_NUMBER;
    match key {
        k if common::left_event(k) => {
            let mut index = app.search_results.selected_display_card_index.unwrap_or(0);
            let mut edges = Vec::new();
            for i in (0..max - 3).step_by(DISPLAY_COLUMN_NUMBER) {
                edges.push(i);
            }
            if !edges.contains(&index) {
                index = (index - 1) % max;
            }
            app.search_results.selected_display_card_index = Some(index);
        }
        k if common::right_event(k) => {
            let mut index = app.search_results.selected_display_card_index.unwrap_or(0);
            let mut edges = Vec::new();
            for i in (DISPLAY_COLUMN_NUMBER - 1..max).step_by(DISPLAY_COLUMN_NUMBER) {
                edges.push(i);
            }
            if !edges.contains(&index) {
                index = (index + 1) % max;
            }

            app.search_results.selected_display_card_index = Some(index);
        }
        k if common::up_event(k) => {
            let mut index = app.search_results.selected_display_card_index.unwrap_or(0);
            if !(0..3).contains(&index) {
                index = index - DISPLAY_COLUMN_NUMBER;
            }
            app.search_results.selected_display_card_index = Some(index);
        }
        k if common::down_event(k) => {
            let mut index = app.search_results.selected_display_card_index.unwrap_or(0);

            if !((max - DISPLAY_COLUMN_NUMBER)..(max - 1)).contains(&index) {
                index = (index + DISPLAY_COLUMN_NUMBER) % max
            }
            app.search_results.selected_display_card_index = Some(index);
        }
        Key::Enter => get_media_detail_page(app),
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

pub fn get_media_detail_page(app: &mut App) {}
