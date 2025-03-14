use crate::app::{App, DISPLAY_COLUMN_NUMBER, DISPLAY_RAWS_NUMBER};
use crate::{app::SelectedSearchTab, event::Key};

use super::common;

pub fn handle_result_block(key: Key, app: &mut App) {
    let max = DISPLAY_COLUMN_NUMBER * DISPLAY_RAWS_NUMBER;
    match key {
        Key::Char('s') => match app.search_results.selected_tab {
            SelectedSearchTab::Anime => {
                app.search_results.selected_tab = SelectedSearchTab::Manga;
            }
            SelectedSearchTab::Manga => {
                app.search_results.selected_tab = SelectedSearchTab::Anime;
            }
        },
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

            if !((max - DISPLAY_COLUMN_NUMBER - 1)..(max - 1)).contains(&index) {
                index = (index + DISPLAY_COLUMN_NUMBER) % max
            }
            app.search_results.selected_display_card_index = Some(index);
        }

        // Key::Char('j') => {
        //     app.search_results.next();
        // }
        // Key::Char('k') => {
        //     app.search_results.previous();
        // }
        // Key::Char('h') => {
        //     app.active_display_block = ActiveDisplayBlock::Empty;
        // }
        // Key::Char('l') => {
        //     app.active_display_block = ActiveDisplayBlock::Empty;
        // }
        // Key::Char('q') => {
        //     app.active_display_block = ActiveDisplayBlock::Empty;
        // }
        // Key::Enter => {
        //     app.active_display_block = ActiveDisplayBlock::Empty;
        // }
        _ => {}
    }
}
