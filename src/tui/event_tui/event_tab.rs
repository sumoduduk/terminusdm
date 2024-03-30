use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

use crate::config::Language;
use crate::tui::app::tabs_state::SelectedTabs;
use crate::tui::app::AppTui;
use crate::tui::app::CurrentScreen;
use crate::tui::app::InputMode;
use tui_input::backend::crossterm::EventHandler;

pub fn handle_tabs_event(app: &mut AppTui, key: KeyEvent) {
    match app.selected_tabs {
        SelectedTabs::Language => handle_lang(app, key),
        _ => handle_tabs_content(app, key),
    }
}

fn handle_lang(app: &mut AppTui, key: KeyEvent) {
    match key.code {
        KeyCode::Tab => app.curr_screen = CurrentScreen::Main,
        KeyCode::Char('q') => app.curr_screen = CurrentScreen::Exiting,
        KeyCode::Char('l') | KeyCode::Right => app.next_tab(),
        KeyCode::Char('h') | KeyCode::Left => app.previous_tab(),
        KeyCode::Down | KeyCode::Char('j') => app.next_lang(),
        KeyCode::Up | KeyCode::Char('k') => app.prev_lang(),
        KeyCode::Enter => match app.lang_state.selected() {
            Some(index) => match index {
                0 => match app.update_config("", Some(Language::English)) {
                    Ok(_) => (),
                    Err(err) => app.set_error_msg(err.to_string()),
                },
                1 => match app.update_config("", Some(Language::Indonesia)) {
                    Ok(_) => (),
                    Err(err) => app.set_error_msg(err.to_string()),
                },
                _ => {}
            },
            None => {}
        },
        _ => {}
    }
}

fn handle_tabs_content(app: &mut AppTui, key: KeyEvent) {
    match app.tab_content_mode {
        InputMode::Normal => handle_input_normal(app, key),
        InputMode::Editing => handle_input_insert(app, key),
    }
}

fn handle_input_normal(app: &mut AppTui, key: KeyEvent) {
    match key.code {
        KeyCode::Tab => app.curr_screen = CurrentScreen::Main,
        KeyCode::Char('q') => app.curr_screen = CurrentScreen::Exiting,
        KeyCode::Char('e') => {
            app.tab_content_mode = InputMode::Editing;
        }
        KeyCode::Char('l') | KeyCode::Right => app.next_tab(),
        KeyCode::Char('h') | KeyCode::Left => app.previous_tab(),
        _ => {}
    }
}

fn handle_input_insert(app: &mut AppTui, key: KeyEvent) {
    match key.code {
        KeyCode::Tab => {
            app.tab_content_input.reset();
            app.tab_content_mode = InputMode::Normal;
            app.curr_screen = CurrentScreen::Main
        }
        KeyCode::Esc => {
            app.tab_content_input.reset();
            app.tab_content_mode = InputMode::Normal;
        }
        KeyCode::Enter => {
            let value = app.tab_content_input.value().to_string();
            match app.update_config(&value, None) {
                Ok(_) => {
                    app.tab_content_input.reset();
                    app.tab_content_mode = InputMode::Normal;
                }
                Err(err) => app.set_error_msg(err.to_string()),
            }
        }
        _ => {
            app.tab_content_input.handle_event(&Event::Key(key));
        }
    }
}
