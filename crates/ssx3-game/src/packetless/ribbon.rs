//! Board wake (slot 90 `DrawBoardWake`) without packets: the wake ribbons
//! between consecutive vertex rings (`EmitBoardWakeStrips`, built from its
//! arguments as `crate::capture` reads them) as VU writes.

use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::Cpu;
use ssx3_render::direct::{Cmd, Step};

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
        if let Some((e, input)) =
            board_wake_input(c, entry, alpha, step, (rings, first, size, n, k - 1, k))
        {
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

/// One GIF A+D block (`DIRECT`): a tag with `regs.len()` loops, EOP.
fn ad_block(regs: &[(u64, u8)]) -> Step {
    let mut q = vec![(regs.len() as u128 | 0x1000_0000_0000_8000) | 0xe << 64];
    q.extend(regs.iter().map(|&(v, r)| v as u128 | (r as u128) << 64));
    Step::Gif(q)
}

/// VU1 qw4-5 (the strip program's guard band), as the shadow volume sets
/// them between passes.
fn guard(c: &mut Cpu, gm: u32, zero_z: bool, steps: &mut Vec<Step>) {
    let mut rows = [v4(q(c, gm + 0x5840)), v4(q(c, gm + 0x5850))];
    if zero_z {
        rows[0][2] = 0;
        rows[1][2] = 0;
    }
    steps.push(Step::Cycle(0x0101));
    steps.push(Step::Write {
        addr: 4,
        rel: false,
        data: rows.iter().map(|r| q4(*r)).collect(),
    });
}

/// `DrawShadowMeshPass` / `DrawShadowEdgePass`: the shadow strips of every
/// list entry {rings, first, count, alpha, step} with more than one
/// vertex; mesh passes draw them at alpha 0.
fn shadow_pass(
    c: &mut Cpu,
    list: u32,
    count: i32,
    ia: u32,
    ib: u32,
    textured: bool,
    edge: bool,
    steps: &mut Vec<Step>,
) {
    let gp = c.g[GP][0] as u32;
    for i in 0..count.max(0) as u32 {
        let e = list + 0x14 * i;
        let n = r32(c, e + 8) as i32;
        if n <= 1 {
            continue;
        }
        let (alpha, step) = if edge {
            (FpuF32(r32(c, e + 0xc)), FpuF32(r32(c, e + 0x10)))
        } else {
            (FpuF32(0), FpuF32(0))
        };
        let entry = ribbon_entry(c, 0x12b0);
        w32(c, gp + 0x12b0, 0);
        let (rings, first) = (r32(c, e), r32(c, e + 4) as i32);
        if let Some((en, input)) =
            crate::capture::shadow_input(c, entry, alpha, step, (rings, first, n, ia, ib, textured))
        {
            input.steps(en, steps);
        }
    }
}

/// `PS2GraphicsMan_DrawShadowVolume(gm, list, count)` (slot 89,
/// 0x386e78): the board trail's shadow volume in one record: the volume's
/// faces into the destination alpha and Z (stencil-like FRAME / ZBUF /
/// TEST / FBA settings between the passes, the guard band with and without
/// depth), then the shadowed edges; afterwards the default render state.
pub fn draw_shadow_volume(c: &mut Cpu) {
    let (gm, list, count) = (c.g[A0][0] as u32, c.g[A1][0] as u32, c.g[A2][0] as i32);
    let st = r32(c, gm + gmf::CUR_STATE);
    for k in 0..5 {
        let v = r32(c, DEFAULT_STATE + 4 * k);
        w32(c, st + 4 * k, v);
    }
    state_word(c, st, 2, 0xffff_fc1f, 0x60);
    state_word(c, st, 0, !0x3c0, 0xc0);
    state_word(c, st, 3, 0xc000_0000, 0);
    ensure_view_matrices(c, gm, 0x0038_6f44);
    let st = r32(c, gm + gmf::CUR_STATE);
    state_word(c, st, 1, 0xffbf_ffff, 0x40_0000);
    state_word(c, st, 1, 0xfe7f_ffff, 0x100_0000);
    state_word(c, st, 1, 0xffcf_ffff, 0);
    state_word(c, st, 1, 0xfff0_0fff, 0);
    state_word(c, st, 1, 0xffff_ff83, 0x14);
    let tex_slot = r32(c, 0x004a_45d4);
    let fog_tex = r32(c, gm.wrapping_add(0xf50).wrapping_add(4 * tex_slot));
    c.bus.write16(st + 0x10, fog_tex as u16);
    let (this, func) = model::vmethod(c, gm, 0x54);
    let width = guest_call(c, func, &[this], 0x0038_7000) as i32;
    let fbw = (if width < 0 { width + 0x3f } else { width } >> 6) as u64;
    let (fbp, zbp) = (r32(c, gm + 0x5a84) as u64, r32(c, gm + 0x5a80) as u64);
    let helper = r32(c, GS_PKT);
    for k in 0..12 {
        w32(c, helper + 4 * k, 0);
    }
    let both = r32(c, 0x004a_45cc) != 0;
    let mut s = Vec::new();
    // Faces into alpha only, Z tested, not written.
    s.push(ad_block(&[
        (fbp | fbw << 16 | 0xff_ffff << 32, 0x4c),
        (zbp | 0x1_0100_0000, 0x4e),
        (0x3_0000, 0x47),
    ]));
    guard(c, gm, false, &mut s);
    shadow_pass(c, list, count, 0, 1, false, false, &mut s);
    s.push(ad_block(&[(0x5_0000, 0x47), (1, 0x4a)]));
    shadow_pass(c, list, count, 0, 1, false, false, &mut s);
    s.push(ad_block(&[
        (zbp | 0x100_0000, 0x4e),
        (0x3_c000, 0x47),
        (0, 0x4a),
    ]));
    guard(c, gm, true, &mut s);
    shadow_pass(c, list, count, 0, 1, false, false, &mut s);
    let mut regs = vec![(fbp | fbw << 16, 0x4c), (0x5_0000, 0x47)];
    if both {
        regs.push((1, 0x4a));
    }
    s.push(ad_block(&regs));
    guard(c, gm, false, &mut s);
    for (a, b) in [(2, 0), (0, 3), (3, 4), (4, 1), (1, 5)] {
        shadow_pass(c, list, count, a, b, true, true, &mut s);
    }
    if both {
        s.push(ad_block(&[(0x5_4000, 0x47), (0, 0x4a)]));
        shadow_pass(c, list, count, 0, 1, true, true, &mut s);
        s.push(ad_block(&[(0x5_0000, 0x47)]));
    }
    let work = s.len() as u64;
    let start = empty_packet(c, gm);
    record(start, Cmd::Vu(s));
    add_record(c, gm, start, start, false);
    clear_type(c, gm);
    let st = r32(c, gm + gmf::CUR_STATE);
    for k in 0..5 {
        let v = r32(c, DEFAULT_STATE + 4 * k);
        w32(c, st + 4 * k, v);
    }
    ret(c, 400 + 40 * work);
}
