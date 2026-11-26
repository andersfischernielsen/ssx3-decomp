//! Staging module oc742 (wave wave242): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc742`.

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
/// `WScriptFn_PlaceRiderAtObject`, `SetRailEnabled`, `PlayScriptList` and
/// `Nop_1440` of this list are already ported in oc732 and not repeated.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0030_05e8, wscript_fn_kill, "WScriptFn_Kill"),
    (
        0x0030_0770,
        wscript_fn_reset_rider_at_object,
        "WScriptFn_ResetRiderAtObject",
    ),
];

/// The 0x70-byte WScript frame and prologue: saves s0..s3/ra, moves the
/// three arguments into s1/s2/s3 (`regs`), loads the once-init guard at
/// `guard` (an address). Returns whether the guard was already set.
fn oc742_prologue(c: &mut Cpu, cycles: u64, guard: u32, regs: [usize; 3]) -> bool {
    c.cycles += cycles;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(guard);
    c.sq(17, sp(c).wrapping_add(0x50));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[regs[0]][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    c.g[S2][0] = c.g[regs[1]][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[regs[2]][0];
    let initialised = c.g[V0][0] != 0;
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    initialised
}

/// The one-time init of the shared 8-byte id block (`ids`, byte copy at
/// `ids + 4`, guard at `guard`): `ids = -1`, the guard set, the tag byte
/// copied to the stack.
fn oc742_init_block(c: &mut Cpu, ids: u32, guard: u32) {
    c.cycles += 7;
    c.g[V0][0] = c.lbu(ids.wrapping_add(4));
    c.g[V1][0] = u64::MAX;
    c.g[A0][0] = 0x1;
    c.sw(ids, c.g[V1][0]);
    c.sw(guard, c.g[A0][0]);
    c.sb(sp(c), c.g[V0][0]);
}

/// `MEM_copy(vp, ids, 8)` with `vp = sp + 0x10` (s0) after storing -1 at
/// `vp`, then the `(params, count)` fill loop: each argument is stored into
/// the block by its slot, as a float when the kind table says float and the
/// argument's kind differs, else as an int.
fn oc742_copy_fill(c: &mut Cpu, src: u32, ret_addr: u32, table: u32, top: u32) -> Flow {
    c.cycles += 10;
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 0x8;
    c.g[A1][0] = sx(src);
    c.g[A0][0] = c.g[S0][0];
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    call_ptr(c, 0x003e_6574, ret_addr)?;
    let (vp, count, params) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
    c.g[A2][0] = params;
    if (count as i64) <= 0 {
        return Ok(());
    }
    c.cycles += 4;
    c.g[A1][0] = count;
    c.g[T0][0] = sx(table);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(vp as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        let mut int_store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
                int_store = false;
            }
        }
        if int_store {
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, top);
        }
    }
    Ok(())
}

/// The zone-tag lookup from `vp` (the x word): the tag table at `gp+0x16c8`
/// maps x (low byte, then the next bits) to a small index in v0; with x = -1
/// v0 is `*(gp+0xce8)` (the caller then loads `->0x290`). Returns whether x
/// was not -1. `x_cycles` is the block charge of the x = -1 path.
fn oc742_zone_lookup(c: &mut Cpu, vp: u64, x_cycles: u64) -> bool {
    let gp = c.g[GP][0] as u32;
    c.cycles += 4;
    c.g[A2][0] = c.lw(vp as u32);
    c.g[V0][0] = !(0u64 | c.g[A2][0]);
    let tagged = c.g[V0][0] != 0;
    c.g[A1][0] = c.lw(gp.wrapping_add(0x16c8));
    if !tagged {
        c.cycles += x_cycles;
        c.g[V0][0] = c.lw(gp.wrapping_add(0xce8));
        return false;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    let found = c.g[V1][0] != 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if found {
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if nonzero {
            return true;
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
    true
}

/// The `luno_gLunoNil` struct return (12 bytes through `sret`).
fn oc742_return_nil(c: &mut Cpu, sret: u64) {
    c.g[V1][0] = 0x4d0000;
    c.g[V0][0] = sret;
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.sdl(4, (sret as u32).wrapping_add(0x7));
    c.sdr(4, sret as u32);
    c.sw((sret as u32).wrapping_add(0x8), c.g[A1][0]);
}

/// Restore s0..s3, ra and the 0x70 frame, return.
fn oc742_epilogue(c: &mut Cpu) {
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
}

/// `WScriptFn_Kill` (0x3005e8): fills an 8-byte block (x), looks up the
/// tagged object for x (x = -1: the current one, `*(gp+0xce8)->0x290`) and,
/// when the object has a script-object part (`+0xc`), calls its vtable method
/// at `+0x118/+0x11c` (the "kill"/lose-focus call). Returns `luno_gLunoNil`
/// through sret (a0).
pub fn wscript_fn_kill(c: &mut Cpu) {
    guest(c, kill_body)
}

fn kill_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc742_prologue(c, 11, gp.wrapping_add(0x2590), [A1, A2, A0]);
    if !initialised {
        oc742_init_block(c, gp.wrapping_add(0x2588), gp.wrapping_add(0x2590));
    }
    oc742_copy_fill(
        c,
        gp.wrapping_add(0x2588),
        0x0030_0650,
        gp.wrapping_add(0xb48),
        0x0030_0668,
    )?;

    let vp = c.g[S0][0];
    let tagged = oc742_zone_lookup(c, vp, 3);
    if !tagged {
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
    }
    c.cycles += 3;
    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    let no_part = c.g[A1][0] == 0;
    c.g[V1][0] = 0x4d0000;
    if !no_part {
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x118));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0030_072c)?;
        c.g[V1][0] = 0x4d0000;
    }
    c.cycles += 15;
    let sret = c.g[S3][0];
    oc742_return_nil(c, sret);
    oc742_epilogue(c);
    Ok(())
}

