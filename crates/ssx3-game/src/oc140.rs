//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path small functions (modifier ctors, AI/boost
//! setup, quadtree flattening, memcard strings, script/movie helpers), each
//! with a differential test against the generated body.
//!
//! The replacements keep their state in the guest registers the originals
//! use (callers may read `v0`/`v1`), follow the call protocol of
//! [`crate::call`] and the event rules of [`crate::events`].

use crate::abi;
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_f0b8, tex_flip_modifier_ctor, "TexFlipModifier_ctor"),
    (0x0012_b698, ai_apply_computer_rider_settings, "AI_ApplyComputerRiderSettings"),
    (0x002c_2088, quad_tree_flatten_node, "QuadTree_FlattenNode"),(0x003a_c800, cwscriptcache_alloc, "cWScriptCache_Alloc"),
    (0x003a_ed40, av_audio_init, "AV_Audio_Init"),
    (0x003b_1fc8, mpeg2_picture_display_extension, "MPEG2_picture_display_extension"),(0x0026_9608, csetupman_host_recv_msg11, "cSetupMan_HostRecvMsg11"),
    (0x0028_a058, sndloadqueue_ctor, "SndLoadQueue_ctor"),(0x0012_41c0, quat_from_euler, "Quat_FromEuler"),(0x003a_8528, cworldcache_free_hull_page, "cWorldCache_FreeHullPage"),
    (0x0015_43e0, bemissionint_count_completed_on_peak, "BEMissionInt_CountCompletedOnPeak"),
    (0x0018_1148, cfestate_charselect_init, "cFEStateCharSelect_Init"),
    (0x0039_ac48, uimenu_find_prev_selectable, "UIMenu_FindPrevSelectable"),
    (0x0027_5dd8, scriptlist_add, "ScriptList_Add"),(0x0016_0130, camera_wobble_update_roll, "CameraWobble_UpdateRoll"),(0x0019_9148, cfestate_charequip_dtor, "cFEStateCharEquip_dtor"),// OVERRIDES_END
];

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `lw` (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// `sra` (arithmetic), then `andi 1`.
#[inline(always)]
fn bit(v: u64, n: u32) -> u64 {
    (((v as u32) as i32 >> n) as u32 & 1) as u64
}

/// `srl`, then `andi 1`.
#[inline(always)]
fn ubit(v: u64, n: u32) -> u64 {
    ((v as u32) >> n & 1) as u64
}

/// `lq reg, src; sq reg, dst` (a quadword copied through a GPR).
#[inline(always)]
fn qcopy(c: &mut Cpu, reg: usize, src: u32, dst: u32) {
    c.lq(reg, src);
    c.sq(reg, dst);
}

// ---------------------------------------------------------------------------
// Functions.

/// `TexFlipModifier_ctor(self, desc, owner)` (0x35f0b8): a texture-flip
/// modifier over a frame sequence: store the owner, the description's first
/// frame, period, frame count and scale, derive the playback rate from the
/// description's rate and the global frame time, choose a random start frame
/// when the description asks for a negative first frame, then initialise the
/// frame table.
pub fn tex_flip_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        let desc = abi::arg(c, 1);
        c.g[V1][0] = lw(c, gp(c, 0x2a74)); // D_004A5B64 (the clock)
        frame.save(c, 16, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(p(this, 0x18), c.g[A2][0]);
        c.sw(p(this, 0x14), 0);
        c.sw(p(this, 0x1c), 0);
        c.sw(p(this, 0x20), 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20); // f20 = 1.0
        let clock = c.g[V1][0] as u32;
        c.g[V0][0] = lw(c, p(desc, 4));
        c.sw(this, c.g[V0][0]);
        // rate = desc->rate / (float)clock->frameTicks
        c.lwc1(0, p(clock, 0x10));
        c.cvt_s_w(0, 0);
        c.lwc1(1, p(desc, 0x10));
        c.div_s(0, 20, 0);
        c.mul_s(1, 1, 0);
        c.swc1(1, p(this, 8));
        c.g[V0][0] = lw(c, p(desc, 0xc));
        c.swc1(1, p(this, 0xc));
        c.sw(p(this, 4), 0);
        c.sw(p(this, 0x10), c.g[V0][0]);
        c.lwc1(1, p(this, 4));
        c.g[V0][0] = lw(c, p(desc, 8));
        c.mtc1(2, 0); // f0 = frame count
        c.cvt_s_w(0, 0);
        c.c_lt_s(0, 1);
        if !c.fcc() {
            // the frame count is in range: start at the description's frame
            c.sw(p(this, 0x14), c.g[V0][0]);
        } else {
            // a negative start frame means "random": 1..=8 frames of jitter
            c.cycles += 18;
            call_ptr(c, 0x0031_77f0, 0x0035_f140)?; // BXrand()
            c.g[V1][0] = 0x7f_0000;
            c.g[A1][0] = 0x3f80_0000;
            c.g[V1][0] |= 0xffff;
            c.g[AT][0] = 0x4100_0000;
            c.mtc1(1, 1); // f1 = 8.0
            c.g[V0][0] &= c.g[V1][0];
            c.g[V0][0] |= c.g[A1][0];
            c.mtc1(2, 2); // f2 = 1.0 + rand/2^23
            c.sub_s(0, 2, 20);
            c.mul_s(0, 0, 1);
            c.cvt_w_s(1, 0);
            c.cvt_s_w(1, 1);
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.add_s(1, 1, 20);
            }
            c.cycles += 2;
            c.cvt_w_s(0, 1);
            c.swc1(0, p(this, 0x14));
        }
        c.cycles += 8;
        c.g[RA][0] = sx(0x0035_f190);
        c.g[A0][0] = c.g[S0][0];
        c.pc = 0x0035_f218;
        ee::s12::TexFlipModifier_Init(c);
        if c.pc != 0x0035_f190 {
            return Err(Unwound);
        }
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `AI_ApplyComputerRiderSettings(self, rider)` (0x12b698): apply the AI
/// options to a computer rider. The boost setting comes from option bits 3,
/// 0 and 5 (3 disables, 0 zeroes, 5 lowers it, else 1); then the boost meter
/// is reset, the boost time and boost enabling follow bits 3 and 1, and the
/// rider's handicap is stored last.
pub fn ai_apply_computer_rider_settings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let rider = abi::arg(c, 1);
        c.g[V0][0] = 0x53_0000;
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        c.g[S1][0] = c.g[V0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
        c.g[S0][0] = c.g[A1][0];
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x18));
        c.g[V0][0] = bit(c.g[V1][0], 3) as u64;
        c.g[A1][0] = 1;
        if c.g[V0][0] == 0 {
            // option bit 3 clear: test bit 0 of the same word
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & 1;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[V1][0] = 2;
                c.g[V0][0] = ubit(lw(c, p(c.g[S1][0] as u32, 0x8b8)), 5);
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = c.g[V1][0];
                }
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0;
            }
        } else {
            c.cycles += 2;
            c.g[A1][0] = 3;
        }
        c.cycles += 8;
        let state = lw(c, p(rider, 0x18));
        c.g[A0][0] = state;
        c.sw(p(state as u32, 0x304), c.g[A1][0]);
        Callee { addr: 0x0012_0d58, f: ee::s00::Rider_ResetBoostMeter }.call(c, 0x0012_b704)?;
        c.g[V0][0] = ubit(lw(c, p(c.g[S1][0] as u32, 0x8b8)), 3);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8b8));
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = lw(c, p(rider, 0x18));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.swc1(0, p(c.g[V0][0] as u32, 0xb24));
        }
        c.cycles += 5;
        c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0x18));
        c.g[V0][0] = bit(c.g[V0][0], 4) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = 2;
        if !t {
            c.cycles += 2;
            c.g[V1][0] = lw(c, p(rider, 0x18));
            c.sw(p(c.g[V1][0] as u32, 0xb28), c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0x18));
        c.g[V0][0] = bit(c.g[V0][0], 1) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = lw(c, p(rider, 0x18));
            c.g[A1][0] = 1;
            Callee { addr: 0x0012_5958, f: ee::s00::Rider_SetBoostEnabled }.call(c, 0x0012_b764)?;
        }
        c.cycles += 9;
        call_ptr(c, 0x0014_f7e8, 0x0012_b76c)?;
        c.g[V1][0] = lw(c, p(rider, 0x18));
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        c.sw(p(c.g[V1][0] as u32, 0xb30), c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `QuadTree_FlattenNode(self, out, count, map, node)` (0x2c2088): emit a
/// quadtree node into the flat array (`out[*count] = {child, payload}`),
/// record its index in `map` (`(node - self->base) >> 3`), then recurse into
/// the four children when the node's first child word has bit 0 set.
pub fn quad_tree_flatten_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x70);
        let this = abi::arg(c, 0);
        let out = abi::arg(c, 1);
        let count = abi::arg(c, 2);
        let map = abi::arg(c, 3);
        frame.save(c, 18, 0x40);
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 20, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, 21, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, 16, 0x60);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, 17, 0x50);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, count);
        c.g[A1][0] = lw(c, p(c.g[T0][0] as u32, 4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
        c.sw(p(c.g[V0][0] as u32, 4), c.g[A1][0]);
        c.g[V1][0] = lw(c, count);
        c.g[A0][0] = lw(c, c.g[T0][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
        c.sw(c.g[V1][0] as u32, c.g[A0][0]);
        c.g[V0][0] = lw(c, p(this, 0x20));
        c.g[V1][0] = lw(c, count);
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
        c.g[V0][0] &= 0xffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.sw(count, c.g[V1][0]);
        c.g[V0][0] = c.lhu(c.g[T0][0] as u32);
        c.g[V0][0] &= 1;
        c.g[S0][0] = c.g[T0][0];
        if c.g[V0][0] != 0 {
            // four children at base + (childWord >> 1)
            c.cycles += 2;
            c.g[S1][0] = 3;
            c.g[T0][0] = c.lhu(c.g[S0][0] as u32);
            loop {
                c.cycles += 13;
                c.g[A0][0] = c.g[S3][0];
                c.g[V0][0] = lw(c, p(c.g[S3][0] as u32, 0x20));
                c.g[A1][0] = c.g[S4][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32) >> 1);
                c.g[A2][0] = c.g[S2][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32) << 3);
                c.g[A3][0] = c.g[S5][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                c.g[RA][0] = sx(0x002c_2148);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                c.pc = 0x002c_2088;
                quad_tree_flatten_node(c);
                if c.pc != 0x002c_2148 {
                    return Err(Unwound);
                }
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                if (c.g[S1][0] as i64) < 0 {
                    break;
                }
                c.g[T0][0] = c.lhu(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x002c_2120);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore(c, 20, 0x20);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cPS2MemCard_GetFreeSpaceString(self, idx, dst, port)` (0x2c53e0): format
/// the free space of memory-card entry `idx` into the wide string `dst`.
/// No entry data, a status of -2 or an unknown status produce the "no card"
/// message; status 0 formats the free-space number through the file-system
/// object's size formatter.
pub fn cps2memcard_get_free_space_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x14;
        // `a3 = idx * sizeof(entry)` (imul rd=a3, rs=a1, rt=20)
        c.mult(0, A3, A1, V0);
        frame.save(c, 19, 0x10);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x188));
        let no_data = c.g[V1][0] == 0;
        c.g[S3][0] = c.g[A2][0];
        // 0: no data, 1: status -2, 2: status 0, 3: any other status
        let mut which = 0u32;
        if !no_data {
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.g[V1][0] = sx(0xffff_fffe);
            c.g[V0][0] = lw(c, p(c.g[V0][0] as u32, 0x184));
            if c.g[V0][0] == c.g[V1][0] {
                c.g[A1][0] = 0x4a_0000;
                which = 1;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    which = 2;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = 0x4a_0000;
                    which = 3;
                }
            }
        }
        if which == 0 {
            c.cycles += 1;
            c.g[A1][0] = 0x4a_0000;
        }
        if which == 2 {
            // status 0: the free-space number, formatted by the file system
            c.cycles += 23;
            c.g[RA][0] = sx(0x002c_5454);
            c.g[S2][0] = 0;
            c.pc = 0x002c_5320;
            ee::s08::cPS2MemCard_GetFreeSpaceForPort(c);
            if c.pc != 0x002c_5454 {
                return Err(Unwound);
            }
            c.g[A1][0] = lw(c, gp(c, -0x848));
            c.g[V1][0] = sx(0xffff_ffff);
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A0][0] = 0x48_0000;
            c.g[S0][0] = lw(c, p(c.g[A1][0] as u32, 0x8c));
            if c.g[V1][0] != 0 {
                c.g[S2][0] = c.g[V0][0];
            }
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x61f8));
            c.g[S1][0] = lw(c, p(c.g[S0][0] as u32, 4));
            c.g[V0][0] = c.lh(p(c.g[S1][0] as u32, 0x20));
            c.g[RA][0] = sx(0x002c_5480);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            call_ptr(c, 0x0031_7670, 0x002c_5480)?; // StrHash(D_004861F8)
            c.g[V1][0] = lw(c, p(c.g[S1][0] as u32, 0x24));
            c.g[A0][0] = c.g[S0][0];
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x002c_5490);
            c.g[A1][0] = c.g[V0][0];
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x002c_5490 {
                return Err(Unwound);
            }
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002c_54a0);
            c.g[A2][0] = c.g[S2][0];
            c.pc = 0x002c_26d0;
            ee::s08::UStr_Sprintf(c);
            if c.pc != 0x002c_54a0 {
                return Err(Unwound);
            }
        } else {
            // "no card" (status -2, or no data) and unknown statuses
            let (cost, ret) = if which == 1 {
                (5, 0x002c_5444)
            } else {
                (4, 0x002c_54b4)
            };
            c.cycles += cost;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(ret);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3968));
            c.pc = 0x002c_2540;
            ee::s08::UStr_FromAscii_2(c);
            if c.pc != ret {
                return Err(Unwound);
            }
        }
        c.lq(16, p(sp(c), 0x40));
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cWScriptCache_Alloc(self, count)` (0x3ac800): resize the world script
/// cache's block array to `count` entries: destroy and release the old
/// array, allocate a new header and array, construct the entries, then
/// publish it and clear the used count.
pub fn cwscriptcache_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        let this = abi::arg(c, 0);
        let count = abi::arg(c, 1);
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0x60);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, 20, 0x20);
        frame.save(c, 21, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw(c, p(c.g[S2][0] as u32, 4));
        let t = c.g[A1][0] == 0;
        c.g[S1][0] = 0x49_0000;
        if !t {
            // destroy the old array back to front, then free its header
            c.cycles += 6;
            c.g[V1][0] = lw(c, (c.g[A1][0] as u32).wrapping_sub(0x10));
            c.g[V0][0] = 0xc;
            c.mult(0, A0, V1, V0); // a0 = old_count * 12
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = lw(c, p(c.g[S2][0] as u32, 4));
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[RA][0] = sx(0x003a_c85c);
                    c.g[A0][0] = c.g[S0][0];
                    c.pc = 0x003a_ca70;
                    ee::s13::tWScriptCache_dtor(c);
                    if c.pc != 0x003a_c85c {
                        return Err(Unwound);
                    }
                    c.g[V0][0] = lw(c, p(c.g[S2][0] as u32, 4));
                    if c.g[V0][0] != c.g[S0][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_c850);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = lw(c, p(c.g[S2][0] as u32, 4));
                    break;
                }
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x003a_c874);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            c.pc = 0x0031_7e98;
            call_ptr(c, 0x0031_7e98, 0x003a_c874)?; // operator delete
        }
        // allocate and construct the new array
        c.cycles += 14;
        c.g[A0][0] = 0xc;
        c.sw(p(c.g[S2][0] as u32, 8), c.g[S3][0]);
        c.mult(0, A0, S3, A0); // a0 = count * 12
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5070));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
        c.g[RA][0] = sx(0x003a_c89c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        c.pc = 0x0031_7e30;
        call_ptr(c, 0x0031_7e30, 0x003a_c89c)?; // operator new
        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sw(c.g[V0][0] as u32, c.g[S3][0]);
        let t = c.g[S3][0] == 0;
        c.g[S0][0] = c.g[S4][0];
        if !t {
            c.cycles += 1;
            c.g[S3][0] = sx(0xffff_ffff);
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003a_c8bc);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                c.pc = 0x003a_ca50;
                ee::s13::tWScriptCache_ctor(c);
                if c.pc != 0x003a_c8bc {
                    return Err(Unwound);
                }
                let t = c.g[S1][0] != c.g[S3][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_c8b0);
                }
            }
        }
        c.cycles += 11;
        c.sw(c.g[S5][0] as u32, c.g[S4][0]);
        c.sw(p(c.g[S2][0] as u32, 0xc), 0);
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore(c, 20, 0x20);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AV_Audio_Init(self, a1, a2)` (0x3aed40): prepare an audio stream cache
/// entry. If the stream module is not up yet, just poke the stream system
/// and return `self`; otherwise clear the entry, build the play parameters,
/// size and open the stream, queue the file, and mark the entry ready.
pub fn av_audio_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let this = abi::arg(c, 0);
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 16, 0x60);
        c.sd(p(sp(c), 0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x003a_ed68);
        c.g[S3][0] = c.g[A2][0];
        c.pc = 0x003b_5440;
        ee::s13::SND_IsInitialized(c);
        if c.pc != 0x003a_ed68 {
            return Err(Unwound);
        }
        let ready = c.g[V0][0] != 0;
        c.g[V0][0] = sx(0xffff_ffff);
        if !ready {
            c.cycles += 4;
            call_ptr(c, 0x003e_62b0, 0x003a_ed78)?;
            c.g[V0][0] = c.g[S1][0];
        } else {
            c.cycles += 37;
            c.sw(p(c.g[S1][0] as u32, 4), 0);
            c.g[A0][0] = c.g[SP][0];
            c.sw(p(c.g[S1][0] as u32, 0xc), c.g[V0][0]);
            c.sw(p(c.g[S1][0] as u32, 0x10), c.g[V0][0]);
            c.g[RA][0] = sx(0x003a_ed94);
            c.pc = 0x003b_a020;
            ee::s13::SND_InitPlayParams(c);
            if c.pc != 0x003a_ed94 {
                return Err(Unwound);
            }
            c.g[A0][0] = 1;
            c.g[RA][0] = sx(0x003a_eda0);
            c.g[A1][0] = 0x1e;
            c.pc = 0x003b_7ac8;
            ee::s13::SNDSTRM_CalcMemSize(c);
            if c.pc != 0x003a_eda0 {
                return Err(Unwound);
            }
            c.g[V1][0] = 0x51_0000;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
            c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_sub(0x6bd0));
            c.g[T0][0] = lw(c, p(c.g[A0][0] as u32, 8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = 0x49_0000;
            c.g[A2][0] = 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5250));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003a_edcc);
            c.g[A3][0] = 0;
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x003a_edcc {
                return Err(Unwound);
            }
            c.g[A1][0] = c.g[SP][0];
            c.g[T1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S2][0];
            c.g[T0][0] = c.g[V0][0];
            c.g[A2][0] = 1;
            c.g[A3][0] = 0x1e;
            c.g[RA][0] = sx(0x003a_edec);
            c.sw(p(c.g[S1][0] as u32, 4), c.g[V0][0]);
            c.pc = 0x003b_7e38;
            ee::s13::SNDSTRM_OpenEx(c);
            if c.pc != 0x003a_edec {
                return Err(Unwound);
            }
            c.g[A2][0] = c.g[S3][0];
            c.g[A0][0] = c.g[V0][0];
            c.sw(p(c.g[S1][0] as u32, 0xc), c.g[V0][0]);
            c.g[RA][0] = sx(0x003a_ee00);
            c.g[A1][0] = sx(0xffff_ffff);
            c.pc = 0x003b_7de8;
            ee::s13::SNDSTRM_QueueFile(c);
            if c.pc != 0x003a_ee00 {
                return Err(Unwound);
            }
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.sw(p(c.g[S1][0] as u32, 0x10), c.g[V0][0]);
            c.swc1(0, c.g[S1][0] as u32);
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        c.g[RA][0] = c.ld(p(sp(c), 0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `MPEG2_picture_display_extension(&bits)` (0x3b1fc8): read the picture
/// display extension: three picture-level fields and two marker bits per
/// picture, according to the state of the MPEG2 decoder globals, and return
/// how many pictures were read.
pub fn mpeg2_picture_display_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V0][0] = 0x51_0000;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = lw(c, (c.g[V0][0] as u32).wrapping_sub(0x6adc));
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        frame.save(c, 19, 0x20);
        frame.save(c, 20, 0x10);
        let t = c.g[V1][0] == 0;
        frame.save_ra(c, 0);
        let mut count;
        if t {
            // no decoder state: one picture
            c.cycles += 5;
            c.g[V0][0] = 0x51_0000;
            c.g[V1][0] = 3;
            c.g[A0][0] = lw(c, (c.g[V0][0] as u32).wrapping_sub(0x6a7c));
            let t = c.g[A0][0] == c.g[V1][0];
            c.g[V1][0] = 0x51_0000;
            if t {
                c.cycles += 3;
                c.g[A1][0] = 2;
                c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_sub(0x6a68));
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = c.g[A0][0];
                }
            } else {
                c.cycles += 2;
                c.g[A1][0] = 1;
            }
            count = c.g[A1][0];
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x51_0000;
            c.g[V1][0] = lw(c, (c.g[V0][0] as u32).wrapping_sub(0x6a68));
            let t = c.g[V1][0] == 0;
            c.g[V0][0] = 0x51_0000;
            if t {
                c.cycles += 2;
                c.g[A1][0] = 1;
                count = c.g[A1][0];
            } else {
                c.cycles += 5;
                c.g[A1][0] = 2;
                c.g[A0][0] = lw(c, (c.g[V0][0] as u32).wrapping_sub(0x6a78));
                c.g[V1][0] = 3;
                if c.g[A0][0] != 0 {
                    c.g[A1][0] = c.g[V1][0];
                }
                count = c.g[A1][0];
            }
        }
        c.cycles += 2;
        let t = (c.g[A1][0] as i64) <= 0;
        c.g[V1][0] = 0x51_0000;
        if !t {
            c.cycles += 7;
            c.g[V0][0] = 0x51_0000;
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6a30));
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6a40));
            c.g[S0][0] = count;
            c.g[S4][0] = 0x49_0000;
            c.g[S3][0] = 0x49_0000;
            loop {
                c.cycles += 14;
                c.g[A0][0] = 0x10;
                c.g[RA][0] = sx(0x003b_206c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                c.pc = 0x003b_1300;
                ee::s13::MPEG2_Get_Bits(c);
                if c.pc != 0x003b_206c {
                    return Err(Unwound);
                }
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                c.g[RA][0] = sx(0x003b_2078);
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5538));
                c.pc = 0x003b_2360;
                ee::s13::MPEG2_marker_bit(c);
                if c.pc != 0x003b_2078 {
                    return Err(Unwound);
                }
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                c.g[RA][0] = sx(0x003b_2084);
                c.g[A0][0] = 0x10;
                c.pc = 0x003b_1300;
                ee::s13::MPEG2_Get_Bits(c);
                if c.pc != 0x003b_2084 {
                    return Err(Unwound);
                }
                c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                c.g[RA][0] = sx(0x003b_2090);
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5568));
                c.pc = 0x003b_2360;
                ee::s13::MPEG2_marker_bit(c);
                if c.pc != 0x003b_2090 {
                    return Err(Unwound);
                }
                let t = c.g[S0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_2060);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `get_cCTBoundObjBox(reader, obj)` / `get_cCTVolumeBox(reader, obj)`
/// (0x171b10 / 0x1730d0): read a camera trigger box from the reader: the
/// box object's centre and axis vectors and its three size floats, through
/// the object's vtable, summing the consumed floats.
fn read_camera_box(c: &mut Cpu, ret: &[u32; 10]) -> crate::call::Flow {
    c.cycles += 60;
    let frame = Frame::open(c, 0x40);
    c.g[V0][0] = 1;
    frame.save(c, 16, 0x30);
    frame.save(c, 17, 0x20);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, 18, 0x10);
    frame.save_ra(c, 0);
    c.g[S2][0] = c.g[A0][0];
    c.sw(p(c.g[S0][0] as u32, 0x28), c.g[V0][0]);
    // slot 1 (centre vector), then slot 5 (axis vector)
    for (i, off) in [(0usize, 8u32), (2, 0x28)] {
        let vt = lw(c, p(c.g[S0][0] as u32, 0x24));
        c.g[V1][0] = vt;
        c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, off));
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, off + 4));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(ret[i]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        ssx3_ee_rt::funcs::call_indirect(c, t);
        if c.pc != ret[i] {
            return Err(Unwound);
        }
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(ret[i + 1]);
        c.g[A1][0] = c.g[V0][0];
        c.pc = 0x002d_25a0;
        call_ptr(c, 0x002d_25a0, ret[i + 1])?;
        if i == 0 {
            c.g[S1][0] = c.g[V0][0];
        } else {
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        }
    }
    // slots 2, 3, 4: the three size floats
    for (i, off) in [(4usize, 0x10u32), (6, 0x18), (8, 0x20)] {
        let vt = lw(c, p(c.g[S0][0] as u32, 0x24));
        c.g[V1][0] = vt;
        c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, off));
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, off + 4));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(ret[i]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        ssx3_ee_rt::funcs::call_indirect(c, t);
        if c.pc != ret[i] {
            return Err(Unwound);
        }
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(ret[i + 1]);
        c.g[A1][0] = c.g[V0][0];
        c.pc = 0x002d_2510;
        call_ptr(c, 0x002d_2510, ret[i + 1])?;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    }
    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    frame.restore(c, 16, 0x30);
    frame.restore(c, 17, 0x20);
    frame.restore(c, 18, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `get_cCTBoundObjBox(reader, obj)` (0x171b10).
pub fn camera_box_bound_obj_read(c: &mut Cpu) {
    guest(c, |c| {
        read_camera_box(
            c,
            &[
                0x0017_1b48, 0x0017_1b54, 0x0017_1b6c, 0x0017_1b78, 0x0017_1b90, 0x0017_1b9c,
                0x0017_1bb4, 0x0017_1bc0, 0x0017_1bd8, 0x0017_1be4,
            ],
        )
    })
}

/// `get_cCTVolumeBox(reader, obj)` (0x1730d0).
pub fn camera_trigger_read_box_trigger_data(c: &mut Cpu) {
    guest(c, |c| {
        read_camera_box(
            c,
            &[
                0x0017_3108, 0x0017_3114, 0x0017_312c, 0x0017_3138, 0x0017_3150, 0x0017_315c,
                0x0017_3174, 0x0017_3180, 0x0017_3198, 0x0017_31a4,
            ],
        )
    })
}

/// `cSetupMan_HostRecvMsg11(self, msg)` (0x269608): handle message type 11
/// from the host: normalise the message type (6..7 -> 8, 9 -> 10), forward
/// it to the character setup as message 11, then build a two-node packet
/// entry (type 0xfd, the default handler) and splice it into the setup
/// manager's list.
pub fn csetupman_host_recv_msg11(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        let msg = abi::arg(c, 0);
        frame.save(c, 16, 0x30);
        c.sd(p(sp(c), 0x20), c.g[RA][0]);
        c.g[V1][0] = lw(c, msg);
        c.g[V0][0] = ((c.g[V1][0] as i64) < 6) as u64;
        if c.g[V0][0] != 0 {
            c.g[A2][0] = lw(c, msg);
        } else if ((c.g[V1][0] as i64) < 8) {
            c.cycles += 3;
            c.g[V0][0] = 8;
            c.cycles += 2;
            c.sw(msg, c.g[V0][0]);
            c.g[A2][0] = lw(c, msg);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 9;
            if c.g[V1][0] == c.g[V0][0] {
                c.g[V0][0] = 0xa;
                c.cycles += 2;
                c.sw(msg, c.g[V0][0]);
                c.g[A2][0] = lw(c, msg);
            } else {
                c.cycles += 2;
                c.g[A2][0] = lw(c, msg);
            }
        }
        c.cycles += 30;
        c.g[A1][0] = 0xb;
        c.g[RA][0] = sx(0x0026_9658);
        c.g[A3][0] = 0;
        c.pc = 0x0026_4e50;
        ee::s07::cSetupMan_SendCharacterMsg(c);
        if c.pc != 0x0026_9658 {
            return Err(Unwound);
        }
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = 0xc;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf18));
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(0x0026_9670);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_9670)?; // operator new(0xc, ...)
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0xfd;
        c.g[V0][0] = 0x48_0000;
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x13e0));
        c.g[A0][0] = 1;
        c.sw(p(c.g[S0][0] as u32, 8), c.g[A0][0]);
        c.g[A1][0] = 0x48_0000;
        c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.g[V1][0] = lw(c, gp(c, -0xc8));
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0xf0));
        c.g[RA][0] = sx(0x0026_96b4);
        c.sw(p(sp(c), 0x10), c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_96b4)?; // operator new(0xc, ...)
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw(p(c.g[A1][0] as u32, 8), c.g[S0][0]);
        }
        c.cycles += 13;
        c.g[V0][0] = lw(c, p(sp(c), 0x10));
        frame.restore(c, 16, 0x30);
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        c.g[RA][0] = c.ld(p(sp(c), 0x20));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 4));
        c.sw(p(c.g[A1][0] as u32, 4), c.g[V1][0]);
        c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 4));
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = lw(c, p(sp(c), 0x10));
        c.sw(p(c.g[V0][0] as u32, 4), c.g[A1][0]);
        c.sw(sp(c), c.g[A1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `SndLoadQueue_ctor(self)` (0x28a058): five 0x18-byte queue nodes with the
/// same vtable, linked into a count-6 ring, then publish the queue in the
/// sound system's global.
pub fn sndloadqueue_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 60;
        let frame = Frame::open(c, 0x40);
        let this = abi::arg(c, 0);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, 16, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3b10));
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[S2][0] = 6;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2820));
        c.sw(p(c.g[S0][0] as u32, 0x18c), c.g[S2][0]);
        c.sw(p(c.g[S0][0] as u32, 4), 0);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(c.g[S0][0] as u32, 8), 0);
        c.g[A0][0] = 0x18;
        c.sw(p(c.g[S0][0] as u32, 0x194), 0);
        c.g[A2][0] = 0;
        c.sw(p(c.g[S0][0] as u32, 0x198), 0);
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0028_a0b0);
        c.sw(p(c.g[S0][0] as u32, 0x1d4), c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x0028_a0b0)?; // operator new(0x18, ...)
        c.sw(p(c.g[S0][0] as u32, 0x19c), c.g[S2][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(c.g[S0][0] as u32, 0x1a4), 0);
        c.g[A0][0] = 0x18;
        c.sw(p(c.g[S0][0] as u32, 0x1a8), 0);
        c.g[A2][0] = 0;
        c.sw(p(c.g[S0][0] as u32, 0x190), c.g[V0][0]);
        c.g[RA][0] = sx(0x0028_a0d4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_a0d4)?;
        c.sw(p(c.g[S0][0] as u32, 0x1ac), c.g[S2][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(c.g[S0][0] as u32, 0x1b4), 0);
        c.g[A0][0] = 0x18;
        c.sw(p(c.g[S0][0] as u32, 0x1b8), 0);
        c.g[A2][0] = 0;
        c.sw(p(c.g[S0][0] as u32, 0x1a0), c.g[V0][0]);
        c.g[RA][0] = sx(0x0028_a0f8);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_a0f8)?;
        c.sw(p(c.g[S0][0] as u32, 0x1bc), c.g[S2][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(c.g[S0][0] as u32, 0x1c4), 0);
        c.g[A0][0] = 0x18;
        c.sw(p(c.g[S0][0] as u32, 0x1c8), 0);
        c.g[A2][0] = 0;
        c.sw(p(c.g[S0][0] as u32, 0x1b0), c.g[V0][0]);
        c.g[RA][0] = sx(0x0028_a11c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_a11c)?;
        c.sw(p(c.g[S0][0] as u32, 0x1c0), c.g[V0][0]);
        c.sw(p(c.g[S0][0] as u32, 0x1cc), 0);
        c.g[V0][0] = c.g[S0][0];
        c.sw(p(c.g[S0][0] as u32, 0x1d0), 0);
        c.sw(gp(c, 0x520), c.g[S0][0]);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BoltFile_Release(self, slot, layer)` (0x19dc20): drop one reference on a
