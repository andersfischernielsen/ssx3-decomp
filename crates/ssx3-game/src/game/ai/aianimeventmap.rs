//! `src/ai/aianimeventmap.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_15e0, ai_anim_event_map_ctor, "AIAnimEventMap_ctor"),
    (
        0x0031_1870,
        ai_anim_event_map_create,
        "AIAnimEventMap_Create",
    ),
    (
        0x0031_18a8,
        ai_anim_event_map_destroy,
        "AIAnimEventMap_Destroy",
    ),
];

// ---------------------------------------------------------------------------
// Direct-call targets.

pub(crate) const EA_MEM_ALLOC_NAMED: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

pub(crate) const BUILTIN_DELETE: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::__builtin_delete,
};

pub(crate) const AI_ANIM_EVENT_MAP_CTOR: Callee = Callee {
    addr: 0x0031_15e0,
    f: ee::s10::f_003115e0,
};

// ---------------------------------------------------------------------------
// AI animation event map.

/// `void AIAnimEventMap_ctor(AIAnimEventMap *self)` (0x3115e0): copy the
/// 0x760-byte event table at 0x489d60 into a stack frame, clear the map's
/// 437 halfword entries at `self + 0x6d2` (stepping down), then rebuild the
/// halfword table from the copied words: entry `id` gets the running index in
/// its low half and the next count in its high half. The copied table is
/// 8-byte aligned, so only the `ld`/`sd` 32-byte copy path runs.
pub fn ai_anim_event_map_ctor(c: &mut Cpu) {
    const TABLE: u32 = 0x489d60; // 0x490000 - 0x62a0
    const LEN: u32 = 0x760;
    const COPY_HEAD: u32 = 0x0031_1660;
    const CLEAR_HEAD: u32 = 0x0031_1698;
    const BUILD_HEAD: u32 = 0x0031_16c8;
    c.cycles += 7;
    c.g[V0][0] = 0x49_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(LEN));
    let this = c.g[A0][0];
    c.g[T0][0] = this;
    c.g[V1][0] = sx(0x49_0000u32.wrapping_sub(0x62a0));
    c.g[V0][0] = c.g[V1][0] & 7;
    c.g[A0][0] = c.g[SP][0];
    // The table address is fixed and 8-byte aligned, so the `ldl/ldr` copy
    // path (0x311604) is never taken; the `ld`/`sd` path below always runs.
    c.cycles += 2;
    c.g[A1][0] = sx(TABLE.wrapping_add(LEN));
    c.g[V0][0] = sx((this as u32).wrapping_add(2));
    loop {
        c.cycles += 11;
        let src = c.g[V1][0] as u32;
        c.g[A2][0] = Addr(src).ld(c);
        c.g[A3][0] = Addr(src.wrapping_add(8)).ld(c);
        c.g[T1][0] = Addr(src.wrapping_add(0x10)).ld(c);
        c.g[T2][0] = Addr(src.wrapping_add(0x18)).ld(c);
        let dst = c.g[A0][0] as u32;
        Addr(dst).sd(c, c.g[A2][0]);
        Addr(dst.wrapping_add(8)).sd(c, c.g[A3][0]);
        Addr(dst.wrapping_add(0x10)).sd(c, c.g[T1][0]);
        Addr(dst.wrapping_add(0x18)).sd(c, c.g[T2][0]);
        c.g[V1][0] = sx(src.wrapping_add(0x20));
        let more = c.g[V1][0] != c.g[A1][0];
        c.g[A0][0] = sx(dst.wrapping_add(0x20));
        if !more {
            c.cycles += 1;
            c.g[A0][0] = 0x1b4;
            break;
        }
        if event_due(c) {
            deliver_event_here(c, COPY_HEAD);
        }
    }
    // Clear 437 halfwords ending at self + 0.
    c.cycles += 2;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6d0));
    loop {
        c.cycles += 7;
        Addr(c.g[V1][0] as u32).sh(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        let count_down = (c.g[A0][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(4));
        if count_down {
            if event_due(c) {
                deliver_event_here(c, CLEAR_HEAD);
            }
            continue;
        }
        c.cycles += 5;
        // Build the halfword table from the copied words.
        c.g[T1][0] = c.g[V0][0];
        c.g[A3][0] = 0x1b6;
        c.g[A2][0] = 0;
        c.g[A1][0] = c.g[SP][0];
        loop {
            c.cycles += 3;
            let at = c.g[A1][0] as u32;
            c.g[V0][0] = Addr(at).lw(c);
            let same = c.g[V0][0] == c.g[A3][0];
            c.g[A1][0] = sx(at.wrapping_add(4));
            if !same {
                c.cycles += 4;
                c.g[A3][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
                c.g[V0][0] = sx((this as u32).wrapping_add(c.g[V0][0] as u32));
                Addr(c.g[V0][0] as u32).sh(c, c.g[A2][0]);
            }
            c.cycles += 8;
            c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = ((c.g[A2][0] as i64) < 472i64) as u64;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            let more = c.g[A0][0] != 0;
            Addr(c.g[V0][0] as u32).sh(c, c.g[V1][0]);
            if !more {
                c.cycles += 2;
                let ret = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(LEN));
                c.pc = ret;
                return;
            }
            if event_due(c) {
                deliver_event_here(c, BUILD_HEAD);
            }
        }
    }
}

/// `AIAnimEventMap *AIAnimEventMap_Create(void)` (0x311870): allocate 0x6d4
/// bytes, store the map in the global at `gp + 0xd0c`, and construct it.
pub fn ai_anim_event_map_create(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x6d4;
        c.g[A1][0] = sx(0x49_0000u32.wrapping_sub(0x5b30));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        EA_MEM_ALLOC_NAMED.call(c, 0x0031_1890)?;
        c.g[A0][0] = c.g[V0][0];
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd0c), c.g[V0][0]);
        AI_ANIM_EVENT_MAP_CTOR.call(c, 0x0031_189c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AIAnimEventMap_Destroy(void)` (0x3118a8): delete the global map
/// (`gp + 0xd0c`) and clear the slot.
pub fn ai_anim_event_map_destroy(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xd0c)).lw(c);
        frame.save_ra(c, 0);
        BUILTIN_DELETE.call(c, 0x0031_18bc)?;
        frame.restore_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_add(0xd0c), 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;

    /// The allocator mutex (`gp`-relative globals); marked as held by "this
    /// thread" (fake `GetThreadId` result 0x42) so the real allocator entry
    /// points take their fast path.
    pub(crate) const MEM_MUTEX: u32 = 0x0053_8b00;

    pub(crate) fn alloc_setup(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(MEM_MUTEX + 4, 0x42);
        c.bus.write32(LOG_RESULTS, 0x42);
        c.bus.write32(MEM_MUTEX + 8, r.next_u32() % 4);
    }

    #[test]
    fn ai_anim_event_map_ctor_matches() {
        check(
            ee::s10::f_003115e0,
            ai_anim_event_map_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0031_15e0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                // The event table copied onto the stack: values are small ids so
                // the stores land in RAM; some equal 0x1b6 to hit the skip branch.
                for i in 0..472u32 {
                    let v = if (i + seed) % 7 == 0 {
                        0x1b6
                    } else {
                        (r.next_u32() % 0x1b6)
                    };
                    c.bus.write32(0x489d60 + i * 4, v);
                }
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn ai_anim_event_map_create_matches() {
        check(
            ee::s10::f_00311870,
            ai_anim_event_map_create,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1870, 0);
                alloc_setup(c, &mut r);
            },
        );
    }

    #[test]
    fn ai_anim_event_map_destroy_matches() {
        check(
            ee::s10::f_003118a8,
            ai_anim_event_map_destroy,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x18a8, 0);
                alloc_setup(c, &mut r);
                // Non-null so `__builtin_delete` really runs.
                c.bus.write32(
                    GAME_GP.wrapping_add(0xd0c),
                    if seed % 4 == 0 { 0 } else { DATA + 0x100 },
                );
            },
        );
    }
}
