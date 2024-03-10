use reqwest::header::HeaderValue;
use reqwest::{header, Url};
use std::path::PathBuf;
use trauma::download::Download;
use trauma::downloader::DownloaderBuilder;

use eyre::OptionExt;

pub async fn start_download(uri: &str, range_header: &[String]) -> eyre::Result<()> {
    let download_dirs = dirs::download_dir()
        .or(Some(PathBuf::from("$HOME/Downloads/")))
        .ok_or_eyre("Error : No Download Dir")?;

    let temp = download_dirs.join("temp");

    tokio::fs::create_dir_all(&temp).await?;

    let filename = temp
        .as_os_str()
        .to_str()
        .ok_or_eyre("Failed to parse os str")?;

    for (i, range) in range_header.iter().enumerate() {
        let filename = format!("{filename}/{i}");

        let dl = Download::new(&Url::parse(uri)?, &filename);
        let header_val = HeaderValue::from_str(&range)?;
        let builder = DownloaderBuilder::new()
            .header(header::RANGE, header_val)
            .build();
        let dl = &[dl];
        let summaries = builder.download(dl).await;
        dbg!(summaries);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{req_lib::HeaderObject, utils::create_range};
    use eyre::{eyre, OptionExt};

    const URI : &str = "https://huggingface.co/datasets/ym0v0my/Time_series_dataset/resolve/main/all_six_datasets.zip?download=true";

    #[tokio::test]
    async fn test_concurent_donwload() -> eyre::Result<()> {
        let header_obj = HeaderObject::new(&URI).await?;
        if !header_obj.is_ranges()? {
            return Err(eyre!("Not Resumable"));
        }

        let sizes = header_obj.get_sizes()?;
        dbg!(sizes);

        let ranges = create_range(sizes, 8).ok_or_eyre("Error: divisor should be non-zero")?;
        dbg!(&ranges);

        let summary = start_download(&URI, &ranges).await;
        dbg!(&summary);
        assert!(summary.is_ok());
        Ok(())
    }
}
