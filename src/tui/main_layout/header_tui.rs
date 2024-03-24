use ratatui::{
    style::{Color, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

pub fn header_comp() -> Paragraph<'static> {
    let title_span = vec![Span::styled(
        format!("[ {} {} ] ", "Terminal Download Manager", symbols::DOT),
        Style::default().fg(Color::Blue),
    )];

    let title_block = Line::from(title_span).centered();
    let title_block = Paragraph::new(title_block).block(title_outer_block());
    title_block
}

fn title_outer_block() -> Block<'static> {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Blue))
        .padding(Padding::proportional(3))
        .border_type(BorderType::Rounded);

    block
}
