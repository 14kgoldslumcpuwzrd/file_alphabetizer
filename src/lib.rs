    use clap::{Parser, Error};
    use std::fs;
    use std::path::PathBuf;
    use anyhow::{ Result, Context };

    #[derive(Parser)]
    pub struct CLI {
        #[clap(parse(from_os_str))]
        pub filepath: std::path::PathBuf
    }

    pub fn get_content(filepath: &PathBuf) -> Result<String> {
        let s = fs::read_to_string(filepath).with_context(|| format!("File not found: {:?}", filepath));
        s
    }
    
    pub fn split(content: &String) -> Vec<&str>  {
        content.split(" ").collect()
    }

    pub fn alphabetize(filepath: &PathBuf) -> Result<(), Error>  {
        let content: String = get_content(filepath).unwrap();
        let mut word_collection: Vec<&str> = split(&content);
        word_collection.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        let _s = fs::write(filepath, word_collection.join(" "))
            .with_context(|| format!("failed to write to file: {:?}", filepath));
        Ok(())
    }