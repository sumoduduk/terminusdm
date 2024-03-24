pub fn string_to_vec(input: &str) -> Vec<String> {
    input.split(',').map(|s| s.trim().to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_str() {
        let names = "alice,bob,       kelly,zerto";
        let names_vec: Vec<String> = string_to_vec(names);

        assert_eq!(
            vec![
                "alice".to_owned(),
                "bob".to_owned(),
                "kelly".to_owned(),
                "zerto".to_owned()
            ],
            names_vec
        );
    }
}
