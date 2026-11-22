//! Native no-reset range predicate0x26a000, retained uninstalled.
//!
//! This leaf is absent from generated/functions.tsv and generated/ee. Its
//! original instructions were read from the USA executable SLUS_207.72.
//! No tests, builds, replays or game runs were added or executed.

use crate::abi;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0026_a000, path_reset_allowed, "Path_ResetAllowed",
)];

const ENTRY: u32 = 0x0026_a000;
const LOOP: u32 = 0x0026_a018;
const END: u32 = 0x0026_a064;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }

/// `bool Path_ResetAllowed(EventRanges* ranges, float distance)`.
/// Return false if any inclusive range covering F12 has runtime type13.
/// The native stored-key table maps that type to key101. Source Tricky
/// instead stores NoReset as104 (runtime29 in its original key table).
pub fn path_reset_allowed(c: &mut Cpu) {
    let have_entries = match c.pc {
        ENTRY => {
            c.cycles += 3;
            c.g[A1][0] = c.lw(r(c, A0));
            let have_entries = (c.g[A1][0] as i64) > 0;
            c.g[V1][0] = 0;
            if have_entries {
                c.cycles += 3;
                c.g[A0][0] = c.lw(r(c, A0).wrapping_add(4));
                c.g[A2][0] = 13;
            }
            have_entries
        }
        LOOP => true,
        pc => {
            c.jump_out(pc, ENTRY, END);
            return;
        }
    };
    if have_entries {
        loop {
            c.cycles += 5;
            c.lwc1(0, r(c, A0).wrapping_add(8));
            c.c_lt_s(12, 0);
            let before_range = c.fcc();
            c.g[V1][0] = sx(r(c, V1).wrapping_add(1));
            let mut inside = !before_range;
            if inside {
                c.cycles += 4;
                c.lwc1(0, r(c, A0).wrapping_add(12));
                c.c_lt_s(0, 12);
                inside = !c.fcc();
            }
            if inside {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r(c, A0));
                let blocked = c.g[V0][0] == c.g[A2][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                if blocked {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    abi::ret(c);
                    return;
                }
                c.cycles += 2;
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            }
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r(c, A0).wrapping_add(16));
            if !more { break }
            if event_due(c) { deliver_event_here(c, LOOP); }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}
