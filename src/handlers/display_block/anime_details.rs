use crate::{app::App, event::Key, handlers::common};

pub fn handler(key: Key, app: &mut App) {
    match key {
        // k if k == app.app_config.keys.toggle => change_tab(app),
        // k if k == app.app_config.keys.open_popup => open_popup(app),
        k if common::up_event(k) => app.anime_details_info_scroll_view_state.scroll_up(),
        k if common::down_event(k) => app.anime_details_info_scroll_view_state.scroll_down(),
        _ => {}
    }
}
