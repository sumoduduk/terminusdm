use crate::app::{screen::Screen, App};

pub fn handle_key_up(app: &mut App) {
    match app.screen_state {
        Screen::Playlist => {
            app.tabs_playlist.select_previous();
        }
        Screen::ListMusic => {
            app.music_list.select_previous();
        }
        Screen::PopUpFileExplorer => {
            app.file_explorer.scroll_up();
        }
        _ => {}
    }
}

pub fn handle_key_down(app: &mut App) {
    match app.screen_state {
        Screen::Playlist => {
            app.tabs_playlist.select_next();
        }
        Screen::ListMusic => {
            app.music_list.select_next();
        }

        Screen::PopUpFileExplorer => {
            app.file_explorer.scroll_down();
        }
        _ => {}
    }
}
