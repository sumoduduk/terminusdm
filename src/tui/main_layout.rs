mod body_tui;
mod footer_tui;
mod header_tui;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Clear,
    Frame,
};

use crate::{AppTui, CurrentScreen};

use self::{
    body_tui::{body_comp, download_process, input_editing, popup_exit},
    footer_tui::{footer_comp_mode, footer_comp_notes},
    header_tui::header_comp,
};

pub fn ui(frame: &mut Frame, app: &AppTui) {
    let screen = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(1),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Fill(2)])
        .split(screen[1]);

    //header
    let title = header_comp();
    frame.render_widget(title, screen[0]);

    //left body
    let list = body_comp(app);
    frame.render_widget(list, body_layout[0]);

    //right body

    let right_body_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(body_layout[1]);

    let width = right_body_layout[0].width.max(3) - 3;

    let scroll_input = app.input_uri.visual_scroll(width as usize);
    let input_par = input_editing(app, width);
    frame.render_widget(input_par, right_body_layout[0]);

    match app.input_mode {
        crate::InputMode::Normal => {}
        crate::InputMode::Editing => frame.set_cursor(
            right_body_layout[0].x
                + ((app.input_uri.visual_cursor().max(scroll_input) - scroll_input) as u16 + 1),
            right_body_layout[0].y + 1,
        ),
    }

    let download_process = download_process();
    frame.render_widget(download_process, right_body_layout[1]);

    let footer_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(screen[2]);

    let mode_footer = footer_comp_mode(app);
    let key_notes_footer = footer_comp_notes(app);

    frame.render_widget(mode_footer, footer_chunk[0]);
    frame.render_widget(key_notes_footer, footer_chunk[1]);

    if let CurrentScreen::Exiting = app.curr_screen {
        frame.render_widget(Clear, frame.size());

        let exit_paragraph = popup_exit();

        let area = centered_rect(60, 25, frame.size());
        frame.render_widget(exit_paragraph, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
