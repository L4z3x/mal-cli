use super::common;
use crate::app::{App, GENERAL_OPTIONS, GENERAL_OPTIONS_RANGE};
use crate::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common::down_event(k) => {
            // calculate the next index in the list
            let next_index = GENERAL_OPTIONS_RANGE.start
                + common::on_down_press(
                    &GENERAL_OPTIONS,
                    Some(app.library.selected_index % GENERAL_OPTIONS_RANGE.len()),
                );
            app.library.selected_index = next_index;
        }

        k if common::up_event(k) => {
            // calculate the next index in the list
            let next_index = GENERAL_OPTIONS_RANGE.start
                + common::on_up_press(
                    &GENERAL_OPTIONS,
                    Some(app.library.selected_index % GENERAL_OPTIONS_RANGE.len()),
                );

            app.library.selected_index = next_index;
        }

        Key::Enter => {
            match app.library.selected_index {
                // Help
                6 => {}
                // About
                7 => {}
                // Quit
                8 => {}

                _ => {}
            };
            app.library.selected_index = 9;
        }
        _ => (),
    };
}
