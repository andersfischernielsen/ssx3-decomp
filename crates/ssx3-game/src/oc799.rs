//! Staging module oc799 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc799`.

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
    (0x0038_b158, ps2_bezier_man_v04, "PS2BezierMan_v04"),
    (0x002a_a408, stream_buffer_stop, "StreamBuffer_Stop"),
];

/// Offset of the embedded `BezierPatchCache` in `cPS2BezierMan`.
pub(crate) const PS2_BEZIER_MAN_V04_CACHE: u32 = 0x10;
/// Offset of the sound stream handle in a `StreamBuffer`.
pub(crate) const STREAM_BUFFER_STOP_HANDLE: u32 = 0x8;

/// `void PS2BezierMan_v04(cPS2BezierMan* man)` (0x38b158, `cPS2BezierMan_Reset`):
/// resets the patch cache embedded at `man + 0x10`.
pub fn ps2_bezier_man_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(PS2_BEZIER_MAN_V04_CACHE));
        call_ptr(c, 0x0037_39d0, 0x0038_b168)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamBuffer_Stop(StreamBuffer* sb)` (0x2aa408,
/// `cStreamInstance_Purge`): `SNDSTRM_Stop(sb->handle)` (handle at +0x8).
pub fn stream_buffer_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(STREAM_BUFFER_STOP_HANDLE));
        call_ptr(c, 0x003b_78f8, 0x002a_a418)?;
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

    /// `PS2BezierMan_v04`: resets the cache at +0x10; pool counts of 0..3 with
    /// valid array pointers so the real `BezierPatchCache_Reset` and `MEM_fill`
    /// run on short paths.
    #[test]
    fn ps2_bezier_man_v04_matches() {
        check(
            ssx3_ee::s12::f_0038b158__generated,
            ps2_bezier_man_v04,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x799a, 0);
                c.pc = 0x0038_b158;
                let man = DATA + 0x1_0000;
                let cache = man + PS2_BEZIER_MAN_V04_CACHE;
                c.g[A0][0] = sx(man);
                for k in 0..3u32 {
                    let (n, nb) = (r.next_u32() % 4, r.next_u32() % 4);
                    c.bus.write32(cache + 4 * k, n);
                    c.bus.write32(cache + 0x1a8 + 4 * k, nb);
                    c.bus
                        .write32(cache + 0xc + 4 * k, DATA + 0x2_0000 + 0x400 * k);
                    c.bus
                        .write32(cache + 0x18 + 4 * k, DATA + 0x3_0000 + 0x400 * k);
                    c.bus
                        .write32(cache + 0x30 + 4 * k, DATA + 0x4_0000 + 0x400 * k);
                    c.bus
                        .write32(cache + 0x1cc + 4 * k, DATA + 0x5_0000 + 0x400 * k);
                }
            },
        );
    }

    /// `StreamBuffer_Stop`: passes the buffer's stream handle to the real
    /// `SNDSTRM_Stop` (no stream is registered, so it takes the not-found path).
    #[test]
    fn stream_buffer_stop_matches() {
        check(
            ssx3_ee::s08::f_002aa408__generated,
            stream_buffer_stop,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x799b, 0);
                c.pc = 0x002a_a408;
                let sb = DATA + 0x1_0000;
                c.bus.write32(sb + STREAM_BUFFER_STOP_HANDLE, r.next_u32());
                c.g[A0][0] = sx(sb);
            },
        );
    }
}
