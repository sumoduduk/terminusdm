mod history;
mod table;
pub mod tabs_state;

use crate::{
    config::{self, Config, Language},
    utils::to_vec::string_to_vec,
    DownloadStage, HistoryDownload,
};
use indexmap::IndexMap;
use ratatui::widgets::ListState;
use tui_input::Input;

use self::{history::Histories, tabs_state::SelectedTabs};
use table::Table;

const HISTORY_FILE_NAME: &str = "history.ron";

pub enum CurrentScreen {
    Main,
    Editing,
    Setting,
    Exiting,
    ErrorScreen,
}

pub enum InputMode {
    Normal,
    Editing,
}

pub struct AppTui {
    pub input_uri: Input,
    pub input_mode: InputMode,
    pub curr_screen: CurrentScreen,
    pub saved_input: Vec<u32>,
    pub error_msg: String,
    pub history: Histories,
    pub table: Table,
    pub selected_tabs: SelectedTabs,
    pub setting: Config,
    pub tab_content_input: Input,
    pub tab_content_mode: InputMode,
    pub lang_state: ListState,
}

impl AppTui {
    pub fn new(config_setting: Config) -> Self {
        let chunk = &config_setting.total_chunk;
        let histo = Histories::new(HISTORY_FILE_NAME);
        let len_histo = histo.len();

        Self {
            input_uri: Input::default(),
            input_mode: InputMode::Normal,
            curr_screen: CurrentScreen::Main,
            saved_input: Vec::new(),
            history: histo,
            table: Table::new(len_histo),
            error_msg: String::new(),
            selected_tabs: SelectedTabs::default(),
            setting: config_setting,
            tab_content_input: Input::default(),
            tab_content_mode: InputMode::Normal,
            lang_state: ListState::default(),
        }
    }

    pub fn next_lang(&mut self) {
        let i = match self.lang_state.selected() {
            Some(i) => {
                if i >= 2 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.lang_state.select(Some(i));
    }

    pub fn prev_lang(&mut self) {
        let i = match self.lang_state.selected() {
            Some(i) => {
                if i == 0 {
                    1
                } else {
                    i - 1
                }
            }
            None => 0, // self.last_selected.unwrap_or(0),
        };
        self.lang_state.select(Some(i));
    }

    pub fn update_config(&mut self, input_str: &str, lang: Option<Language>) -> eyre::Result<()> {
        match self.selected_tabs {
            SelectedTabs::DownloadFolder => {
                self.setting.update_default_folder(input_str)?;
            }
            SelectedTabs::ConcurrentTotal => {
                self.setting.update_concurrent_download(input_str)?;
            }
            SelectedTabs::ChunkSize => {
                self.setting.update_chunk_size(input_str)?;
            }
            SelectedTabs::MinimunSize => {
                self.setting.update_min_size(input_str)?;
            }
            SelectedTabs::Language => {
                if let Some(lang) = lang {
                    self.setting.change_languange(lang);
                }
            }
        }
        self.setting.save_history();
        Ok(())
    }

    pub fn load_pick(&mut self) {
        let indexes = &self.table.picked;
        let nums = indexes.iter().for_each(|idx| {
            let hist = self.history.get_history_by_idx(*idx);
            match hist {
                Ok((num, _)) => self.saved_input.push(*num),
                Err(_) => (),
            }
        });

        self.table.picked.clear();
    }

    pub async fn save_input(&mut self) -> eyre::Result<()> {
        let input_value = self.input_uri.value();
        let chunks = self.setting.total_chunk;

        // TODO
        if input_value.contains(',') {
            let vec_str = string_to_vec(input_value);
            for uri in vec_str {
                let histo = HistoryDownload::new(&uri, chunks).await?;
                let num = self.add_history(histo);
                self.saved_input.push(num);
            }
        } else {
            let histo = HistoryDownload::new(input_value, chunks).await?;
            let num = self.add_history(histo);
            self.saved_input.push(num);
        }

        self.input_uri.reset();
        Ok(())
    }

    pub fn print_vec(&self) -> eyre::Result<()> {
        let output = serde_json::to_string_pretty(&self.saved_input)?;
        println!("{}", output);
        Ok(())
    }

    pub fn list_history(&self) -> &IndexMap<u32, HistoryDownload> {
        self.history.list()
    }

    pub async fn push_to_table(&mut self) {
        let uri = self.input_uri.value();
        let chunks = self.setting.total_chunk;

        let history_download = HistoryDownload::new(uri, chunks).await;
        match history_download {
            Ok(history_download) => {
                self.add_history(history_download);
            }
            Err(err) => self.set_error_msg(err.to_string()),
        }
        self.input_uri.reset();
    }

    pub fn add_history(&mut self, download_history: HistoryDownload) -> u32 {
        let key = self.history.add_history(download_history);
        let len = self.history.len();
        self.table.total_len = len;
        key
    }

    pub fn update_stage(&mut self, num: u32, stage: DownloadStage) {
        self.history.update_stage(num, stage);
    }

    pub fn get_history(&self, num: u32) -> eyre::Result<&HistoryDownload> {
        let res = self.history.get_history(num)?;
        Ok(res)
    }

    pub fn save_history(&self) -> eyre::Result<()> {
        self.history.save_history(HISTORY_FILE_NAME)?;
        Ok(())
    }

    pub fn set_error_msg(&mut self, msg: String) {
        self.error_msg = msg;
        self.curr_screen = CurrentScreen::ErrorScreen;
    }

    pub fn clear_error_msg(&mut self) {
        self.error_msg = String::new();
    }

    pub fn clear_saved_input(&mut self) {
        self.saved_input.clear();
    }

    pub fn next_tab(&mut self) {
        self.selected_tabs = self.selected_tabs.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tabs = self.selected_tabs.prev();
    }
}
