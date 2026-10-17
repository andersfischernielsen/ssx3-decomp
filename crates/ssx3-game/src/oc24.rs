//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Hand-written, typed replacements for small generated functions, each with a
//! differential test against the generated body. This batch is mostly newlib
//! libc internals (reentrant I/O wrappers, kernel syscall wrappers) and the
//! AI manager.

use crate::abi;
use crate::call::{self, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0041_ec08, localeconv_r, "_localeconv_r"),
    (0x0042_39c0, exit, "_Exit"),
    (0x0042_39e0, load_exec_ps2, "_LoadExecPS2"),
    (0x0042_3ae0, enable_intc, "_EnableIntc"),
    (0x0042_3af0, disable_intc, "_DisableIntc"),
    (0x0042_3b00, enable_dmac, "_EnableDmac"),
    (0x0042_3b10, disable_dmac, "_DisableDmac"),
    (0x0042_3b40, i_enable_intc, "_iEnableIntc"),
    (0x0042_3b50, i_disable_intc, "_iDisableIntc"),
    (0x0042_3b60, i_enable_dmac, "_iEnableDmac"),
    (0x0042_3b70, i_disable_dmac, "_iDisableDmac"),
    (0x0042_3bc0, start_thread, "_StartThread"),
    (0x0042_3ce0, i_wakeup_thread, "_iWakeupThread"),
    (0x0042_3d20, i_suspend_thread, "_iSuspendThread"),
    (0x0042_41c0, exec_osd, "_ExecOSD"),
    (0x0042_4210, init_tlb, "_InitTLB"),
    (0x0042_4558, fstat, "_fstat"),
    (0x0041_fb98, close_r, "_close_r"),
    (0x0041_fbf0, fstat_r, "_fstat_r"),
    (0x0041_fc50, lseek_r, "_lseek_r"),
    (0x0041_fcb0, open_r, "_open_r"),
    (0x0041_fd10, read_r, "_read_r"),
    (0x0041_fd70, sbrk_r, "_sbrk_r"),
    (0x0041_fdd0, write_r, "_write_r"),
    (0x0041_7948, sread, "__sread"),
    (0x0041_79b0, swrite, "__swrite"),
    (0x0041_7a30, sseek, "__sseek"),
    (0x0041_7a98, sclose, "__sclose"),
    (0x0041_8ca8, malloc_lock, "__malloc_lock"),
    (0x0041_8d08, malloc_unlock, "__malloc_unlock"),
    (0x0042_4578, kill, "_kill"),
    (0x0042_05b8, bfree, "_Bfree"),
    (0x0042_0830, hi0bits, "__hi0bits"),
    (0x0042_08b8, lo0bits, "__lo0bits"),
    (0x0042_0e28, mcmp, "__mcmp"),
    (0x0042_1020, ulp, "__ulp"),
    (0x0042_14c0, mprec_log10, "_mprec_log10"),
    (0x0042_d320, fixdfdi, "__fixdfdi"),
    (0x0042_d380, fixunsdfdi, "__fixunsdfdi"),
    // `_sfmoreglue`/`_calloc_r` are implemented below but not listed: their
    // real `_malloc_r` cannot initialise on a test machine (free-list bins
    // are only set up at boot), so they have no passing differential test.
    (0x0042_45f0, sync_dcache, "_SyncDCache"),
    (0x0042_4730, invalid_dcache, "_InvalidDCache"),
    (0x0042_0180, sflags, "__sflags"),
    (0x0011_3b10, ai_set_race_state, "AI_SetRaceState"),
    (0x0011_3b48, ai_select_race_state, "AI_SelectRaceState"),
    // `AI_SetupComputerRiders`/`AI_SetupMissionRider` are implemented below
    // but not listed: their `AI_Create*` callees cannot run on a test machine
    // (they construct riders from the disc-loaded database).
    (0x0012_b000, ai_remove_actor_riders, "AI_RemoveActorRiders"),
    (0x0012_b030, ai_remove_computer_riders, "AI_RemoveComputerRiders"),
    (0x0012_b060, ai_remove_mission_riders, "AI_RemoveMissionRiders"),
    (0x0012_87b8, ai_dtor, "AI_dtor"),
    (0x0012_ae38, ai_destroy_computer_riders, "AI_DestroyComputerRiders"),
    (0x0012_aed0, ai_destroy_actor_riders, "AI_DestroyActorRiders"),
    (0x0012_af68, ai_destroy_mission_riders, "AI_DestroyMissionRiders"),
    (0x0012_b090, ai_destroy_all_riders, "AI_DestroyAllRiders"),
    (0x0012_b200, ai_free_anim_sources, "AI_FreeAnimDataSources"),
    (0x0010_fc30, ai_count_riders_on_path, "AI_CountRidersOnPath"),
    (0x0012_97c8, ai_reset_riders, "AI_ResetRiders"),
    (0x0012_9768, ai_restart, "AI_Restart"),
    (0x0012_b788, ai_refresh_rider_overlaps, "AI_RefreshRiderOverlaps"),
];

// ---------------------------------------------------------------------------
// Kernel syscall wrappers. Each sets the syscall code in `v1`, traps (the
// return address is `ret`, the trap itself at `ret - 4`), delivers any due
// event and returns whatever the trap left in `v0`.

/// The `syscall` trap: `pc = ret`, invoke the syscall hook at `at`; `false`
/// when the hook unwound (a thread switch mid-syscall).
#[inline]
fn do_syscall(c: &mut Cpu, at: u32, ret: u32) -> bool {
    c.pc = ret;
    c.syscall(at);
    c.pc == ret
}

/// The event check the syscall wrappers make just before returning (the
/// result of `event` is ignored, as compiled code ignores it).
#[inline]
fn event_if_due(c: &mut Cpu) {
    if event_due(c) {
        deliver_event_here(c, c.pc);
    }
}

/// One `syscall` wrapper: 4 cycles, `v1 = code`, trap at `at`, return to
/// `ret`, then a due-event check and `jr ra`.
#[inline]
fn syscall_wrapper(c: &mut Cpu, code: u32, at: u32, ret: u32) {
    c.cycles += 4;
    c.g[V1][0] = sx(code);
    if !do_syscall(c, at, ret) {
        return;
    }
    event_if_due(c);
    abi::ret(c);
}

/// `void _Exit(int status)` (0x4239c0): syscall 4.
pub fn exit(c: &mut Cpu) {
    syscall_wrapper(c, 4, 0x0042_39c4, 0x0042_39c8);
}

/// `void _LoadExecPS2(const char *path, int argc, char **argv)` (0x4239e0):
/// syscall 6.
pub fn load_exec_ps2(c: &mut Cpu) {
    syscall_wrapper(c, 6, 0x0042_39e4, 0x0042_39e8);
}

/// `int _EnableIntc(int intr)` (0x423ae0): syscall 0x14.
pub fn enable_intc(c: &mut Cpu) {
    syscall_wrapper(c, 0x14, 0x0042_3ae4, 0x0042_3ae8);
}

/// `int _DisableIntc(int intr)` (0x423af0): syscall 0x15.
pub fn disable_intc(c: &mut Cpu) {
    syscall_wrapper(c, 0x15, 0x0042_3af4, 0x0042_3af8);
}

/// `int _EnableDmac(int channel)` (0x423b00): syscall 0x16.
pub fn enable_dmac(c: &mut Cpu) {
    syscall_wrapper(c, 0x16, 0x0042_3b04, 0x0042_3b08);
}

/// `int _DisableDmac(int channel)` (0x423b10): syscall 0x17.
pub fn disable_dmac(c: &mut Cpu) {
    syscall_wrapper(c, 0x17, 0x0042_3b14, 0x0042_3b18);
}

/// `int _iEnableIntc(int intr)` (0x423b40): interrupt-disabled syscall -26.
pub fn i_enable_intc(c: &mut Cpu) {
    syscall_wrapper(c, (-26i32) as u32, 0x0042_3b44, 0x0042_3b48);
}

/// `int _iDisableIntc(int intr)` (0x423b50): syscall -27.
pub fn i_disable_intc(c: &mut Cpu) {
    syscall_wrapper(c, (-27i32) as u32, 0x0042_3b54, 0x0042_3b58);
}

/// `int _iEnableDmac(int channel)` (0x423b60): syscall -28.
pub fn i_enable_dmac(c: &mut Cpu) {
    syscall_wrapper(c, (-28i32) as u32, 0x0042_3b64, 0x0042_3b68);
}

/// `int _iDisableDmac(int channel)` (0x423b70): syscall -29.
pub fn i_disable_dmac(c: &mut Cpu) {
    syscall_wrapper(c, (-29i32) as u32, 0x0042_3b74, 0x0042_3b78);
}

/// `int _StartThread(struct thread *th, void *arg)` (0x423bc0): syscall 0x22.
pub fn start_thread(c: &mut Cpu) {
    syscall_wrapper(c, 0x22, 0x0042_3bc4, 0x0042_3bc8);
}

/// `int _iWakeupThread(int thid)` (0x423ce0): syscall -52.
pub fn i_wakeup_thread(c: &mut Cpu) {
    syscall_wrapper(c, (-52i32) as u32, 0x0042_3ce4, 0x0042_3ce8);
}

/// `int _iSuspendThread(int thid)` (0x423d20): syscall -56.
pub fn i_suspend_thread(c: &mut Cpu) {
    syscall_wrapper(c, (-56i32) as u32, 0x0042_3d24, 0x0042_3d28);
}

/// `void _ExecOSD(int argc, char **argv)` (0x4241c0): syscall 0x7b.
pub fn exec_osd(c: &mut Cpu) {
    syscall_wrapper(c, 0x7b, 0x0042_41c4, 0x0042_41c8);
}

/// `void _InitTLB(void)` (0x424210): syscall 0x82.
pub fn init_tlb(c: &mut Cpu) {
    syscall_wrapper(c, 0x82, 0x0042_4214, 0x0042_4218);
}

// ---------------------------------------------------------------------------
// Leaves.

/// `struct lconv *_localeconv_r(struct _reent *r)` (0x41ec08): the built-in
/// C locale (a static object at 0x49a790); ignores `r`.
pub fn localeconv_r(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(0x0049_a790);
    abi::ret(c);
}

