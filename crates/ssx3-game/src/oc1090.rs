//! Staging module oc1090 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1090`.

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
    (0x003a_65d8, ui_state_flags_ctor, "UIStateFlags_ctor"),
    (0x003a_65e8, ui_text_flags_ctor, "UITextFlags_ctor"),
    (
        0x003a_6600,
        c_ui_text_scroll_flags_ctor,
        "cUITextScrollFlags_ctor",
    ),
    (0x003a_6618, ui_vector_flags_ctor, "UIVectorFlags_ctor"),
    (0x003a_6648, ui_engine_global_ctors, "UIEngine_GlobalCtors"),
    (0x003a_6668, ui_engine_global_dtors, "UIEngine_GlobalDtors"),
    (0x003b_7de8, sndstrm_queue_file, "SNDSTRM_QueueFile"),
    (
        0x003b_7e10,
        sndstrm_queue_file_offset,
        "SNDSTRM_QueueFileOffset",
    ),
];

/// Direct call targets of this module.
const UIENGINE_STATIC_INIT_DESTROY_CALLEE: Callee = Callee {
    addr: 0x003a_3f48,
    f: ee::s13::UIEngine_StaticInitDestroy,
};
const SNDSTRM_QUEUE_REQUEST_CALLEE: Callee = Callee {
    addr: 0x003b_76e8,
    f: ee::s13::SNDSTRM_QueueRequest,
};

/// Initial flag halfword of a `tAttrState` (stored at +0 by its ctor).
const UI_STATE_FLAGS_INIT: u64 = 0x218;
/// `tAttrText` flags: bit 2 set, then bits 0-2 and 7 kept.
const UI_TEXT_FLAGS_SET: u64 = 0x4;
const UI_TEXT_FLAGS_KEEP: u64 = 0x87;
/// `tAttrTextScroll` flags: bits 1, 2, 6 and 7 kept.
const UI_TEXT_SCROLL_FLAGS_KEEP: u64 = 0xc6;
/// `tAttrVector` flags word: bits 0 and 1 cleared, bits 17 and up cleared.
const UI_VECTOR_FLAGS_CLEAR_LOW: u64 = !1;
const UI_VECTOR_FLAGS_CLEAR_HIGH: u64 = !2;
const UI_VECTOR_FLAGS_LIMIT: u64 = 0x1_ffff;

/// `tAttrState::tAttrState(this)` (0x3a65d8): stores the halfword 0x218 at
/// +0 and returns `this`.
pub fn ui_state_flags_ctor(c: &mut Cpu) {
    c.cycles += 4;
    let this = c.g[A0][0];
    c.g[V0][0] = this;
    c.g[V1][0] = UI_STATE_FLAGS_INIT;
    let init = c.g[V1][0];
    Addr(this as u32).sh(c, init);
    ret(c);
}

/// `tAttrText::tAttrText(attr)` (0x3a65e8): the flag byte at +0 becomes
/// `(flags | 4) & 0x87`; returns `attr`.
pub fn ui_text_flags_ctor(c: &mut Cpu) {
    c.cycles += 6;
    let attr = c.g[A0][0];
    let flags = Addr(attr as u32).lbu(c);
    c.g[V1][0] = (flags | UI_TEXT_FLAGS_SET) & UI_TEXT_FLAGS_KEEP;
    c.g[V0][0] = attr;
    let v = c.g[V1][0];
    Addr(attr as u32).sb(c, v);
    ret(c);
}

/// `tAttrTextScroll::tAttrTextScroll(attr)` (0x3a6600): the flag byte at +0
/// becomes `flags & 0xc6`; returns `attr`.
pub fn c_ui_text_scroll_flags_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let attr = c.g[A0][0];
    let flags = Addr(attr as u32).lbu(c);
    c.g[V1][0] = flags & UI_TEXT_SCROLL_FLAGS_KEEP;
    c.g[V0][0] = attr;
    let v = c.g[V1][0];
    Addr(attr as u32).sb(c, v);
    ret(c);
}

/// `tAttrVector::tAttrVector(this)` (0x3a6618): the flag word at +0 keeps
/// only bits 2-16 (`word & 0x1fffc`), stored as a word; returns `this`.
pub fn ui_vector_flags_ctor(c: &mut Cpu) {
    c.cycles += 11;
    let this = c.g[A0][0];
    let word = Addr(this as u32).lw(c);
    c.g[A2][0] = UI_VECTOR_FLAGS_CLEAR_LOW;
    c.g[A3][0] = UI_VECTOR_FLAGS_CLEAR_HIGH;
    c.g[A1][0] = UI_VECTOR_FLAGS_LIMIT;
    let kept = word & c.g[A2][0] & c.g[A3][0] & c.g[A1][0];
    c.g[V1][0] = kept;
    c.g[V0][0] = this;
    Addr(this as u32).sw(c, kept);
    ret(c);
}

