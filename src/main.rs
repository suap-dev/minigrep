#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ignore_case = env::var("IGNORE_CASE").is_ok();

    let config = Config::build(&args, ignore_case).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    };
}
