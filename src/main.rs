use std::fs;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    println!("In file {}", file);

    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    let mut word_collection: Vec<&str> = contents.split(" ").collect();
    println!("{:?}", word_collection);

    // Implement merge sort
    word_collection.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    fs::write(file, word_collection.join(" ")).expect("error within file reconstruction");
}
