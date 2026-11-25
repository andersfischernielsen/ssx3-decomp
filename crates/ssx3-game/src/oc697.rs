//! Staging module oc697 (wave wave237): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc697`.

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
    (0x003e_3618, ea_file_cd_reset, "EAFile_CdReset"),
    (0x003e_3758, ea_file_cd_scan_dir, "EAFile_CdScanDir"),
    (0x003e_3968, ea_file_cd_compare_hash, "EAFile_CdCompareHash"),
    (0x003e_3be0, ea_file_cd_read_thread, "EAFile_CdReadThread"),
    (0x003e_52b0, thread_yieldticks, "THREAD_yieldticks"),
    (0x003e_5698, ea_mutex_init, "EAMutex_Init"),
    (0x003e_56e0, ea_mutex_destroy, "EAMutex_Destroy"),
    (
        0x0040_09a8,
        libcdvd_callback_thread,
        "libcdvd_CallbackThread",
    ),
    (
        0x0040_0b48,
        libcdvd_read_rpc_end_callback,
        "libcdvd_ReadRpcEndCallback",
    ),
    (
        0x0042_5cc0,
        sce_sif_cmd_set_sreg_handler,
        "sceSifCmd_SetSregHandler",
    ),
    (0x0042_5cf0, sce_sif_get_sreg, "sceSifGetSreg"),
    (0x0042_5ff0, sce_sif_set_cmd_buffer, "sceSifSetCmdBuffer"),
];

/// How often `EAFile_CdCompareHash` found two equal hashes (a collision
/// counter, `[0x51ed80]`).
pub(crate) const EA_FILE_CD_HASH_EQUAL_COUNT: u32 = 0x0051_ed80;

/// `int EAFile_CdCompareHash(const u32 *a, const u32 *b)` (0x3e3968): compare
/// two file-name hashes (`-1`, `1`, or `0` when equal). Equal hashes also
/// count a collision in [`EA_FILE_CD_HASH_EQUAL_COUNT`]. The 64-bit difference
/// of the zero-extended words is tested, so the full unsigned range compares
/// correctly.
pub fn ea_file_cd_compare_hash(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = c.lwu(c.g[A0][0] as u32);
    c.g[V0][0] = c.lwu(c.g[A1][0] as u32);
    c.g[V1][0] = c.g[V1][0].wrapping_sub(c.g[V0][0]);
    let diff = c.g[V1][0] as i64;
    c.g[V0][0] = u64::MAX;
    c.cycles += 2;
    if diff < 0 {
        ret(c);
        return;
    }
    c.g[A0][0] = 0x520000;
    if diff > 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V0][0] = 0;
    c.g[V1][0] = c.lw(EA_FILE_CD_HASH_EQUAL_COUNT);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    // The store sits in the return's delay slot.
    c.sw(EA_FILE_CD_HASH_EQUAL_COUNT, c.g[V1][0]);
    ret(c);
}

/// Offsets in the semaphore-backed mutex (`MUTEX`): two counters the lock
/// code keeps (cleared at init) and the kernel semaphore id.
pub(crate) const EA_MUTEX_COUNT_A: u32 = 0x4;
pub(crate) const EA_MUTEX_COUNT_B: u32 = 0x8;
pub(crate) const EA_MUTEX_SEMA: u32 = 0xc;

/// `int EAMutex_Init(MUTEX *m)` (0x3e5698, `MUTEX_create`): clear the mutex's
/// two counters and create its kernel semaphore (initial and maximum count
/// 1, from a parameter block on the stack), keeping the id. Returns 0.
pub fn ea_mutex_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(EA_MUTEX_COUNT_B), 0u64);
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(EA_MUTEX_COUNT_A), 0u64);
        // `ee_sema_t`: max_count and init_count are 1 (count and the rest
        // are whatever the stack held).
        c.sw(frame.at(c, 8), c.g[V0][0]);
        c.g[RA][0] = sx(0x003e_56c4);
        c.sw(frame.at(c, 4), c.g[V0][0]);
        Callee {
            addr: 0x0042_3da0,
            f: ee::s15::CreateSema,
        }
        .call(c, 0x003e_56c4)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(EA_MUTEX_SEMA), c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void EAMutex_Destroy(MUTEX *m)` (0x3e56e0, `MUTEX_destroy`): delete the
/// mutex's kernel semaphore.
pub fn ea_mutex_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_56f0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(EA_MUTEX_SEMA));
        Callee {
            addr: 0x0042_3db0,
            f: ee::s15::DeleteSema,
        }
        .call(c, 0x003e_56f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void sceSifCmd_SetSregHandler(cmd, mgr)` (0x425cc0): the SIF "set SREG"
/// command's handler. `cmd` holds the register index (`+0x10`) and the value
/// (`+0x14`); the value goes to `mgr`'s SREG table (pointer at `+0x1c`).
pub fn sce_sif_cmd_set_sreg_handler(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    // The store sits in the return's delay slot.
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    ret(c);
}

/// The SIF SREG table (`sceSifGetSreg`): 32-bit registers at `0x52be00`.
pub(crate) const SIF_SREG_TABLE: u32 = 0x0052_be00;

/// `u32 sceSifGetSreg(int n)` (0x425cf0): SIF software register `n` (the
/// word the IOP set with the SREG command).
pub fn sce_sif_get_sreg(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x530000;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V0][0] = sx(SIF_SREG_TABLE);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
    ret(c);
}

/// The SIF command state: the receive buffer pointer at `+0x14` and its size
/// at `+0x18` (`sceSifSetCmdBuffer`).
pub(crate) const SIF_CMD_STATE: u32 = 0x0052_bcd8;

/// `void *sceSifSetCmdBuffer(void *buf, int size)` (0x425ff0): install the
/// SIF command receive buffer, returning the previous buffer pointer.
pub fn sce_sif_set_cmd_buffer(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = sx(SIF_CMD_STATE);
    c.g[V0][0] = c.lw(SIF_CMD_STATE + 0x14);
    c.sw(SIF_CMD_STATE + 0x18, c.g[A1][0]);
    // The buffer store sits in the return's delay slot.
    c.sw(SIF_CMD_STATE + 0x14, c.g[A0][0]);
    ret(c);
}

