//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for world objects, scripters and render-object helpers, each
//! with a differential test against the generated body.
//!
//! - **World objects**: `WorldObj_Update` (tail-dispatch to the virtual
//!   update at `*(obj + 0xc) + 0x7c`), `World_IsOccluded` (VU0 clip test over
//!   the occlusion manager's instance list), `DeferMan_SetScissor` (the dedup
//!   table of deferred scissor rectangles).
//! - **Scripts**: `WScriptMan_IsScriptedInstance` and
//!   `World_GetObjectScriptFunc` (object -> script slot through the WScript
//!   cache; 12-byte `sEntry` records, `type` in the first halfword).
//! - **Races**: `BENewRaceInt_GetPeakRivalCharacter` (the rival character of
//!   the human rider on the current peak).
//! - **Move nodes**: `ModObj_v44` (transform bounds present?),
//!   `ModObj_v24` (first child's virtual call, or the node's own matrix).
//! - **Splines**: `SplineModifier_EvalSegment` (cubic Bezier position,
//!   velocity and acceleration through the key's basis) and
//!   `SplineModifier_EvalPathAtDistance` (walk the key chain to distance `t`;
//!   dispatcher-shaped, re-entered at its two walk loop heads).

use crate::abi;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0036_08e8, world_obj_update, "WorldObj_Update"),
    (0x0030_a310, wscript_man_is_scripted_instance, "WScriptMan_IsScriptedInstance"),
    (0x003a_6b78, world_get_object_script_func, "World_GetObjectScriptFunc"),
    (0x002d_2058, world_is_occluded, "World_IsOccluded"),
    (0x0014_5750, benew_race_int_get_peak_rival_character, "BENewRaceInt_GetPeakRivalCharacter"),
    (0x0035_69d0, mod_obj_v44, "ModObj_v44"),
    (0x0035_6078, mod_obj_v24, "ModObj_v24"),
    (0x0034_5048, spline_modifier_eval_segment, "SplineModifier_EvalSegment"),
    (0x0034_5248, spline_modifier_eval_path_at_distance, "SplineModifier_EvalPathAtDistance"),
    (0x0036_4b88, defer_man_set_scissor, "DeferMan_SetScissor"),
];

// ---------------------------------------------------------------------------
// Callees (the names the generated bodies call them by).

mod callee {
    use super::{ee, Callee};

    pub const HAS_OBJECT_SCRIPT1: Callee = Callee { addr: 0x0030_a270, f: ee::s10::f_0030a270 };
    pub const HAS_OBJECT_SCRIPT3: Callee = Callee { addr: 0x0030_a2e8, f: ee::s10::f_0030a2e8 };
    pub const GET_OBJECT_SLOT: Callee = Callee { addr: 0x003a_d120, f: ee::s13::f_003ad120 };
    pub const GET_PLAYER_INSTANCE: Callee = Callee { addr: 0x0014_5970, f: ee::s01::f_00145970 };
    pub const GET_HUMAN_RIDER: Callee = Callee { addr: 0x0014_6e98, f: ee::s01::f_00146e98 };
    pub const GET_CHARACTER: Callee = Callee { addr: 0x0014_7398, f: ee::s01::f_00147398 };
    pub const GET_TRACK_PEAK: Callee = Callee { addr: 0x0014_4c78, f: ee::s01::f_00144c78 };
    pub const GET_TRANSFORM_BOUNDS: Callee = Callee { addr: 0x0035_2b88, f: ee::s11::f_00352b88 };
    pub const EVAL_SEGMENT: Callee = Callee { addr: 0x0034_5048, f: ee::s11::f_00345048 };
}

