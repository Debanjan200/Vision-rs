use anyhow::Result;
use opencv::{core, imgcodecs, imgproc, prelude::*};
use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static mut PREV_FRAME: Option<core::Mat> = None;
static mut LAST_SNAPSHOT_TIME: Option<SystemTime> = None;

const COOLDOWN_DURATION: Duration = Duration::from_secs(1); // 1 second cooldown

/// Finds contours and draws bounding boxes.
pub fn track_objects(image: &core::Mat) -> Result<(core::Mat, Vec<(f64, String)>)> {
    // Convert to BGR so we can draw colored rectangles
    let mut output = core::Mat::default();
    imgproc::cvt_color(image, &mut output, imgproc::COLOR_GRAY2BGR, 0)?;

    let mut contours = core::Vector::<core::Vector<core::Point>>::new();
    imgproc::find_contours(
        image,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        core::Point::new(0, 0),
    )?;
    

    let mut areas = Vec::new();

    for contour in contours {
        // Calculate contour area
        let area = imgproc::contour_area(&contour, false)?;

        // Ignore small contours
        if area < 2000.0 {
            continue;
        }

        let category = if area < 5000.0 {
            "small object"
        } else if area < 15000.0 {
            "medium object"
        } else {
            "large object"
        }
        .to_string();

        areas.push((area, category));

        // Draw bounding box for significant contour
        let rect = imgproc::bounding_rect(&contour)?;
        imgproc::rectangle(
            &mut output,
            rect,
            core::Scalar::new(0.0, 255.0, 0.0, 0.0), // Green
            2,
            imgproc::LINE_8,
            0,
        )?;
    }
    Ok((output, areas))
}

/// Highlight the objects which are moving
pub fn highlight_motion(current: &core::Mat, colored: &mut core::Mat) -> Result<()> {
    let mut gray = core::Mat::default();
    imgproc::cvt_color(current, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut diff = core::Mat::default();
    let mut thresh = core::Mat::default();
    let mut motion_detected = false;

    unsafe {
        if let Some(prev) = &PREV_FRAME {
            core::absdiff(&gray, prev, &mut diff)?;
            imgproc::threshold(&diff, &mut thresh, 25.0, 255.0, imgproc::THRESH_BINARY)?;

            let kernel = imgproc::get_structuring_element(
                imgproc::MORPH_RECT,
                core::Size::new(3, 3),
                core::Point::new(-1, -1),
            )?;
            let mut dst = Mat::default();
            imgproc::dilate(
                &thresh,
                &mut dst,
                &kernel,
                core::Point::new(-1, -1),
                2,
                core::BORDER_CONSTANT,
                imgproc::morphology_default_border_value()?,
            )?;

            thresh = dst;

            let mut contours = opencv::core::Vector::<opencv::core::Vector<opencv::core::Point>>::new();

            imgproc::find_contours(
                &thresh,
                &mut contours,
                imgproc::RETR_EXTERNAL,
                imgproc::CHAIN_APPROX_SIMPLE,
                core::Point::new(0, 0),
            )?;

            for contour in contours {
                let area = imgproc::contour_area(&contour, false)?;
                if area > 500.0 {
                    motion_detected = true;
                    let rect = imgproc::bounding_rect(&contour)?;
                    imgproc::rectangle(
                        colored,
                        rect,
                        core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                        2,
                        imgproc::LINE_8,
                        0,
                    )?;
                }
            }

            if motion_detected {
                let now = SystemTime::now();
                let should_save = match LAST_SNAPSHOT_TIME {
                    Some(last_time) => now.duration_since(last_time).unwrap_or_default() >= COOLDOWN_DURATION,
                    None => true,
                };

                if should_save {
                    let timestamp = now.duration_since(UNIX_EPOCH)?.as_millis();
                    let filename = format!("motion_detection/motion_{}.jpg", timestamp);
                    imgcodecs::imwrite(&filename, colored, &opencv::core::Vector::<i32>::new())?;
                    LAST_SNAPSHOT_TIME = Some(now);
                }
            }
        }

        PREV_FRAME = Some(gray.clone());
    }

    Ok(())
}