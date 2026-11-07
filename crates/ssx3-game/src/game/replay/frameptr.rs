//! `src/replay/frameptr.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::be::gp;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::p_8;
pub(crate) use crate::game::dirtysock::tags2::leave;
pub(crate) use crate::game::dirtysock::tags2::lw;
pub(crate) use crate::game::dirtysock::tags2::p as p_2;
pub(crate) use crate::game::dirtysock::tags2::p as p_3;
pub(crate) use crate::game::dirtysock::tags2::p_2 as p;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::feasyncfile::gp_12 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_4;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0026_d5e8,
        world_state_stream_ctor,
        "WorldStateStream_ctor",
    ),
    (
        0x0026_e908,
        replay_stream_alloc_buffer,
        "ReplayStream_AllocBuffer",
    ),
    (
        0x0026_e950,
        replay_stream_set_buffer,
        "ReplayStream_SetBuffer",
    ),
    (
        0x0026_e968,
        replay_stream_free_buffer,
        "ReplayStream_FreeBuffer",
    ),
    (
        0x0026_e9b0,
        replay_stream_rewind_read,
        "ReplayStream_RewindRead",
    ),
    (0x0026_ea20, replay_stream_read, "ReplayStream_Read"),
    (0x0026_ea68, replay_stream_peek_int, "ReplayStream_PeekInt"),
    (
        0x0026_eb48,
        replay_stream_read_object_ref,
        "ReplayStream_ReadObjectRef",
    ),
    (
        0x0026_ecd8,
        replay_stream_write_object_ref,
        "ReplayStream_WriteObjectRef",
    ),
    (
        0x0026_ed28,
        replay_stream_write_node_ref,
        "ReplayStream_WriteNodeRef",
    ),
    (
        0x0026_ed88,
        replay_snapshot_empty_hook_a,
        "Replay_SnapshotEmptyHookA",
    ),
    (0x0026_ed98, replay_next_camera, "Replay_NextCamera"),
    (0x0026_edd8, replay_apply_camera, "Replay_ApplyCamera"),
    (
        0x0026_ee68,
        replay_enable_replay_cameras,
        "Replay_EnableReplayCameras",
    ),
    (
        0x0026_eea0,
        replay_restore_game_cameras,
        "Replay_RestoreGameCameras",
    ),
    (
        0x0026_ef30,
        replay_enable_instant_replay_cameras,
        "Replay_EnableInstantReplayCameras",
    ),
    (
        0x0026_ef80,
        replay_restore_instant_replay_cameras,
        "Replay_RestoreInstantReplayCameras",
    ),
    (0x0026_f180, replay_init, "Replay_Init"),
    (0x0026_f428, replay_free_buffers, "Replay_FreeBuffers"),
    (
        0x0026_f7b8,
        replay_advance_from_state10,
        "Replay_AdvanceFromState10",
    ),
    (
        0x0026_f7f8,
        replay_save_resume_snapshot,
        "Replay_SaveResumeSnapshot",
    ),
    (0x0026_fb88, replay_handle_input, "Replay_HandleInput"),
    (0x0026_f4a8, replay_update, "Replay_Update"),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1;

    pub(crate) use crate::game::be::beintmission::callee::REPLAY_APPLY_CAMERA;

    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;

    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2;

    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;

    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;

    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;

    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;

    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;

    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;

    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;

    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2;

    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;

    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
}

