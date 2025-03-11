use crate::{
    app::{ActiveDisplayBlock, App},
    event::Key,
};

pub fn handle_display_block(key: Key, app: &mut App) {
    // todo: add handlers for each.
    match &app.active_display_block {
        ActiveDisplayBlock::SearchResultBlock => {}
        ActiveDisplayBlock::Help => {}
        ActiveDisplayBlock::UserInfo => {}
        ActiveDisplayBlock::UserAnimeList => {}
        ActiveDisplayBlock::UserMangaList => {}
        ActiveDisplayBlock::Suggestions => {}
        ActiveDisplayBlock::Seasonal => {}
        ActiveDisplayBlock::AnimeRanking => {}
        ActiveDisplayBlock::MangaRanking => {}
        ActiveDisplayBlock::Loading => {}
        ActiveDisplayBlock::Error => {}
        ActiveDisplayBlock::Empty => {}
    }
}
