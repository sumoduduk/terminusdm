use crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;

use crate::app::{screen::Screen, App, AppResult};

pub async fn handle_key_input_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc => app.screen_state = Screen::Playlist,
        KeyCode::Enter => {
            ui_save_playlit(app).await;
        }
        _ => {
            app.input_playlist.handle_event(&Event::Key(key_event));
        }
    }

    Ok(())
}

async fn ui_save_playlit(app: &mut App) {
    let playlist_input = app.input_playlist.value();

    if playlist_input.contains("youtube") && playlist_input.contains("playlist") {
        let _ = app.playlist.save_playlist(playlist_input).await;
    } else {
        let _ = app.playlist.save_local_playlist(playlist_input);
    }

    app.input_playlist.reset();
    app.tabs_playlist.select_last();
    app.screen_state = Screen::ListMusic;
}
