//! Staging module oc1024 (wave wave262): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1024`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0021_1b10,
        ov_screen_profile_save_pda_shutdown,
        "OVScreenProfileSavePDA_Shutdown",
    ),
    (
        0x0021_1b38,
        ov_screen_profile_save_pda_on_widget_notify,
        "OVScreenProfileSavePDA_OnWidgetNotify",
    ),
    (
        0x0021_2208,
        ov_screen_autosave_update,
        "OVScreenAutosave_Update",
    ),
    (
        0x0021_2508,
        ov_screen_autosave_shutdown,
        "OVScreenAutosave_Shutdown",
    ),
    (0x0028_a230, snd_load_queue_flush, "SndLoadQueue_Flush"),
    (0x0028_bb10, snd_bank_man_dtor, "SndBankMan_dtor"),
];

/// `void OVScreenProfileSavePDA_Shutdown(OVScreenProfileSavePDA* self)`
/// (0x211b10): shuts the memory-card state base down and clears the
/// field at +0x1a4.
pub fn ov_screen_profile_save_pda_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001d_58b8, 0x0021_1b24)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1a4), 0u64);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int OVScreenProfileSavePDA_OnWidgetNotify(self, a1, int key)` (0x211b38):
/// keys 6 and 8 ask the selected memory-card device (a virtual call on the
/// manager's object, +0x434 / +0x428) whether it is actionable (0x101 when
/// it is, else 0x100); key 9 returns 0x100, anything else 0x101.
pub fn ov_screen_profile_save_pda_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        Callee {
            addr: 0x0022_7f80,
            f: ee::s05::f_00227f80,
        }
        .call(c, 0x0021_1b50)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 0x8;
        let is_8 = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[S0][0] < 0x9u64) as u64;
        let mut ask_device = is_8;
        if !is_8 {
            c.cycles += 2;
            let ge_9 = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x6;
            if !ge_9 {
                c.cycles += 2;
                let is_6 = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0x101;
                if is_6 {
                    ask_device = true;
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x10);
                    ov_pda_notify_epilogue(c, frame);
                    return Ok(());
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x9;
                let is_9 = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0x101;
                if is_9 {
                    c.cycles += 1;
                    c.g[V0][0] = 0x100;
                    c.cycles += 1;
                } else {
                    c.cycles += 2;
                }
                frame.restore(c, S0, 0x10);
                ov_pda_notify_epilogue(c, frame);
                return Ok(());
            }
        }
        debug_assert!(ask_device);
        c.cycles += 9;
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x434));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x428));
        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0021_1ba8)?;
        let actionable = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0x101;
        if !actionable {
            c.cycles += 1;
            c.g[V0][0] = 0x100;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        ov_pda_notify_epilogue(c, frame);
        Ok(())
    });
}

/// The shared exit of `OVScreenProfileSavePDA_OnWidgetNotify`.
fn ov_pda_notify_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void OVScreenAutosave_Update(OVScreenAutosave* self)` (0x212208): the
/// autosave state machine. State 1 starts the save (writeSetFileType, state
/// 3); state 3 ends it (state 6) once the selected device is no longer
/// plugged in / valid; then the base screen's update runs.
pub fn ov_screen_autosave_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0022_7f80,
            f: ee::s05::f_00227f80,
        }
        .call(c, 0x0021_2228)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
        c.g[V0][0] = 0x1;
        'tail: {
            let at_check = if c.g[S1][0] != c.g[V0][0] {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                true
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                if c.g[V0][0] != 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                    true
                } else {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[A1][0];
                    c.g[A2][0] = 0u64;
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0023_cae8, 0x0021_2258)?;
                    c.g[V0][0] = 0x3;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f0), c.g[S1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[S1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1dc), c.g[S1][0]);
                    false
                }
            };
            if at_check {
                c.cycles += 3;
                c.g[V0][0] = 0x3;
                let not_saving = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0xe0;
                if not_saving {
                    break 'tail;
                }
                'device_ok: {
                    'device_bad: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
                        c.mult(0, A0, A0, V0);
                        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x350));
                        let unplugged = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.g[V1][0];
                        if !unplugged {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x344));
                            if c.g[V1][0] == 0u64 {
                                c.g[V0][0] = 0x1;
                                break 'device_ok;
                            }
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x348));
                            if c.g[V1][0] != 0u64 {
                                break 'tail;
                            }
                            break 'device_bad;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                }
                c.cycles += 5;
                c.g[V1][0] = 0x6;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1e4), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1dc), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), 0u64);
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_8de0, 0x0021_22cc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void OVScreenAutosave_Shutdown(OVScreenAutosave* self)` (0x212508): base
/// shutdown, then by the state at +0x22c: 1 tells the card manager to stop,
/// 6 and 7 open a popup (resource 0x4c / 0x58) and hand it to the popup
/// handlers, 0 rebuilds the front-end objects and re-runs the init chain.
pub fn ov_screen_autosave_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x1;
        call_ptr(c, 0x001d_58b8, 0x0021_2528)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x22c));
        let not_1 = c.g[V1][0] != c.g[S2][0];
        c.g[V0][0] = 0x6;
        'out: {
            if !not_1 {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                call_ptr(c, 0x0023_0488, 0x0021_2544)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            'popup_done: {
                'popup_tail: {
                    'not_6: {
                        c.cycles += 2;
                        let ne = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x7;
                        if ne {
                            break 'not_6;
                        }
                        c.cycles += 11;
                        c.g[A1][0] = 0x0047_0000;
                        c.g[A0][0] = 0x4c;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1e70));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x0031_7d70, 0x0021_256c)?;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0019_4738, 0x0021_2578)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        break 'popup_tail;
                    }
                    c.cycles += 2;
                    let ne = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = 0x0047_0000;
                    if ne {
                        break 'popup_done;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = 0x58;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1e80));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x0031_7d70, 0x0021_259c)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x001f_3700, 0x0021_25a8)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                }
                c.cycles += 5;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f400, 0x0021_25b8)?;
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            c.cycles += 2;
            let state_set = c.g[V1][0] != 0u64;
            frame.restore(c, S0, 0x30);
            if state_set {
                break 'out;
            }
            c.cycles += 40;
            call_ptr(c, 0x0014_dc80, 0x0021_25d0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014_dd58, 0x0021_25dc)?;
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dc80, 0x0021_25e4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014_dd58, 0x0021_25f0)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x4;
            call_ptr(c, 0x0014_4df0, 0x0021_2600)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xc;
            call_ptr(c, 0x0014_51e8, 0x0021_260c)?;
            c.g[A1][0] = 0u64;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0014_6d98, 0x0021_2618)?;
            c.g[V1][0] = 0x0053_0000;
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[S2][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x5c08), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0021_2638)?;
            c.g[A1][0] = 0x0047_0000;
            c.g[A0][0] = 0x8;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1e90));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x0031_7d70, 0x0021_2650)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0023_2720, 0x0021_2658)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0031_6ef8, 0x0021_2664)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SndLoadQueue_Flush(SndLoadQueue* self)` (0x28a230): finishes the
