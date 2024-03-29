use std::{fs::create_dir_all, path::PathBuf};

use eyre::OptionExt;

enum Language {
    English,
    Indonesia,
}

struct Config {
    default_folder: String,
    concurrent_download: usize,
    total_chunk: u64,
    language: Language,
    minimum_size: u64,
}

impl Config {
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

        let file_path = file_path.exists().then(|| file_path);

        file_path
    }
}