/// bolt-file slot's entry; at zero either clear the slot (when it is not in
/// the loading state) or unpack it and free the slot.
pub fn boltfile_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        let this = abi::arg(c, 0);
        let slot = abi::arg(c, 1);
        let layer = abi::arg(c, 2);
        c.g[A3][0] = 0x1_0000;
        frame.save(c, 18, 0x30);
        c.g[A3][0] |= 0x2000;
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.mult(0, V0, S2, A3); // v0 = slot * 0x12000
        c.g[S3][0] = c.g[A2][0];
        c.g[V1][0] = 0x120;
        frame.save(c, 17, 0x40);
        c.mult(0, V1, S3, V1); // v1 = layer * 0x120
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x50);
        c.sd(p(sp(c), 0x10), c.g[RA][0]);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
        c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        let entry = c.g[S0][0] as u32;
        c.g[V0][0] = lw(c, p(entry, 0x114));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let t = (c.g[V0][0] as i64) > 0;
        c.sw(p(entry, 0x114), c.g[V0][0]);
        let mut skip_one = false;
        if !t {
            c.cycles += 4;
            c.g[V1][0] = lw(c, entry);
            c.g[V0][0] = 2;
            let mut go = true;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = lw(c, p(entry, 0x104));
            } else {
                c.cycles += 5;
                call_ptr(c, 0x003d_f980, 0x0019_dc88)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] != c.g[V1][0] {
                    skip_one = true;
                    go = false;
                } else {
                    c.cycles += 16;
                    c.g[A0][0] = lw(c, p(entry, 0x108));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0019_dca4);
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x003d_f9d8, 0x0019_dca4)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0019_dcb4);
                    c.g[A2][0] = c.g[S0][0];
                    c.pc = 0x0019_d000;
                    ee::s03::BoltFile_UnpackSlot(c);
                    if c.pc != 0x0019_dcb4 {
                        return Err(Unwound);
                    }
                    c.sw(p(c.g[S0][0] as u32, 0x10c), 0);
                    c.g[V1][0] = 0xb_0000;
                    c.sw(c.g[S0][0] as u32, 0);
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x4004));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.sw(p(c.g[V1][0] as u32, 0x4004), c.g[V0][0]);
                    c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x104));
                }
            }
            if go {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(c.g[S0][0] as u32, 0);
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0019_dcec);
                    c.g[A2][0] = c.g[S3][0];
                    c.pc = 0x0019_d4b8;
                    ee::s03::BoltFile_FreeSlot(c);
                    if c.pc != 0x0019_dcec {
                        return Err(Unwound);
                    }
                }
            }
        }
        if !skip_one {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x50);
        c.cycles += 6;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `Quat_FromEuler(out, x, y, z)` (0x1241c0): build a quaternion from three
