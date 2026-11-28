//! Staging module oc1576 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1576`.

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
    (0x0030_1120, wscript_fn_collect_item, "WScriptFn_CollectItem"),
    (0x0030_2048, wscript_fn_start_event, "WScriptFn_StartEvent"),
    (0x0030_3f80, rider_in_tunnel_func, "RiderInTunnelFunc"),
    (0x0030_7128, wscript_mission_ctor, "WScriptMission_ctor"),
    (0x0030_71c8, wscript_mission_dtor, "WScriptMission_dtor"),
    (0x0030_71f0, wscript_mission_v04, "WScriptMission_v04"),
    (0x0030_7240, wscript_mission_v02, "WScriptMission_v02"),
    (0x0030_72b0, wscript_mission_v17, "WScriptMission_v17"),
];

/// Guest functions the replacements call directly (`jal`), with their
/// generated entries.
const WSMAN_COLLECT_ITEM: Callee = Callee {
    addr: 0x0030_b9a0,
    f: ee::s10::f_0030b9a0,
};
const WSMAN_IS_PROCESS_ACTIVE: Callee = Callee {
    addr: 0x0030_b898,
    f: ee::s10::f_0030b898,
};
const WSMAN_DEACTIVATE_ALL: Callee = Callee {
    addr: 0x0030_b7f8,
    f: ee::s10::f_0030b7f8,
};
const WSMAN_LOOKUP_FUNC: Callee = Callee {
    addr: 0x0030_9b70,
    f: ee::s10::f_00309b70,
};
const WSPROC_CTOR: Callee = Callee {
    addr: 0x0030_7598,
    f: ee::s10::f_00307598,
};
const WSPROC_DTOR: Callee = Callee {
    addr: 0x0030_7738,
    f: ee::s10::f_00307738,
};
const WSPROC_V04: Callee = Callee {
    addr: 0x0030_7c40,
    f: ee::s10::f_00307c40,
};

/// Indirect (`jalr`) targets of the replacements.
const MEM_COPY: u32 = 0x003e_6574;
const LUNO_CAST_INT: u32 = 0x0022_6610;
const SECTION_EVENTBEGIN: u32 = 0x0022_d6c8;

/// The `Nil` image the game copies from: the register pair loads it from
/// `0x4d0000 - 0x6f68` (`0x4c9098`).
const NIL_BASE_HI: u64 = 0x004d_0000;
const NIL_OFFSET: u32 = 0x6f68;

/// Part of `WScriptFn_CollectItem` and `WScriptFn_StartEvent`: the record of
/// the current being. The world at `gp-0x848` gives the per-being table at
/// `+0x84` -> `+0xc` (indexed by `being * 4`, entry at `+0x28`); the record's
/// `+0x6c0` block holds the callback whose address is at `+0x4c` and whose
/// argument offset is the halfword at `+0x48`. The callback runs with
/// `A0 = record + 0x6c0 + offset`, and V0 becomes its result inverted.
/// `charge` is the cycles the generated block spends before the call.
fn call_being_entry(c: &mut Cpu, being: u32, ret_addr: u32, charge: u64) -> Flow {
    c.cycles += charge;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[A1][0] = sx(being << 2);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    if c.g[A0][0] == 0 {
        c.g[V1][0] = 0;
    }
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c0));
    if c.g[V1][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x48));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    call_ptr(c, target, ret_addr)?;
    c.g[V0][0] ^= 1;
    Ok(())
}

/// The keyed-argument loop of `WScriptFn_CollectItem` and `WScriptFn_StartEvent`:
/// for each of `count` entries at `A2` (`{ idx, value, kind }`, 16 bytes),
/// store the value at `base + 4*idx`, converted to float when the table
/// `table` declares that field a float (kind 2) and the entry's kind differs.
/// `head` is the loop's own address (the back-edge event check resumes there).
fn copy_keyed_args(c: &mut Cpu, base: u32, table: u32, count: u64, head: u32) {
    c.g[A1][0] = count;
    c.g[T0][0] = sx(table);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(base));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.cycles += 1;
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `WScriptFn_CollectItem` (0x301120, `CollectFunc`): gathers the keyed
/// arguments of a script call into a 12-byte block on the stack (the
/// defaults come from a static copy, guarded by a one-time init), and when
/// the item id is set, collects that item for the current being. Returns
/// `self` with its word set to 1 (collected) or 0.
pub fn wscript_fn_collect_item(c: &mut Cpu) {
    guest(c, collect_item_body);
}

