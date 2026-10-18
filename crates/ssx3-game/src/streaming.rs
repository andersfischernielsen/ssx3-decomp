//! Work package E6 file & stream I/O (docs/investigation/phase2-candidates-3.md).
//!
//! Hand-written, typed replacements for the generated file/stream functions,
//! each with a differential test against the generated body.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_78d8, sndstrm_queue, "SNDSTRM_Queue"),
    (0x003d_3030, path_get_track_stream, "PATH_GetTrackStream"),
    (0x002a_e020, path_plat_timer_callback, "PATH_PlatTimerCallback"),
    (0x003e_00d8, stream_file_on_open, "StreamFile_OnOpen"),
    (0x003d_f980, async_file_is_done, "AsyncFile_IsDone"),
    (0x003e_3350, eafile_platform_read, "EAFile_PlatformRead"),
    (0x002a_f370, path_plat_get_time_ms, "PATH_PlatGetTimeMs"),
];

/// Direct/indirect call targets used by this package.
mod callee {
    use super::Callee;
    use ssx3_ee::{s08, s13, s14};

    pub const SNDSTRM_QUEUE_REQUEST: Callee =
        Callee { addr: 0x003b_76e8, f: s13::SNDSTRM_QueueRequest };
    pub const PATH_FIND_TRACK: Callee = Callee { addr: 0x003d_5290, f: s14::PATH_FindTrack };
    pub const EAFILE_RELEASE_REQUEST: Callee =
        Callee { addr: 0x003d_d310, f: s14::EAFile_ReleaseRequest };
    pub const STREAM_FILE_ISSUE_READ: Callee =
        Callee { addr: 0x003e_03d0, f: s14::StreamFile_IssueRead };
    pub const EAFILE_QUEUE_OPEN: Callee = Callee { addr: 0x003d_d4e8, f: s14::EAFile_QueueOpen };
    pub const EAFILE_SET_REQUEST_CALLBACK: Callee =
        Callee { addr: 0x003d_cf70, f: s14::EAFile_SetRequestCallback };
    pub const EAMUTEX_LOCK: Callee = Callee { addr: 0x003e_5700, f: s14::EAMutex_Lock };
    pub const EAMUTEX_UNLOCK: Callee = Callee { addr: 0x003e_5760, f: s14::EAMutex_Unlock };
    pub const ASYNCFILE_FROM_HANDLE: Callee =
        Callee { addr: 0x003d_ef70, f: s14::AsyncFile_FromHandle };
    pub const EAFILE_CD_READ: Callee = Callee { addr: 0x003e_4648, f: s14::EAFile_CdRead };
    pub const PATH_PLAT_GET_TIME_MS: Callee =
        Callee { addr: 0x002a_f370, f: s08::PATH_PlatGetTimeMs };
}

/// The shared mutex base used by the `EAFile_*` / `AsyncFile_*` functions
/// (`0x520000 - 0x63f8`).
const MUTEX: u32 = 0x0051_9c08;
/// The platform-read parameter block (`0x520000 - 0x63e0`).
const PLAT_READ_PARAMS: u32 = 0x0051_9c20;

// ---------------------------------------------------------------------------
// Simple wrappers.

