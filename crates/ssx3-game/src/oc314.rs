//! Staging wave 213 (oc314): hand-written replacements for functions with no
//! C++ in the reference, ported with Ghidra's decompilation as the readable
//! aid. Each reproduces the generated body's registers, cycles, memory and
//! call order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0036_1038, object_v01, "Object_v01"),
    (0x0027_0ce8, replay_end_highlight, "Replay_EndHighlight"),
];

// ---------------------------------------------------------------------------
// Object_v01 (0x361038)

/// The vtable `Object_v01` installs at `+0xc` before chaining (`D_00490e80`).
pub(crate) const OBJECT_V01_VTABLE_ADDR: u32 = 0x0049_0e80;

/// The object's vtable pointer.
pub(crate) const OBJECT_V01_VTABLE: u32 = 0xc;

const OBJECT_V01_MOD_OBJ_DTOR: Callee = Callee {
    addr: 0x0035_53c0,
    f: ee::s11::ModObj_dtor,
};

/// `Object* Object_v01(Object* self, int flags)` (0x361038, the
/// `railmodifier.cpp` object's virtual destructor): reinstall its own vtable
/// (0x490e80) and chain to `ModObj_dtor(self, flags)` (flags passed
/// through untouched in a1). Leaves v1 = self.
pub fn object_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(OBJECT_V01_VTABLE_ADDR);
        c.sw(r32(c, V1).wrapping_add(OBJECT_V01_VTABLE), c.g[V0][0]);
        OBJECT_V01_MOD_OBJ_DTOR.call(c, 0x0036_1054)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay_EndHighlight (0x270ce8)

/// Size of one highlight slot in the `Replay` and the offset of the slot
/// array.
pub(crate) const REPLAY_END_HIGHLIGHT_SLOT_SIZE: u32 = 0xb4;

pub(crate) const REPLAY_END_HIGHLIGHT_SLOTS: u32 = 0x494;

/// Offset of a slot's snapshot node pointer from the slot array start
/// (`slot + 0xb0`, seen from the replay as `+0x544`).
pub(crate) const REPLAY_END_HIGHLIGHT_SLOT_NODE: u32 = 0x544;

/// Snapshot node flag byte: one "highlighted in slot n" bit per slot.
pub(crate) const REPLAY_END_HIGHLIGHT_NODE_FLAGS: u32 = 0x1e;

const REPLAY_END_HIGHLIGHT_SCORE: Callee = Callee {
    addr: 0x0026_e838,
    f: ee::s07::Replay_HighlightSlotScore,
};

const REPLAY_END_HIGHLIGHT_CLEAR: Callee = Callee {
    addr: 0x0026_e800,
    f: ee::s07::Replay_HighlightSlotClear,
};

const REPLAY_END_HIGHLIGHT_COUNT: Callee = Callee {
    addr: 0x0027_0bd8,
    f: ee::s07::Replay_CountHighlights,
};

const REPLAY_END_HIGHLIGHT_GET_MAX: Callee = Callee {
    addr: 0x0027_0c40,
    f: ee::s07::Replay_GetMaxHighlights,
};

const REPLAY_END_HIGHLIGHT_FIND_WEAKEST: Callee = Callee {
    addr: 0x0027_0c58,
    f: ee::s07::Replay_FindWeakestHighlight,
};

const REPLAY_END_HIGHLIGHT_DETACH: Callee = Callee {
    addr: 0x0026_e7f8,
    f: ee::s07::Replay_HighlightSlotDetach,
};

