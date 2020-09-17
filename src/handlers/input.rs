use crate::app::{ActiveBlock, App, RouteId};
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
        Key::Ctrl('e') => {
            app.input_idx = app.input.len();
            let input_string: String = app.input.iter().collect();
            app.input_cursor_position = UnicodeWidthStr::width(input_string.as_str())
                .try_into()
                .unwrap();
        }
        Key::Ctrl('a') => {
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Left | Key::Ctrl('b') => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input[app.input_idx - 1];
                app.input_idx -= 1;
                app.input_cursor_position -= compute_character_width(last_c);
            }
        }
        Key::Right | Key::Ctrl('f') => {
            if app.input_idx < app.input.len() {
                let next_c = app.input[app.input_idx];
                app.input_idx += 1;
                app.input_cursor_position += compute_character_width(next_c);
            }
        }
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::BasicView));
        }
        Key::Enter => {
            let input_str: String = app.input.iter().collect();

            // Don't do anything if there is no input
            if input_str.is_empty() {
                return;
            }

            app.dispatch(IoEvent::GetSearchResults(input_str));

            // On searching for a track, clear the playlist selection
            app.push_navigation_stack(RouteId::Search, ActiveBlock::SearchResultBlock);
        }
        Key::Char(c) => {
            app.input.insert(app.input_idx, c);
            app.input_idx += 1;
            app.input_cursor_position += compute_character_width(c);
        }
        Key::Backspace | Key::Ctrl('h') => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input.remove(app.input_idx - 1);
                app.input_idx -= 1;
                app.input_cursor_position -= compute_character_width(last_c);
            }
        }
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
