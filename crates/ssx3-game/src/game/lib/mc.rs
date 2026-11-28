//! `src/lib/mc.cpp`: hand-written memory card (`libmc_*`, `sceMc*`) replacements.

use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_9b10, sce_mc_end, "sceMcEnd"),
    (0x0040_9b60, libmc_get_rpc_buffers, "libmc_GetRpcBuffers"),
    (
        0x0040_9b90,
        sce_mc_change_thread_priority,
        "sceMcChangeThreadPriority",
    ),
    (0x0040_9c48, sce_mc_get_slot_max, "sceMcGetSlotMax"),
    (0x0040_9d10, sce_mc_open, "sceMcOpen"),
    (0x0040_9e38, sce_mc_mkdir, "sceMcMkdir"),
    (0x0040_9e70, sce_mc_close, "sceMcClose"),
    (0x0040_9f28, sce_mc_seek, "sceMcSeek"),
    (0x0040_a090, sce_mc_read, "sceMcRead"),
    (0x0040_a1a8, sce_mc_write, "sceMcWrite"),
    (0x0040_a328, libmc_delay_thread, "libmc_DelayThread"),
    (0x0040_a360, sce_mc_sync, "sceMcSync"),
    (0x0040_a498, sce_mc_get_info, "sceMcGetInfo"),
    (0x0040_a618, sce_mc_ud_check_new_card, "sceMcUdCheckNewCard"),
    (0x0040_a688, sce_mc_get_dir, "sceMcGetDir"),
    (0x0040_a7d0, libmc_chdir_callback, "libmc_ChdirCallback"),
    (0x0040_a858, sce_mc_chdir, "sceMcChdir"),
    (0x0040_a998, sce_mc_format, "sceMcFormat"),
    (0x0040_aa68, sce_mc_delete, "sceMcDelete"),
    (0x0040_ab80, sce_mc_flush, "sceMcFlush"),
    (0x0040_ac38, sce_mc_set_file_info, "sceMcSetFileInfo"),
    (0x0040_ae00, sce_mc_rename, "sceMcRename"),
    (0x0040_af58, sce_mc_unformat, "sceMcUnformat"),
    (0x0040_b028, sce_mc_get_ent_space, "sceMcGetEntSpace"),
    (0x0040_a320, libmc_delay_alarm, "libmc_DelayAlarmHandler"),
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `sceMcOpen` (`0x0040_9d10`).
    pub(crate) const MC_OPEN: Callee = Callee {
        addr: 0x0040_9d10,
        f: ee::s15::sceMcOpen,
    };

    /// `libmc_DelayThread` (`0x0040_a328`).
    pub(crate) const MC_DELAY_THREAD: Callee = Callee {
        addr: 0x0040_a328,
        f: ee::s15::libmc_DelayThread,
    };

    pub(super) use crate::game::lib::libc::callee::MEMCPY;

    pub(super) use crate::game::lib::libc::callee::STRLEN;

    pub(super) use crate::game::lib::sif::callee::STRNCPY;

    pub(super) use crate::game::lib::cdvd::callee::SET_ALARM;

    pub(super) use crate::game::lib::sif::callee::DELETE_SEMA;

    pub(crate) use crate::game::lib::gcc::SIGNAL_SEMA;

    pub(super) use crate::game::lib::usbkb::callee::WAIT_SEMA;

    /// `PollSema` (`0x0042_3df0`).
    pub(crate) const POLL_SEMA: Callee = Callee {
        addr: 0x0042_3df0,
        f: ee::s15::PollSema,
    };

    pub(super) use crate::game::lib::kernel::callee::FLUSH_CACHE;

    pub(super) use crate::game::lib::sif::callee::PRINTF;

    pub(super) use crate::game::lib::sif::callee::WRITEBACK;

    pub(super) use crate::game::lib::sif::callee::CALL_RPC;

    pub(super) use crate::game::lib::cdvd::callee::CHECK_STAT;
}

/// Delay-thread semaphore id (read by `libmc_DelayThread` itself).
pub(crate) const LIBMC_DELAY_SEMA: u32 = 0x0045_33c8;

/// Memory-card RPC client block (all `sceMc*` share it); +0x24 is the
/// up word (0: every `sceMc*` returns -100).
pub(crate) const LIBMC_CLIENT: u32 = 0x0052_71c0;

/// Memory-card RPC receive buffer (`t1` of the `sceSifCallRpc` calls).
pub(crate) const MC_RECV: u32 = 0x0052_8780;

/// `void libmc_ChdirCallback(char *pwd)` (0x40a7d0): `sceMcChdir`'s RPC end
/// callback. When `pwd` is non-null, copies the directory string the IOP left
/// in the uncached buffer at `0x527780` into `pwd` (at most 0x3ff bytes) and
/// NUL-terminates it.
pub fn libmc_chdir_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        if c.g[S1][0] != 0 {
            c.cycles += 9;
            c.g[S2][0] = 0x520000;
            c.g[V0][0] = 0x20000000;
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x7780));
            c.g[S0][0] = c.g[V1][0] | c.g[V0][0];
            c.g[RA][0] = sx(0x0040_a804);
            c.g[A0][0] = c.g[S0][0];
            callee::STRLEN.call(c, 0x0040_a804)?;
            c.g[V0][0] = (c.g[V0][0] < 0x400u64) as u64;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = 0x3ff;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0040_a818);
                c.g[A0][0] = c.g[S0][0];
                callee::STRLEN.call(c, 0x0040_a818)?;
                c.g[S0][0] = c.g[V0][0];
            }
            c.cycles += 8;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x7780));
            c.g[A1][0] = 0x20000000;
            c.g[A1][0] = c.g[V0][0] | c.g[A1][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0040_a834);
            c.g[A2][0] = c.g[S0][0];
            callee::MEMCPY.call(c, 0x0040_a834)?;
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.sb(c.g[V1][0] as u32, 0u64);
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x30);
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `void libmc_DelayThread(unsigned int ticks)` (0x40a328): sleep `ticks`
/// (low 16 bits) via a one-shot alarm that signals the libmc delay sema,
/// then wait for that sema. Returns the `WaitSema` result (0 on success).
pub fn libmc_delay_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[A1][0] = sx(0x0041_0000u32.wrapping_sub(0x5ce0));
        c.g[S0][0] = 0x450000;
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[A2][0] = Addr(LIBMC_DELAY_SEMA).lw(c);
        c.g[RA][0] = sx(0x0040_a34c);
        c.g[A0][0] &= 0xffff;
        callee::SET_ALARM.call(c, 0x0040_a34c)?;
        c.g[A0][0] = Addr(LIBMC_DELAY_SEMA).lw(c);
        frame.restore_ra(c, 0x10);
        c.g[S0][0] = c.ld(frame.at(c, 0));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = callee::WAIT_SEMA.addr;
        (callee::WAIT_SEMA.f)(c);
        Ok(())
    })
}

/// `void *libmc_GetRpcBuffers(int *buf_out, int *func_out)` (0x409b60): hand
/// libmc state to the caller: `*buf_out` gets the RPC receive buffer
/// (`0x528780`) and `*func_out` the address of the last-function word
/// (`0x4533c0`); the libmc sema id (`*0x4533c4`) is latched at receive buffer
/// `+ 0x3c`. Returns the RPC client block (`0x5271c0`). A leaf: no calls, no
/// branches.
pub fn libmc_get_rpc_buffers(c: &mut Cpu) {
    c.cycles += 12;
    c.g[A2][0] = 0x530000;
    c.g[V0][0] = 0x450000;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7880));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x33c0));
    Addr(c.g[A0][0] as u32).sw(c, c.g[A2][0]);
    Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
    c.g[A0][0] = 0x450000;
    c.g[V0][0] = 0x520000;
    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x33c4)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x71c0));
    let ra = c.g[RA][0] as u32;
    Addr((c.g[A2][0] as u32).wrapping_add(0x3c)).sw(c, c.g[V1][0]);
    c.pc = ra;
}

/// `int sceMcChangeThreadPriority(int prio)` (0x409b90): change the
/// memory-card RPC thread's priority. Returns -100 when the RPC is not
/// bound, -200 when the libmc sema is busy, else the RPC result.
pub fn sce_mc_change_thread_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x520000;
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x71c0));
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x24)).lw(c);
        let bound = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !bound {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S2][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_9bcc);
            c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x33c4)).lw(c);
            callee::POLL_SEMA.call(c, 0x0040_9bcc)?;
            let free = (c.g[V0][0] as i64) >= 0;
            c.g[A3][0] = 0x520000;
            if !free {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            } else {
                c.cycles += 15;
                c.g[T1][0] = 0x530000;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7240));
                c.g[A0][0] = c.g[S1][0];
                Addr((c.g[A3][0] as u32).wrapping_add(0x14)).sw(c, c.g[S0][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7880));
                Addr(c.g[SP][0] as u32).sw(c, 0);
                c.g[A1][0] = 0x14;
                c.g[A2][0] = 1;
                c.g[T0][0] = 0x30;
                c.g[RA][0] = sx(0x0040_9c0c);
                c.g[T2][0] = 4;
                c.g[T3][0] = 0;
                callee::CALL_RPC.call(c, 0x0040_9c0c)?;
                c.g[S0][0] = c.g[V0][0];
                let rpc_ok = c.g[S0][0] == 0;
                c.g[V1][0] = 0x450000;
                if rpc_ok {
                    c.cycles += 3;
                    c.g[V0][0] = 0x14;
                    Addr((c.g[V1][0] as u32).wrapping_add(0x33c0)).sw(c, c.g[V0][0]);
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_9c2c);
                    c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x33c4)).lw(c);
                    callee::SIGNAL_SEMA.call(c, 0x0040_9c2c)?;
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x40);
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcChdir(int port, int slot, const char *newdir, char *pwd)`
/// (0x40a858): change directory to `newdir` (RPC function 12; at most 0x3ff
/// bytes copied into the send buffer). The IOP's directory string comes back
/// in the buffer at `0x527780`, which `libmc_ChdirCallback` copies to `pwd`
/// when the call ends. Returns -210 for a null or empty `newdir`, -100 when
/// the RPC is not bound, -200 when the libmc sema is busy, else the RPC
/// result (0 records function 12 as the last RPC).
pub fn sce_mc_chdir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        c.sd(frame.at(c, 0x80), c.g[S7][0]);
        c.sd(frame.at(c, 0x70), c.g[S6][0]);
        c.g[S7][0] = sx(LIBMC_CLIENT);
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.g[S6][0] = c.g[A3][0];
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[S4][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S3][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x90), c.g[RA][0]);
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x24));
        c.g[S2][0] = c.g[A2][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S5][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_a8b4);
            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x33c4));
            callee::POLL_SEMA.call(c, 0x0040_a8b4)?;
            let sema_ok = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            if !sema_ok {
                // V0 = -200 already; fall through to the epilogue.
            } else {
                c.cycles += 2;
                let mut proceed = false;
                if c.g[S2][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                    proceed = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x520000;
                }
                if !proceed {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_a8d8);
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x33c4));
                    callee::SIGNAL_SEMA.call(c, 0x0040_a8d8)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_ff2e;
                } else {
                    c.cycles += 29;
                    c.g[S0][0] = 0x520000;
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7270));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x7780));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x7270), c.g[S3][0]);
                    c.g[A1][0] = c.g[S2][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[S4][0]);
                    c.g[RA][0] = sx(0x0040_a908);
                    c.g[A2][0] = 0x3ff;
                    callee::STRNCPY.call(c, 0x0040_a908)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x413), 0u64);
                    c.g[RA][0] = sx(0x0040_a918);
                    c.g[A1][0] = 0x400;
                    callee::WRITEBACK.call(c, 0x0040_a918)?;
                    c.g[T1][0] = sx(MC_RECV);
                    c.g[T3][0] = 0x410000;
                    c.sw(frame.at(c, 0), c.g[S6][0]);
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A3][0] = c.g[S1][0];
                    c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x5830));
                    c.g[A1][0] = 0xc;
                    c.g[A2][0] = 0x1;
                    c.g[T0][0] = 0x414;
                    c.g[RA][0] = sx(0x0040_a948);
                    c.g[T2][0] = 0x4;
                    callee::CALL_RPC.call(c, 0x0040_a948)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = 0x450000;
                    if c.g[S0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = 0xc;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x33c0), c.g[V0][0]);
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x0040_a968);
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x33c4));
                        callee::SIGNAL_SEMA.call(c, 0x0040_a968)?;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                }
            }
        }
        c.cycles += 11;
        frame.restore_ra(c, 0x90);
        c.g[S7][0] = c.ld(frame.at(c, 0x80));
        c.g[S6][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcClose(int fd)` (0x409e70): close `fd`. Returns -100 when the
