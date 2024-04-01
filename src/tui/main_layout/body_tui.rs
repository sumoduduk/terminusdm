use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{block::Title, Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::tui::app::{AppTui, CurrentScreen, InputMode};

pub fn input_editing(app: &AppTui, width: u16) -> Paragraph<'static> {
    let scroll_input = app.input_uri.visual_scroll(width as usize);

    let input_components = Block::default()
        .title("Enter a URL")
        .title(Title::from("Press ENTER to download | ESC to quit").alignment(Alignment::Right))
        .borders(Borders::ALL)
        .border_style(match app.curr_screen {
            CurrentScreen::Editing => Style::default().fg(Color::Cyan),
            _ => Style::default(),
        })
        .border_type(match app.curr_screen {
            CurrentScreen::Editing => BorderType::Thick,
            _ => BorderType::Rounded,
        });

    let value = app.input_uri.value();

    let input_par = Paragraph::new(value.to_string())
        // .style(match app.input_mode {
        //     InputMode::Normal => Style::default(),
        //     InputMode::Editing => Style::default().fg(Color::Magenta),
        // })
        .block(input_components)
        .scroll((0, scroll_input as u16));

    input_par
}

pub fn user_settings(app: &AppTui) -> Block<'static> {
    Block::default()
        .title("Setting")
        .borders(Borders::ALL)
        .border_style(match app.curr_screen {
            CurrentScreen::Setting => Style::default().fg(Color::Cyan),
            _ => Style::default(),
        })
        .border_type(match app.curr_screen {
            CurrentScreen::Setting => BorderType::Thick,
            _ => BorderType::Rounded,
        })
}

pub fn popup_exit() -> Paragraph<'static> {
    let popup_exit_component = Block::default()
        .title("Press Y to confirm/N to cancel")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let exit_text = Text::styled(
        "Would you like to quit? (y/n)",
        Style::default().fg(Color::Red),
    );
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_exit_component)
        .wrap(Wrap { trim: false });

    exit_paragraph
}
