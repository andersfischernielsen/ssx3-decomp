//! Staging module oc455 (wave wave225): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc455`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0014_a5e0,
    be_library_deserialize_save,
    "BELibrary_DeserializeSave",
)];

/// The interface getters the load calls, and their generated shard functions.
const DESER_RACE_GET: Callee = Callee {
    addr: 0x0014_48d8,
    f: ee::s01::f_001448d8,
};
const DESER_PLAYER_GET: Callee = Callee {
    addr: 0x0014_5970,
    f: ee::s01::f_00145970,
};
const DESER_BAGT_GET: Callee = Callee {
    addr: 0x0014_f900,
    f: ee::s01::f_0014f900,
};
const DESER_MISSION_GET: Callee = Callee {
    addr: 0x0015_32a0,
    f: ee::s01::f_001532a0,
};
const DESER_SCORE_GET: Callee = Callee {
    addr: 0x0015_4948,
    f: ee::s01::f_00154948,
};
const DESER_CHAR_GET: Callee = Callee {
    addr: 0x0014_7f78,
    f: ee::s01::f_00147f78,
};

/// One block-copy loop of the generated body. Each step moves 0x20 bytes
/// (four doublewords) from `src` to `dst` through the `temps` registers; the
/// loop runs while `src != end`, with an event check at its back edge (`head`).
/// `per_step` is the cycle charge of one step; `unaligned` picks `ldl`/`ldr`
/// (the source or destination is not 8-aligned) over `ld`/`sd`.
#[derive(Clone, Copy)]
struct DeserCopy {
    head: u32,
    src: usize,
    dst: usize,
    end: usize,
    temps: [usize; 4],
    per_step: u64,
    unaligned: bool,
}

/// The 0x40-byte head of the race block (`cBELibrary_raceDB`), `V1` -> `A2`.
const DESER_RACE_ALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a69c,
    src: V1,
    dst: A2,
    end: V0,
    temps: [T1, T2, A0, A1],
    per_step: 11,
    unaligned: false,
};
const DESER_RACE_UNALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a630,
    src: V1,
    dst: A2,
    end: V0,
    temps: [A0, A1, A3, T0],
    per_step: 19,
    unaligned: true,
};
/// The 0xa0-byte body of the player block (`cBELibrary_playerDB`).
const DESER_PLAYER_ALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a77c,
    src: V1,
    dst: A2,
    end: V0,
    temps: [T1, T2, A0, A1],
    per_step: 11,
    unaligned: false,
};
const DESER_PLAYER_UNALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a720,
    src: V1,
    dst: A2,
    end: V0,
    temps: [A0, A1, A3, T0],
    per_step: 19,
    unaligned: true,
};
/// The 0xf80-byte body of the first profile block, `A2` -> `V1`.
const DESER_PROFILE0_ALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a860,
    src: A2,
    dst: V1,
    end: V0,
    temps: [T2, A0, A1, T0],
    per_step: 11,
    unaligned: false,
};
const DESER_PROFILE0_UNALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a804,
    src: A2,
    dst: V1,
    end: V0,
    temps: [A0, A1, T0, T1],
    per_step: 19,
    unaligned: true,
};
/// The 0xf80-byte body of the second profile block (`D_004B07F8`).
const DESER_PROFILE1_ALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a934,
    src: A2,
    dst: V1,
    end: V0,
    temps: [T0, T1, T2, A0],
    per_step: 11,
    unaligned: false,
};
const DESER_PROFILE1_UNALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a8d8,
    src: A2,
    dst: V1,
    end: V0,
    temps: [T1, T2, A0, A1],
    per_step: 19,
    unaligned: true,
};
/// The 0x9b40-byte body of the rules block (`D_004BA348`), `V1` -> `A1`.
const DESER_RULES_ALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a9e8,
    src: V1,
    dst: A1,
    end: V0,
    temps: [T2, A0, A2, T0],
    per_step: 11,
    unaligned: false,
};
const DESER_RULES_UNALIGNED: DeserCopy = DeserCopy {
    head: 0x0014_a98c,
    src: V1,
    dst: A1,
    end: V0,
    temps: [A0, A2, T0, T1],
    per_step: 19,
    unaligned: true,
};

