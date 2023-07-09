use std::{fs, env};
use std::error::Error;
use regex::RegexBuilder;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {

    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = match args.next() {
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

    return contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| highlight_query(line, query, false))
        .collect();
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {

    let lowercase_query = &query.to_lowercase();
    return contents
        .lines()
        .filter(|line| line.to_lowercase().contains(lowercase_query))
        .map(|line| highlight_query(line, query, true))
        .collect();
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

fn highlight_query<'a>(
    line: &'a str,
    query: &str,
    case_sensitive: bool
) -> &'a str {

    let query_regex = RegexBuilder::new(&format!("{}", query))
        .case_insensitive(case_sensitive)
        .build()
        .unwrap();

    let highlighted = query_regex.replace_all(line, "\x1b[31m$0\x1b[0m");
    let highlighted_ref: &str = Box::leak(highlighted.into_owned().into_boxed_str());

    return highlighted_ref;
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
        let result_no_args = Config::build(Vec::new().into_iter());
        let result_one_arg = Config::build(vec![String::from("arg1")].into_iter());
        let result_two_args = Config::build(vec![String::from("arg1"), String::from("arg2")].into_iter());

        assert!(result_no_args.is_err(), "{}", "Didn't get a query string");
        assert!(result_one_arg.is_err(), "{}",  "Didn't get a query string");
        assert!(result_two_args.is_err(), "{}", "Didn't get a file path");
    }

    #[test]
    fn case_insensitive_arg() {
        let args = vec![
            String::from("arg"),
            String::from("man"),
            String::from("./example"),
            String::from("-i")
        ].into_iter();
        let config = Config::build(args).unwrap();

        assert_eq!(config.ignore_case, true);
    }
}
