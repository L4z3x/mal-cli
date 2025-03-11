use super::common;
use crate::app::{App, ANIME_OPTIONS, ANIME_OPTIONS_RANGE};

use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
    match key {
        // k if common::right_event(k) => common::handle_right_event(app),
        k if common::down_event(k) => {
            // calculate the next index in the list
            let next_index = ANIME_OPTIONS_RANGE.start
                + common::on_down_press(
                    &ANIME_OPTIONS,
                    Some(app.library.selected_index % (ANIME_OPTIONS.len())),
                );
            dbg!(next_index);
            app.library.selected_index = next_index;
        }
        k if common::up_event(k) => {
            // calculate the next index in the list
            let next_index = ANIME_OPTIONS_RANGE.start
                + common::on_up_press(
                    &ANIME_OPTIONS,
                    Some(app.library.selected_index % (ANIME_OPTIONS.len())),
                );
            dbg!(next_index);
            app.library.selected_index = next_index;
        }

        //? idk what this means ??
        // k if common::high_event(k) => {
        //     let next_index = common::on_high_press();
        //     app.library.selected_index = next_index;
        // }
        // k if common::middle_event(k) => {
        //     let next_index = common::on_middle_press(&ANIME_OPTIONS);
        //     app.library.selected_index = next_index;
        // }
        // k if common::low_event(k) => {
        //     let next_index = common::on_low_press(&ANIME_OPTIONS);
        //     app.library.selected_index = next_index
        // }
        // `library` should probably be an array of structs with enums rather than just using indexes
        // like this
        Key::Enter => match app.library.selected_index {
            // Seasonal
            0 => {}
            // Ranking
            1 => {}
            // Suggested
            2 => {}
            // This is required because Rust can't tell if this pattern in exhaustive
            _ => {} //# search is not neaded in the list.
                    // // Search
                    // 3 => {}
        },
        _ => (),
    };
}
