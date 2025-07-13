use std::{env, process};
use minigreprust::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In the file {}", config.file_path);

    if let Err(e) = minigreprust::run(config) {
        println!("Appliccation error: {e}");
        process::exit(1);
    }
    
}


