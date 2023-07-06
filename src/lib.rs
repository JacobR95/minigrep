use std::{fs, process};
use std::io::{BufRead, BufReader};
use std::error::Error;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let result = perform_search(config)?;
    println!("{}", result);

    return Ok(());
}

fn perform_search(command: Config) -> Result<String, &'static str> {

    let file = fs::File::open(command.file_path).unwrap_or_else(|err| {
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
