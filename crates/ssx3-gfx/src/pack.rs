//! Texture packs: the game's textures dumped as PNG files, and replacement
//! images (e.g. upscaled) loaded in their place.
//!
//! A texture is named by what it decodes to: an FNV-1a hash of its pixel and
//! CLUT contents, format, TEX0 (size, PSM, CLUT mode) and TEXA, as 16 hex
//! digits. The same image always gets the same name, wherever and whenever
//! the game uploads it; one with an animated CLUT gets one name per CLUT.
//!
//! - `SSX3_TEX_DUMP=1` (or a directory): every texture decoded, once, as
//!   `textures/dump/<name>.png`.
//! - `textures/replace/<name>.png` (`SSX3_TEX_REPLACE=<dir>`, `0` = off):
//!   used instead of the texture, any size (keep the original's aspect ratio;
//!   texture coordinates are normalised), with a full mipmap chain.
//!
//! Alpha: the GS's is 0..0x80 (0x80 = opaque). The PNGs hold it ×2 (0xFF =
//! opaque), and replacements are scaled back.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::tex::{Decoded, TexKey};

/// The file name (without `.png`) of a texture.
pub fn name(k: &TexKey) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for v in [k.hash, k.format as u64, k.tex0, k.texa] {
        for b in v.to_le_bytes() {
            h = (h ^ b as u64).wrapping_mul(0x100_0000_01b3);
        }
    }
    format!("{h:016x}")
}

pub struct Pack {
    dump: Option<PathBuf>,
    dumped: HashSet<String>,
    /// Replacement images by name.
    replace: HashMap<String, PathBuf>,
}

impl Pack {
    pub fn from_env() -> Pack {
        let dump = std::env::var("SSX3_TEX_DUMP")
            .ok()
            .filter(|v| !v.is_empty() && v != "0")
            .map(|v| {
                if v == "1" {
                    ssx3_core::platform_paths::texture_dump()
                } else {
                    PathBuf::from(v)
                }
            });
        if let Some(d) = &dump {
            match std::fs::create_dir_all(d) {
                Ok(()) => eprintln!("[gfx] dumping textures to {}", d.display()),
                Err(e) => eprintln!("[gfx] cannot create {}: {e}", d.display()),
            }
        }
        let dir = ssx3_core::platform_paths::texture_replacements();
        let mut replace = HashMap::new();
        if dir.as_os_str() != "0" {
            if let Ok(rd) = std::fs::read_dir(&dir) {
                for e in rd.flatten() {
                    let p = e.path();
                    if p.extension().is_some_and(|x| x.eq_ignore_ascii_case("png")) {
                        if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                            replace.insert(stem.to_ascii_lowercase(), p.clone());
                        }
                    }
                }
            }
            if !replace.is_empty() {
                eprintln!(
                    "[gfx] {} replacement textures in {}",
                    replace.len(),
                    dir.display()
                );
            }
        }
        Pack {
            dump,
            dumped: HashSet::new(),
            replace,
        }
    }

    /// Write `d` as `<name>.png` in the dump directory (once).
    pub fn dump(&mut self, k: &TexKey, d: &Decoded) {
        let Some(dir) = &self.dump else { return };
        let n = name(k);
        if !self.dumped.insert(n.clone()) {
            return;
        }
        let path = dir.join(format!("{n}.png"));
        if path.exists() {
            return;
        }
        let rgba: Vec<u8> = d
            .rgba
            .iter()
            .flat_map(|p| {
                let [r, g, b, a] = p.to_le_bytes();
                [r, g, b, (a as u32 * 2).min(255) as u8]
            })
            .collect();
        if let Err(e) = write_png(&path, d.width, d.height, &rgba) {
            eprintln!("[gfx] {}: {e}", path.display());
        }
    }

    /// The replacement for `k`, if there is one: its mipmap chain (GS alpha).
    pub fn load(&self, k: &TexKey) -> Option<Vec<Decoded>> {
        let path = self.replace.get(&name(k))?;
        match read_png(path) {
            Ok((w, h, rgba)) => {
                let px: Vec<u32> = rgba
                    .chunks(4)
                    .map(|c| u32::from_le_bytes([c[0], c[1], c[2], ((c[3] as u32 + 1) / 2) as u8]))
                    .collect();
                Some(mips(Decoded {
                    width: w,
                    height: h,
                    rgba: px,
                }))
            }
            Err(e) => {
                eprintln!("[gfx] {}: {e}", path.display());
                None
            }
        }
    }
}

