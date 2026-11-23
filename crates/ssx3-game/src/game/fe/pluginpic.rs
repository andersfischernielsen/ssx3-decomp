//! Pictures of plugin peaks for the Select Peak screen
//! (`docs/plans/plugin-peaks.md`, 3.7).
//!
//! The screen shows a picture of the highlighted peak: a member of
//! `DATA/UI/MAPGFX.BIG`, a `SHPS` shape of 256x256 indexed texels and a
//! 256-colour palette. A plugin's `picture` (a PNG in its content folder) is
//! converted here, in memory, into such a member (`plug_<n>.ssh`) using the
//! game's own `map_mtn.ssh` as the envelope, and the archive is served with the
//! members added (`native_files::register_generator`). Nothing derived from
//! the game is stored.
//!
//! The PNG is untrusted: at most 4,096 pixels on a side and 16 MB, any failure
//! falls back to the mountain picture.

use crate::game::fe::pluginmenu::MenuPlan;

/// Largest picture accepted, in pixels per side and in file bytes.
pub const MAX_SIDE: u32 = 4096;
pub const MAX_BYTES: usize = 16 << 20;
/// The shape's size.
const SIDE: usize = 256;
/// The game's picture every plugin peak without its own uses.
pub const FALLBACK_MEMBER: &str = "map_mtn.ssh";

/// The archive member of plugin peak `p`.
pub fn member_name(p: usize) -> String {
    format!("plug_{p}.ssh")
}

type R<T> = Result<T, String>;

// ---------------------------------------------------------------------- PNG

/// Decode a PNG to RGBA8, refusing anything large.
pub fn decode_png(data: &[u8]) -> R<(u32, u32, Vec<u8>)> {
    if data.len() > MAX_BYTES {
        return Err(format!(
            "picture is {} bytes (at most {MAX_BYTES})",
            data.len()
        ));
    }
    let mut dec = png::Decoder::new(std::io::Cursor::new(data));
    dec.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = dec.read_info().map_err(|e| format!("not a PNG: {e}"))?;
    let (w, h) = {
        let i = reader.info();
        (i.width, i.height)
    };
    if w == 0 || h == 0 || w > MAX_SIDE || h > MAX_SIDE {
        return Err(format!("picture is {w}x{h} (at most {MAX_SIDE} per side)"));
    }
    let mut buf = vec![0u8; reader.output_buffer_size().ok_or("picture too large")?];
    let frame = reader
        .next_frame(&mut buf)
        .map_err(|e| format!("bad PNG: {e}"))?;
    let px = &buf[..frame.buffer_size()];
    let rgba: Vec<u8> = match frame.color_type {
        png::ColorType::Rgba => px.to_vec(),
        png::ColorType::Rgb => px.chunks(3).flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
        png::ColorType::GrayscaleAlpha => px
            .chunks(2)
            .flat_map(|c| [c[0], c[0], c[0], c[1]])
            .collect(),
        png::ColorType::Grayscale => px.iter().flat_map(|&g| [g, g, g, 255]).collect(),
        png::ColorType::Indexed => return Err("indexed PNG was not expanded".into()),
    };
    if rgba.len() != (w * h * 4) as usize {
        return Err("PNG pixel data has the wrong size".into());
    }
    Ok((w, h, rgba))
}

