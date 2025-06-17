use anyhow::Result;
use opencv::{prelude::*, videoio};

/// Initialize video capture from a file.
pub fn init_video(path: &str) -> Result<videoio::VideoCapture> {
    let cam = videoio::VideoCapture::from_file(path, videoio::CAP_ANY)?;
    Ok(cam)
}

/// Read a frame from the video.
pub fn capture_frame(cam: &mut videoio::VideoCapture) -> Result<opencv::core::Mat> {
    let mut frame = opencv::core::Mat::default();
    cam.read(&mut frame)?;
    if frame.empty() {
        std::process::exit(0); // End of video
    }
    Ok(frame)
}