/// `int _fstat(int fd, struct stat *st)` (0x424558): the stub used when the
/// kernel's `fstat` isn't wired up: zero the size field and mark `st` as a
/// character device (0x2000), return 0.
pub fn fstat(c: &mut Cpu) {
    c.cycles += 5;
    let st = Addr(abi::arg(c, 1));
    st.off(0x48).sd(c, 0);
    st.off(0x4).sw(c, 0x2000);
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Reentrant (newlib `_r`) I/O wrappers. Each shifts the arguments (dropping
// the leading `struct _reent *`), clears the reent errno at 0x4a5bb8, calls
// the underlying syscall stub and, when that returns -1, copies the errno
// into `*reent`.

/// The reentrant struct's `_errno` word (at `0x4a0000 + 0x5bb8`).
const REENT_ERRNO: u32 = 0x004a_0000 + 0x5bb8;

/// The newlib syscall stubs the wrappers call.
const CLOSE: Callee = Callee { addr: 0x0042_4488, f: ssx3_ee::s15::f_00424488 };
const OPEN: Callee = Callee { addr: 0x0042_4460, f: ssx3_ee::s15::f_00424460 };
const READ: Callee = Callee { addr: 0x0042_43e8, f: ssx3_ee::s15::f_004243e8 };
const WRITE: Callee = Callee { addr: 0x0042_4368, f: ssx3_ee::s15::f_00424368 };
const LSEEK: Callee = Callee { addr: 0x0042_4498, f: ssx3_ee::s15::f_00424498 };
const SBRK: Callee = Callee { addr: 0x0042_44a0, f: ssx3_ee::s15::f_004244a0 };
const FSTAT_C: Callee = Callee { addr: 0x0042_4558, f: ssx3_ee::s15::f_00424558 };

/// The shared body: open a 0x30 frame (s0 = reent, s1 = 0x4a0000, ra),
/// shift `shift` arguments off the front, clear the errno, call `target`.
/// `-1` results read the errno back into `v0` and store it through `reent`.
fn reentrant_io(c: &mut Cpu, target: Callee, shift: usize, ret: u32, first: u64) -> call::Flow {
    c.cycles += first;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = 0x004a_0000;
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[A1][0];
    if shift >= 2 {
        c.g[A1][0] = c.g[A2][0];
    }
    if shift >= 3 {
        c.g[A2][0] = c.g[A3][0];
    }
    Addr(REENT_ERRNO).sw(c, 0);
    target.call(c, ret)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = u64::MAX;
    if c.g[V1][0] == u64::MAX {
        c.cycles += 3;
        let errno = Addr(REENT_ERRNO).lw(c);
        c.g[V0][0] = errno;
        if errno != 0 {
            Addr(c.g[S0][0] as u32).sw(c, errno);
        }
        c.cycles += 1;
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 5;
    c.g[V0][0] = c.g[V1][0];
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int _close_r(struct _reent *r, int fd)` (0x41fb98).
pub fn close_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, CLOSE, 1, 0x0041_fbbc, 13))
}

/// `int _fstat_r(struct _reent *r, int fd, struct stat *st)` (0x41fbf0).
pub fn fstat_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, FSTAT_C, 2, 0x0041_fc18, 14))
}

/// `int _lseek_r(struct _reent *r, int fd, int off, int whence)` (0x41fc50).
pub fn lseek_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, LSEEK, 3, 0x0041_fc7c, 15))
}

/// `int _open_r(struct _reent *r, const char *path, int flags, int mode)`
/// (0x41fcb0).
pub fn open_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, OPEN, 3, 0x0041_fcdc, 15))
}

/// `int _read_r(struct _reent *r, int fd, void *buf, int n)` (0x41fd10).
pub fn read_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, READ, 3, 0x0041_fd3c, 15))
}

/// `void *_sbrk_r(struct _reent *r, int incr)` (0x41fd70).
pub fn sbrk_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, SBRK, 1, 0x0041_fd94, 14))
}

/// `int _write_r(struct _reent *r, int fd, const void *buf, int n)`
/// (0x41fdd0).
pub fn write_r(c: &mut Cpu) {
    call::guest(c, |c| reentrant_io(c, WRITE, 3, 0x0041_fdfc, 15))
}

// ---------------------------------------------------------------------------
// stdio stream hooks (`struct __sFILE` at `cookie`; the reent pointer is at
// +0x54, the fd at +0xe, the flags halfword at +0xc, the seek position at
// +0x50). These call the reentrant I/O wrappers above.

const READ_R: Callee = Callee { addr: 0x0041_fd10, f: ssx3_ee::s15::f_0041fd10 };
const WRITE_R: Callee = Callee { addr: 0x0041_fdd0, f: ssx3_ee::s15::f_0041fdd0 };
const LSEEK_R: Callee = Callee { addr: 0x0041_fc50, f: ssx3_ee::s15::f_0041fc50 };
const CLOSE_R: Callee = Callee { addr: 0x0041_fb98, f: ssx3_ee::s15::f_0041fb98 };
const GET_THREAD_ID: Callee = Callee { addr: 0x0042_3c90, f: ssx3_ee::s15::f_00423c90 };
const WAIT_SEMA: Callee = Callee { addr: 0x0042_3de0, f: ssx3_ee::s15::f_00423de0 };
const SIGNAL_SEMA: Callee = Callee { addr: 0x0042_3dc0, f: ssx3_ee::s15::f_00423dc0 };
const LIB_EXIT: Callee = Callee { addr: 0x0042_c6e8, f: ssx3_ee::s15::f_0042c6e8 };

/// `int __sread(void *cookie, char *buf, int n)` (0x417948): `_read_r` on the
/// stream's fd; on error clear flag 0x1000 in the stream flags, else advance
/// the seek position by the bytes read.
pub fn sread(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        let cookie = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = cookie;
        c.g[A2][0] = c.g[A1][0];
        c.g[A0][0] = Addr(cookie as u32).off(0x54).lw(c);
        c.g[A1][0] = Addr(cookie as u32).off(0xe).lh(c);
        READ_R.call(c, 0x0041_7970)?;
        c.g[V1][0] = (c.g[V0][0] as u32 as i32 as i64) as u64;
        if (c.g[V1][0] as i64) < 0 {
            c.g[V0][0] = Addr(cookie as u32).off(0xc).lhu(c);
            c.cycles += 2;
            c.g[V0][0] &= 0xefff;
            Addr(cookie as u32).off(0xc).sh(c, c.g[V0][0]);
        } else {
            c.cycles += 4;
            let pos = Addr(cookie as u32).off(0x50).lw(c);
            c.g[V0][0] = sx((pos as u32).wrapping_add(c.g[V1][0] as u32));
            Addr(cookie as u32).off(0x50).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int __swrite(void *cookie, const char *buf, int n)` (0x4179b0): when the
/// stream is in append mode (flag 0x100), seek to the end first, then
/// `_write_r`; clear flag 0x1000 and return the sign-extended count.
pub fn swrite(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        let cookie = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = cookie;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(cookie as u32).off(0xc).lhu(c);
        c.g[V0][0] &= 0x100;
        c.g[S2][0] = c.g[A2][0];
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = Addr(cookie as u32).off(0x54).lw(c);
            c.g[A2][0] = 0;
            c.g[A1][0] = Addr(cookie as u32).off(0xe).lh(c);
            c.g[A3][0] = 2;
            LSEEK_R.call(c, 0x0041_79f0)?;
        }
        c.cycles += 16;
        let flags = Addr(cookie as u32).off(0xc).lhu(c);
        c.g[V0][0] = flags;
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = Addr(cookie as u32).off(0xe).lh(c);
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = Addr(cookie as u32).off(0x54).lw(c);
        c.g[V0][0] = flags & 0xefff;
        Addr(cookie as u32).off(0xc).sh(c, c.g[V0][0]);
        WRITE_R.call(c, 0x0041_7a10)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = (c.g[V0][0] as u32 as i32 as i64) as u64;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int __sseek(void *cookie, int off, int whence)` (0x417a30):
/// `_lseek_r`; on success store the new position and set flag 0x1000, on
/// failure clear it and return -1.
pub fn sseek(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        let cookie = c.g[A0][0];
        let off = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = cookie;
        c.g[A2][0] = off;
        c.g[A0][0] = Addr(cookie as u32).off(0x54).lw(c);
        c.g[A1][0] = Addr(cookie as u32).off(0xe).lh(c);
        LSEEK_R.call(c, 0x0041_7a58)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = u64::MAX;
        let ok = c.g[A0][0] != u64::MAX;
        c.g[V0][0] = Addr(cookie as u32).off(0xc).lhu(c);
        if ok {
            c.cycles += 4;
            c.g[V1][0] = (c.g[A0][0] as u32 as i32 as i64) as u64;
            Addr(cookie as u32).off(0x50).sw(c, c.g[V1][0]);
            c.g[V0][0] |= 0x1000;
        } else {
            c.cycles += 2;
            c.g[V0][0] &= 0xefff;
        }
        c.cycles += 6;
        Addr(cookie as u32).off(0xc).sh(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int __sclose(void *cookie)` (0x417a98): `_close_r` on the stream's fd.
pub fn sclose(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let cookie = c.g[A0][0];
        c.g[A1][0] = Addr(cookie as u32).off(0xe).lh(c);
        c.g[A0][0] = Addr(cookie as u32).off(0x54).lw(c);
        CLOSE_R.call(c, 0x0041_7aac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// malloc mutex (`0x450000`: owner at +0x4ca8, recursion count at +0x4cac,
// semaphore id at +0x5248).

/// `void __malloc_lock(struct _reent *r)` (0x418ca8): a recursive lock; the
/// first acquisition by a thread waits on the mutex semaphore, and the
/// recursion count is bumped.
pub fn malloc_lock(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x0045_0000;
        GET_THREAD_ID.call(c, 0x0041_8cc0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = Addr(0x0045_0000).off(0x4ca8).lw(c);
        let not_owner = c.g[V0][0] != c.g[S0][0];
        c.g[V0][0] = 0x0045_0000;
        if not_owner {
            c.cycles += 4;
            c.g[A0][0] = Addr(0x0045_0000).off(0x5248).lw(c);
            WAIT_SEMA.call(c, 0x0041_8ce0)?;
            c.g[V1][0] = 0x0045_0000;
            Addr(0x0045_0000).off(0x4ca8).sw(c, c.g[S0][0]);
        } else {
            c.cycles += 2;
            c.g[V1][0] = 0x0045_0000;
        }
        c.cycles += 8;
        let count = Addr(c.g[V1][0] as u32).off(0x4cac).lw(c);
        c.g[V0][0] = sx((count as u32).wrapping_add(1));
        Addr(c.g[V1][0] as u32).off(0x4cac).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void __malloc_unlock(struct _reent *r)` (0x418d08): drop the recursion
/// count; on the last release clear the owner and signal the mutex semaphore.
pub fn malloc_unlock(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 7;
        c.g[V1][0] = 0x0045_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(0x0045_0000).off(0x4cac).lw(c);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(0x0045_0000).off(0x4cac).sw(c, c.g[V0][0]);
        if c.g[V0][0] == 0 {
            c.cycles += 6;
            c.g[V0][0] = 0x0045_0000;
            c.g[A1][0] = 0x0045_0000;
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = Addr(0x0045_0000).off(0x5248).lw(c);
            Addr(0x0045_0000).off(0x4ca8).sw(c, u64::MAX);
            SIGNAL_SEMA.call(c, 0x0041_8d3c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int _kill(int pid, int sig)` (0x424578): pid 1 exits the process; any
/// other pid reports success (the stub can't signal).
pub fn kill(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        let not_self = c.g[A0][0] != 1;
        frame.save_ra(c, 0);
        if !not_self {
            c.cycles += 2;
            c.g[A0][0] = c.g[A1][0];
            LIB_EXIT.call(c, 0x0042_4590)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// newlib bignum / bit helpers.

const CMPDF2: Callee = Callee { addr: 0x0041_2500, f: ssx3_ee::s15::f_00412500 };
const SUBDF3: Callee = Callee { addr: 0x0041_1f70, f: ssx3_ee::s15::f_00411f70 };
const ADDDF3: Callee = Callee { addr: 0x0041_1f18, f: ssx3_ee::s15::f_00411f18 };
const MULDF3: Callee = Callee { addr: 0x0041_1fd8, f: ssx3_ee::s15::f_00411fd8 };
const FLOATDIDF: Callee = Callee { addr: 0x0041_1438, f: ssx3_ee::s15::f_00411438 };
const FIXUNSDFSI: Callee = Callee { addr: 0x0041_26a0, f: ssx3_ee::s15::f_004126a0 };
const FIXUNSDFDI: Callee = Callee { addr: 0x0042_d380, f: ssx3_ee::s15::f_0042d380 };

/// `void _Bfree(struct _reent *r, struct _Bigint *p)` (0x4205b8): push `p`
/// onto the free-list bucket `r->freelist[p->k]` (at `r + 0x4c`, `k` at
/// `p + 4`, `p->next` at `p`).
pub fn bfree(c: &mut Cpu) {
    c.cycles += 2;
    let r = c.g[A0][0];
    let p = c.g[A1][0];
    c.g[A2][0] = r;
    if p != 0 {
        c.cycles += 9;
        c.g[A0][0] = Addr(p as u32).off(0x4).lw(c);
        c.g[V0][0] = Addr(r as u32).off(0x4c).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        Addr(p as u32).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(r as u32).off(0x4c).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[A0][0] as u32).sw(c, p);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int __hi0bits(uint32 x)` (0x420830): number of leading zero bits, 32
/// when `x` is 0 (the original returns 32 for `x == 0`, and never more).
pub fn hi0bits(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] &= c.g[A0][0];
    let t = c.g[V0][0] != 0;
    c.g[A1][0] = 0;
    if !t {
        c.cycles += 2;
        c.g[A1][0] = 0x10;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
    }
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_ff00_0000;
    c.g[V0][0] &= c.g[A0][0];
    let t = c.g[V0][0] != 0;
    if !t {
        c.cycles += 2;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 8);
    }
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_f000_0000;
    c.g[V0][0] &= c.g[A0][0];
    let t = c.g[V0][0] != 0;
    if !t {
        c.cycles += 2;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
    }
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_c000_0000;
    c.g[V0][0] &= c.g[A0][0];
    let t = c.g[V0][0] != 0;
    if !t {
        c.cycles += 2;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(2));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    }
    c.cycles += 2;
    c.g[V1][0] = 0x4000_0000;
    if (c.g[A0][0] as i64) < 0 {
        c.cycles += 1;
        c.g[V0][0] = c.g[A1][0];
    } else {
        c.cycles += 4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V1][0] = c.g[A0][0] & 0x4000_0000;
        c.g[V0][0] = 0x20;
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = c.g[A1][0];
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int __lo0bits(uint32 *x)` (0x4208b8): number of trailing zero bits of
/// `*x`, storing `*x >> n` back (unchanged when `*x` is odd).
pub fn lo0bits(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = c.g[V1][0] & 0x7;
    let aligned = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[V1][0] & 0x1;
    if aligned {
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0] & 0xffff;
        let t = c.g[V0][0] != 0;
        c.g[A1][0] = 0;
        if !t {
            c.cycles += 2;
            c.g[A1][0] = 0x10;
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 16);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0] & 0xff;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] & 0xf;
        if !t {
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 8);
            c.g[V0][0] = c.g[V1][0] & 0xf;
        }
        c.cycles += 2;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] & 0x3;
        if !t {
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 4);
            c.g[V0][0] = c.g[V1][0] & 0x3;
        }
        c.cycles += 2;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] & 0x1;
        if !t {
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(2));
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 2);
            c.g[V0][0] = c.g[V1][0] & 0x1;
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
            c.cycles += 2;
            c.g[V0][0] = c.g[A1][0];
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
        let nonzero = c.g[V1][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        if nonzero {
            c.cycles += 1;
            Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
            c.cycles += 2;
            c.g[V0][0] = c.g[A1][0];
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x20;
        }
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let odd = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[V1][0] & 0x2;
    if odd {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 2);
            c.cycles += 3;
            c.g[V0][0] = 0x2;
        } else {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
            c.g[V0][0] = 0x1;
        }
        Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
    }
}

/// `int __mcmp(struct _Bigint *a, struct _Bigint *b)` (0x420e28): compare the
/// magnitudes from the most significant component down; returns the
/// difference of the lengths when they differ.
pub fn mcmp(c: &mut Cpu) {
    c.cycles += 5;
    let a = c.g[A0][0] as u32;
    let b = c.g[A1][0] as u32;
    c.g[V1][0] = Addr(b).off(0x10).lw(c);
    c.g[V0][0] = Addr(a).off(0x10).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    let same = c.g[V0][0] == 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    if !same {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[A3][0] = sx(a.wrapping_add(0x14));
    c.g[V0][0] = sx(b.wrapping_add(0x14));
    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(4));
    loop {
        c.cycles += 5;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(4));
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        if c.g[V0][0] != c.g[A0][0] {
            c.g[A0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
            c.cycles += 4;
            c.g[V1][0] = 0x1;
            c.g[V0][0] = u64::MAX;
            if c.g[A0][0] == 0 {
                c.g[V0][0] = c.g[V1][0];
            }
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[A3][0] < c.g[A2][0]) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(4));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            crate::events::deliver_event_here(c, 0x0042_0e68);
        }
    }
}

/// `double __ulp(double x)` (0x421020): the unit in the last place of `x` —
/// the smallest power of two that changes it (a small positive value; 0 for
/// out-of-range input).
pub fn ulp(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A0][0] = ((c.g[A0][0] as i64) >> 32) as u64;
    c.g[V0][0] = 0x7ff0_0000;
    c.g[A0][0] &= c.g[V0][0];
    c.g[V1][0] = 0xffff_ffff_fcc0_0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let small = (c.g[A0][0] as i64) <= 0;
    c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A0][0] as u32));
    if small {
        c.cycles += 4;
        c.g[A0][0] = sx((((c.g[V0][0] as u32) as i32) >> 20) as u32);
        c.g[V1][0] = ((c.g[A0][0] as i64) < 20) as u64;
        let t = c.g[V1][0] == 0;
        c.g[V0][0] = 0x8_0000;
        if t {
            // exponent far from the boundary: subtract 20 and shift.
            c.cycles += 6;
            c.g[V1][0] = u64::MAX >> 32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x14));
            c.g[V0][0] = ((c.g[A0][0] as i64) < 31) as u64;
            let t = c.g[V0][0] == 0;
            c.g[A1][0] &= c.g[V1][0];
            if t {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x1f;
                c.g[V1][0] = 0x1;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << (c.g[V0][0] as u32 & 31));
            }
            c.cycles += 6;
            c.g[V0][0] <<= 32;
            c.g[V1][0] = u64::MAX << 32;
            c.g[V0][0] >>= 32;
            c.g[A1][0] &= c.g[V1][0];
            c.g[A1][0] |= c.g[V0][0];
        } else {
            // exponent near the boundary: shift the 0x80000 bit right.
            c.cycles += 10;
            c.g[V1][0] = u64::MAX >> 32;
            c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> (c.g[A0][0] as u32 & 31)) as u32);
            c.g[A1][0] &= c.g[V1][0];
            c.g[V0][0] <<= 32;
            c.g[V1][0] = u64::MAX << 32;
            c.g[A1][0] |= c.g[V0][0];
            c.g[A1][0] &= c.g[V1][0];
        }
    } else {
        c.cycles += 9;
        c.g[V0][0] = u64::MAX >> 32;
        c.g[A0][0] <<= 32;
        c.g[A1][0] &= c.g[V0][0];
        c.g[V1][0] = u64::MAX << 32;
        c.g[A1][0] |= c.g[A0][0];
        c.g[A1][0] &= c.g[V1][0];
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    abi::ret(c);
}

