use std::{fs, env};
use std::error::Error;

static NOT_ENOUGH_ARGS_ERR: &str = "Not enough arguments";

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err(NOT_ENOUGH_ARGS_ERR);
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = match args.get(3) {
            Some(value) => value == "-i",
            None => env::var("IGNORE_CASE").is_ok(),
        };

        return Ok(Config {
            query,
            file_path,
            ignore_case,
        });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }

    return matches;
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {

    let lowercase_query = &query.to_lowercase();
    let mut matches = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(lowercase_query) {
            matches.push(line);
        }
    }

    return matches;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;

    let results: Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    return Ok(());
}

#[cfg(test)]
mod search_tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn should_fail() {
        let result_no_args = Config::build(&vec![]);
        let result_one_arg = Config::build(&vec![String::from("arg1")]);
        let result_two_args = Config::build(&vec![String::from("arg1"), String::from("arg2")]);

        assert!(result_no_args.is_err(), "{}", crate::NOT_ENOUGH_ARGS_ERR);
        assert!(result_one_arg.is_err(), "{}", crate::NOT_ENOUGH_ARGS_ERR);
        assert!(result_two_args.is_err(), "{}", crate::NOT_ENOUGH_ARGS_ERR);
    }

    #[test]
    fn case_insensitive_arg() {
        let args = vec![
            String::from("arg"),
            String::from("man"),
            String::from("./example"),
            String::from("-i")
        ];
        let config = Config::build(&args).unwrap();

        assert_eq!(config.ignore_case, true);
    }
}
