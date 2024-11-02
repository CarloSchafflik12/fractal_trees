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

    /// Print progress of generation
    #[arg(short, long)]
    progress: bool,
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
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
            1,
        );
    }
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

fn fractal_tree_progress(
    canvas: &mut Canvas,
    root_x: u32,
    root_y: u32,
    branch_len: f64,
    branch_angle: f64,
    max_index_conf: u32,
    angle_conf: f64,
    index: u32,
    lines_counter: &mut u32,
    max_lines: u32,
) {
    let root_next_x = (root_x as f64 + (branch_angle.to_radians().sin() * branch_len)) as u32;
    let root_next_y = (root_y as f64 - (branch_angle.to_radians().cos() * branch_len)) as u32;

    // Draw current branch
    print!(
        "\r Progress: {:.2}%                                              ",
        ((*lines_counter as f32 / max_lines as f32) * 100.0)
    );
    canvas.line(root_x, root_y, root_next_x, root_next_y);
    *lines_counter += 1;

    let branch_angle_left = branch_angle - angle_conf;
    let branch_angle_right = branch_angle + angle_conf;

    if index == max_index_conf {
        return;
    }

    // Draw next left branch
    fractal_tree_progress(
        canvas,
        root_next_x,
        root_next_y,
        branch_len * 0.8,
        branch_angle_left,
        max_index_conf,
        angle_conf,
        index + 1,
        lines_counter,
        max_lines,
    );

    // Draw next right branch
    fractal_tree_progress(
        canvas,
        root_next_x,
        root_next_y,
        branch_len * 0.8,
        branch_angle_right,
        max_index_conf,
        angle_conf,
        index + 1,
        lines_counter,
        max_lines,
    );
}
