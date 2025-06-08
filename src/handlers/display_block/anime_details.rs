use crate::{
    api::{
        model::{UserAnimeListStatus, UserMangaListStatus, UserReadStatus, UserWatchStatus},
        UpdateUserAnimeListStatusQuery, UpdateUserMangaStatus,
    },
    app::{ActiveAnimeDetailBlock, ActiveDisplayBlock, ActiveMangaDetailBlock, App, DetailPopup},
    event::Key,
    handlers::common,
    network::IoEvent,
};

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if k == app.app_config.keys.toggle => change_tab(app),

        k if k == Key::Enter || k == app.app_config.keys.open_popup => {
            if app.popup {
                handle_edit(app)
            } else {
                open_popup(app)
            }
        }

        k if common::down_event(k) => match app.active_anime_detail_block {
            ActiveAnimeDetailBlock::SideInfo => {
                app.anime_details_info_scroll_view_state.scroll_down()
            }
            ActiveAnimeDetailBlock::Synopsis => {
                app.anime_details_synopsys_scroll_view_state.scroll_down()
            }
            ActiveAnimeDetailBlock::Episodes => {
                if app.temp_popup_num != 0 {
                    app.temp_popup_num -= 1;
                }
            }
            ActiveAnimeDetailBlock::AddToList => {
                if app.popup {
                    app.selected_popup_status = (app.selected_popup_status + 1) % 5;
                }
            }
            ActiveAnimeDetailBlock::Rate => {
                if app.popup {
                    app.selected_popup_rate = (app.selected_popup_rate + 1) % 11;
                }
            }
        },
        k if common::up_event(k) => match app.active_anime_detail_block {
            ActiveAnimeDetailBlock::SideInfo => {
                app.anime_details_info_scroll_view_state.scroll_up()
            }
            ActiveAnimeDetailBlock::Synopsis => {
                app.anime_details_synopsys_scroll_view_state.scroll_up()
            }
            ActiveAnimeDetailBlock::Episodes => {
                if app.popup {
                    let total_ep = app
                        .anime_details
                        .as_ref()
                        .unwrap()
                        .num_episodes
                        .unwrap_or(10000); // just to let the user update the number even if the total is unkonw just like in mal.
                    if total_ep == 0 || app.temp_popup_num as u64 != total_ep {
                        app.temp_popup_num += 1;
                    }
                }
            }
            ActiveAnimeDetailBlock::AddToList => {
                if app.popup {
                    app.selected_popup_status = (app.selected_popup_status + 4) % 5;
                }
            }
            ActiveAnimeDetailBlock::Rate => {
                if app.popup {
                    app.selected_popup_rate = (app.selected_popup_rate + 9) % 11;
                }
            }
        },
        k if common::right_event(k) && !app.popup => {
            app.active_anime_detail_block = match app.active_anime_detail_block {
                ActiveAnimeDetailBlock::AddToList => ActiveAnimeDetailBlock::Rate,
                ActiveAnimeDetailBlock::Rate => ActiveAnimeDetailBlock::Episodes,
                ActiveAnimeDetailBlock::Episodes => ActiveAnimeDetailBlock::AddToList,
                ActiveAnimeDetailBlock::SideInfo => ActiveAnimeDetailBlock::Episodes,
                ActiveAnimeDetailBlock::Synopsis => ActiveAnimeDetailBlock::SideInfo,
            }
        }
        k if common::left_event(k) && !app.popup => {
            app.active_anime_detail_block = match app.active_anime_detail_block {
                ActiveAnimeDetailBlock::AddToList => ActiveAnimeDetailBlock::Episodes,
                ActiveAnimeDetailBlock::Rate => ActiveAnimeDetailBlock::AddToList,
                ActiveAnimeDetailBlock::Episodes => ActiveAnimeDetailBlock::Rate,
                ActiveAnimeDetailBlock::SideInfo => ActiveAnimeDetailBlock::Synopsis,
                ActiveAnimeDetailBlock::Synopsis => ActiveAnimeDetailBlock::AddToList,
            }
        }
        _ => {}
    }
}

fn change_tab(app: &mut App) {
    if app.popup {
        return;
    }
    app.active_anime_detail_block = match app.active_anime_detail_block {
        ActiveAnimeDetailBlock::AddToList => ActiveAnimeDetailBlock::Rate,
        ActiveAnimeDetailBlock::Rate => ActiveAnimeDetailBlock::Episodes,
        ActiveAnimeDetailBlock::Episodes => ActiveAnimeDetailBlock::SideInfo,
        ActiveAnimeDetailBlock::SideInfo => ActiveAnimeDetailBlock::Synopsis,
        ActiveAnimeDetailBlock::Synopsis => ActiveAnimeDetailBlock::AddToList,
    }
}

