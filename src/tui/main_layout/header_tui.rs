use ratatui::{
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

pub fn header_comp() -> Paragraph<'static> {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title_text = Text::styled(
        "Terminal Download Manager",
        Style::default().fg(Color::Blue),
    )
    .centered();

    let title_paragraph = Paragraph::new(title_text).block(title_block);

    title_paragraph
}
