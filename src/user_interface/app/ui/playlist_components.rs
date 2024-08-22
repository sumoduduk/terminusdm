use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, Padding, StatefulWidget},
};

use crate::app::{screen::Screen, App};

use super::SELECTED_STYLE;

pub fn render_playlist(app: &mut App, playlist_layout: Rect, buf: &mut Buffer) {
    let title_right = app.render_title_right(Screen::Playlist).unwrap_or_default();

    let playlist_block = Block::new()
        .title("Playlist")
        .title_bottom(title_right)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(0, 0, 1, 1))
        .border_style(app.get_border_color(Screen::Playlist));

    render_tab(app, playlist_layout, buf, playlist_block)
}

fn render_tab(app: &mut App, area: Rect, buf: &mut Buffer, block: Block) {
    let titles = app.playlist.list_playlist_titles();

    let list = List::new(titles)
        .block(block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    StatefulWidget::render(list, area, buf, &mut app.tabs_playlist);
}
