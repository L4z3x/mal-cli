use crate::app::{ActiveDisplayBlock, App};
use crate::ui::{draw_error, draw_help_menu};
use ratatui::{layout::Rect, Frame};

pub fn draw_display_layout(f: &mut Frame, app: &App, layout_chunk: Rect) {
    let current_display_block = &app.active_display_block;
    match current_display_block {
        ActiveDisplayBlock::Empty => {
            // drow mal-cli
        }

        ActiveDisplayBlock::Help => {
            draw_help_menu(f, app);
        }

        ActiveDisplayBlock::AnimeRanking => {}

        ActiveDisplayBlock::MangaRanking => {}

        ActiveDisplayBlock::UserAnimeList => {}

        ActiveDisplayBlock::UserMangaList => {}

        ActiveDisplayBlock::UserInfo => {}

        ActiveDisplayBlock::SearchResultBlock => {}

        ActiveDisplayBlock::Seasonal => {}

        ActiveDisplayBlock::Error => {
            draw_error(f, app);
        }

        ActiveDisplayBlock::Loading => {}

        _ => {}
    }
}