/// `char *EAFile_CdReset(void)` (0x3e3618): (re)read the CD sector at the
/// current LBA into the sector buffer `0x519c80` and return it. Retries up to
/// ten times on drive errors (a delay between polls); when the budget is
/// used up it re-initialises the drive (`sceCdInit(0)`, `sceCdMmode`) and
/// starts over. A `match pc` body (back edges deliver events): re-entered at
/// its block heads after an unwind.
pub fn ea_file_cd_reset(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e3618 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.g[V0][0] = 0x450000;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = 0x520000;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = 0xa;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x63c0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S4][0] = 0x520000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc58));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    pc = 0x003e3650;
                    continue;
                }
                0x003e3650 => {
                    c.cycles += 2;
                    let t = c.g[S2][0] != 0u64;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x003e3668;
                        continue;
                    }
                    pc = 0x003e3658;
                    continue;
                }
                0x003e3658 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e3660);
                    c.g[S2][0] = 0xa;
                    call_ptr(c, 0x00401718, 0x003e3660)?;
                    c.g[RA][0] = sx(0x003e3668);
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                    call_ptr(c, 0x00402450, 0x003e3668)?;
                    pc = 0x003e3668;
                    continue;
                }
                0x003e3668 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e3670);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x00401bf0, 0x003e3670)?;
                    let t = true;
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x003e3680;
                        continue;
                    }
                    pc = 0x003e3678;
                    continue;
                }
                0x003e3678 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003e3680);
                    c.g[A0][0] = 0x1e0;
                    Callee {
                        addr: 0x003e35b0,
                        f: ee::s14::EAFile_DelayTicks,
                    }
                    .call(c, 0x003e3680)?;
                    pc = 0x003e3680;
                    continue;
                }
                0x003e3680 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6380));
                    c.g[RA][0] = sx(0x003e3694);
                    c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c));
                    call_ptr(c, 0x00401df8, 0x003e3694)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x003e3678) {
                            return Ok(());
                        }
                        pc = 0x003e3678;
                        continue;
                    }
                    pc = 0x003e369c;
                    continue;
                }
                0x003e369c => {
                    c.cycles += 1;
                    pc = 0x003e36a0;
                    continue;
                }
                0x003e36a0 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x003e36a8);
                    c.g[A0][0] = 0x1;
                    call_ptr(c, 0x004013e8, 0x003e36a8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e36b4);
                    c.g[A0][0] = 0x1e0;
                    Callee {
                        addr: 0x003e35b0,
                        f: ee::s14::EAFile_DelayTicks,
                    }
                    .call(c, 0x003e36b4)?;
                    let t = c.g[S0][0] != 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x003e36a0) {
                            return Ok(());
                        }
                        pc = 0x003e36a0;
                        continue;
                    }
                    pc = 0x003e36bc;
                    continue;
                }
                0x003e36bc => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003e36c4);
                    call_ptr(c, 0x00402300, 0x003e36c4)?;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    if c.g[V0][0] == 0 {
                        c.g[S2][0] = c.g[S1][0];
                    }
                    if t {
                        if back_edge_unwinds(c, 0x003e3650) {
                            return Ok(());
                        }
                        pc = 0x003e3650;
                        continue;
                    }
                    pc = 0x003e36d0;
                    continue;
                }
                0x003e36d0 => {
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6380));
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e3618, 0x003e36f8);
                    return Ok(());
                }
            }
        }
    });
}

/// `void EAFile_CdScanDir(int lba, int sectors)` (0x3e3758): walk the ISO9660
/// directory at `lba` (`sectors` sectors long), appending every file to the
/// file table (`[0x450c68] + n * 0x108`: path, LBA, size; count at
/// `[0x450c6c]`) and recursing into subdirectories with the sector buffer
/// and the path saved around the call. Re-entered at its block heads after
/// an unwind (`match pc` body).
pub fn ea_file_cd_scan_dir(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e3758 => {
                    c.cycles += 29;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x9c0));
                    c.g[V0][0] = 0x520000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x9a0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x990));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x980));
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x970));
                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63a0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x960));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x950));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x940));
                    c.g[S6][0] = 0x450000;
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x930));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x9b0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x920), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x910), c.g[V0][0]);
                    c.g[V0][0] = 0x520000;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[A0][0]);
                    c.g[RA][0] = sx(0x003e37b0);
                    c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1380));
                    Callee {
                        addr: 0x003e3618,
                        f: ee::s14::EAFile_CdReset,
                    }
                    .call(c, 0x003e37b0)?;
                    c.g[A0][0] = c.lb((c.g[V0][0] as u32));
                    c.g[V1][0] = 0x520000;
                    c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = true;
                    c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6380));
                    if t {
                        pc = 0x003e38fc;
                        continue;
                    }
                    pc = 0x003e37cc;
                    continue;
                }
                0x003e37cc => {
                    c.cycles += 1;
                    pc = 0x003e37d0;
                    continue;
                }
                0x003e37d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & 0x2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        pc = 0x003e386c;
                        continue;
                    }
                    pc = 0x003e37dc;
                    continue;
                }
                0x003e37dc => {
                    c.cycles += 36;
                    c.g[A2][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x003e37e8);
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21));
                    call_ptr(c, 0x0041605c, 0x003e37e8)?;
                    c.g[A2][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[A3][0] = 0x2f;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[S7][0];
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.sb((c.g[V1][0] as u32), c.g[A3][0]);
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[SP][0] as u32)));
                    c.g[RA][0] = sx(0x003e3810);
                    c.sb((c.g[V0][0] as u32).wrapping_add(0x1), 0u64);
                    call_ptr(c, 0x004166f4, 0x003e3810)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003e381c);
                    c.g[A0][0] = c.g[S7][0];
                    call_ptr(c, 0x004162d0, 0x003e381c)?;
                    c.g[A2][0] = 0x800;
                    c.g[A1][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x003e382c);
                    c.g[A0][0] = c.g[S4][0];
                    Callee {
                        addr: 0x003e6574,
                        f: ee::s14::EA_MemCopy,
                    }
                    .call(c, 0x003e382c)?;
                    c.lwl(5, (c.g[S1][0] as u32).wrapping_add(0xd));
                    c.lwr(5, (c.g[S1][0] as u32).wrapping_add(0xa));
                    c.lwl(4, (c.g[S1][0] as u32).wrapping_add(0x5));
                    c.lwr(4, (c.g[S1][0] as u32).wrapping_add(0x2));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7ff));
                    c.g[RA][0] = sx(0x003e3848);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 11);
                    Callee {
                        addr: 0x003e3758,
                        f: ee::s14::EAFile_CdScanDir,
                    }
                    .call(c, 0x003e3848)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x003e3858);
                    c.g[A2][0] = 0x800;
                    Callee {
                        addr: 0x003e6574,
                        f: ee::s14::EA_MemCopy,
                    }
                    .call(c, 0x003e3858)?;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[RA][0] = sx(0x003e3864);
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x004166f4, 0x003e3864)?;
                    let t = true;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32));
                    if t {
                        pc = 0x003e3900;
                        continue;
                    }
                    pc = 0x003e386c;
                    continue;
                }
                0x003e386c => {
                    c.cycles += 14;
                    c.g[A2][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21));
                    c.g[RA][0] = sx(0x003e387c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x2));
                    Callee {
                        addr: 0x003e6574,
                        f: ee::s14::EA_MemCopy,
                    }
                    .call(c, 0x003e387c)?;
                    c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = 0x2e;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[SP][0] as u32)));
                    c.sb((c.g[V1][0] as u32).wrapping_sub(0x2), 0u64);
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32));

                    if c.g[V1][0] == c.g[A0][0] {
                        c.sb((c.g[V0][0] as u32), 0u64);
                        pc = 0x003e38a4;
                        continue;
                    }
                    pc = 0x003e38a4;
                    continue;
                }
                0x003e38a4 => {
                    c.cycles += 22;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc6c));
                    c.g[V1][0] = 0x108;
                    c.g[A0][0] = 0x450000;
                    c.mult(0, 2, 2, 3);
                    c.g[S0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc68));
                    c.g[A0][0] = 0x520000;
                    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1380));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x003e38cc);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x004166f4, 0x003e38cc)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003e38d8);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x004162d0, 0x003e38d8)?;
                    c.lwl(4, (c.g[S1][0] as u32).wrapping_add(0x5));
                    c.lwr(4, (c.g[S1][0] as u32).wrapping_add(0x2));
                    c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc6c));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x100), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.lwl(2, (c.g[S1][0] as u32).wrapping_add(0xd));
                    c.lwr(2, (c.g[S1][0] as u32).wrapping_add(0xa));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    c.sw((c.g[S6][0] as u32).wrapping_add(0xc6c), c.g[V1][0]);
                    pc = 0x003e38fc;
                    continue;
                }
                0x003e38fc => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32));
                    pc = 0x003e3900;
                    continue;
                }
                0x003e3900 => {
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    pc = 0x003e3904;
                    continue;
                }
                0x003e3904 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32));

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x19));
                        if back_edge_unwinds(c, 0x003e37d0) {
                            return Ok(());
                        }
                        pc = 0x003e37d0;
                        continue;
                    }
                    pc = 0x003e3910;
                    continue;
                }
                0x003e3910 => {
                    c.cycles += 2;
                    let t = c.g[S2][0] == 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x003e3930;
                        continue;
                    }
                    pc = 0x003e3918;
                    continue;
                }
                0x003e3918 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x003e3928);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    Callee {
                        addr: 0x003e3618,
                        f: ee::s14::EAFile_CdReset,
                    }
                    .call(c, 0x003e3928)?;
                    let t = true;
                    c.g[S1][0] = c.g[V0][0];
                    if t {
                        if back_edge_unwinds(c, 0x003e3904) {
                            return Ok(());
                        }
                        pc = 0x003e3904;
                        continue;
                    }
                    pc = 0x003e3930;
                    continue;
                }
                0x003e3930 => {
                    c.cycles += 14;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x910));
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x9b0));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x9a0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x990));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x980));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x970));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x960));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x950));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x940));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x930));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x920));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x9c0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e3758, 0x003e3968);
                    return Ok(());
                }
            }
        }
    });
}