/// Resize to 256x256: the mean of the covered pixels when shrinking, bilinear
/// when growing. Colour is weighted by alpha so transparent edges do not bleed.
pub fn resize_square(w: usize, h: usize, rgba: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; SIDE * SIDE * 4];
    for y in 0..SIDE {
        for x in 0..SIDE {
            // Source rectangle of this pixel, in source pixels.
            let (x0, x1) = (x * w, (x + 1) * w);
            let (y0, y1) = (y * h, (y + 1) * h);
            let (mut r, mut g, mut b, mut a, mut n) = (0u64, 0u64, 0u64, 0u64, 0u64);
            let mut add = |sx: usize, sy: usize, weight: u64| {
                let p = &rgba[(sy * w + sx) * 4..][..4];
                let al = p[3] as u64 * weight;
                r += p[0] as u64 * al;
                g += p[1] as u64 * al;
                b += p[2] as u64 * al;
                a += al;
                n += weight;
            };
            if w >= SIDE && h >= SIDE {
                for sy in y0 / SIDE..y1.div_ceil(SIDE) {
                    for sx in x0 / SIDE..x1.div_ceil(SIDE) {
                        // Coverage of the source pixel by the destination one.
                        let cx = (x1.min((sx + 1) * SIDE)).saturating_sub(x0.max(sx * SIDE));
                        let cy = (y1.min((sy + 1) * SIDE)).saturating_sub(y0.max(sy * SIDE));
                        if cx * cy > 0 {
                            add(sx.min(w - 1), sy.min(h - 1), (cx * cy) as u64);
                        }
                    }
                }
            } else {
                // Bilinear at the pixel centre.
                let fx = ((x * 2 + 1) * w) as f64 / (SIDE * 2) as f64 - 0.5;
                let fy = ((y * 2 + 1) * h) as f64 / (SIDE * 2) as f64 - 0.5;
                let (ix, iy) = (fx.floor().max(0.0) as usize, fy.floor().max(0.0) as usize);
                let (tx, ty) = (
                    (fx - ix as f64).clamp(0.0, 1.0),
                    (fy - iy as f64).clamp(0.0, 1.0),
                );
                for (dx, dy, wgt) in [
                    (0, 0, (1.0 - tx) * (1.0 - ty)),
                    (1, 0, tx * (1.0 - ty)),
                    (0, 1, (1.0 - tx) * ty),
                    (1, 1, tx * ty),
                ] {
                    let wt = (wgt * 1024.0) as u64;
                    if wt > 0 {
                        add((ix + dx).min(w - 1), (iy + dy).min(h - 1), wt);
                    }
                }
            }
            let o = (y * SIDE + x) * 4;
            if a > 0 {
                out[o] = ((r + a / 2) / a) as u8;
                out[o + 1] = ((g + a / 2) / a) as u8;
                out[o + 2] = ((b + a / 2) / a) as u8;
            }
            out[o + 3] = if n > 0 { ((a + n / 2) / n) as u8 } else { 0 };
        }
    }
    out
}

// ------------------------------------------------------------- quantization

/// 256x256 RGBA to indexed: up to 256 colours by median cut (deterministic:
/// ties break by colour value), one palette entry reserved for transparency
/// when any pixel is mostly transparent. Returns the texel indices and the
/// palette (RGBA, alpha 0..=255).
pub fn quantize(rgba: &[u8]) -> (Vec<u8>, Vec<[u8; 4]>) {
    assert_eq!(rgba.len(), SIDE * SIDE * 4);
    let transparent = rgba.chunks(4).any(|p| p[3] < 128);
    let budget = if transparent { 255 } else { 256 };
    // Opaque colours with their populations, sorted.
    let mut hist: std::collections::BTreeMap<[u8; 3], u32> = Default::default();
    for p in rgba.chunks(4).filter(|p| p[3] >= 128) {
        *hist.entry([p[0], p[1], p[2]]).or_insert(0) += 1;
    }
    let colours: Vec<([u8; 3], u32)> = hist.into_iter().collect();
    let mut boxes: Vec<Vec<usize>> = vec![(0..colours.len()).collect()];
    boxes.retain(|b| !b.is_empty());
    while boxes.len() < budget {
        // Split the box with the widest channel range (population breaks ties).
        let pick = boxes
            .iter()
            .enumerate()
            .filter(|(_, b)| b.len() > 1)
            .map(|(i, b)| {
                let (axis, range) = widest(&colours, b);
                (range, b.len(), std::cmp::Reverse(i), axis)
            })
            .max();
        let Some((_, _, std::cmp::Reverse(i), axis)) = pick else {
            break;
        };
        let mut b = std::mem::take(&mut boxes[i]);
        b.sort_by_key(|&k| (colours[k].0[axis], colours[k].0));
        // Median by population.
        let total: u64 = b.iter().map(|&k| colours[k].1 as u64).sum();
        let (mut acc, mut at) = (0u64, 0usize);
        for (n, &k) in b.iter().enumerate() {
            acc += colours[k].1 as u64;
            at = n + 1;
            if acc * 2 >= total {
                break;
            }
        }
        let at = at.clamp(1, b.len() - 1);
        let upper = b.split_off(at);
        boxes[i] = b;
        boxes.push(upper);
    }
    let mut palette: Vec<[u8; 4]> = Vec::with_capacity(256);
    let mut index_of: std::collections::HashMap<[u8; 3], u8> = Default::default();
    for (n, b) in boxes.iter().enumerate() {
        let total: u64 = b.iter().map(|&k| colours[k].1 as u64).sum::<u64>().max(1);
        let mut avg = [0u8; 3];
        for ch in 0..3 {
            let s: u64 = b
                .iter()
                .map(|&k| colours[k].0[ch] as u64 * colours[k].1 as u64)
                .sum();
            avg[ch] = ((s + total / 2) / total) as u8;
        }
        palette.push([avg[0], avg[1], avg[2], 255]);
        for &k in b {
            index_of.insert(colours[k].0, n as u8);
        }
    }
    let clear = if transparent {
        palette.push([0, 0, 0, 0]);
        Some((palette.len() - 1) as u8)
    } else {
        None
    };
    let texels = rgba
        .chunks(4)
        .map(|p| {
            if p[3] < 128 {
                clear.unwrap_or(0)
            } else {
                index_of[&[p[0], p[1], p[2]]]
            }
        })
        .collect();
    (texels, palette)
}

