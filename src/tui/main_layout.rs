mod body_tui;
mod footer_tui;
mod header_tui;
mod table_layout;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Clear,
    Frame,
};

use self::{
    body_tui::{body_comp, input_editing, popup_exit, user_settings},
    footer_tui::{footer_comp_mode, footer_comp_notes},
    header_tui::header_comp,
    table_layout::{render_scrollbar_table, render_table},
};

use super::app::{AppTui, CurrentScreen, InputMode};

pub fn ui(frame: &mut Frame, app: &mut AppTui) {
    let screen = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(3)])
        .split(frame.size());

    let body_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(screen[0]);

    let upper_body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(body_layout[0]);

    //title

    let title = header_comp();

    frame.render_widget(title, upper_body[1]);

    //lower body - table
    render_table(frame, app, body_layout[1]);
    render_scrollbar_table(frame, app, body_layout[1]);

    //upper body
    let input_setting_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(upper_body[0]);

    let width = input_setting_layout[0].width.max(3) - 3;

    let scroll_input = app.input_uri.visual_scroll(width as usize);
    let input_par = input_editing(app, width);
    frame.render_widget(input_par, input_setting_layout[0]);

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => frame.set_cursor(
            input_setting_layout[0].x
                + ((app.input_uri.visual_cursor().max(scroll_input) - scroll_input) as u16 + 1),
            input_setting_layout[0].y + 1,
        ),
    }

    let setting = user_settings(app);
    frame.render_widget(setting, input_setting_layout[1]);

    //footer
    let footer_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(screen[1]);

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
