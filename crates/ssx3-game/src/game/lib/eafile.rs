//! `src/lib/eafile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
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
    (
        0x003d_d310,
        ea_file_release_request,
        "EAFile_ReleaseRequest",
    ),
    (0x003e_2490, eafile_str_icmp, "EAFile_StrICmp"),
    (0x003d_d648, ea_file_queue_read, "EAFile_QueueRead"),
    (0x003d_e670, ea_file_alloc_request, "EAFile_AllocRequest"),
    (0x003d_d4e8, ea_file_queue_open, "EAFile_QueueOpen"),
    (0x003d_d5a0, ea_file_queue_close, "EAFile_QueueClose"),
    (
        0x003d_e800,
        ea_file_alloc_file_handle,
        "EAFile_AllocFileHandle",
    ),
    (
        0x003d_e8c0,
        ea_file_free_file_handle,
        "EAFile_FreeFileHandle",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::ai::ai::callee::EA_FILE_FREE_FILE_HANDLE;
    pub(crate) use crate::game::ai::ai::callee::EA_FILE_FREE_REQUEST;
}

// ---------------------------------------------------------------------------
// EAFile_ReleaseRequest: a release path chosen from flags in the request and
// the request-state table at `.rodata 0x495d40`.

/// `int EAFile_ReleaseRequest(EAFileRequest* req)` (0x3dd310): clear the
/// request and, depending on its state and channel type, free the associated
/// file handle first. The request is found through a 256-entry hash of its
/// address's top byte (table pointer at 0x51ad98, 0x30-byte entries).
pub fn ea_file_release_request(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x003d_d310 => {
                    c.cycles += 15;
                    let frame = Frame::open(c, 0x40);
                    c.g[A1][0] = 0x30;
                    let req = c.g[A0][0];
                    let sp0 = frame.at(c, 0);
                    Addr(sp0).sw(c, req);
                    c.g[A0][0] = 0x52_0000;
                    frame.save(c, S0, 0x30);
                    c.g[V0][0] = Addr(frame.at(c, 0x3)).lbu(c);
                    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_sub(0x6528)).lw(c);
                    c.mult(0, V0, V0, A1);
                    frame.save(c, S1, 0x20);
                    frame.save_ra(c, 0x10);
                    c.g[S1][0] = 0;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
                    pc = if c.g[V1][0] == 0 {
                        0x003d_d414
                    } else {
                        0x003d_d34c
                    };
                }
                0x003d_d34c => {
                    c.cycles += 2;
                    let positive = (c.g[V1][0] as i64) > 0;
                    c.g[V0][0] = 1;
                    pc = if positive { 0x003d_d368 } else { 0x003d_d354 };
                }
                0x003d_d354 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < -2i64) as u64;
                    pc = if c.g[V0][0] != 0 {
                        0x003d_d414
                    } else {
                        0x003d_d360
                    };
                }
                0x003d_d360 => {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    pc = 0x003d_d3f0;
                }
                0x003d_d368 => {
                    c.cycles += 2;
                    pc = if c.g[V1][0] != c.g[V0][0] {
                        0x003d_d414
                    } else {
                        0x003d_d370
                    };
                }
                0x003d_d370 => {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
                    c.g[V1][0] = (c.g[A0][0] < 0x9u64) as u64;
                    let out = c.g[V1][0] == 0;
                    c.g[V0][0] = 0x49_0000;
                    pc = if out { 0x003d_d414 } else { 0x003d_d38c };
                }
                0x003d_d38c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d40));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match target {
                        0x003d_d3a4 => 0x003d_d3a4,
                        0x003d_d3ac => 0x003d_d3ac,
                        0x003d_d3b8 => 0x003d_d3b8,
                        0x003d_d3c0 => 0x003d_d3c0,
                        0x003d_d3c8 => 0x003d_d3c8,
                        0x003d_d3e8 => 0x003d_d3e8,
                        0x003d_d3f0 => 0x003d_d3f0,
                        _ => {
                            c.jump_out(target, 0x003d_d310, 0x003d_d434);
                            return Ok(());
                        }
                    };
                }
                0x003d_d3a4 => {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x24).lw(c);
                    pc = 0x003d_d414;
                }
                0x003d_d3ac => {
                    c.cycles += 3;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
                    pc = 0x003d_d414;
                }
                0x003d_d3b8 => {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x18).lw(c);
                    pc = 0x003d_d414;
                }
                0x003d_d3c0 => {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
                    pc = 0x003d_d414;
                }
                0x003d_d3c8 => {
                    c.cycles += 8;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x24).lw(c);
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x18).lw(c);
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        let a = c.g[A0][0] as u32;
                        let v = c.g[V1][0];
                        Addr(a.wrapping_add(0x4)).sw(c, v);
                        pc = 0x003d_d414;
                    } else {
                        pc = 0x003d_d3e8;
                    }
                }
                0x003d_d3e8 => {
                    c.cycles += 2;
                    pc = 0x003d_d414;
                }
                0x003d_d3f0 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
                    c.g[A0][0] = c.g[V0][0] & 0xf;
                    let is2 = c.g[A0][0] == c.g[V1][0];
                    c.g[V0][0] = 0x9;
                    pc = if is2 { 0x003d_d40c } else { 0x003d_d404 };
                }
                0x003d_d404 => {
                    c.cycles += 2;
                    pc = if c.g[A0][0] != c.g[V0][0] {
                        0x003d_d414
                    } else {
                        0x003d_d40c
                    };
                }
                0x003d_d40c => {
                    c.cycles += 2;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x24).lw(c);
                    callee::EA_FILE_FREE_FILE_HANDLE.call(c, 0x003d_d414)?;
                    pc = 0x003d_d414;
                }
                0x003d_d414 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    callee::EA_FILE_FREE_REQUEST.call(c, 0x003d_d41c)?;
                    c.g[V0][0] = c.g[S1][0];
                    let frame = Frame { size: 0x40 };
                    frame.restore(c, 16, 0x30);
                    frame.restore(c, 17, 0x20);
                    frame.restore_ra(c, 0x10);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d_d310, 0x003d_d434);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// EAFile_StrICmp (0x3e2490, seg_2DCAE8.c)

