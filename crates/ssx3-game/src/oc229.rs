//! Staging wave 204 (oc229): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0016_2c78,
        chase_camera_update_chase_heading,
        "ChaseCameraAlgorithm_UpdateChaseHeading",
    ),
    (0x0042_4698, sync_dcache, "SyncDCache"),
    (0x002e_6008, snow_fall_render, "SnowFall_Render"),
    (0x0026_68e8, usb_keyboard_update, "cUsbKeyboard_Update"),
    (
        0x002b_b6e0,
        world_painter_man_debug_draw_current,
        "cWorldPainterMan_DebugDrawCurrent",
    ),
    (0x0031_a490, ps2_exec_man_vblank_end_intc, "PS2ExecMan_VBlankEndIntc"),
    (0x0013_5180, rider_spin_sub_get_spin_factor, "RiderSpinSub_GetSpinFactor"),
    (0x0042_6230, sce_sif_cmd_intr_handler, "sceSifCmdIntrHandler"),
    (0x003b_6b10, sndstrm_consume_callback, "SNDSTRM_ConsumeCallback"),
];

/// gp-relative address `gp + off`.
#[inline(always)]
fn gp_at(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// ChaseCameraAlgorithm_UpdateChaseHeading (0x162c78)

/// Fields of the chase-camera algorithm (`this`, a0).
mod chase {
    /// The rider/model object (first word the vtable; +4 the rider).
    pub const RIG: u32 = 0x30;
    /// `vec4` smoothed chase heading.
    pub const HEADING: u32 = 0x80;
    /// `vec4` last valid look direction from the rig.
    pub const LAST_DIR: u32 = 0x1c0;
    /// The heading-angle PID filter (`sHist`): `a[5]` measured angles,
    /// `b[5]` filtered values (+0x14), `c[5]` errors (+0x28), `idx` (+0x3c),
    /// gains `k0`, `k1`, `k2` (+0x40/+0x44/+0x48).
    pub const HIST: u32 = 0x340;
    pub const H_B: u32 = 0x14;
    pub const H_C: u32 = 0x28;
    pub const H_IDX: u32 = 0x3c;
    pub const H_K0: u32 = 0x40;
    pub const H_K1: u32 = 0x44;
    pub const H_K2: u32 = 0x48;
    /// Rider state word (in the rider at rig+4); states 0xb..0xd slow the
    /// heading follow (double square root of the blend).
    pub const RIDER_STATE: u32 = 0x434;
}

/// Fields of the chase info block (a1).
mod chase_info {
    /// `vec4` travel direction (scaled by 1/speed).
    pub const DIR: u32 = 0x20;
    /// `vec4` output: the normalised heading.
    pub const OUT: u32 = 0x30;
    /// Speed (m/s).
    pub const SPEED: u32 = 0x44;
}

/// `.lit4` constants (gp-relative).
mod chase_lit {
    /// 27.78 m/s (100 km/h): below this the heading is left alone.
    pub const MIN_SPEED: i32 = -0x6ae0;
    /// 0.001: shorter vector products give an angle of 0.
    pub const MIN_LEN: i32 = -0x6adc;
    /// pi/2 (acos = pi/2 - asin).
    pub const HALF_PI: i32 = -0x6ad8;
    /// 0.1908 rad dead zone subtracted from the heading error.
    pub const DEAD_ZONE: i32 = -0x6ad4;
    /// 2.52: scale of the filtered angle before clamping to [0, 1].
    pub const ANGLE_SCALE: i32 = -0x6ad0;
    /// Blend `s = (1 - clamp) * 0.0395 + 0.9555`.
    pub const BLEND_RANGE: i32 = -0x6acc;
    pub const BLEND_BASE: i32 = -0x6ac8;
}

/// `vf4.x = a . b` (four lanes) as the original's VU0 macro sequence
/// leaves it: vf3 = a, vf5 = b, vf6.x = 1, ACC.x scratch.
fn vu_dot4(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    vu_dot4_regs(c, 5);
}

/// The dot product of vf3 with vf`b` into vf4.x.
fn vu_dot4_regs(c: &mut Cpu, b: u8) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `vf5 = vf4 * s` with `s` from FPU register `f` (through v1/vf3), stored
/// to `tmp` and copied (via v0) to `dst`, as the original's inlined
/// vector-scale does.
fn vu_scale_to(c: &mut Cpu, f: usize, src: u32, tmp: u32, dst: u32) {
    c.mfc1(3, f);
    c.lqc2(4, src);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, tmp);
    c.lq(2, tmp);
    c.sq(2, dst);
}

/// `void ChaseCameraAlgorithm_UpdateChaseHeading(ChaseCameraAlgorithm *this,
/// ChaseInfo *info)` (0x162c78): above 100 km/h, measure the angle between
/// the smoothed heading and the rig's look direction (virtual call at
/// vtable +0x18; a near-zero direction keeps the last one), run it through
/// the heading PID filter (5-entry history at +0x340), and blend the heading
/// toward the travel direction by the filtered blend factor (slower in rider
/// states 0xb..0xd). Always writes the normalised heading to `info->out`.
pub fn chase_camera_update_chase_heading(c: &mut Cpu) {
    use chase::*;
    use chase_info::*;
    use chase_lit::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xb0);
        c.lwc1(1, gp_at(c, MIN_SPEED));
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S1][0] = c.g[A1][0];
        let info = r32(c, S1);
        c.lwc1(0, info.wrapping_add(SPEED));
        c.c_lt_s(1, 0);
        let fast = c.fcc();
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        if !fast {
            // Slow: keep the heading as it is.
            c.cycles += 1;
            c.lq(2, this.wrapping_add(HEADING));
        } else {
            update_heading(c, frame, this, info)?;
        }

        // Normalise the heading (v0) into info->out.
        c.cycles += 25;
        let tmp = frame.at(c, 0);
        c.sq(2, tmp);
        c.lqc2(3, tmp);
        vu_dot4_regs(c, 3);
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 0);
        frame.restore(c, S0, 0xa0);
        c.mfc1(3, 0);
        c.qmtc2(3, 3);
        c.vu0().div(0, 3, 3, 0);
        let tmp = frame.at(c, 0);
        c.lqc2(4, tmp);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, tmp);
        frame.restore_ra(c, 0x80);
        c.lq(2, tmp);
        c.sq(2, info.wrapping_add(OUT));
        frame.restore(c, S1, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// The fast path of [`chase_camera_update_chase_heading`]: leaves the new
/// heading in v0 (and at `this->heading`).
fn update_heading(c: &mut Cpu, frame: Frame, this: u32, info: u32) -> crate::call::Flow {
    use chase::*;
    use chase_info::*;
    use chase_lit::*;
    // The rig's look direction (virtual, vtable slot +0x18) into sp+0x10.
    c.cycles += 22;
    let rig = Addr(this.wrapping_add(RIG)).lw(c);
    c.g[A2][0] = rig;
    c.g[A0][0] = sx(frame.at(c, 0x10));
    c.g[V0][0] = Addr(rig as u32).lw(c);
    let vt = c.g[V0][0] as u32;
    c.g[A1][0] = Addr(vt.wrapping_add(0x18)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(0x1c)).lw(c);
    c.g[A1][0] = sx((rig as u32).wrapping_add(c.g[A1][0] as u32));
    let method = c.g[V1][0] as u32;
    call_ptr(c, method, 0x0016_2cc0)?;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 4);
    let dir = frame.at(c, 0x10);
    vu_dot4(c, dir, dir);
    c.qmfc2(2, 4);
    c.mtc1(2, 0);
    c.c_lt_s(0, 4);
    let short = c.fcc();
    c.lq(2, dir);
    if short {
        // A near-zero direction: reuse the last one.
        c.cycles += 3;
        c.lq(2, this.wrapping_add(LAST_DIR));
        c.sq(2, dir);
        c.lq(2, dir);
    }

    // The angle between the heading and the direction (acos via pi/2 - asin).
    c.cycles += 25;
    c.lwc1(2, gp_at(c, MIN_LEN));
    c.sq(2, this.wrapping_add(LAST_DIR));
    vu_dot4(c, this.wrapping_add(HEADING), this.wrapping_add(HEADING));
    c.qmfc2(3, 4);
    c.mtc1(3, 0);
    vu_dot4(c, this.wrapping_add(LAST_DIR), this.wrapping_add(LAST_DIR));
    c.qmfc2(2, 4);
    c.mtc1(2, 1);
    c.mul_s(0, 0, 1);
    c.sqrt_s(3, 0);
    c.c_lt_s(3, 2);
    if c.fcc() {
        c.cycles += 3;
        c.mtc1(0, 7);
    } else {
        c.cycles += 16;
        vu_dot4(c, this.wrapping_add(HEADING), this.wrapping_add(LAST_DIR));
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 1);
        c.div_s(0, 0, 3);
        c.c_le_s(1, 0);
        let in_range = c.fcc();
        c.mov_s(12, 1);
        if in_range {
            c.cycles += 1;
            c.min_s(12, 0, 4);
        }
        c.cycles += 5;
        call_ptr(c, 0x0031_c128, 0x0016_2dc4)?; // Math_Asin(clamped cos)
        c.lwc1(1, gp_at(c, HALF_PI));
        c.sub_s(7, 1, 0);
    }
    c.g[V0][0] = Addr(this.wrapping_add(HIST + H_IDX)).lw(c);

    // ang = max(ang - dead zone, 0); prev = b[(idx + 4) % 5].
    c.cycles += 4;
    c.g[V1][0] = 5;
    c.lwc1(1, gp_at(c, DEAD_ZONE));
    c.cycles += 16;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
    c.mtc1(0, 0);
    c.div(0, V0, V1);
    c.sub_s(7, 7, 1);
    c.g[V0][0] = 5;
    let errs = this.wrapping_add(HIST + H_C);
    c.g[T0][0] = sx(errs);
    c.mov_s(2, 0);
    c.max_s(7, 7, 0);
    let hist = this.wrapping_add(HIST);
    c.g[A3][0] = sx(hist);
    c.g[V0][0] = c.g[T0][0];
    c.g[V1][0] = 4;
    c.g[A0][0] = sx(this.wrapping_add((c.hi[0] as u32) << 2));
    c.lwc1(4, (c.g[A0][0] as u32).wrapping_add(HIST + H_B));
    c.sub_s(3, 7, 4);

    // sum = c[0] + ... + c[4] (f2).
    loop {
        c.cycles += 7;
        c.lwc1(0, c.g[V0][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        let more = (c.g[V1][0] as i64) >= 0;
        c.add_s(2, 2, 0);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0016_2e20);
        }
    }

    // val = prev + d*k0 + sum*k1 + (a[p2] - ang)*k2; store a/b/c[idx];
    // idx = (idx + 1) % 5; blend s from val.
    c.cycles += 4;
    c.g[A0][0] = Addr(hist.wrapping_add(H_IDX)).lw(c);
    c.g[A1][0] = 5;
    c.cycles += 52;
    c.g[A2][0] = 5;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
    c.lwc1(0, hist.wrapping_add(H_K0));
    c.div(0, V0, A1);
    c.lwc1(1, hist.wrapping_add(H_K1));
    c.mul_s(0, 3, 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    c.mul_s(1, 2, 1);
    c.lwc1(3, hist.wrapping_add(H_K2));
    c.g[A0][0] = sx(hist.wrapping_add(c.g[A0][0] as u32));
    c.lwc1(6, gp_at(c, ANGLE_SCALE));
    c.add_s(0, 4, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 8);
    c.mtc1(0, 2);
    c.lwc1(4, gp_at(c, BLEND_RANGE));
    c.add_s(0, 0, 1);
    c.lwc1(5, gp_at(c, BLEND_BASE));
    c.g[V1][0] = sx(hist.wrapping_add((c.hi[0] as u32) << 2));
    c.lwc1(1, c.g[V1][0] as u32);
    c.swc1(7, c.g[A0][0] as u32);
    c.sub_s(1, 1, 7);
    c.g[V0][0] = Addr(hist.wrapping_add(H_IDX)).lw(c);
    c.mul_s(1, 1, 3);
    c.g[V0][0] = sx(hist.wrapping_add((c.g[V0][0] as u32) << 2));
    c.add_s(0, 0, 1);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(H_B));
    c.sub_s(1, 7, 0);
    c.mul_s(0, 0, 6);
    c.g[V0][0] = Addr(hist.wrapping_add(H_IDX)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.min_s(0, 0, 8);
    c.g[V0][0] = sx(errs.wrapping_add(c.g[V0][0] as u32));
    c.max_s(0, 0, 2);
    c.swc1(1, c.g[V0][0] as u32);
    c.sub_s(0, 8, 0);
    c.g[V0][0] = Addr(hist.wrapping_add(H_IDX)).lw(c);
    c.mul_s(0, 0, 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.div(0, V0, A1);
    c.g[A0][0] = c.hi[0];
    Addr(hist.wrapping_add(H_IDX)).sw(c, c.g[A0][0]);
    c.g[V1][0] = Addr(this.wrapping_add(RIG)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(RIDER_STATE)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb));
    c.g[V0][0] = (c.g[V0][0] < 3) as u64;
    let slow_state = c.g[V0][0] != 0;
    c.add_s(2, 0, 5);
    if slow_state {
        c.cycles += 2;
        c.sqrt_s(0, 2);
        c.sqrt_s(2, 0);
    }

    // heading = dir * (1/speed) * (1 - s) + heading * s.
    c.cycles += 32;
    c.lwc1(0, info.wrapping_add(SPEED));
    c.sub_s(1, 8, 2);
    c.div_s(0, 8, 0);
    let (t20, t30) = (frame.at(c, 0x20), frame.at(c, 0x30));
    vu_scale_to(c, 0, info.wrapping_add(DIR), t30, t20);
    let (t50, t60, t70) = (frame.at(c, 0x50), frame.at(c, 0x60), frame.at(c, 0x70));
    vu_scale_to(c, 2, this.wrapping_add(HEADING), t60, t50);
    vu_scale_to(c, 1, t20, t70, t60);
    c.lqc2(3, t50);
    c.lqc2(4, t60);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, t70);
    c.lq(2, t70);
    c.sq(2, this.wrapping_add(HEADING));
    let t40 = frame.at(c, 0x40);
    c.sq(2, t40);
    Ok(())
}


