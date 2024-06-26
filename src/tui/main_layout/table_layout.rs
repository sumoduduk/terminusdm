use ratatui::{
    layout::{Alignment, Constraint, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{
        block::Title, Block, BorderType, Borders, Cell, HighlightSpacing, Row, Scrollbar,
        ScrollbarOrientation, Table,
    },
    Frame,
};

use crate::{
    tui::app::{AppTui, CurrentScreen},
    words::WORDS,
};

pub fn render_table(frame: &mut Frame, app: &mut AppTui, area: Rect) {
    let width = area.width;
    let column_width = width / 4;

    let header_style = Style::default()
        .fg(app.table.colors.header_fg)
        .bg(app.table.colors.header_bg);

    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.table.colors.selected_style_fg);

    let header = ["FILE", "URL", "STATUS"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);

    let rows = app.list_history().iter().enumerate().map(|(i, data)| {
        let color = match i {
            num if app.table.picked.contains(&num) => app.table.colors.picked_color,
            num if num % 2 == 0 => app.table.colors.normal_row_color,
            _ => app.table.colors.alt_row_color,
        };

        let (_, data) = data;

        let item = data.ref_array();

        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
            .collect::<Row>()
            .style(Style::new().fg(app.table.colors.row_fg).bg(color))
            .height(3)
    });

    let bar = " █ ";

    let language = &app.setting.language;
    let nav = WORDS::TableNav;

    let title_nav = match app.curr_screen {
        CurrentScreen::Main => nav.load_text(language),
        _ => "".to_string(),
    };

    let tables = Table::new(
        rows,
        [
            Constraint::Length(column_width),
            Constraint::Min(column_width * 2),
            Constraint::Length(20),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("History")
            .title(Title::from(title_nav).alignment(Alignment::Right))
            .border_type(match app.curr_screen {
                CurrentScreen::Main => BorderType::Double,
                _ => BorderType::Rounded,
            })
            .border_style(match app.curr_screen {
                CurrentScreen::Main => Style::default().fg(Color::Cyan),
                _ => Style::default(),
            }),
    )
    .highlight_style(selected_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .bg(app.table.colors.buffer_bg)
    .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(tables, area, &mut app.table.state);
}

pub fn render_scrollbar_table(frame: &mut Frame, app: &mut AppTui, area: Rect) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.table.scroll_state,
    )
}