/// `void EAFile_CdReadThread(void)` (0x3e3be0): the CD read thread, which never
/// returns. Waits on the request semaphore, then handles a drive error or
/// an abort flag (re-issuing the read while retries last), the copy of the
/// sector just read into the caller's buffer, and the next sector run
/// (`EAFile_CdIssueRead`) or the completion of the request.
pub fn ea_file_cd_read_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.g[V0][0] = 0x520000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63a0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = 0x520000;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = 0x1;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S4][0] = 0xfffffffffffffffe;
        'l_3e3c18: loop {
            'b_3e3c7c: {
                'b_3e3c3c: {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x003e3c20);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x00423de0, 0x003e3c20)?;
                    c.g[RA][0] = sx(0x003e3c28);
                    call_ptr(c, 0x00402300, 0x003e3c28)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                    if t {
                        break 'b_3e3c3c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V1][0] & 0x8;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.g[V1][0] & 0x1;
                        if t {
                            break 'b_3e3c7c;
                        } else {
                            break 'b_3e3c3c;
                        }
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = 0xfffffffffffffff7;
                c.g[A0][0] = c.g[V1][0] & c.g[V0][0];
                c.g[V1][0] = c.g[V1][0] & 0x2;
                let t = c.g[V1][0] == 0u64;
                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[A0][0]);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003e3c18);
                    }
                    continue 'l_3e3c18;
                } else {
                    'b_3e3c70: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                        if t {
                            break 'b_3e3c70;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x003e3c68);
                            Callee {
                                addr: 0x003e3b00,
                                f: ee::s14::EAFile_CdIssueRead,
                            }
                            .call(c, 0x003e3c68)?;
                            let t = true;

                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003e3c18);
                                }
                                continue 'l_3e3c18;
                            } else {
                                break 'b_3e3c70;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A0][0] | 0x4;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003e3c18);
                        }
                        continue 'l_3e3c18;
                    } else {
                        break 'b_3e3c7c;
                    }
                }
            }
            'b_3e3ca8: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.g[V1][0] & c.g[S4][0];
                if t {
                    break 'b_3e3ca8;
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3c));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V1][0]);
                    c.g[RA][0] = sx(0x003e3ca8);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[A3][0]);
                    Callee {
                        addr: 0x003e6574,
                        f: ee::s14::EA_MemCopy,
                    }
                    .call(c, 0x003e3ca8)?;
                    break 'b_3e3ca8;
                }
            }
            'b_3e3d54: {
                c.cycles += 3;
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                let t = (c.g[A3][0] as i64) <= 0;
                c.g[A2][0] = ((c.g[A3][0] as i64) < 2048i64) as u64;
                if t {
                    break 'b_3e3d54;
                } else {
                    'b_3e3d38: {
                        'b_3e3cf4: {
                            c.cycles += 10;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                            let t = c.g[A2][0] == 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                            if t {
                                break 'b_3e3cf4;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                                c.sw((c.g[S0][0] as u32), c.g[A3][0]);
                                c.g[V0][0] = c.g[V0][0] | 0x1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                                if t {
                                    break 'b_3e3d38;
                                } else {
                                    break 'b_3e3cf4;
                                }
                            }
                        }
                        'b_3e3d18: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[V0][0] & 0x3f;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                            if t {
                                break 'b_3e3d18;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = 0x800;
                                c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                                c.g[V1][0] = c.g[V1][0] | 0x1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V1][0]);
                                if t {
                                    break 'b_3e3d38;
                                } else {
                                    break 'b_3e3d18;
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                        c.g[V1][0] = 0xfffffffffffff800;
                        c.g[V1][0] = c.g[A3][0] & c.g[V1][0];
                        c.g[V0][0] = c.g[V0][0] & c.g[S4][0];
                        c.g[A0][0] = sx(((c.g[V1][0] as u32) as i32 >> 11) as u32);
                        c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                        c.sw((c.g[S0][0] as u32), c.g[V1][0]);
                        break 'b_3e3d38;
                    }
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                    c.g[RA][0] = sx(0x003e3d4c);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    Callee {
                        addr: 0x003e3b00,
                        f: ee::s14::EAFile_CdIssueRead,
                    }
                    .call(c, 0x003e3d4c)?;
                    let t = true;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003e3c18);
                        }
                        continue 'l_3e3c18;
                    } else {
                        break 'b_3e3d54;
                    }
                }
            }
            c.cycles += 8;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
            c.g[V1][0] = 0xfffffffffffffffd;
            c.g[A0][0] = 0x1;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[RA][0] = sx(0x003e3d6c);
            c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
            Callee {
                addr: 0x003de420,
                f: ee::s14::EAFile_CompleteRequest,
            }
            .call(c, 0x003e3d6c)?;
            let t = true;

            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x003e3c18);
                }
                continue 'l_3e3c18;
            } else {
                c.cycles += 1;
                c.pc = 0x003e_3d78;
                ee::s14::EAFile_CdMountDisc(c);
                return Ok(());
            }
        }
    });
}

