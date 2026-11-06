//! Staging wave 210 (oc287): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_b550, path_data_get_ai_path_index, "PathData_GetAIPathIndex"),
    (0x0035_4720, typed_process_ctor, "TypedProcess_ctor"),
    (0x0032_0058, mem_heap_memalign, "MEM_HeapMemalign"),
    (0x0010_cad8, computer_actor_rider_choose_trick, "ComputerActorRider_ChooseTrick"),
    (0x0013_8ad8, rider_wipeout_state_save_state, "RiderWipeoutState_SaveState"),
    (0x0012_e708, rider_wipeout_sub_save_state, "RiderWipeoutSub_SaveState"),
    (0x0010_cd20, computer_actor_rider_pick_fitted_trick, "ComputerActorRider_PickFittedTrick"),
    (0x0015_5bf0, be_aggression_int_add_aggression, "BEAggressionInt_AddAggression"),
    (0x0036_7bc0, tex_man_free_texture, "TexMan_FreeTexture"),
];

/// `PathData` AI path table: entry count at +0x8, entries (0x40 bytes each)
/// at +0xc.
const PATH_DATA_AI_COUNT: u32 = 0x8;
const PATH_DATA_AI_ENTRIES: u32 = 0xc;
const PATH_DATA_AI_ENTRY_SIZE: u32 = 0x40;

/// `int PathData_GetAIPathIndex(PathData* tree, AIPath* path)` (0x26b550):
/// the index of `path` in the tree's AI path array, or -1 (also for a null
/// path). The loop checks events at its head 0x26b570.
pub fn path_data_get_ai_path_index(c: &mut Cpu) {
    c.cycles += 2;
    let path = c.g[A1][0];
    if path != 0 {
        c.cycles += 3;
        let tree = Addr(c.g[A0][0] as u32);
        let count = tree.off(PATH_DATA_AI_COUNT).lw(c);
        c.g[V0][0] = count;
        c.g[V1][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 3;
            let mut entry = tree.off(PATH_DATA_AI_ENTRIES).lw(c);
            let mut index: u64 = 0;
            let mut count = count;
            c.g[A2][0] = count;
            loop {
                c.cycles += 2;
                if entry == path {
                    c.g[A0][0] = entry;
                    c.g[V1][0] = index;
                    c.cycles += 2;
                    c.g[V0][0] = index;
                    ret(c);
                    return;
                }
                index = sx((index as u32).wrapping_add(1));
                c.cycles += 3;
                let more = (index as i64) < (count as i64);
                entry = sx((entry as u32).wrapping_add(PATH_DATA_AI_ENTRY_SIZE));
                if !more {
                    c.g[A0][0] = entry;
                    c.g[V1][0] = index;
                    c.g[V0][0] = 0;
                    break;
                }
                if event_due(c) {
                    c.g[A0][0] = entry;
                    c.g[V1][0] = index;
                    c.g[V0][0] = 1;
                    deliver_event_here(c, 0x0026_b570);
                    entry = c.g[A0][0];
                    index = c.g[V1][0];
                    count = c.g[A2][0];
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    ret(c);
}

/// `TypedProcess` fields: vtable at +0xc, type at +0x10 (s16), +0x12 (s16).
const TYPED_PROCESS_VTABLE: u32 = 0xc;
const TYPED_PROCESS_TYPE: u32 = 0x10;
const TYPED_PROCESS_FLAGS: u32 = 0x12;
/// The `TypedProcess` vtable.
const TYPED_PROCESS_VT: u32 = 0x0049_1e80;
/// Live process count per type (`int[]`).
const TYPED_PROCESS_COUNTS: u32 = 0x0044_aff0;

/// `TypedProcess* TypedProcess_ctor(TypedProcess* self, void* list_arg,
/// int type)` (0x354720): construct the base `GameProcess` (registers it in
/// its process list), then set the vtable, store the type, clear +0x12 and
/// count one more live process of that type.
pub fn typed_process_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        Callee { addr: 0x0035_4648, f: ee::s11::GameProcess_ctor }.call(c, 0x0035_473c)?;

        let this = Addr(c.g[S0][0] as u32);
        let ty = c.g[S1][0] as u32;
        this.off(TYPED_PROCESS_TYPE).sh(c, ty as u64);
        this.off(TYPED_PROCESS_VTABLE).sw(c, TYPED_PROCESS_VT as u64);
        this.off(TYPED_PROCESS_FLAGS).sh(c, 0);
        let slot = TYPED_PROCESS_COUNTS.wrapping_add(((ty << 16) as i32 >> 14) as u32);
        c.g[A0][0] = sx(slot);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        let n = sx((Addr(slot).lw(c) as u32).wrapping_add(1));
        c.g[V1][0] = n;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        Addr(slot).sw(c, n);
        frame.close(c);
        Ok(())
    })
}

/// `void* MEM_HeapMemalign(DLHeap* heap, uint align, uint size)` (0x320058):
/// forwards to `DLHeap_Memalign` (0x31f2c8) in a 0x10 frame holding `ra`.
pub fn mem_heap_memalign(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Direct(Callee { addr: 0x0031_f2c8, f: ee::s10::DLHeap_Memalign }),
        0x0032_0068,
    );
}

/// `ComputerActorRider` trick-pick fields.
const CAR_SKIP_PICK: u32 = 0xe20;

/// `int ComputerActorRider_ChooseTrick(ComputerActorRider* ai, float height)`
/// (0x10cad8): the trick to start at `height` above the ground, or -1. No
/// pick when the AI is told to skip (+0xe20) and is below 1 unit, at or
/// below 0.75 units, or at exactly 100 units; otherwise the uber, the
/// fitted and the random trick are rolled in turn, and the first roll that
/// succeeds picks (its picker may still answer -1, which moves on to the
/// next roll). Frame 0x50: ra at 0, s2/s1/s0 at 0x10/0x20/0x30, f20 at 0x40.
pub fn computer_actor_rider_choose_trick(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    const NONE: u64 = u64::MAX;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x40);
        c.g[S1][0] = NONE;
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let ai = Addr(c.g[S0][0] as u32);
        c.g[V0][0] = ai.off(CAR_SKIP_PICK).lw(c);
        c.f[20] = c.f[12];
        let height = FpuF32(c.f[20]);
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.f[0] = 0x3f80_0000;
            let low = height.lt(FpuF32(c.f[0]));
            crate::abi::set_fcc(c, low);
            if low {
                c.g[V1][0] = 1;
            }
        }

        c.cycles += 2;
        c.g[V0][0] = c.g[S1][0];
        let mut picked = true;
        if c.g[V1][0] == 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f40_0000;
            c.f[0] = 0x3f40_0000;
            let high_enough = FpuF32(c.f[0]).lt(height);
            crate::abi::set_fcc(c, high_enough);
            if !high_enough {
                picked = false;
            } else {
                c.cycles += 6;
                c.g[AT][0] = 0x42c8_0000;
                c.f[0] = 0x42c8_0000;
                let at_100 = height.eq_s(FpuF32(c.f[0]));
                crate::abi::set_fcc(c, at_100);
                if at_100 {
                    picked = false;
                } else {
                    choose_trick_rolls(c)?;
                }
            }
        }
        if picked {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 6;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The three rolls of [`computer_actor_rider_choose_trick`]: each
/// `Roll*Trick(ai)` that succeeds calls its `Pick*Trick(ai, height)`; the
/// first pick other than -1 wins. Leaves the pick in s1 and v0.
fn choose_trick_rolls(c: &mut Cpu) -> crate::call::Flow {
    const NONE: u64 = u64::MAX;
    let stages: [(Callee, u32, Callee, u32, u64); 3] = [
        (
            Callee { addr: 0x0010_c258, f: ee::s00::ComputerActorRider_RollUberTrick },
            0x0010_cb60,
            Callee { addr: 0x0010_cf68, f: ee::s00::ComputerActorRider_PickUberTrick },
            0x0010_cb70,
            3,
        ),
        (
            Callee { addr: 0x0010_c320, f: ee::s00::ComputerActorRider_RollFittedTrick },
            0x0010_cb88,
            Callee { addr: 0x0010_cd20, f: ee::s00::ComputerActorRider_PickFittedTrick },
            0x0010_cb98,
            2,
        ),
        (
            Callee { addr: 0x0010_c3b8, f: ee::s00::ComputerActorRider_RollRandomTrick },
            0x0010_cbac,
            Callee { addr: 0x0010_cbe0, f: ee::s00::ComputerActorRider_PickRandomTrick },
            0x0010_cbbc,
            1,
        ),
    ];
    for (n, (roll, roll_ret, pick, pick_ret, tail)) in stages.into_iter().enumerate() {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        roll.call(c, roll_ret)?;
        let rolled = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];
        if rolled {
            c.cycles += 3;
            c.f[12] = c.f[20];
            pick.call(c, pick_ret)?;
            c.g[S1][0] = c.g[V0][0];
        }
        c.cycles += tail;
        if n == 0 {
            c.g[S2][0] = NONE;
        }
        c.g[V0][0] = c.g[S1][0];
        if n < 2 && c.g[S1][0] != c.g[S2][0] {
            return Ok(());
        }
    }
    Ok(())
}

