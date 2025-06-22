use clap::Parser;

mod pipeline;
mod telemetry;
mod tracker;
mod vision_gui;
mod video;
mod cli_argument;

const FINAL_RESULT_SNAP_FOLDER_NAME: &str = "snapshots";
const MOTION_DETECTION_SNAP_FOLDER_NAME: &str = "motion_detection";

fn path_remove(){
    // fs::remove_dir_all(FINAL_RESULT_SNAP_FOLDER_NAME)?;
    if std::fs::remove_dir_all(MOTION_DETECTION_SNAP_FOLDER_NAME).is_ok(){
        println!("Path Removal for motion image is successful");
    }
}

fn main() -> Result<(), eframe::Error> {
    let args = cli_argument::Cli::parse();
    let path = args.file;

    if !std::fs::exists(&path).expect("Can't check existence of file"){
        println!("Path doesn't exist");
        return Ok(());
    }

    path_remove();

    let options = eframe::NativeOptions {
        viewport: egui::viewport::ViewportBuilder::default()
            .with_inner_size([960.0, 720.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Vision-rs",
        options,
        Box::new(|cc| Box::new(crate::vision_gui::VisionApp::new(cc, path))),
    )
}
