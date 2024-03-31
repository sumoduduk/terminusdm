use ratatui::{
    layout::Rect,
    style::{Color, Style},
    symbols,
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

const VERSION: &str = "1.0.0";

pub fn header_comp(frame: &mut Frame, area: Rect) {
    let width = area.width;

    let title_content = match width {
        width if width > 28 => "Terminal Download Manager",
        _ => "TDM",
    };

    let title = Line::from(format!("[ {} ]", title_content));
    let version = Line::from(format!("[ v{}{} ] ", symbols::DOT, VERSION));

    let title_block = Paragraph::new(vec![title, version])
        .block(title_outer_block())
        .centered()
        .style(Style::default().fg(Color::Blue));

    frame.render_widget(title_block, area)
}

fn title_outer_block() -> Block<'static> {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Blue))
        .padding(Padding::vertical(3))
        .border_type(BorderType::Rounded);

    block
}
