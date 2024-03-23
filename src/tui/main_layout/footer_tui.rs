use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{AppTui, CurrentScreen};

pub fn footer_comp_mode(app: &AppTui) -> Paragraph<'static> {
    let cur_span = match app.curr_screen {
        CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
        CurrentScreen::Editing => Span::styled("Editing Mode", Style::default().fg(Color::Yellow)),
        CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
    };

    let divider_span = Span::styled(" | ", Style::default().fg(Color::White));

    let is_editing_span = match app.curr_screen {
        CurrentScreen::Editing => {
            Span::styled("Editing Json Key", Style::default().fg(Color::Green))
        }
        _ => Span::styled(
            "Not Editing Anything",
            Style::default().fg(Color::LightGreen),
        ),
    };

    let curr_navigation_text = vec![cur_span, divider_span, is_editing_span];

    let mode_footer = Paragraph::new(Line::from(curr_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    mode_footer
}

pub fn footer_comp_notes(app: &AppTui) -> Paragraph<'static> {
    let current_keys_hint = {
        match app.curr_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / Tab to switch",
                Style::default().fg(Color::LightRed),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::LightBlue),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::LightGreen),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    key_notes_footer
}
