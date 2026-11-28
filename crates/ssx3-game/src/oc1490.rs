//! Staging module oc1490 (wave wave275): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1490`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0011_5358,
        rider_align_stance_to_rail,
        "Rider_AlignStanceToRail",
    ),
    (
        0x002f_22c8,
        attached_emitter_fx_find_bones,
        "AttachedEmitterFX_FindBones",
    ),
    (
        0x002f_e0c0,
        wscript_add_multi_spline_modifier,
        "WScriptFn_AddMultiSplineModifier",
    ),
    (0x003b_7450, sndstrm_open, "SNDSTRM_Open"),
    (
        0x002f_43e0,
        lensfx_set_default_flares,
        "LensFX_SetDefaultFlares",
    ),
];

/// `cRider` fields `Rider_AlignStanceToRail` reads and writes.
pub(crate) const RAILALIGN_FLAG: u32 = 0x320;
pub(crate) const RAILALIGN_MODE: u32 = 0x328;
pub(crate) const RAILALIGN_UP: u32 = 0x120;
pub(crate) const RAILALIGN_AXIS: u32 = 0x1c0;
pub(crate) const RAILALIGN_ANIM: u32 = 0x784;

/// The vector part of the rail alignment: `out = axis x up` through VU0
/// (the `lqc2`/`vopmula`/`vopmsub` sequence the compiler emitted for the
/// vector cross product, with the axis copied to the stack first with its
/// w cleared). Stores the result at `sp + out` and reloads it into `v0`
/// as a quadword (the caller writes it to the rider and to the stack).
fn railalign_cross(c: &mut Cpu, out: u32) {
    let s3 = c.g[S3][0] as u32;
    let sp0 = sp(c);
    c.lq(2, s3.wrapping_add(RAILALIGN_AXIS));
    c.sw(sp0.wrapping_add(0x1c), 0u64);
    c.sq(2, sp0.wrapping_add(0x30));
    c.lwc1(0, sp0.wrapping_add(0x30));
    c.lwc1(1, sp0.wrapping_add(0x34));
    c.lwc1(2, sp0.wrapping_add(0x38));
    c.swc1(0, sp0.wrapping_add(0x10));
    c.swc1(1, sp0.wrapping_add(0x14));
    c.swc1(2, sp0.wrapping_add(0x18));
    c.swc1(0, sp0.wrapping_add(0x20));
    c.swc1(1, sp0.wrapping_add(0x24));
    c.swc1(2, sp0.wrapping_add(0x28));
    c.lqc2(4, sp0.wrapping_add(0x10));
    c.lqc2(5, s3.wrapping_add(RAILALIGN_UP));
    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
    c.sqc2(8, sp0.wrapping_add(out));
    c.lq(2, sp0.wrapping_add(out));
}

/// Writes the new up vector (`v0`, set by [`railalign_cross`]) to the rider
/// and rebuilds the world matrix and the stance (`Rider_BuildWorldMatrix`,
/// `Rider_ToggleStance`). `ret_build`/`ret_toggle` are the return addresses.
fn railalign_rebuild(c: &mut Cpu, ret_build: u32, ret_toggle: u32) -> Flow {
    let s3 = c.g[S3][0] as u32;
    let sp0 = sp(c);
    c.sq(2, s3.wrapping_add(RAILALIGN_UP));
    c.sq(2, sp0);
    Callee {
        addr: 0x0011_e098,
        f: ee::s00::f_0011e098,
    }
    .call(c, ret_build)?;
    c.g[A0][0] = c.g[S3][0];
    Callee {
        addr: 0x0011_5168,
        f: ee::s00::f_00115168,
    }
    .call(c, ret_toggle)
}

/// The board-lean branch (`mode == 4`, or the other modes' mirrored one):
/// rotates the rider about the lean axis by `gp[axis_gp]` radians, rotates
/// the anim roots by the same angle, sets the root transform quad, takes
/// `Math_SinCos` of `gp[angle_gp]` and scales the root's lean vector by the
/// sine. `cycles` is the block charge, `out` the `sin`/`cos` scratch slot
/// on the stack, `rets` the three return addresses (rotate, roots, sincos).
fn railalign_lean(
    c: &mut Cpu,
    cycles: u64,
    axis_gp: u32,
    angle_gp: u32,
    out: u32,
    rets: [u32; 3],
) -> Flow {
    let s3 = c.g[S3][0] as u32;
    let sp0 = sp(c);
    let gp = c.g[GP][0] as u32;
    c.cycles += cycles;
    c.g[A1][0] = c.g[SP][0];
    c.lwc1(20, gp.wrapping_sub(axis_gp));
    c.g[A0][0] = c.g[S3][0];
    c.sq(2, sp0);
    c.g[S0][0] = 0x500000;
    c.mov_s(12, 20);
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xea0));
    Callee {
        addr: 0x0011_dfe0,
        f: ee::s00::f_0011dfe0,
    }
    .call(c, rets[0])?;
    c.g[A0][0] = c.lw(s3.wrapping_add(RAILALIGN_ANIM));
    c.mov_s(12, 20);
    call_ptr(c, 0x0031_1b48, rets[1])?;
    c.g[V0][0] = 0x500000;
    c.g[S2][0] = c.lw(s3.wrapping_add(RAILALIGN_ANIM));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
    c.lwc1(12, gp.wrapping_sub(angle_gp));
    c.lq(3, c.g[V0][0] as u32);
    c.g[A0][0] = sx(sp0.wrapping_add(out));
    c.g[A1][0] = sx(sp0.wrapping_add(out + 4));
    let root = c.g[S2][0] as u32;
    c.sq(3, root.wrapping_add(0x30));
    call_ptr(c, 0x0031_be50, rets[2])?;
    let s1 = c.g[S1][0] as u32;
    let s0 = c.g[S0][0] as u32;
    c.lwc1(1, s1.wrapping_add(0x8));
    c.lwc1(0, sp0.wrapping_add(out));
    c.lwc1(2, s0.wrapping_sub(0xea0));
    c.lwc1(4, s1.wrapping_add(0x4));
    c.mul_s(1, 0, 1);
    c.mul_s(2, 0, 2);
    c.lwc1(3, sp0.wrapping_add(out + 4));
    Ok(())
}

