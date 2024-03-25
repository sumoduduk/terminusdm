mod begin_download;
mod merge_file;
mod req_lib;
pub mod tui;
mod utils;

use eyre::{eyre, OptionExt};
use req_lib::HeaderObject;
use serde::{Deserialize, Serialize};

use crate::{begin_download::start_download, merge_file::merge, utils::create_range};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DownloadStage {
    READY,
    DOWNLOADING,
    MERGING,
    COMPLETE,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryDownload {
    file_name: String,
    url: String,
    stage_download: DownloadStage,
}

pub async fn download_chunk(download_uri: &str) -> eyre::Result<()> {
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

    let download_path = dir_home.join("Downloads").join("tdm");

    let temp = download_path.join("temp").join(&file_name);

    let res = start_download(temp.clone(), &header_obj.get_url(), &ranges).await;

    if let Ok(_) = res {
        merge(&temp, ranges.len(), &download_path, &file_name).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const URI : &str = "https://huggingface.co/datasets/ym0v0my/Time_series_dataset/resolve/main/all_six_datasets.zip?download=true";

    #[tokio::test]
    async fn test_download() -> eyre::Result<()> {
        let ranges = download_chunk(&URI).await;
        dbg!(&ranges);
        assert!(ranges.is_ok());
        Ok(())
    }
}
