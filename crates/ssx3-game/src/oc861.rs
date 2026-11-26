//! Staging module oc861 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc861`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `FEScreenSelMulti_ctor` (0x1a3050): construct the UI state base, then set
/// its state kind (`+0xc` = 0x16) and vtable (`+0x8` = 0x469248); returns `this`.
pub fn fe_screen_sel_multi_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x001a_3064)?; // cUIState_cUIState
        c.g[V0][0] = 0x16;
        c.g[V1][0] = 0x0047_0000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6db8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Load the widget's method slot for `cFEStateMPCircuit_OnWidgetAction`:
/// `a2 = widget->+0x18`, `a3 = *v0`, `v1 = a3->+4`, `a0 = (short)v1[hoff]`
/// (this adjustment), `v0 = v1[foff]` (method address).
fn fe_state_mp_circuit_widget_slot(c: &mut Cpu, hoff: u32, foff: u32) {
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(hoff));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(foff));
}

/// `cFEStateMPCircuit_OnWidgetAction(state, widget, action)` (0x1a33a0):
/// action 5 or 6 calls a widget method of the state's engine (slot +0x24 or
/// +0x2c) and, if it returns a screen state, pushes it on the engine's state
/// stack; action 9 looks the selected list-box entry up in the screen and
/// starts its frame, then runs the empty hook.
pub fn c_fe_state_mp_circuit_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        'exit: {
            'tail: {
                if c.g[A0][0] == 0 {
                    break 'tail;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x6;
                let is6 = c.g[A2][0] == c.g[V0][0];
                c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                if is6 {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    fe_state_mp_circuit_widget_slot(c, 0x28, 0x2c);
                } else {
                    c.cycles += 2;
                    let ge7 = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x5;
                    if ge7 {
                        c.cycles += 3;
                        c.g[V0][0] = 0x9;
                        if c.g[A2][0] != c.g[V0][0] {
                            c.cycles += 2;
                            frame.restore(c, S0, 0x10);
                            break 'exit;
                        }
                        // Action 9: the list box's selected entry.
                        c.cycles += 11;
                        call_ptr(c, 0x0039_a708, 0x001a_3454)?; // UIListBox_GetSelectedData
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0031_7670, 0x001a_345c)?; // StrHash
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x001a_3468)?; // UIScreen_LookupValue
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let no_frame = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] &= 0xffff;
                        if !no_frame {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x0039_c870, 0x001a_3484)?; // UIScreen_StartThread
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x001a_34a8, 0x001a_348c)?; // cFEStateMPCircuit_EmptyHook
                        break 'tail;
                    }
                    c.cycles += 2;
                    if c.g[A2][0] != c.g[V0][0] {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x10);
                        break 'exit;
                    }
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.cycles += 7;
                    fe_state_mp_circuit_widget_slot(c, 0x20, 0x24);
                }
                // Call the widget method; a non-null state is pushed.
                c.cycles += 4;
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x001a_3430)?;
                let none = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if none {
                    break 'tail;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f400, 0x001a_3444)?; // UIStateStack_Push
                frame.restore(c, S0, 0x10);
                break 'exit;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateMPCircuit_EmptyInitHook` (0x1a34a0): returns at once.
pub fn c_fe_state_mp_circuit_empty_init_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateMPCircuit_EmptyHook` (0x1a34a8): returns at once.
pub fn c_fe_state_mp_circuit_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateMPCircuit_GetMaxRiderStateFlags` (0x1a34b8): for the two human
/// riders of the game's player interface, sum the "peak locked" counts over
/// three peaks (low byte) and return the largest sum.
pub fn c_fe_state_mp_circuit_get_max_rider_state_flags(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S4, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        c.g[S5][0] = 0;
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0);
        c.g[S6][0] = 0x0100_0000;
        call_ptr(c, 0x0014_dc80, 0x001a_34ec)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014_dd58, 0x001a_34f8)?;
        c.g[S4][0] = c.g[V0][0];
        loop {
            c.cycles += 8;
            c.g[A1][0] = (0u64 < c.g[S0][0]) as u64;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0014_6e98, 0x001a_350c)?; // cBENewPlayerInterface_getHumanRider
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[S2][0] = c.g[V0][0];
            c.g[S1][0] = 0;
            c.g[A2][0] = 0x1;
            c.g[S0][0] = 0x0200_0000;
            loop {
                c.cycles += 10;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0014_6008, 0x001a_352c)?; // cBENewPlayerInterface_isPeakLocked1
                c.g[V1][0] = c.g[S0][0];
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A2][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                c.g[S1][0] = c.g[V0][0] & 0xff;
                c.g[V0][0] = ((c.g[A2][0] as i64) < 3i64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001a_3520);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[S3][0] as u32) << 24);
            c.g[V1][0] = (c.g[S5][0] < c.g[S1][0]) as u64;
            c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
            let more = c.g[V0][0] != 0;
            if c.g[V1][0] != 0 {
                c.g[S5][0] = c.g[S1][0];
            }
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001a_3500);
                }
                continue;
            }
            break;
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S5][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `FEAsyncManager_CancelAll(this)` (0x1a3698): cancel the five async slots
/// (`FEAsyncManager_Cancel(this, i)` for i in 0..5).
pub fn fe_async_manager_cancel_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001a_38c0, 0x001a_36c0)?; // FEAsyncManager_Cancel
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 5i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001a_36b8);
                }
                continue;
            }
            break;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_3050,
        fe_screen_sel_multi_ctor,
        "FEScreenSelMulti_ctor",
    ),
    (
        0x001a_33a0,
        c_fe_state_mp_circuit_on_widget_action,
        "cFEStateMPCircuit_OnWidgetAction",
    ),
    (
        0x001a_34a0,
        c_fe_state_mp_circuit_empty_init_hook,
        "cFEStateMPCircuit_EmptyInitHook",
    ),
    (
        0x001a_34a8,
        c_fe_state_mp_circuit_empty_hook,
        "cFEStateMPCircuit_EmptyHook",
    ),
    (
        0x001a_34b8,
        c_fe_state_mp_circuit_get_max_rider_state_flags,
        "cFEStateMPCircuit_GetMaxRiderStateFlags",
    ),
    (
        0x001a_3698,
        fe_async_manager_cancel_all,
        "FEAsyncManager_CancelAll",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc861 fakes (clear of the fake call log at DATA+0x100000).
    const OC861_CTRL: u32 = DATA + 0x20_0000;
    const OC861_MAGIC: u32 = 0x0c86_1001;
    /// Per-callee forced results: one word per fake (value + 1; 0 = none).
    const OC861_FORCE: u32 = DATA + 0x20_1000;
    /// Fixture objects.
    const OC861_STATE: u32 = DATA + 0x21_0000;
    const OC861_ENGINE: u32 = DATA + 0x21_1000;
    const OC861_OBJ: u32 = DATA + 0x21_2000;
    const OC861_VT: u32 = DATA + 0x21_3000;
    const OC861_WIDGET: u32 = DATA + 0x21_4000;
    const OC861_SCREEN: u32 = DATA + 0x21_5000;

    fn oc861_mail(addr: u32) -> u32 {
        OC861_FORCE + (addr & 0xfff) * 4
    }

    fn oc861_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc861_mail(addr), v.wrapping_add(1));
    }

    fn oc861_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc861_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 1) as u64;
        }
    }

    macro_rules! oc861_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC861_CTRL) != OC861_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc861_fake_call(c, $addr);
                }
            )*
            fn oc861_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc861_fakes!(
        (
            oc861_fake_39e2a0,
            OC861_PREV_39E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039_e2a0
        ),
        (
            oc861_fake_39f400,
            OC861_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc861_fake_39a708,
            OC861_PREV_39A708,
            ee::s13::OVERRIDE_0039A708,
            ee::s13::f_0039a708__generated,
            0x0039_a708
        ),
        (
            oc861_fake_317670,
            OC861_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc861_fake_39c778,
            OC861_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778
        ),
        (
            oc861_fake_39c870,
            OC861_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc861_fake_14dc80,
            OC861_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc861_fake_14dd58,
            OC861_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc861_fake_146e98,
            OC861_PREV_146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98
        ),
        (
            oc861_fake_146008,
            OC861_PREV_146008,
            ee::s01::OVERRIDE_00146008,
            ee::s01::f_00146008__generated,
            0x0014_6008
        ),
        (
            oc861_fake_1a38c0,
            OC861_PREV_1A38C0,
            ee::s03::OVERRIDE_001A38C0,
            ee::s03::f_001a38c0__generated,
            0x001a_38c0
        ),
    );

    /// A machine whose callees are the gated fakes (`calls`: how many fake calls
    /// the function makes at most, for the unwind index).
    fn oc861_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc861_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC861_CTRL, OC861_MAGIC);
        r
    }

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    #[test]
    fn fe_screen_sel_multi_ctor_matches() {
        check(
            ee::s03::f_001a3050__generated,
            fe_screen_sel_multi_ctor,
            12,
            &|c, seed| {
                let _ = oc861_base(c, seed, 0x1a_3050, 1);
                c.g[A0][0] = sx(OC861_STATE);
            },
        );
    }

    /// Every action path: 5, 6 (method call; null and non-null result), 9 (with
    /// and without a frame for the entry), other actions and a null widget.
    #[test]
    fn c_fe_state_mp_circuit_on_widget_action_matches() {
        check(
            ee::s03::f_001a33a0__generated,
            c_fe_state_mp_circuit_on_widget_action,
            72,
            &|c, seed| {
                let mut r = oc861_base(c, seed, 0x1a_33a0, 5);
                let action = [5u64, 6, 9, 3, 12, 9, 7, 5][(seed % 8) as usize];
                let widget = if (seed / 8) % 5 == 4 { 0 } else { OC861_WIDGET };
                w(c, OC861_STATE + 0x10, OC861_ENGINE);
                w(c, OC861_STATE + 0x40, OC861_SCREEN);
                w(c, OC861_ENGINE, OC861_OBJ);
                w(c, OC861_OBJ + 4, OC861_VT);
                for off in [0x20u32, 0x28] {
                    w(c, OC861_VT + off, r.next_u32() & 0xffff);
                }
                // Method: a real leaf returning 0, or a fake returning a state.
                let method = if (seed / 8) % 2 == 0 {
                    0x0014_06c0
                } else {
                    fakes::FAKE + 0x40
                };
                w(c, OC861_VT + 0x24, method);
                w(c, OC861_VT + 0x2c, method);
                w(c, OC861_WIDGET + 0x18, r.next_u32());
                if (seed / 8) % 3 == 2 {
                    oc861_force(c, 0x0039_c778, 0xffff);
                }
                c.g[A0][0] = sx(OC861_STATE);
                c.g[A1][0] = sx(widget);
                c.g[A2][0] = action;
            },
        );
    }

    #[test]
    fn c_fe_state_mp_circuit_empty_init_hook_matches() {
        check(
            ee::s03::f_001a34a0__generated,
            c_fe_state_mp_circuit_empty_init_hook,
            4,
            &|c, seed| {
                let mut r = Rng(0x1a34_a0 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn c_fe_state_mp_circuit_empty_hook_matches() {
        check(
            ee::s03::f_001a34a8__generated,
            c_fe_state_mp_circuit_empty_hook,
            4,
            &|c, seed| {
                let mut r = Rng(0x1a34_a8 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    /// Two riders, three peaks each; results from the fakes (hash values, so
    /// both "new maximum" and "not larger" occur), some forced small.
    #[test]
    fn c_fe_state_mp_circuit_get_max_rider_state_flags_matches() {
        check(
            ee::s03::f_001a34b8__generated,
            c_fe_state_mp_circuit_get_max_rider_state_flags,
            24,
            &|c, seed| {
                let _ = oc861_base(c, seed, 0x1a_34b8, 10);
                match seed % 3 {
                    0 => oc861_force(c, 0x0014_6008, 0),
                    1 => oc861_force(c, 0x0014_6008, 1 + seed % 5),
                    _ => {}
                }
            },
        );
    }

    #[test]
    fn fe_async_manager_cancel_all_matches() {
        check(
            ee::s03::f_001a3698__generated,
            fe_async_manager_cancel_all,
            16,
            &|c, seed| {
                let _ = oc861_base(c, seed, 0x1a_3698, 5);
                c.g[A0][0] = sx(OC861_STATE);
            },
        );
    }
}
