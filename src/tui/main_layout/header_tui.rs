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
    let height = area.height;

    let title_content = match width {
        width if width > 30 => "Terminus Download Manager",
        _ => "TDM",
    };

    let title = Line::from(format!("[ {} ]", title_content)).centered();
    let version = Line::from(format!("[ v{}{} ] ", symbols::DOT, VERSION)).centered();
    let para = Paragraph::new(vec![title, version]);

    let y = (height - 5) / 2;

    let title_block = para
        .block(title_outer_block(y))
        .centered()
        .style(Style::default().fg(Color::Blue));

    frame.render_widget(title_block, area)
}

fn title_outer_block(num: u16) -> Block<'static> {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Blue))
        .padding(Padding::vertical(num))
        .border_type(BorderType::Rounded);

    block
}
