mod handle_confirm;
mod handle_d;
mod handle_enter;
mod handle_left_right;
mod handle_plus_minus;
mod handle_space;
mod handle_up_down;
pub mod input_playlist_handler;
mod insert_playlist_song;
mod play;

use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use handle_confirm::handle_confirm_popup;
use handle_d::handle_delete_key;
use handle_enter::enter_key;
use handle_left_right::{hande_left, hande_right};
use handle_plus_minus::{handle_minus, handle_plus};
use handle_space::handle_space_key;
use handle_up_down::{handle_key_down, handle_key_up};
use insert_playlist_song::handle_a;
use play::play_and_download;

use super::app::screen::Screen;

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Char('q') => {
            app.quit();
        }

        KeyCode::Esc => match app.screen_state {
            Screen::PopUpFileExplorer => {
                if !app.list_to_add.is_empty() {
                    app.list_to_add.clear();
                }
                app.screen_state = Screen::ListMusic;
            }
            Screen::HelpPopup => {
                app.screen_state = Screen::Playlist;
            }
            _ => {
                app.quit();
            }
        },
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Tab => {
            app.next_screen();
        }
        KeyCode::Left | KeyCode::Char('h') => {
            hande_left(app);
        }

        KeyCode::Right | KeyCode::Char('l') => {
            hande_right(app);
        }
        KeyCode::Char('+') => handle_plus(app),
        KeyCode::Char('-') => handle_minus(app),
        KeyCode::Up | KeyCode::Char('k') => handle_key_up(app),
        KeyCode::Down | KeyCode::Char('j') => handle_key_down(app),
        KeyCode::Enter => enter_key(app).await,
        KeyCode::Char('a') => handle_a(app),
        KeyCode::Char('d') => handle_delete_key(app),
        KeyCode::Char('r') => {
            app.mode_next();
        }
        KeyCode::Char('m') => {
            app.mute_toggle();
        }
        KeyCode::Char(' ') => {
            handle_space_key(app);
        }
        KeyCode::Char('p') => match app.screen_state {
            Screen::Playlist => {
                play_and_download(app).await;
            }
            _ => {}
        },

        KeyCode::Char('?') => match app.screen_state {
            Screen::HelpPopup => {
                app.screen_state = Screen::Playlist;
            }
            _ => {
                app.screen_state = Screen::HelpPopup;
            }
        },

        KeyCode::PageUp => {
            app.prev_music();
        }

        KeyCode::PageDown => {
            app.next_music();
        }

        //add another
        _ => {
            handle_confirm_popup(app, key_event);
        }
    }
    Ok(())
}
