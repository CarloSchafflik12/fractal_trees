pub mod canvas;
pub mod tree_gen;

use canvas::Canvas;
use clap::Parser;
use tree_gen::{fractal_tree, fractal_tree_progress};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Path of output image
    #[arg(long, default_value_t = String::from("out.png"))]
    path: String,

    /// Number of iterations
    #[arg(short, long, default_value_t = 15)]
    iterations: u32,

    /// Angle offset of each branch
    #[arg(short, long, default_value_t = 22.0)]
    angle: f64,

    /// Init branch length
    #[arg(short, long, default_value_t = 300.0)]
    branch_length: f64,

    /// Ratio of branch length
    #[arg(short, long, default_value_t = 0.8)]
    ratio: f64,

    /// Image resolution x
    #[arg(long, default_value_t = 2048)]
    resx: u32,

    /// Image resolution y
    #[arg(long, default_value_t = 2048)]
    resy: u32,

    /// Print progress of generation
    #[arg(short, long)]
    progress: bool,
}

pub fn run(config: &Config) {
    let mut canvas = Canvas::new(config.resx, config.resy);

    // Progress mode
    if config.progress {
        let mut n_lines = 0;
        for _ in 0..config.iterations {
            n_lines = n_lines * 2 + 1;
        }
        let tree_x = canvas.width / 2;
        let tree_y = canvas.height - 1;
        let mut counter = 0;
        fractal_tree_progress(
            &mut canvas,
            tree_x,
            tree_y,
            config.branch_length,
            0.0,
            config.iterations,
            config.angle,
            config.ratio,
            1,
            &mut counter,
            n_lines,
        );
        println!();
    } else {
        let tree_x = canvas.width / 2;
        let tree_y = canvas.height - 1;
        fractal_tree(
            &mut canvas,
            tree_x,
            tree_y,
            config.branch_length,
            0.0,
            config.iterations,
            config.angle,
            config.ratio,
            1,
        );
    }
    canvas.save(config.path.as_str());
}