fn widest(colours: &[([u8; 3], u32)], b: &[usize]) -> (usize, u32) {
    (0..3)
        .map(|ch| {
            let lo = b.iter().map(|&k| colours[k].0[ch]).min().unwrap_or(0);
            let hi = b.iter().map(|&k| colours[k].0[ch]).max().unwrap_or(0);
            (ch, (hi - lo) as u32)
        })
        .max_by_key(|&(ch, r)| (r, std::cmp::Reverse(ch)))
        .unwrap()
}

// ---------------------------------------------------------------------- SSH

/// PSMT8 palettes are stored with index bits 3 and 4 exchanged.
pub fn clut_slot(index: usize) -> usize {
    (index & !0x18) | ((index & 8) << 1) | ((index & 16) >> 1)
}

/// Offsets of the texels and the palette in a `map_mtn.ssh` shaped member:
/// `SHPS`, one image whose header (type 2, size, 256x256) is followed by
/// 65,536 texels, then the palette header and 256 RGBA entries.
fn layout(template: &[u8]) -> R<(usize, usize)> {
    let le = |p: usize| -> R<u32> {
        template
            .get(p..p + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .ok_or_else(|| "template shape is truncated".to_string())
    };
    if template.get(..4) != Some(b"SHPS") || le(8)? != 1 {
        return Err("template is not a one-image SHPS".into());
    }
    let image = le(20)? as usize;
    let head = le(image)?;
    let size = (head >> 8) as usize;
    let w = u16::from_le_bytes([template[image + 4], template[image + 5]]);
    let h = u16::from_le_bytes([template[image + 6], template[image + 7]]);
    if head & 0xff != 2 || (w, h) != (256, 256) || size < 16 + 65536 {
        return Err("template image is not 256x256 indexed".into());
    }
    let palette = image + size;
    if template.len() < palette + 16 + 1024 || template[palette] != 0x21 {
        return Err("template has no 256-colour palette".into());
    }
    Ok((image + 16, palette + 16))
}

/// A `map_mtn.ssh` shaped member showing a PNG.
pub fn ssh_from_png(png: &[u8], template: &[u8]) -> R<Vec<u8>> {
    let (tex, pal) = layout(template)?;
    let (w, h, rgba) = decode_png(png)?;
    let small = resize_square(w as usize, h as usize, &rgba);
    let (texels, palette) = quantize(&small);
    let mut out = template.to_vec();
    out[tex..tex + SIDE * SIDE].copy_from_slice(&texels);
    out[pal..pal + 1024].fill(0);
    for (i, c) in palette.iter().enumerate() {
        let slot = clut_slot(i);
        // PS2 alpha: 0x80 is opaque.
        out[pal + slot * 4..pal + slot * 4 + 4].copy_from_slice(&[
            c[0],
            c[1],
            c[2],
            if c[3] >= 128 { 0x80 } else { 0 },
        ]);
    }
    Ok(out)
}

// --------------------------------------------------------------------- BIGF

/// The members of a BIGF archive: `BIGF`, the file size (LE), the member count
/// and header size (BE), then per member offset and size (BE) and a
/// NUL-terminated name.
pub fn parse_bigf(d: &[u8]) -> R<Vec<(String, Vec<u8>)>> {
    let be = |p: usize| -> R<u32> {
        d.get(p..p + 4)
            .map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
            .ok_or_else(|| "BIGF header is truncated".to_string())
    };
    if d.get(..4) != Some(b"BIGF") {
        return Err("not a BIGF archive".into());
    }
    let count = be(8)? as usize;
    if count > 4096 {
        return Err("BIGF member count is absurd".into());
    }
    let mut pos = 16;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let (off, size) = (be(pos)? as usize, be(pos + 4)? as usize);
        pos += 8;
        let end = d[pos.min(d.len())..]
            .iter()
            .position(|b| *b == 0)
            .ok_or("BIGF member name is unterminated")?
            + pos;
        let name = String::from_utf8_lossy(&d[pos..end]).into_owned();
        pos = end + 1;
        let body = d
            .get(off..off.checked_add(size).ok_or("BIGF member overflows")?)
            .ok_or_else(|| format!("BIGF member {name} is outside the file"))?;
        out.push((name, body.to_vec()));
    }
    Ok(out)
}

