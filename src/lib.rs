use std::fs;
use std::process;
use std::error::Error;

static NOT_ENOUGH_ARGS_ERR: &str = "Not enough arguments";

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err(NOT_ENOUGH_ARGS_ERR);
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    return Ok(());
}

#[cfg(test)]
mod search_tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn it_should_fail_with_less_then_three_args() {
        let result_no_args = Config::build(&vec![]);
        let result_one_arg = Config::build(&vec![String::from("arg1")]);
        let result_two_args = Config::build(&vec![String::from("arg1"), String::from("arg2")]);

        assert!(result_no_args.is_err(), "{}", crate::NOT_ENOUGH_ARGS_ERR);
        assert!(result_one_arg.is_err(), "{}", crate::NOT_ENOUGH_ARGS_ERR);
        assert!(result_two_args.is_err(), "{}", crate::NOT_ENOUGH_ARGS_ERR);
    }
}
