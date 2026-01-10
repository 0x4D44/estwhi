use image::Rgba;
use image::RgbaImage;

pub fn detect_green_background(image: &RgbaImage) -> Option<[u8; 3]> {
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

pub fn whiten_background(image: &mut RgbaImage, bg: [u8; 3]) {
    for px in image.pixels_mut() {
        if color_close([px[0], px[1], px[2]], bg) {
            *px = Rgba([255, 255, 255, 255]);
        }
    }
}

pub fn is_greenish(rgb: [u8; 3]) -> bool {
    let [r, g, b] = rgb;
    g > 120 && r < 100 && b < 100
}

pub fn color_close(a: [u8; 3], b: [u8; 3]) -> bool {
    let dr = a[0] as i16 - b[0] as i16;
    let dg = a[1] as i16 - b[1] as i16;
    let db = a[2] as i16 - b[2] as i16;
    dr.abs() <= 25 && dg.abs() <= 25 && db.abs() <= 25
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_greenish() {
        assert!(is_greenish([0, 255, 0])); // Pure green
        assert!(is_greenish([50, 150, 50])); // Greenish
        assert!(!is_greenish([255, 0, 0])); // Red
        assert!(!is_greenish([0, 0, 255])); // Blue
        assert!(!is_greenish([150, 150, 150])); // Gray
    }

    #[test]
    fn test_color_close() {
        assert!(color_close([100, 100, 100], [100, 100, 100])); // Exact
        assert!(color_close([100, 100, 100], [110, 110, 110])); // Close
        assert!(!color_close([100, 100, 100], [150, 100, 100])); // Not close
    }

    #[test]
    fn test_detect_background() {
        let mut img = RgbaImage::new(10, 10);
        // Pure green corners
        img.put_pixel(0, 0, Rgba([0, 255, 0, 255]));
        assert_eq!(detect_green_background(&img), Some([0, 255, 0]));

        // No green
        let mut img2 = RgbaImage::new(10, 10);
        img2.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        assert_eq!(detect_green_background(&img2), None);
    }

    #[test]
    fn test_whiten() {
        let mut img = RgbaImage::new(2, 2);
        let bg = [0, 255, 0];
        // One pixel close to bg, one not
        img.put_pixel(0, 0, Rgba([0, 255, 0, 255])); // Exact match
        img.put_pixel(0, 1, Rgba([10, 245, 10, 255])); // Close match
        img.put_pixel(1, 0, Rgba([255, 0, 0, 255])); // Red (far)

        whiten_background(&mut img, bg);

        assert_eq!(img.get_pixel(0, 0), &Rgba([255, 255, 255, 255]));
        assert_eq!(img.get_pixel(0, 1), &Rgba([255, 255, 255, 255]));
        assert_eq!(img.get_pixel(1, 0), &Rgba([255, 0, 0, 255]));
    }
}
