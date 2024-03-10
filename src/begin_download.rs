use reqwest::header::HeaderValue;
use reqwest::{header, Url};
use std::path::PathBuf;
use std::sync::Arc;
use trauma::download::Download;
use trauma::downloader::DownloaderBuilder;

use eyre::OptionExt;

pub async fn start_download(uri: &str, range_header: Vec<String>) -> eyre::Result<()> {
    let download_dirs = dirs::download_dir()
        .or(Some(PathBuf::from("$HOME/Downloads/")))
        .ok_or_eyre("Error : No Download Dir")?;

    let arc_temp = Arc::new(download_dirs.join("temp"));

    tokio::fs::create_dir_all(arc_temp.as_ref()).await?;

    let mut handles = Vec::with_capacity(range_header.len());

    let url = Url::parse(uri)?;

    let arc_url = Arc::new(url);

    for (i, range) in range_header.into_iter().enumerate() {
        let url = Arc::clone(&arc_url);
        let temp = Arc::clone(&arc_temp);

        let handle = tokio::spawn(multi_downoad(temp, i, url, range));

        handles.push(handle);
    }

    // for handle in handles {
    //     handles.aw
    // }

    Ok(())
}
async fn multi_downoad(temp: Arc<PathBuf>, i: usize, url: Arc<Url>, range: String) {
    let filename_str = temp
        .as_os_str()
        .to_str()
        .ok_or_eyre("Failed to parse os str");

    if let Ok(filename) = filename_str {
        let filename = format!("{filename}/{i}");

        let dl = Download::new(url.as_ref(), &filename);
        let header_val = HeaderValue::from_str(&range);

        if let Ok(header_val) = header_val {
            let builder = DownloaderBuilder::new()
                .header(header::RANGE, header_val)
                .build();
            let dl = &[dl];

            // let _ = builder.download(dl).await;
        };
    };
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

        let summary = start_download(&URI, ranges).await;
        dbg!(&summary);
        assert!(summary.is_ok());
        Ok(())
    }
}
