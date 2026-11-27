//! Staging module oc969 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc969`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001e_64c8,
        ovscreen_freestyle_standings_on_activate,
        "OVScreenFreestyleStandings_OnActivate",
    ),
    (
        0x001e_7928,
        ovscreen_final_standings_update,
        "OVScreenFinalStandings_Update",
    ),
    (
        0x001e_8e70,
        ov_peak_challenge_results_update,
        "OVScreenPeakChallengeResults_Update",
    ),
    (
        0x001e_8fc0,
        ov_multiplayer_standings_empty_hook,
        "OVScreenMultiplayerStandings_EmptyHook",
    ),
    (0x001f_1338, hud_draw_rect, "HUD_DrawRect"),
    (
        0x001f_4108,
        cfe_state_lodge_rider_detail_on_child_event,
        "cFEStateLodgeRiderDetail_OnChildEvent",
    ),
    (
        0x001f_4218,
        cfe_state_lodge_rider_detail_on_widget_created,
        "cFEStateLodgeRiderDetail_OnWidgetCreated",
    ),
    (
        0x001f_4728,
        cfe_state_buy_attrib_ctor,
        "cFEStateBuyAttrib_ctor",
    ),
];

/// `o->setVisible(0)`-style vtable call on the object in a2: the generated
/// `cycles += 5` sequence. Slot `+0x4c` of the vtable at `o + 8`, `this` is
/// `o + delta` (`+0x48`).
fn oc969_obj_vcall(c: &mut Cpu, ret_addr: u32) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_addr)
}

/// `sw reg, addr` (the low word of a GPR).
fn oc969_sw_reg(c: &mut Cpu, addr: u32, reg: usize) {
    let v = c.g[reg][0];
    Addr(addr).sw(c, v);
}

/// `lwc1 f, addr`.
fn oc969_lwc1(c: &mut Cpu, f: usize, addr: u32) {
    c.f[f] = Addr(addr).read_u32(c);
}

/// `mul.s d, s, t`.
fn oc969_fmul(c: &mut Cpu, d: usize, s: usize, t: usize) {
    c.f[d] = (FpuF32(c.f[s]) * FpuF32(c.f[t])).0;
}

/// `add.s d, s, t`.
fn oc969_fadd(c: &mut Cpu, d: usize, s: usize, t: usize) {
    c.f[d] = (FpuF32(c.f[s]) + FpuF32(c.f[t])).0;
}

/// `cvt.w.s d, s` (truncate toward zero, saturating).
fn oc969_cvt_w(c: &mut Cpu, d: usize, s: usize) {
    c.f[d] = FpuF32(c.f[s]).to_i32() as u32;
}

/// One name test of `cFEStateLodgeRiderDetail_OnWidgetCreated`: the name hash
/// of `0x470000 - off` (a call to `0x317670`, returning to `ret_addr`) is
/// compared with the state's `+0x38` word. On a match `code` is stored at
/// `+0x18` (3 cycles) and true is returned.
fn oc969_widget_name_test(
    c: &mut Cpu,
    off: u32,
    ret_addr: u32,
    code: u64,
) -> Result<bool, crate::call::Unwound> {
    c.g[A0][0] = 0x47_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(off));
    call_ptr(c, 0x0031_7670, ret_addr)?;
    let matched = c.g[S0][0] == c.g[V0][0];
    c.g[A0][0] = 0x47_0000;
    if matched {
        c.cycles += 3;
        c.g[V0][0] = code;
        let store_at = (c.g[S1][0] as u32).wrapping_add(0x18);
        oc969_sw_reg(c, store_at, V0);
    }
    Ok(matched)
}

