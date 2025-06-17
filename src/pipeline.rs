use anyhow::Result;
use opencv::{core, imgproc, prelude::*};

/// Convert image to grayscale and apply Canny edge detection.
pub fn apply_pipeline(image: &core::Mat) -> Result<core::Mat> {
    let mut gray = core::Mat::default();
    imgproc::cvt_color(image, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut edges = core::Mat::default();
    imgproc::canny(&gray, &mut edges, 50.0, 150.0, 3, false)?;

    Ok(edges)
}
