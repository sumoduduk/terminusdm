use std::path::Path;

use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::sort_files::sorting;
use crate::utils::fs_utils::get_available_filename;

pub async fn merge(
    out_folder: &Path,
    divisor: usize,
    parent: &Path,
    output_name: &str,
) -> eyre::Result<()> {
    println!("INFO: Start Merging");

    let file_paths = sorting(out_folder, divisor).await?;

    let output_file = parent.join(output_name);
    let final_file = get_available_filename(output_file);

    let mut file_output = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(final_file)
        .await?;

    for p in file_paths.iter() {
        let buff = fs::read(p).await?;
        file_output.write_all(&buff).await?;
    }

    let res = fs::remove_dir_all(out_folder).await;
    if res.is_err() {
        println!("{:#?}", res);
    }
    println!("INFO: Done Merging");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_merge() {
        let path_str = "test/downloads/temp/";
        let path = Path::new(path_str);

        let res = merge(path, 8, path, "output.zip").await;
        dbg!(&res);

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_bin_merge() {
        let home_dir = dirs::home_dir().unwrap();
        let path_str = "Downloads/tdm/temp/pytorch_model.bin/";
        let path = home_dir.join(path_str);

        let res = merge(&path, 16, &path, "output.bin").await;
        dbg!(&res);

        assert!(res.is_ok());
    }
}