/// `void Rider_AlignStanceToRail(cRider* self)` (0x115358; the reference
/// calls it `cRider_doRailslideModeOrientation`): while grinding a rail the
/// rider's up vector is re-derived as `axis x up` for modes 1 and 2 (when the
/// flag at +0x320 is clear, mode 2 always) and the world matrix and stance
/// are rebuilt; any other mode turns the board with `Rider_RotateAroundAxis`
/// by a lean angle from globals (mode 4 and the others differ in the angle
/// pair) and writes the sine-scaled lean vector into the anim root; every
/// path ends in `Rider_EmptyStanceHook`.
pub fn rider_align_stance_to_rail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0xd0);
        c.g[V0][0] = 0x2;
        frame.save(c, S3, 0x80);
        frame.save(c, S0, 0xb0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 20, 0xc0);
        let s3 = c.g[S3][0] as u32;
        c.g[V1][0] = c.lw(s3.wrapping_add(RAILALIGN_MODE));
        let not_mode2 = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 0x1;
        'exit: {
            'other: {
                'build: {
                    'first: {
                        if !not_mode2 {
                            // mode 2: rebuild unless the flag is set
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(s3.wrapping_add(RAILALIGN_FLAG));
                            if c.g[V0][0] != 0u64 {
                                break 'exit;
                            }
                            c.cycles += 26;
                            railalign_cross(c, 0x40);
                            break 'build;
                        }
                        break 'first;
                    }
                    // not mode 2: mode 1 rebuilds if the flag is set
                    c.cycles += 2;
                    let not_mode1 = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(s3.wrapping_add(RAILALIGN_FLAG));
                    if not_mode1 {
                        break 'other;
                    }
                    c.cycles += 2;
                    let no_flag = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S3][0];
                    if no_flag {
                        break 'exit;
                    }
                    c.cycles += 25;
                    railalign_cross(c, 0x50);
                }
                c.cycles += 7;
                railalign_rebuild(c, 0x0011_547c, 0x0011_5484)?;
                break 'exit;
            }
            // other modes: rebuild when the flag is set, then the lean
            let s3 = c.g[S3][0] as u32;
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = c.lw(s3.wrapping_add(RAILALIGN_MODE));
            } else {
                c.cycles += 32;
                c.g[A0][0] = c.g[S3][0];
                railalign_cross(c, 0x40);
                railalign_rebuild(c, 0x0011_5508, 0x0011_5510)?;
                c.g[V1][0] = c.lw(s3.wrapping_add(RAILALIGN_MODE));
            }
            c.cycles += 3;
            c.g[V0][0] = 0x4;
            let not_mode4 = c.g[V1][0] != c.g[V0][0];
            c.lq(2, s3.wrapping_add(RAILALIGN_AXIS));
            if !not_mode4 {
                railalign_lean(
                    c,
                    28,
                    0x7b80,
                    0x7b7c,
                    0x60,
                    [0x0011_5540, 0x0011_554c, 0x0011_5570],
                )?;
            } else {
                railalign_lean(
                    c,
                    27,
                    0x7b78,
                    0x7b74,
                    0x68,
                    [0x0011_55b0, 0x0011_55bc, 0x0011_55e0],
                )?;
            }
            let sp0 = sp(c);
            let root = c.g[S2][0] as u32;
            c.cycles += 7;
            c.mul_s(0, 0, 4);
            c.swc1(3, sp0.wrapping_add(0xc));
            c.swc1(1, sp0.wrapping_add(0x8));
            c.swc1(2, sp0);
            c.swc1(0, sp0.wrapping_add(0x4));
            c.lq(2, sp0);
            c.sq(2, root.wrapping_add(0x40));
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S3][0];
        Callee {
            addr: 0x0011_6930,
            f: ee::s00::f_00116930,
        }
        .call(c, 0x0011_5620)?;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 20, 0xc0);
        frame.close(c);
        Ok(())
    });
}

/// The bone-name table of `AttachedEmitterFX_FindBones`: for each of the 26
/// handles (`h[0]` at +4 ... `h[25]` at +0x68): the return address of the
/// lookup call, the skeleton (0: rider, 1: board), and the name string.
const ATTACHEDEMITTER_BONES: [(u32, u32, u32); 26] = [
    (0x002f_22f0, 0, 0x004a_3bb8),
    (0x002f_2308, 0, 0x004a_3b08),
    (0x002f_2320, 0, 0x0048_7a58),
    (0x002f_2338, 0, 0x0048_7a48),
    (0x002f_2350, 0, 0x0048_7a38),
    (0x002f_2368, 0, 0x004a_3b00),
    (0x002f_2380, 0, 0x0048_7a68),
    (0x002f_2398, 0, 0x0048_7a88),
    (0x002f_23b0, 0, 0x0048_7a98),
    (0x002f_23c8, 0, 0x0048_7aa8),
    (0x002f_23e0, 0, 0x0048_7ab8),
    (0x002f_23f8, 0, 0x0048_7ac8),
    (0x002f_2410, 0, 0x0048_7ad8),
    (0x002f_2428, 0, 0x0048_7ae8),
    (0x002f_2440, 0, 0x0048_7af8),
    (0x002f_2458, 0, 0x0048_7a78),
    (0x002f_2470, 0, 0x0048_7a18),
    (0x002f_2488, 0, 0x0048_79d8),
    (0x002f_24a0, 0, 0x0048_79e8),
    (0x002f_24b8, 0, 0x0048_7a28),
    (0x002f_24d0, 0, 0x0048_79f8),
    (0x002f_24e8, 0, 0x0048_7a08),
    (0x002f_2504, 1, 0x0048_7cc8),
    (0x002f_2518, 1, 0x0048_7cc8),
    (0x002f_252c, 1, 0x0048_7cc8),
    (0x002f_2540, 1, 0x0048_7cc8),
];
/// The attached emitter's anim model pointer.
pub(crate) const ATTACHEDEMITTER_MODEL: u32 = 0x94;

