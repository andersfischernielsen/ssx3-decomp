//! Staging module oc747 (wave wave242): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc747`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0030_5f40,
        wscript_set_avalanche_node_emitter,
        "WScriptFn_SetAvalancheNodeEmitter",
    ),
    (
        0x0033_78c0,
        nash_find_closest_object_contact,
        "Nash_FindClosestObjectContact",
    ),
    (
        0x0039_e8b8,
        crate::game::ui::uistate::ui_state_hide_object,
        "UIState_HideObject",
    ),
];

/// `gp` offset of the one-time-init flag of the emitter record template.
pub(crate) const NODE_EMITTER_INIT_FLAG: u32 = 0x278c;
/// The emitter record template (static local) and its type table.
#[cfg(test)]
pub(crate) const NODE_EMITTER_TEMPLATE: u32 = 0x004f_be58;
pub(crate) const NODE_EMITTER_TYPES: u32 = 0x0044_67a0;
/// Word offset (bytes) of the avalanche id inside the record.
pub(crate) const NODE_EMITTER_ID: u32 = 0xd8;

/// `WScriptFn_SetAvalancheNodeEmitter` (0x305f40): script builder. One-time
/// init of the default emitter record, copy of it (0xe0 bytes) to the stack,
/// the `(idx, value, type)` parameter loop (floats converted where the type
/// table asks for it), then, when the record names an avalanche (id != -1),
/// looks up its object through the section table and calls
/// `Avalanche_SetNodeEmitter(obj, record)`. Returns the nil value through the
/// hidden slot in `a0`.
fn set_avalanche_node_emitter_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x140);
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_add(NODE_EMITTER_INIT_FLAG)).lw(c);
    frame.save(c, S1, 0x120);
    c.g[A3][0] = 0x0050_0000;
    frame.save(c, S2, 0x110);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S3, 0x100);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S0, 0x130);
    c.g[S3][0] = c.g[A0][0];
    let initialised = c.g[V0][0] != 0;
    frame.save_ra(c, 0xf0);

    if !initialised {
        node_emitter_init(c);
    }

    c.cycles += 10;
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 0xe0;
    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x41a8));
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0xe8)).sw(c, v0);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x003e_6574, 0x0030_60b0)?;

    c.g[A2][0] = c.g[S2][0];
    let count = c.g[S1][0];
    if (count as i64) > 0 {
        node_emitter_fill(c, count);
    }

    // The avalanche named by the record's id, if any.
    c.cycles += 4;
    let s0 = c.g[S0][0] as u32;
    c.g[A2][0] = Addr(s0.wrapping_add(NODE_EMITTER_ID)).lw(c);
    c.g[V0][0] = !(0u64 | c.g[A2][0]);
    let none = c.g[V0][0] == 0;
    c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
    if !none {
        c.cycles += 8;
        c.g[V0][0] = c.g[A2][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        let found = c.g[V1][0] != 0;
        c.g[A0][0] = 0;
        if found {
            c.cycles += 8;
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let nonzero = c.g[V0][0] != 0;
            c.g[A0][0] = 0;
            if nonzero {
                c.cycles += 1;
                c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
            }
        }
        c.cycles += 2;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x002d_9538, 0x0030_616c)?;
    }

    c.cycles += 16;
    c.g[V1][0] = 0x004d_0000;
    c.g[V0][0] = c.g[S3][0];
    let dst = c.g[S3][0] as u32;
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A3][0] as u32;
    c.ldl(4, src.wrapping_add(7));
    c.ldr(4, src);
    c.g[A1][0] = Addr(src.wrapping_add(8)).lw(c);
    c.sdl(4, dst.wrapping_add(7));
    c.sdr(4, dst);
    let a1 = c.g[A1][0];
    Addr(dst.wrapping_add(8)).sw(c, a1);
    frame.restore(c, S0, 0x130);
    frame.restore(c, S1, 0x120);
    frame.restore(c, S2, 0x110);
    frame.restore(c, S3, 0x100);
    frame.restore_ra(c, 0xf0);
    frame.close(c);
    Ok(())
}