/// The ctype table indexed with a signed byte (0x4a0000 - 0x637f); bit 0 is
/// "uppercase".
pub(crate) const EAF_CTYPE: u32 = 0x0049_9c81;

/// `int EAFile_StrICmp(const char* a, const char* b)` (0x3e2490): compares
/// two strings ignoring case (uppercase letters per the ctype table are
/// lowered by +0x20); returns the first non-zero difference, else 0.
pub fn eafile_str_icmp(c: &mut Cpu) {
    c.cycles += 7;
    c.g[T2][0] = c.g[A0][0];
    c.g[T1][0] = c.g[A1][0];
    c.g[T0][0] = Addr(c.g[T2][0] as u32).lb(c);
    c.g[V0][0] = 0x4a_0000;
    c.g[A3][0] = Addr(c.g[T1][0] as u32).lb(c);
    c.g[T3][0] = sx(EAF_CTYPE);
    loop {
        // 0x3e24bc: lowered characters and their difference.
        c.cycles += 13;
        let ca = c.g[T0][0] as u32;
        let cb = c.g[A3][0] as u32;
        let table = c.g[T3][0] as u32;
        c.g[A0][0] = sx(ca.wrapping_add(table));
        c.g[A2][0] = sx(ca.wrapping_add(0x20));
        c.g[A1][0] = sx(cb.wrapping_add(table));
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c) & 1;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lbu(c) & 1;
        c.g[A0][0] = sx(cb.wrapping_add(0x20));
        if c.g[V0][0] == 0 {
            c.g[A2][0] = c.g[T0][0];
        }
        if c.g[V1][0] == 0 {
            c.g[A0][0] = c.g[A3][0];
        }
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        if c.g[V0][0] != 0 {
            break;
        }
        // 0x3e24f0: equal so far; stop at the end of `a`.
        c.cycles += 2;
        let more = c.g[T0][0] != 0;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003e_24b0);
        }
        // 0x3e24b0: next pair.
        c.cycles += 3;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
        c.g[T0][0] = Addr(c.g[T2][0] as u32).lb(c);
        c.g[A3][0] = Addr(c.g[T1][0] as u32).lb(c);
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// EAFile_AllocRequest (0x3de670)
// ---------------------------------------------------------------------------

/// The EAFile request pool (`0x519ab0`): slot count at +0, its mutex at
/// +0x14, the slot array (0x30 bytes each) at +0x28.
#[cfg(test)]
pub(crate) const EAFILE_POOL: u32 = 0x0051_9ab0;

pub(crate) const EAFILE_POOL_SLOTS: u32 = 0x28;

pub(crate) const EAFILE_POOL_MUTEX: u32 = 0x0051_9ac4;

pub(crate) const EAFILE_REQ_SIZE: u32 = 0x30;

/// Slot word 0: bits 0-19 serial, 20-23 state (0 = free, 1 = allocated);
/// byte 3 = the slot's index.
pub(crate) const EAFILE_REQ_INDEX: u32 = 0x3;

/// The next request serial (wraps from 0xfffff to 1).
pub(crate) const EAFILE_REQ_SERIAL: u32 = 0x0045_0c0c;

/// Loop heads: the slot scan, and the spin when the pool is exhausted.
pub(crate) const EAFILE_ALLOC_SCAN_LOOP: u32 = 0x003d_e6d0;

pub(crate) const EAFILE_ALLOC_EXHAUSTED_LOOP: u32 = 0x003d_e768;

pub(crate) const EAFILE_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

pub(crate) const EAFILE_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

