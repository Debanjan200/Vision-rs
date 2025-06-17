use anyhow::Result;
use opencv::{core, highgui, imgproc, prelude::*};
use std::fs;

mod pipeline;
mod telemetry;
mod tracker;
mod video;
mod test;

use std::time::Instant;

const FINAL_RESULT_SNAP_FOLDER_NAME: &str = "snapshots";
const MOTION_DETECTION_SNAP_FOLDER_NAME: &str = "motion_detection";

fn main() -> Result<()> {
    // Load a video file
    let mut cam = video::init_video("asset/sherbrooke_video.avi")?;

    fs::remove_dir_all(FINAL_RESULT_SNAP_FOLDER_NAME)?;
    fs::remove_dir_all(MOTION_DETECTION_SNAP_FOLDER_NAME)?;

    fs::create_dir_all(FINAL_RESULT_SNAP_FOLDER_NAME)?;
    fs::create_dir_all(MOTION_DETECTION_SNAP_FOLDER_NAME)?;

    // Create a resizable window
    highgui::named_window("Final Output", highgui::WINDOW_NORMAL)?;
    highgui::resize_window("Final Output", 800, 600)?;

    highgui::named_window("Actual Frmae", highgui::WINDOW_NORMAL)?;
    highgui::resize_window("Actual Frmae", 800, 600)?;

    highgui::named_window("Motion Detected Result", highgui::WINDOW_NORMAL)?;
    highgui::resize_window("Motion Detected Result", 800, 600)?;

    // Set up telemetry logger
    let mut logger = telemetry::TelemetryLogger::new("telemetry/telemetry.csv")?;

    let start_time = Instant::now();
    let mut frame_number = 0;

    loop {
        // Capture frame
        let frame = video::capture_frame(&mut cam)?;

        let mut colored_frame = frame.clone();

        // Apply grayscale + canny edge detection
        let processed = pipeline::apply_pipeline(&frame)?;

        // Track and draw bounding boxes
        let (mut boxed_frame, areas) = tracker::track_objects(&processed)?;

        // Highlight motion + snapshot
        tracker::highlight_motion(&frame, &mut colored_frame)?;

        // Calculate and overlay elapsed time
        let timestamp_sec = start_time.elapsed().as_secs_f64();
        let time_text = format!("Time: {:.2} sec", timestamp_sec);

        imgproc::put_text(
            &mut boxed_frame,
            &time_text,
            core::Point::new(10, 30),
            imgproc::FONT_HERSHEY_SIMPLEX,
            1.0,
            core::Scalar::new(0.0, 255.0, 255.0, 0.0),
            2,
            imgproc::LINE_AA,
            false,
        )?;

        logger.log(frame_number, timestamp_sec, &areas)?;

        let save_snapshot = {
            // Every 2 seconds
            (timestamp_sec % 2.0) < 0.1 ||

            // If any "large object" detected
            areas.iter().any(|(_, cat)| cat == "large object")
        };

        if save_snapshot {
            let filename = format!(
                "snapshots/frame_{:05}_t{:.1}.png",
                frame_number, timestamp_sec
            );
            opencv::imgcodecs::imwrite(&filename, &boxed_frame, &core::Vector::new())?;
        }

        highgui::imshow("Actual Frame", &frame);

        // Show final result
        highgui::imshow("Final Output", &boxed_frame)?;

        highgui::imshow("Motion Detected Result", &colored_frame)?;

        if highgui::wait_key(10)? == 27 {
            break;
        }

        frame_number += 1;
    }

    Ok(())
}