/// `stream->Write(self, size)`: the shared body of the small `SaveState`
/// methods. The stream's vtable slot 1 (`{delta, fn}` at +8) is called with
/// `(stream + delta, self, size)` in a 0x10 frame holding `ra`; `ret` is
/// the return address after the call.
fn save_state_write(c: &mut Cpu, size: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = size;
        let stream = c.g[A3][0] as u32;
        c.g[V1][0] = Addr(stream).lw(c);
        let vt = Addr(c.g[V1][0] as u32);
        c.g[A0][0] = vt.off(8).lh(c);
        c.g[V0][0] = vt.off(0xc).lw(c);
        let write = c.g[V0][0] as u32;
        c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, write, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderWipeoutState_SaveState(RiderWipeoutState* self, Stream* s)`
/// (0x138ad8): writes the 0x40-byte state to the stream.
pub fn rider_wipeout_state_save_state(c: &mut Cpu) {
    save_state_write(c, 0x40, 0x0013_8b00);
}

/// `void RiderWipeoutSub_SaveState(RiderWipeoutSub* self, Stream* s)`
/// (0x12e708): writes the 0x80-byte state to the stream.
pub fn rider_wipeout_sub_save_state(c: &mut Cpu) {
    save_state_write(c, 0x80, 0x0012_e730);
}

/// `ComputerActorRider` fields the trick pickers write: uber / fitted
/// flags, the pick's slack before and after, the height it was picked at,
/// and the per-trick fitted pick counts (15 ints).
const CAR_PICK_UBER: u32 = 0xe24;
const CAR_PICK_FITTED: u32 = 0xe28;
const CAR_PICK_LEAD: u32 = 0xe2c;
const CAR_PICK_HEIGHT: u32 = 0xe30;
const CAR_PICK_SLACK: u32 = 0xe34;
const CAR_FITTED_COUNTS: u32 = 0xef4;
const CAR_RIDER: u32 = 0x18;
/// Rider fields: air height, uber level, animation controller, flags.
const RIDER_AIR_HEIGHT: u32 = 0x2f0;
const RIDER_UBER_LEVEL: u32 = 0x2f4;
const RIDER_VTABLE: u32 = 0x6c0;
const RIDER_ANIM: u32 = 0x784;
const RIDER_FLAGS: u32 = 0xb2c;
/// "No trick / no animation".
const TRICK_NONE: u64 = 0x1b6;
/// The fitted tricks are 1..14 of the BAGT table.
const FITTED_TRICKS: i64 = 15;

/// The rider's virtual "index" query (slot `{delta, fn}` at +0x38 of the
/// vtable at `rider+0x6c0`), as the pickers make it: `a1` holds the rider
/// (`a0` in the uber-animation call), `v1` its vtable; returns to `ret`.
fn pick_fitted_rider_index(c: &mut Cpu, base: usize, delta_reg: usize, ret: u32) -> crate::call::Flow {
    let vt = Addr(c.g[V1][0] as u32);
    c.g[base][0] = sx((c.g[base][0] as u32).wrapping_add(RIDER_VTABLE));
    c.g[delta_reg][0] = vt.off(0x38).lh(c);
    c.g[V0][0] = vt.off(0x3c).lw(c);
    let f = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[base][0] as u32).wrapping_add(c.g[delta_reg][0] as u32));
    call_ptr(c, f, ret)
}

/// The AI's rider (`*(s0 + 0x18)`, the AI read from s0).
fn pick_fitted_ai_rider(c: &mut Cpu) -> u64 {
    Addr(c.g[S0][0] as u32).off(CAR_RIDER).lw(c)
}

/// `AnimController_GetCurrentAnimTriggerTime(rider->anim, anim, which)`
/// (0x312820) as the pickers call it (`v_reg` holds the AI's rider).
fn pick_fitted_trigger_time(c: &mut Cpu, v_reg: usize, which: u64, ret: u32) -> crate::call::Flow {
    c.g[A2][0] = which;
    c.g[A0][0] = Addr(c.g[v_reg][0] as u32).off(RIDER_ANIM).lw(c);
    call_ptr(c, 0x0031_2820, ret)
}

/// `int ComputerActorRider_PickFittedTrick(ComputerActorRider* ai, float
/// height)` (0x10cd20): among the fitted tricks 1..14 (those with an
/// animation), the one whose wind-up plus trick time (trigger times over
/// the rider's trick skill factor) fits in `height` and that was picked
/// least often so far (ties: the first). Records the pick (`+0xe28` = 1,
/// lead / slack at `+0xe2c` / `+0xe34`, height at `+0xe30`), counts it and
/// returns its index, or -1. While the rider is in the air with an uber
/// trick ready (flag bit 0), only tricks without an uber animation count.
/// The loop checks events at its head 0x10cd98. Frame 0xa0: ra at 0,
/// s6..s0 at 0x10..0x70, f20..f23 at 0x80..0x98.
pub fn computer_actor_rider_pick_fitted_trick(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x70);
        frame.save_fpr(c, 23, 0x98);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.f[23] = c.f[12];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = 1;
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        c.g[S4][0] = u64::MAX;
        frame.save(c, S6, 0x10);
        c.g[S5][0] = 0x2710;
        frame.save_fpr(c, 22, 0x90);
        c.g[S6][0] = TRICK_NONE;
        frame.save(c, S1, 0x60);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        let ai = Addr(c.g[S0][0] as u32);
        ai.off(CAR_PICK_UBER).sw(c, 0);
        let height = FpuF32(c.f[23]);
        ai.off(CAR_PICK_HEIGHT).write_fpu(c, height);

        // The BAGT interface and the rider's trick skill factor.
        call_ptr(c, 0x0014_dc80, 0x0010_cd78)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 6;
        call_ptr(c, 0x0014_dd58, 0x0010_cd84)?;
        c.g[A0][0] = pick_fitted_ai_rider(c);
        c.g[S3][0] = c.g[V0][0];
        Callee { addr: 0x0012_0038, f: ee::s00::Rider_GetTrickSkillFactor }.call(c, 0x0010_cd90)?;
        c.f[22] = c.f[0];
        c.g[A0][0] = pick_fitted_ai_rider(c);

        loop {
            // The trick's uber animation index (0x1b6: not a fitted trick).
            c.cycles += 13;
            c.g[V1][0] = Addr(c.g[A0][0] as u32).off(RIDER_VTABLE).lw(c);
            pick_fitted_rider_index(c, A0, A1, 0x0010_cdb0)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, 0x0015_0138, 0x0010_cdc0)?; // BEBAGTInt_GetTrickField6
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] == c.g[S6][0] {
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            } else {
                pick_fitted_try(c)?;
            }

            c.cycles += 3;
            let more = (c.g[S2][0] as i64) < FITTED_TRICKS;
            c.g[V0][0] = more as u64;
            if !more {
                break;
            }
            c.g[A0][0] = pick_fitted_ai_rider(c);
            if event_due(c) {
                deliver_event_here(c, 0x0010_cd98);
            }
        }

        // Count the pick.
        c.cycles += 3;
        let best = c.g[S4][0];
        c.g[V0][0] = ((best as i64) < FITTED_TRICKS) as u64;
        if best != u64::MAX {
            c.cycles += 2;
            let in_range = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(CAR_FITTED_COUNTS));
            if in_range {
                c.cycles += 5;
                c.g[A0][0] = sx((best as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let slot = Addr(c.g[V0][0] as u32);
                c.g[V1][0] = sx((slot.lw(c) as u32).wrapping_add(1));
                let n = c.g[V1][0];
                slot.sw(c, n);
            }
        }
        c.cycles += 15;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x98);
        frame.restore_fpr(c, 22, 0x90);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// One fitted trick (index s2, uber animation s1 != 0x1b6) of
