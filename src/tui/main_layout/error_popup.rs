use ratatui::{
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::tui::app::AppTui;

pub fn popup_error(app: &AppTui) -> Paragraph<'static> {
    let popup_exit_component = Block::default()
        .title("Press Enter to continue")
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