/// `void AttachedEmitterFX_FindBones(AttachedEmitterFX* self)` (0x2f22c8):
/// looks up the 26 bone handles the emitter follows (head, neck, spine,
/// arms, legs, then four times the board's child bone) with
/// `cAnimModel_getBoneSkelIndex(model, skeleton, name)` (0x310c48), stores
/// them in `self->h[0..26]` (+4..+0x6c) and marks the emitter ready (+0).
/// The retail loop after the lookups (a compiled-out per-handle assert) is
/// empty but keeps its 26 iterations of event checks and cycles.
pub fn attached_emitter_fx_find_bones(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 162;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        for (i, &(ret_addr, skeleton, name)) in ATTACHEDEMITTER_BONES.iter().enumerate() {
            let s1 = c.g[S1][0] as u32;
            if i == 22 {
                // the first four board bones share the name in s0
                c.g[S0][0] = sx(0x0048_7cc8);
            }
            c.g[A0][0] = c.lw(s1.wrapping_add(ATTACHEDEMITTER_MODEL));
            c.g[A1][0] = skeleton as u64;
            c.g[A2][0] = sx(name);
            if i > 0 {
                // the previous lookup's handle (v0 still holds it)
                let v0 = c.g[V0][0];
                c.sw(s1.wrapping_add(4 * i as u32), v0);
            }
            call_ptr(c, 0x0031_0c48, ret_addr)?;
        }
        let s1 = c.g[S1][0] as u32;
        let v0 = c.g[V0][0];
        c.sw(s1.wrapping_add(0x68), v0);
        c.g[V0][0] = 0x19;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        loop {
            c.cycles += 7;
            let more = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002f_2550);
                }
                continue;
            }
            break;
        }
        c.cycles += 7;
        c.g[V0][0] = 0x1;
        frame.restore(c, S0, 0x20);
        let s1 = c.g[S1][0] as u32;
        let ready = c.g[V0][0];
        c.sw(s1, ready);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cLunoVar AddMultiSplineFunc(int numArgs, cLunoCArg* params)` (0x2fe0c0,
/// the WScript function `AddMultiSplineModifier`): on its first call fills
/// the static default argument block (`D_E0C0`: instance id -1, target -1,
/// mode 3, 20.0 speed, unit scale, trailing flags), then copies the 13-word
/// block to the stack, overwrites the entries the script passed
/// (`params[i] = {index, value, _, type}`; a float-typed slot given an int
/// is converted), looks the target instance up (id -1: the script manager's
/// own instance; else through the world cache's section and tag tables) and,
/// when the second argument is not -1 and the instance's active node is a
/// move node (`isMoveNode` virtual), calls `cMoveNode::addSpline(node, buf)`.
/// Returns the nil `cLunoVar` (copied through `a0`'s hidden return pointer).
pub fn wscript_add_multi_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        'b_2fe15c: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2528));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[T0][0] = 0x500000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S2][0] = c.g[A2][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S3][0] = c.g[A0][0];
            let t = c.g[V0][0] != 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
            if t {
                break 'b_2fe15c;
            } else {
                c.cycles += 27;
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4858));
                c.g[V1][0] = 0xffffffffffff0000;
                c.mtc1(0, 0);
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.g[A3][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = 0x3;
                c.g[AT][0] = 0x41a00000;
                c.mtc1(1, 2);
                c.g[A1][0] = 0x2;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 1);
                c.g[A2][0] = 0x1;
                c.sw((c.g[T0][0] as u32).wrapping_sub(0x4858), c.g[V1][0]);
                c.sb((c.g[V0][0] as u32).wrapping_add(0x30), c.g[A3][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x24));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2c), c.g[A1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2528), c.g[A2][0]);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x28), c.g[A2][0]);
                c.sb((c.g[SP][0] as u32), c.g[A3][0]);
                break 'b_2fe15c;
            }
        }
        'b_2fe1e0: {
            c.cycles += 11;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.g[A2][0] = 0x34;
            c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4858));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.g[A0][0] = c.g[S1][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            call_ptr(c, 0x003e6574, 0x002fe180)?;
            let t = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if t {
                break 'b_2fe1e0;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S0][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6208));
                c.g[A3][0] = 0x2;
                'l_2fe198: loop {
                    'b_2fe1d4: {
                        'b_2fe1d0: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_2fe1d0;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_2fe1d0;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    c.swc1(0, (c.g[A0][0] as u32));
                                    break 'b_2fe1d4;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                        break 'b_2fe1d4;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002fe198);
                        }
                        continue 'l_2fe198;
                    } else {
                        break 'b_2fe1e0;
                    }
                }
            }
        }
        'b_2fe240: {
            'b_2fe1fc: {
                c.cycles += 4;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32));
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                if t {
                    break 'b_2fe1fc;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                    break 'b_2fe240;
                }
            }
            'b_2fe23c: {
                'b_2fe238: {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                    if t {
                        break 'b_2fe238;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if t {
                            break 'b_2fe23c;
                        } else {
                            break 'b_2fe238;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_2fe23c;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
            break 'b_2fe240;
        }
        'b_2fe284: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = 0x4d0000;
                break 'b_2fe284;
            } else {
                c.cycles += 3;
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = c.g[S0][0] == 0u64;
                c.g[V1][0] = 0x4d0000;
                if t {
                    break 'b_2fe284;
                } else {
                    'b_2fe280: {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x80));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002fe270)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_2fe280;
                        } else {
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00355b30, 0x002fe280)?;
                            break 'b_2fe280;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = 0x4d0000;
                    break 'b_2fe284;
                }
            }
        }
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, (c.g[A3][0] as u32));
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, (c.g[S3][0] as u32));
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `int SNDSTRM_Open(SNDSTRMPARAMS* src, int n, int maxChunks, void* buf,
/// int size, void* tapStream, int ext)` (0x3b7450): opens a sound stream.
/// Finds a free slot in the stream table (none: -9), carves the stream
/// channel (0x138 bytes), its `n` request records (0x28 bytes each) and the
/// packet-player memory out of `buf`, initialises the allocated and free
/// request lists, allocates a packet instance (negative result: returned),
/// then either adopts `tapStream` (`ext`) or creates a file stream
/// (failure: frees the packet instance, -9). Stores the stream parameters,
/// registers the service callback with the first open stream, publishes the
/// channel in the table, sets the greedy low-water mark (`buffersize / 3`
/// for file streams) and purges (stops) the new slot. Returns the slot.
pub fn sndstrm_open(c: &mut Cpu) {
    guest(c, |c| {
        'b_3b76bc: {
            'b_3b76b8: {
                'b_3b75f0: {
                    'b_3b75ec: {
                        'b_3b75e4: {
                            c.cycles += 23;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                            c.g[V0][0] = 0x510000;
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
                            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                            c.g[S1][0] = 0u64;
                            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.g[S3][0] = c.g[A3][0];
                            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.g[S4][0] = c.g[T0][0];
                            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[S5][0] = c.g[A1][0];
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.g[FP][0] = c.g[A2][0];
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                            c.sw((c.g[SP][0] as u32), c.g[A0][0]);
                            c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x46));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T1][0]);
                            let t = c.g[V1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                            if t {
                                break 'b_3b75e4;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0x510000;
                                c.g[A0][0] = c.g[V1][0];
                                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5f60));
                                'l_3b74b8: loop {
                                    'b_3b74dc: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S2][0] = c.g[S3][0];
                                        if t {
                                            break 'b_3b74dc;
                                        } else {
                                            c.cycles += 4;
                                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] =
                                                ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x003b74b8);
                                                }
                                                continue 'l_3b74b8;
                                            } else {
                                                c.cycles += 2;
                                                c.g[V0][0] = 0xfffffffffffffff7;
                                                break 'b_3b76b8;
                                            }
                                        }
                                    }
                                    'b_3b755c: {
                                        c.cycles += 23;
                                        c.g[A1][0] = 0u64;
                                        c.g[A2][0] = 0x138;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00416210, 0x003b74ec)?;
                                        c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x138));
                                        c.g[V0][0] = 0x28;
                                        c.g[S0][0] = c.g[S3][0];
                                        c.mult(0, 2, 21, 2);
                                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x138));
                                        c.g[A0][0] = c.g[FP][0];
                                        c.g[S6][0] = c.g[S1][0];
                                        c.g[S3][0] =
                                            sx((c.g[S3][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[S4][0] =
                                            sx((c.g[S4][0] as u32)
                                                .wrapping_sub((c.g[V0][0] as u32)));
                                        Callee {
                                            addr: 0x003c65f0,
                                            f: ee::s13::f_003c65f0,
                                        }
                                        .call(c, 0x003b7514)?;
                                        c.g[S7][0] = c.g[S3][0];
                                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x11c));
                                        c.g[S4][0] =
                                            sx((c.g[S4][0] as u32)
                                                .wrapping_sub((c.g[V0][0] as u32)));
                                        c.g[S3][0] =
                                            sx((c.g[S3][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        Callee {
                                            addr: 0x003c6278,
                                            f: ee::s13::f_003c6278,
                                        }
                                        .call(c, 0x003b7528)?;
                                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x128));
                                        Callee {
                                            addr: 0x003c6278,
                                            f: ee::s13::f_003c6278,
                                        }
                                        .call(c, 0x003b7530)?;
                                        let t = (c.g[S5][0] as i64) <= 0;
                                        c.g[A0][0] = c.g[FP][0];
                                        if t {
                                            break 'b_3b755c;
                                        } else {
                                            c.cycles += 2;
                                            c.g[S1][0] = c.g[S5][0];
                                            'l_3b7540: loop {
                                                c.cycles += 6;
                                                c.g[A1][0] = c.g[S0][0];
                                                c.g[A0][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x128));
                                                Callee {
                                                    addr: 0x003c6288,
                                                    f: ee::s13::f_003c6288,
                                                }
                                                .call(c, 0x003b754c)?;
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                                let t = c.g[S1][0] != 0u64;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x28));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x003b7540);
                                                    }
                                                    continue 'l_3b7540;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[A0][0] = c.g[FP][0];
                                                    break 'b_3b755c;
                                                }
                                            }
                                        }
                                    }
                                    'b_3b75a0: {
                                        c.cycles += 14;
                                        c.g[S0][0] = 0x3b0000;
                                        Callee {
                                            addr: 0x003c65f0,
                                            f: ee::s13::f_003c65f0,
                                        }
                                        .call(c, 0x003b7564)?;
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6ad8));
                                        c.g[A1][0] = 0x3b0000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6b10));
                                        c.g[A3][0] = c.g[S7][0];
                                        c.g[T0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.g[S2][0];
                                        Callee {
                                            addr: 0x003c6620,
                                            f: ee::s13::f_003c6620,
                                        }
                                        .call(c, 0x003b7584)?;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        let t = (c.g[V0][0] as i64) >= 0;
                                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_3b75a0;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                                            break 'b_3b76bc;
                                        }
                                    }
                                    'b_3b75b8: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_3b75b8;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = 0x1;
                                            c.sb(
                                                (c.g[S2][0] as u32).wrapping_add(0x11),
                                                c.g[V0][0],
                                            );
                                            c.sw((c.g[S2][0] as u32), c.g[V1][0]);
                                            break 'b_3b75f0;
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x2));
                                    c.g[A3][0] = c.g[S3][0];
                                    c.g[T0][0] = c.g[S4][0];
                                    c.g[A1][0] = 0x1;
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x003e06d8, 0x003b75d0)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                                        break 'b_3b75ec;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        Callee {
                                            addr: 0x003c6da0,
                                            f: ee::s13::f_003c6da0,
                                        }
                                        .call(c, 0x003b75e4)?;
                                        break 'b_3b75e4;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[V0][0] = 0xfffffffffffffff7;
                        break 'b_3b76b8;
                    }
                    c.cycles += 1;
                    c.sb((c.g[S2][0] as u32).wrapping_add(0x11), 0u64);
                    break 'b_3b75f0;
                }
                'b_3b7664: {
                    c.cycles += 20;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                    c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.ldr(5, (c.g[A0][0] as u32));
                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.ldr(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(5, (c.g[S2][0] as u32).wrapping_add(0xf3));
                    c.sdr(5, (c.g[S2][0] as u32).wrapping_add(0xec));
                    c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0xfb));
                    c.sdr(6, (c.g[S2][0] as u32).wrapping_add(0xf4));
                    c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x103));
                    c.sdr(2, (c.g[S2][0] as u32).wrapping_add(0xfc));
                    Callee {
                        addr: 0x003b7410,
                        f: ee::s13::f_003b7410,
                    }
                    .call(c, 0x003b7638)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x510000;
                    if t {
                        break 'b_3b7664;
                    } else {
                        c.cycles += 9;
                        c.g[A0][0] = 0x3b0000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x71d8));
                        Callee {
                            addr: 0x003c6018,
                            f: ee::s13::f_003c6018,
                        }
                        .call(c, 0x003b764c)?;
                        c.g[V0][0] = 0x3b0000;
                        c.g[A0][0] = 0x510000;
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5718));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x69d8));
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1ec), c.g[V0][0]);
                        c.g[A1][0] = 0x510000;
                        break 'b_3b7664;
                    }
                }
                'b_3b76ac: {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5f60));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[S2][0]);
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    if c.g[A2][0] != 0u64 {
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x9c), 0u64);
                        break 'b_3b76ac;
                    } else {
                        'b_3b769c: {
                            c.cycles += 7;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                            call_ptr(c, 0x003e1580, 0x003b7688)?;
                            c.g[V1][0] = 0x3;
                            c.g[A0][0] = c.g[S6][0];
                            c.div(0, 2, 3);
                            if c.g[V1][0] == 0u64 {
                                break 'b_3b769c;
                            } else {
                                break 'b_3b769c;
                            }
                        }
                        c.cycles += 4;
                        c.g[A1][0] = c.lo[0];
                        Callee {
                            addr: 0x003b8120,
                            f: ee::s13::f_003b8120,
                        }
                        .call(c, 0x003b76a8)?;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x9c), 0u64);
                        break 'b_3b76ac;
                    }
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S6][0];
                Callee {
                    addr: 0x003b78f8,
                    f: ee::s13::f_003b78f8,
                }
                .call(c, 0x003b76b4)?;
                c.g[V0][0] = c.g[S6][0];
                break 'b_3b76b8;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            break 'b_3b76bc;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void LensFX_SetDefaultFlares(LensFX* self)` (0x2f43e0): resets the lens
/// flare list to the nine default sun flares: flare `i` is the 0x1c-byte
/// record at `self + 0x30 + 0x1c * i` (type, position along the light
/// axis, scale and a 16-byte colour copied from the static colour table at
/// 0x4fab80..), and `mNumLens` (+0x200) is bumped after each one.
pub fn lensfx_set_default_flares(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 171;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x38f0));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x38ec));
        c.g[A1][0] = 0x1;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = 0x500000;
        c.sq(17, (c.g[SP][0] as u32));
        c.g[T3][0] = 0x3;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x200), 0u64);
        c.g[V1][0] = 0x500000;
        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x34));
        c.g[A2][0] = 0x500000;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x30), c.g[A1][0]);
        c.g[T4][0] = 0x500000;
        c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x38));
        c.g[T5][0] = 0x500000;
        c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5480));
        c.ldl(7, (c.g[T2][0] as u32).wrapping_add(0x7));
        c.ldr(7, (c.g[T2][0] as u32));
        c.ldl(8, (c.g[T2][0] as u32).wrapping_add(0xf));
        c.ldr(8, (c.g[T2][0] as u32).wrapping_add(0x8));
        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x43));
        c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x3c));
        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x4b));
        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x44));
        c.g[T6][0] = 0x500000;
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x38e8));
        c.g[A3][0] = 0x2;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x200));
        c.g[T0][0] = 0x500000;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x38e4));
        c.g[T1][0] = 0x500000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0x50));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x200), c.g[V0][0]);
        c.g[T2][0] = 0x500000;
        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x54));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4c), c.g[T3][0]);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5460));
        c.ldl(15, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.ldr(15, (c.g[V0][0] as u32));
        c.ldl(16, (c.g[V0][0] as u32).wrapping_add(0xf));
        c.ldr(16, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.sdl(15, (c.g[A0][0] as u32).wrapping_add(0x5f));
        c.sdr(15, (c.g[A0][0] as u32).wrapping_add(0x58));
        c.sdl(16, (c.g[A0][0] as u32).wrapping_add(0x67));
        c.sdr(16, (c.g[A0][0] as u32).wrapping_add(0x60));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x38e0));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x200));
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0x70));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x200), c.g[V0][0]);
        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x6c));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x68), 0u64);
        c.g[S1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5440));
        c.ldl(3, (c.g[S1][0] as u32).wrapping_add(0x7));
        c.ldr(3, (c.g[S1][0] as u32));
        c.ldl(15, (c.g[S1][0] as u32).wrapping_add(0xf));
        c.ldr(15, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0x7b));
        c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0x74));
        c.sdl(15, (c.g[A0][0] as u32).wrapping_add(0x83));
        c.sdr(15, (c.g[A0][0] as u32).wrapping_add(0x7c));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x38dc));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x200));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x84), c.g[A3][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0x88));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x200), c.g[V0][0]);
        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8c));
        c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x5420));
        c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[A3][0] as u32));
        c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0xf));
        c.ldr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x97));
        c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x90));
        c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0x9f));
        c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0x98));
        c.sw((c.g[A0][0] as u32).wrapping_add(0xa0), 0u64);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x200));
        c.sw((c.g[A0][0] as u32).wrapping_add(0xa4), 0u64);
        c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0xa8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x200), c.g[V0][0]);
        c.g[S1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x5400));
        c.ldl(8, (c.g[S1][0] as u32).wrapping_add(0x7));
        c.ldr(8, (c.g[S1][0] as u32));
        c.ldl(15, (c.g[S1][0] as u32).wrapping_add(0xf));
        c.ldr(15, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xb3));
        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0xac));
        c.sdl(15, (c.g[A0][0] as u32).wrapping_add(0xbb));
        c.sdr(15, (c.g[A0][0] as u32).wrapping_add(0xb4));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x38d8));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x200));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x38d4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x38d0));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x200), c.g[V0][0]);
        c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0xc0));
        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc4));
        c.sw((c.g[A0][0] as u32).wrapping_add(0xbc), 0u64);
        c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x53e0));
        c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[A3][0] as u32));
        c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0xf));
        c.ldr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0xcf));
        c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0xc8));
        c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0xd7));
        c.sdr(3, (c.g[A0][0] as u32).wrapping_add(0xd0));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x38cc));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x200));
        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xe0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xdc));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x200), c.g[V0][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0xd8), c.g[A1][0]);
        c.g[T7][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x53c0));
        c.ldl(8, (c.g[T7][0] as u32).wrapping_add(0x7));
        c.ldr(8, (c.g[T7][0] as u32));
        c.ldl(9, (c.g[T7][0] as u32).wrapping_add(0xf));
        c.ldr(9, (c.g[T7][0] as u32).wrapping_add(0x8));
        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xeb));
        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0xe4));
        c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0xf3));
        c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0xec));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x38c8));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x200));
        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xf8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[A0][0] as u32).wrapping_add(0xf4), c.g[T3][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x200), c.g[V0][0]);
        c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0xfc));
        c.g[V1][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x53a0));
        c.ldl(16, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.ldr(16, (c.g[V1][0] as u32));
        c.ldl(17, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.ldr(17, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(16, (c.g[A0][0] as u32).wrapping_add(0x107));
        c.sdr(16, (c.g[A0][0] as u32).wrapping_add(0x100));
        c.sdl(17, (c.g[A0][0] as u32).wrapping_add(0x10f));
        c.sdr(17, (c.g[A0][0] as u32).wrapping_add(0x108));
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 1);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x200));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x38c4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x110), c.g[A1][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x200), c.g[V0][0]);
        c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0x114));
        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x118));
        c.g[T0][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x5380));
        c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0x7));
        c.ldr(5, (c.g[T0][0] as u32));
        c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0xf));
        c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x8));
        c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x123));
        c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x11c));
        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x12b));
        c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x124));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x200));
        c.lq(17, (c.g[SP][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x200), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching, covered, machine};
    use crate::difftest::{check, fill, put_vec, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// This module's gate word: its gated callee fakes act only on machines
    /// that carry [`OC1490_MAGIC`] here, so no other test sees them.
    const OC1490_GATE: u32 = DATA + 0x20_0100;
    const OC1490_MAGIC: u32 = 0x1490_c0de;
    /// Forced-result mailboxes (8 bytes per fake: magic, value).
    const OC1490_MBOX: u32 = DATA + 0x20_0200;

    /// A gated stand-in for the guest function at `$addr`: it logs the call
    /// through `fakes::fake` (every register, so the diff compares the
    /// arguments, and an unwind at the chosen call index), then replaces the
    /// result by the mailbox value `$k` when the test forced one. With the
    /// gate off it runs the body the slot held before.
    macro_rules! oc1490_gated_fake {
        ($name:ident, $k:expr, $addr:expr, $generated:path) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1490_GATE) != OC1490_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
                let mbox = OC1490_MBOX + 8 * $k;
                if c.pc == ra && c.bus.read32(mbox) == OC1490_MAGIC {
                    c.g[V0][0] = sx(c.bus.read32(mbox + 4));
                }
            }
        };
    }

    oc1490_gated_fake!(
        oc1490_fake_build_world,
        0,
        0x0011_e098,
        ee::s00::f_0011e098__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_toggle_stance,
        1,
        0x0011_5168,
        ee::s00::f_00115168__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_rotate_axis,
        2,
        0x0011_dfe0,
        ee::s00::f_0011dfe0__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_empty_hook,
        3,
        0x0011_6930,
        ee::s00::f_00116930__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_rotate_roots,
        4,
        0x0031_1b48,
        ee::s10::f_00311b48__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_sincos,
        5,
        0x0031_be50,
        ee::s10::f_0031be50__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_bone_index,
        6,
        0x0031_0c48,
        ee::s10::f_00310c48__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_add_spline,
        7,
        0x0035_5b30,
        ee::s11::f_00355b30__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_memset,
        8,
        0x0041_6210,
        ee::s15::f_00416210__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_calc_slot_mem,
        9,
        0x003c_65f0,
        ee::s13::f_003c65f0__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_list_init,
        10,
        0x003c_6278,
        ee::s13::f_003c6278__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_push_front,
        11,
        0x003c_6288,
        ee::s13::f_003c6288__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_alloc_slot,
        12,
        0x003c_6620,
        ee::s13::f_003c6620__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_stream_create,
        13,
        0x003e_06d8,
        ee::s14::f_003e06d8__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_free_slot,
        14,
        0x003c_6da0,
        ee::s13::f_003c6da0__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_count_open,
        15,
        0x003b_7410,
        ee::s13::f_003b7410__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_add_service,
        16,
        0x003c_6018,
        ee::s13::f_003c6018__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_buffer_size,
        17,
        0x003e_1580,
        ee::s14::f_003e1580__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_set_low_water,
        18,
        0x003b_8120,
        ee::s13::f_003b8120__generated
    );
    oc1490_gated_fake!(
        oc1490_fake_strm_stop,
        19,
        0x003b_78f8,
        ee::s13::f_003b78f8__generated
    );

    /// Hooks every gated fake into its override slot (once per process).
    fn oc1490_install() {
        let hooks: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 20] = [
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011E098),
                oc1490_fake_build_world,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00115168),
                oc1490_fake_toggle_stance,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011DFE0),
                oc1490_fake_rotate_axis,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00116930),
                oc1490_fake_empty_hook,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00311B48),
                oc1490_fake_rotate_roots,
            ),
            (addr_of_mut!(ee::s10::OVERRIDE_0031BE50), oc1490_fake_sincos),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00310C48),
                oc1490_fake_bone_index,
            ),
            (
                addr_of_mut!(ee::s11::OVERRIDE_00355B30),
                oc1490_fake_add_spline,
            ),
            (addr_of_mut!(ee::s15::OVERRIDE_00416210), oc1490_fake_memset),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003C65F0),
                oc1490_fake_calc_slot_mem,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003C6278),
                oc1490_fake_list_init,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003C6288),
                oc1490_fake_push_front,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003C6620),
                oc1490_fake_alloc_slot,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E06D8),
                oc1490_fake_stream_create,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003C6DA0),
                oc1490_fake_free_slot,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003B7410),
                oc1490_fake_count_open,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003C6018),
                oc1490_fake_add_service,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E1580),
                oc1490_fake_buffer_size,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003B8120),
                oc1490_fake_set_low_water,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003B78F8),
                oc1490_fake_strm_stop,
            ),
        ];
        for (slot, fake) in hooks {
            fakes::hook(slot, fake, fakes::prev_cell(fake));
        }
    }

    /// A fake-callee machine with this module's gate on; `calls` bounds the
    /// call index at which a fake may unwind.
    fn oc1490_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1490_install();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1490_GATE, OC1490_MAGIC);
        r
    }

    /// Forces the result of the gated fake `k` (sign-extended 32 bits).
    fn oc1490_force(c: &mut Cpu, k: u32, value: u32) {
        c.bus.write32(OC1490_MBOX + 8 * k, OC1490_MAGIC);
        c.bus.write32(OC1490_MBOX + 8 * k + 4, value);
    }

    /// `Rider_AlignStanceToRail`: every mode (0..5) with the flag set and
    /// clear, all callees faked (logged, with unwinds), the VU0 cross product
    /// and the lean branches included.
    #[test]
    fn rider_align_stance_to_rail_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = oc1490_machine(c, seed, 0x5358, 6);
            let rider = DATA + 0x1000;
            let modes = [2u32, 1, 0, 3, 4, 5, 2, 1, 4];
            c.bus
                .write32(rider + RAILALIGN_MODE, modes[(seed % 9) as usize]);
            c.bus.write32(rider + RAILALIGN_FLAG, (seed / 9) & 1);
            for off in [RAILALIGN_AXIS, RAILALIGN_UP] {
                let v = [r.f32(4.0), r.f32(4.0), r.f32(4.0), r.f32(4.0)];
                put_vec(c, rider + off, v);
            }
            c.bus.write32(rider + RAILALIGN_ANIM, DATA + 0x4000);
            for i in 0..4 {
                let v = r.f32(3.0);
                c.bus.write32(GAME_GP - 0x7b80 + 4 * i, v);
            }
            let q = [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(2.0)];
            put_vec(c, 0x004f_f130, q);
            for i in 0..4 {
                let v = r.f32(2.0);
                c.bus.write32(0x004f_f160 + 4 * i, v);
            }
            c.g[A0][0] = rider as u64;
        };
        covered(
            ssx3_ee::s00::f_00115358__generated,
            rider_align_stance_to_rail,
            180,
            &setup,
        );
        check_reaching(
            ssx3_ee::s00::f_00115358__generated,
            rider_align_stance_to_rail,
            180,
            &[
                0x0011_547c,
                0x0011_5484,
                0x0011_5508,
                0x0011_5510,
                0x0011_5540,
                0x0011_554c,
                0x0011_5570,
                0x0011_55b0,
                0x0011_55bc,
                0x0011_55e0,
                0x0011_5620,
            ],
            &setup,
        );
    }

    /// `AttachedEmitterFX_FindBones`: 26 faked bone lookups (distinct
    /// results, so every handle store is checked) with unwinds at random
    /// lookups.
    #[test]
    fn attached_emitter_fx_find_bones_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let _r = oc1490_machine(c, seed, 0x22c8, 26);
            let emitter = DATA + 0x1000;
            c.bus
                .write32(emitter + ATTACHEDEMITTER_MODEL, DATA + 0x3000);
            c.g[A0][0] = emitter as u64;
        };
        covered(
            ssx3_ee::s09::f_002f22c8__generated,
            attached_emitter_fx_find_bones,
            100,
            &setup,
        );
        check_reaching(
            ssx3_ee::s09::f_002f22c8__generated,
            attached_emitter_fx_find_bones,
            100,
            &[0x002f_22f0, 0x002f_2540],
            &setup,
        );
    }

    /// `WScriptFn_AddMultiSplineModifier`: first call (static init) and
    /// later calls, 0..6 parameter entries (float-typed slots included), the
    /// instance found by the script manager (id -1) or the world cache
    /// (section missing, tag zero, found), no/some node, `isMoveNode` false
    /// (the real leaf 0x1406c0) or true (a fake), and `addSpline` faked.
    #[test]
    fn wscript_add_multi_spline_modifier_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = oc1490_machine(c, seed, 0xe0c0, 2);
            let (out, params) = (DATA + 0x100, DATA + 0x200);
            let bits = r.next_u32();
            let n = [0i32, 1, 2, 4, 6, -1][(seed % 6) as usize];
            c.g[A0][0] = out as u64;
            c.g[A1][0] = n as i64 as u64;
            c.g[A2][0] = params as u64;
            // The static block's first-call flag, the block itself, and the
            // expected-argument-type table.
            c.bus.write32(GAME_GP + 0x2528, seed & 1);
            let stat = 0x004f_b7a8;
            fill(c, &mut r, stat, 13);
            if bits & 1 != 0 {
                c.bus.write32(stat, 0xffff_ffff);
            } else {
                c.bus.write32(stat, 0x507);
            }
            if bits & 2 != 0 {
                c.bus.write32(stat + 4, 0xffff_ffff);
            }
            for i in 0..13 {
                let t = r.next_u32() % 4;
                c.bus.write32(0x0044_6208 + 4 * i, t);
            }
            for i in 0..n.max(0) as u32 {
                let e = params + 16 * i;
                let sel = r.next_u32();
                let idx = match sel % 6 {
                    0 => 0,
                    1 => 1,
                    _ => (sel >> 8) % 13,
                };
                let val = if idx <= 1 && sel & 0x100 != 0 {
                    0xffff_ffff
                } else if idx == 0 {
                    0x507
                } else {
                    r.next_u32()
                };
                let junk = r.next_u32();
                c.bus.write32(e, idx);
                c.bus.write32(e + 4, val);
                c.bus.write32(e + 8, junk);
                c.bus.write32(e + 0xc, (sel >> 16) % 4);
            }
            // The world: cache -> section table -> section -> tag table.
            let (world, cache, sections, section, tags) = (
                DATA + 0x1_0000,
                DATA + 0x1_0100,
                DATA + 0x1_0200,
                DATA + 0x1_0300,
                DATA + 0x1_0400,
            );
            let (inst, node, vt) = (DATA + 0x1_1000, DATA + 0x1_2000, DATA + 0x1_2100);
            c.bus.write32(GAME_GP + 0x16c8, world);
            c.bus.write32(world, cache);
            c.bus.write32(cache + 8, sections);
            c.bus
                .write32(sections + 4 * 7, if bits & 4 != 0 { section } else { 0 });
            c.bus.write32(section + 0x1c, tags);
            let tag = if bits & 8 != 0 {
                (inst >> 2) << 8 | 0x2a
            } else {
                0x2a
            };
            c.bus.write32(tags + 4 * 5, tag);
            // The script manager's instance.
            c.bus.write32(GAME_GP + 0xce8, DATA + 0x1_3000);
            c.bus.write32(DATA + 0x1_3000 + 0x290, inst);
            c.bus
                .write32(inst + 0xc, if bits & 16 != 0 { node } else { 0 });
            c.bus.write32(node + 0xc, vt);
            c.bus
                .write16(vt + 0x80, if bits & 32 != 0 { 0x10 } else { 0 });
            let is_move_node = if bits & 64 != 0 {
                fakes::FAKE
            } else {
                0x0014_06c0
            };
            c.bus.write32(vt + 0x84, is_move_node);
            // The nil value copied to the result.
            fill(c, &mut r, 0x004c_9098, 3);
        };
        // Few seeds reach the faked callees, so `covered`'s call-count floor does
        // not apply; `check_reaching` below runs the diff and requires both calls.
        check_reaching(
            ssx3_ee::s10::f_002fe0c0__generated,
            wscript_add_multi_spline_modifier,
            200,
            &[0x002f_e270, 0x002f_e280],
            &setup,
        );
    }

    /// `SNDSTRM_Open`: stream table full or with a free slot, 0..3 requests,
    /// allocation failure (negative slot result), tap stream versus file
    /// stream (creation failing), first open stream (service callback)
    /// versus later ones, every callee faked with unwinds.
    #[test]
    fn sndstrm_open_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = oc1490_machine(c, seed, 0x7450, 16);
            let bits = r.next_u32();
            let (src, buf) = (DATA + 0x2_0000, DATA + 0x3_0000);
            fill(c, &mut r, src, 8);
            // sndgs.maxstreams (a byte) and the stream pointer table.
            c.bus.write8(0x0050_a92e, (bits % 4) as u8);
            for i in 0..4u32 {
                let used = (bits >> (2 + i)) % 3 == 0;
                c.bus
                    .write32(0x0050_a0a0 + 4 * i, if used { 0x1234_0000 + i } else { 0 });
            }
            c.g[A0][0] = src as u64;
            c.g[A1][0] = [0u64, 1, 3][((bits >> 6) % 3) as usize];
            c.g[A2][0] = 0x40;
            c.g[A3][0] = buf as u64;
            c.g[T0][0] = 0x4000;
            c.g[T2][0] = [0u64, 1][((bits >> 8) & 1) as usize];
            if (bits >> 9) % 4 == 0 {
                oc1490_force(c, 12, 0xffff_fff7);
            }
            if (bits >> 11) % 4 == 0 {
                oc1490_force(c, 13, 0);
            }
            if (bits >> 13) % 2 == 0 {
                oc1490_force(c, 15, 0);
            }
        };
        covered(
            ssx3_ee::s13::f_003b7450__generated,
            sndstrm_open,
            200,
            &setup,
        );
        check_reaching(
            ssx3_ee::s13::f_003b7450__generated,
            sndstrm_open,
            200,
            &[
                0x003b_74ec,
                0x003b_7514,
                0x003b_7528,
                0x003b_7530,
                0x003b_754c,
                0x003b_7564,
                0x003b_7584,
                0x003b_75d0,
                0x003b_75e4,
                0x003b_7638,
                0x003b_764c,
                0x003b_7688,
                0x003b_76a8,
                0x003b_76b4,
            ],
            &setup,
        );
    }

    /// `LensFX_SetDefaultFlares`: a leaf that writes the nine flare records
    /// from globals and the colour table; random constants and a random
    /// previous list.
    #[test]
    fn lensfx_set_default_flares_matches() {
        check(
            ssx3_ee::s09::f_002f43e0__generated,
            lensfx_set_default_flares,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x43e0, 0);
                let lens = DATA + 0x1000;
                fill(c, &mut r, lens, 0x90);
                fill(c, &mut r, GAME_GP - 0x38f0, 12);
                fill(c, &mut r, 0x004f_ab80, 0x80);
                c.g[A0][0] = lens as u64;
            },
        );
    }
}