/// `EAFileRequest* EAFile_AllocRequest(void)` (0x3de670): under the pool
/// mutex, takes the first free request slot, marks it allocated, stores its
/// index and the next serial (1..0xfffff) in it; returns `&slots[index]`.
/// With every slot busy the original spins forever (pc 0x3de768); so does
/// this port.
pub fn ea_file_alloc_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0x0052_0000;
        c.g[S2][0] = sx(EAFILE_POOL_MUTEX);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[A0][0] = c.g[S2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        EAFILE_MUTEX_LOCK.call(c, 0x003d_e69c)?;
        let count = Addr(r32(c, S2).wrapping_sub(0x14)).lw(c);
        c.g[V0][0] = count;
        c.g[S3][0] = 0x0052_0000;
        if (count as i64) > 0 {
            c.cycles += 10;
            c.g[T0][0] = 0xffff_ffff_ff0f_ffff; // clears the state bits
            c.g[A2][0] = 0xf_ffff; // serial mask
            c.g[A0][0] = sx(r32(c, S2).wrapping_sub(0x14));
            c.g[T1][0] = c.g[S3][0];
            c.g[T2][0] = 0x10_0000; // state 1
            c.g[T3][0] = 0xffff_ffff_fff0_0000; // clears the serial
            c.g[T4][0] = 1;
            c.g[A3][0] = 0;
            loop {
                c.cycles += 7;
                let pool = r32(c, A0);
                let slots = Addr(pool.wrapping_add(EAFILE_POOL_SLOTS)).lw(c);
                let slot = sx(r32(c, A3).wrapping_add(slots as u32));
                c.g[A1][0] = slot;
                let word = Addr(slot as u32).lw(c);
                c.g[V1][0] = word;
                let state = sx((word as u32) >> 20) & 0xf;
                c.g[V0][0] = Addr(r32(c, T1).wrapping_sub(0x6550)).lw(c);
                if state == 0 {
                    // Free: claim it.
                    c.cycles += 19;
                    let claimed = (word & c.g[T0][0]) | c.g[T2][0];
                    c.g[V1][0] = claimed;
                    c.g[T0][0] = 0x0045_0000;
                    Addr(slot as u32).sw(c, claimed);
                    let slots = Addr(pool.wrapping_add(EAFILE_POOL_SLOTS)).lw(c);
                    let at = sx(r32(c, A3).wrapping_add(slots as u32));
                    let index = c.g[S1][0];
                    Addr((at as u32).wrapping_add(EAFILE_REQ_INDEX)).sb(c, index);
                    let slots = Addr(pool.wrapping_add(EAFILE_POOL_SLOTS)).lw(c);
                    let serial = Addr(EAFILE_REQ_SERIAL).lw(c);
                    let req = sx(r32(c, A3).wrapping_add(slots as u32));
                    c.g[A0][0] = req;
                    let head = Addr(req as u32).lw(c);
                    let serial_bits = serial & c.g[A2][0];
                    let next = sx((serial as u32).wrapping_add(1));
                    let head = (head & c.g[T3][0]) | serial_bits;
                    let wrap = (c.g[A2][0] as i64) < (next as i64);
                    c.g[A1][0] = serial_bits;
                    c.g[V1][0] = next;
                    c.g[A2][0] = wrap as u64;
                    c.g[V0][0] = head;
                    Addr(req as u32).sw(c, head);
                    Addr(EAFILE_REQ_SERIAL).sw(c, next);
                    if wrap {
                        c.cycles += 2;
                        let one = c.g[T4][0];
                        Addr(EAFILE_REQ_SERIAL).sw(c, one);
                    }
                    break;
                }
                // Busy: next slot.
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(EAFILE_REQ_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, EAFILE_ALLOC_SCAN_LOOP);
                }
            }
        }
        c.cycles += 6;
        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x653c));
        c.g[A0][0] = c.g[S0][0];
        EAFILE_MUTEX_UNLOCK.call(c, 0x003d_e75c)?;
        let count = Addr(r32(c, S0).wrapping_sub(0x14)).lw(c);
        c.g[V0][0] = EAFILE_REQ_SIZE as u64;
        if c.g[S1][0] == count {
            // No free slot: the original spins here forever.
            loop {
                c.cycles += 7;
                if event_due(c) {
                    deliver_event_here(c, EAFILE_ALLOC_EXHAUSTED_LOOP);
                }
            }
        }
        c.cycles += 11;
        c.g[A0][0] = sx(r32(c, S3).wrapping_sub(0x6550));
        let (index, stride) = (r32(c, S1), r32(c, V0));
        let offset = abi::mult(c, index, stride);
        c.g[V0][0] = sx(offset);
        let slots = Addr(r32(c, A0).wrapping_add(EAFILE_POOL_SLOTS)).lw(c);
        c.g[V1][0] = slots;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        c.g[V0][0] = sx((slots as u32).wrapping_add(offset));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAFile_QueueRead.

/// `EAFileRequest` fields.
pub(crate) const EAFILE_QR_STATUS: u32 = 0xc;

pub(crate) const EAFILE_QR_CALLBACK_ARG: u32 = 0x10;

pub(crate) const EAFILE_QR_CALLBACK: u32 = 0x14;

pub(crate) const EAFILE_QR_OFFSET: u32 = 0x18;

pub(crate) const EAFILE_QR_SIZE: u32 = 0x1c;

pub(crate) const EAFILE_QR_BUFFER: u32 = 0x20;

pub(crate) const EAFILE_QR_FILE: u32 = 0x24;

/// `EAFile::size`.
pub(crate) const EAFILE_QR_FILE_SIZE: u32 = 0x4;

/// `int EAFile_QueueRead(EAFile* f, int offset, void* buf, int size, int
/// cbArg, void* cb)` (0x3dd648): allocate a request (`EAFile_AllocRequest`),
/// make it a read (command 4 in bits 20-23 of word 0), store the callback;
/// a null file marks the request failed (status 6, `EAFile_QueueSignalStub`).
/// Clamp the size to the end of the file, fill the request and hand it to
/// `EAFile_ServiceQueue`; returns the request's word 0 (its handle).
pub fn ea_file_queue_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[T1][0]; // callback
        frame.save(c, S3, 0x40);
        c.g[S1][0] = c.g[T0][0]; // callback arg
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A3][0]; // size
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A0][0]; // file
        frame.save(c, S6, 0x10);
        c.g[S5][0] = c.g[A1][0]; // offset
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0);
        c.g[S6][0] = c.g[A2][0]; // buffer
        Callee {
            addr: 0x003d_e670,
            f: ee::s14::EAFile_AllocRequest,
        }
        .call(c, 0x003d_d688)?;
        c.g[S2][0] = c.g[V0][0];
        let req = c.g[S2][0] as u32;
        c.g[V1][0] = 0xffff_ffff_ff0f_ffff;
        c.g[V0][0] = Addr(req).lw(c);
        c.g[A0][0] = 0x0040_0000;
        c.sw(req.wrapping_add(EAFILE_QR_CALLBACK), c.g[S0][0]);
        c.g[V0][0] &= c.g[V1][0];
        c.sw(req.wrapping_add(EAFILE_QR_CALLBACK_ARG), c.g[S1][0]);
        c.g[V0][0] |= c.g[A0][0]; // command: read
        c.sw(req, c.g[V0][0]);
        if c.g[S4][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = 6;
            c.g[A0][0] = c.g[S2][0];
            c.sw(req.wrapping_add(EAFILE_QR_STATUS), c.g[V0][0]);
            Callee {
                addr: 0x003d_e668,
                f: ee::s14::EAFile_QueueSignalStub,
            }
            .call(c, 0x003d_d6c4)?;
        }

        c.cycles += 22;
        let req = c.g[S2][0] as u32;
        c.sw(req.wrapping_add(EAFILE_QR_FILE), c.g[S4][0]);
        c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S3][0] as u32)); // end
        c.g[A0][0] = c.g[S2][0];
        c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(EAFILE_QR_FILE_SIZE)).lw(c);
        c.sw(req.wrapping_add(EAFILE_QR_BUFFER), c.g[S6][0]);
        c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.sw(req.wrapping_add(EAFILE_QR_OFFSET), c.g[S5][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S5][0] as u32));
        if c.g[V1][0] != 0 {
            c.g[S3][0] = c.g[V0][0]; // clamp to the file end
        }
        c.sw(req.wrapping_add(EAFILE_QR_SIZE), c.g[S3][0]);
        Callee {
            addr: 0x003d_dfa8,
            f: ee::s14::EAFile_ServiceQueue,
        }
        .call(c, 0x003d_d6f0)?;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
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
    })
}

