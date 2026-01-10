#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmpInfo {
    pub width: i32,
    pub height: i32,
    pub bpp: u16,
    pub compression: u32,
    pub size_image: u32,
    pub colors_used: u32,
    pub header_size: usize,
    pub palette_size: usize,
    pub masks_size: usize,
    pub data_size: usize,
}

pub fn read_u16_le(buf: &[u8], off: usize) -> Option<u16> {
    if off + 2 <= buf.len() {
        Some(u16::from_le_bytes([buf[off], buf[off + 1]]))
    } else {
        None
    }
}

pub fn read_u32_le(buf: &[u8], off: usize) -> Option<u32> {
    if off + 4 <= buf.len() {
        Some(u32::from_le_bytes([
            buf[off],
            buf[off + 1],
            buf[off + 2],
            buf[off + 3],
        ]))
    } else {
        None
    }
}

pub fn read_i32_le(buf: &[u8], off: usize) -> Option<i32> {
    if off + 4 <= buf.len() {
        Some(i32::from_le_bytes([
            buf[off],
            buf[off + 1],
            buf[off + 2],
            buf[off + 3],
        ]))
    } else {
        None
    }
}

pub fn parse_dib(buf: &[u8], off: usize) -> Option<(BmpInfo, usize)> {
    // Support BITMAPINFOHEADER (40) and BITMAPCOREHEADER (12)
    let size = read_u32_le(buf, off)?;
    if size == 12 {
        // OS/2 BITMAPCOREHEADER
        let w = read_u16_le(buf, off + 4)? as i32;
        let h = read_u16_le(buf, off + 6)? as i32;
        let planes = read_u16_le(buf, off + 8)?;
        let bpp = read_u16_le(buf, off + 10)?;
        if planes != 1 {
            return None;
        }
        if !(bpp == 1 || bpp == 4 || bpp == 8 || bpp == 24) {
            return None;
        }
        let header_size = 12usize;
        let palette_entries = if bpp <= 8 { 1u32 << bpp } else { 0 };
        let palette_size_core = (palette_entries as usize) * 3; // RGBTRIPLE
        let bits_per_row = (w as i64) * (bpp as i64);
        let bytes_per_row = ((bits_per_row + 31) / 32) * 4; // aligned
        let data_rows = (h as i64).unsigned_abs() as usize;
        let data_size = (bytes_per_row as usize) * data_rows;
        let total = header_size + palette_size_core + data_size;
        if off + total > buf.len() {
            return None;
        }
        return Some((
            BmpInfo {
                width: w,
                height: h,
                bpp,
                compression: 0,
                size_image: data_size as u32,
                colors_used: palette_entries,
                header_size,
                palette_size: palette_size_core,
                masks_size: 0,
                data_size,
            },
            total,
        ));
    }
    if size != 40 {
        return None;
    }
    let width = read_i32_le(buf, off + 4)?;
    let height = read_i32_le(buf, off + 8)?;
    let planes = read_u16_le(buf, off + 12)?;
    let bpp = read_u16_le(buf, off + 14)?;
    let compression = read_u32_le(buf, off + 16)?; // 0=BI_RGB, 3=BI_BITFIELDS
    let size_image = read_u32_le(buf, off + 20)?;
    let _xppm = read_i32_le(buf, off + 24)?;
    let _yppm = read_i32_le(buf, off + 28)?;
    let colors_used = read_u32_le(buf, off + 32)?;
    let _colors_imp = read_u32_le(buf, off + 36)?;

    if planes != 1 {
        return None;
    }
    if !(bpp == 1 || bpp == 4 || bpp == 8 || bpp == 24 || bpp == 32) {
        return None;
    }
    if width <= 0 || height == 0 {
        return None;
    }

    let header_size = 40usize;
    let mut masks_size = 0usize;
    if compression == 3 {
        masks_size = 12;
    }

    let mut palette_entries = 0u32;
    if bpp <= 8 {
        palette_entries = if colors_used != 0 {
            colors_used
        } else {
            1u32 << bpp
        };
    }
    let palette_size = (palette_entries as usize) * 4;

    // Compute stride
    let bits_per_row = (width as i64) * (bpp as i64);
    let bytes_per_row = ((bits_per_row + 31) / 32) * 4; // 4-byte aligned
    let data_rows = height.unsigned_abs() as usize;
    let calc_data_size = (bytes_per_row as usize) * data_rows;
    let data_size = if size_image != 0 {
        size_image as usize
    } else {
        calc_data_size
    };

    let total = header_size + masks_size + palette_size + data_size;
    if off + total > buf.len() {
        return None;
    }

    Some((
        BmpInfo {
            width,
            height,
            bpp,
            compression,
            size_image: size_image as u32,
            colors_used: palette_entries,
            header_size,
            palette_size,
            masks_size,
            data_size,
        },
        total,
    ))
}

