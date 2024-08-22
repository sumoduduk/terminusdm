use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Clear, Paragraph, Widget},
};

use crate::{
    app::{centered_rect, App},
    user_interface::cursor::Cursor,
};

pub fn render_popup_playlist(app: &App, area: Rect, buf: &mut Buffer, cursor: &mut Cursor) {
    let pop_up_area = centered_rect(60, 10, area);

    Clear.render(pop_up_area, buf);
    let block = Block::bordered().title("Insert Playlist");

    let width = pop_up_area.width.max(3) - 3;
    let scroll = app.input_playlist.visual_scroll(width as usize);

    Paragraph::new(app.input_playlist.value())
        .scroll((0, scroll as u16))
        .block(block)
        .render(pop_up_area, buf);

    cursor.set(
        pop_up_area.x + ((app.input_playlist.visual_cursor()).max(scroll) - scroll) as u16 + 1,
        pop_up_area.y + 1,
    )
}