// ---------------------------------------------------------------------------
// 0x003de800 EAFile_AllocFileHandle

/// The file-handle manager (`D_00519AB0`): `+4` the handle count, `+0x14`
/// its mutex (0x519ac4), `+0x2c` the handle array. Handles are 0x110 bytes;
/// their first word is the in-use flag.
#[cfg(test)]
pub(crate) const OC325_FILE_MANAGER: u32 = 0x0051_9ab0;

pub(crate) const OC325_FILE_COUNT: u32 = 0x4;

pub(crate) const OC325_FILE_MUTEX: u32 = 0x14;

pub(crate) const OC325_FILE_ARRAY: u32 = 0x2c;

pub(crate) const OC325_FILE_HANDLE: u32 = 0x110;

/// The manager's own mutex (`D_00519AC4`).
#[cfg(test)]
pub(crate) const OC325_FILE_MANAGER_MUTEX: u32 = 0x0051_9ac4;

pub(crate) const OC325_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

pub(crate) const OC325_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

/// `EAFileHandle* EAFile_AllocFileHandle(void)` (0x3de800): take the file
/// manager's lock, claim the first free handle (marking it in use) and
/// return it; spin forever when every handle is taken.
pub fn ea_file_alloc_file_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[A0][0] = 0x0052_0000;
        frame.save(c, S1, 0x20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x653c));
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        OC325_MUTEX_LOCK.call(c, 0x003d_e824)?;
        c.g[S2][0] = 0x0052_0000;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6550));
        loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(OC325_FILE_COUNT)).lw(c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let all_taken = c.g[V0][0] == 0;
            c.g[V0][0] = OC325_FILE_HANDLE as u64;
            if all_taken {
                break;
            }
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(OC325_FILE_ARRAY)).lw(c);
            c.mult(0, V0, S1, V0); // v0 = 0x110 * i
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, 0x003d_e830);
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                break;
            }
        }
        c.cycles += 5;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(OC325_FILE_MUTEX));
        OC325_MUTEX_UNLOCK.call(c, 0x003d_e868)?;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(OC325_FILE_COUNT)).lw(c);
        let no_free = c.g[S1][0] != c.g[V0][0];
        c.g[V0][0] = OC325_FILE_HANDLE as u64;
        if !no_free {
            // Every handle is in use: the original spins here forever.
            loop {
                c.cycles += 7;
                if event_due(c) {
                    deliver_event_here(c, 0x003d_e878);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6550));
        c.mult(0, V0, S1, V0); // v0 = 0x110 * i
        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(OC325_FILE_ARRAY)).lw(c);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAFile_QueueOpen (0x3dd4e8)

/// `EAFileRequest` words QueueOpen fills: arg2 at `+0x10`, arg3 at `+0x14`,
/// arg1 at `+0x18`; its status at `+0xc` and its file handle at `+0x24`.
pub(crate) const OC326_QO_ARG2: u32 = 0x10;

pub(crate) const OC326_QO_ARG3: u32 = 0x14;

pub(crate) const OC326_QO_ARG1: u32 = 0x18;

pub(crate) const OC326_QO_STATUS: u32 = 0xc;

pub(crate) const OC326_QO_HANDLE: u32 = 0x24;

/// Where the file name goes inside the file handle.
pub(crate) const OC326_QO_HANDLE_NAME: u32 = 0x10;

/// The name buffer's length.
pub(crate) const OC326_QO_NAME_LEN: u64 = 0x100;

/// Command bits in the request's word 0: the mask that clears the state
/// field (bits 20-23) and the "open" state (2 << 20).
pub(crate) const OC326_QO_CMD_MASK: u32 = 0xff0f_ffff;

pub(crate) const OC326_QO_CMD_OPEN: u64 = 0x0020_0000;

/// Request status 2: failed / signalled.
pub(crate) const OC326_QO_STATUS_FAILED: u64 = 2;

pub(crate) const OC326_EA_ALLOC_REQUEST: Callee = Callee {
    addr: 0x003d_e670,
    f: ee::s14::EAFile_AllocRequest,
};

pub(crate) const OC326_EA_ALLOC_FILE_HANDLE: Callee = Callee {
    addr: 0x003d_e800,
    f: ee::s14::EAFile_AllocFileHandle,
};

pub(crate) const OC326_EA_QUEUE_SIGNAL_STUB: Callee = Callee {
    addr: 0x003d_e668,
    f: ee::s14::EAFile_QueueSignalStub,
};

pub(crate) const OC326_EA_SERVICE_QUEUE: Callee = Callee {
    addr: 0x003d_dfa8,
    f: ee::s14::EAFile_ServiceQueue,
};

/// `strncpy` (0x416ce0), through the function table.
pub(crate) const OC326_STRNCPY: u32 = 0x0041_6ce0;

