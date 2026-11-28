//! `src/sce/crt0.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_0000, oc1600_crt0_jump, "lbl_00100000"),
    (0x0010_0008, crt_entry, "entry"),
];

/// `entry` (0x00100008): the program entry. Clears the BSS (0x4a4c00..0x53f115, 16 bytes
/// at a time, with a back-edge event check), sets up the main thread (syscall 0x3c: gp, stack,
/// heap arguments) and the heap (syscall 0x3d), runs two init calls, enables interrupts, then
/// calls the game main (0x31af80) with the argument block at 0x4a5c00 and tail-calls `exit`.
/// Resumable: any `pc` of the generated `__entries` twin continues natively.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn crt_entry(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        // The straight-line twin charges the tail's 32 cycles before the first syscall; the
        // resumable twin charges them per block. Both are 105 + the loop, but the calls see
        // `cycles`, so a run from the start charges up front.
        let straight = pc == 0x0010_0008;
        loop {
            match pc {
                0x00100008 => {
                    c.cycles += 62;
                    c.padduw(1, 0, 0);
                    c.padduw(2, 0, 0);
                    c.padduw(3, 0, 0);
                    c.padduw(4, 0, 0);
                    c.padduw(5, 0, 0);
                    c.padduw(6, 0, 0);
                    c.padduw(7, 0, 0);
                    c.padduw(8, 0, 0);
                    c.padduw(9, 0, 0);
                    c.padduw(10, 0, 0);
                    c.padduw(11, 0, 0);
                    c.padduw(12, 0, 0);
                    c.padduw(13, 0, 0);
                    c.padduw(14, 0, 0);
                    c.padduw(15, 0, 0);
                    c.padduw(16, 0, 0);
                    c.padduw(17, 0, 0);
                    c.padduw(18, 0, 0);
                    c.padduw(19, 0, 0);
                    c.padduw(20, 0, 0);
                    c.padduw(21, 0, 0);
                    c.padduw(22, 0, 0);
                    c.padduw(23, 0, 0);
                    c.padduw(24, 0, 0);
                    c.padduw(25, 0, 0);
                    c.padduw(28, 0, 0);
                    c.padduw(29, 0, 0);
                    c.padduw(30, 0, 0);
                    c.padduw(31, 0, 0);
                    c.hi[0] = 0u64;
                    c.hi[1] = 0u64;
                    c.lo[0] = 0u64;
                    c.lo[1] = 0u64;
                    c.mtsah(0, 0);
                    c.mtc1(0, 0);
                    c.mtc1(0, 1);
                    c.mtc1(0, 2);
                    c.mtc1(0, 3);
                    c.mtc1(0, 4);
                    c.mtc1(0, 5);
                    c.mtc1(0, 6);
                    c.mtc1(0, 7);
                    c.mtc1(0, 8);
                    c.mtc1(0, 9);
                    c.mtc1(0, 10);
                    c.mtc1(0, 11);
                    c.mtc1(0, 12);
                    c.mtc1(0, 13);
                    c.mtc1(0, 14);
                    c.mtc1(0, 15);
                    c.mtc1(0, 16);
                    c.mtc1(0, 17);
                    c.mtc1(0, 18);
                    c.mtc1(0, 19);
                    c.mtc1(0, 20);
                    c.mtc1(0, 21);
                    c.mtc1(0, 22);
                    c.mtc1(0, 23);
                    c.mtc1(0, 24);
                    c.mtc1(0, 25);
                    c.mtc1(0, 26);
                    c.mtc1(0, 27);
                    pc = 0x00100100;
                    continue;
                }
                0x00100100 => {
                    c.cycles += 11;
                    c.mtc1(0, 28);
                    c.mtc1(0, 29);
                    c.mtc1(0, 30);
                    c.mtc1(0, 31);
                    c.adda_s(0, 1);
                    c.ctc1(0, 31);
                    c.g[V0][0] = 0x4a0000;
                    c.g[V1][0] = 0x540000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4c00));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xeeb));
                    pc = 0x0010012c;
                    continue;
                }
                0x0010012c => {
                    c.cycles += 7;
                    c.sq(0, c.g[V0][0] as u32);
                    c.g[AT][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    let t = c.g[AT][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                    if t {
                        if back_edge_unwinds(c, 0x0010012c) {
                            return Ok(());
                        }
                        pc = 0x0010012c;
                        continue;
                    }
                    pc = 0x00100148;
                    continue;
                }
                0x00100148 => {
                    c.cycles += if straight { 32 } else { 22 };
                    c.g[A0][0] = 0x4a0000;
                    c.g[A1][0] = 0x1fe0000;
                    c.g[A2][0] = 0x20000;
                    c.g[A3][0] = 0x4a0000;
                    c.g[T0][0] = 0x100000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30f0));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x0));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x0));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5c00));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1d0));
                    c.g[GP][0] = c.g[A0][0];
                    c.g[V1][0] = 0x3c;
                    c.pc = 0x0010017c;
                    c.syscall(0x00100178);
                    if c.pc != 0x0010017c {
                        return Ok(());
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0010017c);
                    }
                    c.g[SP][0] = c.g[V0][0];
                    c.g[A0][0] = 0x540000;
                    c.g[A1][0] = 0x0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xeeb));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = 0x3d;
                    c.pc = 0x00100198;
                    c.syscall(0x00100194);
                    if c.pc != 0x00100198 {
                        return Ok(());
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x00100198);
                    }
                    c.g[RA][0] = sx(0x001001a0);
                    call_ptr(c, 0x0042c300, 0x001001a0)?;
                    pc = 0x001001a0;
                    continue;
                }
                0x001001a0 => {
                    if !straight {
                        c.cycles += 2;
                    }
                    c.g[RA][0] = sx(0x001001a8);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x00424020, 0x001001a8)?;
                    pc = 0x001001a8;
                    continue;
                }
                0x001001a8 => {
                    if !straight {
                        c.cycles += 6;
                    }
                    c.ei();
                    c.g[V0][0] = 0x4a0000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5c00));
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[RA][0] = sx(0x001001c0);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x0031af80, 0x001001c0)?;
                    pc = 0x001001c0;
                    continue;
                }
                0x001001c0 => {
                    if !straight {
                        c.cycles += 2;
                    }
                    c.g[A0][0] = c.g[V0][0];
                    c.pc = 0x0042c6e8;
                    call_indirect(c, 0x0042c6e8);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00100008, 0x001001c8);
                    return Ok(());
                }
            }
        }
    });
}