/// `double _mprec_log10(int exp)` (0x4214c0): 10^exp via a table for
/// `0 < exp < 24`, else repeated multiplication (1.0 for `exp <= 0`).
pub fn mprec_log10(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let exp = c.g[A0][0];
        c.g[S0][0] = exp;
        c.g[A0][0] = 0xffc0u64 << 46; // 1.0
        c.g[V0][0] = ((exp as i64) < 24) as u64;
        let big = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !big {
            c.cycles += 6;
            c.g[V0][0] = 0x004a_0000;
            c.g[V1][0] = sx((c.g[S0][0] as u32) << 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5708));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).ld(c);
        } else {
            c.cycles += 2;
            let nonpos = (c.g[S0][0] as i64) <= 0;
            c.g[V0][0] = c.g[A0][0];
            if !nonpos {
                loop {
                    c.cycles += 7;
                    c.g[A1][0] = 0x8048u64 << 47; // 10.0
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                    MULDF3.call(c, 0x0042_1510)?;
                    c.g[A0][0] = c.g[V0][0];
                    if (c.g[S0][0] as i64) <= 0 {
                        c.cycles += 1;
                        c.g[V0][0] = c.g[A0][0];
                        break;
                    }
                    if event_due(c) {
                        crate::events::deliver_event_here(c, 0x0042_1500);
                    }
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `long long __fixdfdi(double x)` (0x42d320): truncate `x` toward zero,
/// negating the unsigned conversion for negative `x`.
pub fn fixdfdi(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        // `sd` (not `sq`): this function saves only the low 64 bits.
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0x0), c.g[S0][0]);
        c.g[S1][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = c.g[S1][0];
        CMPDF2.call(c, 0x0042_d340)?;
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = c.g[S1][0];
        if neg {
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            SUBDF3.call(c, 0x0042_d350)?;
            c.g[A0][0] = c.g[V0][0];
            FIXUNSDFDI.call(c, 0x0042_d358)?;
            c.g[V0][0] = 0u64.wrapping_sub(c.g[V0][0]);
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            FIXUNSDFDI.call(c, 0x0042_d368)?;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0x20);
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0x0));
        frame.close(c);
        Ok(())
    })
}