/// `cOVState_HEAT_STANDINGS_onGainFocus` (0x001e64c8): the freestyle standings
/// overlay's focus hook. Runs the base focus handler, then pulses the rider's
/// heat rank (clamped to 6) and the run count as two text pulses, and plays
/// menu sound 0xe.
pub fn ovscreen_freestyle_standings_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 60;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e4c0, 0x001e_64e8)?;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc0)).lw(c);
        call_ptr(c, 0x0014_dc80, 0x001e_64f4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014_dd58, 0x001e_6500)?;
        c.g[S3][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_6e98, 0x001e_6510)?;
        c.g[V1][0] = 0x53_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6730));
        c.g[A2][0] = 6;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = 0x47_0000;
        c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1be8));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 7) as u64;
        if c.g[V0][0] == 0 {
            c.g[S0][0] = c.g[A2][0];
        }
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x0041_7828, 0x001e_6548)?;
        c.g[S1][0] = 0x47_0000;
        c.g[A1][0] = c.g[SP][0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1b18));
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a6f0, 0x001e_6560)?;
        c.g[A1][0] = 0x47_0000;
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1b00));
        call_ptr(c, 0x0041_7828, 0x001e_6578)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0020_a6f0, 0x001e_6588)?;
        call_ptr(c, 0x0028_b180, 0x001e_6590)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x0029_4f78, 0x001e_659c)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_FINAL_STANDINGS_onUpdate` (0x001e7928): the final standings
/// overlay's per-frame update. Runs the base update; while the standings
/// block at `*(gp-0x204)` is live it hides two stat widgets, every 30th
/// frame of the race clock it sets the caption, and it marks the block done
/// (`+0x64 = -1`, `D_004A2700 = 8`, `D_004A26FC = 1`) once.
pub fn ovscreen_final_standings_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0x00);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0020_e900, 0x001e_793c)?;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
        'b_1e7a9c: {
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
                break 'b_1e7a9c;
            }
            'b_1e79d4: {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                let t = c.g[V0][0] == 0;
                c.g[V1][0] = 0x1e;
                if t {
                    break 'b_1e79d4;
                }
                'b_1e79d0: {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0020_a4e0, 0x001e_795c)?;
                    let t = c.g[V0][0] == 0;
                    c.g[A0][0] = 0x47_0000;
                    if t {
                        break 'b_1e79d0;
                    }
                    'b_1e7998: {
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x19a0));
                        call_ptr(c, 0x0031_7670, 0x001e_796c)?;
                        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_d860, 0x001e_7978)?;
                        c.g[A2][0] = c.g[V0][0];
                        let t = c.g[A2][0] == 0;
                        c.g[A1][0] = 0;
                        if t {
                            break 'b_1e7998;
                        }
                        oc969_obj_vcall(c, 0x001e_7998)?;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = 0x47_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1808));
                    call_ptr(c, 0x0031_7670, 0x001e_79a4)?;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001e_79b0)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0;
                    c.g[A1][0] = 0;
                    if t {
                        break 'b_1e79d0;
                    }
                    oc969_obj_vcall(c, 0x001e_79d0)?;
                }
                c.cycles += 1;
                c.g[V1][0] = 0x1e;
            }
            {
                c.cycles += 3;
                c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
            }
            'b_1e7a78: {
                c.cycles += 5;
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x18)).lw(c);
                let n = r32(c, V0);
                let d = r32(c, V1);
                let _ = crate::abi::div(c, n, d);
                c.g[A0][0] = c.hi[0];
                let t = c.g[A0][0] != 0;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                if t {
                    break 'b_1e7a78;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                let t = c.g[V1][0] != 0;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
                if t {
                    break 'b_1e7a78;
                }
                'b_1e7a3c: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x47_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1838));
                    call_ptr(c, 0x0031_7670, 0x001e_7a10)?;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001e_7a1c)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0;
                    c.g[A1][0] = 0;
                    if t {
                        break 'b_1e7a3c;
                    }
                    oc969_obj_vcall(c, 0x001e_7a3c)?;
                }
                'b_1e7a74: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x47_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1828));
                    call_ptr(c, 0x0031_7670, 0x001e_7a48)?;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001e_7a54)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0;
                    c.g[A0][0] = 0x47_0000;
                    if t {
                        break 'b_1e7a74;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1818));
                    call_ptr(c, 0x0031_7670, 0x001e_7a68)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a_1000, 0x001e_7a74)?;
                }
                c.cycles += 1;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
            }
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x64)).lw(c);
            let t = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x10);
            if t {
                break 'b_1e7a9c;
            }
            c.cycles += 6;
            c.g[V0][0] = u64::MAX;
            c.g[A0][0] = 8;
            let done_addr = (c.g[V1][0] as u32).wrapping_add(0x64);
            oc969_sw_reg(c, done_addr, V0);
            c.g[V1][0] = 1;
            let gp_a0 = (c.g[GP][0] as u32).wrapping_sub(0x9f0);
            oc969_sw_reg(c, gp_a0, A0);
            let gp_v1 = (c.g[GP][0] as u32).wrapping_sub(0x9f4);
            oc969_sw_reg(c, gp_v1, V1);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x00);
        frame.close(c);
        Ok(())
    })
}

/// `func_001E8E70` (0x001e8e70), `OVScreenPeakChallengeResults_Update`: the
/// results overlay's update is the base overlay update (`0x20e900`).
pub fn ov_peak_challenge_results_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0x00);
        call_ptr(c, 0x0020_e900, 0x001e_8e80)?;
        frame.restore_ra(c, 0x00);
        frame.close(c);
        Ok(())
    })
}

/// `func_001E8FC0` (0x001e8fc0), `OVScreenMultiplayerStandings_EmptyHook`: a
/// leaf that does nothing and returns.
pub fn ov_multiplayer_standings_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `HUD_DrawRect` (0x001f1338): one screen-space quad through the HUD vertex
/// callback. `color` (t0: RGBA floats), `pos` (a1), `scale` (a2) and `size`
/// (a3) are float pairs; the quad runs from `pos` to `pos + size * scale`.
/// Four 0x30-byte vertex records sit in the stack frame (position at `+0x20`,
/// colour words at `+0x10..+0x1c`, q at `+0x8`); the world's draw slot
/// (`this` at `+0x238`, function at `+0x23c`) takes them with the count 4.
pub fn hud_draw_rect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0xe0);
        c.g[T3][0] = c.g[A1][0];
        frame.save_ra(c, 0xd0);
        c.g[T2][0] = c.g[A2][0];
        c.g[V0][0] = 3;
        c.g[V1][0] = u64::MAX;
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[T1][0] = sx(frame.at(c, 0xc0));
            if !t {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001f_1350);
            }
        }
        c.cycles += 25;
        let color = c.g[T0][0] as u32;
        oc969_lwc1(c, 2, color);
        c.g[V1][0] = c.g[SP][0];
        c.g[AT][0] = 0x4300_0000;
        c.f[5] = c.g[AT][0] as u32;
        c.g[AT][0] = 0x437f_0000;
        c.f[3] = c.g[AT][0] as u32;
        oc969_lwc1(c, 0, color.wrapping_add(0x4));
        oc969_fmul(c, 2, 2, 5);
        oc969_lwc1(c, 4, color.wrapping_add(0xc));
        oc969_lwc1(c, 1, color.wrapping_add(0x8));
        oc969_fmul(c, 0, 0, 3);
        oc969_fmul(c, 4, 4, 3);
        c.g[AT][0] = 0x3f80_0000;
        c.f[5] = c.g[AT][0] as u32;
        oc969_fmul(c, 1, 1, 3);
        oc969_cvt_w(c, 3, 2);
        c.g[A2][0] = sx(c.f[3]);
        oc969_cvt_w(c, 2, 0);
        c.g[A1][0] = sx(c.f[2]);
        oc969_cvt_w(c, 0, 1);
        c.g[T0][0] = sx(c.f[0]);
        oc969_cvt_w(c, 0, 4);
        c.g[A0][0] = sx(c.f[0]);
        let first_a = (c.g[V1][0] as u32).wrapping_add(0x1c);
        oc969_sw_reg(c, first_a, A2);
        loop {
            c.cycles += 8;
            let base = c.g[V1][0] as u32;
            oc969_sw_reg(c, base.wrapping_add(0x10), A1);
            oc969_sw_reg(c, base.wrapping_add(0x14), T0);
            oc969_sw_reg(c, base.wrapping_add(0x18), A0);
            let f5 = c.f[5];
            Addr(base.wrapping_add(0x8)).write_u32(c, f5);
            c.g[V1][0] = sx(base.wrapping_add(0x30));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T1][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            let next_a = (c.g[V1][0] as u32).wrapping_add(0x1c);
            oc969_sw_reg(c, next_a, A2);
            if event_due(c) {
                deliver_event_here(c, 0x001f_13d0);
            }
        }
        c.cycles += 47;
        let size = c.g[A3][0] as u32;
        let scale = c.g[T2][0] as u32;
        let pos = c.g[T3][0] as u32;
        oc969_lwc1(c, 1, size);
        c.g[T1][0] = u64::MAX;
        oc969_lwc1(c, 4, scale);
        c.g[A1][0] = 4;
        oc969_lwc1(c, 2, pos.wrapping_add(0x4));
        c.g[A2][0] = c.g[SP][0];
        oc969_fmul(c, 4, 4, 1);
        oc969_lwc1(c, 0, scale.wrapping_add(0x4));
        oc969_lwc1(c, 1, size.wrapping_add(0x4));
        oc969_lwc1(c, 3, pos);
        c.g[A3][0] = 0;
        c.g[AT][0] = 0x3f80_0000;
        c.f[5] = c.g[AT][0] as u32;
        oc969_fmul(c, 0, 0, 1);
        frame.save_fpr(c, 3, 0xc0);
        oc969_fadd(c, 4, 3, 4);
        Addr(frame.at(c, 0xc8)).sw(c, 0);
        frame.save_fpr(c, 2, 0xc4);
        oc969_fadd(c, 0, 2, 0);
        frame.save_fpr(c, 5, 0xcc);
        c.g[T0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
        frame.restore(c, V0, 0xc0);
        frame.save_fpr(c, 4, 0xc0);
        Addr(frame.at(c, 0xc8)).sw(c, 0);
        frame.save(c, V0, 0x20);
        frame.restore(c, V1, 0xc0);
        frame.save_fpr(c, 3, 0xc0);
        Addr(frame.at(c, 0xc8)).sw(c, 0);
        frame.save_fpr(c, 0, 0xc4);
        frame.save(c, V1, 0x50);
        frame.restore(c, A0, 0xc0);
        frame.save_fpr(c, 4, 0xc0);
        Addr(frame.at(c, 0xc8)).sw(c, 0);
        frame.save(c, A0, 0x80);
        frame.restore(c, V0, 0xc0);
        frame.save(c, V0, 0xb0);
        c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(0xe84)).lw(c);
        let short_v = c.g[T1][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x10)).sh(c, short_v);
        c.g[T0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
        c.g[V0][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x10d8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x238)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x23c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001f_14a0)?;
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateLodgeRiderDetail_OnChildEvent` (0x001f4108): the rider-detail
/// popup's child-event handler. On the popup-close event (a2 == 0x16) it
/// reads the popup's result (a name or a flag through the interface) and
/// forwards it to the owning state's virtual method at `+0x4c`.
pub fn cfe_state_lodge_rider_detail_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x00);
        c.g[S3][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x001f_4130)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014_dd58, 0x001f_413c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = 0x16;
        let t = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x40);
        'e0: {
            if t {
                break 'e0;
            }
            'e1: {
                'e2: {
                    c.cycles += 6;
                    c.g[A0][0] = 0x47_0000;
                    c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10c0));
                    call_ptr(c, 0x0031_7670, 0x001f_415c)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
                        break 'e2;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x74)).lw(c);
                    let t = c.g[V0][0] == 0;
                    c.g[S1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x70)).lw(c);
                    if t {
                        break 'e1;
                    }
                    c.cycles += 17;
                    call_ptr(c, 0x0014_dc80, 0x001f_4178)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014_dd58, 0x001f_4184)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x44)).lb(c);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x0014_74a8, 0x001f_4198)?;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001f_41ac)?;
                    frame.restore(c, S0, 0x40);
                    break 'e0;
                }
                c.cycles += 2;
                let t = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x40);
                if t {
                    break 'e0;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x5c)).lw(c);
                let t = c.g[V0][0] == 0;
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                if t {
                    break 'e1;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0041_6810, 0x001f_41d0)?;
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x40);
                    break 'e0;
                }
                c.cycles += 9;
                c.g[A1][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x44)).lb(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x0014_7138, 0x001f_41e8)?;
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001f_41fc)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0x00);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateLodgeRiderDetail_OnWidgetCreated` (0x001f4218): picks the
/// widget's code from its name hash. The state's name hash (`+0x38`) is
/// compared with the hashes of seven fixed names; the first match stores its
/// code at `+0x18`. With no match among the first seven, the eighth name
/// selects the rider's stat block and fills the stat fields from the
/// interface.
pub fn cfe_state_lodge_rider_detail_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0x00);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x38)).lw(c);
        'e0: {
            'e1: {
                if oc969_widget_name_test(c, 0x0000_10b0, 0x001f_423c, 0x9)? {
                    break 'e1;
                }
                c.cycles += 4;
                if oc969_widget_name_test(c, 0x0000_10a0, 0x001f_4258, 0xa)? {
                    break 'e1;
                }
                c.cycles += 4;
                if oc969_widget_name_test(c, 0x0000_1090, 0x001f_4274, 0xb)? {
                    break 'e1;
                }
                c.cycles += 4;
                if oc969_widget_name_test(c, 0x0000_1080, 0x001f_4290, 0xc)? {
                    break 'e1;
                }
                c.cycles += 4;
                if oc969_widget_name_test(c, 0x0000_1070, 0x001f_42ac, 0xd)? {
                    break 'e1;
                }
                c.cycles += 4;
                if oc969_widget_name_test(c, 0x0000_1060, 0x001f_42c8, 0xe)? {
                    break 'e1;
                }
                c.cycles += 4;
                if oc969_widget_name_test(c, 0x0000_1050, 0x001f_42e4, 0xf)? {
                    break 'e1;
                }
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1040));
                call_ptr(c, 0x0031_7670, 0x001f_4300)?;
                if c.g[S0][0] != c.g[V0][0] {
                    frame.restore(c, S0, 0x20);
                    break 'e0;
                }
                c.cycles += 25;
                call_ptr(c, 0x0014_dc80, 0x001f_4310)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014_dd58, 0x001f_431c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0014_6e98, 0x001f_432c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0014_7398, 0x001f_4338)?;
                c.g[S0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
                call_ptr(c, 0x0014_dc80, 0x001f_4344)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0014_dd58, 0x001f_4350)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0;
                call_ptr(c, 0x0014_eec8, 0x001f_4360)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x001f_436c)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0x00);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateBuyAttrib_ctor` (0x001f4728): the buy-attribute state's
