//! A library that alphabetizes a given file.
    use clap::{Error};
    use std::fs;
    use std::path::PathBuf;
    use anyhow::{ Result, Context };

    fn get_content(filepath: &PathBuf) -> Result<String> {
        let s = fs::read_to_string(filepath).with_context(|| format!("File not found: {:?}", filepath));
        s
    }
    
    fn split(content: &String) -> Vec<&str>  {
        content.split(" ").collect()
    }

    /// Alphabetizes a file given a valid file path.
    pub fn alphabetize(filepath: &PathBuf) -> Result<(), Error>  {
        let content: String = get_content(filepath).unwrap();
        let mut word_collection: Vec<&str> = split(&content);
        word_collection.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        let _s = fs::write(filepath, word_collection.join(" "))
            .with_context(|| format!("failed to write to file: {:?}", filepath));
        Ok(())
    }