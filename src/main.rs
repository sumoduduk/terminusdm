use std::{io, thread, time::Duration};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    loop {
        enable_raw_mode()?;
        let mut stderr = io::stderr();

        execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stderr))?;

        let config = tdm::config::Config::new();
        let mut app = tdm::tui::app::AppTui::new(config);

        let res = tdm::tui::event_tui::run_app(&mut terminal, &mut app).await;

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        terminal.clear()?;

        match res {
            Ok(do_print) => {
                if do_print {
                    let vec_value = &app.saved_input.clone();
                    if vec_value.is_empty() {
                        println!("empty");
                    } else {
                        for (key_value, _) in vec_value {
                            let histo = app.get_history(*key_value);
                            tdm::download_chunk(&mut app, *key_value).await?;
                        }
                    }
                } else {
                    break;
                }
            }
            Err(err) => {
                println!("{}", err.to_string());
                break;
            }
        }

        let dura = Duration::from_secs(1);
        thread::sleep(dura);
    }

    Ok(())
}
