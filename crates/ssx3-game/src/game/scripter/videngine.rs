//! `src/scripter/videngine.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
pub(crate) use crate::game::dirtysock::tagsunk::arg;
pub(crate) use crate::game::scripter::ssxscriptengine::oc428_at;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_3b78,
        script_movie_set_paused,
        "ScriptMovie_SetPaused",
    ),
    (0x0028_3bb8, script_movie_get_id, "ScriptMovie_GetId"),
    (0x0028_3c20, script_cmd_queue_clear, "ScriptCmdQueue_Clear"),
    (0x0028_3c30, script_cmd_queue_ctor, "ScriptCmdQueue_ctor"),
    (
        0x0028_3c60,
        script_cmd_queue_next_index,
        "ScriptCmdQueue_NextIndex",
    ),
    (0x0028_3d28, script_cmd_queue_count, "ScriptCmdQueue_Count"),
    (0x0028_3d70, script_cmd_queue_at, "ScriptCmdQueue_At"),
    (
        0x0028_3da0,
        script_cmd_queue_is_empty,
        "ScriptCmdQueue_IsEmpty",
    ),
    (0x0028_3dc0, script_cmd_queue_pop, "ScriptCmdQueue_Pop"),
    (
        0x0028_3af8,
        script_movie_toggle_pause,
        "ScriptMovie_TogglePause",
    ),
    (0x0028_3aa0, script_movie_update, "ScriptMovie_Update"),
    (
        0x0028_3c58,
        script_cmd_queue_is_full,
        "ScriptCmdQueue_IsFull",
    ),
    (0x0028_3c80, script_cmd_queue_push_2, "ScriptCmdQueue_Push"),
    (0x0028_39a8, script_movie_start, "ScriptMovie_Start"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SCRIPT_CMD_QUEUE_CLEAR;

    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_IS_FULL;
}

// ---------------------------------------------------------------------------
// Script command queue (a 4-entry ring of 28-byte entries).

pub(crate) const CMD_READ: u32 = 0x8c;

pub(crate) const CMD_WRITE: u32 = 0x90;

pub(crate) const CMD_FULL: u32 = 0x94;

pub(crate) const CMD_ENTRY_SIZE: u32 = 0x1c;

pub(crate) const CMD_RING_SIZE: u64 = 4;

/// `void ScriptCmdQueue_Clear(ScriptCmdQueue *q)` (0x283c20): reset the read
/// and write cursors and clear the full flag.
pub fn script_cmd_queue_clear(c: &mut Cpu) {
    c.cycles += 4;
    let q = Addr(abi::arg(c, 0));
    q.off(CMD_FULL).sw(c, 0);
    q.off(CMD_READ).sw(c, 0);
    q.off(CMD_WRITE).sw(c, 0);
    abi::ret(c);
}

/// `int ScriptCmdQueue_NextIndex(int index)` (0x283c60): `index + 1`, wrapping
/// back to 0 after the last slot (index 4).
pub fn script_cmd_queue_next_index(c: &mut Cpu) {
    c.cycles += 3;
    let index = c.g[A1][0];
    let next = sx((index as u32).wrapping_add(1));
    c.cycles += 2;
    c.g[V0][0] = if index == CMD_RING_SIZE { 0 } else { next };
    abi::ret(c);
}

/// `ScriptCmdQueueEntry *ScriptCmdQueue_At(ScriptCmdQueue *q, int index)`
/// (0x283d70): address of the ring element at `read + index` (mod 4), the
/// entries being 28 bytes each.
pub fn script_cmd_queue_at(c: &mut Cpu) {
    c.cycles += 5;
    let q = Addr(abi::arg(c, 0));
    let read = q.off(CMD_READ).lw(c) as u32;
    let at = read.wrapping_add(abi::arg(c, 1));
    let rem = abi::div(c, at, 5).1;
    let slot = abi::mult(c, rem, CMD_ENTRY_SIZE);
    c.cycles += 6;
    c.g[A2][0] = 5;
    c.g[A1][0] = sx(rem);
    c.g[V1][0] = sx(slot);
    c.g[V0][0] = sx(slot.wrapping_add(abi::arg(c, 0)));
    abi::ret(c);
}

