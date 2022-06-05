![Github CI](https://github.com/lasagnamassage/file_alphabetizer/actions/workflows/rust.yml/badge.svg)

# file_alphabetizer
A library that alphabetizes a given file. Can be used as a crate in your own applications or (soon) as a standalone binary

## Usage

**Within a Rust project**
1. Import the library into your Cargo.toml like so:
    ```Rust
    [dependencies] 
        file_alphabetizer = "1.0"
    ```
2. Import the crate and use the alphabetize method, like so.
    (Currently, path starts at parent directory of code calling it):
    ```rust
        use std::{path::PathBuf};
        use file_alphabetizer;

        fn main() {
            let path = PathBuf::from("src/test");
            file_alphabetizer::alphabetize(&path).unwrap();
        }
    ```


**As a CLI tool**
1. Run `cargo run` from the command line, within the project's directory.
