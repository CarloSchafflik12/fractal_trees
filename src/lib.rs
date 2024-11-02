pub mod canvas;

use canvas::Canvas;
use std::error::Error;

pub struct Config {
    pub iterations: u32,
    pub angle: f64,
    pub res_xy: u32,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("program needs 2 arguments --> <iterations> <angle>");
        }

        let iterations = match args[1].parse::<u32>() {
            Ok(n) => n,
            Err(_) => return Err("number of iterations not valid"),
        };
        if iterations < 1 {
            return Err("number of iterations must be above 0");
        }

        let angle = match args[2].parse::<f64>() {
            Ok(n) => n,
            Err(_) => return Err("angle not valid"),
        };

        Ok(Config {
            iterations,
            angle,
            res_xy: 1024,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new(2048, 2048);

    /*
    let mut rng = thread_rng();
    for _ in 0..5 {
        let x1: u32 = rng.gen_range(0..config.res_xy);
        let y1: u32 = rng.gen_range(0..config.res_xy);
        let x2: u32 = rng.gen_range(0..config.res_xy);
        let y2: u32 = rng.gen_range(0..config.res_xy);
        canvas.line(x1, y1, x2, y2);
    }
    */
    let tree_x = canvas.width / 2;
    let tree_y = canvas.height - 1;
    fractal_tree(
        &mut canvas,
        tree_x,
        tree_y,
        300.0,
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
