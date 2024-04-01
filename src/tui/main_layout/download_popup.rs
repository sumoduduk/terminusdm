use crate::{tui::app::AppTui, words::WORDS};

use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{block::Title, Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_download_popup(frame: &mut Frame, app: &mut AppTui, area: Rect) {
    let language = &app.setting.language;
    let title = WORDS::DownloadTitle;
    let nav = WORDS::DownloadNav;
    let content = WORDS::DownlaodContent;

    let popup_download_component = Block::default()
        .title(title.load_text(language))
        .title(Title::from(nav.load_text(language)).alignment(Alignment::Right))
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let mut download_text = vec![Line::styled(
        content.load_text(language),
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
    let language = &app.setting.language;
    let title = WORDS::DownloadPrepareTitle;
    let nav = WORDS::DownloadNav;
    let content = WORDS::DownlaodContent;
    let loading = WORDS::DownloadLoading;

    let popup_download_component = Block::default()
        .title(title.load_text(language))
        .title(Title::from(nav.load_text(language)).alignment(Alignment::Right))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black));

    frame.render_widget(popup_download_component, area);

    let block_msg = Block::default().borders(Borders::BOTTOM);

    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]);
    let [upper_l, bottom_l] = layout.areas(area);

    let input_val = &app.input_uri.value();

    let download_text = vec![
        Line::styled(
            content.load_text(language),
            Style::default().fg(Color::White),
        ),
        Line::styled(input_val.to_string(), Style::default().fg(Color::White)),
    ];

    let download_paragraph = Paragraph::new(download_text)
        .block(block_msg)
        .wrap(Wrap { trim: false });

    frame.render_widget(download_paragraph, upper_l);

    let loading = throbber_widgets_tui::Throbber::default()
        .label(loading.load_text(language))
        .style(Style::default().fg(Color::Cyan))
        .throbber_style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )
        .throbber_set(throbber_widgets_tui::CLOCK)
        .use_type(throbber_widgets_tui::WhichUse::Spin);

    frame.render_widget(loading, bottom_l);
}
