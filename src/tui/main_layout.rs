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
    body_tui::{body_comp, input_editing, popup_editing_layout, popup_exit},
    footer_tui::{footer_comp_mode, footer_comp_notes},
    header_tui::header_comp,
};

pub fn ui(frame: &mut Frame, app: &AppTui) {
    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title = header_comp();

    frame.render_widget(title, chunk[0]);

    let list = body_comp(app);

    frame.render_widget(list, chunk[1]);

    let footer_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunk[2]);

    let mode_footer = footer_comp_mode(app);
    let key_notes_footer = footer_comp_notes(app);

    frame.render_widget(mode_footer, footer_chunk[0]);
    frame.render_widget(key_notes_footer, footer_chunk[1]);

    let area_popup = centered_rect(60, 25, frame.size());

    let popup_component = popup_editing_layout();

    let input_comp = popup_component.inner(area_popup);
    frame.render_widget(popup_component, area_popup);

    let input_par = input_editing(app);

    frame.render_widget(input_par, input_comp);

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
