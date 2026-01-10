#![allow(clippy::manual_div_ceil)]
use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

mod bmp;
use bmp::{parse_dib, read_u16_le, read_u32_le, write_bmp};

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
                        hdr[14..16].copy_from_slice(&(info.bpp).to_le_bytes());
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