/// The one-time init of the default emitter record (the template at
/// `0x4fbe58`): flags, floats and the id (-1), then the init flag.
fn node_emitter_init(c: &mut Cpu) {
    let gp = c.g[GP][0] as u32;
    c.cycles += 72;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 1);
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x41a8));
    c.g[AT][0] = 0x4080_0000;
    c.mtc1(1, 2);
    c.g[A0][0] = 1;
    c.mtc1(0, 0);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.g[A2][0] = 0x10;
    let base = c.g[V0][0] as u32;
    c.swc1(1, base.wrapping_add(0x8));
    c.g[V1][0] |= 0xffff;
    c.swc1(2, base.wrapping_add(0x10));
    let a0 = c.g[A0][0];
    Addr((c.g[A3][0] as u32).wrapping_sub(0x41a8)).sw(c, a0);
    Addr(base.wrapping_add(0x4)).sw(c, 0);
    c.swc1(3, base.wrapping_add(0xc));
    let mut off = 0x14u32;
    while off <= 0x80 {
        c.swc1(0, base.wrapping_add(off));
        off += 4;
    }
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(1, 1);
    c.swc1(3, base.wrapping_add(0x84));
    c.g[A1][0] = Addr(base.wrapping_add(0xdc)).lbu(c);
    c.swc1(3, base.wrapping_add(0xb4));
    let a2 = c.g[A2][0];
    Addr(base.wrapping_add(0xc4)).sw(c, a2);
    c.swc1(0, base.wrapping_add(0xcc));
    c.swc1(1, base.wrapping_add(0xd4));
    let v1 = c.g[V1][0];
    Addr(base.wrapping_add(0xd8)).sw(c, v1);
    Addr(gp.wrapping_add(NODE_EMITTER_INIT_FLAG)).sw(c, a0);
    for off in [0x88u32, 0x8c, 0x90] {
        c.swc1(0, base.wrapping_add(off));
    }
    for off in [0x94u32, 0x98, 0x9c, 0xa0, 0xa4] {
        c.swc1(3, base.wrapping_add(off));
    }
    for off in [0xa8u32, 0xac, 0xb0, 0xb8, 0xbc, 0xc0] {
        c.swc1(0, base.wrapping_add(off));
    }
    Addr(base.wrapping_add(0xc8)).sw(c, 0);
    Addr(base.wrapping_add(0xd0)).sw(c, a0);
    let a1 = c.g[A1][0];
    Addr(sp(c)).sb(c, a1);
}

/// The parameter loop: `count` (> 0) records of 16 bytes at `A2`; each stores
/// its value into `record[idx]`, as a float when the type table
/// (`0x4467a0`) asks for type 2 and the record's own type differs.
fn node_emitter_fill(c: &mut Cpu, count: u64) {
    c.cycles += 4;
    c.g[V0][0] = 0x0044_0000;
    c.g[A1][0] = count;
    c.g[T0][0] = sx(NODE_EMITTER_TYPES);
    c.g[A3][0] = 2;
    let buf = c.g[S0][0] as u32;
    loop {
        c.cycles += 8;
        let ent = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(ent).lw(c);
        c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(buf));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let slot = c.g[A0][0] as u32;
        let mut store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
            } else {
                c.cycles += 4;
                c.lwc1(0, ent.wrapping_add(4));
                c.cvt_s_w(0, 0);
                c.swc1(0, slot);
                store = false;
            }
        }
        if store {
            c.cycles += 1;
            let v0 = c.g[V0][0];
            Addr(slot).sw(c, v0);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0030_60c8);
        }
    }
}

/// Size of a `tIntersectInfoElem` hit and the hit buffer's capacity.
pub(crate) const NASH_HIT_SIZE: u32 = 0x80;
/// Fields of a hit: the parameter along the query, the object pointers.
pub(crate) const NASH_HIT_IPARAM: u32 = 0x40;
pub(crate) const NASH_HIT_OBJ_A: u32 = 0x50;
pub(crate) const NASH_HIT_OBJ_B: u32 = 0x54;

