use crate::app::{screen::Screen, App};

pub fn handle_a(app: &mut App) {
    match app.screen_state {
        Screen::Playlist => {
            app.screen_state = Screen::InsertPlaylist;
        }
        Screen::ListMusic => {
            if app.tabs_playlist.selected().is_some() {
                app.screen_state = Screen::PopUpFileExplorer;
            }
        }
        _ => {}
    }
}
