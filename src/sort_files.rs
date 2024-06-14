use std::path::{Path, PathBuf};

pub async fn sorting(out_folder: &Path, divisor: usize) -> eyre::Result<Vec<PathBuf>> {
    let mut dir_entry = tokio::fs::read_dir(out_folder).await?;

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
        let num = p
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .and_then(|str| str.parse::<u32>().ok())
            .expect("ERROR : need file to be number");

        num
    });

    Ok(file_paths)
}
