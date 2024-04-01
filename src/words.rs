use crate::config::Language;
use Language::*;

pub enum WORDS {
    InputTitle,
    InputNavigation,
    ExitTitle,
    ExitContent,
    ConfigsContentFolder,
    ConfigsContentConcurent,
    ConfigsContentChunk,
    ConfigsContentMinimum,
    TabsInputTitle,
    TabsContentLang,
    TabsContentNormal,
    TabsContentEditing,
    ErrorPopup,
    TableNav,
    DownloadTitle,
    DownloadNav,
    DownlaodContent,
    DownloadPrepareTitle,
    DownloadLoading,
}

impl WORDS {
    pub fn load_text(&self, language: &Language) -> String {
        let word = match self {
            Self::InputTitle => match language {
                English => "Input URL",
                Indonesia => "Masukan URL",
            },
            Self::InputNavigation => match language {
                English => "Press ENTER to download | ESC to quit",
                Indonesia => "Tekan ENTER untuk mengunduh | ESC untuk keluar",
            },
            Self::ExitTitle => match language {
                English => "Press Y to exit | N to cancel",
                Indonesia => "Tekan Y untuk keluar | N untuk kembali",
            },
            Self::ExitContent => match language {
                English => "Would you like to quit? (y/n)",
                Indonesia => "Apa anda ingin keluar? (y/n)",
            },
            Self::ConfigsContentFolder => match language {
                English => "Default Folder (absolute path) :",
                Indonesia => "Letak Folder Download ",
            },
            Self::ConfigsContentConcurent => match language {
                English => "Number of Concurrent Download :",
                Indonesia => "Jumlah Unduhan Bersamaan",
            },
            Self::ConfigsContentChunk => match language {
                English => "Total Chunk Download per File :",
                Indonesia => "Jumlah Part Unduhan per File",
            },
            Self::ConfigsContentMinimum => match language {
                English => "Minimum File Size For Concurrent Download (in Kb)",
                Indonesia => "Ukuran File Minimum Untuk Unduhan Bersamaan (dalam Kb)",
            },
            Self::TabsInputTitle => match language {
                English => "Enter new value",
                Indonesia => "Masukan nilai baru",
            },
            Self::TabsContentLang => match language {
                English => "◄ ► to change tab | ▲ ▼  to select language | Press Enter to confirm",
                Indonesia => "◄ ► untuk mengganti tab | ▲ ▼  untuk memilih bahasa | Tekan Enter untuk melanjutkan",
            },
            Self::TabsContentNormal => match language {
                English => "◄ ► to change tab | Press E to edit ",
                Indonesia => "◄ ► untuk mengganti tab | Tekan E untuk mengedit",
            },
            Self::TabsContentEditing => match language {
                English => "Press Esc to cancel | Enter to confirm",
                Indonesia => "Tekan ESC untuk kembali | ENTER untuk melanjutkan",
            },
            Self::ErrorPopup => match language {
                English => "Press Enter to continue",
                Indonesia => "Tekan Enter untuk melanjutkan",
            },
            Self::TableNav => match language {
                English => "▲ ▼  to scroll | Press SPACE to select | Press ENTER to Re-Download",
                Indonesia => "▲ ▼  untuk scroll | Tekan SPACE untuk memilih | Tekan ENTER mendownload ulang",
            },
            Self::DownloadTitle => match language {
                English => "Begin Download",
                Indonesia => "Memulai Download",
            },
            Self::DownloadNav => match language {
                English => "Press Y to download | N to cancel",
                Indonesia => "Tekan Y untuk mengunduh | N untuk kembali",
            },
            Self::DownlaodContent => match language {
                English => "Would you like download from this url? (y/n) : ",
                Indonesia => "Apakah anda ingin mendownload dari url berikut? (y/n) : ",
            },
            Self::DownloadPrepareTitle => match language {
                English => "Begin Download",
                Indonesia => "Memulai Download",
            },
            Self::DownloadLoading => match language {
                English => "preparing to download, please wait...",
                Indonesia => "mempersiapkan untuk mengunduh, harap tunggu",
            },
        };

        word.to_owned()
    }
}
