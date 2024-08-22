use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Clear, Padding, Paragraph, Widget, Wrap},
};

use crate::app::{centered_rect, App};

pub fn render_help(_app: &App, area: Rect, buf: &mut Buffer) {
    let pop_up_area = centered_rect(70, 70, area);

    Clear.render(pop_up_area, buf);
    let block = Block::bordered()
        .title("Help/Config | Press ESC to go back")
        .style(Style::new().fg(Color::White).bg(Color::Red))
        .padding(Padding::new(1, 0, 0, 1))
        .border_style(Color::White);

    let msg_help = [
        "TAB : Switch screen",
        "",
        "Q : Quit app",
        "",
        "? : Go to help screen",
        "",
        "+/- : Volume Control",
        "",
        "Space : Toggle Play/Pause",
        "",
        "A : Add playlist when on playlist screen / add song when on music list screen",
        "",
        "J : Scroll down",
        "",
        "K : Scroll up",
        "",
        "R : Switch Mode Repeat All/ Normal/ Repeat One",
    ];

    let line_msg = msg_help.map(Line::from);

    let text = Text::from(line_msg.to_vec());

    Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true })
        .render(pop_up_area, buf);
}
