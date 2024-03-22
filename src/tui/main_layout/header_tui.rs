use ratatui::{
    style::{Color, Style},
    symbols,
    text::{Line, Span},
};

pub fn header_comp() -> Line<'static> {
    let title_span = vec![
        Span::styled(
            format!("[ {} {} ", "Terminal Download Manager", symbols::DOT),
            Style::default().fg(Color::Blue),
        ),
        Span::styled(
            format!("{} ", "v0.0.1"),
            Style::default().fg(Color::LightCyan),
        ),
        Span::styled("]", Style::default().fg(Color::Blue)),
    ];

    let title_block = Line::from(title_span).right_aligned();
    title_block
}
