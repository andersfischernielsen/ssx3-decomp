//! Staging module oc1128 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1128`.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_5290, ea_thread_wakeup_alarm, "EAThread_WakeupAlarm"),
    (0x003e_5440, ea_thread_is_current, "EAThread_IsCurrent"),
    (0x003e_5498, ea_thread_get_priority, "EAThread_GetPriority"),
    (0x003e_5508, ea_thread_set_priority, "EAThread_SetPriority"),
    (0x003e_5580, ea_thread_is_finished, "EAThread_IsFinished"),
    (0x003e_55e0, ea_thread_wait_for_exit, "EAThread_WaitForExit"),
    (0x003e_5678, ea_thread_exit, "EAThread_Exit"),
    (
        0x003e_57d0,
        ea_timer_clear_callbacks,
        "EATimer_ClearCallbacks",
    ),
    (0x003e_57f8, ea_timer_add_callback, "EATimer_AddCallback"),
    (
        0x003e_58c8,
        ea_timer_remove_callback,
        "EATimer_RemoveCallback",
    ),
    (0x003e_5a10, ea_timer_service_for, "EATimer_ServiceFor"),
    (0x003e_6690, ea_timer_set_video_mode, "EATimer_SetVideoMode"),
];

mod callee {
    use super::ee;
    use crate::call::Callee;

    pub(crate) const EA_TIMER_GET_TICKS: Callee = Callee {
        addr: 0x003e_4e98,
        f: ee::s14::EATimer_GetTicks,
    };
    pub(crate) const EA_TIMER_SERVICE_CALLBACKS: Callee = Callee {
        addr: 0x003e_5928,
        f: ee::s14::EATimer_ServiceCallbacks,
    };
    pub(crate) const EA_MEM_SET: Callee = Callee {
        addr: 0x003e_6448,
        f: ee::s14::EA_MemSet,
    };
    pub(crate) const EA_THREAD_YIELD_TICKS: Callee = Callee {
        addr: 0x003e_52b0,
        f: ee::s14::THREAD_yieldticks,
    };
}

/// `void EAThread_WakeupAlarm(arg)` (0x3e5290): wake the alarm thread.
pub fn ea_thread_wakeup_alarm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_52a0);
        c.g[A0][0] = c.g[A2][0];
        let hook = 0x0042_4c50;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_52a0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAThread_IsCurrent(thread)` (0x3e5440): nonzero when the
/// caller runs on `thread` (or `thread` is null, matching anyone).
pub fn ea_thread_is_current(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_5454);
        c.g[S0][0] = c.g[A0][0];
        let hook = 0x0042_3c90;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_5454)?;
        let this = c.g[S0][0] != 0;
        c.g[A0][0] = c.g[V0][0];
        'b_3e5484: {
            'b_3e547c: {
                'b_3e5468: {
                    if this {
                        break 'b_3e5468;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = 0x450000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xdfc));
                    break 'b_3e547c;
                }
                c.cycles += 4;
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                if c.g[S0][0] == c.g[V0][0] {
                    c.g[V0][0] = 0x1;
                    break 'b_3e5484;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                break 'b_3e547c;
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0] ^ c.g[V0][0];
            c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
            break 'b_3e5484;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAThread_GetPriority(thread)` (0x3e5498): priority of the
/// thread, or of the caller when `thread` is null.
pub fn ea_thread_get_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x40);
        let given = c.g[A0][0] != 0;
        frame.save_ra(c, 0x30);
        'b_3e54cc: {
            'b_3e54b0: {
                if given {
                    break 'b_3e54b0;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x450000;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xdfc));
                break 'b_3e54cc;
            }
            c.cycles += 4;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                break 'b_3e54cc;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_54c8);
            let hook = 0x0042_3c90;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_54c8)?;
            c.g[A0][0] = c.g[V0][0];
            break 'b_3e54cc;
        }
        'b_3e54f8: {
            'b_3e54ec: {
                c.cycles += 4;
                c.g[V0][0] = 0x450000;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xde4));
                let no_main = c.g[V1][0] == 0;
                if no_main {
                    break 'b_3e54ec;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003e_54e4);
                c.g[A1][0] = c.g[SP][0];
                let hook = 0x0042_3cb0;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_54e4)?;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                break 'b_3e54f8;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_54f4);
            c.g[A1][0] = c.g[SP][0];
            let hook = 0x0042_3ca0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_54f4)?;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            break 'b_3e54f8;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAThread_WaitForExit(thread, timeout)` (0x3e55e0): wait for
