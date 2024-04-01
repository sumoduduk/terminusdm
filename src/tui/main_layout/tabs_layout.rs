mod lang_layout;

use ratatui::{
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{block::Title, Block, BorderType, Borders, Paragraph, Tabs},
    Frame,
};
use strum::IntoEnumIterator;

use crate::tui::app::InputMode::*;
use crate::{
    tui::app::{tabs_state::SelectedTabs, AppTui, CurrentScreen},
    words::WORDS,
};

use self::lang_layout::render_lang_layout;

pub fn render_tabs(frame: &mut Frame, app: &mut AppTui, area: Rect) {
    let title_tab = SelectedTabs::iter().map(SelectedTabs::title);

    let highlight_style = (Color::default(), app.selected_tabs.palette().c700);

    let tabs_index = app.selected_tabs as usize;

    let tabs_header = Tabs::new(title_tab)
        .highlight_style(highlight_style)
        .select(tabs_index)
        .padding("", "")
        .divider(" ");

    frame.render_widget(tabs_header, area)
}

pub fn render_tabs_content(frame: &mut Frame, app: &mut AppTui, area: Rect) {
    let block_content = block(app);
    frame.render_widget(block_content, area);
    let inside_area = area.inner(&Margin {
        horizontal: 1,
        vertical: 1,
    });

    let inside_rect = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(3),
    ]);
    let [_, content_layout, input_layout] = inside_rect.areas(inside_area);

    let config = &app.setting;
    let lang = &config.language;

    let text_folder = WORDS::ConfigsContentFolder;
    let text_concur = WORDS::ConfigsContentConcurent;
    let text_chunk = WORDS::ConfigsContentChunk;
    let text_min = WORDS::ConfigsContentMinimum;

    match app.selected_tabs {
        SelectedTabs::DownloadFolder => {
            let content = config.default_folder.display().to_string();

            span_content(
                &text_folder.load_text(lang),
                &content,
                content_layout,
                frame,
                app,
            );
            render_value_input(app, frame, input_layout);
        }
        SelectedTabs::ConcurrentTotal => {
            let content = config.concurrent_download.to_string();
            span_content(
                &text_concur.load_text(lang),
                &content,
                content_layout,
                frame,
                app,
            );

            render_value_input(app, frame, input_layout);
        }
        SelectedTabs::ChunkSize => {
            let content = config.total_chunk.to_string();
            span_content(
                &text_chunk.load_text(lang),
                &content,
                content_layout,
                frame,
                app,
            );

            render_value_input(app, frame, input_layout);
        }
        SelectedTabs::MinimunSize => {
            let mininimum_size = config.minimum_size.to_string();
            span_content(
                &text_min.load_text(lang),
                &mininimum_size,
                content_layout,
                frame,
                app,
            );

            render_value_input(app, frame, input_layout);
        }
        SelectedTabs::Language => {
            render_lang_layout(frame, app, inside_area);
        }
    };
}

fn render_value_input(app: &AppTui, frame: &mut Frame, area: Rect) {
    let lang = &app.setting.language;

    let title_text = WORDS::TabsInputTitle;

    let value = app.tab_content_input.value();
    let border = Block::default()
        .borders(Borders::ALL)
        .title(title_text.load_text(lang))
        .border_style(app.selected_tabs.palette().c400)
        .border_type(BorderType::Rounded);

    let input_par = Paragraph::new(value).block(border);
    let width = area.width.max(3) - 3;
    let scroll_input = app.tab_content_input.visual_scroll(width as usize);

    frame.render_widget(input_par, area);

    match app.tab_content_mode {
        Normal => {}
        Editing => frame.set_cursor(
            area.x
                + ((app.tab_content_input.visual_cursor().max(scroll_input) - scroll_input) as u16
                    + 1),
            area.y + 1,
        ),
    }
}

fn span_content(key: &str, val: &str, area: Rect, frame: &mut Frame, app: &AppTui) {
    let area = area.inner(&Margin {
        horizontal: 1,
        vertical: 0,
    });
    let value_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [left_side, right_side] = value_layout.areas(area);

    let left_key = Line::from(key)
        .centered()
        .style(Style::default().bg(app.selected_tabs.palette().c700));

    frame.render_widget(left_key, left_side);
    let right_val = Line::from(val)
        .centered()
        .style(Style::default().bg(app.selected_tabs.palette().c800));

    frame.render_widget(right_val, right_side);
}

pub fn outer_block_setting(app: &AppTui) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title("Setting")
        .border_type(match app.curr_screen {
            CurrentScreen::Setting => BorderType::Double,
            _ => BorderType::Rounded,
        })
        .border_style(match app.curr_screen {
            CurrentScreen::Setting => Style::default().fg(Color::Cyan),
            _ => Style::default(),
        })
}

fn block(app: &AppTui) -> Block<'static> {
    let lang = &app.setting.language;
    let text_lang = WORDS::TabsContentLang;
    let text_normal = WORDS::TabsContentNormal;
    let text_edit = WORDS::TabsContentEditing;

    let title_tab_content = match app.selected_tabs {
        SelectedTabs::Language => text_lang.load_text(lang),
        _ => match app.tab_content_mode {
            Normal => text_normal.load_text(lang),
            Editing => text_edit.load_text(lang),
        },
    };

    let title_nav = match app.curr_screen {
        CurrentScreen::Setting => title_tab_content,
        _ => "".to_string(),
    };

    Block::default()
        .title(Title::from(title_nav).alignment(Alignment::Right))
        .borders(Borders::TOP)
        .border_style(app.selected_tabs.palette().c500)
}