/// RPC is not bound, -200 when the libmc sema is busy, else the RPC result.
pub fn sce_mc_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x520000;
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x71c0));
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x24)).lw(c);
        let bound = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !bound {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S2][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_9eac);
            c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x33c4)).lw(c);
            callee::POLL_SEMA.call(c, 0x0040_9eac)?;
            let free = (c.g[V0][0] as i64) >= 0;
            c.g[A3][0] = 0x520000;
            if !free {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            } else {
                c.cycles += 15;
                c.g[T1][0] = 0x530000;
                Addr((c.g[A3][0] as u32).wrapping_add(0x7240)).sw(c, c.g[S0][0]);
                c.g[A0][0] = c.g[S1][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7240));
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7880));
                Addr(c.g[SP][0] as u32).sw(c, 0);
                c.g[A1][0] = 3;
                c.g[A2][0] = 1;
                c.g[T0][0] = 0x30;
                c.g[RA][0] = sx(0x0040_9eec);
                c.g[T2][0] = 4;
                c.g[T3][0] = 0;
                callee::CALL_RPC.call(c, 0x0040_9eec)?;
                c.g[S0][0] = c.g[V0][0];
                let rpc_failed = c.g[S0][0] != 0;
                c.g[V1][0] = 0x450000;
                if rpc_failed {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_9f0c);
                    c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x33c4)).lw(c);
                    callee::SIGNAL_SEMA.call(c, 0x0040_9f0c)?;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 3;
                    Addr((c.g[V1][0] as u32).wrapping_add(0x33c0)).sw(c, c.g[V0][0]);
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x40);
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcDelete(int port, int slot, const char *name)` (0x40aa68): delete
/// `name` (RPC function 15; at most 0x3ff bytes copied into the send buffer).
/// Returns -210 for a null or empty name, -100 when the RPC is not bound,
/// -200 when the libmc sema is busy, else the RPC result (0 records function
/// 15 as the last RPC).
pub fn sce_mc_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.g[S5][0] = sx(LIBMC_CLIENT);
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[S4][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S3][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x70), c.g[RA][0]);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = c.g[A2][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S2][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_aab8);
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x33c4));
            callee::POLL_SEMA.call(c, 0x0040_aab8)?;
            let sema_ok = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            if !sema_ok {
                // V0 = -200 already; fall through to the epilogue.
            } else {
                c.cycles += 2;
                let mut proceed = false;
                if c.g[S1][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                    proceed = c.g[V0][0] != 0;
                    c.g[S0][0] = 0x520000;
                }
                if !proceed {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_aadc);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x33c4));
                    callee::SIGNAL_SEMA.call(c, 0x0040_aadc)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_ff2e;
                } else {
                    c.cycles += 24;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x7284));
                    c.g[A2][0] = 0x3ff;
                    c.g[RA][0] = sx(0x0040_aaf8);
                    c.g[A0][0] = c.g[S0][0];
                    callee::STRNCPY.call(c, 0x0040_aaf8)?;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x14));
                    c.sw((c.g[S0][0] as u32).wrapping_sub(0x14), c.g[S3][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S4][0]);
                    c.g[T1][0] = sx(MC_RECV);
                    c.sb((c.g[V1][0] as u32).wrapping_add(0x413), 0u64);
                    c.g[A0][0] = c.g[S5][0];
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[A3][0] = c.g[V1][0];
                    c.sw(frame.at(c, 0), 0u64);
                    c.g[A1][0] = 0xf;
                    c.g[A2][0] = 0x1;
                    c.g[T0][0] = 0x414;
                    c.g[T2][0] = 0x4;
                    c.g[RA][0] = sx(0x0040_ab38);
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x0040_ab38)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = 0x450000;
                    if c.g[S0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = 0xf;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x33c0), c.g[V0][0]);
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x0040_ab58);
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x33c4));
                        callee::SIGNAL_SEMA.call(c, 0x0040_ab58)?;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                }
            }
        }
        c.cycles += 9;
        frame.restore_ra(c, 0x70);
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcEnd(void)` (0x409b10): shut the memory-card RPC down,
/// deleting both libmc semas. Returns 1 in all cases.
pub fn sce_mc_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S0][0] = 0x450000;
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x33c4)).lw(c);
        let initialised = (c.g[A0][0] as i64) >= 0;
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        if initialised {
            c.cycles += 8;
            c.g[RA][0] = sx(0x0040_9b34);
            callee::DELETE_SEMA.call(c, 0x0040_9b34)?;
            c.g[V1][0] = 0x450000;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x33c8)).lw(c);
            c.g[RA][0] = sx(0x0040_9b48);
            Addr((c.g[S0][0] as u32).wrapping_add(0x33c4)).sw(c, c.g[V0][0]);
            callee::DELETE_SEMA.call(c, 0x0040_9b48)?;
            c.g[RA][0] = c.ld(frame.at(c, 0x10));
        }
        c.cycles += 4;
        c.g[V0][0] = 1;
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcFlush(int fd)` (0x40ab80): flush `fd` (RPC function 10). Returns
/// -100 when the RPC is not bound, -200 when the libmc sema is busy, else the
/// RPC result (0 records function 10 as the last RPC).
pub fn sce_mc_flush(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[V1][0] = 0x520000;
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x71c0));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S2][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_abbc);
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x33c4));
            callee::POLL_SEMA.call(c, 0x0040_abbc)?;
            let sema_ok = (c.g[V0][0] as i64) >= 0;
            c.g[A3][0] = 0x520000;
            if !sema_ok {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            } else {
                c.cycles += 15;
                c.g[T1][0] = sx(MC_RECV);
                c.sw((c.g[A3][0] as u32).wrapping_add(0x7240), c.g[S0][0]);
                c.g[A0][0] = c.g[S1][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7240));
                c.sw(frame.at(c, 0), 0u64);
                c.g[A1][0] = 0xa;
                c.g[A2][0] = 0x1;
                c.g[T0][0] = 0x30;
                c.g[T2][0] = 0x4;
                c.g[RA][0] = sx(0x0040_abfc);
                c.g[T3][0] = 0u64;
                callee::CALL_RPC.call(c, 0x0040_abfc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = 0x450000;
                if c.g[S0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = 0xa;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x33c0), c.g[V0][0]);
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_ac1c);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x33c4));
                    callee::SIGNAL_SEMA.call(c, 0x0040_ac1c)?;
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 6;
        frame.restore_ra(c, 0x40);
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcFormat(int port, int slot)` (0x40a998): format the card (RPC
/// function 16). Returns -100 when the RPC is not bound, -200 when the libmc
/// sema is busy, else the RPC result (0 records function 16 as the last RPC).
pub fn sce_mc_format(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S2][0] = sx(LIBMC_CLIENT);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x50), c.g[RA][0]);
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S3][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_a9dc);
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c4));
            callee::POLL_SEMA.call(c, 0x0040_a9dc)?;
            let sema_ok = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = 0x520000;
            if !sema_ok {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            } else {
                c.cycles += 17;
                c.g[T1][0] = sx(MC_RECV);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7240));
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                c.g[A3][0] = c.g[V0][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                c.sw(frame.at(c, 0), 0u64);
                c.g[A1][0] = 0x10;
                c.g[A2][0] = 0x1;
                c.g[T0][0] = 0x30;
                c.g[T2][0] = 0x4;
                c.g[RA][0] = sx(0x0040_aa24);
                c.g[T3][0] = 0u64;
                callee::CALL_RPC.call(c, 0x0040_aa24)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = 0x450000;
                if c.g[S0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = 0x10;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x33c0), c.g[V0][0]);
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_aa44);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c4));
                    callee::SIGNAL_SEMA.call(c, 0x0040_aa44)?;
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 7;
        frame.restore_ra(c, 0x50);
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcGetDir(int port, int slot, const char *name, unsigned int mode,
/// int maxent, void *table)` (0x40a688): list the entries matching `name`
/// into `table` (RPC function 13; at most 0x3ff bytes of `name` copied; the
/// `maxent * 0x40` bytes of `table` are written back from the data cache
/// first when `maxent >= 0`). Returns -210 for a null or empty name, -100
/// when the RPC is not bound, -200 when the libmc sema is busy, else the RPC
/// result (0 records function 13 as the last RPC).
pub fn sce_mc_get_dir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xa0);
        c.sd(frame.at(c, 0x80), c.g[S7][0]);
        c.sd(frame.at(c, 0x70), c.g[S6][0]);
        c.g[S7][0] = sx(LIBMC_CLIENT);
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.g[S6][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[S4][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S3][0] = c.g[A3][0];
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S2][0] = c.g[T1][0];
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = c.g[T0][0];
        c.sd(frame.at(c, 0x90), c.g[RA][0]);
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = c.g[A2][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S5][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_a6ec);
            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x33c4));
            callee::POLL_SEMA.call(c, 0x0040_a6ec)?;
            let sema_ok = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            if !sema_ok {
                // V0 = -200 already; fall through to the epilogue.
            } else {
                c.cycles += 2;
                let mut proceed = false;
                if c.g[S0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    proceed = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x520000;
                }
                if !proceed {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_a710);
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x33c4));
                    callee::SIGNAL_SEMA.call(c, 0x0040_a710)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_ff2e;
                } else {
                    c.cycles += 12;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7270));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x7270), c.g[S6][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S4][0]);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.g[A2][0] = 0x3ff;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S1][0]);
                    c.g[RA][0] = sx(0x0040_a740);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S2][0]);
                    callee::STRNCPY.call(c, 0x0040_a740)?;
                    let skip_writeback = (c.g[S1][0] as i64) < 0;
                    c.sb((c.g[S0][0] as u32).wrapping_add(0x413), 0u64);
                    if !skip_writeback {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x0040_a754);
                        c.g[A1][0] = sx((c.g[S1][0] as u32) << 6);
                        callee::WRITEBACK.call(c, 0x0040_a754)?;
                    }
                    c.cycles += 14;
                    c.g[T1][0] = sx(MC_RECV);
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S7][0];
                    c.sw(frame.at(c, 0), 0u64);
                    c.g[A1][0] = 0xd;
                    c.g[A2][0] = 0x1;
                    c.g[T0][0] = 0x414;
                    c.g[T2][0] = 0x4;
                    c.g[RA][0] = sx(0x0040_a780);
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x0040_a780)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = 0x450000;
                    if c.g[S0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = 0xd;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x33c0), c.g[V0][0]);
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x0040_a7a0);
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x33c4));
                        callee::SIGNAL_SEMA.call(c, 0x0040_a7a0)?;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                }
            }
        }
        c.cycles += 11;
        frame.restore_ra(c, 0x90);
        c.g[S7][0] = c.ld(frame.at(c, 0x80));
        c.g[S6][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcGetEntSpace(int port, int slot, const char *path)` (0x40b028):
/// ask for the free entry space of directory `path` (RPC function 18; at most
/// 0x3ff bytes copied into the send buffer). Returns -210 for a null or empty
/// path, -100 when the RPC is not bound, -200 when the libmc sema is busy,
/// else the RPC result (0 records function 18 as the last RPC).
pub fn sce_mc_get_ent_space(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[S4][0] = sx(LIBMC_CLIENT);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S3][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S2][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x60), c.g[RA][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = c.g[A2][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S1][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_b074);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x33c4));
            callee::POLL_SEMA.call(c, 0x0040_b074)?;
            let failed = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            if !failed {
                c.cycles += 2;
                let mut name_ok = false;
                if c.g[S0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    name_ok = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x520000;
                }
                if !name_ok {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_b098);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x33c4));
                    callee::SIGNAL_SEMA.call(c, 0x0040_b098)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_ff2e;
                    c.cycles += 8;
                    frame.restore_ra(c, 0x60);
                    c.g[S4][0] = c.ld(frame.at(c, 0x50));
                    c.g[S3][0] = c.ld(frame.at(c, 0x40));
                    c.g[S2][0] = c.ld(frame.at(c, 0x30));
                    c.g[S1][0] = c.ld(frame.at(c, 0x20));
                    c.g[S0][0] = c.ld(frame.at(c, 0x10));
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 22;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7270));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7270), c.g[S3][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S2][0]);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[RA][0] = sx(0x0040_b0bc);
                c.g[A2][0] = 0x3ff;
                callee::STRNCPY.call(c, 0x0040_b0bc)?;
                c.sb((c.g[S0][0] as u32).wrapping_add(0x413), 0u64);
                c.g[T1][0] = sx(MC_RECV);
                c.g[A3][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S4][0];
                c.sw(frame.at(c, 0), 0u64);
                c.g[A1][0] = 0x12;
                c.g[A2][0] = 0x1;
                c.g[T0][0] = 0x414;
                c.g[T2][0] = 0x4;
                c.g[RA][0] = sx(0x0040_b0ec);
                c.g[T3][0] = 0u64;
                callee::CALL_RPC.call(c, 0x0040_b0ec)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = 0x450000;
                if c.g[S0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = 0x12;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x33c0), c.g[V0][0]);
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_b10c);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x33c4));
                    callee::SIGNAL_SEMA.call(c, 0x0040_b10c)?;
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 8;
        frame.restore_ra(c, 0x60);
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcGetInfo(int port, int slot, int *type, int *free, int *format)`
/// (0x40a498): queue a card-info request (RPC function 1, no wait): latches
/// `port`, `slot` and one "wanted" flag per non-null output pointer into the
/// send buffer and saves the three pointers at `0x5271e8..0x5271f0`. Returns
/// -100 when the RPC is not bound, -200 when the libmc sema is busy, else the
/// RPC result (0 records function 1 as the last RPC).
pub fn sce_mc_get_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xa0);
        c.sd(frame.at(c, 0x70), c.g[S6][0]);
        c.g[S6][0] = 0x520000;
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x71c0));
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.g[S4][0] = c.g[A3][0];
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[S3][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x90), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x80), c.g[S7][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
        let bound = c.g[V1][0] != 0;
        c.g[S5][0] = c.g[T0][0];
        if !bound {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S7][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_a4f8);
            c.g[A0][0] = Addr((c.g[S7][0] as u32).wrapping_add(0x33c4)).lw(c);
            callee::POLL_SEMA.call(c, 0x0040_a4f8)?;
            let free = (c.g[V0][0] as i64) >= 0;
            c.g[S1][0] = 0x520000;
            if !free {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            } else {
                c.cycles += 7;
                c.g[A0][0] = 0x520000;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7240));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x76c0));
                Addr((c.g[V1][0] as u32).wrapping_add(4)).sw(c, c.g[S2][0]);
                Addr((c.g[V1][0] as u32).wrapping_add(8)).sw(c, c.g[S0][0]);
                let no_type = c.g[S3][0] == 0;
                Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).sw(c, c.g[V0][0]);
                if no_type {
                    c.cycles += 1;
                    Addr((c.g[V1][0] as u32).wrapping_add(0x14)).sw(c, 0);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    Addr((c.g[V1][0] as u32).wrapping_add(0x14)).sw(c, c.g[V0][0]);
                }
                c.cycles += 2;
                let no_a3 = c.g[S4][0] == 0;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7240));
                if no_a3 {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7240));
                    Addr((c.g[V0][0] as u32).wrapping_add(0x10)).sw(c, 0);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    Addr((c.g[V1][0] as u32).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                }
                c.cycles += 2;
                let no_a4 = c.g[S5][0] == 0;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7240));
                if no_a4 {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7240));
                    Addr((c.g[V0][0] as u32).wrapping_add(0xc)).sw(c, 0);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, c.g[V0][0]);
                }
                c.cycles += 25;
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x76c0));
                c.g[V0][0] = 0x520000;
                c.g[V1][0] = 0x520000;
                c.g[A2][0] = 0x520000;
                c.g[A0][0] = c.g[S0][0];
                Addr((c.g[V0][0] as u32).wrapping_add(0x71e8)).sw(c, c.g[S3][0]);
                Addr((c.g[V1][0] as u32).wrapping_add(0x71ec)).sw(c, c.g[S4][0]);
                c.g[A1][0] = 0xc0;
                c.g[RA][0] = sx(0x0040_a594);
                Addr((c.g[A2][0] as u32).wrapping_add(0x71f0)).sw(c, c.g[S5][0]);
                callee::WRITEBACK.call(c, 0x0040_a594)?;
                c.g[T1][0] = 0x530000;
                c.g[T3][0] = 0x410000;
                Addr(c.g[SP][0] as u32).sw(c, c.g[S0][0]);
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x71c0));
                c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7240));
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7880));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x5bc0));
                c.g[A1][0] = 1;
                c.g[A2][0] = 1;
                c.g[T0][0] = 0x30;
                c.g[RA][0] = sx(0x0040_a5c4);
                c.g[T2][0] = 4;
                callee::CALL_RPC.call(c, 0x0040_a5c4)?;
                c.g[S0][0] = c.g[V0][0];
                let rpc_failed = c.g[S0][0] != 0;
                c.g[V1][0] = 0x450000;
                if rpc_failed {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_a5e4);
                    c.g[A0][0] = Addr((c.g[S7][0] as u32).wrapping_add(0x33c4)).lw(c);
                    callee::SIGNAL_SEMA.call(c, 0x0040_a5e4)?;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    Addr((c.g[V1][0] as u32).wrapping_add(0x33c0)).sw(c, c.g[V0][0]);
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 11;
        frame.restore_ra(c, 0x90);
        c.g[S7][0] = c.ld(frame.at(c, 0x80));
        c.g[S6][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcGetSlotMax(int port)` (0x409c48): ask for the slot count of
/// `port` (RPC function 0x15, waiting for the reply). Returns -100 when the
/// RPC is not bound, -200 when the libmc sema is busy, the `sceSifCallRpc`
/// error when the call fails, else the reply word (`*0x528780`), the slot
/// count.
pub fn sce_mc_get_slot_max(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = 0x520000;
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x71c0));
        c.sd(frame.at(c, 0x50), c.g[RA][0]);
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x24)).lw(c);
        let bound = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !bound {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S1][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_9c88);
            c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x33c4)).lw(c);
            callee::POLL_SEMA.call(c, 0x0040_9c88)?;
            let free = (c.g[V0][0] as i64) >= 0;
            c.g[A3][0] = 0x520000;
            if !free {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            } else {
                c.cycles += 15;
                c.g[S3][0] = 0x530000;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7240));
                c.g[A0][0] = c.g[S2][0];
                Addr((c.g[A3][0] as u32).wrapping_add(4)).sw(c, c.g[S0][0]);
                c.g[A1][0] = 0x15;
                Addr(c.g[SP][0] as u32).sw(c, 0);
                c.g[A2][0] = 0;
                c.g[T0][0] = 0x30;
                c.g[T1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x7880));
                c.g[T2][0] = 4;
                c.g[RA][0] = sx(0x0040_9cc8);
                c.g[T3][0] = 0;
                callee::CALL_RPC.call(c, 0x0040_9cc8)?;
                c.g[S0][0] = c.g[V0][0];
                let rpc_ok = c.g[S0][0] == 0;
                if rpc_ok {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0040_9cec);
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x33c4)).lw(c);
                    callee::SIGNAL_SEMA.call(c, 0x0040_9cec)?;
                    c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_sub(0x7880)).lw(c);
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_9cdc);
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x33c4)).lw(c);
                    callee::SIGNAL_SEMA.call(c, 0x0040_9cdc)?;
                    c.g[V0][0] = c.g[S0][0];
                }
            }
        }
        c.cycles += 7;
        frame.restore_ra(c, 0x50);
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcMkdir(int port, int slot, const char *name)` (0x409e38): make
/// directory `name` by calling `sceMcOpen(port, slot, name, 0x40)`. Returns
/// the open result when that fails, else 0 (recording function 11 as the last
/// RPC).
pub fn sce_mc_mkdir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_9e48);
        c.g[A3][0] = 0x40;
        callee::MC_OPEN.call(c, 0x0040_9e48)?;
        c.g[A0][0] = c.g[V0][0];
        let opened = c.g[A0][0] != 0;
        frame.restore_ra(c, 0);
        if !opened {
            c.cycles += 3;
            c.g[V1][0] = 0x450000;
            c.g[V0][0] = 0xb;
            Addr((c.g[V1][0] as u32).wrapping_add(0x33c0)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0];
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcOpen(int port, int slot, const char *name, int mode)`
/// (0x409d10): open `name` (RPC function 2; at most 0x3ff bytes copied into
/// the send buffer). Returns -210 for a null or empty name, -100 when the RPC
/// is not bound, -200 when the libmc sema is busy, else the RPC result (0
/// records function 2 as the last RPC).
pub fn sce_mc_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x520000;
        c.sd(frame.at(c, 0x70), c.g[S6][0]);
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x71c0));
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.g[S5][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[S4][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S3][0] = c.g[A3][0];
        c.sd(frame.at(c, 0x80), c.g[RA][0]);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x24)).lw(c);
        let bound = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A2][0];
        if !bound {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S2][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_9d68);
            c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x33c4)).lw(c);
            callee::POLL_SEMA.call(c, 0x0040_9d68)?;
            let poll_failed = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            if poll_failed {
                // Fall through to the epilogue with -200 in v0.
            } else {
                c.cycles += 2;
                let null_name = c.g[S1][0] == 0;
                if !null_name {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lb(c);
                    let empty_name = c.g[V0][0] == 0;
                    c.g[S0][0] = 0x520000;
                    if !empty_name {
                        c.cycles += 24;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x7284));
                        c.g[A2][0] = 0x3ff;
                        c.g[RA][0] = sx(0x0040_9da8);
                        c.g[A0][0] = c.g[S0][0];
                        callee::STRNCPY.call(c, 0x0040_9da8)?;
                        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x14));
                        Addr((c.g[S0][0] as u32).wrapping_sub(0x14)).sw(c, c.g[S4][0]);
                        Addr((c.g[V1][0] as u32).wrapping_add(8)).sw(c, c.g[S3][0]);
                        c.g[T1][0] = 0x530000;
                        Addr((c.g[V1][0] as u32).wrapping_add(4)).sw(c, c.g[S5][0]);
                        c.g[A0][0] = c.g[S6][0];
                        Addr((c.g[V1][0] as u32).wrapping_add(0x413)).sb(c, 0);
                        c.g[A3][0] = c.g[V1][0];
                        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7880));
                        c.g[A1][0] = 2;
                        Addr(c.g[SP][0] as u32).sw(c, 0);
                        c.g[A2][0] = 1;
                        c.g[T0][0] = 0x414;
                        c.g[RA][0] = sx(0x0040_9de8);
                        c.g[T2][0] = 4;
                        c.g[T3][0] = 0;
                        callee::CALL_RPC.call(c, 0x0040_9de8)?;
                        c.g[S0][0] = c.g[V0][0];
                        let rpc_failed = c.g[S0][0] != 0;
                        c.g[V1][0] = 0x450000;
                        if rpc_failed {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x0040_9e08);
                            c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x33c4)).lw(c);
                            callee::SIGNAL_SEMA.call(c, 0x0040_9e08)?;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 2;
                            Addr((c.g[V1][0] as u32).wrapping_add(0x33c0)).sw(c, c.g[V0][0]);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S0][0];
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0040_9d8c);
                        c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x33c4)).lw(c);
                        callee::SIGNAL_SEMA.call(c, 0x0040_9d8c)?;
                        c.g[V0][0] = 0xffff_ffff_ffff_ff2e;
                    }
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_9d8c);
                    c.g[A0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x33c4)).lw(c);
                    callee::SIGNAL_SEMA.call(c, 0x0040_9d8c)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_ff2e;
                }
            }
        }
        c.cycles += 10;
        frame.restore_ra(c, 0x80);
        c.g[S6][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcRead(int fd, void *buf, int size)` (0x40a090): read `size`
/// bytes into `buf` (flushed from the data cache first so the IOP side
/// sees it). Returns -100 when the RPC is not bound, -200 when the libmc
/// sema is busy, else the RPC result.
pub fn sce_mc_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x520000;
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x71c0));
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[S4][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S3][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x80), c.g[RA][0]);
        c.sd(frame.at(c, 0x70), c.g[S6][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[V0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0x24)).lw(c);
        let bound = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[A2][0];
        if !bound {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S6][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_a0e4);
            c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x33c4)).lw(c);
            callee::POLL_SEMA.call(c, 0x0040_a0e4)?;
            let free = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = 0x520000;
            if !free {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            } else {
                c.cycles += 28;
                c.g[S0][0] = 0x520000;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x76c0));
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7240));
                Addr((c.g[V0][0] as u32).wrapping_add(0x7240)).sw(c, c.g[S4][0]);
                c.g[A0][0] = c.g[S3][0];
                Addr((c.g[S1][0] as u32).wrapping_add(0x1c)).sw(c, c.g[S0][0]);
                c.g[A1][0] = c.g[S2][0];
                Addr((c.g[S1][0] as u32).wrapping_add(0x18)).sw(c, c.g[S3][0]);
                c.g[RA][0] = sx(0x0040_a11c);
                Addr((c.g[S1][0] as u32).wrapping_add(0xc)).sw(c, c.g[S2][0]);
                callee::WRITEBACK.call(c, 0x0040_a11c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0040_a128);
                c.g[A1][0] = 0xc0;
                callee::WRITEBACK.call(c, 0x0040_a128)?;
                c.g[T1][0] = 0x530000;
                c.g[T3][0] = 0x410000;
                Addr(c.g[SP][0] as u32).sw(c, c.g[S0][0]);
                c.g[A0][0] = c.g[S5][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7880));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6000));
                c.g[A1][0] = 5;
                c.g[A2][0] = 1;
                c.g[T0][0] = 0x30;
                c.g[RA][0] = sx(0x0040_a158);
                c.g[T2][0] = 4;
                callee::CALL_RPC.call(c, 0x0040_a158)?;
                c.g[S0][0] = c.g[V0][0];
                let rpc_failed = c.g[S0][0] != 0;
                c.g[V1][0] = 0x450000;
                if rpc_failed {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_a178);
                    c.g[A0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x33c4)).lw(c);
                    callee::SIGNAL_SEMA.call(c, 0x0040_a178)?;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 5;
                    Addr((c.g[V1][0] as u32).wrapping_add(0x33c0)).sw(c, c.g[V0][0]);
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 10;
        frame.restore_ra(c, 0x80);
        c.g[S6][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcRename(int port, int slot, const char *name, const char
/// *newname)` (0x40ae00): rename `name` to `newname` through the
/// set-file-info RPC (function 14, field mask 0x10): `newname` (at most 0x20
/// bytes) goes into the name field of the info block at `0x527200`. Returns
/// -210 for a null or empty `name` or a null `newname`, -100 when the RPC is
/// not bound, -200 when the libmc sema is busy, else the RPC result (0
/// records function 19 as the last RPC).
pub fn sce_mc_rename(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x90);
        c.sd(frame.at(c, 0x70), c.g[S6][0]);
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.g[S6][0] = sx(LIBMC_CLIENT);
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[S4][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S3][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S2][0] = c.g[A3][0];
        c.sd(frame.at(c, 0x80), c.g[RA][0]);
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = c.g[A2][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S5][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_ae58);
            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x33c4));
            callee::POLL_SEMA.call(c, 0x0040_ae58)?;
            let sema_ok = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            if !sema_ok {
                // V0 = -200 already; fall through to the epilogue.
            } else {
                c.cycles += 2;
                let mut proceed = false;
                if c.g[S0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        proceed = c.g[S2][0] != 0;
                        c.g[V0][0] = 0x520000;
                    }
                }
                if !proceed {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_ae84);
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x33c4));
                    callee::SIGNAL_SEMA.call(c, 0x0040_ae84)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_ff2e;
                } else {
                    c.cycles += 35;
                    c.g[V1][0] = 0x10;
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7270));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x7270), c.g[S4][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[S3][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[RA][0] = sx(0x0040_aeb0);
                    c.g[A2][0] = 0x3ff;
                    callee::STRNCPY.call(c, 0x0040_aeb0)?;
                    c.g[S0][0] = 0x520000;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x7220));
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x413), 0u64);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0040_aecc);
                    c.g[A2][0] = 0x20;
                    callee::STRNCPY.call(c, 0x0040_aecc)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                    c.g[A0][0] = 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                    c.g[RA][0] = sx(0x0040_aee0);
                    c.sb((c.g[S0][0] as u32).wrapping_add(0x3f), 0u64);
                    callee::FLUSH_CACHE.call(c, 0x0040_aee0)?;
                    c.g[T1][0] = sx(MC_RECV);
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A3][0] = c.g[S1][0];
                    c.sw(frame.at(c, 0), 0u64);
                    c.g[A1][0] = 0xe;
                    c.g[A2][0] = 0x1;
                    c.g[T0][0] = 0x414;
                    c.g[T2][0] = 0x4;
                    c.g[RA][0] = sx(0x0040_af0c);
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x0040_af0c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = 0x450000;
                    if c.g[S0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = 0x13;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x33c0), c.g[V0][0]);
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x0040_af2c);
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x33c4));
                        callee::SIGNAL_SEMA.call(c, 0x0040_af2c)?;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                }
            }
        }
        c.cycles += 10;
        frame.restore_ra(c, 0x80);
        c.g[S6][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcSeek(int fd, int offset, int whence)` (0x409f28): move the
/// file position. Returns -100 when the RPC is not bound, -200 when the
/// libmc sema is busy, else the RPC result.
pub fn sce_mc_seek(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x520000;
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x71c0));
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x60), c.g[RA][0]);
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x24)).lw(c);
        let bound = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A1][0];
        if !bound {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S4][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_9f74);
            c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x33c4)).lw(c);
            callee::POLL_SEMA.call(c, 0x0040_9f74)?;
            let free = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = 0x520000;
            if !free {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            } else {
                c.cycles += 17;
                c.g[T1][0] = 0x530000;
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7240));
                Addr((c.g[V0][0] as u32).wrapping_add(0x7240)).sw(c, c.g[S2][0]);
                Addr((c.g[A3][0] as u32).wrapping_add(0x10)).sw(c, c.g[S0][0]);
                c.g[A0][0] = c.g[S3][0];
                Addr((c.g[A3][0] as u32).wrapping_add(0x14)).sw(c, c.g[S1][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7880));
                Addr(c.g[SP][0] as u32).sw(c, 0);
                c.g[A1][0] = 4;
                c.g[A2][0] = 1;
                c.g[T0][0] = 0x30;
                c.g[RA][0] = sx(0x0040_9fbc);
                c.g[T2][0] = 4;
                c.g[T3][0] = 0;
                callee::CALL_RPC.call(c, 0x0040_9fbc)?;
                c.g[S0][0] = c.g[V0][0];
                let rpc_failed = c.g[S0][0] != 0;
                c.g[V1][0] = 0x450000;
                if rpc_failed {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_9fdc);
                    c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x33c4)).lw(c);
                    callee::SIGNAL_SEMA.call(c, 0x0040_9fdc)?;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 4;
                    Addr((c.g[V1][0] as u32).wrapping_add(0x33c0)).sw(c, c.g[V0][0]);
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 8;
        frame.restore_ra(c, 0x60);
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcSetFileInfo(int port, int slot, const char *name, const void
/// *info, unsigned int valid)` (0x40ac38): update `name`'s directory entry
/// from the 0x40-byte `info` (copied to `0x527200`), with `valid & 7` as the
/// field mask (RPC function 14, data cache flushed first). Returns -210 for a
/// null or empty name, -100 when the RPC is not bound, -200 when the libmc
/// sema is busy, else the RPC result (0 records function 14 as the last RPC).
pub fn sce_mc_set_file_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xa0);
        c.sd(frame.at(c, 0x80), c.g[S7][0]);
        c.sd(frame.at(c, 0x70), c.g[S6][0]);
        c.g[S7][0] = sx(LIBMC_CLIENT);
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.g[S6][0] = c.g[A3][0];
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[S4][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S3][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S2][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x90), c.g[RA][0]);
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x24));
        c.g[S1][0] = c.g[T0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S5][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_ac98);
            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x33c4));
            callee::POLL_SEMA.call(c, 0x0040_ac98)?;
            let sema_ok = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            if !sema_ok {
                // V0 = -200 already; fall through to the epilogue.
            } else {
                c.cycles += 2;
                let mut proceed = false;
                if c.g[S2][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                    proceed = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x520000;
                }
                if !proceed {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0040_acbc);
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x33c4));
                    callee::SIGNAL_SEMA.call(c, 0x0040_acbc)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_ff2e;
                } else {
                    c.cycles += 62;
                    c.g[S1][0] = c.g[S1][0] & 0x7;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7270));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x7270), c.g[S4][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S3][0]);
                    c.g[V0][0] = 0x520000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7200));
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7200));
                    c.ldl(6, (c.g[S6][0] as u32).wrapping_add(0x7));
                    c.ldr(6, c.g[S6][0] as u32);
                    c.ldl(7, (c.g[S6][0] as u32).wrapping_add(0xf));
                    c.ldr(7, (c.g[S6][0] as u32).wrapping_add(0x8));
                    c.ldl(8, (c.g[S6][0] as u32).wrapping_add(0x17));
                    c.ldr(8, (c.g[S6][0] as u32).wrapping_add(0x10));
                    c.sdl(6, (c.g[T1][0] as u32).wrapping_add(0x7));
                    c.sdr(6, c.g[T1][0] as u32);
                    c.sdl(7, (c.g[T1][0] as u32).wrapping_add(0xf));
                    c.sdr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
                    c.sdl(8, (c.g[T1][0] as u32).wrapping_add(0x17));
                    c.sdr(8, (c.g[T1][0] as u32).wrapping_add(0x10));
                    c.ldl(6, (c.g[S6][0] as u32).wrapping_add(0x1f));
                    c.ldr(6, (c.g[S6][0] as u32).wrapping_add(0x18));
                    c.ldl(7, (c.g[S6][0] as u32).wrapping_add(0x27));
                    c.ldr(7, (c.g[S6][0] as u32).wrapping_add(0x20));
                    c.ldl(8, (c.g[S6][0] as u32).wrapping_add(0x2f));
                    c.ldr(8, (c.g[S6][0] as u32).wrapping_add(0x28));
                    c.sdl(6, (c.g[T1][0] as u32).wrapping_add(0x1f));
                    c.sdr(6, (c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sdl(7, (c.g[T1][0] as u32).wrapping_add(0x27));
                    c.sdr(7, (c.g[T1][0] as u32).wrapping_add(0x20));
                    c.sdl(8, (c.g[T1][0] as u32).wrapping_add(0x2f));
                    c.sdr(8, (c.g[T1][0] as u32).wrapping_add(0x28));
                    c.ldl(6, (c.g[S6][0] as u32).wrapping_add(0x37));
                    c.ldr(6, (c.g[S6][0] as u32).wrapping_add(0x30));
                    c.ldl(7, (c.g[S6][0] as u32).wrapping_add(0x3f));
                    c.ldr(7, (c.g[S6][0] as u32).wrapping_add(0x38));
                    c.sdl(6, (c.g[T1][0] as u32).wrapping_add(0x37));
                    c.sdr(6, (c.g[T1][0] as u32).wrapping_add(0x30));
                    c.sdl(7, (c.g[T1][0] as u32).wrapping_add(0x3f));
                    c.sdr(7, (c.g[T1][0] as u32).wrapping_add(0x38));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.g[RA][0] = sx(0x0040_ad78);
                    c.g[A2][0] = 0x3ff;
                    callee::STRNCPY.call(c, 0x0040_ad78)?;
                    c.sb((c.g[S0][0] as u32).wrapping_add(0x413), 0u64);
                    c.g[RA][0] = sx(0x0040_ad84);
                    c.g[A0][0] = 0u64;
                    callee::FLUSH_CACHE.call(c, 0x0040_ad84)?;
                    c.g[T1][0] = sx(MC_RECV);
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S7][0];
                    c.sw(frame.at(c, 0), 0u64);
                    c.g[A1][0] = 0xe;
                    c.g[A2][0] = 0x1;
                    c.g[T0][0] = 0x414;
                    c.g[T2][0] = 0x4;
                    c.g[RA][0] = sx(0x0040_adb0);
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x0040_adb0)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = 0x450000;
                    if c.g[S0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = 0xe;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x33c0), c.g[V0][0]);
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x0040_add0);
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x33c4));
                        callee::SIGNAL_SEMA.call(c, 0x0040_add0)?;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                }
            }
        }
        c.cycles += 11;
        frame.restore_ra(c, 0x90);
        c.g[S7][0] = c.ld(frame.at(c, 0x80));
        c.g[S6][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `sceMcSync`'s entry and its wait loop's head (the delay before the next
/// status check), where an unwinding event leaves it and the trampoline
/// re-enters it.
const MC_SYNC_ENTRY: u32 = 0x0040_a360;
const MC_SYNC_DELAY: u32 = 0x0040_a3c8;
const MC_SYNC_END: u32 = 0x0040_a440;

/// `int sceMcSync(int mode, int *cmd, int *result)` (0x40a360): finish the
/// outstanding memory-card command. Returns -1 when none is outstanding.
/// Mode 0 waits (a `libmc_DelayThread(0x3c)` between status checks) until
/// the RPC is done; any other mode checks once and returns 0 while it is
/// still busy. Once done: the command number to `*cmd` and the reply word
/// to `*result` (each when non-null), the command cleared, the libmc sema
/// released, and 1 returned. Resumable at the wait loop's head.
pub fn sce_mc_sync(c: &mut Cpu) {
    guest(c, |c| {
        let resumed = match c.pc {
            MC_SYNC_ENTRY => false,
            MC_SYNC_DELAY => true,
            pc => {
                c.jump_out(pc, MC_SYNC_ENTRY, MC_SYNC_END);
                return Ok(());
            }
        };
        let frame = Frame { size: 0x70 };
        'exit: {
            let mut wait = resumed;
            if !resumed {
                c.cycles += 14;
                let frame = Frame::open(c, 0x70);
                c.sd(frame.at(c, 0x30), c.g[S3][0]);
                c.g[S3][0] = 0x450000;
                c.sd(frame.at(c, 0x50), c.g[S5][0]);
                c.sd(frame.at(c, 0x40), c.g[S4][0]);
                c.g[S5][0] = c.g[A2][0];
                c.sd(frame.at(c, 0x20), c.g[S2][0]);
                c.g[S4][0] = c.g[A1][0];
                c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x33c0)).lw(c);
                c.g[S2][0] = c.g[A0][0];
                frame.save_ra(c, 0x60);
                c.sd(frame.at(c, 0x10), c.g[S1][0]);
                let outstanding = c.g[V0][0] != 0;
                c.sd(frame.at(c, 0), c.g[S0][0]);
                if !outstanding {
                    c.cycles += 2;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    break 'exit;
                }
                c.cycles += 5;
                c.g[S1][0] = 0x520000;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x71c0));
                callee::CHECK_STAT.call(c, 0x0040_a3ac)?;
                let poll = c.g[S2][0] != 0;
                c.g[S0][0] = c.g[V0][0];
                if !poll {
                    c.cycles += 2;
                    if c.g[S0][0] != 0 {
                        // Busy: check again before the first delay.
                        c.cycles += 2;
                        wait = true;
                    }
                }
            }
            // Resumed at the delay, or from the busy first check.
            let mut delay = resumed;
            while wait {
                if delay {
                    c.cycles += 2;
                    c.g[A0][0] = 0x3c;
                    callee::MC_DELAY_THREAD.call(c, 0x0040_a3d0)?;
                }
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x71c0));
                callee::CHECK_STAT.call(c, 0x0040_a3d8)?;
                wait = c.g[V0][0] != 0;
                c.g[S0][0] = 0;
                if wait && back_edge_unwinds(c, MC_SYNC_DELAY) {
                    return Ok(());
                }
                delay = true;
            }
            // Settled when the status check reported done (s0 == 0).
            c.cycles += 2;
            let no_cmd = c.g[S4][0] == 0;
            c.g[S0][0] = (c.g[S0][0] < 1) as u64;
            if !no_cmd {
                c.cycles += 2;
                c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x33c0)).lw(c);
                Addr(c.g[S4][0] as u32).sw(c, c.g[V0][0]);
            }
            c.cycles += 2;
            let busy = c.g[S0][0] == 0;
            c.g[V0][0] = c.g[S0][0];
            if busy {
                break 'exit;
            }
            c.cycles += 2;
            let no_result = c.g[S5][0] == 0;
            Addr((c.g[S3][0] as u32).wrapping_add(0x33c0)).sw(c, 0);
            if !no_result {
                c.cycles += 3;
                c.g[V0][0] = 0x530000;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0x7880)).lw(c);
                Addr(c.g[S5][0] as u32).sw(c, c.g[V1][0]);
            }
            c.cycles += 4;
            c.g[V0][0] = 0x450000;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x33c4)).lw(c);
            callee::SIGNAL_SEMA.call(c, 0x0040_a418)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 9;
        frame.restore_ra(c, 0x60);
        c.g[S5][0] = c.ld(frame.at(c, 0x50));
        c.g[S4][0] = c.ld(frame.at(c, 0x40));
        c.g[S3][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcUdCheckNewCard(void)` (0x40a618): ask the memory-card
/// server whether a new card was inserted (RPC function 0x35, no wait).
/// Returns the reply word once the call is issued, -1 after a debug print
/// when it fails.
pub fn sce_mc_ud_check_new_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x520000;
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[A3][0] = 0x520000;
        c.g[S0][0] = 0x530000;
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x71c0));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7240));
        c.sw(frame.at(c, 0), 0u64);
        c.g[A1][0] = 0x35;
        c.g[A2][0] = 0u64;
        c.g[T0][0] = 0x30;
        c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7880));
        c.g[T2][0] = 0x4;
        c.g[RA][0] = sx(0x0040_a658);
        c.g[T3][0] = 0u64;
        callee::CALL_RPC.call(c, 0x0040_a658)?;
        let issued = c.g[V0][0] == 0;
        c.g[A0][0] = 0x4a0000;
        if issued {
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x7880));
        } else {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0040_a668);
            c.g[A0][0] = sx(0x0049_8158);
            callee::PRINTF.call(c, 0x0040_a668)?;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0x20);
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcUnformat(int port, int slot)` (0x40af58): unformat the card (RPC
/// function 17). Returns -100 when the RPC is not bound, -200 when the libmc
/// sema is busy, else the RPC result (0 records function 17 as the last RPC).
pub fn sce_mc_unformat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S2][0] = sx(LIBMC_CLIENT);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x50), c.g[RA][0]);
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
        c.g[S0][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S3][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_af9c);
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c4));
            callee::POLL_SEMA.call(c, 0x0040_af9c)?;
            let sema_ok = (c.g[V0][0] as i64) >= 0;
            c.g[V0][0] = 0x520000;
            if !sema_ok {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            } else {
                c.cycles += 17;
                c.g[T1][0] = sx(MC_RECV);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7240));
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S0][0]);
                c.g[A3][0] = c.g[V0][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                c.sw(frame.at(c, 0), 0u64);
                c.g[A1][0] = 0x11;
                c.g[A2][0] = 0x1;
                c.g[T0][0] = 0x30;
                c.g[T2][0] = 0x4;
                c.g[RA][0] = sx(0x0040_afe4);
                c.g[T3][0] = 0u64;
                callee::CALL_RPC.call(c, 0x0040_afe4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = 0x450000;
                if c.g[S0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = 0x11;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x33c0), c.g[V0][0]);
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_b004);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c4));
                    callee::SIGNAL_SEMA.call(c, 0x0040_b004)?;
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 7;
        frame.restore_ra(c, 0x50);
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int sceMcWrite(int fd, void *buf, int size)` (0x40a1a8): write `size`
/// bytes. Returns -100 when the RPC is not bound, -200 when the libmc
/// sema is busy, else the RPC result. The leading unaligned bytes (fewer
/// than 16) are copied into the send buffer by the CPU; the rest is left
/// for the IOP side (`FlushCache` first). Small writes (< 17 bytes) store
/// the whole size as the head count.
pub fn sce_mc_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        c.sd(frame.at(c, 0x50), c.g[S4][0]);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[S4][0] = 0x520000;
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x71c0));
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x70), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.sd(frame.at(c, 0x60), c.g[S5][0]);
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
        let bound = c.g[V1][0] != 0;
        c.g[S0][0] = c.g[A2][0];
        if !bound {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ff9c;
        } else {
            c.cycles += 5;
            c.g[S5][0] = 0x450000;
            c.g[RA][0] = sx(0x0040_a1f8);
            c.g[A0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0x33c4)).lw(c);
            callee::POLL_SEMA.call(c, 0x0040_a1f8)?;
            let free = (c.g[V0][0] as i64) >= 0;
            c.g[S3][0] = 0x520000;
            if !free {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ff38;
            } else {
                c.cycles += 4;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 17) as u64;
                c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x7240));
                let aligned = c.g[V0][0] == 0;
                Addr((c.g[S3][0] as u32).wrapping_add(0x7240)).sw(c, c.g[S2][0]);
                if !aligned {
                    c.cycles += 4;
                    Addr((c.g[A2][0] as u32).wrapping_add(0x14)).sw(c, c.g[S0][0]);
                    Addr((c.g[A2][0] as u32).wrapping_add(0x18)).sw(c, 0);
                    Addr((c.g[A2][0] as u32).wrapping_add(0xc)).sw(c, 0);
                } else {
                    c.cycles += 11;
                    c.g[V1][0] = 0xffff_ffff_ffff_0000;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                    c.g[V1][0] |= 0xfff0;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] &= c.g[V1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    Addr((c.g[A2][0] as u32).wrapping_add(0x18)).sw(c, c.g[V1][0]);
                    Addr((c.g[A2][0] as u32).wrapping_add(0xc)).sw(c, c.g[A1][0]);
                    Addr((c.g[A2][0] as u32).wrapping_add(0x14)).sw(c, c.g[V0][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x7240));
                c.g[A3][0] = c.g[S3][0];
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
                let no_head = c.g[V1][0] == 0;
                c.g[A2][0] = 0;
                if !no_head {
                    c.cycles += 4;
                    c.g[S0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x7240));
                    loop {
                        c.cycles += 8;
                        c.g[A1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                        Addr((c.g[V1][0] as u32).wrapping_add(0x20)).sb(c, c.g[A1][0]);
                        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x14)).lw(c);
                        c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0040_a278);
                            }
                            continue;
                        } else {
                            c.cycles += 2;
                            break;
                        }
                    }
                } else {
                    c.cycles += 1;
                    c.g[S0][0] = 0x530000;
                }
                c.cycles += 15;
                c.g[RA][0] = sx(0x0040_a2ac);
                c.g[A0][0] = 0;
                callee::FLUSH_CACHE.call(c, 0x0040_a2ac)?;
                c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7880));
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x71c0));
                c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x7240));
                Addr(c.g[SP][0] as u32).sw(c, 0);
                c.g[A1][0] = 6;
                c.g[A2][0] = 1;
                c.g[T0][0] = 0x30;
                c.g[RA][0] = sx(0x0040_a2d4);
                c.g[T2][0] = 4;
                c.g[T3][0] = 0;
                callee::CALL_RPC.call(c, 0x0040_a2d4)?;
                c.g[S0][0] = c.g[V0][0];
                let rpc_failed = c.g[S0][0] != 0;
                c.g[V1][0] = 0x450000;
                if rpc_failed {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0040_a2f4);
                    c.g[A0][0] = Addr((c.g[S5][0] as u32).wrapping_add(0x33c4)).lw(c);
                    callee::SIGNAL_SEMA.call(c, 0x0040_a2f4)?;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 6;
                    Addr((c.g[V1][0] as u32).wrapping_add(0x33c0)).sw(c, c.g[V0][0]);
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 9;
        frame.restore_ra(c, 0x70);
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

pub(crate) const ALARM_CALL_SIGNAL_SEMA: Callee = Callee {
    addr: 0x0042_3dd0,
    f: ee::s15::iSignalSema,
};

/// `void libmc_DelayAlarmHandler(alarmId, ticks, sema)` (0x40a320): signals the
/// semaphore (the third argument) through `iSignalSema`.
pub fn libmc_delay_alarm(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = c.g[A2][0];
    c.pc = ALARM_CALL_SIGNAL_SEMA.addr;
    (ALARM_CALL_SIGNAL_SEMA.f)(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::require_paths;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::game::lib::sif::tests::{SIF_RPC_POOL, SIF_RPC_POOL_COUNT};
    use crate::game::lib::sif::MRPC_POOL;
    use ssx3_ee_rt::Cpu;

    /// Last RPC function id (written on success, e.g. 2/3/4/5/6, 11, 20).
    pub(crate) const LIBMC_LAST_FN: u32 = 0x0045_33c0;

    /// Main libmc semaphore id.
    pub(crate) const LIBMC_MAIN_SEMA: u32 = 0x0045_33c4;

    /// `LIBMC_CLIENT + 0x24`: nonzero once the RPC is bound.
    pub(crate) const LIBMC_RPC_READY: u32 = 0x0052_71e4;

    /// Chdir reply path in RAM (`libmc_ChdirCallback` copies it from the
    /// uncached alias `0x2052_7780`).
    pub(crate) const CHDIR_REPLY: u32 = 0x0052_7780;

    /// ChdirCallback: null destination (early return), empty and short
    /// reply paths, and 0x400- and 0x410-byte ones (both cap the copy at
    /// 0x3ff). Fully covered.
    #[test]
    fn libmc_chdir_callback_matches() {
        let mc_chdir_callback_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_rpc_setup(c, seed, 0x0040_a7d0);
            c.pc = 0x0040_a7d0;
            if seed % 4 == 0 {
                c.g[A0][0] = 0;
            } else {
                c.g[A0][0] = CHDIR_DST as u64;
                let path: Vec<u8> = match seed % 4 {
                    1 => b"".to_vec(),
                    2 => b"BA-DATA".to_vec(),
                    _ => vec![b'P'; [0x400, 0x410][seed as usize / 4 % 2]],
                };
                for (i, b) in path.iter().enumerate() {
                    c.bus.write8(CHDIR_REPLY + i as u32, *b);
                }
                c.bus.write8(CHDIR_REPLY + path.len() as u32, 0);
            }
        };
        check(
            ee::s15::f_0040a7d0__generated,
            libmc_chdir_callback,
            200,
            mc_chdir_callback_call,
        );
        require_paths(
            ee::s15::f_0040a7d0__generated,
            200,
            &[13, 123],
            mc_chdir_callback_call,
        );
    }

    /// DelayThread: a bogus delay sema (the wait fails) and a real one
    /// (count 1, the wait succeeds at once). Fully covered.
    #[test]
    fn libmc_delay_thread_matches() {
        let mc_delay_thread_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_setup(c, seed, 0x0040_a328);
            if seed % 2 != 0 {
                let id = mc_real_sema(c);
                c.bus.write32(LIBMC_DELAY_SEMA, id);
            }
            c.g[A0][0] = [0u64, 1, 0x3c, 0x100, 0xffff][seed as usize % 5];
        };
        check(
            ee::s15::f_0040a328__generated,
            libmc_delay_thread,
            200,
            mc_delay_thread_call,
        );
        require_paths(
            ee::s15::f_0040a328__generated,
            200,
            &[22, 28],
            mc_delay_thread_call,
        );
    }

    /// GetRpcBuffers: leaf, fully covered (both outs written, the delay
    /// sema id latched at send + 0x3c, for several sema words).
    #[test]
    fn libmc_get_rpc_buffers_matches() {
        let mc_get_rpc_buffers_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_setup(c, seed, 0x0040_9b60);
            c.g[A0][0] = (DATA + 0x3020) as u64;
            c.g[A1][0] = (DATA + 0x3024) as u64;
            c.bus.write32(LIBMC_MAIN_SEMA, 0x100 + seed);
        };
        check(
            ee::s15::f_00409b60__generated,
            libmc_get_rpc_buffers,
            200,
            mc_get_rpc_buffers_call,
        );
        require_paths(
            ee::s15::f_00409b60__generated,
            200,
            &[12],
            mc_get_rpc_buffers_call,
        );
    }

    /// ChangeThreadPriority: unbound (-100), sema busy (-200) and the RPC leg.
    /// Again with the RPC answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_change_thread_priority_matches() {
        let mc_change_thread_priority_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_setup(c, seed, 0x0040_9b90);
            mc_legs(c, seed);
            if seed / 9 % 2 == 1 {
                mc_free_packets(c);
            }
            c.g[A0][0] = [0u64, 1, 0x10, 0x30][seed as usize % 4];
        };
        check(
            ee::s15::f_00409b90__generated,
            sce_mc_change_thread_priority,
            120,
            mc_change_thread_priority_call,
        );
        require_paths(
            ee::s15::f_00409b90__generated,
            120,
            &[18, 27, 116],
            mc_change_thread_priority_call,
        );
        check_answered(
            ee::s15::f_00409b90__generated,
            sce_mc_change_thread_priority,
            36,
            &[335],
            mc_change_thread_priority_call,
        );
    }

    /// Chdir: layer down (-100), sema busy (-200), null and empty paths (-210),
    /// and a valid path (RPC fails fast, -1). Again with the RPC answered
    /// (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_chdir_matches() {
        let mc_chdir_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_rpc_setup(c, seed, 0x0040_a858);
            c.pc = 0x0040_a858;
            c.g[A0][0] = 0;
            c.g[A1][0] = 0;
            c.g[A3][0] = 0x99;
            match seed % 4 {
                0 => {}
                1 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    c.bus.write32(LIBMC_MAIN_SEMA, 0x1234);
                }
                2 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    if seed % 2 == 0 {
                        c.g[A2][0] = 0;
                    } else {
                        c.g[A2][0] = MC_NAME as u64;
                        put_name(c, MC_NAME, b"");
                    }
                }
                _ => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    c.g[A2][0] = MC_NAME as u64;
                    put_name(c, MC_NAME, b"BEDATA-SYSTEM");
                }
            }
        };
        check(
            ee::s15::f_0040a858__generated,
            sce_mc_chdir,
            200,
            mc_chdir_call,
        );
        require_paths(
            ee::s15::f_0040a858__generated,
            200,
            &[31, 38, 48, 7405],
            mc_chdir_call,
        );
        check_answered(
            ee::s15::f_0040a858__generated,
            sce_mc_chdir,
            36,
            &[48, 51, 7669, 7755],
            mc_chdir_call,
        );
    }

    /// Close: unbound (-100), sema busy (-200) and the RPC leg. Again with the
    /// RPC answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_close_matches() {
        let mc_close_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_setup(c, seed, 0x0040_9e70);
            mc_legs(c, seed);
            if seed / 9 % 2 == 1 {
                mc_free_packets(c);
            }
            c.g[A0][0] = seed as u64;
        };
        check(
            ee::s15::f_00409e70__generated,
            sce_mc_close,
            120,
            mc_close_call,
        );
        require_paths(
            ee::s15::f_00409e70__generated,
            120,
            &[18, 27, 116],
            mc_close_call,
        );
        check_answered(
            ee::s15::f_00409e70__generated,
            sce_mc_close,
            36,
            &[335],
            mc_close_call,
        );
    }

    /// Delete: layer down (-100), sema busy (-200), null and empty names
    /// (-210), and a valid name (RPC fails fast, -1). Again with the RPC
    /// answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_delete_matches() {
        let mc_delete_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_rpc_setup(c, seed, 0x0040_aa68);
            c.pc = 0x0040_aa68;
            c.g[A0][0] = 0;
            c.g[A1][0] = 1;
            match seed % 4 {
                0 => {}
                1 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    c.bus.write32(LIBMC_MAIN_SEMA, 0x1234);
                }
                2 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    if seed % 2 == 0 {
                        c.g[A2][0] = 0;
                    } else {
                        c.g[A2][0] = MC_NAME as u64;
                        put_name(c, MC_NAME, b"");
                    }
                }
                _ => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    c.g[A2][0] = MC_NAME as u64;
                    put_name(c, MC_NAME, b"BEDATA-SYSTEM");
                }
            }
        };
        check(
            ee::s15::f_0040aa68__generated,
            sce_mc_delete,
            200,
            mc_delete_call,
        );
        require_paths(
            ee::s15::f_0040aa68__generated,
            200,
            &[26, 33, 43, 7339],
            mc_delete_call,
        );
        check_answered(
            ee::s15::f_0040aa68__generated,
            sce_mc_delete,
            36,
            &[43, 46, 7604],
            mc_delete_call,
        );
    }

    /// End: a negative sema word (no deletes) and bogus/real sema ids
    /// (both deletes run; always returns 1). Fully covered.
    #[test]
    fn sce_mc_end_matches() {
        let mc_end_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_setup(c, seed, 0x0040_9b10);
            match seed % 3 {
                0 => c.bus.write32(LIBMC_MAIN_SEMA, 0xffff_ffff),
                1 => {
                    c.bus.write32(LIBMC_MAIN_SEMA, 0x1234);
                    c.bus.write32(LIBMC_DELAY_SEMA, 0x1235);
                }
                _ => {
                    let id = mc_real_sema(c);
                    c.bus.write32(LIBMC_MAIN_SEMA, id);
                    let id2 = mc_real_sema(c);
                    c.bus.write32(LIBMC_DELAY_SEMA, id2);
                }
            }
        };
        check(ee::s15::f_00409b10__generated, sce_mc_end, 120, mc_end_call);
        require_paths(
            ee::s15::f_00409b10__generated,
            120,
            &[11, 19, 27],
            mc_end_call,
        );
    }

    /// Flush: layer down (-100), sema busy (-200), and the RPC (fails fast,
    /// -1). Again with the RPC answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_flush_matches() {
        let mc_flush_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_rpc_setup(c, seed, 0x0040_ab80);
            c.pc = 0x0040_ab80;
            c.g[A0][0] = (seed % 2) as u64;
            match seed % 3 {
                0 => {}
                1 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    c.bus.write32(LIBMC_MAIN_SEMA, 0x1234);
                }
                _ => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                }
            }
        };
        check(
            ee::s15::f_0040ab80__generated,
            sce_mc_flush,
            200,
            mc_flush_call,
        );
        require_paths(
            ee::s15::f_0040ab80__generated,
            200,
            &[18, 27, 116],
            mc_flush_call,
        );
        check_answered(
            ee::s15::f_0040ab80__generated,
            sce_mc_flush,
            36,
            &[335],
            mc_flush_call,
        );
    }

    /// Format: layer down (-100), sema busy (-200), and the RPC (fails fast,
    /// -1). Again with the RPC answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_format_matches() {
        let mc_format_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_rpc_setup(c, seed, 0x0040_a998);
            c.pc = 0x0040_a998;
            c.g[A0][0] = (seed % 2) as u64;
            c.g[A1][0] = (seed % 3) as u64;
            match seed % 3 {
                0 => {}
                1 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    c.bus.write32(LIBMC_MAIN_SEMA, 0x1234);
                }
                _ => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                }
            }
        };
        check(
            ee::s15::f_0040a998__generated,
            sce_mc_format,
            200,
            mc_format_call,
        );
        require_paths(
            ee::s15::f_0040a998__generated,
            200,
            &[21, 30, 121],
            mc_format_call,
        );
        check_answered(
            ee::s15::f_0040a998__generated,
            sce_mc_format,
            36,
            &[340],
            mc_format_call,
        );
    }

    /// GetDir: layer down (-100), sema busy (-200), null and empty paths
    /// (-210), and valid paths with negative, zero and positive entry sizes
    /// (RPC fails fast, -1). Again with the RPC answered (`mc_answered`): the
    /// RPC-success leg.
    #[test]
    fn sce_mc_get_dir_matches() {
        let mc_get_dir_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_rpc_setup(c, seed, 0x0040_a688);
            c.pc = 0x0040_a688;
            c.g[A0][0] = 0;
            c.g[A1][0] = 0;
            c.g[A3][0] = 1;
            c.g[T0][0] = [-1i64 as u64, 0, 2][seed as usize % 3];
            c.g[T1][0] = MC_ENTRIES as u64;
            match seed % 4 {
                0 => {}
                1 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    c.bus.write32(LIBMC_MAIN_SEMA, 0x1234);
                }
                2 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    if seed % 2 == 0 {
                        c.g[A2][0] = 0;
                    } else {
                        c.g[A2][0] = MC_NAME as u64;
                        put_name(c, MC_NAME, b"");
                    }
                }
                _ => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    c.g[A2][0] = MC_NAME as u64;
                    put_name(c, MC_NAME, b"BEDATA-SYSTEM");
                }
            }
        };
        check(
            ee::s15::f_0040a688__generated,
            sce_mc_get_dir,
            200,
            mc_get_dir_call,
        );
        require_paths(
            ee::s15::f_0040a688__generated,
            200,
            &[33, 40, 50, 7348, 7357, 7382],
            mc_get_dir_call,
        );
        check_answered(
            ee::s15::f_0040a688__generated,
            sce_mc_get_dir,
            36,
            &[50, 53, 7613, 7622, 7647],
            mc_get_dir_call,
        );
    }

    /// GetEntSpace: layer down (-100), sema busy (-200), null and empty names
    /// (-210), and a valid name (RPC fails fast, -1). On half the live-sema
    /// seeds PollSema reports 0, which still counts as taken. Again with the
    /// RPC answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_get_ent_space_matches() {
        let mc_get_ent_space_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_rpc_setup(c, seed, 0x0040_b028);
            c.pc = 0x0040_b028;
            c.g[A0][0] = 0;
            c.g[A1][0] = 1;
            match seed % 4 {
                0 => {}
                1 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    c.bus.write32(LIBMC_MAIN_SEMA, 0x1234);
                }
                2 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    c.g[A2][0] = if seed / 4 % 2 == 0 { 0 } else { MC_NAME as u64 };
                    put_name(c, MC_NAME, b"");
                }
                _ => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    c.g[A2][0] = MC_NAME as u64;
                    put_name(c, MC_NAME, b"BISLUS-20772");
                }
            }
            if seed % 4 >= 2 && seed / 8 % 2 == 1 {
                poll_sema_zero(c);
            }
        };
        check(
            ee::s15::f_0040b028__generated,
            sce_mc_get_ent_space,
            200,
            mc_get_ent_space_call,
        );
        require_paths(
            ee::s15::f_0040b028__generated,
            200,
            &[24, 31, 41, 7333],
            mc_get_ent_space_call,
        );
        check_answered(
            ee::s15::f_0040b028__generated,
            sce_mc_get_ent_space,
            36,
            &[41, 44, 7578, 7598],
            mc_get_ent_space_call,
        );
    }

    /// GetInfo: unbound (-100), sema busy (-200) and the RPC leg (the real RPC
    /// fails fast without an IOP, so `SignalSema` runs and the RPC error
    /// returns). Again with the RPC answered (`mc_answered`): the RPC-success
    /// leg.
    #[test]
    fn sce_mc_get_info_matches() {
        let mc_get_info_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_setup(c, seed, 0x0040_a498);
            mc_legs(c, seed);
            c.g[A0][0] = seed as u64;
            c.g[A1][0] = 0x100 + seed as u64;
            c.g[A2][0] = (seed % 2) as u64;
            c.g[A3][0] = ((seed / 2) % 2) as u64;
            c.g[T0][0] = ((seed / 4) % 2) as u64;
        };
        check(
            ee::s15::f_0040a498__generated,
            sce_mc_get_info,
            120,
            mc_get_info_call,
        );
        require_paths(
            ee::s15::f_0040a498__generated,
            120,
            &[32, 41, 194, 195, 196, 197, 198],
            mc_get_info_call,
        );
        check_answered(
            ee::s15::f_0040a498__generated,
            sce_mc_get_info,
            36,
            &[412, 413, 414, 415, 416],
            mc_get_info_call,
        );
    }

    /// GetSlotMax: unbound (-100), sema busy (-200) and the RPC leg. Again with
    /// the RPC answered (`mc_answered`): the RPC-success leg, returning the
    /// random reply word.
    #[test]
    fn sce_mc_get_slot_max_matches() {
        let mc_get_slot_max_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_setup(c, seed, 0x0040_9c48);
            mc_legs(c, seed);
            c.g[A0][0] = (seed % 2) as u64;
        };
        check(
            ee::s15::f_00409c48__generated,
            sce_mc_get_slot_max,
            120,
            mc_get_slot_max_call,
        );
        require_paths(
            ee::s15::f_00409c48__generated,
            120,
            &[20, 29, 119],
            mc_get_slot_max_call,
        );
        check_answered(
            ee::s15::f_00409c48__generated,
            sce_mc_get_slot_max,
            36,
            &[358],
            mc_get_slot_max_call,
        );
    }

    /// Mkdir: the open always fails without an IOP, so the error leg (the open
    /// result) is covered for null, empty and real names. Again with the RPC
    /// answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_mkdir_matches() {
        let mc_mkdir_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_setup(c, seed, 0x0040_9e38);
            mc_legs(c, seed);
            let name = DATA + 0x4200;
            match (seed / 3) % 3 {
                0 => c.g[A2][0] = 0,
                1 => {
                    c.bus.write32(name, 0);
                    c.g[A2][0] = name as u64;
                }
                _ => {
                    cstr(c, name, b"mc0:/TEST-DATA");
                    c.g[A2][0] = name as u64;
                }
            }
            c.g[A0][0] = seed as u64;
            c.g[A1][0] = seed as u64;
        };
        check(
            ee::s15::f_00409e38__generated,
            sce_mc_mkdir,
            120,
            mc_mkdir_call,
        );
        require_paths(
            ee::s15::f_00409e38__generated,
            120,
            &[39, 46, 56, 59, 7354],
            mc_mkdir_call,
        );
        check_answered(
            ee::s15::f_00409e38__generated,
            sce_mc_mkdir,
            36,
            &[56, 59, 7622],
            mc_mkdir_call,
        );
    }

    /// Open: null/empty names (-210), unbound (-100), sema busy (-200) and the
    /// RPC leg (`strncpy` runs, the real RPC fails fast). On half the real-sema
    /// seeds PollSema reports 0, which still counts as taken. Again with the
    /// RPC answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_open_matches() {
        let mc_open_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_setup(c, seed, 0x0040_9d10);
            mc_legs(c, seed);
            if seed / 9 % 2 == 1 {
                mc_free_packets(c);
            }
            let name = DATA + 0x4200;
            match (seed / 3) % 3 {
                0 => c.g[A2][0] = 0,
                1 => {
                    c.bus.write32(name, 0);
                    c.g[A2][0] = name as u64;
                }
                _ => {
                    cstr(c, name, b"mc0:/TEST-DATA");
                    c.g[A2][0] = name as u64;
                }
            }
            if seed % 3 == 2 && seed / 18 % 2 == 1 {
                poll_sema_zero(c);
            }
            c.g[A0][0] = seed as u64;
            c.g[A1][0] = seed as u64;
            c.g[A3][0] = seed as u64;
        };
        check(
            ee::s15::f_00409d10__generated,
            sce_mc_open,
            120,
            mc_open_call,
        );
        require_paths(
            ee::s15::f_00409d10__generated,
            120,
            &[29, 36, 46, 49, 7344],
            mc_open_call,
        );
        check_answered(
            ee::s15::f_00409d10__generated,
            sce_mc_open,
            36,
            &[46, 49, 7609],
            mc_open_call,
        );
    }

    /// Read: unbound (-100), sema busy (-200) and the RPC leg. Again with the
    /// RPC answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_read_matches() {
        let mc_read_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_setup(c, seed, 0x0040_a090);
            mc_legs(c, seed);
            c.g[A0][0] = seed as u64;
            c.g[A1][0] = (DATA + 0x4100) as u64;
            c.g[A2][0] = 0x40 + (seed % 0x40) as u64;
        };
        check(
            ee::s15::f_0040a090__generated,
            sce_mc_read,
            120,
            mc_read_call,
        );
        require_paths(
            ee::s15::f_0040a090__generated,
            120,
            &[28, 37, 208],
            mc_read_call,
        );
        check_answered(
            ee::s15::f_0040a090__generated,
            sce_mc_read,
            36,
            &[419, 426],
            mc_read_call,
        );
    }

    /// Rename: layer down (-100), sema busy (-200), null/empty old and null new
    /// names (-210), and valid names (RPC fails fast, -1). Again with the RPC
    /// answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_rename_matches() {
        let mc_rename_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_rpc_setup(c, seed, 0x0040_ae00);
            c.pc = 0x0040_ae00;
            c.g[A0][0] = 0;
            c.g[A1][0] = 0;
            match seed % 5 {
                0 => {}
                1 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    c.bus.write32(LIBMC_MAIN_SEMA, 0x1234);
                }
                2 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    // Null old name.
                    c.g[A2][0] = 0;
                    c.g[A3][0] = MC_NAME as u64;
                }
                3 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    // Empty old name / null new name.
                    c.g[A2][0] = MC_NAME as u64;
                    put_name(c, MC_NAME, b"");
                    c.g[A3][0] = 0;
                }
                _ => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    c.g[A2][0] = MC_NAME as u64;
                    put_name(c, MC_NAME, b"BEDATA-SYSTEM");
                    c.g[A3][0] = (MC_NAME + 0x40) as u64;
                    put_name(c, MC_NAME + 0x40, b"NEWNAME");
                }
            }
        };
        check(
            ee::s15::f_0040ae00__generated,
            sce_mc_rename,
            200,
            mc_rename_call,
        );
        require_paths(
            ee::s15::f_0040ae00__generated,
            200,
            &[29, 36, 46, 49, 7642],
            mc_rename_call,
        );
        check_answered(
            ee::s15::f_0040ae00__generated,
            sce_mc_rename,
            36,
            &[46, 49, 7907],
            mc_rename_call,
        );
    }

    /// Seek: unbound (-100), sema busy (-200) and the RPC leg. Again with the
    /// RPC answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_seek_matches() {
        let mc_seek_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_setup(c, seed, 0x0040_9f28);
            mc_legs(c, seed);
            if seed / 9 % 2 == 1 {
                mc_free_packets(c);
            }
            c.g[A0][0] = seed as u64;
            c.g[A1][0] = (seed as u64).wrapping_mul(0x1_0000_0001);
            c.g[A2][0] = (seed % 3) as u64;
        };
        check(
            ee::s15::f_00409f28__generated,
            sce_mc_seek,
            120,
            mc_seek_call,
        );
        require_paths(
            ee::s15::f_00409f28__generated,
            120,
            &[24, 33, 124],
            mc_seek_call,
        );
        check_answered(
            ee::s15::f_00409f28__generated,
            sce_mc_seek,
            36,
            &[343],
            mc_seek_call,
        );
    }

    /// SetFileInfo: layer down (-100), sema busy (-200), null and empty names
    /// (-210), and a valid name with a 64-byte info struct (RPC fails fast,
    /// -1). Again with the RPC answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_set_file_info_matches() {
        let mc_set_file_info_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = mc_rpc_setup(c, seed, 0x0040_ac38);
            c.pc = 0x0040_ac38;
            c.g[A0][0] = 0;
            c.g[A1][0] = 0;
            c.g[T0][0] = seed as u64;
            for i in 0..16 {
                c.bus.write32(MC_INFO + i * 4, r.next_u32());
            }
            match seed % 4 {
                0 => {}
                1 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    c.bus.write32(LIBMC_MAIN_SEMA, 0x1234);
                }
                2 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    if seed % 2 == 0 {
                        c.g[A2][0] = 0;
                    } else {
                        c.g[A2][0] = MC_NAME as u64;
                        put_name(c, MC_NAME, b"");
                    }
                    c.g[A3][0] = MC_INFO as u64;
                }
                _ => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                    c.g[A2][0] = MC_NAME as u64;
                    put_name(c, MC_NAME, b"BEDATA-SYSTEM");
                    c.g[A3][0] = MC_INFO as u64;
                }
            }
        };
        check(
            ee::s15::f_0040ac38__generated,
            sce_mc_set_file_info,
            200,
            mc_set_file_info_call,
        );
        require_paths(
            ee::s15::f_0040ac38__generated,
            200,
            &[32, 39, 49, 7387],
            mc_set_file_info_call,
        );
        check_answered(
            ee::s15::f_0040ac38__generated,
            sce_mc_set_file_info,
            36,
            &[49, 52, 7652],
            mc_set_file_info_call,
        );
    }

    /// Gate word for [`fake_delay_thread`] (in the machine's RAM).
    const DELAY_GATE: u32 = DATA + 0x6100;
    const DELAY_MAGIC: u32 = 0x444c_4159;
    /// Delays left before the fake completes the busy RPC packet.
    const DELAYS_LEFT: u32 = DATA + 0x6104;
    /// The RPC packet `sceSifCheckStatRpc` finds busy (see [`mc_rpc_busy`]).
    const BUSY_PACKET: u32 = DATA + 0x600;

    /// `libmc_DelayThread` with its gate set: counts down and, on the last
    /// delay, clears the busy packet's flag so the RPC reads as finished
    /// (the real one sleeps on an alarm the harness never fires in time).
    fn fake_delay_thread(c: &mut Cpu) {
        if c.bus.read32(DELAY_GATE) != DELAY_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_delay_thread),
                ee::s15::f_0040a328__generated,
            );
        }
        let left = c.bus.read32(DELAYS_LEFT);
        if left <= 1 {
            c.bus.write32(BUSY_PACKET + 0x10, 0);
        }
        c.bus.write32(DELAYS_LEFT, left.wrapping_sub(1));
        c.g[V0][0] = 0;
        c.cycles += 6;
        crate::abi::ret(c);
    }

    /// Gate word for [`fake_poll_sema`] (in the machine's RAM).
    const POLL_GATE: u32 = DATA + 0x6120;
    const POLL_MAGIC: u32 = 0x504f_4c4c;

    /// `PollSema` with its gate set: reports 0, a taken sema with the id 0
    /// the kernel model never hands out (it numbers semas from 1).
    fn fake_poll_sema(c: &mut Cpu) {
        if c.bus.read32(POLL_GATE) != POLL_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_poll_sema),
                ee::s15::f_00423df0__generated,
            );
        }
        c.g[V0][0] = 0;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// Make `PollSema` report 0 (see [`fake_poll_sema`]).
    fn poll_sema_zero(c: &mut Cpu) {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00423DF0),
            fake_poll_sema,
            crate::difftest::fakes::prev_cell(fake_poll_sema),
        );
        c.bus.write32(POLL_GATE, POLL_MAGIC);
    }

    /// Make `sceSifCheckStatRpc(&LIBMC_CLIENT)` report busy (the client's
    /// packet id matches and the packet's busy bit is set) until
    /// [`fake_delay_thread`] has run `delays` times.
    fn mc_rpc_busy(c: &mut Cpu, delays: u32) {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_0040A328),
            fake_delay_thread,
            crate::difftest::fakes::prev_cell(fake_delay_thread),
        );
        c.bus.write32(LIBMC_CLIENT, BUSY_PACKET);
        c.bus.write32(BUSY_PACKET + 0x18, 0x1111);
        c.bus.write32(LIBMC_CLIENT + 4, 0x1111);
        c.bus.write32(BUSY_PACKET + 0x10, 1);
        c.bus.write32(DELAY_GATE, DELAY_MAGIC);
        c.bus.write32(DELAYS_LEFT, delays);
    }

    /// `sceMcSync` call state: waiting (mode 0) or polling; nothing
    /// outstanding or a command in flight; the RPC finished, or busy for
    /// `seed / 8 % 4` delays; with or without the `cmd`/`result` outputs.
    fn mc_sync_call(c: &mut Cpu, seed: u32) -> Rng {
        let r = mc_setup(c, seed, 0x0040_a360);
        let outstanding = seed / 2 % 4 != 0;
        c.bus
            .write32(LIBMC_LAST_FN, if outstanding { 0x8000_0001 } else { 0 });
        c.bus.write32(LIBMC_CLIENT, 0);
        let delays = seed / 8 % 4;
        if delays > 0 {
            mc_rpc_busy(c, delays);
        }
        // The sema first: creating it uses a0.
        let id = mc_real_sema(c);
        c.bus.write32(LIBMC_MAIN_SEMA, id);
        c.g[A0][0] = (seed % 2) as u64;
        c.g[A1][0] = if seed / 32 % 2 == 0 {
            0
        } else {
            (DATA + 0x3014) as u64
        };
        c.g[A2][0] = if seed / 64 % 2 == 0 {
            0
        } else {
            (DATA + 0x3019) as u64
        };
        r
    }

    /// Sync: nothing outstanding (-1); the poll leg; the wait leg with the
    /// RPC done at once or after one to three delays; each settling through
    /// `SignalSema` with the outputs written or not.
    #[test]
    fn sce_mc_sync_matches() {
        let call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_sync_call(c, seed);
        };
        check(ee::s15::f_0040a360__generated, sce_mc_sync, 128, call);
        require_paths(
            ee::s15::f_0040a360__generated,
            128,
            &[
                25, 45, 47, 49, 50, 51, 52, 54, 101, 103, 104, 106, 126, 128, 129, 131, 151, 153,
                154, 156,
            ],
            call,
        );
    }
    driven!(gen_sce_mc_sync, ee::s15::f_0040a360__generated);
    driven!(hand_sce_mc_sync, sce_mc_sync);

    /// Sync with live events: the wait loop's delay head (0x40a3c8) is
    /// where an unwinding event re-enters it.
    #[test]
    fn sce_mc_sync_events() {
        check_events(gen_sce_mc_sync, hand_sce_mc_sync, 384, &|c, seed| {
            let mut r = mc_sync_call(c, seed / 3);
            schedule(c, &mut r, seed, 0x0040_a360)
        });
    }

    /// UdCheckNewCard: the RPC fails (no free packets: prints, -1). Again with
    /// the RPC answered (`mc_answered`): the RPC is issued and returns the
    /// random reply word.
    #[test]
    fn sce_mc_ud_check_new_card_matches() {
        let mc_ud_check_new_card_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_rpc_setup(c, seed, 0x0040_a618);
            c.pc = 0x0040_a618;
            if seed % 2 == 1 {
                mc_free_packets(c);
            }
        };
        check(
            ee::s15::f_0040a618__generated,
            sce_mc_ud_check_new_card,
            60,
            mc_ud_check_new_card_call,
        );
        require_paths(
            ee::s15::f_0040a618__generated,
            60,
            &[185],
            mc_ud_check_new_card_call,
        );
        check_answered(
            ee::s15::f_0040a618__generated,
            sce_mc_ud_check_new_card,
            36,
            &[332],
            mc_ud_check_new_card_call,
        );
    }

    /// Unformat: layer down (-100), sema busy (-200), and the RPC (fails fast,
    /// -1). Again with the RPC answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_unformat_matches() {
        let mc_unformat_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_rpc_setup(c, seed, 0x0040_af58);
            c.pc = 0x0040_af58;
            c.g[A0][0] = (seed % 2) as u64;
            c.g[A1][0] = (seed % 3) as u64;
            match seed % 3 {
                0 => {}
                1 => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    c.bus.write32(LIBMC_MAIN_SEMA, 0x1234);
                }
                _ => {
                    c.bus.write32(LIBMC_CLIENT + 0x24, 1);
                    mc_live_sema(c);
                }
            }
        };
        check(
            ee::s15::f_0040af58__generated,
            sce_mc_unformat,
            200,
            mc_unformat_call,
        );
        require_paths(
            ee::s15::f_0040af58__generated,
            200,
            &[21, 30, 121],
            mc_unformat_call,
        );
        check_answered(
            ee::s15::f_0040af58__generated,
            sce_mc_unformat,
            36,
            &[340],
            mc_unformat_call,
        );
    }

    /// Write: unbound (-100), sema busy (-200) and the RPC leg, with empty,
    /// small (< 17) and larger sizes, and aligned and misaligned sources (the
    /// head-copy loop only runs for misaligned ones). Again with the RPC
    /// answered (`mc_answered`): the RPC-success leg.
    #[test]
    fn sce_mc_write_matches() {
        let mc_write_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            mc_setup(c, seed, 0x0040_a1a8);
            mc_legs(c, seed);
            if seed / 9 % 2 == 1 {
                mc_free_packets(c);
            }
            let src = DATA + 0x4000 + (seed % 3);
            for i in 0..64 {
                c.bus.write8(src + i as u32, (i * 7 + seed) as u8);
            }
            c.g[A0][0] = seed as u64;
            c.g[A1][0] = src as u64;
            c.g[A2][0] = [0u64, 1, 5, 16, 17, 20, 32, 40][seed as usize % 8];
        };
        check(
            ee::s15::f_0040a1a8__generated,
            sce_mc_write,
            120,
            mc_write_call,
        );
        require_paths(
            ee::s15::f_0040a1a8__generated,
            120,
            &[26, 35, 142, 155, 187, 266, 275],
            mc_write_call,
        );
        check_answered(
            ee::s15::f_0040a1a8__generated,
            sce_mc_write,
            36,
            &[361, 368, 374, 406, 485, 493, 494],
            mc_write_call,
        );
    }

    /// `memcpy` destination for the `libmc_ChdirCallback` test.
    pub(crate) const CHDIR_DST: u32 = DATA + 0x900;

    /// Fixture entry buffer for the `sceMcGetDir` writeback.
    pub(crate) const MC_ENTRIES: u32 = DATA + 0x700;

    /// File-info struct for the `sceMcSetFileInfo` test.
    pub(crate) const MC_INFO: u32 = DATA + 0x800;

    /// Fixture name string for the `sceMc*` tests.
    pub(crate) const MC_NAME: u32 = DATA + 0x600;

    /// NUL-terminated `s` at `addr`.
    pub(crate) fn cstr(c: &mut Cpu, addr: u32, s: &[u8]) {
        for (i, &b) in s.iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// Steer the bind/sema legs: seed % 3 == 0 leaves the RPC unbound
    /// (-100), == 1 leaves the main sema bogus (PollSema fails, -200),
    /// else binds with a real sema (the RPC leg).
    pub(crate) fn mc_legs(c: &mut Cpu, seed: u32) {
        if seed % 3 != 0 {
            c.bus.write32(LIBMC_RPC_READY, 1);
        }
        if seed % 3 == 2 {
            let id = mc_real_sema(c);
            c.bus.write32(LIBMC_MAIN_SEMA, id);
        }
    }

    /// A real kernel sema with count 1, for the PollSema/WaitSema legs.
    /// Saves `pc`: the dispatcher ports are entered by `pc`.
    pub(crate) fn mc_real_sema(c: &mut Cpu) -> u32 {
        let pc = c.pc;
        c.bus.write32(DATA + 0x5000, 0);
        c.bus.write32(DATA + 0x5004, 1);
        c.bus.write32(DATA + 0x5008, 1);
        c.bus.write32(DATA + 0x500c, 0);
        c.bus.write32(DATA + 0x5010, 0);
        c.bus.write32(DATA + 0x5014, 0);
        c.g[A0][0] = (DATA + 0x5000) as u64;
        ee::s15::CreateSema(c);
        let id = c.g[V0][0] as u32;
        c.pc = pc;
        id
    }

    /// Packet array for [`mc_free_packets`] (0x40 bytes each).
    const SIF_RPC_PACKETS: u32 = DATA + 0x7004;

    /// Give the SIF RPC pool four free packets (a test machine has none), so
    /// `sceSifCallRpc` takes one and records its arguments, the end
    /// callback in t3 among them, in the client.
    fn mc_free_packets(c: &mut Cpu) {
        c.bus.write32(SIF_RPC_POOL, 0);
        c.bus.write32(SIF_RPC_POOL + 4, SIF_RPC_PACKETS);
        c.bus.write32(SIF_RPC_POOL + 8, 4);
        for i in 0..4 {
            for w in 0..16 {
                c.bus.write32(SIF_RPC_PACKETS + i * 0x40 + w * 4, 0);
            }
            c.bus.write32(SIF_RPC_PACKETS + i * 0x40 + 0x18, 0x1000 + i);
        }
    }

    /// Fresh libmc state: scrambled regs, game gp, `pc` at `entry`, the
    /// RPC marked unbound and the sema/result words cleared. No SIF
    /// packets either, so a real `sceSifCallRpc` fails fast (-1) instead
    /// of talking to the IOP.
    pub(crate) fn mc_setup(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = Rng(0x0040_9c48 ^ (seed as u64) << 12 ^ entry as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = entry;
        // No free SIF packets: the RPC calls fail fast.
        c.bus.write32(0x0052_d688, 0);
        c.bus.write32(LIBMC_RPC_READY, 0);
        c.bus.write32(LIBMC_MAIN_SEMA, 0);
        c.bus.write32(LIBMC_DELAY_SEMA, 0);
        c.bus.write32(LIBMC_LAST_FN, 0);
        r
    }

    /// Create a real sema with count 1 and install it as the MC serial
    /// sema, so `PollSema` succeeds and the failure paths `SignalSema`
    /// a live id. Returns the id.
    pub(crate) fn mc_live_sema(c: &mut Cpu) -> u32 {
        for (i, w) in [0u32, 1, 1, 0, 0, 0].iter().enumerate() {
            c.bus.write32(SEMA_TEMPLATE + i as u32 * 4, *w);
        }
        c.g[A0][0] = SEMA_TEMPLATE as u64;
        ee::s15::CreateSema(c);
        let id = c.g[V0][0] as u32;
        c.bus.write32(LIBMC_MAIN_SEMA, id);
        id
    }

    /// Write a NUL-terminated fixture name.
    pub(crate) fn put_name(c: &mut Cpu, addr: u32, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            c.bus.write8(addr + i as u32, *b);
        }
        c.bus.write8(addr + bytes.len() as u32, 0);
    }

    /// Common fixture: ELF image (format strings, tables), scrambled
    /// registers, game-like gp, the SIF RPC pool drained (the real
    /// `sceSifCallRpc` fails fast with -1), the MC client down, and the
    /// SIF packet pool zeroed. Each test raises what its seeds need.
    pub(crate) fn mc_rpc_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x0040_b6d0 ^ (seed as u64) << 12 ^ salt);
        elf_image(c);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(SIF_RPC_POOL_COUNT, 0);
        c.bus.write32(LIBMC_CLIENT + 0x24, 0);
        c.bus.write32(MRPC_POOL, 0);
        c.bus.write32(MRPC_POOL + 4, 0);
        c.bus.write32(MRPC_POOL + 8, 0);
        r
    }

    /// Every memory-card RPC answered, on top of a port's own setup: the
    /// layer bound with a live libmc sema, the SIF RPC library set up with
    /// free packets (`cdvd::tests::rpc_answered`: sends go out and the fake
    /// `WaitSema` returns at once for the blocking calls), random reply words
    /// in the receive buffer, on the main guest thread. Keeps the registers
    /// and `pc` the setup left.
    fn mc_answered(c: &mut Cpu, seed: u32) {
        let (regs, pc) = (c.g, c.pc);
        let mut r = Rng(0x0052_71c0 ^ (seed as u64) << 12);
        c.bus.write32(LIBMC_RPC_READY, 1);
        mc_live_sema(c);
        crate::game::lib::cdvd::tests::rpc_answered(c, LIBMC_CLIENT, 0);
        crate::difftest::fill(c, &mut r, MC_RECV, 0x110);
        ssx3_ps2::kernel::start_main_thread(c, pc, 0, 0);
        (c.g, c.pc) = (regs, pc);
    }

    /// `check_paths` of a port's setup with every RPC answered
    /// ([`mc_answered`]).
    fn check_answered(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        paths: &[u64],
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        let answered = |c: &mut Cpu, seed: u32| {
            setup(c, seed);
            mc_answered(c, seed);
        };
        crate::difftest::check_paths(generated, hand, seeds, paths, &answered);
    }

    /// Sema template for the `sceMc*` PollSema-success path (count 1).
    pub(crate) const SEMA_TEMPLATE: u32 = DATA + 0x5000;

    driven!(sce_mc_write_gen_driven, ee::s15::f_0040a1a8__generated);
    driven!(sce_mc_write_hand_driven, sce_mc_write);

    /// Writing to the card with the layer up and the card sema free: the
    /// bytes before the source's first 16-byte boundary (or a whole small
    /// write) are copied one at a time, and a live event lands in that copy.
    #[test]
    fn sce_mc_write_events() {
        check_events(
            sce_mc_write_gen_driven,
            sce_mc_write_hand_driven,
            216,
            &|c, seed| {
                // mc_setup leaves the SIF pool drained: the RPC fails fast
                // (-1) and the sema is signalled back.
                let mut r = mc_setup(c, seed, 0x0040_a1a8);
                c.bus.write32(LIBMC_RPC_READY, 1);
                let id = mc_real_sema(c);
                c.bus.write32(LIBMC_MAIN_SEMA, id);
                // Small writes copy every byte; 17 and 40 copy the 13..15
                // bytes up to the boundary.
                let size = [13u64, 14, 15, 16, 17, 40][(seed / 3 % 6) as usize];
                let src = DATA + 0x4000 + 1 + seed / 3 / 6 % 3;
                for i in 0..64u32 {
                    c.bus.write8(src + i, (i * 7 + seed) as u8 | 1);
                }
                c.g[A0][0] = (seed / 3 / 18 % 2) as u64;
                c.g[A1][0] = src as u64;
                c.g[A2][0] = size;
                let mode = schedule(c, &mut r, seed, 0x0040_a1a8);
                if mode != Events::Off {
                    // Past PollSema's own check (about 24 cycles in), inside
                    // the copy (first back edge about 49 cycles in, the
                    // shortest copy's last about 137).
                    c.next_event = c.cycles + 60 + (r.next_u32() % 76) as u64;
                }
                mode
            },
        );
    }

    use crate::difftest::fakes::machine;
    pub(crate) use crate::game::seg::seg_2fe810::tests::oc1592_arm;

    pub(crate) use crate::game::seg::seg_2fe810::tests::oc1592_install;

    pub(crate) fn oc1592_alarm_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x1598, 1);
        oc1592_arm(c, seed);
        c.g[A2][0] = (seed as u64) * 3 + 1;
    }

    #[test]
    fn libmc_delay_alarm_matches() {
        oc1592_install();
        check(
            ee::s15::f_0040a320__generated,
            libmc_delay_alarm,
            16,
            &oc1592_alarm_setup,
        );
    }
}
