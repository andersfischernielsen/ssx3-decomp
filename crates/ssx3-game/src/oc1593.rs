//! Staging module oc1593 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1593`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_cef8, usbkb_copy_report, "usbkb_copy_report"),
    (0x0040_cf88, usbkb_count_repeats, "usbkb_count_repeats"),
    (0x0041_2cb4, eh_pop_then_dispatch, "eh_pop_then_dispatch"),
    (0x0041_2cc4, eh_resume_412b98, "eh_resume_412b98"),
    (0x0041_2ccc, eh_terminate_bad_cast, "eh_terminate_bad_cast"),
    (0x0041_2d4c, eh_store_typeinfo_412d4c, "eh_store_typeinfo_412d4c"),
    (0x0041_2dcc, eh_store_typeinfo_412dcc, "eh_store_typeinfo_412dcc"),
    (0x0042_1eb0, print_then_tail_421eb0, "print_then_tail_421eb0"),
];

/// Address of the usbkb repeat table `D_00529000`: the record count at +0, the
/// pointer to the record array at +4.
const REPEAT_TABLE: u32 = 0x0052_9000;

/// `int usbkb_copy_report(void)` (0x40cef8): when byte 6 of the received 7-byte
/// report (at 0x529140) is zero, copy the 7 bytes to the buffer whose pointer
/// is stored at 0x529098 and return 0; otherwise return -1.
pub fn usbkb_copy_report(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x0053_0000;
    c.g[V1][0] = 0x0053_0000;
    c.g[A0][0] = sx(0x0053_0000u32.wrapping_sub(0x6ec0));
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x6)).lbu(c);
    let copy = c.g[V0][0] == 0;
    c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x6f68)).lw(c);
    if !copy {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[A1][0] = 0x6;
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        let byte = c.g[V0][0];
        Addr(c.g[V1][0] as u32).sb(c, byte);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        if (c.g[A1][0] as i64) >= 0 {
            if event_due(c) {
                deliver_event_here(c, 0x0040_cf20);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `void usbkb_count_repeats(void)` (0x40cf88): for each key-repeat record of
/// the table at [`REPEAT_TABLE`] (records 0x38 bytes apart, the first at +0x10
/// of the array: state at +0, count at +4, delay at +8, rate at +0xc), count
/// one more when the delay and rate are positive and the state is 1 or 2.
/// Then re-enables interrupts and returns 0.
pub fn usbkb_count_repeats(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = 0x0053_0000;
    c.g[V0][0] = Addr(REPEAT_TABLE).lw(c);
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A0][0] = 0;
    if !none {
        c.cycles += 4;
        c.g[V0][0] = sx(REPEAT_TABLE);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        loop {
            'record: {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
                let no_delay = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = Addr(REPEAT_TABLE).lw(c);
                if no_delay {
                    break 'record;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
                let no_rate = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = Addr(REPEAT_TABLE).lw(c);
                if no_rate {
                    break 'record;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(REPEAT_TABLE).lw(c);
                    break 'record;
                }
                // The state is 1 or 2 (positive, below 3): count the repeat.
                c.cycles += 2;
                let negative = (c.g[V0][0] as i64) < 0;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
                if !negative {
                    c.cycles += 2;
                    let state_high = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(REPEAT_TABLE).lw(c);
                    if state_high {
                        break 'record;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    let count = c.g[V0][0];
                    Addr((c.g[V1][0] as u32).wrapping_add(0x4)).sw(c, count);
                }
                c.cycles += 1;
                c.g[V0][0] = Addr(REPEAT_TABLE).lw(c);
            }
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x38));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0040_cfa8);
            }
        }
    }
    c.cycles += 4;
    c.ei();
    c.g[V0][0] = 0;
    ret(c);
}

/// `__cp_pop_exception(s4)`, then a jump into the dispatcher's resume at
/// 0x412ca0 (`__throw`), the landing pad of 0x412cb4. The generated `if t`
/// is always true, so only that path is kept.
pub fn eh_pop_then_dispatch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[A0][0] = c.g[S4][0];
        Callee {
            addr: 0x0041_2a28,
            f: ee::s15::f_00412a28,
        }
        .call(c, 0x0041_2cbc)?;
        c.pc = 0x0041_2ca0;
        call_indirect(c, 0x0041_2ca0);
        Ok(())
    });
}

