#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use std::env;
use std::fs;

// Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
// As long as your command line parsing logic is small, it can remain in main.rs.
// When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Self { query, file_path }
    }
}
