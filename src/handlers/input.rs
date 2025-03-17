use crate::app::{ActiveBlock, ActiveDisplayBlock, App};
use crate::event::Key;
use crate::network::IoEvent;
use std::convert::TryInto;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub fn handler(key: Key, app: &mut App) {
    match key {
        // Delete everything after the cursor including selected character
        Key::Ctrl('k') => {
            app.input.drain(app.input_idx..app.input.len());
        }

        // Delete everything before the cursor not including selected character
        Key::Ctrl('u') => {
            app.input.drain(..app.input_idx);
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }

        // Deletes everything in input
        Key::Ctrl('l') => {
            app.input = vec![];
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        // Delete word before cursor
        Key::Ctrl('w') => {
            if app.input_cursor_position == 0 {
                return;
            }
            let word_end = match app.input[..app.input_idx].iter().rposition(|&x| x != ' ') {
                Some(index) => index + 1,
                None => 0,
            };
            let word_start = match app.input[..word_end].iter().rposition(|&x| x == ' ') {
                Some(index) => index + 1,
                None => 0,
            };
            let deleted: String = app.input[word_start..app.input_idx].iter().collect();
            let deleted_len: u16 = UnicodeWidthStr::width(deleted.as_str()).try_into().unwrap();
            app.input.drain(word_start..app.input_idx);
            app.input_idx = word_start;
            app.input_cursor_position -= deleted_len;
        }

        // Move cursor to the end of the input
        Key::Ctrl('e') => {
            app.input_idx = app.input.len();
            let input_string: String = app.input.iter().collect();
            app.input_cursor_position = UnicodeWidthStr::width(input_string.as_str())
                .try_into()
                .unwrap();
        }

        // Move cursor to the start of the input
        Key::Ctrl('a') => {
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }

        // Move cursor to left
        Key::Left | Key::Ctrl('b') => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input[app.input_idx - 1];
                app.input_idx -= 1;
                app.input_cursor_position -= compute_character_width(last_c);
            }
        }

        // Move cursor to right
        Key::Right | Key::Ctrl('f') => {
            if app.input_idx < app.input.len() {
                let next_c = app.input[app.input_idx];
                app.input_idx += 1;
                app.input_cursor_position += compute_character_width(next_c);
            }
        }

        // end input mode
        Key::Esc => {
            app.active_block = ActiveBlock::DisplayBlock;
        }

        // Submit search query
        Key::Enter => {
            let input_str: String = app.input.iter().collect();

            // Don't do anything if there is no input
            if input_str.is_empty() {
                return;
            }
            app.active_display_block = ActiveDisplayBlock::Loading;
            app.active_block = ActiveBlock::DisplayBlock;
            app.display_block_title = format!("Search Results: {}", input_str).to_string();

            app.dispatch(IoEvent::GetSearchResults(input_str.clone()));

            // On searching for a track, clear the playlist selection
        }

        // add character to input
        Key::Char(c) => {
            app.input.insert(app.input_idx, c);
            app.input_idx += 1;
            app.input_cursor_position += compute_character_width(c);
        }

        // delete character before cursor
        Key::Backspace | Key::Ctrl('h') => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input.remove(app.input_idx - 1);
                app.input_idx -= 1;
                app.input_cursor_position -= compute_character_width(last_c);
            }
        }

        // ! not working ??
        Key::Delete | Key::Ctrl('d') => {
            if !app.input.is_empty() && app.input_idx < app.input.len() {
                app.input.remove(app.input_idx);
            }
        }

        _ => {}
    }
}

fn compute_character_width(character: char) -> u16 {
    UnicodeWidthChar::width(character)
        .unwrap()
        .try_into()
        .unwrap()
}
