use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{
        block::Title, Block, BorderType, Borders, HighlightSpacing, List, Padding, StatefulWidget,
    },
};

use crate::app::{screen::Screen, App};

use super::SELECTED_STYLE;

pub fn render_music_list(app: &mut App, music_list_layout: Rect, buf: &mut Buffer) {
    let title_right = app
        .render_title_right(Screen::ListMusic)
        .unwrap_or_default();
    let music_block = Block::new()
        .title("Music List")
        .title(Title::from(title_right).alignment(Alignment::Right))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(0, 0, 1, 1))
        .border_style(app.get_border_color(Screen::ListMusic));

    let indx = app.tabs_playlist.selected();
    let music = app.list_playlist_music(indx).unwrap_or_default();

    let music_list = List::new(music)
        .block(music_block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    StatefulWidget::render(music_list, music_list_layout, buf, &mut app.music_list);
}
