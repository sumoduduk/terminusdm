mod event_tab;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{backend::Backend, widgets::Paragraph, Terminal};
use tui_input::backend::crossterm::EventHandler;

use super::{
    app::{AppTui, CurrentScreen, InputMode},
    main_layout::ui,
};

use event_tab::handle_tabs_event;

pub async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut AppTui,
) -> eyre::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match app.curr_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Tab => app.curr_screen = CurrentScreen::Editing,
                        KeyCode::Char(' ') => app.table.pick(),
                        KeyCode::Char('q') => app.curr_screen = CurrentScreen::Exiting,
                        KeyCode::Char('j') | KeyCode::Down => app.table.next(),
                        KeyCode::Char('k') | KeyCode::Up => app.table.previous(),
                        KeyCode::Enter => {
                            if !app.table.picked.is_empty() {
                                app.saved_input.clear();
                                app.load_pick();
                                app.curr_screen = CurrentScreen::Download;
                            }
                        }
                        _ => {}
                    },
                    CurrentScreen::Setting => {
                        handle_tabs_event(app, key);
                    }
                    CurrentScreen::PrepareDownload => match key.code {
                        KeyCode::Char('n') => app.curr_screen = CurrentScreen::Main,
                        KeyCode::Enter | KeyCode::Char('y') => match app.save_input().await {
                            Ok(_) => {
                                app.save_history()?;
                                return Ok(true);
                                app.input_uri.reset();
                            }
                            Err(err) => {
                                app.set_error_msg(err.to_string());
                                app.input_uri.reset();
                            }
                        },
                        _ => {}
                    },
                    CurrentScreen::Download => match key.code {
                        KeyCode::Char('y') | KeyCode::Enter => return Ok(true),
                        KeyCode::Char('n') => app.curr_screen = CurrentScreen::Main,
                        KeyCode::Char('q') => app.curr_screen = CurrentScreen::Exiting,
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('n') | KeyCode::Char('q') => return Ok(false),
                        _ => (),
                    },
                    CurrentScreen::Editing => match key.code {
                        KeyCode::Enter => {
                            if app.input_uri.value().len() > 0 {
                                app.clear_saved_input();
                                app.curr_screen = CurrentScreen::PrepareDownload;
                            }
                        }
                        KeyCode::Esc => {
                            app.input_uri.reset();
                            app.curr_screen = CurrentScreen::Exiting;
                        }
                        KeyCode::Tab => app.curr_screen = CurrentScreen::Setting,
                        _ => {
                            app.input_uri.handle_event(&Event::Key(key));
                        }
                    },
                    // CurrentScreen::Editing => match app.input_mode {
                    //     InputMode::Normal => match key.code {
                    //         KeyCode::Char('i') => app.input_mode = InputMode::Editing,
                    //         KeyCode::Tab => app.curr_screen = CurrentScreen::Setting,
                    //         _ => {}
                    //     },
                    //     InputMode::Editing => match (key.modifiers, key.code) {
                    //         (modifiers, KeyCode::Enter) => match modifiers {
                    //             KeyModifiers::CONTROL => app.push_to_table().await,
                    //             KeyModifiers::NONE => {
                    //                 if app.input_uri.value().len() > 0 {
                    //                     app.clear_saved_input();
                    //                     app.input_mode = InputMode::Normal;
                    //                     app.curr_screen = CurrentScreen::PrepareDownload;
                    //                 }
                    //             }
                    //             _ => {}
                    //         },
                    //         (KeyModifiers::NONE, KeyCode::Esc) => {
                    //             app.input_mode = InputMode::Normal
                    //         }
                    //         (KeyModifiers::NONE, KeyCode::Tab) => {
                    //             app.input_mode = InputMode::Normal;
                    //             app.curr_screen = CurrentScreen::Setting;
                    //         }
                    //         _ => {
                    //             app.input_uri.handle_event(&Event::Key(key));
                    //         }
                    //     },
                    // },
                    CurrentScreen::ErrorScreen => match key.code {
                        KeyCode::Enter => {
                            app.clear_error_msg();
                            app.curr_screen = CurrentScreen::Main;
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}