/// `unsigned long long __fixunsdfdi(double x)` (0x42d380): truncate a
/// non-negative `x`; negative input gives 0.
pub fn fixunsdfdi(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0;
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x0), c.g[S0][0]);
        CMPDF2.call(c, 0x0042_d3a0)?;
        let neg = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = 0;
        if !neg {
            c.cycles += 9;
            c.g[A1][0] = 0xf7c0;
            c.g[A1][0] <<= 46; // 2^46 as a double
            c.g[A0][0] = c.g[S1][0];
            MULDF3.call(c, 0x0042_d3b8)?;
            c.g[A0][0] = c.g[V0][0];
            FIXUNSDFSI.call(c, 0x0042_d3c0)?;
            c.g[S0][0] = c.g[V0][0] << 32;
            let high_set = (c.g[S0][0] as i64) < 0;
            c.g[V0][0] = c.g[S0][0] & 1;
            if !high_set {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                FLOATDIDF.call(c, 0x0042_d3d4)?;
                c.g[A0][0] = c.g[S1][0];
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0] >> 1;
                c.g[A0][0] = c.g[V0][0] | c.g[A0][0];
                FLOATDIDF.call(c, 0x0042_d3e8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[A0][0];
                ADDDF3.call(c, 0x0042_d3f4)?;
                c.g[A0][0] = c.g[S1][0];
            }
            c.cycles += 9;
            c.g[A1][0] = c.g[V0][0];
            SUBDF3.call(c, 0x0042_d400)?;
            c.g[S2][0] = 0;
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S1][0];
            CMPDF2.call(c, 0x0042_d414)?;
            let nonneg = (c.g[V0][0] as i64) >= 0;
            c.g[A0][0] = c.g[S2][0];
            if nonneg {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                FIXUNSDFSI.call(c, 0x0042_d444)?;
                c.g[V0][0] <<= 32;
                c.g[V0][0] >>= 32;
                c.g[S0][0] = c.g[S0][0].wrapping_add(c.g[V0][0]);
            } else {
                c.cycles += 8;
                c.g[A1][0] = c.g[S1][0];
                SUBDF3.call(c, 0x0042_d424)?;
                c.g[A0][0] = c.g[V0][0];
                FIXUNSDFSI.call(c, 0x0042_d42c)?;
                c.g[V0][0] <<= 32;
                c.g[V0][0] >>= 32;
                c.g[S0][0] = c.g[S0][0].wrapping_sub(c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x30);
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0x0));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Misc newlib: allocator glue and the cache-range loops.

const MALLOC_R: Callee = Callee { addr: 0x0041_dbf8, f: ssx3_ee::s15::f_0041dbf8 };
const MEMSET: Callee = Callee { addr: 0x0041_6210, f: ssx3_ee::s15::f_00416210 };

