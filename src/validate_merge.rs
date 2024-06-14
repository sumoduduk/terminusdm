use std::path::Path;

use eyre::OptionExt;

use crate::{sort_files::sorting, FilePartSizeList};

pub async fn check(
    range_headers: &[(u64, u64)],
    path_parent: &Path,
    divisor: usize,
) -> eyre::Result<FilePartSizeList> {
    let files = sorting(path_parent, divisor).await?;

    let check_files = files.iter().zip(range_headers);

    let mut incomplete_files: FilePartSizeList = Vec::with_capacity(range_headers.len());

    for file_check in check_files {
        let target = file_check.1;
        let file = file_check.0;

        let target_size = (target.1 - target.0) + 1;
        let file_size = file.metadata()?.len();

        if target_size != file_size {
            let file_name = file
                .file_stem()
                .ok_or_eyre("Error: failed to get filename OS str")?
                .to_string_lossy()
                .to_string();

            incomplete_files.push((file_name, *target));
        }
    }

    Ok(incomplete_files)
}
