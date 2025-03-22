use crate::{
    app::{ActiveDisplayBlock, App},
    event::Key,
    handlers::user::is_user_manga_list_data_available,
    network::IoEvent,
};

use super::result;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if k == app.app_config.keys.toggle => change_tab(app),
        k if k == app.app_config.keys.open_popup => open_popup(app),
        _ => result::handler(key, app),
    }
}

fn open_popup(app: &mut App) {}

fn change_tab(app: &mut App) {
    let next_status = app.next_anime_list_status();
    app.anime_list_status = next_status.clone();
    let is_data_available = is_user_manga_list_data_available(app);
    if is_data_available.1 {
        app.load_next_route();
        return;
    }
    if is_data_available.0 {
        app.load_route(is_data_available.2.unwrap() as usize);
    } else {
        app.active_display_block = ActiveDisplayBlock::Loading;
        app.dispatch(IoEvent::GetAnimeList(next_status));
    }
}
