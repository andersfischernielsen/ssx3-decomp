//! 8x8 inverse DCT: separable, double precision (IEEE 1180 accurate), with
//! all-zero rows skipped. Output is rounded and saturated to [-256, 255] —
//! the range of the IPU's RAW16 output.

use std::sync::OnceLock;

/// `C[x][u] = c(u)/2 * cos((2x+1)uπ/16)`, `c(0) = 1/√2`.
fn cosines() -> &'static [[f64; 8]; 8] {
    static C: OnceLock<[[f64; 8]; 8]> = OnceLock::new();
    C.get_or_init(|| {
        let mut c = [[0.0; 8]; 8];
        for (x, row) in c.iter_mut().enumerate() {
            for (u, v) in row.iter_mut().enumerate() {
                let k = if u == 0 { 0.5 / 2f64.sqrt() } else { 0.5 };
                *v = k * (((2 * x + 1) * u) as f64 * std::f64::consts::PI / 16.0).cos();
            }
        }
        c
    })
}

/// `inp` is in raster order (`inp[v * 8 + u]`, v = vertical frequency).
pub fn idct(inp: &[i32; 64], out: &mut [i16; 64]) {
    let c = cosines();
    // Fast path: DC only.
    if inp[1..].iter().all(|&v| v == 0) {
        let v = (inp[0] as f64 / 8.0 + 0.5).floor() as i32;
        out.fill(v.clamp(-256, 255) as i16);
        return;
    }
    // Rows: horizontal 1-D IDCT of each frequency row v.
    let mut rows = [[0f64; 8]; 8];
    for (v, row) in rows.iter_mut().enumerate() {
        let src = &inp[v * 8..v * 8 + 8];
        if src.iter().all(|&s| s == 0) {
            continue;
        }
        for (x, r) in row.iter_mut().enumerate() {
            *r = (0..8).map(|u| c[x][u] * src[u] as f64).sum();
        }
    }
    // Columns.
    for y in 0..8 {
        for x in 0..8 {
            let s: f64 = (0..8).map(|v| c[y][v] * rows[v][x]).sum();
            out[y * 8 + x] = ((s + 0.5).floor() as i32).clamp(-256, 255) as i16;
        }
    }
}
