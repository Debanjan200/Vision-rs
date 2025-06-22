use eframe::{egui, egui::TextureHandle};
use opencv::{
    core::{self, Mat},
    imgproc,
    prelude::*,
    videoio,
};
use std::{sync::Arc, thread};


pub struct VisionApp {
    cam: videoio::VideoCapture,
    frame_texture: Option<TextureHandle>,
    telemetry: crate::telemetry::TelemetryLogger,
    frame_index: usize,
    paused: bool,
}

impl VisionApp {
    pub fn new(cc: &eframe::CreationContext<'_>, path: String) -> Self {
        let mut cam = crate::video::init_video(&path)
            .expect("Cannot open video file");

        cam.set(videoio::CAP_PROP_FRAME_WIDTH, 960.0).unwrap();
        cam.set(videoio::CAP_PROP_FRAME_HEIGHT, 720.0).unwrap();

        let telemetry = crate::telemetry::TelemetryLogger::new("telemetry/telemetry.csv")
        .expect("Telemetry file not found");

        Self {
            cam,
            frame_texture: None,
            telemetry,
            frame_index: 0,
            paused: false,
        }
    }
}

impl eframe::App for VisionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .button(if self.paused {
                        "▶️ Play"
                    } else {
                        "⏸ Pause"
                    })
                    .clicked()
                {
                    self.paused = !self.paused;
                }
            });

            if !self.paused {
                if let Ok(frame) = crate::video::capture_frame(&mut self.cam) {
                    self.frame_index += 1;

                    let processed = match crate::pipeline::apply_pipeline(&frame) {
                        Ok(p) => p,
                        Err(_) => return,
                    };

                    let mut display_frame = frame.clone();

                    let processed_arc = Arc::new(processed);
                    let processed_arc_1 = Arc::clone(&processed_arc);

                    let th1 = thread::spawn(move ||{
                        crate::tracker::highlight_motion(processed_arc.as_ref(), display_frame);
                    });

                    let th2 = thread::spawn(move ||{
                        crate::tracker::track_objects(processed_arc_1.as_ref()).unwrap()
                    });

                    th1.join().unwrap();
                    let contour_count = th2.join().unwrap();

                    let _ = self.telemetry.log(self.frame_index, &contour_count.1);

                    let mut resized = Mat::default();
                    imgproc::resize(
                        &contour_count.0,
                        &mut resized,
                        core::Size::new(960, 720),
                        0.0,
                        0.0,
                        imgproc::INTER_LINEAR,
                    )
                    .unwrap();

                    let mut rgba = Mat::default();
                    imgproc::cvt_color(&resized, &mut rgba, imgproc::COLOR_BGR2RGBA, 0).unwrap();

                    let size = rgba.size().unwrap();
                    let bytes = rgba.data_bytes().unwrap();
                    let image = egui::ColorImage::from_rgba_unmultiplied(
                        [size.width as usize, size.height as usize],
                        bytes,
                    );

                    let texture = ctx.load_texture("video_frame", image, Default::default());
                    self.frame_texture = Some(texture);
                }
            }

            if let Some(texture) = &self.frame_texture {
                ui.add(
                    egui::Image::from_texture(texture).fit_to_exact_size(egui::vec2(960.0, 720.0)),
                );
            }
        });

        ctx.request_repaint(); // Keep the video playing
    }
}
