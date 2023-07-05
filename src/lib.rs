use std::fs;
use std::io::{BufRead, BufReader, Error};

pub fn run(args: Vec<String>) -> Result<(), Error> {

    let command = get_grep_command(args);
    perform_search(command)?;

    return Ok(());
}

#[derive(Debug, Clone)]
pub struct MiniGrepError;

struct GrepCommand {
    query: String,
    file_name: String,
}

fn get_grep_command(args: Vec<String>) -> GrepCommand {

    if args.len() == 1 {
        panic!("No arguments provided.");
    }
    if args.len() == 2 {
        panic!("Only one argument was provided but 2 are required.");
    }

    let query = args[1].clone();
    let file_name = args[2].clone();

    return GrepCommand { query, file_name, };
}

fn perform_search(command: GrepCommand) -> Result<(), Error> {

    let file = fs::File::open(command.file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(command.query.as_str()) {
            println!("Found '{}' in line: {}", command.query, line);
        }
    }

    return Ok(());
}