pub fn write_bmp(
    out: &std::path::Path,
    header: &[u8],
    masks: &[u8],
    palette: &[u8],
    pixels: &[u8],
) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::fs::File::create(out)?;
    // BITMAPFILEHEADER (14 bytes)
    // WORD bfType = 'BM'
    // DWORD bfSize
    // WORD bfReserved1=0, bfReserved2=0
    // DWORD bfOffBits = 14 + header.len() + masks.len() + palette.len()
    let bf_off = 14u32 + header.len() as u32 + masks.len() as u32 + palette.len() as u32;
    let bf_size = bf_off + (pixels.len() as u32);
    let mut bf = [0u8; 14];
    bf[0] = 0x42;
    bf[1] = 0x4D; // 'BM'
    bf[2..6].copy_from_slice(&bf_size.to_le_bytes());
    bf[6..8].copy_from_slice(&0u16.to_le_bytes());
    bf[8..10].copy_from_slice(&0u16.to_le_bytes());
    bf[10..14].copy_from_slice(&bf_off.to_le_bytes());
    f.write_all(&bf)?;
    f.write_all(header)?;
    if !masks.is_empty() {
        f.write_all(masks)?;
    }
    if !palette.is_empty() {
        f.write_all(palette)?;
    }
    f.write_all(pixels)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_primitives() {
        let buf = [0x01, 0x02, 0x03, 0x04, 0x05];
        assert_eq!(read_u16_le(&buf, 0), Some(0x0201));
        assert_eq!(read_u16_le(&buf, 4), None); // OOB
        assert_eq!(read_u32_le(&buf, 0), Some(0x04030201));
        assert_eq!(read_u32_le(&buf, 2), None); // OOB
    }

    #[test]
    fn test_parse_dib_core_header() {
        let mut buf = [0u8; 100];
        // BITMAPCOREHEADER (12 bytes)
        let size = 12u32;
        buf[0..4].copy_from_slice(&size.to_le_bytes());
        let w = 4u16;
        buf[4..6].copy_from_slice(&w.to_le_bytes());
        let h = 4u16;
        buf[6..8].copy_from_slice(&h.to_le_bytes());
        let planes = 1u16;
        buf[8..10].copy_from_slice(&planes.to_le_bytes());
        let bpp = 24u16;
        buf[10..12].copy_from_slice(&bpp.to_le_bytes());

        // 24bpp -> no palette.
        // width 4 -> 12 bytes/row -> padded to 12.
        // data size = 12 * 4 = 48 bytes.
        // total = 12 + 48 = 60.

        // ensure buffer is big enough
        let total_size = 60;

        let (info, consumed) = parse_dib(&buf, 0).expect("parse core");
        assert_eq!(info.width, 4);
        assert_eq!(info.height, 4);
        assert_eq!(info.bpp, 24);
        assert_eq!(consumed, total_size);
    }

    #[test]
    fn test_parse_dib_info_header() {
        let mut buf = [0u8; 200];
        // BITMAPINFOHEADER (40 bytes)
        let size = 40u32;
        buf[0..4].copy_from_slice(&size.to_le_bytes());
        let w = 4i32;
        buf[4..8].copy_from_slice(&w.to_le_bytes());
        let h = 4i32;
        buf[8..12].copy_from_slice(&h.to_le_bytes());
        let planes = 1u16;
        buf[12..14].copy_from_slice(&planes.to_le_bytes());
        let bpp = 8u16;
        buf[14..16].copy_from_slice(&bpp.to_le_bytes());
        // compression 0, size_image 0

        // 8bpp -> 256 palette entries * 4 bytes = 1024 bytes!
        // Wait, palette entries is 1<<8 = 256.
        // 40 + 1024 + data.
        // My buffer is too small.
        let mut big_buf = vec![0u8; 2000];
        big_buf[0..40].copy_from_slice(&buf[0..40]);

        // width 4, 8bpp -> 4 bytes/row -> 4 byte aligned.
        // data = 4 * 4 = 16 bytes.
        // total = 40 + 1024 + 16 = 1080.

        let (info, consumed) = parse_dib(&big_buf, 0).expect("parse info");
        assert_eq!(info.width, 4);
        assert_eq!(info.height, 4);
        assert_eq!(info.bpp, 8);
        assert_eq!(info.palette_size, 1024);
        assert_eq!(consumed, 1080);
    }

    #[test]
    fn test_write_bmp() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_write.bmp");
        let header = vec![0u8; 40];
        let masks = vec![];
        let palette = vec![0u8; 1024];
        let pixels = vec![0u8; 16];

        assert!(write_bmp(&path, &header, &masks, &palette, &pixels).is_ok());
        assert!(path.exists());
        let _ = std::fs::remove_file(path);
    }
}
