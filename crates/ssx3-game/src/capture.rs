//! Draw capture (plan step N2, direct rendering, D1;
//! `docs/plans/native-rendering.md`): wrappers around the game's draw
//! functions. Each runs the generated body unchanged (guest memory, cycles
//! and event points stay exactly as without it) and records the draw as a
//! typed command for the renderer (`ssx3_render::direct`), keyed by the
//! packet the body wrote. Slots only: installed at boot unless
//! `SSX3_DIRECT=0`, not part of `OVERRIDES`; they record nothing while no
//! renderer takes the captures.

use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{self, Cmd, Recorded, STRIP_BATCH};

const DRAW_TRI_STRIP: u32 = 0x0037_9860;
const EMIT_BOARD_WAKE_STRIPS: u32 = 0x0038_85e0;
const EMIT_SHADOW_STRIPS: u32 = 0x0038_7ec0;

/// Install the wrappers; false when switched off.
pub fn install() -> bool {
    if std::env::var("SSX3_DIRECT").is_ok_and(|v| v == "0") {
        return false;
    }
    for (a, f, name) in [
        (DRAW_TRI_STRIP, draw_tri_strip as fn(&mut Cpu), "PS2GraphicsMan_DrawTriStrip"),
        (EMIT_BOARD_WAKE_STRIPS, emit_board_wake_strips, "PS2GraphicsMan_EmitBoardWakeStrips"),
        (EMIT_SHADOW_STRIPS, emit_shadow_strips, "PS2GraphicsMan_EmitShadowStrips"),
    ] {
        assert!(ssx3_ee::set_override(a, f), "no override slot for {name}: list it in overrides.txt");
    }
    true
}

fn qword(c: &mut Cpu, a: u32) -> u128 {
    if a & 15 == 0 {
        return c.bus.read128(a);
    }
    (0..4).fold(0u128, |q, l| q | (c.bus.read32(a + 4 * l) as u128) << (32 * l))
}

/// `PS2GraphicsMan_DrawTriStrip(gm, n, verts)` (slot 71, 0x379860): n
/// vertices of 3 qwords {STQ, RGBA, XYZW}; the packet goes to `gm+0x5a00`.
fn draw_tri_strip(c: &mut Cpu) {
    let ret = c.g[RA][0] as u32;
    let (gm, n, verts) = (c.g[A0][0] as u32, c.g[A1][0] as i64, c.g[A2][0] as u32);
    let pre = direct::active().then(|| {
        let entry = if c.bus.read32(c.g[GP][0] as u32 + 0x1290) == 0 { 0x12b8 } else { 0x12b0 };
        let state = c.bus.read32(gm + 0xe84);
        let prim = if (c.bus.read32(state + 0x10) as u16 as i16) < 0 { 0x4c } else { 0x5c };
        let start = c.bus.read32(gm + 0x5a00);
        let n = n.max(0) as u32;
        let v: Vec<[u128; 3]> = (0..if n > 2 { n } else { 0 }).map(|i| std::array::from_fn(|k| qword(c, verts + 48 * i + 16 * k as u32))).collect();
        (entry, prim, start, v)
    });
    ssx3_ee::s12::f_00379860__generated(c);
    // (Not when the body unwound: the packet takes the decoding path.)
    let Some((entry, prim, start, verts)) = pre.filter(|p| c.pc == ret && !p.3.is_empty()) else { return };
    // The packet's first DMA tag: CNT over the first batch (tag, count,
    // 3 qwords per vertex, the MSCAL qword), VIF NOPs; built in the
    // scratchpad, so not read back from RAM here.
    let k = verts.len().min(STRIP_BATCH) as u128;
    let first_tag = (3 * k + 4) | 0x1000_0000;
    let end = c.bus.read32(gm + 0x5a00).wrapping_sub(0x10);
    direct::record(Recorded { start, end, first_tag, cmd: Cmd::Strip { entry, prim, verts } });
}