/// `float Nash_FindClosestObjectContact(Nash* nash, Query* query,
/// tIntersectInfoElem* out, int full)` (0x3378c0): collects up to 128 hits of
/// the query into a 0x4000-byte stack buffer (`Nash_CollideObjects`), asks the
/// query for its ideal parameter (virtual call through the adjusted vtable at
/// `query+0x50`), picks the hit whose `Iparam` is closest to it (ties: the
/// hit whose first object pointer is null, else the one with the smaller
/// `+0x78` / `+0x150` words of its objects), copies that hit to `out` and
/// returns its `Iparam` in `f0`. No hits: -1.0.
fn nash_find_closest_object_contact_body(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x4050);
    c.g[V0][0] = 0x7f;
    frame.save(c, S0, 0x4040);
    c.g[V1][0] = u64::MAX;
    frame.save(c, S3, 0x4010);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S1, 0x4030);
    c.g[S3][0] = c.g[A2][0];
    frame.save(c, S2, 0x4020);
    frame.save_ra(c, 0x4000);
    // The stack probe: 128 empty iterations.
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let more = c.g[V0][0] != c.g[V1][0];
        c.g[T0][0] = c.g[A3][0];
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0033_78e8);
        }
    }

    c.cycles += 7;
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[SP][0];
    c.g[A3][0] = 0x80;
    call_ptr(c, 0x0033_6f30, 0x0033_7914)?;
    c.g[S2][0] = c.g[V0][0];
    if c.g[S2][0] == 0 {
        // No hits.
        c.cycles += 4;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        frame.restore(c, S0, 0x4040);
    } else {
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x50)).lw(c);
        c.cycles += 13;
        c.g[S1][0] = 0;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0033_7944)?;
        c.mov_s(2, 0);
        c.g[A3][0] = 1;
        let first = sp(c).wrapping_add(NASH_HIT_IPARAM);
        c.lwc1(0, first);
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S2][0] as i64)) as u64;
        c.sub_s(0, 0, 2);
        c.abs_s(1, 0);
        let scan = c.g[V0][0] != 0;
        c.g[A2][0] = sx(sp(c).wrapping_add(NASH_HIT_SIZE));
        if scan {
            c.cycles += 1;
            loop {
                'tail: {
                    c.cycles += 6;
                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(NASH_HIT_IPARAM));
                    c.sub_s(0, 0, 2);
                    c.abs_s(0, 0);
                    c.c_lt_s(0, 1);
                    if c.fcc() {
                        // Closer than the best so far.
                        c.cycles += 3;
                        c.mov_s(1, 0);
                        c.g[S1][0] = c.g[A3][0];
                        break 'tail;
                    }
                    c.cycles += 4;
                    c.c_eq_s(0, 1);
                    c.g[V0][0] = sx((c.g[S1][0] as u32) << 7);
                    if !c.fcc() {
                        break 'tail;
                    }
                    // Equal distance: the tie-break.
                    let better = nash_tie_break(c);
                    c.cycles += 1;
                    if better != 0 {
                        c.g[S1][0] = c.g[A3][0];
                    }
                    c.g[V0][0] = better;
                }
                c.cycles += 4;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(NASH_HIT_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0033_7968);
                }
            }
        }
        // Copy the best hit to `out`, 32 bytes at a time.
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[S1][0] as u32) << 7);
        c.g[V1][0] = c.g[S3][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(sp(c)));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(NASH_HIT_SIZE));
        loop {
            c.cycles += 7;
            c.lq(5, c.g[V0][0] as u32);
            c.lq(6, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.sq(5, c.g[V1][0] as u32);
            c.sq(6, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
            let more = c.g[V0][0] != c.g[A0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0033_7a0c);
            }
        }
        c.cycles += 2;
        c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(NASH_HIT_IPARAM));
        frame.restore(c, S0, 0x4040);
    }
    c.cycles += 6;
    frame.restore(c, S1, 0x4030);
    frame.restore(c, S2, 0x4020);
    frame.restore(c, S3, 0x4010);
    frame.restore_ra(c, 0x4000);
    frame.close(c);
    Ok(())
}

/// The equal-distance tie-break of `Nash_FindClosestObjectContact` (hit `A2`
/// against the best hit, `V0 = best * 0x80`): is the hit better? Leaves the
/// answer in `V0` (the generated code's scratch registers too).
fn nash_tie_break(c: &mut Cpu) -> u64 {
    c.cycles += 3;
    c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(NASH_HIT_OBJ_A)).lw(c);
    let a_null = c.g[A0][0] == 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(sp(c)));
    if !a_null {
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(NASH_HIT_OBJ_A)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 1;
            return 1;
        }
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x78)).lw(c);
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x78)).lw(c);
        c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        return c.g[V0][0];
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(NASH_HIT_OBJ_A)).lw(c);
    let best_a = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if best_a {
        return 0;
    }
    c.cycles += 5;
    c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(NASH_HIT_OBJ_B)).lw(c);
    c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(NASH_HIT_OBJ_B)).lw(c);
    c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x150)).lw(c);
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x150)).lw(c);
    c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
    c.g[V0][0]
}