/// `void THREAD_yieldticks(int ticks)` (0x3e52b0): sleep `ticks` thread ticks:
/// an alarm for `ticks * rate / tick_rate` (masked to 16 bits) that wakes this
/// thread, then `SleepThread`. With `ticks <= 0` it rotates this thread's
/// ready queue instead (the `i` syscalls when in interrupt context).
pub fn thread_yieldticks(c: &mut Cpu) {
    guest(c, |c| {
        'b_3e5380: {
            'b_3e532c: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                let t = (c.g[S2][0] as i64) <= 0;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                if t {
                    break 'b_3e532c;
                } else {
                    'b_3e52e8: {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x003e52d4);
                        c.g[S1][0] = 0x3e0000;
                        Callee {
                            addr: 0x003e5008,
                            f: ee::s14::EATimer_GetTickRate,
                        }
                        .call(c, 0x003e52d4)?;
                        c.g[V1][0] = 0x450000;
                        c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xda0));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5290));
                        if c.g[V0][0] == 0u64 {
                            break 'b_3e52e8;
                        } else {
                            break 'b_3e52e8;
                        }
                    }
                    'b_3e531c: {
                        c.cycles += 11;
                        c.mult(0, 16, 18, 16);
                        c.div(0, 16, 2);
                        c.g[S0][0] = c.lo[0];
                        c.g[RA][0] = sx(0x003e52fc);
                        c.g[S0][0] = c.g[S0][0] & 0xffff;
                        call_ptr(c, 0x00423c90, 0x003e52fc)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x003e530c);
                        c.g[A2][0] = c.g[V0][0];
                        call_ptr(c, 0x00423b20, 0x003e530c)?;
                        let t = (c.g[V0][0] as i64) >= 0;
                        c.g[A0][0] = 0x490000;
                        if t {
                            break 'b_3e531c;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x003e531c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5e20));
                            Callee {
                                addr: 0x003e6070,
                                f: ee::s14::EA_DebugMessage,
                            }
                            .call(c, 0x003e531c)?;
                            break 'b_3e531c;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e5324);
                    call_ptr(c, 0x00423cc0, 0x003e5324)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_3e5380;
                    } else {
                        break 'b_3e532c;
                    }
                }
            }
            'b_3e5360: {
                c.cycles += 4;
                c.g[V0][0] = 0x450000;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xde4));
                let t = c.g[V1][0] == 0u64;

                if t {
                    break 'b_3e5360;
                } else {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x003e5344);
                    call_ptr(c, 0x00423c90, 0x003e5344)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e5350);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00423cb0, 0x003e5350)?;
                    c.g[RA][0] = sx(0x003e5358);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x00424ce8, 0x003e5358)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_3e5380;
                    } else {
                        break 'b_3e5360;
                    }
                }
            }
            c.cycles += 8;
            c.g[RA][0] = sx(0x003e5368);
            call_ptr(c, 0x00423c90, 0x003e5368)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003e5374);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x00423ca0, 0x003e5374)?;
            c.g[RA][0] = sx(0x003e537c);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x00423c50, 0x003e537c)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_3e5380;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `void libcdvd_CallbackThread(void)` (0x4009a8): the libcdvd callback thread.
/// Waits on its semaphore, exits (and deletes) itself when the request word is
/// -1, otherwise optionally prints the pending callback, runs the registered
/// callback with the saved argument (swapping in its `gp`) and clears the
/// busy flag.
pub fn libcdvd_callback_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[FP][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S7][0]);
        c.g[FP][0] = 0x520000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S6][0]);
        c.g[S7][0] = 0x450000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S5][0]);
        c.g[S6][0] = 0x490000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S4][0]);
        c.g[S5][0] = 0x520000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S3][0]);
        c.g[S4][0] = 0x450000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
        c.g[S3][0] = 0x450000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.g[S2][0] = 0x520000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
        c.g[S1][0] = 0x450000;
        c.sd((c.g[SP][0] as u32), c.g[S0][0]);
        c.g[V0][0] = 0x450000;
        'l_4009f8: loop {
            'b_400a2c: {
                c.cycles += 6;
                c.g[RA][0] = sx(0x00400a00);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1620));
                Callee {
                    addr: 0x00423de0,
                    f: ee::s15::WaitSema,
                }
                .call(c, 0x00400a00)?;
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1658));
                c.g[V0][0] = 0xffffffffffffffff;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x1610));
                if t {
                    break 'b_400a2c;
                } else {
                    c.cycles += 7;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1634), 0u64);
                    c.g[V0][0] = 0x450000;
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x1658), 0u64);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1614), 0u64);
                    c.g[RA][0] = sx(0x00400a28);
                    c.sw((c.g[FP][0] as u32).wrapping_add(0x64d4), 0u64);
                    Callee {
                        addr: 0x00423be0,
                        f: ee::s15::ExitDeleteThread,
                    }
                    .call(c, 0x00400a28)?;
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x1610));
                    break 'b_400a2c;
                }
            }
            'b_400a40: {
                c.cycles += 2;
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x79e0));
                if t {
                    break 'b_400a40;
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x64c0));
                    c.g[RA][0] = sx(0x00400a40);
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x165c));
                    Callee {
                        addr: 0x00425c60,
                        f: ee::s15::scePrintf,
                    }
                    .call(c, 0x00400a40)?;
                    break 'b_400a40;
                }
            }
            'b_400a6c: {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x64c0));
                let t = c.g[V1][0] == 0u64;

                if t {
                    break 'b_400a6c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x165c));
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        break 'b_400a6c;
                    } else {
                        c.cycles += 5;
                        c.g[S0][0] = c.g[GP][0];
                        c.g[GP][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x64c4));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00400a68);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x165c));
                        call_ptr(c, t, 0x00400a68)?;
                        c.g[GP][0] = c.g[S0][0];
                        break 'b_400a6c;
                    }
                }
            }
            c.cycles += 3;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x1634), 0u64);
            let t = true;
            c.g[V0][0] = 0x450000;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x004009f8);
                }
                continue 'l_4009f8;
            } else {
                c.pc = 0x0040_0a78;
                ee::s15::sceCdInitEeCB(c);
                return Ok(());
            }
        }
    });
}

