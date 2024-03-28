use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    symbols,
    widgets::{block::Title, Block, BorderType, Borders, Padding, Paragraph, Tabs},
    Frame,
};
use strum::IntoEnumIterator;

use crate::tui::app::{tabs_state::SelectedTabs, AppTui, CurrentScreen};

pub fn render_tabs(frame: &mut Frame, app: &mut AppTui, area: Rect) {
    let title_tab = SelectedTabs::iter().map(SelectedTabs::title);

    let highlight_style = (Color::default(), app.selected_tabs.palette().c700);

    let tabs_index = app.selected_tabs as usize;

    let tabs_header = Tabs::new(title_tab)
        .highlight_style(highlight_style)
        .select(tabs_index)
        .padding("", "")
        .divider(" ");

    frame.render_widget(tabs_header, area)
}

fn render_tab0(app: &AppTui) -> Paragraph<'static> {
    Paragraph::new("Download Folder").block(block(app))
}

fn render_tab1(app: &AppTui) -> Paragraph<'static> {
    Paragraph::new("Concurrent Total").block(block(app))
}

fn render_tab2(app: &AppTui) -> Paragraph<'static> {
    Paragraph::new("Chunk Size").block(block(app))
}

fn render_tab3(app: &AppTui) -> Paragraph<'static> {
    Paragraph::new("Language").block(block(app))
}

/// A block surrounding the tab's content
fn block(app: &AppTui) -> Block<'static> {
    Block::default()
        .title(
            Title::from("◄ ► to change tab | Press e to edit | Enter to confirm")
                .alignment(Alignment::Right),
        )
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .border_style(app.selected_tabs.palette().c700)
}

pub fn outer_block_setting(app: &AppTui) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title("Config")
        .border_type(match app.curr_screen {
            CurrentScreen::Setting => BorderType::Thick,
            _ => BorderType::Rounded,
        })
        .border_style(match app.curr_screen {
            CurrentScreen::Setting => Style::default().fg(Color::Cyan),
            _ => Style::default(),
        })
}

pub fn render_tabs_content(frame: &mut Frame, app: &AppTui, area: Rect) {
    let content = match app.selected_tabs {
        SelectedTabs::DownloadFolder => render_tab0(app),
        SelectedTabs::ConcurrentTotal => render_tab1(app),
        SelectedTabs::ChunkSize => render_tab2(app),
        SelectedTabs::Language => render_tab3(app),
    };

    frame.render_widget(content, area);
}
