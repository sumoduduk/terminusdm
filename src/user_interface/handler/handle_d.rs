use crate::app::{screen::Screen, App};

pub fn handle_delete_key(app: &mut App) {
    match app.screen_state {
        Screen::Playlist | Screen::ListMusic => {
            app.pop_up_confirm = true;
        }

        _ => {}
    }
}
