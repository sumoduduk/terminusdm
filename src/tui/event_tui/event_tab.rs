use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

use crate::tui::app::tabs_state::SelectedTabs;
use crate::tui::app::AppTui;
use crate::tui::app::CurrentScreen;
use crate::tui::app::InputMode;
use tui_input::backend::crossterm::EventHandler;

pub fn handle_tabs_event(app: &mut AppTui) {
    match app.selected_tabs {
        SelectedTabs::DownloadFolder => todo!(),
        SelectedTabs::ConcurrentTotal => todo!(),
        SelectedTabs::ChunkSize => todo!(),
        SelectedTabs::Language => todo!(),
    }
}

fn handle_tabs_download(app: &mut AppTui) {
    match app.input_mode {
        InputMode::Normal => todo!(),
        InputMode::Editing => todo!(),
    }
}

fn handle_input_normal(app: &mut AppTui, key: KeyEvent) {
    match key.code {
        KeyCode::Tab => app.curr_screen = CurrentScreen::Main,
        KeyCode::Char('q') => app.curr_screen = CurrentScreen::Exiting,
        KeyCode::Char('e') => {
            todo!();
            app.input_mode = InputMode::Editing;
        }
        KeyCode::Char('l') | KeyCode::Right => app.next_tab(),
        KeyCode::Char('h') | KeyCode::Left => app.previous_tab(),
        _ => {}
    }
}

fn handle_input_insert(app: &mut AppTui, key: KeyEvent) {
    match key.code {
        KeyCode::Tab => {
            app.input_mode = InputMode::Normal;
            app.curr_screen = CurrentScreen::Main
        }
        KeyCode::Enter => {
            todo!();
        }
        _ => {
            app.tabs_content().handle_event(&Event::Key(key));
        }
    }
}
