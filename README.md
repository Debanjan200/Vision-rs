
# VisionRS

A real-time image processing pipeline in Rust using OpenCV.

## Features
- Captures frames from video
- Converts to grayscale, blurs, and detects edges (Canny)
- Detects contours for basic object tracking
- Compare frame differences and highlight moving objects
- Store final result of every frame
- Display Actaul Frame, Final output and motion-detected result
- Gui feature to play or pause video
- Command-Line Args source video path can be provided
- Contour size, area are stored in a csv for log every frame

## Build Instructions

### Requirements
- Rust (stable)
- OpenCV (installed on your system)
    - Ubuntu: `sudo apt install libopencv-dev`
    - Windows: Use pre-built OpenCV and set environment paths
    - macOS: `brew install opencv`

### Run

```bash
cargo run
```

Press ESC to exit the video window.