/// the thread to finish, or until `timeout` ticks elapse.
pub fn ea_thread_wait_for_exit(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_10: u32 = 0x003e_5610;
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save_ra(c, 0x30);
        c.g[RA][0] = sx(0x003e_5600);
        c.g[S1][0] = c.g[A1][0];
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_5600)?;
        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        'l_3e5610: loop {
            'b_3e565c: {
                'b_3e5658: {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x003e_5618);
                    c.g[A1][0] = c.g[SP][0];
                    let hook = 0x0042_3ca0;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_5618)?;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = c.g[V1][0] & 0x10;
                    let dead = c.g[V0][0] != 0;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if dead {
                        break 'b_3e5658;
                    }
                    c.cycles += 2;
                    if c.g[V1][0] == 0 {
                        frame.restore(c, S0, 0x60);
                        break 'b_3e565c;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_5638);
                    c.g[A0][0] = 0;
                    callee::EA_THREAD_YIELD_TICKS.call(c, 0x003e_5638)?;
                    let infinite = c.g[S1][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if infinite {
                        if event_due(c) {
                            deliver_event_here(c, HEAD_10);
                        }
                        continue 'l_3e5610;
                    }
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003e_5648);
                    callee::EA_TIMER_GET_TICKS.call(c, 0x003e_5648)?;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let done = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if done {
                        if event_due(c) {
                            deliver_event_here(c, HEAD_10);
                        }
                        continue 'l_3e5610;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    break 'b_3e5658;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x60);
                break 'b_3e565c;
            }
            break 'l_3e5610;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[V0][0] ^ 0x10;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void EAThread_Exit()` (0x3e5678): stop this thread.
pub fn ea_thread_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_5688);
        let hook = 0x0042_3be0;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_5688)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EATimer_ClearCallbacks()` (0x3e57d0): drop all timer
/// callbacks.
pub fn ea_timer_clear_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x520000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1268));
        c.g[RA][0] = sx(0x003e_57ec);
        c.g[A2][0] = 0x100;
        callee::EA_MEM_SET.call(c, 0x003e_57ec)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EATimer_AddCallback(fn, mode, delay)` (0x3e57f8): register a
/// timer callback, reusing a slot for the same fn.
pub fn ea_timer_add_callback(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_40: u32 = 0x003e_5840;
        c.cycles += 5;
        c.g[T1][0] = c.g[A0][0];
        c.g[T0][0] = c.g[A1][0];
        c.g[A0][0] = 0xffffffffffffffff;
        let new_mode = c.g[T0][0] != c.g[A0][0];
        c.g[T4][0] = c.g[A2][0];
        'b_3e581c: {
            'b_3e5814: {
                if new_mode {
                    break 'b_3e5814;
                }
                c.cycles += 2;
                c.g[T0][0] = 0;
                break 'b_3e581c;
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            if c.g[T0][0] == 0 {
                c.g[T0][0] = c.g[V0][0];
            }
            break 'b_3e581c;
        }
        c.cycles += 9;
        c.g[V0][0] = 0x450000;
        c.g[T3][0] = 0x520000;
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe00));
        c.g[T5][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        c.g[T2][0] = 0xffffffffffffffff;
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
        c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x1268));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xe00), c.g[V1][0]);
        'l_3e5840: loop {
            'b_3e586c: {
                'b_3e5868: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                    if c.g[V0][0] == c.g[T1][0] {
                        c.g[A0][0] = c.g[A2][0];
                        break 'b_3e5868;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        break 'b_3e586c;
                    }
                    c.cycles += 2;
                    if c.g[A0][0] != c.g[T2][0] {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        break 'b_3e586c;
                    }
                    c.cycles += 2;
                    if c.g[A1][0] == 0 {
                        c.g[A0][0] = c.g[A2][0];
                        break 'b_3e5868;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    break 'b_3e5868;
                }
                c.cycles += 1;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                break 'b_3e586c;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 16) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
            if more {
                if event_due(c) {
                    deliver_event_here(c, HEAD_40);
                }
                continue 'l_3e5840;
            }
            break 'l_3e5840;
        }
        'b_3e58b4: {
            c.cycles += 3;
            c.g[V0][0] = 0xffffffffffffffff;
            let missing = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x1268));
            if missing {
                break 'b_3e58b4;
            }
            c.cycles += 12;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
            c.g[A1][0] = 0x450000;
            c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xdd0));
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw(c.g[A2][0] as u32, c.g[T1][0]);
            c.g[A1][0] = c.g[A3][0];
            c.sw((c.g[A3][0] as u32).wrapping_add(0x4), c.g[T0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[V1][0] = c.g[A1][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0);
            break 'b_3e58b4;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0xe00));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sw((c.g[T5][0] as u32).wrapping_add(0xe00), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

/// `void EATimer_RemoveCallback(fn)` (0x3e58c8): drop the callback
/// slot holding `fn`.
pub fn ea_timer_remove_callback(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_E8: u32 = 0x003e_58e8;
        c.cycles += 6;
        c.g[V0][0] = 0x520000;
        c.g[A2][0] = c.g[A0][0];
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x1268));
        c.g[A0][0] = c.g[V0][0];
        let present = c.g[V1][0] == c.g[A2][0];
        c.g[A1][0] = 0;
        'b_3e5920: {
            'b_3e5900: {
                if present {
                    break 'b_3e5900;
                }
                c.cycles += 2;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1268));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                'l_3e58e8: loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 16) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    if done {
                        break 'b_3e5920;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    if c.g[V0][0] != c.g[A2][0] {
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        if event_due(c) {
                            deliver_event_here(c, HEAD_E8);
                        }
                        continue 'l_3e58e8;
                    }
                    break 'b_3e5900;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 16) as u64;
            let missing = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1268));
            if missing {
                break 'b_3e5920;
            }
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            if c.g[A0][0] == c.g[A2][0] {
                c.sw(c.g[V0][0] as u32, 0);
            }
            break 'b_3e5920;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

