//! EE MMI (128-bit integer SIMD) operations on register images, for
//! replacements of code that uses them (newlib's string scans, fills).
//!
//! A [`Gpr`] is a 128-bit GPR as `Cpu::g` holds it: lane 0 (the low 64
//! bits) first. Read one with `c.g[r]`, write one with `c.g[r] = v`
//! (`abi::gpr128`/`abi::set_gpr128` for a `u128`).

use ssx3_ee_rt::Cpu;

/// A 128-bit GPR image (lane 0 first), as MMI instructions see it.
pub type Gpr = [u64; 2];

fn gpr_bytes(a: Gpr) -> [u8; 16] {
    (a[0] as u128 | (a[1] as u128) << 64).to_le_bytes()
}

/// `psubb`: per-byte wrapping subtraction.
pub fn psubb(a: Gpr, b: Gpr) -> Gpr {
    let (x, y) = (gpr_bytes(a), gpr_bytes(b));
    let mut o = [0u8; 16];
    for i in 0..16 {
        o[i] = x[i].wrapping_sub(y[i]);
    }
    let v = u128::from_le_bytes(o);
    [v as u64, (v >> 64) as u64]
}

/// `psubw`: per-word wrapping subtraction.
pub fn psubw(a: Gpr, b: Gpr) -> Gpr {
    let w = |x: u64, y: u64| {
        ((x as u32).wrapping_sub(y as u32) as u64)
            | (((x >> 32) as u32).wrapping_sub((y >> 32) as u32) as u64) << 32
    };
    [w(a[0], b[0]), w(a[1], b[1])]
}

/// `pand`.
pub fn pand(a: Gpr, b: Gpr) -> Gpr {
    [a[0] & b[0], a[1] & b[1]]
}

/// `pnor rd, $zero, x`: the complement.
pub fn pnot(a: Gpr) -> Gpr {
    [!a[0], !a[1]]
}

/// `pcpyld rd, rs, rt`: lower lanes, rt's below rs's.
pub fn pcpyld(rs: Gpr, rt: Gpr) -> Gpr {
    [rt[0], rs[0]]
}

/// `pcpyud rd, rs, rt`: upper lanes, rs's below rt's.
pub fn pcpyud(rs: Gpr, rt: Gpr) -> Gpr {
    [rs[1], rt[1]]
}

/// `lq` as a register image (straight to the bus; the low 4 address bits
/// are ignored).
pub fn lq(c: &mut Cpu, a: u32) -> Gpr {
    let v = c.bus.read128(a);
    [v as u64, (v >> 64) as u64]
}
