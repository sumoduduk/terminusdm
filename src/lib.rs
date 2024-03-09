mod begin_download;
mod file_fn;
mod req_lib;
mod utils;

use eyre::{eyre, OptionExt};
use req_lib::HeaderObject;

use crate::utils::create_range;

pub async fn download_chunk(download_uri: &str) -> eyre::Result<()> {
    let header_obj = HeaderObject::new(download_uri).await?;
    if !header_obj.is_ranges()? {
        return Err(eyre!("Not Resumable"));
    }

    let sizes = header_obj.get_sizes()?;
    dbg!(sizes);

    let ranges = create_range(sizes, 8).ok_or_eyre("Error: divisor should be non-zero")?;
    dbg!(ranges);

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
