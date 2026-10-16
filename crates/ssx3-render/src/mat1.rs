//! # Type 1 (`f4babac5`) natively: Bezier terrain, uniform LOD, bit-exact
//!
//! A readable Rust implementation of the VU1 microprogram with hash
//! `0xf4babac52752998a` (`generated/vu/vu_f4babac52752998a.rs` is the
//! literal translation; `docs/investigation/phase3-render.md` §1.3). [`run`]
//! takes the program's VU1 data memory and produces the same XGKICK packets
//! **and** the same data memory as the translated program, bit for bit
//! (checked on real frames by `SSX3_VERIFY_BEZIER=1`,
//! `crates/ssx3-ps2/src/bezier_verify.rs`).
//!
//! The EE tessellates the patch on the CPU (`BezierPatchCache_Tess*`) into
//! an n×n grid (n = 4, 6 or 8, all four edges at the same LOD) and uploads,
//! from TOP: n² positions (TOP+0), n² UVs (TOP+0x40), and for the second
//! pass either n² UV2s (TOP+0x80) or a 4-column matrix at TOP+0x80. The
//! program draws the grid as n-1 two-row strips split over two packets:
//!
//! | Entries | What |
//! |---|---|
//! | 0x0610 / 0x0648 / 0x0680 | n = 4 / 6 / 8: grid setup, ctx1 strips |
//! | 0x06b8 / 0x0700 / 0x0748 | + pass 2 "flag 0": ST := UV2·q in place, re-kick with the ctx2 tag (clipped fans are not redrawn) |
//! | 0x0790 / 0x07d8 / 0x0820 | + pass 2 "flag 1": full re-draw with UV2, ctx2 strip and fan tags |
//! | 0x08d8 / 0x0910 | projector: UV2 = (L·P).xy for the previous call's positions, then pass 2 flag 0 / 1 |
//! | 0x0948 / 0x0980 | shine: UV2 = (R·N).xy for normals at TOP+0x40, then pass 2 flag 0 / 1 |
//! | 0x0868 / 0x08a0 | relight (3 point lights → RGBA); never used by the game, not implemented |
//!
//! Persistent state between calls lives in data memory only: qw39 {n,
//! table offset, n²} and qw510.x (the TOP of the last strip call: the
//! projector and pass 2 flag 1 read the positions from there), the tag
//! slots at 0x60/0x121 and the fan tags, the fan toggle at qw41.x.
//!
//! Per-n strip tables (qw11-16, written by the EE; `t` = 0/2/4 for n =
//! 4/6/8): qw11+t = {strips in packet A, strips in packet B, -, qword
//! offset of packet B's first row}; qw12+t = {NLOOP A, NLOOP B, ST-rewrite
//! qwords A, B}.

use crate::bezier::{
    emit_strip, project_xy, rewrite_st, ClipFlags, Mem, Raster, Rgba, Vertex, COLOUR, EOP, FAN_TAG_A, FAN_TAG_B,
    FAN_TAG_CTX1, FAN_TAG_CTX2, FAN_TOGGLE, LAST_TOP, PACKET_A, PACKET_B, RETURN_SAVE, STRIP_TAG_CTX1,
    STRIP_TAG_CTX2,
};

/// Hash of the program this module implements.
pub const HASH: u64 = 0xf4ba_bac5_2752_998a;

/// Register state the program carries from one call to the next.
///
/// Empty: no implemented entry reads a VU register before writing it
/// (checked against the translated program on real frames); everything that
/// crosses calls is in data memory. It exists so the API matches
/// [`crate::mat3`].
#[derive(Clone, Debug, Default)]
pub struct VuState {}

/// qw39: {n, strip table offset, n²} of the current grid.
const GRID: u16 = 39;
/// Strip tables: qw11+t, qw12+t (see the module doc).
const STRIP_TABLE: u16 = 11;
const PACKET_TABLE: u16 = 12;
/// UV grid, second-pass UV grid (or matrix), relative to TOP.
const UV: u16 = 0x40;
const UV2: u16 = 0x80;
/// Normals for the shine pass, relative to TOP.
const NORMALS: u16 = 0x40;

/// Entries [`run`] implements natively (MSCAL byte addresses).
pub fn implemented(entry: u32) -> bool {
    entry_kind(entry).is_some()
}

#[derive(Clone, Copy)]
enum Pass2 {
    None,
    /// 0x0c30: rewrite ST, re-kick.
    St,
    /// 0x0d40: full re-draw.
    Full,
}