/// `UIEngine_GlobalCtors` (0x3a6648): `UIEngine_StaticInitDestroy(1, 0xffff)`.
pub fn ui_engine_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 1;
        c.g[A1][0] = 0xffff;
        UIENGINE_STATIC_INIT_DESTROY_CALLEE.call(c, 0x003a_665c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIEngine_GlobalDtors` (0x3a6668): `UIEngine_StaticInitDestroy(0, 0xffff)`.
pub fn ui_engine_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        c.g[A1][0] = 0xffff;
        UIENGINE_STATIC_INIT_DESTROY_CALLEE.call(c, 0x003a_667c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SNDSTRM_QueueFile(handle, a, file)` (0x3b7de8): `SNDSTRMI_queue(handle,
/// a, 0, file, 2)`, the file request passed in `a3`.
pub fn sndstrm_queue_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[T0][0] = 2;
        SNDSTRM_QUEUE_REQUEST_CALLEE.call(c, 0x003b_7e00)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SNDSTRM_QueueFileOffset(handle, hold, buffer, offset)` (0x3b7e10):
/// `SNDSTRMI_queue(handle, hold, buffer + offset, 0, 1)`.
pub fn sndstrm_queue_file_offset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        let buffer = (c.g[A2][0] as u32).wrapping_add(c.g[A3][0] as u32);
        c.g[A2][0] = sx(buffer);
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[T0][0] = 1;
        SNDSTRM_QUEUE_REQUEST_CALLEE.call(c, 0x003b_7e28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const SEEDS: u32 = 200;

    /// Gates of the callee fakes: each answers only while its own word holds
    /// `OC1090_MAGIC` (otherwise the body it replaced runs). Placed above the
    /// pointer area so random fixture pointers never hit them.
    const OC1090_STATIC_CTRL: u32 = DATA + 0x0020_0000;
    const OC1090_SNDSTRM_CTRL: u32 = DATA + 0x0020_0010;
    const OC1090_MAGIC: u32 = 0x1090_c0de;

    static OC1090_PREV_STATIC: AtomicUsize = AtomicUsize::new(0);
    static OC1090_PREV_SNDSTRM: AtomicUsize = AtomicUsize::new(0);

    /// `UIEngine_StaticInitDestroy`: logs the call and returns.
    fn oc1090_fake_static_init(c: &mut Cpu) {
        if c.bus.read32(OC1090_STATIC_CTRL) != OC1090_MAGIC {
            return fakes::fall_back(c, &OC1090_PREV_STATIC, ee::s13::f_003a3f48__generated);
        }
        fakes::fake_call(c, 0x003a_3f48);
    }

    /// `SNDSTRM_QueueRequest`: logs the call and returns.
    fn oc1090_fake_sndstrm_queue(c: &mut Cpu) {
        if c.bus.read32(OC1090_SNDSTRM_CTRL) != OC1090_MAGIC {
            return fakes::fall_back(c, &OC1090_PREV_SNDSTRM, ee::s13::f_003b76e8__generated);
        }
        fakes::fake_call(c, 0x003b_76e8);
    }

    fn oc1090_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A3F48),
            oc1090_fake_static_init,
            &OC1090_PREV_STATIC,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B76E8),
            oc1090_fake_sndstrm_queue,
            &OC1090_PREV_SNDSTRM,
        );
    }

    #[test]
    fn ui_state_flags_ctor_matches() {
        check(
            ee::s13::f_003a65d8__generated,
            ui_state_flags_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1090_0001, 0);
                let this = fakes::ptr(&mut r);
                c.g[A0][0] = this as u64;
                c.bus.write32(this, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_text_flags_ctor_matches() {
        check(
            ee::s13::f_003a65e8__generated,
            ui_text_flags_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1090_0002, 0);
                let attr = fakes::ptr(&mut r);
                c.g[A0][0] = attr as u64;
                c.bus.write8(attr, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn c_ui_text_scroll_flags_ctor_matches() {
        check(
            ee::s13::f_003a6600__generated,
            c_ui_text_scroll_flags_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1090_0003, 0);
                let attr = fakes::ptr(&mut r);
                c.g[A0][0] = attr as u64;
                c.bus.write8(attr, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn ui_vector_flags_ctor_matches() {
        check(
            ee::s13::f_003a6618__generated,
            ui_vector_flags_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1090_0004, 0);
                let this = fakes::ptr(&mut r);
                c.g[A0][0] = this as u64;
                c.bus.write32(this, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_engine_global_ctors_matches() {
        oc1090_install_fakes();
        check(
            ee::s13::f_003a6648__generated,
            ui_engine_global_ctors,
            SEEDS,
            &|c, seed| {
                fakes::machine(c, seed, 0x1090_0005, 1);
                c.bus.write32(OC1090_STATIC_CTRL, OC1090_MAGIC);
            },
        );
    }

    #[test]
    fn ui_engine_global_dtors_matches() {
        oc1090_install_fakes();
        check(
            ee::s13::f_003a6668__generated,
            ui_engine_global_dtors,
            SEEDS,
            &|c, seed| {
                fakes::machine(c, seed, 0x1090_0006, 1);
                c.bus.write32(OC1090_STATIC_CTRL, OC1090_MAGIC);
            },
        );
    }

    #[test]
    fn sndstrm_queue_file_matches() {
        oc1090_install_fakes();
        check(
            ee::s13::f_003b7de8__generated,
            sndstrm_queue_file,
            SEEDS,
            &|c, seed| {
                fakes::machine(c, seed, 0x1090_0007, 1);
                c.bus.write32(OC1090_SNDSTRM_CTRL, OC1090_MAGIC);
            },
        );
    }

    #[test]
    fn sndstrm_queue_file_offset_matches() {
        oc1090_install_fakes();
        check(
            ee::s13::f_003b7e10__generated,
            sndstrm_queue_file_offset,
            SEEDS,
            &|c, seed| {
                fakes::machine(c, seed, 0x1090_0008, 1);
                c.bus.write32(OC1090_SNDSTRM_CTRL, OC1090_MAGIC);
            },
        );
    }
}
