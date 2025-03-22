use crate::app::App;
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
        _ => crate::handlers::handle_result_block(key, app),
    }
}
