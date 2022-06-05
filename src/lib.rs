//! A library that alphabetizes a given file.
use clap::{Error};
use std::{fs};
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
/// 
/// # Examples
/// 
/// Basic usage:
/// 
/// ```
/// use std::{ path::PathBuf };
/// 
/// let p = PathBuf::from("src/test");
/// file_alphabetizer::alphabetize(&p).unwrap();
/// 
/// ```
pub fn alphabetize(filepath: &PathBuf) -> Result<(), Error>  {
    let content: String = get_content(filepath).unwrap();
    let mut word_collection: Vec<&str> = split(&content);
    word_collection.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    let _s = fs::write(filepath, word_collection.join(" "))
        .with_context(|| format!("failed to write to file: {:?}", filepath));
    Ok(())
}

//  TESTS BELOW

#[test]
#[should_panic]
fn  should_return_error_if_file_doesnt_exist() {
    let file = PathBuf::new();
    get_content(&file).unwrap();
}

#[test]
fn should_return_file_contents() {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create("src/test").unwrap();
    let contents = String::from("this is a file");
    file.write(contents.as_bytes()).unwrap();
    let file = PathBuf::from("src/test");
    assert_eq!(get_content(&file).unwrap(), "this is a file")
}

#[test]
fn should_split_content() {
    let content = String::from("Test string");
    let content_vec = vec!["Test", "string"];
    assert_eq!(split(&content), content_vec);
}