/// [`computer_actor_rider_pick_fitted_trick`]: skip it while airborne with
/// an uber animation, else time it and keep it when it fits and was picked
/// less often than the best so far. Advances s2.
fn pick_fitted_try(c: &mut Cpu) -> crate::call::Flow {
    use crate::ps2f::FpuF32;
    c.cycles += 5;
    c.g[A1][0] = pick_fitted_ai_rider(c);
    let uber_ready = Addr(c.g[A1][0] as u32).off(RIDER_FLAGS).lw(c) & 1 != 0;
    c.g[V0][0] = TRICK_NONE;
    if uber_ready {
        c.cycles += 15;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(RIDER_VTABLE).lw(c);
        pick_fitted_rider_index(c, A1, A0, 0x0010_cdf8)?;
        c.g[V1][0] = pick_fitted_ai_rider(c);
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S2][0];
        let level = Addr(c.g[V1][0] as u32).off(RIDER_UBER_LEVEL).lw(c);
        c.g[A3][0] = ((level as i64) < 5) as u64 ^ 1;
        call_ptr(c, 0x0015_0198, 0x0010_ce18)?; // BEBAGTInt_GetTrickAnim
        c.g[A1][0] = pick_fitted_ai_rider(c);
    }

    c.cycles += 6;
    c.f[1] = 0;
    let rider = Addr(c.g[A1][0] as u32);
    c.f[0] = rider.off(RIDER_AIR_HEIGHT).read_fpu(c).0;
    let airborne = FpuF32(c.f[1]).lt(FpuF32(c.f[0]));
    crate::abi::set_fcc(c, airborne);
    if airborne {
        c.cycles += 2;
        if c.g[V0][0] != c.g[S6][0] {
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            return Ok(());
        }
        c.cycles += 1;
    }
    c.g[V1][0] = rider.off(RIDER_VTABLE).lw(c);

    // Wind-up (field 0's animation) and trick time.
    c.cycles += 32;
    pick_fitted_rider_index(c, A1, A0, 0x0010_ce54)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, 0x0015_00d8, 0x0010_ce64)?; // BEBAGTInt_GetTrickField0
    c.g[V1][0] = pick_fitted_ai_rider(c);
    c.g[A1][0] = c.g[V0][0];
    pick_fitted_trigger_time(c, V1, 1, 0x0010_ce78)?;
    c.g[V0][0] = pick_fitted_ai_rider(c);
    c.f[20] = FpuF32(c.f[0]).div(FpuF32(c.f[22])).0;
    c.g[A1][0] = c.g[S1][0];
    pick_fitted_trigger_time(c, V0, 1, 0x0010_ce90)?;
    c.g[V0][0] = pick_fitted_ai_rider(c);
    c.f[21] = FpuF32(c.f[0]).div(FpuF32(c.f[22])).0;
    c.g[A1][0] = c.g[S1][0];
    pick_fitted_trigger_time(c, V0, 2, 0x0010_cea8)?;
    let end = FpuF32(c.f[0]).div(FpuF32(c.f[22]));
    c.f[0] = end.0;
    let total = end + FpuF32(c.f[20]);
    c.f[1] = total.0;
    let height = FpuF32(c.f[23]);
    let fits = total.lt(height);
    crate::abi::set_fcc(c, fits);
    c.g[A0][0] = sx((c.g[S2][0] as u32) << 2);
    let ai = Addr(c.g[S0][0] as u32);
    if fits {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(CAR_FITTED_COUNTS));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let count = Addr(c.g[V1][0] as u32).lw(c);
        let fewer = (count as i64) < (c.g[S5][0] as i64);
        c.g[V0][0] = 1;
        if fewer {
            c.cycles += 7;
            let lead = end - FpuF32(c.f[21]);
            let slack = height - total;
            c.f[0] = lead.0;
            c.f[1] = slack.0;
            ai.off(CAR_PICK_FITTED).sw(c, 1);
            c.g[S4][0] = c.g[S2][0];
            ai.off(CAR_PICK_LEAD).write_fpu(c, lead);
            ai.off(CAR_PICK_SLACK).write_fpu(c, slack);
            c.g[S5][0] = Addr(c.g[V1][0] as u32).lw(c);
        }
    }
    c.cycles += 1;
    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
    Ok(())
}

/// The aggression records: `[profile][character]` blocks of 0xf88 bytes
/// (0x9b50 per profile), with a 3-byte `{state, level, points}` entry per
/// rival character at +0xbc1.
const AGGRESSION_RECORDS: u32 = 0x004a_6ca8;
const AGGRESSION_PROFILE_SIZE: u64 = 0x9b50;
const AGGRESSION_CHAR_SIZE: u64 = 0xf88;
const AGGRESSION_STATE: u32 = 0xbc1;
const AGGRESSION_LEVEL: u32 = 0xbc2;
const AGGRESSION_POINTS: u32 = 0xbc3;
/// Set while aggression messages are suppressed.
const AGGRESSION_MESSAGES_OFF: u32 = 0x0053_05f9;

/// Where [`be_aggression_int_add_aggression`]'s level-up rules continue.
enum AggressionExit {
    /// Reset to level 1 / 5 points (`0x155dcc`), then as `Clamped`.
    Floor,
    /// A clamped level (`0x155de8`): reload a0, then as `Kept`.
    Clamped,
    /// `0x155dec`: reload a1.
    Kept,
    /// Store (`0x155df0`).
    Store,
}