/// `void ReplayStream_SetBuffer(ReplayStream *stream, void *buf, uint len)`
/// (0x26e950): set begin/end/cursor and the length.
pub fn replay_stream_set_buffer(c: &mut Cpu) {
    c.cycles += 6;
    let stream = Addr(abi::arg(c, 0));
    let buf = abi::arg(c, 1);
    let len = abi::arg(c, 2);
    c.g[V0][0] = sx(buf.wrapping_add(len));
    stream.off(0x10).sw(c, c.g[A2][0]);
    stream.off(0x8).sw(c, c.g[V0][0]);
    stream.off(0xc).sw(c, c.g[A1][0]);
    stream.off(0x4).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void ReplayStream_RewindRead(ReplayStream *stream)` (0x26e9b0): reset the
/// read cursor to the buffer begin.
pub fn replay_stream_rewind_read(c: &mut Cpu) {
    c.cycles += 3;
    let stream = Addr(abi::arg(c, 0));
    let begin = stream.off(0x4).lw(c);
    c.g[V0][0] = begin;
    stream.off(0xc).sw(c, begin);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Replay stream.

/// `void ReplayStream_WriteObjectRef(Stream* out, void* obj)` (0x26ecd8):
/// write the object's id at +0x78 (or -1) as four bytes through the stream's
/// virtual slot 1.
pub fn replay_stream_write_object_ref(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 7;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] |= 0xffff;
        frame.save_ra(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        let none = c.g[A1][0] == 0;
        let sp = sp(c);
        c.sw(sp, c.g[V0][0]);
        if !none {
            c.cycles += 2;
            c.g[V0][0] = c.lw(at(c, A1, 0x78));
            c.sw(sp, c.g[V0][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = c.lw(r32(c, A3));
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh(at(c, V0, 0x8));
        c.g[V1][0] = c.lw(at(c, V0, 0xc));
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0026_ed18)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------

/// `void ReplayStream_WriteNodeRef(ReplayStream *out, NodeRef *node)`
/// (0x0026ed28): append `{ u32 id; u16 type; }` (id = `node->+0x18->+0x78`,
/// type = `+0x18->+8`; `-1`/0 when the node has no reference) through
/// `out->vt[1](out + delta, &rec, 8)`.
pub fn replay_stream_write_node_ref(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] |= 0xffff;
        frame.save_ra(c, 0x10);
        let rec = frame.at(c, 0);
        c.stw(rec, c.g[V0][0]);
        let out = abi::arg(c, 0);
        let node = abi::arg(c, 1);
        c.g[A3][0] = sx(out);
        c.g[V1][0] = c.ldw(off(node, 0x18));
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            c.sth(off(rec, 4), 0);
        } else {
            c.cycles += 5;
            let p = c.g[V1][0] as u32;
            c.g[V0][0] = c.ldw(off(p, 0x78));
            c.stw(rec, c.g[V0][0]);
            c.g[V1][0] = c.ldhu(off(p, 8));
            c.sth(off(rec, 4), c.g[V1][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = c.ldw(out);
        c.g[A1][0] = sx(frame.at(c, 0));
        c.g[A2][0] = 8;
        c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 8));
        c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0xc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = addu(c.g[A3][0], c.g[A0][0]);
        call_ptr(c, target, 0x0026_ed7c)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay.

/// `void Replay_EnableReplayCameras(Replay* self)` (0x26ee68): refresh the
/// camera triggers, then apply the replay camera.
pub fn replay_enable_replay_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x004c_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5830));
        call_ptr(c, callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1, 0x0026_ee84)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REPLAY_APPLY_CAMERA.call(c, 0x0026_ee8c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_RestoreInstantReplayCameras(void)` (0x26ef80): re-arm the
/// in-game camera triggers and restore the replay camera controller.
pub fn replay_restore_instant_replay_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004c_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5830));
        callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS.call(c, 0x0026_ef94)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A1][0] = 2;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x84).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
        callee::GAME_CAMERA_SET_CONTROLLER.call(c, 0x0026_efac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World / replay streams.

/// `WorldStateStream* WorldStateStream_ctor(WorldStateStream*)` (0x26D5E8): the
/// replay-stream constructor, vtable `0x481850`, and an empty stream.
pub fn world_state_stream_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::REPLAY_STREAM_CTOR.call(c, 0x0026_d5fc)?;
        c.g[V1][0] = sx(0x0048_1850);
        c.g[A0][0] = c.g[S0][0];
        Addr(this as u32).sw(c, 0x0048_1850);
        callee::WORLD_STATE_STREAM_RESET.call(c, 0x0026_d610)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_NextCamera(Replay* self)` (0x26ED98): step the camera index at
/// +0x630 through the nine replay cameras (`(n + 1) % 9`) and apply it.
pub fn replay_next_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 9;
        c.cycles += 9;
        let index = c.lw_r_2(V0, r(c, A2).wrapping_add(0x630));
        let bumped = sx((index as u32).wrapping_add(1));
        c.g[V0][0] = bumped;
        let divisor = c.g[A1][0] as u32;
        let (_, rem) = abi::div(c, bumped as u32, divisor);
        c.g[V1][0] = sx(rem);
        Addr(r(c, A2).wrapping_add(0x630)).sw(c, sx(rem));
        callee::REPLAY_APPLY_CAMERA.call(c, 0x0026_edc8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_AdvanceFromState10(Replay* self)` (0x26F7B8): a replay in state
/// 10 moves on to state 11; states 14 and 15 are ignored.
pub fn replay_advance_from_state10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let state = c.lw_r_2(V1, r(c, A0));
        let frame = Frame::open(c, 0x10);
        let d = sx((state as u32).wrapping_sub(0xe));
        c.g[V0][0] = d;
        let ignored = d < 2;
        c.g[V0][0] = (ignored as u64) ^ 1;
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0xa;
            let not_ten = c.g[V1][0] != c.g[V0][0];
            frame.restore_ra(c, 0);
            if !not_ten {
                c.cycles += 2;
                c.g[A1][0] = 0xb;
                // The call returns to 0x26F7E8, which is the loop head: control
                // falls through the shared tail block (one more cycle, `ra`
                // reloaded from the frame) instead of returning straight away.
                callee::REPLAY_SET_STATE_2.call(c, 0x0026_f7e8)?;
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void ReplayStream_FreeBuffer(ReplayStream* self)` (0x26e968): free the
/// stream's frame buffer and clear its block fields.
pub fn replay_stream_free_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(at(c, S0, 0x04));
        if c.g[A0][0] == 0 {
            sw(c, r32(c, S0), 0x10, 0);
        } else {
            c.cycles += 3;
            call_ptr(c, callee::EA_MEM_FREE_2, 0x0026_e98c)?;
            sw(c, r32(c, S0), 0x10, 0);
        }
        c.cycles += 7;
        sw(c, r32(c, S0), 0x04, 0);
        sw(c, r32(c, S0), 0x08, 0);
        sw(c, r32(c, S0), 0x0c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ReplayStream_Read(ReplayStream* self, void* dst /*a1*/, int n
/// /*a2*/)` (0x26ea20): copy `n` bytes to `dst` and advance the stream's
/// write cursor.
pub fn replay_stream_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = c.lw(at(c, S0, 0x0c));
        call_ptr(c, callee::REPLAY_STREAM_WRITE, 0x0026_ea44)?;
        c.g[V0][0] = c.lw(at(c, S0, 0x0c));
        c.g[RA][0] = c.ld(p(sp(c), 0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        sw(c, r32(c, S0), 0x0c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        leave(c, 0x30);
        Ok(())
    });
}

/// `void Replay_RestoreResumeSnapshot(Replay* self)` (0x26f850): restore the
/// resume snapshot, then free it.
pub fn replay_restore_resume_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw(at(c, S0, 0x3d0));
        if c.g[A1][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 7;
            callee::REPLAY_RESTORE_SNAPSHOT.call(c, 0x0026_f874)?;
            c.g[A1][0] = c.lw(at(c, S0, 0x3d0));
            c.g[A0][0] = c.g[S0][0];
            callee::REPLAY_FREE_SNAPSHOT.call(c, 0x0026_f880)?;
            sw(c, r32(c, S0), 0x3d0, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ReplayStream_AllocBuffer(self, size)` (0x26e908): allocate the
/// replay stream's buffer and point `+0xc`, `+0x4` and `+0x8` at it, with the
/// write cursor (`+0x10`) cleared.
pub fn replay_stream_alloc_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(0x48_0000u32.wrapping_add(0x1730));
        call_ptr(c, 0x0031_7e30, 0x0026_e930)?;
        let self_ = abi::r32(c, S0);
        Addr(self_.wrapping_add(0x10)).sw(c, 0);
        Addr(self_.wrapping_add(0xc)).sw(c, c.g[V0][0]);
        Addr(self_.wrapping_add(4)).sw(c, c.g[V0][0]);
        Addr(self_.wrapping_add(8)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay.

/// `void Replay_EnableInstantReplayCameras(ReplayCache* self)` (0x26ef30):
/// refresh the camera triggers and step the instant-replay camera index
/// (`(n + 1) % 9`).
pub fn replay_enable_instant_replay_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x4c0000;
        c.g[A0][0] = sx(0x004c_5830);
        call_ptr(c, 0x0016_d1d8, 0x0026_ef4c)?;
        c.g[V1][0] = Addr(this).off(0x634).lw(c);
        c.g[V0][0] = 9;
        c.cycles += 8;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        frame.restore_ra(c, 0);
        c.div(0, V1, V0);
        c.g[A0][0] = c.hi[0];
        Addr(this).off(0x634).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay.

/// `int ReplayStream_PeekInt(ReplayStream* s)` (0x26ea68): save the read
/// cursor (+0xc), read the next 4 bytes through the stream's vtable slot 0x10
/// into a stack header, restore the cursor and return the 16-bit id.
/// 20 cycles plus the call.
pub fn replay_stream_peek_int(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 4;
        frame.save(c, 16, 0x30);
        c.g[A1][0] = sx(frame.at(c, 0));
        c.sd(p_2(frame.at(c, 0), 0x10), c.g[RA][0]);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        let pos = c.lw(p_2(self_, 0xc));
        c.g[S1][0] = pos;
        let vt = c.lw(self_);
        c.g[V1][0] = vt;
        c.g[V0][0] = c.lw(p_2(vt as u32, 0x14));
        let m = VSlot::read(c, vt, 0x10);
        c.g[A0][0] = m.this(sx(self_));
        call_ptr(c, m.func as u32, 0x0026_ea9c)?;
        c.sw(p_2(self_, 0xc), c.g[S1][0]);
        frame.restore(c, 16, 0x30);
        c.g[V0][0] = Addr(frame.at(c, 0)).lh(c);
        frame.restore(c, 17, 0x20);
        c.g[RA][0] = c.ld(p_2(frame.at(c, 0), 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SaveResumeSnapshot(self)` (0x26f7f8): if the resume block has
/// not been saved yet, record and add an empty frame snapshot.
pub fn replay_save_resume_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x3d0).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 11;
            c.g[V0][0] = 1;
            c.g[A1][0] = sx(c.g[SP][0] as u32);
            c.sh(c.g[SP][0] as u32, c.g[V0][0]);
            c.sb(p_3(c.g[SP][0] as u32, 2), 0);
            c.g[RA][0] = sx(0x0026_f82c);
            c.sb(p_3(c.g[SP][0] as u32, 3), 0);
            callee::REPLAY_RECORD_SNAPSHOT.call(c, 0x0026_f82c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0026_f83c);
            Addr(p_3(r32(c, S0), 0x3d0)).sw(c, c.g[V0][0]);
            callee::REPLAY_ADD_SNAPSHOT.call(c, 0x0026_f83c)?;
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SnapshotEmptyHookA(void)` (0x26ed88): replay snapshot hook, not
/// installed.
pub fn replay_snapshot_empty_hook_a(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void Replay_FreeBuffers(Replay* self)` (0x26f428): free every replay
/// stream buffer and clear both snapshot lists.
pub fn replay_free_buffers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at_2(c, S2, 8)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = sx(at_2(c, S2, 0x2c));
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, 0x0026_e968, 0x0026_f45c)?;
                c.g[V0][0] = Addr(at_2(c, S2, 8)).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_f450);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = sx(at_2(c, S2, 0x3b0));
        call_ptr(c, 0x0026_e5c0, 0x0026_f474)?;
        c.g[A0][0] = sx(at_2(c, S2, 0x3bc));
        call_ptr(c, 0x0026_e5c0, 0x0026_f47c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ReplayStream_Decompress(ReplayStream* self, u8** pp, u8* src)`
/// (0x26eab8): decode one compressed frame: header, body, then rewind the
/// read pointer.
pub fn replay_stream_decompress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        call_ptr(c, 0x003e_6574, 0x0026_eae0)?;
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = c.g[SP][0] | 4;
        c.g[A2][0] = 4;
        call_ptr(c, 0x003e_6574, 0x0026_eaf0)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        Addr(r32(c, S0)).sw(c, c.g[A0][0]);
        c.g[A2][0] = Addr(p_4(r32(c, S1), 4)).lw(c);
        call_ptr(c, 0x002c_85a0, 0x0026_eb08)?;
        c.g[V1][0] = Addr(p_4(r32(c, SP), 4)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
        Addr(r32(c, S0)).sw(c, c.g[A1][0]);
        c.g[V1][0] = Addr(p_4(r32(c, S1), 4)).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        Addr(p_4(r32(c, S1), 8)).sw(c, c.g[V1][0]);
        callee::REPLAY_STREAM_REWIND_READ.call(c, 0x0026_eb2c)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_ApplyCamera(void* self)` (0x26edd8): apply the recorded
/// camera index to every rider in the world's rider list.
pub fn replay_apply_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p_4(r32(c, V1), 0x84)).lw(c);
        c.g[V1][0] = Addr(p_4(r32(c, V0), 0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        let empty = (c.g[A0][0] as i64) <= 0;
        c.g[V0][0] = 0x0044_0000;
        if !empty {
            c.cycles += 5;
            c.g[S1][0] = sx(p_4(r32(c, V1), 4));
            c.g[S3][0] = sx(r32(c, V0).wrapping_add(0x5438));
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = Addr(p_4(r32(c, S2), 0x630)).lw(c);
            loop {
                c.cycles += 10;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                c.g[V1][0] = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                c.g[A0][0] = Addr(p_4(r32(c, V1), 0xa8)).lw(c);
                c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0016_1fa0, 0x0026_ee40)?;
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(p_4(r32(c, S2), 0x630)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_ee20);
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

/// `void Replay_RestoreGameCameras(Replay* self)` (0x26eea0): after restoring
/// the in-game camera triggers, hand every rider's camera back to it and
/// clear the replay's camera-lock flag.
pub fn replay_restore_game_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        frame.save(c, S3, 0x10);
        c.g[S3][0] = sx(self_);
        frame.save(c, S0, 0x40);
        c.g[A0][0] = 0x004c_0000;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p_5(r32(c, A0), 0x5830));
        call_ptr(
            c,
            callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS,
            0x0026_eec8,
        )?;
        c.g[V0][0] = c.lw(gp_2(c, -0x848));
        c.g[V1][0] = c.lw(at(c, V0, 0x84));
        c.g[V0][0] = c.lw(at(c, V1, 0x84));
        c.g[V1][0] = c.lw(r32(c, V0));
        let none = (c.g[V1][0] as i64) <= 0;
        c.g[S2][0] = sx(p_5(r32(c, V0), 4));
        if !none {
            c.cycles += 2;
            c.g[S1][0] = c.g[V1][0];
            c.g[V0][0] = c.lw(r32(c, S2));
            loop {
                c.cycles += 10;
                c.g[A1][0] = 2;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(4));
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                c.g[A0][0] = c.g[V0][0];
                c.g[S0][0] = c.lw(at(c, V0, 0xa8));
                call_ptr(c, callee::RIDER_SET_INPUT_ENABLED, 0x0026_ef00)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::CAMERA_SET_OWNER, 0x0026_ef08)?;
                if c.g[S1][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw(r32(c, S2));
                if event_due(c) {
                    deliver_event_here(c, 0x0026_eee8);
                }
            }
        }
        c.cycles += 8;
        c.sw(at(c, S3, 0x630), 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_Init(self)` (0x26f180): size the replay buffer, allocate its
/// blocks, then reset the replay stream.
pub fn replay_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0x1_0000;
        c.g[A1][0] = 0xc_0000;
        c.g[V1][0] = c.g[V1][0] | 0x4000;
        c.g[A1][0] = c.g[A1][0] | 0x9800;
        c.div(0, 5, 3);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[V0][0] = 0x1_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[V0][0] = c.g[V0][0] | 0x4000;
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        Addr(p_6(r32(c, S0), 0x14)).sw(c, c.g[V1][0]);
        c.g[S1][0] = sx(p_6(r32(c, S0), 0x2c));
        c.cycles += 4;
        Addr(p_6(r32(c, S0), 0x18)).sw(c, 0);
        c.g[A1][0] = c.lo[0];
        Addr(p_6(r32(c, S0), 8)).sw(c, c.g[A1][0]);
        c.g[A1][0] = Addr(p_6(r32(c, S0), 0x14)).lw(c);
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.g[S1][0] = sx(p_6(r32(c, S1), 0x38));
            call_ptr(c, 0x0026_e908, 0x0026f1e8)?;
            c.g[V0][0] = Addr(p_6(r32(c, S0), 8)).lw(c);
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = Addr(p_6(r32(c, S0), 0x14)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_f1d8);
                }
                continue;
            }
            c.cycles += 11;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0026_f4a0, 0x0026f204)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0026_f228, 0x0026f20c)?;
            break;
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `ReplayStream_ReadObjectRef` (0x0026eb48).
//
// `void* ReplayStream::ReadObjectRef(self)`: read an object reference from
// the stream and resolve it against the current layer's object table.

pub fn replay_stream_read_object_ref(c: &mut Cpu) {
    guest(c, |c| {
        'b_26ebf8: {
            'b_26ebf4: {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = 0xffffffffffff0000;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = 0x4;
                c.g[S0][0] = 0u64;
                c.g[A3][0] = c.lw(c.g[A0][0] as u32);
                c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0026eb80);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, t, 0x0026eb80)?;
                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                c.g[V0][0] = !(0u64 | c.g[A1][0]);
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_26ebf4;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[RA][0] = sx(0x0026eb9c);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                    call_ptr(c, 0x003512c0, 0x0026eb9c)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] != 0u64;
                    c.g[V0][0] = c.g[S0][0];
                    if t {
                        break 'b_26ebf8;
                    } else {
                        'b_26ebf0: {
                            'b_26ebec: {
                                c.cycles += 10;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                c.g[V0][0] = c.g[A1][0] & 0xff;
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                let t = c.g[V1][0] == 0u64;
                                c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                                if t {
                                    break 'b_26ebec;
                                } else {
                                    c.cycles += 7;
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    if t {
                                        break 'b_26ebf0;
                                    } else {
                                        break 'b_26ebec;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = 0u64;
                            break 'b_26ebf0;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = c.g[V0][0];
                        break 'b_26ebf4;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
            break 'b_26ebf8;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `Replay_dtor(self, flags)` (0x26f088): tear a replay object down: end
/// playback, drop the reset hook, free the buffers, run the destructors of
/// the embedded objects, and free the object itself when `flags & 1`.
pub fn replay_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        let _this = abi::arg(c, 0);
        let _flags = abi::arg(c, 1);
        c.g[V1][0] = 0;
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, c.g[S1][0] as u32);
        let t = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !t {
            c.cycles += 1;
            c.g[V1][0] = ((c.g[V0][0] as i64) < 10) as u64;
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0026_f0c8);
            c.g[A0][0] = c.g[S1][0];
            c.pc = 0x0026_f980;
            ee::s07::Replay_EndPlayback(c);
            if c.pc != 0x0026_f0c8 {
                return Err(Unwound);
            }
        }
        c.g[A0][0] = c.g[S1][0];
        c.cycles += 8;
        c.g[RA][0] = sx(0x0026_f0d4);
        c.g[S3][0] = c.g[S0][0] & 1;
        c.pc = 0x0026_f498;
        ee::s07::Replay_EmptyResetHook(c);
        if c.pc != 0x0026_f0d4 {
            return Err(Unwound);
        }
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c));
        c.g[RA][0] = sx(0x0026_f0e0);
        c.g[A0][0] = c.g[S1][0];
        c.pc = 0x0026_f428;
        ee::s07::Replay_FreeBuffers(c);
        if c.pc != 0x0026_f0e0 {
            return Err(Unwound);
        }
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x494));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5fc));
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 2;
            loop {
                c.cycles += 7;
                if c.g[V1][0] == c.g[V0][0] {
                    break;
                }
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb4));
                if event_due(c) {
                    deliver_event_here(c, 0x0026_f100);
                }
            }
        }
        c.cycles += 2;
        let t = c.g[S2][0] == 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3ac));
        if !t {
            c.cycles += 2;
            let t = c.g[S2][0] == c.g[S0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x38));
            if !t {
                c.cycles += 1;
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = lw(c, c.g[S0][0] as u32);
                    c.g[A0][0] = c.lh(p_8(c.g[V0][0] as u32, 0x38));
                    c.g[V1][0] = lw(c, p_8(c.g[V0][0] as u32, 0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0026_f148);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.pc = t;
                    ssx3_ee_rt::funcs::call_indirect(c, t);
                    if c.pc != 0x0026_f148 {
                        return Err(Unwound);
                    }
                    let t = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x38));
                    if !t {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_f130);
                    }
                }
            }
        }
        c.cycles += 2;
        let t = c.g[S3][0] == 0;
        frame.restore(c, 16, 0x40);
        if !t {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e50, 0x0026_f160)?; // operator delete(this)
            frame.restore(c, 16, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay_HandleInput (0x26fb88).

/// Where a [`replay_handle_input`] path continues (the shared tails).
enum InputExit {
    /// `+0x618 = s2`, then the result.
    Store,
    /// `v0 = s2 > 0`.
    Result,
    /// Return with `v0` as it is.
    Done,
}

/// The step hold-repeat clock key: the toggle word `+0x20`.
const REPLAY_STEP_TOGGLE: u32 = 0x20;
/// The hold delay `+0x1c` before stepping repeats (10 frames at 60 Hz).
const REPLAY_STEP_DELAY: u32 = 0x1c;

/// `InputBinding_EvalBool(player input, binding)` (0x320c48) returning to
/// `ret`: `a0` is the player's input binding (`s0`).
fn replay_binding(c: &mut Cpu, binding: u64, ret: u32) -> Flow {
    c.g[A1][0] = binding;
    call_ptr(c, 0x0032_0c48, ret)
}

/// The gate of the step and slow-motion commands (`v1` = the replay state):
/// not paused (state 3) gives 7; paused, the command's `result` unless a
/// state 1..9 replay sits on its last frame (`+0x484 == +0x3cc`), then 0.
fn replay_paused_gate(c: &mut Cpu, result: u64) -> InputExit {
    c.g[V0][0] = c.g[V1][0] ^ 3;
    if c.g[V0][0] != 0 {
        c.g[S2][0] = 7;
        return InputExit::Store;
    }
    c.cycles += 3;
    c.g[V0][0] = 0;
    c.g[A0][0] = 0;
    if c.g[V1][0] != 0 {
        c.cycles += 1;
        c.g[A0][0] = ((c.g[V1][0] as i64) < 10) as u64;
    }
    c.cycles += 2;
    c.g[S2][0] = result;
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        let replay = r32(c, S1);
        c.g[V0][0] = Addr(replay).off(0x484).lw(c);
        c.g[V1][0] = Addr(replay).off(0x3cc).lw(c);
        c.g[V0][0] ^= c.g[V1][0];
        c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.g[S2][0] = 0;
    }
    InputExit::Store
}

/// The gate of the two speed commands (`v1` = the replay state): `None`
/// when not paused (`s2 = 7`), else whether the replay is at its end
/// (`+0x484 == +0x3cc`, or only `+0x484 == 0` for `at_end_only`), with
/// `v0` = the speed controller's vtable.
fn replay_speed_gate(c: &mut Cpu, at_end_only: bool) -> Option<bool> {
    c.g[V0][0] = c.g[V1][0] ^ 3;
    if c.g[V0][0] != 0 {
        c.g[S2][0] = 7;
        return None;
    }
    c.cycles += 3;
    c.g[V0][0] = 0;
    c.g[A0][0] = 0;
    if c.g[V1][0] != 0 {
        c.cycles += 1;
        c.g[A0][0] = ((c.g[V1][0] as i64) < 10) as u64;
    }
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        let replay = r32(c, S1);
        if at_end_only {
            c.cycles += 2;
            c.g[V0][0] = Addr(replay).off(0x484).lw(c);
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(replay).off(0x484).lw(c);
            c.g[V1][0] = Addr(replay).off(0x3cc).lw(c);
            c.g[V0][0] ^= c.g[V1][0];
        }
        c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    }
    c.cycles += 2;
    let at_end = c.g[V0][0] != 0;
    c.g[V0][0] = Addr(r32(c, S3)).lw(c);
    Some(at_end)
}

/// The speed controller's slot `+0x10` with `a1` = 4 (at the end) or 2
/// (`s2 = command`), then the command is stored.
fn replay_set_speed(c: &mut Cpu, at_end: bool, command: u64) -> Result<InputExit, Unwound> {
    if at_end {
        c.cycles += 1;
        c.g[A1][0] = 4;
    } else {
        c.cycles += 3;
        c.g[A1][0] = 2;
        c.g[S2][0] = command;
    }
    c.cycles += 6;
    let vt = r32(c, V0);
    c.g[A0][0] = Addr(vt).off(0x10).lh(c);
    c.g[V1][0] = Addr(vt).off(0x14).lw(c);
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, r32(c, V1), 0x0026_fdf4)?;
    Addr(r32(c, S1)).off(0x618).sw(c, c.g[S2][0]);
    Ok(InputExit::Result)
}

/// The body of [`replay_handle_input`] up to its shared tails.
fn replay_handle_input_body(c: &mut Cpu) -> Result<InputExit, Unwound> {
    const NEXT_CAMERA: Callee = Callee {
        addr: 0x0026_ed98,
        f: ee::s07::Replay_NextCamera,
    };
    let replay = r32(c, S1);
    let world = r32(c, A3);
    c.g[A0][0] = Addr(world).off(0x84).lw(c);
    c.g[A2][0] = Addr(replay).off(0x618).lw(c);
    c.g[V1][0] = Addr(r32(c, A0)).off(0x48).lw(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(0x14).lw(c);
    if c.g[V0][0] != 0 {
        c.g[S3][0] = c.g[V0][0];
    }
    c.g[V0][0] = 0;
    if c.g[A2][0] != 0 {
        // A command is still pending.
        return Ok(InputExit::Done);
    }
    c.cycles += 7;
    c.g[V0][0] = Addr(replay).off(0x28).lw(c);
    c.g[S2][0] = 0;
    c.g[V1][0] = Addr(replay).off(0x61c).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(world.wrapping_add(c.g[V0][0] as u32));
    c.g[S0][0] = Addr(r32(c, V0)).off(0xb0).lw(c);
    if c.g[V1][0] != 0 {
        return Ok(InputExit::Store);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(replay).off(0x620).lw(c);
    if c.g[V0][0] != 0 {
        Addr(replay).off(0x618).sw(c, c.g[S2][0]);
        return Ok(InputExit::Result);
    }

    // Step (0x6a pressed, 0x6b held): the first press steps at once and
    // arms a 10-frame hold delay, then a held button steps every other
    // frame.
    c.cycles += 5;
    c.g[A0][0] = c.g[S0][0];
    replay_binding(c, 0x6a, 0x0026_fc04)?;
    let pressed = c.g[V0][0] != 0;
    c.g[V0][0] = 0xa;
    if pressed {
        c.cycles += 4;
        c.g[V1][0] = Addr(replay).lw(c);
        Addr(replay).off(REPLAY_STEP_DELAY).sw(c, c.g[V0][0]);
        Addr(replay).off(REPLAY_STEP_TOGGLE).sw(c, 0);
        // Tick rate: a new hold starts its own 60 Hz clock.
        crate::tick::frame60_ran(c, replay.wrapping_add(REPLAY_STEP_TOGGLE), u32::MAX);
        c.cycles += 3;
        return Ok(replay_paused_gate(c, 5));
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S0][0];
    replay_binding(c, 0x6b, 0x0026_fc28)?;
    let held = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if held {
        // Tick rate: the delay and the every-other-frame toggle are in
        // 60 Hz frames; between two 60 Hz frames of the hold nothing
        // happens (as on the ticks the original waits).
        if !crate::tick::frame60_due(c, replay.wrapping_add(REPLAY_STEP_TOGGLE), 0) {
            return Ok(InputExit::Store);
        }
        c.cycles += 3;
        let delay = Addr(replay).off(REPLAY_STEP_DELAY).lw(c);
        c.g[V0][0] = sx((delay as u32).wrapping_sub(1));
        if (delay as i64) > 0 {
            c.cycles += 1;
            Addr(replay).off(REPLAY_STEP_DELAY).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[V0][0] = Addr(replay).off(REPLAY_STEP_TOGGLE).lw(c);
        c.g[V1][0] = Addr(replay).off(REPLAY_STEP_DELAY).lw(c);
        c.g[V0][0] ^= 1;
        Addr(replay).off(REPLAY_STEP_TOGGLE).sw(c, c.g[V0][0]);
        if (c.g[V1][0] as i64) > 0 {
            return Ok(InputExit::Store);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            Addr(replay).off(0x618).sw(c, c.g[S2][0]);
            return Ok(InputExit::Result);
        }
        c.cycles += 1;
        c.g[V1][0] = Addr(replay).lw(c);
        c.cycles += 3;
        return Ok(replay_paused_gate(c, 5));
    }

    // 0x67: command 6.
    c.cycles += 4;
    replay_binding(c, 0x67, 0x0026_fca4)?;
    let on = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if on {
        c.cycles += 4;
        c.g[V1][0] = Addr(replay).lw(c);
        return Ok(replay_paused_gate(c, 6));
    }

    // 0x66: leave the replay (with `a1`: end the playback; command 8), or
    // command 7 unless paused.
    c.cycles += 4;
    replay_binding(c, 0x66, 0x0026_fcf4)?;
    let on = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if on {
        c.cycles += 2;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        let store = if c.g[S4][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(r32(c, V0)).off(0x84).lw(c);
            call_ptr(c, 0x0023_05c8, 0x0026_fd0c)?; // GameModule_EndReplayPlayback
            c.g[V0][0] = 8;
            true
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(replay).lw(c);
            c.g[V0][0] ^= 3;
            let paused = c.g[V0][0] == 0;
            c.g[V0][0] = 7;
            !paused
        };
        if store {
            c.cycles += 1;
            Addr(replay).off(0x618).sw(c, c.g[V0][0]);
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        return Ok(InputExit::Done);
    }

    // 0x68 / 0x69: the speed commands 4 and 3.
    c.cycles += 4;
    replay_binding(c, 0x68, 0x0026_fd38)?;
    let on = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if on {
        c.cycles += 4;
        c.g[V1][0] = Addr(replay).lw(c);
        return match replay_speed_gate(c, false) {
            None => Ok(InputExit::Store),
            Some(at_end) => replay_set_speed(c, at_end, 4),
        };
    }
    c.cycles += 4;
    replay_binding(c, 0x69, 0x0026_fd94)?;
    let on = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[S0][0];
    if on {
        c.cycles += 4;
        c.g[V1][0] = Addr(replay).lw(c);
        return match replay_speed_gate(c, true) {
            None => Ok(InputExit::Store),
            Some(at_end) => replay_set_speed(c, at_end, 3),
        };
    }

    // 0x6c: the next camera (the controller's slot `+0x10` with 2 first).
    c.cycles += 4;
    replay_binding(c, 0x6c, 0x0026_fe04)?;
    let on = c.g[V0][0] != 0;
    c.g[A1][0] = 2;
    if on {
        c.cycles += 7;
        c.g[V0][0] = Addr(r32(c, S3)).lw(c);
        let vt = r32(c, V0);
        c.g[A0][0] = Addr(vt).off(0x10).lh(c);
        c.g[V1][0] = Addr(vt).off(0x14).lw(c);
        c.g[A0][0] = sx(r32(c, S3).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, r32(c, V1), 0x0026_fe20)?;
        c.g[A0][0] = c.g[S1][0];
        NEXT_CAMERA.call(c, 0x0026_fe28)?;
    }
    Ok(InputExit::Store)
}

/// `int Replay_HandleInput(Replay* self, int canExit)` (0x26fb88): read the
/// replay controls of the player watching (`world + 0xb0 + 4 * self[+0x28]`)
/// and queue one command in `+0x618` (0 none; 3/4 speed, 5 step, 6, 7
/// unpause, 8 end playback); returns whether a command was queued. Nothing
/// happens while a command is pending (`+0x618`) or input is locked
/// (`+0x61c`, `+0x620`). `canExit` lets the exit binding end the playback
/// (`GameModule_EndReplayPlayback`).
///
/// Tick rate: holding the step binding repeats after a 10-frame delay
/// (`+0x1c`) every other frame (`+0x20` toggles), both in 60 Hz frames.
/// Above 60 the hold logic runs only on the hold's own 60 Hz frames
/// ([`crate::tick::frame60_due`] keyed by `self + 0x20`, restarted by a new
/// press), so the repeat starts after 1/6 s and runs at 30 steps/s at any
/// rate. Nothing changes at 60.
pub fn replay_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0);
        let mut exit = replay_handle_input_body(c)?;
        loop {
            match exit {
                InputExit::Store => {
                    c.cycles += 1;
                    Addr(r32(c, S1)).off(0x618).sw(c, c.g[S2][0]);
                    exit = InputExit::Result;
                }
                InputExit::Result => {
                    c.cycles += 1;
                    c.g[V0][0] = (0 < c.g[S2][0]) as u64;
                    exit = InputExit::Done;
                }
                InputExit::Done => break,
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

pub(crate) mod replay_callee {
    use super::ee;
    use super::Callee;

    pub const SET_STATE: Callee = Callee {
        addr: 0x0026_f4a0,
        f: ee::s07::Replay_SetState,
    };

    pub const HANDLE_INPUT: Callee = Callee {
        addr: 0x0026_fb88,
        f: ee::s07::Replay_HandleInput,
    };

    pub const PROCESS_COMMAND: Callee = Callee {
        addr: 0x0026_fe50,
        f: ee::s07::Replay_ProcessCommand,
    };

    pub const END_INSTANT_REPLAY: Callee = Callee {
        addr: 0x0027_0670,
        f: ee::s07::Replay_EndInstantReplay,
    };

    pub const CANCEL_INSTANT_REPLAY: Callee = Callee {
        addr: 0x0027_0628,
        f: ee::s07::Replay_CancelInstantReplay,
    };

    pub const CLEAR_ALL_HIGHLIGHTS: Callee = Callee {
        addr: 0x0027_0e18,
        f: ee::s07::Replay_ClearAllHighlights,
    };

    pub const PRUNE_SNAPSHOTS: Callee = Callee {
        addr: 0x0027_0538,
        f: ee::s07::Replay_PruneSnapshots,
    };

    pub const RECORD_SNAPSHOT: Callee = Callee {
        addr: 0x0026_d760,
        f: ee::s07::Replay_RecordSnapshot,
    };

    pub const ADD_SNAPSHOT: Callee = Callee {
        addr: 0x0026_d740,
        f: ee::s07::Replay_AddSnapshot,
    };

    pub const BEGIN_PLAYBACK: Callee = Callee {
        addr: 0x0026_f8a0,
        f: ee::s07::Replay_BeginPlayback,
    };

    /// `bool InputBinding_EvalBool(InputBinding*, int)` (0x320c48, another
    /// shard: through the function table).
    pub const INPUT_EVAL_BOOL: u32 = 0x0032_0c48;
}

/// Replay manager fields.
pub(crate) const REPLAY_STATE: u32 = 0x000;

pub(crate) const REPLAY_F20: u32 = 0x020;

pub(crate) const REPLAY_PLAYER: u32 = 0x028;

pub(crate) const REPLAY_FRAME: u32 = 0x3c8;

pub(crate) const REPLAY_SECONDS: u32 = 0x3cc;

pub(crate) const REPLAY_BLOCK: u32 = 0x3d0;

pub(crate) const REPLAY_START: u32 = 0x3d4;

pub(crate) const REPLAY_KEY_RATE: u32 = 0x018;

pub(crate) const REPLAY_LAST: u32 = 0x484;

pub(crate) const REPLAY_F604: u32 = 0x604;

pub(crate) const REPLAY_F608: u32 = 0x608;

pub(crate) const REPLAY_F60C: u32 = 0x60c;

pub(crate) const REPLAY_PAUSED: u32 = 0x610;

pub(crate) const REPLAY_TIMER: u32 = 0x614;

pub(crate) const REPLAY_F618: u32 = 0x618;

pub(crate) const REPLAY_F61C: u32 = 0x61c;

pub(crate) const REPLAY_F620: u32 = 0x620;

pub(crate) const REPLAY_F624: u32 = 0x624;

/// The state switch's jump table (0x480000 + 0x1750), 14 entries.
pub(crate) const REPLAY_STATE_TABLE: u32 = 0x0048_1750;

/// Where a state handler of `Replay_Update` continues.
pub(crate) enum ReplayNext {
    /// `Replay_ProcessCommand(this)` then out (0x26f73c).
    Command,
    /// `ProcessCommand` with the 2-cycle block (0x26f7a0).
    CommandShort,
    /// The common "restore s0" exit (0x26f7a8).
    Out,
    /// s0 already restored (the `lq` in the branch delay slot).
    Done,
}

/// `void Replay_Update(Replay* this)` (0x0026f4a8): the replay manager's
/// per-frame step. Counts down the timer, clears the per-frame flag, drops
/// back to state 3 on request (+0x618 == 7), then runs the state: record
/// (0/12: snapshot every 60 frames, a key frame every `keyRate` snapshots,
/// count seconds and frames; or start the pending instant replay or
/// playback), instant replay (2), playback (3, with the rider's skip
/// check), the transitions 8/9/11 and the command processing of the rest.
/// The state switch reads the original's jump table (other targets leave
/// through `jump_out`). Entries other than the start go to the generated
/// `__entries` twin (override stub).
pub fn replay_update(c: &mut Cpu) {
    use replay_callee::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        let field = |off: u32| this.wrapping_add(off);
        c.g[V0][0] = c.lw(field(REPLAY_STATE));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64 ^ 1;
        let next = if c.g[V0][0] == 0 {
            // States 14/15: nothing to do.
            frame.restore(c, S0, 0x20);
            ReplayNext::Done
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(field(REPLAY_TIMER));
            let expired = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            if !expired {
                c.cycles += 1;
                c.sw(field(REPLAY_TIMER), c.g[V0][0]);
            }
            c.cycles += 4;
            c.g[V1][0] = c.lw(field(REPLAY_F618));
            c.g[V0][0] = 7;
            c.sw(field(REPLAY_F604), 0);
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 3;
                SET_STATE.call(c, 0x0026_f4fc)?;
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw(field(REPLAY_STATE));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(3));
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            if c.g[V0][0] == 0 {
                c.sw(field(REPLAY_F20), 0);
            }
            c.cycles += 4;
            c.g[V1][0] = c.lw(field(REPLAY_STATE));
            c.g[V0][0] = (c.g[V1][0] < 0xe) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[V0][0] = 0x48_0000;
            if out_of_range {
                ReplayNext::CommandShort
            } else {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx(REPLAY_STATE_TABLE);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                match target {
                    0x0026_f538 => {
                        // 11: on to 12.
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0xc;
                        SET_STATE.call(c, 0x0026_f544)?;
                        frame.restore(c, S0, 0x20);
                        ReplayNext::Done
                    }
                    0x0026_f54c => replay_instant(c, this)?,
                    0x0026_f57c => replay_record(c, this, frame)?,
                    0x0026_f708 => replay_playback(c, this)?,
                    0x0026_f74c => {
                        // 9: start playback from the top, state 1.
                        c.cycles += 11;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 1;
                        BEGIN_PLAYBACK.call(c, 0x0026_f760)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 1;
                        SET_STATE.call(c, 0x0026_f76c)?;
                        c.g[V0][0] = 1;
                        c.sw(field(REPLAY_F61C), c.g[V0][0]);
                        ReplayNext::Out
                    }
                    0x0026_f778 | 0x0026_f78c => {
                        if target == 0x0026_f778 {
                            // 8: rewind first.
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0;
                            c.g[A2][0] = 0;
                            c.g[A3][0] = 0;
                            BEGIN_PLAYBACK.call(c, 0x0026_f78c)?;
                        }
                        // 4..7: back to playback (3).
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 3;
                        SET_STATE.call(c, 0x0026_f798)?;
                        frame.restore(c, S0, 0x20);
                        ReplayNext::Done
                    }
                    0x0026_f7a0 => ReplayNext::CommandShort,
                    0x0026_f7a8 => ReplayNext::Out,
                    _ => {
                        c.jump_out(target, 0x0026_f4a8, 0x0026_f7b8);
                        return Ok(());
                    }
                }
            }
        };
        match next {
            ReplayNext::Command => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                PROCESS_COMMAND.call(c, 0x0026_f744)?;
                frame.restore(c, S0, 0x20);
            }
            ReplayNext::CommandShort => {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                PROCESS_COMMAND.call(c, 0x0026_f7a8)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            ReplayNext::Out => {
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            ReplayNext::Done => {}
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// State 2 (instant replay): past the last recorded frame -> handle input
/// and process commands; else end the instant replay and record (state 0).
pub(crate) fn replay_instant(c: &mut Cpu, this: u32) -> Result<ReplayNext, crate::call::Unwound> {
    use replay_callee::*;
    c.cycles += 6;
    c.g[V0][0] = c.lw(this + REPLAY_FRAME);
    c.g[V1][0] = c.lw(this + REPLAY_LAST);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[A0][0] = c.g[S0][0];
    if c.g[V1][0] != 0 {
        c.cycles += 4;
        c.g[A1][0] = 0;
        HANDLE_INPUT.call(c, 0x0026_f56c)?;
        return Ok(ReplayNext::Command);
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S0][0];
    END_INSTANT_REPLAY.call(c, 0x0026_f57c)?;
    let frame = Frame { size: 0x30 };
    replay_record(c, this, frame)
}

/// Push one snapshot: `AddSnapshot(this, RecordSnapshot(this, {key, 0,
/// 0}))` with the 4-byte argument at `sp`.
pub(crate) fn replay_snapshot(
    c: &mut Cpu,
    key: u64,
    ret_record: u32,
    ret_add: u32,
) -> crate::call::Flow {
    use replay_callee::*;
    let sp_ = sp(c);
    c.g[A1][0] = c.g[SP][0];
    c.sh(sp_, key);
    c.sb(sp_ + 2, 0);
    c.sb(sp_ + 3, 0);
    RECORD_SNAPSHOT.call(c, ret_record)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    ADD_SNAPSHOT.call(c, ret_add)
}

/// States 0/12 (recording).
pub(crate) fn replay_record(
    c: &mut Cpu,
    this: u32,
    frame: Frame,
) -> Result<ReplayNext, crate::call::Unwound> {
    use replay_callee::*;
    c.cycles += 6;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    SET_STATE.call(c, 0x0026_f588)?;
    c.g[V0][0] = c.lw(this + REPLAY_F60C);
    c.sw(this + REPLAY_BLOCK, 0);
    if c.g[V0][0] != 0 {
        // A finished recording: close it with a key snapshot, state 13.
        c.cycles += 3;
        c.g[V0][0] = c.lw(this + REPLAY_F608);
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
            return Ok(ReplayNext::Done);
        }
        c.cycles += 21;
        c.g[A0][0] = c.g[S0][0];
        CANCEL_INSTANT_REPLAY.call(c, 0x0026_f5a8)?;
        c.g[A0][0] = c.g[S0][0];
        CLEAR_ALL_HIGHLIGHTS.call(c, 0x0026_f5b0)?;
        c.g[A0][0] = c.g[S0][0];
        PRUNE_SNAPSHOTS.call(c, 0x0026_f5b8)?;
        c.g[V0][0] = 1;
        c.g[A0][0] = c.g[S0][0];
        replay_snapshot(c, 1, 0x0026_f5d4, 0x0026_f5e0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0xd;
        SET_STATE.call(c, 0x0026_f5ec)?;
        frame.restore(c, S0, 0x20);
        return Ok(ReplayNext::Done);
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(this + REPLAY_F624);
    c.g[A0][0] = c.g[S0][0];
    if c.g[V0][0] != 0 {
        // A pending instant replay: start it (state 2).
        c.cycles += 11;
        c.g[A1][0] = c.lw(this + REPLAY_START);
        c.g[A2][0] = 1;
        c.g[A3][0] = 0;
        BEGIN_PLAYBACK.call(c, 0x0026_f610)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        SET_STATE.call(c, 0x0026_f61c)?;
        c.g[V0][0] = 1;
        c.sw(this + REPLAY_F624, 0);
        c.sw(this + REPLAY_F620, c.g[V0][0]);
        return Ok(ReplayNext::Out);
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(this + REPLAY_PAUSED);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = c.lw(this + REPLAY_FRAME);
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.lw(this + REPLAY_FRAME);
        c.g[V0][0] = 0x3c;
        c.cycles += 4;
        c.div(0, A0, V0);
        c.g[V1][0] = c.hi[0];
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.lw(this + REPLAY_PAUSED);
        } else {
            // Every 60 frames: prune and push a snapshot (a key one on
            // frame 0 and every keyRate frames).
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            PRUNE_SNAPSHOTS.call(c, 0x0026_f660)?;
            c.g[V0][0] = c.lw(this + REPLAY_FRAME);
            let key = if c.g[V0][0] == 0 {
                c.g[V0][0] = 1;
                true
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lw(this + REPLAY_KEY_RATE);
                if c.g[V1][0] == 0 {
                    c.g[A0][0] = c.g[S0][0];
                    false
                } else {
                    c.cycles += 3;
                    c.div(0, V0, V1);
                    c.cycles += 3;
                    c.g[V0][0] = c.hi[0];
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.g[S0][0];
                        false
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 1;
                        true
                    }
                }
            };
            if key {
                c.cycles += 11;
                c.g[A0][0] = c.g[S0][0];
                let k = c.g[V0][0];
                replay_snapshot(c, k, 0x0026_f6ac, 0x0026_f6b8)?;
            } else {
                c.cycles += 9;
                replay_snapshot(c, 0, 0x0026_f6d4, 0x0026_f6e0)?;
            }
            c.g[V0][0] = c.lw(this + REPLAY_PAUSED);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lw(this + REPLAY_FRAME);
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(this + REPLAY_SECONDS);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sw(this + REPLAY_SECONDS, c.g[V0][0]);
            c.g[V0][0] = c.lw(this + REPLAY_FRAME);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.sw(this + REPLAY_FRAME, c.g[V0][0]);
    Ok(ReplayNext::Out)
}

/// State 3 (playback): the followed player's input binding may ask for a
/// skip (0x6b) -> handle input; then process commands.
pub(crate) fn replay_playback(c: &mut Cpu, this: u32) -> Result<ReplayNext, crate::call::Unwound> {
    use replay_callee::*;
    c.cycles += 7;
    c.g[V0][0] = c.lw(this + REPLAY_PLAYER);
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xb0));
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        c.g[A1][0] = 0x6b;
        call_ptr(c, INPUT_EVAL_BOOL, 0x0026_f72c)?;
        let skip = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];
        if skip {
            c.cycles += 2;
            c.g[A1][0] = 0;
            HANDLE_INPUT.call(c, 0x0026_f73c)?;
        }
    }
    Ok(ReplayNext::Command)
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

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::leaf;

    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;

    pub(crate) use crate::game::dirtysock::tags2::tests::put_vtable;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;

    pub(crate) use crate::game::fe::festatecharequipdetail::tests::world;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::input::inputmap::tests::put_f32;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    use ssx3_ee as ee;

    #[test]
    fn replay_stream_set_buffer_matches() {
        check(
            ssx3_ee::s07::f_0026e950,
            replay_stream_set_buffer,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c13_0012 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                for i in 0..5 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn replay_stream_rewind_read_matches() {
        check(
            ssx3_ee::s07::f_0026e9b0,
            replay_stream_rewind_read,
            50,
            &|c, seed| {
                let mut r = Rng(0x0c13_0013 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 4, r.next_u32());
                c.bus.write32(DATA + 0xc, r.next_u32());
            },
        );
    }

    #[test]
    fn replay_stream_write_object_ref_matches() {
        covered(
            ssx3_ee::s07::f_0026ecd8__generated,
            replay_stream_write_object_ref,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 41, 1);
                let out = DATA + 0x100;
                let vt = vtable(c, &mut r, &[0x8]);
                c.bus.write32(out, vt);
                if seed % 2 == 0 {
                    let obj = DATA + 0x400;
                    c.bus.write32(obj + 0x78, r.next_u32());
                    c.g[A1][0] = sx(obj);
                } else {
                    c.g[A1][0] = 0;
                }
                c.g[A0][0] = sx(out);
            },
        );
    }

    #[test]
    fn replay_stream_write_node_ref_matches() {
        covered(
            ssx3_ee::s07::f_0026ed28__generated,
            replay_stream_write_node_ref,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26ed28, 1);
                let out = DATA;
                let node = DATA + 0x400;
                c.g[A0][0] = out as u64;
                c.g[A1][0] = node as u64;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(out, vt);
                let target = if seed % 3 == 0 { 0 } else { DATA + 0x800 };
                c.bus.write32(node + 0x18, target);
                put_f32(c, target + 0x78, r.next_u32());
                c.bus.write16(target + 8, r.next_u32() as u16);
            },
        );
    }

    // ------------------------------------------------------------------
    // Replay.

    #[test]
    fn replay_enable_replay_cameras_matches() {
        check(
            ssx3_ee::s07::f_0026ee68__generated,
            replay_enable_replay_cameras,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26ee68, 0);
                world(c);
                // No active triggers (`+4 <= 0`) and no riders (`*riders <= 0`).
                c.bus.write32(0x004c_5834, 0);
                c.bus.write32(DATA + 0x7000, 0);
                fill(c, &mut r, DATA + 0x300, 0x20);
                c.g[A0][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn replay_restore_instant_replay_cameras_matches() {
        check(
            ee::s07::f_0026ef80__generated,
            replay_restore_instant_replay_cameras,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x26ef_80, 0);
                // No in-game camera triggers: CameraTriggerMan_UpdateTriggers exits.
                c.bus.write32(0x004c_5830 + 4, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
                zero(c, DATA + 0x1000, 0x400);
                c.bus.write32(DATA + 0x1000 + 0x84, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100 + 0x84, DATA + 0x1200);
                c.bus.write32(DATA + 0x1200 + 4, DATA + 0x1300);
                zero(c, DATA + 0x1300, 0x100);
            },
        );
    }

    // ------------------------------------------------------------- replay

    #[test]
    fn world_state_stream_ctor_matches() {
        check(
            ssx3_ee::s07::f_0026d5e8__generated,
            world_state_stream_ctor,
            60,
            &|c, seed| {
                machine(c, seed, 0x26d5_e8, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn replay_next_camera_matches() {
        check_lohi(
            ssx3_ee::s07::f_0026ed98__generated,
            replay_next_camera,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x26ed_98, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x630, seed % 9);
            },
        );
    }

    #[test]
    fn replay_advance_from_state10_matches() {
        check(
            ssx3_ee::s07::f_0026f7b8__generated,
            replay_advance_from_state10,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x26f7_b8, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                let state = match seed % 3 {
                    0 => 0x0a,
                    1 => 0x0e,
                    _ => 3,
                };
                w(c, DATA, state);
            },
        );
    }

    #[test]
    fn replay_stream_free_buffer_matches() {
        check(
            ssx3_ee::s07::f_0026e968__generated,
            replay_stream_free_buffer,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x26e968, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 4, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn replay_stream_read_matches() {
        check(
            ssx3_ee::s07::f_0026ea20__generated,
            replay_stream_read,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x26ea20, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus.write32(this_ + 0x0c, 0x100);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = (seed % 0x40) as u64;
            },
        );
    }

    #[test]
    fn replay_stream_alloc_buffer_matches() {
        check(
            ee::s07::f_0026e908__generated,
            replay_stream_alloc_buffer,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x26e9_08, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(0x40 + (seed as u32 % 16) * 4);
                let _ = r.next_u32();
            },
        );
    }

    // ------------------------------------------------------------------
    // Replay.

    #[test]
    fn replay_enable_instant_replay_cameras_matches() {
        check(
            ee::s07::f_0026ef30__generated,
            replay_enable_instant_replay_cameras,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x26ef_30, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(this + 0x634, [0, 8, 9, 0xffff_ffff][(seed % 4) as usize]);
                // The camera trigger manager reads this global.
                clear(c, 0x004c_5830, 0x40);
            },
        );
    }

    // --------------------------------------------------------------- replay

    #[test]
    fn replay_stream_peek_int_matches() {
        check(
            ee::s07::f_0026ea68__generated,
            replay_stream_peek_int,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x26ea);
                let s = DATA;
                let vt = put_vtable(c, &mut r, DATA + 0x2000, &[0x10]);
                c.bus.write32(s, vt);
                c.bus.write32(s + 0xc, r.next_u32());
                fill(c, &mut r, DATA + 0x40, 16);
                c.g[A0][0] = sx(s);
                c.g[A2][0] = 4;
            },
        );
    }

    #[test]
    fn replay_save_resume_snapshot_matches() {
        check(
            ee::s07::f_0026f7f8__generated,
            replay_save_resume_snapshot,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x26f7f8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x800);
                c.bus
                    .write32(this + 0x3d0, if seed % 3 == 0 { 0 } else { DATA + 0x3000 });
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn replay_snapshot_empty_hook_a_matches() {
        leaf(
            ssx3_ee::s07::f_0026ed88__generated,
            replay_snapshot_empty_hook_a,
            10,
        );
    }

    #[test]
    fn replay_free_buffers_matches() {
        check(
            ee::s07::f_0026f428__generated,
            replay_free_buffers,
            60,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x26f4_28);
                let count = seed % 4;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 8, count);
                for i in 0..count {
                    let e = DATA + 0x2c + i * 0x38;
                    c.bus.write32(e + 4, 0);
                    fill(c, &mut r, e, 0xe);
                }
                fill(c, &mut r, DATA + 0x3b0, 3);
                fill(c, &mut r, DATA + 0x3bc, 3);
            },
        );
    }

    #[test]
    fn replay_apply_camera_matches() {
        check(
            ee::s07::f_0026edd8__generated,
            replay_apply_camera,
            100,
            &|c, seed| {
                let _ = base_3(c, seed, 0x26_edd8);
                globals(c);
                const LIST: u32 = DATA + 0x4000;
                let n = (seed % 3) as u32;
                // world -> +0x84 (game) -> +0x84 (rider list)
                let game = WORLD + 0x1000;
                w(c, WORLD + 0x84, game);
                w(c, game + 0x84, LIST);
                w(c, LIST, n);
                for i in 0..3u32 {
                    let rider = DATA + 0x5000 + 0x100 * i;
                    w(c, LIST + 4 + 4 * i, rider);
                    w(c, rider + 0xa8, DATA + 0x6000 + 0x100 * i);
                }
                w(c, 0x0044_5438, 0);
                w(c, DATA + 0x630, [0, 1, 2][seed as usize % 3]);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn replay_restore_game_cameras_matches() {
        check(
            ee::s07::f_0026eea0__generated,
            replay_restore_game_cameras,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x26ee, 0);
                env(c);
                let this = DATA + 0x400;
                let list = DATA + 0x2_4000;
                let n = (seed % 3) as u32;
                w32(c, list, n);
                for i in 0..n {
                    let rider = DATA + 0x3000 + 0x100 * i;
                    w32(c, DATA + 0x2_4000 + 4 + 4 * i, rider);
                    w32(c, rider + 0xa8, DATA + 0x2800 + 0x100 * i);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn replay_init_matches() {
        check(
            ee::s07::f_0026f180__generated,
            replay_init,
            4,
            &|c, seed| {
                let _ = base_4(c, seed, 0x26f180, 4);
                c.g[A0][0] = OBJ as u64;
                fill(c, &mut Rng(seed as u64 ^ 0x26f1), OBJ, 0x120 / 4);
            },
        );
    }

    #[test]
    fn replay_stream_read_object_ref_matches() {
        strict(
            ee::s07::f_0026eb48__generated,
            replay_stream_read_object_ref,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26eb, 8);
                let this = DATA;
                let obj = DATA + 0x1000;
                c.bus.write32(this, obj);
                c.bus.write16(obj + 0x10, 0);
                c.bus.write32(obj + 0x14, FAKE);
                c.bus.write32(GAME_GP - 0x848, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x88, FAKE);
                c.bus.write32(GAME_GP + 0x16c8, DATA + 0x4000);
                c.bus.write32(DATA + 0x4000, DATA + 0x5000);
                c.bus.write32(DATA + 0x5000 + 8, DATA + 0x6000);
                c.g[A0][0] = sx(this);
                c.g[A2][0] = sx(r.next_u32() & 0xff);
            },
        );
    }

    // ---------------------------------------------------- Replay_HandleInput

    /// The replay controls in order of evaluation.
    const REPLAY_BINDINGS: [u32; 7] = [0x6a, 0x6b, 0x67, 0x66, 0x68, 0x69, 0x6c];

    /// Fixture of `Replay_HandleInput`: the replay at `DATA`, a world at
    /// `DATA + 0x1000` (its game `+0x84` at `DATA + 0x1200`: speed
    /// controller at `+0x48 -> +0x14`, an empty camera rider list at `+0x84`,
    /// a replay in state 14 at `+0x28` and a UI at `+0xa4` for
    /// `GameModule_EndReplayPlayback`), and every player's input binding at
    /// `DATA + 0x1800` whose input map runs, per binding, a two-word program
    /// "`v[0] = value`; stop" (the binding's result).
    fn replay_input_fixture(c: &mut Cpu, r: &mut Rng, controller: bool) -> u32 {
        // The executable's data: `InputVM_Execute` dispatches its opcodes
        // through the jump table at 0x48e330 (zero RAM would jump to 0).
        elf_image(c);
        let replay = DATA;
        for k in 0..0x1c0u32 {
            c.bus.write32(replay + 4 * k, 0);
        }
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
        crate::game::fe::feasyncfile::tests::heap_init_5(c);
        let world = DATA + 0x1000;
        let game = DATA + 0x1200;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, game);
        let h = DATA + 0x1300;
        c.bus.write32(game + 0x48, h);
        let obj = DATA + 0x1700;
        c.bus.write32(h + 0x14, if controller { obj } else { 0 });
        // The controller's vtable at a fixed address (a random one may land
        // on the fixture): slot `+0x10` is a fake.
        let _ = r.next_u32();
        let vt = DATA + 0x1780;
        c.bus.write16(vt + 0x10, 0);
        c.bus.write32(vt + 0x14, FAKE);
        c.bus.write32(obj, vt);
        c.bus.write32(GAME_GP.wrapping_add(0x2968), vt);
        let list = DATA + 0x1380;
        c.bus.write32(list, 0);
        c.bus.write32(game + 0x84, list);
        let ended = DATA + 0x1400;
        c.bus.write32(ended, 14);
        c.bus.write32(game + 0x28, ended);
        // The replay UI: one menu page whose close slot is a fake.
        let ui = DATA + 0x1480;
        for k in 0..0xa0u32 {
            c.bus.write32(ui + 4 * k, 0);
        }
        c.bus.write32(game + 0xa4, ui);
        let page = DATA + 0x1500;
        let page_vt = DATA + 0x1680;
        c.bus.write32(ui, 1);
        c.bus.write32(ui + 4, page);
        c.bus.write32(page + 0x12c, page_vt);
        c.bus.write32(page_vt + 0x3c, FAKE + 0x40);
        let binding = DATA + 0x1800;
        let map = DATA + 0x1900;
        for i in 0..4u32 {
            c.bus.write32(world + 0xb0 + 4 * i, binding);
        }
        c.bus.write32(binding, r.next_u32());
        c.bus.write32(binding + 4, map);
        c.bus.write32(map + 4, DATA + 0x1a00);
        c.bus.write32(map + 0x14, DATA + 0x1c00);
        for (i, &b) in REPLAY_BINDINGS.iter().enumerate() {
            let i = i as u32;
            c.bus.write32(DATA + 0x1a00 + 4 * b, 3 * i);
            c.bus.write32(DATA + 0x1c00 + 12 * i, 0);
            c.bus.write32(DATA + 0x1c00 + 12 * i + 8, 0x20);
        }
        replay_input_set(c, &[false; 7]);
        c.g[A0][0] = sx(replay);
        replay
    }

    /// The results of the seven bindings (in [`REPLAY_BINDINGS`] order).
    fn replay_input_set(c: &mut Cpu, on: &[bool; 7]) {
        for (i, &v) in on.iter().enumerate() {
            let bits = if v { 1.0f32.to_bits() } else { 0 };
            c.bus.write32(DATA + 0x1c00 + 12 * i as u32 + 4, bits);
        }
    }

    #[test]
    fn replay_handle_input_matches() {
        check(
            ee::s07::f_0026fb88__generated,
            replay_handle_input,
            320,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26fb88, 2);
                let replay = replay_input_fixture(c, &mut r, seed % 3 != 0);
                // The first binding that is on (or none), the others random.
                let first = (seed % 8) as usize;
                let mut on = [false; 7];
                for (i, v) in on.iter_mut().enumerate() {
                    *v = i == first || (i > first && r.next_u32() % 2 == 0);
                }
                replay_input_set(c, &on);
                let pick = |r: &mut Rng, xs: &[u32]| xs[(r.next_u32() as usize) % xs.len()];
                c.bus.write32(replay, pick(&mut r, &[3, 3, 0, 1, 5, 12]));
                c.bus
                    .write32(replay + 0x1c, pick(&mut r, &[0, 1, 2, 5, 0xffff_ffff]));
                c.bus.write32(replay + 0x20, r.next_u32() % 2);
                c.bus.write32(replay + 0x28, r.next_u32() % 4);
                let end = r.next_u32() % 3;
                c.bus.write32(replay + 0x484, end);
                c.bus.write32(
                    replay + 0x3cc,
                    if r.next_u32() % 2 == 0 { end } else { end + 1 },
                );
                c.bus.write32(replay + 0x630, r.next_u32() % 9);
                // Pending command / locked input on a few seeds.
                c.bus
                    .write32(replay + 0x618, if seed % 11 == 0 { 3 } else { 0 });
                c.bus
                    .write32(replay + 0x61c, if seed % 13 == 0 { 1 } else { 0 });
                c.bus
                    .write32(replay + 0x620, if seed % 17 == 0 { 1 } else { 0 });
                c.g[A1][0] = (r.next_u32() % 2) as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x69c), r.next_u32() % 4);
            },
        );
    }

    /// Holding the step binding: a step at the press, the first repeat after
    /// 10 frames, then one every other frame, in 60 Hz frames at any rate
    /// (each step within one frame of its 60 Hz time).
    #[test]
    fn replay_handle_input_step_repeat_follows_the_rate() {
        fn steps(hz: u32) -> Vec<f32> {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = machine(c, 3, 0x26fb88, 0);
            let replay = replay_input_fixture(c, &mut r, true);
            crate::tick::set_for_test(c, hz);
            c.bus.write32(replay, 3);
            c.bus.write32(replay + 0x484, 1);
            c.bus.write32(replay + 0x3cc, 2);
            let mut out = Vec::new();
            for t in 0..hz {
                let mut on = [false; 7];
                on[if t == 0 { 0 } else { 1 }] = true;
                replay_input_set(c, &on);
                c.g[A0][0] = sx(replay);
                c.g[A1][0] = 0;
                c.g[RA][0] = sx(crate::difftest::RA);
                c.pc = 0x0026_fb88;
                replay_handle_input(c);
                assert_eq!(
                    c.pc,
                    crate::difftest::RA,
                    "{hz} Hz, tick {t}: pc {:#x}, fake calls {}",
                    c.pc,
                    c.bus.read32(fakes::LOG_COUNT)
                );
                if c.bus.read32(replay + 0x618) == 5 {
                    out.push(t as f32 * 60.0 / hz as f32);
                    c.bus.write32(replay + 0x618, 0);
                }
            }
            out
        }
        let base = steps(60);
        assert_eq!(base.len(), 26, "{base:?}");
        assert_eq!(&base[..3], &[0.0, 10.0, 12.0]);
        for hz in [120, 240] {
            let s = steps(hz);
            assert!(s.len() >= 25, "{hz} Hz: {s:?}");
            for (a, b) in base.iter().zip(&s).take(25) {
                assert!((a - b).abs() <= 1.0, "{hz} Hz: {s:?} vs {base:?}");
            }
        }
    }

    /// Replay_Update with its real callees on their short paths (the
    /// game's jump tables from the ELF; a world whose player slot is empty;
    /// `+0x618` outside 1..8 except 7, so `ProcessCommand` only runs in
    /// state 3 where it is short). Covers: states 14/15, the timer, the
    /// "+0x618 == 7" drop to state 3, playback (3), 4-7, 10, 11, 13, and
    /// recording (0/12, also via 2 -> EndInstantReplay) with the finished
    /// and paused / not-on-a-60-frame-boundary paths. Not reached (their
    /// callees snapshot or step the whole world): the snapshot pushes,
    /// BeginPlayback (8, 9, +0x624), HandleInput in state 2, states >= 16,
    /// and a non-empty player slot.
    #[test]
    fn replay_update_matches() {
        use crate::difftest::{fill, scramble, Rng, GAME_GP};
        check(
            ee::s07::f_0026f4a8__generated,
            replay_update,
            300,
            &|c, seed| {
                let mut r = Rng(0x26f4a8 ^ (seed as u64) << 24);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA;
                fill(c, &mut r, this, 0x640 / 4);
                // World: player slot 0 (+0xb0) empty; the chain HandleInput
                // reads (+0x84 -> +0x48 -> +0x14).
                let (world, sub, sub2) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, sub);
                c.bus.write32(world + 0xb0, 0);
                c.bus.write32(sub + 0x48, sub2);
                c.bus.write32(sub2 + 0x14, 0);
                c.bus.write32(this + REPLAY_PLAYER, 0);
                const STATES: [u32; 13] = [14, 15, 3, 3, 4, 5, 6, 7, 11, 10, 13, 0, 12];
                let i = seed as usize % 16;
                let (state, f618) = match i {
                    0..=12 => (STATES[i], if i == 3 { 7 } else { 0 }),
                    13 => (2, 0),
                    14 => (0, 0),
                    _ => (12, 0x1_0000),
                };
                c.bus.write32(this + REPLAY_STATE, state);
                c.bus.write32(this + REPLAY_F618, f618);
                c.bus
                    .write32(this + REPLAY_TIMER, (seed / 16 % 3).wrapping_sub(1));
                // State 2: never past the last frame (EndInstantReplay path).
                c.bus.write32(this + REPLAY_LAST, 0x7fff_ffff);
                c.bus.write32(this + REPLAY_F620, 0);
                // Recording variants.
                let rec = seed / 48 % 4;
                c.bus.write32(this + REPLAY_F60C, (rec == 0) as u32);
                c.bus.write32(this + REPLAY_F608, 1);
                c.bus.write32(this + REPLAY_F624, 0);
                c.bus.write32(this + REPLAY_PAUSED, (rec == 1) as u32);
                let frame = 60 * (seed % 7) + 1 + seed % 59;
                c.bus.write32(this + REPLAY_FRAME, frame);
                c.g[A0][0] = sx(this);
                c.pc = 0x0026_f4a8;
            },
        );
    }
}
