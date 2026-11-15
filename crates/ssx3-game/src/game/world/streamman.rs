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
pub(crate) use crate::game::ealib::seg_2dcae8::SFIR_MEM_COPY;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::festatetrophyroom::r;
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
    (0x003a_7218, cstream_man_decompress, "cStreamMan_Decompress"),
    (
        0x003a_7238,
        cstreamman_parse_chunks,
        "cStreamMan_ParseChunks",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::SPRINTF;

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

/// `EAC_Unpack` (0x3b47f8).
pub(crate) const OC327_EAC_UNPACK: Callee = Callee {
    addr: 0x003b_47f8,
    f: ee::s13::EAC_Unpack,
};

/// `void cStreamMan_Decompress(a0, a1, a2, a3)` (0x3a7218): decompress `a3`
/// through `EAC_Unpack`, returning its result.
pub fn cstream_man_decompress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003a_7228);
        c.g[A0][0] = c.g[A3][0];
        OC327_EAC_UNPACK.call(c, 0x003a_7228)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cStreamMan_ParseChunks
// ---------------------------------------------------------------------------

pub(crate) const SMPC_ALLOC_RESOURCE: Callee = Callee {
    addr: 0x003a_8a90,
    f: ee::s13::f_003a8a90,
};

pub(crate) const SMPC_RESOLVE_RESOURCE: Callee = Callee {
    addr: 0x003a_8cd0,
    f: ee::s13::f_003a8cd0,
};

/// `cStreamMan` chunk parser fields: `+0` the world cache, `+0x94` group,
/// `+0x98` chunks remaining, `+0xa0` bytes of a split header, `+0xa4` the
/// split header, `+0xac` retry the allocation, `+0xb8` id, `+0xbc` key,
/// `+0xc0` bytes left, `+0xc4` resolve an empty chunk, `+0xc8` copying,
/// `+0xcc` the allocation, `+0xd0` the copy destination.
pub(crate) const SMPC_REMAINING: u32 = 0x98;

pub(crate) const SMPC_PARTIAL: u32 = 0xa0;

pub(crate) const SMPC_RETRY: u32 = 0xac;

pub(crate) const SMPC_SIZE: u32 = 0xc0;

pub(crate) const SMPC_ENTRY: u32 = 0x003a_7238;

/// [`cstreamman_parse_chunks`]: `cWorldCache_AllocResource(cache, group,
/// id, key, size)` for the current chunk, its result to `+0xcc`; true
/// when it answered "later" (-1).
pub(crate) fn smpc_alloc(c: &mut Cpu, ret: u32) -> Result<bool, crate::call::Unwound> {
    let me = Addr(c.g[S0][0] as u32);
    c.g[A0][0] = me.lw(c);
    c.g[A1][0] = me.off(0x94).lw(c);
    c.g[A2][0] = me.off(0xb8).lw(c);
    c.g[A3][0] = me.off(0xbc).lw(c);
    c.g[T0][0] = me.off(SMPC_SIZE).lw(c);
    SMPC_ALLOC_RESOURCE.call(c, ret)?;
    let later = c.g[V0][0] == c.g[S4][0];
    let v0 = c.g[V0][0];
    me.off(0xcc).sw(c, v0);
    Ok(later)
}

/// [`cstreamman_parse_chunks`]: the header in the stack buffer (`sp`) to
/// id / key / size (`v0` its 8 bytes).
pub(crate) fn smpc_header(c: &mut Cpu) {
    let me = Addr(c.g[S0][0] as u32);
    let sp_m2 = c.g[SP][0] as u32;
    c.g[A0][0] = Addr(sp_m2).lbu(c);
    c.g[V1][0] = Addr(sp_m2 + 4).lw(c);
    c.g[V0][0] <<= 24;
    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
    c.g[V0][0] &= c.g[S5][0];
    let (a0, v1, v0) = (c.g[A0][0], c.g[V1][0], c.g[V0][0]);
    me.off(0xb8).sw(c, a0);
    me.off(0xbc).sw(c, v1);
    me.off(SMPC_SIZE).sw(c, v0);
}

