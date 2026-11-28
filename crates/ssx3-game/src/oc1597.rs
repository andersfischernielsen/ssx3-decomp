//! Staging module oc1597 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1597`.

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
    (0x0042_c4d0, kcopy_3_impl, "kCopy_3_Impl"),
    (0x0042_c780, kcopy_4_impl, "kCopy_4_Impl"),
    (0x0042_cb78, kcopy_5_impl, "kCopy_5_Impl"),
    (0x0042_e278, rcmp_codec_dtor, "RCMP_CODEC_dtor"),
    (0x0042_e498, rcmp_system_dtor, "RCMP_SYSTEM_dtor"),
];

/// `kCopy_3_Impl(dst, src, count)` (0x42c4d0): copies `count` bytes from
/// `src` to `dst` one at a time, then returns 0. The loop checks events at
/// its back edge (head 0x42c4d8).
pub fn kcopy_3_impl(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A3][0] = 0u64;
    if c.g[A2][0] != 0u64 {
        loop {
            c.cycles += 8;
            c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = (c.g[A3][0] < c.g[A2][0]) as u64;
            c.sb(c.g[A0][0] as u32, c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            if c.g[V0][0] == 0u64 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0042_c4d8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// The word copy shared by `kCopy_4_Impl` and `kCopy_5_Impl`: `a2` is a byte
/// count, copied as `a2 >> 2` words from `a1` to `a0`, then returns 0. The
/// back edge checks events at `head`.
fn kcopy_words(c: &mut Cpu, head: u32) {
    c.cycles += 3;
    c.g[A2][0] = sx((c.g[A2][0] as u32) >> 2);
    let empty = c.g[A2][0] == 0u64;
    c.g[A3][0] = 0u64;
    if !empty {
        c.cycles += 1;
        loop {
            c.cycles += 8;
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = (c.g[A3][0] < c.g[A2][0]) as u64;
            c.sw(c.g[A0][0] as u32, c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            if c.g[V0][0] == 0u64 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `kCopy_4_Impl(dst, src, bytes)` (0x42c780): word copy, see [`kcopy_words`].
pub fn kcopy_4_impl(c: &mut Cpu) {
    kcopy_words(c, 0x0042_c790);
}

/// `kCopy_5_Impl(dst, src, bytes)` (0x42cb78): the same word copy as
/// [`kcopy_4_impl`], with its own loop head (0x42cb88).
pub fn kcopy_5_impl(c: &mut Cpu) {
    kcopy_words(c, 0x0042_cb88);
}

/// `RCMP_CODEC::~RCMP_CODEC(this, flags)` (0x42e278): stores the codec vtable
/// word (0x456810) at `this`, then, when bit 0 of `flags` is set, calls the
/// function at `D_00509434[0]`.
pub fn rcmp_codec_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x450000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6810));
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let call_hook = c.g[A1][0] != 0u64;
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        if call_hook {
            c.cycles += 4;
            c.g[V1][0] = 0x510000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bcc));
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0042_e2a4)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `RCMP_SYSTEM::~RCMP_SYSTEM(this, flags)` (0x42e498): stores the system
/// vtable word (0x456890) at `this + 0xc`, then deletes the object (through
/// the function table, 0x317e50) when bit 0 of `flags` is set.
pub fn rcmp_system_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x450000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6890));
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let delete_object = c.g[A1][0] != 0u64;
        c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        if delete_object {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0042_e4bc)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the `__builtin_delete` fake below: its own address in the
    /// machine's RAM (beyond the fakes' call log), armed with `OC1597_MAGIC`.
    const OC1597_MAGIC: u32 = 0x1597_0001;
    const OC1597_DELETE_CTRL: u32 = DATA + 0x5e_0000;

    static OC1597_PREV_DELETE: AtomicUsize = AtomicUsize::new(0);

    /// `__builtin_delete`: a logged no-op call while its gate is armed.
    fn oc1597_fake_delete(c: &mut Cpu) {
        if c.bus.read32(OC1597_DELETE_CTRL) != OC1597_MAGIC {
            return fakes::fall_back(c, &OC1597_PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        fakes::fake_call(c, 0x0031_7e50);
    }

    fn oc1597_install_delete() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1597_fake_delete,
            &OC1597_PREV_DELETE,
        );
    }

    /// The copy fixture: the event schedule for `seed`, then a2 = `count`,
    /// a0 = dst and a1 = src in DATA, both filled with random words.
    fn oc1597_copy_setup(c: &mut Cpu, seed: u32, entry: u32, count: u32) -> Events {
        let mut r = machine(c, seed, entry as u64, 0);
        let mode = schedule(c, &mut r, seed, entry);
        fill(c, &mut r, DATA + 0x1000, 32);
        fill(c, &mut r, DATA + 0x2000, 32);
        c.g[A0][0] = (DATA + 0x1000) as u64;
        c.g[A1][0] = (DATA + 0x2000) as u64;
        c.g[A2][0] = count as u64;
        mode
    }

    #[test]
    fn kcopy_3_impl_matches() {
        check_events(
            ee::s15::f_0042c4d0__generated,
            kcopy_3_impl,
            200,
            &|c, seed| {
                let n = if seed % 7 == 0 { 0 } else { 1 + seed % 20 };
                oc1597_copy_setup(c, seed, 0x0042_c4d0, n)
            },
        );
    }

    #[test]
    fn kcopy_4_impl_matches() {
        check_events(
            ee::s15::f_0042c780__generated,
            kcopy_4_impl,
            200,
            &|c, seed| {
                // a2 = words * 4 + (0..3): the shift drops the low bits.
                let words = if seed % 7 == 0 { 0 } else { 1 + seed % 9 };
                oc1597_copy_setup(c, seed, 0x0042_c780, words * 4 + seed % 4)
            },
        );
    }

    #[test]
    fn kcopy_5_impl_matches() {
        check_events(
            ee::s15::f_0042cb78__generated,
            kcopy_5_impl,
            200,
            &|c, seed| {
                let words = if seed % 7 == 0 { 0 } else { 1 + seed % 9 };
                oc1597_copy_setup(c, seed, 0x0042_cb78, words * 4 + seed % 4)
            },
        );
    }

    #[test]
    fn rcmp_codec_dtor_matches() {
        check(
            ee::s15::f_0042e278__generated,
            rcmp_codec_dtor,
            200,
            &|c, seed| {
                oc1597_install_delete();
                let mut r = machine(c, seed, 0x0042_e278, 1);
                c.bus.write32(OC1597_DELETE_CTRL, OC1597_MAGIC);
                // D_00509434[0] is the delete fake: a call through the table.
                c.bus.write32(0x0050_9434, 0x0031_7e50);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = ((r.next_u32() & !1) | (seed & 1)) as u64;
            },
        );
    }

    #[test]
    fn rcmp_system_dtor_matches() {
        check(
            ee::s15::f_0042e498__generated,
            rcmp_system_dtor,
            200,
            &|c, seed| {
                oc1597_install_delete();
                let mut r = machine(c, seed, 0x0042_e498, 1);
                c.bus.write32(OC1597_DELETE_CTRL, OC1597_MAGIC);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = ((r.next_u32() & !1) | (seed & 1)) as u64;
            },
        );
    }
}
