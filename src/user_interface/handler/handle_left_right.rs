use crate::app::{screen::Screen, App};

pub fn hande_left(app: &mut App) {
    match &app.screen_state {
        Screen::PopUpFileExplorer => {
            let _ = app.file_explorer.go_back();
        }
        Screen::Playback => {
            app.button_prev();
        }
        Screen::Playlist => app.screen_state = Screen::ListMusic,

        Screen::ListMusic => app.screen_state = Screen::Playlist,
        _ => {
            app.seek_backward();
        }
    }
}

pub fn hande_right(app: &mut App) {
    match &app.screen_state {
        Screen::PopUpFileExplorer => {
            let _ = app.file_explorer.enter_dir();
        }

        Screen::Playback => {
            app.button_next();
        }

        Screen::Playlist => app.screen_state = Screen::ListMusic,

        Screen::ListMusic => app.screen_state = Screen::Playlist,
        _ => {
            app.seek_forward();
        }
    }
}
