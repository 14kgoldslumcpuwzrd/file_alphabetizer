use std::fs;
use clap::Parser;
use file_alphabetizer::lib;

fn main() {

    let args: lib::CLI = lib::CLI::parse();
    let file = &args.filepath;
    let content: String = lib::get_content(&args.filepath);
    let mut word_collection: Vec<&str> = lib::split(&content);

    word_collection.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    fs::write(file, word_collection.join(" ")).expect("error within file reconstruction");
}

