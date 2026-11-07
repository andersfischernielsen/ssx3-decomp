//! `src/world/streamman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tags::lbu_r;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2 as sp;
pub(crate) use crate::game::wscript::wscriptman::p;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_6e20, c_stream_man_dtor, "cStreamMan_dtor"),
    (0x003a_6e98, stream_man_init, "cStreamMan_Init"),
    (0x003a_6ed8, c_stream_man_reset, "cStreamMan_Reset"),
    (
        0x003a_6f38,
        c_stream_man_free_buffers,
        "cStreamMan_FreeBuffers",
    ),
    (0x003a_6f88, c_stream_man_queue_read, "cStreamMan_QueueRead"),
    (0x003a_7058, cstream_man_set_rate, "cStreamMan_SetRate"),
    (0x003a_7098, c_stream_man_update, "cStreamMan_Update"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;

    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;

    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE;

    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;

    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {

    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;

    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
}

// ---------------------------------------------------------------------------
// Stream manager

/// `void cStreamMan_SetRate(cStreamMan *man, int rate)` (0x3a7058): clamp the
/// rate to at least 0x8000 and store it and the tick period (from the global
/// timer) at +0xd8/+0xdc.
pub fn cstream_man_set_rate(c: &mut Cpu) {
    c.cycles += 16;
    let man = c.g[A0][0] as u32;
    c.g[V0][0] = 0x8000;
    c.g[AT][0] = 0x3800_0000;
    c.f[2] = c.g[1][0] as u32;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V0][0] == 0 {
        c.g[V1][0] = c.g[A1][0];
    }
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.f[1] = FpuF32::from_i32(c.g[V1][0] as u32 as i32).0;
    Addr(man).off(0xd8).sw(c, c.g[V1][0]);
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[2])).0;
    c.f[0] = Addr(c.g[V0][0] as u32).off(0x10).read_fpu(c).0;
    c.f[0] = FpuF32::from_i32(c.f[0] as i32).0;
    c.f[0] = FpuF32(c.f[0]).div(FpuF32(c.f[1])).0;
    Addr(man).off(0xdc).write_fpu(c, FpuF32(c.f[0]));
    abi::ret(c);
}

/// `int cStreamMan_Init(cStreamMan* self, const char* name, void* owner)`
/// (0x3A6E98): remember the owner at +0, format the stream's name into +4 and
/// report success.
pub fn stream_man_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        let name = c.g[A1][0];
        c.g[V0][0] = name;
        frame.save_ra(c, 0);
        let this = r(c, A0);
        let owner = c.g[A2][0];
        Addr(this).sw(c, owner);
        c.g[A0][0] = sx(this.wrapping_add(4));
        c.g[A1][0] = sx(0x004a_47d8);
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = sx(0x004a_47e0);
        callee::SPRINTF.call(c, 0x003a_6ec4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

/// `void cStreamMan_FreeBuffers(cStreamMan* self)` (0x3a6f38): destroy the
/// stream file and free the two buffer blocks.
pub fn c_stream_man_free_buffers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(this, 0x88)).lw(c);
        call_ptr(c, table::STREAM_FILE_DESTROY, 0x003a_6f50)?;
        c.g[A0][0] = Addr(p(this, 0x84)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x003a_6f64)?;
        }
        c.g[A0][0] = Addr(p(this, 0x8c)).lw(c);
        c.cycles += 2;
        let none = c.g[A0][0] == 0;
        frame.restore(c, S0, 0x10);
        if !none {
            c.cycles += 3;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x003a_6f78)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cStreamMan_Reset(cStreamMan* man)` (0x3a6ed8): clear the streaming
/// queues and re-apply the default rate.
pub fn c_stream_man_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = u64::MAX;
        c.g[A2][0] = 0x1;
        c.g[A1][0] = 0x1_0000;
        Addr(at(c, V0, 0x94)).sw(c, c.g[V1][0]);
        Addr(at(c, V0, 0xd4)).sw(c, c.g[A2][0]);
        c.g[A1][0] = c.g[A1][0] | 0x9000;
        Addr(at(c, V0, 0x98)).sw(c, 0);
        Addr(at(c, V0, 0x4)).sb(c, 0);
        Addr(at(c, V0, 0x90)).sw(c, 0);
        Addr(at(c, V0, 0xc0)).sw(c, 0);
        Addr(at(c, V0, 0xcc)).sw(c, 0);
        Addr(at(c, V0, 0xd0)).sw(c, 0);
        Addr(at(c, V0, 0xc8)).sw(c, 0);
        Addr(at(c, V0, 0xc4)).sw(c, 0);
        Addr(at(c, V0, 0xac)).sw(c, 0);
        Addr(at(c, V0, 0xb0)).sw(c, 0);
        Addr(at(c, V0, 0xb4)).sw(c, 0);
        call_ptr(c, 0x003a_7058, 0x003a_6f2c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cStreamMan_dtor(self, flags)` (0x3a6e20): free the stream buffer when