/// `int EAFile_QueueOpen(const char* name, int a1, int a2, int a3)`
/// (0x3dd4e8): take a request (`EAFile_AllocRequest`), make it an open (the
/// command bits of word 0), store the three arguments, take a file handle for
/// it (`EAFile_AllocFileHandle`, word `+0x24`); a null handle marks the
/// request failed (status 2, signal stub). Copy the name (0x100 bytes) into
/// the handle at `+0x10`, hand the request to `EAFile_ServiceQueue`, and
/// return the request's word 0 (its handle).
pub fn ea_file_queue_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A0][0]; // name
        OC326_EA_ALLOC_REQUEST.call(c, 0x003d_d518)?;
        c.g[S3][0] = c.g[V0][0]; // request
        let req = r32(c, S3);
        c.g[A0][0] = OC326_QO_CMD_OPEN;
        c.g[V1][0] = Addr(req).lw(c);
        c.g[V0][0] = sx(OC326_QO_CMD_MASK); // 0xffff_ffff_ff0f_ffff
        c.g[V0][0] |= 0xffff;
        c.sw(req.wrapping_add(OC326_QO_ARG3), c.g[S0][0]);
        c.g[V1][0] &= c.g[V0][0];
        c.sw(req.wrapping_add(OC326_QO_ARG1), c.g[S1][0]);
        c.g[V1][0] |= c.g[A0][0];
        c.sw(req.wrapping_add(OC326_QO_ARG2), c.g[S2][0]);
        c.sw(req, c.g[V1][0]);
        OC326_EA_ALLOC_FILE_HANDLE.call(c, 0x003d_d548)?;
        let has_handle = c.g[V0][0] != 0;
        c.sw(req.wrapping_add(OC326_QO_HANDLE), c.g[V0][0]);
        if !has_handle {
            c.cycles += 4;
            c.g[V0][0] = OC326_QO_STATUS_FAILED;
            c.g[A0][0] = c.g[S3][0];
            c.sw(req.wrapping_add(OC326_QO_STATUS), c.g[V0][0]);
            OC326_EA_QUEUE_SIGNAL_STUB.call(c, 0x003d_d560)?;
        }
        c.cycles += 16;
        // strncpy(handle->name, name, 0x100)
        c.g[A0][0] = Addr(r32(c, S3)).off(OC326_QO_HANDLE).lw(c);
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = OC326_QO_NAME_LEN;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(OC326_QO_HANDLE_NAME));
        call_ptr(c, OC326_STRNCPY, 0x003d_d574)?;
        c.g[A0][0] = c.g[S3][0];
        OC326_EA_SERVICE_QUEUE.call(c, 0x003d_d57c)?;
        c.g[V0][0] = Addr(r32(c, S3)).lw(c);
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

// ---------------------------------------------------------------------------
// EAFile_QueueClose (0x3dd5a0)

/// The EAFile request pool (`0x519ab0`) and its request-list head (`D_00519AE4`,
/// reached as `0x520000 - 0x651c`).
#[cfg(test)]
pub(crate) const EAFILE_QC_LIST_HEAD: u32 = 0x0051_9ae4;

/// A request's words: the flags/state (+0), the queue link (+0xc), the
/// priority (+0x10), the second argument (+0x14) and the node id (+0x24).
pub(crate) const EAFILE_QC_LINK: u32 = 0xc;

pub(crate) const EAFILE_QC_PRIORITY: u32 = 0x10;

pub(crate) const EAFILE_QC_ARG2: u32 = 0x14;

pub(crate) const EAFILE_QC_ID: u32 = 0x24;

/// A request-list node: its id (+0xc) and next link (+0x14).
pub(crate) const EAFILE_QC_LIST_ID: u32 = 0xc;

pub(crate) const EAFILE_QC_LIST_NEXT: u32 = 0x14;

/// `int EAFile_QueueClose(int id, int arg1, int arg2)` (0x3dd5a0): take a
/// request from the pool, set its state to 3 ("close"), store the arguments
/// and the node id, and mark it signalled when a request-list node with that
/// id exists; then hand it to the service queue and return its state word.
/// Frame 0x50 (s0 = arg1, s1 = arg2, s2 = request, s3 = id).
pub fn ea_file_queue_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        Callee {
            addr: 0x003d_e670,
            f: ee::s14::EAFile_AllocRequest,
        }
        .call(c, 0x003d_d5c8)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = 0x0052_0000;
        c.g[V1][0] = Addr(r32(c, S2)).lw(c);
        c.g[V0][0] = 0xffff_ffff_ff0f_ffff; // clear the state bits
        c.g[A0][0] = 0x30_0000; // state 3
        c.g[V1][0] &= c.g[V0][0];
        c.g[A1][0] = Addr(r32(c, A1).wrapping_sub(0x651c)).lw(c);
        c.g[V1][0] |= c.g[A0][0];
        c.g[A2][0] = 3;
        Addr(r32(c, S2)).sw(c, c.g[V1][0]);
        Addr(r32(c, S2).wrapping_add(EAFILE_QC_PRIORITY)).sw(c, c.g[S0][0]);
        Addr(r32(c, S2).wrapping_add(EAFILE_QC_ARG2)).sw(c, c.g[S1][0]);
        Addr(r32(c, S2).wrapping_add(EAFILE_QC_ID)).sw(c, c.g[S3][0]);
        loop {
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A1).wrapping_add(EAFILE_QC_LIST_ID)).lw(c);
                if c.g[V0][0] != c.g[S3][0] {
                    c.g[A1][0] = Addr(r32(c, A1).wrapping_add(EAFILE_QC_LIST_NEXT)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_d600);
                    }
                    continue;
                }
                c.cycles += 3;
                Addr(r32(c, S2).wrapping_add(EAFILE_QC_LINK)).sw(c, c.g[A2][0]);
                c.g[A0][0] = c.g[S2][0];
                Callee {
                    addr: 0x003d_e668,
                    f: ee::s14::EAFile_QueueSignalStub,
                }
                .call(c, 0x003d_d620)?;
            }
            break;
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x003d_dfa8,
            f: ee::s14::EAFile_ServiceQueue,
        }
        .call(c, 0x003d_d628)?;
        c.g[V0][0] = Addr(r32(c, S2)).lw(c);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAFile_FreeFileHandle (0x3de8c0)

