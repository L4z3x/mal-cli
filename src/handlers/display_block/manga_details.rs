use crate::{
    app::{ActiveMangaDetailBlock, App, DetailPopup},
    event::Key,
    handlers::common,
};

use super::anime_details::{get_user_status_index, handle_edit};

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if k == app.app_config.keys.toggle => change_tab(app),
        k if k == app.app_config.keys.open_popup => {
            if app.popup {
                handle_edit(app)
            } else {
                open_popup(app)
            }
        }
        k if common::down_event(k) => match app.active_manga_detail_block {
            ActiveMangaDetailBlock::SideInfo => {
                app.manga_details_info_scroll_view_state.scroll_down()
            }
            ActiveMangaDetailBlock::Synopsis => {
                app.manga_details_synopsys_scroll_view_state.scroll_down()
            }
            ActiveMangaDetailBlock::Chapters => {
                if app.popup && app.temp_popup_num != 0 {
                    app.temp_popup_num -= 1;
                }
            }
            ActiveMangaDetailBlock::AddToList => {
                if app.popup {
                    app.selected_popup_status = if app.selected_popup_status == 4 {
                        0
                    } else {
                        app.selected_popup_status + 1
                    }
                }
            }
            ActiveMangaDetailBlock::Rate => {
                if app.popup {
                    app.selected_popup_rate = if app.selected_popup_rate == 10 {
                        0
                    } else {
                        app.selected_popup_rate + 1
                    }
                }
            }
            ActiveMangaDetailBlock::Volumes => {
                if app.popup && app.temp_popup_num != 0 {
                    app.temp_popup_num -= 1;
                }
            }
        },
        k if common::up_event(k) => match app.active_manga_detail_block {
            ActiveMangaDetailBlock::SideInfo => {
                app.manga_details_info_scroll_view_state.scroll_up();
            }
            ActiveMangaDetailBlock::Synopsis => {
                app.manga_details_synopsys_scroll_view_state.scroll_up();
            }
            ActiveMangaDetailBlock::Chapters => {
                if app.popup {
                    let total_ch = app
                        .manga_details
                        .as_ref()
                        .unwrap()
                        .num_chapters
                        .unwrap_or(10000); // just to let the user update the number even if the total is unkonw just like in mal.
                    if total_ch == 0 || app.temp_popup_num as u64 != total_ch {
                        app.temp_popup_num += 1;
                    }
                }
            }
            ActiveMangaDetailBlock::AddToList => {
                if app.popup {
                    app.selected_popup_status = if app.selected_popup_status == 0 {
                        4
                    } else {
                        app.selected_popup_status - 1
                    }
                }
            }
            ActiveMangaDetailBlock::Rate => {
                if app.popup {
                    app.selected_popup_rate = if app.selected_popup_rate == 0 {
                        10
                    } else {
                        app.selected_popup_rate - 1
                    }
                }
            }
            ActiveMangaDetailBlock::Volumes => {
                if app.popup {
                    let total_vol = app
                        .manga_details
                        .as_ref()
                        .unwrap()
                        .num_volumes
                        .unwrap_or(10000); // just to let the user update the number even if the total is unkonw just like in mal.
                    if total_vol == 0 || app.temp_popup_num as u64 != total_vol {
                        app.temp_popup_num += 1;
                    }
                }
            }
        },
        k if common::right_event(k) => {
            if app.popup {
                return;
            }
            match app.active_manga_detail_block {
                ActiveMangaDetailBlock::AddToList => {
                    app.active_manga_detail_block = ActiveMangaDetailBlock::Rate;
                }
                ActiveMangaDetailBlock::Rate => {
                    app.active_manga_detail_block = ActiveMangaDetailBlock::Chapters;
                }
                ActiveMangaDetailBlock::Chapters => {
                    app.active_manga_detail_block = ActiveMangaDetailBlock::Volumes;
                }
                ActiveMangaDetailBlock::Volumes => {
                    app.active_manga_detail_block = ActiveMangaDetailBlock::AddToList;
                }
                _ => {}
            };
        }
        k if common::left_event(k) => {
            if app.popup {
                return;
            }
            match app.active_manga_detail_block {
                ActiveMangaDetailBlock::AddToList => {
                    app.active_manga_detail_block = ActiveMangaDetailBlock::Volumes;
                }
                ActiveMangaDetailBlock::Volumes => {
                    app.active_manga_detail_block = ActiveMangaDetailBlock::Chapters;
                }
                ActiveMangaDetailBlock::Chapters => {
                    app.active_manga_detail_block = ActiveMangaDetailBlock::Rate;
                }
                ActiveMangaDetailBlock::Rate => {
                    app.active_manga_detail_block = ActiveMangaDetailBlock::AddToList;
                }
                _ => {}
            }
        }
        k if k == Key::Enter || k == app.app_config.keys.open_popup => {
            if app.popup {
                handle_edit(app)
            } else {
                open_popup(app)
            }
        }
        _ => {}
    }
}

fn change_tab(app: &mut App) {
    if app.popup {
        return;
    }
    match app.active_manga_detail_block {
        ActiveMangaDetailBlock::AddToList => {
            app.active_manga_detail_block = ActiveMangaDetailBlock::Rate;
        }
        ActiveMangaDetailBlock::Rate => {
            app.active_manga_detail_block = ActiveMangaDetailBlock::Chapters;
        }
        ActiveMangaDetailBlock::Chapters => {
            app.active_manga_detail_block = ActiveMangaDetailBlock::Volumes;
        }
        ActiveMangaDetailBlock::Volumes => {
            app.active_manga_detail_block = ActiveMangaDetailBlock::SideInfo;
        }
        ActiveMangaDetailBlock::SideInfo => {
            app.active_manga_detail_block = ActiveMangaDetailBlock::Synopsis;
        }
        ActiveMangaDetailBlock::Synopsis => {
            app.active_manga_detail_block = ActiveMangaDetailBlock::AddToList;
        }
    }
}

fn open_popup(app: &mut App) {
    match app.active_manga_detail_block {
        ActiveMangaDetailBlock::AddToList => {
            app.active_detail_popup = DetailPopup::AddToList;
            app.selected_popup_status = app
                .manga_details
                .as_ref()
                .unwrap()
                .my_list_status
                .as_ref()
                .map_or(0, |list| {
                    get_user_status_index(list.status.to_string().as_str())
                });
            app.popup = true;
        }
        ActiveMangaDetailBlock::Rate => {
            app.active_detail_popup = DetailPopup::Rate;
            app.selected_popup_rate = app
                .manga_details
                .as_ref()
                .unwrap()
                .my_list_status
                .as_ref()
                .map_or(0, |list| list.score);
            app.popup = true;
        }
        ActiveMangaDetailBlock::Chapters => {
            app.active_detail_popup = DetailPopup::Chapters;
            app.temp_popup_num = app
                .manga_details
                .as_ref()
                .unwrap()
                .my_list_status
                .as_ref()
                .map_or(0, |list| list.num_chapters_read as u16);
            app.popup = true;
        }
        ActiveMangaDetailBlock::Volumes => {
            app.active_detail_popup = DetailPopup::Volumes;
            app.temp_popup_num = app
                .manga_details
                .as_ref()
                .unwrap()
                .my_list_status
                .as_ref()
                .map_or(0, |list| list.num_volumes_read as u16);
            app.popup = true;
        }
        _ => {}
    }
}
