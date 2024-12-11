use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let file_path = match args.get(1) {
        Some(file_path) => file_path,
        None => "No file path given.",
    };

    println!("File: {file_path}");

    let contents = fs::read_to_string(file_path).expect("Error while reading the file.");

    println!("Content: {contents}");
}