/// One 0x20-byte step with unaligned accesses: four `ldl`/`ldr` pairs into
/// `t`, then four `sdl`/`sdr` pairs, in the generated order.
fn deser_step_unaligned(c: &mut Cpu, from: u32, to: u32, t: [usize; 4]) {
    for (i, &r) in t.iter().enumerate() {
        let o = 8 * i as u32;
        c.ldl(r, from.wrapping_add(o + 7));
        c.ldr(r, from.wrapping_add(o));
    }
    for (i, &r) in t.iter().enumerate() {
        let o = 8 * i as u32;
        c.sdl(r, to.wrapping_add(o + 7));
        c.sdr(r, to.wrapping_add(o));
    }
}

/// One 0x20-byte step with aligned doublewords: four `ld` into `t`, then four
/// `sd`.
fn deser_step_aligned(c: &mut Cpu, from: u32, to: u32, t: [usize; 4]) {
    for (i, &r) in t.iter().enumerate() {
        c.g[r][0] = c.ld(from.wrapping_add(8 * i as u32));
    }
    for (i, &r) in t.iter().enumerate() {
        let v = c.g[r][0];
        c.sd(to.wrapping_add(8 * i as u32), v);
    }
}

/// Runs one block-copy loop to its exit (the caller charges the exit cycles).
fn deser_run_copy(c: &mut Cpu, k: DeserCopy) {
    loop {
        c.cycles += k.per_step;
        let from = c.g[k.src][0] as u32;
        let to = c.g[k.dst][0] as u32;
        if k.unaligned {
            deser_step_unaligned(c, from, to, k.temps);
        } else {
            deser_step_aligned(c, from, to, k.temps);
        }
        c.g[k.src][0] = sx(from.wrapping_add(0x20));
        let more = c.g[k.src][0] != c.g[k.end][0];
        c.g[k.dst][0] = sx(to.wrapping_add(0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, k.head);
        }
    }
}

/// `obj->onLoad()` with the interface object in `v0` (the race and player
/// interfaces): `v1 = vtable`, `a0 = this delta`, `a1 = method`, then the call
/// with `a0 = obj + delta`.
fn deser_on_load_v0(c: &mut Cpu, ret_addr: u32) -> Flow {
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let target = c.g[A1][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_addr)
}

/// The same call with the interface object copied to `a1` first (the BAGT,
/// mission, score and character interfaces): `v0 = vtable`, `v1 = method`.
fn deser_on_load_a1(c: &mut Cpu, ret_addr: u32) -> Flow {
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_addr)
}

