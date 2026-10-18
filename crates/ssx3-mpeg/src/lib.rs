//! MPEG-2 video for the game's movies: the pieces the IPU model and the
//! native decoder share (VLC tables, lookups, dequantisation, IDCT,
//! YCbCr -> RGB32) and the native decoder itself ([`decoder`], N5 in
//! `docs/plans/native-rendering.md`).

pub mod csc;
pub mod decoder;
pub mod idct;
pub mod tables;

use std::sync::OnceLock;

use tables::{Dct, Vlc};

/// MPEG default intra quantiser matrix, raster order.
pub const DEFAULT_INTRA: [u8; 64] = [
    8, 16, 19, 22, 26, 27, 29, 34, 16, 16, 22, 24, 27, 29, 34, 37, 19, 22, 26, 27, 29, 34, 34, 38,
    22, 22, 26, 27, 29, 34, 37, 40, 22, 26, 27, 29, 32, 35, 40, 48, 26, 27, 29, 32, 35, 40, 48, 58,
    26, 27, 29, 34, 38, 46, 56, 69, 27, 29, 35, 38, 46, 56, 69, 83,
];

/// MPEG-2 non-linear quantiser_scale (q_scale_type = 1).
pub const NON_LINEAR_SCALE: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16, 18, 20, 22, 24, 28, 32, 36, 40, 44, 48, 52, 56, 64,
    72, 80, 88, 96, 104, 112,
];

// ------------------------------------------------------------------ lookups

/// Flat lookup over `bits` bits: entries are `(length, value)`, length 0 =
/// invalid code.
pub struct Lookup {
    pub bits: u32,
    pub entries: Vec<(u8, u32)>,
}

pub struct DctLookup {
    pub bits: u32,
    pub entries: Vec<(u8, u8, i16)>,
}

pub fn build(table: &[Vlc]) -> Lookup {
    let bits = table.iter().map(|e| e.length as u32).max().unwrap();
    let mut entries = vec![(0u8, 0u32); 1 << bits];
    for e in table {
        let shift = bits - e.length as u32;
        for i in 0..1u32 << shift {
            entries[((e.code as u32) << shift | i) as usize] = (e.length, e.value);
        }
    }
    Lookup { bits, entries }
}

pub fn build_dct(table: &[Dct]) -> DctLookup {
    let bits = table.iter().map(|e| e.length as u32).max().unwrap();
    let mut entries = vec![(0u8, 0u8, 0i16); 1 << bits];
    for e in table {
        let shift = bits - e.length as u32;
        for i in 0..1u32 << shift {
            entries[((e.code as u32) << shift | i) as usize] = (e.length, e.run, e.level);
        }
    }
    DctLookup { bits, entries }
}

pub struct Luts {
    pub mbai: Lookup,
    pub mb_i: Lookup,
    pub mb_p: Lookup,
    pub mb_b: Lookup,
    pub motion: Lookup,
    pub dmv: Lookup,
    pub dc_luma: Lookup,
    pub dc_chroma: Lookup,
    pub cbp: Lookup,
    pub dct0: DctLookup,
    pub dct1: DctLookup,
}

pub fn luts() -> &'static Luts {
    static L: OnceLock<Luts> = OnceLock::new();
    L.get_or_init(|| Luts {
        mbai: build(&tables::MB_ADDRESS_INCREMENT),
        mb_i: build(&tables::MB_TYPE_I),
        mb_p: build(&tables::MB_TYPE_P),
        mb_b: build(&tables::MB_TYPE_B),
        motion: build(&tables::MOTION_CODE),
        dmv: build(&tables::DM_VECTOR),
        dc_luma: build(&tables::DC_SIZE_LUMA),
        dc_chroma: build(&tables::DC_SIZE_CHROMA),
        cbp: build(&tables::CODED_BLOCK_PATTERN),
        dct0: build_dct(&tables::DCT_TABLE_0),
        dct1: build_dct(&tables::DCT_TABLE_1),
    })
}

/// Dequantise one block's coefficients (`coef` in scan order) into raster
/// order: intra DC times `dc_mult`, the rest by weight and scale, saturated
/// to [-2048, 2047], then MPEG-2 mismatch control (MPEG-1: oddification).
pub fn dequantise(coef: &[i32; 64], scan: &[u8; 64], weights: &[u8; 64], intra: bool, scale: i32, dc_mult: i32, mpeg1: bool) -> [i32; 64] {
    let mut f = [0i32; 64];
    let mut sum = 0i32;
    for (k, &v) in coef.iter().enumerate() {
        let r = scan[k] as usize;
        let mut x = if intra && k == 0 {
            v * dc_mult
        } else {
            if v == 0 {
                continue;
            }
            let sign = v.signum();
            let w = weights[r] as i32;
            let mut x = if intra {
                (v * w * scale * 2) / 32
            } else {
                ((2 * v + sign) * w * scale) / 32
            };
            if mpeg1 && x != 0 && x & 1 == 0 {
                x -= sign; // MPEG-1 oddification
            }
            x
        };
        x = x.clamp(-2048, 2047);
        f[r] = x;
        sum += x;
    }
    if !mpeg1 && sum & 1 == 0 {
        f[63] ^= 1; // MPEG-2 mismatch control
    }
    f
}

