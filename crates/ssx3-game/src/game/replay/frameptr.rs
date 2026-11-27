//! `src/replay/frameptr.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
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
pub(crate) use crate::game::dirtysock::tagsunk::lw;
pub(crate) use crate::game::dirtysock::tagsunk::p_2 as p;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::feasyncfile::gp_12 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::replay::playbackman::oc683_direct;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_4;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::sound::bankmonitor::p as p_2;
pub(crate) use crate::game::sound::bankmonitor::p as p_3;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
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
    (0x0026_f228, replay_reset, "Replay_Reset"),
    (0x0026_efb8, replay_ctor, "Replay_ctor"),
    (
        0x0026_f498,
        replay_empty_reset_hook,
        "Replay_EmptyResetHook",
    ),
    (
        0x0026_ec08,
        replay_stream_read_node_ref,
        "ReplayStream_ReadNodeRef",
    ),
    (
        0x0026_ed90,
        replay_snapshot_empty_hook_b,
        "Replay_SnapshotEmptyHookB",
    ),
    (0x0026_f088, replay_dtor_2, "Replay_dtor"),
    (
        0x0026_f898,
        replay_set_controller_index,
        "Replay_SetControllerIndex",
    ),
    (0x0026_fe50, replay_process_command, "Replay_ProcessCommand"),
    (
        0x0026_fa78,
        replay_update_camera_input_2,
        "Replay_UpdateCameraInput",
    ),
    (
        0x0026_eab8,
        replay_stream_decompress_4,
        "ReplayStream_Decompress",
    ),
    (
        0x0026_f850,
        replay_restore_resume_snapshot_4,
        "Replay_RestoreResumeSnapshot",
    ),
    (0x0026_f8a0, replay_begin_playback_3, "Replay_BeginPlayback"),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1;

    pub(crate) use crate::game::be::beintmission::callee::REPLAY_APPLY_CAMERA;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::EA_MEM_FREE_2;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::GAME_CAMERA_SET_CONTROLLER;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::REPLAY_ADD_SNAPSHOT;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::REPLAY_FREE_SNAPSHOT;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::REPLAY_RESTORE_SNAPSHOT;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::REPLAY_STREAM_WRITE;

    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;

    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::REPLAY_SET_STATE_2;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::REPLAY_STREAM_CTOR;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::WORLD_STATE_STREAM_RESET;

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

pub(crate) const OC423_REPLAY_EMPTY_RESET_HOOK: Callee = Callee {
    addr: 0x0026_f498,
    f: ee::s07::Replay_EmptyResetHook,
};

pub(crate) const OC423_REPLAY_SET_STATE: Callee = Callee {
    addr: 0x0026_f4a0,
    f: ee::s07::Replay_SetState,
};

pub(crate) const OC423_REPLAY_CACHE_RESET: Callee = Callee {
    addr: 0x0026_d168,
    f: ee::s07::ReplayCache_Reset,
};

pub(crate) const OC423_SNAPSHOT_LIST_CLEAR: Callee = Callee {
    addr: 0x0026_e5c0,
    f: ee::s07::SnapshotList_Clear,
};

pub(crate) const OC423_SNAPSHOT_LIST_PUSH_BACK: Callee = Callee {
    addr: 0x0026_e5d0,
    f: ee::s07::SnapshotList_PushBack,
};

