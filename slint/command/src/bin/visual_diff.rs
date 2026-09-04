//! Small deterministic image comparator used by the Command visual workflow.

use std::{env, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let reference_path = args
        .next()
        .ok_or("usage: visual_diff <reference> <candidate> <diff>")?;
    let candidate_path = args.next().ok_or("missing candidate image")?;
    let diff_path = args.next().ok_or("missing diff output path")?;

    let reference = image::open(&reference_path)?.to_rgba8();
    let candidate = image::open(&candidate_path)?.to_rgba8();
    if reference.dimensions() != candidate.dimensions() {
        return Err(format!(
            "dimension mismatch: reference={:?}, candidate={:?}",
            reference.dimensions(),
            candidate.dimensions()
        )
        .into());
    }

    let (width, height) = reference.dimensions();
    let mut output = image::RgbaImage::new(width, height);
    let mut absolute_sum = 0_u64;
    let mut squared_sum = 0_u64;
    let mut changed = 0_u64;

    for (x, y, reference_pixel) in reference.enumerate_pixels() {
        let candidate_pixel = candidate.get_pixel(x, y);
        let mut max_delta = 0_u8;
        for channel in 0..3 {
            let delta = reference_pixel[channel].abs_diff(candidate_pixel[channel]);
            absolute_sum += u64::from(delta);
            squared_sum += u64::from(delta) * u64::from(delta);
            max_delta = max_delta.max(delta);
        }
        if max_delta > 12 {
            changed += 1;
        }
        let heat = max_delta.saturating_mul(4);
        let context = ((u16::from(candidate_pixel[0])
            + u16::from(candidate_pixel[1])
            + u16::from(candidate_pixel[2]))
            / 9) as u8;
        output.put_pixel(x, y, image::Rgba([heat, context / 2, context / 2, 255]));
    }

    if let Some(parent) = Path::new(&diff_path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    output.save(diff_path)?;

    let samples = u64::from(width) * u64::from(height) * 3;
    let pixels = u64::from(width) * u64::from(height);
    let mae = absolute_sum as f64 / samples as f64;
    let rmse = (squared_sum as f64 / samples as f64).sqrt();
    let changed_percent = changed as f64 * 100.0 / pixels as f64;
    println!("mae={mae:.3} rmse={rmse:.3} changed_gt_12={changed_percent:.2}%");
    Ok(())
}