/// A BIGF archive of `members`, packed back to back on 4-byte boundaries.
pub fn pack_bigf(members: &[(String, Vec<u8>)]) -> Vec<u8> {
    let header: usize = 16 + members.iter().map(|(n, _)| 8 + n.len() + 1).sum::<usize>();
    let mut off = header.next_multiple_of(4);
    let mut out = vec![0u8; off];
    let mut table = Vec::new();
    for (name, body) in members {
        table.extend_from_slice(&(off as u32).to_be_bytes());
        table.extend_from_slice(&(body.len() as u32).to_be_bytes());
        table.extend_from_slice(name.as_bytes());
        table.push(0);
        out.extend_from_slice(body);
        off = out.len().next_multiple_of(4);
        out.resize(off, 0);
    }
    out[..4].copy_from_slice(b"BIGF");
    let total = out.len() as u32;
    out[4..8].copy_from_slice(&total.to_le_bytes());
    out[8..12].copy_from_slice(&(members.len() as u32).to_be_bytes());
    out[12..16].copy_from_slice(&(header as u32).to_be_bytes());
    out[16..16 + table.len()].copy_from_slice(&table);
    out
}

/// `MAPGFX.BIG` with a picture member per plugin peak that has a readable
/// picture. A peak whose picture cannot be used is logged and keeps the
/// mountain's.
pub fn patch_mapgfx(
    original: &[u8],
    plan: &MenuPlan,
    read: &dyn Fn(&std::path::Path) -> std::io::Result<Vec<u8>>,
) -> R<Vec<u8>> {
    let mut members = parse_bigf(original)?;
    let template = members
        .iter()
        .find(|(n, _)| n == FALLBACK_MEMBER)
        .map(|(_, b)| b.clone())
        .ok_or_else(|| format!("MAPGFX.BIG has no {FALLBACK_MEMBER}"))?;
    let mut added = 0;
    for p in &plan.peaks {
        let Some(path) = &p.picture else { continue };
        let made = read(path)
            .map_err(|e| e.to_string())
            .and_then(|png| ssh_from_png(&png, &template));
        match made {
            Ok(ssh) => {
                members.push((member_name(p.index), ssh));
                added += 1;
            }
            Err(e) => eprintln!(
                "[plugin-menu] picture of {} ({}): {e}; the mountain is shown instead",
                p.id,
                path.display()
            ),
        }
    }
    if added == 0 {
        return Err("no plugin picture could be used".into());
    }
    Ok(pack_bigf(&members))
}

