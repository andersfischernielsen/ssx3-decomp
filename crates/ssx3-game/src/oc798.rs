//! Staging module oc798 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc798`.

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
    (0x0039_7138, ui_engine_store_word, "UIEngine_StoreWord"),
    (0x0036_7340, tex_man_init_temp_heap, "TexMan_InitTempHeap"),
    (
        0x002a_dbf0,
        bank_monitor_install_tick_callback,
        "BankMonitor_InstallTickCallback",
    ),
    (
        0x0018_0280,
        feui_input_is_cam_rotate,
        "FEUIInput_IsCamRotate",
    ),
    (0x0018_0260, feui_input_is_cam_pan, "FEUIInput_IsCamPan"),
    (0x0018_0240, feui_input_is_cam_zoom, "FEUIInput_IsCamZoom"),
];

/// `void UIEngine_StoreWord(int* dst, int value)` (0x397138): `*dst = value`.
pub fn ui_engine_store_word(c: &mut Cpu) {
    c.cycles += 2;
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    ret(c);
}

/// `void TexMan_InitTempHeap(TexMan* this)` (0x367340): initialises the temp
/// VRAM heap at `this + 0x4350` (`VramHeap_Init(heap, this + 8)`).
pub fn tex_man_init_temp_heap(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4350));
        Callee {
            addr: 0x0036_5e40,
            f: ee::s12::VramHeap_Init,
        }
        .call(c, 0x0036_7354)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_InstallTickCallback(void)` (0x2adbf0): registers the
/// bank monitor's tick callback (0x2b0000 - 0x2668) with `SND_AddTickCallback`.
pub fn bank_monitor_install_tick_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x2b0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2668));
        call_ptr(c, 0x003b_5910, 0x002a_dc04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of the `FEUIInput_IsCam*` wrappers:
/// `InputBinding_EvalBool(this->binding (+0x18), action)`.
fn feui_input_eval_action(c: &mut Cpu, action: u64, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = action;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x18)).lw(c);
        call_ptr(c, 0x0032_0c48, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool FEUIInput::IsCamRotate(FEUIInput* this)` (0x180280): action 0x91.
pub fn feui_input_is_cam_rotate(c: &mut Cpu) {
    feui_input_eval_action(c, 0x91, 0x0018_0294);
}

/// `bool FEUIInput::IsCamPan(FEUIInput* this)` (0x180260): action 0x92.
pub fn feui_input_is_cam_pan(c: &mut Cpu) {
    feui_input_eval_action(c, 0x92, 0x0018_0274);
}

/// `bool FEUIInput::IsCamZoom(FEUIInput* this)` (0x180240): action 0x90.
pub fn feui_input_is_cam_zoom(c: &mut Cpu) {
    feui_input_eval_action(c, 0x90, 0x0018_0254);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA};
    use std::sync::atomic::AtomicUsize;

    pub(crate) const OC798_CTRL: u32 = DATA + 0x30_0000;
    pub(crate) const OC798_MAGIC: u32 = 0x0798_f4ce;
    const OC798_OBJ: u32 = DATA + 0x1000;

    pub(crate) fn oc798_arm(c: &mut Cpu) {
        c.bus.write32(OC798_CTRL, OC798_MAGIC);
    }

    macro_rules! oc798_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC798_CTRL) != OC798_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc798_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc798_fakes!(
        (
            oc798_fk_320c48,
            OC798_PREV_320C48,
            ee::s10::OVERRIDE_00320C48,
            ee::s10::f_00320c48__generated,
            0x0032_0c48
        ),
        (
            oc798_fk_365e40,
            OC798_PREV_365E40,
            ee::s12::OVERRIDE_00365E40,
            ee::s12::f_00365e40__generated,
            0x0036_5e40
        ),
        (
            oc798_fk_3b5910,
            OC798_PREV_3B5910,
            ee::s13::OVERRIDE_003B5910,
            ee::s13::f_003b5910__generated,
            0x003b_5910
        ),
    );

    /// `UIEngine_StoreWord`: stores `a1` at `a0`.
    #[test]
    fn ui_engine_store_word_matches() {
        check(
            ee::s13::f_00397138__generated,
            ui_engine_store_word,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x798a, 0);
                c.pc = 0x0039_7138;
                c.g[RA][0] = sx(crate::difftest::RA);
                c.g[A0][0] = sx(OC798_OBJ + 4 * (seed & 7));
                c.g[A1][0] = r.next_u32() as u64 | ((seed as u64) << 40);
            },
        );
    }

    /// `TexMan_InitTempHeap`: forwards to `VramHeap_Init`, including unwinds.
    #[test]
    fn tex_man_init_temp_heap_matches() {
        oc798_install_fakes();
        fakes::covered(
            ee::s12::f_00367340__generated,
            tex_man_init_temp_heap,
            60,
            &|c, seed| {
                machine(c, seed, 0x798b, 1);
                oc798_arm(c);
                c.pc = 0x0036_7340;
                c.g[RA][0] = sx(crate::difftest::RA);
                c.g[A0][0] = sx(OC798_OBJ + 0x10 * (seed & 3));
            },
        );
    }

    /// `BankMonitor_InstallTickCallback`: calls `SND_AddTickCallback`.
    #[test]
    fn bank_monitor_install_tick_callback_matches() {
        oc798_install_fakes();
        fakes::covered(
            ee::s08::f_002adbf0__generated,
            bank_monitor_install_tick_callback,
            60,
            &|c, seed| {
                machine(c, seed, 0x798c, 1);
                oc798_arm(c);
                c.pc = 0x002a_dbf0;
                c.g[RA][0] = sx(crate::difftest::RA);
            },
        );
    }

    fn oc798_feui_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        machine(c, seed, salt, 1);
        oc798_arm(c);
        c.pc = entry;
        c.g[RA][0] = sx(crate::difftest::RA);
        c.g[A0][0] = sx(OC798_OBJ);
        c.bus
            .write32(OC798_OBJ + 0x18, DATA + 0x2000 + 0x10 * (seed & 7));
    }

    /// `FEUIInput_IsCamRotate`.
    #[test]
    fn feui_input_is_cam_rotate_matches() {
        oc798_install_fakes();
        fakes::covered(
            ee::s02::f_00180280__generated,
            feui_input_is_cam_rotate,
            60,
            &|c, seed| oc798_feui_setup(c, seed, 0x798d, 0x0018_0280),
        );
    }

    /// `FEUIInput_IsCamPan`.
    #[test]
    fn feui_input_is_cam_pan_matches() {
        oc798_install_fakes();
        fakes::covered(
            ee::s02::f_00180260__generated,
            feui_input_is_cam_pan,
            60,
            &|c, seed| oc798_feui_setup(c, seed, 0x798e, 0x0018_0260),
        );
    }

    /// `FEUIInput_IsCamZoom`.
    #[test]
    fn feui_input_is_cam_zoom_matches() {
        oc798_install_fakes();
        fakes::covered(
            ee::s02::f_00180240__generated,
            feui_input_is_cam_zoom,
            60,
            &|c, seed| oc798_feui_setup(c, seed, 0x798f, 0x0018_0240),
        );
    }
}
