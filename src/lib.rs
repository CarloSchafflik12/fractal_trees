pub mod canvas;

use canvas::Canvas;
use clap::Parser;
use std::error::Error;

#[derive(Parser)]
pub struct Config {
    /// Number of iterations
    #[arg(short, long, default_value_t = 15)]
    iterations: u32,

    /// Angle offset of each branch
    #[arg(short, long, default_value_t = 22.0)]
    angle: f64,

    /// Init branch length
    #[arg(short, long, default_value_t = 300.0)]
    branch_length: f64,

    /// Image resolution x
    #[arg(long, default_value_t = 2048)]
    resx: u32,

    /// Image resolution y
    #[arg(long, default_value_t = 2048)]
    resy: u32,
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new(config.resx, config.resy);

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
        1,
    );
    canvas.save("out.png");

    Ok(())
}

fn fractal_tree(
    canvas: &mut Canvas,
    root_x: u32,
    root_y: u32,
    branch_len: f64,
    branch_angle: f64,
    max_index_conf: u32,
    angle_conf: f64,
    index: u32,
) {
    let root_next_x = (root_x as f64 + (branch_angle.to_radians().sin() * branch_len)) as u32;
    let root_next_y = (root_y as f64 - (branch_angle.to_radians().cos() * branch_len)) as u32;

    // Draw current branch
    canvas.line(root_x, root_y, root_next_x, root_next_y);

    let branch_angle_left = branch_angle - angle_conf;
    let branch_angle_right = branch_angle + angle_conf;

    if index == max_index_conf {
        return;
    }

    // Draw next left branch
    fractal_tree(
        canvas,
        root_next_x,
        root_next_y,
        branch_len * 0.8,
        branch_angle_left,
        max_index_conf,
        angle_conf,
        index + 1,
    );

    // Draw next right branch
    fractal_tree(
        canvas,
        root_next_x,
        root_next_y,
        branch_len * 0.8,
        branch_angle_right,
        max_index_conf,
        angle_conf,
        index + 1,
    );
}
