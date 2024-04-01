mod begin_download;
pub mod config;
mod merge_file;
mod req_lib;
pub mod tui;
mod utils;
mod words;

use eyre::OptionExt;
use req_lib::HeaderObject;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use trauma::{download::Download, downloader::DownloaderBuilder};
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
    is_resumable: bool,
    sizes: u64,
    total_chunk: u64,
}

//need fix
impl HistoryDownload {
    async fn new(uri: &str, total_chunk: u64) -> eyre::Result<Self> {
        let header_obj = HeaderObject::new(uri).await?;
        let is_resumable = header_obj.is_ranges();
        let file_name = header_obj
            .get_filename()
            .ok_or_eyre("Error: Can't get file_name")?;

        let sizes = header_obj.get_sizes()?;

        //TODO : load total_chunk from config
        //
        let hist = Self {
            file_name,
            url: uri.to_string(),
            stage_download: DownloadStage::READY,
            is_resumable,
            total_chunk,
            sizes,
        };

        Ok(hist)
    }

    fn ref_array(&self) -> [String; 3] {
        [
            self.file_name.clone(),
            self.url.clone(),
            self.stage_download.to_string(),
        ]
    }

    fn url(&self) -> &str {
        &self.url
    }
}

pub async fn download_chunk(app: &mut AppTui, key: u32) -> eyre::Result<()> {
    let history = app.get_history(key)?.clone();
    let url = history.url();
    let is_resumable = &history.is_resumable;
    println!("Begin Download : {url}");

    //todo - load from config
    let dir_home = dirs::home_dir().ok_or_eyre("ERROR: failed to get home dir")?;
    let download_path = dir_home.join("Downloads").join("tdm");

    let size_min = &app.setting.minimum_size;
    let sizes = &history.sizes;
    let is_minimun = size_min * 1000 > *sizes;

    if !is_resumable || is_minimun {
        let downloder = vec![Download::try_from(url)?];
        let build = DownloaderBuilder::new()
            .directory(download_path.clone())
            .build();
        build.download(&downloder).await;
        app.update_stage(key, DownloadStage::COMPLETE);
        let _ = app.save_history();
    } else {
        let file_name = &history.file_name;
        let total_chunk = &history.total_chunk;
        let url = Url::parse(url)?;
        let number_concurrent = app.setting.concurrent_download;

        let ranges =
            create_range(*sizes, *total_chunk).ok_or_eyre("Error: divisor should be non-zero")?;

        let temp = download_path.join("temp").join(&file_name);

        app.update_stage(key, DownloadStage::DOWNLOADING);
        let _ = app.save_history();
        let res = start_download(temp.clone(), &url, &ranges, number_concurrent).await;

        if let Ok(_) = res {
            app.update_stage(key, DownloadStage::MERGING);
            let _ = app.save_history();
            merge(&temp, ranges.len(), &download_path, &file_name).await?;
            app.update_stage(key, DownloadStage::COMPLETE);
            let _ = app.save_history();
        }
    }
    Ok(())
}