/// `BELibrary_DeserializeSave` (0x14a5e0), the body of `cBELibrary_readFromData`:
/// loads the save's game data at `data` (`a0`). Layout: the race block (0x4c
/// bytes) into `cBELibrary_raceDB`, then the race interface's `onLoad`; the
/// player block (0xa8) into `cBELibrary_playerDB`, then the player interface's
/// `onLoad`; one 0xf88-byte profile block per player into
/// `cBELibrary_profileLib[cCharacterID]` (the second one only when its id is
/// not negative, into `D_004B07F8`); the rules block (0x9b50 bytes, then 0x20
/// bytes of rule options) into `D_004BA348` and `cBERules_mRules`; then
/// `onLoad` on the BAGT, mission, score and character interfaces. Every block
/// copy is an aligned `ld`/`sd` loop or an unaligned `ldl`/`ldr` one, chosen
/// by `(src | dst) & 7`, as the generated body does.
pub fn be_library_deserialize_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[V0][0] = 0x530000;
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b0));
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = c.g[A1][0] | c.g[A2][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        c.g[V0][0] &= 0x7;
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        let aligned = c.g[V0][0] == 0;
        frame.save_ra(c, 0);

        // Race block: 0x40 bytes by the loop, then 12 more.
        c.cycles += 6;
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = 0x530000;
        c.g[S2][0] = 0x4a0000;
        c.g[S3][0] = 0x4c0000;
        c.g[S4][0] = 0x530000;
        if aligned {
            deser_run_copy(c, DESER_RACE_ALIGNED);
        } else {
            deser_run_copy(c, DESER_RACE_UNALIGNED);
            c.cycles += 2;
        }
        c.cycles += 20;
        let (src, dst) = (c.g[V1][0] as u32, c.g[A2][0] as u32);
        c.ldl(7, src.wrapping_add(0x7));
        c.ldr(7, src);
        c.lwl(8, src.wrapping_add(0xb));
        c.lwr(8, src.wrapping_add(0x8));
        c.sdl(7, dst.wrapping_add(0x7));
        c.sdr(7, dst);
        c.swl(8, dst.wrapping_add(0xb));
        c.swr(8, dst.wrapping_add(0x8));
        DESER_RACE_GET.call(c, 0x0014_a6ec)?;
        deser_on_load_v0(c, 0x0014_a700)?;

        // Player block: 0xa8 bytes at data + 0x4c; 0xa0 by the loop, 8 more.
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5b20));
        c.g[V0][0] = c.g[A1][0] | c.g[A2][0];
        c.g[V0][0] &= 0x7;
        let aligned = c.g[V0][0] == 0;
        c.g[V1][0] = c.g[A1][0];
        c.cycles += 2;
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa8));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa0));
        if aligned {
            deser_run_copy(c, DESER_PLAYER_ALIGNED);
        } else {
            deser_run_copy(c, DESER_PLAYER_UNALIGNED);
            c.cycles += 2;
        }
        c.cycles += 21;
        let (src, dst) = (c.g[V1][0] as u32, c.g[A2][0] as u32);
        c.ldl(7, src.wrapping_add(0x7));
        c.ldr(7, src);
        c.sdl(7, dst.wrapping_add(0x7));
        c.sdr(7, dst);
        DESER_PLAYER_GET.call(c, 0x0014_a7bc)?;
        deser_on_load_v0(c, 0x0014_a7d0)?;

        // First profile block: the slot of playerDB[0].cCharacterID.
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5b20));
        c.g[V1][0] = 0xf88;
        c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x11));
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6ca8));
        c.g[A1][0] = c.g[S0][0];
        c.mult(0, 3, 2, 3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = c.g[A1][0] | c.g[V1][0];
        c.g[V0][0] &= 0x7;
        let aligned = c.g[V0][0] == 0;
        c.g[A2][0] = c.g[A1][0];
        c.cycles += 2;
        c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf88));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf80));
        if aligned {
            deser_run_copy(c, DESER_PROFILE0_ALIGNED);
            c.cycles += 1;
        } else {
            deser_run_copy(c, DESER_PROFILE0_UNALIGNED);
            c.cycles += 2;
        }
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5b20));

        // The first profile block's last 8 bytes, then the second profile
        // block when playerDB[1].cCharacterID is not negative.
        c.cycles += 7;
        c.g[A1][0] = c.g[A3][0];
        c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x2d));
        let (src, dst) = (c.g[A2][0] as u32, c.g[V1][0] as u32);
        c.ldl(9, src.wrapping_add(0x7));
        c.ldr(9, src);
        c.sdl(9, dst.wrapping_add(0x7));
        let no_second = (c.g[V0][0] as i64) < 0;
        c.sdr(9, dst);
        if !no_second {
            c.cycles += 9;
            c.g[A0][0] = 0xf88;
            c.g[V1][0] = 0x4b0000;
            c.mult(0, 4, 2, 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7f8));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = c.g[A1][0] | c.g[V1][0];
            c.g[V0][0] &= 0x7;
            let aligned = c.g[V0][0] == 0;
            c.g[A2][0] = c.g[A1][0];
            c.cycles += 2;
            c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf88));
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf80));
            if aligned {
                deser_run_copy(c, DESER_PROFILE1_ALIGNED);
            } else {
                deser_run_copy(c, DESER_PROFILE1_UNALIGNED);
                c.cycles += 2;
            }
            c.cycles += 4;
            let (src, dst) = (c.g[A2][0] as u32, c.g[V1][0] as u32);
            c.ldl(5, src.wrapping_add(0x7));
            c.ldr(5, src);
            c.sdl(5, dst.wrapping_add(0x7));
            c.sdr(5, dst);
        }

        // Rules block: 0x9b40 bytes by the loop, 16 more, then 32 bytes of
        // rule options from the end of the block.
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x5cb8));
        c.g[V0][0] = c.g[A3][0] | c.g[A1][0];
        c.g[V0][0] &= 0x7;
        let aligned = c.g[V0][0] == 0;
        c.g[V1][0] = c.g[A3][0];
        c.cycles += 2;
        c.g[V0][0] = 0x9b40;
        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if aligned {
            deser_run_copy(c, DESER_RULES_ALIGNED);
        } else {
            deser_run_copy(c, DESER_RULES_UNALIGNED);
            c.cycles += 2;
        }
        c.cycles += 67;
        let (src, dst) = (c.g[V1][0] as u32, c.g[A1][0] as u32);
        c.ldl(9, src.wrapping_add(0x7));
        c.ldr(9, src);
        c.ldl(10, src.wrapping_add(0xf));
        c.ldr(10, src.wrapping_add(0x8));
        c.sdl(9, dst.wrapping_add(0x7));
        c.sdr(9, dst);
        c.sdl(10, dst.wrapping_add(0xf));
        c.g[V0][0] = 0x9b50;
        c.sdr(10, dst.wrapping_add(0x8));
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x8b8));
        let (src, dst) = (c.g[A1][0] as u32, c.g[T2][0] as u32);
        c.ldl(7, src.wrapping_add(0x7));
        c.ldr(7, src);
        c.ldl(8, src.wrapping_add(0xf));
        c.ldr(8, src.wrapping_add(0x8));
        c.ldl(9, src.wrapping_add(0x17));
        c.ldr(9, src.wrapping_add(0x10));
        c.sdl(7, dst.wrapping_add(0x7));
        c.sdr(7, dst);
        c.sdl(8, dst.wrapping_add(0xf));
        c.sdr(8, dst.wrapping_add(0x8));
        c.sdl(9, dst.wrapping_add(0x17));
        c.sdr(9, dst.wrapping_add(0x10));
        c.ldl(7, src.wrapping_add(0x1f));
        c.ldr(7, src.wrapping_add(0x18));
        c.sdl(7, dst.wrapping_add(0x1f));
        c.sdr(7, dst.wrapping_add(0x18));

        // The interfaces' onLoad, in the generated order.
        DESER_BAGT_GET.call(c, 0x0014_aa88)?;
        deser_on_load_a1(c, 0x0014_aaa0)?;
        DESER_MISSION_GET.call(c, 0x0014_aaa8)?;
        deser_on_load_a1(c, 0x0014_aac0)?;
        DESER_SCORE_GET.call(c, 0x0014_aac8)?;
        deser_on_load_a1(c, 0x0014_aae0)?;
        DESER_CHAR_GET.call(c, 0x0014_aae8)?;
        deser_on_load_a1(c, 0x0014_ab00)?;

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
    use crate::difftest::check_lohi;
    use crate::difftest::fakes;
    use crate::difftest::fill;
    use crate::difftest::fakes::machine;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word for the getter fakes, and the interface objects they return
    /// (one slot per getter at `+0x10 + 4k`). Above the test's data.
    pub(crate) const DESER_TEST_CTRL: u32 = DATA + 0x30_0000;
    pub(crate) const DESER_TEST_MAGIC: u32 = 0x4f43_4534;

    /// A getter fake: while the gate word is set it logs the call and returns
    /// the interface object of slot `k`; otherwise it runs the body it replaced.
    fn deser_getter(
        c: &mut Cpu,
        target: u32,
        k: u32,
        prev: &AtomicUsize,
        generated: fn(&mut Cpu),
    ) {
        if c.bus.read32(DESER_TEST_CTRL) != DESER_TEST_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        let ret_addr = c.g[RA][0] as u32;
        fakes::fake(c, target, ret_addr);
        c.g[V0][0] = c.bus.read32(DESER_TEST_CTRL + 0x10 + 4 * k) as u64;
    }

    static DESER_PREV_RACE: AtomicUsize = AtomicUsize::new(0);
    static DESER_PREV_PLAYER: AtomicUsize = AtomicUsize::new(0);
    static DESER_PREV_BAGT: AtomicUsize = AtomicUsize::new(0);
    static DESER_PREV_MISSION: AtomicUsize = AtomicUsize::new(0);
    static DESER_PREV_SCORE: AtomicUsize = AtomicUsize::new(0);
    static DESER_PREV_CHAR: AtomicUsize = AtomicUsize::new(0);

    fn deser_fake_race(c: &mut Cpu) {
        deser_getter(c, 0x0014_48d8, 0, &DESER_PREV_RACE, ee::s01::f_001448d8__generated)
    }
    fn deser_fake_player(c: &mut Cpu) {
        deser_getter(c, 0x0014_5970, 1, &DESER_PREV_PLAYER, ee::s01::f_00145970__generated)
    }
    fn deser_fake_bagt(c: &mut Cpu) {
        deser_getter(c, 0x0014_f900, 2, &DESER_PREV_BAGT, ee::s01::f_0014f900__generated)
    }
    fn deser_fake_mission(c: &mut Cpu) {
        deser_getter(c, 0x0015_32a0, 3, &DESER_PREV_MISSION, ee::s01::f_001532a0__generated)
    }
    fn deser_fake_score(c: &mut Cpu) {
        deser_getter(c, 0x0015_4948, 4, &DESER_PREV_SCORE, ee::s01::f_00154948__generated)
    }
    fn deser_fake_char(c: &mut Cpu) {
        deser_getter(c, 0x0014_7f78, 5, &DESER_PREV_CHAR, ee::s01::f_00147f78__generated)
    }

    fn deser_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001448D8),
            deser_fake_race,
            &DESER_PREV_RACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00145970),
            deser_fake_player,
            &DESER_PREV_PLAYER,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014F900),
            deser_fake_bagt,
            &DESER_PREV_BAGT,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001532A0),
            deser_fake_mission,
            &DESER_PREV_MISSION,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00154948),
            deser_fake_score,
            &DESER_PREV_SCORE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147F78),
            deser_fake_char,
            &DESER_PREV_CHAR,
        );
    }

    /// A save image of 0xbc00 bytes at `DATA + 0x1000` (+4 on odd seeds, so
    /// the copies alternate between aligned and unaligned), with the two
    /// character ids the load reads (playerDB[0] in 0..3, playerDB[1] in
    /// -1..3, the negative one skipping the second profile block), and six
    /// interface objects whose vtables point at fake methods.
    fn deser_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let buf = DATA + 0x1000 + if seed & 1 == 1 { 4 } else { 0 };
        fill(c, r, buf, 0x2f00);
        c.bus.write8(buf + 0x4c + 0x11, (seed % 4) as u8);
        c.bus.write8(buf + 0x4c + 0x2d, (((seed / 4) % 5) as i32 - 1) as u8);
        c.bus.write32(DESER_TEST_CTRL, DESER_TEST_MAGIC);
        for k in 0..6u32 {
            let obj = DATA + 0x30_1000 + 0x100 * k;
            let vt = DATA + 0x30_8000 + 0x40 * k;
            c.bus.write32(obj + 0xc, vt);
            c.bus.write16(vt + 0x10, (r.next_u32() as i16 & !3) as u16);
            c.bus.write32(vt + 0x14, fakes::FAKE + 0x10 * k);
            c.bus.write32(DESER_TEST_CTRL + 0x10 + 4 * k, obj);
        }
        c.g[A0][0] = sx(buf);
    }

    /// `BELibrary_DeserializeSave`: aligned and unaligned copies of every
    /// block, both second-profile cases, LO/HI too; the getters and the
    /// vtable methods are fake, the copies and the register dance are real.
    #[test]
    fn be_library_deserialize_save_matches() {
        deser_install_fakes();
        check_lohi(
            ee::s01::f_0014a5e0__generated,
            be_library_deserialize_save,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_a5e0, 12);
                deser_fixture(c, &mut r, seed);
                c.pc = 0x0014_a5e0;
            },
        );
    }
}
