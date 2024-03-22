use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::AppTui;

pub fn body_comp(app: &AppTui) -> List<'static> {
    let mut list_item: Vec<ListItem> = Vec::new();

    for name_input in &app.saved_input {
        list_item.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25}", name_input),
            Style::default().fg(Color::Yellow),
        ))))
    }

    let list = List::new(list_item);

    list
}

pub fn popup_editing_layout() -> Block<'static> {
    let popup_component = Block::default()
        .title("Enter a URI")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::White));

    popup_component
}

pub fn input_editing(app: &AppTui) -> Paragraph<'static> {
    let input_components = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::LightBlue).fg(Color::Black));

    let input_par = Paragraph::new(app.input_uri.clone())
        .block(input_components)
        .wrap(Wrap { trim: false });

    input_par
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
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_exit_component)
        .wrap(Wrap { trim: false });

    exit_paragraph
}