/// `void Replay_Reset(Replay *replay)` (0x26f228): resets the replay
/// recorder/player unless it is in state 14 or 15 (`SaveState` 15 mid-reset):
/// sets state 15 and the replay state to 10, clears all playback and write
/// bookkeeping fields, releases and re-initialises the two replay caches (their
/// highlight slots at `+0x494`, stride 0xb4, each with 0xac bytes of data at
/// `+0x498`), empties the frame list (`+0x3b0`) and the free list (`+0x3bc`),
/// refills the free list with the `MaxFrames` (+8) frames at `+0x2c` (stride
/// 0x38) and sets the highlight/chapter limits (3 and 3 when the AI object's
/// field +0x7c is 1, else 1 and 0).
pub fn replay_reset(c: &mut Cpu) {
    /// Fields cleared to zero, in the compiler's store order.
    const CLEARED: [u32; 32] = [
        0x28, 0x608, 0x484, 0x3c8, 0x3cc, 0x488, 0x3d4, 0x3ac, 0x3d0, 0x1c, 0x20, 0x618, 0x61c,
        0x620, 0x624, 0x63c, 0x650, 0x640, 0x644, 0x648, 0x638, 0x64c, 0x654, 0x658, 0x65c, 0x660,
        0x664, 0x66c, 0x674, 0x678, 0x67c, 0x668,
    ];
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        c.g[V0][0] ^= 1;
        if c.g[V0][0] == 0 {
            // States 14 and 15: nothing to reset.
            frame.restore(c, S0, 0x80);
        } else {
            c.cycles += 51;
            c.g[S2][0] = 0;
            OC423_REPLAY_EMPTY_RESET_HOOK.call(c, 0x0026_f274)?;
            c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3b0));
            c.g[V0][0] = 0xf;
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V0][0]);
            c.g[A1][0] = 0xa;
            OC423_REPLAY_SET_STATE.call(c, 0x0026_f28c)?;
            c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3bc));
            c.g[S7][0] = u64::MAX;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x494));
            c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x498));
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48c));
            let this = c.g[S0][0] as u32;
            for off in [0x60c, 0x610, 0x614, 0x628, 0x62c] {
                c.sw(this.wrapping_add(off), 0);
            }
            for off in CLEARED {
                c.sw(this.wrapping_add(off), 0);
            }
            // The two highlight slots: release the cache, clear the slot.
            loop {
                c.cycles += 3;
                c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    OC423_REPLAY_CACHE_RESET.call(c, 0x0026_f34c)?;
                }
                c.sw((c.g[S1][0] as u32).wrapping_add(0xb0), 0);
                c.cycles += 12;
                c.g[A0][0] = c.g[S4][0];
                c.sw(c.g[S1][0] as u32, c.g[S7][0]);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0xac;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0xb4));
                call_ptr(c, 0x003e_6448, 0x0026_f368)?; // EA_MemSet
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(4));
                c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < 2) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_f338);
                }
            }
            c.cycles += 8;
            c.g[A0][0] = c.g[S6][0];
            c.g[S2][0] = 0;
            OC423_SNAPSHOT_LIST_CLEAR.call(c, 0x0026_f38c)?;
            c.g[A0][0] = c.g[S5][0];
            OC423_SNAPSHOT_LIST_CLEAR.call(c, 0x0026_f394)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
            if (c.g[V0][0] as i64) > 0 {
                // Refill the free list with the frames.
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S5][0];
                    OC423_SNAPSHOT_LIST_PUSH_BACK.call(c, 0x0026_f3ac)?;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_f3a0);
                    }
                }
            }
            c.cycles += 9;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x630), 0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x634), 0);
            c.g[A1][0] = 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x7c));
            c.g[A2][0] = 3;
            if c.g[V0][0] != c.g[A1][0] {
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0);
            } else {
                c.cycles += 3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x80);
        }
        c.cycles += 10;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay* Replay_ctor(Replay* self)` (0x26efb8): constructs the 16 replay frames (0x38
/// bytes from `+0x2c`), the two snapshot lists (`+0x3b0`, `+0x3bc`) and the two highlight
/// slots (0xb4 bytes from `+0x494`), enters replay state 15 and clears the two cache
/// words at `+0x48c`. Returns `self`.
pub fn replay_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2c));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = 0xf;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = 0xffffffffffffffff;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        'l_26efe8: loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            Callee {
                addr: 0x0026d5e8,
                f: ee::s07::WorldStateStream_ctor,
            }
            .call(c, 0x0026eff4)?;
            let t = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0026efe8);
                }
                continue 'l_26efe8;
            } else {
                c.cycles += 9;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3b0));
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x494));
                Callee {
                    addr: 0x0026e5a8,
                    f: ee::s07::SnapshotList_ctor,
                }
                .call(c, 0x0026f008)?;
                c.g[S1][0] = 0x1;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3bc));
                c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x48c));
                Callee {
                    addr: 0x0026e5a8,
                    f: ee::s07::SnapshotList_ctor,
                }
                .call(c, 0x0026f018)?;
                c.g[S3][0] = 0xffffffffffffffff;
                'l_26f020: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    Callee {
                        addr: 0x00272288,
                        f: ee::s07::Replay_HighlightSlotInit,
                    }
                    .call(c, 0x0026f02c)?;
                    let t = c.g[S1][0] != c.g[S3][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026f020);
                        }
                        continue 'l_26f020;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = 0xf;
                        Callee {
                            addr: 0x0026f4a0,
                            f: ee::s07::Replay_SetState,
                        }
                        .call(c, 0x0026f040)?;
                        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = 0x1;
                        'l_26f048: loop {
                            c.cycles += 7;
                            c.sw(c.g[V0][0] as u32, 0u64);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[V1][0] as i64) >= 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0026f048);
                                }
                                continue 'l_26f048;
                            } else {
                                c.cycles += 9;
                                c.g[V0][0] = c.g[S2][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                let t = c.g[RA][0] as u32;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                                c.pc = t;
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    })
}

/// `Replay_EmptyResetHook(void)` (0x26f498): empty.
pub fn replay_empty_reset_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `sNode_EC08* ReplayStream_ReadNodeRef(stream, owner)` (0x26ec08): reads an
/// 8-byte `{id, type}` record through the stream's virtual `read`. An id of
/// -1 (`~id == 0`) gives no node and the result is 0. Otherwise the id is
/// resolved through the ref table behind `gp+0x16c8` (the same lookup as
/// `world_state_read_dead_objects`): the node's owner (+0xc) becomes `owner`
/// and its flags (+0x8) keep bits 0xFFFF0300 and take `type & 0xFCFF`.
pub fn replay_stream_read_node_ref(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[A2][0] = 8;
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = 0;
        let stream = c.g[A0][0] as u32;
        c.g[A3][0] = c.lw(stream);
        let rec = sp(c);
        c.sw(rec, u64::MAX);
        // stream->read(&rec, 8)
        let vt = c.g[A3][0] as u32;
        c.g[V1][0] = c.lh(vt.wrapping_add(0x10));
        c.g[V0][0] = c.lw(vt.wrapping_add(0x14));
        let read = c.g[V0][0] as u32;
        c.g[A0][0] = sx(stream.wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, read, 0x0026_ec48)?;
        let rec = sp(c);
        let id = c.lw(rec);
        c.g[A2][0] = id;
        c.g[V0][0] = !id;
        let none = c.g[V0][0] == 0;
        let ctx_slot = (c.g[GP][0] as u32).wrapping_add(0x16c8);
        c.g[A1][0] = c.lw(ctx_slot);
        if !none {
            c.cycles += 8;
            c.g[V0][0] = sx(((c.g[A2][0] & 0xff) as u32) << 2);
            let ctx = c.g[A1][0] as u32;
            c.g[V1][0] = c.lw(ctx);
            let table = c.g[V1][0] as u32;
            c.g[A0][0] = c.lw(table.wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let entry_slot = c.g[V0][0] as u32;
            c.g[V1][0] = c.lw(entry_slot);
            let entry_null = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            // Node pointer in v0 (0 when the table holds none).
            let mut has_node = false;
            if !entry_null {
                c.cycles += 7;
                let entry = c.g[V1][0] as u32;
                c.g[A0][0] = c.lw(entry.wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let node_slot = c.g[V0][0] as u32;
                c.g[V1][0] = c.lw(node_slot);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                has_node = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            }
            if !has_node {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 10;
            c.g[S0][0] = c.g[V0][0];
            let node = c.g[S0][0] as u32;
            let owner = c.g[S1][0];
            c.sw(node.wrapping_add(0xc), owner);
            c.g[A0][0] = 0xffff_ffff_ffff_0300;
            c.g[V1][0] = c.lw(node.wrapping_add(0x8));
            let type_slot = sp(c).wrapping_add(0x4);
            c.g[V0][0] = c.lhu(type_slot);
            c.g[V1][0] &= c.g[A0][0];
            c.g[V0][0] &= 0xfcff;
            c.g[V1][0] |= c.g[V0][0];
            let flags = c.g[V1][0];
            c.sw(node.wrapping_add(0x8), flags);
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_SnapshotEmptyHookB` (0x26ed90): an empty hook.
pub fn replay_snapshot_empty_hook_b(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Replay_SetControllerIndex` (0x26f898): `self->controllerIndex (+0x28) = a1`.
pub fn replay_set_controller_index(c: &mut Cpu) {
    c.cycles += 2;
    let this = c.g[A0][0] as u32;
    let index = c.g[A1][0];
    c.sw(this.wrapping_add(0x28), index);
    ret(c);
}

/// `Replay_EndPlayback` (0x26f980): the replay's stop call.
pub(crate) const OC532_END_PLAYBACK: Callee = Callee {
    addr: 0x0026_f980,
    f: ee::s07::f_0026f980,
};

/// `Replay_EmptyResetHook` (0x26f498).
pub(crate) const OC532_EMPTY_RESET_HOOK: Callee = Callee {
    addr: 0x0026_f498,
    f: ee::s07::f_0026f498,
};

/// `Replay_FreeBuffers` (0x26f428), the `cReplay_purge` of the reference.
pub(crate) const OC532_FREE_BUFFERS: Callee = Callee {
    addr: 0x0026_f428,
    f: ee::s07::f_0026f428,
};

/// `cReplay__cReplay` (0x26f088), listed as `Replay_dtor`: stops a running
/// replay (`*self` in 1..9), syncs and purges it, destroys the highlight
/// array at `+0x494` (stride 0xb4) and the frame array at `+0x2c` (stride
/// 0x38, the virtual destructor in slot 0x38/0x3c of each frame's vtable),
/// then frees `self` when `flags & 1` is set.
pub fn replay_dtor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S1][0] as u32;
        c.g[V0][0] = c.lw(this);
        let state_zero = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !state_zero {
            c.cycles += 1;
            c.g[V1][0] = ((c.g[V0][0] as i64) < 10) as u64;
        }
        // Stop the replay if it was running.
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            OC532_END_PLAYBACK.call(c, 0x0026_f0c8)?;
        }
        c.g[A0][0] = c.g[S1][0];
        // Sync and purge, then the EmptyResetHook/FreeBuffers pair.
        c.cycles += 8;
        c.g[S3][0] = c.g[S0][0] & 0x1;
        OC532_EMPTY_RESET_HOOK.call(c, 0x0026_f0d4)?;
        c.g[S2][0] = sx(this.wrapping_add(0x2c));
        c.g[A0][0] = c.g[S1][0];
        OC532_FREE_BUFFERS.call(c, 0x0026_f0e0)?;
        // Highlights at +0x494: destroy from the end, stride 0xb4.
        c.g[V1][0] = sx(this.wrapping_add(0x494));
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = sx(this.wrapping_add(0x5fc));
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb4));
                loop {
                    c.cycles += 7;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb4));
                        if event_due(c) {
                            deliver_event_here(c, 0x0026_f100);
                        }
                        continue;
                    }
                    break;
                }
            }
        }
        // Frames at +0x2c: destroy from the end, stride 0x38.
        c.cycles += 2;
        let frames_zero = c.g[S2][0] == 0;
        c.g[S0][0] = sx(this.wrapping_add(0x3ac));
        if !frames_zero {
            c.cycles += 2;
            let last = c.g[S2][0] == c.g[S0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x38));
            if !last {
                c.cycles += 1;
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    let slot = c.g[S0][0] as u32;
                    c.g[V0][0] = c.lw(slot);
                    let obj = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lh(obj.wrapping_add(0x38));
                    c.g[V1][0] = c.lw(obj.wrapping_add(0x3c));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(slot.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0026_f148)?;
                    let more = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x38));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_f130);
                    }
                }
            }
        }
        // The self-delete when flags & 1.
        c.cycles += 2;
        let no_delete = c.g[S3][0] == 0;
        frame.restore(c, S0, 0x40);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0026_f160)?;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_ProcessCommand` (0x0026fe50): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn replay_process_command(c: &mut Cpu) {
    if c.pc != 0x0026fe50 {
        return ee::s07::f_0026fe50__generated(c);
    }
    guest(c, |c| {
        'b_270020: {
            'b_27001c: {
                'b_26fffc: {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x618));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = (c.g[A0][0] < 0x8u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[S1][0] = 0u64;
                    if t {
                        break 'b_26fffc;
                    } else {
                        'b_26ffec: {
                            'b_26ffe8: {
                                'b_26ffc8: {
                                    'b_26ffa8: {
                                        'b_26ff98: {
                                            'b_26ff70: {
                                                'b_26ff5c: {
                                                    'b_26fef0: {
                                                        'b_26fec4: {
                                                            c.cycles += 7;
                                                            c.g[V0][0] = 0x480000;
                                                            c.g[V1][0] =
                                                                sx((c.g[A0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1790));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                            let t = c.g[A0][0] as u32;
                                                            match t {
                                                                0x0026fe94 => {
                                                                    c.cycles += 10;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    c.g[RA][0] = sx(0x0026fea0);
                                                                    c.g[A1][0] = 0u64;
                                                                    Callee { addr: 0x002702f8, f: ee::s07::Replay_SeekToSnapshot }.call(c, 0x0026fea0)?;
                                                                    c.g[S1][0] = 0x1;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    c.g[RA][0] = sx(0x0026feb0);
                                                                    c.g[A1][0] = 0x1;
                                                                    Callee { addr: 0x002703f0, f: ee::s07::Replay_StepFrame }.call(c, 0x0026feb0)?;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x484));
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    if t {
                                                                        break 'b_26ff5c;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = true;

                                                                        if t {
                                                                            break 'b_26ff98;
                                                                        } else {
                                                                            break 'b_26fec4;
                                                                        }
                                                                    }
                                                                }
                                                                0x0026fec4 => {
                                                                    break 'b_26fec4;
                                                                }
                                                                0x0026fef0 => {
                                                                    break 'b_26fef0;
                                                                }
                                                                0x0026ff70 => {
                                                                    break 'b_26ff70;
                                                                }
                                                                0x0026ffa8 => {
                                                                    break 'b_26ffa8;
                                                                }
                                                                0x0026ffc8 => {
                                                                    break 'b_26ffc8;
                                                                }
                                                                0x0026ffe8 => {
                                                                    break 'b_26ffe8;
                                                                }
                                                                0x0027001c => {
                                                                    break 'b_27001c;
                                                                }
                                                                _ => {
                                                                    c.jump_out(
                                                                        t, 0x0026fe50, 0x00270070,
                                                                    );
                                                                    return Ok(());
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 9;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x0026fed0);
                                                        c.g[S1][0] = 0x1;
                                                        Callee {
                                                            addr: 0x00270378,
                                                            f: ee::s07::Replay_SeekToEnd,
                                                        }
                                                        .call(c, 0x0026fed0)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x0026fedc);
                                                        c.g[A1][0] = 0x1;
                                                        Callee {
                                                            addr: 0x002703f0,
                                                            f: ee::s07::Replay_StepFrame,
                                                        }
                                                        .call(c, 0x0026fedc)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x484));
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        if t {
                                                            break 'b_26ff5c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;

                                                            if t {
                                                                break 'b_26ff98;
                                                            } else {
                                                                break 'b_26fef0;
                                                            }
                                                        }
                                                    }
                                                    'b_26ff48: {
                                                        'b_26ff28: {
                                                            c.cycles += 10;
                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x484));
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x0026ff00);
                                                            c.g[S1][0] = 0x1;
                                                            Callee {
                                                                addr: 0x0026e448,
                                                                f: ee::s07::Replay_FindSnapshot,
                                                            }
                                                            .call(c, 0x0026ff00)?;
                                                            c.g[V1][0] = c.g[V0][0];
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x484));
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x488),
                                                                c.g[V1][0],
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x30));
                                                            if c.g[A0][0] != c.g[V0][0] {
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x14));
                                                                break 'b_26ff28;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x0026ff20);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                Callee { addr: 0x0026e568, f: ee::s07::Replay_SeekPrevSnapshot }.call(c, 0x0026ff20)?;
                                                                let t = true;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                if t {
                                                                    break 'b_26ff48;
                                                                } else {
                                                                    break 'b_26ff28;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[A0][0] = c.g[S0][0];
                                                            break 'b_26ff48;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x30));
                                                            let t = c.g[V0][0] != c.g[A0][0];
                                                            c.g[A0][0] = c.g[S0][0];
                                                            if t {
                                                                break 'b_26ff48;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[RA][0] = sx(0x0026ff44);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                Callee { addr: 0x0026e568, f: ee::s07::Replay_SeekPrevSnapshot }.call(c, 0x0026ff44)?;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                break 'b_26ff48;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[RA][0] = sx(0x0026ff50);
                                                    c.g[A1][0] = 0x1;
                                                    Callee {
                                                        addr: 0x002703f0,
                                                        f: ee::s07::Replay_StepFrame,
                                                    }
                                                    .call(c, 0x0026ff50)?;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x484)
                                                        );
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    if t {
                                                        break 'b_26ff98;
                                                    } else {
                                                        break 'b_26ff5c;
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[RA][0] = sx(0x0026ff68);
                                                c.g[A1][0] = 0x6;
                                                Callee {
                                                    addr: 0x0026f4a0,
                                                    f: ee::s07::Replay_SetState,
                                                }
                                                .call(c, 0x0026ff68)?;
                                                let t = true;
                                                c.sw((c.g[S0][0] as u32).wrapping_add(0x618), 0u64);
                                                if t {
                                                    break 'b_270020;
                                                } else {
                                                    break 'b_26ff70;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x488));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[S1][0] = 0x1;
                                            if t {
                                                break 'b_27001c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[RA][0] = sx(0x0026ff88);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x0026e528,
                                                    f: ee::s07::Replay_SeekNextSnapshot,
                                                }
                                                .call(c, 0x0026ff88)?;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[RA][0] = sx(0x0026ff94);
                                                c.g[A1][0] = 0x1;
                                                Callee {
                                                    addr: 0x002703f0,
                                                    f: ee::s07::Replay_StepFrame,
                                                }
                                                .call(c, 0x0026ff94)?;
                                                c.g[A0][0] = c.g[S0][0];
                                                break 'b_26ff98;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x0026ffa0);
                                        c.g[A1][0] = 0x5;
                                        Callee {
                                            addr: 0x0026f4a0,
                                            f: ee::s07::Replay_SetState,
                                        }
                                        .call(c, 0x0026ffa0)?;
                                        let t = true;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x618), 0u64);
                                        if t {
                                            break 'b_270020;
                                        } else {
                                            break 'b_26ffa8;
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x0026ffb4);
                                    c.g[A1][0] = 0u64;
                                    Callee {
                                        addr: 0x002703f0,
                                        f: ee::s07::Replay_StepFrame,
                                    }
                                    .call(c, 0x0026ffb4)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x0026ffc0);
                                    c.g[A1][0] = 0x4;
                                    Callee {
                                        addr: 0x0026f4a0,
                                        f: ee::s07::Replay_SetState,
                                    }
                                    .call(c, 0x0026ffc0)?;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x618), 0u64);
                                    if t {
                                        break 'b_270020;
                                    } else {
                                        break 'b_26ffc8;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V0][0] = 0x3;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_26ffec;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0026ffe0);
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x0026f4a0,
                                        f: ee::s07::Replay_SetState,
                                    }
                                    .call(c, 0x0026ffe0)?;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x618), 0u64);
                                    if t {
                                        break 'b_270020;
                                    } else {
                                        break 'b_26ffe8;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.g[S0][0];
                            break 'b_26ffec;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0026fff4);
                        c.g[A1][0] = 0x3;
                        Callee {
                            addr: 0x0026f4a0,
                            f: ee::s07::Replay_SetState,
                        }
                        .call(c, 0x0026fff4)?;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x618), 0u64);
                        if t {
                            break 'b_270020;
                        } else {
                            break 'b_26fffc;
                        }
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = 0x3;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_27001c;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x604), c.g[V0][0]);
                    c.g[RA][0] = sx(0x0027001c);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x002703f0,
                        f: ee::s07::Replay_StepFrame,
                    }
                    .call(c, 0x0027001c)?;
                    break 'b_27001c;
                }
            }
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x618), 0u64);
            break 'b_270020;
        }
        'b_27005c: {
            c.cycles += 2;
            let t = c.g[S1][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_27005c;
            } else {
                c.cycles += 13;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa8));
                c.g[RA][0] = sx(0x00270040);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001620d0, 0x00270040)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00270054);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00270054)?;
                c.g[RA][0] = sx(0x0027005c);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0022e840, 0x0027005c)?;
                break 'b_27005c;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void Replay_UpdateCameraInput(Replay* self)` (0x26fa78): while the