fn collect_item_body(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x80);
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
    frame.save(c, S1, 0x60);
    c.g[V0][0] = u64::MAX;
    frame.save(c, S3, 0x40);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S4, 0x30);
    c.g[S3][0] = c.g[A1][0];
    frame.save(c, S0, 0x70);
    frame.save_ra(c, 0x20);
    frame.save(c, S2, 0x50);
    c.g[S2][0] = c.lw(c.g[V1][0] as u32);
    let none = c.g[S2][0] == c.g[V0][0];
    c.g[S4][0] = c.g[A2][0];
    if none {
        c.cycles += 1;
        c.g[V0][0] = 0;
    } else {
        let being = c.g[S2][0] as u32;
        call_being_entry(c, being, 0x0030_1198, 18)?;
    }
    collect_item_part_b(c)?;
    c.cycles += 9;
    c.g[V0][0] = c.g[S1][0];
    frame.restore(c, S0, 0x70);
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// The second half of `WScriptFn_CollectItem`: the argument block and the
/// collection. Ends with the result call that stores V0 into `self + 8`
/// (the cast) and `self`'s word; the collect path makes its own calls and
/// skips that tail.
fn collect_item_part_b(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let a_zero = c.g[V0][0] == 0;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x25c4));
    if a_zero {
        // The part A result was 1: nothing to collect; V0 = 1 below.
        c.cycles += 1;
        c.g[V0][0] = 1;
    } else {
        c.cycles += 2;
        let inited = c.g[V0][0] != 0;
        c.g[A2][0] = 0x500000;
        if !inited {
            c.cycles += 10;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4430));
            c.g[V0][0] = u64::MAX;
            c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
            c.sw((c.g[A2][0] as u32).wrapping_sub(0x4430), c.g[V0][0]);
            c.g[A0][0] = 1;
            c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x25c4), c.g[A0][0]);
            c.sb(sp(c), c.g[A1][0]);
        }
        // The argument block at sp+0x10: the copied defaults, then the keys.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4430));
        c.g[A2][0] = 0xc;
        c.sw(sp(c).wrapping_add(0x14), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, MEM_COPY, 0x0030_11fc)?;
        let no_keys = (c.g[S3][0] as i64) <= 0;
        c.g[A2][0] = c.g[S4][0];
        if !no_keys {
            c.cycles += 3;
            let base = c.g[S0][0] as u32;
            let table = (c.g[GP][0] as u32).wrapping_add(0xb78);
            let count = c.g[S3][0];
            copy_keyed_args(c, base, table, count, 0x0030_1210);
        }
        c.cycles += 5;
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        let no_item = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 1;
        if !no_item {
            // The item id is set: collect it for the current being.
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            if c.g[V0][0] != 0 {
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            }
            c.cycles += 10;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            c.g[S0][0] = 1;
            WSMAN_COLLECT_ITEM.call(c, 0x0030_12a4)?;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, LUNO_CAST_INT, 0x0030_12b0)?;
            c.sw(c.g[V0][0] as u32, c.g[S0][0]);
            return Ok(());
        }
    }
    // The tail: `self + 8` takes V0 through the cast, and `*V0 = 0`.
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, LUNO_CAST_INT, 0x0030_12c8)?;
    c.sw(c.g[V0][0] as u32, 0);
    Ok(())
}

/// `WScriptFn_StartEvent` (0x302048): gathers the keyed arguments like
/// `WScriptFn_CollectItem`, then starts the section event the block names
/// (the mode from the table at `0x445e40`, after deactivating the missions
/// when the event is not a free ride). Returns the `Nil` result copied from
/// the game image.
pub fn wscript_fn_start_event(c: &mut Cpu) {
    guest(c, start_event_body);
}