/// `SNDSTRM_Queue(...)` (0x3b78d8): forward to `SNDSTRM_QueueRequest` with
/// the fifth argument (`t0`) zero.
pub fn sndstrm_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[T0][0] = 0;
        callee::SNDSTRM_QUEUE_REQUEST.call(c, 0x003b_78e8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_GetTrackStream(int track)` (0x3d3030): the track's stream handle
/// at +0x54, or -8 when the track does not exist.
pub fn path_get_track_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::PATH_FIND_TRACK.call(c, 0x003d_3040)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = sx(0xffff_fff8);
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x54)).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PATH_PlatTimerCallback(void)` (0x2ae020): call the timer callback
/// pointer stored at `gp + 0x6a8` if it is set, then return 0.
pub fn path_plat_timer_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let callback = Addr((c.g[GP][0] as u32).wrapping_add(0x6a8)).lw(c);
        c.g[V0][0] = callback;
        frame.save_ra(c, 0);
        if callback != 0 {
            c.cycles += 2;
            call_ptr(c, callback as u32, 0x002a_e038)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Stream file callbacks.

/// `void StreamFile_OnOpen(?, ?, StreamFile *self)` (0x3e00d8): release the
/// pending request and, on success, issue the next read.
pub fn stream_file_on_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let file = c.g[A2][0];
        c.g[S0][0] = file;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((file as u32).wrapping_add(0x174)).lw(c);
        callee::EAFILE_RELEASE_REQUEST.call(c, 0x003e_00f0)?;
        let request = c.g[V0][0];
        Addr((file as u32).wrapping_add(0x16c)).sw(c, request);
        if request != 0 {
            c.cycles += 3;
            c.g[A0][0] = file;
            c.g[A1][0] = Addr((file as u32).wrapping_add(0x40)).lw(c);
            callee::STREAM_FILE_ISSUE_READ.call(c, 0x003e_0104)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamFile_OnClose(?, ?, StreamFile *self)` (0x3e0118): release the
/// pending request and queue a fresh open request.
pub fn stream_file_on_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let file = c.g[A2][0];
        c.g[S0][0] = file;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((file as u32).wrapping_add(0x174)).lw(c);
        callee::EAFILE_RELEASE_REQUEST.call(c, 0x003e_0130)?;
        c.g[A2][0] = Addr((file as u32).wrapping_add(0x40)).lw(c);
        c.g[A0][0] = (file as u32).wrapping_add(0x6c) as u64;
        c.g[A1][0] = 1;
        c.g[A3][0] = file;
        callee::EAFILE_QUEUE_OPEN.call(c, 0x003e_0144)?;
        let request = c.g[V0][0];
        Addr((file as u32).wrapping_add(0x174)).sw(c, request);
        if request != 0 {
            c.cycles += 4;
            c.g[A0][0] = request;
            c.g[A1][0] = 0x003e_00d8;
            callee::EAFILE_SET_REQUEST_CALLBACK.call(c, 0x003e_015c)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool AsyncFile_IsDone(int handle)` (0x3df980): whether the async file's
/// request has finished, or -1 if the handle is unknown.
pub fn async_file_is_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = 0x0052_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(MUTEX);
        callee::EAMUTEX_LOCK.call(c, 0x003d_f9a0)?;
        c.g[A0][0] = c.g[S0][0];
        callee::ASYNCFILE_FROM_HANDLE.call(c, 0x003d_f9a8)?;
        let file = c.g[V0][0];
        c.g[S0][0] = sx(0xffff_ffff);
        if file != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr((file as u32).wrapping_add(0x1c)).lw(c);
            c.g[S0][0] = (c.g[V0][0] < 1) as u64;
        }
        c.cycles += 8;
        c.g[A0][0] = sx(MUTEX);
        callee::EAMUTEX_UNLOCK.call(c, 0x003d_f9c0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Platform read.

/// `int EAFile_PlatformRead(int handle, int p1, int p2, int p3)` (0x3e3350):
/// when the handle's channel byte is 1, read from CD (`EAFile_CdRead` with
/// the low 24 bits); otherwise record the request in the platform block.
pub fn eafile_platform_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let masked = c.g[A0][0] & 0x00ff_ffff;
        c.g[T1][0] = masked;
        c.g[T0][0] = 1;
        let channel = sx((((c.g[A0][0] as u32) as i32) >> 24) as u32);
        c.g[A0][0] = channel;
        frame.save_ra(c, 0);
        if channel == 1 {
            c.cycles += 3;
            c.g[A0][0] = c.g[T1][0];
            callee::EAFILE_CD_READ.call(c, 0x003e_33a0)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 10;
            c.g[V1][0] = 0x0052_0000;
            c.g[V0][0] = 1;
            Addr(PLAT_READ_PARAMS).sw(c, c.g[T0][0]);
            c.g[V1][0] = sx(PLAT_READ_PARAMS);
            Addr(PLAT_READ_PARAMS + 0x4).sw(c, c.g[T1][0]);
            Addr(PLAT_READ_PARAMS + 0x8).sw(c, c.g[A1][0]);
            Addr(PLAT_READ_PARAMS + 0xc).sw(c, c.g[A2][0]);
            Addr(PLAT_READ_PARAMS + 0x10).sw(c, c.g[A3][0]);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Time.

/// `int PATH_PlatGetTimeMs(void)` (0x2af370): current tick count divided by
/// the tick period, scaled to milliseconds.
pub fn path_plat_get_time_ms(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x003e_4e98, 0x002a_f380)?;
        let count = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
        abi::set_fpr(c, 12, count);
        let period = FpuF32::from_i32(Addr(0x0045_0dc8).read_fpu(c).0 as i32);
        abi::set_fpr(c, 0, period);
        c.g[AT][0] = 0x447a_0000;
        abi::set_fpr(c, 1, FpuF32(0x447a_0000));
        let ratio = count.div(period);
        abi::set_fpr(c, 12, ratio);
        abi::set_fpr(c, 12, ratio * FpuF32(0x447a_0000));
        call_ptr(c, 0x0041_39f8, 0x002a_f3a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, ptr, FAKE};
    use crate::difftest::{check, GAME_GP};

    const SEEDS: u32 = 150;

    #[test]
    fn sndstrm_queue_matches() {
        check(ssx3_ee::s13::f_003b78d8, sndstrm_queue, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x51, 0);
            let this = ptr(&mut r);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn path_get_track_stream_matches() {
        check(ssx3_ee::s14::f_003d3030, path_get_track_stream, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x52, 0);
            c.g[A0][0] = (r.next_u32() % 6) as u64;
            // A plausible track table so both branches are reachable.
            let table = ptr(&mut r);
            c.bus.write32(0x0051_765c, table);
            c.bus.write32(table, if seed % 2 == 0 { 0 } else { ptr(&mut r) });
        });
    }

    #[test]
    fn path_plat_timer_callback_matches() {
        covered(ssx3_ee::s08::f_002ae020, path_plat_timer_callback, SEEDS, &|c, seed| {
            machine(c, seed, 0x53, 1);
            let cb = if seed % 4 == 0 { 0 } else { FAKE + 0x40 };
            c.bus.write32(GAME_GP + 0x6a8, cb);
        });
    }

    #[test]
    fn stream_file_on_open_matches() {
        check(ssx3_ee::s14::f_003e00d8, stream_file_on_open, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x54, 0);
            let file = ptr(&mut r);
            c.g[A2][0] = file as u64;
            c.bus.write32(file + 0x174, if seed % 3 == 0 { 0 } else { ptr(&mut r) });
            c.bus.write32(file + 0x40, r.next_u32());
        });
    }

    #[test]
    fn stream_file_on_close_matches() {
        check(ssx3_ee::s14::f_003e0118, stream_file_on_close, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x55, 0);
            let file = ptr(&mut r);
            c.g[A2][0] = file as u64;
            c.bus.write32(file + 0x174, if seed % 3 == 0 { 0 } else { ptr(&mut r) });
            c.bus.write32(file + 0x40, r.next_u32());
            // EAFile's request and file-handle pools (counts 0x519ab0 /
            // 0x519ab4, tables 0x519ad8 / 0x519adc, entries 0x30 / 0x110
            // bytes): with no free entry the allocators spin forever.
            for (count, table, at, size) in [(0x0051_9ab0, 0x0051_9ad8, 0xf_0000, 0x30), (0x0051_9ab4, 0x0051_9adc, 0xf_1000, 0x110)] {
                let base = crate::difftest::DATA + at;
                c.bus.write32(count, 4);
                c.bus.write32(table, base);
                for i in 0..size {
                    c.bus.write32(base + 4 * i, 0);
                }
            }
        });
    }

    #[test]
    fn async_file_is_done_matches() {
        check(ssx3_ee::s14::f_003df980, async_file_is_done, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x56, 0);
            c.g[A0][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn eafile_platform_read_matches() {
        check(ssx3_ee::s14::f_003e3350, eafile_platform_read, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x57, 0);
            // Cover both the CD path (byte 3 == 1) and the platform-block path.
            let handle = if seed % 2 == 0 {
                0x0100_0000 | (r.next_u32() & 0x00ff_ffff)
            } else {
                r.next_u32()
            };
            c.g[A0][0] = handle as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn path_plat_get_time_ms_matches() {
        check(ssx3_ee::s08::f_002af370, path_plat_get_time_ms, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x58, 0);
            c.bus.write32(0x0045_0dcc, r.next_u32());
            c.bus.write32(0x0045_0dc8, if seed % 4 == 0 { 0 } else { r.next_u32() });
        });
    }
}