/// `int cStreamMan_ParseChunks(cStreamMan* self, int* count, u8** data)`
/// (0x3a7238): consumes streamed bytes: 8-byte chunk headers (id, 24-bit
/// size, key; a header split across calls is kept at `+0xa4`), each
/// chunk's space from the world cache (0 if "later": the caller retries),
/// its bytes copied there (or skipped) and resolved when complete, until
/// no chunks remain; keeps a trailing partial header. Dispatcher-shaped:
/// its loops resume at their heads.
pub fn cstreamman_parse_chunks(c: &mut Cpu) {
    guest(c, |c| {
        let me = |c: &Cpu| Addr(c.g[S0][0] as u32);
        let count = |c: &Cpu| Addr(c.g[S2][0] as u32);
        let data = |c: &Cpu| Addr(c.g[S3][0] as u32);
        let sp_m2 = |c: &Cpu| c.g[SP][0] as u32;
        let mut pc = c.pc;
        loop {
            match pc {
                SMPC_ENTRY => {
                    c.cycles += 18;
                    let frame = Frame::open(c, 0x90);
                    frame.save(c, S4, 0x40);
                    frame.save(c, S5, 0x30);
                    frame.save(c, S0, 0x80);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, S2, 0x60);
                    c.g[S2][0] = c.g[A1][0];
                    frame.save(c, S3, 0x50);
                    c.g[S3][0] = c.g[A2][0];
                    frame.save(c, S6, 0x20);
                    frame.save(c, S1, 0x70);
                    c.g[S4][0] = u64::MAX;
                    frame.save_ra(c, 0x10);
                    c.g[S6][0] = 1;
                    c.g[S5][0] = 0xff_ffff;
                    c.cycles += 1;
                    c.g[V1][0] = count(c).lw(c);
                    pc = 0x003a_74f0;
                }
                0x003a_7280 => {
                    // Loop head: retry, a new header, or more bytes.
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = me(c).off(SMPC_SIZE).lw(c);
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.g[A2][0] = count(c).lw(c);
                            pc = 0x003a_7414;
                        } else {
                            pc = 0x003a_72c0;
                        }
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = me(c).lw(c);
                        pc = 0x003a_728c;
                    }
                }
                0x003a_728c => {
                    // Retry the allocation.
                    c.cycles += 7;
                    if smpc_alloc(c, 0x003a_72a0)? {
                        pc = 0x003a_73e8;
                        continue;
                    }
                    c.cycles += 2;
                    me(c).off(SMPC_RETRY).sw(c, 0);
                    if c.g[V0][0] != 0 {
                        pc = 0x003a_73f8;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = me(c).off(SMPC_SIZE).lw(c);
                        pc = 0x003a_7404;
                    }
                }
                0x003a_72c0 => {
                    // A new header: complete a split one or read all 8.
                    c.cycles += 3;
                    c.g[V0][0] = me(c).off(SMPC_PARTIAL).lw(c);
                    c.g[T0][0] = c.g[SP][0];
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        let s4 = c.g[S4][0];
                        Addr(sp_m2(c) + 4).sw(c, s4);
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A1][0] = 7;
                        pc = 0x003a_7378;
                        continue;
                    }
                    c.cycles += 3;
                    let s4 = c.g[S4][0];
                    Addr(sp_m2(c) + 4).sw(c, s4);
                    c.g[A2][0] = 0;
                    if (c.g[V0][0] as i64) <= 0 {
                        pc = 0x003a_72fc;
                    } else {
                        c.cycles += 2;
                        c.g[A3][0] = c.g[V0][0];
                        c.g[A1][0] = sx(me(c).0.wrapping_add(0xa4));
                        pc = 0x003a_72e0;
                    }
                }
                0x003a_72e0 => {
                    // The kept bytes of the split header.
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let v1 = c.g[V1][0];
                    Addr(c.g[A0][0] as u32).sb(c, v1);
                    if c.g[V0][0] != 0 {
                        if crate::events::back_edge_unwinds(c, 0x003a_72e0) {
                            return Ok(());
                        }
                    } else {
                        pc = 0x003a_72fc;
                    }
                }
                0x003a_72fc => {
                    c.cycles += 4;
                    c.g[A2][0] = me(c).off(SMPC_PARTIAL).lw(c);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 8) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = Addr(sp_m2(c)).ld(c);
                    if more {
                        c.cycles += 1;
                        pc = 0x003a_7310;
                    } else {
                        pc = 0x003a_7344;
                    }
                }
                0x003a_7310 => {
                    // The rest of the split header from the data.
                    c.cycles += 12;
                    c.g[V0][0] = data(c).lw(c);
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                    c.g[A1][0] = ((c.g[A2][0] as i64) < 8) as u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    let (v0, v1) = (c.g[V0][0], c.g[V1][0]);
                    data(c).sw(c, v0);
                    Addr(c.g[A0][0] as u32).sb(c, v1);
                    c.g[V0][0] = sx((count(c).lw(c) as u32).wrapping_sub(1));
                    let v0 = c.g[V0][0];
                    count(c).sw(c, v0);
                    if c.g[A1][0] != 0 {
                        if crate::events::back_edge_unwinds(c, 0x003a_7310) {
                            return Ok(());
                        }
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(sp_m2(c)).ld(c);
                        pc = 0x003a_7344;
                    }
                }
                0x003a_7344 => {
                    c.cycles += 10;
                    smpc_header(c);
                    me(c).off(SMPC_PARTIAL).sw(c, 0);
                    pc = 0x003a_73c8;
                }
                0x003a_7378 => {
                    // A whole header from the data.
                    c.cycles += 11;
                    c.g[V0][0] = data(c).lw(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    let (v0, v1) = (c.g[V0][0], c.g[V1][0]);
                    data(c).sw(c, v0);
                    Addr(c.g[A0][0] as u32).sb(c, v1);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = sx((count(c).lw(c) as u32).wrapping_sub(1));
                    let v0 = c.g[V0][0];
                    count(c).sw(c, v0);
                    if (c.g[A1][0] as i64) >= 0 {
                        if crate::events::back_edge_unwinds(c, 0x003a_7378) {
                            return Ok(());
                        }
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = Addr(sp_m2(c)).ld(c);
                        smpc_header(c);
                        pc = 0x003a_73c8;
                    }
                }
                0x003a_73c8 => {
                    c.cycles += 8;
                    if smpc_alloc(c, 0x003a_73e0)? {
                        pc = 0x003a_73e8;
                        continue;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = me(c).off(SMPC_SIZE).lw(c);
                        pc = 0x003a_7404;
                    } else {
                        pc = 0x003a_73f8;
                    }
                }
                0x003a_73e8 => {
                    // "Later": stop, the caller retries.
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    pc = 0x003a_756c;
                }
                0x003a_73f8 => {
                    // Space: copy into it.
                    c.cycles += 3;
                    let (v0, s6) = (c.g[V0][0], c.g[S6][0]);
                    me(c).off(0xd0).sw(c, v0);
                    me(c).off(0xc8).sw(c, s6);
                    c.cycles += 1;
                    c.g[A2][0] = count(c).lw(c);
                    pc = 0x003a_7414;
                }
                0x003a_7404 => {
                    // No space: skip it (an empty chunk still resolves).
                    c.cycles += 2;
                    let size = c.g[V0][0];
                    me(c).off(0xc8).sw(c, 0);
                    if size == 0 {
                        c.cycles += 1;
                        let s6 = c.g[S6][0];
                        me(c).off(0xc4).sw(c, s6);
                    }
                    c.cycles += 1;
                    c.g[A2][0] = count(c).lw(c);
                    pc = 0x003a_7414;
                }
                0x003a_7414 => {
                    // The chunk's bytes in this data.
                    c.cycles += 2;
                    c.g[S1][0] = c.g[A2][0];
                    if c.g[A2][0] == 0 {
                        pc = 0x003a_74d8;
                        continue;
                    }
                    c.cycles += 7;
                    c.g[V0][0] = me(c).off(SMPC_SIZE).lw(c);
                    c.g[A0][0] = me(c).off(0xc8).lw(c);
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[S1][0] = c.g[V0][0];
                    }
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    let v0 = c.g[V0][0];
                    me(c).off(SMPC_SIZE).sw(c, v0);
                    if c.g[A0][0] != 0 {
                        c.cycles += 9;
                        c.g[A0][0] = me(c).off(0xd0).lw(c);
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A1][0] = data(c).lw(c);
                        call_ptr(c, SFIR_MEM_COPY.addr, 0x003a_7448)?;
                        c.g[V0][0] = me(c).off(0xd0).lw(c);
                        c.g[V1][0] = me(c).off(SMPC_SIZE).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        let v0 = c.g[V0][0];
                        me(c).off(0xd0).sw(c, v0);
                        if c.g[V1][0] == 0 {
                            // Complete: resolve with the allocation.
                            c.cycles += 11;
                            let m = me(c);
                            c.g[A0][0] = m.lw(c);
                            c.g[A1][0] = m.off(0x94).lw(c);
                            c.g[A2][0] = m.off(0xb8).lw(c);
                            c.g[A3][0] = m.off(0xbc).lw(c);
                            c.g[T0][0] = m.off(0xcc).lw(c);
                            SMPC_RESOLVE_RESOURCE.call(c, 0x003a_7474)?;
                            let m = me(c);
                            c.g[V0][0] = m.off(SMPC_REMAINING).lw(c);
                            m.off(0xcc).sw(c, 0);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            let v0 = c.g[V0][0];
                            m.off(SMPC_REMAINING).sw(c, v0);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = data(c).lw(c);
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = data(c).lw(c);
                        } else {
                            // A skipped chunk ends: resolve an empty one.
                            c.cycles += 5;
                            let m = me(c);
                            c.g[V0][0] = m.off(SMPC_REMAINING).lw(c);
                            c.g[V1][0] = m.off(0xc4).lw(c);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            let v0 = c.g[V0][0];
                            m.off(SMPC_REMAINING).sw(c, v0);
                            if c.g[V1][0] != 0 {
                                c.cycles += 7;
                                c.g[A0][0] = m.lw(c);
                                c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x16e0));
                                c.g[A1][0] = m.off(0x94).lw(c);
                                c.g[A2][0] = m.off(0xb8).lw(c);
                                c.g[A3][0] = m.off(0xbc).lw(c);
                                SMPC_RESOLVE_RESOURCE.call(c, 0x003a_74bc)?;
                                me(c).off(0xc4).sw(c, 0);
                            }
                            c.cycles += 1;
                            c.g[V0][0] = data(c).lw(c);
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    let v0 = c.g[V0][0];
                    data(c).sw(c, v0);
                    c.g[V1][0] = sx((count(c).lw(c) as u32).wrapping_sub(c.g[S1][0] as u32));
                    let v1 = c.g[V1][0];
                    count(c).sw(c, v1);
                    pc = 0x003a_74d8;
                }
                0x003a_74d8 => {
                    // All chunks done: drop the rest.
                    c.cycles += 3;
                    c.g[V0][0] = me(c).off(SMPC_REMAINING).lw(c);
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = count(c).lw(c);
                        pc = 0x003a_74f0;
                    } else {
                        c.cycles += 2;
                        count(c).sw(c, 0);
                        pc = 0x003a_7518;
                    }
                }
                0x003a_74f0 => {
                    // Go on while a header fits, a chunk continues or a
                    // retry is due.
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 8) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = me(c).off(SMPC_RETRY).lw(c);
                        if crate::events::back_edge_unwinds(c, 0x003a_7280) {
                            return Ok(());
                        }
                        pc = 0x003a_7280;
                        continue;
                    }
                    c.cycles += 2;
                    if (c.g[V1][0] as i64) > 0 {
                        c.cycles += 3;
                        c.g[V0][0] = me(c).off(SMPC_SIZE).lw(c);
                        let going = c.g[V0][0] != 0;
                        c.g[V0][0] = me(c).off(SMPC_RETRY).lw(c);
                        if going {
                            if crate::events::back_edge_unwinds(c, 0x003a_7280) {
                                return Ok(());
                            }
                            pc = 0x003a_7280;
                            continue;
                        }
                    } else {
                        c.g[V0][0] = me(c).off(SMPC_RETRY).lw(c);
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = me(c).lw(c);
                        if crate::events::back_edge_unwinds(c, 0x003a_728c) {
                            return Ok(());
                        }
                        pc = 0x003a_728c;
                    } else {
                        pc = 0x003a_7518;
                    }
                }
                0x003a_7518 => {
                    // Keep a trailing partial header.
                    c.cycles += 3;
                    c.g[V0][0] = count(c).lw(c);
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = 1;
                        pc = 0x003a_756c;
                        continue;
                    }
                    c.cycles += 4;
                    let v0 = c.g[V0][0];
                    me(c).off(SMPC_PARTIAL).sw(c, v0);
                    c.g[V0][0] = count(c).lw(c);
                    c.g[A1][0] = 0;
                    if (c.g[V0][0] as i64) <= 0 {
                        pc = 0x003a_7568;
                    } else {
                        c.cycles += 3;
                        c.g[A2][0] = sx(me(c).0.wrapping_add(0xa4));
                        c.g[V1][0] = data(c).lw(c);
                        pc = 0x003a_7540;
                    }
                }
                0x003a_7540 => {
                    c.cycles += 10;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lbu(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    let (v0, v1) = (c.g[V0][0], c.g[V1][0]);
                    Addr(c.g[A0][0] as u32).sb(c, v0);
                    data(c).sw(c, v1);
                    c.g[V0][0] = count(c).lw(c);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = data(c).lw(c);
                        if crate::events::back_edge_unwinds(c, 0x003a_7540) {
                            return Ok(());
                        }
                    } else {
                        pc = 0x003a_7568;
                    }
                }
                0x003a_7568 => {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    pc = 0x003a_756c;
                }
                0x003a_756c => {
                    c.cycles += 10;
                    let frame = Frame { size: 0x90 };
                    for (r, off) in [
                        (S0, 0x80),
                        (S1, 0x70),
                        (S2, 0x60),
                        (S3, 0x50),
                        (S4, 0x40),
                        (S5, 0x30),
                        (S6, 0x20),
                    ] {
                        frame.restore(c, r, off);
                    }
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, SMPC_ENTRY, 0x003a_7594);
                    return Ok(());
                }
            }
        }
    })
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
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS;
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

    use crate::difftest::fakes::covered;

    pub(crate) use crate::game::ealib::seg_2b4578::tests::oc327_install_refpack;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::OC327_CTRL;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::OC327_MAGIC;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::OC327_RET;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::RET_REFPACK;

    /// `cStreamMan_Decompress`: moves `a3` to `a0` and calls `EAC_Unpack`.
    #[test]
    fn cstream_man_decompress_matches() {
        oc327_install_refpack();
        covered(
            ee::s13::f_003a7218__generated,
            cstream_man_decompress,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x003a_7218, 1);
                c.pc = 0x003a_7218;
                c.bus.write32(OC327_CTRL, OC327_MAGIC);
                c.bus.write32(OC327_RET + 4 * RET_REFPACK, 0x2000 + seed);
                c.g[A0][0] = sx(DATA + 0x200);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x600);
                c.g[A3][0] = sx(DATA + 0x800);
            },
        );
    }

    use crate::difftest::fill;

    pub(crate) use crate::game::sound::soundsys::tests::oc371_machine;
    pub(crate) use crate::game::sound::soundsys::tests::oc371_script;

    pub(crate) use crate::game::sound::soundsys::tests::oc371_install_fakes;

    /// A stream of chunks (sizes 0-20) in 0-40 bytes: a split header kept
    /// from before (0-7 bytes) or none, a chunk in progress (copying or
    /// skipping, empty ones resolving) or a pending retry; the cache
    /// answering "later", no space or space; chunks running out; a
    /// trailing partial header kept. The real `EA_MemCopy` copies; the
    /// cache callees are logged fakes (one may unwind).
    #[test]
    fn cstreamman_parse_chunks_matches() {
        oc371_install_fakes();
        covered(
            ee::s13::f_003a7238__generated,
            cstreamman_parse_chunks,
            500,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x003a_7238, 6);
                c.pc = 0x003a_7238;
                let me = DATA;
                let stream = DATA + 0x1000;
                fill(c, &mut r, me, 0x38);
                c.bus.write32(me + SMPC_REMAINING, 1 + r.next_u32() % 4);
                c.bus.write32(
                    me + SMPC_PARTIAL,
                    if r.next_u32() % 3 == 0 {
                        r.next_u32() % 8
                    } else {
                        0
                    },
                );
                c.bus
                    .write32(me + SMPC_RETRY, (r.next_u32() % 5 == 0) as u32);
                c.bus.write32(
                    me + SMPC_SIZE,
                    if r.next_u32() % 3 == 0 {
                        r.next_u32() % 16
                    } else {
                        0
                    },
                );
                c.bus.write32(me + 0xc4, r.next_u32() % 2);
                c.bus.write32(me + 0xc8, r.next_u32() % 2);
                c.bus.write32(me + 0xd0, DATA + 0x8000);
                // The stream: headers (id, 24-bit size, key) and their bytes.
                let mut at = stream;
                while at < stream + 0x40 {
                    let size = r.next_u32() % 21;
                    c.bus.write8(at, r.next_u32() as u8);
                    c.bus.write8(at + 1, size as u8);
                    c.bus.write16(at + 2, 0);
                    c.bus.write32(at + 4, r.next_u32());
                    fill(c, &mut r, at + 8, (size + 3) / 4);
                    at += 8 + size;
                }
                c.bus.write32(DATA + 0x100, r.next_u32() % 41);
                c.bus.write32(DATA + 0x104, stream);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x104);
                let space = match r.next_u32() % 4 {
                    0 => u32::MAX,
                    1 => 0,
                    _ => DATA + 0x9000,
                };
                oc371_script(c, 0x003a_8a90, space);
            },
        );
    }
}
