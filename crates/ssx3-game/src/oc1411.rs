//! Staging module oc1411 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1411`.

use crate::call::{guest, Callee, Frame};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Callees of this module's ports, with main's names.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `DLHeap_Consolidate` (`0x0031eee8`).
    pub(crate) const DLHEAP_CONSOLIDATE: Callee = Callee {
        addr: 0x0031_eee8,
        f: ee::s10::DLHeap_Consolidate,
    };
}

/// The `switch (param + 4)` jump table of `DLHeap_Mallopt` (6 words, in the
/// game image).
pub(crate) const MALLOPT_TABLE: u32 = 0x0048_db50;

/// The case labels the table holds, in table order (`param` = -4 .. 1).
pub(crate) const MALLOPT_MMAP_MAX: u32 = 0x0031_fef0;
pub(crate) const MALLOPT_MMAP_THRESHOLD: u32 = 0x0031_fee4;
pub(crate) const MALLOPT_TOP_PAD: u32 = 0x0031_fed8;
pub(crate) const MALLOPT_TRIM_THRESHOLD: u32 = 0x0031_fecc;
pub(crate) const MALLOPT_NONE: u32 = 0x0031_ff04;
pub(crate) const MALLOPT_MXFAST: u32 = 0x0031_fe80;

/// `malloc_state` fields `DLHeap_Mallopt` writes: `max_fast` (with its two
/// flag bits) and the four tunables.
pub(crate) const MALLOPT_MAX_FAST: u32 = 0;
pub(crate) const MALLOPT_TRIM_THRESHOLD_FIELD: u32 = 0x344;
pub(crate) const MALLOPT_TOP_PAD_FIELD: u32 = 0x348;
pub(crate) const MALLOPT_MMAP_THRESHOLD_FIELD: u32 = 0x34c;
pub(crate) const MALLOPT_N_MMAPS_MAX_FIELD: u32 = 0x354;

