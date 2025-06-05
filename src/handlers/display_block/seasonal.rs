use chrono::Datelike;

use super::result::handle_result_block;
use crate::{
    api::model::Season,
    app::{ActiveDisplayBlock, App},
    event::Key,
    network::IoEvent,
};

use crate::handlers::common;

pub fn handler(key: Key, app: &mut App) {
    if app.popup {
        handle_popup(key, app);
    } else {
        match key {
            // Key::Enter => open anime detail),
            k if k == app.app_config.keys.toggle => app.popup = true,

            // Key::Char('s') => app.active_display_block = ActiveDisplayBlock::,
            _ => handle_result_block(key, app),
        }
    }
}

fn reload_seasonal(app: &mut App) {
    app.reset_result_index();
    app.active_display_block = ActiveDisplayBlock::Loading;
    app.popup = false;
    app.anime_season.anime_season.season = get_season(app);
    app.anime_season.anime_season.year = app.anime_season.selected_year as u64;
    app.dispatch(IoEvent::GetSeasonalAnime);
}

fn handle_popup(key: Key, app: &mut App) {
    let is_season_selected = app.anime_season.popup_season_highlight;
    match key {
        k if k == app.app_config.keys.toggle => {
            app.anime_season.popup_season_highlight = !is_season_selected;
        }

        k if common::down_event(k) => {
            if is_season_selected {
                app.anime_season.selected_season = (app.anime_season.selected_season + 1) % 4;
            } else if app.anime_season.selected_year > 1917 {
                // Ensure the selected year does not go below 1917, which is the last year available
                app.anime_season.selected_year -= 1;
            } else {
                app.anime_season.selected_year = 1917;
            }
        }

        k if common::up_event(k) => {
            if is_season_selected {
                if app.anime_season.selected_season == 0 {
                    app.anime_season.selected_season = 3;
                } else {
                    app.anime_season.selected_season = (app.anime_season.selected_season - 1) % 4;
                }
            } else if app.anime_season.selected_year < chrono::Utc::now().year_ce().1 as u16 {
                app.anime_season.selected_year += 1;
            } else {
                app.anime_season.selected_year = chrono::Utc::now().year_ce().1 as u16;
            }
        }

        Key::Enter => {
            app.popup = false;
            reload_seasonal(app);
        }

        _ => {}
    }
}

fn get_season(app: &App) -> Season {
    let season = app.anime_season.selected_season;
    match season {
        0 => Season::Winter,
        1 => Season::Spring,
        2 => Season::Summer,
        3 => Season::Fall,
        _ => panic!("Invalid season"),
    }
}
