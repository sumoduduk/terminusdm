use crate::app::{screen::Screen, App};

pub fn handle_space_key(app: &mut App) {
    match &app.screen_state {
        Screen::PopUpFileExplorer => {
            app.push_song_from_explorer();
        }
        _ => {
            app.pause_toggle();
        }
    }
}
