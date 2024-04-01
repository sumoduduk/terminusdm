use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{HighlightSpacing, List},
    Frame,
};

use crate::tui::app::AppTui;

pub fn render_lang_layout(frame: &mut Frame, app: &mut AppTui, area: Rect) {
    let list = List::new(["English", "Indonesia"])
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>  ")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(list, area, &mut app.lang_state);
}
