use ratatui::widgets::ListState;

#[derive(Default)]
pub struct Cursor {
    position: Option<(u16, u16)>,
}

impl Cursor {
    pub fn set(&mut self, x: u16, y: u16) {
        self.position = Some((x, y));
    }

    pub fn take(&mut self) -> Option<(u16, u16)> {
        self.position.take()
    }
}

#[derive(Default)]
pub struct AddSongListState {
    pub file_scroll: ListState,
    pub music_scroll: ListState,
}

#[derive(Default)]
pub struct AppState {
    pub cursor: Cursor,
    pub add_song_state: AddSongListState,
}
