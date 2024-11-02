use clap::Parser;
use fractal_tree::Config;

fn main() {
    let config = Config::parse();

    fractal_tree::run(&config);
}
