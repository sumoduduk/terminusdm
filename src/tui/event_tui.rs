use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{backend::Backend, Terminal};
use tui_input::backend::crossterm::EventHandler;

use super::{
    app::{AppTui, CurrentScreen, InputMode},
    main_layout::ui,
};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppTui) -> eyre::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.curr_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Tab => app.curr_screen = CurrentScreen::Editing,
                    KeyCode::Char('q') => app.curr_screen = CurrentScreen::Exiting,
                    _ => {}
                },

                CurrentScreen::Setting => match key.code {
                    KeyCode::Tab => app.curr_screen = CurrentScreen::Main,
                    KeyCode::Char('q') => app.curr_screen = CurrentScreen::Exiting,
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => return Ok(true),
                    KeyCode::Char('n') | KeyCode::Char('q') => return Ok(false),
                    _ => (),
                },
                CurrentScreen::Editing => match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('i') => app.input_mode = InputMode::Editing,
                        KeyCode::Tab => app.curr_screen = CurrentScreen::Setting,
                        _ => {}
                    },
                    InputMode::Editing => match (key.modifiers, key.code) {
                        (KeyModifiers::NONE, KeyCode::Enter) => {
                            app.save_input();
                            app.curr_screen = CurrentScreen::Exiting
                        }
                        (KeyModifiers::NONE, KeyCode::Esc) => app.input_mode = InputMode::Normal,
                        (KeyModifiers::NONE, KeyCode::Tab) => {
                            app.input_mode = InputMode::Normal;
                            app.curr_screen = CurrentScreen::Setting;
                        }
                        _ => {
                            app.input_uri.handle_event(&Event::Key(key));
                        }
                    },
                },
            }
        }
    }
}
