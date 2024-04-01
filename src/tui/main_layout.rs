mod body_tui;
mod download_popup;
mod error_popup;
mod footer_tui;
mod header_tui;
mod table_layout;
mod tabs_layout;

use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    widgets::Clear,
    Frame,
};

use self::{
    body_tui::{input_editing, popup_exit},
    download_popup::{render_begin_download, render_download_popup},
    error_popup::popup_error,
    footer_tui::{footer_comp_mode, footer_comp_notes},
    header_tui::header_comp,
    table_layout::{render_scrollbar_table, render_table},
    tabs_layout::{outer_block_setting, render_tabs, render_tabs_content},
};

use super::app::{AppTui, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &mut AppTui) {
    let screen = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(3)])
        .split(frame.size());

    let body_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(screen[0]);

    let upper_body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(body_layout[0]);

    //title
    header_comp(frame, upper_body[1]);

    //lower body - table
    render_table(frame, app, body_layout[1]);
    render_scrollbar_table(frame, app, body_layout[1]);

    //upper body
    let input_setting_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Fill(1)])
        .split(upper_body[0]);

    let width = input_setting_layout[0].width.max(3) - 3;

    let scroll_input = app.input_uri.visual_scroll(width as usize);
    let input_par = input_editing(app, width);
    frame.render_widget(input_par, input_setting_layout[0]);

    //setting
    let tabs_layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(0)]);
    let setting_inner = input_setting_layout[1].inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });

    let [tabs_header, tabs_content] = tabs_layout.areas(setting_inner);

    let setting_outer = outer_block_setting(app);
    frame.render_widget(setting_outer, input_setting_layout[1]);

    render_tabs(frame, app, tabs_header);
    render_tabs_content(frame, app, tabs_content);

    //footer
    let footer_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(screen[1]);

    let mode_footer = footer_comp_mode(app);
    let key_notes_footer = footer_comp_notes(app);

    frame.render_widget(mode_footer, footer_chunk[0]);
    frame.render_widget(key_notes_footer, footer_chunk[1]);

    // match app.input_mode {
    //     InputMode::Normal => {}
    //     InputMode::Editing => frame.set_cursor(
    //         input_setting_layout[0].x
    //             + ((app.input_uri.visual_cursor().max(scroll_input) - scroll_input) as u16 + 1),
    //         input_setting_layout[0].y + 1,
    //     ),
    // }

    if let CurrentScreen::Editing = app.curr_screen {
        frame.set_cursor(
            input_setting_layout[0].x
                + ((app.input_uri.visual_cursor().max(scroll_input) - scroll_input) as u16 + 1),
            input_setting_layout[0].y + 1,
        );
    }

    if let CurrentScreen::ErrorScreen = app.curr_screen {
        frame.render_widget(Clear, frame.size());

        let error_widget = popup_error(app);

        let area = centered_rect(60, 25, frame.size());
        frame.render_widget(error_widget, area);
    }

    if let CurrentScreen::Download = app.curr_screen {
        frame.render_widget(Clear, frame.size());
        let area = centered_rect(60, 25, frame.size());
        render_download_popup(frame, app, area);
    }

    if let CurrentScreen::PrepareDownload = app.curr_screen {
        frame.render_widget(Clear, frame.size());
        let area = centered_rect(60, 25, frame.size());
        render_begin_download(frame, app, area);
    }

    if let CurrentScreen::Exiting = app.curr_screen {
        frame.render_widget(Clear, frame.size());

        let exit_paragraph = popup_exit(app);

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