/// `bool ScriptCmdQueue_IsEmpty(ScriptCmdQueue *q)` (0x283da0): true when the
/// read cursor still equals the write cursor and the queue is not marked full.
pub fn script_cmd_queue_is_empty(c: &mut Cpu) {
    c.cycles += 4;
    let q = Addr(abi::arg(c, 0));
    let write = q.off(CMD_WRITE).lw(c);
    let read = q.off(CMD_READ).lw(c);
    c.g[V1][0] = write;
    let mut empty = 0u64;
    if write == read {
        c.cycles += 2;
        empty = (q.off(CMD_FULL).lw(c) < 1) as u64;
    }
    c.cycles += 2;
    c.g[A1][0] = empty;
    c.g[V0][0] = empty;
    abi::ret(c);
}

/// `void ScriptCmdQueue_ctor(ScriptCmdQueue* self)` (0x283c30): clear the
/// queue and return it.
pub fn script_cmd_queue_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_CMD_QUEUE_CLEAR.call(c, 0x0028_3c44)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Movies.

/// `int ScriptMovie_GetId(ScriptMovie* self)` (0x283bb8): the id of the
/// loaded movie, or 0 when it has no data.
pub fn script_movie_get_id(c: &mut Cpu) {
    'outer: {
        c.cycles += 3;
        c.g[V1][0] = Addr(arg(c, 0)).lw(c);
        if c.g[V1][0] == 0 {
            break 'outer;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(arg(c, 0).wrapping_add(0x4)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            abi::ret(c);
            return;
        }
        break 'outer;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void ScriptMovie_SetPaused(Movie* m, int inc /*a1*/)` (0x283b78): count
/// pause requests (`+0xc`) up or down, saturating at zero, once the movie
/// (`+0`) and its stream (`+4`) exist.
pub fn script_movie_set_paused(c: &mut Cpu) {
    c.cycles += 3;
    let m = abi::arg(c, 0);
    c.g[V0][0] = Addr(m).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(m).off(4).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let dec = c.g[A1][0] == 0;
    c.g[V0][0] = Addr(m).off(0xc).lw(c);
    if dec {
        c.cycles += 2;
        let zero = c.g[V0][0] == 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        if zero {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        c.cycles += 1;
        c.sw(p(m, 0xc), c.g[V0][0]);
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    c.sw(p(m, 0xc), c.g[V0][0]);
    abi::ret(c);
}

/// `int ScriptCmdQueue_Count(self)` (0x283d28): the number of queued script
/// commands (0x8c = write cursor, 0x90 = read cursor, wrapped at 5), or 5
/// when the queue is full.
pub fn script_cmd_queue_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_CMD_QUEUE_IS_FULL.call(c, 0x0028_3d3c)?;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 5;
        } else {
            let self_ = abi::r32(c, S0);
            c.cycles += 7;
            c.g[V1][0] = Addr(self_.wrapping_add(0x8c)).lw(c);
            c.g[A0][0] = u64::MAX;
            c.g[V0][0] = Addr(self_.wrapping_add(0x90)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(5));
            if c.g[A0][0] == 0 {
                c.g[V0][0] = c.g[V1][0];
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script command queue.

/// `int ScriptCmdQueue_Pop(ScriptCmdQueue* self, ScriptCmd* out)` (0x283dc0):
/// pop the record at `+0x8c` into `out`, advance the ring index and return 1;
/// 0 when the queue is empty.
pub fn script_cmd_queue_pop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0028_3da0, 0x0028_3ddc)?; // IsEmpty
        let empty = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            let no_out = c.g[S1][0] == 0;
            sw(c, p(c.g[S0][0] as u32, 0x94), 0);
            if !no_out {
                c.cycles += 18;
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x8c));
                c.g[V1][0] = 0x1c;
                c.mult(0, 4, 2, 3);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.ldl(5, p(c.g[V0][0] as u32, 7));
                c.ldr(5, c.g[V0][0] as u32);
                c.ldl(6, p(c.g[V0][0] as u32, 0xf));
                c.ldr(6, p(c.g[V0][0] as u32, 8));
                c.ldl(3, p(c.g[V0][0] as u32, 0x17));
                c.ldr(3, p(c.g[V0][0] as u32, 0x10));
                c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x18));
                c.sdl(5, p(c.g[S1][0] as u32, 7));
                c.sdr(5, c.g[S1][0] as u32);
                c.sdl(6, p(c.g[S1][0] as u32, 0xf));
                c.sdr(6, p(c.g[S1][0] as u32, 8));
                c.sdl(3, p(c.g[S1][0] as u32, 0x17));
                c.sdr(3, p(c.g[S1][0] as u32, 0x10));
                sw(c, p(c.g[S1][0] as u32, 0x18), c.g[A0][0]);
            }
            c.cycles += 5;
            c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0x8c));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0028_3c60, 0x0028_3e40)?; // NextIndex
            sw(c, p(c.g[S0][0] as u32, 0x8c), c.g[V0][0]);
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int ScriptCmdQueue_Push(self, const Cmd* cmd)` (0x283c80): append the
/// command to the ring and flag the queue full when the index wraps to the
/// tail.
pub fn script_cmd_queue_push(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0028_3c58, 0x00283c9c)?;
        let full = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !full {
            c.cycles += 24;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x90)).lw(c);
            c.g[V1][0] = 0x1c;
            c.g[A0][0] = c.g[S0][0];
            c.mult(0, 5, 2, 3);
            c.g[V0][0] = sx(p(r32(c, A1), r32(c, S0)));
            c.ldl(6, p(r32(c, S1), 7));
            c.ldr(6, r32(c, S1));
            c.ldl(7, p(r32(c, S1), 0xf));
            c.ldr(7, p(r32(c, S1), 8));
            c.ldl(3, p(r32(c, S1), 0x17));
            c.ldr(3, p(r32(c, S1), 0x10));
            c.g[A1][0] = Addr(p(r32(c, S1), 0x18)).lw(c);
            c.sdl(6, p(r32(c, V0), 7));
            c.sdr(6, r32(c, V0));
            c.sdl(7, p(r32(c, V0), 0xf));
            c.sdr(7, p(r32(c, V0), 8));
            c.sdl(3, p(r32(c, V0), 0x17));
            c.sdr(3, p(r32(c, V0), 0x10));
            Addr(p(r32(c, V0), 0x18)).sw(c, c.g[A1][0]);
            c.g[A1][0] = Addr(p(r32(c, S0), 0x90)).lw(c);
            call_ptr(c, 0x0028_3c60, 0x00283cf8)?;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x8c)).lw(c);
            let wrapped = c.g[V0][0] != c.g[V1][0];
            Addr(p(r32(c, S0), 0x90)).sw(c, c.g[V0][0]);
            if !wrapped {
                c.cycles += 2;
                c.g[V0][0] = 1;
                Addr(p(r32(c, S0), 0x94)).sw(c, c.g[V0][0]);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `ScriptMovie_Open` (0x002838e8).
//
// `int ScriptMovie_Open(ScriptMovie* self, int file)`: if a movie is already
// open, report 0; otherwise allocate a video object, set up its parameters and
// start playback (0 on failure, 1 on success).

pub fn script_movie_open(c: &mut Cpu) {
    guest(c, |c| {
        'b_283990: {
            'b_283910: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S0][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                let t = c.g[V0][0] == 0u64;
                c.g[S1][0] = c.g[A1][0];
                if t {
                    break 'b_283910;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_283990;
                    } else {
                        break 'b_283910;
                    }
                }
            }
            c.cycles += 25;
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = 0x100;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1fc8));
            c.g[A3][0] = 0u64;
            c.g[RA][0] = sx(0x00283928);
            c.g[A0][0] = 0x44;
            call_ptr(c, 0x00317d70, 0x00283928)?;
            c.g[RA][0] = sx(0x00283930);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x002533c8, 0x00283930)?;
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[RA][0] = sx(0x0028393c);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x00253390, 0x0028393c)?;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = 0x1;
            c.g[V1][0] = 0x100;
            c.g[A2][0] = 0x9;
            c.sw(c.g[SP][0] as u32, c.g[S1][0]);
            c.g[A1][0] = c.g[SP][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
            c.g[RA][0] = sx(0x0028396c);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
            call_ptr(c, 0x002534a8, 0x0028396c)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_283990;
            } else {
                'b_28398c: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    if c.g[A0][0] == 0u64 {
                        c.sw(c.g[S0][0] as u32, 0u64);
                        break 'b_28398c;
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x00283988);
                        c.g[A1][0] = 0x3;
                        call_ptr(c, 0x00253418, 0x00283988)?;
                        c.sw(c.g[S0][0] as u32, 0u64);
                        break 'b_28398c;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_283990;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    })
}

pub(crate) mod script_movie {

    /// ScriptMovie: the player, playing flag, pause request.
    pub const PLAYER: u32 = 0x0;

    pub const PLAYING: u32 = 0x4;

    pub const PAUSE_REQ: u32 = 0x8;

    /// VidEnginePlayer: paused flag.
    pub const PLAYER_PAUSED: u32 = 0x8;

    /// `VidEnginePlayer_Resume`, `_Pause`, `_Render(player, paused)`.
    pub const RESUME: u32 = 0x0025_3a40;

    pub const PAUSE: u32 = 0x0025_39e0;

    pub const RENDER: u32 = 0x0025_3890;
}

/// `void ScriptMovie_TogglePause(ScriptMovie *self)` (0x283af8): for a
/// playing movie, apply the pause request: resume a paused player when one is
/// pending, else pause a running one; then render the player with the
/// request's opposite and clear the request.
pub fn script_movie_toggle_pause(c: &mut Cpu) {
    use script_movie::*;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let movie = r32(c, S0);
        c.g[A0][0] = Addr(movie + PLAYER).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(movie + PLAYING).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                let request = Addr(movie + PAUSE_REQ).lw(c) != 0;
                let player = r32(c, A0);
                c.g[V0][0] = Addr(player + PLAYER_PAUSED).lw(c);
                c.cycles += 2;
                let reload = if request {
                    if c.g[V0][0] == 0 {
                        false
                    } else {
                        c.cycles += 4;
                        call_ptr(c, RESUME, 0x0028_3b3c)?;
                        true
                    }
                } else {
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        call_ptr(c, PAUSE, 0x0028_3b54)?;
                    }
                    true
                };
                let movie = r32(c, S0);
                c.g[A1][0] = Addr(movie + PAUSE_REQ).lw(c);
                if reload {
                    c.cycles += 1;
                    c.g[A0][0] = Addr(movie + PLAYER).lw(c);
                }
                c.cycles += 4;
                c.g[A1][0] ^= 1;
                call_ptr(c, RENDER, 0x0028_3b64)?;
                Addr(r32(c, S0) + PAUSE_REQ).sw(c, 0);
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ScriptMovie_Update (0x283aa0)

/// Script movie: the player (+0), "started" (+4), "updated" (+8)
/// and "finished" (+0xc).
pub(crate) const SCRIPT_MOVIE_PLAYER: u32 = 0x0;

pub(crate) const SCRIPT_MOVIE_STARTED: u32 = 0x4;

pub(crate) const SCRIPT_MOVIE_UPDATED: u32 = 0x8;

pub(crate) const SCRIPT_MOVIE_DONE: u32 = 0xc;

/// `VidEnginePlayer_Update` (0x253938, through the function table).
pub(crate) const VID_ENGINE_PLAYER_UPDATE: u32 = 0x0025_3938;

/// `void ScriptMovie_Update(ScriptMovie *movie)` (0x283aa0): while a
/// script-driven movie has a player, has started and is not done, updates
/// its video player and sets +8. Frame 0x20 (ra at 0, s0 at 0x10).
pub fn script_movie_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let movie = Addr(r32(c, S0));
        c.g[A0][0] = movie.off(SCRIPT_MOVIE_PLAYER).lw(c);
        'out: {
            if c.g[A0][0] == 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = movie.off(SCRIPT_MOVIE_STARTED).lw(c);
            if c.g[V0][0] == 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = movie.off(SCRIPT_MOVIE_DONE).lw(c);
            if c.g[V0][0] != 0 {
                break 'out;
            }
            c.cycles += 5;
            call_ptr(c, VID_ENGINE_PLAYER_UPDATE, 0x0028_3adc)?;
            c.g[V0][0] = 1;
            Addr(r32(c, S0)).off(SCRIPT_MOVIE_UPDATED).sw(c, 1);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptCmdQueue` full-flag offset.