// ---------------------------------------------------------------------------
// SyncDCache (0x424698)

/// `void SyncDCache(void* start, void* end)` (0x424698, libkernel): write
/// back the data cache lines covering [start, end] (both rounded down to 64
/// bytes, `_SyncDCache` at 0x4245f0) with interrupts disabled; re-enables
/// them (tail call to `EIntr`) if they were enabled on entry (Status.EIE,
/// COP0 r12 bit 16). Same shape as `InvalidDCache`.
pub fn sync_dcache(c: &mut Cpu) {
    use crate::game::lib::kernel::{DINTR, EINTR};
    const SYNC_DCACHE_LINES: Callee = Callee {
        addr: 0x0042_45f0,
        f: ee::s15::f_004245f0,
    };
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        let at = frame.at(c, 0x20);
        c.sd(at, c.g[S2][0]);
        let at = frame.at(c, 0x10);
        c.sd(at, c.g[S1][0]);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        let at = frame.at(c, 0);
        c.sd(at, c.g[S0][0]);
        c.mfc0(S0, 12);
        c.g[V0][0] = 0x10000;
        c.g[S0][0] &= 0x10000;
        let enabled = c.g[S0][0] != 0;
        if enabled {
            c.cycles += 2;
            DINTR.call(c, 0x0042_46d0)?;
        }
        c.cycles += 7;
        const LINE_MASK: u64 = 0xffff_ffff_ffff_ffc0;
        c.g[A0][0] = LINE_MASK;
        c.g[A1][0] = c.g[S1][0] & LINE_MASK;
        c.g[A0][0] = c.g[S2][0] & LINE_MASK;
        SYNC_DCACHE_LINES.call(c, 0x0042_46e4)?;
        frame.restore_ra(c, 0x30);
        c.cycles += 5;
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        if enabled {
            // Tail call: EIntr returns to our caller.
            c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
            c.pc = EINTR;
            ee::s15::f_0042c0c0(c);
        } else {
            frame.close(c);
        }
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SnowFall_Render (0x2e6008)

/// Renderer context (`gp+0x2a90`) fields.
mod rctx {
    /// Top of the 0x14-byte render-state stack.
    pub const STATE_TOP: u32 = 0xe84;
    /// The current texture handle (copied into the pushed state +0x10).
    pub const TEX: u32 = 0xf70;
    /// The vptr; slot +0x118 returns the view matrix.
    pub const VPTR: u32 = 0x10d8;
    pub const GET_VIEW: u32 = 0x118;
}

/// Copy a 0x14-byte render state (two unaligned doublewords through GPRs
/// `ra`/`rb`, then a word through `rw`) from `src` to `dst`, as the
/// original's `ldl/ldr/sdl/sdr` sequence does.
fn copy_render_state(c: &mut Cpu, src: u32, dst: u32, ra: usize, rb: usize, rw: usize) {
    c.ldl(ra, src.wrapping_add(0x7));
    c.ldr(ra, src);
    c.ldl(rb, src.wrapping_add(0xf));
    c.ldr(rb, src.wrapping_add(0x8));
    c.g[rw][0] = Addr(src.wrapping_add(0x10)).lw(c);
    c.sdl(ra, dst.wrapping_add(0x7));
    c.sdr(ra, dst);
    c.sdl(rb, dst.wrapping_add(0xf));
    c.sdr(rb, dst.wrapping_add(0x8));
    Addr(dst.wrapping_add(0x10)).sw(c, c.g[rw][0]);
}

/// `void SnowFall_Render(SnowFall *this)` (0x2e6008): unless rendering is
/// off (`gp+0x1434`), take the current rider's movement since the last
/// frame (position at rider+0x20 minus `this->last[k]` at +0x40 + 16k),
/// push a render state (the template at 0x501420; alpha/blend bits, the
/// fog bit from `gp+0x1670`, the current texture), fetch the view matrix
/// (renderer vtable +0x118) with its translation replaced by 0x4ff130, and
/// for each of the 12 snow boxes (+0x10) owned by that rider wrap it by the
/// movement (`SnowBox_WrapOffset`) and draw it (`SnowBox_Render`) tinted by
/// the rider's colour * 0.6 - 0.5. Pops the render state.
pub fn snow_fall_render(c: &mut Cpu) {
    const WRAP: Callee = Callee {
        addr: 0x002e_5430,
        f: ee::s09::f_002e5430,
    };
    const RENDER: Callee = Callee {
        addr: 0x002e_55d8,
        f: ee::s09::f_002e55d8,
    };
    /// The render-state template pushed for the snow.
    const STATE_TEMPLATE: u32 = 0x0050_1420;
    /// Translation row put into the view matrix copy.
    const VIEW_ORIGIN: u32 = 0x004f_f130;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xf0);
        c.g[V0][0] = Addr(gp_at(c, 0x1434)).lw(c);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 22, 0xe0);
        frame.save_fpr(c, 21, 0xd8);
        frame.save_fpr(c, 20, 0xd0);
        let disabled = c.g[V0][0] != 0;
        if !disabled {
            let this = r32(c, S0);
            // The current rider k and its movement since the last frame.
            c.cycles += 64;
            c.g[V0][0] = Addr(gp_at(c, -0x848)).lw(c);
            c.g[A3][0] = 0x50_0000;
            c.g[A2][0] = 0xffff_ffff_ffbf_ffff;
            c.g[T0][0] = 0x40_0000;
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[T1][0] = 0xffff_ffff_ffff_fc1f;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[S3][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
            let k = r32(c, S3);
            c.g[V1][0] = sx(k << 2);
            c.g[A0][0] = sx(k << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(k << 2));
            let last = this.wrapping_add(k << 4);
            c.g[A1][0] = sx(last);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(4)).lw(c);
            c.g[A0][0] = c.g[A1][0];
            let rider = r32(c, V1);
            let (t0, t10, t20) = (frame.at(c, 0), frame.at(c, 0x10), frame.at(c, 0x20));
            c.lq(2, rider.wrapping_add(0x20));
            c.sq(2, t0);
            c.lqc2(3, t0);
            c.lqc2(4, last.wrapping_add(0x40));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, t20);
            c.lq(2, t20);
            c.lq(3, t0);
            c.sq(2, t10);
            c.g[S1][0] = Addr(gp_at(c, 0x2a90)).lw(c);
            let rc = r32(c, S1);
            c.sq(3, last.wrapping_add(0x40));

            // Push the render state: duplicate the top, then load the template.
            let top = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c) as u32;
            c.g[V0][0] = sx(top);
            copy_render_state(c, top, top.wrapping_add(0x14), V1, A0, A1);
            c.g[V1][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            let top = (c.g[V1][0] as u32).wrapping_add(0x14);
            c.g[V0][0] = sx(top);
            Addr(rc.wrapping_add(rctx::STATE_TOP)).sw(c, c.g[V0][0]);
            c.g[T2][0] = sx(STATE_TEMPLATE);
            let prev = r32(c, V1);
            copy_render_state(c, STATE_TEMPLATE, prev.wrapping_add(0x14), V0, A0, A1);
            // Word 1: bit 22 set; word 2: bits 5..9 = 7.
            c.g[A0][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            let st = r32(c, A0);
            c.g[V0][0] = (Addr(st.wrapping_add(4)).lw(c) & c.g[A2][0]) | c.g[T0][0];
            Addr(st.wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A1][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            let st = r32(c, A1);
            c.g[V0][0] = (Addr(st.wrapping_add(8)).lw(c) & c.g[T1][0]) | 0xe0;
            Addr(st.wrapping_add(8)).sw(c, c.g[V0][0]);
            // Word 1 bits 2..6: 7 with the fog flag, else 5.
            let fog = Addr(gp_at(c, 0x1670)).lw(c) != 0;
            c.g[V1][0] = 0xffff_ffff_ffff_ff83;
            c.g[A0][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            let st = r32(c, A0);
            c.g[V0][0] = Addr(st.wrapping_add(4)).lw(c) & c.g[V1][0];
            if fog {
                c.cycles += 5;
                c.g[V0][0] |= 0x1c;
            } else {
                c.cycles += 4;
                c.g[V0][0] |= 0x14;
            }
            // Word 1 bits 0..1 = 2; word 2 bits 10..28 cleared; texture.
            c.cycles += 58;
            Addr(st.wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A1][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            let st = r32(c, A1);
            c.g[A0][0] = 0xffff_ffff_ffff_fffc;
            c.g[V1][0] = 0xffff_ffff_e000_03ff;
            c.g[V0][0] = (Addr(st.wrapping_add(4)).lw(c) & c.g[A0][0]) | 2;
            Addr(st.wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A0][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            let st = r32(c, A0);
            c.g[V0][0] = Addr(st.wrapping_add(8)).lw(c) & c.g[V1][0];
            Addr(st.wrapping_add(8)).sw(c, c.g[V0][0]);
            c.g[V0][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
            c.g[A1][0] = Addr(rc.wrapping_add(rctx::TEX)).lw(c);
            Addr((c.g[V0][0] as u32).wrapping_add(0x10)).sh(c, c.g[A1][0]);

            // The view matrix (renderer vtable +0x118) into sp+0x30.
            c.g[V1][0] = Addr(rc.wrapping_add(rctx::VPTR)).lw(c);
            let vt = r32(c, V1);
            c.g[A0][0] = Addr(vt.wrapping_add(rctx::GET_VIEW)).lh(c);
            c.g[V0][0] = Addr(vt.wrapping_add(rctx::GET_VIEW + 4)).lw(c);
            c.g[A0][0] = sx(rc.wrapping_add(c.g[A0][0] as u32));
            let method = r32(c, V0);
            call_ptr(c, method, 0x002e_61b4)?;
            let m = frame.at(c, 0x30);
            c.g[V1][0] = sx(m);
            let src = r32(c, V0);
            for i in 0..4u32 {
                c.lqc2(1 + i as usize, src.wrapping_add(0x10 * i));
            }
            for i in 0..4u32 {
                c.sqc2(1 + i as usize, m.wrapping_add(0x10 * i));
            }
            c.g[V0][0] = 0x50_0000;
            c.g[S4][0] = c.g[V1][0];
            c.g[V0][0] = sx(VIEW_ORIGIN);
            c.g[A1][0] = Addr(gp_at(c, -0x848)).lw(c);
            c.lq(3, VIEW_ORIGIN);
            c.g[S0][0] = sx(this.wrapping_add(0x10));
            c.lwc1(3, gp_at(c, -0x3a04));
            c.g[S2][0] = 0xb;
            let t60 = frame.at(c, 0x60);
            c.sq(3, t60);
            c.g[AT][0] = 0xffff_ffff_bf00_0000;
            c.mtc1(1, 4);
            // The tint: rider colour (+0x80) * 0.6 - 0.5 into f22/f21/f20.
            c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
            c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x80));
            c.sq(2, t20);
            c.lwc1(0, t20);
            c.lwc1(1, t20 + 4);
            c.lwc1(2, t20 + 8);
            c.mul_s(0, 0, 3);
            c.mul_s(1, 1, 3);
            c.mul_s(2, 2, 3);
            c.add_s(22, 0, 4);
            c.add_s(21, 1, 4);
            c.add_s(20, 2, 4);

            // The 12 snow boxes.
            loop {
                c.cycles += 3;
                let slot = r32(c, S0);
                c.g[A0][0] = Addr(slot).lw(c);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                    if c.g[V0][0] == c.g[S3][0] {
                        c.cycles += 9;
                        c.g[A1][0] = sx(frame.at(c, 0x10));
                        WRAP.call(c, 0x002e_6268)?;
                        c.g[A0][0] = Addr(slot).lw(c);
                        c.g[A1][0] = c.g[S4][0];
                        c.mov_s(12, 22);
                        c.mov_s(13, 21);
                        c.mov_s(14, 20);
                        RENDER.call(c, 0x002e_6280)?;
                    }
                }
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                c.cycles += 2;
                let more = (c.g[S2][0] as i64) >= 0;
                c.g[S0][0] = sx(slot.wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_6248);
                }
            }
            // Pop the render state.
            c.cycles += 3;
            c.g[V0][0] = sx((Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c) as u32).wrapping_sub(0x14));
            Addr(rc.wrapping_add(rctx::STATE_TOP)).sw(c, c.g[V0][0]);
        }
        c.cycles += 11;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 22, 0xe0);
        frame.restore_fpr(c, 21, 0xd8);
        frame.restore_fpr(c, 20, 0xd0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cUsbKeyboard_Update (0x2668e8)

/// The USB keyboard library entry points (libusbkb, all reached through the
/// function table).
mod usbkb {
    /// `UsbKb_Poll()`: advance the library's async state machine.
    pub const POLL: u32 = 0x003f_63f0;
    /// `UsbKb_IsConnected(port) -> bool`.
    pub const IS_CONNECTED: u32 = 0x003f_6520;
    /// `UsbKb_GetMaxPorts() -> int`.
    pub const MAX_PORTS: u32 = 0x003f_6348;
    /// `UsbKb_GetData(port) -> KbData*` (null when nothing new).
    pub const GET_DATA: u32 = 0x003f_64e0;
    /// `UsbKb_GetKey(data, 0, i) -> u8[2]` (character, raw code).
    pub const GET_KEY: u32 = 0x003f_68f0;
    /// `UsbKb_IsKeyDown(data, 0, 0, key) -> bool`.
    pub const IS_KEY_DOWN: u32 = 0x003f_6838;
}

/// `cUsbKeyboard` fields: 8 port slots of 0x8c bytes (`count` +0, `start`
/// +4, a 64-entry ring of 2-byte keys at +8, flags at +0x88), and the
/// enable flag.
mod usbk {
    pub const SLOT_SIZE: u32 = 0x8c;
    pub const SLOT_COUNT: u32 = 0x0;
    pub const SLOT_START: u32 = 0x4;
    pub const SLOT_RING: u32 = 0x8;
    /// Flags: bit 0 connected, bits 1..3 the data's "on" bit, bit 4 key
    /// 0x10 held.
    pub const SLOT_FLAGS: u32 = 0x88;
    pub const ENABLED: u32 = 0x460;
    /// `KbData`: the "on" word (+0) and the key count (+8).
    pub const DATA_COUNT: u32 = 0x8;
}

/// `void cUsbKeyboard_Update(cUsbKeyboard *this)` (0x2668e8): when enabled,
/// poll the USB keyboard library, refresh each port's connected bit (a
/// disconnected port's flags are cleared), then for each connected port with
/// new data append its non-empty keys to the port's 64-entry ring (dropping
/// the oldest when full), and record the held state of key 0x10 and the
/// data's "on" bit in the port flags.
pub fn usb_keyboard_update(c: &mut Cpu) {
    use usbk::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S3, 0x80);
        frame.save(c, S0, 0xb0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        frame.save_ra(c, 0x20);
        let this = r32(c, S3);
        c.g[V0][0] = Addr(this.wrapping_add(ENABLED)).lw(c);
        let enabled = c.g[V0][0] != 0;
        frame.restore(c, S0, 0xb0);
        if enabled {
            usb_keyboard_poll(c, frame, this)?;
            c.cycles += 1;
            frame.restore(c, S0, 0xb0);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The enabled body of [`usb_keyboard_update`].
fn usb_keyboard_poll(c: &mut Cpu, frame: Frame, this: u32) -> crate::call::Flow {
    use usbk::*;
    c.cycles += 5;
    c.g[S1][0] = 0;
    call_ptr(c, usbkb::POLL, 0x0026_692c)?;

    // The connected bit of each of the 8 ports (s1 = port, s0 = slot).
    c.g[S2][0] = 0xffff_ffff_ffff_fffe;
    c.g[S0][0] = c.g[S3][0];
    loop {
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, usbkb::IS_CONNECTED, 0x0026_6940)?;
        let flags = Addr(r32(c, S0).wrapping_add(SLOT_FLAGS));
        c.g[V1][0] = flags.lw(c);
        c.g[V0][0] = ((c.g[V0][0] ^ 1) < 1) as u64;
        c.g[V1][0] = (c.g[V1][0] & c.g[S2][0]) | c.g[V0][0];
        c.g[V0][0] = c.g[V1][0] & 1;
        flags.sw(c, c.g[V1][0]);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            flags.sw(c, 0);
        }
        c.cycles += 4;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[S1][0] as i64) < 8) as u64;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(SLOT_SIZE));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0026_6938);
        }
    }

    // Each port's new data (s0 = port, s7 = slot size, n at sp+0).
    c.cycles += 4;
    c.g[S0][0] = 0;
    call_ptr(c, usbkb::MAX_PORTS, 0x0026_697c)?;
    let none = (c.g[V0][0] as i64) <= 0;
    Addr(frame.at(c, 0)).sw(c, c.g[V0][0]);
    if none {
        return Ok(());
    }
    c.cycles += 3;
    c.g[S7][0] = SLOT_SIZE as u64;
    c.mult(0, V0, S0, S7);
    loop {
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(this));
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
        let next = Addr(frame.at(c, 4));
        next.sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOT_FLAGS)).lw(c) & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, usbkb::GET_DATA, 0x0026_69b4)?;
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] != 0 {
                usb_keyboard_read_port(c, frame, this)?;
            }
        }
        c.g[S0][0] = next.lw(c);
        c.cycles += 4;
        c.g[V1][0] = Addr(frame.at(c, 0)).lw(c);
        let more = (c.g[S0][0] as i64) < (c.g[V1][0] as i64);
        c.g[V0][0] = more as u64;
        c.mult(0, V0, S0, S7);
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0026_6990);
        }
    }
}