fn start_event_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x70);
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x263c));
    frame.save(c, S2, 0x40);
    frame.save(c, S0, 0x60);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S3, 0x30);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S1, 0x50);
    c.g[S3][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);
    let inited = c.g[V0][0] != 0;
    c.g[A2][0] = 0x500000;
    if !inited {
        c.cycles += 9;
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x43f0));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 1;
        c.sw((c.g[A2][0] as u32).wrapping_sub(0x43f0), c.g[V1][0]);
        c.sb((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x263c), c.g[A1][0]);
        c.sb(sp(c), c.g[A0][0]);
    }
    // The argument block at sp+0x10 (copied from the defaults at 0x4fc10).
    c.cycles += 7;
    c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x43f0));
    c.g[A2][0] = 0xc;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, MEM_COPY, 0x0030_20b0)?;
    let no_keys = (c.g[S0][0] as i64) <= 0;
    c.g[A2][0] = c.g[S2][0];
    if !no_keys {
        c.cycles += 4;
        let base = c.g[S1][0] as u32;
        let table = (c.g[GP][0] as u32).wrapping_add(0xbd0);
        let count = c.g[S0][0];
        copy_keyed_args(c, base, table, count, 0x0030_20c8);
    }
    // The being's record, when there is one.
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
    let no_being = c.g[V0][0] == c.g[V1][0];
    if no_being {
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.cycles += 1;
        c.g[V0][0] = 0;
    } else {
        let being = c.g[V0][0] as u32;
        call_being_entry(c, being, 0x0030_2160, 17)?;
    }
    // The section event for the block's mode, unless the block has a second
    // value (then it is left alone).
    c.cycles += 2;
    let a_zero = c.g[V0][0] == 0;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    let mut exit_early = false;
    if !a_zero {
        c.cycles += 7;
        c.g[S0][0] = 0;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = (c.g[A1][0] < 0x8u64) as u64;
        let id_in_range = c.g[V0][0] != 0;
        c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
        if id_in_range {
            c.cycles += 5;
            c.g[V0][0] = 0x440000;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5e40));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = c.lw(c.g[V1][0] as u32);
        }
        c.cycles += 2;
        let second_set = c.g[A0][0] != 0;
        c.g[V1][0] = NIL_BASE_HI;
        if second_set {
            exit_early = true;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let free_ride = c.g[S0][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            if !free_ride {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                WSMAN_DEACTIVATE_ALL.call(c, 0x0030_21c0)?;
                c.g[A0][0] = c.g[S1][0];
            }
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, SECTION_EVENTBEGIN, 0x0030_21cc)?;
        }
    }
    if !exit_early {
        c.cycles += 1;
        c.g[V1][0] = NIL_BASE_HI;
    }
    // The epilogue: copy the Nil result to `self` and return `self`.
    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(NIL_OFFSET));
    let nil = c.g[A3][0] as u32;
    c.ldl(A0, nil.wrapping_add(0x7));
    c.ldr(A0, nil);
    c.g[A1][0] = c.lw(nil.wrapping_add(0x8));
    let this = c.g[S3][0] as u32;
    c.sdl(A0, this.wrapping_add(0x7));
    c.sdr(A0, this);
    c.sw(this.wrapping_add(0x8), c.g[A1][0]);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `RiderInTunnelFunc` (0x303f80): when the current being is set, marks its
/// rider as in a tunnel (the `+0x3fc` flag of its record); returns the `Nil`
/// result copied from the game image.
pub fn rider_in_tunnel_func(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = c.lw(c.g[V1][0] as u32);
    let no_being = c.g[V1][0] == c.g[V0][0];
    c.g[A3][0] = c.g[A0][0];
    if !no_being {
        c.cycles += 12;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A2][0] = 1;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        if c.g[A0][0] == 0 {
            c.g[V1][0] = 0;
        }
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c0));
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0;
        }
        c.sw((c.g[V0][0] as u32).wrapping_add(0x3fc), c.g[A2][0]);
    }
    c.cycles += 10;
    c.g[V1][0] = NIL_BASE_HI;
    c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(NIL_OFFSET));
    let nil = c.g[T0][0] as u32;
    c.ldl(A0, nil.wrapping_add(0x7));
    c.ldr(A0, nil);
    c.g[A1][0] = c.lw(nil.wrapping_add(0x8));
    let this = c.g[A3][0] as u32;
    c.sdl(A0, this.wrapping_add(0x7));
    c.sdr(A0, this);
    c.sw(this.wrapping_add(0x8), c.g[A1][0]);
    c.g[V0][0] = c.g[A3][0];
    ret(c);
}

