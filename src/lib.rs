use std::fs::File;
use std::process;
use std::io::{BufRead, BufReader};
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

fn perform_search(command: Config) -> Result<String, &'static str> {

    let file = File::open(command.file_path).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let reader = BufReader::new(file);

    for l in reader.lines() {
        let line = l.unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
        if line.contains(command.query.as_str()) {
            return Ok(format!("Found '{:?}' in line: {:?}", command.query, line));
        }
    }

    return Ok(String::from("No matches found"));
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let result = perform_search(config)?;
    println!("{}", result);

    return Ok(());
}

#[cfg(test)]
mod config_tests {
    use crate::Config;

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

#[cfg(test)]
mod run_tests {
    use crate::Config;

    #[test]
    fn it_should_fail_with_less_then_three_args() {

    }
}




