/// constructor. Sets the state's header and widget fields, then fills seven
/// per-stat experience pairs from the rider's stat profile (the stat value,
/// and that value divided by 5 as a signed byte).
pub fn cfe_state_buy_attrib_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x68));
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        c.g[S4][0] = 6;
        frame.save_ra(c, 0x00);
        c.g[S5][0] = 5;
        call_ptr(c, 0x0039_e2a0, 0x001f_475c)?;
        c.g[V0][0] = 0x47_0000;
        c.g[V1][0] = 0x29;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3908));
        let this_a = (c.g[S2][0] as u32).wrapping_add(0xc);
        let v1_word = c.g[V1][0];
        Addr(this_a).sw(c, v1_word);
        let this_b = (c.g[S2][0] as u32).wrapping_add(0x8);
        let v0_word = c.g[V0][0];
        Addr(this_b).sw(c, v0_word);
        c.g[A1][0] = 0;
        Addr((c.g[S2][0] as u32).wrapping_add(0x44)).sb(c, 0);
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        call_ptr(c, 0x001a_1cd0, 0x001f_4784)?;
        let flag_b = (c.g[S2][0] as u32).wrapping_add(0x15);
        let flag_v0 = c.g[V0][0];
        Addr(flag_b).sb(c, flag_v0);
        call_ptr(c, 0x0014_dc80, 0x001f_478c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x3;
        call_ptr(c, 0x0014_dd58, 0x001f_4798)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x84));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x1c;
        call_ptr(c, 0x003e_6448, 0x001f_47ac)?;
        c.g[V0][0] = 0x48_0000;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f50));
        loop {
            c.cycles += 14;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0;
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x0014_8158, 0x001f_47cc)?;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A0][0] = c.g[S3][0];
            let stat_at = (c.g[S1][0] as u32).wrapping_sub(0x1c);
            oc969_sw_reg(c, stat_at, V0);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_8158, 0x001f_47e0)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            let n = r32(c, V0);
            let d = r32(c, S5);
            let _ = crate::abi::div(c, n, d);
            c.cycles += 6;
            c.g[V0][0] = c.lo[0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            let level_at = c.g[S1][0] as u32;
            oc969_sw_reg(c, level_at, V0);
            let t = (c.g[S4][0] as i64) >= 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if !t {
                c.cycles += 10;
                c.g[V0][0] = c.g[S2][0];
                frame.restore(c, S0, 0x60);
                frame.restore(c, S1, 0x50);
                frame.restore(c, S2, 0x40);
                frame.restore(c, S3, 0x30);
                frame.restore(c, S4, 0x20);
                frame.restore(c, S5, 0x10);
                frame.restore_ra(c, 0x00);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x001f_47b8);
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes: only this module's test
    /// machines set it, so every other test falls through to the body.
    const OC969_GATE: u32 = DATA + 0xf_6900;
    const OC969_MAGIC: u32 = 0x0969_6a7e;
    /// Mailbox words: word i (when non-zero) is the result of callee fake i;
    /// `OC969_NULL` makes the result 0.
    const OC969_MBX: u32 = DATA + 0xf_6800;
    const OC969_NULL: u32 = 1;

    /// A callee fake: logs the call (`fakes::fake`, as the machine's miss hook
    /// does), then returns the mailbox word `$mbx` when the test set one. Gate
    /// off, it runs the generated body it replaced.
    macro_rules! callee_fake {
        ($fake:ident, $gen:path, $addr:expr, $mbx:expr) => {
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC969_GATE) != OC969_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
                let m = c.bus.read32(OC969_MBX + 4 * $mbx);
                if m != 0 {
                    c.g[V0][0] = if m == OC969_NULL { 0 } else { m as u64 };
                }
            }
        };
    }

    callee_fake!(
        oc969_fake_0039e4c0,
        ee::s13::f_0039e4c0__generated,
        0x0039_e4c0,
        0
    );
    callee_fake!(
        oc969_fake_0014dc80,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80,
        0
    );
    callee_fake!(
        oc969_fake_0014dd58,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58,
        1
    );
    callee_fake!(
        oc969_fake_00146e98,
        ee::s01::f_00146e98__generated,
        0x0014_6e98,
        2
    );
    callee_fake!(
        oc969_fake_00417828,
        ee::s15::f_00417828__generated,
        0x0041_7828,
        0
    );
    callee_fake!(
        oc969_fake_0020a6f0,
        ee::s05::f_0020a6f0__generated,
        0x0020_a6f0,
        0
    );
    callee_fake!(
        oc969_fake_0028b180,
        ee::s07::f_0028b180__generated,
        0x0028_b180,
        0
    );
    callee_fake!(
        oc969_fake_00294f78,
        ee::s08::f_00294f78__generated,
        0x0029_4f78,
        0
    );
    callee_fake!(
        oc969_fake_0020e900,
        ee::s05::f_0020e900__generated,
        0x0020_e900,
        0
    );
    callee_fake!(
        oc969_fake_00317670,
        ee::s10::f_00317670__generated,
        0x0031_7670,
        3
    );
    callee_fake!(
        oc969_fake_0039d860,
        ee::s13::f_0039d860__generated,
        0x0039_d860,
        4
    );
    callee_fake!(
        oc969_fake_003a1000,
        ee::s13::f_003a1000__generated,
        0x003a_1000,
        0
    );
    callee_fake!(
        oc969_fake_001474a8,
        ee::s01::f_001474a8__generated,
        0x0014_74a8,
        0
    );
    callee_fake!(
        oc969_fake_00416810,
        ee::s15::f_00416810__generated,
        0x0041_6810,
        5
    );
    callee_fake!(
        oc969_fake_00147138,
        ee::s01::f_00147138__generated,
        0x0014_7138,
        0
    );
    callee_fake!(
        oc969_fake_0014eec8,
        ee::s01::f_0014eec8__generated,
        0x0014_eec8,
        0
    );
    callee_fake!(
        oc969_fake_00147398,
        ee::s01::f_00147398__generated,
        0x0014_7398,
        0
    );
    callee_fake!(
        oc969_fake_003a0c50,
        ee::s13::f_003a0c50__generated,
        0x003a_0c50,
        0
    );
    callee_fake!(
        oc969_fake_003e6448,
        ee::s14::f_003e6448__generated,
        0x003e_6448,
        0
    );
    callee_fake!(
        oc969_fake_001a1cd0,
        ee::s03::f_001a1cd0__generated,
        0x001a_1cd0,
        0
    );
    callee_fake!(
        oc969_fake_00148158,
        ee::s01::f_00148158__generated,
        0x0014_8158,
        0
    );
    callee_fake!(
        oc969_fake_0039e2a0,
        ee::s13::f_0039e2a0__generated,
        0x0039_e2a0,
        0
    );

    macro_rules! install {
        ($slot:path, $fake:ident) => {
            fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
        };
    }

    /// Hooks this module's callee fakes (once per process) and gates them on
    /// in the test machine `c`.
    fn oc969_setup(c: &mut Cpu) {
        install!(ee::s13::OVERRIDE_0039E4C0, oc969_fake_0039e4c0);
        install!(ee::s01::OVERRIDE_0014DC80, oc969_fake_0014dc80);
        install!(ee::s01::OVERRIDE_0014DD58, oc969_fake_0014dd58);
        install!(ee::s01::OVERRIDE_00146E98, oc969_fake_00146e98);
        install!(ee::s15::OVERRIDE_00417828, oc969_fake_00417828);
        install!(ee::s05::OVERRIDE_0020A6F0, oc969_fake_0020a6f0);
        install!(ee::s07::OVERRIDE_0028B180, oc969_fake_0028b180);
        install!(ee::s08::OVERRIDE_00294F78, oc969_fake_00294f78);
        install!(ee::s05::OVERRIDE_0020E900, oc969_fake_0020e900);
        install!(ee::s10::OVERRIDE_00317670, oc969_fake_00317670);
        install!(ee::s13::OVERRIDE_0039D860, oc969_fake_0039d860);
        install!(ee::s13::OVERRIDE_003A1000, oc969_fake_003a1000);
        install!(ee::s01::OVERRIDE_001474A8, oc969_fake_001474a8);
        install!(ee::s15::OVERRIDE_00416810, oc969_fake_00416810);
        install!(ee::s01::OVERRIDE_00147138, oc969_fake_00147138);
        install!(ee::s01::OVERRIDE_0014EEC8, oc969_fake_0014eec8);
        install!(ee::s01::OVERRIDE_00147398, oc969_fake_00147398);
        install!(ee::s13::OVERRIDE_003A0C50, oc969_fake_003a0c50);
        install!(ee::s14::OVERRIDE_003E6448, oc969_fake_003e6448);
        install!(ee::s03::OVERRIDE_001A1CD0, oc969_fake_001a1cd0);
        install!(ee::s01::OVERRIDE_00148158, oc969_fake_00148158);
        install!(ee::s13::OVERRIDE_0039E2A0, oc969_fake_0039e2a0);
        c.bus.write32(OC969_GATE, OC969_MAGIC);
    }

    #[test]
    fn ovscreen_freestyle_standings_on_activate_matches() {
        check(
            ee::s04::f_001e64c8__generated,
            ovscreen_freestyle_standings_on_activate,
            200,
            &|c, seed| {
                oc969_setup(c);
                let mut r = fakes::machine(c, seed, 0x969_0001, 12);
                let this = DATA + 0x2000;
                let world = DATA + 0x2300;
                let heat = DATA + 0x2400;
                c.g[A0][0] = sx(this);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0xc0, heat);
                c.bus.write32(heat, r.next_u32() % 20);
                c.bus.write32(OC969_MBX + 4, DATA + 0x2500);
                c.bus.write32(OC969_MBX + 8, 1 + seed % 8);
                for i in 0..10 {
                    c.bus.write32(0x0053_6730 + 4 * i, r.next_u32() % 12);
                }
            },
        );
    }

    #[test]
    fn ovscreen_final_standings_update_matches() {
        // The real `0x20a4e0` (RecvChatMessage, no override slot) returns 0
        // on this fixture, so the branches that need it to return non-zero
        // are not reached here.
        check(
            ee::s04::f_001e7928__generated,
            ovscreen_final_standings_update,
            200,
            &|c, seed| {
                oc969_setup(c);
                let mut r = fakes::machine(c, seed, 0x969_0002, 8);
                let this = DATA + 0x2000;
                let block = DATA + 0x2300;
                let beat = DATA + 0x2400;
                let obj = DATA + 0x2500;
                let vt = DATA + 0x2600;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x40, DATA + 0x2700);
                c.bus.write32(GAME_GP - 0x204, block);
                let live = if seed % 4 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 5
                };
                c.bus.write32(block, live);
                c.bus
                    .write32(block + 0x64, [0, u32::MAX, 7][(seed % 3) as usize]);
                c.bus.write32(GAME_GP + 0x2a74, beat);
                let race_clock = if seed % 2 == 0 {
                    30 * (r.next_u32() % 1000)
                } else {
                    r.next_u32()
                };
                c.bus.write32(beat + 0x18, race_clock);
                let object = if seed % 3 == 1 { OC969_NULL } else { obj };
                c.bus.write32(OC969_MBX + 16, object);
                c.bus.write32(obj + 8, vt);
                c.bus.write16(vt + 0x48, 0);
                c.bus.write32(vt + 0x4c, fakes::FAKE);
            },
        );
    }

    #[test]
    fn ov_peak_challenge_results_update_matches() {
        check(
            ee::s04::f_001e8e70__generated,
            ov_peak_challenge_results_update,
            60,
            &|c, seed| {
                oc969_setup(c);
                fakes::machine(c, seed, 0x969_0003, 2);
            },
        );
    }

    #[test]
    fn ov_multiplayer_standings_empty_hook_matches() {
        check(
            ee::s04::f_001e8fc0__generated,
            ov_multiplayer_standings_empty_hook,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x969_0004, 0);
            },
        );
    }

    #[test]
    fn hud_draw_rect_matches() {
        check(
            ee::s04::f_001f1338__generated,
            hud_draw_rect,
            200,
            &|c, seed| {
                oc969_setup(c);
                let mut r = fakes::machine(c, seed, 0x969_0005, 4);
                let color = DATA + 0x100;
                let pos = DATA + 0x120;
                let scale = DATA + 0x140;
                let size = DATA + 0x160;
                for i in 0..4 {
                    let v = if (seed + i) % 3 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(2.0)
                    };
                    c.bus.write32(color + 4 * i, v);
                }
                for i in 0..2 {
                    c.bus.write32(pos + 4 * i, r.f32(300.0));
                    c.bus.write32(scale + 4 * i, r.f32(4.0));
                    c.bus.write32(size + 4 * i, r.f32(80.0));
                }
                c.g[A1][0] = sx(pos);
                c.g[A2][0] = sx(scale);
                c.g[A3][0] = sx(size);
                c.g[T0][0] = sx(color);
                let world = DATA + 0x3000;
                let ptr_e = DATA + 0x4000;
                let vt = DATA + 0x5000;
                c.bus.write32(GAME_GP - 0x854, world);
                c.bus.write32(world + 0xe84, ptr_e);
                c.bus.write32(world + 0x10d8, vt);
                c.bus.write16(vt + 0x238, r.next_u32() as u16);
                c.bus.write32(vt + 0x23c, fakes::FAKE);
            },
        );
    }

    #[test]
    fn cfe_state_lodge_rider_detail_on_child_event_matches() {
        check(
            ee::s04::f_001f4108__generated,
            cfe_state_lodge_rider_detail_on_child_event,
            300,
            &|c, seed| {
                oc969_setup(c);
                let mut r = fakes::machine(c, seed, 0x969_0006, 6);
                let rider = DATA + 0x6000;
                let state = DATA + 0x6100;
                let q = DATA + 0x6200;
                let iface = DATA + 0x6300;
                let vt = DATA + 0x6400;
                let inner = DATA + 0x6500;
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(state);
                c.g[A2][0] = if seed % 2 == 0 {
                    0x16
                } else {
                    r.next_u32() as u64 % 40
                };
                c.bus.write32(rider + 0x44, r.next_u32());
                c.bus
                    .write32(state + 0xc, if seed % 3 == 0 { 0 } else { q });
                c.bus
                    .write32(state + 0x5c, if seed % 5 == 0 { 0 } else { inner });
                c.bus.write32(state + 0x70, inner + 0x40);
                c.bus
                    .write32(state + 0x74, if seed % 4 == 0 { 0 } else { inner + 0x80 });
                c.bus.write32(OC969_MBX + 4, iface);
                c.bus
                    .write32(OC969_MBX + 12, if seed % 2 == 0 { q } else { 0 });
                c.bus
                    .write32(OC969_MBX + 20, if seed % 2 == 0 { OC969_NULL } else { 0 });
                c.bus.write32(iface + 0xc, vt);
                c.bus.write16(vt + 0x8, 0);
                c.bus.write32(vt + 0xc, fakes::FAKE);
            },
        );
    }

    #[test]
    fn cfe_state_lodge_rider_detail_on_widget_created_matches() {
        check(
            ee::s04::f_001f4218__generated,
            cfe_state_lodge_rider_detail_on_widget_created,
            300,
            &|c, seed| {
                oc969_setup(c);
                fakes::machine(c, seed, 0x969_0007, 12);
                let state = DATA + 0x6600;
                let value = 0x1230_0000 + seed;
                // The step whose name hash matches (0..7; 8 = no match).
                let k = seed % 9;
                c.g[A0][0] = sx(DATA + 0x6700);
                c.g[A1][0] = sx(state);
                c.bus.write32(state + 0x38, value);
                if k < 8 {
                    c.bus.write32(fakes::LOG_RESULTS + 4 * k, value);
                }
            },
        );
    }

    #[test]
    fn cfe_state_buy_attrib_ctor_matches() {
        check(
            ee::s04::f_001f4728__generated,
            cfe_state_buy_attrib_ctor,
            200,
            &|c, seed| {
                oc969_setup(c);
                fakes::machine(c, seed, 0x969_0008, 12);
                let this = DATA + 0x6a00;
                let field = DATA + 0x6c00;
                let inner = DATA + 0x6d00;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x10, field);
                c.bus.write32(field, inner);
                c.bus.write32(inner, seed);
            },
        );
    }
}
