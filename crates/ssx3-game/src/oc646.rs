//! Staging module oc646 (wave g26.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc646`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0023_7140,
    input_sync_add_local_pad,
    "InputSync_AddLocalPad",
)];

/// `void InputSync_AddLocalPad(InputSync* self, const void* src)` (0x237140):
/// appends a local pad record to the table at `+0x14` (0x18-byte records, the
/// count at `+0x10`). The record holds a zeroed buffer of `count * 0x64`
/// bytes (first word of each 0x64 block set to 0x18), the sync's word 0 (`+8`),
/// `*src` (`+0xc`), a 0x2a4-byte block (`+0x10`, its first word cleared) and
/// an 8-byte `ValuePair` (`+0x14`) made from `+0x10` and `src[1]`. Frame 0x50.
pub fn input_sync_add_local_pad(c: &mut Cpu) {
    const ALLOC_BUFFER: u32 = 0x0031_7e30;
    const ALLOC_BLOCK: u32 = 0x0031_7d70;
    const MEM_FILL: u32 = 0x003e_6448;
    const VALUE_PAIR_SET_FIRST: u32 = 0x0024_4400;
    const VALUE_PAIR_SET_SECOND: u32 = 0x0024_4408;
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x18;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0x64;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x3f88));
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x10));
        c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(4));
        c.mult(0, 3, 2, 3);
        c.mult(0, 0, 4, 16);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.sw(r32(c, S1).wrapping_add(0x10), c.g[V0][0]);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x14));
        c.g[A0][0] = c.lo[0];
        c.g[S2][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
        call_ptr(c, ALLOC_BUFFER, 0x0023_71a0)?;
        c.sw(r32(c, S2), c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0];
        let count = c.lw(r32(c, S1).wrapping_add(4));
        c.g[A2][0] = count;
        c.mult(0, 0, 6, 16);
        c.g[A2][0] = c.lo[0];
        c.g[A1][0] = 0;
        call_ptr(c, MEM_FILL, 0x0023_71bc)?;
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(4));
        let no_blocks = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !no_blocks {
            c.cycles += 2;
            c.g[A1][0] = 0x18;
            c.g[A0][0] = 0;
            loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(r32(c, S2));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.sw(r32(c, V0), c.g[A1][0]);
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(4));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let again = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x64));
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0023_71d0);
                }
            }
        }
        c.cycles += 36;
        c.sw(r32(c, S2).wrapping_add(4), 0);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x3f78));
        c.g[A0][0] = 0x2a4;
        c.g[V1][0] = c.lw(r32(c, S1));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.sw(r32(c, S2).wrapping_add(8), c.g[V1][0]);
        c.g[V0][0] = c.lw(r32(c, S3));
        c.sw(r32(c, S2).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, ALLOC_BLOCK, 0x0023_721c)?;
        c.sw(r32(c, S2).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x3f68));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 8;
        c.sw(r32(c, V0), 0);
        call_ptr(c, ALLOC_BLOCK, 0x0023_723c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(r32(c, S2).wrapping_add(0x10));
        c.g[S1][0] = c.lw(r32(c, S3).wrapping_add(4));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, VALUE_PAIR_SET_FIRST, 0x0023_7250)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, VALUE_PAIR_SET_SECOND, 0x0023_725c)?;
        c.sw(r32(c, S2).wrapping_add(0x14), c.g[S0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA};
    use crate::game::ai::computer::tests::rbc_test_allocator;

    /// The record table of `input_sync_add_local_pad` against the generated
    /// body: the count 0..3 (the zero count skips the fill loop), a record
    /// index 0..2, the allocator pools and the real `ValuePair` setters.
    #[test]
    fn input_sync_add_local_pad_matches() {
        check(ee::s06::f_00237140__generated, input_sync_add_local_pad, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0023_7140, 0);
            rbc_test_allocator(c);
            let this = DATA + 0x1000;
            let src = DATA + 0x3000;
            let count = [0u32, 1, 2, 3][seed as usize % 4];
            c.bus.write32(this, r.next_u32());
            c.bus.write32(this + 4, count);
            c.bus.write32(this + 0x10, (seed / 4) % 3);
            c.bus.write32(src, r.next_u32());
            c.bus.write32(src + 4, r.next_u32());
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(src);
        });
    }
}