/// `WScriptFn_ResetRiderAtObject` (0x300770): the twin of
/// `PlaceRiderAtObject` with its own block (`gp+0x2598`), kind table
/// (`gp+0xb50`) and rider vtable slot (`+0x50/+0x54`): fills an 8-byte block
/// (x), looks up the tagged object for x, and when the rider index at
/// `*(gp+0xce8)` is not negative calls the object's vtable method and the
/// world's rider vtable method. Returns `luno_gLunoNil` through sret (a0).
pub fn wscript_fn_reset_rider_at_object(c: &mut Cpu) {
    guest(c, reset_rider_body)
}

fn reset_rider_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc742_prologue(c, 11, gp.wrapping_add(0x25a0), [A1, A2, A0]);
    if !initialised {
        oc742_init_block(c, gp.wrapping_add(0x2598), gp.wrapping_add(0x25a0));
    }
    oc742_copy_fill(
        c,
        gp.wrapping_add(0x2598),
        0x0030_07d8,
        gp.wrapping_add(0xb50),
        0x0030_07f0,
    )?;

    let vp = c.g[S0][0];
    let tagged = oc742_zone_lookup(c, vp, 4);
    if !tagged {
        // x == -1: a2 = v0 = *(gp+0xce8); v0 = a2->0x290.
        c.g[A2][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
    } else {
        c.cycles += 1;
        c.g[A2][0] = c.lw(gp.wrapping_add(0xce8));
    }

    c.cycles += 3;
    c.g[S0][0] = c.lw(c.g[A2][0] as u32);
    let negative = (c.g[S0][0] as i64) < 0;
    c.g[V1][0] = 0x4d0000;
    let mut skip_move = false;
    if !negative {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = c.g[S3][0];
            skip_move = true;
        } else {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let no_obj = c.g[A2][0] == 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            if !no_obj {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0030_08d0)?;
            }
            c.cycles += 14;
            c.g[A0][0] = c.lw(gp.wrapping_sub(0x848));
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, target, 0x0030_0904)?;
            c.g[V1][0] = 0x4d0000;
        }
    }
    if !skip_move {
        c.cycles += 1;
        c.g[V0][0] = c.g[S3][0];
    }
    c.cycles += 14;
    let sret = c.g[S3][0];
    oc742_return_nil(c, sret);
    oc742_epilogue(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fakes, DATA, GAME_GP};

    // Fixture objects, in DATA+0x200000 and up (clear of the fake log).
    const OC742_WORLD: u32 = DATA + 0x21_0000;
    const OC742_OBJ: u32 = DATA + 0x22_0000;
    const OC742_ARR: u32 = DATA + 0x23_0000;
    const OC742_RIDER: u32 = DATA + 0x24_0000;
    const OC742_VT2: u32 = DATA + 0x25_0000;
    const OC742_PARAMS: u32 = DATA + 0x26_0000;
    const OC742_SRET: u32 = DATA + 0x27_0000;
    const OC742_TAGS: u32 = DATA + 0x28_0000;
    const OC742_TAGS2: u32 = DATA + 0x28_1000;
    const OC742_TAGTBL: u32 = DATA + 0x29_0000;
    const OC742_ENTRY: u32 = DATA + 0x2a_0000;
    const OC742_ENTRY2: u32 = DATA + 0x2b_0000;
    const OC742_PREV: u32 = DATA + 0x2c_0000;
    const OC742_OBJ2: u32 = DATA + 0x2d_0000;
    const OC742_EOBJ: u32 = DATA + 0x2e_0000;
    const OC742_EVT: u32 = DATA + 0x2f_0000;

    /// The globals and objects the two bodies read, varied by `seed`: the
    /// world and its rider array, the zone-tag tables, the current-object
    /// record at `gp+0xce8` (rider index, tagged object and its vtable
    /// chain), the once-init guard at `gp+guard`, the kind table at `table`
    /// and four argument records (the first often x = -1).
    fn oc742_fixture(c: &mut Cpu, seed: u32, salt: u64, guard: u32, table: u32, rider_vt: u32) {
        let mut r = machine(c, seed, salt, 4);
        let gp = GAME_GP;
        c.bus.write32(gp.wrapping_sub(0x848), OC742_WORLD);
        c.bus.write32(OC742_WORLD + 0x84, OC742_OBJ);
        c.bus.write32(OC742_OBJ + 0xc, OC742_ARR);
        for i in 0..8u32 {
            c.bus.write32(OC742_ARR + 0x28 + 4 * i, OC742_RIDER);
        }
        c.bus.write32(OC742_RIDER + 0x6c0, OC742_VT2);
        c.bus.write16(
            OC742_VT2 + rider_vt,
            (r.next_u32() as i16 as i32 & !3) as u16,
        );
        c.bus.write32(OC742_VT2 + rider_vt + 4, fakes::FAKE + 0x10);
        // The tag tables.
        c.bus.write32(gp.wrapping_add(0x16c8), OC742_TAGS);
        c.bus.write32(OC742_TAGS, OC742_TAGS2);
        c.bus.write32(OC742_TAGS2 + 0x8, OC742_TAGTBL);
        for i in 0..256u32 {
            let e = if r.next_u32() % 4 == 0 {
                0
            } else {
                OC742_ENTRY
            };
            c.bus.write32(OC742_TAGTBL + 4 * i, e);
        }
        c.bus.write32(OC742_ENTRY + 0x1c, OC742_ENTRY2);
        for i in 0..0x100u32 {
            c.bus
                .write32(OC742_ENTRY2 + 4 * i, (r.next_u32() & 0xff) << 8);
        }
        // The current-object record: rider index, tagged object chain.
        c.bus.write32(gp.wrapping_add(0xce8), OC742_PREV);
        let idx = [u32::MAX, 0, 1, 2, 3, 5][(r.next_u32() % 6) as usize];
        c.bus.write32(OC742_PREV, idx);
        c.bus.write32(
            OC742_PREV + 0x290,
            if r.next_u32() % 4 == 0 { 0 } else { OC742_OBJ2 },
        );
        c.bus.write32(
            OC742_OBJ2 + 0xc,
            if r.next_u32() % 4 == 0 { 0 } else { OC742_EOBJ },
        );
        c.bus.write32(OC742_EOBJ + 0xc, OC742_EVT);
        // Vtable slots of the object's script part: {half offset, method}
        // at +0xc0 (Reset/Place) and +0x118 (Kill); methods are fakes.
        for (k, off) in [0xc0u32, 0x118].into_iter().enumerate() {
            c.bus
                .write16(OC742_EVT + off, (r.next_u32() as i16 as i32 & !3) as u16);
            c.bus
                .write32(OC742_EVT + off + 4, fakes::FAKE + 0x20 + 0x10 * k as u32);
        }
        c.bus.write32(gp.wrapping_add(guard), r.next_u32() & 1);
        for k in 0..4u32 {
            c.bus.write32(table + 4 * k, r.next_u32() % 3);
        }
        for i in 0..4u32 {
            let p = OC742_PARAMS + 16 * i;
            let first = i == 0;
            c.bus.write32(
                p,
                if first && r.next_u32() % 2 == 0 {
                    0
                } else {
                    r.next_u32() % 4
                },
            );
            let v = match r.next_u32() % 6 {
                0 | 1 if first => u32::MAX,
                0 => u32::MAX,
                1 => 0,
                2 => 1,
                3 => 2,
                4 => 3,
                _ => r.next_u32() & 0xffff,
            };
            c.bus.write32(p + 0x4, v);
            c.bus.write32(p + 0xc, r.next_u32() % 3);
        }
        let count = [0u64, 1, 2, 3, 4, u64::MAX][(seed % 6) as usize];
        c.g[A0][0] = sx(OC742_SRET);
        c.g[A1][0] = count;
        c.g[A2][0] = sx(OC742_PARAMS);
    }

    /// `WScriptFn_Kill`: block guard `gp+0x2590`, kill method at `+0x118`.
    #[test]
    fn wscript_fn_kill_matches() {
        check(
            ee::s10::f_003005e8__generated,
            wscript_fn_kill,
            400,
            &|c, seed| {
                oc742_fixture(
                    c,
                    seed,
                    0x0030_05e8,
                    0x2590,
                    GAME_GP.wrapping_add(0xb48),
                    0x60,
                );
                c.pc = 0x0030_05e8;
            },
        );
    }

    /// `WScriptFn_ResetRiderAtObject`: block guard `gp+0x25a0`, rider
    /// vtable slot at `+0x50`.
    #[test]
    fn wscript_fn_reset_rider_at_object_matches() {
        check(
            ee::s10::f_00300770__generated,
            wscript_fn_reset_rider_at_object,
            400,
            &|c, seed| {
                oc742_fixture(
                    c,
                    seed,
                    0x0030_0770,
                    0x25a0,
                    GAME_GP.wrapping_add(0xb50),
                    0x50,
                );
                c.pc = 0x0030_0770;
            },
        );
    }
}
