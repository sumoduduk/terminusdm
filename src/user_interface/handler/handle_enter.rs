use crate::{
    app::{screen::Screen, widget_playback_buttons::SelectedButton, App},
    playback::PlaybackEvent,
};

use super::play::play_and_download;

pub async fn enter_key(app: &mut App) {
    match app.screen_state {
        Screen::Playlist => {
            app.screen_state = Screen::ListMusic;
        }
        Screen::PopUpFileExplorer => {
            app.save_song_to_playlist();
            app.screen_state = Screen::ListMusic;
        }
        Screen::ListMusic => {
            let Some(song_id) = app.music_list.selected() else {
                return;
            };

            let sender = app.tx_playback.clone();
            let _ = sender.send(PlaybackEvent::TrackPlay(song_id));

            let sender2 = app.tx_playback.clone();

            match app.get_now_playing_id() {
                Some(id) => {
                    if let Some(index) = app.tabs_playlist.selected() {
                        if id != index {
                            play_and_download(app).await;
                        } else {
                            let _ = sender2.send(PlaybackEvent::Forward);
                        }
                    }
                }
                None => {
                    play_and_download(app).await;
                }
            }
        }

        Screen::Playback => match app.playback_button {
            SelectedButton::Previous => {
                app.prev_music();
            }
            SelectedButton::Rewind => {
                app.seek_backward();
            }
            SelectedButton::Play => {
                app.pause_toggle();
            }
            SelectedButton::Forward => {
                app.seek_forward();
            }
            SelectedButton::Next => {
                app.next_music();
            }
        },
        _ => {}
    }
}