/// Euler angles (half-angle sines and cosines through `Math_SinCos`, then the
/// standard product/sum combinations) and store it in `out`.
pub fn quat_from_euler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 60;
        let frame = Frame::open(c, 0x70);
        let out = abi::arg(c, 1);
        c.swc1(20, p(sp(c), 0x50));
        c.g[A0][0] = sx(p(sp(c), 0x10));
        c.swc1(22, p(sp(c), 0x60));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 20); // f20 = 0.5
        c.mov_s(22, 12);
        frame.save(c, 16, 0x40);
        c.mul_s(12, 14, 20);
        c.swc1(21, p(sp(c), 0x58));
        c.g[S0][0] = c.g[A1][0];
        c.mov_s(21, 13);
        c.sd(p(sp(c), 0x30), c.g[RA][0]);
        c.g[RA][0] = sx(0x0012_41fc);
        c.g[A1][0] = sx(p(sp(c), 0x14));
        c.pc = 0x0031_be50;
        call_ptr(c, 0x0031_be50, 0x0012_41fc)?; // Math_SinCos(sin, cos, z*0.5)
        c.mul_s(12, 21, 20);
        c.g[A0][0] = sx(p(sp(c), 0x18));
        c.g[RA][0] = sx(0x0012_420c);
        c.g[A1][0] = sx(p(sp(c), 0x1c));
        c.pc = 0x0031_be50;
        call_ptr(c, 0x0031_be50, 0x0012_420c)?;
        c.mul_s(12, 22, 20);
        c.g[A0][0] = sx(p(sp(c), 0x20));
        c.g[RA][0] = sx(0x0012_421c);
        c.g[A1][0] = sx(p(sp(c), 0x24));
        c.pc = 0x0031_be50;
        call_ptr(c, 0x0031_be50, 0x0012_421c)?;
        c.lwc1(2, p(sp(c), 0x14));
        c.lwc1(5, p(sp(c), 0x10));
        c.lwc1(1, p(sp(c), 0x1c));
        c.lwc1(6, p(sp(c), 0x24));
        c.lwc1(0, p(sp(c), 0x18));
        c.mul_s(7, 2, 1);
        c.lwc1(4, p(sp(c), 0x20));
        c.mul_s(11, 1, 6);
        c.mul_s(8, 5, 0);
        c.g[RA][0] = c.ld(p(sp(c), 0x30));
        c.mul_s(10, 0, 4);
        c.lwc1(22, p(sp(c), 0x60));
        c.mul_s(0, 2, 0);
        c.lwc1(21, p(sp(c), 0x58));
        c.mul_s(1, 5, 1);
        c.lwc1(20, p(sp(c), 0x50));
        c.mul_s(3, 5, 11);
        c.mul_s(9, 2, 10);
        c.mul_s(7, 7, 4);
        c.mul_s(8, 8, 6);
        c.mul_s(0, 0, 6);
        c.mul_s(1, 1, 4);
        c.mul_s(2, 2, 11);
        c.mul_s(5, 5, 10);
        c.sub_s(3, 3, 9);
        c.add_s(0, 0, 1);
        c.add_s(2, 2, 5);
        c.sub_s(7, 7, 8);
        c.swc1(3, sp(c));
        c.swc1(0, p(sp(c), 4));
        c.swc1(2, p(sp(c), 0xc));
        c.swc1(7, p(sp(c), 8));
        c.lq(2, sp(c));
        c.sq(2, out);
        frame.restore(c, 16, 0x40);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `RailModifier_GetWorldMatrix(self, out)` (0x35c5a0): ask the instance's