/// `void EAThread_SetPriority(thread, prio)` (0x3e5508): change the
/// thread's priority (own thread when null).
pub fn ea_thread_set_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let given = c.g[A0][0] != 0;
        c.g[S0][0] = c.g[A1][0];
        'b_3e5544: {
            'b_3e5528: {
                if given {
                    break 'b_3e5528;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x450000;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xdfc));
                break 'b_3e5544;
            }
            c.cycles += 4;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                break 'b_3e5544;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_5540);
            let hook = 0x0042_3c90;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_5540)?;
            c.g[A0][0] = c.g[V0][0];
            break 'b_3e5544;
        }
        'b_3e5570: {
            'b_3e5564: {
                c.cycles += 4;
                c.g[V0][0] = 0x450000;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xde4));
                let no_main = c.g[V1][0] == 0;
                if no_main {
                    break 'b_3e5564;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003e_555c);
                c.g[A1][0] = c.g[S0][0];
                let hook = 0x0042_3c40;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_555c)?;
                frame.restore(c, S0, 0x10);
                break 'b_3e5570;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_556c);
            c.g[A1][0] = c.g[S0][0];
            let hook = 0x0042_3c30;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_556c)?;
            frame.restore(c, S0, 0x10);
            break 'b_3e5570;
        }
        c.cycles += 4;
        c.g[V0][0] = 0x1;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAThread_IsFinished(thread)` (0x3e5580): nonzero once the
/// thread is done (a live thread reports its status word).
pub fn ea_thread_is_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xde4));
        let no_main = c.g[V0][0] == 0;
        frame.save_ra(c, 0x30);
        'b_3e55b8: {
            'b_3e55a8: {
                if no_main {
                    break 'b_3e55a8;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[RA][0] = sx(0x003e_55a0);
                c.g[A1][0] = c.g[SP][0];
                let hook = 0x0042_3cb0;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_55a0)?;
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                break 'b_3e55b8;
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[RA][0] = sx(0x003e_55b4);
            c.g[A1][0] = c.g[SP][0];
            let hook = 0x0042_3ca0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_55b4)?;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            break 'b_3e55b8;
        }
        'b_3e55d4: {
            'b_3e55cc: {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 0x10;
                let dead = c.g[V0][0] != 0;
                c.g[A0][0] = 0;
                if dead {
                    break 'b_3e55cc;
                }
                c.cycles += 2;
                let live = c.g[V1][0] != 0;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                if live {
                    break 'b_3e55d4;
                }
                break 'b_3e55cc;
            }
            c.cycles += 2;
            c.g[A0][0] = 0x1;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_3e55d4;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0];
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `u32 EATimer_ServiceFor(ticks)` (0x3e5a10): service timer callbacks
/// until `ticks` elapse; returns the last nonzero callback result.
pub fn ea_timer_service_for(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_38: u32 = 0x003e_5a38;
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_5a2c);
        c.g[S1][0] = 0;
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_5a2c)?;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        'service: loop {
            // Head 0x3e5a44: re-read the clock, exit on expiry or result.
            c.cycles += 5;
            c.g[RA][0] = sx(0x003e_5a4c);
            callee::EA_TIMER_GET_TICKS.call(c, 0x003e_5a4c)?;
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            let expired = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[S1][0];
            if expired {
                break 'service;
            }
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                break 'service;
            }
            // Heads 0x3e5a34/0x3e5a38: pump the callbacks once.
            // (Gen's 5a34 `+1` is dead: the entry always jumps to 5a44.)
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_5a40);
            c.g[A0][0] = 0;
            callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003e_5a40)?;
            c.g[S1][0] = c.g[V0][0];
            if event_due(c) {
                deliver_event_here(c, HEAD_38);
            }
            continue 'service;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EATimer_SetVideoMode(mode)` (0x3e6690): record the display
