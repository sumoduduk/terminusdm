mod begin_download;
mod merge_file;
mod req_lib;
pub mod tui;
mod utils;

use eyre::{eyre, OptionExt};
use req_lib::HeaderObject;
use serde::{Deserialize, Serialize};
use tui::app::AppTui;

use crate::{begin_download::start_download, merge_file::merge, utils::create_range};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DownloadStage {
    READY,
    DOWNLOADING,
    MERGING,
    COMPLETE,
}

impl ToString for DownloadStage {
    fn to_string(&self) -> String {
        match self {
            DownloadStage::READY => String::from("READY"),
            DownloadStage::DOWNLOADING => String::from("DOWNLOADING"),
            DownloadStage::MERGING => String::from("MERGING"),
            DownloadStage::COMPLETE => String::from("COMPLETE"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryDownload {
    file_name: String,
    url: String,
    stage_download: DownloadStage,
}

//need fix
impl HistoryDownload {
    fn ref_array(&self) -> [String; 3] {
        [
            self.file_name.clone(),
            self.url.clone(),
            self.stage_download.to_string(),
        ]
    }

    fn file(&self) -> &str {
        &self.file_name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn status(&self) -> String {
        self.stage_download.to_string()
    }
}

pub async fn download_chunk(app: &mut AppTui, download_uri: &str) -> eyre::Result<()> {
    let header_obj = HeaderObject::new(download_uri).await?;
    if !header_obj.is_ranges()? {
        //todo still download even is not range
        return Err(eyre!("ERROR : File Download Not Resumable"));
    }

    let sizes = header_obj.get_sizes()?;

    let ranges = create_range(sizes, 16).ok_or_eyre("Error: divisor should be non-zero")?;

    let dir_home = dirs::home_dir().ok_or_eyre("ERROR: failed to get home dir")?;
    let file_name = header_obj
        .get_filename()
        .ok_or_eyre("Error: Can't get file_name")?;

    let history_download = HistoryDownload {
        file_name: file_name.clone(),
        url: download_uri.to_string(),
        stage_download: DownloadStage::READY,
    };

    let key = app.add_history(history_download);
    app.save_history();

    let download_path = dir_home.join("Downloads").join("tdm");

    let temp = download_path.join("temp").join(&file_name);

    app.update_stage(key, DownloadStage::DOWNLOADING);
    app.save_history();
    let res = start_download(temp.clone(), &header_obj.get_url(), &ranges).await;

    if let Ok(_) = res {
        app.update_stage(key, DownloadStage::MERGING);
        app.save_history();
        merge(&temp, ranges.len(), &download_path, &file_name).await?;
        app.update_stage(key, DownloadStage::COMPLETE);
        app.save_history();
    }

    Ok(())
}
