use std::env;
use std::fs;

    // Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
    // As long as your command line parsing logic is small, it can remain in main.rs.
    // When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
    

fn main() {
    let args: Vec<String> = env::args().collect();    
    let (query, file_path) = parse_config(&args);

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    // ignoring args[0] - file path
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}