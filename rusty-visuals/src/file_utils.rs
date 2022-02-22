use chrono::Local;
use nannou::prelude::*;
use std::path::PathBuf;

const RUSTY_VISUALS_OUTPUT_DIR: &str = "/home/arjun/Projects/rusty-visuals/outputs";

// capture_frame_to_path will capture the current frame to a file
// named after the currently executing executable.
pub fn capture_frame_to_path(app: &App) {
    let file_path = get_timestamp_path_for_output(app);
    app.main_window().capture_frame(file_path);
}

pub fn create_app_output_dir_all(app: &App) {
    // Capture all frames to a directory called `/<path_to_nannou>/art_circles`.
    let dir_path: PathBuf = PathBuf::from(RUSTY_VISUALS_OUTPUT_DIR)
        .join(app.exe_name().unwrap());
    std::fs::create_dir_all(dir_path).unwrap();
}

pub fn get_timestamp_path_for_output(app: &App) -> PathBuf {
    let now = Local::now();
    let file_path: PathBuf = PathBuf::from(RUSTY_VISUALS_OUTPUT_DIR)
        // Capture all frames to a directory called `/<path_to_nannou>/art_circles`.
        .join(app.exe_name().unwrap())
        .join(format!("{}", now.format("%Y%m%d-%H:%M:%S")))
        .with_extension("png");
    file_path
}
