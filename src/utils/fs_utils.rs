use std::fs;
use std::path::PathBuf;

pub fn get_available_filename(mut filename: PathBuf) -> PathBuf {
    while filename.exists() {
        let name_file = filename.file_stem().unwrap().to_string_lossy();

        let ext = filename.extension().unwrap().to_string_lossy();

        let new_name = if name_file.ends_with(')') {
            let index_split = name_file.rfind('(').unwrap_or(name_file.len());
            let (prefix, suffix) = name_file.split_at(index_split);

            let new_suffix = match suffix
                .trim_start_matches('(')
                .trim_end_matches(')')
                .parse::<u32>()
            {
                Ok(num) => (num + 1).to_string(),
                Err(_) => "1".to_string(),
            };

            format!("{}({})", prefix, new_suffix)
        } else {
            format!("{name_file}(1)")
        };

        let new_filename = format!("{new_name}.{ext}",);
        filename.set_file_name(new_filename);
    }
    filename
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avail() {
        let filename = PathBuf::from("test/test.txt");

        let avail = get_available_filename(filename);
        let expect_name = PathBuf::from("test/test(1).txt");

        assert_eq!(expect_name, avail);
    }

    #[test]
    fn test_avail_2() {
        let filename = PathBuf::from("test/file.txt");

        let avail = get_available_filename(filename);
        let expect_name = PathBuf::from("test/file(4).txt");

        assert_eq!(expect_name, avail);
    }

    #[test]
    fn test_avail_3() {
        let filename = PathBuf::from("test/nonexist.txt");

        let avail = get_available_filename(filename);
        let expect_name = PathBuf::from("test/nonexist.txt");

        assert_eq!(expect_name, avail);
    }
}