/// `int DLHeap_Mallopt(DLHeap *heap, int param, int value)` (0x31fe30):
/// dlmalloc's `mALLOPt` on the heap's `malloc_state` (`*heap`), after
/// consolidating it. Returns 1 when the setting was taken, else 0. `M_MXFAST`
/// (1) accepts 0..80: 0 sets the "no fastbins" flag (8), else the request
/// size rounded (`(value + 19) & ~15`) keeps the state's low two flag bits;
/// `M_MMAP_MAX` (-4) only accepts 0 (no mmap here).
pub fn dlheap_mallopt(c: &mut Cpu) {
    use callee::*;
    if c.pc != 0x0031_fe30 {
        return ee::s10::f_0031fe30__generated(c);
    }
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A1][0] = c.g[S2][0];
        DLHEAP_CONSOLIDATE.call(c, 0x0031_fe58)?;

        let state = Addr(c.g[S2][0] as u32);
        let value = c.g[S1][0];
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        let in_table = c.g[V1][0] < 6;
        c.g[V0][0] = in_table as u64;
        c.g[V0][0] = 0x0049_0000;
        let label = if in_table {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(MALLOPT_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(MALLOPT_TABLE));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A0][0] as u32
        } else {
            MALLOPT_NONE
        };
        let field = match label {
            MALLOPT_MXFAST => {
                c.cycles += 3;
                if value >= 0x51 {
                    c.g[V0][0] = 0;
                    None
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = state.off(MALLOPT_MAX_FAST).lw(c);
                    c.g[A2][0] = c.g[V0][0] & 0x3;
                    c.g[V0][0] = if value == 0 {
                        c.cycles += 1;
                        c.g[A2][0] | 0x8
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = sx((value as u32).wrapping_add(0x13));
                        // Never below 0x10 for an accepted `value` (1..80).
                        if c.g[V1][0] < 0x10 {
                            c.g[V0][0] = 1;
                            c.g[A2][0] | 0x10
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = 0xffff_ffff_ffff_fff0;
                            (c.g[V1][0] & 0xffff_ffff_ffff_fff0) | c.g[A2][0]
                        }
                    };
                    c.cycles += 3;
                    let v = c.g[V0][0];
                    state.off(MALLOPT_MAX_FAST).sw(c, v);
                    c.g[V0][0] = 1;
                    None
                }
            }
            MALLOPT_TRIM_THRESHOLD => Some(MALLOPT_TRIM_THRESHOLD_FIELD),
            MALLOPT_TOP_PAD => Some(MALLOPT_TOP_PAD_FIELD),
            MALLOPT_MMAP_THRESHOLD => Some(MALLOPT_MMAP_THRESHOLD_FIELD),
            MALLOPT_MMAP_MAX => {
                c.cycles += 2;
                c.g[V0][0] = 0;
                if value == 0 {
                    c.cycles += 3;
                    state.off(MALLOPT_N_MMAPS_MAX_FIELD).sw(c, 0);
                    c.g[V0][0] = 1;
                }
                None
            }
            MALLOPT_NONE => {
                c.cycles += 1;
                c.g[V0][0] = 0;
                None
            }
            _ => {
                c.jump_out(label, 0x0031_fe30, 0x0031_ff20);
                return Ok(());
            }
        };
        if let Some(off) = field {
            c.cycles += 3;
            state.off(off).sw(c, value);
            c.g[V0][0] = 1;
        }

        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0031_fe30, dlheap_mallopt, "DLHeap_Mallopt")];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check_paths, elf_image, fill, DATA};

    /// A real dlmalloc arena (`DLHeap_InitArena`), the heap header at
    /// `MALLOPT_HEAP`; returns its `malloc_state`.
    const MALLOPT_HEAP: u32 = DATA + 0x2000;
    const MALLOPT_ARENA: u32 = DATA + 0x4_0000;

    fn mallopt_arena(c: &mut Cpu) -> u32 {
        let ra = c.g[RA][0];
        c.g[A0][0] = MALLOPT_HEAP as u64;
        c.g[A1][0] = MALLOPT_ARENA as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = 0x0000_7770;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, 0x0000_7770, "InitArena did not return");
        c.g[RA][0] = ra;
        c.bus.read32(MALLOPT_HEAP)
    }

    /// The parameters: the six table cases, values just outside the table
    /// (-5, 2) and far outside; the values: 0, 1..80 (M_MXFAST's range),
    /// 81 and above, negative and random words.
    pub(crate) fn mallopt_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0031_fe30, 0);
        elf_image(c);
        let state = mallopt_arena(c);
        assert_ne!(state, 0, "InitArena left no malloc_state");
        // Run `InitState` on the state here so the tunables below
        // survive: the port's `Consolidate` initializes a fresh arena on
        // entry, which would otherwise overwrite them with defaults.
        // (`InitState` takes the state, not the heap header.)
        c.g[A0][0] = state as u64;
        c.g[RA][0] = 0x0000_7770;
        c.pc = 0x0031_e260;
        ee::s10::f_0031e260__generated(c);
        assert_eq!(c.pc, 0x0000_7770, "InitState did not return");
        // `max_fast` (with steered flag bits) and the tunables get
        // distinct random words.
        c.bus.write32(state, 0x50 | (seed % 4));
        fill(c, &mut r, state + 0x340, 7);
        let param: i32 = [1, -1, -2, -3, -4, 0, -5, 2, 0x7fff_0000][seed as usize % 9];
        let value: i64 = match seed / 9 % 6 {
            0 => 0,
            1 => 1 + (r.next_u32() % 80) as i64,
            2 => 80,
            3 => 81 + (r.next_u32() % 0x1000) as i64,
            4 => -(1 + (r.next_u32() % 0x100) as i64),
            _ => r.next_u32() as i32 as i64,
        };
        c.g[A0][0] = MALLOPT_HEAP as u64;
        c.g[A1][0] = param as i64 as u64;
        c.g[A2][0] = value as u64;
        c.pc = 0x0031_fe30;
    }

    #[test]
    fn mallopt_table_holds_the_case_labels() {
        let mut c = crate::difftest::fresh();
        elf_image(&mut c);
        let table: Vec<u32> = (0..6)
            .map(|i| c.bus.read32(MALLOPT_TABLE + 4 * i))
            .collect();
        assert_eq!(
            table,
            [
                MALLOPT_MMAP_MAX,
                MALLOPT_MMAP_THRESHOLD,
                MALLOPT_TOP_PAD,
                MALLOPT_TRIM_THRESHOLD,
                MALLOPT_NONE,
                MALLOPT_MXFAST
            ]
        );
    }

    /// Every case: the three tunable stores, M_MMAP_MAX taken (0) and
    /// refused, M_MXFAST refused (> 80), zero (flag 8) and a size, the
    /// in-table `param 0` and the out-of-table default. Consolidate runs for
    /// real on the initialized arena, so the totals are fixed (`InitState`
    /// runs in the fixture, not the port).
    #[test]
    fn dlheap_mallopt_matches() {
        check_paths(
            ee::s10::f_0031fe30__generated,
            dlheap_mallopt,
            108,
            // default 1, param 0 7, M_MMAP_MAX refused 8, a tunable or
            // M_MXFAST refused 9, M_MMAP_MAX taken 11, M_MXFAST 0 16, size 24.
            &[93, 99, 100, 101, 103, 108, 116],
            &mallopt_fixture,
        );
    }
}
