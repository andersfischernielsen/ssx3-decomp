//! Board wake (slot 90 `DrawBoardWake`) without packets: the wake ribbons
//! between consecutive vertex rings (`EmitBoardWakeStrips`, built from its
//! arguments as `crate::capture` reads them) as VU writes.

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::Cmd;

use super::*;
use crate::capture::{board_wake_input, ribbon_entry};
use crate::ps2f::FpuF32;

/// The GsPkt helper the ribbon and shadow-volume draws clear first.
const GS_PKT: u32 = 0x004a_5b84;
const DEFAULT_STATE: u32 = 0x0050_1420;

/// `PS2GraphicsMan_DrawBoardWake(f12 alpha, f13 step, gm, rings, count,
/// first, ring size, vertices)` (slot 90, 0x3883b8): one record for the
/// whole wake; afterwards the default render state.
pub fn draw_board_wake(c: &mut Cpu) {
    let gp = c.g[GP][0] as u32;
    let g = |c: &Cpu, r: usize| c.g[r][0] as u32;
    let (gm, rings, count) = (g(c, A0), g(c, A1), g(c, A2) as i32);
    let (first, size, n) = (g(c, A3) as i32, g(c, T0) as i32, g(c, T1) as i32);
    let (alpha, step) = (FpuF32(c.f[12]), FpuF32(c.f[13]));
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 0, !0x3c0, 0xc0);
    state_word(c, st, 3, 0xc000_0000, 0);
    ensure_view_matrices(c, gm, 0x0038_8444);
    let helper = r32(c, GS_PKT);
    for k in 0..12 {
        w32(c, helper + 4 * k, 0);
    }
    let mut steps = Vec::new();
    for k in 1..count.max(1) as u32 {
        let entry = ribbon_entry(c, 0x12c0);
        w32(c, gp + 0x12c0, 0);
        if let Some((e, input)) = board_wake_input(c, entry, alpha, step, (rings, first, size, n, k - 1, k)) {
            input.steps(e, &mut steps);
        }
    }
    let work = steps.len() as u64;
    let start = empty_packet(c, gm);
    record(start, Cmd::Vu(steps));
    add_record(c, gm, start, start, false);
    clear_type(c, gm);
    let st = r32(c, gm + gmf::CUR_STATE);
    for k in 0..5 {
        let v = r32(c, DEFAULT_STATE + 4 * k);
        w32(c, st + 4 * k, v);
    }
    ret(c, 200 + 40 * work);
}