fn open_popup(app: &mut App) {
    app.result_popup = false;
    match app.active_anime_detail_block {
        ActiveAnimeDetailBlock::AddToList => {
            app.active_detail_popup = DetailPopup::AddToList;
            app.selected_popup_status = app
                .anime_details
                .as_ref()
                .unwrap()
                .my_list_status
                .as_ref()
                .map_or(0, |list| {
                    get_user_status_index(list.status.to_string().as_str())
                });
            app.popup = true;
        }
        ActiveAnimeDetailBlock::Rate => {
            app.active_detail_popup = DetailPopup::Rate;
            app.selected_popup_rate = app
                .anime_details
                .as_ref()
                .unwrap()
                .my_list_status
                .as_ref()
                .map_or(0, |list| list.score);
            app.popup = true;
        }
        ActiveAnimeDetailBlock::Episodes => {
            app.active_detail_popup = DetailPopup::Episodes;
            app.temp_popup_num = app
                .anime_details
                .as_ref()
                .unwrap()
                .my_list_status
                .as_ref()
                .map_or(0, |list| list.num_episodes_watched as u16);
            app.popup = true;
        }
        _ => {}
    }
}

pub fn get_user_status_index(status: &str) -> u8 {
    match status {
        "watching" | "reading" => 0,
        "completed" => 1,
        "on_hold" => 2,
        "dropped" => 3,
        "plan_to_watch" | "plan_to_read" => 4,
        _ => 0,
    }
}

pub fn handle_edit(app: &mut App) {
    match app.active_display_block {
        ActiveDisplayBlock::AnimeDetails => {
            let my_list = &app.anime_details.as_ref().unwrap().my_list_status;
            let anime_update_query: Option<UpdateUserAnimeListStatusQuery> =
                match app.active_anime_detail_block {
                    ActiveAnimeDetailBlock::AddToList => {
                        let status = get_watch_status_from_index(app.selected_popup_status);
                        if my_list.is_some() {
                            // if selected the current status do nothing
                            if my_list.as_ref().unwrap().status == status {
                                app.popup = false;
                                app.result_popup = false;
                                None
                            } else {
                                Some(user_list_to_anime_query(
                                    my_list.as_ref().unwrap(),
                                    Some(status),
                                    None,
                                    None,
                                ))
                            }
                        } else {
                            Some(anime_query_with_one_field(Some(status), None, None))
                        }
                    }

                    ActiveAnimeDetailBlock::Episodes => {
                        let ep_num = app.temp_popup_num;
                        if my_list.is_some() {
                            // if selected the current status do nothing
                            if my_list.as_ref().unwrap().num_episodes_watched == ep_num as u64 {
                                app.popup = false;
                                None
                            } else {
                                Some(user_list_to_anime_query(
                                    my_list.as_ref().unwrap(),
                                    None,
                                    None,
                                    Some(ep_num as u64),
                                ))
                            }
                        } else {
                            Some(anime_query_with_one_field(None, None, Some(ep_num as u64)))
                        }
                    }

                    ActiveAnimeDetailBlock::Rate => {
                        let score = app.selected_popup_rate;

                        if my_list.is_some() {
                            // if selected the current status do nothing
                            if my_list.as_ref().unwrap().score == score {
                                app.popup = false;
                                None
                            } else {
                                Some(user_list_to_anime_query(
                                    my_list.as_ref().unwrap(),
                                    None,
                                    Some(score),
                                    None,
                                ))
                            }
                        } else {
                            Some(anime_query_with_one_field(None, Some(score), None))
                        }
                    }

                    _ => Some(anime_query_with_one_field(None, None, None)),
                };
            if anime_update_query.is_none() {
                return;
            }
            let anime_id = app.anime_details.as_ref().unwrap().id;
            app.dispatch(IoEvent::UpdateAnimeListStatus(
                anime_id,
                anime_update_query.unwrap(),
            ));
            app.popup_is_loading = true;
            app.result_popup = true;
        }

        ActiveDisplayBlock::MangaDetails => {
            let my_list = &app.manga_details.as_ref().unwrap().my_list_status;
            let manga_update_query: Option<UpdateUserMangaStatus> =
                match app.active_manga_detail_block {
                    ActiveMangaDetailBlock::AddToList => {
                        let status = get_read_status_from_index(app.selected_popup_status);
                        if my_list.is_some() {
                            if my_list.as_ref().unwrap().status == status {
                                app.result_popup = false;
                                app.popup = false;
                                None
                            } else {
                                Some(user_list_to_manga_query(
                                    my_list.as_ref().unwrap(),
                                    Some(status),
                                    None,
                                    None,
                                    None,
                                ))
                            }
                        } else {
                            // if there isn't a list status we create one
                            Some(manga_query_with_one_field(Some(status), None, None, None))
                        }
                    }

                    ActiveMangaDetailBlock::Chapters => {
                        let ch_num = app.temp_popup_num;

                        if my_list.is_some() {
                            if my_list.as_ref().unwrap().num_chapters_read == ch_num as u64 {
                                app.popup = false;
                                app.result_popup = false;
                                None
                            } else {
                                Some(user_list_to_manga_query(
                                    my_list.as_ref().unwrap(),
                                    None,
                                    None,
                                    Some(ch_num as u64),
                                    None,
                                ))
                            }
                        } else {
                            Some(manga_query_with_one_field(
                                None,
                                None,
                                Some(ch_num as u64),
                                None,
                            ))
                        }
                    }

                    ActiveMangaDetailBlock::Rate => {
                        let score = app.selected_popup_rate;

                        if my_list.is_some() {
                            if my_list.as_ref().unwrap().score == score {
                                app.popup = false;
                                app.result_popup = false;
                                None
                            } else {
                                Some(user_list_to_manga_query(
                                    my_list.as_ref().unwrap(),
                                    None,
                                    Some(score),
                                    None,
                                    None,
                                ))
                            }
                        } else {
                            Some(manga_query_with_one_field(None, Some(score), None, None))
                        }
                    }

                    ActiveMangaDetailBlock::Volumes => {
                        let vol_num = app.temp_popup_num;

                        if my_list.is_some() {
                            if my_list.as_ref().unwrap().num_volumes_read == vol_num as u64 {
                                app.popup = false;
                                app.result_popup = false;
                                None
                            } else {
                                Some(user_list_to_manga_query(
                                    my_list.as_ref().unwrap(),
                                    None,
                                    None,
                                    None,
                                    Some(vol_num as u64),
                                ))
                            }
                        } else {
                            Some(manga_query_with_one_field(
                                None,
                                None,
                                None,
                                Some(vol_num as u64),
                            ))
                        }
                    }
                    _ => None,
                };
            if manga_update_query.is_none() {
                return;
            }

            let manga_id = app.manga_details.as_ref().unwrap().id;
            app.dispatch(IoEvent::UpdateMangaListStatus(
                manga_id,
                manga_update_query.unwrap(),
            ));
            app.result_popup = true;
            app.popup_is_loading = true;
        }

        _ => {}
    }
}

