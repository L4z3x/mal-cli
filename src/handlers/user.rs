use super::common;
use crate::app::{ActiveBlock, App, RouteId, USER_OPTIONS};

use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common::right_event(k) => common::handle_right_event(app),
        k if common::down_event(k) => {
            let next_index = common::on_down_press(&USER_OPTIONS, Some(app.library.selected_index));
            app.library.selected_index = next_index;
        }
        k if common::up_event(k) => {
            let next_index = common::on_up_press(&USER_OPTIONS, Some(app.library.selected_index));
            app.library.selected_index = next_index;
        }
        k if common::high_event(k) => {
            let next_index = common::on_high_press();
            app.library.selected_index = next_index;
        }
        k if common::middle_event(k) => {
            let next_index = common::on_middle_press(&USER_OPTIONS);
            app.library.selected_index = next_index;
        }
        k if common::low_event(k) => {
            let next_index = common::on_low_press(&USER_OPTIONS);
            app.library.selected_index = next_index
        }
        // `library` should probably be an array of structs with enums rather than just using indexes
        // like this
        Key::Enter => match app.library.selected_index {
            // Stats
            0 => {}
            // AnimeList
            1 => {}
            // MangaList
            2 => {}
            // This is required because Rust can't tell if this pattern in exhaustive
            _ => {}
        },
        _ => (),
    };
}