/// One port with data (`s1`, port `s0`): append its keys to the ring and
/// update the port flags.
fn usb_keyboard_read_port(c: &mut Cpu, frame: Frame, this: u32) -> crate::call::Flow {
    use usbk::*;
    let data = r32(c, S1);
    c.cycles += 3;
    c.g[V0][0] = Addr(data.wrapping_add(DATA_COUNT)).lw(c);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[S2][0] = 0;
    if !empty {
        c.cycles += 7;
        c.g[T3][0] = SLOT_SIZE as u64;
        c.g[FP][0] = u64::MAX;
        c.g[S6][0] = sx(this.wrapping_add(SLOT_START));
        c.g[S5][0] = sx(this.wrapping_add(SLOT_RING));
        c.g[S4][0] = sx(this.wrapping_add(SLOT_RING + 1));
        let spill = frame.at(c, 0x10);
        c.sq(T3, spill);
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, usbkb::GET_KEY, 0x0026_69f8)?;
            c.g[T1][0] = c.g[V0][0];
            let key = r32(c, T1);
            c.g[V0][0] = Addr(key).lbu(c);
            let mut store = c.g[V0][0] != 0;
            let spill = frame.at(c, 0x10);
            c.lq(T3, spill);
            if !store {
                c.cycles += 3;
                c.g[V0][0] = Addr(key.wrapping_add(1)).lbu(c);
                store = c.g[V0][0] != 0;
                if !store {
                    c.g[V0][0] = Addr(data.wrapping_add(DATA_COUNT)).lw(c);
                }
            }
            if store {
                // idx = count % 64; a full ring advances start.
                c.cycles += 14;
                c.mult(0, T0, S0, T3);
                let slot = r32(c, T0);
                c.g[T2][0] = sx(this.wrapping_add(slot));
                c.g[A3][0] = sx(r32(c, S6).wrapping_add(slot));
                let count_at = Addr(r32(c, T2));
                let start_at = Addr(r32(c, A3));
                c.g[A0][0] = count_at.lw(c);
                c.g[A2][0] = start_at.lw(c);
                c.g[V1][0] = ((c.g[FP][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f));
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                c.g[A1][0] = ((c.g[A1][0] as i64) < 64) as u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                let room = c.g[A1][0] != 0;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                if !room {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    start_at.sw(c, c.g[V0][0]);
                }
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                c.g[A1][0] = Addr(key).lbu(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(slot));
                c.g[V0][0] = sx(r32(c, S5).wrapping_add(c.g[V1][0] as u32));
                Addr(r32(c, V0)).sb(c, c.g[A1][0]);
                c.g[V1][0] = sx(r32(c, S4).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = Addr(key.wrapping_add(1)).lbu(c);
                Addr(r32(c, V1)).sb(c, c.g[A0][0]);
                c.g[V0][0] = sx((count_at.lw(c) as u32).wrapping_add(1));
                count_at.sw(c, c.g[V0][0]);
                c.g[V0][0] = Addr(data.wrapping_add(DATA_COUNT)).lw(c);
            }
            c.cycles += 4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            let spill = frame.at(c, 0x10);
            c.sq(T3, spill);
            if event_due(c) {
                deliver_event_here(c, 0x0026_69e8);
            }
        }
    }

    // Bit 4: key 0x10 held.
    c.cycles += 7;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0x10;
    call_ptr(c, usbkb::IS_KEY_DOWN, 0x0026_6aa8)?;
    let held = c.g[V0][0] != 0;
    c.mult(0, V0, S0, S7);
    c.cycles += 5;
    if held {
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(this));
        let flags = Addr(r32(c, V1).wrapping_add(SLOT_FLAGS));
        c.g[V0][0] = flags.lw(c) | 0x10;
        flags.sw(c, c.g[V0][0]);
    } else {
        c.g[V1][0] = 0xffff_ffff_ffff_ffef;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(this));
        let flags = Addr(r32(c, A0).wrapping_add(SLOT_FLAGS));
        c.g[V0][0] = flags.lw(c) & c.g[V1][0];
        flags.sw(c, c.g[V0][0]);
    }

    // Bits 2, 3 and 1: the data's "on" bit.
    c.cycles += 25;
    c.mult(0, V0, S0, S7);
    c.g[A1][0] = 0xffff_ffff_ffff_fffb;
    c.g[A2][0] = 0xffff_ffff_ffff_fff7;
    c.g[A3][0] = 0xffff_ffff_ffff_fffd;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(this));
    let flags = Addr(r32(c, A0).wrapping_add(SLOT_FLAGS));
    for (mask, shift) in [(A1, 2u32), (A2, 3), (A3, 1)] {
        c.g[V0][0] = Addr(data).lw(c);
        if shift == 2 {
            c.g[V1][0] = flags.lw(c);
        }
        c.g[V0][0] = sx(((c.g[V0][0] & 1) as u32) << shift);
        c.g[V1][0] = (c.g[V1][0] & c.g[mask][0]) | c.g[V0][0];
        flags.sw(c, c.g[V1][0]);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// cWorldPainterMan_DebugDrawCurrent (0x2bb6e0)

/// `void cWorldPainterMan_DebugDrawCurrent(cWorldPainterMan *this)`
/// (0x2bb6e0): the paint-map debug overlay. When enabled (`gp+0x13c4`),
/// take the current course's paint set (`this->sets[gp+0x770]`, 12 bytes
/// each, at this+4), pick its layer `gp+0x13c8` (clamped to the last; <= 0
/// the first), and if that layer's map has a quadtree, push the template
/// render state (0x501420), clear five 0x30-byte vertex records on the
/// stack, compute the map's bounds (origin, origin + 32768/scale) and draw
/// the tree from its root (`WorldPainter_DebugDrawNode`); pop the state.
pub fn world_painter_man_debug_draw_current(c: &mut Cpu) {
    const DRAW_NODE: Callee = Callee {
        addr: 0x002b_b928,
        f: ee::s08::f_002bb928,
    };
    const STATE_TEMPLATE: u32 = 0x0050_1420;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x130);
        c.g[V0][0] = Addr(gp_at(c, 0x13c4)).lw(c);
        frame.save(c, S0, 0x120);
        c.g[A3][0] = c.g[A0][0];
        let this = r32(c, A3);
        frame.save_ra(c, 0x110);
        'restore_s0: {
            'out: {
                if c.g[V0][0] == 0 {
                    break 'restore_s0;
                }
                // The paint set of the current course.
                c.cycles += 7;
                c.g[V0][0] = Addr(gp_at(c, 0x770)).lw(c);
                c.g[V1][0] = 0xc;
                c.g[A0][0] = Addr(this.wrapping_add(4)).lw(c);
                c.mult(0, V0, V0, V1);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                frame.restore(c, S0, 0x120);
                if c.g[A0][0] == 0 {
                    break 'out;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
                c.g[A1][0] = c.g[V0][0];
                if c.g[A0][0] == 0 {
                    break 'out;
                }
                // The layer: gp+0x13c8 clamped to count-1; <= 0 the first.
                let set = r32(c, A1);
                let layers = r32(c, A0);
                c.cycles += 4;
                c.g[V0][0] = Addr(set.wrapping_add(4)).lw(c);
                c.g[V1][0] = Addr(gp_at(c, 0x13c8)).lw(c);
                let want = c.g[V1][0] as i64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                let last_index = if want > 0 {
                    c.cycles += 3;
                    c.g[V0][0] = (want < c.g[V0][0] as i64) as u64;
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        c.g[V0][0] = Addr(set.wrapping_add(4)).lw(c);
                        true
                    } else {
                        false
                    }
                } else {
                    c.cycles += 2;
                    if (c.g[V0][0] as i64) <= 0 {
                        c.g[V0][0] = Addr(set.wrapping_add(4)).lw(c);
                        true
                    } else {
                        false
                    }
                };
                if last_index {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.cycles += 1;
                    c.g[V0][0] = sx(layers.wrapping_add(c.g[V0][0] as u32));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    if want > 0 {
                        c.cycles += 1;
                        c.g[V0][0] = sx(layers.wrapping_add(c.g[V0][0] as u32));
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[A0][0];
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                frame.restore(c, S0, 0x120);
                if c.g[V0][0] == 0 {
                    break 'out;
                }
                c.cycles += 3;
                c.g[T2][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[S0][0] = Addr(gp_at(c, 0x2a90)).lw(c);
                if c.g[T2][0] == 0 {
                    break 'restore_s0;
                }
                let map = r32(c, T2);
                let rc = r32(c, S0);

                // Push the template render state.
                c.cycles += 31;
                c.g[A0][0] = 0x50_0000;
                c.g[A1][0] = 0x4;
                c.g[T0][0] = sx(frame.at(c, 0xf0));
                let top = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c) as u32;
                c.g[V0][0] = sx(top);
                c.g[T1][0] = sx(frame.at(c, 0x100));
                c.g[A2][0] = u64::MAX;
                copy_render_state(c, top, top.wrapping_add(0x14), V1, T3, T4);
                c.g[V1][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
                let prev = r32(c, V1);
                c.g[V0][0] = sx(prev.wrapping_add(0x14));
                Addr(rc.wrapping_add(rctx::STATE_TOP)).sw(c, c.g[V0][0]);
                c.g[T5][0] = sx(STATE_TEMPLATE);
                copy_render_state(c, STATE_TEMPLATE, prev.wrapping_add(0x14), V0, T3, T4);

                // A delay loop the compiler left from the unrolled clear.
                loop {
                    c.cycles += 7;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                    let more = c.g[A1][0] != c.g[A2][0];
                    c.g[A0][0] = 0xff;
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_b800);
                    }
                }
                // Five vertex records: xyz 0, w 1, colour (0, 0xff, 0, 0xff).
                c.cycles += 5;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 0);
                c.g[V1][0] = c.g[SP][0];
                c.g[A1][0] = sx(frame.at(c, 0xf0));
                Addr(r32(c, V1).wrapping_add(0x1c)).sw(c, c.g[A0][0]);
                loop {
                    c.cycles += 11;
                    let v = r32(c, V1);
                    Addr(v.wrapping_add(0x10)).sw(c, 0);
                    Addr(v.wrapping_add(0x14)).sw(c, c.g[A0][0]);
                    Addr(v.wrapping_add(0x18)).sw(c, 0);
                    c.swc1(0, v.wrapping_add(0x8));
                    Addr(v.wrapping_add(0xc)).sw(c, 0);
                    Addr(v).sw(c, 0);
                    Addr(v.wrapping_add(0x4)).sw(c, 0);
                    c.g[V1][0] = sx(v.wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    Addr(r32(c, V1).wrapping_add(0x1c)).sw(c, c.g[A0][0]);
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_b830);
                    }
                }

                // Bounds: min = (int)origin (sp+0xf0), max = (int)(origin +
                // 32768/scale) (sp+0x100); draw from the root node.
                c.cycles += 47;
                c.lwc1(0, map.wrapping_add(0x4));
                c.g[A0][0] = c.g[A3][0];
                c.lwc1(1, map.wrapping_add(0x8));
                c.g[A1][0] = c.g[T2][0];
                c.cvt_w_s(2, 0);
                c.mfc1(V0, 2);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 4);
                c.cvt_w_s(0, 1);
                c.mfc1(V1, 0);
                let at = |c: &Cpu, off: u32| sp_at(c, off);
                Addr(at(c, 0xf8)).sw(c, 0);
                c.swc1(4, at(c, 0xfc));
                c.g[A3][0] = c.g[SP][0];
                c.mtc1(V0, 1);
                c.cvt_s_w(1, 1);
                c.g[AT][0] = 0x4700_0000;
                c.mtc1(AT, 2);
                c.mtc1(V1, 0);
                c.cvt_s_w(0, 0);
                c.swc1(1, at(c, 0xf0));
                c.swc1(0, at(c, 0xf4));
                c.lwc1(1, map);
                c.lwc1(0, map.wrapping_add(0x8));
                c.div_s(2, 2, 1);
                c.lwc1(3, map.wrapping_add(0x4));
                c.swc1(4, at(c, 0x10c));
                Addr(at(c, 0x108)).sw(c, 0);
                c.add_s(0, 2, 0);
                c.add_s(2, 2, 3);
                c.cvt_w_s(1, 0);
                c.mfc1(V0, 1);
                c.cvt_w_s(0, 2);
                c.mfc1(V1, 0);
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.mtc1(V1, 1);
                c.cvt_s_w(1, 1);
                c.swc1(0, at(c, 0x104));
                c.swc1(1, at(c, 0x100));
                c.g[A2][0] = Addr(map.wrapping_add(0x14)).lhu(c);
                c.g[V0][0] = Addr(map.wrapping_add(0x20)).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                DRAW_NODE.call(c, 0x002b_b90c)?;
                c.g[V0][0] = sx((Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c) as u32).wrapping_sub(0x14));
                Addr(rc.wrapping_add(rctx::STATE_TOP)).sw(c, c.g[V0][0]);
                break 'restore_s0;
            }
            // Out early (s0 already restored).
            c.cycles += 3;
            frame.restore_ra(c, 0x110);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x120);
        c.cycles += 3;
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}