fn get_watch_status_from_index(index: u8) -> UserWatchStatus {
    match index {
        0 => UserWatchStatus::Watching,

        1 => UserWatchStatus::Completed,

        2 => UserWatchStatus::OnHold,

        3 => UserWatchStatus::Dropped,

        4 => UserWatchStatus::PlanToWatch,

        _ => UserWatchStatus::Other("None".to_string()),
    }
}

fn get_read_status_from_index(index: u8) -> UserReadStatus {
    match index {
        0 => UserReadStatus::Reading,
        1 => UserReadStatus::Completed,
        2 => UserReadStatus::OnHold,
        3 => UserReadStatus::Dropped,
        4 => UserReadStatus::PlanToRead,
        _ => UserReadStatus::Other("None".to_string()),
    }
}

pub fn user_list_to_anime_query(
    my_list: &UserAnimeListStatus,
    status: Option<UserWatchStatus>,
    score: Option<u8>,
    ep: Option<u64>,
) -> UpdateUserAnimeListStatusQuery {
    UpdateUserAnimeListStatusQuery {
        status: status.or_else(|| Some(my_list.status.clone())),
        score: score.or(Some(my_list.score)),
        num_watched_episodes: ep.or(Some(my_list.num_episodes_watched)),
        comments: my_list.comments.clone(),
        is_rewatching: Some(my_list.is_rewatching),
        num_times_rewatched: my_list.num_times_rewatched,
        priority: my_list.priority,
        rewatch_value: my_list.rewatch_value,
        tags: my_list.tags.clone().map(|v| v.join(",")),
    }
}

pub fn user_list_to_manga_query(
    my_list: &UserMangaListStatus,
    status: Option<UserReadStatus>,
    score: Option<u8>,
    ch_num: Option<u64>,
    vol_num: Option<u64>,
) -> UpdateUserMangaStatus {
    UpdateUserMangaStatus {
        status: status.or_else(|| Some(my_list.status.clone())),
        score: score.or(Some(my_list.score)),
        num_chapters_read: ch_num.or(Some(my_list.num_chapters_read)),
        num_volumes_read: vol_num.or(Some(my_list.num_volumes_read)),
        priority: my_list.priority,
        reread_value: my_list.reread_value,
        tags: my_list.tags.clone().map(|v| v.join(",")),
        comments: my_list.comments.clone(),
        is_rereading: Some(my_list.is_rereading),
        num_times_reread: my_list.num_times_reread,
    }
}

pub fn manga_query_with_one_field(
    status: Option<UserReadStatus>,
    score: Option<u8>,
    num_chapters_read: Option<u64>,
    num_volumes_read: Option<u64>,
) -> UpdateUserMangaStatus {
    UpdateUserMangaStatus {
        status,
        score,
        num_volumes_read,
        num_chapters_read,
        num_times_reread: None,
        comments: None,
        reread_value: None,
        is_rereading: None,
        priority: None,
        tags: None,
    }
}

pub fn anime_query_with_one_field(
    status: Option<UserWatchStatus>,
    score: Option<u8>,
    num_watched_episodes: Option<u64>,
) -> UpdateUserAnimeListStatusQuery {
    UpdateUserAnimeListStatusQuery {
        status,
        is_rewatching: None,
        score,
        num_watched_episodes,
        priority: None,
        num_times_rewatched: None,
        rewatch_value: None,
        tags: None,
        comments: None,
    }
}