#[derive(Clone, Copy)]
enum First {
    /// Grid setup for n, then the ctx1 draw.
    Draw(u16),
    /// 0x0b60: UV2 from the last call's positions through the matrix at TOP+0x80.
    Projector,
    /// 0x0bc8: UV2 from the normals at TOP+0x40 through the matrix at TOP+0x80.
    Shine,
}

fn entry_kind(entry: u32) -> Option<(First, Pass2)> {
    let n = |base: u32| -> u16 {
        match (entry - base) / 0x38 {
        0 => 4,
        1 => 6,
            _ => 8,
        }
    };
    Some(match entry {
        0x0610 | 0x0648 | 0x0680 => (First::Draw(n(0x0610)), Pass2::None),
        0x06b8 | 0x0700 | 0x0748 => (First::Draw(n(0x06b8)), Pass2::St),
        0x0790 | 0x07d8 | 0x0820 => (First::Draw(n(0x0790)), Pass2::Full),
        0x08d8 => (First::Projector, Pass2::St),
        0x0910 => (First::Projector, Pass2::Full),
        0x0948 => (First::Shine, Pass2::St),
        0x0980 => (First::Shine, Pass2::Full),
        _ => return None,
    })
}

/// The grid size and its tables, as qw39 holds them.
#[derive(Clone, Copy)]
struct Grid {
    n: u16,
    table: u16,
}

impl Grid {
    fn read(m: &Mem) -> Grid {
        Grid { n: m.ilw(GRID, 0), table: m.ilw(GRID, 1) }
    }
    /// The 0x0000 / 0x0048 / 0x0090 setup for n = 4 / 6 / 8.
    fn setup(m: &mut Mem, n: u16, top: u16) -> Grid {
        let table = n - 4;
        m.isw(LAST_TOP, 0, top);
        m.isw(GRID, 0, n);
        m.isw(GRID, 1, table);
        m.isw(GRID, 2, n * n);
        Grid { n, table }
    }
    fn strips(&self, m: &Mem) -> V2 {
        let t = m.ld(STRIP_TABLE + self.table);
        V2 { a: t[0] as u16, b: t[1] as u16, b_offset: t[3] as u16 }
    }
    fn packets(&self, m: &Mem) -> [u16; 4] {
        let t = m.ld(PACKET_TABLE + self.table);
        std::array::from_fn(|i| t[i] as u16)
    }
}

/// Strips in packets A and B, and packet B's first row (qword offset).
struct V2 {
    a: u16,
    b: u16,
    b_offset: u16,
}

/// The vertex order of `strips` two-row strips of an n-wide grid from
/// `pos`/`uv`: row k, row k+1 alternating, rows advancing by n.
fn grid_strips(n: u16, strips: u16, pos: u16, uv: u16) -> Vec<Vec<Vertex>> {
    (0..strips)
        .map(|s| {
            (0..2 * n)
                .map(|i| {
                    let off = (s + i % 2) * n + i / 2;
                    Vertex { pos: pos.wrapping_add(off), uv: uv.wrapping_add(off) }
                })
                .collect()
        })
        .collect()
}

/// The return address a `BAL vi15` at `pc` leaves in vi15 (code address / 8).
fn ret(pc: u32) -> u16 {
    ((pc + 16) >> 3) as u16
}

