use std::path::Path;

use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn merge(out_folder: &Path, divisor: usize, output_name: &str) -> eyre::Result<()> {
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

    file_paths.sort();

    let output_name = out_folder.join(output_name);

    let mut file_output = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_name)
        .await?;

    // let mut writer = BufWriter::new(&mut file_output);
    for p in file_paths.iter() {
        let buff = fs::read(p).await?;
        file_output.write_all(&buff).await?;

        // writer.write(&buff).await?;

        println!("done write");
    }

    // writer.flush().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_merge() {
        let path_str = "test/Downloads/temp/";
        let path = Path::new(path_str);

        let res = merge(path, 8, "output.zip").await;
        dbg!(&res);

        assert!(res.is_ok());
    }
}