#[cfg(test)]
mod tests {
    use ssx3_plugin::catalog::{Catalog, Event, Peak};
    use ssx3_plugin::manifest::Kind;

    use super::*;

    fn png_of(w: u32, h: u32, color: png::ColorType, data: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut enc = png::Encoder::new(&mut out, w, h);
        enc.set_color(color);
        enc.set_depth(png::BitDepth::Eight);
        enc.write_header().unwrap().write_image_data(data).unwrap();
        out
    }

    /// A template shaped like `map_mtn.ssh`: SHPS, one 256x256 image, a palette
    /// and 32 bytes of tail.
    fn template() -> Vec<u8> {
        let mut t = vec![0u8; 66736];
        t[..4].copy_from_slice(b"SHPS");
        t[8..12].copy_from_slice(&1u32.to_le_bytes());
        t[20..24].copy_from_slice(&112u32.to_le_bytes());
        t[112] = 2;
        t[113..116].copy_from_slice(&65552u32.to_le_bytes()[..3]);
        t[116..118].copy_from_slice(&256u16.to_le_bytes());
        t[118..120].copy_from_slice(&256u16.to_le_bytes());
        let pal = 112 + 65552;
        t[pal] = 0x21;
        t[pal + 12 + 1] = 0x20;
        t
    }

    fn gradient_png(w: u32, h: u32) -> Vec<u8> {
        let data: Vec<u8> = (0..w * h)
            .flat_map(|i| {
                let (x, y) = (i % w, i / w);
                [(x * 255 / w) as u8, (y * 255 / h) as u8, 90]
            })
            .collect();
        png_of(w, h, png::ColorType::Rgb, &data)
    }

    #[test]
    fn png_decoding_is_bounded() {
        assert!(decode_png(b"not a png").is_err());
        let ok = png_of(2, 2, png::ColorType::Grayscale, &[0, 64, 128, 255]);
        let (w, h, px) = decode_png(&ok).unwrap();
        assert_eq!((w, h, px.len()), (2, 2, 16));
        assert_eq!(&px[4..8], &[64, 64, 64, 255]);
        let big = png_of(
            MAX_SIDE + 1,
            1,
            png::ColorType::Grayscale,
            &vec![0; MAX_SIDE as usize + 1],
        );
        assert!(decode_png(&big).unwrap_err().contains("at most"));
        let huge = vec![0u8; MAX_BYTES + 1];
        assert!(decode_png(&huge).unwrap_err().contains("bytes"));
        // Truncated data.
        assert!(decode_png(&ok[..ok.len() - 12]).is_err());
    }

    #[test]
    fn resizing_keeps_flat_colours_and_alpha() {
        for (w, h) in [(7usize, 3usize), (256, 256), (1000, 513)] {
            let rgba: Vec<u8> = (0..w * h).flat_map(|_| [10, 200, 30, 255]).collect();
            let out = resize_square(w, h, &rgba);
            assert!(out.chunks(4).all(|p| p == [10, 200, 30, 255]), "{w}x{h}");
        }
        // Transparent pixels do not bleed their colour into opaque ones.
        let mut rgba = vec![0u8; 2 * 1 * 4];
        rgba[..4].copy_from_slice(&[255, 0, 0, 255]);
        rgba[4..8].copy_from_slice(&[0, 255, 0, 0]);
        let out = resize_square(2, 1, &rgba);
        assert_eq!(&out[..4], &[255, 0, 0, 255]);
        assert_eq!(out[(255) * 4 + 3], 0);
    }

