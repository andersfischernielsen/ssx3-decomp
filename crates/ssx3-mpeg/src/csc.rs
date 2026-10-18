//! Colour-space conversion stages of the IPU: YCbCr (RAW8) -> RGB32, the
//! alpha thresholds (SETTH), RGB32 -> RGB16 (with optional dithering) and
//! RGB16 -> INDX4 vector quantisation (PACK with OFM=0).
//!
//! Arithmetic follows PCSX2's reference `yuv2rgb`/`ipu_csc`/`ipu_dither`/
//! `ipu_vq`, which model the hardware's fixed-point coefficients.

/// One decoded macroblock in RAW8 layout: 256 Y (16x16), 64 Cb, 64 Cr.
pub type Raw8 = [u8; 384];

/// One macroblock of RGB32 pixels (`0xAABBGGRR`), row-major 16x16.
pub type Rgb32 = [u32; 256];

const Y_COEFF: i32 = 0x95; // 1.164 * 128
const GCR_COEFF: i32 = -0x68; // -0.813 * 128
const GCB_COEFF: i32 = -0x32; // -0.391 * 128
const RCR_COEFF: i32 = 0xCC; // 1.596 * 128
const BCB_COEFF: i32 = 0x102; // 2.018 * 128
const Y_BIAS: i32 = 16;

/// The IPU's 4x4 ordered-dither matrix used when DTE is set.
const DITHER: [[i32; 4]; 4] = [
    [-4, 0, -3, 1],
    [2, -2, 3, -1],
    [-3, 1, -4, 0],
    [3, -1, 2, -2],
];

/// YCbCr -> RGB32, then the SETTH alpha thresholds and (IDEC SGN) sign flip.
///
/// Alpha is 0x80; pixels with R, G and B all below TH0 become fully zero, and
/// otherwise those with all three below TH1 get alpha 0x40.
pub fn ycbcr_to_rgb32(raw: &Raw8, th0: u16, th1: u16, sgn: bool) -> Rgb32 {
    let mut out = [0u32; 256];
    for y in 0..16 {
        for x in 0..16 {
            let c = (y / 2) * 8 + x / 2;
            let cb = raw[256 + c] as i32 - 128;
            let cr = raw[320 + c] as i32 - 128;
            let lum = (Y_COEFF * (raw[y * 16 + x] as i32 - Y_BIAS).max(0)) >> 6;
            let rcr = (RCR_COEFF * cr) >> 6;
            let gcr = (GCR_COEFF * cr) >> 6;
            let gcb = (GCB_COEFF * cb) >> 6;
            let bcb = (BCB_COEFF * cb) >> 6;
            let r = ((lum + rcr + 1) >> 1).clamp(0, 255) as u32;
            let g = ((lum + gcr + gcb + 1) >> 1).clamp(0, 255) as u32;
            let b = ((lum + bcb + 1) >> 1).clamp(0, 255) as u32;
            out[y * 16 + x] = r | g << 8 | b << 16 | 0x80 << 24;
        }
    }
    let (th0, th1) = (th0 as u32, th1 as u32);
    let below = |p: u32, t: u32| (p & 0xFF) < t && ((p >> 8) & 0xFF) < t && ((p >> 16) & 0xFF) < t;
    if th0 > 0 {
        for p in out.iter_mut() {
            if below(*p, th0) {
                *p = 0;
            } else if below(*p, th1) {
                *p = (*p & 0x00FF_FFFF) | 0x40 << 24;
            }
        }
    } else if th1 > 0 {
        for p in out.iter_mut() {
            if below(*p, th1) {
                *p = (*p & 0x00FF_FFFF) | 0x40 << 24;
            }
        }
    }
    if sgn {
        for p in out.iter_mut() {
            *p ^= 0x0080_8080;
        }
    }
    out
}

/// RGB32 -> RGB16 (5:5:5:1). The A bit is set where the RGB32 alpha is 0x40
/// (between the two SETTH thresholds). `dte` enables the ordered dither.
pub fn rgb32_to_rgb16(rgb: &Rgb32, dte: bool) -> [u16; 256] {
    let mut out = [0u16; 256];
    for y in 0..16 {
        for x in 0..16 {
            let p = rgb[y * 16 + x];
            let d = if dte { DITHER[y & 3][x & 3] } else { 0 };
            let ch = |s: u32| ((((p >> s) & 0xFF) as i32 + d).clamp(0, 255) >> 3) as u16;
            let a = ((p >> 24) == 0x40) as u16;
            out[y * 16 + x] = ch(0) | ch(8) << 5 | ch(16) << 10 | a << 15;
        }
    }
    out
}

/// RGB16 -> INDX4 against the SETVQ colour table (nearest by squared RGB
/// distance). Two pixels per byte, the left pixel in the low nibble.
pub fn rgb16_to_indx4(rgb16: &[u16; 256], clut: &[u16; 16]) -> [u8; 128] {
    let comp = |v: u16| {
        [
            (v & 31) as i32,
            ((v >> 5) & 31) as i32,
            ((v >> 10) & 31) as i32,
        ]
    };
    let closest = |p: u16| {
        let a = comp(p);
        let mut best = (i32::MAX, 0u8);
        for (k, c) in clut.iter().enumerate() {
            let b = comp(*c);
            let dist = (0..3).map(|i| (a[i] - b[i]) * (a[i] - b[i])).sum::<i32>();
            if dist < best.0 {
                best = (dist, k as u8);
            }
        }
        best.1
    };
    let mut out = [0u8; 128];
    for (i, o) in out.iter_mut().enumerate() {
        *o = closest(rgb16[i * 2]) | closest(rgb16[i * 2 + 1]) << 4;
    }
    out
}
