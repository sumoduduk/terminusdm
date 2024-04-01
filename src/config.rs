use std::{
    fs::{self, create_dir_all, File},
    path::PathBuf,
};

use eyre::OptionExt;
use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Debug, Display)]
pub enum Language {
    #[strum(to_string = "English")]
    English,
    #[strum(to_string = "Indonesia")]
    Indonesia,
}

const CONFIG_FILENAME: &str = "tdm_config.ron";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub default_folder: PathBuf,
    pub concurrent_download: usize,
    pub total_chunk: u64,
    pub language: Language,
    pub minimum_size: u64,
}

impl Config {
    pub fn new() -> Self {
        let path_folder = Self::check_config_folder()
            .map_err(|err| println!("ERR : {err}"))
            .unwrap();

        match Self::check_config_file(path_folder, CONFIG_FILENAME) {
            Some(file_config) => {
                let file_path =
                    File::open(file_config).expect("ERROR : Error while open config folder");
                let conf: Config = from_reader(file_path)
                    .map_err(|err| println!("ERROR: {err}"))
                    .unwrap();
                conf
            }
            None => {
                let dir_home = dirs::home_dir()
                    .ok_or_eyre("ERROR: failed to get home dir")
                    .unwrap();
                let download_path = dir_home.join("Downloads").join("tdm");

                let default_config = Config {
                    default_folder: download_path,
                    concurrent_download: 4,
                    total_chunk: 16,
                    language: Language::English,
                    minimum_size: 2048,
                };

                Self::create_config(&default_config, CONFIG_FILENAME)
                    .expect("Error: create config file");
                default_config
            }
        }
    }

    pub fn update_default_folder(&mut self, path_str: &str) -> eyre::Result<()> {
        let path = PathBuf::from(path_str);

        if !path.exists() {
            create_dir_all(&path)?
        }

        self.default_folder = path;

        Ok(())
    }

    pub fn update_concurrent_download(&mut self, amount: &str) -> eyre::Result<()> {
        let num = amount.parse::<usize>()?;
        self.concurrent_download = num;

        Ok(())
    }

    pub fn update_chunk_size(&mut self, amount: &str) -> eyre::Result<()> {
        let chunk_size = amount.parse::<u64>()?;
        self.total_chunk = chunk_size;

        Ok(())
    }

    pub fn update_min_size(&mut self, amount: &str) -> eyre::Result<()> {
        let min_size = amount.parse::<u64>()?;
        self.minimum_size = min_size;

        Ok(())
    }

    pub fn change_languange(&mut self, lang: Language) {
        self.language = lang
    }

    fn create_config(conf: &Config, config_filename: &str) -> eyre::Result<()> {
        let file_path = Self::get_file_history(config_filename)?;
        let pretty_config = PrettyConfig::new().depth_limit(4).enumerate_arrays(true);
        let pretty_str = to_string_pretty(conf, pretty_config)?;

        fs::write(file_path, pretty_str)?;

        Ok(())
    }

    pub fn save_history(&self) -> eyre::Result<()> {
        self.save_inner_history(CONFIG_FILENAME)?;
        Ok(())
    }

    fn save_inner_history(&self, history_filename: &str) -> eyre::Result<()> {
        let file_path = Self::get_file_history(history_filename)?;
        let pretty_config = PrettyConfig::new().depth_limit(4).enumerate_arrays(true);
        let pretty_str = to_string_pretty(self, pretty_config)?;

        fs::write(file_path, pretty_str)?;

        Ok(())
    }

    fn get_file_history(history_filename: &str) -> eyre::Result<PathBuf> {
        let dir_config = dirs::config_dir().ok_or_eyre("ERROR: config directory not available")?;

        let config_file = dir_config.join("tdm").join(history_filename);

        Ok(config_file)
    }

    fn check_config_folder() -> eyre::Result<PathBuf> {
        let dir_config = dirs::config_dir().ok_or_eyre("ERROR: config directory not available")?;

        let config_file = dir_config.join("tdm");

        if !config_file.exists() {
            create_dir_all(&config_file)?;
        }

        Ok(config_file)
    }

    fn check_config_file(path: PathBuf, history_filename: &str) -> Option<PathBuf> {
        let file_path = path.join(history_filename);

        file_path.exists().then_some(file_path)
    }
}