/// `void Replay_EndHighlight(Replay* replay, int slot, Player* player, int
/// mode)` (0x270ce8): close the highlight being recorded in `slot`, if the
/// slot has a snapshot node. Score it (`Replay_HighlightSlotScore`); a
/// worthless one is dropped (`Replay_HighlightSlotClear`). A kept one that
/// brings the slot's highlight count to the maximum evicts the weakest
/// highlight (`Replay_FindWeakestHighlight`): that node loses the slot's bit
/// in its flag byte. Finally the slot lets go of its node
/// (`Replay_HighlightSlotDetach`).
pub fn replay_end_highlight(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A0][0]; // replay
        c.g[S3][0] = c.g[A1][0]; // slot
        c.g[A0][0] = REPLAY_END_HIGHLIGHT_SLOT_SIZE as u64;
        c.mult(0, V0, S3, A0);
        frame.save(c, S2, 0x30);
        frame.save(c, S0, 0x50);
        c.g[A1][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(REPLAY_END_HIGHLIGHT_SLOT_NODE));
        c.g[S2][0] = c.g[A3][0]; // mode
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            c.mult(0, S4, S3, A0); // slot offset
            c.g[A2][0] = c.g[S2][0];
            c.g[V0][0] = sx(r32(c, S4).wrapping_add(REPLAY_END_HIGHLIGHT_SLOTS));
            c.g[S0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0))); // the slot
            c.g[A0][0] = c.g[S0][0];
            REPLAY_END_HIGHLIGHT_SCORE.call(c, 0x0027_0d40)?;
            c.g[A1][0] = c.g[S3][0];
            if c.g[V0][0] == 0 {
                // Not worth keeping.
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                REPLAY_END_HIGHLIGHT_CLEAR.call(c, 0x0027_0dac)?;
                c.cycles += 1;
                c.g[A0][0] = REPLAY_END_HIGHLIGHT_SLOT_SIZE as u64;
            } else {
                c.cycles += 10;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                REPLAY_END_HIGHLIGHT_COUNT.call(c, 0x0027_0d54)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                REPLAY_END_HIGHLIGHT_GET_MAX.call(c, 0x0027_0d64)?;
                c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[S0][0] != 0 {
                    c.g[A0][0] = REPLAY_END_HIGHLIGHT_SLOT_SIZE as u64;
                } else {
                    // Full: evict the weakest highlight from this slot.
                    c.cycles += 13;
                    c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, S1)));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(REPLAY_END_HIGHLIGHT_SLOT_NODE));
                    c.g[A3][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S3][0];
                    REPLAY_END_HIGHLIGHT_FIND_WEAKEST.call(c, 0x0027_0d88)?;
                    let node = r32(c, V0).wrapping_add(REPLAY_END_HIGHLIGHT_NODE_FLAGS);
                    c.g[A0][0] = c.lbu(node);
                    c.g[V1][0] = sx(1u32 << (r32(c, S3) & 31));
                    c.g[V1][0] = !c.g[V1][0];
                    c.g[A0][0] &= c.g[V1][0];
                    c.sb(node, c.g[A0][0]);
                    c.cycles += 1;
                    c.g[A0][0] = REPLAY_END_HIGHLIGHT_SLOT_SIZE as u64;
                }
            }
            c.cycles += 4;
            c.mult(0, A0, S3, A0);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(REPLAY_END_HIGHLIGHT_SLOTS));
            c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
            REPLAY_END_HIGHLIGHT_DETACH.call(c, 0x0027_0dc0)?;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `Object_v01`: the vtable store, then the real `ModObj_dtor` chain on
    /// the fixture of `mod_obj_dtor_matches` (no modifier block, or an empty
    /// one from a dlmalloc arena; WorldObj_dtor without the delete bit). The
    /// mutex syscalls are fakes; some seeds unwind at one of the calls.
    #[test]
    fn object_v01_matches() {
        use crate::difftest::fakes::{machine, LOG_COUNT, LOG_UNWIND_AT};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        use crate::game::bx::cubicspline::tests::oc238_run;
        use crate::game::object::movenode::MOD_OBJ_BLOCK;
        check(ee::s12::f_00361038__generated, object_v01, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x36_1038, 0);
            let obj = DATA + 0x1000;
            let inst = DATA + 0x1100;
            fill(c, &mut r, obj, 8);
            c.bus.write16(obj + 0x10, (r.next_u32() % 8) as u16);
            if seed % 3 == 0 {
                c.bus.write32(obj + 4, 0);
            } else {
                c.bus.write32(obj + 4, DATA + 0x1200);
                c.bus
                    .write32(obj + 8, if seed % 3 == 1 { 0 } else { DATA + 0x1300 });
            }
            c.bus
                .write32(obj + 0x18, if seed % 7 == 0 { 0 } else { inst });
            c.bus.write32(inst + 0x8, r.next_u32());
            c.bus
                .write32(inst + 0xc, if seed % 2 == 0 { obj } else { r.next_u32() });
            let block = if seed % 4 == 0 {
                0
            } else {
                // The game heap over test memory, pools off.
                c.bus.write32(GAME_GP + 0xdd8, 0);
                oc238_run(
                    c,
                    0x0031_ff60,
                    ee::s10::f_0031ff60,
                    &[0x004f_f250, (DATA + 0x40000) as u64, 0x40000],
                );
                let b = oc238_run(c, 0x0031_9e48, ee::s10::f_00319e48, &[0x40, 0]) as u32;
                for i in 0..0x10 {
                    c.bus.write32(b + 4 * i, 0);
                }
                b
            };
            c.bus.write32(obj + MOD_OBJ_BLOCK, block);
            // Unwind at one of the next calls (the setup made some).
            if seed % 3 == 1 {
                let n = c.bus.read32(LOG_COUNT);
                c.bus.write32(LOG_UNWIND_AT, n + r.next_u32() % 6);
            }
            c.pc = 0x0036_1038;
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (r.next_u32() & !1) as u64;
        });
    }

    /// `Replay_EndHighlight` with the real callees (all short leaves): a
    /// replay with a snapshot list of 0..4 nodes (random flag bytes and
    /// per-slot values), slot 0..5 whose node pointer is null (nothing to
    /// end), a list node or a lone node; score inputs that give 0 (clear) or
    /// 1 (kept), maximum counts that make the count below or at the limit
    /// (evict the weakest), both modes. LO/HI are compared (`mult`).
    #[test]
    fn replay_end_highlight_matches() {
        use crate::difftest::{check_lohi, fill, scramble, Rng, DATA};
        check_lohi(
            ee::s07::f_00270ce8__generated,
            replay_end_highlight,
            300,
            &|c, seed| {
                let mut r = Rng(0x0270_ce80 ^ (seed as u64) << 4);
                scramble(c, &mut r);
                let replay = DATA + 0x1000;
                let player = DATA + 0x3000;
                let nodes = DATA + 0x4000;
                let lone = DATA + 0x5000;
                fill(c, &mut r, replay, 0x300);
                fill(c, &mut r, player, 0x40);
                let slot_n = seed % 6;
                let slot =
                    replay + REPLAY_END_HIGHLIGHT_SLOTS + slot_n * REPLAY_END_HIGHLIGHT_SLOT_SIZE;
                // Snapshot list: node+0x14 next, +0x1e slot bits, +0x1f skip,
                // +0x20 + 4 * slot value.
                let count = r.next_u32() % 5;
                let init_node = |c: &mut Cpu, r: &mut Rng, n: u32| {
                    fill(c, r, n, 0x20);
                    c.bus
                        .write8(n + REPLAY_END_HIGHLIGHT_NODE_FLAGS, r.next_u32() as u8);
                    c.bus.write8(n + 0x1f, (r.next_u32() % 4 == 0) as u8);
                    for i in 0..8 {
                        c.bus
                            .write32(n + 0x20 + 4 * i, (r.next_u32() % 7).wrapping_sub(3));
                    }
                };
                for i in 0..count {
                    let n = nodes + i * 0x80;
                    init_node(c, &mut r, n);
                    c.bus
                        .write32(n + 0x14, if i + 1 < count { n + 0x80 } else { 0 });
                }
                init_node(c, &mut r, lone);
                c.bus
                    .write32(replay + 0x3b0, if count == 0 { 0 } else { nodes });
                let node = match seed % 5 {
                    0 => 0,
                    1 => lone,
                    _ if count > 0 => nodes + (r.next_u32() % count) * 0x80,
                    _ => lone,
                };
                c.bus.write32(slot + 0xb0, node);
                c.bus.write32(slot, r.next_u32() % 8); // slot index
                c.bus.write32(replay + 0xc, r.next_u32() % 4); // max highlights
                                                               // Score inputs: zero/near/far progress, distance gate at 5.0.
                let base = r.next_u32() % 5000;
                c.bus.write32(slot + 0xa0, base);
                c.bus.write32(
                    player + 0x9c,
                    base + [0, 400, 1500][(r.next_u32() % 3) as usize],
                );
                let t = r.next_u32() % 100;
                c.bus.write32(slot + 0xa8, t);
                c.bus.write32(
                    player + 0xa4,
                    if r.next_u32() % 3 == 0 {
                        t
                    } else {
                        t + 1 + r.next_u32() % 50
                    },
                );
                c.bus.write32(
                    player + 0x44,
                    (if r.next_u32() % 2 == 0 {
                        4.0f32
                    } else {
                        6.0f32
                    })
                    .to_bits(),
                );
                let mode = match (seed / 6) % 3 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32() as u64 | 1,
                };
                c.pc = 0x0027_0ce8;
                c.g[A0][0] = sx(replay);
                c.g[A1][0] = slot_n as u64;
                c.g[A2][0] = sx(player);
                c.g[A3][0] = mode;
            },
        );
    }
}
