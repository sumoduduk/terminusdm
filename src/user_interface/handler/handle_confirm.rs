use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{screen::Screen, App},
    playback::PlaybackEvent,
};

pub fn handle_y_key(app: &mut App) {
    match app.screen_state {
        Screen::Playlist => {
            if let Some(playlist_index) = app.tabs_playlist.selected() {
                app.playlist.delete_playlist(playlist_index);
            }
        }

        Screen::ListMusic => {
            if let Some(playlist_index) = app.tabs_playlist.selected() {
                if let Some(song_index) = app.music_list.selected() {
                    let sender = app.tx_playback.clone();
                    let res = sender.send(PlaybackEvent::DeleteTrack(song_index));

                    if res.is_ok() {
                        app.playlist.delete_song(playlist_index, song_index);
                    }
                }
            }
        }

        _ => {}
    }
}

pub fn handle_confirm_popup(app: &mut App, key_code: KeyEvent) {
    match key_code.code {
        KeyCode::Char('y') => {
            handle_y_key(app);
            app.pop_up_confirm = false;
        }

        KeyCode::Char('n') => {
            app.pop_up_confirm = false;
        }

        _ => {}
    }
}
