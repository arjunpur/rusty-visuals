use chrono::Local;
use nannou::prelude::*;
use std::path::PathBuf;

// capture_frame_to_path will capture the current frame to a file
// named after the currently executing executable.
pub fn capture_frame_to_path(app: &App) {
    let now = Local::now();
    let output_dir = "/home/arjun/Projects/rusty-visuals/outputs";
    let file_path: PathBuf = PathBuf::from(output_dir)
        // Capture all frames to a directory called `/<path_to_nannou>/art_circles`.
        .join(app.exe_name().unwrap())
        .join(format!("{}", now.format("%Y%m%d-%H:%M:%S")))
        .with_extension("png");
    app.main_window().capture_frame(file_path);
}
