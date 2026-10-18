//! Draw capture (plan step N2, direct rendering, D1;
//! `docs/plans/native-rendering.md`): wrappers around the game's draw
//! functions. Each runs the generated body unchanged (guest memory, cycles
//! and event points stay exactly as without it) and records the draw as a
//! typed command for the renderer (`ssx3_render::direct`), keyed by the
//! packet the body wrote. Slots only: installed at boot unless
//! `SSX3_DIRECT=0`, not part of `OVERRIDES`; they record nothing while no
//! renderer takes the captures.

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{self, Cmd, Recorded, STRIP_BATCH};

const DRAW_TRI_STRIP: u32 = 0x0037_9860;

/// Install the wrappers; false when switched off.
pub fn install() -> bool {
    if std::env::var("SSX3_DIRECT").is_ok_and(|v| v == "0") {
        return false;
    }
    assert!(ssx3_ee::set_override(DRAW_TRI_STRIP, draw_tri_strip), "no override slot for PS2GraphicsMan_DrawTriStrip: list it in overrides.txt");
    true
}

fn qword(c: &mut Cpu, a: u32) -> u128 {
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
