use ratatui::{layout::Rect, Frame};

use crate::app::App;

use super::{draw_keys_bar, results};

pub fn draw_suggestions(f: &mut Frame, app: &App, chunk: Rect) {
    let chunk = draw_keys_bar(f, app, chunk);
    results::draw_anime_search_results(f, app, chunk);
}