/// A ribbon emitter's packet as VU1 input, per program start: (offset from
/// TOP, qword). Built from the emitter's arguments and the vertex rings
/// exactly as its UNPACKs place them (literal ports below).
struct RibbonInput {
    calls: Vec<Vec<(u16, u128)>>,
    cur: Vec<(u16, u128)>,
}

impl RibbonInput {
    fn new() -> Self {
        Self { calls: Vec::new(), cur: Vec::new() }
    }
    /// `n` vertices (3 qwords each) from guest memory at `src`, unpacked at
    /// offset `at`; `None` for a count the packet could not have.
    fn verts(&mut self, c: &mut Cpu, at: u32, src: u32, n: i32) -> Option<()> {
        if !(0..=0x48).contains(&n) {
            return None;
        }
        for i in 0..3 * n as u32 {
            let q = qword(c, src.wrapping_add(16 * i));
            self.cur.push(((at + i) as u16, q));
        }
        Some(())
    }
    /// The strip tag and the counts / alpha ramp (offsets 0, 1).
    fn head(&mut self, n: i32, tag_lo: u64, regs: u64, alpha: FpuF32, step: FpuF32) {
        let n = n as u128;
        self.cur.push((0, (2 * n) | tag_lo as u128 | (regs as u128) << 64));
        self.cur.push((1, 3 * n | n << 32 | (step.0 as u128) << 64 | (alpha.0 as u128) << 96));
    }
    fn start(&mut self) {
        self.calls.push(std::mem::take(&mut self.cur));
    }
}

/// The emitters' entry: 0x3a08, or 0x3a00 once after the flag at
/// `gp + flag` was set (the body clears it).
fn ribbon_entry(c: &mut Cpu, flag: u32) -> u32 {
    if c.bus.read32(c.g[GP][0] as u32 + flag) == 0 { 0x3a08 } else { 0x3a00 }
}

/// Run a ribbon emitter's body and record its segment of the packet.
fn capture_ribbon(c: &mut Cpu, body: fn(&mut Cpu), input: Option<(u32, RibbonInput)>) {
    let ret = c.g[RA][0] as u32;
    let gm = c.g[A0][0] as u32;
    let start = c.bus.read32(gm + 0x5a00);
    body(c);
    let Some((entry, input)) = input.filter(|(_, i)| c.pc == ret && !i.calls.is_empty()) else { return };
    let end = c.bus.read32(gm + 0x5a00);
    direct::record(Recorded { start, end, first_tag: 0x1000_0003, cmd: Cmd::Ribbon { entry, calls: input.calls } });
}

/// `PS2GraphicsMan_EmitBoardWakeStrips(f12 alpha, f13 step, gm, rings,
/// first, ring size, count, ring A, ring B)` (0x3885e0).
fn emit_board_wake_strips(c: &mut Cpu) {
    let input = direct::active().then(|| board_wake_input(c)).flatten();
    capture_ribbon(c, ssx3_ee::s12::f_003885e0__generated, input);
}

fn board_wake_input(c: &mut Cpu) -> Option<(u32, RibbonInput)> {
    let entry = ribbon_entry(c, 0x12c0);
    let (mut alpha, step) = (FpuF32(c.f[12]), FpuF32(c.f[13]));
    let rings = c.g[A1][0] as u32;
    let (mut p5, p6, mut p7) = (c.g[A2][0] as i32, c.g[A3][0] as i32, c.g[T0][0] as i32);
    let ring_a = c.bus.read32(rings.wrapping_add(4 * c.g[T1][0] as u32));
    let ring_b = c.bus.read32(rings.wrapping_add(4 * c.g[T2][0] as u32));
    let mut r = RibbonInput::new();
    let mut first = true;
    while p7 != 0 {
        if p7 < 0 || p6 <= 0 || r.calls.len() > 64 {
            return None;
        }
        let n = p7.min(0x24);
        let mut l12 = n.min(p6 - p5);
        let i13 = (n - l12).max(0);
        let (mut i10, mut i15) = (1, p5);
        if first {
            l12 -= 1;
            i15 = p5 + 1;
            i10 = p5;
        }
        r.head(n, 0x302e_4000_0000_8000, 0x412, alpha, step);
        let mut at = 2u32;
        for ring in [ring_a, ring_b] {
            if i10 >= 0 {
                r.verts(c, at, ring.wrapping_add(i10 as u32 * 0x30), 1)?;
                at += 3;
            }
            if l12 != 0 {
                r.verts(c, at, ring.wrapping_add(i15 as u32 * 0x30), l12)?;
                at += 3 * l12 as u32;
            }
            if i13 != 0 {
                r.verts(c, at, ring, i13)?;
                at += 3 * i13 as u32;
            }
        }
        r.start();
        p7 -= n;
        alpha = alpha + step * FpuF32::from_i32(n);
        p5 = i15;
        if p7 > 0 {
            p7 += 1;
            p5 = (i15 + n - 1) % p6;
            first = false;
        }
    }
    Some((entry, r))
}

