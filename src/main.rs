#![warn(clippy::all, clippy::nursery, clippy::pedantic)]

use std::{env, process};

use minigrep::Config;

// TODO
// For another exercise on your own, try controlling case sensitivity through either
// a command line argument or an environment variable. Decide whether the command line
// argument or the environment variable should take precedence if the program is run
// with one set to case sensitive and one set to ignore case.

fn main() {
    let ignore_case = env::var("IGNORE_CASE").is_ok();    
    let args = env::args();
    
    let config = Config::build(args, ignore_case).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    };
}