    #[test]
    fn quantization_is_deterministic_and_exact_for_few_colours() {
        let mut rgba = Vec::new();
        for i in 0..SIDE * SIDE {
            let c = (i % 5) as u8 * 40;
            rgba.extend_from_slice(&[c, 255 - c, c / 2, 255]);
        }
        let (t1, p1) = quantize(&rgba);
        let (t2, p2) = quantize(&rgba);
        assert_eq!((&t1, &p1), (&t2, &p2));
        assert_eq!(p1.len(), 5);
        // Each of the five colours is kept exactly.
        for (i, px) in rgba.chunks(4).enumerate() {
            assert_eq!(&p1[t1[i] as usize][..3], &px[..3]);
        }
        // Many colours: 256 entries at most, close to the source.
        let rgba: Vec<u8> = (0..SIDE * SIDE)
            .flat_map(|i| [(i % 256) as u8, (i / 256) as u8, ((i * 7) % 256) as u8, 255])
            .collect();
        let (t, p) = quantize(&rgba);
        assert!(p.len() <= 256);
        let err: u64 = rgba
            .chunks(4)
            .zip(&t)
            .map(|(px, &i)| {
                (0..3)
                    .map(|c| (px[c] as i32 - p[i as usize][c] as i32).unsigned_abs() as u64)
                    .sum::<u64>()
            })
            .sum();
        assert!(
            err / (SIDE * SIDE) as u64 <= 40,
            "mean error {}",
            err / (SIDE * SIDE) as u64
        );
        // Transparency gets its own entry.
        let mut rgba = vec![200u8; SIDE * SIDE * 4];
        rgba[3] = 0;
        let (t, p) = quantize(&rgba);
        assert_eq!(p[t[0] as usize][3], 0);
        assert_eq!(p[t[1] as usize][3], 255);
    }

    #[test]
    fn the_palette_slot_exchanges_bits_three_and_four() {
        assert_eq!(clut_slot(0), 0);
        assert_eq!(clut_slot(8), 16);
        assert_eq!(clut_slot(16), 8);
        assert_eq!(clut_slot(24), 24);
        assert_eq!(clut_slot(0xff), 0xff);
        let mut seen = [false; 256];
        (0..256).for_each(|i| seen[clut_slot(i)] = true);
        assert!(seen.iter().all(|s| *s));
    }

    #[test]
    fn a_png_becomes_a_shape_of_the_templates_size_and_shows_the_picture() {
        let t = template();
        let ssh = ssh_from_png(&gradient_png(300, 200), &t).unwrap();
        assert_eq!(ssh.len(), t.len());
        let (tex, pal) = layout(&ssh).unwrap();
        // Envelope and tail are the template's; decoding the shape gives the picture back.
        assert_eq!(&ssh[..tex - 16 + 16], &t[..tex]);
        assert_eq!(&ssh[pal + 1024..], &t[pal + 1024..]);
        let src = resize_square(300, 200, &decode_png(&gradient_png(300, 200)).unwrap().2);
        let mut err = 0u64;
        for i in 0..SIDE * SIDE {
            let slot = clut_slot(ssh[tex + i] as usize);
            let c = &ssh[pal + slot * 4..pal + slot * 4 + 4];
            assert_eq!(c[3], 0x80);
            err += (0..3)
                .map(|k| (c[k] as i32 - src[i * 4 + k] as i32).unsigned_abs() as u64)
                .sum::<u64>();
        }
        assert!(
            err / (SIDE * SIDE) as u64 <= 18,
            "mean error {}",
            err / (SIDE * SIDE) as u64
        );
        // Deterministic.
        assert_eq!(ssh, ssh_from_png(&gradient_png(300, 200), &t).unwrap());
        // A template that is not a shape is refused.
        assert!(ssh_from_png(&gradient_png(4, 4), b"nope").is_err());
    }

    #[test]
    fn bigf_archives_round_trip_with_members_added() {
        let members = vec![
            ("a.ssh".to_string(), vec![1u8; 10]),
            ("map_mtn.ssh".to_string(), template()),
        ];
        let big = pack_bigf(&members);
        assert_eq!(&big[..4], b"BIGF");
        assert_eq!(
            u32::from_le_bytes(big[4..8].try_into().unwrap()) as usize,
            big.len()
        );
        assert_eq!(parse_bigf(&big).unwrap(), members);
        let mut more = members.clone();
        more.push((member_name(0), vec![9u8; 5]));
        assert_eq!(parse_bigf(&pack_bigf(&more)).unwrap(), more);
        // Damaged archives are errors.
        assert!(parse_bigf(b"BIGF").is_err());
        assert!(parse_bigf(&big[..big.len() - 70_000.min(big.len() - 40)]).is_err());
        let mut bad = big.clone();
        bad[16..20].copy_from_slice(&u32::MAX.to_be_bytes());
        assert!(parse_bigf(&bad).is_err());
    }

