use super::Canvas;

pub fn fractal_tree(
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

pub fn fractal_tree_progress(
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
