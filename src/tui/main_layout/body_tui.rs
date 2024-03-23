use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::{AppTui, InputMode};

pub fn body_comp(app: &AppTui) -> List<'static> {
    let block_body = Block::default()
        .title("Download History")
        .borders(Borders::ALL)
        .border_style(match app.curr_screen {
            crate::CurrentScreen::Main => Style::default().fg(Color::Cyan),
            _ => Style::default(),
        })
        .border_type(match app.curr_screen {
            crate::CurrentScreen::Main => BorderType::Thick,
            _ => BorderType::Rounded,
        });

    let mut list_item: Vec<ListItem> = Vec::new();

    for name_input in &app.saved_input {
        list_item.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25}", name_input),
            Style::default().fg(Color::Yellow),
        ))))
    }

    let list = List::new(list_item).block(block_body);

    list
}

pub fn input_editing(app: &AppTui, width: u16) -> Paragraph<'static> {
    let scroll_input = app.input_uri.visual_scroll(width as usize);

    let input_components = Block::default()
        .title("Enter a URI")
        .borders(Borders::ALL)
        .border_style(match app.curr_screen {
            crate::CurrentScreen::Editing => Style::default().fg(Color::Cyan),
            _ => Style::default(),
        })
        .border_type(match app.curr_screen {
            crate::CurrentScreen::Editing => BorderType::Thick,
            _ => BorderType::Rounded,
        });

    let value = app.input_uri.value();

    let input_par = Paragraph::new(value.to_string())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Magenta),
        })
        .block(input_components)
        .scroll((0, scroll_input as u16))
        .wrap(Wrap { trim: false });

    input_par
}

pub fn download_process() -> Block<'static> {
    Block::default()
        .title("Download Process")
        .borders(Borders::ALL)
}

pub fn popup_exit() -> Paragraph<'static> {
    let popup_exit_component = Block::default()
        .title("Y/N")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let exit_text = Text::styled(
        "Would you like to output the buffer as json? (y/n)",
        Style::default().fg(Color::Red),
    );
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_exit_component)
        .wrap(Wrap { trim: false });

    exit_paragraph
}
