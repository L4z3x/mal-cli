use super::common;
use crate::app::{ActiveDisplayBlock, App, Data, USER_OPTIONS, USER_OPTIONS_RANGE};

use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common::down_event(k) => {
            let next_index = USER_OPTIONS_RANGE.start
                + common::on_down_press(
                    &USER_OPTIONS,
                    Some(app.library.selected_index % USER_OPTIONS_RANGE.len()),
                );
            app.library.selected_index = next_index;
        }
        k if common::up_event(k) => {
            let next_index = USER_OPTIONS_RANGE.start
                + common::on_up_press(
                    &USER_OPTIONS,
                    Some(app.library.selected_index % USER_OPTIONS_RANGE.len()),
                );
            app.library.selected_index = next_index;
        }

        Key::Enter => {
            match app.library.selected_index {
                // profile
                3 => get_user_profile(app),
                // animeList
                4 => get_user_anime_list(app),
                // mangaList
                5 => get_user_manga_list(app),
                // This is required because Rust can't tell if this pattern in exhaustive
                _ => {}
            };
            app.library.selected_index = 9;
        }
        _ => (),
    };
}

fn get_user_anime_list(app: &mut App) {
    let (is_data_available, is_next, index) = is_user_anime_list_data_available(app);
    if is_next {
        app.load_next_route();
        return;
    }
    if is_data_available {
        app.load_route(index.unwrap());
    } else {
        app.active_display_block = ActiveDisplayBlock::Loading;
        app.dispatch(IoEvent::GetAnimeList(app.anime_list_status.clone()));
    }
}

fn get_user_manga_list(app: &mut App) {
    let (is_data_available, is_next, index) = is_user_manga_list_data_available(app);
    if is_next {
        app.load_next_route();
        return;
    }
    if is_data_available {
        app.load_route(index.unwrap());
    } else {
        app.active_display_block = ActiveDisplayBlock::Loading;
        app.dispatch(IoEvent::GetMangaList(app.manga_list_status.clone()));
    }
}

fn get_user_profile(app: &mut App) {
    let (is_data_available, is_next, index) = is_user_profile_data_available(app);
    if is_next {
        app.load_next_route();
        return;
    }
    if is_data_available {
        app.load_route(index.unwrap());
    } else {
        app.active_display_block = ActiveDisplayBlock::Loading;
        app.dispatch(IoEvent::GetUserInfo);
    }
}

pub fn is_user_anime_list_data_available(app: &App) -> (bool, bool, Option<u16>) {
    for i in 0..(app.navigator.history.len()) {
        let id: u16 = app.navigator.history[i];
        if app.navigator.data[&id].block == ActiveDisplayBlock::UserAnimeList
            && app.navigator.data[&id].data.is_some()
        {
            if let Data::UserAnimeList(d) = app.navigator.data[&id].data.as_ref().unwrap() {
                if d.status == app.anime_list_status {
                    let is_next = app.navigator.index + 1 == i;
                    return (true, is_next, Some(id));
                }
            }
        }
    }
    (false, false, None)
}

pub fn is_user_manga_list_data_available(app: &App) -> (bool, bool, Option<u16>) {
    for i in 0..(app.navigator.history.len()) {
        let id = app.navigator.history[i];
        if app.navigator.data[&id].block == ActiveDisplayBlock::UserMangaList
            && app.navigator.data[&id].data.is_some()
        {
            if let Data::UserMangaList(d) = app.navigator.data[&id].data.as_ref().unwrap() {
                if d.status == app.manga_list_status {
                    let is_next = app.navigator.index + 1 == i;
                    return (true, is_next, Some(id));
                }
            }
        }
    }
    (false, false, None)
}

fn is_user_profile_data_available(app: &App) -> (bool, bool, Option<u16>) {
    for i in 0..(app.navigator.history.len()) {
        let id = app.navigator.history[i];
        if app.navigator.data[&id].block == ActiveDisplayBlock::UserInfo
            && app.navigator.data[&id].data.is_some()
        {
            let is_next = app.navigator.index + 1 == i;
            return (true, is_next, Some(id));
        }
    }
    (false, false, None)
}
