use std::{env::args, io::stdout};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph, Terminal};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            frame.render_widget(
                Paragraph::new("Hello from terminal, pres q to exit!")
                    .white()
                    .on_dark_gray(),
                area,
            );
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())

    // let mut args = args();
    // args.next();
    // let download_uri = args.next().expect("ERROR: argument is empty");
    // dbg!(&download_uri);
    //
    // let _ = tdm::download_chunk(&download_uri).await;
}