    fn plan_with_picture(path: Option<&str>) -> MenuPlan {
        let cat = Catalog {
            peaks: vec![Peak {
                id: "t/p".into(),
                plugin: "t".into(),
                name: "P".into(),
                picture: path.map(Into::into),
            }],
            events: vec![Event {
                id: "t/e".into(),
                plugin: "t".into(),
                name: None,
                name_key: None,
                peak: "t/p".into(),
                kind: Kind::Race,
                template: "ARA1".into(),
                world: "w".into(),
            }],
            problems: vec![],
        };
        MenuPlan::build(&cat)
    }

    #[test]
    fn mapgfx_gets_a_member_per_usable_picture() {
        let original = pack_bigf(&[
            ("map_mtn.ssh".to_string(), template()),
            ("other.ssh".to_string(), vec![5; 40]),
        ]);
        let png = gradient_png(64, 64);
        let plan = plan_with_picture(Some("ui/peak.png"));
        let out = patch_mapgfx(&original, &plan, &|_| Ok(png.clone())).unwrap();
        let members = parse_bigf(&out).unwrap();
        assert_eq!(members.len(), 3);
        assert_eq!(members[2].0, "plug_0.ssh");
        assert_eq!(members[2].1.len(), template().len());
        assert_eq!(members[0], parse_bigf(&original).unwrap()[0]);
        // A picture that cannot be read or decoded: no member, an error for the caller to leave the archive alone.
        let missing = patch_mapgfx(&original, &plan, &|_| {
            Err(std::io::ErrorKind::NotFound.into())
        });
        assert!(missing.is_err());
        assert!(patch_mapgfx(&original, &plan, &|_| Ok(b"junk".to_vec())).is_err());
        // No picture: nothing to add.
        assert!(patch_mapgfx(&original, &plan_with_picture(None), &|_| Ok(png.clone())).is_err());
        // No template in the archive.
        let bare = pack_bigf(&[("x".to_string(), vec![0; 4])]);
        assert!(patch_mapgfx(&bare, &plan, &|_| Ok(png.clone())).is_err());
    }

    #[test]
    fn the_real_mapgfx_takes_a_picture_and_keeps_its_members() {
        let iso = ssx3_core::platform_paths::data_file(
            "SSX3_ISO",
            "SSX 3 (USA).iso",
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../SSX 3 (USA).iso"),
        );
        let Some(original) = ssx3_core::iso::Disc::open(&iso).ok().and_then(|mut d| {
            let e = d.find("DATA/UI/MAPGFX.BIG")?;
            let data = d.read_vec(e.sector, e.size.div_ceil(2048));
            Some(data[..e.size as usize].to_vec())
        }) else {
            eprintln!("skipped: no SSX 3 disc image");
            return;
        };
        let before = parse_bigf(&original).unwrap();
        assert_eq!(before.len(), 26);
        let plan = plan_with_picture(Some("x.png"));
        let png = gradient_png(512, 300);
        let out = patch_mapgfx(&original, &plan, &|_| Ok(png.clone())).unwrap();
        let after = parse_bigf(&out).unwrap();
        assert_eq!(after.len(), 27);
        assert_eq!(&after[..26], &before[..]);
        let mtn = &before.iter().find(|(n, _)| n == FALLBACK_MEMBER).unwrap().1;
        assert_eq!(after[26].1.len(), mtn.len());
        // The envelope (everything but texels and palette) is the retail one.
        let (tex, pal) = layout(mtn).unwrap();
        assert_eq!(&after[26].1[..tex], &mtn[..tex]);
        assert_eq!(&after[26].1[tex + 65536..pal], &mtn[tex + 65536..pal]);
    }
}
