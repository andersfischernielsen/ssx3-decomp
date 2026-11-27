//! Staging module oc1082 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1082`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::call::{tail_wrapper, Target};
use crate::events::back_edge_unwinds;
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_c518, ui_thread_destroy, "UIThread_Destroy"),
    (
        0x0039_c820,
        ui_screen_replace_thread,
        "UIScreen_ReplaceThread",
    ),
    (0x0039_cd90, ui_screen_thread_nop_a, "UIScreen_ThreadNopA"),
    (0x0039_cd98, ui_screen_thread_nop_b, "UIScreen_ThreadNopB"),
    (
        0x0039_d008,
        ui_screen_create_object_by_id,
        "UIScreen_CreateObjectById",
    ),
    (0x0039_d948, c_ui_scroll_load, "cUIScroll_Load"),
    (0x0039_df28, c_ui_slider_dtor, "cUISlider_dtor"),
    (
        0x0039_dfe8,
        c_ui_slider_handle_input,
        "cUISlider_HandleInput",
    ),
];

/// `void UIThread_Destroy(UIThread* t)` (0x39c518): unlinks the thread from
/// its list, then, for a non-null thread, calls the method at `+0xc` of the
/// object at `t+8` with `this = t + delta` (delta at `+8` of that object).
pub fn ui_thread_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_7740, 0x0039_c52c)?;
        let null = c.g[S0][0] == 0;
        c.g[A1][0] = 0x3;
        if !null {
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            let method = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0039_c548)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void UIScreen_ReplaceThread(UIScreen* s, u16 flags)` (0x39c820): destroys
/// the screen's primary thread (if any), then starts a new thread with the
/// flags and a zero argument.
pub fn ui_screen_replace_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0] & 0xffff;
        call_ptr(c, 0x0039_c7c0, 0x0039_c83c)?;
        let none = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !none {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0039_c518, 0x0039_c84c)?;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0039_c870, 0x0039_c85c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void UIScreen_ThreadNopA(void)` (0x39cd90): returns at once.