/// `void libcdvd_ReadRpcEndCallback(void *rpc)` (0x400b48): the read RPC's end
/// callback. Copies the packet's two byte runs (`[0]` bytes from `+0x10`, `[4]`
/// bytes from `+0x50`) to the destinations at `+8` and `+0xc` (cached and
/// uncached views), then tail-calls `libcdvd_CallbackCmdHandler` for the
/// pending command at `0x451658`.
pub fn libcdvd_read_rpc_end_callback(c: &mut Cpu) {
    guest(c, |c| {
        'b_400bb0: {
            'b_400ba8: {
                c.cycles += 5;
                c.g[A1][0] = 0x20000000;
                c.g[A3][0] = c.g[A0][0] | c.g[A1][0];
                c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A2][0] = 0u64;
                if t {
                    break 'b_400ba8;
                } else {
                    c.cycles += 3;
                    c.g[T1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[T2][0] = c.g[T1][0] | c.g[A1][0];
                    if t {
                        break 'b_400ba8;
                    } else {
                        c.cycles += 4;
                        c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
                        c.g[T3][0] = 0x450000;
                        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                        'l_400b78: loop {
                            c.cycles += 10;
                            c.g[A1][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                            c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
                            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            c.sb((c.g[A1][0] as u32), c.g[V1][0]);
                            c.sb((c.g[A0][0] as u32), c.g[V1][0]);
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00400b78);
                                }
                                continue 'l_400b78;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_400bb0;
                                } else {
                                    break 'b_400ba8;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[T3][0] = 0x450000;
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
            break 'b_400bb0;
        }
        'b_400bf8: {
            c.cycles += 2;
            let t = (c.g[V1][0] as i64) <= 0;
            c.g[V0][0] = 0x20000000;
            if t {
                break 'b_400bf8;
            } else {
                c.cycles += 4;
                c.g[T1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                c.g[A2][0] = 0u64;
                let t = (c.g[V1][0] as i64) <= 0;
                c.g[T2][0] = c.g[T1][0] | c.g[V0][0];
                if t {
                    break 'b_400bf8;
                } else {
                    c.cycles += 2;
                    c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    'l_400bd0: loop {
                        c.cycles += 10;
                        c.g[A1][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                        c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
                        c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        c.sb((c.g[A1][0] as u32), c.g[V1][0]);
                        c.sb((c.g[A0][0] as u32), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00400bd0);
                            }
                            continue 'l_400bd0;
                        } else {
                            break 'b_400bf8;
                        }
                    }
                }
            }
        }
        c.cycles += 2;
        c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1658));
        c.pc = 0x0040_0908;
        ee::s15::libcdvd_CallbackCmdHandler(c);
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{self, covered, machine, FAKE, LOG_UNWIND_AT};
    use crate::difftest::{check, check_paths, elf_image, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // -------------------------------------------------------------------
    // Leaf and simple functions.

    /// Hash compare: less, greater and equal (with the collision counter),
    /// over the full unsigned range.
    #[test]
    fn ea_file_cd_compare_hash_matches() {
        check_paths(
            ee::s14::f_003e3968__generated,
            ea_file_cd_compare_hash,
            200,
            &[7, 9, 12],
            &|c, seed| {
                let mut r = Rng(0x003e_3968 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x003e_3968;
                let x = if seed % 4 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    r.next_u32()
                };
                let y = match seed % 3 {
                    0 => x,
                    1 => x.wrapping_add(1 + r.next_u32() % 4),
                    _ => x.wrapping_sub(1 + r.next_u32() % 4),
                };
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x110) as u64;
                c.bus.write32(DATA + 0x100, x);
                c.bus.write32(DATA + 0x110, y);
                c.bus.write32(EA_FILE_CD_HASH_EQUAL_COUNT, r.next_u32());
            },
        );
    }

    /// `sceSifGetSreg`: random register numbers over a random table.
    #[test]
    fn sce_sif_get_sreg_matches() {
        check_paths(
            ee::s15::f_00425cf0__generated,
            sce_sif_get_sreg,
            100,
            &[6],
            &|c, seed| {
                let mut r = Rng(0x0042_5cf0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x0042_5cf0;
                let n = r.next_u32() % 0x40;
                c.g[A0][0] = n as u64;
                for i in 0..0x40 {
                    c.bus.write32(SIF_SREG_TABLE + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `sceSifCmd_SetSregHandler`: a command (register index, value) against
    /// a manager with a table pointer.
    #[test]
    fn sce_sif_cmd_set_sreg_handler_matches() {
        check_paths(
            ee::s15::f_00425cc0__generated,
            sce_sif_cmd_set_sreg_handler,
            100,
            &[7],
            &|c, seed| {
                let mut r = Rng(0x0042_5cc0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x0042_5cc0;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.bus.write32(DATA + 0x110, r.next_u32() % 16);
                c.bus.write32(DATA + 0x114, r.next_u32());
                c.bus.write32(DATA + 0x21c, DATA + 0x300);
                for i in 0..16 {
                    c.bus.write32(DATA + 0x300 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `sceSifSetCmdBuffer`: random buffer and size over random state.
    #[test]
    fn sce_sif_set_cmd_buffer_matches() {
        check_paths(
            ee::s15::f_00425ff0__generated,
            sce_sif_set_cmd_buffer,
            100,
            &[6],
            &|c, seed| {
                let mut r = Rng(0x0042_5ff0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x0042_5ff0;
                c.g[A0][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(SIF_CMD_STATE + 0x14, r.next_u32());
                c.bus.write32(SIF_CMD_STATE + 0x18, r.next_u32());
            },
        );
    }

    /// `EAMutex_Init`: the semaphore call is a fake (it unwinds on some
    /// seeds).
    #[test]
    fn ea_mutex_init_matches() {
        covered(
            ee::s14::f_003e5698__generated,
            ea_mutex_init,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003e_5698, 1);
                c.pc = 0x003e_5698;
                c.g[A0][0] = (DATA + 0x100 + 0x10 * (seed % 8)) as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `EAMutex_Destroy`: the delete call is a fake (it unwinds on some
    /// seeds).
    #[test]
    fn ea_mutex_destroy_matches() {
        covered(
            ee::s14::f_003e56e0__generated,
            ea_mutex_destroy,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003e_56e0, 1);
                c.pc = 0x003e_56e0;
                c.g[A0][0] = (DATA + 0x100 + 0x10 * (seed % 8)) as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
                }
            },
        );
    }

    // -------------------------------------------------------------------
    // Gated slot fakes for the library calls of the CD functions.

    /// Gate word (in the machine's RAM): the fakes below only act when it
    /// holds [`OC697_MAGIC`]; with the gate off they run the body they
    /// replaced.
    const OC697_CTRL: u32 = DATA + 0x9_0000;
    const OC697_MAGIC: u32 = 0x6970_0697;
    /// Per-callee result scripts: `[count, next, v0...]` (0x40 bytes each).
    const OC697_SCRIPT: u32 = DATA + 0x9_0100;
    /// While nonzero, the fake `sceCdRead` copies a sector into its buffer:
    /// sector 0 on the first call, sector 1 on every later one.
    const OC697_SECT_ON: u32 = DATA + 0x9_0f00;
    const OC697_SECT_NEXT: u32 = DATA + 0x9_0f04;
    /// The two 0x800-byte sectors.
    const OC697_SECTORS: u32 = DATA + 0x2_0000;
    /// While nonzero, `SetAlarm` fails (-1).
    const OC697_FAIL_ALARM: u32 = DATA + 0x9_0f08;

    /// Script slot of a faked callee.
    fn oc697_slot(addr: u32) -> u32 {
        match addr {
            0x0040_1718 => 0,
            0x0040_2450 => 1,
            0x0040_1bf0 => 2,
            0x0040_1df8 => 3,
            0x0040_13e8 => 4,
            0x0040_2300 => 5,
            0x0042_3de0 => 6,
            0x003e_35b0 => 7,
            _ => 8,
        }
    }

    /// Give callee `addr` the results `vals` (cycled) instead of the fake's
    /// hash.
    fn oc697_script(c: &mut Cpu, addr: u32, vals: &[u32]) {
        let base = OC697_SCRIPT + oc697_slot(addr) * 0x40;
        c.bus.write32(base, vals.len() as u32);
        c.bus.write32(base + 4, 0);
        for (i, v) in vals.iter().enumerate() {
            c.bus.write32(base + 8 + 4 * i as u32, *v);
        }
    }

    /// The logging fake (every register to the call log, maybe an unwind),
    /// then the scripted result and the sector copy.
    fn oc697_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let dest = c.g[A2][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let base = OC697_SCRIPT + oc697_slot(addr) * 0x40;
        let n = c.bus.read32(base);
        if n != 0 {
            let i = c.bus.read32(base + 4);
            c.bus.write32(base + 4, i.wrapping_add(1));
            let v = c.bus.read32(base + 8 + 4 * (i % n));
            c.g[V0][0] = sx(v);
        }
        if addr == 0x0040_1df8 && c.bus.read32(OC697_SECT_ON) != 0 {
            let i = c.bus.read32(OC697_SECT_NEXT);
            c.bus.write32(OC697_SECT_NEXT, i.wrapping_add(1));
            let src = OC697_SECTORS + if i == 0 { 0 } else { 0x800 };
            for w in 0..0x200 {
                let v = c.bus.read32(src + 4 * w);
                c.bus.write32(dest + 4 * w, v);
            }
        }
    }

    macro_rules! oc697_fakes {
        ($($fake:ident, $prev:ident, $addr:literal, $slot:path, $gen:path;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC697_CTRL) != OC697_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc697_fake_call(c, $addr)
                }
            )*
            fn oc697_hook_all() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc697_fakes! {
        oc697_fake_cd_init, OC697_PREV_CD_INIT, 0x0040_1718,
            ee::s15::OVERRIDE_00401718, ee::s15::f_00401718__generated;
        oc697_fake_cd_mmode, OC697_PREV_CD_MMODE, 0x0040_2450,
            ee::s15::OVERRIDE_00402450, ee::s15::f_00402450__generated;
        oc697_fake_cd_disk_ready, OC697_PREV_CD_DISK_READY, 0x0040_1bf0,
            ee::s15::OVERRIDE_00401BF0, ee::s15::f_00401bf0__generated;
        oc697_fake_cd_read, OC697_PREV_CD_READ, 0x0040_1df8,
            ee::s15::OVERRIDE_00401DF8, ee::s15::f_00401df8__generated;
        oc697_fake_cd_sync, OC697_PREV_CD_SYNC, 0x0040_13e8,
            ee::s15::OVERRIDE_004013E8, ee::s15::f_004013e8__generated;
        oc697_fake_cd_get_error, OC697_PREV_CD_GET_ERROR, 0x0040_2300,
            ee::s15::OVERRIDE_00402300, ee::s15::f_00402300__generated;
        oc697_fake_wait_sema, OC697_PREV_WAIT_SEMA, 0x0042_3de0,
            ee::s15::OVERRIDE_00423DE0, ee::s15::f_00423de0__generated;
        oc697_fake_delay_ticks, OC697_PREV_DELAY_TICKS, 0x003e_35b0,
            ee::s14::OVERRIDE_003E35B0, ee::s14::f_003e35b0__generated;
        oc697_fake_complete_request, OC697_PREV_COMPLETE_REQUEST, 0x003d_e420,
            ee::s14::OVERRIDE_003DE420, ee::s14::f_003de420__generated;
    }

    /// The gate open, empty scripts and no sector copy; the fakes unwind at
    /// call `unwind` (`u32::MAX`: never).
    fn oc697_open(c: &mut Cpu, unwind: u32) {
        oc697_hook_all();
        c.bus.write32(OC697_CTRL, OC697_MAGIC);
        c.bus.write32(LOG_UNWIND_AT, unwind);
        for slot in 0..9 {
            c.bus.write32(OC697_SCRIPT + slot * 0x40, 0);
        }
        c.bus.write32(OC697_SECT_ON, 0);
        c.bus.write32(OC697_SECT_NEXT, 0);
        c.bus.write32(OC697_FAIL_ALARM, 0);
    }

    /// A test machine (fake syscalls and unknown callees) with the CD
    /// fakes open.
    fn oc697_machine(c: &mut Cpu, seed: u32, salt: u64, unwind: u32) -> Rng {
        let r = machine(c, seed, salt, 0);
        oc697_open(c, unwind);
        r
    }

    /// The syscall hook with `SetAlarm` failing when the gate is set.
    fn oc697_syscall(c: &mut Cpu, at: u32) {
        fakes::fake_syscall(c, at);
        if at == 0x0042_3b24 && c.bus.read32(OC697_FAIL_ALARM) == OC697_MAGIC {
            c.g[V0][0] = u64::MAX;
        }
    }

    // -------------------------------------------------------------------
    // EAFile_CdReset

    /// Scripted drive for `EAFile_CdReset`: `Read` fails `reads` times first,
    /// `Sync(1)` reports busy once, `GetError` reports `errors` errors before
    /// success.
    fn oc697_reset_scripts(c: &mut Cpu, reads: u32, errors: u32) {
        let mut read = vec![0; reads as usize];
        read.push(1);
        oc697_script(c, 0x0040_1df8, &read);
        oc697_script(c, 0x0040_13e8, &[1, 0]);
        let mut err = vec![1; errors as usize];
        err.push(0);
        oc697_script(c, 0x0040_2300, &err);
    }

    /// `EAFile_CdReset`: read retries, busy syncs, drive errors (up to
    /// the ten-error budget that re-initialises the drive), and fakes that
    /// unwind in the middle.
    #[test]
    fn ea_file_cd_reset_matches() {
        covered(
            ee::s14::f_003e3618__generated,
            ea_file_cd_reset,
            200,
            &|c, seed| {
                let mut r = Rng(0x003e_3618 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let unwind = if seed % 4 == 0 {
                    r.next_u32() % 30
                } else {
                    u32::MAX
                };
                let mut g = oc697_machine(c, seed, 0x003e_3618, unwind);
                c.pc = 0x003e_3618;
                oc697_reset_scripts(c, seed % 3, (seed / 3) % 13);
                c.bus.write32(0x0045_0c60, g.next_u32());
            },
        );
    }

    driven!(oc697_reset_gen, ee::s14::f_003e3618__generated);
    driven!(oc697_reset_hand, ea_file_cd_reset);

    /// `EAFile_CdReset` on a running thread with events due in the poll
    /// loops (read retry, busy sync, error retry).
    #[test]
    fn ea_file_cd_reset_events() {
        check_events(oc697_reset_gen, oc697_reset_hand, 96, &|c, seed| {
            let mut r = Rng(0x003e_3618 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            oc697_open(c, u32::MAX);
            oc697_reset_scripts(c, 1 + seed / 3 % 2, 2 + seed / 6 % 11);
            c.bus.write32(0x0045_0c60, r.next_u32());
            schedule(c, &mut r, seed, 0x003e_3618)
        });
    }

    // -------------------------------------------------------------------
    // EAFile_CdScanDir

    /// Write an ISO9660-style directory record at `at` and return its
    /// length: `len`, LBA at +2 and size at +10 (little endian, unaligned),
    /// flags at +0x19, name length at +0x20 and the name at +0x21.
    fn oc697_dir_record(c: &mut Cpu, at: u32, lba: u32, size: u32, flags: u8, name: &[u8]) -> u32 {
        let len = (0x21 + name.len() as u32 + 1) & !1;
        c.bus.write8(at, len as u8);
        for i in 0..4 {
            c.bus.write8(at + 2 + i, (lba >> (8 * i)) as u8);
            c.bus.write8(at + 10 + i, (size >> (8 * i)) as u8);
        }
        c.bus.write8(at + 0x19, flags);
        c.bus.write8(at + 0x20, name.len() as u8);
        for (i, b) in name.iter().enumerate() {
            c.bus.write8(at + 0x21 + i as u32, *b);
        }
        len
    }

    /// The two sectors `sceCdRead` hands out: sector 0 lists "." then
    /// `entries` files and (when `dirs`) directories, sector 1 only files.
    /// Names end in ".;1", ";1" or a bare dot, to reach each branch of the
    /// name clean-up.
    fn oc697_sectors(c: &mut Cpu, r: &mut Rng, entries: u32, dirs: bool) {
        for s in 0..2u32 {
            let base = OC697_SECTORS + s * 0x800;
            let mut at = base;
            at += oc697_dir_record(c, at, 0x100, 0x800, 2, b".");
            let n = if s == 0 {
                entries
            } else {
                1 + r.next_u32() % 3
            };
            for j in 0..n {
                let dir = s == 0 && dirs && (j + r.next_u32()) % 2 == 0;
                let name: Vec<u8> = match r.next_u32() % 3 {
                    0 => b"AB.;1".to_vec(),
                    1 => b"CDE;1".to_vec(),
                    _ => b"FG.H;1".to_vec(),
                };
                let size = 0x7f0 + (r.next_u32() % 3) * 0x400;
                at += oc697_dir_record(
                    c,
                    at,
                    0x200 + 0x10 * j,
                    size,
                    if dir { 2 } else { 0 },
                    if dir { &b"SUBDIR"[..] } else { &name[..] },
                );
            }
            c.bus.write8(at, 0);
        }
    }

    /// The `ScanDir` fixture: the file table and its count, an empty path,
    /// the sector script and a quiet drive.
    fn oc697_scan_setup(c: &mut Cpu, seed: u32, r: &mut Rng) {
        oc697_reset_scripts(c, 0, 0);
        oc697_script(c, 0x0040_1df8, &[1]);
        oc697_script(c, 0x0040_13e8, &[0]);
        c.bus.write32(OC697_SECT_ON, 1);
        c.bus.write32(0x0045_0c68, DATA + 0x6000);
        c.bus.write32(0x0045_0c6c, seed % 3);
        c.bus.write32(0x0051_9c6c, r.next_u32());
        let entries = 1 + seed % 5;
        oc697_sectors(c, r, entries, seed / 5 % 3 != 0);
        c.g[A0][0] = (0x100 + seed % 7) as u64;
        c.g[A1][0] = (1 + seed / 15 % 3) as u64;
    }

    /// `EAFile_CdScanDir`: a root directory with files and subdirectories
    /// (the recursion), one to three sectors, the sector buffer and path
    /// saved around the recursion.
    #[test]
    fn ea_file_cd_scan_dir_matches() {
        covered(
            ee::s14::f_003e3758__generated,
            ea_file_cd_scan_dir,
            120,
            &|c, seed| {
                let mut r = Rng(0x003e_3758 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let unwind = if seed % 5 == 0 {
                    r.next_u32() % 60
                } else {
                    u32::MAX
                };
                oc697_machine(c, seed, 0x003e_3758, unwind);
                c.pc = 0x003e_3758;
                oc697_scan_setup(c, seed, &mut r);
            },
        );
    }

    driven!(oc697_scan_gen, ee::s14::f_003e3758__generated);
    driven!(oc697_scan_hand, ea_file_cd_scan_dir);

    /// `EAFile_CdScanDir` on a running thread with events due at its back
    /// edges (the record walk, the next sector).
    #[test]
    fn ea_file_cd_scan_dir_events() {
        check_events(oc697_scan_gen, oc697_scan_hand, 60, &|c, seed| {
            let mut r = Rng(0x003e_3758 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            oc697_open(c, u32::MAX);
            oc697_scan_setup(c, seed, &mut r);
            schedule(c, &mut r, seed, 0x003e_3758)
        });
    }

    // -------------------------------------------------------------------
    // EAFile_CdReadThread

    /// The CD request state: flags `0x519c40`, semaphore `+0xc`, retries
    /// `+0x10`, `+0x14`, the buffer base `+0x3c`; the current read at
    /// `0x519c60` (length, remaining, offset, sector, sector count, dest).
    const OC697_CD_STATE: u32 = 0x0051_9c40;
    const OC697_CD_READ: u32 = 0x0051_9c60;

    /// `EAFile_CdReadThread`: the thread loops forever, so each seed picks
    /// the call at which a fake unwinds. Random flags, retry counts,
    /// remaining lengths (none, a short tail, unaligned and aligned
    /// runs), drive errors, and failing re-reads.
    #[test]
    fn ea_file_cd_read_thread_matches() {
        covered(
            ee::s14::f_003e3be0__generated,
            ea_file_cd_read_thread,
            300,
            &|c, seed| {
                let mut r = Rng(0x003e_3be0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                oc697_machine(c, seed, 0x003e_3be0, 1 + seed % 14);
                c.pc = 0x003e_3be0;
                let flags = r.next_u32() & 0xf;
                c.bus.write32(OC697_CD_STATE, flags);
                c.bus.write32(OC697_CD_STATE + 0xc, r.next_u32());
                c.bus.write32(OC697_CD_STATE + 0x10, r.next_u32() % 3);
                c.bus.write32(OC697_CD_STATE + 0x14, r.next_u32());
                c.bus.write32(OC697_CD_STATE + 0x3c, DATA + 0x6_0000);
                let len = [0, 1, 0x10, 0x7ff, 0x800][(r.next_u32() % 5) as usize];
                let remaining =
                    [0, 0x40, 0x7ff, 0x800, 0x1000, 0x1801][(r.next_u32() % 6) as usize];
                c.bus.write32(OC697_CD_READ, len);
                c.bus.write32(OC697_CD_READ + 4, remaining);
                c.bus.write32(OC697_CD_READ + 8, r.next_u32() % 0x40);
                c.bus.write32(OC697_CD_READ + 0xc, r.next_u32() % 0x1000);
                c.bus.write32(OC697_CD_READ + 0x10, r.next_u32() % 4);
                c.bus.write32(
                    OC697_CD_READ + 0x14,
                    DATA + 0x4_0000 + 0x40 * (r.next_u32() % 4),
                );
                let errors: Vec<u32> = (0..8).map(|_| (r.next_u32() % 3 == 0) as u32).collect();
                oc697_script(c, 0x0040_2300, &errors);
                let reads: Vec<u32> = (0..4).map(|_| (r.next_u32() % 2) as u32).collect();
                oc697_script(c, 0x0040_1df8, &reads);
                for w in 0..16 {
                    c.bus.write32(DATA + 0x6_0000 + 4 * w, r.next_u32());
                }
            },
        );
    }

    // -------------------------------------------------------------------
    // THREAD_yieldticks

    /// `THREAD_yieldticks`: positive, zero and negative tick counts; tick
    /// rates including zero (the divide by zero); alarm failing (the debug
    /// message) or not; in interrupt context or not; fakes unwinding.
    #[test]
    fn thread_yieldticks_matches() {
        covered(
            ee::s14::f_003e52b0__generated,
            thread_yieldticks,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003e_52b0, 6);
                c.hooks.syscall = oc697_syscall;
                elf_image(c);
                c.pc = 0x003e_52b0;
                c.g[A0][0] = match seed % 5 {
                    0 => 0,
                    1 => sx(0xffff_ff00 | (r.next_u32() & 0xff)),
                    2 => 1 + (r.next_u32() % 100) as u64,
                    3 => 0x7fff,
                    _ => r.next_u32() as u64,
                };
                c.bus
                    .write32(0x0045_0dc8, [60, 1000, 0, 7][(seed / 5 % 4) as usize]);
                c.bus.write32(0x0045_0da0, r.next_u32() % 2000);
                c.bus.write32(0x0045_0de4, (seed / 20) % 2);
                if seed / 40 % 2 == 0 {
                    c.bus.write32(OC697_FAIL_ALARM, OC697_MAGIC);
                    // The debug message prints through [0x450dd8] (a fake) an
                    // empty string.
                    c.bus.write32(0x0045_0dd8, FAKE + 0x20);
                    c.bus.write8(0x0049_5e20, 0);
                }
            },
        );
    }

    // -------------------------------------------------------------------
    // libcdvd_CallbackThread

    /// `libcdvd_CallbackThread`: the thread loops forever, so each seed picks
    /// the call at which a fake unwinds. The request word is -1 (exit), 0 or
    /// a command; debug printing on or off; a callback registered or not.
    #[test]
    fn libcdvd_callback_thread_matches() {
        covered(
            ee::s15::f_004009a8__generated,
            libcdvd_callback_thread,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0040_09a8, 0);
                oc697_open(c, 1 + seed % 9);
                elf_image(c);
                c.pc = 0x0040_09a8;
                c.bus.write32(0x0045_1620, r.next_u32());
                c.bus
                    .write32(0x0045_1658, [0xffff_ffff, 0, 5][(seed % 3) as usize]);
                c.bus.write32(0x0045_165c, r.next_u32());
                c.bus.write32(0x0045_1610, (seed / 3) % 2);
                c.bus
                    .write32(0x0052_64c0, if seed / 6 % 2 == 0 { 0 } else { FAKE + 0x40 });
                c.bus.write32(0x0052_64c4, r.next_u32());
                c.bus.write32(0x0045_1634, r.next_u32());
                c.bus.write32(0x0045_1614, r.next_u32());
                c.bus.write32(0x0052_64d4, r.next_u32());
                // `scePrintf` gets an empty format.
                c.bus.write8(0x0049_79e0, 0);
            },
        );
    }

    // -------------------------------------------------------------------
    // libcdvd_ReadRpcEndCallback

    /// The read-RPC packet and the two destinations.
    const OC697_RPC: u32 = DATA + 0x100;
    const OC697_RPC_DST1: u32 = DATA + 0x400;
    const OC697_RPC_DST2: u32 = DATA + 0x600;

    /// Packet, destinations and the pending-command words the tail call
    /// reads.
    fn oc697_rpc_setup(c: &mut Cpu, seed: u32, r: &mut Rng) {
        c.g[A0][0] = OC697_RPC as u64;
        let n1 = match seed % 4 {
            0 => 0,
            1 => 0xffff_ffff,
            _ => 1 + r.next_u32() % 0x40,
        };
        let n2 = match seed / 4 % 4 {
            0 => 0,
            1 => 0xffff_ffff,
            _ => 1 + r.next_u32() % 0x40,
        };
        c.bus.write32(OC697_RPC, n1);
        c.bus.write32(OC697_RPC + 4, n2);
        c.bus.write32(OC697_RPC + 8, OC697_RPC_DST1);
        c.bus.write32(OC697_RPC + 0xc, OC697_RPC_DST2);
        for w in 0..0x20 {
            c.bus.write32(OC697_RPC + 0x10 + 4 * w, r.next_u32());
        }
        c.bus.write32(
            0x0045_1658,
            if seed % 3 == 0 {
                0xb
            } else {
                r.next_u32() % 13
            },
        );
        c.bus.write32(0x0045_1628, r.next_u32() & 0xffff);
        c.bus.write32(0x0045_1620, r.next_u32() & 0xffff);
        c.bus
            .write32(0x0045_1614, if seed % 5 == 1 { 0 } else { 0x0052_0000 });
        c.bus
            .write32(0x0052_64c0, if seed % 7 == 2 { 0 } else { seed });
    }

    /// `libcdvd_ReadRpcEndCallback`: empty, negative and short byte runs,
    /// and the pending command handler the call ends in (real, over fake
    /// syscalls).
    #[test]
    fn libcdvd_read_rpc_end_callback_matches() {
        check(
            ee::s15::f_00400b48__generated,
            libcdvd_read_rpc_end_callback,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0040_0b48, 0);
                elf_image(c);
                c.pc = 0x0040_0b48;
                oc697_rpc_setup(c, seed, &mut r);
            },
        );
    }

    driven!(oc697_rpc_gen, ee::s15::f_00400b48__generated);
    driven!(oc697_rpc_hand, libcdvd_read_rpc_end_callback);

    /// `libcdvd_ReadRpcEndCallback` on a running thread with events due in
    /// the byte-copy loops.
    #[test]
    fn libcdvd_read_rpc_end_callback_events() {
        check_events(oc697_rpc_gen, oc697_rpc_hand, 96, &|c, seed| {
            let mut r = Rng(0x0040_0b48 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            elf_image(c);
            // Copies of at least two bytes in both runs reach the back edges.
            oc697_rpc_setup(c, 2 + seed % 2 + 4 * (2 + seed / 3 % 2), &mut r);
            schedule(c, &mut r, seed, 0x0040_0b48)
        });
    }
}
