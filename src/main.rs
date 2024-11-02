use std::env;
use std::process;

use fractal_tree::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Parsing error: {err}");
        process::exit(1);
    });

    if let Err(e) = fractal_tree::run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