/// `WScriptMission_ctor` (0x307128, `cWScriptMission_cWScriptMission`): the
/// process base constructor, then the vtable, the entry points of the
/// mission's four-entry list (looked up one by one) and the list's tail.
/// Returns `self`.
pub fn wscript_mission_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        c.g[S0][0] = c.g[A3][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
        WSPROC_CTOR.call(c, 0x0030_715c)?;
        c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x68));
        c.g[V0][0] = 0x490000;
        c.g[S5][0] = 3;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x65d0));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x64), c.g[S1][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x60), c.g[V1][0]);
        loop {
            c.cycles += 8;
            c.g[A1][0] = c.lw(c.g[S4][0] as u32);
            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(1));
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(4));
            WSMAN_LOOKUP_FUNC.call(c, 0x0030_7194)?;
            c.sw(c.g[S3][0] as u32, c.g[V0][0]);
            let more = (c.g[S5][0] as i64) >= 0;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_7180);
            }
        }
        c.cycles += 10;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptMission_dtor` (0x3071c8, `cWScriptMission__cWScriptMission`): the
/// process base destructor after setting the mission vtable.
pub fn wscript_mission_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x65d0));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
        WSPROC_DTOR.call(c, 0x0030_71e4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptMission_v04` (0x3071f0, `WScriptMission_processStart`): the
