use image::{ImageBuffer, ImageError, Rgb, Rgba, RgbaImage};
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

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

fn detect_green_background(image: &RgbaImage) -> Option<[u8; 3]> {
    let w = image.width();
    let h = image.height();
    let candidates = [
        (0, 0),
        (w.saturating_sub(1), 0),
        (0, h.saturating_sub(1)),
        (w.saturating_sub(1), h.saturating_sub(1)),
    ];
    for (x, y) in candidates {
        let px = image.get_pixel(x, y);
        let rgb = [px[0], px[1], px[2]];
        if is_greenish(rgb) {
            return Some(rgb);
        }
    }
    None
}

fn whiten_background(image: &mut RgbaImage, bg: [u8; 3]) {
    for px in image.pixels_mut() {
        if color_close([px[0], px[1], px[2]], bg) {
            *px = Rgba([255, 255, 255, 255]);
        }
    }
}

fn is_greenish(rgb: [u8; 3]) -> bool {
    let [r, g, b] = rgb;
    g > 120 && r < 100 && b < 100
}

fn color_close(a: [u8; 3], b: [u8; 3]) -> bool {
    let dr = a[0] as i16 - b[0] as i16;
    let dg = a[1] as i16 - b[1] as i16;
    let db = a[2] as i16 - b[2] as i16;
    dr.abs() <= 25 && dg.abs() <= 25 && db.abs() <= 25
}
