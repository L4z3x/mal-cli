use crate::{
    api::{model::UserWatchStatus, UpdateUserAnimeListStatusQuery},
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
                    app.selected_popup_status = if app.selected_popup_status == 5 {
                        0
                    } else {
                        app.selected_popup_status + 1
                    }
                }
            }
            ActiveAnimeDetailBlock::Rate => {
                if app.popup {
                    app.selected_popup_rate = if app.selected_popup_rate == 10 {
                        0
                    } else {
                        app.selected_popup_rate + 1
                    }
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
                        .unwrap_or(10000); //? is this the right move ? , we should inspect this later.
                    if app.temp_popup_num as u64 != total_ep {
                        app.temp_popup_num += 1;
                    }
                }
            }
            ActiveAnimeDetailBlock::AddToList => {
                if app.popup {
                    app.selected_popup_status = if app.selected_popup_status == 0 {
                        5
                    } else {
                        app.selected_popup_status - 1
                    }
                }
            }
            ActiveAnimeDetailBlock::Rate => {
                if app.popup {
                    app.selected_popup_rate = if app.selected_popup_rate == 0 {
                        10
                    } else {
                        app.selected_popup_rate - 1
                    }
                }
            }
        },
        k if common::right_event(k) => {
            if app.popup {
                return;
            }
            match app.active_anime_detail_block {
                ActiveAnimeDetailBlock::AddToList => {
                    app.active_anime_detail_block = ActiveAnimeDetailBlock::Rate;
                }
                ActiveAnimeDetailBlock::Rate => {
                    app.active_anime_detail_block = ActiveAnimeDetailBlock::Episodes;
                }
                ActiveAnimeDetailBlock::Episodes => {
                    app.active_anime_detail_block = ActiveAnimeDetailBlock::AddToList;
                }
                _ => {}
            }
        }
        k if common::left_event(k) => {
            if app.popup {
                return;
            }
            match app.active_anime_detail_block {
                ActiveAnimeDetailBlock::AddToList => {
                    app.active_anime_detail_block = ActiveAnimeDetailBlock::Episodes;
                }
                ActiveAnimeDetailBlock::Rate => {
                    app.active_anime_detail_block = ActiveAnimeDetailBlock::AddToList;
                }
                ActiveAnimeDetailBlock::Episodes => {
                    app.active_anime_detail_block = ActiveAnimeDetailBlock::Rate;
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn change_tab(app: &mut App) {
    if app.popup {
        return;
    }
    match app.active_anime_detail_block {
        ActiveAnimeDetailBlock::AddToList => {
            app.active_anime_detail_block = ActiveAnimeDetailBlock::Rate;
        }
        ActiveAnimeDetailBlock::Rate => {
            app.active_anime_detail_block = ActiveAnimeDetailBlock::Episodes;
        }
        ActiveAnimeDetailBlock::Episodes => {
            app.active_anime_detail_block = ActiveAnimeDetailBlock::SideInfo;
        }
        ActiveAnimeDetailBlock::SideInfo => {
            app.active_anime_detail_block = ActiveAnimeDetailBlock::Synopsis;
        }
        ActiveAnimeDetailBlock::Synopsis => {
            app.active_anime_detail_block = ActiveAnimeDetailBlock::AddToList;
        }
    }
}

fn open_popup(app: &mut App) {
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
        ActiveDisplayBlock::AnimeDetails => match app.active_anime_detail_block {
            ActiveAnimeDetailBlock::AddToList => {
                let status = get_watch_status_from_index(app.selected_popup_status);
                let current_status = app
                    .anime_details
                    .as_ref()
                    .unwrap()
                    .my_list_status
                    .as_ref()
                    .map_or(None, |l| Some(l.status.clone()));

                // if selected the current status do nothing
                if current_status.is_some() {
                    if current_status.unwrap() == status {
                        app.popup = false;
                        return;
                    }
                }

                let status = Some(status);
                let status_query = UpdateUserAnimeListStatusQuery {
                    status,
                    score: None,
                    comments: None,
                    is_rewatching: None,
                    num_times_rewatched: None,
                    num_watched_episodes: None,
                    priority: None,
                    rewatch_value: None,
                    tags: None,
                };

                let anime_id = app.anime_details.as_ref().unwrap().id;
                app.dispatch(IoEvent::UpdateAnimeListStatus(anime_id, status_query));

                // TODO: handle displaying the success message, maybe we do a loading popup screen until we get the response
                // TODO: let's worry with submitting the request first
                app.popup = false;
            }
            ActiveAnimeDetailBlock::Episodes => {}
            ActiveAnimeDetailBlock::Rate => {}
            _ => {}
        },
        ActiveDisplayBlock::MangaDetails => match app.active_manga_detail_block {
            ActiveMangaDetailBlock::AddToList => {}
            ActiveMangaDetailBlock::Chapters => {}
            ActiveMangaDetailBlock::Rate => {}
            ActiveMangaDetailBlock::Volumes => {}
            _ => {}
        },
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
