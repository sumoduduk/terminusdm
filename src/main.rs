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
        let mut app = tdm::tui::app::AppTui::new();

        let res = tdm::tui::event_tui::run_app(&mut terminal, &mut app);

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
                    // app.print_vec()?;
                    let vec_value = app.saved_input;
                    if vec_value.is_empty() {
                        println!("empty");
                    } else {
                        for input_value in &vec_value {
                            println!("Downloading file : {}", input_value);
                            tdm::download_chunk(input_value).await?;
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

        // let time = Duration::from_secs(3);
        // thread::sleep(time);
    }

    Ok(())
}