/// Run the program at `entry` with VIF `top` on VU1 data memory `mem`,
/// handing every XGKICK packet to `kick`. Returns `false` (and does
/// nothing) when the entry, or the variant selected by the input, is not
/// implemented natively.
pub fn run(
    entry: u32,
    top: u32,
    _itop: u32,
    mem: &mut [u8; 16384],
    _regs: &mut VuState,
    kick: &mut dyn FnMut(&[u128]),
) -> bool {
    let Some((first, pass2)) = entry_kind(entry) else {
        return false;
    };
    let top = (top & 0x3ff) as u16;
    let mut m = Mem(mem);
    // Inputs the original's do-while loops would not finish on.
    let grid = match first {
        First::Draw(n) => Grid { n, table: n - 4 },
        _ => Grid::read(&m),
    };
    if !matches!(grid.n, 4 | 6 | 8) || grid.table != grid.n - 4 {
        return false;
    }
    let s = grid.strips(&m);
    let pk = grid.packets(&m);
    if !(1..=8).contains(&s.a) || !(1..=8).contains(&s.b) {
        return false;
    }
    if matches!(pass2, Pass2::St) && (pk[2] == 0 || pk[2] % 12 != 0 || pk[3] == 0 || pk[3] % 12 != 0 || pk[2] > 0x200 || pk[3] > 0x200) {
        return false;
    }
    if matches!(first, First::Projector | First::Shine) && (pk[0] < 4 || pk[0] % 4 != 0) {
        return false;
    }

    match first {
        First::Draw(n) => {
            let g = Grid::setup(&mut m, n, top);
            m.isw(RETURN_SAVE, 0, ret(entry + 0x10));
            draw(&mut m, g, top, top.wrapping_add(UV), true, kick);
        }
        First::Projector | First::Shine => {
            // 0x0b60 / 0x0bc8 (BAL vi14: nothing saved).
            let mat = m.matrix(top.wrapping_add(0x80));
            let src = match first {
                First::Projector => m.ilw(LAST_TOP, 0),
                _ => top.wrapping_add(NORMALS),
            };
            project_xy(&mut m, &mat, src, top.wrapping_add(UV2), pk[0]);
        }
    }
    let pass2_bal = match first {
        First::Draw(_) => entry + 0x20,
        _ => entry + 0x10,
    };
    match pass2 {
        Pass2::None => {}
        Pass2::St => {
            m.isw(RETURN_SAVE, 0, ret(pass2_bal));
            pass2_st(&mut m, grid, top, kick);
        }
        Pass2::Full => {
            m.isw(RETURN_SAVE, 0, ret(pass2_bal));
            for (slot, tag) in [(PACKET_A, STRIP_TAG_CTX2), (PACKET_B, STRIP_TAG_CTX2), (FAN_TAG_A, FAN_TAG_CTX2), (FAN_TAG_B, FAN_TAG_CTX2)] {
                m.st_mask(slot, m.ld(tag), 0b0111);
            }
            let pos = m.ilw(LAST_TOP, 0);
            draw(&mut m, grid, pos, top.wrapping_add(UV2), false, kick);
        }
    }
    true
}

/// The strip draw (0x09b0 ctx1 / 0x0d40 ctx2 re-draw): packet A from the
/// first row, packet B from row `b_offset`, both kicked from fixed slots.
/// `ctx1` also writes the tags (NLOOP from the table, ctx1 PRIM) and resets
/// the fan toggle.
fn draw(m: &mut Mem, g: Grid, pos: u16, uv: u16, ctx1: bool, kick: &mut dyn FnMut(&[u128])) {
    let s = g.strips(m);
    if ctx1 {
        let pk = g.packets(m);
        m.st_mask(FAN_TAG_A, m.ld(FAN_TAG_CTX1), 0b0111);
        m.st_mask(FAN_TAG_B, m.ld(FAN_TAG_CTX1), 0b0111);
        m.isw(FAN_TOGGLE, 0, FAN_TAG_B);
        m.isw(PACKET_A, 0, pk[0] | EOP);
        m.st_mask(PACKET_A, m.ld(STRIP_TAG_CTX1), 0b0111);
        m.isw(PACKET_B, 0, pk[1] | EOP);
        m.st_mask(PACKET_B, m.ld(STRIP_TAG_CTX1), 0b0111);
    }
    let r = Raster::load(m);
    let colour = Rgba::Store(m.ld(COLOUR));
    let mut clip = ClipFlags::default();
    for (packet, strips, off) in [(PACKET_A, s.a, 0), (PACKET_B, s.b, s.b_offset)] {
        let mut out = packet + 1;
        for strip in grid_strips(g.n, strips, pos.wrapping_add(off), uv.wrapping_add(off)) {
            emit_strip(m, &r, &mut clip, &strip, &mut out, colour, &|_| None, kick);
        }
        kick(&m.gif_packet(packet));
    }
}

/// Pass 2 "flag 0" (0x0c30): ctx2 strip tags, ST := UV2·q in both packets
/// (the vertex order of the strips: rows k, k+1 alternating), re-kick.
fn pass2_st(m: &mut Mem, g: Grid, top: u16, kick: &mut dyn FnMut(&[u128])) {
    m.st_mask(PACKET_A, m.ld(STRIP_TAG_CTX2), 0b0111);
    m.st_mask(PACKET_B, m.ld(STRIP_TAG_CTX2), 0b0111);
    let s = g.strips(m);
    let pk = g.packets(m);
    let n = g.n;
    for (packet, qwords, off) in [(PACKET_A, pk[2], 0), (PACKET_B, pk[3], s.b_offset)] {
        let a = top.wrapping_add(off).wrapping_add(UV2);
        let uv2 = (0..qwords / 3).map(|i| a.wrapping_add(i / 2 + (i % 2) * n));
        rewrite_st(m, packet + 1, uv2);
        kick(&m.gif_packet(packet));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Replays sampled calls (`BEZIER_DUMP`, from `SSX3_VERIFY_BEZIER_DUMP`).
    #[test]
    fn dumped_calls() {
        crate::bezier::dump::replay(0, &|e, t, i, m, k| run(e, t, i, m, &mut VuState::default(), k));
    }
}
