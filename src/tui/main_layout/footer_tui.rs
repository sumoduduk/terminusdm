mod text_footer;

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::app::{AppTui, CurrentScreen};

use self::text_footer::TextFooter;

pub fn footer_comp_mode(app: &AppTui) -> Paragraph<'static> {
    let language = &app.setting.language;
    let span_editing = TextFooter::SpanEditing;

    let cur_span = match app.curr_screen {
        CurrentScreen::Main => Span::styled("Table Mode", Style::default().fg(Color::Green)),
        CurrentScreen::Editing => {
            Span::styled("Input URL Mode", Style::default().fg(Color::Yellow))
        }
        CurrentScreen::Setting => Span::styled("Setting Mode", Style::default().fg(Color::Blue)),
        CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        CurrentScreen::PrepareDownload => {
            Span::styled("Prepare Download", Style::default().fg(Color::LightRed))
        }
        CurrentScreen::Download => {
            Span::styled("Download Mode", Style::default().fg(Color::LightRed))
        }
        CurrentScreen::ErrorScreen => {
            Span::styled("ERROR MESSAGE", Style::default().fg(Color::LightRed))
        }
    };

    let divider_span = Span::styled(" | ", Style::default().fg(Color::White));

    let is_editing_span = match app.curr_screen {
        CurrentScreen::Main => Span::styled(
            span_editing.load_text(language),
            Style::default().fg(Color::Green),
        ),
        _ => Span::styled(" ", Style::default().fg(Color::LightGreen)),
    };

    let curr_navigation_text = vec![cur_span, divider_span, is_editing_span];

    let mode_footer = Paragraph::new(Line::from(curr_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    mode_footer
}

pub fn footer_comp_notes(app: &AppTui) -> Paragraph<'static> {
    let language = &app.setting.language;
    let main_note = TextFooter::NoteMain;
    let edit_note = TextFooter::NoteEditing;
    let set_note = TextFooter::NoteSetting;
    let base_note = TextFooter::NoteBase;

    let current_keys_hint = {
        match app.curr_screen {
            CurrentScreen::Main => Span::styled(
                main_note.load_text(language),
                Style::default().fg(Color::LightRed),
            ),
            CurrentScreen::Editing => Span::styled(
                edit_note.load_text(language),
                Style::default().fg(Color::LightBlue),
            ),
            CurrentScreen::Setting => Span::styled(
                set_note.load_text(language),
                Style::default().fg(Color::LightBlue),
            ),
            _ => Span::styled(
                base_note.load_text(language),
                Style::default().fg(Color::LightGreen),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    key_notes_footer
}