/// The address `sp + off`.
#[inline(always)]
fn sp_at(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// PS2ExecMan_VBlankEndIntc (0x31a490)

/// `int PS2ExecMan_VBlankEndIntc(void *arg, int (*handler)(void *))`
/// (0x31a490): the VBlank-end interrupt trampoline. Saves all 32 FPU
/// registers on the stack (through v0), calls `handler(arg)`, restores
/// them, re-enables interrupts (`ei`) and returns 1.
pub fn ps2_exec_man_vblank_end_intc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 138;
        let frame = Frame::open(c, 0x90);
        frame.save_ra(c, 0x80);
        for f in 0..32usize {
            c.mfc1(V0, f);
            Addr(frame.at(c, 4 * f as u32)).sw(c, c.g[V0][0]);
        }
        let handler = abi::arg(c, 1);
        call_ptr(c, handler, 0x0031_a5a0)?;
        for f in 0..32usize {
            c.g[V0][0] = Addr(frame.at(c, 4 * f as u32)).lw(c);
            c.mtc1(V0, f);
        }
        c.ei();
        frame.restore_ra(c, 0x80);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderSpinSub_GetSpinFactor (0x135180)

/// `.lit4` constants of the spin factor (gp-relative).
mod spin_lit {
    /// 1/7: stick magnitude scale.
    pub const MAG_SCALE: i32 = -0x7380;
    /// atan2(y, 0): the value for y < 0, then the one for y >= 0.
    pub const ATAN_X0_NEG: i32 = -0x737c;
    pub const ATAN_X0_POS: i32 = -0x7378;
    /// atan2 quadrant fix-ups (+pi / -pi).
    pub const PI_ADD: i32 = -0x7374;
    pub const PI_SUB: i32 = -0x7370;
    /// Dead zone 0.3 and range 0.7 of the magnitude.
    pub const DEAD_ZONE: i32 = -0x736c;
    pub const RANGE: i32 = -0x7368;
    /// pi/4 (the diagonal) and 1/0.7.
    pub const DIAGONAL: i32 = -0x7364;
    pub const RANGE_INV: i32 = -0x7360;
}

/// `float RiderSpinSub_GetSpinFactor(RiderSpinSub *this)` (0x135180): how
/// far the rider's stick (input at `this->input` +0x58, x at +0x2dc, y at
/// +0x2e0) is pushed along the diagonal: `clamp(|stick|/7 - 0.3, 0, 0.7) /
/// 0.7 * (1 - |atan2(|y|, |x|) - pi/4| / (pi/4))` (atan2 inlined around
/// `Math_Atan`).
pub fn rider_spin_sub_get_spin_factor(c: &mut Cpu) {
    use spin_lit::*;
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.lwc1(2, gp_at(c, MAG_SCALE));
        frame.save_fpr(c, 23, 0x28);
        frame.save_fpr(c, 22, 0x20);
        frame.save_fpr(c, 21, 0x18);
        frame.save_fpr(c, 20, 0x10);
        frame.save_ra(c, 0);
        c.mtc1(0, 22); // f22 = 0
        c.g[V0][0] = Addr(abi::arg(c, 0).wrapping_add(0x58)).lw(c);
        let input = r32(c, V0);
        c.lwc1(1, input.wrapping_add(0x2e0));
        c.lwc1(0, input.wrapping_add(0x2dc));
        c.abs_s(21, 1); // |y|
        c.abs_s(20, 0); // |x|
        c.mul_s(1, 1, 1);
        c.mul_s(0, 0, 0);
        c.c_eq_s(20, 22);
        c.add_s(0, 0, 1);
        c.sqrt_s(0, 0);
        let x_zero = c.fcc();
        c.mul_s(23, 0, 2); // magnitude / 7

        // f12 = atan2(|y|, |x|).
        if x_zero {
            c.cycles += 4;
            c.c_eq_s(21, 22);
            let y_zero = c.fcc();
            c.mov_s(12, 21);
            if !y_zero {
                c.cycles += 4;
                c.c_le_s(22, 21);
                c.lwc1(12, gp_at(c, ATAN_X0_NEG));
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(12, gp_at(c, ATAN_X0_POS));
                }
            }
        } else {
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, 0x0031_c228, 0x0013_5208)?; // Math_Atan(|y|/|x|)
            c.c_lt_s(20, 22);
            let x_neg = c.fcc();
            c.mov_s(1, 0);
            if x_neg {
                c.cycles += 3;
                c.c_lt_s(22, 21);
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(0, gp_at(c, PI_ADD));
                    c.add_s(1, 1, 0);
                } else {
                    c.cycles += 2;
                    c.lwc1(0, gp_at(c, PI_SUB));
                    c.sub_s(1, 1, 0);
                }
            }
            c.cycles += 1;
            c.mov_s(12, 1);
        }

        // f4 = clamp(mag - 0.3, 0, 0.7).
        c.cycles += 8;
        c.lwc1(0, gp_at(c, DEAD_ZONE));
        c.mtc1(0, 1);
        c.sub_s(0, 23, 0);
        c.lwc1(2, gp_at(c, RANGE));
        c.c_le_s(1, 0);
        let above = c.fcc();
        c.mov_s(4, 1);
        if above {
            c.cycles += 1;
            c.min_s(4, 0, 2);
        }

        // f0 = f4 / 0.7 * (1 - |angle - pi/4| / (pi/4)).
        c.cycles += 17;
        c.lwc1(3, gp_at(c, DIAGONAL));
        c.lwc1(2, gp_at(c, RANGE_INV));
        c.sub_s(1, 12, 3);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.abs_s(1, 1);
        c.mul_s(2, 4, 2);
        c.div_s(1, 1, 3);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x28);
        frame.restore_fpr(c, 22, 0x20);
        frame.restore_fpr(c, 21, 0x18);
        frame.restore_fpr(c, 20, 0x10);
        c.sub_s(0, 0, 1);
        c.mul_s(0, 2, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// sceSifCmdIntrHandler (0x426230)

/// The SIF command state (libkernel `_sif_cmd_data`, 0x52bcd8).
mod sif_cmd {
    pub const DATA: u32 = 0x0052_bcd8;
    /// Pointer to the receive buffer (first byte: packet size).
    pub const RECV: u32 = 0x0;
    /// System command handlers (`{func, data}` pairs) and their count.
    pub const SYS_TABLE: u32 = 0xc;
    pub const SYS_COUNT: u32 = 0x10;
    /// User command handlers and their count.
    pub const USR_TABLE: u32 = 0x14;
    pub const USR_COUNT: u32 = 0x18;
    /// Packet header: the command id (bit 31: system command).
    pub const PKT_CID: u32 = 0x8;
}

/// `int sceSifCmdIntrHandler(void)` (0x426230, libkernel): the SIF DMA
/// interrupt handler. Re-enables interrupts, and if a packet arrived (its
/// size byte non-zero) copies it to the stack (rounded up to quadwords),
/// clears the size byte, re-arms the receive chain (`isceSifSetDChain`),
/// and dispatches the command id to the system (bit 31 set) or user handler
/// table (`func(packet, data)`) when it is in range and set; then `ei`.
/// Returns 0.
pub fn sce_sif_cmd_intr_handler(c: &mut Cpu) {
    use sif_cmd::*;
    const EINTR: Callee = Callee {
        addr: 0x0042_c0c0,
        f: ee::s15::f_0042c0c0,
    };
    const SET_DCHAIN: Callee = Callee {
        addr: 0x0042_4190,
        f: ee::s15::f_00424190,
    };
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        let at = frame.at(c, 0x70);
        c.sd(at, c.g[S0][0]);
        frame.save_ra(c, 0x80);
        EINTR.call(c, 0x0042_6244)?;
        c.g[V1][0] = 0x53_0000;
        c.g[A3][0] = Addr(DATA + RECV).lw(c);
        c.g[S0][0] = sx(DATA);
        let recv = r32(c, A3);
        c.g[V0][0] = Addr(recv).lbu(c);
        c.g[A1][0] = c.g[V0][0] & 0xff;
        let size = c.g[A1][0];
        c.g[V0][0] = 0;
        if size != 0 {
            // Copy the packet (size rounded up to 16) to the stack.
            c.cycles += 10;
            c.g[V0][0] = sx((size as u32).wrapping_add(0xf));
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = sx((size as u32).wrapping_add(0x1e));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.g[V0][0];
            }
            c.g[A2][0] = c.g[A3][0];
            c.g[A1][0] = sx(((c.g[A0][0] as u32) as i32 >> 4) as u32);
            Addr(recv).sb(c, 0);
            let quads = c.g[A1][0] as i64;
            c.g[A0][0] = c.g[A1][0];
            if quads > 0 {
                c.cycles += 2;
                c.g[V1][0] = c.g[SP][0];
                loop {
                    c.cycles += 7;
                    c.lq(V0, r32(c, A2));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.sq(V0, r32(c, V1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    if c.g[A0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0042_6290);
                    }
                }
            }

            c.cycles += 5;
            SET_DCHAIN.call(c, 0x0042_62b4)?;
            let pkt = frame.at(c, 0);
            c.g[V1][0] = Addr(pkt + PKT_CID).lw(c);
            let system = (c.g[V1][0] as i64) < 0;
            // (table, count, return address) of the handler table used.
            let handler = if system {
                c.cycles += 8;
                c.g[V0][0] = Addr(pkt + PKT_CID).lw(c);
                c.g[V1][0] = 0x7fff_ffff;
                c.g[A0][0] = Addr(DATA + SYS_COUNT).lw(c);
                c.g[A1][0] = c.g[V0][0] & c.g[V1][0];
                (SYS_TABLE, 0x0042_6300u32, 5u64)
            } else {
                c.cycles += 5;
                c.g[A1][0] = Addr(pkt + PKT_CID).lw(c);
                c.g[V0][0] = Addr(DATA + USR_COUNT).lw(c);
                (USR_TABLE, 0x0042_633c, 3)
            };
            let (table, ret_to, call_cycles) = handler;
            // The count is in a0 (system) or v0 (user); the compare result
            // replaces it.
            let count_reg = if system { A0 } else { V0 };
            let in_range = (c.g[A1][0] as i64) < (c.g[count_reg][0] as i64);
            c.g[count_reg][0] = in_range as u64;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
            if in_range {
                c.cycles += 5;
                c.g[V1][0] = Addr(DATA + table).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                let entry = r32(c, V0);
                c.g[A2][0] = Addr(entry).lw(c);
                if c.g[A2][0] != 0 {
                    c.cycles += call_cycles;
                    c.g[A1][0] = Addr(entry + 4).lw(c);
                    c.g[A0][0] = c.g[SP][0];
                    let func = r32(c, A2);
                    call_ptr(c, func, ret_to)?;
                }
            }
            c.cycles += 3;
            c.ei();
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0x80);
        let at = frame.at(c, 0x70);
        c.g[S0][0] = c.ld(at);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SNDSTRM_ConsumeCallback (0x3b6b10)

/// Sound-stream request fields (the requests queued on a stream at +0x11c).
mod snd_req {
    /// The stream's active-request list head.
    pub const STREAM_ACTIVE: u32 = 0x11c;
    /// Owner id (stream index in the low byte).
    pub const OWNER: u32 = 0xc;
    /// Consumed position, end position, bytes left.
    pub const POS: u32 = 0x14;
    pub const END: u32 = 0x18;
    pub const LEFT: u32 = 0x1c;
}

/// `void SNDSTRM_ConsumeCallback(?, unsigned n, SndStream *stream)`
/// (0x3b6b10): consume `n` bytes from the stream's active requests: advance
/// the head request (at most its bytes left; the rest carries over to the
/// next head), retiring it (`SNDSTRM_RetireRequest`) once its position
/// reaches its end. After 200 rounds the head is retired unconditionally.
pub fn sndstrm_consume_callback(c: &mut Cpu) {
    use snd_req::*;
    const RETIRE: Callee = Callee {
        addr: 0x003b_6a70,
        f: ee::s13::f_003b6a70,
    };
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0; // the carried-over remainder
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0; // rounds
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        let stream = r32(c, S2);
        loop {
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 201) as u64;
            c.g[A0][0] = Addr(stream.wrapping_add(STREAM_ACTIVE)).lw(c);
            let req = r32(c, A0);
            if c.g[V0][0] == 0 {
                c.cycles += 5;
                c.g[A0][0] = Addr(req.wrapping_add(OWNER)).lw(c);
                RETIRE.call(c, 0x003b_6b4c)?;
                frame.restore(c, S0, 0x30);
                break;
            }
            // Take at most the request's bytes left; keep the rest.
            c.cycles += 4;
            c.g[V0][0] = Addr(req.wrapping_add(LEFT)).lw(c);
            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(req.wrapping_add(LEFT)).lw(c);
                c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
            }
            c.cycles += 11;
            let n = c.g[A1][0] as u32;
            c.g[V0][0] = sx((Addr(req.wrapping_add(POS)).lw(c) as u32).wrapping_add(n));
            Addr(req.wrapping_add(POS)).sw(c, c.g[V0][0]);
            c.g[V1][0] = sx((Addr(req.wrapping_add(LEFT)).lw(c) as u32).wrapping_sub(n));
            Addr(req.wrapping_add(LEFT)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(req.wrapping_add(POS)).lw(c);
            c.g[V1][0] = Addr(req.wrapping_add(END)).lw(c);
            c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[A0][0] = Addr(req.wrapping_add(OWNER)).lw(c);
                RETIRE.call(c, 0x003b_6ba8)?;
            }
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            if c.g[S0][0] == 0 {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
                break;
            }
            c.cycles += 2;
            c.g[S0][0] = 0;
            if event_due(c) {
                deliver_event_here(c, 0x003b_6b30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, elf_image, fill, put_vec, put_words, scramble, Rng, DATA, GAME_GP, SP};

    /// ChaseCameraAlgorithm_UpdateChaseHeading: slow (speed below 100 km/h)
    /// and fast paths; the rig direction short (falls back to the last one)
    /// or not; a zero heading (angle 0) or the asin path with the cosine
    /// clamped low/high; the history index in range; rider states in and
    /// out of 0xb..0xd. The virtual call is a fake that sometimes unwinds;
    /// Math_Asin runs for real on the game's constants (ELF image).
    #[test]
    fn chase_camera_update_chase_heading_matches() {
        covered(ssx3_ee::s01::f_00162c78__generated, chase_camera_update_chase_heading, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x162c78, 1);
            elf_image(c);
            c.pc = 0x0016_2c78;
            let this = DATA;
            let info = DATA + 0x800;
            let rig = DATA + 0x1000;
            let vt = DATA + 0x1100;
            let rider = DATA + 0x1200;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = info as u64;
            fill(c, &mut r, this, 0x100);
            fill(c, &mut r, info, 0x20);
            let v = |r: &mut Rng, big: bool| {
                let k = if big { 3.0 } else { 0.3 };
                [r.f32(k), r.f32(k), r.f32(k), r.f32(k * 0.5)]
            };
            c.bus.write32(this + chase::RIG, rig);
            c.bus.write32(rig, vt);
            c.bus.write32(rig + 4, rider);
            c.bus.write16(vt + 0x18, (r.next_u32() as u16) & 0x7c);
            c.bus.write32(vt + 0x1c, FAKE);
            let state = match seed % 4 {
                0 => 0xb + r.next_u32() % 3,
                _ => r.next_u32() % 0x20,
            };
            c.bus.write32(rider + chase::RIDER_STATE, state);
            let speed = if seed % 7 == 0 { 10.0f32 } else { 30.0 + (r.next_u32() % 40) as f32 };
            c.bus.write32(info + chase_info::SPEED, speed.to_bits());
            let d = v(&mut r, true);
            put_vec(c, info + chase_info::DIR, d);
            let h = if seed % 9 == 1 { [0; 4] } else { v(&mut r, true) };
            put_vec(c, this + chase::HEADING, h);
            let l = v(&mut r, seed % 2 == 0);
            put_vec(c, this + chase::LAST_DIR, l);
            // The rig's direction (the fake leaves sp+0x10 as it is).
            let g = v(&mut r, seed % 3 != 0);
            put_vec(c, SP - 0xb0 + 0x10, g);
            let hist = this + chase::HIST;
            for i in 0..15 {
                c.bus.write32(hist + 4 * i, r.f32(0.5));
            }
            c.bus.write32(hist + chase::H_IDX, r.next_u32() % 5);
            put_words(
                c,
                hist + chase::H_K0,
                &[r.f32(1.0), r.f32(1.0), r.f32(1.0)],
            );
        });
    }

    /// SyncDCache: interrupts enabled or not on entry (Status bit 16, other
    /// Status bits random), random ranges; the real DIntr/_SyncDCache/EIntr.
    #[test]
    fn sync_dcache_matches() {
        check(ee::s15::f_00424698__generated, sync_dcache, 200, &|c, seed| {
            let mut r = Rng(0x4698 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0042_4698;
            let status = r.next_u32() & !0x1_0000;
            c.cop0[12] = if seed % 2 == 0 { status | 0x1_0000 } else { status };
            c.cop0[28] = r.next_u32();
            let start = r.next_u32() % 0x0200_0000;
            let end = start.wrapping_add(r.next_u32() % 0x2000);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | start as u64;
            c.g[A1][0] = sx(end);
        });
    }

    /// SnowFall_Render: rendering off (`gp+0x1434`) or on; fog flag on/off;
    /// the 12 box slots empty, owned by another rider, or by the current
    /// one (WrapOffset and an inactive SnowBox_Render run for real). The
    /// view-matrix virtual is a fake that sometimes unwinds.
    #[test]
    fn snow_fall_render_matches() {
        covered(ee::s09::f_002e6008__generated, snow_fall_render, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x2e6008, 1);
            c.pc = 0x002e_6008;
            let gp = |off: i32| GAME_GP.wrapping_add(off as u32);
            let this = DATA;
            let (world, riders, list) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
            let (rc, vt, stack) = (DATA + 0x5000, DATA + 0x5f00, DATA + 0x6000);
            c.g[A0][0] = sx(this);
            c.bus.write32(gp(0x1434), if seed % 10 == 0 { 1 } else { 0 });
            c.bus.write32(gp(0x1670), r.next_u32() & 1);
            c.bus.write32(gp(-0x3a04), 0.6f32.to_bits());
            c.bus.write32(gp(-0x848), world);
            c.bus.write32(world + 0x84, riders);
            c.bus.write32(riders + 0x84, list);
            let k = r.next_u32() % 4;
            c.bus.write32(list + 0x14, k);
            for i in 0..4 {
                let rider = DATA + 0x4400 + 0x100 * i;
                c.bus.write32(list + 4 + 4 * i, rider);
                fill(c, &mut r, rider, 0x40);
            }
            fill(c, &mut r, this, 0x100);
            c.bus.write32(gp(0x2a90), rc);
            c.bus.write32(rc + rctx::STATE_TOP, stack + 0x14 * (r.next_u32() % 4));
            fill(c, &mut r, stack, 0x40);
            c.bus.write32(rc + rctx::TEX, r.next_u32());
            c.bus.write32(rc + rctx::VPTR, vt);
            c.bus.write16(vt + rctx::GET_VIEW, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + rctx::GET_VIEW + 4, FAKE);
            fill(c, &mut r, 0x0050_1420, 8);
            fill(c, &mut r, 0x004f_f130, 4);
            for i in 0..12 {
                let bx = DATA + 0x7000 + 0x100 * i;
                let slot = match r.next_u32() % 3 {
                    0 => 0,
                    1 => bx,
                    _ => bx,
                };
                c.bus.write32(this + 0x10 + 4 * i, slot);
                fill(c, &mut r, bx, 0x30);
                let owner = if r.next_u32() % 3 == 0 { (k + 1) % 4 } else { k };
                c.bus.write32(bx, owner);
                c.bus.write32(bx + 8, 0); // inactive: SnowBox_Render returns at once
                c.bus.write32(bx + 0xc, 1.0f32.to_bits());
            }
        });
    }

    /// cUsbKeyboard_Update: disabled or enabled; ports connected or not;
    /// 0..8 ports (and a non-positive count); data present or not; 0..5
    /// keys each, empty or not; rings not full / full; ring counts negative
    /// too (the signed `% 64`). The libusbkb callees run for real on the
    /// game's tables (ELF image) with the library state at 0x525f80 (the
    /// poll state machine idle).
    #[test]
    fn usb_keyboard_update_matches() {
        check(ee::s07::f_002668e8__generated, usb_keyboard_update, 300, &|c, seed| {
            let mut r = Rng(0x2668e8 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0026_68e8;
            let this = DATA;
            let lib = 0x0052_5f80u32;
            let conns = DATA + 0x2000;
            c.g[A0][0] = sx(this);
            fill(c, &mut r, this, 0x460 / 4);
            for j in 0..8 {
                let slot = this + j * usbk::SLOT_SIZE;
                let count = (r.next_u32() % 300) as i32 - 80;
                let start = count - (r.next_u32() % 70) as i32;
                c.bus.write32(slot + usbk::SLOT_COUNT, count as u32);
                c.bus.write32(slot + usbk::SLOT_START, start as u32);
            }
            c.bus.write32(this + usbk::ENABLED, (seed % 8 != 0) as u32);
            c.bus.write32(lib + 0x9c, 4);
            for i in 0..8 {
                c.bus.write8(lib + 0x10 + i, (r.next_u32() % 3 != 0) as u8);
            }
            let ports = match seed % 7 {
                0 => 0,
                1 => (-(r.next_u32() as i32 % 3)) as u32,
                _ => 1 + r.next_u32() % 8,
            };
            c.bus.write32(0x0052_6010, ports);
            c.bus.write32(lib + 0xa8, conns);
            for j in 0..8 {
                let d = conns + j * 0x8c;
                fill(c, &mut r, d, 0x23);
                c.bus.write8(d + 0x88, (r.next_u32() % 4 != 0) as u8);
                c.bus.write32(d + usbk::DATA_COUNT, r.next_u32() % 6);
                for k in 0..6 {
                    let code = match r.next_u32() % 4 {
                        0 => 0,
                        1 => 0x10,
                        _ => r.next_u32() % 0x100,
                    };
                    c.bus.write16(d + 0xc + 2 * k, code as u16);
                }
            }
        });
    }

    /// cWorldPainterMan_DebugDrawCurrent: overlay off/on; no paint sets or
    /// a null set; every layer-clamp case (wanted layer <= 0, in range, past
    /// the last; one-layer sets); a null layer or map; a full draw of a leaf
    /// root node (WorldPainter_DebugDrawNode for real, its two virtual calls
    /// faked, sometimes unwinding).
    #[test]
    fn world_painter_man_debug_draw_current_matches() {
        covered(ee::s08::f_002bb6e0__generated, world_painter_man_debug_draw_current, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x2bb6e0, 2);
            c.pc = 0x002b_b6e0;
            let gp = |off: i32| GAME_GP.wrapping_add(off as u32);
            let this = DATA;
            let (sets, set, rc) = (DATA + 0x100, DATA + 0x200, DATA + 0x5000);
            c.g[A0][0] = sx(this);
            c.bus.write32(gp(0x13c4), (seed % 8 != 0) as u32);
            let course = r.next_u32() % 3;
            c.bus.write32(gp(0x770), course);
            c.bus.write32(this + 4, if seed % 13 == 5 { (course * 12).wrapping_neg() } else { sets });
            let set_ptr = if seed % 17 == 3 { 0xffff_fff8 } else { set };
            c.bus.write32(sets + course * 12 + 8, set_ptr);
            let n = 1 + r.next_u32() % 5;
            c.bus.write32(set + 4, n);
            c.bus.write32(gp(0x13c8), (r.next_u32() % 9).wrapping_sub(2));
            for i in 0..5 {
                let layer = DATA + 0x400 + 0x40 * i;
                let map = DATA + 0x800 + 0x40 * i;
                let nodes = DATA + 0xc00;
                c.bus.write32(set + 8 + 4 * i, if r.next_u32() % 8 == 0 { 0 } else { layer });
                c.bus.write32(layer, if r.next_u32() % 8 == 0 { 0 } else { map });
                let scale = 0.5 + (r.next_u32() % 8) as f32 * 0.5;
                c.bus.write32(map, scale.to_bits());
                c.bus.write32(map + 4, r.f32(2000.0));
                c.bus.write32(map + 8, r.f32(2000.0));
                c.bus.write16(map + 0x14, (r.next_u32() % 4) as u16);
                c.bus.write32(map + 0x20, nodes);
                for k in 0..4 {
                    c.bus.write16(nodes + 8 * k, (r.next_u32() as u16) & !1); // leaves
                }
            }
            // DebugDrawNode's leaf: the world chain to a vtable (+0x28) and
            // the renderer's vtable (+0x248).
            let w = DATA + 0x3000;
            c.bus.write32(gp(-0x848), w);
            c.bus.write32(w + 0x84, w + 0x100);
            c.bus.write32(w + 0x100 + 0xc, w + 0x200);
            c.bus.write32(w + 0x200 + 0x40, w + 0x300);
            c.bus.write32(w + 0x300 + 0x18, w + 0x400);
            c.bus.write32(w + 0x400 + 0x6c0, w + 0x1000);
            c.bus.write16(w + 0x1000 + 0x28, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(w + 0x1000 + 0x2c, FAKE);
            c.bus.write32(gp(0x2a90), rc);
            c.bus.write32(rc + rctx::STATE_TOP, DATA + 0x6000);
            fill(c, &mut r, DATA + 0x6000, 0x10);
            fill(c, &mut r, 0x0050_1420, 8);
            c.bus.write32(rc + rctx::VPTR, rc + 0x1100);
            c.bus.write16(rc + 0x1100 + 0x248, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(rc + 0x1100 + 0x24c, FAKE);
        });
    }

    /// PS2ExecMan_VBlankEndIntc: random FPU registers and status; the
    /// handler is a fake that clobbers f0 and sometimes unwinds.
    #[test]
    fn ps2_exec_man_vblank_end_intc_matches() {
        covered(ee::s10::f_0031a490__generated, ps2_exec_man_vblank_end_intc, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x31a490, 1);
            c.pc = 0x0031_a490;
            c.cop0[12] = r.next_u32();
            c.g[A1][0] = FAKE as u64;
        });
    }

    /// RiderSpinSub_GetSpinFactor: stick x zero (y zero / positive /
    /// negative) or not (x negative with y positive / not, x positive);
    /// magnitudes below, inside and above the dead-zone range. Math_Atan
    /// runs for real on the game's constants (ELF image).
    #[test]
    fn rider_spin_sub_get_spin_factor_matches() {
        check(ee::s01::f_00135180__generated, rider_spin_sub_get_spin_factor, 300, &|c, seed| {
            let mut r = Rng(0x135180 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0013_5180;
            let (this, input) = (DATA, DATA + 0x100);
            c.g[A0][0] = sx(this);
            c.bus.write32(this + 0x58, input);
            let pick = |r: &mut Rng, k: u32| match k {
                0 => 0,
                1 => 0x8000_0000,
                2 => r.edge_f32(),
                _ => r.f32(8.0),
            };
            let x = pick(&mut r, seed % 5);
            let y = pick(&mut r, (seed / 5) % 5);
            c.bus.write32(input + 0x2dc, x);
            c.bus.write32(input + 0x2e0, y);
        });
    }

    /// sceSifCmdIntrHandler: no packet (size 0) or a packet of 1..0x70
    /// bytes; system or user command ids in range or not, handlers set or
    /// null. The SetDChain syscall and the handler are fakes (sometimes
    /// unwinding); EIntr runs for real.
    #[test]
    fn sce_sif_cmd_intr_handler_matches() {
        covered(ee::s15::f_00426230__generated, sce_sif_cmd_intr_handler, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x426230, 2);
            c.pc = 0x0042_6230;
            c.cop0[12] = r.next_u32();
            let recv = DATA;
            let (sys, usr) = (DATA + 0x200, DATA + 0x300);
            fill(c, &mut r, recv, 0x20);
            let size = if seed % 6 == 0 { 0 } else { 1 + r.next_u32() % 0x70 };
            c.bus.write8(recv, size as u8);
            let n = r.next_u32() % 8;
            let cid = match r.next_u32() % 4 {
                0 => 0x8000_0000 | n,
                1 => n,
                2 => 0x8000_0000 | (5 + r.next_u32() % 8),
                _ => 5 + r.next_u32() % 8,
            };
            c.bus.write32(recv + sif_cmd::PKT_CID, cid);
            put_words(c, sif_cmd::DATA, &[recv]);
            c.bus.write32(sif_cmd::DATA + sif_cmd::SYS_TABLE, sys);
            c.bus.write32(sif_cmd::DATA + sif_cmd::SYS_COUNT, 6);
            c.bus.write32(sif_cmd::DATA + sif_cmd::USR_TABLE, usr);
            c.bus.write32(sif_cmd::DATA + sif_cmd::USR_COUNT, 6);
            for k in 0..8 {
                let f = if r.next_u32() % 5 == 0 { 0 } else { FAKE + 0x10 * k };
                c.bus.write32(sys + 8 * k, f);
                c.bus.write32(sys + 8 * k + 4, r.next_u32());
                let f = if r.next_u32() % 5 == 0 { 0 } else { FAKE + 0x10 * k };
                c.bus.write32(usr + 8 * k, f);
                c.bus.write32(usr + 8 * k + 4, r.next_u32());
            }
        });
    }

    /// SNDSTRM_ConsumeCallback: 1..3 queued requests of the stream, `n`
    /// smaller or larger than the head's bytes left (carrying over to the
    /// next request), positions reaching their end or not (the real
    /// SNDSTRM_RetireRequest moves the head to the free list); sometimes a
    /// 200-round cap is hit (a zero-left request chain with a huge `n`).
    #[test]
    fn sndstrm_consume_callback_matches() {
        check(ee::s13::f_003b6b10__generated, sndstrm_consume_callback, 300, &|c, seed| {
            let mut r = Rng(0x3b6b10 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x003b_6b10;
            let id = r.next_u32() % 8;
            let stream = DATA;
            c.bus.write8(0x0050_a92e, 8); // stream count
            c.bus.write32(0x0050_a0a0 + 4 * id, stream);
            fill(c, &mut r, stream + 0x11c, 7);
            c.g[A2][0] = sx(stream);
            let nreq = 1 + r.next_u32() % 3;
            let looping = seed % 10 == 0;
            let mut prev = 0u32;
            for k in (0..nreq).rev() {
                let q = DATA + 0x400 + 0x40 * k;
                fill(c, &mut r, q, 0x10);
                // A looping list (the last points back to the first).
                let next = if k == nreq - 1 && looping { DATA + 0x400 } else { prev };
                c.bus.write32(q, next);
                c.bus.write32(q + 4, 0);
                c.bus.write32(q + snd_req::OWNER, if looping { 0x8000_0000 | id } else { id });
                let pos = r.next_u32() % 0x1000;
                c.bus.write32(q + snd_req::POS, pos);
                c.bus.write32(q + snd_req::END, pos + r.next_u32() % 0x800);
                c.bus.write32(q + snd_req::LEFT, if looping { 0 } else { r.next_u32() % 0x400 });
                prev = q;
            }
            c.bus.write32(stream + snd_req::STREAM_ACTIVE, DATA + 0x400);
            c.bus.write32(stream + 0x120, prev);
            c.bus.write32(stream + 0x124, nreq);
            c.bus.write32(stream + 0x128, 0);
            c.bus.write32(stream + 0x12c, 0);
            c.bus.write32(stream + 0x130, 0);
            c.g[A1][0] = sx(match seed % 3 {
                0 => r.next_u32() % 0x100,
                1 => r.next_u32() % 0x800,
                _ => r.next_u32(),
            });
        });
    }
}