fn write_png(path: &std::path::Path, w: u32, h: u32, rgba: &[u8]) -> Result<(), String> {
    let f = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(f), w, h);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut wr = enc.write_header().map_err(|e| e.to_string())?;
    wr.write_image_data(rgba).map_err(|e| e.to_string())
}

/// A PNG as RGBA8 (grey, grey+alpha and RGB are expanded; 16-bit is cut to
/// 8; palettes are expanded by the decoder).
fn read_png(path: &std::path::Path) -> Result<(u32, u32, Vec<u8>), String> {
    let f = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut dec = png::Decoder::new(std::io::BufReader::new(f));
    dec.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut rd = dec.read_info().map_err(|e| e.to_string())?;
    let mut buf = vec![0; rd.output_buffer_size().ok_or("image too large")?];
    let info = rd.next_frame(&mut buf).map_err(|e| e.to_string())?;
    let (w, h) = (info.width, info.height);
    let px = &buf[..info.buffer_size()];
    let rgba = match info.color_type {
        png::ColorType::Rgba => px.to_vec(),
        png::ColorType::Rgb => px.chunks(3).flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
        png::ColorType::GrayscaleAlpha => px
            .chunks(2)
            .flat_map(|c| [c[0], c[0], c[0], c[1]])
            .collect(),
        png::ColorType::Grayscale => px.iter().flat_map(|&g| [g, g, g, 255]).collect(),
        png::ColorType::Indexed => return Err("indexed PNG not expanded".into()),
    };
    Ok((w, h, rgba))
}

/// `d` and its mip levels down to 1×1 (2×2 box filter; odd sizes repeat the
/// edge).
pub fn mips(d: Decoded) -> Vec<Decoded> {
    let mut out = vec![d];
    loop {
        let p = out.last().unwrap();
        if p.width == 1 && p.height == 1 {
            break;
        }
        let (w, h) = ((p.width / 2).max(1), (p.height / 2).max(1));
        let at =
            |x: u32, y: u32| p.rgba[(y.min(p.height - 1) * p.width + x.min(p.width - 1)) as usize];
        let mut rgba = Vec::with_capacity((w * h) as usize);
        for y in 0..h {
            for x in 0..w {
                let q = [
                    at(2 * x, 2 * y),
                    at(2 * x + 1, 2 * y),
                    at(2 * x, 2 * y + 1),
                    at(2 * x + 1, 2 * y + 1),
                ];
                let mut v = 0u32;
                for c in 0..4 {
                    let s: u32 = q.iter().map(|t| (t >> (8 * c)) & 0xff).sum();
                    v |= ((s + 2) / 4) << (8 * c);
                }
                rgba.push(v);
            }
        }
        out.push(Decoded {
            width: w,
            height: h,
            rgba,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mip_chain_halves_to_one_pixel() {
        let d = Decoded {
            width: 8,
            height: 2,
            rgba: vec![0x8040_2010; 16],
        };
        let m = mips(d);
        let sizes: Vec<(u32, u32)> = m.iter().map(|d| (d.width, d.height)).collect();
        assert_eq!(sizes, [(8, 2), (4, 1), (2, 1), (1, 1)]);
        assert!(m.iter().all(|d| d.rgba.iter().all(|&p| p == 0x8040_2010)));
    }

    #[test]
    fn png_round_trip_keeps_gs_alpha() {
        let dir = std::env::temp_dir().join(format!("ssx3-pack-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let k = TexKey {
            info: 1,
            pixels: 2,
            clut: 3,
            format: 4,
            tex0: 5,
            texa: 6,
            hash: 7,
        };
        let d = Decoded {
            width: 2,
            height: 1,
            rgba: vec![0x8011_2233, 0x0044_5566],
        };
        let mut p = Pack {
            dump: Some(dir.clone()),
            dumped: HashSet::new(),
            replace: HashMap::new(),
        };
        p.dump(&k, &d);
        let file = dir.join(format!("{}.png", name(&k)));
        assert!(file.exists());
        p.replace.insert(name(&k), file);
        let back = p.load(&k).unwrap();
        assert_eq!(back[0].rgba, d.rgba);
        assert_eq!(back.len(), 2);
        let _ = std::fs::remove_dir_all(dir);
    }
}