/// current load until the busy word (+4) clears (waiting with a sleep
/// syscall between rounds), then frees the buffer at +0x1cc.
/// A resumable function: entered at any other pc it runs the generated body.
pub fn snd_load_queue_flush(c: &mut Cpu) {
    if c.pc != 0x0028_a230 {
        return ee::s07::f_0028a230__generated(c);
    }
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x1;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let mut first = true;
        loop {
            if !first {
                // 0x28a248: wait a moment, then finish again.
                c.cycles += 4;
                c.g[A0][0] = 0x2;
                call_ptr(c, 0x003e_4fb0, 0x0028_a250)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
            }
            first = false;
            c.cycles += 5;
            Callee {
                addr: 0x0028_9df0,
                f: ee::s07::f_00289df0,
            }
            .call(c, 0x0028_a260)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            if c.g[V0][0] == 0u64 {
                break;
            }
            if back_edge_unwinds(c, 0x0028_a248) {
                return Ok(());
            }
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1cc));
        if c.g[A0][0] == 0u64 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 5;
            call_ptr(c, 0x0031_7e98, 0x0028_a280)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1d0), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1cc), 0u64);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SndBankMan_dtor(SndBankMan* self, int flags)` (0x28bb10): resets the
/// vtables (flags 0: also a stack copy of the sound system's vtable with the
/// this-adjust fixed up), clears the global bank manager, destroys the bank
/// array and the base sub-objects, and per flag bit 2 / bit 1 destroys the
/// sound system / frees `self`.
pub fn snd_bank_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S1, 0x60);
        c.g[V1][0] = 0x0048_0000;
        frame.save(c, S0, 0x70);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3b48));
        frame.save(c, S3, 0x40);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3ad8));
        frame.save(c, S4, 0x30);
        c.g[A2][0] = 0x0048_0000;
        frame.save_ra(c, 0x20);
        c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3ab8));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xab0), c.g[V0][0]);
        c.g[T0][0] = c.g[A1][0];
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1d8));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1d4), c.g[V1][0]);
        let flags_set = c.g[T0][0] != 0u64;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
        if !flags_set {
            // The sound system's vtable, copied to the stack with the
            // this-adjust of entry 1 fixed up.
            c.cycles += 16;
            c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
            c.g[A2][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x3ab8));
            c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1d8));
            let spv = sp(c);
            c.sd(spv.wrapping_add(0x8), c.g[V0][0]);
            c.sd(spv.wrapping_add(0x10), c.g[V1][0]);
            c.sd(spv, c.g[A2][0]);
            c.sd(spv.wrapping_add(0x18), c.g[A0][0]);
            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[SP][0]);
            c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1d8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xad0));
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sh(spv.wrapping_add(0x8), c.g[V1][0]);
        }
        // The bank array (+0xacc): destroy the elements back to front.
        c.cycles += 6;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xacc));
        c.g[S4][0] = c.g[T0][0] & 0x1;
        c.sw((c.g[GP][0] as u32).wrapping_add(0x524), 0u64);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1d8));
        let no_array = c.g[A1][0] == 0u64;
        c.g[S3][0] = c.g[T0][0] & 0x2;
        if !no_array {
            c.cycles += 6;
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x10));
            c.g[V0][0] = 0x60;
            c.mult(0, A0, V1, V0);
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xacc));
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x60));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0028_b278,
                        f: ee::s07::f_0028b278,
                    }
                    .call(c, 0x0028_bbe4)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xacc));
                    if c.g[V0][0] != c.g[S0][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x60));
                        if event_due(c) {
                            deliver_event_here(c, 0x0028_bbd8);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xacc));
                    break;
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x0028_bbfc)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x002a_ce40, 0x0028_bc08)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0u64;
        Callee {
            addr: 0x0028_a148,
            f: ee::s07::f_0028a148,
        }
        .call(c, 0x0028_bc14)?;
        if c.g[S3][0] != 0u64 {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1d8));
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x002a_dee8, 0x0028_bc28)?;
        }
        c.cycles += 2;
        let no_free = c.g[S4][0] == 0u64;
        frame.restore(c, S0, 0x70);
        if !no_free {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0028_bc38)?;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, check_lohi, elf_image, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (own address and magic in the machine's
    /// RAM: other tests' machines run the real bodies).
    const OC1024_CTRL: u32 = DATA + 0x20_0000;
    const OC1024_MAGIC: u32 = 0x1024_f00d;
    /// Fixture objects (clear of the fakes' result pointers, `DATA..+0x40000`).
    const OC1024_THIS: u32 = DATA + 0x5_0000;
    const OC1024_MGR: u32 = DATA + 0x5_4000;

    /// Every faked callee: a gated fake in its override slot that logs the
    /// call (all registers) and returns the next forced result or a pointer.
    macro_rules! oc1024_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1024_CTRL) != OC1024_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                    oc1024_after(c, $addr);
                }
            )*
            /// Puts every fake in its override slot (once per process).
            fn oc1024_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    /// `SndLoadQueue_FinishCurrent` fake: each call lets the busy word (+4 of
    /// `s0`) count down by one.
    fn oc1024_after(c: &mut Cpu, target: u32) {
        if target == 0x0028_9df0 {
            let this = c.g[S0][0] as u32;
            let busy = c.bus.read32(this + 4);
            if busy != 0 {
                c.bus.write32(this + 4, busy - 1);
            }
        }
    }

    oc1024_fakes! {
        oc1024_fake_001d58b8, 0x001d_58b8, ee::s04::OVERRIDE_001D58B8, ee::s04::f_001d58b8__generated;
        oc1024_fake_00227f80, 0x0022_7f80, ee::s05::OVERRIDE_00227F80, ee::s05::f_00227f80__generated;
        oc1024_fake_0023cae8, 0x0023_cae8, ee::s06::OVERRIDE_0023CAE8, ee::s06::f_0023cae8__generated;
        oc1024_fake_001d8de0, 0x001d_8de0, ee::s04::OVERRIDE_001D8DE0, ee::s04::f_001d8de0__generated;
        oc1024_fake_00230488, 0x0023_0488, ee::s06::OVERRIDE_00230488, ee::s06::f_00230488__generated;
        oc1024_fake_00317d70, 0x0031_7d70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated;
        oc1024_fake_00194738, 0x0019_4738, ee::s02::OVERRIDE_00194738, ee::s02::f_00194738__generated;
        oc1024_fake_001f3700, 0x001f_3700, ee::s04::OVERRIDE_001F3700, ee::s04::f_001f3700__generated;
        oc1024_fake_0039f400, 0x0039_f400, ee::s13::OVERRIDE_0039F400, ee::s13::f_0039f400__generated;
        oc1024_fake_0014dc80, 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc1024_fake_0014dd58, 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc1024_fake_00144df0, 0x0014_4df0, ee::s01::OVERRIDE_00144DF0, ee::s01::f_00144df0__generated;
        oc1024_fake_001451e8, 0x0014_51e8, ee::s01::OVERRIDE_001451E8, ee::s01::f_001451e8__generated;
        oc1024_fake_00146d98, 0x0014_6d98, ee::s01::OVERRIDE_00146D98, ee::s01::f_00146d98__generated;
        oc1024_fake_00232720, 0x0023_2720, ee::s06::OVERRIDE_00232720, ee::s06::f_00232720__generated;
        oc1024_fake_00316ef8, 0x0031_6ef8, ee::s10::OVERRIDE_00316EF8, ee::s10::f_00316ef8__generated;
        oc1024_fake_003e4fb0, 0x003e_4fb0, ee::s14::OVERRIDE_003E4FB0, ee::s14::f_003e4fb0__generated;
        oc1024_fake_00289df0, 0x0028_9df0, ee::s07::OVERRIDE_00289DF0, ee::s07::f_00289df0__generated;
        oc1024_fake_00317e98, 0x0031_7e98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated;
        oc1024_fake_0028b278, 0x0028_b278, ee::s07::OVERRIDE_0028B278, ee::s07::f_0028b278__generated;
        oc1024_fake_002ace40, 0x002a_ce40, ee::s08::OVERRIDE_002ACE40, ee::s08::f_002ace40__generated;
        oc1024_fake_0028a148, 0x0028_a148, ee::s07::OVERRIDE_0028A148, ee::s07::f_0028a148__generated;
        oc1024_fake_002adee8, 0x002a_dee8, ee::s08::OVERRIDE_002ADEE8, ee::s08::f_002adee8__generated;
        oc1024_fake_00317e50, 0x0031_7e50, ee::s10::OVERRIDE_00317E50, ee::s10::f_00317e50__generated;
    }

    /// Hooked fakes, scrambled registers, the gate on.
    fn oc1024_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1024_hook();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1024_CTRL, OC1024_MAGIC);
        r
    }

    /// Distinct call sequences (addresses) the generated body makes over the seeds.
    fn oc1024_paths(generated: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, generated);
            let n = c.bus.read32(fakes::LOG_COUNT);
            let seq: Vec<u32> = (0..n)
                .map(|i| c.bus.read32(fakes::LOG_START + i * 0x220))
                .collect();
            seen.insert((seq, c.cycles));
        }
        seen.len()
    }

    fn oc1024_shutdown_pda_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1024_machine(c, seed, 0x21_1b10, 1);
        c.bus.write32(OC1024_THIS + 0x1a4, r.next_u32());
        c.g[A0][0] = sx(OC1024_THIS);
        c.pc = 0x0021_1b10;
    }

    /// `OVScreenProfileSavePDA_Shutdown`: the base shutdown call, the field
    /// clear, and a callee unwind.
    #[test]
    fn ov_screen_profile_save_pda_shutdown_matches() {
        check(
            ee::s05::f_00211b10__generated,
            ov_screen_profile_save_pda_shutdown,
            40,
            &oc1024_shutdown_pda_setup,
        );
    }

    fn oc1024_pda_notify_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1024_machine(c, seed, 0x21_1b38, 2);
        let key = [6u32, 8, 9, 0, 5, 7, 10, 0x101, 3, 6, 8, 9, 4, 6][(seed % 14) as usize];
        let (obj, vt) = (DATA + 0x5_1000, DATA + 0x5_2000);
        // MCManager_Get is fake call 0: force the manager.
        c.bus.write32(fakes::LOG_RESULTS, OC1024_MGR);
        c.bus.write32(OC1024_MGR + 0x434, obj);
        c.bus.write32(OC1024_MGR + 0x428, r.next_u32());
        c.bus.write32(obj, vt);
        c.bus.write16(vt + 0x1b0, (r.next_u32() & 0xfffc) as u16);
        // The device answers non-zero (a fake) or zero (a real leaf).
        let leaf0 = 0x0014_06c0;
        c.bus.write32(
            vt + 0x1b4,
            if (seed / 14) % 2 == 0 {
                fakes::FAKE
            } else {
                leaf0
            },
        );
        c.g[A0][0] = r.next_u32() as u64;
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = key as u64;
        c.pc = 0x0021_1b38;
    }

    /// `OVScreenProfileSavePDA_OnWidgetNotify`: keys 6/8 (device actionable or
    /// not), 9, below and above, and callee unwinds.
    #[test]
    fn ov_screen_profile_save_pda_on_widget_notify_matches() {
        check(
            ee::s05::f_00211b38__generated,
            ov_screen_profile_save_pda_on_widget_notify,
            140,
            &oc1024_pda_notify_setup,
        );
        let paths = oc1024_paths(
            ee::s05::f_00211b38__generated,
            140,
            &oc1024_pda_notify_setup,
        );
        assert!(paths >= 5, "only {paths} paths");
    }

    fn oc1024_autosave_update_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1024_machine(c, seed, 0x21_2208, 3);
        c.bus.write32(fakes::LOG_RESULTS, OC1024_MGR);
        // (state, +0x1a8 word, device variant)
        let k = seed % 16;
        let (state, f1a8, variant) = match k {
            0..=2 => (1u32, 0u32, 0u32),
            3 => (1, 5, 0),
            4 => (0, 0, 0),
            5 => (6, 0, 0),
            _ => (3, 0, k - 6),
        };
        let this = OC1024_THIS;
        c.bus.write32(this + 0x1c0, state);
        c.bus.write32(this + 0x1a8, f1a8);
        for off in [0x1f0u32, 0x1dc, 0x1e4] {
            c.bus.write32(this + off, r.next_u32());
        }
        // The selected device record (0xe0 bytes each): plugged (+0x350),
        // valid (+0x344), formatted (+0x348).
        let idx = (seed / 16) % 3;
        c.bus.write32(OC1024_MGR + 0x428, idx);
        let dev = OC1024_MGR + idx * 0xe0;
        let (plugged, valid, formatted) =
            [(0u32, 0u32, 0u32), (1, 0, 0), (1, 1, 1), (1, 1, 0)][(variant % 4) as usize];
        c.bus.write32(dev + 0x350, plugged);
        c.bus.write32(dev + 0x344, valid);
        c.bus.write32(dev + 0x348, formatted);
        c.g[A0][0] = sx(this);
        c.pc = 0x0021_2208;
    }

    /// `OVScreenAutosave_Update`: state 1 (start save, or already started),
    /// state 3 with the device unplugged / invalid / formatted / not, other
    /// states, and callee unwinds. LO/HI too (the device index multiply).
    #[test]
    fn ov_screen_autosave_update_matches() {
        check_lohi(
            ee::s05::f_00212208__generated,
            ov_screen_autosave_update,
            192,
            &oc1024_autosave_update_setup,
        );
        let paths = oc1024_paths(
            ee::s05::f_00212208__generated,
            192,
            &oc1024_autosave_update_setup,
        );
        assert!(paths >= 5, "only {paths} paths");
    }

    fn oc1024_autosave_shutdown_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1024_machine(c, seed, 0x21_2508, 14);
        let state = [1u32, 6, 7, 0, 3, 0, 6, 7, 1, 2][(seed % 10) as usize];
        let this = OC1024_THIS;
        c.bus.write32(this + 0x22c, state);
        c.bus.write32(this + 0x10, r.next_u32());
        c.g[A0][0] = sx(this);
        c.pc = 0x0021_2508;
    }

    /// `OVScreenAutosave_Shutdown`: state 1, 6, 7, 0 (the long rebuild chain,
    /// ending in a virtual call through a zero vtable word) and others, with
    /// callee unwinds at random positions.
    #[test]
    fn ov_screen_autosave_shutdown_matches() {
        check(
            ee::s05::f_00212508__generated,
            ov_screen_autosave_shutdown,
            200,
            &oc1024_autosave_shutdown_setup,
        );
        let paths = oc1024_paths(
            ee::s05::f_00212508__generated,
            200,
            &oc1024_autosave_shutdown_setup,
        );
        assert!(paths >= 5, "only {paths} paths");
    }

    fn oc1024_flush_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1024_machine(c, seed, 0x28_a230, 8);
        let this = DATA + 0x5_0000;
        c.bus.write32(this + 4, seed % 4);
        c.bus.write32(
            this + 0x1cc,
            if (seed / 4) % 2 == 1 {
                DATA + 0x5_8000
            } else {
                0
            },
        );
        c.bus.write32(this + 0x1d0, r.next_u32());
        c.g[A0][0] = sx(this);
        c.pc = 0x0028_a230;
        if seed % 10 == 9 {
            // A resume entry inside the function (after an unwind): the
            // frame is already open, the loop is over.
            let sp = crate::difftest::SP - 0x20;
            c.g[SP][0] = sx(sp);
            c.bus.write64(sp, crate::difftest::RA as u64);
            c.bus
                .write64(sp + 0x10, (r.next_u32() as u64) << 32 | r.next_u32() as u64);
            c.bus
                .write64(sp + 0x18, (r.next_u32() as u64) << 32 | r.next_u32() as u64);
            c.g[S0][0] = sx(this);
            c.pc = if (seed / 10) % 2 == 0 {
                0x0028_a26c
            } else {
                0x0028_a278
            };
        }
    }

    /// `SndLoadQueue_Flush`: the busy word counting down over 0..3 rounds
    /// (the wait call between them), with and without a buffer to free, callee
    /// unwinds, and resumes at the tail blocks.
    #[test]
    fn snd_load_queue_flush_matches() {
        check(
            ee::s07::f_0028a230__generated,
            snd_load_queue_flush,
            160,
            &oc1024_flush_setup,
        );
        let paths = oc1024_paths(ee::s07::f_0028a230__generated, 160, &oc1024_flush_setup);
        assert!(paths >= 6, "only {paths} paths");
    }

    fn oc1024_dtor_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1024_machine(c, seed, 0x28_bb10, 8);
        elf_image(c);
        let this = OC1024_THIS;
        let sound = DATA + 0x5_6000;
        let flags = seed % 4;
        c.bus.write32(this + 0x1d8, sound);
        // The bank array: none, or `count` 0x60-byte elements behind a 0x10 header.
        let array = DATA + 0x5_8010;
        let count = (seed / 4) % 4;
        let has_array = (seed / 16) % 4 != 0;
        c.bus
            .write32(this + 0xacc, if has_array { array } else { 0 });
        c.bus.write32(array - 0x10, count);
        c.bus.write32(sound + 4, r.next_u32());
        c.bus.write32(this + 0xab0, r.next_u32());
        c.bus.write32(this + 0x1d4, r.next_u32());
        c.g[A0][0] = sx(this);
        c.g[A1][0] = flags as u64;
        c.pc = 0x0028_bb10;
    }

    /// `SndBankMan_dtor`: flags 0..3 (the stack vtable copy only for 0), no
    /// array, an empty array and arrays of 1..3 banks, callee unwinds.
    #[test]
    fn snd_bank_man_dtor_matches() {
        check_lohi(
            ee::s07::f_0028bb10__generated,
            snd_bank_man_dtor,
            128,
            &oc1024_dtor_setup,
        );
        let paths = oc1024_paths(ee::s07::f_0028bb10__generated, 128, &oc1024_dtor_setup);
        assert!(paths >= 8, "only {paths} paths");
    }
}