/// replay is in state 3, poll the camera controller at index `+0x28` and
/// reset it on the button presses of the replay pad.
pub fn replay_update_camera_input_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x30);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c) ^ 3;
        'b_26fb74: {
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
                break 'b_26fb74;
            }
            'b_26fad4: {
                c.cycles += 10;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x28)).lw(c);
                c.g[A1][0] = 0x6c;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xb0)).lw(c);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0032_0c48, 0x0026_fac0)?;
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                if none {
                    break 'b_26fad4;
                }
                c.cycles += 3;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                call_ptr(c, 0x0022_e840, 0x0026_fad4)?;
            }
            c.cycles += 9;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x630)).lw(c);
            c.g[V1][0] = 0x44_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5438));
            c.g[A1][0] = 0xb;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            if c.g[A0][0] != c.g[A1][0] {
                frame.restore(c, S0, 0x20);
                break 'b_26fb74;
            }
            'b_26fb60: {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                abi::set_fpr(c, 20, FpuF32::ZERO);
                c.g[A1][0] = 0x31;
                call_ptr(c, 0x0032_0bf0, 0x0026_fb08)?;
                let eq = c_eq_f0_f20(c);
                c.g[A0][0] = c.g[S1][0];
                if !eq {
                    break 'b_26fb60;
                }
                c.cycles += 6;
                c.g[A1][0] = 0x30;
                call_ptr(c, 0x0032_0bf0, 0x0026_fb20)?;
                let eq = c_eq_f0_f20(c);
                c.g[A0][0] = c.g[S1][0];
                if !eq {
                    break 'b_26fb60;
                }
                c.cycles += 6;
                c.g[A1][0] = 0x32;
                call_ptr(c, 0x0032_0bf0, 0x0026_fb38)?;
                let eq = c_eq_f0_f20(c);
                c.g[A0][0] = c.g[S1][0];
                if !eq {
                    break 'b_26fb60;
                }
                c.cycles += 6;
                c.g[A1][0] = 0x33;
                call_ptr(c, 0x0032_0bf0, 0x0026_fb50)?;
                let eq = c_eq_f0_f20(c);
                frame.restore(c, S0, 0x20);
                if eq {
                    break 'b_26fb74;
                }
                break 'b_26fb60;
            }
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
            call_ptr(c, 0x0022_e840, 0x0026_fb70)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_fpr(c, 20, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `c.c.eq.s f0, f20` followed by the FCC latch: the compare result.
pub(crate) fn c_eq_f0_f20(c: &mut Cpu) -> bool {
    let eq = FpuF32(c.f[0]).eq_s(FpuF32(c.f[20]));
    abi::set_fcc(c, eq);
    eq
}

/// `cReplayFramePtr_loadBlock` (0x26eab8), listed as `ReplayStream_Decompress`:
/// reads a block tag from `src` and its size from `*pp`, advances `*pp` past
/// the 4-byte size, Huffman-decodes `size` bytes into the stream's block
/// (`self+4`), moves `*pp` past them, sets `self+8` (WriteData) to the end of
/// the decoded bytes and rewinds the stream. Returns 1.
pub fn replay_stream_decompress_4(c: &mut Cpu) {
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
        // MEM_copy(&hdr.tag, src, 4)
        call_ptr(c, 0x003e_6574, 0x0026_eae0)?;
        let pp = c.g[S0][0] as u32;
        c.g[A1][0] = c.lw(pp);
        c.g[A0][0] = c.g[SP][0] | 0x4;
        c.g[A2][0] = 4;
        // MEM_copy(&hdr.size, *pp, 4)
        call_ptr(c, 0x003e_6574, 0x0026_eaf0)?;
        c.g[A0][0] = c.lw(pp);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        let advanced = c.g[A0][0];
        c.sw(pp, advanced);
        // HUFF_decode(*pp, 0, self->Block)
        let self_ptr = c.g[S1][0] as u32;
        c.g[A2][0] = c.lw(self_ptr.wrapping_add(0x4));
        call_ptr(c, 0x002c_85a0, 0x0026_eb08)?;
        let size_slot = sp(c).wrapping_add(0x4);
        c.g[V1][0] = c.lw(size_slot);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.lw(pp);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let moved = c.g[A1][0];
        c.sw(pp, moved);
        let self_ptr = c.g[S1][0] as u32;
        c.g[V1][0] = c.lw(self_ptr.wrapping_add(0x4));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let write_data = c.g[V1][0];
        c.sw(self_ptr.wrapping_add(0x8), write_data);
        oc683_direct(0x0026_e9b0).call(c, 0x0026_eb2c)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cReplay_popReplaySaveFrame` (0x26f850), listed as
/// `Replay_RestoreResumeSnapshot`: when the save frame (`self+0x3d0`) is set,
/// restores it, frees it and clears the slot.
pub fn replay_restore_resume_snapshot_4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        let save_frame = c.lw(this.wrapping_add(0x3d0));
        c.g[A1][0] = save_frame;
        if save_frame != 0 {
            c.cycles += 7;
            oc683_direct(0x0026_db88).call(c, 0x0026_f874)?;
            c.g[A1][0] = c.lw(this.wrapping_add(0x3d0));
            c.g[A0][0] = c.g[S0][0];
            oc683_direct(0x0027_0238).call(c, 0x0026_f880)?;
            c.sw(this.wrapping_add(0x3d0), 0);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cReplay_startReplay(cReplay* self, int frame, int instantReplay,
/// int autoReplay)` (0x26f8a0). Does nothing while the replay is in a
/// transition (state 14 or 15). Otherwise: take a resume snapshot if none is
/// held (the field at +0x608), enter the replay state 7, clear the command
/// and auto-replay fields (+0x618, +0x61c), remember the instant flag (+0x620),
/// rewind to `frame`, step one frame, then enable the instant-replay cameras
/// (or the normal ones). A normal, non-automatic replay also starts the audio
/// side (`SSXAudioSystem_OnReplayBegin`).
pub fn replay_begin_playback_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);

        // ok = !(state == 14 || state == 15): the unsigned compare of the
        // sign-extended (state - 14) with 2.
        let state = Addr(c.g[S0][0] as u32).lw(c);
        let skip = sx((state as u32).wrapping_sub(0xe)) < 2;
        c.g[V0][0] = if skip { 0 } else { 1 };
        c.g[S3][0] = c.g[A3][0];

        if skip {
            // 'b_26f960 exit: the two restores, then the epilogue.
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
            c.cycles += 1;
            frame.restore(c, S1, 0x30);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x608).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                call_ptr(c, 0x0026_f7f8, 0x0026_f8f0)?; // Replay_SaveResumeSnapshot
            }
            c.g[A0][0] = c.g[S0][0];

            c.cycles += 13;
            c.g[A1][0] = 7;
            call_ptr(c, 0x0026_f4a0, 0x0026_f8fc)?; // Replay_SetState
            Addr(c.g[S0][0] as u32).off(0x618).sw(c, 0);
            c.g[A1][0] = c.g[S2][0];
            Addr(c.g[S0][0] as u32).off(0x61c).sw(c, 0);
            c.g[A0][0] = c.g[S0][0];
            let instant = c.g[S1][0];
            Addr(c.g[S0][0] as u32).off(0x620).sw(c, instant);
            call_ptr(c, 0x0027_02f8, 0x0026_f914)?; // Replay_SeekToSnapshot
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0027_03f0, 0x0026_f920)?; // Replay_StepFrame

            if c.g[S1][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ef30, 0x0026_f930)?; // Replay_EnableInstantReplayCameras
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ee68, 0x0026_f940)?; // Replay_EnableReplayCameras
            }

            c.cycles += 2;
            frame.restore(c, S0, 0x40);
            if c.g[S1][0] != 0 {
                c.cycles += 1;
                frame.restore(c, S1, 0x30);
            } else {
                c.cycles += 2;
                frame.restore(c, S1, 0x30);
                if c.g[S3][0] == 0 {
                    c.cycles += 4;
                    call_ptr(c, 0x0028_b180, 0x0026_f958)?; // SSXAudioSystem_GetInstance
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0028_f200, 0x0026_f960)?; // SSXAudioSystem_OnReplayBegin
                    c.cycles += 1;
                    frame.restore(c, S0, 0x40);
                    c.cycles += 1;
                    frame.restore(c, S1, 0x30);
                }
            }
        }

        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_BeginPlayback(Replay* self, int frame, int instant, int auto)`
/// (0x26f8a0), the C++ `cReplay_startReplay`. Unless the replay is already
/// in a playback state (14 or 15): saves the resume snapshot when none is
/// loaded, enters state 7, clears the command and auto-replay words, stores
/// `instant` (+0x620), seeks to `frame`, steps one frame and sets up the
/// instant-replay or the replay cameras; a full replay that is not an auto
/// replay also tells the audio system the replay begins.
#[cfg(test)] // duplicate of oc729/oc730, kept for its test only
pub fn replay_begin_playback_4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
        c.g[V0][0] ^= 0x1;
        let can_begin = c.g[V0][0] != 0;
        c.g[S3][0] = c.g[A3][0];
        // Where the epilogue chain is entered: 0 runs it all (`+1, lq s0`,
        // `+1, lq s1`, the rest), 1 skips the first step, 2 the first two.
        let mut entry = 0;
        'done: {
            if can_begin {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x608)).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[S0][0];
                } else {
                    c.cycles += 3;
                    call_ptr(c, 0x0026_f7f8, 0x0026_f8f0)?; // Replay_SaveResumeSnapshot
                    c.g[A0][0] = c.g[S0][0];
                }
                c.cycles += 13;
                c.g[A1][0] = 0x7;
                call_ptr(c, 0x0026_f4a0, 0x0026_f8fc)?; // Replay_SetState(self, 7)
                let this = c.g[S0][0] as u32;
                c.sw(this.wrapping_add(0x618), 0);
                c.g[A1][0] = c.g[S2][0];
                c.sw(this.wrapping_add(0x61c), 0);
                c.g[A0][0] = c.g[S0][0];
                c.sw(this.wrapping_add(0x620), c.g[S1][0]);
                call_ptr(c, 0x0027_02f8, 0x0026_f914)?; // Replay_SeekToSnapshot
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0027_03f0, 0x0026_f920)?; // Replay_StepFrame(self, 1)
                if c.g[S1][0] == 0 {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0026_ee68, 0x0026_f940)?; // Replay_EnableReplayCameras
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0026_ef30, 0x0026_f930)?; // Replay_EnableInstantReplayCameras
                }
                c.cycles += 2;
                let instant = c.g[S1][0] != 0;
                frame.restore(c, S0, 0x40);
                if instant {
                    entry = 1;
                    break 'done;
                }
                c.cycles += 2;
                let auto = c.g[S3][0] != 0;
                frame.restore(c, S1, 0x30);
                if auto {
                    entry = 2;
                    break 'done;
                }
                c.cycles += 4;
                call_ptr(c, 0x0028_b180, 0x0026_f958)?; // SSXAudioSystem_GetInstance
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0028_f200, 0x0026_f960)?; // SSXAudioSystem_OnReplayBegin
            }
        }
        if entry == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        if entry <= 1 {
            c.cycles += 1;
            frame.restore(c, S1, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
// The fixtures of the dropped (uninstalled) replay ports stay with their ports.
#[allow(dead_code)]
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

    pub(crate) use crate::game::dirtysock::tagsunk::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::put_vtable;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;

    pub(crate) use crate::game::fe::festatecharequipdetail::tests::world;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_2;
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

    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w_5 as w_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC423_CTRL;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC423_MAGIC;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc423_fake;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc423_fake_call;

    oc423_fake!(
        oc423_fake_replay_set_state,
        OC423_PREV_REPLAY_SET_STATE,
        0x0026_f4a0,
        ee::s07::f_0026f4a0__generated
    );

    oc423_fake!(
        oc423_fake_replay_cache_reset,
        OC423_PREV_REPLAY_CACHE_RESET,
        0x0026_d168,
        ee::s07::f_0026d168__generated
    );

    oc423_fake!(
        oc423_fake_snapshot_clear,
        OC423_PREV_SNAPSHOT_CLEAR,
        0x0026_e5c0,
        ee::s07::f_0026e5c0__generated
    );

    oc423_fake!(
        oc423_fake_snapshot_push_back,
        OC423_PREV_SNAPSHOT_PUSH_BACK,
        0x0026_e5d0,
        ee::s07::f_0026e5d0__generated
    );

    /// Resettable and non-resettable states (13/14/15/16), 0..3 frames, the
    /// two cache slots set or not, both AI player-count variants.
    #[test]
    fn replay_reset_matches() {
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026F4A0),
            oc423_fake_replay_set_state,
            &OC423_PREV_REPLAY_SET_STATE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026D168),
            oc423_fake_replay_cache_reset,
            &OC423_PREV_REPLAY_CACHE_RESET,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026E5C0),
            oc423_fake_snapshot_clear,
            &OC423_PREV_SNAPSHOT_CLEAR,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026E5D0),
            oc423_fake_snapshot_push_back,
            &OC423_PREV_SNAPSHOT_PUSH_BACK,
        );
        fakes::covered(
            ee::s07::f_0026f228__generated,
            replay_reset,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0026_f228, 8);
                w_2(c, OC423_CTRL, OC423_MAGIC);
                let replay = DATA;
                // Dirty every field the reset clears.
                for i in 0..0x700u32 {
                    w_2(c, replay + 4 * i, r.next_u32() | 1);
                }
                w_2(
                    c,
                    replay,
                    [13, 14, 15, 16, 0, 2, 13, 13][(seed % 8) as usize],
                );
                w_2(c, replay + 8, seed / 8 % 4);
                for slot in 0..2u32 {
                    let cache = if (seed / 32 + slot) % 2 == 0 {
                        0
                    } else {
                        DATA + 0x3000
                    };
                    w_2(c, replay + 0x48c + 4 * slot, cache);
                }
                let world = DATA + 0x4000;
                let manager = DATA + 0x4100;
                let ai = DATA + 0x4200;
                w_2(c, GAME_GP - 0x848, world);
                w_2(c, world + 0x84, manager);
                w_2(c, manager + 0xc, ai);
                w_2(c, ai + 0x7c, if seed % 3 == 0 { 1 } else { 2 });
                c.g[A0][0] = replay as u64;
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::oc434_setup;

    /// `Replay_ctor`: 16 frames, two snapshot lists, two highlight slots, the state
    /// change (21 fake calls, unwinding in any of them).
    #[test]
    fn replay_ctor_matches() {
        fakes::covered(
            ee::s07::f_0026efb8__generated,
            replay_ctor,
            24,
            &|c, seed| {
                let _ = oc434_setup(c, seed, 0x26efb8, 21, 0x0026_efb8);
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    #[test]
    fn replay_empty_reset_hook_matches() {
        check(
            ee::s07::f_0026f498__generated,
            replay_empty_reset_hook,
            20,
            &|c, seed| {
                machine(c, seed, 0x26f498, 0);
                c.pc = 0x0026_f498;
            },
        );
    }

    // Fixture of `world_state_read_dead_objects`: the stream object and its
    // vtable (`read` at slot 0x14, the `this` delta at 0x10), the table chain
    // behind `gp+0x16c8`, and the script the read fake serves (fake call n
    // returns script word n: the header count, then the refs).
    pub(crate) const RDO_STREAM: u32 = DATA + 0x20_0000;

    pub(crate) const RDO_VT: u32 = DATA + 0x20_0100;

    pub(crate) const RDO_CTX: u32 = DATA + 0x20_0200;

    pub(crate) const RDO_TA: u32 = DATA + 0x20_0300;

    pub(crate) const RDO_TB: u32 = DATA + 0x20_0400;

    pub(crate) const RDO_TC: u32 = DATA + 0x20_0800;

    pub(crate) const RDO_ENTRY: u32 = DATA + 0x20_0c00;

    pub(crate) const RDO_SCRIPT: u32 = DATA + 0x20_1000;

    pub(crate) const RDO_NODES: u32 = DATA + 0x20_3000;

    /// The stream object and its vtable: `read` at slot 0x14, the `this`
    /// delta at 0x10 (the read fakes write the buffer, not `this`).
    pub(crate) fn dead_stream_fixture(c: &mut Cpu, r: &mut Rng) {
        c.bus.write16(RDO_VT + 0x10, (r.next_u32() as u16) & !3);
        c.bus.write32(RDO_VT + 0x14, fakes::FAKE);
        c.bus.write32(RDO_STREAM, RDO_VT);
    }

    /// The ref table behind `gp+0x16c8`: ctx -> table A, [+8] -> table B (by
    /// the low byte of a ref), entry +0x1c -> table C (by `ref >> 8`). Refs
    /// below 0x4000 reach the nodes at `RDO_NODES`, zero for some entries.
    pub(crate) fn dead_table_fixture(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP + 0x16c8, RDO_CTX);
        c.bus.write32(RDO_CTX, RDO_TA);
        c.bus.write32(RDO_TA + 8, RDO_TB);
        c.bus.write32(RDO_ENTRY + 0x1c, RDO_TC);
        for lo in 0..256u32 {
            let entry = if lo % 3 == 0 { 0 } else { RDO_ENTRY };
            c.bus.write32(RDO_TB + 4 * lo, entry);
        }
        for j in 0..64u32 {
            let word = if j % 4 == 0 {
                0
            } else {
                // `(word >> 8) << 2` is the node address RDO_NODES + 4 * j.
                (((RDO_NODES >> 2) + j) << 8) | (r.next_u32() & 0xff)
            };
            c.bus.write32(RDO_TC + 4 * j, word);
        }
    }

    /// The read fake of `replay_stream_read_node_ref`: serves the scripted id
    /// and type (`{id, type}`), then logs the call.
    pub(crate) fn rnr_miss(c: &mut Cpu, target: u32) {
        let buf = c.g[A1][0] as u32;
        let id = c.bus.read32(RDO_SCRIPT);
        let ty = c.bus.read32(RDO_SCRIPT + 4);
        c.bus.write32(buf, id);
        c.bus.write32(buf + 4, ty);
        fakes::fake_call(c, target);
    }

    pub(crate) fn rnr_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x26_ec08, 0);
        c.hooks.miss = rnr_miss;
        let id = if seed % 4 == 0 {
            u32::MAX
        } else {
            r.next_u32() & 0x3fff
        };
        c.bus.write32(RDO_SCRIPT, id);
        c.bus.write32(RDO_SCRIPT + 4, r.next_u32());
        dead_stream_fixture(c, &mut r);
        dead_table_fixture(c, &mut r);
        c.g[A0][0] = RDO_STREAM as u64;
        c.g[A1][0] = (RDO_NODES + 0x1000) as u64;
    }

    #[test]
    fn replay_stream_read_node_ref_matches() {
        check(
            ee::s07::f_0026ec08__generated,
            replay_stream_read_node_ref,
            200,
            &rnr_setup,
        );
    }

    // Fixture of `replay_stream_decompress`: the stream (`Block` at +4,
    // `WriteData` at +8), the `char**` cursor and the source block: a tag at
    // `src`, then the size and the coded bytes at `*pp`.
    pub(crate) const RS_SELF: u32 = DATA + 0x20_8800;

    pub(crate) const RS_CURSOR: u32 = DATA + 0x20_8900;

    pub(crate) const RS_SRC: u32 = DATA + 0x20_8a00;

    pub(crate) const RS_BLOCK: u32 = DATA + 0x20_9000;

    pub(crate) const RS_CODED: u32 = DATA + 0x20_9800;

    pub(crate) fn replay_stream_decompress_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x26_eab8, 0);
        c.bus.write32(RS_SELF + 0x4, RS_BLOCK);
        c.bus.write32(RS_SELF + 0x8, 0);
        c.bus.write32(RS_CURSOR, RS_CODED);
        c.bus.write32(RS_SRC, r.next_u32());
        c.bus.write32(RS_CODED, 4 + seed % 8);
        for i in 0..16 {
            c.bus.write32(RS_CODED + 4 + 4 * i, r.next_u32());
        }
        c.g[A0][0] = RS_SELF as u64;
        c.g[A1][0] = RS_CURSOR as u64;
        c.g[A2][0] = RS_SRC as u64;
    }

    #[test]
    fn replay_snapshot_empty_hook_b_matches() {
        check(
            ee::s07::f_0026ed90__generated,
            replay_snapshot_empty_hook_b,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x26_ed90, 0);
            },
        );
    }

    #[test]
    fn replay_set_controller_index_matches() {
        check(
            ee::s07::f_0026f898__generated,
            replay_set_controller_index,
            50,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x26_f898, 0);
                let this = DATA + 0x20_a300;
                c.bus.write32(this + 0x28, r.next_u32());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    // The save frame of a replay sits at `self+0x3d0`; a non-zero one is
    // restored and freed (real callees on a zeroed frame).

    // The replay's stop, reset hook and buffer purge are gated fakes (the
    // gate word in RAM); the frame destructors are a vtable whose slot
    // points at the miss fake. Flags are even (no self delete here).
    pub(crate) const DTOR_THIS: u32 = DATA + 0x20_b000;

    pub(crate) const DTOR_GATE: u32 = DATA + 0x20_b800;

    pub(crate) const DTOR_MAGIC: u32 = 0x4454_5230;

    pub(crate) const DTOR_VT: u32 = DATA + 0x20_bc00;

    pub(crate) const DTOR_OBJS: u32 = DATA + 0x20_c000;

    pub(crate) static OC532_DTOR_PREV_END: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) static OC532_DTOR_PREV_RESET: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) static OC532_DTOR_PREV_PURGE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn dtor_gated(
        c: &mut Cpu,
        prev: &std::sync::atomic::AtomicUsize,
        generated: GuestFn,
        target: u32,
    ) {
        if c.bus.read32(DTOR_GATE) != DTOR_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        fakes::fake_call(c, target);
    }

    pub(crate) fn dtor_fake_end(c: &mut Cpu) {
        dtor_gated(
            c,
            &OC532_DTOR_PREV_END,
            ee::s07::f_0026f980__generated,
            0x0026_f980,
        )
    }

    pub(crate) fn dtor_fake_reset(c: &mut Cpu) {
        dtor_gated(
            c,
            &OC532_DTOR_PREV_RESET,
            ee::s07::f_0026f498__generated,
            0x0026_f498,
        )
    }

    pub(crate) fn dtor_fake_purge(c: &mut Cpu) {
        dtor_gated(
            c,
            &OC532_DTOR_PREV_PURGE,
            ee::s07::f_0026f428__generated,
            0x0026_f428,
        )
    }

    pub(crate) fn dtor_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F980),
            dtor_fake_end,
            &OC532_DTOR_PREV_END,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F498),
            dtor_fake_reset,
            &OC532_DTOR_PREV_RESET,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F428),
            dtor_fake_purge,
            &OC532_DTOR_PREV_PURGE,
        );
    }

    pub(crate) fn replay_dtor_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x26_f088, 0);
        dtor_install();
        c.bus.write32(DTOR_GATE, DTOR_MAGIC);
        c.bus.write16(DTOR_VT + 0x38, (r.next_u32() as u16) & !3);
        c.bus.write32(DTOR_VT + 0x3c, fakes::FAKE);
        // State 0 (idle), 3 (running: stopped) or 12 (finished).
        c.bus.write32(DTOR_THIS, [0, 3, 12][(seed % 3) as usize]);
        for k in 0..14 {
            let obj = DTOR_OBJS + 0x40 * k;
            c.bus.write32(obj, DTOR_VT);
            c.bus.write32(DTOR_THIS + 0x2c + 0x38 * k, obj);
        }
        c.g[A0][0] = DTOR_THIS as u64;
        c.g[A1][0] = (r.next_u32() & !1) as u64;
    }

    #[test]
    fn replay_dtor_matches() {
        check(
            ee::s07::f_0026f088__generated,
            replay_dtor_2,
            60,
            &replay_dtor_setup,
        );
    }

    // Begin-playback's callees are log-only gated fakes (the gate word in
    // RAM): with the gate set, a call is logged and returns to its caller;
    // otherwise the body the slot held runs. `StepFrame` has no slot and runs
    // as generated.
    pub(crate) const BP_SELF: u32 = DATA + 0x20_d000;

    pub(crate) const BP_GATE: u32 = DATA + 0x20_d800;

    pub(crate) const BP_MAGIC: u32 = 0x4250_4c59;

    pub(crate) static OC532_BP_PREV_26F7F8: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn bp_fake_26f7f8(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_26F7F8,
            ee::s07::f_0026f7f8__generated,
            0x0026f7f8,
        )
    }

    pub(crate) static OC532_BP_PREV_26F4A0: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn bp_fake_26f4a0(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_26F4A0,
            ee::s07::f_0026f4a0__generated,
            0x0026f4a0,
        )
    }

    pub(crate) static OC532_BP_PREV_2702F8: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn bp_fake_2702f8(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_2702F8,
            ee::s07::f_002702f8__generated,
            0x002702f8,
        )
    }

    pub(crate) static OC532_BP_PREV_26EF30: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn bp_fake_26ef30(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_26EF30,
            ee::s07::f_0026ef30__generated,
            0x0026ef30,
        )
    }

    pub(crate) static OC532_BP_PREV_26EE68: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn bp_fake_26ee68(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_26EE68,
            ee::s07::f_0026ee68__generated,
            0x0026ee68,
        )
    }

    pub(crate) static OC532_BP_PREV_28B180: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn bp_fake_28b180(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_28B180,
            ee::s07::f_0028b180__generated,
            0x0028b180,
        )
    }

    pub(crate) static OC532_BP_PREV_28F200: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn bp_fake_28f200(c: &mut Cpu) {
        bp_gated(
            c,
            &OC532_BP_PREV_28F200,
            ee::s07::f_0028f200__generated,
            0x0028f200,
        )
    }

    pub(crate) fn bp_gated(
        c: &mut Cpu,
        prev: &std::sync::atomic::AtomicUsize,
        generated: GuestFn,
        target: u32,
    ) {
        if c.bus.read32(BP_GATE) != BP_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        fakes::fake_call(c, target);
    }

    pub(crate) fn bp_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F7F8),
            bp_fake_26f7f8,
            &OC532_BP_PREV_26F7F8,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F4A0),
            bp_fake_26f4a0,
            &OC532_BP_PREV_26F4A0,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_002702F8),
            bp_fake_2702f8,
            &OC532_BP_PREV_2702F8,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026EF30),
            bp_fake_26ef30,
            &OC532_BP_PREV_26EF30,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026EE68),
            bp_fake_26ee68,
            &OC532_BP_PREV_26EE68,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            bp_fake_28b180,
            &OC532_BP_PREV_28B180,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0028F200),
            bp_fake_28f200,
            &OC532_BP_PREV_28F200,
        );
    }

    pub(crate) fn replay_begin_playback_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x26_f8a0, 0);
        bp_install();
        c.bus.write32(BP_GATE, BP_MAGIC);
        // States 3, 7 (running) and 14, 15 (stopping: nothing starts).
        c.bus.write32(BP_SELF, [3, 14, 7, 15][(seed % 4) as usize]);
        c.bus
            .write32(BP_SELF + 0x608, if seed % 3 == 0 { 0 } else { 1 });
        c.g[A0][0] = BP_SELF as u64;
        c.g[A1][0] = (DATA + 0x20_d400 + (r.next_u32() & 0x3f0)) as u64;
        c.g[A2][0] = (seed % 2) as u64;
        c.g[A3][0] = (seed % 5 == 0) as u64;
    }

    // `Replay_UpdateCameraInput`: the input and view-manager callees are
    // gated fakes (the gate word in RAM, else the body the slot held). The
    // stick inputs answer from a script: `getValue(code)` gives the word at
    // `RUCI_VALUES + 4 * (code & 3)` as f0.
    pub(crate) const RUCI_SELF: u32 = DATA + 0x20_d900;

    pub(crate) const RUCI_APP: u32 = DATA + 0x20_da00;

    pub(crate) const RUCI_INPUT: u32 = DATA + 0x20_db00;

    pub(crate) const RUCI_VM: u32 = DATA + 0x20_dc00;

    pub(crate) const RUCI_VALUES: u32 = DATA + 0x20_dd00;

    pub(crate) const RUCI_GATE: u32 = DATA + 0x20_dd80;

    pub(crate) const RUCI_MAGIC: u32 = 0x5255_4349;

    pub(crate) const RUCI_SUBSET: u32 = 0x0044_5438;

    pub(crate) static OC532_RUCI_PREV_GETSTATE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) static OC532_RUCI_PREV_GETVALUE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) static OC532_RUCI_PREV_UPDATE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn ruci_fake_getstate(c: &mut Cpu) {
        if c.bus.read32(RUCI_GATE) != RUCI_MAGIC {
            return fakes::fall_back(c, &OC532_RUCI_PREV_GETSTATE, ee::s10::f_00320c48__generated);
        }
        fakes::fake_call(c, 0x0032_0c48);
    }

    pub(crate) fn ruci_fake_getvalue(c: &mut Cpu) {
        if c.bus.read32(RUCI_GATE) != RUCI_MAGIC {
            return fakes::fall_back(c, &OC532_RUCI_PREV_GETVALUE, ee::s10::f_00320bf0__generated);
        }
        let value = c.bus.read32(RUCI_VALUES + 4 * (c.g[A1][0] as u32 & 3));
        fakes::fake_call(c, 0x0032_0bf0);
        c.f[0] = value;
    }

    pub(crate) fn ruci_fake_update(c: &mut Cpu) {
        if c.bus.read32(RUCI_GATE) != RUCI_MAGIC {
            return fakes::fall_back(c, &OC532_RUCI_PREV_UPDATE, ee::s05::f_0022e840__generated);
        }
        fakes::fake_call(c, 0x0022_e840);
    }

    pub(crate) fn ruci_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00320C48),
            ruci_fake_getstate,
            &OC532_RUCI_PREV_GETSTATE,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00320BF0),
            ruci_fake_getvalue,
            &OC532_RUCI_PREV_GETVALUE,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s05::OVERRIDE_0022E840),
            ruci_fake_update,
            &OC532_RUCI_PREV_UPDATE,
        );
    }

    pub(crate) fn replay_update_camera_input_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x26_fa78, 0);
        ruci_install();
        c.bus.write32(RUCI_GATE, RUCI_MAGIC);
        // State 3 runs the body; 7 leaves at once.
        c.bus.write32(RUCI_SELF, [3, 7, 3, 3][(seed % 4) as usize]);
        let idx = seed % 2;
        c.bus.write32(RUCI_SELF + 0x28, idx);
        c.bus.write32(RUCI_SELF + 0x630, idx);
        // Camera subset 11 (or another) for that index.
        c.bus
            .write32(RUCI_SUBSET + 4 * idx, if seed % 3 == 0 { 5 } else { 11 });
        c.bus.write32(GAME_GP - 0x848, RUCI_APP);
        c.bus.write32(RUCI_APP + 0xb0 + 4 * idx, RUCI_INPUT);
        c.bus.write32(RUCI_APP + 0x84, RUCI_VM);
        c.bus.write32(RUCI_VM + 0x84, r.next_u32());
        for k in 0..4 {
            c.bus.write32(RUCI_VALUES + 4 * k, 0);
        }
        if seed % 2 == 0 {
            c.bus
                .write32(RUCI_VALUES + 4 * ((seed / 2) % 4), 0x3f80_0000);
        }
        c.g[A0][0] = RUCI_SELF as u64;
    }

    #[test]
    fn replay_process_command_matches() {
        check(
            ee::s07::f_0026fe50__generated,
            replay_process_command,
            200,
            &|c, seed| {
                machine(c, seed, 0xfe50, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0026fe50;
            },
        );
    }

    /// The entry return address the fixtures give every call.
    pub(crate) const RET: u64 = crate::difftest::RA as u64;

    #[test]
    fn replay_update_camera_input_matches() {
        const THIS: u32 = DATA + 0x30_4000;
        const WORLD: u32 = DATA + 0x30_5000;
        const CAM: u32 = DATA + 0x30_5800;
        const OBJ: u32 = DATA + 0x30_6000;
        check(
            ee::s07::f_0026fa78__generated,
            replay_update_camera_input_2,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x26_fa78, 0);
                c.pc = 0x0026_fa78;
                c.g[RA][0] = RET;
                c.bus.write32(THIS, if seed % 4 == 0 { 2 } else { 3 });
                c.bus.write32(THIS + 0x28, seed % 4);
                c.bus.write32(THIS + 0x630, seed % 2);
                c.bus.write32(GAME_GP_848, WORLD);
                c.bus.write32(WORLD + 0x84, CAM);
                c.bus.write32(CAM + 0x84, OBJ);
                c.bus.write32(WORLD + 4 * (seed % 4) + 0xb0, OBJ);
                c.bus.write32(
                    0x0044_5438 + 4 * (seed % 2),
                    if seed % 3 == 0 { 0xb } else { 0 },
                );
                c.bus.write32(OBJ + 0x84, r.next_u32());
                c.g[A0][0] = THIS as u64;
            },
        );
    }

    /// The gp-relative world pointer (`GAME_GP - 0x848`).
    pub(crate) const GAME_GP_848: u32 = crate::difftest::GAME_GP - 0x848;

    pub(crate) use crate::game::replay::playbackman::tests::oc683_in_child;
    pub(crate) use crate::game::replay::playbackman::tests::oc683_machine;

    pub(crate) use crate::game::replay::playbackman::tests::OC683_STREAM;

    pub(crate) use crate::game::replay::playbackman::tests::oc683_world;
    pub(crate) use crate::game::replay::playbackman::tests::OC683_STATE;

    // The stream of `ReplayStream_Decompress`: the block pointer at +4 and
    // the write pointer at +8, the `u8 **` cursor and the source tag.
    pub(crate) const OC683_RS_SELF: u32 = DATA + 0x20_8000;

    pub(crate) const OC683_RS_CURSOR: u32 = DATA + 0x20_8100;

    pub(crate) const OC683_RS_SRC: u32 = DATA + 0x20_8200;

    pub(crate) const OC683_RS_BLOCK: u32 = DATA + 0x20_9000;

    pub(crate) const OC683_RS_CODED: u32 = DATA + 0x20_9800;

    /// `ReplayStream_Decompress(self, &p, src)`: tag, size and Huffman body
    /// (the copies and the decoder are logged fakes), then the rewind.
    #[test]
    fn replay_stream_decompress_matches() {
        oc683_in_child("replay_stream_decompress_matches", &|| {
            check(
                ee::s07::f_0026eab8__generated,
                replay_stream_decompress,
                100,
                &|c, seed| {
                    let mut r = oc683_machine(c, seed, 0x0026_eab8, 4);
                    c.bus.write32(OC683_RS_SELF + 0x4, OC683_RS_BLOCK);
                    c.bus.write32(OC683_RS_SELF + 0x8, r.next_u32());
                    c.bus.write32(OC683_RS_CURSOR, OC683_RS_CODED);
                    c.bus.write32(OC683_RS_SRC, r.next_u32());
                    c.bus.write32(OC683_RS_CODED, 4 + seed % 8);
                    for i in 0..16 {
                        c.bus.write32(OC683_RS_CODED + 4 + 4 * i, r.next_u32());
                    }
                    c.g[A0][0] = sx(OC683_RS_SELF);
                    c.g[A1][0] = sx(OC683_RS_CURSOR);
                    c.g[A2][0] = sx(OC683_RS_SRC);
                    c.pc = 0x0026_eab8;
                },
            );
        });
    }

    /// `Replay_RestoreResumeSnapshot(self)`: with a resume snapshot at `+0x3d0`
    /// it is restored (the real `Replay_RestoreSnapshot` chain), freed and
    /// cleared; without one nothing is called.
    #[test]
    fn replay_restore_resume_snapshot_matches() {
        oc683_in_child("replay_restore_resume_snapshot_matches", &|| {
            check(
                ee::s07::f_0026f850__generated,
                replay_restore_resume_snapshot,
                60,
                &|c, seed| {
                    let _ = oc683_machine(c, seed, 0x0026_f850, 40);
                    oc683_world(c);
                    let snapshot = if seed % 3 == 0 { 0 } else { OC683_STREAM };
                    c.bus.write32(OC683_STATE + 0x3d0, snapshot);
                    c.g[A0][0] = sx(OC683_STATE);
                    c.pc = 0x0026_f850;
                },
            );
        });
    }

    pub(crate) use crate::game::sound::asyncsys::tests::oc738_install_fakes;
    pub(crate) use crate::game::sound::asyncsys::tests::oc738_machine;

    /// Gate for the replay callee fakes: they run the real callee (the slot's
    /// previous body) unless this word holds the magic value in the machine's
    /// RAM. Each test's fixture sets it on both sides.
    pub(crate) const OC729_RBP_CTRL: u32 = DATA + 0x2000;

    pub(crate) const OC729_RBP_MAGIC: u32 = 0x729b_0a1d;

    #[inline]
    pub(crate) fn oc729_rbp_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC729_RBP_CTRL) == OC729_RBP_MAGIC
    }

    pub(crate) fn oc729_fake_save_resume(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc729_fake_save_resume),
                ee::s07::f_0026f7f8,
            );
        }
        fakes::fake_call(c, 0x0026_f7f8)
    }

    pub(crate) fn oc729_fake_set_state(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc729_fake_set_state),
                ee::s07::f_0026f4a0,
            );
        }
        fakes::fake_call(c, 0x0026_f4a0)
    }

    pub(crate) fn oc729_fake_seek(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(c, fakes::prev_cell(oc729_fake_seek), ee::s07::f_002702f8);
        }
        fakes::fake_call(c, 0x0027_02f8)
    }

    pub(crate) fn oc729_fake_step(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(c, fakes::prev_cell(oc729_fake_step), ee::s07::f_002703f0);
        }
        fakes::fake_call(c, 0x0027_03f0)
    }

    pub(crate) fn oc729_fake_enable_instant(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc729_fake_enable_instant),
                ee::s07::f_0026ef30,
            );
        }
        fakes::fake_call(c, 0x0026_ef30)
    }

    pub(crate) fn oc729_fake_enable_cameras(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc729_fake_enable_cameras),
                ee::s07::f_0026ee68,
            );
        }
        fakes::fake_call(c, 0x0026_ee68)
    }

    pub(crate) fn oc729_fake_audio_instance(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc729_fake_audio_instance),
                ee::s07::f_0028b180,
            );
        }
        fakes::fake_call(c, 0x0028_b180)
    }

    pub(crate) fn oc729_fake_audio_replay_begin(c: &mut Cpu) {
        if !oc729_rbp_gated(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc729_fake_audio_replay_begin),
                ee::s07::f_0028f200,
            );
        }
        fakes::fake_call(c, 0x0028_f200)
    }

    /// Hooks the eight callee fakes in their override slots (once per process).
    pub(crate) fn oc729_install_fakes() {
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 8] = [
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F7F8),
                oc729_fake_save_resume,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F4A0),
                oc729_fake_set_state,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_002702F8),
                oc729_fake_seek,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_002703F0),
                oc729_fake_step,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026EF30),
                oc729_fake_enable_instant,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026EE68),
                oc729_fake_enable_cameras,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0028B180),
                oc729_fake_audio_instance,
            ),
            (
                std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0028F200),
                oc729_fake_audio_replay_begin,
            ),
        ];
        for (slot, fake) in slots {
            fakes::hook(slot, fake, fakes::prev_cell(fake));
        }
    }

    #[test]
    fn replay_begin_playback_matches() {
        use crate::difftest::fakes::machine;
        oc729_install_fakes();
        check(
            ee::s07::f_0026f8a0__generated,
            replay_begin_playback_3,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0026_f8a0, 0);
                c.bus.write32(OC729_RBP_CTRL, OC729_RBP_MAGIC);
                c.pc = 0x0026_f8a0;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(r.next_u32());
                // instantReplay and autoReplay: all four combinations.
                c.g[A2][0] = (seed & 1) as u64;
                c.g[A3][0] = ((seed >> 1) & 1) as u64;
                // state 14 and 15 are transitions (skipped); 0 and 3 run.
                let state = [14u32, 15, 0, 3][(seed % 4) as usize];
                c.bus.write32(DATA, state);
                // The resume snapshot is held or not.
                let snapshot = if (seed / 4) % 2 == 0 {
                    0
                } else {
                    r.next_u32() | 1
                };
                c.bus.write32(DATA + 0x608, snapshot);
            },
        );
    }

    /// `Replay_BeginPlayback`: already playing (14, 15) or not, a loaded
    /// snapshot or not, instant or full replay, auto replay or not.
    #[test]
    fn replay_begin_playback_matches_2() {
        oc738_install_fakes();
        fakes::covered(
            ee::s07::f_0026f8a0,
            replay_begin_playback_4,
            400,
            &|c, seed| {
                let mut r = oc738_machine(c, seed, 0x26_f8a0, 8);
                let this = DATA + 0x1000;
                let state = [14u32, 15, 0, 7, 13, 16, 3][(seed % 7) as usize];
                c.bus.write32(this, state);
                c.bus.write32(this + 0x608, (seed / 7) % 2);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = ((seed / 14) % 2) as u64 * (1 + r.next_u32() as u64 % 5);
                c.g[A3][0] = ((seed / 28) % 2) as u64;
                c.pc = 0x0026_f8a0;
            },
        );
    }
}
