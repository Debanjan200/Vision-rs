use std::fs::{remove_file, File, OpenOptions};
use std::io::Write;
use std::path::Path;

/// Logs telemetry data per frame into a CSV file.
pub struct TelemetryLogger {
    file: File,
}

impl TelemetryLogger {
    /// Creates and initializes the CSV file with a header.
    pub fn new(path: &str) -> std::io::Result<Self> {
        // Suppose alreday file is present, need to remove that file
        remove_file(path)?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        writeln!(
            file,
            "frame_number,timestamp_sec,total_count,small,medium,large,avg_area,max_area,min_area"
        )?;

        Ok(Self { file })
    }

    /// Logs a row of data for a given frame.
    pub fn log(
        &mut self,
        frame_number: usize,
        timestamp_sec: f64,
        objects: &[(f64, String)],
    ) -> std::io::Result<()> {
        let mut areas = Vec::new();
        let mut small = 0;
        let mut medium = 0;
        let mut large = 0;

        for (area, category) in objects {
            areas.push(*area);
            match category.as_str() {
                "small object" => small += 1,
                "medium object" => medium += 1,
                "large object" => large += 1,
                _ => {}
            }
        }

        let count = areas.len();
        let avg = if count > 0 {
            areas.iter().sum::<f64>() / count as f64
        } else {
            0.0
        };
        let max = areas.iter().cloned().fold(0.0, f64::max);
        let min = areas.iter().cloned().fold(f64::MAX, f64::min);

        writeln!(
            self.file,
            "{},{:.2},{},{},{},{},{:.2},{:.2},{:.2}",
            frame_number,
            timestamp_sec,
            count,
            small,
            medium,
            large,
            avg,
            max,
            if count > 0 { min } else { 0.0 }
        )?;
        
        Ok(())
    }
}
