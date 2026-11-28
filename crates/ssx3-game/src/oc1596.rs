//! Staging module oc1596 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1596`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::lib::kernel::{do_syscall, event_if_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_4a60, i_enable_dmac_wrapper, "iEnableDmac"),
    (0x0042_4a80, i_disable_dmac_wrapper, "iDisableDmac"),
    (0x0042_4ce8, i_rotate_thread_ready_queue, "iRotateThreadReadyQueue"),
    (0x0042_4d68, i_suspend_thread_checked, "iSuspendThread"),
    (0x0042_c130, word_copy_0042c130, "func_0042C130"),
    (0x0042_c168, scan_words_0042c168, "func_0042C168"),
    (0x0042_c360, word_copy_0042c360, "func_0042C360"),
];

/// `_iEnableDmac` (0x423b60), `_iDisableDmac` (0x423b70), `iSignalSema`
/// (0x423dd0) and `_iSuspendThread` (0x423d20): the calls these bodies make.
const KERNL_I_ENABLE_DMAC: Callee = Callee {
    addr: 0x0042_3b60,
    f: ee::s15::f_00423b60,
};
const KERNL_I_DISABLE_DMAC: Callee = Callee {
    addr: 0x0042_3b70,
    f: ee::s15::f_00423b70,
};
const KERNL_I_SIGNAL_SEMA: Callee = Callee {
    addr: 0x0042_3dd0,
    f: ee::s15::f_00423dd0,
};
const KERNL_I_SUSPEND_THREAD: Callee = Callee {
    addr: 0x0042_3d20,
    f: ee::s15::f_00423d20,
};