/// `void BEAggressionInt_AddAggression(BEAggressionInt* self, int rider,
/// int rival, int event)` (0x155bf0): add the event's aggression points
/// (events 0..3: 1, 2, 4, 6) to `rider`'s record of `rival`; the level is
/// points / 5. On a level-up the player (rider 0) gets the aggression
/// message unless messages are off, and the record's state clamps the
/// level: state 0 caps at level 2 (10 points), state 1 turns into state 3
/// at level 4 (level 2, 10 points), state 2 stays within 1..3 and state 3
/// within 1..4 (5 points per level). Frame 0xa0: ra at 0, fp at 0x10,
/// s7..s0 at 0x20..0x90.
pub fn be_aggression_int_add_aggression(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S4, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S1, 0x80);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S3, 0x60);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S4][0];
        Callee { addr: 0x0014_a0e0, f: ee::s01::BE_GetRiderProfile }.call(c, 0x0015_5c30)?;
        c.g[S7][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S4][0];
        Callee { addr: 0x0014_a080, f: ee::s01::BE_GetRiderCharacter }.call(c, 0x0015_5c3c)?;
        c.g[S6][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        Callee { addr: 0x0014_a080, f: ee::s01::BE_GetRiderCharacter }.call(c, 0x0015_5c48)?;

        // The record: profile s7, character s6, rival character s5.
        c.g[A0][0] = AGGRESSION_CHAR_SIZE;
        c.g[A1][0] = AGGRESSION_PROFILE_SIZE;
        c.mult(0, A0, S6, A0);
        c.g[S5][0] = c.g[V0][0];
        c.mult(0, A1, S7, A1);
        let rival3 = (c.g[S5][0] as u32).wrapping_mul(3);
        c.g[FP][0] = 0x4a_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = sx(rival3.wrapping_add(c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V1][0] = sx(AGGRESSION_RECORDS);
        c.g[A2][0] = 1;
        let rec = AGGRESSION_RECORDS.wrapping_add(c.g[V0][0] as u32);
        c.g[A1][0] = sx(rec);
        c.g[V1][0] = sx(rec);
        let rec = Addr(rec);
        c.g[A0][0] = rec.off(AGGRESSION_LEVEL).lb(c);
        c.g[S1][0] = rec.off(AGGRESSION_POINTS).lb(c);
        c.g[S3][0] = rec.off(AGGRESSION_STATE).lb(c);

        // Points for the event.
        let event = c.g[S2][0] as i64;
        let (add, cycles) = match event {
            1 => (2, 4),
            0 => (1, 9),
            2 => (4, 9),
            3 => (6, 10),
            e if e < 0 => (0, 7),
            _ => (0, 9),
        };
        c.cycles += cycles;
        c.g[A3][0] = add;
        c.g[V0][0] = 5;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(add as u32));

        c.cycles += 3;
        c.div(0, S1, V0);
        c.cycles += 5;
        c.g[S0][0] = c.lo[0];
        c.g[S2][0] = c.g[S0][0];
        let level_up = (c.g[A0][0] as i64) < (c.g[S2][0] as i64);
        c.g[V0][0] = level_up as u64;
        let exit = if !level_up {
            c.g[A0][0] = AGGRESSION_CHAR_SIZE;
            AggressionExit::Kept
        } else {
            aggression_level_up(c)?
        };
        if let AggressionExit::Floor = exit {
            c.cycles += 3;
            c.g[S2][0] = 1;
            c.g[S1][0] = 5;
        }
        if matches!(exit, AggressionExit::Floor | AggressionExit::Clamped) {
            c.cycles += 1;
            c.g[A0][0] = AGGRESSION_CHAR_SIZE;
        }
        if !matches!(exit, AggressionExit::Store) {
            c.cycles += 1;
            c.g[A1][0] = AGGRESSION_PROFILE_SIZE;
        }

        // Store the record.
        c.cycles += 25;
        c.mult(0, A0, S6, A0);
        c.mult(0, A1, S7, A1);
        let rival3 = (c.g[S5][0] as u32).wrapping_mul(3);
        c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6ca8));
        let off = rival3.wrapping_add(c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32);
        c.g[V0][0] = sx(off);
        let rec = (c.g[V1][0] as u32).wrapping_add(off);
        c.g[A0][0] = sx(rec);
        c.g[A1][0] = sx(rec);
        c.g[V1][0] = sx(rec);
        let rec = Addr(rec);
        let (points, level, state) = (c.g[S1][0], c.g[S2][0], c.g[S3][0]);
        rec.off(AGGRESSION_POINTS).sb(c, points);
        rec.off(AGGRESSION_LEVEL).sb(c, level);
        rec.off(AGGRESSION_STATE).sb(c, state);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The level-up part of [`be_aggression_int_add_aggression`]: the message
/// for the player, then the state's level rules (level s0 = s2, points s1,
/// state s3).
fn aggression_level_up(c: &mut Cpu) -> Result<AggressionExit, crate::call::Unwound> {
    use AggressionExit::*;
    c.cycles += 2;
    c.g[V0][0] = 1;
    if c.g[S4][0] == 0 {
        c.cycles += 4;
        c.g[V1][0] = Addr(AGGRESSION_MESSAGES_OFF).lb(c);
        c.g[V0][0] = 1;
        if c.g[V1][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S1][0];
            // MessageCenter_CheckAggressionMessage(rival, points)
            call_ptr(c, 0x001e_2a08, 0x0015_5d38)?;
            c.g[V0][0] = 1;
        }
    }
    let state = c.g[S3][0] as i64;
    let level = c.g[S0][0] as i64;
    c.cycles += 2;
    c.g[V0][0] = (state < 2) as u64;
    if state == 1 {
        c.cycles += 3;
        c.g[V0][0] = (level < 4) as u64;
        c.g[A0][0] = AGGRESSION_CHAR_SIZE;
        if level >= 4 {
            c.cycles += 4;
            c.g[S2][0] = 2;
            c.g[S1][0] = 10;
            c.g[S3][0] = 3;
        }
        return Ok(Kept);
    }
    c.cycles += 2;
    c.g[V0][0] = 2;
    if state < 2 {
        c.cycles += 2;
        c.g[A0][0] = AGGRESSION_CHAR_SIZE;
        if state == 0 {
            c.cycles += 3;
            c.g[V0][0] = (level < 3) as u64;
            c.g[A1][0] = AGGRESSION_PROFILE_SIZE;
            if level < 3 {
                return Ok(Store);
            }
            c.cycles += 3;
            c.g[S2][0] = 2;
            c.g[S1][0] = 10;
            return Ok(Clamped);
        }
        c.cycles += 2;
        c.g[A1][0] = AGGRESSION_PROFILE_SIZE;
        return Ok(Store);
    }
    c.cycles += 2;
    c.g[V0][0] = 3;
    if state == 2 {
        c.cycles += 2;
        c.g[V0][0] = (level < 4) as u64;
        if level <= 0 {
            return Ok(Floor);
        }
        c.cycles += 2;
        c.g[A0][0] = AGGRESSION_CHAR_SIZE;
        if level >= 4 {
            c.cycles += 3;
            c.g[S2][0] = 3;
            c.g[S1][0] = 15;
        }
        return Ok(Kept);
    }
    c.cycles += 2;
    c.g[A0][0] = AGGRESSION_CHAR_SIZE;
    if state == 3 {
        c.cycles += 2;
        c.g[V0][0] = (level < 5) as u64;
        if level <= 0 {
            return Ok(Floor);
        }
        c.cycles += 2;
        c.g[A0][0] = AGGRESSION_CHAR_SIZE;
        if level >= 5 {
            c.cycles += 2;
            c.g[S2][0] = 4;
            c.g[S1][0] = 20;
            return Ok(Clamped);
        }
        return Ok(Kept);
    }
    c.cycles += 2;
    c.g[A1][0] = AGGRESSION_PROFILE_SIZE;
    Ok(Store)
}

