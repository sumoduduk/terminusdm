mod confirm_popup;
mod help_popup;
mod pop_up_plylist;

use confirm_popup::render_popup_confirm;
use help_popup::render_help;
use pop_up_plylist::render_popup_playlist;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    app::{screen::Screen, App},
    user_interface::cursor::AppState,
};

pub fn render_popup(app: &mut App, area: Rect, buf: &mut Buffer, app_state: &mut AppState) {
    match &app.screen_state {
        Screen::InsertPlaylist => render_popup_playlist(app, area, buf, &mut app_state.cursor),
        Screen::HelpPopup => render_help(app, area, buf),
        _ => {
            if app.pop_up_confirm {
                render_popup_confirm(app, area, buf);
            }
        }
    }
}