/// `iEnableDmac(void)` (0x424a60): `_iEnableDmac()` in a 0x10-byte frame.
pub fn i_enable_dmac_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        KERNL_I_ENABLE_DMAC.call(c, 0x0042_4a70)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `iDisableDmac(void)` (0x424a80): `_iDisableDmac()` in a 0x10-byte frame.
pub fn i_disable_dmac_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        KERNL_I_DISABLE_DMAC.call(c, 0x0042_4a90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int iRotateThreadReadyQueue(int priority)` (0x424ce8): queues a rotate
/// request (kind 1) for `priority` in the interrupt helper's wake queue (header
/// at `D_0052B3A8`: index word at +4, 2-byte entries from +8) and signals its
/// semaphore (`D_0052B3A0`). Returns `priority`, or -1 when `priority >= 0x80`
/// or the queue is not set up (`D_00455170 == 0`).
pub fn i_rotate_thread_ready_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        let saved_s0 = c.g[S0][0];
        Addr(sp(c)).sd(c, saved_s0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[S0][0] < 0x80) as u64;
        let saved_ra = c.g[RA][0];
        Addr(sp(c).wrapping_add(0x10)).sd(c, saved_ra);
        let ready = if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x0045_0000;
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x5170)).lw(c);
            let set = c.g[V1][0] != 0;
            c.g[V1][0] = 0x0053_0000;
            set
        } else {
            false
        };
        'done: {
            if !ready {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                break 'done;
            }
            c.cycles += 15;
            c.g[A1][0] = 0x0053_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4c58));
            c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_sub(0x4c60)).lw(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[A3][0] = 1;
            c.g[V0][0] &= 0x1ff;
            c.g[A2][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            let index = c.g[V0][0];
            Addr((c.g[V1][0] as u32).wrapping_add(0x4)).sw(c, index);
            let kind = c.g[A3][0];
            Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sb(c, kind);
            c.g[V1][0] = c.g[A1][0];
            let priority = c.g[S0][0];
            Addr((c.g[V1][0] as u32).wrapping_add(0x9)).sb(c, priority);
            KERNL_I_SIGNAL_SEMA.call(c, 0x0042_4d50)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        c.g[RA][0] = Addr(sp(c).wrapping_add(0x10)).ld(c);
        c.g[S0][0] = Addr(sp(c)).ld(c);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `int iSuspendThread(int threadId)` (0x424d68): asks the kernel for the
/// calling thread's id (GetThreadId, syscall -47). When it is `threadId`, queues
/// a suspend request (kind 2) for it in the helper's wake queue, signals the
/// semaphore and returns the id; -1 when the id is 0x100 or more or the queue
/// is not set up. Any other thread goes to `_iSuspendThread(threadId)`.
pub fn i_suspend_thread_checked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        let saved_ra = c.g[RA][0];
        Addr(sp(c).wrapping_add(0x10)).sd(c, saved_ra);
        let saved_s0 = c.g[S0][0];
        Addr(sp(c)).sd(c, saved_s0);
        c.g[V1][0] = 0xffff_ffff_ffff_ffd1;
        if !do_syscall(c, 0x0042_4d78, 0x0042_4d7c) {
            return Ok(());
        }
        event_if_due(c);
        c.g[S0][0] = c.g[V0][0];
        let same = c.g[S0][0] == c.g[A0][0];
        c.g[V0][0] = (c.g[S0][0] < 0x100) as u64;
        'df4: {
            if !same {
                c.cycles += 4;
                KERNL_I_SUSPEND_THREAD.call(c, 0x0042_4d90)?;
            } else {
                'df0: {
                    c.cycles += 2;
                    let in_range = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x0045_0000;
                    let ready = if in_range {
                        c.cycles += 3;
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x5170)).lw(c);
                        let set = c.g[V1][0] != 0;
                        c.g[V1][0] = 0x0053_0000;
                        set
                    } else {
                        false
                    };
                    if !ready {
                        c.cycles += 2;
                        c.g[V0][0] = u64::MAX;
                        break 'df0;
                    }
                    c.cycles += 15;
                    c.g[A1][0] = 0x0053_0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4c58));
                    c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_sub(0x4c60)).lw(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4)).lw(c);
                    c.g[A3][0] = 2;
                    c.g[V0][0] &= 0x1ff;
                    c.g[A2][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    let index = c.g[V0][0];
                    Addr((c.g[V1][0] as u32).wrapping_add(0x4)).sw(c, index);
                    let kind = c.g[A3][0];
                    Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sb(c, kind);
                    c.g[V1][0] = c.g[A1][0];
                    let thread = c.g[S0][0];
                    Addr((c.g[V1][0] as u32).wrapping_add(0x9)).sb(c, thread);
                    KERNL_I_SIGNAL_SEMA.call(c, 0x0042_4dec)?;
                    c.g[V0][0] = c.g[S0][0];
                }
                c.cycles += 1;
            }
            c.g[RA][0] = Addr(sp(c).wrapping_add(0x10)).ld(c);
        }
        c.cycles += 3;
        c.g[S0][0] = Addr(sp(c)).ld(c);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// The copy loop of `func_0042C130` (0x42c130) and its twin `func_0042C360`
/// (0x42c360), which differ only in the address of the loop's event check
/// (`head`): copies `size >> 2` words from `a1` to `a0` and returns 0.
fn word_copy_body(c: &mut Cpu, head: u32) {
    c.cycles += 3;
    c.g[A2][0] = sx((c.g[A2][0] as u32) >> 2);
    let empty = c.g[A2][0] == 0;
    c.g[A3][0] = 0;
    if !empty {
        c.cycles += 1;
        loop {
            c.cycles += 8;
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            c.g[V0][0] = (c.g[A3][0] < c.g[A2][0]) as u64;
            let word = c.g[V1][0];
            Addr(c.g[A0][0] as u32).sw(c, word);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `unsigned func_0042C130(void *dst, const void *src, unsigned size)` (0x42c130).
pub fn word_copy_0042c130(c: &mut Cpu) {
    word_copy_body(c, 0x0042_c140);
}

/// `unsigned func_0042C360(void *dst, const void *src, unsigned size)` (0x42c360).
pub fn word_copy_0042c360(c: &mut Cpu) {
    word_copy_body(c, 0x0042_c370);
}

/// `const u32 *func_0042C168(const u32 *cursor, const u32 *end, u32 value)`
/// (0x42c168): the first word from `cursor` below `end` that equals `value`,
/// and 0 when there is none (`cursor` reached `end`, or it was not below it).
pub fn scan_words_0042c168(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    let hit = c.g[V0][0] == c.g[A2][0];
    c.g[V0][0] = (c.g[A0][0] < c.g[A1][0]) as u64;
    'done: {
        if !hit {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = 0;
                break 'done;
            }
            c.cycles += 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                let hit = c.g[V0][0] == c.g[A2][0];
                c.g[V0][0] = (c.g[A0][0] < c.g[A1][0]) as u64;
                if hit {
                    break;
                }
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                    if event_due(c) {
                        deliver_event_here(c, 0x0042_c180);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[A0][0] = 0;
                break 'done;
            }
        }
        c.cycles += 1;
        if c.g[V0][0] == 0 {
            c.g[A0][0] = 0;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, LOG_RESULTS, LOG_UNWIND_AT};
    use crate::difftest::{check, fill, scramble, Rng, DATA};

    const SEEDS: u32 = 200;
    /// `D_00455170`: the helper queue is set up when it is non-zero.
    const QUEUE_READY: u32 = 0x0045_0000 + 0x5170;
    /// `D_0052B3A8` (the wake queue header) and its index word.
    const QUEUE: u32 = 0x0053_0000 - 0x4c58;
    const QUEUE_INDEX: u32 = QUEUE + 4;
    /// `D_0052B3A0`: the helper semaphore id.
    const QUEUE_SEMA: u32 = 0x0053_0000 - 0x4c60;

    #[test]
    fn i_enable_dmac_wrapper_matches() {
        covered(
            ee::s15::f_00424a60__generated,
            i_enable_dmac_wrapper,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1596_0001, 1);
            },
        );
    }

    #[test]
    fn i_disable_dmac_wrapper_matches() {
        covered(
            ee::s15::f_00424a80__generated,
            i_disable_dmac_wrapper,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1596_0002, 1);
            },
        );
    }

    #[test]
    fn i_rotate_thread_ready_queue_matches() {
        covered(
            ee::s15::f_00424ce8__generated,
            i_rotate_thread_ready_queue,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1596_0005, 1);
                c.bus.write32(QUEUE_READY, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(QUEUE_SEMA, r.next_u32());
                c.bus.write32(QUEUE_INDEX, r.next_u32());
                c.g[A0][0] = match seed % 4 {
                    0 => 0x80,
                    1 => u64::MAX,
                    _ => (r.next_u32() % 0x80) as u64,
                };
            },
        );
    }

    #[test]
    fn i_suspend_thread_checked_matches() {
        covered(
            ee::s15::f_00424d68__generated,
            i_suspend_thread_checked,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1596_0006, 3);
                // The GetThreadId result (the first call the body makes).
                let id: u32 = [0x42, 0x07, 0x100, 0x1ff][(seed % 4) as usize];
                c.bus.write32(LOG_RESULTS, id);
                c.g[A0][0] = if seed % 3 == 0 {
                    id as u64
                } else {
                    (r.next_u32() | 0x1000) as u64
                };
                c.bus.write32(QUEUE_READY, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(QUEUE_SEMA, r.next_u32());
                c.bus.write32(QUEUE_INDEX, r.next_u32());
            },
        );
    }

    #[test]
    fn scan_words_0042c168_matches() {
        check(
            ee::s15::f_0042c168__generated,
            scan_words_0042c168,
            300,
            &|c, seed| {
                let mut r = Rng(0x42c1_6800 ^ seed as u64);
                scramble(c, &mut r);
                let base = DATA + 0x3000;
                for i in 0u32..16 {
                    c.bus.write32(base + 4 * i, r.next_u32() % 8);
                }
                let start = seed % 5;
                let end = 1 + seed % 12;
                c.g[A0][0] = sx(base + 4 * start);
                c.g[A1][0] = sx(base + 4 * end);
                // Searched value: never present, or present at a random word.
                c.g[A2][0] = if seed % 3 == 0 {
                    9
                } else {
                    let k = r.next_u32() % 16;
                    c.bus.read32(base + 4 * k) as u64
                };
            },
        );
    }

    fn word_copy_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x42c1_3000 ^ seed as u64);
        scramble(c, &mut r);
        fill(c, &mut r, DATA + 0x2000, 16);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(DATA + 0x2000);
        // Sizes 0..40 bytes: zero words, and sizes that are not a multiple of 4.
        c.g[A2][0] = (seed % 41) as u64;
    }

    #[test]
    fn word_copy_0042c130_matches() {
        check(
            ee::s15::f_0042c130__generated,
            word_copy_0042c130,
            200,
            &|c, seed| word_copy_setup(c, seed),
        );
    }

    #[test]
    fn word_copy_0042c360_matches() {
        check(
            ee::s15::f_0042c360__generated,
            word_copy_0042c360,
            200,
            &|c, seed| word_copy_setup(c, seed),
        );
    }
}