pub(crate) const SCRIPT_CMD_QUEUE_FULL: u32 = 0x94;

/// `int ScriptCmdQueue_IsFull(ScriptCmdQueue* self)` (0x283c58): the word at
/// `+0x94`.
pub fn script_cmd_queue_is_full(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw(oc428_at(c.g[A0][0], SCRIPT_CMD_QUEUE_FULL));
    ret(c);
}

/// `bool ScriptCmdQueue_Push(self, const Cmd* cmd)` (0x283c80): pushes a
/// 0x1c-byte script command into the ring buffer (entries at `self + i *
/// 0x1c`, 4 slots, head at +0x90, tail at +0x8c, "full" flag at +0x94).
/// Returns 0 when full; otherwise copies the command to the head slot,
/// advances the head (wrapping 4 -> 0) and sets the full flag when it reaches
/// the tail; returns 1. Frame 0x30 (s0, s1).
pub fn script_cmd_queue_push_2(c: &mut Cpu) {
    const IS_FULL: Callee = Callee {
        addr: 0x0028_3c58,
        f: ee::s07::f_00283c58,
    };
    const NEXT_INDEX: Callee = Callee {
        addr: 0x0028_3c60,
        f: ee::s07::f_00283c60,
    };
    const TAIL: u32 = 0x8c;
    const HEAD: u32 = 0x90;
    const FULL: u32 = 0x94;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        IS_FULL.call(c, 0x0028_3c9c)?;
        let full = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !full {
            c.cycles += 24;
            let this = r32(c, S0);
            let cmd = r32(c, S1);
            c.g[V0][0] = c.lw(this.wrapping_add(HEAD));
            c.g[V1][0] = 0x1c;
            c.g[A0][0] = c.g[S0][0];
            c.mult(0, A1, V0, V1);
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(this));
            let slot = r32(c, V0);
            c.ldl(A2, cmd.wrapping_add(0x7));
            c.ldr(A2, cmd);
            c.ldl(A3, cmd.wrapping_add(0xf));
            c.ldr(A3, cmd.wrapping_add(0x8));
            c.ldl(V1, cmd.wrapping_add(0x17));
            c.ldr(V1, cmd.wrapping_add(0x10));
            c.g[A1][0] = c.lw(cmd.wrapping_add(0x18));
            c.sdl(A2, slot.wrapping_add(0x7));
            c.sdr(A2, slot);
            c.sdl(A3, slot.wrapping_add(0xf));
            c.sdr(A3, slot.wrapping_add(0x8));
            c.sdl(V1, slot.wrapping_add(0x17));
            c.sdr(V1, slot.wrapping_add(0x10));
            c.sw(slot.wrapping_add(0x18), c.g[A1][0]);
            c.g[A1][0] = c.lw(this.wrapping_add(HEAD));
            NEXT_INDEX.call(c, 0x0028_3cf8)?;
            c.g[V1][0] = c.lw(this.wrapping_add(TAIL));
            let not_full_now = c.g[V0][0] != c.g[V1][0];
            c.sw(this.wrapping_add(HEAD), c.g[V0][0]);
            if !not_full_now {
                c.cycles += 2;
                c.g[V0][0] = 1;
                c.sw(this.wrapping_add(FULL), c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptMovie_Start` (0x2839a8, `cVidEngine_PlayVideo`): with no movie
/// player returns 0, and 1 when one is already playing. Otherwise it starts
/// the player, pushes the pause state (10) on the pause stack, saves the TV
/// display format, moves the cut-scene manager to regular mode (remembering
/// whether it had to) and pauses the SFX. Returns 1.
pub fn script_movie_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let movie = c.g[S0][0] as u32;
        c.g[A0][0] = Addr(movie).lw(c);
        c.g[V0][0] = 0;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(movie).off(0x4).lw(c);
            let playing = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if !playing {
                start_movie(c)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The starting half of [`script_movie_start`]: the player is in `a0` and
/// `s0` is the movie object.
pub(crate) fn start_movie(c: &mut Cpu) -> Flow {
    let movie = c.g[S0][0] as u32;
    let gp = c.g[GP][0] as u32;
    c.cycles += 23;
    call_ptr(c, 0x0025_3860, 0x0028_39d8)?;
    c.g[A1][0] = Addr(gp.wrapping_sub(0x69c)).lw(c);
    c.g[V1][0] = 0x530000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
    c.g[A3][0] = 1;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
    let playing_flag = c.g[A3][0];
    Addr(movie).off(0x4).sw(c, playing_flag);
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A2][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = 0xa;
    let stack_slot = c.g[V0][0] as u32;
    let pause_state = c.g[A0][0];
    Addr(stack_slot).sw(c, pause_state);
    c.g[V1][0] = 0x440000;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2918)).lw(c);
    c.g[A0][0] = Addr(c.g[A2][0] as u32).off(0x84).lw(c);
    let saved_flags = c.g[V0][0];
    Addr(gp.wrapping_sub(0x6a0)).sw(c, saved_flags);
    c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x64).lw(c);
    let stack_index = c.g[A1][0];
    Addr(gp.wrapping_sub(0x69c)).sw(c, stack_index);
    Addr(movie).off(0x14).sw(c, 0);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x10).lw(c);
    let state_zero = c.g[V1][0] == 0;
    c.g[V0][0] = 2;
    if !state_zero {
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            Addr(movie).off(0x14).sw(c, playing_flag);
        }
    }
    c.cycles += 22;
    call_ptr(c, 0x002e_a820, 0x0028_3a3c)?;
    c.g[A1][0] = Addr(gp.wrapping_sub(0x854)).lw(c);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x10d8).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x150).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x154).lw(c);
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, 0x0028_3a54)?;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[A1][0] = 0;
    c.g[A2][0] = Addr(gp.wrapping_sub(0x854)).lw(c);
    let display_format = c.g[V1][0];
    Addr(movie).off(0x10).sw(c, display_format);
    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x10d8).lw(c);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x140).lh(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x144).lw(c);
    let set_method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, set_method, 0x0028_3a78)?;
    Callee {
        addr: 0x0028_b180,
        f: ee::s07::f_0028b180,
    }
    .call(c, 0x0028_3a80)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0029_ce28, 0x0028_3a88)?;
    c.g[V0][0] = 1;
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::rng_2 as rng;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::w;

    #[test]
    fn script_cmd_queue_clear_matches() {
        check(
            ssx3_ee::s07::f_00283c20,
            script_cmd_queue_clear,
            50,
            &|c, seed| {
                let mut r = Rng(0x1111_0002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..40 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn script_cmd_queue_next_index_matches() {
        check(
            ssx3_ee::s07::f_00283c60,
            script_cmd_queue_next_index,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0003 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = if seed % 2 == 0 {
                    4
                } else {
                    ((r.next_u32() as u64) << 32) | r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn script_cmd_queue_at_matches() {
        check_lohi(
            ssx3_ee::s07::f_00283d70,
            script_cmd_queue_at,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0004 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(DATA + CMD_READ, r.next_u32());
            },
        );
    }

    #[test]
    fn script_cmd_queue_is_empty_matches() {
        check(
            ssx3_ee::s07::f_00283da0,
            script_cmd_queue_is_empty,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0005 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let read = r.next_u32();
                let write = if seed % 3 == 0 { read } else { r.next_u32() };
                c.bus.write32(DATA + CMD_READ, read);
                c.bus.write32(DATA + CMD_WRITE, write);
                c.bus.write32(
                    DATA + CMD_FULL,
                    if seed % 4 == 0 { 0 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn script_movie_get_id_matches() {
        check(
            ssx3_ee::s07::f_00283bb8__generated,
            script_movie_get_id,
            300,
            &|c, seed| {
                let mut r = rng(seed, 0x39);
                scramble(c, &mut r);
                let movie = DATA + 0x4000;
                let data = DATA + 0x5000;
                c.bus.write32(movie, if seed % 3 == 0 { 0 } else { data });
                c.bus.write32(movie + 4, (seed % 5 != 0) as u32);
                c.bus.write32(data, r.next_u32());
                c.g[A0][0] = sx(movie);
            },
        );
    }

    #[test]
    fn script_cmd_queue_ctor_matches() {
        check(
            ssx3_ee::s07::f_00283c30__generated,
            script_cmd_queue_ctor,
            150,
            &|c, seed| {
                let mut r = rng(seed, 0x3a);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x4000);
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x4000 + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn script_movie_set_paused_matches() {
        check(
            ee::s07::f_00283b78__generated,
            script_movie_set_paused,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x283b78, 0);
                let m = DATA + 0x200;
                c.g[A0][0] = m as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(m, if seed % 4 == 0 { 0 } else { 1 });
                c.bus.write32(m + 4, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(
                    m + 0xc,
                    match seed % 3 {
                        0 => 0,
                        1 => 1,
                        _ => seed % 7,
                    },
                );
            },
        );
    }

    #[test]
    fn script_cmd_queue_count_matches() {
        check(
            ee::s07::f_00283d28__generated,
            script_cmd_queue_count,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x283d_28, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(this + 0x94, if seed % 4 == 0 { 1 } else { 0 });
                c.bus.write32(this + 0x8c, r.next_u32() % 6);
                c.bus.write32(this + 0x90, r.next_u32() % 6);
            },
        );
    }

    // -------------------------------------------------------- command queue

    #[test]
    fn script_cmd_queue_pop_matches() {
        check(
            ssx3_ee::s07::f_00283dc0__generated,
            script_cmd_queue_pop,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x0028_3dc0);
                let q = DATA + 0x100;
                let out = DATA + 0x2000;
                clear(c, q, 0x200);
                clear(c, out, 0x40);
                if seed % 2 == 0 {
                    w(c, q + 0x8c, (seed % 5) as u32);
                    w(c, q + 0x90, (seed % 5) as u32);
                    w(c, q + 0x94, 0);
                } else {
                    w(c, q + 0x8c, (seed % 4) as u32);
                    w(c, q + 0x90, (seed % 4 + 1) as u32);
                    w(c, q + 0x94, 1);
                }
                for k in 0..8 {
                    w(c, q + 0x1c * k, r.next_u32());
                }
                c.g[A0][0] = sx(q);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(out) };
            },
        );
    }

    /// Movies with and without a player, playing or not, every combination
    /// of the pause request and the player's paused flag. The player is
    /// mostly not set up (its +0x38 is 0, so the real Resume/Pause/Render
    /// return at once), sometimes with only its stream (+0x38, no +0x3c).
    #[test]
    fn script_movie_toggle_pause_matches() {
        use script_movie::*;
        check(
            ssx3_ee::s07::f_00283af8__generated,
            script_movie_toggle_pause,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x283af8, 0);
                c.pc = 0x0028_3af8;
                let movie = DATA;
                let player = DATA + 0x100;
                let flag = |r: &mut Rng, k: u32| {
                    if (seed >> k) & 1 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    }
                };
                c.bus
                    .write32(movie + PLAYER, if seed % 9 == 8 { 0 } else { player });
                c.bus.write32(
                    movie + PLAYING,
                    if seed % 7 == 6 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(movie + PAUSE_REQ, flag(&mut r, 0));
                c.bus.write32(player + PLAYER_PAUSED, flag(&mut r, 1));
                c.bus.write32(
                    player + 0x38,
                    flag(&mut r, 2) & (seed / 4 % 2).wrapping_neg(),
                );
                c.bus.write32(player + 0x3c, 0);
                c.g[A0][0] = movie as u64;
            },
        );
    }

    /// ScriptMovie_Update: no player, not started, done, or running (the
    /// real VidEnginePlayer_Update takes its short paths: already playing,
    /// then done or a pending request).
    #[test]
    fn script_movie_update_matches() {
        check(
            ee::s07::f_00283aa0__generated,
            script_movie_update,
            200,
            &|c, seed| {
                let mut r = Rng(0x0028_3aa0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0028_3aa0;
                let movie = DATA;
                let player = DATA + 0x1000;
                let pick = |r: &mut Rng, zero: bool| if zero { 0 } else { r.next_u32() | 1 };
                c.bus.write32(
                    movie + SCRIPT_MOVIE_PLAYER,
                    if seed % 5 == 0 { 0 } else { player },
                );
                let v = pick(&mut r, seed % 7 == 1);
                c.bus.write32(movie + SCRIPT_MOVIE_STARTED, v);
                let v = r.next_u32();
                c.bus.write32(movie + SCRIPT_MOVIE_UPDATED, v);
                let v = pick(&mut r, seed % 3 != 2);
                c.bus.write32(movie + SCRIPT_MOVIE_DONE, v);
                // The video player: already playing (+0x40), then finished
                // (+0xc) or with a pending request (+0).
                c.bus.write32(player + 0x40, 1);
                let done = seed % 2 == 0;
                c.bus.write32(player + 0xc, if done { 1 } else { 0 });
                c.bus
                    .write32(player, if done { 0 } else { r.next_u32() | 1 });
                c.g[A0][0] = sx(movie);
            },
        );
    }

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc428_getter_test;

    /// `ScriptCmdQueue_IsFull` reads `+0x94`.
    #[test]
    fn script_cmd_queue_is_full_matches() {
        oc428_getter_test(
            ee::s07::f_00283c58__generated,
            script_cmd_queue_is_full,
            0x283c58,
            SCRIPT_CMD_QUEUE_FULL,
        );
    }

    /// `ScriptCmdQueue_Push`: full/not-full queue, head 0..4 (wrapping), tail
    /// equal to the next head or not; the command lives at an unaligned
    /// address.
    #[test]
    fn script_cmd_queue_push_matches() {
        check(
            ee::s07::f_00283c80__generated,
            script_cmd_queue_push_2,
            200,
            &|c, seed| {
                let mut r = Rng(0x283c80 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let this = DATA + 0x5_0000;
                let cmd = DATA + 0x5_1000 + seed % 8;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = cmd as u64;
                let head = r.next_u32() % 5;
                c.bus.write32(this + 0x90, head);
                c.bus.write32(this + 0x94, (r.next_u32() % 4 == 0) as u32);
                let next = if head == 4 { 0 } else { head + 1 };
                c.bus.write32(
                    this + 0x8c,
                    if r.next_u32() % 2 == 0 {
                        next
                    } else {
                        r.next_u32() % 5
                    },
                );
                crate::difftest::fill(c, &mut r, cmd, 7);
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc554_gate_on;

    #[test]
    fn script_movie_start_matches() {
        check(
            ee::s07::f_002839a8__generated,
            script_movie_start,
            300,
            &|c, seed| {
                let mut r = oc554_gate_on(c, seed, 0x554_0011);
                let movie = DATA + 0x7000;
                let grman = DATA + 0x5000;
                let vt = DATA + 0x6400;
                let world = DATA + 0x5400;
                let game = DATA + 0x5800;
                let cut = DATA + 0x5c00;
                c.bus
                    .write32(movie, if seed % 5 == 0 { 0 } else { DATA + 0x7100 });
                c.bus
                    .write32(movie + 0x4, if seed % 3 == 0 { 1 } else { 0 });
                c.bus.write32(movie + 0x14, r.next_u32());
                c.bus.write32(GAME_GP - 0x854, grman);
                c.bus.write32(grman + 0x10d8, vt);
                c.bus.write16(vt + 0x140, 0x20);
                c.bus.write32(vt + 0x144, fakes::FAKE);
                c.bus.write16(vt + 0x150, 0x10);
                c.bus.write32(vt + 0x154, fakes::FAKE);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0x64, cut);
                c.bus.write32(cut + 0x10, seed % 4);
                c.bus.write32(GAME_GP - 0x69c, seed % 4);
                c.g[A0][0] = movie as u64;
            },
        );
    }
}
