//! The window icon, made from the game's own SSX 3 logo: the `ssxt` texture
//! of `DATA/UI/FE_1.SSH` (the title screen's logo; the riders' silhouettes
//! below it on the same sheet are cut off), on a dark blue square.
//! `tools/bundle_icon.py` makes the same icon for the app bundle (macOS) and
//! the executable (Windows); Linux executables carry none, so this is the
//! icon there.

/// The icon as RGBA at `size` x `size`, from the disc (`None` without one).
pub fn from_disc(size: u32) -> Option<Vec<u8>> {
    let iso = std::env::var("SSX3_ISO").map(std::path::PathBuf::from).unwrap_or_else(|_| crate::game::repo_root().join("SSX 3 (USA).iso"));
    let mut disc = ssx3_core::iso::Disc::open(&iso).ok()?;
    let e = disc.find("DATA/UI/FE_1.SSH")?;
    let mut sheet = disc.read_vec(e.sector, e.size.div_ceil(2048));
    sheet.truncate(e.size as usize);
    let (w, h, rgba) = ssh_entry(&sheet, b"ssxt")?;
    let (lw, lh, logo) = crop_logo(w, h, &rgba)?;
    Some(compose(size, lw, lh, &logo))
}

/// (width, height, RGBA) of an 8-bit paletted SSH entry.
fn ssh_entry(d: &[u8], name: &[u8; 4]) -> Option<(usize, usize, Vec<u8>)> {
    let u32_at = |o: usize| d.get(o..o + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()));
    if d.get(..4)? != b"SHPS" {
        return None;
    }
    let count = u32_at(8)? as usize;
    let off = (0..count).find(|i| d.get(16 + 8 * i..20 + 8 * i) == Some(name)).and_then(|i| u32_at(20 + 8 * i))? as usize;
    let next = (u32_at(off)? >> 8) as usize;
    let (w, h) = (u16::from_le_bytes([d[off + 4], d[off + 5]]) as usize, u16::from_le_bytes([d[off + 6], d[off + 7]]) as usize);
    if d[off] != 0x02 || next == 0 || d.get(off + next)? != &0x21 {
        return None;
    }
    let px = d.get(off + 16..off + 16 + w * h)?;
    let pal = d.get(off + next + 16..off + next + 16 + 1024)?;
    // PS2 palettes swap entries 8-15 and 16-23 of each 32 (CSM1); alpha is
    // 0..0x80.
    let lut: Vec<[u8; 4]> = (0..256usize)
        .map(|k| {
            let k = (k & 0xe7) | ((k & 8) << 1) | ((k & 16) >> 1);
            let c = &pal[4 * k..4 * k + 4];
            [c[0], c[1], c[2], (c[3] as u32 * 2).min(255) as u8]
        })
        .collect();
    Some((w, h, px.iter().flat_map(|&i| lut[i as usize]).collect()))
}

/// The logo: the first block of rows with pixels (the silhouettes follow an
/// empty gap), cropped to its pixels.
fn crop_logo(w: usize, h: usize, rgba: &[u8]) -> Option<(usize, usize, Vec<u8>)> {
    let lit = |x: usize, y: usize| rgba[4 * (y * w + x) + 3] > 16;
    let rows: Vec<bool> = (0..h).map(|y| (0..w).any(|x| lit(x, y))).collect();
    let top = rows.iter().position(|&r| r)?;
    let mut bottom = top;
    while bottom < h && rows[bottom..(bottom + 24).min(h)].iter().any(|&r| r) {
        bottom += 1;
    }
    let cols: Vec<bool> = (0..w).map(|x| (top..bottom).any(|y| lit(x, y))).collect();
    let left = cols.iter().position(|&c| c)?;
    let right = w - cols.iter().rev().position(|&c| c)?;
    let mut out = Vec::with_capacity((right - left) * (bottom - top) * 4);
    for y in top..bottom {
        out.extend_from_slice(&rgba[4 * (y * w + left)..4 * (y * w + right)]);
    }
    Some((right - left, bottom - top, out))
}

/// A square with a vertical blue gradient and the logo centred
/// (area-averaged, alpha-weighted), as in `tools/bundle_icon.py`.
fn compose(size: u32, lw: usize, lh: usize, logo: &[u8]) -> Vec<u8> {
    let s = size as f32;
    let scale = (s * 0.84 / lw as f32).min(s * 0.84 / lh as f32);
    let (tw, th) = ((lw as f32 * scale).round().max(1.0) as usize, (lh as f32 * scale).round().max(1.0) as usize);
    let (ox, oy) = (((s - tw as f32) / 2.0).round() as usize, ((s - th as f32) / 2.0).round() as usize);
    let n = size as usize;
    let mut out = vec![0u8; n * n * 4];
    for y in 0..n {
        let t = y as f32 / (s - 1.0);
        let bg = [10.0 + 8.0 * t, 26.0 + 20.0 * t, 58.0 + 40.0 * t];
        for x in 0..n {
            let mut c = bg;
            if (ox..ox + tw).contains(&x) && (oy..oy + th).contains(&y) {
                let (lx, ly) = (x - ox, y - oy);
                let (x0, x1) = (lx as f32 / scale, (lx + 1) as f32 / scale);
                let (y0, y1) = (ly as f32 / scale, (ly + 1) as f32 / scale);
                let (mut acc, mut area) = ([0.0f32; 4], 0.0f32);
                for sy in y0 as usize..(y1.ceil() as usize).min(lh) {
                    let wy = y1.min(sy as f32 + 1.0) - y0.max(sy as f32);
                    for sx in x0 as usize..(x1.ceil() as usize).min(lw) {
                        let wgt = wy * (x1.min(sx as f32 + 1.0) - x0.max(sx as f32));
                        let p = &logo[4 * (sy * lw + sx)..4 * (sy * lw + sx) + 4];
                        let aw = p[3] as f32 * wgt;
                        for k in 0..3 {
                            acc[k] += p[k] as f32 * aw;
                        }
                        acc[3] += aw;
                        area += wgt;
                    }
                }
                if acc[3] > 0.0 {
                    let k = (acc[3] / area) / 255.0;
                    for i in 0..3 {
                        c[i] = acc[i] / acc[3] * k + c[i] * (1.0 - k);
                    }
                }
            }
            let o = 4 * (y * n + x);
            out[o..o + 4].copy_from_slice(&[c[0].round() as u8, c[1].round() as u8, c[2].round() as u8, 255]);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    /// With the disc image present: an opaque square; written to
    /// `target/bundle/icon-preview/window128.png`.
    #[test]
    fn icon_from_disc() {
        let Some(rgba) = super::from_disc(128) else {
            eprintln!("no disc image: skipped");
            return;
        };
        assert_eq!(rgba.len(), 128 * 128 * 4);
        assert!(rgba.chunks(4).all(|p| p[3] == 255));
        assert_eq!(rgba[4 * (64 * 128 + 64) + 3], 255);
        let dir = crate::game::repo_root().join("target/bundle/icon-preview");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(dir.join("window128.png"), crate::png::encode(128, 128, &rgba.chunks(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect::<Vec<_>>()));
    }
}