/// node matrix, then multiply the rail modifier's transform by it in VU0 and
/// store the result in `out` (also copying it back into the modifier's own
/// matrix).
pub fn railmodifier_get_world_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 61;
        let frame = Frame::open(c, 0xb0);
        let this = abi::arg(c, 0);
        let out = abi::arg(c, 1);
        frame.save(c, 16, 0xa0);
        frame.save(c, 17, 0x90);
        c.g[S0][0] = c.g[A0][0];
        c.sd(p(sp(c), 0x80), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = lw(c, p(this, 0x40));
        c.g[RA][0] = sx(0x0035_c5c8);
        c.g[A1][0] = lw(c, p(this, 0x34));
        c.pc = 0x0034_fed8;
        ee::s11::Instance_GetNodeMatrix(c);
        if c.pc != 0x0035_c5c8 {
            return Err(Unwound);
        }
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = sx(p(sp(c), 0x40));
        c.lqc2(4, out);
        c.lqc2(5, p(out, 0x10));
        c.lqc2(6, p(out, 0x20));
        c.lqc2(7, p(out, 0x30));
        c.lqc2(8, c.g[S0][0] as u32);
        c.lqc2(9, p(c.g[S0][0] as u32, 0x10));
        c.lqc2(10, p(c.g[S0][0] as u32, 0x20));
        c.lqc2(11, p(c.g[S0][0] as u32, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.vu0().mula(0b1111, 4, Src::Bc(9, X));
        c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
        c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
        c.vu0().mula(0b1111, 4, Src::Bc(10, X));
        c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
        c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
        c.vu0().mula(0b1111, 4, Src::Bc(11, X));
        c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
        c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
        c.sqc2(12, c.g[V0][0] as u32);
        c.sqc2(13, p(c.g[V0][0] as u32, 0x10));
        c.sqc2(14, p(c.g[V0][0] as u32, 0x20));
        c.sqc2(15, p(c.g[V0][0] as u32, 0x30));
        c.lqc2(1, c.g[V0][0] as u32);
        c.lqc2(2, p(c.g[V0][0] as u32, 0x10));
        c.lqc2(3, p(c.g[V0][0] as u32, 0x20));
        c.lqc2(4, p(c.g[V0][0] as u32, 0x30));
        c.sqc2(1, sp(c));
        c.sqc2(2, p(sp(c), 0x10));
        c.sqc2(3, p(sp(c), 0x20));
        c.sqc2(4, p(sp(c), 0x30));
        c.lqc2(1, sp(c));
        c.lqc2(2, p(sp(c), 0x10));
        c.lqc2(3, p(sp(c), 0x20));
        c.lqc2(4, p(sp(c), 0x30));
        c.sqc2(1, out);
        c.sqc2(2, p(out, 0x10));
        c.sqc2(3, p(out, 0x20));
        c.sqc2(4, p(out, 0x30));
        frame.restore(c, 16, 0xa0);
        frame.restore(c, 17, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `MEM_ValidateBlock(block)` (0x2522b0): validate an allocated block's
/// magic (`BS`, `BF`, `BM`), its flag word and (for `BM`) the end marker,
/// recording the error code in `gp - 0x27c` and returning 1 when valid.
pub fn mem_validate_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let block = abi::arg(c, 0);
        frame.save(c, 16, 0x10);
        c.g[V0][0] = 0x4253;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = c.lhu(c.g[S0][0] as u32);
        let t = c.g[V1][0] != c.g[V0][0];
        c.g[T0][0] = 0;
        // 0: done without the error store, 1: error code in V0, 2: error
        // code written to gp - 0x27c
        let mut state = 0u32;
        if !t {
            c.cycles += 4;
            c.g[V0][0] = c.lhu(p(c.g[S0][0] as u32, 2));
            c.g[V0][0] &= 0x8000;
            if c.g[V0][0] != 0 {
                c.g[T0][0] = 1;
                state = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 2;
                state = 2;
            }
        }
        if state == 0 {
            // not `BS`: `BF` (free) and `BM` (mixed) blocks
            c.cycles += 3;
            c.g[V0][0] = 0x4246;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0x424d;
            let mut direct = false;
            if !t {
                c.cycles += 4;
                c.g[V0][0] = c.lhu(p(c.g[S0][0] as u32, 2));
                c.g[V0][0] &= 0x4000;
                if c.g[V0][0] == 0 {
                    direct = true;
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x10));
                    call_ptr(c, 0x0025_2248, 0x0025_230c)?;
                    c.g[T0][0] = c.g[V0][0];
                    if c.g[T0][0] == 0 {
                        direct = true;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
                        call_ptr(c, 0x0025_2248, 0x0025_2320)?;
                        c.g[T0][0] = c.g[V0][0];
                        state = 1;
                    }
                }
            } else {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 4;
                if t {
                    state = 2;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu(p(c.g[S0][0] as u32, 2));
                    c.g[V0][0] &= 0x2000;
                    let t = c.g[V0][0] == 0;
                    c.g[A2][0] = 0x4245_0000;
                    if t {
                        c.cycles += 2;
                        c.g[T0][0] = 1;
                        state = 1;
                    } else {
                        c.cycles += 16;
                        c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, 4));
                        c.g[A2][0] |= 0x4e44;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                        c.g[A0][0] = c.lbu(p(c.g[V1][0] as u32, 1));
                        c.g[A1][0] = c.lbu(p(c.g[V1][0] as u32, 2));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
                        c.g[A3][0] = c.lbu(p(c.g[V1][0] as u32, 3));
                        c.g[V0][0] |= c.g[A0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
                        c.g[V0][0] |= c.g[A1][0];
                        c.g[V0][0] |= c.g[A3][0];
                        let t = c.g[V0][0] != c.g[A2][0];
                        c.g[V0][0] = 3;
                        if t {
                            state = 2;
                        } else {
                            c.cycles += 2;
                            c.g[T0][0] = 1;
                            state = 1;
                        }
                    }
                }
            }
            if direct {
                state = 3;
            }
        }
        if state == 2 {
            c.cycles += 1;
            c.sw(gp(c, -0x27c), c.g[V0][0]);
        }
        if state != 3 {
            c.cycles += 1;
        }
        c.cycles += 4;
        c.g[V0][0] = c.g[T0][0];
        frame.restore_ra(c, 0);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `ScriptEngine_CreateScriptDatabase(self, count, a2, a3)` (0x281f30):
/// create the script database: allocate the header, construct the
/// instruction sequence, allocate one 8-byte record per script and clear
/// them.
pub fn scriptengine_create_script_database(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x50);
        let this = abi::arg(c, 0);
        let count = abi::arg(c, 1);
        let a2 = abi::arg(c, 2);
        let size = abi::arg(c, 3);
        frame.save(c, 17, 0x30);
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 18, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f48));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x0028_1f70);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_1f70)?; // operator new(size, ...)
        c.g[A1][0] = 0x4a_0000;
        c.g[A0][0] = 0x20;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x34f0));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0028_1f8c);
        c.sw(p(this, 0x2bc), c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0028_1f8c)?; // operator new(0x20, ...)
        c.g[A1][0] = lw(c, p(this, 0x2bc));
        c.g[T0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2ac));
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0028_1fa4);
        c.g[A3][0] = c.g[S2][0];
        c.pc = 0x0027_2788;
        ee::s07::InSeq_ctor(c);
        if c.pc != 0x0028_1fa4 {
            return Err(Unwound);
        }
        c.sw(p(this, 0x2b4), c.g[S1][0]);
        c.g[A1][0] = 0x48_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f58));
        c.sw(p(this, 0x2b8), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[S1][0] as u32) << 3);
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x0028_1fc4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_1fc4)?; // operator new(count * 8, ...)
        c.g[V1][0] = lw(c, p(this, 0x2b4));
        c.g[A1][0] = 0;
        let t = (c.g[V1][0] as i64) <= 0;
        c.sw(p(this, 0x2b0), c.g[V0][0]);
        if !t {
            c.cycles += 1;
            c.g[V1][0] = lw(c, p(this, 0x2b0));
            loop {
                c.cycles += 11;
                c.g[A0][0] = sx((c.g[A1][0] as u32) << 3);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw(c.g[V1][0] as u32, 0);
                c.g[V0][0] = lw(c, p(this, 0x2b0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sh(p(c.g[A0][0] as u32, 4), 0);
                c.g[V0][0] = lw(c, p(this, 0x2b4));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw(c, p(this, 0x2b0));
                if event_due(c) {
                    deliver_event_here(c, 0x0028_1fd8);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cWorldCache_FreeHullPage(self, page)` (0x3a8528): release every resource
/// of a cached hull page, destroy the page object and clear the cache slot
/// and its page table entry.
pub fn cworldcache_free_hull_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x70);
        let this = abi::arg(c, 0);
        c.g[V0][0] = 0x18;
        frame.save(c, 19, 0x30);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, 20, 0x20);
        c.mult(0, V1, S3, V0); // v1 = page * 0x18
        frame.save(c, 16, 0x60);
        frame.save(c, 17, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, 21, 0x10);
        c.g[S0][0] = sx(0xffff_0000);
        frame.save_ra(c, 0);
        frame.save(c, 18, 0x40);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
        c.g[S2][0] = lw(c, p(c.g[V0][0] as u32, 0x3f8));
        c.g[V1][0] = lw(c, p(c.g[S2][0] as u32, 8));
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let t = (c.g[S1][0] as i64) < 0;
        c.g[S0][0] |= 0xffff;
        if !t {
            c.cycles += 3;
            c.g[S5][0] = sx(0xffff_ff00);
            c.g[V1][0] = lw(c, p(c.g[S2][0] as u32, 0xc));
            loop {
                c.cycles += 17;
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[S0][0] &= c.g[S5][0];
                c.g[A0][0] = c.g[S4][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = c.lbu(p(c.g[V0][0] as u32, 1));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                c.g[V1][0] = c.lhu(p(c.g[V0][0] as u32, 2));
                c.g[S0][0] |= c.g[A2][0];
                c.g[A2][0] = c.lbu(c.g[V0][0] as u32);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                c.g[V0][0] = c.g[S0][0] & 0xff;
                c.g[S0][0] = c.g[V0][0] | c.g[V1][0];
                c.g[RA][0] = sx(0x003a_85bc);
                c.g[A3][0] = c.g[S0][0];
                c.pc = 0x003a_8e20;
                ee::s13::cWorldCache_ReleaseResource(c);
                if c.pc != 0x003a_85bc {
                    return Err(Unwound);
                }
                if (c.g[S1][0] as i64) < 0 {
                    break;
                }
                c.g[V1][0] = lw(c, p(c.g[S2][0] as u32, 0xc));
                if event_due(c) {
                    deliver_event_here(c, 0x003a_8580);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 0x18;
        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3f8));
        c.mult(0, 16, S3, V0); // a0 = page * 0x18
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.g[A0][0] = lw(c, c.g[S1][0] as u32);
        let t = c.g[A0][0] == 0;
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        if !t {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003a_85e8);
            c.g[A1][0] = 3;
            c.pc = 0x003a_7c30;
            ee::s13::cHullPage_dtor(c);
            if c.pc != 0x003a_85e8 {
                return Err(Unwound);
            }
            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        }
        c.cycles += 11;
        c.sw(c.g[S1][0] as u32, 0);
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore(c, 20, 0x20);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        c.sw(p(c.g[V0][0] as u32, 0x3f0), 0);
        frame.close(c);
        Ok(())
    })
}

/// `BEMissionInt_CountCompletedOnPeak(self, a1)` (0x1543e0): count, over the
/// 88 missions of the current peak and their 22 sub-goals, how many are
/// completed: a matching id pair queries `BEMissionInt_IsMissionComplete`.
pub fn bemissionint_count_completed_on_peak(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        let this = abi::arg(c, 0);
        let a1 = abi::arg(c, 1);
        c.g[V0][0] = 0x44_0000;
        frame.save(c, 20, 0x50);
        c.g[V1][0] = 0;
        frame.save(c, 22, 0x30);
        c.g[S4][0] = 0;
        frame.save(c, 23, 0x20);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, 30, 0x10);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, 16, 0x90);
        c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x11f0));
        frame.save(c, 17, 0x80);
        frame.save(c, 18, 0x70);
        frame.save(c, 19, 0x60);
        frame.save(c, 21, 0x40);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x24;
        loop {
            c.cycles += 10;
            c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.mult(0, V0, V1, A0); // v0 = i * 0x24
            c.g[S1][0] = 0;
            c.g[V1][0] = 0x44_0000;
            c.g[A0][0] = 0x44_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x26b0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11e0));
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x54));
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            loop {
                c.cycles += 3;
                c.g[V0][0] = lw(c, c.g[S2][0] as u32);
                let mut next = false;
                if c.g[V0][0] != c.g[S1][0] {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    next = true;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, c.g[S0][0] as u32);
                    if c.g[V0][0] != c.g[S6][0] {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        next = true;
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = lw(c, c.g[S3][0] as u32);
                        c.g[RA][0] = sx(0x0015_4474);
                        c.g[A0][0] = c.g[S7][0];
                        c.pc = 0x0015_3d78;
                        ee::s01::BEMissionInt_IsMissionComplete(c);
                        if c.pc != 0x0015_4474 {
                            return Err(Unwound);
                        }
                        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                        if c.g[V0][0] != 0 {
                            c.g[S4][0] = c.g[V1][0];
                        }
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        next = true;
                    }
                }
                let _ = next;
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 22) as u64;
                let t = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x64));
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0015_4450);
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[S5][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 88) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[A0][0] = 0x24;
            if event_due(c) {
                deliver_event_here(c, 0x0015_4428);
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, 16, 0x90);
        frame.restore(c, 17, 0x80);
        frame.restore(c, 18, 0x70);
        frame.restore(c, 19, 0x60);
        frame.restore(c, 20, 0x50);
        frame.restore(c, 21, 0x40);
        frame.restore(c, 22, 0x30);
        frame.restore(c, 23, 0x20);
        frame.restore(c, 30, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCharSelect_Init(self)` (0x181148): build the character-select
/// screen: create its menu from the state's menu hash, then lay out the
/// rider preview, the confirm prompts and the enter animation.
pub fn cfestate_charselect_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        let this = abi::arg(c, 0);
        frame.save(c, 17, 0x30);
        c.g[S1][0] = c.g[A0][0];
        c.sd(p(sp(c), 0x20), c.g[RA][0]);
        frame.save(c, 16, 0x40);
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a00));
        c.g[RA][0] = sx(0x0018_116c);
        c.g[S0][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
        call_ptr(c, 0x0031_7670, 0x0018_116c)?; // StrHash(name)
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0018_1180);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_1180)?; // the screen's menu
        let t = c.g[V0][0] == 0;
        c.sw(p(c.g[S1][0] as u32, 0x40), c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x0018_1198);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_1198)?;
        }
        c.cycles += 40;
        c.g[V0][0] = lw(c, gp(c, -0x848));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.g[A2][0] = sx(p(sp(c), 0x10));
        c.g[S0][0] = lw(c, p(c.g[V0][0] as u32, 0x7c));
        c.g[AT][0] = 0x4348_0000;
        c.mtc1(1, 0);
        c.swc1(1, p(sp(c), 0x1c));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.swc1(0, p(sp(c), 4));
        c.sw(sp(c), 0);
        c.sw(p(sp(c), 8), 0);
        c.swc1(1, p(sp(c), 0xc));
        c.sw(p(sp(c), 0x10), 0);
        c.sw(p(sp(c), 0x14), 0);
        c.g[RA][0] = sx(0x0018_11e0);
        c.sw(p(sp(c), 0x18), 0);
        call_ptr(c, 0x0015_e050, 0x0018_11e0)?;
        c.lwc1(0, gp(c, -0x5a28));
        c.g[V0][0] = lw(c, gp(c, -0x848));
        c.swc1(0, p(c.g[S0][0] as u32, 0x10));
        c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x7c));
        c.g[A1][0] = c.lb(p(c.g[S1][0] as u32, 0x44));
        c.g[RA][0] = sx(0x0018_11fc);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        call_ptr(c, 0x001a_0548, 0x0018_11fc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0018_1208);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0019_e538, 0x0018_1208)?;
        c.g[AT][0] = sx(0xbf80_0000);
        c.mtc1(1, 0);
        c.g[V0][0] = 1;
        c.sw(p(c.g[S1][0] as u32, 0x80), c.g[V0][0]);
        c.swc1(0, p(c.g[S1][0] as u32, 0x5c));
        c.sw(p(c.g[S1][0] as u32, 0x60), 0);
        c.sw(p(c.g[S1][0] as u32, 0x58), 0);
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `UIMenu_FindPrevSelectable(self)` (0x39ac48): walk the menu's item list
/// backwards from the current-ish index and return the previous enabled item
/// index (0 when none is found). The generated body is a dispatcher: it can
/// be re-entered at the loop head after an event unwind, so the replacement
/// keeps the same block addresses as states.
pub fn uimenu_find_prev_selectable(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0039_ac48 => {
                    c.cycles += 12;
                    let frame = Frame::open(c, 0x50);
                    frame.save(c, 16, 0x40);
                    frame.save(c, 17, 0x30);
                    frame.save(c, 19, 0x10);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, 18, 0x20);
                    c.g[S3][0] = 0xff;
                    frame.save_ra(c, 0);
                    c.g[V0][0] = c.lbu(p(c.g[S1][0] as u32, 0x95));
                    c.g[S2][0] = c.lbu(p(c.g[S1][0] as u32, 0x96));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[S0][0] = c.g[V0][0] & 0xff;
                    pc = 0x0039_ac78;
                }
                0x0039_ac78 => {
                    c.cycles += 2;
                    if c.g[S0][0] == c.g[S3][0] {
                        c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x14));
                        pc = 0x0039_acec;
                    } else {
                        pc = 0x0039_ac80;
                    }
                }
                0x0039_ac80 => {
                    c.cycles += 4;
                    c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x90));
                    c.g[V0][0] &= 4;
                    let t = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                    pc = if t { 0x0039_acc4 } else { 0x0039_ac90 };
                }
                0x0039_ac90 => {
                    c.cycles += 11;
                    c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x5c));
                    c.g[A2][0] = 1;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = lw(c, p(c.g[V0][0] as u32, 0xd0));
                    c.g[V1][0] = lw(c, p(c.g[T0][0] as u32, 8));
                    c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0xa0));
                    c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0xa4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0039_acb4);
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.pc = t;
                    ssx3_ee_rt::funcs::call_indirect(c, t);
                    if c.pc != 0x0039_acb4 {
                        return Err(Unwound);
                    }
                    let zero = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                    pc = if zero { 0x0039_ad08 } else { 0x0039_acbc };
                }
                0x0039_acbc => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    pc = 0x0039_ad1c;
                }
                0x0039_acc4 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[RA][0] = sx(0x0039_acd0);
                    c.g[A1][0] = c.g[S0][0];
                    c.pc = 0x0039_7870;
                    ee::s13::UIList_GetAt(c);
                    if c.pc != 0x0039_acd0 {
                        return Err(Unwound);
                    }
                    c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x14));
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                    c.g[V1][0] &= 1;
                    let t = c.g[V1][0] == 0;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                    if t {
                        if back_edge_unwinds(c, 0x0039_acbc) {
                            return Ok(());
                        }
                        pc = 0x0039_acbc;
                    } else {
                        pc = 0x0039_ace4;
                    }
                }
                0x0039_ace4 => {
                    c.cycles += 2;
                    c.g[S0][0] = c.g[V0][0] & 0xff;
                    pc = 0x0039_ad0c;
                }
                0x0039_acec => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                    c.g[V0][0] &= 1;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lbu(p(c.g[S1][0] as u32, 0x96));
                        pc = 0x0039_ad04;
                    } else {
                        pc = 0x0039_acfc;
                    }
                }
                0x0039_acfc => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lbu(p(c.g[S1][0] as u32, 0x95));
                    pc = 0x0039_ad1c;
                }
                0x0039_ad04 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    pc = 0x0039_ad08;
                }
                0x0039_ad08 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.g[V0][0] & 0xff;
                    pc = 0x0039_ad0c;
                }
                0x0039_ad0c => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    c.g[S2][0] = c.g[V0][0] & 0xff;
                    let t = c.g[S2][0] != c.g[S3][0];
                    c.g[V0][0] = 0;
                    if t {
                        if back_edge_unwinds(c, 0x0039_ac78) {
                            return Ok(());
                        }
                        pc = 0x0039_ac78;
                    } else {
                        pc = 0x0039_ad1c;
                    }
                }
                0x0039_ad1c => {
                    c.cycles += 7;
                    let frame = Frame { size: 0x50 };
                    frame.restore(c, 16, 0x40);
                    frame.restore(c, 17, 0x30);
                    frame.restore(c, 18, 0x20);
                    frame.restore(c, 19, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039_ac48, 0x0039_ad38);
                    return Ok(());
                }
            }
        }
    })
}