/// `PS2GraphicsMan_EmitShadowStrips(f12 alpha, f13 step, gm, rings, first,
/// count, ring A, ring B, textured)` (0x387ec0): rings of 0x36 vertices.
fn emit_shadow_strips(c: &mut Cpu) {
    let input = direct::active().then(|| shadow_input(c)).flatten();
    capture_ribbon(c, ssx3_ee::s12::f_00387ec0__generated, input);
}

fn shadow_input(c: &mut Cpu) -> Option<(u32, RibbonInput)> {
    let entry = ribbon_entry(c, 0x12b0);
    let (mut alpha, step) = (FpuF32(c.f[12]), FpuF32(c.f[13]));
    let rings = c.g[A1][0] as u32;
    let (mut p5, mut p6) = (c.g[A2][0] as i32, c.g[A3][0] as i32);
    let ring_a = c.bus.read32(rings.wrapping_add(4 * c.g[T0][0] as u32));
    let ring_b = c.bus.read32(rings.wrapping_add(4 * c.g[T1][0] as u32));
    let (prim, regs) = if c.g[T2][0] == 0 { (4u64, 0x41f) } else { (0x5c, 0x412) };
    let tag_lo = 0x8000 | prim << 47 | 1 << 46 | 3 << 60;
    // The closing pair's default (a game global, -1/-1 when unused).
    let default_pair = c.bus.read32(0x004a_43a8) as u64 | (c.bus.read32(0x004a_43ac) as u64) << 32;
    let mut r = RibbonInput::new();
    while p6 != 0 {
        if p6 < 0 || r.calls.len() > 64 {
            return None;
        }
        let n = p6.min(0x24);
        let mut l14 = n.min(0x36 - p5);
        let mut i12 = (n - l14).max(0);
        let mut pair = default_pair;
        if p6 <= n {
            pair = (((p5 + n - 1) % 0x36) as u32 as u64) << 32 | ((p5 + n - 2) % 0x36) as u32 as u64;
            if i12 < 2 {
                l14 = l14 - 2 + i12;
                i12 = 0;
            } else {
                i12 -= 2;
            }
        }
        r.head(n, tag_lo, regs, alpha, step);
        let mut at = 2u32;
        for ring in [ring_a, ring_b] {
            if l14 != 0 {
                r.verts(c, at, ring.wrapping_add(p5 as u32 * 0x30), l14)?;
                at += 3 * l14 as u32;
            }
            if i12 != 0 {
                r.verts(c, at, ring, i12)?;
                at += 3 * i12 as u32;
            }
            if pair as i32 >= 0 {
                r.verts(c, at, ring.wrapping_add((pair as u32).wrapping_mul(0x30)), 1)?;
                r.verts(c, at + 3, ring.wrapping_add(((pair >> 32) as u32).wrapping_mul(0x30)), 1)?;
                at += 6;
            }
        }
        r.start();
        p6 -= n;
        alpha = alpha + step * FpuF32::from_i32(n);
        if p6 > 0 {
            p6 += 1;
            p5 = (p5 + n - 1) % 0x36;
        }
    }
    Some((entry, r))
}
