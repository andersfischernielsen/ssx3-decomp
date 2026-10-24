//! `src/scripter/videngine.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_2;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_2;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
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
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_MANAGER_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SHARED_LOGIN_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_VOIP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::EVENT_SELECT_ON_WIDGET_NOTIFY;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_SET_LISTENER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOAD_INDICATOR_CLEAR_BUSY;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_SEND_LOCK_CHAL;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_READ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_ALLOC_NAMED_ALIGNED;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_GET_WORLD_SECTION_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_SPAWN_MISSIONS;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_PAUSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_PAUSE as IN_SEQ_INSTANCE_PAUSE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_MODE1;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_STATES;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_DISCONNECT_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_4 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_4 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_CACHE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D3;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D4;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_IS_FULL;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ACTIVATE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ALLOC_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_2 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_WRITE;
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
                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
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
            c.sw((c.g[S0][0] as u32), c.g[V0][0]);
            c.g[RA][0] = sx(0x0028393c);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x00253390, 0x0028393c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
            c.g[V0][0] = 0x1;
            c.g[V1][0] = 0x100;
            c.g[A2][0] = 0x9;
            c.sw((c.g[SP][0] as u32), c.g[S1][0]);
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
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                    if c.g[A0][0] == 0u64 {
                        c.sw((c.g[S0][0] as u32), 0u64);
                        break 'b_28398c;
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x00283988);
                        c.g[A1][0] = 0x3;
                        call_ptr(c, 0x00253418, 0x00283988)?;
                        c.sw((c.g[S0][0] as u32), 0u64);
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
        return Ok(());

        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2 as rng;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals;
    pub(crate) use crate::game::wscript::wscriptman::tests::w;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD;

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
}
