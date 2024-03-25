use std::{
    collections::BTreeMap,
    fs::{self, create_dir_all, File},
    path::PathBuf,
};

use eyre::OptionExt;
use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum DownloadStage {
    READY,
    DOWNLOADING,
    MERGING,
    COMPLETE,
}

#[derive(Serialize, Deserialize)]
struct HistoryDownload {
    file_name: String,
    url: String,
    stage_download: DownloadStage,
}

#[derive(Serialize, Deserialize)]
struct Histories {
    history: BTreeMap<u32, HistoryDownload>,
}

impl Histories {
    fn new() -> Self {
        let dir_path = Self::check_config_folder()
            .map_err(|err| println!("ERROR : {err}"))
            .expect("ERROR: error creating config folder");
        let file_path = Self::check_config_file(dir_path)
            .map_err(|err| println!("ERROR : {err}"))
            .expect("ERROR: error creating config file");
        let file_path = File::open(file_path).expect("ERROR : Opening config file");
        let history: Histories = match from_reader(file_path) {
            Ok(hist) => hist,
            Err(_) => {
                let map_history = BTreeMap::new();
                Self {
                    history: map_history,
                }
            }
        };

        history
    }

    fn add_history(&mut self, download_history: HistoryDownload) {
        let last = self
            .history
            .last_key_value()
            .and_then(|l| Some(*l.0))
            .unwrap_or_default();

        self.history.insert(last, download_history);
    }

    fn save_history(&self) -> eyre::Result<()> {
        let file_path = Self::get_file_history()?;

        let pretty_config = PrettyConfig::new().depth_limit(4).enumerate_arrays(true);
        let history = &self.history;

        let pretty_str = to_string_pretty(history, pretty_config)?;

        fs::write(&file_path, pretty_str)?;

        Ok(())
    }

    fn get_file_history() -> eyre::Result<PathBuf> {
        let dir_config = dirs::config_dir().ok_or_eyre("ERROR: config directory not available")?;

        let config_file = dir_config.join("tdm").join("history.ron");

        Ok(config_file)
    }

    fn check_config_folder() -> eyre::Result<PathBuf> {
        let dir_config = dirs::config_dir().ok_or_eyre("ERROR: config directory not available")?;

        let config_file = dir_config.join("tdm");

        match config_file.try_exists() {
            Ok(_) => {}
            Err(_) => {
                create_dir_all(&config_file)?;
            }
        }

        Ok(config_file)
    }

    fn check_config_file(path: PathBuf) -> eyre::Result<PathBuf> {
        let file_path = path.join("history.ron");

        match file_path.try_exists() {
            Ok(_) => {}
            Err(_) => {
                let content = String::new();
                fs::write(&file_path, content)?;
            }
        }

        Ok(file_path)
    }
}
