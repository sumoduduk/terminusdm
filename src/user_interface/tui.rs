use ratatui::backend::Backend;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::layout::{Constraint, Layout};
use ratatui::Terminal;
use std::io;
use std::panic;

use super::app::{centered_rect, App, AppResult};
use super::cursor::AppState;
use super::event::EventHandler;

use Constraint::*;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        ratatui::crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: ratatui::Terminal::draw
    /// [`rendering`]: crate::ui::render
    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| {
            let mut app_state = AppState::default();

            if let Screen::PopUpFileExplorer = &app.screen_state {
                let pop_up_area = centered_rect(80, 50, frame.size());

                let top_bottom_area = Layout::vertical([Min(3), Percentage(100)]);

                let [top_layout, botom_layout] = top_bottom_area.areas(pop_up_area);

                let pop_up_top = render_pop_up_top();
                frame.render_widget(pop_up_top, top_layout);

                let main_pop_layout = Layout::horizontal([Percentage(50), Percentage(50)]);

                let [file_layout, file_add_layout] = main_pop_layout.areas(botom_layout);

                let add_widget = render_list_to_add(app);

                frame.render_widget(add_widget, file_add_layout);

                frame.render_widget(&app.file_explorer.widget(), file_layout);
            } else {
                frame.render_stateful_widget(app, frame.size(), &mut app_state);
            }

            if let Some(position) = app_state.cursor.take() {
                frame.set_cursor(position.0, position.1);
            }
        })?;

        Ok(())
    }

    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> AppResult<()> {
        terminal::disable_raw_mode()?;
        ratatui::crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> AppResult<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
