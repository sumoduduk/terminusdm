use reqwest::Url;
use std::path::PathBuf;
use trauma::download::{Download, Summary};
use trauma::downloader::DownloaderBuilder;

pub async fn start_download(
    temp: PathBuf,
    uri: &Url,
    range_header: &[(u64, u64)],
    num_concur: usize,
) -> eyre::Result<Vec<Summary>> {
    tokio::fs::create_dir_all(&temp).await?;

    let mut batch_dl = Vec::with_capacity(range_header.len());

    for (i, range) in range_header.iter().enumerate() {
        let dl = Download::new(uri, &i.to_string(), Some(*range));
        batch_dl.push(dl);
    }

    let begin = DownloaderBuilder::new()
        .directory(temp)
        .concurrent_downloads(num_concur)
        .build();

    let summary = begin.download(&batch_dl).await;

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{merge_file::merge, req_lib::HeaderObject, utils::create_range};
    use eyre::{eyre, OptionExt};

    const URI : &str = "https://huggingface.co/datasets/ym0v0my/Time_series_dataset/resolve/main/all_six_datasets.zip?download=true";

    #[test]
    fn test_dirs() {
        if let Some(dir) = dirs::home_dir() {
            let down = dir.join("Downloads");
            dbg!(down);
        };

        assert_eq!(true, true);
    }

    #[tokio::test]
    async fn test_concurent_donwload() -> eyre::Result<()> {
        let download_dirs = dirs::download_dir()
            .or(Some(PathBuf::from("test/Downloads/")))
            .ok_or_eyre("Error : No Download Dir")?;

        let temp = download_dirs.join("temp");

        let header_obj = HeaderObject::new(&URI).await?;
        if !header_obj.is_ranges()? {
            return Err(eyre!("Not Resumable"));
        }
        let uri = header_obj.get_url();

        let sizes = header_obj.get_sizes()?;
        dbg!(sizes);
        let divisor = 8;

        let ranges =
            create_range(sizes, divisor).ok_or_eyre("Error: divisor should be non-zero")?;
        let out_file = "output.zip";

        let summary = start_download(temp.clone(), uri, &ranges, 4).await;
        assert!(summary.is_ok());

        let res = merge(&temp, divisor as usize, &temp, out_file).await;

        dbg!(&res);
        assert!(res.is_ok());

        Ok(())
    }
}
