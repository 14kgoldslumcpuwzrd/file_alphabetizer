#[cfg(test)]
mod unit_tests {
    use std::path::PathBuf;
    use file_alphabetizer::lib;

    #[test]
    #[should_panic(expected = "File wasn't found: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }")]
    fn  should_return_error_if_file_doesnt_exist() {
        let file = PathBuf::new();
        lib::get_content(&file);
    }

    #[test]
    fn should_return_file_contents() {
        let file = PathBuf::from("src/test");
        assert_eq!(lib::get_content(&file), "this is a file")
    }

    #[test]
    fn should_split_content() {
        let content = String::from("Test string");
        let content_vec = vec!["Test", "string"];
        assert_eq!( lib::split(&content), content_vec);
    }
}