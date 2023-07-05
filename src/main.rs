use std::env;

use minigrep::run;

fn main() {

    let args: Vec<String> = env::args().collect();

    match run(args) {
        Ok(_) => {},
        Err(err) => print!("Error: {}", err),
    };
}
