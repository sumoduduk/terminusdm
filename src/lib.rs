mod begin_download;
pub mod config;
mod merge_file;
mod req_lib;
mod sort_files;
pub mod tui;
mod utils;
mod validate_merge;
mod words;

use begin_download::re_download;
use eyre::OptionExt;
use req_lib::HeaderObject;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use tui::app::AppTui;
use validate_merge::check;

use crate::{
    begin_download::{
        start_download,
        trauma::{download::Download, downloader::DownloaderBuilder},
    },
    merge_file::merge,
    utils::create_range,
};

type RangeDownload = (u64, u64);
type FilePartSizeList = Vec<(String, RangeDownload)>;

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
        let real_url = Self::get_download_url(uri).await?;
        let header_obj = HeaderObject::new(real_url.clone()).await?;
        let is_resumable = header_obj.is_ranges();
        let file_name = header_obj
            .get_filename()
            .ok_or_eyre("Error: Can't get file_name")?;

        let sizes = match header_obj.get_sizes() {
            Ok(size) => size,
            Err(_) => 0,
        };

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

    async fn get_download_url(url: &str) -> eyre::Result<Url> {
        let res = reqwest::get(url).await?;
        Ok(res.url().to_owned())
    }

    async fn get_real_url(&self) -> eyre::Result<Url> {
        let res = reqwest::get(self.url()).await?;
        Ok(res.url().to_owned())
    }
}

pub async fn download_chunk(app: &mut AppTui, key: u32) -> eyre::Result<()> {
    let history = app.get_history(key)?.clone();
    let url = history.url();
    let is_resumable = &history.is_resumable;
    println!("Begin Download : {url}");

    let file_name = &history.file_name;

    let real_url = history.get_real_url().await?;

    let download_path = &app.setting.default_folder.clone();

    let size_min = &app.setting.minimum_size;
    let sizes = &history.sizes;
    let is_minimun = size_min * 1000 > *sizes;

    if !is_resumable || is_minimun {
        let downloder = vec![Download::new(&real_url, file_name, None)];
        let build = DownloaderBuilder::new()
            .directory(download_path.clone())
            .build();
        build.download(&downloder).await;
        app.update_stage(key, DownloadStage::COMPLETE);
        let _ = app.save_history();
    } else {
        let total_chunk = &history.total_chunk;
        let number_concurrent = app.setting.concurrent_download;

        let ranges =
            create_range(*sizes, *total_chunk).ok_or_eyre("Error: divisor should be non-zero")?;

        let temp = download_path.join("temp").join(&file_name);

        app.update_stage(key, DownloadStage::DOWNLOADING);
        let _ = app.save_history();
        let _ = start_download(temp.clone(), &real_url, &ranges, number_concurrent).await?;

        let mut list_incomplete = check(&ranges, &temp, ranges.len()).await?;

        while !list_incomplete.is_empty() {
            let _ =
                re_download(temp.clone(), &real_url, &list_incomplete, number_concurrent).await?;

            list_incomplete = check(&ranges, &temp, ranges.len()).await?;
        }

        app.update_stage(key, DownloadStage::MERGING);
        let _ = app.save_history();

        merge(&temp, ranges.len(), &download_path, &file_name).await?;

        app.update_stage(key, DownloadStage::COMPLETE);
        let _ = app.save_history();
    }
    Ok(())
}
