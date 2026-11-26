//! Staging module oc902 (wave wave253): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc902`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x001c_3570, cfe_state_stats_v25, "cFEStateStats_v25")];

/// `StrHash` (0x317670) of the string at `string` (in `a0`); the hash comes
/// back in `v0`.
fn v25_hash(c: &mut Cpu, string: u64, back: u32) -> Flow {
    c.g[A0][0] = string;
    call_ptr(c, 0x0031_7670, back)
}

/// `cFEStateNetHelpPopup_SetButtonHelp(menu, index, text, code)` (0x1da510):
/// `a0 = menu`, `a2 = v0` (the hash from `v25_hash`), `a1 = index`, `a3 = code`.
fn v25_button_help(c: &mut Cpu, index: u64, code: u64, back: u32) -> Flow {
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = index;
    c.g[A3][0] = code;
    call_ptr(c, 0x001d_a510, back)
}

/// `cFEStateNetHelpPopup_SetBodyText(menu, 0, text)` (0x1da530): `a3` is left
/// as the previous call set it, as the generated body does.
fn v25_body_text(c: &mut Cpu, back: u32) -> Flow {
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x001d_a530, back)
}

/// `void cFEStateStats_v25(menu)` (0x1c3570), the help popup's option page:
/// the menu is in `a1`. Sets the option count to 5 (`cFEStateNetHelpPopup_
/// SetNumButtons`), stores the title hash at `+0x54`, sets the body text and
/// the five button rows, each a `StrHash` of a string in the image and a code.
pub fn cfe_state_stats_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 62;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[A1][0] = 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001c_358c)?;
        v25_hash(c, 0x0046_5dc0, 0x001c_3598)?;
        let title = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x54)).sw(c, title);
        v25_hash(c, 0x0046_5dd8, 0x001c_35a8)?;
        v25_body_text(c, 0x001c_35b8)?;
        v25_hash(c, 0x0045_fe60, 0x001c_35c4)?;
        v25_button_help(c, 0, 3, 0x001c_35d8)?;
        v25_hash(c, 0x0045_fe40, 0x001c_35e4)?;
        v25_button_help(c, 1, 4, 0x001c_35f8)?;
        v25_hash(c, 0x0046_1458, 0x001c_3604)?;
        v25_button_help(c, 2, 2, 0x001c_3618)?;
        v25_hash(c, 0x0046_5df8, 0x001c_3624)?;
        v25_button_help(c, 3, 8, 0x001c_3638)?;
        v25_hash(c, 0x0046_14e8, 0x001c_3644)?;
        v25_button_help(c, 4, 1, 0x001c_3658)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    #[test]
    fn cfe_state_stats_v25_matches() {
        check(
            ee::s03::f_001c3570__generated,
            cfe_state_stats_v25,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x1c35, 0);
                c.pc = 0x001c_3570;
                c.g[A1][0] = (DATA + 0x2000 + (seed % 4) * 0x100) as u64;
            },
        );
    }
}
