mod footer_components;
mod music_list_components;
mod playback_component;
mod playlist_components;
mod pop_up;
mod volume_component;

use footer_components::render_footer;
use music_list_components::render_music_list;
use playback_component::render_playback;
use playlist_components::render_playlist;
use pop_up::render_popup;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Modifier, Style},
    widgets::Widget,
};
use volume_component::Volume;

use crate::user_interface::cursor::AppState;

use super::App;
use Constraint::*;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer, app_state: &mut AppState) {
    let main_screen = Layout::vertical([Percentage(100), Min(3)]);
    let [main_layout, footer_layout] = main_screen.areas(area);

    let horizontal_screen = Layout::horizontal([Percentage(35), Percentage(65)]);
    let [main_layout, music_list_layout] = horizontal_screen.areas(main_layout);

    let main_frame = Layout::vertical([Percentage(35), Min(3), Percentage(65)]);
    let [playback_layout, volume_layout, playlist_layout] = main_frame.areas(main_layout);

    render_playback(app, playback_layout, buf);
    Volume::new(app.volume).render(volume_layout, buf);
    render_playlist(app, playlist_layout, buf);
    render_music_list(app, music_list_layout, buf);
    render_footer(app, footer_layout, buf);
    render_popup(app, area, buf, app_state);
}
