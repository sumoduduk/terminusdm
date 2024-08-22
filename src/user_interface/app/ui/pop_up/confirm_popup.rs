use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Widget},
};

use crate::app::{centered_rect, App};

pub fn render_popup_confirm(app: &App, area: Rect, buf: &mut Buffer) {
    let pop_up_area = centered_rect(40, 30, area);

    let (title, msg) = app.pop_up_msg().unwrap_or_default();

    let block = Block::new()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .style(Style::new().fg(Color::White).bg(Color::Red))
        .padding(Padding::new(0, 0, pop_up_area.height / 4, 0))
        .border_style(Color::White);

    let line_msg = Line::from(msg);

    let line_confirm = Line::from(vec![
        Span::raw("(Y) Yes     "),
        Span::raw("|"),
        Span::raw("     (N) No"),
    ]);

    let text = Text::from(vec![line_msg, Line::default(), line_confirm]);

    Clear.render(pop_up_area, buf);
    Paragraph::new(text)
        .block(block)
        .centered()
        .render(pop_up_area, buf);
}