/// Tail jump into the exception dispatcher's resume at 0x412b98 (0x412cc4),
/// which throws. Leaves `ra` as it was.
pub fn eh_resume_412b98(c: &mut Cpu) {
    c.cycles += 2;
    c.pc = 0x0041_2b98;
    call_indirect(c, 0x0041_2b98);
}

/// `terminate()`, then the tail call to `__throw_bad_cast` (0x412ccc).
pub fn eh_terminate_bad_cast(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        Callee {
            addr: 0x0041_2800,
            f: ee::s15::f_00412800,
        }
        .call(c, 0x0041_2cd4)?;
        c.pc = 0x0041_2cd8;
        ee::s15::f_00412cd8(c);
        Ok(())
    });
}

/// Stores the type-info word `S2 - 0x6748` at the stack slot and tail-jumps
/// to `target` through the function table (the bodies of 0x412d4c and
/// 0x412dcc differ only in that target).
fn eh_store_typeinfo_and_call(c: &mut Cpu, target: u32) {
    c.cycles += 3;
    let info = sx((c.g[S2][0] as u32).wrapping_sub(0x6748));
    c.g[V0][0] = info;
    let slot = sp(c);
    c.sw(slot, info);
    c.pc = target;
    call_indirect(c, target);
}

/// Store the type-info word, then tail-jump to the throw in 0x412d08 (0x412d4c).
pub fn eh_store_typeinfo_412d4c(c: &mut Cpu) {
    eh_store_typeinfo_and_call(c, 0x0041_2d10);
}

/// Store the type-info word, then tail-jump to the throw in 0x412d88 (0x412dcc).
pub fn eh_store_typeinfo_412dcc(c: &mut Cpu) {
    eh_store_typeinfo_and_call(c, 0x0041_2d90);
}

