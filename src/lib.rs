use std::{env, fs, result};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config { query, file_path, ignore_case, })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content =
        fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(query: &str, content:&'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "the great Agrim.";
        let content = "\
Rust:
Here we are, with
the great Agrim.";
        assert_eq!(vec!["the great Agrim."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "the Great Agrim.";
        let content = "\
Rust:
Here we are, with
the great Agrim.";
        assert_eq!(vec!["the great Agrim."], search_case_insensitive(query, content));
    }
}