pub fn ui_screen_thread_nop_a(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void UIScreen_ThreadNopB(void)` (0x39cd98): returns at once.
pub fn ui_screen_thread_nop_b(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void* UIScreen_CreateObjectById(UIScreen* s, u32 id)` (0x39d008): walks
/// the screen's element list at `s+0x38` (count at +0, elements from +4 with
/// the stride as a halfword at +2 and the id at +4). On a match it creates
/// the object, stores the id at its `+0x38` and returns it; otherwise null.
pub fn ui_screen_create_object_by_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x38)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        let empty = c.g[V1][0] == 0;
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        'b068: {
            'b064: {
                if empty {
                    break 'b064;
                }
                loop {
                    'b04c: {
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x4)).lw(c);
                        if c.g[V0][0] != c.g[S0][0] {
                            c.g[A0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x38)).lw(c);
                            break 'b04c;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[A3][0];
                        call_ptr(c, 0x0039_d078, 0x0039_d044)?;
                        let id = c.g[S0][0];
                        Addr((c.g[V0][0] as u32).wrapping_add(0x38)).sw(c, id);
                        break 'b068;
                    }
                    c.cycles += 6;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x2)).lhu(c);
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                    c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if !more {
                        break 'b064;
                    }
                    if back_edge_unwinds(c, 0x0039_d030) {
                        return Ok(());
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cUIScroll_Load(void* s)` (0x39d948): forwards to `UIObject_Load`.
pub fn c_ui_scroll_load(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0039_fe00), 0x0039_d958);
}

/// `void cUISlider_dtor(cUISlider* self, int flags)` (0x39df28): sets the
/// slider's vtable word, calls the method of each of the two sub-objects at
/// `+0x88` and `+0x8c` that exist, then runs `UIObject_dtor(self, flags)`.
pub fn c_ui_slider_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x490000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4348));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let vtbl = c.g[V0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtbl);
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x88)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 6;
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            let method = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0039_df6c)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8c)).lw(c);
        let none2 = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !none2 {
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            let method = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0039_df8c)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_fc48, 0x0039_df98)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int cUISlider_HandleInput(cUISlider* self, UIObject* obj)` (0x39dfe8):
/// asks the object for its state; a first method answering non-zero handles
/// the input in the slider's own thread path, a zero answer goes to a second
/// method. Returns 1, or 0 when the second method answers 0.
pub fn c_ui_slider_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0039_ff50, 0x0039_e00c)?;
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x98)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x9c)).lw(c);
        if c.g[A1][0] != 0 {
            c.g[S2][0] = c.g[A1][0];
        }
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0039_e028)?;
        'l1: {
            'l2: {
                'l3: {
                    'l4: {
                        if c.g[V0][0] == 0 {
                            c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x8)).lw(c);
                            break 'l4;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x74)).lw(c);
                        let no_threads = (c.g[V0][0] as i64) <= 0;
                        c.g[A1][0] = 0x8;
                        if no_threads {
                            break 'l2;
                        }
                        'b064: {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0039_ff78, 0x0039_e044)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[V0][0] = 0xffff;
                            if c.g[A1][0] == c.g[V0][0] {
                                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x74)).lw(c);
                                break 'b064;
                            }
                            c.cycles += 4;
                            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x5c)).lw(c);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x0039_c870, 0x0039_e060)?;
                            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x74)).lw(c);
                        }
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x5c)).lw(c);
                        c.g[A2][0] = 0x9;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        break 'l3;
                    }
                    // Reached when the first method answered zero.
                    c.cycles += 6;
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xa0)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xa4)).lw(c);
                    let target2 = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target2, 0x0039_e088)?;
                    let zero = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if zero {
                        break 'l1;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x74)).lw(c);
                    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x78)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let no_more = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x9;
                    if no_more {
                        break 'l2;
                    }
                    'b0d0: {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0039_ff78, 0x0039_e0b0)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        if c.g[A1][0] == c.g[V0][0] {
                            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x74)).lw(c);
                            break 'b0d0;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x5c)).lw(c);
                        c.g[A2][0] = 0x1;
                        call_ptr(c, 0x0039_c870, 0x0039_e0cc)?;
                        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x74)).lw(c);
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x5c)).lw(c);
                    c.g[A2][0] = 0x9;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                }
                // After the thread path: the slider's value, then two methods.
                c.cycles += 13;
                let value = c.g[V0][0];
                Addr((c.g[S0][0] as u32).wrapping_add(0x74)).sw(c, value);
                c.g[A3][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xd0)).lw(c);
                c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x98)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x9c)).lw(c);
                let target3 = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target3, 0x0039_e0fc)?;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                c.g[A1][0] = 0x1;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
                let target4 = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target4, 0x0039_e114)?;
                break 'l2;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the gated fakes answer only while it holds the magic.
    const OC1082_CTRL: u32 = DATA + 0x2e_0000;
    const OC1082_MAGIC: u32 = 0x1082_0001;
    /// Mailbox of forced v0 values, one word per gated fake (`K_*`).
    const OC1082_RES: u32 = DATA + 0x2e_0010;
    const K_UNLINK: u32 = 0;
    const K_PRIMARY: u32 = 1;
    const K_START: u32 = 2;
    const K_CREATE: u32 = 3;
    const K_LOAD: u32 = 4;
    const K_DTOR: u32 = 5;
    const K_STATE: u32 = 6;
    const K_EVENT: u32 = 7;
    const OC1082_ID: u32 = 0x1234;

    const NODE: u32 = DATA + 0x1000;
    const OBJ: u32 = DATA + 0x1100;

    fn oc1082_res(k: u32) -> u32 {
        OC1082_RES + 4 * k
    }

    /// The gated fake body: with the gate on, log the call and answer the
    /// mailbox word `k` in v0 (zero included); with it off, run the body the
    /// slot held before (`prev`), or the generated one.
    fn oc1082_gated(c: &mut Cpu, prev: &AtomicUsize, generated: fn(&mut Cpu), target: u32, k: u32) {
        if c.bus.read32(OC1082_CTRL) != OC1082_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc == ra {
            c.g[V0][0] = sx(c.bus.read32(oc1082_res(k)));
        }
    }

    static OC1082_PREV_UNLINK: AtomicUsize = AtomicUsize::new(0);
    static OC1082_PREV_PRIMARY: AtomicUsize = AtomicUsize::new(0);
    static OC1082_PREV_START: AtomicUsize = AtomicUsize::new(0);
    static OC1082_PREV_CREATE: AtomicUsize = AtomicUsize::new(0);
    static OC1082_PREV_LOAD: AtomicUsize = AtomicUsize::new(0);
    static OC1082_PREV_DTOR: AtomicUsize = AtomicUsize::new(0);
    static OC1082_PREV_STATE: AtomicUsize = AtomicUsize::new(0);
    static OC1082_PREV_EVENT: AtomicUsize = AtomicUsize::new(0);

    fn oc1082_fake_unlink(c: &mut Cpu) {
        oc1082_gated(
            c,
            &OC1082_PREV_UNLINK,
            ee::s13::f_00397740__generated,
            0x0039_7740,
            K_UNLINK,
        );
    }
    fn oc1082_fake_primary(c: &mut Cpu) {
        oc1082_gated(
            c,
            &OC1082_PREV_PRIMARY,
            ee::s13::f_0039c7c0__generated,
            0x0039_c7c0,
            K_PRIMARY,
        );
    }
    fn oc1082_fake_start(c: &mut Cpu) {
        oc1082_gated(
            c,
            &OC1082_PREV_START,
            ee::s13::f_0039c870__generated,
            0x0039_c870,
            K_START,
        );
    }
    fn oc1082_fake_create(c: &mut Cpu) {
        oc1082_gated(
            c,
            &OC1082_PREV_CREATE,
            ee::s13::f_0039d078__generated,
            0x0039_d078,
            K_CREATE,
        );
    }
    fn oc1082_fake_load(c: &mut Cpu) {
        oc1082_gated(
            c,
            &OC1082_PREV_LOAD,
            ee::s13::f_0039fe00__generated,
            0x0039_fe00,
            K_LOAD,
        );
    }
    fn oc1082_fake_dtor(c: &mut Cpu) {
        oc1082_gated(
            c,
            &OC1082_PREV_DTOR,
            ee::s13::f_0039fc48__generated,
            0x0039_fc48,
            K_DTOR,
        );
    }
    fn oc1082_fake_state(c: &mut Cpu) {
        oc1082_gated(
            c,
            &OC1082_PREV_STATE,
            ee::s13::f_0039ff50__generated,
            0x0039_ff50,
            K_STATE,
        );
    }
    fn oc1082_fake_event(c: &mut Cpu) {
        oc1082_gated(
            c,
            &OC1082_PREV_EVENT,
            ee::s13::f_0039ff78__generated,
            0x0039_ff78,
            K_EVENT,
        );
    }

    /// Installs the gated fakes on their override slots (once per process).
    fn oc1082_hook() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397740),
            oc1082_fake_unlink,
            &OC1082_PREV_UNLINK,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C7C0),
            oc1082_fake_primary,
            &OC1082_PREV_PRIMARY,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc1082_fake_start,
            &OC1082_PREV_START,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D078),
            oc1082_fake_create,
            &OC1082_PREV_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039FE00),
            oc1082_fake_load,
            &OC1082_PREV_LOAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039FC48),
            oc1082_fake_dtor,
            &OC1082_PREV_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039FF50),
            oc1082_fake_state,
            &OC1082_PREV_STATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039FF78),
            oc1082_fake_event,
            &OC1082_PREV_EVENT,
        );
    }

    /// A method address for an indirect call: the real leaf at `0x1406c0`
    /// (returns zero) when `zero`, else a logged fake that answers a hash.
    fn oc1082_method(zero: bool, seed: u32) -> u32 {
        if zero {
            0x0014_06c0
        } else {
            FAKE + 0x10 * (seed % 4)
        }
    }

    fn oc1082_delta(r: &mut Rng) -> u16 {
        (r.next_u32() as i16 as i32 & !3) as u16
    }

    /// A UIThread node at `NODE` whose object at `OBJ` holds the delta and the method.
    fn oc1082_thread(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(NODE + 8, OBJ);
        let delta = oc1082_delta(r);
        c.bus.write16(OBJ + 8, delta);
        c.bus.write32(OBJ + 0xc, oc1082_method(seed % 3 == 0, seed));
    }

    #[test]
    fn ui_thread_destroy_matches() {
        oc1082_hook();
        check(
            ee::s13::f_0039c518__generated,
            ui_thread_destroy,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_c518, 3);
                c.bus.write32(OC1082_CTRL, OC1082_MAGIC);
                oc1082_thread(c, &mut r, seed);
                c.g[A0][0] = if seed % 5 == 0 { 0 } else { NODE as u64 };
            },
        );
    }

    #[test]
    fn ui_screen_replace_thread_matches() {
        oc1082_hook();
        check(
            ee::s13::f_0039c820__generated,
            ui_screen_replace_thread,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_c820, 4);
                c.bus.write32(OC1082_CTRL, OC1082_MAGIC);
                oc1082_thread(c, &mut r, seed);
                let primary = if seed % 4 == 0 { 0 } else { NODE };
                c.bus.write32(oc1082_res(K_PRIMARY), primary);
                let start_v0 = r.next_u32();
                c.bus.write32(oc1082_res(K_START), start_v0);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                let flags = r.next_u32();
                c.g[A1][0] = flags as u64;
            },
        );
    }

    #[test]
    fn ui_screen_thread_nops_match() {
        check(
            ee::s13::f_0039cd90__generated,
            ui_screen_thread_nop_a,
            50,
            &|c, seed| {
                machine(c, seed, 0x0039_cd90, 0);
            },
        );
        check(
            ee::s13::f_0039cd98__generated,
            ui_screen_thread_nop_b,
            50,
            &|c, seed| {
                machine(c, seed, 0x0039_cd98, 0);
            },
        );
    }

    #[test]
    fn ui_screen_create_object_by_id_matches() {
        oc1082_hook();
        check(
            ee::s13::f_0039d008__generated,
            ui_screen_create_object_by_id,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_d008, 3);
                c.bus.write32(OC1082_CTRL, OC1082_MAGIC);
                let screen = DATA + 0x3000;
                let base = DATA + 0x3100;
                c.bus.write32(screen + 0x38, base);
                c.bus.write32(base, seed % 5);
                for i in 0..4u32 {
                    let e = base + 4 + 0x10 * i;
                    c.bus.write16(e + 2, 0x10);
                    let id = if r.next_u32() % 2 == 0 {
                        OC1082_ID
                    } else {
                        0x9999
                    };
                    c.bus.write32(e + 4, id);
                }
                c.bus.write32(oc1082_res(K_CREATE), DATA + 0x3400);
                c.g[A0][0] = screen as u64;
                let wanted = if seed % 2 == 0 { OC1082_ID } else { 0x5678 };
                c.g[A1][0] = wanted as u64;
            },
        );
    }

    #[test]
    fn c_ui_scroll_load_matches() {
        oc1082_hook();
        check(
            ee::s13::f_0039d948__generated,
            c_ui_scroll_load,
            100,
            &|c, seed| {
                machine(c, seed, 0x0039_d948, 2);
                c.bus.write32(OC1082_CTRL, OC1082_MAGIC);
                c.bus.write32(oc1082_res(K_LOAD), seed);
            },
        );
    }

    #[test]
    fn c_ui_slider_dtor_matches() {
        oc1082_hook();
        check(
            ee::s13::f_0039df28__generated,
            c_ui_slider_dtor,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_df28, 4);
                c.bus.write32(OC1082_CTRL, OC1082_MAGIC);
                let obj = DATA + 0x4000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                let slots = [
                    (0x88u32, DATA + 0x4100, DATA + 0x4300),
                    (0x8cu32, DATA + 0x4200, DATA + 0x4400),
                ];
                for (i, (off, p, q)) in slots.into_iter().enumerate() {
                    if seed % (3 + i as u32) == 0 {
                        c.bus.write32(obj + off, 0);
                    } else {
                        c.bus.write32(obj + off, p);
                        c.bus.write32(p + 8, q);
                        let delta = oc1082_delta(&mut r);
                        c.bus.write16(q + 8, delta);
                        c.bus
                            .write32(q + 0xc, oc1082_method(seed % 2 == 0, seed + i as u32));
                    }
                }
            },
        );
    }

    #[test]
    fn c_ui_slider_handle_input_matches() {
        oc1082_hook();
        const THIS: u32 = DATA + 0x5000;
        const OBJ_IN: u32 = DATA + 0x5400;
        const VT: u32 = DATA + 0x5800;
        const STATE: u32 = DATA + 0x5c00;
        const S2_CELL: u32 = DATA + 0x5e00;
        const OWNER: u32 = DATA + 0x6400;
        const A3_OBJ: u32 = DATA + 0x6800;
        const VT3: u32 = DATA + 0x6c00;
        const Q: u32 = DATA + 0x7000;
        check(
            ee::s13::f_0039dfe8__generated,
            c_ui_slider_handle_input,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0039_dfe8, 4);
                c.bus.write32(OC1082_CTRL, OC1082_MAGIC);
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = OBJ_IN as u64;
                c.bus.write32(oc1082_res(K_STATE), STATE);
                let event = if (seed / 2) % 2 == 0 { 0xffff } else { 0x42 };
                c.bus.write32(oc1082_res(K_EVENT), event);
                // The state's +0x14 names the s2 cell, or the global at gp+0x2968.
                let s2_named = if seed % 4 == 0 { 0 } else { S2_CELL };
                c.bus.write32(STATE + 0x14, s2_named);
                let cell = if s2_named != 0 {
                    S2_CELL
                } else {
                    GAME_GP + 0x2968
                };
                c.bus.write32(cell, Q);
                let d1 = oc1082_delta(&mut r);
                let d2 = oc1082_delta(&mut r);
                let d3 = oc1082_delta(&mut r);
                let d4 = oc1082_delta(&mut r);
                c.bus.write32(OBJ_IN + 8, VT);
                c.bus.write16(VT + 0x98, d1);
                c.bus.write32(VT + 0x9c, oc1082_method(seed % 3 == 1, seed));
                c.bus.write16(VT + 0xa0, d2);
                c.bus
                    .write32(VT + 0xa4, oc1082_method((seed / 3) % 2 == 0, seed + 1));
                let field74 = [0xffff_ffffu32, 0, 1, 2, 5][seed as usize % 5];
                c.bus.write32(THIS + 0x74, field74);
                let field78 = [0u32, 1, 3][(seed / 5) as usize % 3];
                c.bus.write32(THIS + 0x78, field78);
                c.bus.write32(THIS + 0x5c, OWNER);
                c.bus.write32(OWNER + 0xd0, A3_OBJ);
                c.bus.write32(A3_OBJ + 8, VT3);
                c.bus.write16(VT3 + 0x98, d3);
                c.bus.write32(VT3 + 0x9c, FAKE + 0x40);
                c.bus.write16(Q + 0x10, d4);
                c.bus.write32(Q + 0x14, FAKE + 0x50);
            },
        );
    }
}
