use ratatui::{
    style::{palette::tailwind, Color},
    widgets::{ScrollbarState, TableState},
};

const ITEM_HEIGHT: usize = 3;

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

pub struct TableColors {
    pub buffer_bg: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub selected_style_fg: Color,
    pub normal_row_color: Color,
    pub alt_row_color: Color,
    pub footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

pub struct Table {
    pub state: TableState,
    pub scroll_state: ScrollbarState,
    pub colors: TableColors,
    pub total_len: usize,
}

impl Table {
    pub fn new(len: usize) -> Self {
        Self {
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new(len),
            colors: TableColors::new(&PALETTES[0]),
            total_len: len,
        }
    }

    pub fn next(&mut self) {
        let index = match self.state.selected() {
            Some(i) => {
                if i >= self.total_len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(index));
        self.scroll_state = self.scroll_state.position(index + ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let index = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.total_len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(index));
        self.scroll_state = self.scroll_state.position(index + ITEM_HEIGHT);
    }
}