/// `struct glue *_sfmoreglue(struct _reent *r, int n)` (0x41ff98): allocate
/// `n * 0x58 + 0xc` for a `FILE` glue record and zero its data area.
pub fn sfmoreglue(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x58;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.mult(0, S2, S1, V0); // s2 = n * 0x58, LO/HI
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
        MALLOC_R.call(c, 0x0041_ffc0)?;
        c.g[S0][0] = c.g[V0][0];
        let null = c.g[S0][0] == 0;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        if null {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 9;
            Addr(c.g[S0][0] as u32).off(0x4).sw(c, c.g[S1][0]);
            Addr(c.g[S0][0] as u32).sw(c, 0);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S2][0];
            Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0;
            MEMSET.call(c, 0x0041_ffe8)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *_calloc_r(struct _reent *r, size_t n, size_t size)` (0x4224a0):
/// `malloc(n * size)` then zero it (inline for small blocks, `memset`
/// otherwise).
pub fn calloc_r(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 10;
        c.mult(0, 0, A1, A2); // LO/HI = n * size
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lo[0];
        MALLOC_R.call(c, 0x0042_24bc)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_sub(4)).lw(c);
            c.cycles += 7;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V0][0] |= 0xfffc;
            c.g[V1][0] &= c.g[V0][0];
            c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(4));
            c.g[V0][0] = (c.g[A2][0] < 0x25) as u64;
            let large = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[A2][0] < 0x14) as u64;
            if large {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                MEMSET.call(c, 0x0042_2548)?;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[S0][0];
                } else {
                    c.cycles += 5;
                    Addr(c.g[S0][0] as u32).sw(c, 0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = (c.g[A2][0] < 0x1c) as u64;
                    let t = c.g[V0][0] != 0;
                    Addr(c.g[S0][0] as u32).off(0x4).sw(c, 0);
                    if !t {
                        c.cycles += 5;
                        c.g[V0][0] = (c.g[A2][0] < 0x24) as u64;
                        Addr(c.g[S0][0] as u32).off(0x8).sw(c, 0);
                        Addr(c.g[S0][0] as u32).off(0xc).sw(c, 0);
                        let t = c.g[V0][0] != 0;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                        if !t {
                            c.cycles += 3;
                            Addr(c.g[S0][0] as u32).off(0x10).sw(c, 0);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                            Addr(c.g[S0][0] as u32).off(0x14).sw(c, 0);
                        }
                    }
                }
                c.cycles += 5;
                Addr(c.g[A0][0] as u32).sw(c, 0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                Addr(c.g[A0][0] as u32).off(0x4).sw(c, 0);
                Addr(c.g[A0][0] as u32).sw(c, 0);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One pass of the cache-range loop: compute `address` from the CP0 tag
/// register (`mfc0 $2, $28`), masked to a page and offset by the index, and
/// classify both sub-blocks (cycle count only; the branches always fall
/// through). Returns the cycles charged.
#[inline]
fn cache_pass(c: &mut Cpu) {
    c.cycles += 10;
    c.mfc0(2, 28);
    c.g[V0][0] &= c.g[A3][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
    c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.cycles += 3;
        }
    }
    c.cycles += 10;
    c.mfc0(2, 28);
    c.g[V0][0] &= c.g[A3][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
    c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.cycles += 3;
        }
    }
}

/// The shared body of `_SyncDCache`/`_InvalidDCache`: walk 4 KiB in 0x40
/// steps; `head` is the loop head for event delivery.
fn cache_range(c: &mut Cpu, head: u32) {
    c.cycles += 4;
    c.g[A3][0] = 0xffff_ffff_ffff_0000;
    c.g[A2][0] = 0;
    c.g[A3][0] |= 0xf000;
    loop {
        cache_pass(c);
        c.cycles += 5;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = ((c.g[A2][0] as i64) < 4096) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            crate::events::deliver_event_here(c, head);
        }
    }
}

/// `void _SyncDCache(void *start, void *end)` (0x4245f0): write back the
/// cache lines covering `[start, end)`.
pub fn sync_dcache(c: &mut Cpu) {
    cache_range(c, 0x0042_4600);
}

/// `void _InvalidDCache(void *start, void *end)` (0x424730): invalidate the
/// cache lines covering `[start, end)`.
pub fn invalid_dcache(c: &mut Cpu) {
    cache_range(c, 0x0042_4740);
}

/// `int __sflags(struct _reent *r, const char *mode, int *optr)` (0x420180):
/// decode a stdio mode ("r", "w", "a", optionally with "+"): returns the
/// `__SL64`/`__SWR`/`__SRD` flags and stores the open flags through `optr`.
pub fn sflags(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lb(c);
    c.g[V0][0] = 0x72; // 'r'
    let is_r = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[V1][0] < 115) as u64; // < 's'
    if is_r {
        c.cycles += 4;
        c.g[T1][0] = 0x4;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
    } else {
        c.cycles += 2; // entry into the first-char class block
        if c.g[V0][0] != 0 {
            // first char below 's': only 'a' remains.
            c.cycles += 2;
            let is_a = c.g[V1][0] == 0x61; // 'a'
            c.g[V1][0] = 0x16;
            if is_a {
                c.cycles += 4;
                c.g[T1][0] = 0x108;
                c.g[A3][0] = 1;
                c.g[T0][0] = 0x208;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.cycles += 2;
                Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
                abi::ret(c);
                return;
            }
        } else {
            // first char at or above 's': only 'w' remains.
            c.cycles += 3;
            c.g[V0][0] = 0x77; // 'w'
            let is_w = c.g[V1][0] == c.g[V0][0];
            c.g[V1][0] = 0x16;
            if is_w {
                c.cycles += 4;
                c.g[T1][0] = 0x8;
                c.g[A3][0] = 1;
                c.g[T0][0] = 0x600;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.cycles += 2;
                Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
                abi::ret(c);
                return;
            }
        }
    }
    // The "+" (update) suffix, at mode[1] or mode[2].
    c.cycles += 4;
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(1)).lb(c);
    c.g[V1][0] = 0x2b; // '+'
    if c.g[V0][0] == c.g[V1][0] {
        c.g[T1][0] = 0x10;
        c.cycles += 2;
        c.g[A3][0] = 2;
        c.g[V1][0] = c.g[A3][0] | c.g[T0][0];
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(2)).lb(c);
        let not_plus = c.g[V0][0] != c.g[V1][0];
        c.g[V1][0] = c.g[A3][0] | c.g[T0][0];
        if !not_plus {
            c.cycles += 1;
            c.g[T1][0] = 0x10;
            c.cycles += 2;
            c.g[A3][0] = 2;
            c.g[V1][0] = c.g[A3][0] | c.g[T0][0];
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[T1][0];
    Addr(c.g[A2][0] as u32).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// AI manager: race-state selection.

const AI_SELECT_RACE_STATE: Callee = Callee { addr: 0x0011_3b48, f: ssx3_ee::s00::f_00113b48 };
const AI_SET_RACE_STATE: Callee = Callee { addr: 0x0011_3b10, f: ssx3_ee::s00::f_00113b10 };

/// `void AI_SelectRaceState(AI *ai, int state)` (0x113b48): look up the
/// entry point for `state` in the dispatch table at `0x450000 + 0x6ca0`
/// (`(state-1) * 4`) and store the state's handler address (+0xa0), mode
/// (+0) and label pointer (+0x98); out-of-range states zero those fields.
/// The `__entries` twin handles the table's mid-function targets.
pub fn ai_select_race_state(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = sx(abi::arg(c, 1).wrapping_sub(1));
    c.g[V0][0] = (c.g[V1][0] < 7) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[A2][0] = c.g[A0][0];
    if out_of_range {
        c.cycles += 4;
        Addr(c.g[A2][0] as u32).sw(c, 0);
        Addr(c.g[A2][0] as u32).off(0x98).sw(c, 0);
        Addr(c.g[A2][0] as u32).off(0xa0).sw(c, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[V0][0] = 0x0045_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca0));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let target = c.g[A0][0] as u32;
    // (mode, handler address, label offset).
    let (mode, handler, label): (u64, u32, u32) = match target {
        0x0011_3b74 => (1, 0x0045_0000 + 0x6c58, 0xac),
        0x0011_3b94 => (2, 0x0045_0000 + 0x6c68, 0xb0),
        0x0011_3ba8 => (3, 0x004a_0000 + 0xe90, 0xb4),
        0x0011_3bbc => (4, 0x0045_0000 + 0x6c78, 0xbc),
        0x0011_3bd0 => (5, 0x004a_0000 + 0xe98, 0xc0),
        0x0011_3be4 => (6, 0x004a_0000 + 0xea0, 0xc4),
        0x0011_3bf8 => (7, 0x0045_0000 + 0x6c88, 0xc8),
        _ => {
            c.jump_out(target, 0x0011_3b48, 0x0011_3c1c);
            return;
        }
    };
    c.cycles += if mode == 1 { 4 } else { 5 };
    c.g[A0][0] = mode;
    c.g[V0][0] = sx(handler);
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(label));
    c.cycles += 4;
    Addr(c.g[A2][0] as u32).off(0x98).sw(c, c.g[V1][0]);
    Addr(c.g[A2][0] as u32).sw(c, c.g[A0][0]);
    Addr(c.g[A2][0] as u32).off(0xa0).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void AI_SetRaceState(AI *ai, int state)` (0x113b10): when the state
/// changes, remember the old one (+4), snapshot the path pointer (+0x98 into
/// +0x9c) and reselect.
pub fn ai_set_race_state(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        let ai = c.g[A0][0];
        c.g[V1][0] = ai;
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(ai as u32).lw(c);
        if c.g[A1][0] == c.g[A2][0] {
            c.g[RA][0] = c.ld(frame.at(c, 0));
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(ai as u32).off(0x98).lw(c);
            Addr(ai as u32).off(0x4).sw(c, c.g[A2][0]);
            Addr(ai as u32).off(0x9c).sw(c, c.g[V0][0]);
            AI_SELECT_RACE_STATE.call(c, 0x0011_3b38)?;
            c.g[RA][0] = c.ld(frame.at(c, 0));
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AI manager: rider setup/teardown wrappers. The destroy/reset callees take
// their short path when the AI object's rider counts are zero.

const AI_DESTROY_MISSION_RIDERS: Callee = Callee { addr: 0x0012_af68, f: ssx3_ee::s00::f_0012af68 };
const AI_DESTROY_ACTOR_RIDERS: Callee = Callee { addr: 0x0012_aed0, f: ssx3_ee::s00::f_0012aed0 };
const AI_DESTROY_COMPUTER_RIDERS: Callee = Callee { addr: 0x0012_ae38, f: ssx3_ee::s00::f_0012ae38 };
const AI_DESTROY_ALL_RIDERS: Callee = Callee { addr: 0x0012_b090, f: ssx3_ee::s00::f_0012b090 };
const AI_CREATE_COMPUTER_RIDERS: Callee = Callee { addr: 0x0012_9e20, f: ssx3_ee::s00::f_00129e20 };
const AI_CREATE_MISSION_RIDER: Callee = Callee { addr: 0x0012_9c00, f: ssx3_ee::s00::f_00129c00 };
const AI_RESET_RIDERS: Callee = Callee { addr: 0x0012_97c8, f: ssx3_ee::s00::f_001297c8 };
const AI_RESET_RIDER_RELATIONS: Callee = Callee { addr: 0x0010_f3b8, f: ssx3_ee::s00::f_0010f3b8 };
const AI_FREE_ANIM_SOURCES: Callee = Callee { addr: 0x0012_b200, f: ssx3_ee::s00::f_0012b200 };

/// `void AI_SetupComputerRiders(AI *ai)` (0x128958): tear down any mission
/// riders, then recreate the computer riders.
pub fn ai_setup_computer_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(c.g[S0][0] as u32).off(0x94).sw(c, 0);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x88).lw(c);
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            AI_DESTROY_MISSION_RIDERS.call(c, 0x0012_8980)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        AI_CREATE_COMPUTER_RIDERS.call(c, 0x0012_8988)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_SetupMissionRider(AI *ai)` (0x128ac0): create the mission rider
/// and reset all rider records.
pub fn ai_setup_mission_rider(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        AI_CREATE_MISSION_RIDER.call(c, 0x0012_8ad4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        AI_RESET_RIDERS.call(c, 0x0012_8ae0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_RemoveActorRiders(AI *ai)` (0x12b000): destroy the actor riders
/// and refresh the rider relations.
pub fn ai_remove_actor_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        AI_DESTROY_ACTOR_RIDERS.call(c, 0x0012_b014)?;
        c.g[A0][0] = c.g[S0][0];
        AI_RESET_RIDER_RELATIONS.call(c, 0x0012_b01c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_RemoveComputerRiders(AI *ai)` (0x12b030): destroy the computer
/// riders, refresh relations and clear the actor state.
pub fn ai_remove_computer_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        AI_DESTROY_COMPUTER_RIDERS.call(c, 0x0012_b044)?;
        c.g[A0][0] = c.g[S0][0];
        AI_RESET_RIDER_RELATIONS.call(c, 0x0012_b04c)?;
        Addr(c.g[S0][0] as u32).off(0x10).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_RemoveMissionRiders(AI *ai)` (0x12b060): destroy the mission
/// riders, refresh relations and clear the actor state.
pub fn ai_remove_mission_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        AI_DESTROY_MISSION_RIDERS.call(c, 0x0012_b074)?;
        c.g[A0][0] = c.g[S0][0];
        AI_RESET_RIDER_RELATIONS.call(c, 0x0012_b07c)?;
        Addr(c.g[S0][0] as u32).off(0x10).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_dtor(AI *ai, int free_memory)` (0x1287b8): destroy all riders,
/// delete the focus/danger managers, and (when `free_memory` is odd) the AI
/// itself.
pub fn ai_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7b78));
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(c.g[S1][0] as u32).off(0xcc).sw(c, c.g[V0][0]);
        AI_DESTROY_ALL_RIDERS.call(c, 0x0012_87e0)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0xa4).lw(c);
        call::call_ptr(c, 0x0031_7e50, 0x0012_87e8)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0xa8).lw(c);
        call::call_ptr(c, 0x0031_7e50, 0x0012_87f0)?;
        c.g[S0][0] &= 1;
        let free_memory = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if free_memory {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call::call_ptr(c, 0x0031_7e50, 0x0012_8804)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of `AI_Destroy{Computer,Actor,Mission}Riders`: iterate the
/// `AIController` pointer array at `base_off` (count at `count_off`), call
/// each rider's vtable destructor (`rider[0x18] -> +0x6c0` slot), clear the
/// slot, then fold the count into the total at +0x78 and clear +`clear_off`.
fn ai_destroy_riders(
    c: &mut Cpu,
    count_off: u32,
    base_off: u32,
    clear_off: u32,
    head: u32,
    ret: u32,
) -> call::Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(count_off).lw(c);
    c.g[S2][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 1;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(base_off));
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = 0x3;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lw(c);
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
                c.g[A2][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = c.g[A2][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                call::call_ptr(c, target, ret)?;
            }
            c.cycles += 6;
            Addr(c.g[S0][0] as u32).sw(c, 0);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(count_off).lw(c);
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                crate::events::deliver_event_here(c, head);
            }
        }
    }
    c.cycles += 12;
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x78).lw(c);
    c.g[V1][0] = Addr(c.g[S1][0] as u32).off(count_off).lw(c);
    Addr(c.g[S1][0] as u32).off(clear_off).sw(c, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    Addr(c.g[S1][0] as u32).off(count_off).sw(c, 0);
    Addr(c.g[S1][0] as u32).off(0x78).sw(c, c.g[V0][0]);
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void AI_DestroyComputerRiders(AI *ai)` (0x12ae38).
pub fn ai_destroy_computer_riders(c: &mut Cpu) {
    call::guest(c, |c| ai_destroy_riders(c, 0x80, 0x48, 0x14, 0x0012_ae60, 0x0012_ae88))
}

/// `void AI_DestroyActorRiders(AI *ai)` (0x12aed0).
pub fn ai_destroy_actor_riders(c: &mut Cpu) {
    call::guest(c, |c| ai_destroy_riders(c, 0x84, 0x5c, 0x8c, 0x0012_aef8, 0x0012_af20))
}

/// `void AI_DestroyMissionRiders(AI *ai)` (0x12af68).
pub fn ai_destroy_mission_riders(c: &mut Cpu) {
    call::guest(c, |c| ai_destroy_riders(c, 0x88, 0x70, 0x90, 0x0012_af90, 0x0012_afb8))
}

/// `void AI_DestroyAllRiders(AI *ai)` (0x12b090): destroy the record and
/// actor riders (vtable destructors), the mission/computer/actor rider sets,
/// free the animation data sources and clear the counts.
pub fn ai_destroy_all_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x78).lw(c);
        c.g[S0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x28));
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
            loop {
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x790).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xfc));
                call::call_ptr(c, 0x0015_4a58, 0x0012_b0d8)?;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x78).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_b0c0);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x7c).lw(c);
        c.g[S2][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A1][0] = 0x3;
                if c.g[V0][0] != 0 {
                    c.cycles += 7;
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lw(c);
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
                    c.g[A2][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                    let target = c.g[A2][0] as u32;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call::call_ptr(c, target, 0x0012_b120)?;
                }
                c.cycles += 6;
                Addr(c.g[S0][0] as u32).sw(c, 0);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x7c).lw(c);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_b0f8);
                }
            }
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S1][0];
        AI_DESTROY_MISSION_RIDERS.call(c, 0x0012_b140)?;
        c.g[A0][0] = c.g[S1][0];
        AI_DESTROY_COMPUTER_RIDERS.call(c, 0x0012_b148)?;
        c.g[A0][0] = c.g[S1][0];
        AI_DESTROY_ACTOR_RIDERS.call(c, 0x0012_b150)?;
        Addr(c.g[S1][0] as u32).off(0x7c).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr(c.g[S1][0] as u32).off(0x78).sw(c, 0);
        AI_FREE_ANIM_SOURCES.call(c, 0x0012_b160)?;
        Addr(c.g[S1][0] as u32).off(0x74).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_FreeAnimDataSources(AI *ai)` (0x12b200): release animation data
/// source slots (the original walks a global source table; the loop only
/// ever visits slot 0).
pub fn ai_free_anim_sources(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[S2][0] & 0xff;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[GP][0] as u32).off(0xd08).lw(c);
            c.g[S1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A1][0] = c.g[S0][0];
            if c.g[S0][0] != 0 {
                c.cycles += 9;
                c.g[A0][0] = Addr(c.g[GP][0] as u32).off(0xd8c).lw(c);
                call::call_ptr(c, 0x0031_4fe8, 0x0012_b244)?;
                c.g[A0][0] = Addr(c.g[GP][0] as u32).off(0xd08).lw(c);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                Addr(c.g[V0][0] as u32).sw(c, 0);
                call::call_ptr(c, 0x0031_1110, 0x0012_b254)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x3;
                call::call_ptr(c, 0x0031_12c8, 0x0012_b260)?;
            }
            c.cycles += 2;
            let wrap = (c.g[S2][0] as i64) <= 0;
            c.g[V1][0] = c.g[S2][0] & 0xff;
            if !wrap {
                break;
            }
            if event_due(c) {
                crate::events::deliver_event_here(c, 0x0012_b220);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AI_CountRidersOnPath(AI *ai, int path)` (0x10fc30): count riders