/// `Nash_FindClosestObjectContact` (0x3378c0): see
/// [`nash_find_closest_object_contact_body`].
pub fn nash_find_closest_object_contact(c: &mut Cpu) {
    guest(c, nash_find_closest_object_contact_body);
}

/// `WScriptFn_SetAvalancheNodeEmitter` (0x305f40): see
/// [`set_avalanche_node_emitter_body`].
pub fn wscript_set_avalanche_node_emitter(c: &mut Cpu) {
    guest(c, set_avalanche_node_emitter_body);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use crate::game::ui::uistate::tests as hide;

    /// `WScriptFn_SetAvalancheNodeEmitter`: both init paths, 0-3 parameter
    /// records (idx 0..2 and the id slot, word and float conversions), and the
    /// avalanche lookup with a missing, empty or live section entry. The
    /// avalanche list head (`gp+0x9c0`) is 0 so the real
    /// `Avalanche_SetNodeEmitter` returns at once.
    #[test]
    fn wscript_set_avalanche_node_emitter_matches() {
        check(
            ee::s10::f_00305f40__generated,
            wscript_set_avalanche_node_emitter,
            400,
            &|c, seed| {
                let mut r = Rng(0x305f_40 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_5f40;
                c.g[GP][0] = sx(GAME_GP);
                c.bus.write32(GAME_GP + NODE_EMITTER_INIT_FLAG, seed % 2);
                c.bus.write32(GAME_GP + 0x9c0, 0);
                // Template contents (read when the init is skipped).
                for i in 0..0x38u32 {
                    c.bus.write32(NODE_EMITTER_TEMPLATE + 4 * i, r.next_u32());
                }
                // Section table: gp+0x16c8 -> table -> records -> entries.
                c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, DATA + 0x4000);
                c.bus.write32(DATA + 0x4008, DATA + 0x5000);
                for j in 0..4u32 {
                    let sec = DATA + 0x6000 + 0x100 * j;
                    c.bus.write32(
                        DATA + 0x5000 + 4 * j,
                        if (seed + j) % 5 == 0 { 0 } else { sec },
                    );
                    c.bus.write32(sec + 0x1c, DATA + 0x7000 + 0x100 * j);
                    for m in 0..4u32 {
                        let e = if (seed + m + j) % 3 == 0 {
                            0
                        } else {
                            (1 + r.next_u32() % 255) << 8
                        };
                        c.bus.write32(DATA + 0x7000 + 0x100 * j + 4 * m, e);
                    }
                }
                // Type table (idx 0..3 and the id slot 54).
                for i in 0..4u32 {
                    c.bus.write32(NODE_EMITTER_TYPES + 4 * i, r.next_u32() % 3);
                }
                c.bus.write32(
                    NODE_EMITTER_TYPES + 4 * (NODE_EMITTER_ID / 4),
                    r.next_u32() % 3,
                );
                let n = seed / 2 % 4;
                for i in 0..n {
                    let e = DATA + 0x9100 + 16 * i;
                    let id_slot = i == n - 1 && seed % 3 != 0;
                    c.bus
                        .write32(e, if id_slot { NODE_EMITTER_ID / 4 } else { i });
                    let key = (((seed / 8) % 4) << 8) | ((seed / 32) % 4);
                    c.bus
                        .write32(e + 4, if id_slot { key } else { r.next_u32() });
                    c.bus.write32(e + 0xc, r.next_u32() % 3);
                }
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    /// `Nash_FindClosestObjectContact`: no root cell, so the real
    /// `Nash_CollideObjects` returns the count at `nash+0x24` and leaves the
    /// hit buffer (the test pre-fills it on the stack). 0-5 hits with
    /// `Iparam`s from a small set (closer, farther and exactly equal
    /// distances to the ideal value), objects with and without the first
    /// pointer, and both tie-break comparisons. The ideal value comes from a
    /// real getter (`CollSegment_GetDiameter`, `lwc1 f0, 0xa0(a0)`) on half the
    /// seeds, else from a fake method (unwinds on some seeds; `Nash_CollideObjects`
    /// has no override slot to fake, so the fakes are not counted: `check`).
    #[test]
    fn nash_find_closest_object_contact_matches() {
        check(
            ee::s11::f_003378c0__generated,
            nash_find_closest_object_contact,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0033_78c0, 1);
                c.pc = 0x0033_78c0;
                let (nash, query, vt, out, objs) = (
                    DATA + 0x100,
                    DATA + 0x200,
                    DATA + 0x300,
                    DATA + 0x500,
                    DATA + 0x2000,
                );
                let sp0 = DATA + 0x3_0000;
                c.g[SP][0] = sx(sp0);
                let buf = sp0 - 0x4050;
                let n = if seed % 5 == 0 { 0 } else { 1 + (seed / 5) % 5 };
                c.bus.write32(nash + 0x10, 0);
                c.bus.write32(nash + 0x24, n);
                for k in 0..8u32 {
                    c.bus.write32(query + 0x20 + 4 * k, r.next_u32());
                }
                c.bus.write32(query + 0x50, vt);
                c.bus.write16(vt + 0x18, 0);
                let real = seed % 2 == 0;
                c.bus
                    .write32(vt + 0x1c, if real { 0x0034_0a10 } else { FAKE });
                let ideal = if r.next_u32() % 2 == 0 { 1.0f32 } else { 2.0 };
                c.bus.write32(query + 0xa0, ideal.to_bits());
                let params = [0.5f32, 1.0, 1.5, 2.0, 2.5, 3.0];
                let words = [1u32, 2, 3, 0x8000_0000];
                for k in 0..4u32 {
                    let o = objs + 0x400 * k;
                    c.bus.write32(o + 0x78, words[(r.next_u32() % 4) as usize]);
                    c.bus.write32(o + 0x150, words[(r.next_u32() % 4) as usize]);
                }
                for i in 0..n {
                    let h = buf + NASH_HIT_SIZE * i;
                    for w in 0..0x20u32 {
                        c.bus.write32(h + 4 * w, r.next_u32());
                    }
                    let p = params[(r.next_u32() % 6) as usize];
                    c.bus.write32(h + NASH_HIT_IPARAM, p.to_bits());
                    let a = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        objs + 0x400 * (r.next_u32() % 4)
                    };
                    c.bus.write32(h + NASH_HIT_OBJ_A, a);
                    c.bus
                        .write32(h + NASH_HIT_OBJ_B, objs + 0x400 * (r.next_u32() % 4));
                }
                c.g[A0][0] = sx(nash);
                c.g[A1][0] = sx(query);
                c.g[A2][0] = sx(out);
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// `UIState_HideObject(state, name)`: walks the state's object list for the
    /// first screen object whose hash name matches and calls its virtual
    /// `hide` (slot `+0x4c`, this adjusted by the `lh` at `+0x48`). Paths:
    /// empty list, a matching node, a node without a screen (list end).
    #[test]
    fn ui_state_hide_object_matches() {
        check(
            ee::s13::f_0039e8b8__generated,
            crate::game::ui::uistate::ui_state_hide_object,
            hide::SEEDS,
            &|c, seed| {
                machine(c, seed, 0x39e8, 0);
                c.pc = 0x0039_e8b8;
                hide::env(c);
                let this = DATA + 0x400;
                let name = DATA + 0x800;
                hide::cstr(c, name, "OBJ_HELLO");
                let list = this + 0x24;
                let sentinel = DATA + 0x1000;
                let node = DATA + 0x1100;
                hide::w32(c, sentinel + 4, sentinel);
                match seed % 3 {
                    0 => {
                        hide::w32(c, list + 4, sentinel);
                    }
                    1 => {
                        let obj = DATA + 0x1200;
                        let vt = DATA + 0x1300;
                        hide::w32(c, node, sentinel);
                        hide::w32(c, node + 4, sentinel);
                        hide::w32(c, list + 4, node);
                        hide::w32(c, node + 0x38, DATA + 0x1400);
                        hide::w32(c, node + 0x3c, 1);
                        hide::w32(c, DATA + 0x1400, obj);
                        let hv = hide::hash(c, name);
                        hide::w32(c, obj + 0x38, hv);
                        hide::w32(c, obj + 8, vt);
                        hide::slot(c, vt, 0x48, hide::EMPTY);
                    }
                    _ => {
                        hide::w32(c, node, sentinel);
                        hide::w32(c, node + 4, sentinel);
                        hide::w32(c, list + 4, node);
                        hide::w32(c, node + 0x3c, 0);
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
            },
        );
    }
}
