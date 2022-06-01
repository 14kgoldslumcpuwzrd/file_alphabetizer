#[cfg(test)]
mod unit_tests {
    use std::path::PathBuf;
    use file_alphabetizer;

    #[test]
    #[should_panic]
    fn  should_return_error_if_file_doesnt_exist() {
        let file = PathBuf::new();
        file_alphabetizer::get_content(&file).unwrap();
    }

    #[test]
    fn should_return_file_contents() {
        let file = PathBuf::from("src/test");
        assert_eq!(file_alphabetizer::get_content(&file).unwrap(), "this is a file")
    }

    #[test]
    fn should_split_content() {
        let content = String::from("Test string");
        let content_vec = vec!["Test", "string"];
        assert_eq!(file_alphabetizer::split(&content), content_vec);
    }
}