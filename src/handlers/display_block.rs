use std::u16;

use super::{result, seasonal};
use crate::{
    app::{ActiveDisplayBlock, App},
    event::Key,
};

pub fn handle_display_block(key: Key, app: &mut App) {
    // todo: add handlers for each.
    match &app.active_display_block {
        ActiveDisplayBlock::SearchResultBlock => {
            result::handler(key, app);
        }
        ActiveDisplayBlock::Help => {}
        ActiveDisplayBlock::UserInfo => {}
        ActiveDisplayBlock::UserAnimeList => {}
        ActiveDisplayBlock::UserMangaList => {}
        ActiveDisplayBlock::Suggestions => {}
        ActiveDisplayBlock::Seasonal => {
            seasonal::handler(key, app);
        }
        ActiveDisplayBlock::AnimeRanking => {}
        ActiveDisplayBlock::MangaRanking => {}
        ActiveDisplayBlock::Loading => {}
        ActiveDisplayBlock::Error => {}
        ActiveDisplayBlock::Empty => {
            //? add toggle color for fun
            //? hard one: add playing the banner and moving it around
        }
    }
}
