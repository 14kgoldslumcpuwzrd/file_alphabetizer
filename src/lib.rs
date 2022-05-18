pub mod lib {
    use clap::Parser;
    use std::fs;
    use std::path::PathBuf;

    #[derive(Parser)]
    pub struct CLI {
        #[clap(parse(from_os_str))]
        pub filepath: std::path::PathBuf
    }

    pub fn get_content(filepath: &PathBuf) -> String {
        fs::read_to_string(filepath).expect("File wasn't found")
    }
    
    pub fn split(content: &String) -> Vec<&str>  {
        content.split(" ").collect()
    }
}