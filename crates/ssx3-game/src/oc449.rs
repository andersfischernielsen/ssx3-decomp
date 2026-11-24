//! Staging module oc449 (wave wave224): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc449`.

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
    (0x0012_aa78, ai_listed_riders_have_render_slots, "AI_ListedRidersHaveRenderSlots"),
    (0x0012_adc0, ai_mission_riders_have_render_slots, "AI_MissionRidersHaveRenderSlots"),
    (0x0012_b948, ai_load_state, "AI_LoadState"),
    (0x0012_de80, rider_wipeout_sub_play_end_anim_b, "RiderWipeoutSub_PlayEndAnimB"),
    (0x0013_6168, rider_spin_sub_get_flip_hud_blend, "RiderSpinSub_GetFlipHUDBlend"),
];

/// `base + off` as a 32-bit guest address (the `(reg as u32).wrapping_add(off)`
/// the generated code computes for every load and store).
#[inline]
fn oc449_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `int AI_ListedRidersHaveRenderSlots(cAI* self)` (0x12aa78): 1 unless a
/// listed rider (the `count` entries at `self+0x48`, count at `+0x80`; each
/// entry holds its rider at `+0x18`) is not safe to delete and its AI index
/// (`vt[getAIIndex]` of the rider object at `+0x6c0`) is in the special list
/// (`AI_IsRiderInSpecialList`, 0x12a1f0). 8 cycles on the exit path.
pub fn ai_listed_riders_have_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(oc449_at(c.g[S2][0], 0x80));
        c.g[S3][0] = 1;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S0][0] = sx(oc449_at(c.g[S2][0], 0x48));
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A0][0] = c.lw(oc449_at(c.g[V0][0], 0x18));
                call_ptr(c, 0x0011_c0e0, 0x0012_aab4)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw(oc449_at(c.g[S2][0], 0x80));
                } else {
                    c.cycles += 13;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A0][0] = c.lw(oc449_at(c.g[V0][0], 0x18));
                    c.g[V1][0] = c.lw(oc449_at(c.g[A0][0], 0x6c0));
                    c.g[A0][0] = sx(oc449_at(c.g[A0][0], 0x6c0));
                    c.g[A1][0] = c.lh(oc449_at(c.g[V1][0], 0x38));
                    c.g[V0][0] = c.lw(oc449_at(c.g[V1][0], 0x3c));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(oc449_at(c.g[A0][0], c.g[A1][0] as u32));
                    call_ptr(c, target, 0x0012_aadc)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0012_a1f0, 0x0012_aae8)?;
                    if c.g[V0][0] != 0 {
                        c.g[S3][0] = 0;
                    }
                    c.g[V0][0] = c.lw(oc449_at(c.g[S2][0], 0x80));
                }
                c.cycles += 4;
                c.g[S1][0] = sx(oc449_at(c.g[S1][0], 1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(oc449_at(c.g[S0][0], 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_aaa8);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int AI_MissionRidersHaveRenderSlots(cAI* self)` (0x12adc0): 1 unless a
/// mission rider is not safe to delete. The `NumberOfMissionRidersLoaded`
/// count (`+0x88`) indexes the array at `self+0x70`; each entry holds its
/// rider at `+0x18` (`cRider_isSafeToDelete`, 0x11c0e0).
pub fn ai_mission_riders_have_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(oc449_at(c.g[S2][0], 0x88));
        c.g[S3][0] = 1;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S0][0] = sx(oc449_at(c.g[S2][0], 0x70));
            loop {
                c.cycles += 9;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[S1][0] = sx(oc449_at(c.g[S1][0], 1));
                c.g[S0][0] = sx(oc449_at(c.g[S0][0], 4));
                c.g[A0][0] = c.lw(oc449_at(c.g[V0][0], 0x18));
                call_ptr(c, 0x0011_c0e0, 0x0012_ae04)?;
                c.g[V1][0] = c.lw(oc449_at(c.g[S2][0], 0x88));
                c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                if c.g[V0][0] == 0 {
                    c.g[S3][0] = 0;
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_adf0);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// One element of a replay-read list (`cAI_readFromReplayFrame`'s loops):
/// `obj = elem->+0x18`, `vt = obj->+0x77c->+0xde8`, then `vt[slot 0x1c]`
/// with `a0 = that field + delta` (`a1 = frame`, `a2 = fn`).
fn oc449_read_obj(c: &mut Cpu, elem: u64, ret_to: u32) -> Flow {
    c.g[V0][0] = c.lw(oc449_at(elem, 0x18));
    c.g[A3][0] = c.lw(oc449_at(c.g[V0][0], 0x77c));
    c.g[V1][0] = c.lw(oc449_at(c.g[A3][0], 0xde8));
    c.g[A0][0] = c.lh(oc449_at(c.g[V1][0], 0x18));
    c.g[A2][0] = c.lw(oc449_at(c.g[V1][0], 0x1c));
    let target = c.g[A2][0] as u32;
    c.g[A0][0] = sx(oc449_at(c.g[A3][0], c.g[A0][0] as u32));
    call_ptr(c, target, ret_to)
}

/// The shared element loop of `cAI_readFromReplayFrame`: each pass reads one
/// element (`a1 = frame`), then loops while `++i < self->count` (count at
/// `count_off`). Entered with `s0` at the first element slot and `v1` holding
/// its value. Returns when the loop exits (`v0` is the compare result).
fn oc449_read_loop(c: &mut Cpu, count_off: u32, ret_to: u32, head: u32) -> Flow {
    loop {
        c.cycles += 14;
        c.g[A1][0] = c.g[S3][0];
        c.g[S2][0] = sx(oc449_at(c.g[S2][0], 1));
        c.g[S0][0] = sx(oc449_at(c.g[S0][0], 4));
        let elem = c.g[V1][0];
        oc449_read_obj(c, elem, ret_to)?;
        let count = c.lw(oc449_at(c.g[S1][0], count_off));
        c.g[V0][0] = ((c.g[S2][0] as i64) < (count as i64)) as u64;
        if c.g[V0][0] == 0 {
            return Ok(());
        }
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void cAI_readFromReplayFrame(cAI* self, ReplayFrame* frame)` (0x12b948):
/// read the AI state back from a replay frame. The frame's vtable slot 2
/// (`+0x14`, delta `+0x10`) reads the header (size 0x28); the state is then
/// re-forced (`cAI_forceAIState`, 0x113b48) after copying the current state
/// handler into `PreviousAIStateHandler` (`+0x9c`); the players (`+0x40`,
/// count `+0x7c`), computers (`+0x48`, `+0x80`) and actors (`+0x5c`, `+0x84`)
/// are each read through `oc449_read_obj`; finally the render state is read
/// (`cRenderStateMan_readFromReplayFrame`, 0x2ef2d8).
pub fn ai_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x28;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
        c.g[A0][0] = c.lh(oc449_at(c.g[V1][0], 0x10));
        c.g[V0][0] = c.lw(oc449_at(c.g[V1][0], 0x14));
        let header = c.g[V0][0] as u32;
        c.g[A0][0] = sx(oc449_at(c.g[S3][0], c.g[A0][0] as u32));
        call_ptr(c, header, 0x0012_b988)?;
        c.g[A1][0] = c.lw(oc449_at(c.g[S1][0], 4));
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.lw(c.g[S1][0] as u32);
        call_ptr(c, 0x0011_3b48, 0x0012_b998)?;
        c.g[V0][0] = c.lw(oc449_at(c.g[S1][0], 0x98));
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        c.sw(oc449_at(c.g[S1][0], 0x9c), c.g[V0][0]);
        call_ptr(c, 0x0011_3b48, 0x0012_b9ac)?;

        // Players: array at +0x40, count +0x7c.
        c.g[V0][0] = c.lw(oc449_at(c.g[S1][0], 0x7c));
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[S0][0] = sx(oc449_at(c.g[S1][0], 0x40));
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            oc449_read_loop(c, 0x7c, 0x0012_b9e8, 0x0012_b9c0)?;
            c.cycles += 1;
        }
        c.g[V0][0] = c.lw(oc449_at(c.g[S1][0], 0x80));

        // Computers: array at +0x48, count +0x80.
        c.cycles += 2;
        let no_computers = (c.g[V0][0] as i64) <= 0;
        c.g[S2][0] = 0;
        if !no_computers {
            c.cycles += 3;
            c.g[S0][0] = sx(oc449_at(c.g[S1][0], 0x48));
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            oc449_read_loop(c, 0x80, 0x0012_ba38, 0x0012_ba10)?;
        }

        // Actors: array at +0x5c, count +0x84.
        c.cycles += 3;
        c.g[V0][0] = c.lw(oc449_at(c.g[S1][0], 0x84));
        let no_actors = (c.g[V0][0] as i64) <= 0;
        c.g[S2][0] = 0;
        if !no_actors {
            c.cycles += 3;
            c.g[S0][0] = sx(oc449_at(c.g[S1][0], 0x5c));
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            oc449_read_loop(c, 0x84, 0x0012_ba88, 0x0012_ba60)?;
        }

        c.cycles += 9;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x002e_f2d8, 0x0012_baa0)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void RiderWipeoutSub_PlayEndAnimB(RiderWipeoutSub* self)` (0x12de80):
/// the wipeout's end animation. The current anim ID of `self->mRider`'s anim
/// base (layer 2) is mapped through a jump table to its end variant: 373..=382
/// become 399..=408, and when that changes the anim, `cRiderAnimBase_play`
/// (0x3128e8) plays the new one with blend 0 and speed -1.0. Other IDs stay.
pub fn rider_wipeout_sub_play_end_anim_b(c: &mut Cpu) {
    if c.pc != 0x0012_de80 {
        return ee::s00::f_0012de80__entries(c);
    }
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 2;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(oc449_at(c.g[S0][0], 0x80));
        c.g[A0][0] = c.lw(oc449_at(c.g[V0][0], 0x784));
        call_ptr(c, 0x0031_2aa0, 0x0012_dea0)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V1][0] = sx(oc449_at(c.g[A1][0], 0u32.wrapping_sub(0x175)));
        c.g[V0][0] = (c.g[V1][0] < 0xa) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[A2][0] = c.g[A1][0];
        if in_range {
            c.cycles += 7;
            c.g[V0][0] = 0x0046_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f20));
            c.g[V1][0] = sx(oc449_at(c.g[V1][0], c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let target = c.g[A0][0] as u32;
            // The jump table's cases: entry k is 0x12ded0 + 8k, next anim 399 + k.
            let slot = match target {
                0x0012_ded0 => 0,
                0x0012_ded8 => 1,
                0x0012_dee0 => 2,
                0x0012_dee8 => 3,
                0x0012_def0 => 4,
                0x0012_def8 => 5,
                0x0012_df00 => 6,
                0x0012_df08 => 7,
                0x0012_df10 => 8,
                0x0012_df18 => 9,
                _ => {
                    c.jump_out(target, 0x0012_de80, 0x0012_df48);
                    return Ok(());
                }
            };
            c.cycles += if slot == 9 { 1 } else { 2 };
            c.g[A1][0] = 0x18f + slot as u64;
        }
        c.cycles += 2;
        let changed = c.g[A1][0] != c.g[A2][0];
        c.g[A2][0] = 0;
        if changed {
            c.cycles += 5;
            c.g[V0][0] = c.lw(oc449_at(c.g[S0][0], 0x80));
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.lw(oc449_at(c.g[V0][0], 0x784));
            call_ptr(c, 0x0031_28e8, 0x0012_df38)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `gp - off` as a 32-bit address (the gp-relative globals).
#[inline]
fn oc449_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `float RiderSpinSub_GetFlipHUDBlend(RiderSpinSub* self)` (0x136168): the
/// HUD blend of the flip indicator, from the spin value at `+0x1c`. The spin
/// is wrapped to a fraction (`x*k + 0.5`, truncated, minus one when that
/// rounded down), its distance from 0.5 is scaled by the globals at
/// `gp-0x72f4` and `gp-0x72f0`, mirrored past 1.0 (`2 - x`), offset by 0.25,
/// clamped at 0 and scaled by `gp-0x72ec` (4/3). Leaf; the result is in `f0`.
pub fn rider_spin_sub_get_flip_hud_blend(c: &mut Cpu) {
    let spin = Addr(oc449_at(c.g[A0][0], 0x1c)).read_fpu(c);
    c.cycles += 11;
    let k_wrap = Addr(oc449_gp(c, 0x72f8)).read_fpu(c);
    let half = FpuF32(0x3f00_0000);
    let one = FpuF32(0x3f80_0000);
    c.g[AT][0] = 0x3f00_0000;
    let shifted = spin * k_wrap + half;
    let trunc = FpuF32::from_i32(shifted.to_i32());
    let lt = shifted.lt(trunc);
    crate::abi::set_fcc(c, lt);
    let mut f1 = trunc;
    if lt {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        f1 = trunc - one;
    }

    c.cycles += 11;
    let k_b = Addr(oc449_gp(c, 0x72f4)).read_fpu(c);
    let k_c = Addr(oc449_gp(c, 0x72f0)).read_fpu(c);
    let mut f0 = f1 * k_b;
    c.g[AT][0] = 0x3f80_0000;
    f1 = one;
    f0 = spin - f0;
    f0 = f0.abs();
    let mut f2 = f0 * k_c;
    let lt2 = f1.lt(f2);
    crate::abi::set_fcc(c, lt2);
    if lt2 {
        c.cycles += 3;
        c.g[AT][0] = 0x4000_0000;
        f0 = FpuF32(0x4000_0000);
        f2 = f0 - f2;
    }

    c.cycles += 8;
    c.g[AT][0] = 0x3e80_0000;
    let quarter = FpuF32(0x3e80_0000);
    f1 = FpuF32(0);
    f2 = f2 - quarter;
    let k_d = Addr(oc449_gp(c, 0x72ec)).read_fpu(c);
    f2 = f2.max(f1);
    let result = f2 * k_d;
    c.f[1] = f1.0;
    c.f[2] = f2.0;
    c.f[3] = spin.0;
    crate::abi::set_fret(c, result);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::vtable;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// Self with `count` (`+off_count`) entries at `+off_list`; each entry
    /// holds a rider at `+0x18`. The rider's object (`+0x6c0`) has a vtable
    /// with a `getAIIndex` slot at `+0x38`.
    fn listed_setup(c: &mut Cpu, seed: u32, off_list: u32, off_count: u32, with_vt: bool) {
        let mut r = Rng(0x12aa_78 ^ seed as u64 ^ (off_list as u64) << 20);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        let count = seed % 4;
        c.bus.write32(DATA + off_count, count);
        for i in 0..count {
            let entry = DATA + 0x1000 + i * 0x100;
            let rider = DATA + 0x3000 + i * 0x1000;
            c.bus.write32(DATA + off_list + 4 * i, entry);
            c.bus.write32(entry + 0x18, rider);
            if with_vt {
                let vt = vtable(c, &mut r, &[0x38]);
                c.bus.write32(rider + 0x6c0, vt);
            }
        }
    }

    #[test]
    fn ai_listed_riders_have_render_slots_matches() {
        check(
            ssx3_ee::s00::f_0012aa78__generated,
            ai_listed_riders_have_render_slots,
            200,
            &|c, seed| {
                c.pc = 0x0012_aa78;
                listed_setup(c, seed, 0x48, 0x80, true);
            },
        );
    }

    /// The replay frame (vtable slot 2 is the header reader), self with its
    /// three lists (players, computers, actors) of 0..2 objects each; every
    /// object's `+0x77c` block has a vtable whose slot `+0x1c` is a fake.
    fn load_state_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x12b9_48 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        let frame = DATA + 0x8000;
        c.g[A1][0] = frame as u64;
        let fvt = vtable(c, &mut r, &[0x10]);
        c.bus.write32(frame, fvt);
        let counts = [seed % 3, (seed / 3) % 3, (seed / 9) % 3];
        c.bus.write32(DATA + 0x7c, counts[0]);
        c.bus.write32(DATA + 0x80, counts[1]);
        c.bus.write32(DATA + 0x84, counts[2]);
        let arrays = [0x40u32, 0x48, 0x5c];
        let mut idx = 0;
        for (l, &array_off) in arrays.iter().enumerate() {
            for i in 0..counts[l] {
                let elem = DATA + 0x10000 + idx * 0x100;
                let obj = DATA + 0x20000 + idx * 0x100;
                let p = DATA + 0x30000 + idx * 0x100;
                c.bus.write32(DATA + array_off + 4 * i, elem);
                c.bus.write32(elem + 0x18, obj);
                c.bus.write32(obj + 0x77c, p);
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(p + 0xde8, vt);
                idx += 1;
            }
        }
    }

    #[test]
    fn ai_load_state_matches() {
        check(ssx3_ee::s00::f_0012b948__generated, ai_load_state, 200, &|c, seed| {
            c.pc = 0x0012_b948;
            load_state_setup(c, seed);
        });
    }

    #[test]
    fn rider_wipeout_sub_play_end_anim_b_matches() {
        check(
            ssx3_ee::s00::f_0012de80__generated,
            rider_wipeout_sub_play_end_anim_b,
            200,
            &|c, seed| {
                let mut r = Rng(0x12de_80 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0012_de80;
                c.g[A0][0] = DATA as u64;
                let rider = DATA + 0x4000;
                let anim = DATA + 0x6000;
                c.bus.write32(DATA + 0x80, rider);
                c.bus.write32(rider + 0x784, anim);
                // getCurrentAnimID(anim, 2) reads the word at anim + 8:
                // below the table, each of its ten IDs, and above it.
                c.bus.write32(anim + 8, 370 + seed % 16);
            },
        );
    }

    #[test]
    fn rider_spin_sub_get_flip_hud_blend_matches() {
        check(
            ssx3_ee::s01::f_00136168__generated,
            rider_spin_sub_get_flip_hud_blend,
            300,
            &|c, seed| {
                let mut r = Rng(0x1361_68 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let spin = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(4.0)
                };
                c.bus.write32(DATA + 0x1c, spin);
            },
        );
    }

    #[test]
    fn ai_mission_riders_have_render_slots_matches() {
        check(
            ssx3_ee::s00::f_0012adc0__generated,
            ai_mission_riders_have_render_slots,
            200,
            &|c, seed| {
                c.pc = 0x0012_adc0;
                listed_setup(c, seed, 0x70, 0x88, false);
            },
        );
    }
}
