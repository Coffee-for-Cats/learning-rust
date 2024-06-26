use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("not enough arguments!");
    //     }
    //     let query = args[1].clone();
    //     let filename = args[2].clone();
    //     let ignore_case = env::var("IGNORE_CASE").is_ok();
    //     Ok(Config {
    //         query,
    //         filename,
    //         ignore_case,
    //     })
    // }

    // recieves a mutable reference to something iterable.
    // returns a Result, or config object.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // minigrep <inputname> <outputname>
        // 1        2           3
        args.next();

        // args is iterable; iterable things has ".next"
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path."),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

// results in or nothing, or a Box with a dynamic error ???
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // lines returns array.
    // Filter returns iterrator
    // Collect execute and transform into string/array again.
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "/
Rust:
Safe, fast, productive.
Pick Three.
Duct tape";

        assert_eq!(vec!["Safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "/
Rust:
Safe, fast, productive.
Pick Three.
Trust Me";

        assert_eq!(
            vec!["Rust:", "Trust Me"],
            search_case_insensitive(query, contents)
        );
    }
}
