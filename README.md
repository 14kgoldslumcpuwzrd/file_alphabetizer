![Github CI](https://github.com/lasagnamassage/file_alphabetizer/actions/workflows/rust.yml/badge.svg)

# file_alphabetizer
An library that alphabetizes a given file. Can be used as a crate in your own applications or as a standalone binary

## Usage

**Within a Rust project**
    1. Import the library into your Cargo.toml like so:
        `[dependencies]
        `   file_alphabetizer = "1.0"`
    2. Import the crate and use the alphabetize method, like so.
        (Currently, path starts at parent directory of code calling it)
        ```
            use std::{path::PathBuf};
            use file_alphabetizer;

            let path = PathBuf::from("src/test");
            file_alphabetizer::alphabetize(&path).unwrap();
        ```


**As a CLI tool**
Run `cargo run` from the command line, within the project's directory.
