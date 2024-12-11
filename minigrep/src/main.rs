use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsion arguments: {err}");
        process::exit(1);
    });

    println!("File: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Error while reading the file.");

    println!("Content: {contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("Not enough arguments.");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