/// `print_f(out, ...)` with the format words of 0x421eb0, then the tail call
/// into the epilogue at 0x421f0c (the generated `if t` is always true).
pub fn print_then_tail_421eb0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[T1][0] = 0x1;
        c.g[A2][0] = c.g[A1][0];
        if c.g[A3][0] == 0 {
            c.g[T1][0] = c.g[T2][0];
        }
        c.g[A0][0] = c.g[T3][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[T2][0] = 0x3;
        Callee {
            addr: 0x0042_1530,
            f: ee::s15::f_00421530,
        }
        .call(c, 0x0042_1ecc)?;
        c.g[V0][0] = c.g[S0][0];
        c.pc = 0x0042_1f0c;
        call_indirect(c, 0x0042_1f0c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;

    /// Gate words of the test fakes (each in the machine's own RAM).
    const OC1593_POP_GATE: u32 = DATA + 0x3b_0000;
    const OC1593_THROW_GATE: u32 = DATA + 0x3b_0010;
    const OC1593_TERM_GATE: u32 = DATA + 0x3b_0020;
    const OC1593_BAD_CAST_GATE: u32 = DATA + 0x3b_0030;
    const OC1593_PRINT_GATE: u32 = DATA + 0x3b_0040;
    const OC1593_POP_MAGIC: u32 = 0x1593_0001;
    const OC1593_THROW_MAGIC: u32 = 0x1593_0002;
    const OC1593_TERM_MAGIC: u32 = 0x1593_0003;
    const OC1593_BAD_CAST_MAGIC: u32 = 0x1593_0004;
    const OC1593_PRINT_MAGIC: u32 = 0x1593_0005;
    /// A pc that is not a return address: the callee unwinds.
    const OC1593_UNWIND_PC: u32 = 0xdead_0040;

    /// Logs a call to `target` and returns to `ra`, or unwinds when the
    /// mailbox at `gate + 4` is set. With the gate off it runs what the slot
    /// held before the test hooked it.
    fn oc1593_return_fake(
        c: &mut Cpu,
        gate: u32,
        magic: u32,
        target: u32,
        me: fn(&mut Cpu),
        generated: fn(&mut Cpu),
    ) {
        if c.bus.read32(gate) != magic {
            return fakes::fall_back(c, fakes::prev_cell(me), generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.bus.read32(gate + 4) != 0 {
            c.pc = OC1593_UNWIND_PC;
        }
    }

    /// Gated fake of `__cp_pop_exception` (0x412a28).
    fn oc1593_fake_pop(c: &mut Cpu) {
        oc1593_return_fake(
            c,
            OC1593_POP_GATE,
            OC1593_POP_MAGIC,
            0x0041_2a28,
            oc1593_fake_pop,
            ee::s15::f_00412a28__generated,
        )
    }

    /// Gated fake of `terminate` (0x412800).
    fn oc1593_fake_term(c: &mut Cpu) {
        oc1593_return_fake(
            c,
            OC1593_TERM_GATE,
            OC1593_TERM_MAGIC,
            0x0041_2800,
            oc1593_fake_term,
            ee::s15::f_00412800__generated,
        )
    }

    /// Gated fake of `__throw_bad_cast` (0x412cd8).
    fn oc1593_fake_bad_cast(c: &mut Cpu) {
        oc1593_return_fake(
            c,
            OC1593_BAD_CAST_GATE,
            OC1593_BAD_CAST_MAGIC,
            0x0041_2cd8,
            oc1593_fake_bad_cast,
            ee::s15::f_00412cd8__generated,
        )
    }

    /// Gated fake of `print_f` (0x421530).
    fn oc1593_fake_print(c: &mut Cpu) {
        oc1593_return_fake(
            c,
            OC1593_PRINT_GATE,
            OC1593_PRINT_MAGIC,
            0x0042_1530,
            oc1593_fake_print,
            ee::s15::f_00421530__generated,
        )
    }

    /// Gated fake of `__throw` (0x410f48): always unwinds, so the dispatcher
    /// resumes of the landing pads return at their first throw.
    fn oc1593_fake_throw(c: &mut Cpu) {
        if c.bus.read32(OC1593_THROW_GATE) != OC1593_THROW_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1593_fake_throw),
                ee::s15::f_00410f48__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0041_0f48, ra);
        c.pc = OC1593_UNWIND_PC;
    }

    fn oc1593_hook_all() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00412A28),
            oc1593_fake_pop,
            fakes::prev_cell(oc1593_fake_pop),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00412800),
            oc1593_fake_term,
            fakes::prev_cell(oc1593_fake_term),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00412CD8),
            oc1593_fake_bad_cast,
            fakes::prev_cell(oc1593_fake_bad_cast),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00421530),
            oc1593_fake_print,
            fakes::prev_cell(oc1593_fake_print),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00410F48),
            oc1593_fake_throw,
            fakes::prev_cell(oc1593_fake_throw),
        );
    }

    /// Arms the gates; `unwind` makes the pop, terminate and print fakes unwind.
    fn oc1593_arm(c: &mut Cpu, unwind: bool) {
        for (gate, magic) in [
            (OC1593_POP_GATE, OC1593_POP_MAGIC),
            (OC1593_THROW_GATE, OC1593_THROW_MAGIC),
            (OC1593_TERM_GATE, OC1593_TERM_MAGIC),
            (OC1593_BAD_CAST_GATE, OC1593_BAD_CAST_MAGIC),
            (OC1593_PRINT_GATE, OC1593_PRINT_MAGIC),
        ] {
            c.bus.write32(gate, magic);
            c.bus.write32(gate + 4, unwind as u32);
        }
    }

    /// Both byte-copy outcomes: the flag byte set or clear, and the copy
    /// through the destination pointer.
    #[test]
    fn usbkb_copy_report_matches() {
        check(ee::s15::f_0040cef8__generated, usbkb_copy_report, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0040_cef8, 0);
            c.pc = 0x0040_cef8;
            for i in 0..7u32 {
                c.bus.write8(0x0052_9140 + i, r.next_u32() as u8);
            }
            let flag = if seed % 3 == 0 {
                1 + (r.next_u32() % 255) as u8
            } else {
                0
            };
            c.bus.write8(0x0052_9146, flag);
            c.bus.write32(0x0052_9098, DATA + 0x1000 + 0x10 * (seed % 8));
        });
    }

    /// Empty, negative and populated tables; every state, delay and rate class.
    #[test]
    fn usbkb_count_repeats_matches() {
        check(ee::s15::f_0040cf88__generated, usbkb_count_repeats, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0040_cf88, 0);
            c.pc = 0x0040_cf88;
            let n = match seed % 8 {
                0 => 0,
                1 => u32::MAX,
                _ => 1 + r.next_u32() % 6,
            };
            c.bus.write32(0x0052_9000, n);
            c.bus.write32(0x0052_9004, DATA + 0x1000);
            for k in 0..6u32 {
                let rec = DATA + 0x1010 + 0x38 * k;
                let state = [0, 1, 2, 3, u32::MAX, 5][(r.next_u32() % 6) as usize];
                let delay = [0, 1, 2, u32::MAX][(r.next_u32() % 4) as usize];
                let rate = [0, 1, u32::MAX][(r.next_u32() % 3) as usize];
                c.bus.write32(rec, state);
                c.bus.write32(rec + 4, r.next_u32());
                c.bus.write32(rec + 8, delay);
                c.bus.write32(rec + 0xc, rate);
            }
        });
    }

    /// The pop returns or unwinds; the dispatcher resume throws and unwinds.
    #[test]
    fn eh_pop_then_dispatch_matches() {
        oc1593_hook_all();
        check(ee::s15::f_00412cb4__generated, eh_pop_then_dispatch, 200, &|c, seed| {
            let _ = machine(c, seed, 0x0041_2cb4, 0);
            c.pc = 0x0041_2cb4;
            oc1593_arm(c, seed % 3 == 0);
        });
    }

    /// The jump into the dispatcher's resume at 0x412b98 throws and unwinds.
    #[test]
    fn eh_resume_412b98_matches() {
        oc1593_hook_all();
        check(ee::s15::f_00412cc4__generated, eh_resume_412b98, 100, &|c, seed| {
            let _ = machine(c, seed, 0x0041_2cc4, 0);
            c.pc = 0x0041_2cc4;
            oc1593_arm(c, false);
        });
    }

    /// Terminate returns or unwinds; the tail call reaches `__throw_bad_cast`.
    #[test]
    fn eh_terminate_bad_cast_matches() {
        oc1593_hook_all();
        check(ee::s15::f_00412ccc__generated, eh_terminate_bad_cast, 200, &|c, seed| {
            let _ = machine(c, seed, 0x0041_2ccc, 0);
            c.pc = 0x0041_2ccc;
            oc1593_arm(c, seed % 3 == 0);
        });
    }

    /// The store of the type-info word and the tail jump into the throw of
    /// 0x412d08 (resumed at 0x412d10, unwinding).
    #[test]
    fn eh_store_typeinfo_412d4c_matches() {
        oc1593_hook_all();
        check(ee::s15::f_00412d4c__generated, eh_store_typeinfo_412d4c, 150, &|c, seed| {
            let _ = machine(c, seed, 0x0041_2d4c, 0);
            c.pc = 0x0041_2d4c;
            oc1593_arm(c, false);
        });
    }

    /// The same shape, tail-jumping into the throw of 0x412d88 (at 0x412d90).
    #[test]
    fn eh_store_typeinfo_412dcc_matches() {
        oc1593_hook_all();
        check(ee::s15::f_00412dcc__generated, eh_store_typeinfo_412dcc, 150, &|c, seed| {
            let _ = machine(c, seed, 0x0041_2dcc, 0);
            c.pc = 0x0041_2dcc;
            oc1593_arm(c, false);
        });
    }

    /// `print_f` returns or unwinds; the tail call reaches the 0x421f0c epilogue
    /// with the saved return address. A3 zero and nonzero take both paths.
    #[test]
    fn print_then_tail_421eb0_matches() {
        oc1593_hook_all();
        check(ee::s15::f_00421eb0__generated, print_then_tail_421eb0, 200, &|c, seed| {
            let _ = machine(c, seed, 0x0042_1eb0, 0);
            c.pc = 0x0042_1eb0;
            oc1593_arm(c, seed % 4 == 0);
            if seed % 2 == 0 {
                c.g[A3][0] = 0;
            }
            c.bus.write64(crate::difftest::SP, crate::difftest::RA as u64);
        });
    }
}
