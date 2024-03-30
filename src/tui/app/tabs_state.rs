use ratatui::{
    style::{palette::tailwind, Stylize},
    text::Line,
};
use strum::{Display, EnumIter, FromRepr};

#[derive(Default, FromRepr, EnumIter, Display, Clone, Copy)]
pub enum SelectedTabs {
    #[default]
    #[strum(to_string = "Folder")]
    DownloadFolder,
    #[strum(to_string = "Concurent Download")]
    ConcurrentTotal,
    #[strum(to_string = "Chunk")]
    ChunkSize,
    #[strum(to_string = "Minimum Size")]
    MinimunSize,
    #[strum(to_string = "Language")]
    Language,
}

impl SelectedTabs {
    pub fn prev(self) -> Self {
        let curr_idx: usize = self as usize;
        let index = curr_idx.saturating_sub(1);

        Self::from_repr(index).unwrap_or(self)
    }

    pub fn next(self) -> Self {
        let curr_idx: usize = self as usize;
        let index = curr_idx.saturating_add(1);

        Self::from_repr(index).unwrap_or(self)
    }

    pub fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    pub const fn palette(self) -> tailwind::Palette {
        match self {
            Self::DownloadFolder => tailwind::BLUE,
            Self::ConcurrentTotal => tailwind::EMERALD,
            Self::ChunkSize => tailwind::INDIGO,
            Self::MinimunSize => tailwind::YELLOW,
            Self::Language => tailwind::RED,
        }
    }
}
