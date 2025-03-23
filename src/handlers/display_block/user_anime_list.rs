use crate::{
    app::{ActiveDisplayBlock, App, Data},
    event::Key,
    handlers::user::is_user_anime_list_data_available,
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

fn change_tab(app: &mut App) {
    // we need to checkif the next route is the same as the the next status route then we call load_next_route() else we call load_route()
    // this way we won't overide the next route if it's the same as the next status route
    let next_status = app.next_anime_list_status();
    app.anime_list_status = next_status.clone();

    let (is_data_available, is_next, index) = is_user_anime_list_data_available(app);

    if is_next {
        app.load_next_route();
        return;
    }
    if is_data_available {
        app.load_route(index.unwrap());
    } else {
        app.active_display_block = ActiveDisplayBlock::Loading;
        app.dispatch(IoEvent::GetAnimeList(next_status));
    }
}

fn open_popup(app: &mut App) {}
