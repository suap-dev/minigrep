#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use std::env;
use std::error::Error;
use std::fs;
use std::process;

// Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
// As long as your command line parsing logic is small, it can remain in main.rs.
// When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(err) = run(config) {
        println!("Application error: {err}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { query, file_path })
    }
}
