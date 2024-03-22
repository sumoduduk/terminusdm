use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{AppTui, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &AppTui) {
    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Terminal Download Manager",
        Style::default().fg(Color::Blue),
    ))
    .block(title_block);

    frame.render_widget(title, chunk[0]);

    //body
    let mut list_item: Vec<ListItem> = Vec::new();

    for name_input in &app.saved_input {
        list_item.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25}", name_input),
            Style::default().fg(Color::Yellow),
        ))))
    }

    let list = List::new(list_item);

    frame.render_widget(list, chunk[1]);

    let cur_span = match app.curr_screen {
        CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
        CurrentScreen::Editing => Span::styled("Editing Mode", Style::default().fg(Color::Yellow)),
        CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
    };

    let divider_span = Span::styled(" | ", Style::default().fg(Color::White));

    let is_editing_span = match app.curr_screen {
        CurrentScreen::Editing => {
            Span::styled("Editing Json Key", Style::default().fg(Color::Green))
        }
        _ => Span::styled(
            "Not Editing Anything",
            Style::default().fg(Color::LightGreen),
        ),
    };

    let curr_navigation_text = vec![cur_span, divider_span, is_editing_span];

    let mode_footer = Paragraph::new(Line::from(curr_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.curr_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunk[2]);

    frame.render_widget(mode_footer, footer_chunk[0]);
    frame.render_widget(key_notes_footer, footer_chunk[1]);

    //editing

    let popup_component = Block::default()
        .title("Enter a URI")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let area_popup = centered_rect(60, 25, frame.size());

    frame.render_widget(popup_component, area_popup);

    let input_components = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::LightBlue).fg(Color::Black));

    let input_par = Paragraph::new(app.input_uri.clone()).block(input_components);

    frame.render_widget(input_par, area_popup);

    if let CurrentScreen::Exiting = app.curr_screen {
        frame.render_widget(Clear, frame.size());

        let popup_exit_component = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_exit_component)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.size());
        frame.render_widget(exit_paragraph, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
