mod history;
mod table;

use crate::{utils::to_vec::string_to_vec, DownloadStage, HistoryDownload};
use indexmap::IndexMap;
use tui_input::Input;

use self::history::Histories;
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
    pub saved_input: Vec<String>,
    pub error_msg: String,
    pub history: Histories,
    pub table: Table,
}

impl AppTui {
    pub fn new() -> Self {
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
        }
    }

    pub fn save_input(&mut self) {
        let input_value = self.input_uri.value();

        let down_histo = HistoryDownload {
            file_name: input_value.to_owned(),
            url: "https:download.com/2sadw".to_owned(),
            stage_download: DownloadStage::DOWNLOADING,
            is_resumable: false,
            sizes: 1000,
            total_chunk: 16,
        };

        self.add_history(down_histo);

        //TODO
        // if input_value.contains(',') {
        //     let mut vec_str = string_to_vec(input_value);
        //     self.saved_input.append(&mut vec_str);
        // } else {
        //     self.saved_input.push(input_value.into());
        // }

        self.input_uri.reset();
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

        let history_download = HistoryDownload::new(uri).await;
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
}
