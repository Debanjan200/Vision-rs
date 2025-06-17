use opencv::{core, imgcodecs, prelude::*, Result};

use crate::pipeline;

fn compare_edges(actual: &core::Mat, expected: &core::Mat) -> Result<bool> {
    let mut diff = core::Mat::default();
    core::absdiff(actual, expected, &mut diff)?;

    let mut non_zero = core::Mat::default();
    core::compare(&diff, &core::Scalar::all(0.0), &mut non_zero, core::CMP_NE)?;


    let count = core::count_non_zero(&non_zero)?;
    println!("Pixel difference: {}", count);

    // Define a tolerance (you can tune this)
    Ok(count < 500)
}

#[test]
fn test_edge_pipeline_against_expected() -> Result<()> {
    let input = imgcodecs::imread("test_image/actual/RGB_001.png", imgcodecs::IMREAD_COLOR)?;
    let expected = imgcodecs::imread("test_image/expected/RGB_001.png", imgcodecs::IMREAD_GRAYSCALE)?;

    

    let actual = pipeline::apply_pipeline(&input).map_err(|e| opencv::Error::new(0, e.to_string()))?;

    assert_eq!(actual.size()?, expected.size()?);

    let matched = compare_edges(&actual, &expected)?;
    assert!(matched, "Edge output does not match expected!");

    Ok(())
}
