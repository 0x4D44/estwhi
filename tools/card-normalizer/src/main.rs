use image::{ImageBuffer, ImageError, Rgb};
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

mod img_utils;
use img_utils::*;

fn main() {
    if let Err(err) = run() {
        eprintln!("card-normalizer error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let input = args
        .next()
        .ok_or("Usage: card-normalizer <input-dir> <output-dir>")?;
    let output = args
        .next()
        .ok_or("Usage: card-normalizer <input-dir> <output-dir>")?;
    if args.next().is_some() {
        return Err("Usage: card-normalizer <input-dir> <output-dir>".into());
    }

    let input_path = Path::new(&input);
    let output_path = Path::new(&output);
    if !input_path.is_dir() {
        return Err(format!("Input '{}' is not a directory", input_path.display()).into());
    }
    fs::create_dir_all(output_path)?;

    for entry in WalkDir::new(input_path).min_depth(1).max_depth(1) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path
            .extension()
            .map(|e| e.eq_ignore_ascii_case("bmp"))
            .unwrap_or(false)
        {
            let dest = output_path.join(path.file_name().unwrap());
            normalize_bitmap(path, &dest)?;
            println!("normalized {} -> {}", path.display(), dest.display());
        }
    }
    Ok(())
}

fn normalize_bitmap(input: &Path, output: &Path) -> Result<(), ImageError> {
    let img = image::open(input)?;
    let mut rgba = img.to_rgba8();
    if let Some(bg) = detect_green_background(&rgba) {
        whiten_background(&mut rgba, bg);
    }
    let rgb: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_fn(rgba.width(), rgba.height(), |x, y| {
            let p = rgba.get_pixel(x, y);
            Rgb([p[0], p[1], p[2]])
        });
    rgb.save(output)
}
