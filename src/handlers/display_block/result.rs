use crate::app::{ActiveDisplayBlock, App, DISPLAY_COLUMN_NUMBER, DISPLAY_RAWS_NUMBER};
use crate::handlers::{common, get_media_detail_page};
use crate::ui::get_end_card_index;
use crate::{app::SelectedSearchTab, event::Key};
pub fn handler(key: Key, app: &mut App) {
    match key {
        k if k == app.app_config.keys.toggle => match app.search_results.selected_tab {
            SelectedSearchTab::Anime => {
                app.search_results.selected_tab = SelectedSearchTab::Manga;
            }

            SelectedSearchTab::Manga => {
                app.search_results.selected_tab = SelectedSearchTab::Anime;
            }
        },
        _ => handle_result_block(key, app),
    }
}

pub fn handle_result_block(key: Key, app: &mut App) {
    //? max is the last index of the current card list
    let max = get_end_card_index(app) - app.start_card_list_index as usize;
    match key {
        k if common::left_event(k) => {
            let mut index = app.search_results.selected_display_card_index.unwrap_or(0);
            let mut edges = Vec::new();
            for i in (0..=max - 2).step_by(DISPLAY_COLUMN_NUMBER) {
                edges.push(i);
            }
            if !edges.contains(&index) {
                index = (index - 1) % (max + 1);
            }
            app.search_results.selected_display_card_index = Some(index);
        }

        k if common::right_event(k) => {
            let mut index = app.search_results.selected_display_card_index.unwrap_or(0);
            let mut edges = Vec::new();
            for i in (DISPLAY_COLUMN_NUMBER - 1..=max).step_by(DISPLAY_COLUMN_NUMBER) {
                edges.push(i);
            }
            if !edges.contains(&index) {
                index = (index + 1) % (max + 1);
            }

            app.search_results.selected_display_card_index = Some(index);
        }

        k if common::up_event(k) => {
            let mut index = app.search_results.selected_display_card_index.unwrap_or(0);
            if (0..3).contains(&index) {
                scroll_results_up(app);
            } else if !(0..3).contains(&index) {
                index = index - DISPLAY_COLUMN_NUMBER;
                app.search_results.selected_display_card_index = Some(index);
            }
        }

        k if common::down_event(k) => {
            let mut index = app.search_results.selected_display_card_index.unwrap_or(0);
            if ((DISPLAY_COLUMN_NUMBER * DISPLAY_RAWS_NUMBER - 3)
                ..(DISPLAY_COLUMN_NUMBER * DISPLAY_RAWS_NUMBER))
                .contains(&index)
            {
                scroll_results_down(app);
            } else {
                index = index + DISPLAY_COLUMN_NUMBER;
                if index > max {
                    index = max; // Ensure we don't go out of bounds
                }
                app.search_results.selected_display_card_index = Some(index);
            }
        }

        Key::Enter => get_media_detail_page(app),
        _ => {}
    }
}

fn scroll_results_up(app: &mut App) {
    app.start_card_list_index = app
        .start_card_list_index
        .saturating_sub(DISPLAY_COLUMN_NUMBER as u16);
}

fn scroll_results_down(app: &mut App) {
    let data_length = get_data_length(app) as usize;
    // Ensure that the end index does not exceed the data length
    // If it does, reset the the index to the start

    // keep the position of the index
    // println!(
    //     "s:[{}],e:[{}],d: [{}]",
    //     app.start_card_list_index,
    //     get_end_card_index(app),
    //     data_length
    // );
    if get_end_card_index(app) + DISPLAY_COLUMN_NUMBER > data_length - 1 {
        app.start_card_list_index =
            (data_length - DISPLAY_COLUMN_NUMBER * DISPLAY_RAWS_NUMBER) as u16;
    } else if get_end_card_index(app) > data_length - 1 {
        let index_positoin = app
            .search_results
            .selected_display_card_index
            .as_ref()
            .unwrap()
            % DISPLAY_COLUMN_NUMBER as usize;
        app.search_results.selected_display_card_index = Some(index_positoin);

        app.start_card_list_index = 0;
    } else {
        // If the end index is within bounds, increment the indixes
        app.start_card_list_index = app.start_card_list_index + DISPLAY_COLUMN_NUMBER as u16
    }
}

pub fn get_data_length(app: &App) -> u16 {
    let data_length = match app.active_display_block {
        ActiveDisplayBlock::SearchResultBlock => match app.search_results.selected_tab {
            SelectedSearchTab::Manga => app.search_results.manga.as_ref().unwrap().data.len(),
            SelectedSearchTab::Anime => app.search_results.anime.as_ref().unwrap().data.len(),
        },
        ActiveDisplayBlock::MangaRanking => app.manga_ranking_data.as_ref().unwrap().data.len(),
        ActiveDisplayBlock::AnimeRanking => app.anime_ranking_data.as_ref().unwrap().data.len(),
        _ => app.search_results.anime.as_ref().unwrap().data.len(),
    };
    data_length as u16
}
