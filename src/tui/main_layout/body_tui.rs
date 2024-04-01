use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{block::Title, Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::{
    tui::app::{AppTui, CurrentScreen},
    words::WORDS,
};

pub fn input_editing(app: &AppTui, width: u16) -> Paragraph<'static> {
    let scroll_input = app.input_uri.visual_scroll(width as usize);
    let lang = &app.setting.language;

    let input_title = WORDS::InputTitle;
    let input_nav = WORDS::InputNavigation;

    let input_components = Block::default()
        .title(input_title.load_text(lang))
        .title(Title::from(input_nav.load_text(lang)).alignment(Alignment::Right))
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
        .block(input_components)
        .scroll((0, scroll_input as u16));

    input_par
}

pub fn popup_exit(app: &AppTui) -> Paragraph<'static> {
    let lang = &app.setting.language;

    let input_title = WORDS::ExitTitle;
    let input_nav = WORDS::ExitContent;

    let popup_exit_component = Block::default()
        .title(input_title.load_text(lang))
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let exit_text = Text::styled(input_nav.load_text(lang), Style::default().fg(Color::Red));
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_exit_component)
        .wrap(Wrap { trim: false });

    exit_paragraph
}
