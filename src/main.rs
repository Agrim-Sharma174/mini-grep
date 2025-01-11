use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let file_path = &args[2];
    // let config = parse_config(&args);

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In the file {}", config.file_path);

    let content = fs::read_to_string(config.file_path).expect("Should have been able to read the code.");

    println!("The text is:\n{content}");

}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config {query, file_path}
}