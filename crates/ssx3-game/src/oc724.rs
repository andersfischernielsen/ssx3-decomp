//! Staging module oc724 (wave wave240): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc724`.

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
    (0x0021_ccc8, ui_icon_flags_ctor, "UIIconFlags_ctor"),
    (
        0x0029_3b90,
        ssx_audio_calc_surface_loop_remote_group2,
        "SSXAudioSystem_CalcSurfaceLoopRemoteGroup2",
    ),
    (0x0039_c870, ui_screen_start_thread, "UIScreen_StartThread"),
    (0x0037_1548, m_emitter_alloc_data, "mEmitter_AllocData"),
];

/// `UIIconFlags::ctor` (0x21ccc8): keeps the high nibble of the flags byte
/// at `*a0` and clears the low one. Returns the flags pointer in v0.
pub fn ui_icon_flags_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let flags = c.g[A0][0];
    let masked = Addr(flags as u32).lbu(c) & 0xf0;
    c.g[V1][0] = masked;
    c.g[V0][0] = flags;
    c.g[V1][0] = masked;
    Addr(flags as u32).sb(c, masked);
    ret(c);
}

/// `SSXAudioSystem::CalcSurfaceLoopRemoteGroup2` (0x293b90): a 0x10 frame
/// around one call to `RailGlideProgram` (0x293948); its arguments pass
/// through unchanged.
pub fn ssx_audio_calc_surface_loop_remote_group2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0029_3948,
            f: ee::s07::f_00293948,
        }
        .call(c, 0x0029_3ba0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIScreen::StartThread` (0x39c870): `(self a0, thread type a1 & 0xffff,
/// a2)`. When `a2 == 0` the screen's primary thread (if any) is destroyed
/// first. Allocates a 0x100-byte UI thread, initialises its fields, pushes
/// it on the screen's thread list (`self + 0x18`) and returns it in v0.
pub fn ui_screen_start_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0] & 0xffff;
        frame.save_ra(c, 0);
        if c.g[S1][0] == 0 {
            c.cycles += 4;
            Callee {
                addr: 0x0039_c7c0,
                f: ee::s13::f_0039c7c0,
            }
            .call(c, 0x0039_c8a0)?;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = 0x490000;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[V0][0];
                Callee {
                    addr: 0x0039_c518,
                    f: ee::s13::f_0039c518,
                }
                .call(c, 0x0039_c8b0)?;
                c.cycles += 1;
                c.g[A1][0] = 0x490000;
            }
        } else {
            c.cycles += 1;
            c.g[A1][0] = 0x490000;
        }
        c.cycles += 49;
        c.g[A0][0] = 0x14;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3e10));
        call_ptr(c, 0x0031_7d70, 0x0039_c8c8)?;
        c.g[S0][0] = c.g[V0][0];
        let node = c.g[S0][0] as u32;
        c.g[V1][0] = 1;
        c.g[V0][0] = 0xffff;
        let v = c.g[V1][0];
        Addr(node.wrapping_add(0x12)).sb(c, v);
        let v = c.g[V0][0];
        Addr(node.wrapping_add(0x10)).sh(c, v);
        c.g[A2][0] = 0xffff_ffff_feff_0000;
        c.g[A2][0] |= 0xffff;
        c.g[A3][0] = 0xffff_ffff_fdff_0000;
        c.g[V0][0] = Addr(node.wrapping_add(0x10)).lw(c);
        c.g[A3][0] |= 0xffff;
        c.g[T0][0] = 0xffff;
        c.g[A0][0] = 1;
        c.g[V0][0] &= c.g[A2][0];
        c.g[T1][0] = 0x0100_0000;
        c.g[V0][0] &= c.g[A3][0];
        c.g[V1][0] = (c.g[S1][0] < 0x1) as u64;
        let v = c.g[V0][0];
        Addr(node.wrapping_add(0x10)).sw(c, v);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 25);
        c.g[V0][0] = 0x490000;
        let v = c.g[A0][0];
        Addr(node.wrapping_add(0x12)).sb(c, v);
        let v = c.g[T0][0];
        Addr(node.wrapping_add(0x10)).sh(c, v);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x46a0));
        let v = c.g[V0][0];
        Addr(node.wrapping_add(0x8)).sw(c, v);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = Addr(node.wrapping_add(0x10)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        let v = c.g[S3][0];
        Addr(node.wrapping_add(0xc)).sh(c, v);
        c.g[V0][0] &= c.g[A2][0];
        let v = c.g[T0][0];
        Addr(node.wrapping_add(0xe)).sh(c, v);
        c.g[V0][0] |= c.g[T1][0];
        let v = c.g[S0][0];
        Addr(node.wrapping_add(0x4)).sw(c, v);
        c.g[V0][0] &= c.g[A3][0];
        let v = c.g[S0][0];
        Addr(node).sw(c, v);
        c.g[V0][0] |= c.g[V1][0];
        let v = c.g[V0][0];
        Addr(node.wrapping_add(0x10)).sw(c, v);
        Callee {
            addr: 0x0039_7910,
            f: ee::s13::f_00397910,
        }
        .call(c, 0x0039_c958)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `mEmitter::AllocData` (0x371548): allocates the emitter's data block
/// through `DynamicEmitter_AllocData` (self + 0x204), then `count` slots of
/// 4 bytes at `self + 0x178` via the pool allocator (0x317e30). Stores the
/// block's result in the word at `self + 0x204`; returns -1 for count 0 and
/// the allocator's result otherwise.
pub fn m_emitter_alloc_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x204));
        Callee {
            addr: 0x0037_0c60,
            f: ee::s12::f_00370c60,
        }
        .call(c, 0x0037_1568)?;
        let self_ = c.g[S0][0] as u32;
        c.g[S1][0] = Addr(self_.wrapping_add(0x178)).lw(c);
        c.g[A1][0] = 0x490000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2690));
        c.g[A2][0] = 0;
        c.g[A0][0] = sx((c.g[S1][0] as u32) << 2);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0037_1584)?;
        let t = c.g[S1][0] == 0;
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
        if !t {
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 7;
        let v0 = c.g[V0][0];
        Addr(c.g[S2][0] as u32).sw(c, v0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA};

    #[test]
    fn ui_icon_flags_ctor_matches() {
        check(
            ee::s05::f_0021ccc8__generated,
            ui_icon_flags_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x0021_ccc8 ^ seed as u64);
                scramble(c, &mut r);
                let p = DATA + 0x100 + 0x10 * (seed % 16);
                c.g[A0][0] = sx(p);
                c.bus.write8(p, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn ssx_audio_calc_surface_loop_remote_group2_matches() {
        check(
            ee::s07::f_00293b90__generated,
            ssx_audio_calc_surface_loop_remote_group2,
            200,
            &|c, seed| {
                let _ = fakes::machine(c, seed, 0x0029_3b90, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
            },
        );
    }

    #[test]
    fn ui_screen_start_thread_matches() {
        check(
            ee::s13::f_0039c870__generated,
            ui_screen_start_thread,
            200,
            &|c, seed| {
                let mut r = Rng(0x0039_c870 ^ seed as u64);
                scramble(c, &mut r);
                crate::game::ai::computer::tests::rbc_test_allocator(c);
                let this = DATA + 0x6000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = if seed % 3 == 0 { 1 } else { 0 };
            },
        );
    }

    #[test]
    fn m_emitter_alloc_data_matches() {
        check(
            ee::s12::f_00371548__generated,
            m_emitter_alloc_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x0037_1548 ^ seed as u64);
                scramble(c, &mut r);
                crate::game::ai::computer::tests::rbc_test_allocator(c);
                let this = DATA + 0x7000;
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(this + 0x178, if seed % 4 == 0 { 0 } else { seed % 9 });
            },
        );
    }
}
