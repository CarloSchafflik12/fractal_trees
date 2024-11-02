use std::process;

use clap::Parser;
use fractal_tree::Config;

fn main() {
    let config = Config::parse();

    if let Err(e) = fractal_tree::run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