/// `TexMan` fields: texture pointers by id (+8), the cached lookups reset
/// on a free (+0x1f4c, +0x1f50, +0x1f54), the VRAM heaps (+0x1f60, and
/// +0x4350 for kind 9 textures).
const TEX_MAN_TEXTURES: u32 = 0x8;
const TEX_MAN_CACHE: u32 = 0x1f50;
const TEX_MAN_CACHE_LAST: u32 = 0x1f54;
const TEX_MAN_VRAM: u32 = 0x1f60;
const TEX_MAN_VRAM_KIND9: u32 = 0x4350;
/// Texture fields: owned flag, kind, the two CPU buffers, the two VRAM
/// blocks (-1: none) and their ids.
const TEX_OWNED: u32 = 0x0;
const TEX_KIND: u32 = 0xc;
const TEX_BUF_A: u32 = 0x1c;
const TEX_BUF_B: u32 = 0x20;
const TEX_SLOT_A: u32 = 0x28;
const TEX_SLOT_B: u32 = 0x2c;
const TEX_ID_A: u32 = 0x30;
const TEX_ID_B: u32 = 0x34;
/// Ids below this are B textures (`TexMan_FreeBTexInfo`), others NB.
const TEX_FIRST_NB: i64 = 1500;

/// `void TexMan_FreeTexture(TexMan* self, int id)` (0x367bc0): free a
/// texture's VRAM blocks (block A in the kind-9 heap for kind 9 textures),
/// its CPU buffers when it owns them, its info record (B or NB by id), and
/// reset the cached lookups (+0x1f4c..+0x1f54 = -1). The two-word reset
/// loop checks events at 0x367c90. Frame 0x50: ra at 0, s3..s0 at
/// 0x10..0x40.
pub fn tex_man_free_texture(c: &mut Cpu) {
    const VRAM_FREE: Callee = Callee { addr: 0x0036_63d8, f: ee::s12::VramHeap_FreeBlock };
    const NONE: u64 = u64::MAX;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let man = c.g[S1][0] as u32;
        let id = c.g[S2][0] as u32;
        c.g[V0][0] = sx(man.wrapping_add(id << 2));
        // (s0..s3 hold the texture, manager, id and -1 across the calls.)
        frame.save(c, S0, 0x40);
        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(TEX_MAN_TEXTURES).lw(c);
        c.g[S3][0] = NONE;
        let tex = Addr(c.g[S0][0] as u32);
        let slot_a = tex.off(TEX_SLOT_A).lw(c);
        c.g[V1][0] = sx(man.wrapping_add(TEX_MAN_VRAM_KIND9));
        if slot_a != NONE {
            c.cycles += 6;
            let kind = tex.off(TEX_KIND).lw(c);
            c.g[A0][0] = sx(man.wrapping_add(TEX_MAN_VRAM));
            c.g[A1][0] = tex.off(TEX_ID_A).lw(c);
            c.g[V0][0] = kind ^ 9;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = c.g[V1][0];
            }
            VRAM_FREE.call(c, 0x0036_7c14)?;
        }

        c.cycles += 3;
        let tex = Addr(c.g[S0][0] as u32);
        let man = c.g[S1][0] as u32;
        c.g[V0][0] = tex.off(TEX_SLOT_B).lw(c);
        if c.g[V0][0] != NONE {
            c.cycles += 4;
            c.g[A1][0] = tex.off(TEX_ID_B).lw(c);
            c.g[A0][0] = sx(man.wrapping_add(TEX_MAN_VRAM));
            VRAM_FREE.call(c, 0x0036_7c2c)?;
        }
        let tex = Addr(c.g[S0][0] as u32);
        c.g[V0][0] = tex.off(TEX_OWNED).lw(c);

        c.cycles += 2;
        let owned = c.g[V0][0] != 0;
        let is_b = (c.g[S2][0] as i64) < TEX_FIRST_NB;
        c.g[V0][0] = is_b as u64;
        if owned {
            c.cycles += 3;
            c.g[A0][0] = tex.off(TEX_BUF_A).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0036_7c4c)?; // EA_MemFree
            }
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(TEX_BUF_B).lw(c);
            c.cycles += 2;
            c.g[V0][0] = ((c.g[S2][0] as i64) < TEX_FIRST_NB) as u64;
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0036_7c60)?; // EA_MemFree
                c.g[V0][0] = ((c.g[S2][0] as i64) < TEX_FIRST_NB) as u64;
            }
        }

        c.cycles += 2;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            Callee { addr: 0x0036_72c0, f: ee::s12::TexMan_FreeBTexInfo }.call(c, 0x0036_7c74)?;
        } else {
            c.cycles += 3;
            Callee { addr: 0x0036_71c8, f: ee::s12::TexMan_FreeNBTexInfo }.call(c, 0x0036_7c84)?;
        }
        c.g[A0][0] = NONE;

        // Reset the cached lookups (+0x1f50, +0x1f4c), then +0x1f54.
        c.cycles += 2;
        c.g[V1][0] = 1;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(TEX_MAN_CACHE));
        loop {
            c.cycles += 7;
            let none = c.g[A0][0];
            Addr(c.g[V0][0] as u32).sw(c, none);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_7c90);
            }
        }
        c.cycles += 9;
        c.g[V0][0] = NONE;
        frame.restore(c, S0, 0x40);
        Addr((c.g[S1][0] as u32).wrapping_add(TEX_MAN_CACHE_LAST)).sw(c, NONE);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::{check, scramble, Rng, DATA, RA as SENTINEL};

    /// A tree whose AI path array holds 0..6 entries, asked for a null path,
    /// one of its entries, or a pointer that is not in the array.
    fn path_index_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        scramble(c, r);
        let tree = DATA;
        let entries = DATA + 0x1000;
        let count = match seed % 7 {
            0 => 0,
            1 => (-(1 + (r.next_u32() % 3) as i32)) as u32,
            _ => 1 + r.next_u32() % 6,
        };
        c.bus.write32(tree + PATH_DATA_AI_COUNT, count);
        c.bus.write32(tree + PATH_DATA_AI_ENTRIES, entries);
        let n = if (count as i32) > 0 { count } else { 1 };
        let path = match seed % 5 {
            0 => 0,
            1 => entries + 0x40 * n + 4,
            2 => 0xdead_0000,
            _ => entries + 0x40 * (r.next_u32() % n),
        };
        c.g[A0][0] = sx(tree);
        c.g[A1][0] = sx(path);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0026_b550;
    }

    /// `PathData_GetAIPathIndex`: found, not found, null, empty array.
    #[test]
    fn path_data_get_ai_path_index_matches() {
        check(
            ee::s07::f_0026b550__generated,
            path_data_get_ai_path_index,
            300,
            &|c, seed| {
                let mut r = Rng(0x26b5_5000 ^ seed as u64);
                path_index_fixture(c, &mut r, seed);
            },
        );
    }

    /// Event delivery at the search loop head (0x26b570).
    #[test]
    fn path_data_get_ai_path_index_events_match() {
        check_events(
            ee::s07::f_0026b550__generated,
            path_data_get_ai_path_index,
            150,
            &|c, seed| {
                let mut r = Rng(0x26b5_5001 ^ seed as u64);
                path_index_fixture(c, &mut r, 3 + seed % 4);
                // Long array, path near the end (or absent).
                c.bus.write32(DATA + PATH_DATA_AI_COUNT, 6);
                let path = if seed % 2 == 0 { DATA + 0x1000 + 0x40 * 5 } else { 4 };
                c.g[A1][0] = sx(path);
                schedule(c, &mut r, seed, 0x0026_b550)
            },
        );
    }

    /// `TypedProcess_ctor` with the real `GameProcess_ctor`/`ProcessList_Add`
    /// on a small process list (both list kinds, by the entry's flag bit),
    /// types with junk in the upper half and negative types.
    #[test]
    fn typed_process_ctor_matches() {
        use crate::difftest::GAME_GP;
        const LIST: u32 = GAME_GP + 0x2898;
        check(ee::s11::f_00354720__generated, typed_process_ctor, 300, &|c, seed| {
            let mut r = Rng(0x3547_2000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let table = DATA + 0x2000;
            c.bus.write32(LIST + 4, table);
            let kind = r.next_u32() % 4;
            for k in 0..4 {
                let e = table + 0x44 * k;
                c.bus.write32(e, r.next_u32());
                c.bus.write32(e + 0x8, DATA + 0x3000 + 0x40 * k);
                c.bus.write32(e + 0x28, DATA + 0x3400 + 0x40 * k);
            }
            let ty = match seed % 4 {
                0 => r.next_u32() % 16,
                1 => (r.next_u32() << 16) | (r.next_u32() % 16),
                2 => 0xffff_0000 | (0x1_0000 - 1 - r.next_u32() % 4),
                _ => r.next_u32() % 64,
            };
            let base = TYPED_PROCESS_COUNTS.wrapping_add(((ty << 16) as i32 >> 14) as u32);
            c.bus.write32(base, r.next_u32() % 100);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = kind as u64;
            c.g[A2][0] = ty as u64 | ((r.next_u32() as u64) << 32);
            c.g[RA][0] = SENTINEL as u64;
            c.pc = 0x0035_4720;
        });
    }

    /// `MEM_HeapMemalign` on a real dlmalloc heap (`DLHeap_InitArena` on a
    /// 256 KiB arena, a few mallocs first), with small, power-of-two and odd
    /// alignments and sizes from zero to too large.
    #[test]
    fn mem_heap_memalign_matches() {
        use crate::difftest::fakes::machine;
        const HEAP: u32 = DATA + 0x2000;
        const ARENA: u32 = DATA + 0x4_0000;
        const BACK: u32 = 0x0000_7770;
        fn heap_call(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) {
            for (i, a) in args.iter().enumerate() {
                c.g[[A0, A1, A2][i]][0] = *a;
            }
            c.g[RA][0] = BACK as u64;
            c.pc = addr;
            f(c);
            assert_eq!(c.pc, BACK, "setup call {addr:#x} did not return");
        }
        check(ee::s10::f_00320058__generated, mem_heap_memalign, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0032_0058, 0);
            heap_call(
                c,
                0x0031_ff60,
                ee::s10::DLHeap_InitArena,
                &[HEAP as u64, ARENA as u64, 0x4_0000],
            );
            for _ in 0..r.next_u32() % 4 {
                let n = (r.next_u32() % 0x300) as u64;
                heap_call(c, 0x0031_e818, ee::s10::DLHeap_Malloc, &[HEAP as u64, n]);
            }
            c.g[RA][0] = SENTINEL as u64;
            c.pc = 0x0032_0058;
            c.g[A0][0] = sx(HEAP);
            c.g[A1][0] = match seed % 6 {
                0 => (r.next_u32() % 17) as u64,
                1 => 1u64 << (5 + r.next_u32() % 8),
                _ => (17 + r.next_u32() % 4096) as u64,
            };
            c.g[A2][0] = match seed % 11 {
                0 => 0,
                1 => 0x10_0000 + (r.next_u32() % 0x1000) as u64,
                2 => sx(0xffff_ffe0 + r.next_u32() % 0x20),
                _ => (r.next_u32() % 0x2000) as u64,
            };
        });
    }

    // ------------------------------------------------ the AI trick world

    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::GAME_GP;

    const TW_AI: u32 = DATA + 0x1_0000;
    const TW_RIDER: u32 = DATA + 0x2_0000;
    const TW_RIDER_VT: u32 = DATA + 0x3_0000;
    const TW_ANIM: u32 = DATA + 0x3_1000;
    const TW_ANIM_VT: u32 = DATA + 0x3_1100;
    const TW_BE_LIB: u32 = DATA + 0x3_2000;
    const TW_BE_IFACE: u32 = DATA + 0x3_3000;
    const TW_BE_VT: u32 = DATA + 0x3_3100;
    /// What `BELibrary_GetInterface` answers (the BAGT interface).
    const TW_IFACE: u32 = DATA + 0x3_4000;
    /// `AnimController_GetCurrentAnimTriggerTime`'s tables (`gp+0xd8c`,
    /// `gp+0xd08`).
    const TT_BASE: u32 = DATA + 0x3_5000;
    const TT_PTRS: u32 = DATA + 0x3_7000;
    const TT_HDR: u32 = DATA + 0x3_7100;
    const TT_RECS: u32 = DATA + 0x3_7200;
    const TT_TIMES: u32 = DATA + 0x3_7400;
    /// The rider index the rider's virtual answers.
    const TW_IDX: u32 = DATA + 0x3_8000;
    /// Fake calls made, and the call index that unwinds.
    const TW_CALLS: u32 = DATA + 0x3_8004;
    const TW_UNWIND_AT: u32 = DATA + 0x3_8008;
    /// Uber trick animation lists (`trick.+0x10`).
    const TW_UBER_ANIMS: u32 = DATA + 0x3_9000;
    /// The BAGT trick table (0x14 bytes per trick).
    const TRICK_TABLE: u32 = 0x0045_aeb8;
    /// The current track (`BENewRaceInt_GetTrack`), 0xe = no uber tricks.
    const TRACK: u32 = 0x0053_5c08;
    /// `Rand_NextGame`'s state.
    const RAND_STATE: u32 = 0x004f_f030;

    /// The fake callees of the trick world: the rider's index query
    /// (`FAKE+0x10`), the animation controller's anim lookup (`FAKE+0x20`:
    /// ids below 8 map to themselves, others to 0x207, "no trigger time")
    /// and the BE library's interface slot (`FAKE+0x80`). Unwinds at the
    /// call index stored at [`TW_UNWIND_AT`].
    fn trick_world_hook(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(TW_CALLS);
        c.bus.write32(TW_CALLS, n + 1);
        c.g[V0][0] = match target.wrapping_sub(FAKE) {
            0x10 => c.bus.read32(TW_IDX) as u64,
            0x20 => {
                let anim = c.g[A1][0] as u32;
                if anim < 8 {
                    anim as u64
                } else {
                    0x207
                }
            }
            0x80 => TW_IFACE as u64,
            _ => 0,
        };
        c.g[A0][0] = 0x1234;
        c.cycles += 3;
        c.pc = if n == c.bus.read32(TW_UNWIND_AT) {
            0xdead_0000 | n << 4
        } else {
            ra
        };
    }

    /// An AI (`TW_AI`) with its rider, the BE library and BAGT interface,
    /// the trick table (some tricks without an animation, 0x1b6), uber
    /// animation lists, the animation trigger-time tables, the rand state,
    /// the trick counts and the roll / pick inputs, all chosen by `r`.
    fn trick_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let pick = |r: &mut Rng, v: &[u32]| v[(r.next_u32() as usize) % v.len()];
        let f = |x: f32| x.to_bits();
        c.hooks.miss = trick_world_hook;
        c.bus.write32(TW_CALLS, 0);
        let unwind = if seed % 5 == 0 { r.next_u32() % 40 } else { u32::MAX };
        c.bus.write32(TW_UNWIND_AT, unwind);
        c.bus.write32(TW_IDX, r.next_u32() % 4);
        // BE library singleton and its interface slot.
        c.bus.write32(GAME_GP - 0x1f04, TW_BE_LIB);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(0x0053_08a8, TW_BE_IFACE);
        c.bus.write32(TW_BE_IFACE + 4, TW_BE_VT);
        c.bus.write16(TW_BE_VT + 8, 0);
        c.bus.write32(TW_BE_VT + 0xc, FAKE + 0x80);
        c.bus.write32(TRACK, pick(&mut *r, &[0xe, 3]));
        for i in 0..6 {
            c.bus.write32(RAND_STATE + 4 * i, r.next_u32());
        }
        // Trick skill factor: `lit * attribute + 1`.
        c.bus.write32(GAME_GP - 0x7960, f(0.5));
        c.bus.write8(0x0053_08d8 + 0xa, 2);
        // The trick table.
        for i in 0..16u32 {
            let t = TRICK_TABLE + 0x14 * i;
            c.bus.write16(t, pick(&mut *r, &[0, 1, 3, 5, 7, 9, 0x1b6]) as u16);
            c.bus.write16(t + 6, pick(&mut *r, &[0x1b6, 2, 4, 6, 8]) as u16);
            let uber = r.next_u32() % 2;
            c.bus.write16(t + 0xc, uber as u16);
            c.bus.write32(t + 0x10, TW_UBER_ANIMS + 0x10 * i);
            c.bus
                .write16(TW_UBER_ANIMS + 0x10 * i, pick(&mut *r, &[0x1b6, 3, 6]) as u16);
        }
        // Trigger times: anim id i -> record i, 0..3 times of 0..60 frames.
        c.bus.write32(GAME_GP + 0xd8c, TT_BASE);
        c.bus.write32(GAME_GP + 0xd08, TT_PTRS);
        c.bus.write32(GAME_GP - 0x322c, f(1.0 / 30.0));
        c.bus.write32(TT_PTRS, TT_HDR);
        c.bus.write32(TT_HDR + 8, TT_RECS);
        c.bus.write32(TT_HDR + 0x10, TT_TIMES);
        for i in 0..8u32 {
            c.bus.write32(TT_BASE + 0x1030 + 4 * i, i << 8);
            c.bus.write16(TT_RECS + 0x14 * i + 0x10, (3 * i) as u16);
            c.bus.write16(TT_RECS + 0x14 * i + 0x12, (r.next_u32() % 4) as u16);
        }
        for j in 0..32u32 {
            c.bus.write16(TT_TIMES + 4 * j, (r.next_u32() % 60) as u16);
        }
        // The rider.
        let rider = TW_RIDER;
        c.bus.write32(TW_AI + 0x18, rider);
        c.bus.write32(rider + 0x6c0, TW_RIDER_VT);
        c.bus.write16(TW_RIDER_VT + 0x38, 0);
        c.bus.write32(TW_RIDER_VT + 0x3c, FAKE + 0x10);
        c.bus.write32(rider + 0x784, TW_ANIM);
        c.bus.write32(TW_ANIM + 0x58, TW_ANIM_VT);
        c.bus.write16(TW_ANIM_VT + 0x20, 0);
        c.bus.write32(TW_ANIM_VT + 0x24, FAKE + 0x20);
        c.bus.write32(rider + 0x2f0, pick(&mut *r, &[f(-1.0), 0, f(2.0), f(2.0)]));
        c.bus.write32(rider + 0x2f4, r.next_u32() % 8);
        c.bus.write32(rider + 0x86c, r.next_u32() % 4);
        c.bus.write32(rider + 0xb34, 1 + r.next_u32() % 3);
        c.bus.write32(rider + 0xb2c, r.next_u32() & 0xff);
        // The AI: roll skill, trick counts.
        let ai = TW_AI;
        c.bus.write32(ai + 0xdfc, pick(&mut *r, &[f(-1.0), 0, f(0.5), f(1.0)]));
        c.bus.write32(ai + 0xe20, pick(&mut *r, &[0, 0, 1]));
        for i in 0..15u32 {
            for base in [0xe7cu32, 0xeb8, 0xef4] {
                c.bus.write32(ai + base + 4 * i, r.next_u32() % 20);
            }
        }
    }

    /// `ComputerActorRider_ChooseTrick` on the trick world with the real
    /// rolls and pickers: skipped (+0xe20 below 1 unit), too low, exactly
    /// 100, each roll failing or succeeding and its pick found or not, and
    /// unwinds out of any of the callees.
    #[test]
    fn computer_actor_rider_choose_trick_matches() {
        check(
            ee::s00::f_0010cad8__generated,
            computer_actor_rider_choose_trick,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_cad8, 0);
                trick_world(c, &mut r, seed);
                let heights = [0.5f32, 0.75, 0.9, 1.5, 3.0, 10.0, 100.0, 40.0];
                c.f[12] = heights[(r.next_u32() % 8) as usize].to_bits();
                c.g[A0][0] = sx(TW_AI);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0010_cad8;
            },
        );
    }

    /// A stream at a random address whose vtable's slot 1 is a fake (which
    /// unwinds in some seeds), and a random `self`.
    fn save_state_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        use crate::difftest::fakes::{ptr, vtable};
        let mut r = machine(c, seed, salt, 1);
        let stream = ptr(&mut r);
        let vt = vtable(c, &mut r, &[8]);
        c.bus.write32(stream, vt);
        c.g[A0][0] = sx(ptr(&mut r));
        c.g[A1][0] = sx(stream);
        c.g[RA][0] = SENTINEL as u64;
    }

    #[test]
    fn rider_wipeout_state_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s01::f_00138ad8__generated,
            rider_wipeout_state_save_state,
            300,
            &|c, seed| {
                save_state_fixture(c, seed, 0x0013_8ad8);
                c.pc = 0x0013_8ad8;
            },
        );
    }

    #[test]
    fn rider_wipeout_sub_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s00::f_0012e708__generated,
            rider_wipeout_sub_save_state,
            300,
            &|c, seed| {
                save_state_fixture(c, seed, 0x0012_e708);
                c.pc = 0x0012_e708;
            },
        );
    }

    /// `ComputerActorRider_PickFittedTrick` on the trick world: tricks
    /// without an animation, airborne with and without uber animations,
    /// trigger times that fit or not, fewer or more earlier picks, no pick
    /// at all, and unwinds out of any callee.
    #[test]
    fn computer_actor_rider_pick_fitted_trick_matches() {
        check(
            ee::s00::f_0010cd20__generated,
            computer_actor_rider_pick_fitted_trick,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_cd20, 0);
                trick_world(c, &mut r, seed);
                let heights = [0.0f32, 0.5, 1.0, 1.5, 2.0, 3.0, 5.0, -1.0];
                c.f[12] = heights[(r.next_u32() % 8) as usize].to_bits();
                c.g[A0][0] = sx(TW_AI);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0010_cd20;
            },
        );
    }

    /// Event delivery at the trick loop head (0x10cd98).
    #[test]
    fn computer_actor_rider_pick_fitted_trick_events_match() {
        check_events(
            ee::s00::f_0010cd20__generated,
            computer_actor_rider_pick_fitted_trick,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0010_cd21, 0);
                trick_world(c, &mut r, 1 + seed % 4);
                c.bus.write32(TW_UNWIND_AT, u32::MAX);
                c.g[A0][0] = sx(TW_AI);
                c.g[RA][0] = SENTINEL as u64;
                let mode = schedule(c, &mut r, seed, 0x0010_cd20);
                c.f[12] = 2.0f32.to_bits();
                mode
            },
        );
    }

    /// `BEAggressionInt_AddAggression`: riders 0..2 (rider 0 is the player:
    /// the aggression message, suppressed or not, with the message centre's
    /// short path), profiles 0..2 (one by the "-1" flag), events -1..4,
    /// records in every state with levels and points around each clamp.
    #[test]
    fn be_aggression_int_add_aggression_matches() {
        const SLOTS: u32 = 0x0053_05b0;
        const CHARS: u32 = 0x0053_5b20;
        const WORLD: u32 = DATA + 0x5_0000;
        check(
            ee::s01::f_00155bf0__generated,
            be_aggression_int_add_aggression,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0015_5bf0, 0);
                let pick = |r: &mut Rng, v: &[i32]| v[(r.next_u32() as usize) % v.len()];
                // Rider slots and character records.
                for idx in 0..4u32 {
                    c.bus.write32(SLOTS + 4 * idx, idx);
                    let rec = CHARS + 0x1c * idx;
                    let flag = pick(&mut r, &[-1, 0, 1]) as u32;
                    c.bus.write32(rec + 0xc, flag);
                    c.bus.write32(rec + 0x10, r.next_u32() % 4);
                    c.bus.write8(rec + 0x11, (r.next_u32() % 5) as u8);
                }
                let rider = if seed % 3 == 0 { 0 } else { r.next_u32() % 3 };
                let rival = 1 + r.next_u32() % 3;
                // Fill the records this pair can reach (every profile).
                for profile in 0..3u32 {
                    for ch in 0..5u32 {
                        for rv in 0..5u32 {
                            let e = AGGRESSION_RECORDS + profile * 0x9b50 + ch * 0xf88 + rv * 3;
                            c.bus.write8(e + 0xbc1, pick(&mut r, &[-1, 0, 1, 2, 3, 4]) as u8);
                            c.bus.write8(e + 0xbc2, pick(&mut r, &[-1, 0, 1, 2, 3, 4, 5]) as u8);
                            c.bus.write8(e + 0xbc3, pick(&mut r, &[-3, 0, 4, 9, 14, 15, 19, 24, 30]) as u8);
                        }
                    }
                }
                c.bus.write8(AGGRESSION_MESSAGES_OFF, (r.next_u32() % 2) as u8);
                // MessageCenter_CheckAggressionMessage's short path.
                c.bus.write32(GAME_GP - 0x848, WORLD);
                c.bus.write32(WORLD + 0x84, WORLD + 0x100);
                c.bus.write32(WORLD + 0x100 + 0x28, WORLD + 0x200);
                c.bus.write32(WORLD + 0x200, 1 + r.next_u32() % 9);
                c.g[A0][0] = sx(DATA + 0x6_0000);
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = rival as u64;
                c.g[A3][0] = sx(pick(&mut r, &[-1, 0, 1, 2, 3, 4]) as u32);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0015_5bf0;
            },
        );
    }

    /// Call a generated function for a fixture (up to six arguments).
    fn oc287_setup_call(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        const BACK: u32 = 0x0000_7770;
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = BACK as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, BACK, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// `TexMan_FreeTexture`: a texture manager with both VRAM heaps built
    /// by the game's own `VramHeap_Init` / `VramHeap_Alloc`; textures of
    /// kind 9 or not, with VRAM blocks A / B or none (-1), owned or not,
    /// CPU buffers null or small pool pointers (the test machine's pools
    /// start at 0, so `EA_MemFree` takes the pool path), B ids (< 1500) and
    /// NB ids. The texture record itself is a small pool pointer too (its
    /// info is deleted).
    #[test]
    fn tex_man_free_texture_matches() {
        const MAN: u32 = DATA + 0x1_0000;
        const NODES: u32 = DATA + 0x2_0000;
        const OWNERS: u32 = DATA + 0x3_0000;
        check(ee::s12::f_00367bc0__generated, tex_man_free_texture, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0036_7bc0, 0);
            let pick = |r: &mut Rng, v: &[u32]| v[(r.next_u32() as usize) % v.len()];
            for i in 0..16 {
                c.bus.write32(OWNERS + 4 * i, OWNERS + 0x100 + 0x40 * i);
            }
            // The two VRAM heaps, two blocks each.
            let mut blocks = Vec::new();
            for (h, heap) in [(0u32, MAN + TEX_MAN_VRAM), (1, MAN + TEX_MAN_VRAM_KIND9)] {
                c.bus.write32(heap + 0x1ff0, NODES + 0x8000 * h);
                oc287_setup_call(
                    c,
                    0x0036_5e40,
                    ee::s12::VramHeap_Init,
                    &[heap as u64, 0, 0x20_0000, OWNERS as u64],
                );
                let mut ids = Vec::new();
                for _ in 0..2 {
                    let size = 0x100 * (1 + r.next_u32() % 16);
                    let owner = r.next_u32() % 7;
                    ids.push(oc287_setup_call(
                        c,
                        0x0036_6ce0,
                        ee::s12::VramHeap_Alloc,
                        &[heap as u64, size as u64, owner as u64, 7, 50, 0],
                    ) as u32);
                }
                blocks.push(ids);
            }
            c.g[GP][0] = GAME_GP as u64;
            // The texture (a small pool pointer, deleted by the info free).
            let tex = 0x100 + 0x40 * (r.next_u32() % 8);
            let kind9 = r.next_u32() % 3 == 0;
            c.bus.write32(tex + TEX_KIND, if kind9 { 9 } else { pick(&mut r, &[0, 1, 8, 10]) });
            let heap_a = if kind9 { 1 } else { 0 };
            let has_a = r.next_u32() % 3 != 0;
            c.bus.write32(tex + TEX_SLOT_A, if has_a { r.next_u32() % 4 } else { u32::MAX });
            c.bus.write32(tex + TEX_ID_A, blocks[heap_a][0]);
            let has_b = r.next_u32() % 3 != 0;
            c.bus.write32(tex + TEX_SLOT_B, if has_b { r.next_u32() % 4 } else { u32::MAX });
            c.bus.write32(tex + TEX_ID_B, blocks[0][1]);
            c.bus.write32(tex + TEX_OWNED, pick(&mut r, &[0, 1, 0x100]));
            let buf = |r: &mut Rng| if r.next_u32() % 3 == 0 { 0 } else { 0x500 + 8 * (r.next_u32() % 0x40) };
            let (a, b) = (buf(&mut r), buf(&mut r));
            c.bus.write32(tex + TEX_BUF_A, a);
            c.bus.write32(tex + TEX_BUF_B, b);
            // The manager: the id's texture, counts, NB free-list head.
            let id = pick(&mut r, &[3, 100, 1499, 1500, 1501, 1700]);
            c.bus.write32(MAN + TEX_MAN_TEXTURES + 4 * id, tex);
            c.bus.write32(MAN, 1 + r.next_u32() % 50);
            c.bus.write32(MAN + 4, 1 + r.next_u32() % 50);
            c.bus.write32(MAN + 0x1f48, MAN + 8 + 4 * (r.next_u32() % 1400));
            for off in [0x1f4cu32, 0x1f50, 0x1f54] {
                c.bus.write32(MAN + off, r.next_u32());
            }
            c.g[A0][0] = sx(MAN);
            c.g[A1][0] = id as u64;
            c.g[RA][0] = SENTINEL as u64;
            c.pc = 0x0036_7bc0;
        });
    }

    /// Event delivery at the reset loop's head (0x367c90): a texture with
    /// nothing to free in VRAM or CPU memory.
    #[test]
    fn tex_man_free_texture_events_match() {
        const MAN: u32 = DATA + 0x1_0000;
        check_events(ee::s12::f_00367bc0__generated, tex_man_free_texture, 90, &|c, seed| {
            let mut r = machine(c, seed, 0x0036_7bc1, 0);
            let tex = 0x100 + 0x40 * (r.next_u32() % 8);
            c.bus.write32(tex + TEX_SLOT_A, u32::MAX);
            c.bus.write32(tex + TEX_SLOT_B, u32::MAX);
            c.bus.write32(tex + TEX_OWNED, 0);
            let id = if seed % 2 == 0 { 7 } else { 1600 };
            c.bus.write32(MAN + TEX_MAN_TEXTURES + 4 * id, tex);
            c.bus.write32(MAN + 0x1f48, MAN + 0x100);
            c.g[A0][0] = sx(MAN);
            c.g[A1][0] = id as u64;
            c.g[RA][0] = SENTINEL as u64;
            let mode = schedule(c, &mut r, seed, 0x0036_7bc0);
            mode
        });
    }
}
