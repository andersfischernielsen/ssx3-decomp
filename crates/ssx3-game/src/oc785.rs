//! Staging module oc785 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc785`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0017_fec0, feui_input_is_next, "FEUIInput_IsNext"),
    (0x002c_2580, ustr_to_ascii, "UStr_ToAscii"),
];

/// `FEUIInput_IsNext(input)` (0x17fec0): asks the pad object at `*(input + 0x18)`
/// whether button 0x7a ("next") is pressed (callee 0x320c48) and returns its answer.
pub fn feui_input_is_next(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x7a;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0032_0c48, 0x0017_fed4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `char* UStr_ToAscii(char* dst, const u16* src)` (0x2c2580): narrows a
/// zero-terminated UTF-16 string to bytes (low byte of each unit), writes the
/// terminator and returns `dst`.
pub fn ustr_to_ascii(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
    c.g[V1][0] = c.g[A0][0];
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(2));
            c.sb(c.g[A0][0] as u32, c.g[V0][0]);
            c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_2590);
            }
        }
    }
    c.cycles += 3;
    c.sb(c.g[A0][0] as u32, 0);
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[V1][0];
    c.pc = t;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA};

    /// `FEUIInput_IsNext`: the button query and its unwinds.
    #[test]
    fn feui_input_is_next_matches() {
        fakes::covered(
            ee::s02::f_0017fec0__generated,
            feui_input_is_next,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x785a, 1);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `UStr_ToAscii`: empty and 1..=9 unit strings (high bytes arbitrary, units nonzero).
    #[test]
    fn ustr_to_ascii_matches() {
        check(
            ee::s08::f_002c2580__generated,
            ustr_to_ascii,
            40,
            &|c, seed| {
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                let n = seed % 10;
                for i in 0..n {
                    let unit = 0x0100 * (i + 1) + 0x41 + (seed + i) % 20;
                    c.bus.write16(DATA + 0x2000 + 2 * i, unit as u16);
                }
                c.bus.write16(DATA + 0x2000 + 2 * n, 0);
                c.g[V0][0] = 0x1234;
                c.g[V1][0] = 0x5678;
            },
        );
    }
}