/// A `gp`-relative global address.
#[inline(always)]
fn gp_at(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// World objects.

/// `void WorldObj_Update(WorldObj* obj)` (0x3608e8): call the object's
/// virtual update at `*(*(obj + 0xc) + 0x7c)` with `this + (short)
/// *((obj + 0xc) + 0x78)` as the argument (the virtual body sits at that
/// offset inside the embedded rail object, so the adjustment is added to the
/// `this` pointer). No value returned.
pub fn world_obj_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(abi::arg(c, 0)).off(0xc).lw(c);
        let virt = c.g[V1][0] as u32;
        c.g[A1][0] = Addr(virt).off(0x78).lh(c);
        let target = Addr(virt).off(0x7c).lw(c);
        c.g[V0][0] = target;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target as u32, 0x0036_0904)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DeferMan_SetScissor(DeferMan* self /*a0*/, const ScissorKey* k
/// /*a1*/)` (0x364b88): find `k`'s four halfwords among `self`'s `count`
/// 8-byte keys (base `+0x6aed4`, count `+0x6aed0`); store the index in
/// `found` (`+0x69cc4`), and if absent append `k` at `count` and set
/// `found = count`, `count++`.
pub fn defer_man_set_scissor(c: &mut Cpu) {
    let this = abi::arg(c, 0);
    let key = abi::arg(c, 1);
    c.cycles += 8;
    c.g[V0][0] = 0x60000;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] |= 0x8000;
    c.g[A3][0] = 0;
    c.g[A2][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[T0][0] = Addr(c.g[A2][0] as u32).off(0x2ed0).lw(c);
    let none = (c.g[T0][0] as i64) <= 0;
    Addr(c.g[A2][0] as u32).off(0x1cc4).sw(c, c.g[V1][0]);
    if !none {
        c.cycles += 4;
        c.g[V0][0] = 0x60000;
        c.g[T1][0] = c.g[A2][0];
        c.g[V0][0] |= 0xaeda;
        c.g[A2][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
        let mut found = false;
        loop {
            c.cycles += 4;
            let entry = c.g[A2][0] as u32;
            c.g[V1][0] = Addr(entry.wrapping_sub(0x6)).lhu(c);
            c.g[V0][0] = Addr(key).lhu(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(entry.wrapping_sub(0x4)).lhu(c);
                c.g[V0][0] = Addr(key).off(2).lhu(c);
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(entry.wrapping_sub(0x2)).lhu(c);
                    c.g[V0][0] = Addr(key).off(4).lhu(c);
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = Addr(entry).lhu(c);
                        c.g[V0][0] = Addr(key).off(6).lhu(c);
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                        } else {
                            c.cycles += 2;
                            Addr(c.g[T1][0] as u32).off(0x1cc4).sw(c, c.g[A3][0]);
                            found = true;
                        }
                    }
                }
            }
            if found {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_4bb8);
            }
        }
    }
    // Not found: append the key at `count` and record its index.
    c.cycles += 6;
    c.g[V0][0] = 0x60000;
    c.g[V0][0] |= 0x8000;
    c.g[A2][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x1cc4).lw(c);
    if (c.g[V1][0] as i64) < 0 {
        c.cycles += 24;
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x2ed0).lw(c);
        c.g[A0][0] = Addr(key).lhu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[V0][0] as u32).off(0x2ed4).sh(c, c.g[A0][0]);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x2ed0).lw(c);
        c.g[A0][0] = Addr(key).off(2).lhu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V1][0] as u32).off(0x2ed6).sh(c, c.g[A0][0]);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x2ed0).lw(c);
        c.g[A0][0] = Addr(key).off(4).lhu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[V0][0] as u32).off(0x2ed8).sh(c, c.g[A0][0]);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x2ed0).lw(c);
        c.g[A0][0] = Addr(key).off(6).lhu(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V1][0] as u32).off(0x2eda).sh(c, c.g[A0][0]);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x2ed0).lw(c);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        Addr(c.g[A2][0] as u32).off(0x1cc4).sw(c, c.g[V0][0]);
        Addr(c.g[A2][0] as u32).off(0x2ed0).sw(c, c.g[V1][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int World_IsOccluded(sQ128* p)` (0x2d2058): over the occlusion manager
/// `*(world + 0x84) -> +0x8c`, for each instance whose bit is set in the
/// candidate mask (`+0x110`, `count` at `+0x10c`), run the VU0 clip-test
/// microprogram on the instance's six quadwords at `+0x50..+0x90` and the
/// point `p`; the microprogram reports "occluded" in `vi1 == 1`. 1 when any
/// instance occludes the point, else 0.
pub fn world_is_occluded(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = Addr(gp_at(c, -0x848)).lw(c);
    c.g[A1][0] = 0;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
    c.g[A3][0] = Addr(c.g[V1][0] as u32).off(0x8c).lw(c);
    let inst = c.g[A3][0] as u32;
    c.g[V0][0] = Addr(inst).off(0x10c).lw(c);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[T1][0] = Addr(inst).off(0x110).lw(c);
    if !empty {
        c.cycles += 5;
        c.g[V0][0] = 0;
        c.g[T2][0] = 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x828));
        c.g[A2][0] = sx(inst.wrapping_add(0x8));
        // The microprogram address, as VI27 expects it (byte address >> 3).
        c.g[T0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[T2][0] as u32) << ((c.g[A1][0] as u32) & 31));
            c.g[V0][0] = c.g[T1][0] & c.g[V0][0];
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr(inst).off(0x10c).lw(c);
            } else {
                c.cycles += 15;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                let m = c.g[V0][0] as u32;
                c.lqc2(1, m.wrapping_add(0x50));
                c.lqc2(2, m.wrapping_add(0x60));
                c.lqc2(3, m.wrapping_add(0x70));
                c.lqc2(4, m.wrapping_add(0x80));
                c.lqc2(5, m.wrapping_add(0x90));
                c.lqc2(6, c.g[A0][0] as u32);
                c.ctc2(8, 27);
                c.vu0().vnop();
                c.vu0().vnop();
                c.vcallmsr();
                c.cfc2(3, 1);
                c.g[V1][0] ^= 0x1;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = Addr(inst).off(0x10c).lw(c);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_2088);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// WScript.

/// `int WScriptMan_IsScriptedInstance(WScriptMan* self, WorldObj* obj)`
/// (0x30a310): 1 if the object is a scripted instance — it has a live
/// virtual at `+0xc` (a rail object with an update), or its 12-byte script
/// table entry (`*(*(self + 0x28c) + 4) + 4`, indexed by the object's
/// `+0x78` byte) has type 3 and the object has a script 1 or script 3 slot.
pub fn wscript_man_is_scripted_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = abi::arg64(c, 1);
        frame.save_ra(c, 0);
        let obj = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(obj).off(0xc).lw(c);
        let has_virtual = c.g[V0][0] != 0;
        c.g[S1][0] = abi::arg64(c, 0);
        if has_virtual {
            // Falls through to the common "return 1" tail (+1 cycle).
            c.cycles += 1;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 11;
            c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x28c).lw(c);
            c.g[A1][0] = 0xc;
            c.g[V1][0] = Addr(obj).off(0x78).lbu(c);
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(4).lw(c);
            // v1 = id * 12 (the table's record stride), in the delay slot.
            c.mult(0, 3, 3, 5);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lh(c);
            c.g[V0][0] ^= 0x3;
            let not_type3 = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if not_type3 {
                // Not a scripted type: return 0 (no extra cycle).
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                callee::HAS_OBJECT_SCRIPT1.call(c, 0x0030_a368)?;
                let has_script1 = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if !has_script1 {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    callee::HAS_OBJECT_SCRIPT3.call(c, 0x0030_a37c)?;
                    let no_script3 = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if no_script3 {
                        // Return 0.
                    } else {
                        // Common return-1 tail: one extra cycle.
                        c.cycles += 1;
                        c.g[V0][0] = 1;
                    }
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int World_GetObjectScriptFunc(WScriptCache* cache, WorldObj* obj,
/// int flag)` (0x3a6b78): the object's script slot — the script table is at
/// `*(*(cache + 4) + 4)`, records are 12 bytes (`(unsigned char)
/// *(obj + 0x78)` indexes them), and the returned value is
/// `tWScriptCache_GetObjectSlot(table + id * 12, obj, flag)`.
pub fn world_get_object_script_func(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = 0xc;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(abi::arg(c, 1)).off(0x78).lbu(c);
        c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
        // v0 = id * 12 (record stride), in the delay slot.
        c.mult(0, 2, 2, 7);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        callee::GET_OBJECT_SLOT.call(c, 0x003a_6b9c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Races.

/// `int BENewRaceInt_GetPeakRivalCharacter(BENewRaceInt* self)` (0x145750):
/// the rival character for the human rider on the race's current peak: the
/// player character id `c` (`BENewPlayerInt_GetCharacter`) is compared with
/// the character at the peak table entry
/// `*(0x535c08)` (`BENewRaceInt_GetTrackPeakByIndex`):
/// peak 1 -> `c == 7 ? 4 : 7`; peak 0 -> `c == 3 ? 5 : 3`;
/// peak 2 -> `c == 8 ? 6 : 8`; else 3.
pub fn benew_race_int_get_peak_rival_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = abi::arg64(c, 0);
        callee::GET_PLAYER_INSTANCE.call(c, 0x0014_5768)?;
        c.g[S0][0] = c.g[V0][0];
        callee::GET_PLAYER_INSTANCE.call(c, 0x0014_5770)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::GET_HUMAN_RIDER.call(c, 0x0014_577c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        callee::GET_CHARACTER.call(c, 0x0014_5788)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = 0x530000;
        c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x5c08).lw(c);
        callee::GET_TRACK_PEAK.call(c, 0x0014_579c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        let first_peak = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S0][0] ^ 0x7;
        if first_peak {
            c.cycles += 4;
            c.g[V1][0] = 7;
            c.g[V0][0] = 4;
            if c.g[A0][0] != 0 {
                c.g[V0][0] = c.g[V1][0];
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            let mut done = false;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
            } else {
                c.cycles += 2;
                let peak_zero = c.g[V1][0] == 0;
                c.g[A0][0] = c.g[S0][0] ^ 0x3;
                if !peak_zero {
                    c.cycles += 2;
                    c.g[V0][0] = 3;
                    done = true;
                }
            }
            if !done {
                if c.g[V1][0] == 0 {
                    // Peak 0: the third character of the group.
                    c.cycles += 4;
                    c.g[V1][0] = 3;
                    c.g[V0][0] = 5;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                } else {
                    c.cycles += 2;
                    let second_peak = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0] ^ 0x8;
                    if second_peak {
                        c.cycles += 3;
                        c.g[V1][0] = 8;
                        c.g[V0][0] = 6;
                        if c.g[A0][0] != 0 {
                            c.g[V0][0] = c.g[V1][0];
                        }
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 3;
                    }
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Move nodes.

/// `int ModObj_v44(ModObj* self)` (0x3569d0): whether the modifier block at
/// `self + 0x1c` has transform bounds (`ModifierBlock_GetTransformBounds`
/// non-zero); 0 without a block.
pub fn mod_obj_v44(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x1c).lw(c);
        let no_block = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !no_block {
            c.cycles += 3;
            callee::GET_TRANSFORM_BOUNDS.call(c, 0x0035_69ec)?;
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* ModObj_v24(ModObj* self)` (0x356078): the first child at
/// `*(*(self + 0x1c))`; when it exists, dispatch `child + (short)
/// *(vtable + 0x90)` through `*(vtable + 0x94)` (the child's update), else
/// return `*(self + 0x18) + 0x10` (the node's own matrix).
pub fn mod_obj_v24(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[V1][0] = Addr(this).off(0x1c).lw(c);
        let no_head = c.g[V1][0] == 0;
        c.g[A1][0] = 0;
        if !no_head {
            c.cycles += 1;
            c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        }
        c.cycles += 2;
        if c.g[A1][0] != 0 {
            let child = c.g[A1][0] as u32;
            c.g[V1][0] = Addr(child).lw(c);
            c.cycles += 4;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x90).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x94).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0035_60b4)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x18).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Splines.

/// One `basis * quad` apply of the Bezier evaluation: `lqc2 vf8, from;
/// lqc2 vf4..7, basis; vmulax/vmadday/vmaddaz/vmaddw; sqc2 vf12, to`.
#[inline(always)]
fn basis_apply(c: &mut Cpu, basis: u32, from: u32, to: u32) {
    c.lqc2(8, from);
    c.lqc2(4, basis);
    c.lqc2(5, basis.wrapping_add(0x10));
    c.lqc2(6, basis.wrapping_add(0x20));
    c.lqc2(7, basis.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, to);
}

/// `void SplineModifier_EvalSegment(sDebCurve* self, float t, sDebVec4* pos,
/// sDebVec4* vel, sDebVec4* acc)` (0x345048): evaluate the cubic
/// `u = ((c0*t + c1)*t + c2)*t + c3` (and its first/second derivatives) at
/// `t - self->t0` and map the Bezier position/velocity/acceleration vectors
/// through the key's 4x4 basis at `self + 0x10`.
pub fn spline_modifier_eval_segment(c: &mut Cpu) {
    c.cycles += 93;
    let curve = abi::arg(c, 0);
    let (pos, vel, acc) = (abi::arg(c, 1), abi::arg(c, 2), abi::arg(c, 3));
    let frame_sp = sp(c).wrapping_sub(0x50);
    c.lwc1(0, curve.wrapping_add(0x84)); // key start time
    c.g[SP][0] = sx(frame_sp);
    c.lwc1(1, curve.wrapping_add(0x50)); // c0
    c.g[V1][0] = sx(curve.wrapping_add(0x10)); // basis
    let basis = c.g[V1][0] as u32;
    c.sub_s(12, 12, 0); // t -= t0
    c.lwc1(2, curve.wrapping_add(0x54)); // c1
    c.lwc1(7, curve.wrapping_add(0x58)); // c2
    c.g[AT][0] = 0x4040_0000; // 3.0
    c.mtc1(1, 6);
    c.add_s(8, 2, 2); // 2*c1
    c.mul_s(0, 1, 12); // c0*t
    c.lwc1(9, curve.wrapping_add(0x5c)); // c3
    c.mul_s(5, 12, 6); // 3t
    c.g[AT][0] = 0x40c0_0000; // 6.0
    c.mtc1(1, 4);
    c.g[AT][0] = 0x4000_0000; // 2.0
    c.mtc1(1, 3);
    c.add_s(0, 0, 2);
    c.sw(frame_sp.wrapping_add(0x8), 0);
    c.mul_s(1, 1, 5); // c0*3t
    c.swc1(3, frame_sp.wrapping_add(0x4)); // 2.0
    c.sw(frame_sp.wrapping_add(0xc), 0);
    c.mul_s(0, 0, 12);
    c.g[AT][0] = 0x3f80_0000; // 1.0
    c.mtc1(1, 10);
    c.add_s(1, 1, 8); // c0*3t + 2c1
    c.add_s(0, 0, 7); // (c0 t + c1) t + c2
    c.mul_s(1, 1, 12); // *t
    c.mul_s(0, 0, 12); // *t
    c.add_s(1, 1, 7); // du = ((..)*t + c2)
    c.add_s(0, 0, 9); // u = (..)*t + c3
    c.mul_s(4, 0, 4); // u*6
    c.mul_s(2, 0, 0); // u*u
    c.add_s(5, 0, 0); // 2u
    c.swc1(4, frame_sp); // acceleration input x = u*6
    // acc = basis * (u*6, 2, 0, 0) * du
    basis_apply(c, basis, frame_sp, frame_sp.wrapping_add(0x30));
    c.mul_s(3, 2, 0); // u^3
    c.lq(2, frame_sp.wrapping_add(0x30));
    c.mul_s(6, 2, 6); // 3u^2
    c.sq(2, frame_sp.wrapping_add(0x20));
    c.mfc1(4, 1); // du bits into $a0 (low word of g4)
    c.lqc2(4, frame_sp.wrapping_add(0x20));
    c.qmtc2(4, 3); // vf3 = (du, *, *, *)
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, frame_sp.wrapping_add(0x30));
    c.lq(2, frame_sp.wrapping_add(0x30));
    c.sq(2, frame_sp.wrapping_add(0x10));
    c.sq(2, acc);
    // pos = basis * (u^3, u^2, u, 1)
    c.swc1(3, frame_sp.wrapping_add(0x10));
    c.swc1(2, frame_sp.wrapping_add(0x14));
    c.swc1(0, frame_sp.wrapping_add(0x18));
    c.swc1(10, frame_sp.wrapping_add(0x1c));
    basis_apply(c, basis, frame_sp.wrapping_add(0x10), frame_sp.wrapping_add(0x30));
    c.lq(2, frame_sp.wrapping_add(0x30));
    c.sq(2, frame_sp.wrapping_add(0x20));
    c.sq(2, pos);
    // vel = basis * (3u^2, 2u, 1, 0)
    c.swc1(6, frame_sp.wrapping_add(0x20));
    c.swc1(5, frame_sp.wrapping_add(0x24));
    c.swc1(10, frame_sp.wrapping_add(0x28));
    c.sw(frame_sp.wrapping_add(0x2c), 0);
    basis_apply(c, basis, frame_sp.wrapping_add(0x20), frame_sp.wrapping_add(0x40));
    c.lq(2, frame_sp.wrapping_add(0x40));
    c.sq(2, vel);
    c.sq(2, frame_sp.wrapping_add(0x30));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(frame_sp.wrapping_add(0x50));
    c.pc = ra;
}

// Stages of `SplineModifier_EvalPathAtDistance` (the generated dispatcher's
// block labels; the two loop heads are re-entry points after an unwind).
const EP_ENTRY: u32 = 0x0034_5248;
const EP_TRUNC_NEG: u32 = 0x0034_5274;
const EP_SUB: u32 = 0x0034_5284;
const EP_BACK_ENTRY: u32 = 0x0034_5294;
const EP_BACK: u32 = 0x0034_5298;
const EP_LT: u32 = 0x0034_52b0;
const EP_BACK_TEST: u32 = 0x0034_52c0;
const EP_NEXT: u32 = 0x0034_52cc;
const EP_FWD_ENTRY: u32 = 0x0034_52d4;
const EP_FWD: u32 = 0x0034_52d8;
const EP_NEXT_TEST: u32 = 0x0034_52f0;
const EP_LE: u32 = 0x0034_52f8;
const EP_EVAL: u32 = 0x0034_530c;

/// `void SplineModifier_EvalPathAtDistance(SplineModifier* self, float t,
/// sDebVec4* pos, sDebVec4* vel, sDebVec4* acc)` (0x345248): reduce `t`
/// modulo the key's length, walk the key chain backwards to the segment that
/// starts at or before `t` (while `t < key->t0`), then forwards over segments
/// that end before `t`, and evaluate that segment. `self + 4` is the key
/// index, `self + 8` the current key, `self + 0xc` its length. Dispatcher-
/// shaped in the original: it is re-entered at the two walk loop heads
/// (0x345298, 0x3452d8) after an unwind.
pub fn spline_modifier_eval_path_at_distance(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                EP_ENTRY => {
                    c.cycles += 11;
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
                    c.g[T0][0] = abi::arg64(c, 0);
                    c.sd(sp(c), c.g[RA][0]);
                    c.lwc1(2, Addr(c.g[T0][0] as u32).off(0xc).0);
                    c.div_s(0, 12, 2);
                    c.cvt_w_s(1, 0);
                    c.cvt_s_w(1, 1);
                    c.c_lt_s(0, 1);
                    if c.fcc() {
                        pc = EP_TRUNC_NEG;
                    } else {
                        c.mul_s(1, 1, 2);
                        pc = EP_SUB;
                    }
                }
                EP_TRUNC_NEG => {
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.sub_s(1, 1, 0);
                    c.mul_s(1, 1, 2);
                    pc = EP_SUB;
                }
                EP_SUB => {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(c.g[T0][0] as u32).off(0x8).lw(c);
                    c.lwc1(0, Addr(c.g[V0][0] as u32).off(0x84).0);
                    c.sub_s(12, 12, 1); // t -= f * length
                    pc = EP_LT;
                }
                EP_BACK_ENTRY => {
                    c.cycles += 1;
                    pc = EP_BACK;
                }
                EP_BACK => {
                    c.cycles += 6;
                    let self_ = c.g[T0][0] as u32;
                    c.g[V1][0] = Addr(self_).off(4).lw(c);
                    c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x60).lw(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    Addr(self_).off(4).sw(c, c.g[V1][0]);
                    Addr(self_).off(8).sw(c, c.g[V0][0]);
                    c.lwc1(0, Addr(c.g[V0][0] as u32).off(0x84).0);
                    pc = EP_LT;
                }
                EP_LT => {
                    c.cycles += 4;
                    c.c_lt_s(12, 0);
                    if c.fcc() {
                        pc = EP_BACK_TEST;
                    } else {
                        c.g[V0][0] = Addr(c.g[T0][0] as u32).off(8).lw(c);
                        pc = EP_NEXT;
                    }
                }
                EP_BACK_TEST => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x60).lw(c);
                    let has_prev = c.g[V0][0] != 0;
                    c.g[V0][0] = Addr(c.g[T0][0] as u32).off(8).lw(c);
                    if has_prev {
                        if crate::events::back_edge_unwinds(c, EP_BACK) {
                            return Ok(());
                        }
                        pc = EP_BACK;
                    } else {
                        pc = EP_NEXT;
                    }
                }
                EP_NEXT => {
                    c.cycles += 2;
                    let _always = true;
                    c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x64).lw(c);
                    pc = if _always { EP_NEXT_TEST } else { EP_FWD_ENTRY };
                }
                EP_FWD_ENTRY => {
                    c.cycles += 1;
                    pc = EP_FWD;
                }
                EP_FWD => {
                    c.cycles += 6;
                    let self_ = c.g[T0][0] as u32;
                    c.g[V1][0] = Addr(self_).off(4).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x64).lw(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    Addr(self_).off(4).sw(c, c.g[V1][0]);
                    Addr(self_).off(8).sw(c, c.g[A0][0]);
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x64).lw(c);
                    pc = EP_NEXT_TEST;
                }
                EP_NEXT_TEST => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        pc = EP_EVAL;
                    } else {
                        pc = EP_LE;
                    }
                }
                EP_LE => {
                    c.cycles += 5;
                    c.lwc1(0, Addr(c.g[V0][0] as u32).off(0x84).0);
                    c.c_le_s(0, 12);
                    if c.fcc() {
                        c.g[V0][0] = Addr(c.g[T0][0] as u32).off(8).lw(c);
                        if crate::events::back_edge_unwinds(c, EP_FWD) {
                            return Ok(());
                        }
                        pc = EP_FWD;
                    } else {
                        pc = EP_EVAL;
                    }
                }
                EP_EVAL => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0034_5314);
                    c.g[A0][0] = Addr(c.g[T0][0] as u32).off(8).lw(c);
                    callee::EVAL_SEGMENT.call(c, 0x0034_5314)?;
                    c.g[RA][0] = c.ld(sp(c));
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                other => {
                    c.jump_out(other, EP_ENTRY, 0x0034_5320);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, put_vec, scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 250;

    /// `wscript` fixture addresses.
    const WORLD: u32 = DATA + 0x200;
    const TABLE: u32 = DATA + 0x300;
    const SLOT1: u32 = DATA + 0x400;
    const SLOT2: u32 = DATA + 0x440;
    const BASE: u32 = DATA + 0x600;

    /// A float bit pattern in `[lo, hi)`.
    fn uf(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        (lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)).to_bits()
    }

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// Scrambled registers, the game's gp, and optional fake-call hooks.
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// `*(man + 0x28c) -> cache -> +4 -> world -> +4 -> table`, with the
    /// object's 12-byte table record's type halfword.
    fn put_script_cache(c: &mut Cpu, man: u32, cache: u32, obj: u32, id: u8, kind: u16) {
        c.bus.write32(man + 0x28c, cache);
        c.bus.write32(cache + 4, WORLD);
        c.bus.write32(WORLD + 4, TABLE);
        c.bus.write8(obj + 0x78, id);
        c.bus.write16(TABLE + 12 * id as u32, kind);
    }

    /// `tWScriptCache_GetObjectSlot` fixture: `obj + 0x88 -> +8` indexes
    /// `BASE + 4 * flag`, through `slot + 8 -> +0x1c -> BASE`.
    fn put_object_slots(c: &mut Cpu, obj: u32, slot: u32, word1: u32, word3: u32) {
        c.bus.write32(obj + 0x88, SLOT1);
        c.bus.write32(SLOT1 + 8, 0);
        c.bus.write32(slot + 8, SLOT2);
        c.bus.write32(SLOT2 + 0x1c, BASE);
        c.bus.write32(BASE + 4, word1);
        c.bus.write32(BASE + 12, word3);
    }

    /// The VU0 microprogram at 0x828 the occlusion test runs: `vi1 = vi14`.
    /// Hand-written microcode has no translation: use the reference
    /// interpreter.
    fn put_occlusion_microprogram(c: &mut Cpu) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.vu0.fallback =
            Some(ssx3_ps2::vu::reference::run_program);
        let at = 0x1100_0000 + 0x828;
        c.bus.write32(at, 0x8000_7070); // IADD vi1, vi14, vi0
        c.bus.write32(at + 4, 0x4000_02ff); // NOP, E
    }

    /// Four halfwords (one scissor key).
    fn put_half4(c: &mut Cpu, at: u32, v: [u16; 4]) {
        for (i, x) in v.iter().enumerate() {
            c.bus.write16(at + 2 * i as u32, *x);
        }
    }

    #[test]
    fn world_obj_update_matches() {
        covered(ee::s12::f_003608e8__generated, world_obj_update, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3608, 1);
            let this = DATA;
            c.bus.write32(this + 0xc, DATA + 0x100);
            c.bus.write16(DATA + 0x178, (r.next_u32() as i16 & !3) as u16);
            c.bus.write32(DATA + 0x17c, FAKE);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn wscript_man_is_scripted_instance_matches() {
        check(ee::s10::f_0030a310__generated, wscript_man_is_scripted_instance, SEEDS, &|c, seed| {
            let _r = base(c, seed, 0x30a3);
            let (man, cache, obj) = (DATA, DATA + 0x100, DATA + 0x140);
            let id = (seed % 3) as u8;
            let kind = if seed % 8 == 1 { 5 } else { 3 };
            put_script_cache(c, man, cache, obj, id, kind);
            let slot = TABLE + 12 * id as u32;
            c.bus.write32(obj + 0xc, if seed % 8 == 0 { DATA + 0x700 } else { 0 });
            match seed % 8 {
                2 => c.bus.write32(obj + 0x88, 0),
                3 => {
                    c.bus.write32(obj + 0x88, SLOT1);
                    c.bus.write32(SLOT1 + 8, 0);
                }
                4 => {
                    c.bus.write32(obj + 0x88, SLOT1);
                    c.bus.write32(SLOT1 + 8, 0xffff_ffff);
                    c.bus.write32(slot + 8, SLOT2);
                    c.bus.write32(SLOT2 + 0x1c, BASE);
                }
                5 => put_object_slots(c, obj, slot, 0, 0x55),
                6 => put_object_slots(c, obj, slot, 7, 0),
                _ => put_object_slots(c, obj, slot, 7, 9),
            }
            c.g[A0][0] = sx(man);
            c.g[A1][0] = sx(obj);
        });
    }

    #[test]
    fn world_get_object_script_func_matches() {
        check(ee::s13::f_003a6b78__generated, world_get_object_script_func, SEEDS, &|c, seed| {
            let _r = base(c, seed, 0x3a6b);
            let (cache, obj) = (DATA, DATA + 0x140);
            let id = (seed % 3) as u8;
            put_script_cache(c, DATA + 0x700, cache, obj, id, 3);
            let slot = TABLE + 12 * id as u32;
            match seed % 4 {
                0 => c.bus.write32(obj + 0x88, 0),
                1 => {
                    c.bus.write32(obj + 0x88, SLOT1);
                    c.bus.write32(SLOT1 + 8, 0);
                }
                2 => {
                    c.bus.write32(obj + 0x88, SLOT1);
                    c.bus.write32(SLOT1 + 8, 0xffff_ffff);
                    c.bus.write32(slot + 8, SLOT2);
                    c.bus.write32(SLOT2 + 0x1c, BASE);
                }
                _ => put_object_slots(c, obj, slot, 0x1234, 0x4321),
            }
            c.g[A0][0] = sx(cache);
            c.g[A1][0] = sx(obj);
            c.g[A2][0] = if seed % 2 == 0 { 1 } else { 3 };
        });
    }

    #[test]
    fn world_is_occluded_matches() {
        check(ee::s09::f_002d2058__generated, world_is_occluded, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x2d20);
            put_occlusion_microprogram(c);
            // vi14 (the object's clip word the microprogram copies to vi1):
            // 1 means "occluded".
            c.g[14][0] = (seed % 3) as u64;
            c.ctc2(14, 14);
            let (world, mgr, inst, pts) = (DATA, DATA + 0x100, DATA + 0x200, DATA + 0x3000);
            c.bus.write32((GAME_GP as u32).wrapping_sub(0x848), world);
            c.bus.write32(world + 0x84, mgr);
            c.bus.write32(mgr + 0x8c, inst);
            let count = (seed % 4) as u32;
            c.bus.write32(inst + 0x10c, count);
            c.bus.write32(inst + 0x110, if count == 0 { 0 } else { (r.next_u32() | 1) & 0xf });
            for i in 0..3u32 {
                let m = DATA + 0x1000 + 0x100 * i;
                c.bus.write32(inst + 8 + 8 * i, m);
                for q in 0..5u32 {
                    put_vec(
                        c,
                        m + 0x50 + 0x10 * q,
                        [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                    );
                }
            }
            put_vec(c, pts, [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()]);
            c.g[A0][0] = sx(pts);
        });
    }

    #[test]
    fn benew_race_int_get_peak_rival_character_matches() {
        check(ee::s01::f_00145750__generated, benew_race_int_get_peak_rival_character, SEEDS, &|c, seed| {
            let _r = base(c, seed, 0x1457);
            // `BENewPlayerInt_GetInstance` singleton (no allocation).
            c.bus.write32((GAME_GP as u32).wrapping_sub(0x1f30), DATA + 0x800);
            // `GetSlot(0)` is the slot number the character table is indexed
            // by; 0 -> the byte at 0x534ff1.
            c.bus.write32(0x5305b0, 0);
            c.bus.write8(0x534ff1, (seed % 10) as u8);
            // The peak the race asks for, and the peak record it comes from.
            c.bus.write32(0x535c08, 0);
            c.bus.write32(0x43d950 + 0x54, (seed % 5) as u32);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn mod_obj_v44_matches() {
        covered(ee::s11::f_003569d0__generated, mod_obj_v44, 200, &|c, seed| {
            let _r = machine(c, seed, 0x3569, 1);
            let this = DATA;
            c.g[A0][0] = sx(this);
            let block = DATA + 0x100;
            c.bus.write32(this + 0x1c, if seed % 3 == 0 { 0 } else { block });
            if seed % 3 == 1 {
                c.bus.write32(block, 0);
            } else if seed % 3 == 2 {
                let obj = DATA + 0x200;
                let vt = DATA + 0x300;
                c.bus.write32(block, obj);
                c.bus.write32(obj, vt);
                c.bus.write16(vt + 0x60, 0);
                c.bus.write32(vt + 0x64, FAKE);
            }
        });
    }

    #[test]
    fn mod_obj_v24_matches() {
        covered(ee::s11::f_00356078__generated, mod_obj_v24, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3560, 1);
            let this = DATA;
            c.g[A0][0] = sx(this);
            let head = DATA + 0x100;
            c.bus.write32(this + 0x18, r.next_u32());
            match seed % 3 {
                0 => c.bus.write32(this + 0x1c, 0),
                1 => {
                    c.bus.write32(this + 0x1c, head);
                    c.bus.write32(head, 0);
                }
                _ => {
                    let child = DATA + 0x200;
                    let vt = DATA + 0x300;
                    c.bus.write32(this + 0x1c, head);
                    c.bus.write32(head, child);
                    c.bus.write32(child, vt);
                    c.bus.write16(vt + 0x90, (r.next_u32() % 128) as u16);
                    c.bus.write32(vt + 0x94, FAKE);
                }
            }
        });
    }

    #[test]
    fn spline_modifier_eval_segment_matches() {
        check(ee::s11::f_00345048__generated, spline_modifier_eval_segment, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x3450);
            let curve = DATA;
            for i in 0..4u32 {
                c.bus.write32(curve + 0x50 + 4 * i, r.edge_f32());
            }
            c.bus.write32(curve + 0x84, r.edge_f32());
            for q in 0..4u32 {
                put_vec(
                    c,
                    curve + 0x10 + 0x10 * q,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
            }
            c.g[A0][0] = sx(curve);
            c.f[12] = r.edge_f32();
            c.g[A1][0] = sx(DATA + 0x1000);
            c.g[A2][0] = sx(DATA + 0x1100);
            c.g[A3][0] = sx(DATA + 0x1200);
        });
    }

    #[test]
    fn spline_modifier_eval_path_at_distance_matches() {
        check(ee::s11::f_00345248__generated, spline_modifier_eval_path_at_distance, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x3452);
            let node = DATA;
            let keys = [DATA + 0x1000, DATA + 0x2000, DATA + 0x3000];
            for (i, &k) in keys.iter().enumerate() {
                c.bus.write32(k + 0x84, f(i as f32));
                c.bus.write32(k + 0x60, if i == 0 { 0 } else { keys[i - 1] });
                c.bus.write32(k + 0x64, if i == 2 { 0 } else { keys[i + 1] });
                for j in 0..4u32 {
                    c.bus.write32(k + 0x50 + 4 * j, r.edge_f32());
                }
                for q in 0..4u32 {
                    put_vec(
                        c,
                        k + 0x10 + 0x10 * q,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                }
            }
            let start = (seed % 3) as usize;
            c.bus.write32(node + 0xc, f(1.0 + (seed % 4) as f32));
            c.bus.write32(node + 4, start as u32);
            c.bus.write32(node + 8, keys[start]);
            c.f[12] = uf(&mut r, -2.0, 4.5);
            c.g[A0][0] = sx(node);
            c.g[A1][0] = sx(DATA + 0x5000);
            c.g[A2][0] = sx(DATA + 0x5100);
            c.g[A3][0] = sx(DATA + 0x5200);
            c.pc = EP_ENTRY;
        });
    }

    #[test]
    fn defer_man_set_scissor_matches() {
        check(ee::s12::f_00364b88__generated, defer_man_set_scissor, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x364b);
            let table = DATA;
            let key = DATA + 0x10000;
            let (count_addr, keys) = (table + 0x6aed0, table + 0x6aed4);
            let k = [
                (r.next_u32() % 4) as u16,
                (r.next_u32() % 4) as u16,
                (r.next_u32() % 4) as u16,
                (r.next_u32() % 4) as u16,
            ];
            put_half4(c, key, k);
            let mismatch = |v: [u16; 4], field: usize| {
                let mut w = v;
                w[field] ^= 1;
                w
            };
            let (count, entries): (u32, Vec<[u16; 4]>) = match seed % 6 {
                0 => (0, Vec::new()),
                1 => (1, vec![k]),
                2 => (3, vec![mismatch(k, 0), k, mismatch(k, 3)]),
                3 => (2, vec![mismatch(k, 0), mismatch(k, 1)]),
                4 => (2, vec![mismatch(k, 2), mismatch(k, 3)]),
                _ => (1, vec![mismatch(k, 1)]),
            };
            for (i, e) in entries.iter().enumerate() {
                put_half4(c, keys + 8 * i as u32, *e);
            }
            c.bus.write32(count_addr, count);
            c.g[A0][0] = sx(table);
            c.g[A1][0] = sx(key);
        });
    }
}