/// the state says so, reset and free the buffers, and delete the object when
/// `flags & 1`.
pub fn c_stream_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(16, sp(c).wrapping_add(0x20));
        c.sq(17, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = lw_r(c, S0, 0xcc);
        let none = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 3;
            c.g[V0][0] = lw_r(c, S0, 0xc0);
            let zero = c.g[V0][0] == 0;
            c.g[V0][0] = 0xff;
            if !zero {
                c.cycles += 3;
                c.g[V1][0] = lbu_r(c, S0, 0xbc);
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 2;
                    callee::EA_MEM_FREE.call(c, 0x003a_6e60)?;
                }
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        callee::CSTREAM_MAN_RESET.call(c, 0x003a_6e68)?;
        c.g[A0][0] = c.g[S0][0];
        callee::CSTREAM_MAN_FREE_BUFFERS.call(c, 0x003a_6e70)?;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] == 0 {
            c.lq(16, sp(c).wrapping_add(0x20));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::BUILTIN_DELETE.addr, 0x003a_6e84)?;
            c.lq(16, sp(c).wrapping_add(0x20));
        }
        c.cycles += 4;
        c.lq(17, sp(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `int cStreamMan_QueueRead(cStreamMan* self, u32 offset, u32 size, u32 tag)`
/// (0x3a6f88): queue a read of the stream file (tag "DNEC"); returns 1 when a
/// read was queued.
pub fn c_stream_man_queue_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x40);
        let sp0 = spv(c);
        c.sq(16, at_2(sp0, 0x30));
        c.sq(17, at_2(sp0, 0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, at_2(sp0, 0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S2][0] = c.g[A2][0];
        let this = c.g[S0][0];
        c.g[V0][0] = lw_at(c, this, 0x90);
        let busy = c.g[V0][0] != 0;
        c.g[A2][0] = c.g[A3][0];
        let mut queued = false;
        if !busy {
            c.cycles += 7;
            c.g[A0][0] = lw_at(c, this, 0x88);
            c.g[A3][0] = 0x444e0000;
            c.g[A1][0] = sx(at_2(this as u32, 4));
            c.g[A3][0] = c.g[A3][0] | 0x4543;
            call_ptr(c, callee::STREAM_FILE_QUEUE_RANGE, 0x003a_6fc8)?;
            let ok = c.g[V0][0] == 0;
            c.sw(at_2(this as u32, 0x9c), c.g[V0][0]);
            if !ok {
                c.cycles += 9;
                c.g[V0][0] = 1;
                c.sw(at_2(this as u32, 0x94), c.g[S1][0]);
                c.sw(at_2(this as u32, 0x90), c.g[V0][0]);
                c.sw(at_2(this as u32, 0x98), c.g[S2][0]);
                c.sw(at_2(this as u32, 0xe0), 0);
                c.sw(at_2(this as u32, 0xa0), 0);
                c.sw(at_2(this as u32, 0xc4), 0);
                c.sw(at_2(this as u32, 0xe4), 0);
                queued = true;
            }
        }
        if !queued {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        c.lq(16, at_2(sp0, 0x30));
        c.lq(17, at_2(sp0, 0x20));
        c.lq(18, at_2(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

pub(crate) mod stream_man {

    /// The world cache this manager streams into (`+0`).
    pub const CACHE: u32 = 0x0;

    /// The stream file handle (`+0x88`).
    pub const FILE: u32 = 0x88;

    /// The decompression buffer (`+0x8c`).
    pub const BUFFER: u32 = 0x8c;

    /// A hull is streaming (`+0x90`) and which one (`+0x94`).
    pub const ACTIVE: u32 = 0x90;

    pub const HULL: u32 = 0x94;

    /// Parsing stopped mid-buffer and must be resumed first (`+0xac`).
    pub const PENDING: u32 = 0xac;

    /// Bytes left in the buffer (`+0xb0`) and the read cursor (`+0xb4`).
    pub const LEFT: u32 = 0xb0;

    pub const CURSOR: u32 = 0xb4;

    /// Throttled streaming (`+0xd4`): a chunk every `+0xdc` audio ticks,
    /// `+0xe0` the ticks still to wait, `+0xe4` the tick last seen.
    pub const THROTTLED: u32 = 0xd4;

    pub const INTERVAL: u32 = 0xdc;

    pub const WAIT: u32 = 0xe0;

    pub const LAST_TICK: u32 = 0xe4;

    /// `gp+0x2a74`: the audio frame clock; its tick count at `+0x18`.
    pub const GP_CLOCK: u32 = 0x2a74;

    pub const CLOCK_TICKS: u32 = 0x18;
}

pub(crate) mod stream_callee {
    use super::ee;
    use super::Callee;

    pub const PARSE_CHUNKS: Callee = Callee {
        addr: 0x003a_7238,
        f: ee::s13::cStreamMan_ParseChunks,
    };

    pub const DECOMPRESS: Callee = Callee {
        addr: 0x003a_7218,
        f: ee::s13::cStreamMan_Decompress,
    };

    pub const SET_HULL_RESIDENT: Callee = Callee {
        addr: 0x003a_8650,
        f: ee::s13::cWorldCache_SetHullResident,
    };

    /// `StreamFile_GetChunk`, `StreamFile_ReleaseChunk`, `StreamFile_IsIdle`
    /// (called through the function table).
    pub const GET_CHUNK: u32 = 0x003e_12e0;

    pub const RELEASE_CHUNK: u32 = 0x003e_13e8;

    pub const IS_IDLE: u32 = 0x003e_1530;
}

/// `void cStreamMan_Update(cStreamMan* self)` (0x3a7098): per-frame hull
/// streaming. While a hull is streaming: resume a parse that ran out of
/// buffer; when throttled, count the audio ticks down and wait; then take
/// every chunk the stream file has, decompress it into the buffer and parse
/// its resources (stopping after one chunk when throttled, or when parsing
/// stalls, which marks it pending). When the file is idle, the hull is
/// resident: tell the world cache and stop.
pub fn c_stream_man_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(this + stream_man::ACTIVE).lw(c);
        // The paths through the block at 0x3a71fc charge one more cycle.
        let mut tail = false;
        if c.g[V0][0] != 0 {
            tail = stream_man_update_active(c, this)?;
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body of [`c_stream_man_update`] for an active manager; `Ok(true)`
/// when it leaves through the block at 0x3a71fc (pending set, or hull done).
pub(crate) fn stream_man_update_active(
    c: &mut Cpu,
    this: u32,
) -> Result<bool, crate::call::Unwound> {
    use stream_callee::*;
    use stream_man::*;
    let at = |off: u32| Addr(this.wrapping_add(off));

    // Resume a parse that stopped mid-buffer (a0 is still `this`).
    c.cycles += 3;
    c.g[V0][0] = at(PENDING).lw(c);
    c.g[A1][0] = sx(this.wrapping_add(LEFT));
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[A2][0] = sx(this.wrapping_add(CURSOR));
        PARSE_CHUNKS.call(c, 0x003a_70d0)?;
        if c.g[V0][0] == 0 {
            return Ok(false);
        }
    }

    // Throttled: count the audio ticks since the last frame off the wait.
    c.cycles += 3;
    c.g[V0][0] = at(THROTTLED).lw(c);
    let throttled = c.g[V0][0] != 0;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_add(GP_CLOCK)).lw(c);
    if throttled {
        c.cycles += 4;
        c.g[A0][0] = at(LAST_TICK).lw(c);
        let clock = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(clock.wrapping_add(CLOCK_TICKS)).lw(c);
        let same = c.g[A0][0] == c.g[V1][0];
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        if !same {
            c.cycles += 6;
            let wait = at(WAIT).read_fpu(c);
            let ticks = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
            abi::set_fpr(c, 1, ticks);
            let now = c.g[V1][0];
            at(LAST_TICK).sw(c, now);
            let wait = wait - ticks;
            abi::set_fpr(c, 0, wait);
            at(WAIT).write_fpu(c, wait);
        }
        c.cycles += 6;
        let wait = at(WAIT).read_fpu(c);
        abi::set_fpr(c, 1, wait);
        abi::set_fpr(c, 0, FpuF32::ZERO);
        if abi::c_lt_s(c, FpuF32::ZERO, wait) {
            return Ok(false);
        }
    }

    // Take the chunks the stream file has.
    c.cycles += 5;
    c.g[A0][0] = at(FILE).lw(c);
    call_ptr(c, GET_CHUNK, 0x003a_712c)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[S2][0] = 1;
    if c.g[S1][0] != 0 {
        c.cycles += 2;
        let interval = at(INTERVAL).read_fpu(c);
        abi::set_fpr(c, 0, interval);
        loop {
            // One chunk: tag/size word at +4, payload at +8.
            let chunk = c.g[S1][0] as u32;
            let ch = |off: u32| Addr(chunk.wrapping_add(off));
            c.cycles += 28;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = at(BUFFER).lw(c);
            c.g[A3][0] = sx(chunk.wrapping_add(8));
            let interval = FpuF32(c.f[0]);
            at(WAIT).write_fpu(c, interval);
            let b7 = ch(7).lbu(c);
            c.g[V0][0] = sx((ch(6).lbu(c) as u32) << 16);
            c.g[V1][0] = sx((ch(5).lbu(c) as u32) << 8);
            c.g[T0][0] = ch(4).lbu(c);
            let size = sx((b7 as u32) << 24) | c.g[V0][0] | c.g[V1][0] | c.g[T0][0];
            c.g[A2][0] = sx((size as u32).wrapping_sub(8));
            DECOMPRESS.call(c, 0x003a_7180)?;
            c.g[A0][0] = at(FILE).lw(c);
            c.g[A1][0] = c.g[S1][0];
            let left = c.g[V0][0];
            at(LEFT).sw(c, left);
            call_ptr(c, RELEASE_CHUNK, 0x003a_7190)?;
            c.g[V0][0] = at(BUFFER).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(this.wrapping_add(LEFT));
            c.g[A2][0] = sx(this.wrapping_add(CURSOR));
            let buffer = c.g[V0][0];
            at(CURSOR).sw(c, buffer);
            PARSE_CHUNKS.call(c, 0x003a_71a8)?;
            if c.g[V0][0] == 0 {
                // Stalled: resume next frame.
                c.cycles += 2;
                let one = c.g[S2][0];
                at(PENDING).sw(c, one);
                return Ok(true);
            }
            c.g[V0][0] = at(THROTTLED).lw(c);
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                break;
            }
            c.cycles += 5;
            c.g[A0][0] = at(FILE).lw(c);
            call_ptr(c, GET_CHUNK, 0x003a_71c8)?;
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] == 0 {
                break;
            }
            let interval = at(INTERVAL).read_fpu(c);
            abi::set_fpr(c, 0, interval);
            if event_due(c) {
                deliver_event_here(c, 0x003a_7140);
            }
        }
    }

    // Everything read and parsed: the hull is resident.
    c.cycles += 4;
    c.g[A0][0] = at(FILE).lw(c);
    call_ptr(c, IS_IDLE, 0x003a_71dc)?;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    c.cycles += 6;
    c.g[A0][0] = at(CACHE).lw(c);
    c.g[A1][0] = at(HULL).lw(c);
    SET_HULL_RESIDENT.call(c, 0x003a_71f0)?;
    c.g[V0][0] = u64::MAX;
    at(ACTIVE).sw(c, 0);
    at(HULL).sw(c, u64::MAX);
    Ok(true)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::fepopup::tests::b;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS;
    pub(crate) use crate::game::main::gamemode::tests::be_library;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::object::animnode::tests::heap_block;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_2;
    use ssx3_ee as ee;

    #[test]
    fn cstream_man_set_rate_matches() {
        check(
            ssx3_ee::s13::f_003a7058,
            cstream_man_set_rate,
            200,
            &|c, seed| {
                let mut r = Rng(0x3a70_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match seed % 3 {
                    0 => (seed % 100) as u64,
                    1 => 0x8000,
                    _ => (r.next_u32() as u64) | ((r.next_u32() as u64) << 32),
                };
                c.bus.write32(GAME_GP + 0x2a74, DATA + 0x400);
                c.bus.write32(DATA + 0x410, r.next_u32());
            },
        );
    }

    #[test]
    fn stream_man_init_matches() {
        check(
            ssx3_ee::s13::f_003a6e98__generated,
            stream_man_init,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3a6e_98, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = 0x1234;
                let name = b"stream";
                for (i, &ch) in name.iter().enumerate() {
                    b(c, DATA + 0x100 + i as u32, ch);
                }
                b(c, DATA + 0x100 + name.len() as u32, 0);
            },
        );
    }

    #[test]
    fn c_stream_man_free_buffers_matches() {
        check(
            ee::s13::f_003a6f38__generated,
            c_stream_man_free_buffers,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x3a6f_38, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let file = DATA + 0x6000;
                clear(c, this, 0x100);
                clear(c, file, 0x40);
                // Not a stream-file magic, so `StreamFile_Destroy` returns at
                // once instead of waiting for the I/O thread.
                c.bus.write32(this + 0x88, file);
                let a = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                let b = if seed % 3 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                c.bus.write32(this + 0x84, a);
                c.bus.write32(this + 0x8c, b);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_stream_man_reset_matches() {
        check(
            ssx3_ee::s13::f_003a6ed8__generated,
            c_stream_man_reset,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x3a6ed8, 0);
                // The audio frame duration used by cStreamMan_SetRate.
                w_2(c, GAME_GP + 0x2a74, 50 + seed % 20);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 0x8000) as u64;
            },
        );
    }

    #[test]
    fn c_stream_man_dtor_matches() {
        check(
            ee::s13::f_003a6e20__generated,
            c_stream_man_dtor,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a6e20, 4);
                be_library(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0xcc, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
                w(c, obj + 0xc0, if seed % 3 == 0 { 0 } else { 1 });
                b(
                    c,
                    obj + 0xbc,
                    if seed % 4 == 0 {
                        0xff
                    } else {
                        r.next_u32() as u8 & 0xfe
                    },
                );
            },
        );
    }

    #[test]
    fn c_stream_man_queue_read_matches() {
        check(
            ee::s13::f_003a6f88__generated,
            c_stream_man_queue_read,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a6f88, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 0x1000) as u64;
                c.g[A2][0] = (seed % 0x800) as u64;
                c.g[A3][0] = 0x444e4543;
                c.bus.write32(DATA + 0x90, (seed % 2) as u32);
                c.bus.write32(DATA + 0x88, DATA + 0x1000);
            },
        );
    }

    /// cStreamMan_Update over its paths: inactive; a pending parse resumed
    /// (no resource, a resource allocated, or the allocation failing so the
    /// update stops); throttled waits (ticks counted down or still waiting);
    /// 0-3 chunks of junk (decompress to nothing) or of a RefPack stream
    /// holding one resource header (allocated, or failing so the update
    /// marks the parse pending); a throttled run stopping after one chunk;
    /// the stream file idle or not (and an invalid handle). The real
    /// StreamFile/ParseChunks/AllocResource callees run; their mutex kernel
    /// calls are fakes (some unwind).
    #[test]
    fn c_stream_man_update_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        check(
            ee::s13::f_003a7098__generated,
            c_stream_man_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_7098, 6);
                c.pc = 0x003a_7098;
                let this = DATA;
                let cache = DATA + 0x1000;
                let (table, res, refc) = (DATA + 0x1400, DATA + 0x1500, DATA + 0x1600);
                let (allocs, alloc, block) = (DATA + 0x1700, DATA + 0x1800, DATA + 0x1900);
                let (queue, queue_items) = (DATA + 0x1a00, DATA + 0x1b00);
                let (h, man, ring) = (DATA + 0x2000, DATA + 0x2100, DATA + 0x2800);
                let buffer = DATA + 0x3800;
                let clock = DATA + 0x3c00;
                for a in (this..this + 0x100).step_by(4) {
                    w(c, a, 0);
                }
                // The manager.
                w(c, this, cache);
                w(c, this + 0x88, if seed % 17 == 0 { 0 } else { h });
                w(c, this + 0x8c, buffer);
                w(c, this + 0x90, (seed % 11 != 0) as u32);
                let hull = r.next_u32() % 4;
                w(c, this + 0x94, hull);
                w(c, this + 0x98, r.next_u32() % 2);
                // A pending parse: an empty buffer and a resource of type 1 to
                // allocate (size 0: nothing to allocate).
                w(c, this + 0xac, (seed % 3 == 0) as u32);
                w(c, this + 0xb8, 1);
                w(c, this + 0xbc, 0);
                w(c, this + 0xc0, if seed % 2 == 0 { 0 } else { 0x40 });
                // Throttling: the audio clock and the wait.
                let throttled = seed % 5 == 1 || seed % 5 == 2;
                w(c, this + 0xd4, throttled as u32);
                w(
                    c,
                    this + 0xdc,
                    (1.0f32 + (r.next_u32() % 4) as f32).to_bits(),
                );
                let ticks = r.next_u32() % 8;
                w(c, GAME_GP + 0x2a74, clock);
                w(c, clock + 0x18, ticks);
                w(
                    c,
                    this + 0xe4,
                    if seed % 4 == 0 {
                        ticks
                    } else {
                        r.next_u32() % 8
                    },
                );
                w(c, this + 0xe0, ((r.next_u32() % 6) as f32 - 2.0).to_bits());
                // The world cache: resource table -> refcounts; allocator for
                // type 1 -> one block whose base 0 makes the allocation fail.
                w(c, cache + 8, table);
                w(c, table, res);
                w(c, res + 4 + 8, refc);
                for a in (refc..refc + 0x10).step_by(4) {
                    w(c, a, 0);
                }
                w(c, cache + 0xc, allocs);
                w(c, allocs + 0x3c, alloc);
                w(c, alloc + 8, 1);
                w(c, alloc + 0x10, block);
                let base = if seed % 7 == 3 { 0 } else { DATA + 0x5000 };
                w(c, block, base);
                w(c, block + 4, 0x1_0000);
                w(c, block + 8, 0);
                w(c, block + 0xc, 0);
                for i in 0..4 {
                    w(c, cache + 0x3f8 + 0x18 * i, queue);
                }
                w(c, queue + 8, 0);
                w(c, queue + 0xc, queue_items);
                // The stream file: handle -> manager ("STRM", mutex, ring).
                w(c, h, man);
                w(c, man, 0x4d52_5453);
                w(c, man + 8, r.next_u32() % 4);
                w(c, man + 0xc, r.next_u32() % 3);
                w(c, man + 0x10, r.next_u32() % 64);
                w(c, man + 0x30, ring);
                // Chunks lie past the ring end: ReleaseChunk leaves them alone.
                w(c, man + 0x34, ring + 4);
                w(c, man + 0x38, (seed % 6 == 5) as u32);
                let tag = 1 + r.next_u32() % 255;
                w(c, h + 4, tag);
                let k = r.next_u32() % 4;
                let mut at = ring;
                for _ in 0..k {
                    let size = 0x20;
                    w(c, at, 0);
                    w(c, at + 4, tag << 24 | size);
                    let p = at + 8;
                    if r.next_u32() % 2 == 0 {
                        // RefPack: 8 literal bytes = a resource header (type 1,
                        // size 0x40, id 0).
                        let stream: [u8; 14] =
                            [0x10, 0xfb, 0, 0, 8, 0xe1, 1, 0x40, 0, 0, 0, 0, 0, 0];
                        for (i, &b) in stream.iter().enumerate() {
                            c.bus.write8(p + i as u32, b);
                        }
                        c.bus.write8(p + 14, 0xfc);
                    } else {
                        w(c, p, r.next_u32());
                        c.bus.write8(p + 1, 0);
                    }
                    at += size;
                }
                w(c, h + 8, 0x20 * k);
                w(c, h + 0xc, ring);
                c.g[A0][0] = sx(this);
            },
        );
    }
}
