#![allow(clippy::manual_div_ceil)]
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
struct BmpInfo {
    width: i32,
    height: i32,
    _bpp: u16,
    _compression: u32,
    _size_image: u32,
    _colors_used: u32,
    header_size: usize,
    palette_size: usize,
    masks_size: usize,
    data_size: usize,
}

fn read_u16_le(buf: &[u8], off: usize) -> Option<u16> {
    if off + 2 <= buf.len() {
        Some(u16::from_le_bytes([buf[off], buf[off + 1]]))
    } else {
        None
    }
}
fn read_u32_le(buf: &[u8], off: usize) -> Option<u32> {
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
fn read_i32_le(buf: &[u8], off: usize) -> Option<i32> {
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

fn parse_dib(buf: &[u8], off: usize) -> Option<(BmpInfo, usize)> {
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
                _bpp: bpp,
                _compression: 0,
                _size_image: data_size as u32,
                _colors_used: palette_entries,
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
            _bpp: bpp,
            _compression: compression,
            _size_image: size_image,
            _colors_used: colors_used,
            header_size,
            palette_size,
            masks_size,
            data_size,
        },
        total,
    ))
}

fn write_bmp(
    out: &Path,
    header: &[u8],
    masks: &[u8],
    palette: &[u8],
    pixels: &[u8],
) -> io::Result<()> {
    let mut f = File::create(out)?;
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

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: extract-res <input.res|exe> <output-dir>");
        std::process::exit(2);
    }
    let input = &args[0];
    let outdir = Path::new(&args[1]);
    fs::create_dir_all(outdir)?;

    let mut data = Vec::new();
    File::open(input)?.read_to_end(&mut data)?;

    // Parse Microsoft .RES records (DataSize + HeaderSize + header + data), and NE/PE may also contain these blocks
    let mut off = 0usize;
    let mut count_all = 0usize;
    let mut count_cards = 0usize;
    let mut count_small = 0usize;
    let mut count_other = 0usize;

    let mut rcinc: Vec<(u16, PathBuf)> = Vec::new();

    while off + 8 <= data.len() {
        let data_size = match read_u32_le(&data, off) {
            Some(v) => v as usize,
            None => break,
        };
        let header_size = match read_u32_le(&data, off + 4) {
            Some(v) => v as usize,
            None => break,
        };
        if data_size == 0 && header_size == 0 {
            break;
        }
        if off + 8 + header_size > data.len() {
            break;
        }
        let header = &data[off + 8..off + 8 + header_size];
        // Parse type (either 0xFFFF,ID or UTF-16 string terminated by 0)
        let mut p = 0usize;
        let type_id = if let Some(0xFFFF) = read_u16_le(header, p) {
            if let Some(id) = read_u16_le(header, p + 2) {
                p += 4;
                id
            } else {
                off += 1;
                continue;
            }
        } else {
            while let Some(w) = read_u16_le(header, p) {
                p += 2;
                if w == 0 {
                    break;
                }
            }
            0
        };
        // Parse name
        let name_id = if let Some(0xFFFF) = read_u16_le(header, p) {
            if let Some(id) = read_u16_le(header, p + 2) {
                p += 4;
                Some(id)
            } else {
                off += 1;
                continue;
            }
        } else {
            while let Some(w) = read_u16_le(header, p) {
                p += 2;
                if w == 0 {
                    break;
                }
            }
            None
        };
        // Skip rest of header to DWORD boundary
        let _head_consumed = p;
        let header_rounded = ((8 + header_size + 3) / 4) * 4; // rounded absolute header end
        let data_off = off + header_rounded;
        if data_off + data_size > data.len() {
            break;
        }

        if type_id == 2
        /* RT_BITMAP */
        {
            if let Some((info, _total)) = parse_dib(&data, data_off) {
                // Build BIH + palette + pixels (the data is already that)
                let (hdr_bytes, masks, pal_bytes, px_bytes): (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) =
                    if info.header_size == 40 {
                        let hdr = data[data_off..data_off + info.header_size].to_vec();
                        let masks = data[data_off + info.header_size
                            ..data_off + info.header_size + info.masks_size]
                            .to_vec();
                        let pal = data[data_off + info.header_size + info.masks_size
                            ..data_off + info.header_size + info.masks_size + info.palette_size]
                            .to_vec();
                        let px =
                            data[data_off + info.header_size + info.masks_size + info.palette_size
                                ..data_off
                                    + info.header_size
                                    + info.masks_size
                                    + info.palette_size
                                    + info.data_size]
                                .to_vec();
                        (hdr, masks, pal, px)
                    } else {
                        let w = info.width as u32;
                        let mut hdr = vec![0u8; 40];
                        hdr[0..4].copy_from_slice(&40u32.to_le_bytes());
                        hdr[4..8].copy_from_slice(&(w as i32).to_le_bytes());
                        hdr[8..12].copy_from_slice(&info.height.to_le_bytes());
                        hdr[12..14].copy_from_slice(&1u16.to_le_bytes());
                        hdr[14..16].copy_from_slice(&(info._bpp).to_le_bytes());
                        hdr[16..20].copy_from_slice(&0u32.to_le_bytes());
                        hdr[20..24].copy_from_slice(&(info.data_size as u32).to_le_bytes());
                        let core_pal = &data[data_off + info.header_size
                            ..data_off + info.header_size + info.palette_size];
                        let mut pal = Vec::with_capacity((core_pal.len() / 3) * 4);
                        for chunk in core_pal.chunks_exact(3) {
                            pal.extend_from_slice(&[chunk[0], chunk[1], chunk[2], 0]);
                        }
                        let px = data[data_off + info.header_size + info.palette_size
                            ..data_off + info.header_size + info.palette_size + info.data_size]
                            .to_vec();
                        (hdr, Vec::new(), pal, px)
                    };

                let (name, cat_ref) = if info.width == 71 && info.height.abs() == 96 {
                    ("card", &mut count_cards)
                } else if info.width == 31 && info.height.abs() == 31 {
                    ("small", &mut count_small)
                } else {
                    ("res", &mut count_other)
                };
                *cat_ref += 1;
                count_all += 1;

                let filename = if let Some(id) = name_id {
                    format!("{}_id_{:03}.bmp", name, id)
                } else {
                    format!(
                        "{}_{}x{}_{:03}.bmp",
                        name,
                        info.width,
                        info.height.abs(),
                        *cat_ref
                    )
                };
                let file = outdir.join(filename);
                write_bmp(&file, &hdr_bytes, &masks, &pal_bytes, &px_bytes)?;

                if name == "card" {
                    if let Some(id) = name_id {
                        rcinc.push((id, file));
                    }
                }
            }
        }

        // Advance to next record; data is DWORD-aligned too
        let data_rounded = ((data_size + 3) / 4) * 4;
        off = data_off + data_rounded;
    }

    // rcinc mapping for cards by resource ID 1..52
    if !rcinc.is_empty() {
        rcinc.sort_by_key(|(id, _)| *id);
        let mut map = String::new();
        for (id, path) in rcinc.iter().filter(|(id, _)| *id >= 1 && *id <= 52) {
            // Use workspace-relative path
            let rel = path.to_string_lossy().replace('\\', "/");
            map.push_str(&format!("{} BITMAP \"{}\"\n", id, rel));
        }
        fs::write(outdir.join("cards.rcinc"), map)?;
    }

    eprintln!(
        "Extracted: total={} cards={} small={} other={}",
        count_all, count_cards, count_small, count_other
    );
    Ok(())
}
