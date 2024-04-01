use std::path::Path;

use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::utils::fs_utils::get_available_filename;

pub async fn merge(
    out_folder: &Path,
    divisor: usize,
    parent: &Path,
    output_name: &str,
) -> eyre::Result<()> {
    println!("INFO: Start Merging");
    let mut dir_entry = fs::read_dir(out_folder).await?;

    let mut file_paths = Vec::with_capacity(divisor);

    while let Some(file_chunk) = dir_entry.next_entry().await? {
        let path_file = file_chunk.path();

        if path_file.is_file() {
            let ext = path_file.extension();
            if ext.is_none() {
                file_paths.push(path_file);
            }
        }
    }

    file_paths.sort_by_key(|p| {
        let str_path = p.file_name().and_then(|os_str| os_str.to_str());

        let mut num = 0;

        match str_path {
            Some(file_str) => match file_str.parse::<u32>() {
                Ok(num_res) => num = num_res,
                Err(err) => println!("ERROR: {err}"),
            },
            None => println!("ERROR: failed to conver os str to str"),
        }

        num
    });

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
        let path_str = "/home/calista/Downloads/tdm/temp/pytorch_model.bin/";
        let path = Path::new(path_str);

        let res = merge(path, 16, path, "output.bin").await;
        dbg!(&res);

        assert!(res.is_ok());
    }
}