/// mode for the timer interrupt.
pub fn ea_timer_set_video_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.g[V0][0] = 0x2;
        let pal = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
        if pal {
            c.cycles += 4;
            c.g[V1][0] = 0x450000;
            c.g[V0][0] = 0x3d09;
            let t = c.g[RA][0] as u32;
            c.sw((c.g[V1][0] as u32).wrapping_add(0xda0), c.g[V0][0]);
            c.pc = t;
            return Ok(());
        }
        c.cycles += 2;
        let hi = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if hi {
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            let ntsc = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = 0x450000;
            if ntsc {
                c.cycles += 3;
                c.g[V0][0] = 0x7080;
                let t = c.g[RA][0] as u32;
                c.sw((c.g[V1][0] as u32).wrapping_add(0xda0), c.g[V0][0]);
                c.pc = t;
                return Ok(());
            }
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.pc = t;
            return Ok(());
        }
        c.cycles += 2;
        let sixty = c.g[A0][0] == c.g[V0][0];
        c.g[V1][0] = 0x450000;
        if sixty {
            c.cycles += 3;
            c.g[V0][0] = 0x3d86;
            let t = c.g[RA][0] as u32;
            c.sw((c.g[V1][0] as u32).wrapping_add(0xda0), c.g[V0][0]);
            c.pc = t;
            return Ok(());
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::driven;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::DATA;

    /// Write `s` as bytes at `addr`, NUL-terminated.
    fn put_ascii(c: &mut Cpu, s: &str, addr: u32) {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// A zeroed data block.
    fn zero_block(c: &mut Cpu, base: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(base + 4 * i, 0);
        }
    }

    /// The timer callback table (16 slots of fn/mode/due/fC).
    const TABLE: u32 = 0x0051_ed98;

    driven!(gen_ea_thread_wakeup_alarm, ee::s14::f_003e5290__generated);
    driven!(hand_ea_thread_wakeup_alarm, ea_thread_wakeup_alarm);

    /// Alarm wake through the syscall fake.
    #[test]
    fn ea_thread_wakeup_alarm_matches() {
        check(
            gen_ea_thread_wakeup_alarm,
            hand_ea_thread_wakeup_alarm,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5290, 0);
                c.g[A2][0] = [0u32, 1, 0x1234][seed as usize % 3] as u64;
                c.pc = 0x003e_5290;
            },
        );
    }

    driven!(gen_ea_thread_is_current, ee::s14::f_003e5440__generated);
    driven!(hand_ea_thread_is_current, ea_thread_is_current);

    /// Null, self (-1) and other threads.
    #[test]
    fn ea_thread_is_current_matches() {
        check(
            gen_ea_thread_is_current,
            hand_ea_thread_is_current,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5440, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x9abc);
                c.g[A0][0] = match seed % 3 {
                    0 => 0,
                    1 => 0xffff_ffff_ffff_ffff,
                    _ => sx(DATA + 0x500),
                };
                c.pc = 0x003e_5440;
            },
        );
    }

    driven!(gen_ea_thread_get_priority, ee::s14::f_003e5498__generated);
    driven!(hand_ea_thread_get_priority, ea_thread_get_priority);

    /// Null, self and explicit threads; live and dead main slots.
    #[test]
    fn ea_thread_get_priority_matches() {
        check(
            gen_ea_thread_get_priority,
            hand_ea_thread_get_priority,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5498, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x14);
                c.bus.write32(0x0045_0de4, (seed % 2) as u32);
                c.g[A0][0] = match seed / 2 % 3 {
                    0 => 0,
                    1 => 0xffff_ffff_ffff_ffff,
                    _ => sx(DATA + 0x500),
                };
                c.pc = 0x003e_5498;
            },
        );
    }

    driven!(gen_ea_thread_set_priority, ee::s14::f_003e5508__generated);
    driven!(hand_ea_thread_set_priority, ea_thread_set_priority);

    /// Null, self and explicit threads; live and dead main slots.
    #[test]
    fn ea_thread_set_priority_matches() {
        check(
            gen_ea_thread_set_priority,
            hand_ea_thread_set_priority,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5508, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x14);
                c.bus.write32(0x0045_0de4, (seed % 2) as u32);
                c.g[A0][0] = match seed / 2 % 3 {
                    0 => 0,
                    1 => 0xffff_ffff_ffff_ffff,
                    _ => sx(DATA + 0x500),
                };
                c.g[A1][0] = [0u32, 1, 0x63][seed as usize / 6 % 3] as u64;
                c.pc = 0x003e_5508;
            },
        );
    }

    driven!(gen_ea_thread_is_finished, ee::s14::f_003e5580__generated);
    driven!(hand_ea_thread_is_finished, ea_thread_is_finished);

    /// Dead (0x10), gone (0) and live status words; both main slots.
    #[test]
    fn ea_thread_is_finished_matches() {
        check(
            gen_ea_thread_is_finished,
            hand_ea_thread_is_finished,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5580, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x1234);
                c.bus.write32(0x0045_0de4, (seed % 2) as u32);
                c.g[A0][0] = sx(DATA + 0x500);
                // Status word the ReferThreadStatus fake leaves behind.
                let sp = c.g[SP][0] as u32;
                c.bus
                    .write32(sp - 0x40, [0x10u32, 0, 0x20][seed as usize / 2 % 3]);
                c.pc = 0x003e_5580;
            },
        );
    }

    driven!(gen_ea_thread_wait_for_exit, ee::s14::f_003e55e0__generated);
    driven!(hand_ea_thread_wait_for_exit, ea_thread_wait_for_exit);

    /// Dead exits at once, gone exits at once, live with a live deadline
    /// yields once then exits. (Live with an infinite timeout would spin
    /// on frozen test ticks.)
    #[test]
    fn ea_thread_wait_for_exit_matches() {
        check(
            gen_ea_thread_wait_for_exit,
            hand_ea_thread_wait_for_exit,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_55e0, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x1234);
                c.g[A0][0] = sx(DATA + 0x500);
                let status = [0x10u32, 0, 0x20][seed as usize % 3];
                // Status word the ReferThreadStatus fake leaves behind.
                let sp = c.g[SP][0] as u32;
                c.bus.write32(sp - 0x70, status);
                c.g[A1][0] = if status == 0x20 { 100 } else { 0 };
                c.pc = 0x003e_55e0;
            },
        );
    }

    driven!(gen_ea_thread_exit, ee::s14::f_003e5678__generated);
    driven!(hand_ea_thread_exit, ea_thread_exit);

    /// Exit through the syscall fake.
    #[test]
    fn ea_thread_exit_matches() {
        check(gen_ea_thread_exit, hand_ea_thread_exit, 48, &|c, seed| {
            let _r = machine(c, seed, 0x3e_5678, 0);
            c.pc = 0x003e_5678;
        });
    }

    driven!(gen_ea_timer_clear_callbacks, ee::s14::f_003e57d0__generated);
    driven!(hand_ea_timer_clear_callbacks, ea_timer_clear_callbacks);

    /// Planted table entries are all cleared.
    #[test]
    fn ea_timer_clear_callbacks_matches() {
        check(
            gen_ea_timer_clear_callbacks,
            hand_ea_timer_clear_callbacks,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_57d0, 0);
                for i in 0..64 {
                    c.bus.write32(TABLE + 4 * i, 0xdead_0000 + seed + i);
                }
                c.pc = 0x003e_57d0;
            },
        );
    }

    driven!(gen_ea_timer_add_callback, ee::s14::f_003e57f8__generated);
    driven!(hand_ea_timer_add_callback, ea_timer_add_callback);

    /// New and existing fns; modes -1/0/other; zero and nonzero delays.
    #[test]
    fn ea_timer_add_callback_matches() {
        check(
            gen_ea_timer_add_callback,
            hand_ea_timer_add_callback,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_57f8, 0);
                zero_block(c, TABLE, 64);
                c.bus.write32(0x0045_0dd0, 1000);
                c.bus.write32(0x0045_0e00, (seed % 3) as u32);
                let f = DATA + 0x400 + (seed % 2) * 0x10;
                put_ascii(c, "cb", f);
                if seed / 2 % 2 == 1 {
                    // Existing fn in slot 2: the port must reuse it.
                    c.bus.write32(TABLE + 2 * 0x10, f);
                    c.bus.write32(TABLE + 2 * 0x10 + 4, 1);
                    c.bus.write32(TABLE + 2 * 0x10 + 8, 500);
                }
                c.g[A0][0] = sx(f);
                c.g[A1][0] = [0xffff_ffff_ffff_ffff, 0, 5][seed as usize / 4 % 3];
                c.g[A2][0] = [0u32, 100][seed as usize / 12 % 2] as u64;
                c.pc = 0x003e_57f8;
            },
        );
    }

    driven!(gen_ea_timer_remove_callback, ee::s14::f_003e58c8__generated);
    driven!(hand_ea_timer_remove_callback, ea_timer_remove_callback);

    /// Fn in slot 0, in a later slot, or absent.
    #[test]
    fn ea_timer_remove_callback_matches() {
        check(
            gen_ea_timer_remove_callback,
            hand_ea_timer_remove_callback,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_58c8, 0);
                zero_block(c, TABLE, 64);
                let f = DATA + 0x400;
                put_ascii(c, "cb", f);
                match seed % 3 {
                    0 => {
                        c.bus.write32(TABLE, f);
                    }
                    1 => {
                        c.bus.write32(TABLE + 3 * 0x10, f);
                        c.bus.write32(TABLE + 3 * 0x10 + 4, 2);
                    }
                    _ => {}
                }
                c.g[A0][0] = sx(f);
                c.pc = 0x003e_58c8;
            },
        );
    }

    driven!(gen_ea_timer_service_for, ee::s14::f_003e5a10__generated);
    driven!(hand_ea_timer_service_for, ea_timer_service_for);

    /// Zero duration exits at once; a live fake callback that returns
    /// nonzero ends the wait with its result. (A zero-returning callback
    /// would spin on frozen test ticks.)
    #[test]
    fn ea_timer_service_for_matches() {
        check(
            gen_ea_timer_service_for,
            hand_ea_timer_service_for,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5a10, 0);
                zero_block(c, TABLE, 64);
                c.bus.write32(0x0045_0dd0, 1000);
                if seed % 2 == 0 {
                    c.g[A0][0] = 0;
                } else {
                    // Due callback through the miss-hook fake.
                    c.bus.write32(TABLE, FAKE);
                    c.bus.write32(TABLE + 4, 1);
                    c.bus.write32(TABLE + 8, 0);
                    c.g[A0][0] = 0x1_0000;
                }
                c.pc = 0x003e_5a10;
            },
        );
    }

    driven!(gen_ea_timer_set_video_mode, ee::s14::f_003e6690__generated);
    driven!(hand_ea_timer_set_video_mode, ea_timer_set_video_mode);

    /// All five mode arms (PAL/NTSC/60Hz/fallthrough).
    #[test]
    fn ea_timer_set_video_mode_matches() {
        check(
            gen_ea_timer_set_video_mode,
            hand_ea_timer_set_video_mode,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_6690, 0);
                c.bus.write32(0x0045_0da0, 0);
                c.g[A0][0] = (seed % 5) as u64;
                c.pc = 0x003e_6690;
            },
        );
    }
}
