use std::{io, time::Duration};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();

    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr))?;
    let mut app = tdm::AppTui::new();

    let tick_rate = Duration::from_millis(250);

    let res = tdm::tui::event_tui::run_app(&mut terminal, &mut app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match res {
        Ok(do_print) => {
            if do_print {
                app.print_vec()?;
            }
        }
        Err(err) => {
            println!("{}", err.to_string());
        }
    }

    Ok(())

    // let mut args = args();
    // args.next();
    // let download_uri = args.next().expect("ERROR: argument is empty");
    // dbg!(&download_uri);
    //
    // let _ = tdm::download_chunk(&download_uri).await;
}