/// whose `vtable[0x48/0x4c]` query returns non-null and whose `+0xab8`
/// path id equals `path`.
pub fn ai_count_riders_on_path(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0;
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
        c.g[S4][0] = c.g[A1][0];
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
            loop {
                c.cycles += 9;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call::call_ptr(c, target, 0x0010_fc84)?;
                let no_rider = c.g[V0][0] == 0;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                if !no_rider {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xab8).lw(c);
                    c.g[V1][0] ^= c.g[S4][0];
                    if c.g[V1][0] == 0 {
                        c.g[S1][0] = c.g[A0][0];
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0010_fc68);
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

const AI_RESET_RIDER_RELATIONS_2: Callee = Callee { addr: 0x0010_f398, f: ssx3_ee::s00::f_0010f398 };
const RIDER_RESET: Callee = Callee { addr: 0x0011_d390, f: ssx3_ee::s00::f_0011d390 };
const FOCUSMAN_RESET: Callee = Callee { addr: 0x0010_3358, f: ssx3_ee::s00::f_00103358 };
const DANGERMAN_RESET: Callee = Callee { addr: 0x0010_1688, f: ssx3_ee::s00::f_00101688 };

/// `void AI_ResetRiders(AI *ai, int clear)` (0x1297c8): reset relations and
/// zero the counters (+8, +0xc when `clear`), then reset every record
/// rider (via its vtable query) and every mission rider.
pub fn ai_reset_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        AI_RESET_RIDER_RELATIONS_2.call(c, 0x0012_97ec)?;
        let clear = c.g[S3][0] != 0;
        Addr(c.g[S2][0] as u32).off(0x8).sw(c, 0);
        if clear {
            c.cycles += 1;
            Addr(c.g[S2][0] as u32).off(0xc).sw(c, 0);
        }
        c.cycles += 6;
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
        Addr(c.g[S2][0] as u32).off(0x18).sw(c, u64::MAX);
        c.g[S1][0] = 0;
        let none = (c.g[V1][0] as i64) <= 0;
        Addr(c.g[S2][0] as u32).off(0x1c).sw(c, 0);
        if !none {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
            loop {
                c.cycles += 9;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call::call_ptr(c, target, 0x0012_9834)?;
                let reset = if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    true
                } else {
                    c.cycles += 2;
                    if c.g[S3][0] != 0 {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        true
                    } else {
                        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
                        false
                    }
                };
                if reset {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x790).lw(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xfc));
                    call::call_ptr(c, 0x0015_4a58, 0x0012_9858)?;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    RIDER_RESET.call(c, 0x0012_9860)?;
                    c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_9818);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x88).lw(c);
        let no_missions = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !no_missions {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            loop {
                c.cycles += 8;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lw(c);
                RIDER_RESET.call(c, 0x0012_9898)?;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x88).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_9888);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_Restart(AI *ai, int state)` (0x129768): reset the focus and
/// danger managers, clear the path pointer and reselect the race state.
pub fn ai_restart(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xa4).lw(c);
        FOCUSMAN_RESET.call(c, 0x0012_9788)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xa8).lw(c);
        DANGERMAN_RESET.call(c, 0x0012_9790)?;
        Addr(c.g[S0][0] as u32).off(0x98).sw(c, 0);
        c.g[A1][0] = c.g[S1][0];
        Addr(c.g[S0][0] as u32).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        AI_SET_RACE_STATE.call(c, 0x0012_97a4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        AI_RESET_RIDERS.call(c, 0x0012_97b0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

const RIDER_REFRESH_OVERLAPS: Callee = Callee { addr: 0x0012_0e50, f: ssx3_ee::s00::f_00120e50 };

/// `void AI_RefreshRiderOverlaps(AI *ai)` (0x12b788): refresh the overlap
/// sets of every record rider.
pub fn ai_refresh_rider_overlaps(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
        c.g[S1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            loop {
                c.cycles += 7;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                RIDER_REFRESH_OVERLAPS.call(c, 0x0012_b7c4)?;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_b7b8);
                }
            }
        }
        c.cycles += 6;
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
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{covered, machine};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 200;

    // ------------------------------------------------------------ syscalls

    /// Every syscall wrapper: fake syscalls (which may unwind), all argument
    /// registers scrambled.
    fn syscall_test(gen: GuestFn, hand: GuestFn, salt: u64) {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 1);
            // A few varied pointers in the argument registers.
            for i in 0..4 {
                c.g[[A0, A1, A2, A3][i]][0] = (DATA + (r.next_u32() & 0xfff0)) as u64;
            }
        };
        covered(gen, hand, SEEDS, &setup);
    }

    #[test]
    fn exit_matches() {
        syscall_test(ssx3_ee::s15::f_004239c0, exit, 1);
    }
    #[test]
    fn load_exec_ps2_matches() {
        syscall_test(ssx3_ee::s15::f_004239e0, load_exec_ps2, 2);
    }
    #[test]
    fn enable_intc_matches() {
        syscall_test(ssx3_ee::s15::f_00423ae0, enable_intc, 3);
    }
    #[test]
    fn disable_intc_matches() {
        syscall_test(ssx3_ee::s15::f_00423af0, disable_intc, 4);
    }
    #[test]
    fn enable_dmac_matches() {
        syscall_test(ssx3_ee::s15::f_00423b00, enable_dmac, 5);
    }
    #[test]
    fn disable_dmac_matches() {
        syscall_test(ssx3_ee::s15::f_00423b10, disable_dmac, 6);
    }
    #[test]
    fn i_enable_intc_matches() {
        syscall_test(ssx3_ee::s15::f_00423b40, i_enable_intc, 7);
    }
    #[test]
    fn i_disable_intc_matches() {
        syscall_test(ssx3_ee::s15::f_00423b50, i_disable_intc, 8);
    }
    #[test]
    fn i_enable_dmac_matches() {
        syscall_test(ssx3_ee::s15::f_00423b60, i_enable_dmac, 9);
    }
    #[test]
    fn i_disable_dmac_matches() {
        syscall_test(ssx3_ee::s15::f_00423b70, i_disable_dmac, 10);
    }
    #[test]
    fn start_thread_matches() {
        syscall_test(ssx3_ee::s15::f_00423bc0, start_thread, 11);
    }
    #[test]
    fn i_wakeup_thread_matches() {
        syscall_test(ssx3_ee::s15::f_00423ce0, i_wakeup_thread, 12);
    }
    #[test]
    fn i_suspend_thread_matches() {
        syscall_test(ssx3_ee::s15::f_00423d20, i_suspend_thread, 13);
    }
    #[test]
    fn exec_osd_matches() {
        syscall_test(ssx3_ee::s15::f_004241c0, exec_osd, 14);
    }
    #[test]
    fn init_tlb_matches() {
        syscall_test(ssx3_ee::s15::f_00424210, init_tlb, 15);
    }

    // ------------------------------------------------------------ leaves

    #[test]
    fn localeconv_r_matches() {
        check(ssx3_ee::s15::f_0041ec08, localeconv_r, SEEDS, &|c, seed| {
            let mut r = Rng(0x1111_2222 ^ seed as u64);
            scramble(c, &mut r);
        });
    }

    // ------------------------------------------------- reentrant wrappers

    /// Scrambled state with `reent` and the arguments the wrapper forwards.
    fn reent_args(c: &mut Cpu, seed: u32, salt: u64, n: usize) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = (DATA + 0x400) as u64;
        for i in 0..n {
            c.g[[A1, A2, A3][i]][0] = (r.next_u32() % 0x2000) as u64;
        }
        r
    }

    #[test]
    fn close_r_matches() {
        check(ssx3_ee::s15::f_0041fb98, close_r, SEEDS, &|c, seed| {
            reent_args(c, seed, 20, 1);
        });
    }

    #[test]
    fn fstat_r_matches() {
        check(ssx3_ee::s15::f_0041fbf0, fstat_r, SEEDS, &|c, seed| {
            reent_args(c, seed, 21, 3);
            c.g[A2][0] = (DATA + 0x800) as u64; // struct stat
        });
    }

    #[test]
    fn lseek_r_matches() {
        check(ssx3_ee::s15::f_0041fc50, lseek_r, SEEDS, &|c, seed| {
            reent_args(c, seed, 22, 3);
        });
    }

    #[test]
    fn open_r_matches() {
        check(ssx3_ee::s15::f_0041fcb0, open_r, SEEDS, &|c, seed| {
            reent_args(c, seed, 23, 3);
            c.g[A1][0] = (DATA + 0x100) as u64; // path
        });
    }

    #[test]
    fn read_r_matches() {
        check(ssx3_ee::s15::f_0041fd10, read_r, SEEDS, &|c, seed| {
            let mut r = reent_args(c, seed, 24, 3);
            // A non-zero fd keeps `_read` on its quick `-1` path.
            c.g[A1][0] = (1 + r.next_u32() % 8) as u64;
            c.g[A2][0] = (DATA + 0x1000) as u64; // buffer
        });
    }

    #[test]
    fn write_r_matches() {
        check(ssx3_ee::s15::f_0041fdd0, write_r, SEEDS, &|c, seed| {
            let mut r = reent_args(c, seed, 25, 3);
            // fd 0 or >= 3 keeps `_write` off the tty path.
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { (3 + r.next_u32() % 8) as u64 };
            c.g[A2][0] = (DATA + 0x1000) as u64; // buffer
        });
    }

    #[test]
    fn sbrk_r_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 26, 1);
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = (r.next_u32() % 0x1000) as u64;
        };
        covered(ssx3_ee::s15::f_0041fd70, sbrk_r, SEEDS, &setup);
    }

    // -------------------------------------------------------- stream hooks

    /// The common fields of a `FILE` cookie at `DATA + 0x400`: flags, fd,
    /// seek position and the reent pointer; returns the RNG for the caller
    /// to pick buffers and arguments.
    fn stream_cookie(c: &mut Cpu, seed: u32, salt: u64, fd: u16) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        let cookie = DATA + 0x400;
        c.g[A0][0] = cookie as u64;
        let flags = (r.next_u32() as u16) | if seed % 2 == 0 { 0x0100 } else { 0 };
        c.bus.write16(cookie + 0xc, flags);
        c.bus.write16(cookie + 0xe, fd);
        c.bus.write32(cookie + 0x50, r.next_u32());
        c.bus.write32(cookie + 0x54, DATA + 0x800);
        c.g[A1][0] = (DATA + 0x1000) as u64;
        c.g[A2][0] = (r.next_u32() % 0x1000) as u64;
        r
    }

    #[test]
    fn sread_matches() {
        check(ssx3_ee::s15::f_00417948, sread, SEEDS, &|c, seed| {
            let mut r = stream_cookie(c, seed, 30, 1);
            let fd = 1 + (r.next_u32() % 100) as u16;
            c.bus.write16(DATA + 0x400 + 0xe, fd);
        });
    }

    #[test]
    fn swrite_matches() {
        check(ssx3_ee::s15::f_004179b0, swrite, SEEDS, &|c, seed| {
            stream_cookie(c, seed, 31, if seed % 2 == 0 { 0 } else { 3 + (seed as u16 % 8) });
        });
    }

    #[test]
    fn sseek_matches() {
        check(ssx3_ee::s15::f_00417a30, sseek, SEEDS, &|c, seed| {
            stream_cookie(c, seed, 32, (seed as u16 % 32) + 3);
        });
    }

    #[test]
    fn sclose_matches() {
        check(ssx3_ee::s15::f_00417a98, sclose, SEEDS, &|c, seed| {
            stream_cookie(c, seed, 33, (seed as u16 % 32) + 3);
        });
    }

    // --------------------------------------------------------- malloc lock

    #[test]
    fn malloc_lock_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 40, 2);
            // Force GetThreadId's result so the owner-equals-self branch is
            // reachable.
            let tid = 1 + r.next_u32() % 0xffff;
            c.bus.write32(crate::difftest::fakes::LOG_RESULTS, tid);
            c.bus.write32(0x0045_0000 + 0x4ca8, if seed % 2 == 0 { tid } else { r.next_u32() });
            c.bus.write32(0x0045_0000 + 0x4cac, r.next_u32());
            c.bus.write32(0x0045_0000 + 0x5248, r.next_u32());
        };
        covered(ssx3_ee::s15::f_00418ca8, malloc_lock, SEEDS, &setup);
    }

    #[test]
    fn malloc_unlock_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 41, 1);
            c.bus.write32(0x0045_0000 + 0x4cac, if seed % 2 == 0 { 1 } else { 2 + r.next_u32() % 5 });
            c.bus.write32(0x0045_0000 + 0x5248, r.next_u32());
        };
        covered(ssx3_ee::s15::f_00418d08, malloc_unlock, SEEDS, &setup);
    }

    // ---------------------------------------------------------------- kill

    #[test]
    fn kill_matches() {
        check(ssx3_ee::s15::f_00424578, kill, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 42, 0);
            // Any pid but 1: the exit path runs the kernel's shutdown code,
            // which cannot run on a test machine.
            c.g[A0][0] = (2 + r.next_u32() % 30) as u64;
            c.g[A1][0] = (r.next_u32() % 32) as u64;
        });
    }

    // --------------------------------------------------- bignum / bit helpers

    #[test]
    fn bfree_matches() {
        check(ssx3_ee::s15::f_004205b8, bfree, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 60, 0);
            let reent = DATA + 0x400;
            let bucket = DATA + 0x1000;
            c.g[A0][0] = reent as u64;
            c.bus.write32(reent + 0x4c, bucket);
            let p = if seed % 5 == 0 { 0 } else { (DATA + 0x500) as u64 };
            c.g[A1][0] = p;
            if p != 0 {
                c.bus.write32(p as u32 + 0x4, seed % 4);
                c.bus.write32(p as u32, r.next_u32());
            }
        });
    }

    #[test]
    fn hi0bits_matches() {
        check(ssx3_ee::s15::f_00420830, hi0bits, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 61, 0);
            let x = match seed % 6 {
                0 => 0,
                1 => 1,
                2 => 0x8000_0000,
                3 => 0x0001_0000,
                _ => r.next_u32(),
            };
            c.g[A0][0] = x as u64;
        });
    }

    #[test]
    fn lo0bits_matches() {
        check(ssx3_ee::s15::f_004208b8, lo0bits, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 62, 0);
            let x = match seed % 6 {
                0 => 0,
                1 => 1,
                2 => 0x8000_0000,
                3 => 0x0000_0010,
                4 => 0x0000_0004,
                _ => r.next_u32(),
            };
            let cell = DATA + 0x600;
            c.bus.write32(cell, x);
            c.g[A0][0] = cell as u64;
        });
    }

    #[test]
    fn mcmp_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 63, 0);
            let a = DATA + 0x1000;
            let b = DATA + 0x2000;
            // Long enough that the compare loop reaches a scheduled event.
            let ka = 16 + r.next_u32() % 96;
            let kb = if seed % 5 == 0 { 1 + r.next_u32() % 96 } else { ka };
            c.bus.write32(a + 0x10, ka);
            c.bus.write32(b + 0x10, kb);
            let differ_at = if seed % 2 == 0 { u32::MAX } else { r.next_u32() % ka };
            for i in 0..96 {
                let x = r.next_u32();
                let y = if i == differ_at { x ^ (1 << (r.next_u32() % 32)) } else { x };
                c.bus.write32(a + 0x14 + 4 * i, x);
                c.bus.write32(b + 0x14 + 4 * i, y);
            }
            c.g[A0][0] = a as u64;
            c.g[A1][0] = b as u64;
            schedule(c, &mut r, seed, 0x0042_0e28)
        };
        check_events(ssx3_ee::s15::f_00420e28, mcmp, SEEDS, &setup);
    }

    #[test]
    fn ulp_matches() {
        check(ssx3_ee::s15::f_00421020, ulp, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 64, 0);
            let x = match seed % 6 {
                0 => 0,
                1 => 0x3ff0_0000_0000_0000,
                2 => 0x0010_0000_0000_0000,
                3 => 0x7ff0_0000_0000_0000,
                4 => 0x7fef_ffff_ffff_ffff,
                _ => r.next_u32() as u64 | ((r.next_u32() as u64) << 32),
            };
            c.g[A0][0] = x;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn mprec_log10_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 65, 0);
            let exp = match seed % 5 {
                0 => (r.next_u32() % 4) as i32 - 2, // <= 0
                1 => (r.next_u32() % 24) as i32,
                2 => 24 + (r.next_u32() % 8) as i32,
                _ => (r.next_u32() % 40) as i32,
            };
            c.g[A0][0] = sx(exp as u32);
            schedule(c, &mut r, seed, 0x0042_14c0)
        };
        check_events(ssx3_ee::s15::f_004214c0, mprec_log10, SEEDS, &setup);
    }

    #[test]
    fn fixdfdi_matches() {
        check(ssx3_ee::s15::f_0042d320, fixdfdi, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 66, 0);
            let x = match seed % 6 {
                0 => 0x3ff0_0000_0000_0000,
                1 => 0xbff0_0000_0000_0000, // negative
                2 => 0x41e0_0000_0000_0000,
                3 => 0xc1e0_0000_0000_0000,
                _ => r.next_u32() as u64 | ((r.next_u32() as u64) << 32),
            };
            c.g[A0][0] = x;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn fixunsdfdi_matches() {
        check(ssx3_ee::s15::f_0042d380, fixunsdfdi, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 67, 0);
            let x = match seed % 7 {
                0 => 0x3ff0_0000_0000_0000,
                1 => 0xbff0_0000_0000_0000, // negative
                2 => 0x4000_0000_0000_0000, // high bit of the int part
                3 => 0x4330_0000_0000_0000, // large
                4 => 0x0000_0000_0000_0000,
                _ => r.next_u32() as u64 | ((r.next_u32() as u64) << 32),
            };
            c.g[A0][0] = x;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    fn cache_test(gen: GuestFn, hand: GuestFn, entry: u32, salt: u64) {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, salt, 0);
            c.g[A0][0] = (r.next_u32() & 0xffff) as u64;
            c.g[A1][0] = (r.next_u32() & 0xffff) as u64;
            schedule(c, &mut r, seed, entry)
        };
        check_events(gen, hand, SEEDS, &setup);
    }

    #[test]
    fn sync_dcache_matches() {
        cache_test(ssx3_ee::s15::f_004245f0, sync_dcache, 0x0042_45f0, 70);
    }

    #[test]
    fn invalid_dcache_matches() {
        cache_test(ssx3_ee::s15::f_00424730, invalid_dcache, 0x0042_4730, 71);
    }

    #[test]
    fn sflags_matches() {
        check(ssx3_ee::s15::f_00420180, sflags, SEEDS, &|c, seed| {
            machine(c, seed, 72, 0);
            let mode = DATA + 0x400;
            let modes: [&[u8]; 10] = [
                b"r", b"w", b"a", b"x", b"s", b"r+", b"w+", b"a+", b"r+b", b"wb",
            ];
            let m = modes[(seed as usize) % modes.len()];
            for (i, b) in m.iter().enumerate() {
                c.bus.write8(mode + i as u32, *b);
            }
            c.bus.write8(mode + m.len() as u32, 0);
            c.g[A0][0] = (DATA + 0x500) as u64;
            c.g[A1][0] = mode as u64;
            c.g[A2][0] = (DATA + 0x504) as u64;
        });
    }

    // -------------------------------------------------------- AI race state

    /// The dispatch table at 0x450000 + 0x6ca0, valid or pointed at an
    /// unknown address.
    fn race_state_table(c: &mut Cpu, seed: u32) {
        const VALID: [u32; 7] = [
            0x0011_3b74, 0x0011_3b94, 0x0011_3ba8, 0x0011_3bbc, 0x0011_3bd0, 0x0011_3be4, 0x0011_3bf8,
        ];
        for (i, v) in VALID.iter().enumerate() {
            let val = if seed % 3 == 0 { 0xdead_beef } else { *v };
            c.bus.write32(0x0045_0000 + 0x6ca0 + 4 * i as u32, val);
        }
    }

    #[test]
    fn ai_select_race_state_matches() {
        check(ssx3_ee::s00::f_00113b48, ai_select_race_state, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 73, 0);
            race_state_table(c, seed);
            c.pc = 0x0011_3b48;
            let ai = DATA + 0x400;
            c.bus.write32(ai, 1 + r.next_u32() % 8);
            c.bus.write32(ai + 0x98, r.next_u32());
            c.g[A0][0] = ai as u64;
            c.g[A1][0] = (seed % 10) as u64;
        });
    }

    #[test]
    fn ai_set_race_state_matches() {
        check(ssx3_ee::s00::f_00113b10, ai_set_race_state, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 74, 0);
            race_state_table(c, seed);
            let ai = DATA + 0x400;
            let state = (seed % 10) as u32;
            let cur = if seed % 4 == 0 { state } else { r.next_u32() % 10 };
            c.bus.write32(ai, cur);
            c.bus.write32(ai + 0x98, r.next_u32());
            c.g[A0][0] = ai as u64;
            c.g[A1][0] = state as u64;
        });
    }

    // ---------------------------------------------------- AI setup/teardown

    /// An AI object at `DATA + 0x400` with the rider counts zeroed (so the
    /// destroy/create/reset callees take their short paths), plus noise.
    fn ai_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        let ai = DATA + 0x400;
        fill(c, &mut r, ai + 0x70, 0x30);
        for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x94] {
            c.bus.write32(ai + off, 0);
        }
        c.bus.write32(ai + 0xa4, DATA + 0x1000);
        c.bus.write32(ai + 0xa8, DATA + 0x2000);
        c.g[A0][0] = ai as u64;
        r
    }

    #[test]
    fn ai_remove_actor_riders_matches() {
        check(ssx3_ee::s00::f_0012b000, ai_remove_actor_riders, SEEDS, &|c, seed| {
            ai_setup(c, seed, 77, 2);
        });
    }

    #[test]
    fn ai_remove_computer_riders_matches() {
        check(ssx3_ee::s00::f_0012b030, ai_remove_computer_riders, SEEDS, &|c, seed| {
            ai_setup(c, seed, 78, 2);
        });
    }

    #[test]
    fn ai_remove_mission_riders_matches() {
        check(ssx3_ee::s00::f_0012b060, ai_remove_mission_riders, SEEDS, &|c, seed| {
            ai_setup(c, seed, 79, 2);
        });
    }

    #[test]
    fn ai_dtor_matches() {
        check(ssx3_ee::s00::f_001287b8, ai_dtor, SEEDS, &|c, seed| {
            let mut r = ai_setup(c, seed, 80, 3);
            c.g[A1][0] = (r.next_u32() & 1) as u64;
        });
    }

    /// Build `count` rider pointers at `ai + base_off`, each with the vtable
    /// chain the destructor walks; the destructor target is an unregistered
    /// address that lands in the fake-callee hook.
    fn ai_rider_entries(c: &mut Cpu, base_off: u32, count: u32) {
        let ai = DATA + 0x400;
        for i in 0..count {
            let rider = DATA + 0x2000 + i * 0x100;
            let vtobj = DATA + 0x8000 + i * 0x100;
            let vt = DATA + 0x9000;
            c.bus.write32(ai + base_off + 4 * i, rider);
            c.bus.write32(rider + 0x18, vtobj);
            c.bus.write32(vtobj + 0x6c0, vt);
            c.bus.write16(vt + 8, 0);
            c.bus.write32(vt + 0xc, 0x0000_5000);
        }
    }

    #[test]
    fn ai_destroy_computer_riders_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 81, 1);
            let ai = DATA + 0x400;
            let count = 1 + r.next_u32() % 8;
            c.bus.write32(ai + 0x80, count);
            c.bus.write32(ai + 0x78, count + (r.next_u32() % 4));
            c.bus.write32(ai + 0x14, r.next_u32());
            ai_rider_entries(c, 0x48, count);
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_ae38)
        };
        check_events(ssx3_ee::s00::f_0012ae38, ai_destroy_computer_riders, SEEDS, &setup);
    }

    #[test]
    fn ai_destroy_actor_riders_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 82, 1);
            let ai = DATA + 0x400;
            let count = 1 + r.next_u32() % 6;
            c.bus.write32(ai + 0x84, count);
            c.bus.write32(ai + 0x78, count + (r.next_u32() % 4));
            c.bus.write32(ai + 0x8c, r.next_u32());
            ai_rider_entries(c, 0x5c, count);
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_aed0)
        };
        check_events(ssx3_ee::s00::f_0012aed0, ai_destroy_actor_riders, SEEDS, &setup);
    }

    #[test]
    fn ai_destroy_mission_riders_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 83, 1);
            let ai = DATA + 0x400;
            let count = 1 + r.next_u32() % 4;
            c.bus.write32(ai + 0x88, count);
            c.bus.write32(ai + 0x78, count + (r.next_u32() % 4));
            c.bus.write32(ai + 0x90, r.next_u32());
            ai_rider_entries(c, 0x70, count);
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_af68)
        };
        check_events(ssx3_ee::s00::f_0012af68, ai_destroy_mission_riders, SEEDS, &setup);
    }

    #[test]
    fn ai_destroy_all_riders_matches() {
        check(ssx3_ee::s00::f_0012b090, ai_destroy_all_riders, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 84, 4);
            let ai = DATA + 0x400;
            fill(c, &mut r, ai, 0x60);
            for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88] {
                c.bus.write32(ai + off, 0);
            }
            c.bus.write32(DATA + 0x100, r.next_u32()); // free-anim slot 0 = null
            c.bus.write32(GAME_GP + 0xd08, 0);
            c.g[A0][0] = ai as u64;
        });
    }

    #[test]
    fn ai_free_anim_sources_matches() {
        check(ssx3_ee::s00::f_0012b200, ai_free_anim_sources, SEEDS, &|c, seed| {
            machine(c, seed, 85, 3);
            c.bus.write32(GAME_GP + 0xd08, 0);
            c.g[A0][0] = (DATA + 0x400) as u64;
        });
    }

    #[test]
    fn ai_count_riders_on_path_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 86, 1);
            let ai = DATA + 0x400;
            let count = 1 + r.next_u32() % 4;
            c.bus.write32(ai + 0x78, count);
            let path = r.next_u32() % 4;
            for i in 0..count {
                let rider = DATA + 0x2000 + i * 0x100;
                let vt = DATA + 0x9000;
                c.bus.write32(ai + 0x28 + 4 * i, rider);
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write16(vt + 0x48, 0);
                c.bus.write32(vt + 0x4c, 0x0000_5000);
                // Half the riders are on the queried path.
                c.bus.write32(rider + 0xab8, if i % 2 == 0 { path } else { r.next_u32() });
            }
            c.g[A0][0] = ai as u64;
            c.g[A1][0] = path as u64;
            schedule(c, &mut r, seed, 0x0010_fc30)
        };
        check_events(ssx3_ee::s00::f_0010fc30, ai_count_riders_on_path, SEEDS, &setup);
    }

    #[test]
    fn ai_reset_riders_matches() {
        check(ssx3_ee::s00::f_001297c8, ai_reset_riders, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 87, 2);
            let ai = DATA + 0x400;
            fill(c, &mut r, ai, 0x100);
            c.bus.write32(ai + 0x78, 0);
            c.bus.write32(ai + 0x88, 0);
            c.g[A0][0] = ai as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn ai_restart_matches() {
        check(ssx3_ee::s00::f_00129768, ai_restart, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 88, 3);
            let ai = DATA + 0x400;
            fill(c, &mut r, ai, 0x100);
            c.bus.write32(ai + 0x78, 0);
            c.bus.write32(ai + 0x88, 0);
            // Focus and danger managers with zero counts / in-range fields.
            let focus = DATA + 0x3000;
            let danger = DATA + 0x2000;
            fill(c, &mut r, focus, 0x120);
            fill(c, &mut r, danger, 0xc00);
            c.bus.write32(focus + 0xd8, 0);
            c.bus.write32(ai + 0xa4, focus);
            c.bus.write32(ai + 0xa8, danger);
            c.g[A0][0] = ai as u64;
            c.g[A1][0] = (r.next_u32() % 10) as u64;
        });
    }

    #[test]
    fn ai_refresh_rider_overlaps_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 89, 1);
            let ai = DATA + 0x400;
            let count = r.next_u32() % 4;
            c.bus.write32(ai + 0x78, count);
            for i in 0..count {
                let rider = DATA + 0x2000 + i * 0x100;
                c.bus.write32(ai + 0x28 + 4 * i, rider);
            }
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_b788)
        };
        check_events(ssx3_ee::s00::f_0012b788, ai_refresh_rider_overlaps, SEEDS, &setup);
    }

    #[test]
    fn fstat_matches() {
        check(ssx3_ee::s15::f_00424558, fstat, SEEDS, &|c, seed| {
            let mut r = Rng(0x3333_4444 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (r.next_u32() % 8) as u64;
            c.g[A1][0] = (DATA + (r.next_u32() & 0xfff0)) as u64;
        });
    }
}
