use ratatui::{
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::{tui::app::AppTui, words::WORDS};

pub fn popup_error(app: &AppTui) -> Paragraph<'static> {
    let language = &app.setting.language;
    let error_popup = WORDS::ErrorPopup;

    let popup_exit_component = Block::default()
        .title(error_popup.load_text(language))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Red).fg(Color::White));

    let msg = app.error_msg.clone();

    let exit_text = Text::styled(msg, Style::default().fg(Color::Red));
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_exit_component)
        .centered()
        .wrap(Wrap { trim: false });

    exit_paragraph
}