/// `ScriptList_Add(self, cmd, arg)` (0x275dd8): queue a script command on
/// the list: push a command record, and when the pending list is in its
/// running state, prepare the next entry and advance it. Returns 1 when
/// queued, 0 when the command queue is full.
pub fn scriptlist_add(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A2][0];
        c.sd(p(sp(c), 0x20), c.g[RA][0]);
        c.g[S3][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0027_5e08);
        c.g[A0][0] = c.g[S1][0];
        c.pc = 0x0028_3c58;
        ee::s07::ScriptCmdQueue_IsFull(c);
        if c.pc != 0x0027_5e08 {
            return Err(Unwound);
        }
        let full = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !full {
            c.cycles += 14;
            c.g[V0][0] = sx(0xffff_ffff);
            c.sw(sp(c), c.g[S3][0]);
            c.sw(p(sp(c), 8), c.g[S2][0]);
            c.g[A0][0] = c.g[S1][0];
            c.sw(p(sp(c), 0x18), c.g[V0][0]);
            c.g[A1][0] = c.g[SP][0];
            c.sw(p(sp(c), 4), c.g[V0][0]);
            c.sw(p(sp(c), 0xc), c.g[V0][0]);
            c.sw(p(sp(c), 0x10), c.g[V0][0]);
            c.g[RA][0] = sx(0x0027_5e3c);
            c.sw(p(sp(c), 0x14), c.g[V0][0]);
            c.pc = 0x0028_3c80;
            ee::s07::ScriptCmdQueue_Push(c);
            if c.pc != 0x0027_5e3c {
                return Err(Unwound);
            }
            c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, 0xa4));
            let t = c.g[V1][0] == 0;
            c.g[V0][0] = 1;
            if !t {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0xbc));
                let t = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = 1;
                if !t {
                    let mut direct = false;
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x0027_5e5c);
                    c.g[A0][0] = c.g[S1][0];
                    c.pc = 0x0028_3d28;
                    ee::s07::ScriptCmdQueue_Count(c);
                    if c.pc != 0x0027_5e5c {
                        return Err(Unwound);
                    }
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[RA][0] = sx(0x0027_5e6c);
                    c.g[A2][0] = 0;
                    c.pc = 0x0027_7450;
                    ee::s07::ScriptList_PrepareEntry(c);
                    if c.pc != 0x0027_5e6c {
                        return Err(Unwound);
                    }
                    c.g[A0][0] = lw(c, p(c.g[S0][0] as u32, 0xa4));
                    c.g[V1][0] = 3;
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.sw(p(c.g[S0][0] as u32, 0xbc), c.g[V0][0]);
                    if !t {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x0027_5e88);
                        c.g[A1][0] = 0;
                        c.pc = 0x0028_3d70;
                        ee::s07::ScriptCmdQueue_At(c);
                        if c.pc != 0x0027_5e88 {
                            return Err(Unwound);
                        }
                        c.g[A1][0] = lw(c, p(c.g[V0][0] as u32, 0xc));
                        c.g[RA][0] = sx(0x0027_5e94);
                        c.g[A0][0] = lw(c, c.g[S0][0] as u32);
                        c.pc = 0x0028_2bf0;
                        ee::s07::ScriptEngine_GetProcessInSeq(c);
                        if c.pc != 0x0027_5e94 {
                            return Err(Unwound);
                        }
                        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x20));
                        c.g[V1][0] ^= 4;
                        let t = c.g[V1][0] != 0;
                        c.g[V0][0] = 1;
                        if t {
                            direct = true;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x0027_5eac);
                            c.g[A0][0] = c.g[S0][0];
                            c.pc = 0x0027_76e0;
                            ee::s07::ScriptList_CheckNextReady(c);
                            if c.pc != 0x0027_5eac {
                                return Err(Unwound);
                            }
                        }
                    }
                    if !direct {
                        c.cycles += 1;
                        c.g[V0][0] = 1;
                    }
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        c.g[RA][0] = c.ld(p(sp(c), 0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `ScriptViewObj_Deactivate(self, a1)` (0x281400): clear the object's
/// camera bindings (two slots), stop its transition, drop its world binding,
/// then reset and deactivate the view object.
pub fn scriptviewobj_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        let this = abi::arg(c, 0);
        frame.save(c, 17, 0x50);
        frame.save(c, 21, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x60);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, 18, 0x40);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xdc));
        frame.save(c, 19, 0x30);
        c.g[S2][0] = 1;
        frame.save(c, 20, 0x20);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0028_143c);
        c.g[S4][0] = sx(0xffff_ffff);
        c.pc = 0x0027_d2e8;
        ee::s07::ScriptEngine_GetInstance(c);
        if c.pc != 0x0028_143c {
            return Err(Unwound);
        }
        c.g[A1][0] = lw(c, p(c.g[S1][0] as u32, 4));
        c.g[RA][0] = sx(0x0028_1448);
        c.g[A0][0] = c.g[V0][0];
        c.pc = 0x0027_d170;
        ee::s07::ScriptEngine_ClearObjBindingFilter(c);
        if c.pc != 0x0028_1448 {
            return Err(Unwound);
        }
        loop {
            c.cycles += 3;
            c.g[V0][0] = lw(c, c.g[S0][0] as u32);
            let t = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if !t {
                c.cycles += 5;
                c.g[A1][0] = 0;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[RA][0] = sx(0x0028_1464);
                c.g[A0][0] = lw(c, c.g[V0][0] as u32);
                c.pc = 0x0027_d688;
                ee::s07::ScriptObjCam_SetBinding(c);
                if c.pc != 0x0028_1464 {
                    return Err(Unwound);
                }
                c.sw(c.g[S0][0] as u32, c.g[S4][0]);
            }
            c.cycles += 3;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            let t = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !t {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_1448);
            }
        }
        c.cycles += 3;
        c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0xd8));
        let t = c.g[A0][0] == 0;
        c.g[A1][0] = sx(0xffff_ffff);
        if !t {
            c.cycles += 6;
            c.mtc1(0, 12);
            c.g[A2][0] = 9;
            c.mov_s(13, 12);
            c.mov_s(14, 12);
            c.g[RA][0] = sx(0x0028_1498);
            c.mov_s(15, 12);
            c.pc = 0x0028_2db0;
            ee::s07::cTransitionMan_StopTransition(c);
            if c.pc != 0x0028_1498 {
                return Err(Unwound);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x110));
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = lw(c, gp(c, -0x848));
        if !t {
            c.cycles += 6;
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x100));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x84));
            c.g[A2][0] = lw(c, p(c.g[V1][0] as u32, 0xc));
            c.g[RA][0] = sx(0x0028_14b8);
            c.g[A0][0] = lw(c, p(c.g[A2][0] as u32, 0xa4));
            call_ptr(c, 0x0010_33f8, 0x0028_14b8)?;
            c.sw(p(c.g[S1][0] as u32, 0x110), 0);
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x0028_14c4);
        c.g[A0][0] = c.g[S1][0];
        c.pc = 0x0028_1a50;
        ee::s07::ScriptViewObj_Reset(c);
        if c.pc != 0x0028_14c4 {
            return Err(Unwound);
        }
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0028_14d0);
        c.g[A1][0] = c.g[S5][0];
        c.pc = 0x0028_3440;
        ee::s07::ScriptObj_Deactivate(c);
        if c.pc != 0x0028_14d0 {
            return Err(Unwound);
        }
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore(c, 20, 0x20);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraWobble_UpdateRoll(self, roll)` (0x160130): shake the camera roll
/// towards a random target while the wobble is active. A zero input clears
/// the wobble; otherwise it steps the roll towards the target and picks a new
/// target once close enough. Dispatcher-shaped (the generated body re-enters
/// at its loop blocks), so the replacement keeps the same block addresses.
pub fn camera_wobble_update_roll(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0016_0130 => {
                    c.cycles += 12;
                    c.mtc1(0, 0);
                    let frame = Frame::open(c, 0x50);
                    frame.save(c, 16, 0x30);
                    c.c_eq_s(12, 0);
                    frame.save(c, 17, 0x20);
                    frame.save(c, 18, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.swc1(21, p(sp(c), 0x48));
                    c.swc1(20, p(sp(c), 0x40));
                    let t = c.fcc();
                    c.swc1(12, p(c.g[S0][0] as u32, 0x1c));
                    pc = if t { 0x0016_01f8 } else { 0x0016_0160 };
                }
                0x0016_0160 => {
                    c.cycles += 6;
                    c.lwc1(1, p(c.g[S0][0] as u32, 0x24));
                    c.lwc1(0, p(c.g[S0][0] as u32, 0x2c));
                    c.c_lt_s(0, 1);
                    let t = !c.fcc();
                    c.g[S1][0] = 0x7f_0000;
                    pc = if t { 0x0016_01e8 } else { 0x0016_0178 };
                }
                0x0016_0178 => {
                    c.cycles += 9;
                    c.lwc1(0, p(c.g[S0][0] as u32, 0x14));
                    c.g[AT][0] = 0x3f00_0000;
                    c.mtc1(1, 20);
                    c.g[S1][0] |= 0xffff;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 21);
                    c.g[S2][0] = 0x3f80_0000;
                    c.swc1(0, p(c.g[S0][0] as u32, 8));
                    pc = 0x0016_01c4;
                }
                0x0016_019c => {
                    c.cycles += 1;
                    pc = 0x0016_01a0;
                }
                0x0016_01a0 => {
                    c.cycles += 9;
                    call_ptr(c, 0x0031_77f0, 0x0016_01a8)?; // BXrand()
                    c.g[V0][0] &= c.g[S1][0];
                    c.g[V0][0] |= c.g[S2][0];
                    c.mtc1(2, 1);
                    c.sub_s(0, 1, 21);
                    c.sub_s(0, 0, 20);
                    c.add_s(0, 0, 0);
                    c.swc1(0, p(c.g[S0][0] as u32, 0x14));
                    pc = 0x0016_01c4;
                }
                0x0016_01c4 => {
                    c.cycles += 7;
                    c.lwc1(0, p(c.g[S0][0] as u32, 8));
                    c.lwc1(1, p(c.g[S0][0] as u32, 0x14));
                    c.sub_s(0, 0, 1);
                    c.abs_s(0, 0);
                    c.c_lt_s(0, 20);
                    let t = c.fcc();
                    if t {
                        if back_edge_unwinds(c, 0x0016_01a0) {
                            return Ok(());
                        }
                        pc = 0x0016_01a0;
                    } else {
                        pc = 0x0016_01e0;
                    }
                }
                0x0016_01e0 => {
                    c.cycles += 2;
                    c.sw(p(c.g[S0][0] as u32, 0x24), 0);
                    pc = 0x0016_0204;
                }
                0x0016_01e8 => {
                    c.cycles += 4;
                    c.lwc1(0, gp(c, -0x6c20));
                    c.add_s(0, 1, 0);
                    c.swc1(0, p(c.g[S0][0] as u32, 0x24));
                    pc = 0x0016_0204;
                }
                0x0016_01f8 => {
                    c.cycles += 3;
                    c.swc1(0, p(c.g[S0][0] as u32, 8));
                    c.swc1(0, p(c.g[S0][0] as u32, 0x24));
                    c.swc1(0, p(c.g[S0][0] as u32, 0x14));
                    pc = 0x0016_0204;
                }
                0x0016_0204 => {
                    c.cycles += 8;
                    let frame = Frame { size: 0x50 };
                    frame.restore(c, 16, 0x30);
                    frame.restore(c, 17, 0x20);
                    frame.restore(c, 18, 0x10);
                    frame.restore_ra(c, 0);
                    frame.restore_fpr(c, 21, 0x48);
                    frame.restore_fpr(c, 20, 0x40);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0016_0130, 0x0016_0224);
                    return Ok(());
                }
            }
        }
    })
}

/// `Replay_dtor(self, flags)` (0x26f088): tear a replay object down: end
/// playback, drop the reset hook, free the buffers, run the destructors of
/// the embedded objects, and free the object itself when `flags & 1`.
pub fn replay_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        let this = abi::arg(c, 0);
        let flags = abi::arg(c, 1);
        c.g[V1][0] = 0;
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, c.g[S1][0] as u32);
        let t = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !t {
            c.cycles += 1;
            c.g[V1][0] = ((c.g[V0][0] as i64) < 10) as u64;
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0026_f0c8);
            c.g[A0][0] = c.g[S1][0];
            c.pc = 0x0026_f980;
            ee::s07::Replay_EndPlayback(c);
            if c.pc != 0x0026_f0c8 {
                return Err(Unwound);
            }
        }
        c.g[A0][0] = c.g[S1][0];
        c.cycles += 8;
        c.g[RA][0] = sx(0x0026_f0d4);
        c.g[S3][0] = c.g[S0][0] & 1;
        c.pc = 0x0026_f498;
        ee::s07::Replay_EmptyResetHook(c);
        if c.pc != 0x0026_f0d4 {
            return Err(Unwound);
        }
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c));
        c.g[RA][0] = sx(0x0026_f0e0);
        c.g[A0][0] = c.g[S1][0];
        c.pc = 0x0026_f428;
        ee::s07::Replay_FreeBuffers(c);
        if c.pc != 0x0026_f0e0 {
            return Err(Unwound);
        }
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x494));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5fc));
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 2;
            loop {
                c.cycles += 7;
                if c.g[V1][0] == c.g[V0][0] {
                    break;
                }
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb4));
                if event_due(c) {
                    deliver_event_here(c, 0x0026_f100);
                }
            }
        }
        c.cycles += 2;
        let t = c.g[S2][0] == 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3ac));
        if !t {
            c.cycles += 2;
            let t = c.g[S2][0] == c.g[S0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x38));
            if !t {
                c.cycles += 1;
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = lw(c, c.g[S0][0] as u32);
                    c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x38));
                    c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0026_f148);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.pc = t;
                    ssx3_ee_rt::funcs::call_indirect(c, t);
                    if c.pc != 0x0026_f148 {
                        return Err(Unwound);
                    }
                    let t = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x38));
                    if !t {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_f130);
                    }
                }
            }
        }
        c.cycles += 2;
        let t = c.g[S3][0] == 0;
        frame.restore(c, 16, 0x40);
        if !t {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e50, 0x0026_f160)?; // operator delete(this)
            frame.restore(c, 16, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCharEquip_dtor(self, flags)` (0x199148): release the character
