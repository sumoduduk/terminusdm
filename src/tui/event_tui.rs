use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};

use crate::{AppTui, CurrentScreen};

use super::main_layout::ui;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut AppTui,
    tick_rate: Duration,
) -> eyre::Result<bool> {
    let last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                match app.curr_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('e') => app.curr_screen = CurrentScreen::Editing,
                        KeyCode::Char('q') => app.curr_screen = CurrentScreen::Exiting,
                        _ => {}
                    },

                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => return Ok(true),
                        KeyCode::Char('n') | KeyCode::Char('q') => return Ok(false),
                        _ => (),
                    },
                    CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => {
                            app.save_input();
                        }
                        KeyCode::Backspace => {
                            app.input_uri.pop();
                        }
                        KeyCode::Esc => {
                            app.curr_screen = CurrentScreen::Main;
                        }
                        KeyCode::Char(char_input) => {
                            app.input_uri.push(char_input);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
