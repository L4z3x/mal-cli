use crate::{
    app::{ActiveDisplayBlock, App},
    event::Key,
};
mod ranking;
mod result;
mod seasonal;
pub mod top_three;
mod user_anime_list;
mod user_manga_list;
pub fn handle_display_block(key: Key, app: &mut App) {
    // todo: add handlers for each.
    match &app.active_display_block {
        ActiveDisplayBlock::SearchResultBlock => result::handler(key, app),
        ActiveDisplayBlock::Suggestions => result::handler(key, app),
        ActiveDisplayBlock::Help => {}
        ActiveDisplayBlock::UserInfo => {}
        ActiveDisplayBlock::UserAnimeList => user_anime_list::handler(key, app),
        ActiveDisplayBlock::UserMangaList => user_manga_list::handler(key, app),
        ActiveDisplayBlock::Seasonal => seasonal::handler(key, app),
        ActiveDisplayBlock::AnimeRanking => ranking::handler(key, app),
        ActiveDisplayBlock::MangaRanking => ranking::handler(key, app),
        ActiveDisplayBlock::AnimeDetails => {}
        ActiveDisplayBlock::MangaDetails => {}
        ActiveDisplayBlock::Loading => {}
        ActiveDisplayBlock::Error => {}
        ActiveDisplayBlock::Empty => {
            //? add toggle color for fun
            //? hard one: add playing the banner and moving it around
        }
    }
}
