use crate::tui::app::AppTui;

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_download_popup(frame: &mut Frame, app: &mut AppTui, area: Rect) {
    let popup_download_component = Block::default()
        .title("Begin Download")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let mut download_text = vec![Line::styled(
        "Would you like download from these url? (y/n) : ",
        Style::default().fg(Color::White),
    )];

    for (_, url) in &app.saved_input {
        let url_format = format!(" - {}", url);

        let line_url = Line::styled(url_format, Style::default().fg(Color::White));

        download_text.push(line_url);
    }

    let download_paragraph = Paragraph::new(download_text)
        .block(popup_download_component)
        .wrap(Wrap { trim: false });

    frame.render_widget(download_paragraph, area);
}

pub fn render_begin_download(frame: &mut Frame, app: &mut AppTui, area: Rect) {
    let loading = throbber_widgets_tui::Throbber::default()
        .label("preparing for download, please wait...")
        .style(Style::default().fg(Color::Cyan))
        .throbber_style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )
        .throbber_set(throbber_widgets_tui::CLOCK)
        .use_type(throbber_widgets_tui::WhichUse::Spin);

    frame.render_widget(loading, area);
}
