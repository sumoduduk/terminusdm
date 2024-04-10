use crate::config::Language;

pub enum TextFooter {
    SpanEditing,
    NoteMain,
    NoteEditing,
    NoteSetting,
    NoteBase,
}

use Language::*;
use TextFooter::*;

impl TextFooter {
    pub fn load_text(&self, language: &Language) -> String {
        let text = match self {
            SpanEditing => match language {
                English => "Press TAB to Switch Pane",
                Indonesia => "Tekan Tab untuk berpindah Mode",
            },
            NoteMain => match language {
                English => "(q) to quit/Tab to switch/Space to select URL",
                Indonesia => "(q) untuk keluar/Tab untuk berpindah mode",
            },
            NoteEditing => match language {
                English => "(ESC) to cancel/(Tab) to switch boxes/ Enter to complete",
                Indonesia => "(ESC) untuk keluar/(Tab) untuk berpindah mode",
            },
            NoteSetting => match language {
                English => "(q) to quit/(Tab) to switch boxes",
                Indonesia => "(q) untuk keluar/Tab untuk berpindah mode",
            },
            NoteBase => match language {
                English => "(q) to quit",
                Indonesia => "(q) untuk keluar",
            },
        };

        text.to_owned()
    }
}