/// `lbl_00100000` (0x00100000): the 8-byte stub in front of the program entry: charges two
/// cycles and jumps (tail call, no frame) to `entry` (0x00100008).
pub fn oc1600_crt0_jump(c: &mut Cpu) {
    c.cycles += 2;
    c.pc = 0x0010_0008;
    ee::s00::f_00100008(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes;
    use crate::difftest::fakes::check_reaching;

    #[allow(unused_imports)]
    use crate::game::fe::ovstatereplay::tests::oc1526_hook;
    pub(crate) use crate::game::fe::ovstatereplay::tests::OC1526_CTRL;
    pub(crate) use crate::game::fe::ovstatereplay::tests::OC1526_MAGIC;
    pub(crate) use crate::game::sound::soundsys::tests::oc1600_machine;

    /// Fixture for `entry`: the init calls, the main call and `exit` are fakes, the two
    /// kernel syscalls go to the fake syscall hook; the function takes no inputs.
    pub(crate) fn oc1526_entry_setup(c: &mut Cpu, seed: u32) -> crate::difftest::Rng {
        oc1526_hook();
        let r = fakes::machine(c, seed, 0x1526, 6);
        c.bus.write32(OC1526_CTRL, OC1526_MAGIC);
        c.pc = 0x0010_0008;
        r
    }

    /// Clears the BSS, both syscalls, the init calls, main and the `exit` tail call.
    #[test]
    fn crt_entry_matches() {
        check_reaching(
            ee::s00::f_00100008__generated,
            crt_entry,
            12,
            &[0x0010_01a0, 0x0010_01a8, 0x0010_01c0],
            &|c, seed| {
                oc1526_entry_setup(c, seed);
            },
        );
    }

    /// Every resume point of the generated twin (after an unwound call or event) and a
    /// `pc` outside any block (the `jump_out` arm) continue the same way natively.
    #[test]
    fn crt_entry_resumes_match() {
        for pc in [
            0x0010_0100u32,
            0x0010_012c,
            0x0010_0148,
            0x0010_01a0,
            0x0010_01a8,
            0x0010_01c0,
            0x0010_0010,
        ] {
            check(ee::s00::f_00100008__generated, crt_entry, 6, &|c, seed| {
                oc1526_entry_setup(c, seed);
                c.pc = pc;
                if pc == 0x0010_012c {
                    // Inside the clear loop, a few stores from its end.
                    c.g[V0][0] = (0x0053_f100 - 16 * (seed % 4)) as u64;
                    c.g[V1][0] = 0x0053_f115;
                }
            });
        }
    }

    /// `lbl_00100000`: the tail jump to the program entry (2 cycles, `pc = 0x100008`).
    #[test]
    fn oc1600_crt0_jump_matches() {
        check(
            ee::s00::f_00100000__generated,
            oc1600_crt0_jump,
            8,
            &|c, seed| {
                oc1600_machine(c, seed, 0x1600_0, 1);
            },
        );
    }
}