#[cfg(test)]
/// The EAFile file-handle pool mutex (`0x520000 - 0x653c`).
pub(crate) const EAF_HANDLE_MUTEX: u32 = 0x0051_9ac4;

/// Size of one EAFile file-handle record.
pub(crate) const EAF_HANDLE_SIZE: u64 = 0x110;

pub(crate) const EAF_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

pub(crate) const EAF_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

pub(crate) const EAF_MEMSET: Callee = Callee {
    addr: 0x003e_6448,
    f: ee::s14::EA_MemSet,
};

/// `void EAFile_FreeFileHandle(EAFileHandle* handle)` (0x3de8c0): under the
/// file-handle pool mutex, clear the 0x110-byte handle record (a free handle
/// is an all-zero record).
pub fn ea_file_free_file_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0x0052_0000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x653c));
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        EAF_MUTEX_LOCK.call(c, 0x003d_e8e4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = EAF_HANDLE_SIZE;
        EAF_MEMSET.call(c, 0x003d_e8f4)?;
        c.g[A0][0] = c.g[S0][0];
        EAF_MUTEX_UNLOCK.call(c, 0x003d_e8fc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::fake_check;

    // -------------------------------------------------- EAFile_ReleaseRequest

    #[test]
    fn ea_file_release_request_matches() {
        fake_check(
            ssx3_ee::s14::f_003dd310__generated,
            ea_file_release_request,
            44,
            &|c, _seed, r| {
                c.g[A0][0] = DATA as u64; // its top byte is 0
                c.g[SP][0] = (STACK - 0x100) as u64;
                // The hash table entry for top byte 0 points at the request.
                c.bus.write32(0x0051_ad98, DATA);
                // A positive state that equals itself in the compare.
                c.bus.write32(DATA + 8, 1);
                // Request flags: channel class 2..10, so the dispatch is taken.
                let lane = 2 + r.next_u32() % 9;
                c.bus
                    .write32(DATA, (lane << 20) | (r.next_u32() & 0x000f_ffff));
                // The state table (0x495d40) stays zero: default jump_out.
            },
        );
    }

    /// EAFile_StrICmp: strings of 0..5 characters over a mixed-case
    /// alphabet, often `b` a case-flipped copy of `a` (equal, end reached),
    /// sometimes a prefix; uppercase bits set in the ctype table.
    #[test]
    fn eafile_str_icmp_matches() {
        check(
            ee::s14::f_003e2490__generated,
            eafile_str_icmp,
            300,
            &|c, seed| {
                let mut r = Rng(0x003e_2490 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003e_2490;
                for ch in b'A'..=b'Z' {
                    c.bus.write8(EAF_CTYPE + ch as u32, 1);
                }
                const ALPHABET: [u8; 7] = [b'a', b'A', b'b', b'B', b'1', b'_', 0xc1];
                let a = DATA + 0x1000;
                let b = DATA + 0x2000;
                let n = seed % 6;
                for i in 0..n {
                    let ch = ALPHABET[(r.next_u32() % 7) as usize];
                    c.bus.write8(a + i, ch);
                }
                c.bus.write8(a + n, 0);
                let m = if seed % 7 == 0 {
                    n.saturating_sub(1)
                } else {
                    n
                };
                for i in 0..m {
                    let x = c.bus.read8(a + i);
                    let ch = if seed % 3 != 2 {
                        if x.is_ascii_lowercase() && r.next_u32() % 2 == 0 {
                            x - 32
                        } else {
                            x
                        }
                    } else {
                        ALPHABET[(r.next_u32() % 7) as usize]
                    };
                    c.bus.write8(b + i, ch);
                }
                c.bus.write8(b + m, 0);
                c.g[A0][0] = sx(a);
                c.g[A1][0] = sx(b);
            },
        );
    }

    /// EAFile_AllocRequest: pools of 1..6 slots with the first free one
    /// anywhere (or a negative count: no scan), serials near the wrap; the
    /// real EAMutex lock/unlock with fake thread-id/semaphore syscalls (some
    /// unwinding). The all-busy case spins forever in the original and is
    /// not run.
    #[test]
    fn ea_file_alloc_request_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        const SLOTS: u32 = DATA + 0x1000;
        check(
            ssx3_ee::s14::f_003de670__generated,
            ea_file_alloc_request,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_e670, 6);
                c.pc = 0x003d_e670;
                let count: i32 = if seed % 7 == 0 {
                    -1 - (r.next_u32() % 3) as i32
                } else {
                    1 + (r.next_u32() % 6) as i32
                };
                c.bus.write32(EAFILE_POOL, count as u32);
                c.bus.write32(EAFILE_POOL + EAFILE_POOL_SLOTS, SLOTS);
                // Mutex: owner never a fake thread id, depth, semaphore.
                c.bus
                    .write32(EAFILE_POOL_MUTEX + 0x4, 0x7700_0000 | r.next_u32() & 0xffff);
                c.bus.write32(EAFILE_POOL_MUTEX + 0x8, r.next_u32() & 3);
                c.bus.write32(EAFILE_POOL_MUTEX + 0xc, r.next_u32() & 0xff);
                c.bus.write32(0x0045_0de4, r.next_u32() & 1);
                let free = if count > 0 {
                    r.next_u32() % count as u32
                } else {
                    0
                };
                for i in 0..8u32 {
                    let mut w = r.next_u32();
                    if i == free {
                        w &= !0x00f0_0000;
                    } else if i < free && w & 0x00f0_0000 == 0 {
                        w |= 0x0010_0000 << (r.next_u32() % 4);
                    }
                    c.bus.write32(SLOTS + EAFILE_REQ_SIZE * i, w);
                }
                let serial = match seed % 3 {
                    0 => 0xf_fffe + r.next_u32() % 3,
                    1 => r.next_u32(),
                    _ => r.next_u32() & 0xf_ffff,
                };
                c.bus.write32(EAFILE_REQ_SERIAL, serial);
            },
        );
    }

    /// EAFile_QueueRead: the real EAFile_AllocRequest on a pool with a free
    /// slot (EAFile_AllocRequest's own fixture: mutex with fake syscalls,
    /// some unwinding), a null file or one whose size clamps the read or
    /// not, and the real EAFile_ServiceQueue on its short path (a request
    /// already active at pool+0x24: insert into the priority queue of 0..2
    /// requests, unlock, return).
    #[test]
    fn ea_file_queue_read_matches() {
        use crate::difftest::fakes::machine;
        const POOL: u32 = 0x0051_9ab0;
        const MUTEX: u32 = POOL + 0x14;
        const SLOTS: u32 = DATA + 0x1000;
        const FILE: u32 = DATA + 0x2000;
        const QUEUED: u32 = DATA + 0x3000;
        check(
            ee::s14::f_003dd648__generated,
            ea_file_queue_read,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_d648, 8);
                c.pc = 0x003d_d648;
                let count = 1 + r.next_u32() % 6;
                c.bus.write32(POOL, count);
                c.bus.write32(POOL + 0x28, SLOTS);
                c.bus
                    .write32(MUTEX + 0x4, 0x7700_0000 | r.next_u32() & 0xffff);
                c.bus.write32(MUTEX + 0x8, r.next_u32() & 3);
                c.bus.write32(MUTEX + 0xc, r.next_u32() & 0xff);
                c.bus.write32(0x0045_0de4, r.next_u32() & 1);
                let free = r.next_u32() % count;
                for i in 0..8u32 {
                    let mut w = r.next_u32();
                    if i == free {
                        w &= !0x00f0_0000;
                    } else if i < free && w & 0x00f0_0000 == 0 {
                        w |= 0x0010_0000;
                    }
                    c.bus.write32(SLOTS + 0x30 * i, w);
                }
                c.bus.write32(0x0045_0c0c, r.next_u32() & 0xf_ffff);
                // The service queue: a request active; 0..2 queued.
                c.bus.write32(POOL + 0x24, QUEUED + 0x100);
                let queued = seed % 3;
                c.bus
                    .write32(POOL + 0x30, if queued == 0 { 0 } else { QUEUED });
                for k in 0..queued {
                    let q = QUEUED + 0x40 * k;
                    c.bus.write32(q + 0x10, r.next_u32() % 8);
                    c.bus
                        .write32(q + 0x2c, if k + 1 < queued { q + 0x40 } else { 0 });
                }
                c.bus.write32(POOL + 0xc, r.next_u32() & 0xff);
                c.bus
                    .write32(FILE + EAFILE_QR_FILE_SIZE, r.next_u32() & 0xf_ffff);
                c.g[A0][0] = if seed % 5 == 0 { 0 } else { sx(FILE) };
                c.g[A1][0] = sx(r.next_u32() & 0x7_ffff);
                c.g[A2][0] = sx(DATA + 0x8000);
                c.g[A3][0] = sx(r.next_u32() & 0x7_ffff);
                c.g[T0][0] = sx(r.next_u32() % 8);
                c.g[T1][0] = sx(r.next_u32());
            },
        );
    }

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::RA as SENTINEL;

    // -- 0x003de800 EAFile_AllocFileHandle -----------------------------------

    /// The file manager with 1..4 handles, at least one free (the original
    /// spins forever when every handle is taken), and its mutex owned by the
    /// thread id the fakes return (0x42), so both mutex calls take the fast
    /// path.
    pub(crate) fn oc325_file_alloc_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x003d_e800, 3);
        for i in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * i, 0x42);
        }
        c.bus.write32(OC325_FILE_MANAGER_MUTEX + 4, 0x42);
        c.bus.write32(OC325_FILE_MANAGER_MUTEX + 8, 1);
        c.bus.write32(OC325_FILE_MANAGER_MUTEX + 0xc, 32);
        c.bus.write32(0x0045_0de4, 0);
        let count = 1 + r.next_u32() % 4;
        let array = DATA + 0x1_0000;
        c.bus.write32(OC325_FILE_MANAGER + OC325_FILE_COUNT, count);
        c.bus.write32(OC325_FILE_MANAGER + OC325_FILE_ARRAY, array);
        let free = r.next_u32() % count;
        for i in 0..count {
            let handle = array + OC325_FILE_HANDLE * i;
            c.bus.write32(handle, if i == free { 0 } else { 1 });
            c.bus.write32(handle + 4, r.next_u32());
        }
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x003d_e800;
    }

    /// The mutex fast path on both calls and a free handle at a random
    /// index; some mutex calls unwind (propagated).
    #[test]
    fn ea_file_alloc_file_handle_matches() {
        covered(
            ee::s14::f_003de800__generated,
            ea_file_alloc_file_handle,
            200,
            &|c, seed| oc325_file_alloc_setup(c, seed),
        );
    }

    // ------------------------------------------------------ EAFile_QueueOpen

    /// EAFile_QueueOpen: the real EAFile_AllocRequest (a pool with a free
    /// request slot; mutex with fake syscalls, some unwinding) and
    /// EAFile_AllocFileHandle (a free handle slot), a name buffer, and the
    /// real EAFile_ServiceQueue on its short path (a request already active
    /// at pool+0x24, 0..2 queued). The null-handle branch is not reached: the
    /// real AllocFileHandle spins when every handle is busy instead of
    /// returning 0.
    #[test]
    fn ea_file_queue_open_matches() {
        use crate::game::lib::eafile::{EAFILE_POOL_SLOTS, EAFILE_REQ_SIZE};
        const POOL: u32 = 0x0051_9ab0;
        const MUTEX: u32 = POOL + 0x14;
        const SLOTS: u32 = DATA + 0x1000;
        const HANDLES: u32 = DATA + 0x4000;
        const QUEUED: u32 = DATA + 0x8000;
        const NAME: u32 = DATA + 0xc000;
        const HANDLE_COUNT: u32 = POOL + 0x4;
        const HANDLE_SLOTS: u32 = POOL + 0x2c;
        check(
            ee::s14::f_003dd4e8__generated,
            ea_file_queue_open,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_d4e8, 8);
                c.pc = 0x003d_d4e8;
                // The request pool: 1..6 slots, the first free one anywhere.
                let count = 1 + r.next_u32() % 6;
                c.bus.write32(POOL, count);
                c.bus.write32(POOL + EAFILE_POOL_SLOTS, SLOTS);
                // The handle pool: 1..6 slots, all free.
                let handles = 1 + r.next_u32() % 6;
                c.bus.write32(HANDLE_COUNT, handles);
                c.bus.write32(HANDLE_SLOTS, HANDLES);
                for i in 0..6u32 {
                    c.bus.write32(HANDLES + 0x110 * i, 0);
                }
                // Mutex: owner never a fake thread id, depth, semaphore.
                c.bus
                    .write32(MUTEX + 0x4, 0x7700_0000 | r.next_u32() & 0xffff);
                c.bus.write32(MUTEX + 0x8, r.next_u32() & 3);
                c.bus.write32(MUTEX + 0xc, r.next_u32() & 0xff);
                c.bus.write32(0x0045_0de4, r.next_u32() & 1);
                let free = r.next_u32() % count;
                for i in 0..8u32 {
                    let mut w = r.next_u32();
                    if i == free {
                        w &= !0x00f0_0000;
                    } else if i < free && w & 0x00f0_0000 == 0 {
                        w |= 0x0010_0000;
                    }
                    c.bus.write32(SLOTS + EAFILE_REQ_SIZE * i, w);
                }
                c.bus.write32(0x0045_0c0c, r.next_u32() & 0xf_ffff);
                // The service queue: a request active; 0..2 queued.
                c.bus.write32(POOL + 0x24, QUEUED + 0x100);
                let queued = seed % 3;
                c.bus
                    .write32(POOL + 0x30, if queued == 0 { 0 } else { QUEUED });
                for k in 0..queued {
                    let q = QUEUED + 0x40 * k;
                    c.bus.write32(q + 0x10, r.next_u32() % 8);
                    c.bus
                        .write32(q + 0x2c, if k + 1 < queued { q + 0x40 } else { 0 });
                }
                c.bus.write32(POOL + 0xc, r.next_u32() & 0xff);
                // The name: a short, NUL-terminated string.
                let len = r.next_u32() % 0x40;
                for i in 0..len {
                    c.bus.write8(NAME + i, b'a' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(NAME + len, 0);
                c.g[A0][0] = sx(NAME);
                c.g[A1][0] = sx(r.next_u32() & 0x7_ffff);
                c.g[A2][0] = sx(r.next_u32() & 0x7_ffff);
                c.g[A3][0] = sx(r.next_u32() & 0x7_ffff);
            },
        );
    }

    /// `EAFile_QueueClose`: a real request pool (two free slots), a
    /// request-list with no matching, a matching or a second matching node,
    /// and priorities that keep the real service queue on its short path.
    #[test]
    fn ea_file_queue_close_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        const POOL: u32 = 0x0051_9ab0;
        const SLOTS: u32 = DATA + 0x1_0000;
        const NODE: u32 = DATA + 0x2_0000;
        const NODE2: u32 = DATA + 0x2_0100;
        const ID: u32 = 0x1234;
        check(
            ee::s14::f_003dd5a0__generated,
            ea_file_queue_close,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_d5a0, 0);
                c.pc = 0x003d_d5a0;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // The request pool: two free slots, a zeroed mutex and a zeroed
                // request queue (the real service queue then takes its short
                // path).
                for i in 0..0x10u32 {
                    w(c, POOL + 4 * i, 0);
                }
                w(c, POOL, 2);
                w(c, POOL + 0x28, SLOTS);
                for i in 0..0x20u32 {
                    w(c, SLOTS + 4 * i, 0);
                }
                // The request list.
                let head = if seed % 3 == 0 { 0 } else { NODE };
                w(c, EAFILE_QC_LIST_HEAD, head);
                fill(c, &mut r, NODE, 0x20);
                fill(c, &mut r, NODE2, 0x20);
                w(
                    c,
                    NODE + EAFILE_QC_LIST_ID,
                    if seed % 2 == 0 { ID } else { ID + 1 },
                );
                w(
                    c,
                    NODE + EAFILE_QC_LIST_NEXT,
                    if seed % 4 == 0 { NODE2 } else { 0 },
                );
                w(c, NODE2 + EAFILE_QC_LIST_ID, ID);
                w(c, NODE2 + EAFILE_QC_LIST_NEXT, 0);
                c.g[A0][0] = ID as u64;
                c.g[A1][0] = 1 + (r.next_u32() % 0x100) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    // --------------------------------------------------- EAFile_FreeFileHandle

    /// A real handle pool mutex (short path) and a handle record to clear.
    #[test]
    fn ea_file_free_file_handle_matches() {
        check(
            ee::s14::f_003de8c0__generated,
            ea_file_free_file_handle,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_e8c0, 4);
                c.pc = 0x003d_e8c0;
                let owner = 0x40 + seed % 3;
                c.bus.write32(EAF_HANDLE_MUTEX + 4, owner);
                c.bus.write32(EAF_HANDLE_MUTEX + 8, 1 + seed % 2);
                c.bus.write32(EAF_HANDLE_MUTEX, r.next_u32() & 0xff);
                let handle = DATA + 0x100 + (r.next_u32() % 0x40) * 4;
                for i in 0..(EAF_HANDLE_SIZE / 4) as u32 {
                    c.bus.write32(handle + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(handle);
            },
        );
    }
}