/// equipment screen's preview and its four bolt-file bindings, then run the
/// UI state destructor (and free the object when `flags & 1`).
pub fn cfestate_charequip_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        let this = abi::arg(c, 0);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, 16, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ab8));
        frame.save(c, 18, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.sw(p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        c.g[A2][0] = lw(c, p(c.g[S0][0] as u32, 0xa64));
        let t = (c.g[A2][0] as i64) < 0;
        c.g[S2][0] = c.g[A1][0];
        if !t {
            c.cycles += 7;
            c.g[V0][0] = lw(c, gp(c, -0x848));
            c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0xbc));
            c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x7c));
            c.g[RA][0] = sx(0x0019_918c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
            c.pc = 0x0019_dc20;
            ee::s03::BoltFile_Release(c);
            if c.pc != 0x0019_918c {
                return Err(Unwound);
            }
            c.g[V0][0] = sx(0xffff_ffff);
            c.sw(p(c.g[S0][0] as u32, 0xa64), c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0xa68));
        let t = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = lw(c, gp(c, -0x854));
        if !t {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x190));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x194));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0019_91b4);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x0019_91b4 {
                return Err(Unwound);
            }
        }
        c.cycles += 7;
        c.g[S1][0] = sx(0xffff_ffff);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0019_91c4);
        c.sw(p(c.g[S0][0] as u32, 0xa68), c.g[S1][0]);
        c.pc = 0x0019_a498;
        ee::s03::cFEStateCharEquip_ReleasePreview(c);
        if c.pc != 0x0019_91c4 {
            return Err(Unwound);
        }
        c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0xa70));
        let t = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = lw(c, gp(c, -0x854));
        if !t {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x190));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x194));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0019_91e4);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x0019_91e4 {
                return Err(Unwound);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0xa74));
        let t = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = lw(c, gp(c, -0x854));
        if !t {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x190));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x194));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0019_9204);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x0019_9204 {
                return Err(Unwound);
            }
        }
        c.cycles += 14;
        c.g[V0][0] = 0x47_0000;
        c.sw(p(c.g[S0][0] as u32, 0xa74), c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
        c.sw(p(c.g[S0][0] as u32, 0xa70), c.g[S1][0]);
        c.sw(p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0019_9224);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0039_e390, 0x0019_9224)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateStats_v25(self, state)` (0x1c3570): the stats screen's second
/// constructor vtable entry: set the numeric field, hash and store the title
/// string, then hash and install the five statistic rows.
pub fn cfestate_stats_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 62;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 5;
        c.g[RA][0] = sx(0x001c_358c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001c_358c)?; // set number 5
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5dc0));
        call_ptr(c, 0x0031_7670, 0x001c_3598)?; // StrHash(title)
        c.g[A0][0] = 0x46_0000;
        c.sw(p(c.g[S0][0] as u32, 0x54), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5dd8));
        call_ptr(c, 0x0031_7670, 0x001c_35a8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001c_35b8);
        c.g[A1][0] = 0;
        call_ptr(c, 0x001d_a530, 0x001c_35b8)?;
        // the five statistic rows: (label string, row, a3)
        for (i, (name, row, a3)) in [
            (0x45_fe60u32, 0u64, 3u64),
            (0x45_fe40, 1, 4),
            (0x46_1458, 2, 2),
            (0x46_5df8, 3, 8),
            (0x46_14e8, 4, 1),
        ]
        .into_iter()
        .enumerate()
        {
            let ret = 0x001c_35c4 + 0x1c * i as u32;
            c.g[A0][0] = sx(name);
            call_ptr(c, 0x0031_7670, ret)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = row;
            c.g[A3][0] = a3;
            call_ptr(c, 0x001d_a510, ret + 0x14)?;
        }
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// FUNCS_END

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{compare, put_vec, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 200;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Every compared state must match: hard, caller-saved (`soft`) and LO/HI.
    fn strict(g: GuestFn, h: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(g, h, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    /// The `BELibrary` singleton (`gp - 0x1f04`) and the world (`gp - 0x848`).
    fn globals(c: &mut Cpu) {
        let be = DATA + 0x3_0000;
        let world = DATA + 0x8_0000;
        w(c, GAME_GP.wrapping_sub(0x1f04), be);
        w(c, GAME_GP.wrapping_sub(0x848), world);
        let game = world + 0x1000;
        w(c, world + 0x84, game);
        let race = world + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// The engine's size-class pool allocator behind `gp + 0xdd8`.
    fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x400 * i as u32;
            for n in 0..8u32 {
                let node = base + 0x40 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x40 });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot, pool);
        }
    }

    /// Scrambled state, the fake call hooks, `gp`, the globals and the pools.
    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        globals(c);
        pool(c);
        r
    }

    #[test]
    fn tex_flip_modifier_ctor_matches() {
        strict(ee::s12::f_0035f0b8__generated, tex_flip_modifier_ctor, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0xf0b8, 0);
            let clock = DATA + 0x400;
            let desc = DATA + 0x1000;
            let this = DATA + 0x2000;
            let owner = DATA + 0x3000;
            w(c, GAME_GP.wrapping_add(0x2a74), clock);
            w(c, clock + 0x10, r.next_u32() | 1);
            w(c, desc + 4, r.next_u32());
            w(c, desc + 0xc, r.next_u32());
            // a negative first frame asks for a random start frame
            let n = if seed % 3 == 0 {
                (r.next_u32() % 0x8000_0000) as i32
            } else {
                -((r.next_u32() % 64) as i32) - 1
            };
            w(c, desc + 8, n as u32);
            put_vec(c, desc + 0x10, [r.edge_f32(), 0, 0, 0]);
            // the owner's entry table is empty: the init loop is skipped
            w(c, owner + 0x94, owner + 0x100);
            w(c, owner + 0x100, 0);
            c.bus.write32(this + 0x18, 0);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(desc);
            c.g[A2][0] = sx(owner);
        });
    }

    #[test]
    fn ai_apply_computer_rider_settings_matches() {
        strict(ee::s00::f_0012b698__generated, ai_apply_computer_rider_settings, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0xb698, 0);
            let rider = DATA + 0x100;
            let state = DATA + 0x2000;
            w(c, rider + 0x18, state);
            w(
                c,
                0x0053_08d0,
                if seed % 3 == 0 {
                    8
                } else if seed % 3 == 1 {
                    1
                } else {
                    r.next_u32() & !9
                },
            );
            w(c, 0x0053_08b8, r.next_u32());
            w(c, 0x0053_08d4, r.next_u32() % 5);
            c.g[A0][0] = sx(DATA + 0x800);
            c.g[A1][0] = sx(rider);
        });
    }


    /// A camera trigger box whose vtable slots are fake calls, read from a
    /// dummy byte stream.
    fn camera_box_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = base(c, seed, salt, 0);
        let reader = DATA + 0x100;
        let obj = DATA + 0x2000;
        let vt = DATA + 0x3000;
        w(c, reader, DATA + 0x4000);
        w(c, obj + 0x24, vt);
        for (i, off) in [8u32, 0x10, 0x18, 0x20, 0x28].into_iter().enumerate() {
            c.bus.write16(vt + off, (r.next_u32() as i16 & !3) as u16);
            w(c, vt + off + 4, crate::difftest::fakes::FAKE + 0x10 * i as u32);
        }
        for k in 0..8u32 {
            w(c, DATA + 0x4000 + 4 * k, r.next_u32());
        }
        c.g[A0][0] = sx(reader);
        c.g[A1][0] = sx(obj);
    }

    #[test]
    fn quad_tree_flatten_node_matches() {
        strict(ee::s08::f_002c2088__generated, quad_tree_flatten_node, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x2088, 0);
            let this = DATA + 0x1000;
            let base = DATA + 0x4000;
            let count = DATA + 0x100;
            let out = DATA + 0x2000;
            let map = DATA + 0x3000;
            w(c, count, 0);
            w(c, this + 0x20, base);
            // the root's four children are leaves at base + 0x100 + 0x10*k
            for k in 0..4u32 {
                let child = base + 0x100 + 0x10 * k;
                let val = 2 * (0x100 + 0x10 * k) + 1;
                c.bus.write16(base + 2 * k, val as u16);
                w(c, child, 0);
                w(c, child + 8, r.next_u32());
            }
            w(c, base + 8, r.next_u32());
            for k in 0..4u32 {
                c.bus.write16(base + 8 + 2 * k, if seed % 2 == 0 { 0 } else { (0x201 + 0x20 * k) as u16 });
            }
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(out);
            c.g[A2][0] = sx(count);
            c.g[A3][0] = sx(map);
            c.g[T0][0] = sx(base);
        });
    }

        #[test]
    fn cwscriptcache_alloc_matches() {
        strict(ee::s13::f_003ac800__generated, cwscriptcache_alloc, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0xc800, 0);
            let this = DATA + 0x100;
            let count = 1 + (seed % 8);
            if seed % 2 == 0 {
                w(c, this + 4, 0);
            } else {
                // an existing array of two zeroed entries behind a header
                let mem = DATA + 0x2000;
                w(c, mem, 2);
                w(c, this + 4, mem + 0x10);
                for k in 0..2u32 {
                    w(c, mem + 0x10 + 0xc * k, 0);
                    w(c, mem + 0x14 + 0xc * k, 0);
                    w(c, mem + 0x18 + 0xc * k, 0);
                }
            }
            w(c, this + 8, r.next_u32());
            c.g[A0][0] = sx(this);
            c.g[A1][0] = count as u64;
        });
    }

    #[test]
    fn av_audio_init_matches() {
        // Only the "stream module not up yet" path: the ready path opens a
        // stream through the IOP, which a test machine cannot do.
        strict(ee::s13::f_003aed40__generated, av_audio_init, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0xed40, 0);
            let _ = r;
            let this = DATA + 0x100;
            c.bus.write8(0x0050_aa5c, 0);
            w(c, 0x0045_0da4, 0);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(DATA + 0x200);
            c.g[A2][0] = (seed % 3) as u64;
        });
    }

    #[test]
    fn mpeg2_picture_display_extension_matches() {
        strict(ee::s13::f_003b1fc8__generated, mpeg2_picture_display_extension, SEEDS, &|c, seed| {
            let _r = base(c, seed, 0x1fc8, 0);
            // no decoder state: one picture, three fields, two marker bits
            w(c, 0x0050_9524, 0);
            w(c, 0x0050_9584, if seed % 3 == 0 { 3 } else { 0 });
            let st = DATA + 0x3000;
            w(c, 0x0050_9588, st);
            w(c, st, DATA + 0x4000);
            w(c, st + 4, 0);
            w(c, st + 8, 0);
            c.g[A0][0] = sx(DATA + 0x500);
        });
    }

            #[test]
    fn csetupman_host_recv_msg11_matches() {
        strict(ee::s07::f_00269608__generated, csetupman_host_recv_msg11, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x9608, 0);
            let _ = r;
            let msg = DATA + 0x100;
            w(c, msg, (seed % 12) as u32);
            // the setup manager's packet list behind gp - 0xc8
            let list = DATA + 0x2000;
            let node = DATA + 0x2100;
            let prev = DATA + 0x2200;
            w(c, GAME_GP.wrapping_sub(0xc8), list);
            w(c, list + 0xf0, node);
            w(c, node + 4, prev);
            w(c, prev, 0);
            c.g[A0][0] = sx(DATA + 0x3000);
            c.g[A1][0] = sx(msg);
        });
    }

    #[test]
    fn sndloadqueue_ctor_matches() {
        strict(ee::s07::f_0028a058__generated, sndloadqueue_ctor, SEEDS, &|c, seed| {
            let _r = base(c, seed, 0xa058, 0);
            c.g[A0][0] = sx(DATA + 0x1000);
        });
    }

        #[test]
    fn quat_from_euler_matches() {
        strict(ee::s00::f_001241c0__generated, quat_from_euler, SEEDS, &|c, seed| {
            let mut r = Rng(0x0012_41c0 ^ (seed as u64) << 10);
            let _ = base(c, seed, 0x41c0, 0);
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            c.f[14] = r.edge_f32();
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
        });
    }

                #[test]
    fn cworldcache_free_hull_page_matches() {
        strict(ee::s13::f_003a8528__generated, cworldcache_free_hull_page, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x8528, 0);
            let this = DATA + 0x100;
            let page = DATA + 0x2000;
            let page_idx = (seed % 3) as u32;
            w(c, this + 0x3f8 + 0x18 * page_idx, page);
            // no resources in the page: the release loop is skipped
            w(c, page + 8, 0);
            w(c, page + 0xc, DATA + 0x3000);
            w(c, page, 0);
            let _ = r;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = page_idx as u64;
        });
    }

    #[test]
    fn bemissionint_count_completed_on_peak_matches() {
        strict(ee::s01::f_001543e0__generated, bemissionint_count_completed_on_peak, SEEDS, &|c, seed| {
            let _r = base(c, seed, 0x43e0, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            // a non-zero sub-goal id: no mission lookup is taken
            c.g[A1][0] = sx(0x1234 + (seed as u32 & 1));
        });
    }

    #[test]
    fn cfestate_charselect_init_matches() {
        strict(ee::s02::f_00181148__generated, cfestate_charselect_init, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x1148, 0);
            let this = DATA + 0x100;
            let world = DATA + 0x8_0000;
            w(c, world + 0x7c, DATA + 0x2000);
            w(c, this + 0x10, if seed % 3 == 0 { 0 } else { r.next_u32() & !7 });
            c.bus.write8(this + 0x44, (seed % 4) as u8);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn uimenu_find_prev_selectable_matches() {
        strict(ee::s13::f_0039ac48__generated, uimenu_find_prev_selectable, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0xac48, 0);
            let menu = DATA + 0x100;
            let items = DATA + 0x2000;
            // the list object behind menu + 0x74 and its entries
            w(c, menu + 0x74, items);
            let n = 3 + seed % 5;
            for k in 0..8u32 {
                let item = DATA + 0x3000 + 0x40 * k;
                w(c, items + 4 + 4 * k, item);
                w(c, item + 0x14, r.next_u32());
                w(c, item + 0x90, r.next_u32());
            }
            w(c, items, n);
            c.bus.write8(menu + 0x95, n as u8);
            c.bus.write8(menu + 0x96, (seed % 3) as u8);
            w(c, menu + 0x14, r.next_u32());
            w(c, menu + 0x90, r.next_u32());
            w(c, menu + 0x5c, DATA + 0x2800);
            c.g[A0][0] = sx(menu);
            c.pc = 0x0039_ac48;
        });
    }

    #[test]
    fn scriptlist_add_matches() {
        strict(ee::s07::f_00275dd8__generated, scriptlist_add, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x5dd8, 0);
            let this = DATA + 0x100;
            // no pending list state: the call queues and returns 1
            w(c, this + 0xa4, 0);
            w(c, this + 0xbc, r.next_u32());
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(r.next_u32());
            c.g[A2][0] = sx(r.next_u32());
        });
    }

        #[test]
    fn camera_wobble_update_roll_matches() {
        strict(ee::s01::f_00160130__generated, camera_wobble_update_roll, SEEDS, &|c, seed| {
            let mut r = Rng(0x0016_0130 ^ (seed as u64) << 9);
            let _ = base(c, seed, 0x0130, 0);
            let this = DATA + 0x100;
            // a zero input clears the wobble; otherwise the roll chases a
            // random target
            c.f[12] = if seed % 4 == 0 { 0 } else { r.edge_f32() };
            w(c, this + 0x24, r.f32(0.5));
            w(c, this + 0x2c, r.f32(0.5));
            w(c, this + 0x14, r.f32(0.5));
            w(c, this + 8, r.f32(0.5));
            c.g[A0][0] = sx(this);
            c.pc = 0x0016_0130;
        });
    }

        #[test]
    fn cfestate_charequip_dtor_matches() {
        strict(ee::s03::f_00199148__generated, cfestate_charequip_dtor, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x9148, 0);
            let this = DATA + 0x100;
            let world = DATA + 0x8_0000;
            w(c, world + 0x7c, DATA + 0x2000);
            // no bolt-file bindings: the release paths are skipped
            w(c, this + 0xa64, 0xffff_ffff);
            w(c, this + 0xa68, 0xffff_ffff);
            w(c, this + 0xa70, 0xffff_ffff);
            w(c, this + 0xa74, 0xffff_ffff);
            w(c, this + 0xbc, r.next_u32() & 3);
            w(c, GAME_GP.wrapping_sub(0x854), 0);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    }