/// record of the mission's index in its owner's table, its start hook at
/// 0x10f2b8, then `WScriptProcess_v04(self)`.
pub fn wscript_mission_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x0010_f2b8, 0x0030_7224)?;
        c.g[A0][0] = c.g[S0][0];
        WSPROC_V04.call(c, 0x0030_722c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptMission_v02` (0x307240, `WScriptMission_processComplete`): when the
/// process is still active, the mission's two state callbacks, then the
/// audio hook and `big challenge complete` on its result.
pub fn wscript_mission_v02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        WSMAN_IS_PROCESS_ACTIVE.call(c, 0x0030_725c)?;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 15;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_7278)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x90));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_728c)?;
            call_ptr(c, 0x0028_b180, 0x0030_7294)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0029_d8e0, 0x0030_729c)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptMission_v17` (0x3072b0, `WScriptMission_missionRestart`): when the
/// process is in state 2 (update), its first callback; then the second.
pub fn wscript_mission_v17(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 2;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        let not_update = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
        if !not_update {
            c.cycles += 5;
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_72e0)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
        }
        c.cycles += 8;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0030_72f4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine_rt};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// This module's gate word (the gated fakes below answer only when it is
    /// set in their own machine), its mailbox word (GATE + 4), and the magic.
    const GATE: u32 = DATA + 0x0070_1570;
    const MAGIC: u32 = 0x1576_0001;
    /// The WScriptMan: `+0` the being, `+0x290` the instance.
    const WS: u32 = DATA + 0x0070_2000;
    const INST: u32 = DATA + 0x0070_2400;
    /// The world (`gp-0x848`): `+0x84` the section data, `+0xc` the per-being
    /// table at `+0x28`, `+0x78` the section manager.
    const WORLD: u32 = DATA + 0x0070_3000;
    const WORLD_P: u32 = DATA + 0x0070_3400;
    const WORLD_Q: u32 = DATA + 0x0070_3800;
    const SECTION: u32 = DATA + 0x0070_3c00;
    /// Records `RECORDS + 0x100 k` and their callback blocks `TARGETS + 0x100 k`.
    const RECORDS: u32 = DATA + 0x0070_4000;
    const TARGETS: u32 = DATA + 0x0070_5000;
    const PARAMS: u32 = DATA + 0x0070_6000;
    const SELF: u32 = DATA + 0x0070_7000;
    const OBJ: u32 = DATA + 0x0070_8000;
    const MGR: u32 = DATA + 0x0070_9000;
    const STATE: u32 = DATA + 0x0070_a000;
    /// The keyed-argument defaults at `0x4fbd0` (collect) and `0x4fc10`
    /// (start event), the `Nil` image at `0x4c9098`, the mode table at `0x445e40`.
    const COLLECT_DEFAULTS: u32 = 0x0004_fbd0;
    const EVENT_DEFAULTS: u32 = 0x0004_fc10;
    const NIL_IMAGE: u32 = 0x004c_9098;
    const MODE_TABLE: u32 = 0x0044_5e40;

    fn oc1576_no_prev(_c: &mut Cpu) {
        unreachable!("oc1576 fake called before it was hooked");
    }

    /// Gated fake of `WScriptMan_CollectItem` (0x30b9a0): logs the call.
    fn oc1576_fake_collect_item(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1576_fake_collect_item),
                oc1576_no_prev,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0030_b9a0, ra);
    }

    /// Gated fake of `luno_cLunoVar_castInt` (0x226610): logs the call.
    fn oc1576_fake_cast_int(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1576_fake_cast_int), oc1576_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0022_6610, ra);
    }

    /// Gated fake of `WScriptMan_DeactivateAll` (0x30b7f8).
    fn oc1576_fake_deactivate(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1576_fake_deactivate), oc1576_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0030_b7f8, ra);
    }

    /// Gated fake of `cSectionMan_eventbegin` (0x22d6c8).
    fn oc1576_fake_eventbegin(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1576_fake_eventbegin), oc1576_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0022_d6c8, ra);
    }

    /// Gated fake of `WScriptProcess_ctor` (0x307598).
    fn oc1576_fake_process_ctor(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1576_fake_process_ctor),
                oc1576_no_prev,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0030_7598, ra);
    }

    /// Gated fake of `WScriptMan_LookupFunc` (0x309b70).
    fn oc1576_fake_lookup_func(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1576_fake_lookup_func), oc1576_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0030_9b70, ra);
    }

    /// Gated fake of `WScriptProcess_dtor` (0x307738).
    fn oc1576_fake_process_dtor(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1576_fake_process_dtor),
                oc1576_no_prev,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0030_7738, ra);
    }

    /// Gated fake of the rider start hook (0x10f2b8).
    fn oc1576_fake_rider_flag(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1576_fake_rider_flag), oc1576_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0010_f2b8, ra);
    }

    /// Gated fake of `WScriptProcess_processStart` (0x307c40).
    fn oc1576_fake_process_v04(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1576_fake_process_v04), oc1576_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0030_7c40, ra);
    }

    /// Gated fake of `WScriptMan_IsProcessActive` (0x30b898): its result is
    /// the mailbox word (GATE + 4).
    fn oc1576_fake_is_active(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1576_fake_is_active), oc1576_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0030_b898, ra);
        c.g[V0][0] = c.bus.read32(GATE + 4) as u64;
    }

    /// Gated fake of the audio hook (0x28b180).
    fn oc1576_fake_bx_audio(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1576_fake_bx_audio), oc1576_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0028_b180, ra);
    }

    /// Gated fake of `cSSXAudio_BigChallengeComplete` (0x29d8e0).
    fn oc1576_fake_big_challenge(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1576_fake_big_challenge),
                oc1576_no_prev,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0029_d8e0, ra);
    }

    /// The WScriptMan with the given being (`gp+0xce8`).
    fn wscript_man(c: &mut Cpu, being: u32) {
        c.bus.write32(GAME_GP + 0xce8, WS);
        c.bus.write32(WS, being);
        c.bus.write32(WS + 0x290, INST);
        c.bus.write32(INST + 0x78, 0x0000_0042);
    }

    /// The world (`gp-0x848`) with a record for each being; the record is
    /// zero for the beings whose bit is set in `zero_mask`.
    fn world(c: &mut Cpu, r: &mut Rng, zero_mask: u32) {
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, WORLD_P);
        c.bus.write32(WORLD_P + 0xc, WORLD_Q);
        c.bus.write32(WORLD_P + 0x78, SECTION);
        c.bus.write32(0x6c0, TARGETS);
        for k in 0..4u32 {
            let rec = RECORDS + 0x100 * k;
            let target = TARGETS + 0x100 * k;
            let zero = (zero_mask >> k) & 1 == 1;
            c.bus
                .write32(WORLD_Q + 0x28 + 4 * k, if zero { 0 } else { rec });
            c.bus.write32(rec + 0x6c0, target);
            c.bus.write16(target + 0x48, (r.next_u32() % 0x40) as u16);
            c.bus.write32(target + 0x4c, fakes::FAKE);
        }
    }

    /// Three keyed-argument entries at `PARAMS`: an index below `idx_max`, a
    /// value, and a kind (1 to 3, so the float conversion is hit too).
    fn keyed_entries(c: &mut Cpu, r: &mut Rng, idx_max: u32) {
        for i in 0..3u32 {
            let p = PARAMS + 16 * i;
            c.bus.write32(p, r.next_u32() % idx_max);
            c.bus.write32(p + 4, r.next_u32());
            c.bus.write32(p + 0xc, 1 + r.next_u32() % 3);
        }
    }

    /// `WScriptFn_CollectItem`: no record, a record with and without the
    /// fake's forced result, the init guard on and off, item ids set and
    /// unset, the float conversion and the raw copy, and zero to three keys.
    #[test]
    fn wscript_fn_collect_item_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030B9A0),
            oc1576_fake_collect_item,
            fakes::prev_cell(oc1576_fake_collect_item),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00226610),
            oc1576_fake_cast_int,
            fakes::prev_cell(oc1576_fake_cast_int),
        );
        check(
            ee::s10::f_00301120__generated,
            wscript_fn_collect_item,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0030_1120, 4);
                c.bus.write32(GATE, MAGIC);
                let being = [u32::MAX, 0, 1, 2, 3][(seed % 5) as usize];
                wscript_man(c, being);
                world(c, &mut r, (seed >> 2) & 0xf);
                c.bus.write32(GAME_GP + 0x25c4, seed % 2);
                for k in 0..4u32 {
                    c.bus.write32(GAME_GP + 0xb78 + 4 * k, 1 + r.next_u32() % 3);
                }
                let first = if seed % 3 == 0 {
                    u32::MAX
                } else {
                    r.next_u32()
                };
                c.bus.write32(COLLECT_DEFAULTS, first);
                c.bus.write32(COLLECT_DEFAULTS + 4, r.next_u32());
                c.bus.write32(COLLECT_DEFAULTS + 8, r.next_u32());
                keyed_entries(c, &mut r, 3);
                if seed % 4 == 1 {
                    c.bus.write32(fakes::LOG_RESULTS, 1);
                }
                let n = [0u32, 1, 2, 3, u32::MAX][(seed % 5) as usize];
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(PARAMS);
            },
        );
    }

    /// `WScriptFn_StartEvent`: the being with and without a record, the second
    /// value set and unset, the modes of the table (free ride or not), and
    /// the init guard on and off.
    #[test]
    fn wscript_fn_start_event_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030B7F8),
            oc1576_fake_deactivate,
            fakes::prev_cell(oc1576_fake_deactivate),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D6C8),
            oc1576_fake_eventbegin,
            fakes::prev_cell(oc1576_fake_eventbegin),
        );
        check(
            ee::s10::f_00302048__generated,
            wscript_fn_start_event,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0030_2048, 4);
                c.bus.write32(GATE, MAGIC);
                let being = [u32::MAX, 0, 1, 2, 3][(seed % 5) as usize];
                wscript_man(c, being);
                world(c, &mut r, (seed >> 2) & 0xf);
                c.bus.write32(GAME_GP + 0x263c, seed % 2);
                for k in 0..4u32 {
                    c.bus.write32(GAME_GP + 0xbd0 + 4 * k, 1 + r.next_u32() % 3);
                }
                for k in 0..8u32 {
                    c.bus.write32(MODE_TABLE + 4 * k, r.next_u32() % 3);
                }
                let id = if seed % 4 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 10
                };
                c.bus.write32(EVENT_DEFAULTS, id);
                c.bus
                    .write32(EVENT_DEFAULTS + 4, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(EVENT_DEFAULTS + 8, r.next_u32());
                keyed_entries(c, &mut r, 2);
                if seed % 4 == 1 {
                    c.bus.write32(fakes::LOG_RESULTS, 1);
                }
                let n = [0u32, 1, 2, 3, u32::MAX][(seed % 5) as usize];
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(PARAMS);
            },
        );
    }

    /// `RiderInTunnelFunc`: a being with a record (zero or not), and none.
    #[test]
    fn rider_in_tunnel_func_matches() {
        check(
            ee::s10::f_00303f80__generated,
            rider_in_tunnel_func,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0030_3f80, 0);
                let being = [u32::MAX, 0, 1, 2, 3][(seed % 5) as usize];
                wscript_man(c, being);
                world(c, &mut r, (seed >> 2) & 0xf);
                fill(c, &mut r, NIL_IMAGE, 3);
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    /// `WScriptMission_ctor`: the base constructor's fake, then the four
    /// list entries through the lookup fake, the list at `self`'s `+0x40`.
    #[test]
    fn wscript_mission_ctor_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00307598),
            oc1576_fake_process_ctor,
            fakes::prev_cell(oc1576_fake_process_ctor),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00309B70),
            oc1576_fake_lookup_func,
            fakes::prev_cell(oc1576_fake_lookup_func),
        );
        check(
            ee::s10::f_00307128__generated,
            wscript_mission_ctor,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0030_7128, 0);
                c.bus.write32(GATE, MAGIC);
                for i in 0..4u32 {
                    c.bus.write32(OBJ + 0x40 + 4 * i, r.next_u32());
                }
                c.bus.write32(OBJ + 0x50, r.next_u32());
                c.bus.write32(SELF + 0x10, MGR);
                c.g[A0][0] = sx(SELF);
                c.g[A3][0] = sx(OBJ);
                c.g[T0][0] = sx(0x0040_1000 + seed);
            },
        );
    }

    /// `WScriptMission_dtor`: the base destructor's fake, after the vtable store.
    #[test]
    fn wscript_mission_dtor_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00307738),
            oc1576_fake_process_dtor,
            fakes::prev_cell(oc1576_fake_process_dtor),
        );
        check(
            ee::s10::f_003071c8__generated,
            wscript_mission_dtor,
            100,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x0030_71c8, 0);
                c.bus.write32(GATE, MAGIC);
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    /// `WScriptMission_v04`: the record of the mission's index, the rider
    /// start hook's fake, then `WScriptProcess_v04`'s fake.
    #[test]
    fn wscript_mission_v04_matches() {
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0010F2B8),
            oc1576_fake_rider_flag,
            fakes::prev_cell(oc1576_fake_rider_flag),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00307C40),
            oc1576_fake_process_v04,
            fakes::prev_cell(oc1576_fake_process_v04),
        );
        check(
            ee::s10::f_003071f0__generated,
            wscript_mission_v04,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0030_71f0, 0);
                c.bus.write32(GATE, MAGIC);
                world(c, &mut r, (seed >> 2) & 0xf);
                c.bus.write32(OBJ, seed % 4);
                c.bus.write32(SELF + 0x10, OBJ);
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    /// `WScriptMission_v02`: the process-active fake (its result from the
    /// mailbox: zero and not zero), the two state callbacks and the audio hooks.
    #[test]
    fn wscript_mission_v02_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030B898),
            oc1576_fake_is_active,
            fakes::prev_cell(oc1576_fake_is_active),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            oc1576_fake_bx_audio,
            fakes::prev_cell(oc1576_fake_bx_audio),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_0029D8E0),
            oc1576_fake_big_challenge,
            fakes::prev_cell(oc1576_fake_big_challenge),
        );
        check(
            ee::s10::f_00307240__generated,
            wscript_mission_v02,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0030_7240, 4);
                c.bus.write32(GATE, MAGIC);
                c.bus.write32(GATE + 4, seed % 2);
                c.bus.write32(SELF + 0x10, MGR);
                c.bus.write32(SELF + 0x5c, STATE);
                c.bus.write16(STATE + 0x30, (r.next_u32() % 0x40) as u16);
                c.bus.write32(STATE + 0x34, fakes::FAKE);
                c.bus.write16(STATE + 0x90, (r.next_u32() % 0x40) as u16);
                c.bus.write32(STATE + 0x94, fakes::FAKE);
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    /// `WScriptMission_v17`: the state word (update or not), both callbacks
    /// through the miss hook.
    #[test]
    fn wscript_mission_v17_matches() {
        check(
            ee::s10::f_003072b0__generated,
            wscript_mission_v17,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0030_72b0, 4);
                let state = if seed % 2 == 0 { 2 } else { seed % 5 };
                c.bus.write32(SELF, state);
                c.bus.write32(SELF + 0x5c, STATE);
                c.bus.write16(STATE + 0x20, (r.next_u32() % 0x40) as u16);
                c.bus.write32(STATE + 0x24, fakes::FAKE);
                c.bus.write16(STATE + 0x30, (r.next_u32() % 0x40) as u16);
                c.bus.write32(STATE + 0x34, fakes::FAKE);
                c.g[A0][0] = sx(SELF);
            },
        );
    }
}
