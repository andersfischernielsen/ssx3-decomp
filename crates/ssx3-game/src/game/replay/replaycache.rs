//! `src/replay/replaycache.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::dirtysock::tagsunk::at_3 as at;
pub(crate) use crate::game::dirtysock::tagsunk::deleting_dtor;
pub(crate) use crate::game::dirtysock::tagsunk::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tagsunk::p_2 as p;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::feasyncfile::addu;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p as p_7;
pub(crate) use crate::game::fe::feasyncfile::p as p_9;
pub(crate) use crate::game::fe::feasyncfile::p as p_11;
pub(crate) use crate::game::fe::feasyncfile::p as p_12;
pub(crate) use crate::game::fe::feasyncfile::p as p_16;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_4;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_14;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sub;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw_4;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_5;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_5;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_8;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_10;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p_3;
pub(crate) use crate::game::sound::bankmonitor::p as p_2;
pub(crate) use crate::game::sound::soundsys::p as p_15;
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
        0x0026_d628,
        world_state_stream_reset_2,
        "WorldStateStream_Reset",
    ),
    (
        0x0027_05a8,
        replay_empty_rider_hook,
        "Replay_EmptyRiderHook",
    ),
    (0x0027_22e0, in_seq_heap_init, "InSeq_HeapInit"),
    (
        0x0027_42c8,
        in_seq_channel_get_track,
        "InSeq_ChannelGetTrack",
    ),
    (
        0x0027_4348,
        in_seq_channel_get_length,
        "InSeq_ChannelGetLength",
    ),
    (0x0027_43c8, in_seq_channel_get_id, "InSeq_ChannelGetId"),
    (
        0x0027_0070,
        player_alloc_replay_cache,
        "Player_AllocReplayCache",
    ),
    (
        0x0027_00d8,
        player_free_replay_cache,
        "Player_FreeReplayCache",
    ),
    (0x0027_0130, replay_alloc_snapshot, "Replay_AllocSnapshot"),
    (0x0027_0280, replay_is_paused, "Replay_IsPaused"),
    (0x0027_0378, replay_seek_to_end, "Replay_SeekToEnd"),
    (0x0027_0538, replay_prune_snapshots, "Replay_PruneSnapshots"),
    (
        0x0027_05b0,
        replay_check_instant_replay,
        "Replay_CheckInstantReplay",
    ),
    (
        0x0027_0658,
        replay_request_instant_replay,
        "Replay_RequestInstantReplay",
    ),
    (
        0x0027_0670,
        replay_end_instant_replay,
        "Replay_EndInstantReplay",
    ),
    (0x0027_06b8, replay_open_menu, "Replay_OpenMenu"),
    (
        0x0027_0730,
        replay_find_rider_index,
        "Replay_FindRiderIndex",
    ),
    (0x0027_07e0, replay_on_rider_launch, "Replay_OnRiderLaunch"),
    (0x0027_0870, replay_on_rider_land, "Replay_OnRiderLand"),
    (
        0x0027_0ab0,
        replay_on_rider_checkpoint,
        "Replay_OnRiderCheckpoint",
    ),
    (
        0x0027_0bd8,
        replay_count_highlights,
        "Replay_CountHighlights",
    ),
    (
        0x0027_0c40,
        replay_get_max_highlights,
        "Replay_GetMaxHighlights",
    ),
    (0x0027_0ce8, replay_end_highlight, "Replay_EndHighlight"),
    (
        0x0027_0de0,
        replay_cancel_highlight,
        "Replay_CancelHighlight",
    ),
    (
        0x0027_0e18,
        replay_clear_all_highlights,
        "Replay_ClearAllHighlights",
    ),
    (0x0027_0e68, replay_begin_save, "Replay_BeginSave"),
    (
        0x0027_0f20,
        replay_alloc_save_buffers,
        "Replay_AllocSaveBuffers",
    ),
    (
        0x0027_0f78,
        replay_free_save_buffers,
        "Replay_FreeSaveBuffers",
    ),
    (
        0x0027_0fc0,
        replay_flush_save_output,
        "Replay_FlushSaveOutput",
    ),
    (0x0027_1058, replay_save_emit, "Replay_SaveEmit"),
    (
        0x0027_1228,
        replay_save_build_header,
        "Replay_SaveBuildHeader",
    ),
    (
        0x0027_12c0,
        replay_save_write_header,
        "Replay_SaveWriteHeader",
    ),
    (
        0x0027_12f8,
        replay_count_save_snapshots,
        "Replay_CountSaveSnapshots",
    ),
    (
        0x0027_1348,
        replay_save_begin_snapshots,
        "Replay_SaveBeginSnapshots",
    ),
    (
        0x0027_14e0,
        replay_save_begin_caches,
        "Replay_SaveBeginCaches",
    ),
    (
        0x0027_1620,
        replay_save_compute_padding,
        "Replay_SaveComputePadding",
    ),
    (
        0x0027_1688,
        replay_save_write_padding,
        "Replay_SaveWritePadding",
    ),
    (
        0x0027_1708,
        replay_save_write_footer,
        "Replay_SaveWriteFooter",
    ),
    (0x0027_21d0, replay_stream_dtor, "ReplayStream_dtor"),
    (
        0x0027_2258,
        world_state_stream_dtor,
        "WorldStateStream_dtor",
    ),
    (
        0x0027_2288,
        replay_highlight_slot_init,
        "Replay_HighlightSlotInit",
    ),
    (0x0027_23a8, in_seq_heap_free, "InSeq_HeapFree"),
    (0x0027_2570, math_eval_cubic, "Math_EvalCubic"),
    (
        0x0027_25b8,
        in_seq_curve_track_ctor,
        "InSeq_CurveTrack_ctor",
    ),
    (
        0x0027_25f0,
        in_seq_curve_track_start,
        "InSeq_CurveTrackStart",
    ),
    (
        0x0027_26a8,
        in_seq_curve_track_update,
        "InSeq_CurveTrackUpdate",
    ),
    (0x0027_28d0, in_seq_dtor, "InSeq_dtor"),
    (0x0027_28f8, in_seq_update_all, "InSeq_UpdateAll"),
    (0x0027_2938, in_seq_start_sequence, "InSeq_StartSequence"),
    (0x0027_29a8, in_seq_free_sequence, "InSeq_FreeSequence"),
    (0x0027_2ad0, in_seq_play, "InSeq_Play"),
    (0x0027_2b58, in_seq_stop, "InSeq_Stop"),
    (0x0027_2d68, in_seq_span_track_ctor, "InSeq_SpanTrack_ctor"),
    (0x0027_2da0, inseq_span_track_dtor, "InSeq_SpanTrack_dtor"),
    (0x0027_2df0, inseq_span_track_start, "InSeq_SpanTrackStart"),
    (0x0027_2ec0, in_seq_span_track_stop, "InSeq_SpanTrackStop"),
    (
        0x0027_3040,
        in_seq_span_track_get_next_event_time,
        "InSeq_SpanTrackGetNextEventTime",
    ),
    (
        0x0027_30c0,
        in_seq_span_track_activate_key,
        "InSeq_SpanTrackActivateKey",
    ),
    (
        0x0027_3140,
        in_seq_span_track_next_key,
        "InSeq_SpanTrackNextKey",
    ),
    (
        0x0027_3180,
        in_seq_event_track_ctor,
        "InSeq_EventTrack_ctor",
    ),
    (0x0027_31b8, inseq_event_track_dtor, "InSeq_EventTrack_dtor"),
    (
        0x0027_3208,
        in_seq_event_track_start,
        "InSeq_EventTrackStart",
    ),
    (0x0027_32c8, in_seq_event_track_stop, "InSeq_EventTrackStop"),
    (
        0x0027_33e0,
        in_seq_event_track_get_next_event_time,
        "InSeq_EventTrackGetNextEventTime",
    ),
    (
        0x0027_3418,
        in_seq_event_track_fire_key,
        "InSeq_EventTrackFireKey",
    ),
    (
        0x0027_3468,
        in_seq_event_track_next_key,
        "InSeq_EventTrackNextKey",
    ),
    (0x0027_34a8, in_seq_ramp_track_ctor, "InSeq_RampTrack_ctor"),
    (0x0027_34e0, inseq_ramp_track_dtor, "InSeq_RampTrack_dtor"),
    (0x0027_3630, in_seq_ramp_track_stop, "InSeq_RampTrackStop"),
    (
        0x0027_37e0,
        in_seq_ramp_track_get_next_event_time,
        "InSeq_RampTrackGetNextEventTime",
    ),
    (
        0x0027_3860,
        in_seq_ramp_track_activate_key,
        "InSeq_RampTrackActivateKey",
    ),
    (
        0x0027_38f8,
        in_seq_ramp_track_next_key,
        "InSeq_RampTrackNextKey",
    ),
    (0x0027_3938, in_seq_channel_ctor, "InSeq_Channel_ctor"),
    (0x0027_3970, in_seq_channel_dtor, "InSeq_Channel_dtor"),
    (0x0027_39b8, in_seq_channel_setup, "InSeq_ChannelSetup"),
    (0x0027_3a00, in_seq_channel_release, "InSeq_ChannelRelease"),
    (
        0x0027_3a60,
        in_seq_channel_select_block,
        "InSeq_ChannelSelectBlock",
    ),
    (
        0x0027_3d20,
        in_seq_channel_flush_events,
        "InSeq_ChannelFlushEvents",
    ),
    (
        0x0027_4240,
        in_seq_channel_set_listener,
        "InSeq_ChannelSetListener",
    ),
    (0x0027_4518, in_seq_instance_ctor, "InSeq_Instance_ctor"),
    (0x0027_4830, in_seq_instance_start, "InSeq_InstanceStart"),
    (0x0027_49e8, in_seq_instance_pause, "InSeq_InstancePause"),
    (0x0027_4a08, in_seq_instance_resume, "InSeq_InstanceResume"),
    (0x0027_4a30, in_seq_instance_update, "InSeq_InstanceUpdate"),
    (
        0x0027_4c20,
        in_seq_instance_get_channel,
        "InSeq_InstanceGetChannel",
    ),
    (0x0027_4c38, in_seq_get_header_extra, "InSeq_GetHeaderExtra"),
    (
        0x0027_4c70,
        in_seq_instance_get_length,
        "InSeq_InstanceGetLength",
    ),
    (
        0x0027_4d08,
        in_seq_instance_get_remaining_steps,
        "InSeq_InstanceGetRemainingSteps",
    ),
    (
        0x0027_4d70,
        in_seq_instance_get_listener,
        "InSeq_InstanceGetListener",
    ),
];

/// `void WorldStateStream_Reset(WorldStateStream *stream)` (0x26d628):
/// reinitialise the stream fields (a scratch 32-bit zero on the stack is used
/// to store the first list word unaligned).
pub fn world_state_stream_reset(c: &mut Cpu) {
    c.cycles += 20;
    let stream = Addr(abi::arg(c, 0));
    let sp = Addr((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[SP][0] = sx(sp.0);
    c.g[V0][0] = u64::MAX; // -1
    sp.sh(c, 0);
    sp.off(2).sb(c, 0);
    sp.off(3).sb(c, 0);
    stream.off(0x30).sw(c, 0);
    stream.off(0x34).sw(c, c.g[V0][0]);
    c.lwl(V1, sp.off(3).0);
    c.lwr(V1, sp.0);
    c.swl(V1, stream.off(0x1f).0);
    c.swr(V1, stream.off(0x1c).0);
    stream.off(0x2c).sw(c, c.g[V0][0]);
    stream.off(0x10).sw(c, 0);
    stream.off(0x20).sw(c, c.g[V0][0]);
    stream.off(0x24).sw(c, c.g[V0][0]);
    stream.off(0x28).sw(c, c.g[V0][0]);
    stream.off(0x8).sw(c, 0);
    stream.off(0xc).sw(c, 0);
    c.g[SP][0] = sx(sp.0.wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Instrument-sequence heap.

/// `void* InSeq_HeapInit(Heap* heap, void* data, uint size)` (0x2722e0):
/// initialise a heap node (head at +0x0, tail +0x8, size +0x4), terminating the
/// block with a free marker at `data + 4` and the end offset.
pub fn in_seq_heap_init(c: &mut Cpu) {
    const TAIL: u32 = 0x8;
    const SIZE: u32 = 0x4;
    const BLOCK_NEXT: u32 = 0x4;
    c.cycles += 9;
    let heap = Addr(abi::arg(c, 0));
    let data = c.g[A1][0];
    heap.off(TAIL).sw(c, data);
    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x8));
    heap.sw(c, data);
    c.g[V0][0] = c.g[A0][0];
    heap.off(SIZE).sw(c, c.g[A2][0]);
    Addr(data as u32).off(BLOCK_NEXT).sw(c, 0);
    c.g[V1][0] = heap.off(TAIL).lw(c);
    Addr(c.g[V1][0] as u32).sw(c, c.g[A3][0]);
    abi::ret(c);
}

/// `void* InSeq_ChannelGetTrack(InSeq_Channel* ch, int index)` (0x2742c8): the
/// track pointer `index` in the table at `ch + 0x24`.
pub fn in_seq_channel_get_track(c: &mut Cpu) {
    const TABLE: u32 = 0x24;
    c.cycles += 5;
    let table = Addr(abi::arg(c, 0)).off(TABLE).lw(c);
    c.g[V0][0] = table;
    let offset = sx((c.g[A1][0] as u32).wrapping_shl(2));
    c.g[A1][0] = offset;
    let slot = sx((offset as u32).wrapping_add(table as u32));
    c.g[A1][0] = slot;
    c.g[V0][0] = Addr(slot as u32).lw(c);
    abi::ret(c);
}

/// `int InSeq_ChannelGetLength(InSeq_Channel* ch)` (0x274348): the length field
/// at `channel->track + 0x4`.
pub fn in_seq_channel_get_length(c: &mut Cpu) {
    const TRACK: u32 = 0x1c;
    c.cycles += 3;
    let track = Addr(abi::arg(c, 0)).off(TRACK).lw(c);
    c.g[V1][0] = track;
    c.g[V0][0] = Addr(track as u32).off(0x4).lw(c);
    abi::ret(c);
}

/// `int InSeq_ChannelGetId(InSeq_Channel* ch)` (0x2743c8): the id byte at
/// `channel->track + 0x3`.
pub fn in_seq_channel_get_id(c: &mut Cpu) {
    const TRACK: u32 = 0x1c;
    c.cycles += 3;
    let track = Addr(abi::arg(c, 0)).off(TRACK).lw(c);
    c.g[V1][0] = track;
    c.g[V0][0] = Addr(track as u32).off(0x3).lbu(c);
    abi::ret(c);
}

/// `WorldStateStream* WorldStateStream_Reset(WorldStateStream *self)`
/// (0x26d628): reset the stream's counters and the 12-byte field at +0x1c.
pub fn world_state_stream_reset_2(c: &mut Cpu) {
    c.cycles += 20;
    let stream = Addr(abi::arg(c, 0));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = u64::MAX;
    let sp = c.g[SP][0] as u32;
    Addr(sp).sh(c, 0);
    Addr(sp.wrapping_add(0x2)).sb(c, 0);
    Addr(sp.wrapping_add(0x3)).sb(c, 0);
    stream.off(0x30).sw(c, 0);
    stream.off(0x34).sw(c, u64::MAX);
    c.lwl(V1, sp.wrapping_add(0x3));
    c.lwr(V1, sp);
    c.swl(V1, stream.0.wrapping_add(0x1f));
    c.swr(V1, stream.0.wrapping_add(0x1c));
    stream.off(0x2c).sw(c, u64::MAX);
    stream.off(0x10).sw(c, 0);
    stream.off(0x20).sw(c, u64::MAX);
    stream.off(0x24).sw(c, u64::MAX);
    stream.off(0x28).sw(c, u64::MAX);
    stream.off(0x8).sw(c, 0);
    stream.off(0xc).sw(c, 0);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

/// `void Replay_EmptyRiderHook(Rider* r)` (0x2705a8): replay rider hook, not
/// installed.
pub fn replay_empty_rider_hook(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::be::beintmission::callee::EA_MEMSET;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_GET_INSTANCE;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_INSTANCE_STOP;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::APP_SOMETHING;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::EA_MEM_FREE_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::INSEQ_GET_INSTANCE as INSEQ_GET_INSTANCE_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::INSEQ_INSTANCE_START;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_HEAP_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_ACTIVATE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::fepopup::callee::C0026D4D8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026DB88;
    pub(crate) use crate::game::fe::fepopup::callee::C0026E448;
    pub(crate) use crate::game::fe::fepopup::callee::C0026F4A0;
    pub(crate) use crate::game::fe::fepopup::callee::C002702F8;
    pub(crate) use crate::game::fe::fepopup::callee::C002703F0;
    pub(crate) use crate::game::fe::festaterewards::callee::IN_SEQ_INSTANCE_SETUP;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT as REPLAY_FIND_SNAPSHOT_2;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT as REPLAY_SAVE_EMIT_2;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;

    pub(crate) const GET_LENGTH: Callee = Callee {
        addr: 0x0027_4c70,
        f: ee::s07::f_00274c70,
    };

    pub(crate) const TICK: Callee = Callee {
        addr: 0x0027_4de0,
        f: ee::s07::f_00274de0,
    };

    pub(crate) const STOP: Callee = Callee {
        addr: 0x0027_4918,
        f: ee::s07::f_00274918,
    };

    pub(crate) const RESTART: Callee = Callee {
        addr: 0x0027_4810,
        f: ee::s07::f_00274810,
    };

    pub(crate) const PAUSE: Callee = Callee {
        addr: 0x0027_49e8,
        f: ee::s07::f_002749e8,
    };
}

pub(crate) mod in_seq_curve {

    /// The current curve segment (`{u16 start, u16 end, f32 p[4]}`).
    pub const CURVE: u32 = 0x24;

    /// The float value sampled last time.
    pub const VALUE: u32 = 0x1c;

    /// The segment being played.
    pub const SEGMENT: u32 = 0x14;

    /// The key index inside that segment.
    pub const KEY: u32 = 0x18;

    /// The current value's address inside the curve.
    pub const CURSOR: u32 = 0x20;

    /// The curve track's length in keys.
    pub const LENGTH: u32 = 0x10;
}

pub(crate) mod table_callee {
    pub(crate) use crate::game::be::bedbreward::table_callee::OPERATOR_DELETE;
}

pub(crate) mod table {
    pub(crate) use crate::game::ai::computer::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::ai::computer::table::OPERATOR_NEW_NAMED;
}

/// `void Replay_RequestInstantReplay(Replay *replay)` (0x270658): request an
/// instant replay only when one is not already active.
pub fn replay_request_instant_replay(c: &mut Cpu) {
    const ACTIVE: u32 = 0x620;
    const REQUESTED: u32 = 0x624;

    c.cycles += 3;
    let replay = Addr(abi::arg(c, 0));
    let active = replay.off(ACTIVE).lw(c);
    c.g[V0][0] = 1;
    if active == 0 {
        c.cycles += 1;
        replay.off(REQUESTED).sw(c, 1);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Replay recording.

/// `void Replay_SaveBeginSnapshots(Replay* r)` (0x271348): select the current
/// snapshot (the two pointers at +0x3b0/+0x3d0; when they are equal advance
/// through the node's +0x14 link into +0x65c), bump the record serial at
/// +0x638 and clear the per-record flags.
pub fn replay_save_begin_snapshots(c: &mut Cpu) {
    const FIRST: u32 = 0x3b0;
    const LAST: u32 = 0x3d0;
    const NODE_NEXT: u32 = 0x14;
    const CURRENT: u32 = 0x65c;
    const SERIAL: u32 = 0x638;
    const FLAG_660: u32 = 0x660;
    const FLAG_644: u32 = 0x644;
    const FLAG_648: u32 = 0x648;
    const FLAG_668: u32 = 0x668;
    c.cycles += 4;
    let r = Addr(abi::arg(c, 0));
    let first = r.off(FIRST).lw(c);
    c.g[V1][0] = first;
    let last = r.off(LAST).lw(c);
    c.g[V0][0] = last;
    let differs = first != last;
    r.off(CURRENT).sw(c, first);
    if !differs {
        c.cycles += 2;
        let next = Addr(first as u32).off(NODE_NEXT).lw(c);
        c.g[V0][0] = next;
        r.off(CURRENT).sw(c, next);
    }
    c.cycles += 8;
    let serial = r.off(SERIAL).lw(c);
    c.g[V0][0] = serial;
    r.off(FLAG_668).sw(c, 0);
    c.g[V0][0] = sx((serial as u32).wrapping_add(1));
    r.off(FLAG_660).sw(c, 0);
    r.off(SERIAL).sw(c, c.g[V0][0]);
    r.off(FLAG_644).sw(c, 0);
    r.off(FLAG_648).sw(c, 0);
    abi::ret(c);
}

/// `void Replay_SaveBeginCaches(Replay* r)` (0x2714e0): mark the cache phase
/// (state +0x658 = 2), bump the record serial at +0x638 and clear the cache
/// bookkeeping fields.
pub fn replay_save_begin_caches(c: &mut Cpu) {
    const SERIAL: u32 = 0x638;
    const STATE: u32 = 0x658;
    const CACHE_A: u32 = 0x654;
    const CACHE_B: u32 = 0x674;
    const CACHE_C: u32 = 0x66c;
    const CACHE_D: u32 = 0x670;
    const FLAG_644: u32 = 0x644;
    const FLAG_648: u32 = 0x648;
    c.cycles += 12;
    let r = Addr(abi::arg(c, 0));
    let serial = r.off(SERIAL).lw(c);
    c.g[V0][0] = serial;
    c.g[V1][0] = 0x2;
    r.off(STATE).sw(c, c.g[V1][0]);
    c.g[V0][0] = sx((serial as u32).wrapping_add(1));
    r.off(CACHE_A).sw(c, 0);
    r.off(SERIAL).sw(c, c.g[V0][0]);
    r.off(CACHE_B).sw(c, 0);
    r.off(CACHE_C).sw(c, 0);
    r.off(CACHE_D).sw(c, 0);
    r.off(FLAG_644).sw(c, 0);
    r.off(FLAG_648).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Cubic evaluation.

/// `float Math_EvalCubic(const float* coeffs, float t)` (0x272570): evaluate
/// `((c0*t + c1)*t + c2)*t + c3` (f12 = t), returning `c3` immediately when
/// `t` compares equal to zero.
pub fn math_eval_cubic(c: &mut Cpu) {
    const C1: u32 = 0x4;
    const C2: u32 = 0x8;
    const C3: u32 = 0xc;
    c.cycles += 4;
    let t = FpuF32(c.f[12]);
    // mtc1 f0, r0 / c.eq.s f12, f0.
    c.f[0] = 0;
    let eq = t.eq_s(FpuF32::ZERO);
    abi::set_fcc(c, eq);
    let coeffs = Addr(abi::arg(c, 0));
    if eq {
        c.cycles += 2;
        let c3 = coeffs.off(C3).read_fpu(c);
        c.f[0] = c3.0;
        abi::ret(c);
        return;
    }
    c.cycles += 11;
    let c0 = coeffs.read_fpu(c);
    let c1 = coeffs.off(C1).read_fpu(c);
    let mut f1 = c0 * t;
    let c2 = coeffs.off(C2).read_fpu(c);
    let c3 = coeffs.off(C3).read_fpu(c);
    let mut f0 = f1 + c1;
    f0 = f0 * t;
    f0 = f0 + c2;
    f1 = f0 * t;
    f0 = f1 + c3;
    c.f[0] = f0.0;
    c.f[1] = f1.0;
    c.f[2] = c1.0;
    c.f[3] = c2.0;
    c.f[4] = c3.0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Span tracks.

/// `void InSeq_SpanTrackActivateKey(SpanTrack* track, SpanKey* key)`
/// (0x2730c0): copy a span key into the track's `index * 0x1c` record at
/// `track + 0x20` and link it into the active list at `track + 0x24`.
pub fn in_seq_span_track_activate_key(c: &mut Cpu) {
    const ACTIVE_LIST: u32 = 0x20;
    const LINK: u32 = 0x24;
    const COUNT: u32 = 0x28;
    const RECORD: u32 = 0x1c;
    const KEY_INDEX: u32 = 0x2;
    const KEY_START: u32 = 0x8;
    const KEY_END: u32 = 0xc;
    const KEY_FLAGS: u32 = 0x4;
    const KEY_KIND: u32 = 0x6;
    c.cycles += 22;
    let key = c.g[A1][0];
    c.g[A2][0] = key;
    let track = c.g[A0][0];
    c.g[A3][0] = track;
    let index = Addr(key as u32).off(KEY_INDEX).lhu(c);
    c.g[V0][0] = index;
    c.g[A1][0] = RECORD as u64;
    let base = Addr(track as u32).off(ACTIVE_LIST).lw(c);
    c.g[A0][0] = base;
    let scaled = abi::mult(c, index as u32, RECORD);
    c.g[V0][0] = sx(scaled);
    let start = Addr(key as u32).off(KEY_START).lw(c);
    c.g[V1][0] = start;
    let slot = sx((base as u32).wrapping_add(scaled));
    c.g[A0][0] = slot;
    Addr(slot as u32).sw(c, start);
    let index2 = Addr(key as u32).off(KEY_INDEX).lhu(c);
    c.g[V0][0] = index2;
    Addr(slot as u32).off(0x4).sw(c, index2);
    let end = Addr(key as u32).off(KEY_END).lw(c);
    c.g[V1][0] = end;
    Addr(slot as u32).off(0x10).sw(c, end);
    let flags = Addr(key as u32).off(KEY_FLAGS).lhu(c);
    c.g[V0][0] = flags;
    Addr(slot as u32).off(0x8).sw(c, flags);
    let kind = Addr(key as u32).off(KEY_KIND).lhu(c);
    c.g[V1][0] = kind;
    Addr(slot as u32).off(0x14).sw(c, 0);
    Addr(slot as u32).off(0xc).sw(c, kind);
    let kind_field = Addr(key as u32).lhu(c);
    c.g[V0][0] = kind_field;
    let short = (kind_field < 0x11) as u64;
    c.g[V0][0] = short;
    if short != 0 {
        c.g[V1][0] = Addr(track as u32).off(LINK).lw(c);
    } else {
        c.cycles += 3;
        let payload = sx((key as u32).wrapping_add(0x10));
        c.g[V0][0] = payload;
        Addr(slot as u32).off(0x14).sw(c, payload);
        c.g[V1][0] = Addr(track as u32).off(LINK).lw(c);
    }
    c.cycles += 7;
    c.g[V0][0] = c.g[A0][0];
    Addr(slot as u32).off(0x18).sw(c, c.g[V1][0]);
    Addr(track as u32).off(LINK).sw(c, c.g[A0][0]);
    let count = Addr(track as u32).off(COUNT).lw(c);
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    Addr(track as u32).off(COUNT).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void* InSeq_SpanTrackNextKey(SpanTrack* track)` (0x273140): advance the
/// cursor at `track + 0x18` to the next span (adding the current span's length)
/// or clear the cursor at +0x14 at the end of the list.
pub fn in_seq_span_track_next_key(c: &mut Cpu) {
    const KEYS: u32 = 0x10;
    const CURSOR: u32 = 0x14;
    const INDEX: u32 = 0x18;
    c.cycles += 7;
    let track = c.g[A0][0];
    c.g[A1][0] = track;
    let keys = Addr(track as u32).off(KEYS).lw(c);
    c.g[V1][0] = keys;
    let cur = Addr(track as u32).off(INDEX).lw(c);
    c.g[A0][0] = cur;
    let count = Addr(keys as u32).lhu(c);
    c.g[V0][0] = count;
    let last = sx((count as u32).wrapping_sub(1));
    c.g[V0][0] = last;
    if cur != last {
        let next = Addr(track as u32).off(CURSOR).lw(c);
        c.g[V0][0] = next;
        c.cycles += 6;
        c.g[V1][0] = sx((cur as u32).wrapping_add(1));
        let len = Addr(next as u32).lhu(c);
        c.g[A0][0] = len;
        Addr(track as u32).off(INDEX).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((next as u32).wrapping_add(len as u32));
        Addr(track as u32).off(CURSOR).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 2;
        Addr(track as u32).off(CURSOR).sw(c, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Event tracks.

/// `void InSeq_EventTrackFireKey(EventTrack* track, EventKey* key)` (0x273418):
/// store a key into the event array element `track + 0x1c + index*8` and bump
/// the element count at +0x20.
pub fn in_seq_event_track_fire_key(c: &mut Cpu) {
    const ELEMS: u32 = 0x1c;
    const COUNT: u32 = 0x20;
    const KEY_VALUE: u32 = 0x4;
    c.cycles += 12;
    let track = c.g[A0][0];
    c.g[A2][0] = track;
    let key = c.g[A1][0];
    let value = Addr(key as u32).off(KEY_VALUE).lw(c);
    c.g[V1][0] = value;
    let index = Addr(track as u32).off(COUNT).lw(c);
    c.g[V0][0] = index;
    let elems = Addr(track as u32).off(ELEMS).lw(c);
    c.g[A0][0] = elems;
    let delta = sx((index as u32).wrapping_shl(3));
    c.g[V0][0] = delta;
    let slot = sx((elems as u32).wrapping_add(delta as u32));
    c.g[A0][0] = slot;
    Addr(slot as u32).sw(c, value);
    Addr(slot as u32).off(0x4).sw(c, 0);
    let kind = Addr(key as u32).lhu(c);
    c.g[V0][0] = kind;
    let short = (kind < 9) as u64;
    c.g[V0][0] = short;
    let count;
    if short != 0 {
        count = Addr(track as u32).off(COUNT).lw(c);
        c.g[V1][0] = count;
    } else {
        c.cycles += 3;
        let payload = sx((key as u32).wrapping_add(0x8));
        c.g[V0][0] = payload;
        Addr(slot as u32).off(0x4).sw(c, payload);
        count = Addr(track as u32).off(COUNT).lw(c);
        c.g[V1][0] = count;
    }
    c.cycles += 4;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    Addr(track as u32).off(COUNT).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void* InSeq_EventTrackNextKey(EventTrack* track)` (0x273468): same cursor
/// walk as the span track, over the event list.
pub fn in_seq_event_track_next_key(c: &mut Cpu) {
    const KEYS: u32 = 0x10;
    const CURSOR: u32 = 0x14;
    const INDEX: u32 = 0x18;
    c.cycles += 7;
    let track = c.g[A0][0];
    c.g[A1][0] = track;
    let keys = Addr(track as u32).off(KEYS).lw(c);
    c.g[V1][0] = keys;
    let cur = Addr(track as u32).off(INDEX).lw(c);
    c.g[A0][0] = cur;
    let count = Addr(keys as u32).lhu(c);
    c.g[V0][0] = count;
    let last = sx((count as u32).wrapping_sub(1));
    c.g[V0][0] = last;
    if cur != last {
        let next = Addr(track as u32).off(CURSOR).lw(c);
        c.g[V0][0] = next;
        c.cycles += 6;
        c.g[V1][0] = sx((cur as u32).wrapping_add(1));
        let len = Addr(next as u32).lhu(c);
        c.g[A0][0] = len;
        Addr(track as u32).off(INDEX).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((next as u32).wrapping_add(len as u32));
        Addr(track as u32).off(CURSOR).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 2;
        Addr(track as u32).off(CURSOR).sw(c, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Ramp tracks.

/// `void InSeq_RampTrackActivateKey(RampTrack* track, RampKey* key)`
/// (0x273860): copy a ramp key (including its converted integer/float fields)
/// into the `index * 0x24` record at `track + 0x1c` and link it into the active
/// list at +0x20.
pub fn in_seq_ramp_track_activate_key(c: &mut Cpu) {
    const ACTIVE_LIST: u32 = 0x1c;
    const LINK: u32 = 0x20;
    const COUNT: u32 = 0x24;
    const RECORD: u32 = 0x24;
    const KEY_INDEX: u32 = 0x2;
    const KEY_START: u32 = 0x8;
    const KEY_INT: u32 = 0xc;
    const KEY_FLOAT: u32 = 0x10;
    const KEY_END: u32 = 0x14;
    const KEY_FLAGS: u32 = 0x4;
    const KEY_KIND: u32 = 0x6;
    c.cycles += 27;
    let key = c.g[A1][0];
    c.g[A3][0] = key;
    let track = c.g[A0][0];
    c.g[T0][0] = track;
    let index = Addr(key as u32).off(KEY_INDEX).lhu(c);
    c.g[V0][0] = index;
    c.g[A1][0] = RECORD as u64;
    let base = Addr(track as u32).off(ACTIVE_LIST).lw(c);
    c.g[A0][0] = base;
    let scaled = abi::mult(c, index as u32, RECORD);
    c.g[V0][0] = sx(scaled);
    let start = Addr(key as u32).off(KEY_START).lw(c);
    c.g[V1][0] = start;
    let slot = sx((base as u32).wrapping_add(scaled));
    c.g[A2][0] = slot;
    Addr(slot as u32).sw(c, start);
    let end = Addr(key as u32).off(KEY_END).lw(c);
    c.g[V0][0] = end;
    Addr(slot as u32).off(0x18).sw(c, end);
    let converted = FpuF32::from_i32(Addr(key as u32).off(KEY_INT).read_fpu(c).0 as i32);
    c.f[0] = converted.0;
    Addr(slot as u32).off(0x10).write_fpu(c, converted);
    let rate = Addr(key as u32).off(KEY_FLOAT).read_fpu(c);
    c.f[1] = rate.0;
    Addr(slot as u32).off(0x14).write_fpu(c, rate);
    let index2 = Addr(key as u32).off(KEY_INDEX).lhu(c);
    c.g[V0][0] = index2;
    Addr(slot as u32).off(0xc).sw(c, index2);
    let flags = Addr(key as u32).off(KEY_FLAGS).lhu(c);
    c.g[V1][0] = flags;
    Addr(slot as u32).off(0x4).sw(c, flags);
    let kind = Addr(key as u32).off(KEY_KIND).lhu(c);
    c.g[A0][0] = kind;
    Addr(slot as u32).off(0x1c).sw(c, 0);
    Addr(slot as u32).off(0x8).sw(c, kind);
    let kind_field = Addr(key as u32).lhu(c);
    c.g[V0][0] = kind_field;
    let short = (kind_field < 0x19) as u64;
    c.g[V0][0] = short;
    let link;
    if short != 0 {
        link = Addr(track as u32).off(LINK).lw(c);
        c.g[V1][0] = link;
    } else {
        c.cycles += 3;
        let payload = sx((key as u32).wrapping_add(0x18));
        c.g[V0][0] = payload;
        Addr(slot as u32).off(0x1c).sw(c, payload);
        link = Addr(track as u32).off(LINK).lw(c);
        c.g[V1][0] = link;
    }
    c.cycles += 7;
    c.g[V0][0] = c.g[A2][0];
    Addr(slot as u32).off(0x20).sw(c, c.g[V1][0]);
    Addr(track as u32).off(LINK).sw(c, c.g[A2][0]);
    let count = Addr(track as u32).off(COUNT).lw(c);
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    Addr(track as u32).off(COUNT).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void* InSeq_RampTrackNextKey(RampTrack* track)` (0x2738f8): same cursor
/// walk as the span track, over the ramp list.
pub fn in_seq_ramp_track_next_key(c: &mut Cpu) {
    const KEYS: u32 = 0x10;
    const CURSOR: u32 = 0x14;
    const INDEX: u32 = 0x18;
    c.cycles += 7;
    let track = c.g[A0][0];
    c.g[A1][0] = track;
    let keys = Addr(track as u32).off(KEYS).lw(c);
    c.g[V1][0] = keys;
    let cur = Addr(track as u32).off(INDEX).lw(c);
    c.g[A0][0] = cur;
    let count = Addr(keys as u32).lhu(c);
    c.g[V0][0] = count;
    let last = sx((count as u32).wrapping_sub(1));
    c.g[V0][0] = last;
    if cur != last {
        let next = Addr(track as u32).off(CURSOR).lw(c);
        c.g[V0][0] = next;
        c.cycles += 6;
        c.g[V1][0] = sx((cur as u32).wrapping_add(1));
        let len = Addr(next as u32).lhu(c);
        c.g[A0][0] = len;
        Addr(track as u32).off(INDEX).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((next as u32).wrapping_add(len as u32));
        Addr(track as u32).off(CURSOR).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 2;
        Addr(track as u32).off(CURSOR).sw(c, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Channels.

/// `InSeq_Channel* InSeq_Channel_ctor(InSeq_Channel* self)` (0x273938): zero the
/// non-head fields (offsets 4..0x2c) and return the channel.
pub fn in_seq_channel_ctor(c: &mut Cpu) {
    c.cycles += 13;
    let ch = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    ch.off(0x4).sw(c, 0);
    ch.off(0x8).sw(c, 0);
    ch.off(0xc).sw(c, 0);
    ch.off(0x10).sw(c, 0);
    ch.off(0x14).sw(c, 0);
    ch.off(0x18).sw(c, 0);
    ch.off(0x1c).sw(c, 0);
    ch.off(0x20).sw(c, 0);
    ch.off(0x24).sw(c, 0);
    ch.off(0x28).sw(c, 0);
    ch.off(0x2c).sw(c, 0);
    abi::ret(c);
}

/// `void InSeq_ChannelSetup(InSeq_Channel* ch, void* data, void* listener)`
/// (0x2739b8): bind the data block and listener, record the first track at
/// `+0x1c` (+0x20 when the block's kind is short) and return the bound data.
pub fn in_seq_channel_setup(c: &mut Cpu) {
    const DATA: u32 = 0x4;
    const LISTENER: u32 = 0x8;
    const DATA_SAVED: u32 = 0x10;
    const DATA_END: u32 = 0x14;
    const TRACK: u32 = 0x1c;
    const TRACK_LONG: u32 = 0x20;
    const DATA_OFFSET: u32 = 0x8;
    c.cycles += 13;
    let ch = Addr(abi::arg(c, 0));
    let data = c.g[A1][0];
    c.g[V0][0] = sx((data as u32).wrapping_add(DATA_OFFSET));
    ch.off(DATA).sw(c, data);
    ch.off(DATA_SAVED).sw(c, data);
    ch.off(LISTENER).sw(c, c.g[A2][0]);
    ch.off(DATA_END).sw(c, c.g[V0][0]);
    ch.off(0x18).sw(c, 0);
    let delta = Addr(data as u32).off(DATA_OFFSET).lw(c);
    c.g[V0][0] = delta;
    let track = sx((data as u32).wrapping_add(delta as u32));
    c.g[A1][0] = track;
    ch.off(TRACK).sw(c, track);
    let kind = Addr(track as u32).lhu(c);
    c.g[V0][0] = kind;
    let short = (kind < 9) as u64;
    c.g[V0][0] = short;
    if short != 0 {
        ch.off(TRACK_LONG).sw(c, 0);
    } else {
        c.cycles += 2;
        c.g[V0][0] = sx((track as u32).wrapping_add(DATA_OFFSET));
        ch.off(TRACK_LONG).sw(c, c.g[V0][0]);
    }
    c.cycles += 3;
    let saved = ch.off(DATA_SAVED).lw(c);
    c.g[V1][0] = saved;
    c.g[V0][0] = Addr(saved as u32).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Instances.

/// `InSeq_Instance* InSeq_Instance_ctor(InSeq_Instance* self)` (0x274518):
/// initialise the instance (gain 1.0 at +0x24, -1 at +0/+0x30/+0x34, zero
/// elsewhere) and return it.
pub fn in_seq_instance_ctor(c: &mut Cpu) {
    const GAIN: u32 = 0x24;
    const MINUS_ONE_A: u32 = 0x30;
    const MINUS_ONE_B: u32 = 0x34;
    c.cycles += 23;
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = c.g[AT][0] as u32;
    let minus_one = 0xffff_ffff_ffff_ffffu64;
    c.g[V1][0] = minus_one;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    this.off(MINUS_ONE_B).sw(c, minus_one);
    this.off(GAIN).write_fpu(c, FpuF32(c.f[0]));
    this.sw(c, minus_one);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x18).sw(c, 0);
    this.off(0x1c).sw(c, 0);
    this.off(0x20).sw(c, 0);
    this.off(0x28).sw(c, 0);
    this.off(0x2c).sw(c, 0);
    this.off(MINUS_ONE_A).sw(c, minus_one);
    this.off(0x38).sw(c, 0);
    this.off(0x3c).sw(c, 0);
    this.off(0x40).sw(c, 0);
    this.off(0x44).sw(c, 0);
    abi::ret(c);
}

/// `void InSeq_InstanceStart(InSeq_Instance* inst, void* data)` (0x274830):
/// start playback, recording the current track at +0x1c; if the instance is
/// already running and has a pending start request at +0x3c, clear +0x40.
pub fn in_seq_instance_start(c: &mut Cpu) {
    const ACTIVE: u32 = 0x20;
    const TRACK: u32 = 0x1c;
    const PENDING: u32 = 0x3c;
    const REQUEST: u32 = 0x40;
    c.cycles += 3;
    let inst = Addr(abi::arg(c, 0));
    let active = inst.off(ACTIVE).lw(c);
    c.g[V0][0] = active;
    let one = 0x1u64;
    c.g[V0][0] = one;
    if active == 0 {
        c.cycles += 2;
        inst.off(TRACK).sw(c, c.g[A1][0]);
        inst.off(ACTIVE).sw(c, one);
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    let pending = inst.off(PENDING).lw(c);
    c.g[V0][0] = pending;
    if pending != 0 {
        c.cycles += 2;
        inst.off(REQUEST).sw(c, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void InSeq_InstancePause(InSeq_Instance* inst)` (0x2749e8): while running,
/// increment the pause counter at +0x2c.
pub fn in_seq_instance_pause(c: &mut Cpu) {
    const ACTIVE: u32 = 0x20;
    const PAUSE_COUNT: u32 = 0x2c;
    c.cycles += 3;
    let inst = Addr(abi::arg(c, 0));
    let active = inst.off(ACTIVE).lw(c);
    c.g[V0][0] = active;
    if active != 0 {
        c.cycles += 3;
        let count = inst.off(PAUSE_COUNT).lw(c);
        c.g[V0][0] = sx((count as u32).wrapping_add(1));
        inst.off(PAUSE_COUNT).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void InSeq_InstanceResume(InSeq_Instance* inst)` (0x274a08): while running,
/// decrement the pause counter at +0x2c, clamping at zero.
pub fn in_seq_instance_resume(c: &mut Cpu) {
    const ACTIVE: u32 = 0x20;
    const PAUSE_COUNT: u32 = 0x2c;
    c.cycles += 3;
    let inst = Addr(abi::arg(c, 0));
    let active = inst.off(ACTIVE).lw(c);
    c.g[V0][0] = active;
    if active != 0 {
        c.cycles += 4;
        let count = inst.off(PAUSE_COUNT).lw(c);
        c.g[V0][0] = sx((count as u32).wrapping_sub(1));
        let value = c.g[V0][0];
        inst.off(PAUSE_COUNT).sw(c, value);
        if (value as i64) < 0 {
            c.cycles += 1;
            inst.off(PAUSE_COUNT).sw(c, 0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Sequences.

/// `InSeqChannel *InSeq_InstanceGetChannel(InSeq *self, int index)` (0x274c20):
/// the `index`-th channel record (0x3c bytes each) whose array starts at
/// `self + 0x18`. Returns a pointer computed as `base + index*0x3c`.
pub fn in_seq_instance_get_channel(c: &mut Cpu) {
    c.cycles += 5;
    const CHANNELS: u32 = 0x18;
    const CHANNEL_SIZE: u32 = 0x3c;
    c.g[V1][0] = CHANNEL_SIZE as u64;
    let base = Addr(abi::arg(c, 0)).off(CHANNELS).lw(c);
    c.g[V0][0] = base;
    let offset = abi::mult(c, abi::arg(c, 1), CHANNEL_SIZE);
    c.g[A1][0] = sx(offset);
    c.g[V0][0] = sx((base as u32).wrapping_add(offset));
    abi::ret(c);
}

/// `InSeqListener *InSeq_InstanceGetListener(InSeq *self)` (0x274d70): the
/// listener cached at `self + 0x44` when set, otherwise the object reached
/// through `self + 0x8` (`*(self->+0x8)`).
pub fn in_seq_instance_get_listener(c: &mut Cpu) {
    c.cycles += 3;
    const CACHE: u32 = 0x44;
    const LINK: u32 = 0x8;
    let self_ = Addr(abi::arg(c, 0));
    let cached = self_.off(CACHE).lw(c);
    c.g[V0][0] = cached;
    if cached != 0 {
        c.cycles += 2;
    } else {
        c.cycles += 3;
        let link = self_.off(LINK).lw(c);
        c.g[V1][0] = link;
        c.g[V0][0] = Addr(link as u32).lw(c);
    }
    abi::ret(c);
}

/// `int Replay_GetMaxHighlights(Replay* r, int unused)` (0x270c40): the
/// highlight count at +0xc, or 1 when there is no replay.
pub fn replay_get_max_highlights(c: &mut Cpu) {
    c.cycles += 2;
    let unused = c.g[A1][0];
    c.g[V0][0] = 0x1;
    if unused == 0 {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        let selfp = abi::arg(c, 0);
        c.g[V0][0] = c.lw(selfp.wrapping_add(0xc));
        abi::ret(c);
    }
}

/// `void *InSeq_GetHeaderExtra(SeqHeader *h)` (0x00274c38): the extra header
/// at `+0xc` for format versions >= 13, else null.
pub fn in_seq_get_header_extra(c: &mut Cpu) {
    c.cycles += 4;
    let version = Addr((c.g[A0][0] as u32).wrapping_add(2)).lhu(c);
    let old = (version < 0xd) as u64;
    c.g[V0][0] = old;
    c.cycles += 2;
    if old != 0 {
        c.g[V0][0] = 0;
    } else {
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Deleting destructors.

/// The shared body of the `(flags & 1)` deleting destructors: delete `this`
/// when the low bit of `a1` is set (the frame is still opened either way).
pub(crate) fn destroy_if_flag(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 4;
    let frame = Frame::open(c, 0x10);
    c.g[A1][0] &= 1;
    frame.save_ra(c, 0);
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        call_ptr(c, table_callee::OPERATOR_DELETE, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void InSeq_dtor(InSeq *this, int flags)` (0x2728d0).
pub fn in_seq_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x0027_28e8))
}

// ---------------------------------------------------------------------------
// Sequence / audio / object helpers.

/// `void InSeq_Stop(InSeq *self, int slot)` (0x272b58): fetch the sequence
/// instance for `slot` and stop it.
pub fn in_seq_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::INSEQ_GET_INSTANCE.call(c, 0x0027_2b68)?;
        c.g[A0][0] = c.g[V0][0];
        callee::INSEQ_INSTANCE_STOP.call(c, 0x0027_2b70)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ReplayStream_dtor(ReplayStream *self, int flags)` (0x2721d0):
/// republishes `D_00481898` at `self+0`, deletes on bit 0.
pub fn replay_stream_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_1898, 0, 0x0027_21f4))
}

/// `void WorldStateStream_dtor(WorldStateStream *self, int flags)`
/// (0x272258): republishes `D_00481898` at `self+0`, deletes on bit 0.
pub fn world_state_stream_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_1898, 0, 0x0027_227c))
}

// ---------------------------------------------------------------------------
// Script / sequence.

/// `void InSeq_Play(Channel *ch, int index, void *owner)` (0x272ad0): fetch
/// the instrument-sequence instance for the slot and start it with `a2`.
pub fn in_seq_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        callee::INSEQ_GET_INSTANCE_2.call(c, 0x0027_2ae4)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        callee::INSEQ_INSTANCE_START.call(c, 0x0027_2af0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_HighlightSlotInit(ReplaySlot* slot)` (0x272288): mark the
/// slot empty and clear its 0xac-byte payload.
pub fn replay_highlight_slot_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = u64::MAX;
        Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        Addr((c.g[V0][0] as u32).wrapping_add(0xb0)).sw(c, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0xac;
        call_ptr(c, callee::EA_MEMSET, 0x0027_22b0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// InSeq track constructors: install the class vtable at +4 and clear the
// following fields.

/// Shared body: install the class vtable at +4, clear the following fields
/// and return `self` (the ctors are leaves, so `pc` goes to `ra`).
pub(crate) fn in_seq_track_ctor(c: &mut Cpu, vtable: u32, zero_offsets: &[u32]) {
    c.g[V1][0] = 0x0048_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(vtable);
    Addr((c.g[A0][0] as u32).wrapping_add(8)).sw(c, 0);
    Addr((c.g[A0][0] as u32).wrapping_add(4)).sw(c, c.g[V1][0]);
    for &off in zero_offsets {
        Addr((c.g[A0][0] as u32).wrapping_add(off)).sw(c, 0);
    }
    crate::abi::ret(c);
}

/// `InSeq_CurveTrack* InSeq_CurveTrack_ctor(InSeq_CurveTrack* self)`
/// (0x2725b8).
pub fn in_seq_curve_track_ctor(c: &mut Cpu) {
    c.cycles += 13;
    in_seq_track_ctor(c, 0x0048_1a10, &[0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24]);
}

/// `InSeq_SpanTrack* InSeq_SpanTrack_ctor(InSeq_SpanTrack* self)`
/// (0x272d68).
pub fn in_seq_span_track_ctor(c: &mut Cpu) {
    c.cycles += 13;
    in_seq_track_ctor(c, 0x0048_19b0, &[0xc, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28]);
}

/// `InSeq_EventTrack* InSeq_EventTrack_ctor(InSeq_EventTrack* self)`
/// (0x273180).
pub fn in_seq_event_track_ctor(c: &mut Cpu) {
    c.cycles += 13;
    in_seq_track_ctor(c, 0x0048_1950, &[0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24]);
}

/// `InSeq_RampTrack* InSeq_RampTrack_ctor(InSeq_RampTrack* self)`
/// (0x2734a8).
pub fn in_seq_ramp_track_ctor(c: &mut Cpu) {
    c.cycles += 13;
    in_seq_track_ctor(c, 0x0048_18f0, &[0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24]);
}

/// `void Replay_OpenMenu(ReplayCache* self)` (0x2706b8): open the replay menu
/// unless a dialog is active or it is already open.
pub fn replay_open_menu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x61c).lw(c);
        let busy = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if busy {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
        c.g[V0][0] = 9;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A1][0] = 9;
        callee::REPLAY_SET_STATE.call(c, 0x0027_06e4)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_CancelHighlight(sRcSlots* self, int slot)` (0x270de0): clear the
/// highlight flag of slot `i` when the replay is idle.
pub fn replay_cancel_highlight(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        let active = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !active {
            c.cycles += 6;
            c.g[A0][0] = 0xb4;
            c.mult(0, A0, A1, A0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x494));
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            callee::REPLAY_HIGHLIGHT_SLOT_CLEAR.call(c, 0x0027_0e0c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SaveWriteHeader(ReplayCache* self)` (0x2712c0): emit the 0x28
/// byte save header and bump the record counter.
pub fn replay_save_write_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x28;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x640).lw(c);
        callee::REPLAY_SAVE_EMIT.call(c, 0x0027_12dc)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x638).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let v0 = c.g[V0][0];
        let s0 = c.g[S0][0];
        Addr(s0 as u32).off(0x638).sw(c, v0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_EndInstantReplay(Replay* self)` (0x270670): when an instant
/// replay is running, end the playback and let the rider resume.
pub fn replay_end_instant_replay(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(at(c, A0, 0x620));
        let idle = c.g[V0][0] == 0;
        c.g[RA][0] = c.ld(p(sp(c), 0));
        if !idle {
            c.cycles += 10;
            callee::REPLAY_END_PLAYBACK.call(c, 0x0027_068c)?;
            c.g[V0][0] = c.lw(gp(c, -0x848));
            c.g[V1][0] = c.lw(at(c, V0, 0x84));
            c.g[A0][0] = c.lw(at(c, V1, 0x0c));
            c.g[V0][0] = c.lw(at(c, A0, 0x40));
            c.g[V1][0] = c.lw(at(c, V0, 0x18));
            c.g[A0][0] = c.lw(at(c, V1, 0x790));
            call_ptr(c, callee::APP_SOMETHING, 0x0027_06a8)?;
            c.g[RA][0] = c.ld(p(sp(c), 0));
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_FreeSaveBuffers(Replay* self)` (0x270f78): free both save
/// buffers and clear the fields that point at them.
pub fn replay_free_save_buffers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(at(c, S0, 0x63c));
        call_ptr(c, callee::BUILTIN_DELETE_2, 0x0027_0f90)?;
        c.g[A0][0] = c.lw(at(c, S0, 0x640));
        if c.g[A0][0] == 0 {
            sw(c, r32(c, S0), 0x640, 0);
        } else {
            c.cycles += 3;
            call_ptr(c, callee::EA_MEM_FREE_2, 0x0027_0fa4)?;
            sw(c, r32(c, S0), 0x640, 0);
        }
        c.cycles += 5;
        sw(c, r32(c, S0), 0x63c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// In-sequence channel data.

/// `void InSeq_Channel_dtor(InSeq_Channel* self, int flags)` (0x273970):
/// release the channel's block, then delete it from the heap when `flags &
/// 1`.
pub fn in_seq_channel_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::IN_SEQ_CHANNEL_RELEASE.call(c, 0x0027_398c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::IN_SEQ_HEAP_DELETE.call(c, 0x0027_39a0)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void InSeq_ChannelSelectBlock(InSeq_Channel* self, int index)`
/// (0x273a60): select the sequence block at `index` and point at its first
/// event record.
pub fn in_seq_channel_select_block(c: &mut Cpu) {
    c.cycles += 13;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V1][0] = c.lw(at(c, A2, 0x14));
    sw(c, r32(c, A2), 0x18, c.g[A1][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = c.lw(at(c, A2, 0x04));
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    sw(c, r32(c, A2), 0x1c, c.g[A0][0]);
    c.g[V0][0] = c.lhu(r32(c, A0));
    c.g[V0][0] = (c.g[V0][0] < 9u64) as u64;
    let small = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x08));
    if small {
        c.cycles += 2;
        sw(c, r32(c, A2), 0x20, 0);
    } else {
        c.cycles += 2;
        sw(c, r32(c, A2), 0x20, c.g[V0][0]);
    }
    abi::ret(c);
}

/// `void Replay_SaveEmit(ReplayCache* self, const signed char* data, int
/// size)` (0x271058): account `size` bytes (and their byte sum) and remember
/// the last emitted block.
pub fn replay_save_emit(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.g[V0][0] = Addr(this).off(0x680).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
    let empty = (c.g[A2][0] as i64) <= 0;
    Addr(this).off(0x680).sw(c, c.g[V0][0]);
    if !empty {
        c.cycles += 2;
        c.g[T0][0] = c.g[A1][0];
        c.g[A3][0] = c.g[A2][0];
        loop {
            c.cycles += 7;
            c.g[V1][0] = c.lb(r32(c, T0));
            c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
            c.g[V0][0] = Addr(this).off(0x684).lw(c);
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            let more = c.g[A3][0] != 0;
            Addr(this).off(0x684).sw(c, c.g[V0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_1070);
            }
        }
    }
    c.cycles += 6;
    c.g[V0][0] = Addr(this).off(0x678).lw(c);
    Addr(this).off(0x644).sw(c, c.g[A1][0]);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
    Addr(this).off(0x648).sw(c, c.g[A2][0]);
    Addr(this).off(0x678).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `int Replay_CountSaveSnapshots(ReplayCache* self)` (0x2712f8): walk the
/// snapshot chain from `+0x3b0`, skipping the one at `+0x3d0`, and return how
/// many others there are.
pub fn replay_count_save_snapshots(c: &mut Cpu) {
    c.cycles += 4;
    let this = r32(c, A0);
    c.g[V0][0] = Addr(this).off(0x3b0).lw(c);
    c.g[V1][0] = 0;
    let empty = c.g[V0][0] == 0;
    Addr(this).off(0x65c).sw(c, c.g[V0][0]);
    if !empty {
        c.cycles += 2;
        c.g[A1][0] = Addr(this).off(0x3d0).lw(c);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x65c).lw(c);
            if c.g[V0][0] != c.g[A1][0] {
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V0)).off(0x14).lw(c);
                Addr(this).off(0x65c).sw(c, c.g[V0][0]);
                c.g[V0][0] = Addr(this).off(0x65c).lw(c);
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, V0)).off(0x14).lw(c);
            let more = c.g[V0][0] != 0;
            Addr(this).off(0x65c).sw(c, c.g[V0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_1310);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

/// `void Replay_SaveWriteFooter(ReplayCache* self)` (0x271708): clear the
/// 8-byte header block and emit it as the save footer.
pub fn replay_save_write_footer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 8;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = sx(this.wrapping_add(0x680));
        c.g[A0][0] = Addr(this).off(0x640).lw(c);
        call_ptr(c, 0x003e_6574, 0x0027_1728)?;
        c.g[A1][0] = Addr(this).off(0x640).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 8;
        callee::REPLAY_SAVE_EMIT_2.call(c, 0x0027_1738)?;
        c.g[V0][0] = Addr(this).off(0x638).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        Addr(this).off(0x638).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_ClearAllHighlights(Replay* r)` (0x270e18): unless a replay is
/// running, clear the two highlight slots at +0x494 and +0x548. 7 (+3, 5×2+1)
/// + 4 cycles plus the callees.
pub fn replay_clear_all_highlights(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        let state = c.lw(self_);
        c.g[V0][0] = state;
        if state == 0 {
            c.cycles += 3;
            let mut slot = p_2(self_, 0x494);
            c.g[S0][0] = sx(slot);
            c.g[S1][0] = 1;
            loop {
                c.cycles += 5;
                c.g[A0][0] = sx(slot);
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2.call(c, 0x0027_0e4c)?;
                let more = c.g[S1][0] as i64 >= 0;
                slot = p_2(slot, 0xb4);
                c.g[S0][0] = sx(slot);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_0e40);
                    }
                    continue;
                }
                c.cycles += 1;
                frame.restore(c, 16, 0x20);
                break;
            }
        } else {
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sequencer tracks.

/// The shared body of the three `InSeq_*Track` destructors: install the class
/// vtable at +4, stop the track and, when bit 0 of `flags` is set, delete the
/// object. 13 + 4 (+3) cycles plus the callees.
pub(crate) fn inseq_track_dtor(c: &mut Cpu, vt: u32, stop: Callee, ret: u32) -> Flow {
    c.cycles += 13;
    let _frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(vt);
    c.sq(17, p_2(sp(c), 0x10));
    c.sq(16, p_2(sp(c), 0x20));
    c.g[S1][0] = c.g[A0][0];
    c.sd(p_2(sp(c), 0), c.g[RA][0]);
    let flags = c.g[A1][0];
    c.g[S0][0] = flags;
    c.sw(p_2(r32(c, S1), 4), c.g[V0][0]);
    stop.call(c, ret)?;
    c.g[S0][0] = c.g[S0][0] & 1;
    let keep = c.g[S0][0] == 0;
    c.lq(16, p_2(sp(c), 0x20));
    if !keep {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        callee::INSEQ_HEAP_DELETE.call(c, ret + 0x14)?;
        c.lq(16, p_2(sp(c), 0x20));
    }
    c.cycles += 4;
    c.lq(17, p_2(sp(c), 0x10));
    c.g[RA][0] = c.ld(p_2(sp(c), 0));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
    c.pc = ra;
    Ok(())
}

/// `void InSeq_SpanTrack_dtor(InSeq_SpanTrack* t, int flags)` (0x272da0).
pub fn inseq_span_track_dtor(c: &mut Cpu) {
    guest(c, |c| {
        inseq_track_dtor(c, 0x0048_19b0, callee::INSEQ_SPAN_TRACK_STOP, 0x0027_2dc8)
    })
}

/// `void InSeq_EventTrack_dtor(InSeq_EventTrack* t, int flags)` (0x2731b8).
pub fn inseq_event_track_dtor(c: &mut Cpu) {
    guest(c, |c| {
        inseq_track_dtor(c, 0x0048_1950, callee::INSEQ_EVENT_TRACK_STOP, 0x0027_31e0)
    })
}

/// `void InSeq_RampTrack_dtor(InSeq_RampTrack* t, int flags)` (0x2734e0).
pub fn inseq_ramp_track_dtor(c: &mut Cpu) {
    guest(c, |c| {
        inseq_track_dtor(c, 0x0048_18f0, callee::INSEQ_RAMP_TRACK_STOP, 0x0027_3508)
    })
}

/// `void Player_FreeReplayCache(Player* p_3, void* cache)` (0x2700d8): find
/// `cache` in the player's two replay slots, destroy it and clear the slot.
pub fn player_free_replay_cache(c: &mut Cpu) {
    c.cycles += 4;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    let found_at = c.g[A1][0];
    frame.save_ra(c, 0);
    if found_at != 0 {
        c.cycles += 2;
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48c));
        c.g[V1][0] = 0;
        loop {
            c.cycles += 3;
            c.g[A0][0] = Addr(p_3(c, S0, 0)).lw(c);
            if c.g[A0][0] != c.g[A1][0] {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.sw(p_3(c, S0, 0), 0);
                    break;
                }
                c.cycles += 4;
                c.g[A1][0] = 3;
                if callee::REPLAY_CACHE_DTOR.call(c, 0x0027_010c).is_err() {
                    return;
                }
                c.sw(p_3(c, S0, 0), 0);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_00f0);
            }
        }
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void Replay_AllocSaveBuffers(Replay* r)` (0x270f20): allocate the 16 KiB
/// replay buffer and the 50 KiB save buffer.
pub fn replay_alloc_save_buffers(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = sx(0x0048_1800);
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = 0x4000;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    if call_ptr(c, table::EA_MEM_ALLOC_NAMED, 0x0027_0f48).is_err() {
        return;
    }
    c.g[A1][0] = sx(0x0048_1810);
    c.sw(p_3(c, S0, 0x63c), c.g[V0][0]);
    c.g[A0][0] = 0xc800;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    if call_ptr(c, table::OPERATOR_NEW_NAMED, 0x0027_0f64).is_err() {
        return;
    }
    c.sw(p_3(c, S0, 0x640), c.g[V0][0]);
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// Return -1 (the "no event" sentinel): the shared tail of the three
/// `InSeq_*TrackGetNextEventTime` queries.
pub(crate) fn ret_minus_one(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Replay-cache track event times.

/// `int InSeq_SpanTrackGetNextEventTime(track)` (0x273040): the earliest event
/// time in the span track's node list (`+0x24`, time at `+0xc`, next at
/// `+0x18`) or on the current node (`+0x14`, time at `+4`); -1 when the track
/// or its event table is empty.
pub fn in_seq_span_track_get_next_event_time(c: &mut Cpu) {
    let track = Addr(abi::arg(c, 0));
    c.cycles += 3;
    c.g[V0][0] = track.off(0x8).lw(c);
    if c.g[V0][0] == 0 {
        return ret_minus_one(c);
    }
    c.g[V0][0] = track.off(0x10).lw(c);
    c.cycles += 3;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
    let empty = c.g[V1][0] == 0;
    c.g[A2][0] = 0xffff_ffff_ffff_ffff;
    if empty {
        return ret_minus_one(c);
    }

    c.cycles += 3;
    c.g[V1][0] = track.off(0x24).lw(c);
    let no_nodes = c.g[V1][0] == 0;
    c.g[A0][0] = track.off(0x14).lw(c);
    if !no_nodes {
        c.cycles += 1;
        loop {
            c.cycles += 2;
            let best_negative = (c.g[A2][0] as i64) < 0;
            let node = c.g[V1][0] as u32;
            c.g[A1][0] = Addr(node).off(0xc).lw(c);
            let mut improve = best_negative;
            if !best_negative {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    improve = true;
                }
            }
            if improve {
                c.cycles += 2;
                c.g[A2][0] = c.g[A1][0];
            }
            let node = c.g[V1][0] as u32;
            c.g[V1][0] = Addr(node).off(0x18).lw(c);
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_3070);
            }
        }
    }

    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 2;
        let best_negative = (c.g[A2][0] as i64) < 0;
        let current = c.g[A0][0] as u32;
        c.g[A0][0] = Addr(current).off(0x4).lhu(c);
        let mut improve = best_negative;
        if !best_negative {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            improve = c.g[V0][0] != 0;
        }
        if improve {
            c.cycles += 1;
            c.g[A2][0] = c.g[A0][0];
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

/// `int InSeq_EventTrackGetNextEventTime(track)` (0x2733e0): the current event
/// node's time (`track + 0x14`, +2), or -1.
pub fn in_seq_event_track_get_next_event_time(c: &mut Cpu) {
    let track = Addr(abi::arg(c, 0));
    'fallback: {
        c.cycles += 3;
        c.g[V0][0] = track.off(0x8).lw(c);
        if c.g[V0][0] == 0 {
            break 'fallback;
        }
        c.g[V0][0] = track.off(0x10).lw(c);
        c.cycles += 3;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
        if c.g[V1][0] == 0 {
            break 'fallback;
        }
        c.cycles += 3;
        c.g[A0][0] = track.off(0x14).lw(c);
        let no_current = c.g[A0][0] == 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !no_current {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x2).lhu(c);
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    ret_minus_one(c);
}

/// `int InSeq_RampTrackGetNextEventTime(track)` (0x2737e0): the same query for
/// the ramp track (nodes at `+0x20`, time at `+8`, next at `+0x20`).
pub fn in_seq_ramp_track_get_next_event_time(c: &mut Cpu) {
    let track = Addr(abi::arg(c, 0));
    c.cycles += 3;
    c.g[V0][0] = track.off(0x8).lw(c);
    if c.g[V0][0] == 0 {
        return ret_minus_one(c);
    }
    c.g[V0][0] = track.off(0x10).lw(c);
    c.cycles += 3;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
    let empty = c.g[V1][0] == 0;
    c.g[A2][0] = 0xffff_ffff_ffff_ffff;
    if empty {
        return ret_minus_one(c);
    }

    c.cycles += 3;
    c.g[V1][0] = track.off(0x20).lw(c);
    let no_nodes = c.g[V1][0] == 0;
    c.g[A0][0] = track.off(0x14).lw(c);
    if !no_nodes {
        c.cycles += 1;
        loop {
            c.cycles += 2;
            let best_negative = (c.g[A2][0] as i64) < 0;
            let node = c.g[V1][0] as u32;
            c.g[A1][0] = Addr(node).off(0x8).lw(c);
            let mut improve = best_negative;
            if !best_negative {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    improve = true;
                }
            }
            if improve {
                c.cycles += 2;
                c.g[A2][0] = c.g[A1][0];
            }
            let node = c.g[V1][0] as u32;
            c.g[V1][0] = Addr(node).off(0x20).lw(c);
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_3810);
            }
        }
    }

    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 2;
        let best_negative = (c.g[A2][0] as i64) < 0;
        let current = c.g[A0][0] as u32;
        c.g[A0][0] = Addr(current).off(0x4).lhu(c);
        let mut improve = best_negative;
        if !best_negative {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
            improve = c.g[V0][0] != 0;
        }
        if improve {
            c.cycles += 1;
            c.g[A2][0] = c.g[A0][0];
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

/// `void InSeq_ChannelRelease(InSeqChannel* self)` (0x273a00): stop and clear
/// the channel when one is bound.
pub fn in_seq_channel_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p_4(c.g[S0][0], 4));
        if c.g[V0][0] != 0 {
            c.cycles += 14;
            call_ptr(c, 0x0027_3dc0, 0x0027_3a24)?;
            Addr(p_4(c.g[S0][0], 0x38)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 4)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 8)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0xc)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x10)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x14)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x18)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x1c)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x20)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x30)).sw(c, 0);
            Addr(p_4(c.g[S0][0], 0x34)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Replay.

/// `void Replay_SaveComputePadding(Replay* self)` (0x271620): clear the save
/// buffer, measure the payload and leave the padding count in `+0x67c`.
pub fn replay_save_compute_padding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0xc800;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p_5(c.g[S0][0] as u32, 0x640)).lw(c);
        call_ptr(c, 0x003e_6448, 0x0027_1640)?;
        call_ptr(c, 0x0014_dc80, 0x0027_1648)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_e048, 0x0027_1650)?;
        c.g[A1][0] = Addr(p_5(c.g[S0][0] as u32, 0x678)).lw(c);
        c.g[V1][0] = 0x0007_0000;
        c.g[A0][0] = Addr(p_5(c.g[S0][0] as u32, 0x638)).lw(c);
        c.g[V1][0] = c.g[V1][0] | 0xfff8;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        Addr(p_5(c.g[S0][0] as u32, 0x638)).sw(c, c.g[A0][0]);
        Addr(p_5(c.g[S0][0] as u32, 0x67c)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `void Replay_SaveWritePadding(Replay* self)` (0x271688): emit at most 0x4000
/// bytes of pending padding, or count an empty write.
pub fn replay_save_write_padding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p_5(c.g[S1][0] as u32, 0x67c)).lw(c);
        let no_padding = (c.g[V1][0] as i64) <= 0;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 16385) as u64;
        if no_padding {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_5(c.g[S1][0] as u32, 0x638)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(p_5(c.g[S1][0] as u32, 0x638)).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 9;
            c.g[S0][0] = 0x4000;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = c.g[V1][0];
            }
            c.g[A1][0] = Addr(p_5(c.g[S1][0] as u32, 0x640)).lw(c);
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0027_1058, 0x0027_16bc)?;
            c.g[V0][0] = Addr(p_5(c.g[S1][0] as u32, 0x67c)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
            Addr(p_5(c.g[S1][0] as u32, 0x67c)).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int InSeq_InstanceGetRemainingSteps(InSeq* self)` (0x274d08): the number
/// of remaining steps, rounded up.
pub fn in_seq_instance_get_remaining_steps(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0027_4c70, 0x0027_4d1c)?;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.lwc1(1, p_5(c.g[S0][0] as u32, 0x28));
        c.lwc1(2, p_5(c.g[S0][0] as u32, 0x24));
        c.sub_s(0, 0, 1);
        c.div_s(0, 0, 2);
        c.cvt_w_s(1, 0);
        c.cvt_s_w(1, 1);
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.cycles += 3;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.add_s(1, 1, 0);
        }
        c.cycles += 6;
        c.cvt_w_s(0, 1);
        c.mfc1(2, 0);
        frame.restore(c, S0, 0x10);
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `Replay_IsPaused`'s state table (`0x480000 + 0x17c0`), indexed by the
/// replay object's first word (only states < 0x10 are looked up).
pub(crate) const REPLAY_TABLE: u32 = 0x0048_17c0;

// ---------------------------------------------------------------------------
// Replay / audio predicates.

/// `int Replay_IsPaused(Replay *r)` (0x270280): the replay state word at
/// `+0` picks a handler from the table at [`REPLAY_TABLE`]. States 1 and 7
/// compare two counters (`+0x484` against `+0x3cc`, and `+0x604` must be
/// zero); states 3 and 5 return true, state 4 returns `+0x484 >= +0x3cc`,
/// most states return false. States >= 0x10 return false without a lookup.
///
/// One block per table target, in the generated code's order. The function
/// makes no calls and checks no events, so its only entry is the top.
pub fn replay_is_paused(c: &mut Cpu) {
    c.cycles += 5;
    let replay = r32(c, A0);
    c.g[A2][0] = c.g[A0][0];
    c.g[V1][0] = Addr(replay).lw(c);
    c.g[V0][0] = (c.g[V1][0] < 0x10) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[A3][0] = 0;
    if !out_of_range {
        c.cycles += 7;
        c.g[V0][0] = 0x0048_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx(REPLAY_TABLE);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        match c.g[A0][0] as u32 {
            // State 1 / 7: a pause request is visible in the two counters.
            0x0027_02b0 => {
                c.cycles += 9;
                c.g[A0][0] = 0x1;
                c.g[A1][0] = Addr(replay).off(0x3cc).lw(c);
                c.g[V1][0] = Addr(replay).off(0x604).lw(c);
                c.g[A3][0] = c.g[A0][0];
                c.g[V0][0] = Addr(replay).off(0x484).lw(c);
                if c.g[V1][0] != 0 {
                    c.g[A3][0] = 0;
                }
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[A0][0];
                }
            }
            // State 4: not yet caught up.
            0x0027_02d4 => {
                c.cycles += 5;
                c.g[V1][0] = Addr(replay).off(0x3cc).lw(c);
                c.g[V0][0] = Addr(replay).off(0x484).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[A3][0] = c.g[V0][0] ^ 0x1;
            }
            // States 3 and 5: paused.
            0x0027_02e8 => {
                c.cycles += 1;
                c.g[A3][0] = 0x1;
            }
            // Any other table entry keeps the zero result.
            0x0027_02ec => {}
            target => {
                c.jump_out(target, 0x0027_0280, 0x0027_02f4);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    abi::ret(c);
}

/// The three `InSeq_*TrackStop` bodies: release the track object through the
/// sequence's heap and clear the track's eight fields when an owner exists.
pub(crate) fn in_seq_track_stop(c: &mut Cpu, field: u32, clear: &[u32], ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        let owner = lw_at(c, this, 0xc);
        c.g[V0][0] = owner;
        if owner == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A1][0] = lw_at(c, this, field);
            let track = c.g[A1][0];
            if track == 0 {
                Addr(p_6(this, field + 8)).sw(c, 0);
            } else {
                c.cycles += 5;
                c.g[V0][0] = lw_at(c, owner as u32, 8);
                c.g[A0][0] = Addr(p_6(r32(c, V0), 8)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
                call_ptr(c, 0x0027_23a8, ret)?;
                Addr(p_6(this, field + 8)).sw(c, 0);
            }
            c.cycles += 8;
            for &off in clear {
                Addr(p_6(this, off)).sw(c, 0);
            }
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_SpanTrackStop(self)` 0x272ec0: release the span track at `+0x20`.
pub fn in_seq_span_track_stop(c: &mut Cpu) {
    in_seq_track_stop(
        c,
        0x20,
        &[0x8, 0xc, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28],
        0x0027_2ef8,
    )
}

/// `InSeq_EventTrackStop(self)` 0x2732c8: release the event track at `+0x1c`.
pub fn in_seq_event_track_stop(c: &mut Cpu) {
    in_seq_track_stop(
        c,
        0x1c,
        &[0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20],
        0x0027_3300,
    )
}

/// `InSeq_RampTrackStop(self)` 0x273630: release the ramp track at `+0x1c`.
pub fn in_seq_ramp_track_stop(c: &mut Cpu) {
    in_seq_track_stop(
        c,
        0x1c,
        &[0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20],
        0x0027_3668,
    )
}

/// `Player_AllocReplayCache(player)` 0x270070: allocate the first empty replay
/// cache slot (two at `+0x48c`), or return 0 when both are taken.
pub fn player_alloc_replay_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(p_6(this, 0x48c));
        loop {
            let slot = r32(c, S0);
            c.cycles += 3;
            c.g[V0][0] = Addr(slot).lw(c);
            let taken = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if !taken {
                c.cycles += 9;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x17b0));
                c.g[A0][0] = 0x10;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0027_00a8)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0026_d0a0, 0x0027_00b0)?;
                let cache = c.g[V0][0];
                Addr(slot).sw(c, cache);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_0088);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_CountHighlights(replay, bit, positive)` 0x270bd8: count the nodes
/// whose value bit is set and whose value has the requested sign.
pub fn replay_count_highlights(c: &mut Cpu) {
    c.cycles += 3;
    let this = abi::arg(c, 0);
    let bit = abi::arg(c, 1);
    let positive = abi::arg64(c, 2);
    c.g[A3][0] = lw_at(c, this, 0x3b0);
    c.g[T0][0] = 0;
    if c.g[A3][0] != 0 {
        c.cycles += 3;
        c.g[T1][0] = sx(bit << 2);
        c.g[T2][0] = u64::MAX;
        c.g[V0][0] = Addr(p_6(r32(c, A3), 0x1e)).lbu(c);
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> (bit & 31)) as u32);
            c.g[V0][0] &= 1;
            if c.g[V0][0] == 0 {
                c.g[A3][0] = Addr(p_6(r32(c, A3), 0x14)).lw(c);
            } else {
                c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, T1)));
                if positive == 0 {
                    c.cycles += 2;
                    c.cycles += 5;
                    c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, T1)));
                    c.g[A0][0] = sx(r32(c, T0).wrapping_add(1));
                    c.g[V1][0] = Addr(p_6(r32(c, V0), 0x20)).lw(c);
                    c.g[V1][0] = ((c.g[T2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[T0][0] = c.g[A0][0];
                    }
                } else {
                    c.cycles += 2;
                    c.cycles += 4;
                    c.g[V0][0] = Addr(p_6(r32(c, V1), 0x20)).lw(c);
                    c.g[V0][0] = ((0i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                }
                c.cycles += 1;
                c.g[A3][0] = Addr(p_6(r32(c, A3), 0x14)).lw(c);
            }
            c.cycles += 2;
            if c.g[A3][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(p_6(r32(c, A3), 0x1e)).lbu(c);
            if event_due(c) {
                deliver_event_here(c, 0x0027_0bf0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T0][0];
    abi::ret(c);
}

/// `Replay_BeginSave(self)` (0x270e68): allocate the save buffers and clear
/// the replay writer state (ending in the `0xdeadbeef` sentinel).
pub fn replay_begin_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0027_0f20, 0x0027_0e7c)?;
        c.g[V0][0] = 0xffff_ffff_dead_0000;
        Addr(p_7(c.g[S0][0] as u32, 0x638)).sw(c, 0);
        c.g[V0][0] = c.g[V0][0] | 0xbeef;
        Addr(p_7(c.g[S0][0] as u32, 0x644)).sw(c, 0);
        Addr(p_7(c.g[S0][0] as u32, 0x684)).sw(c, c.g[V0][0]);
        for off in [
            0x648u32, 0x64c, 0x650, 0x654, 0x658, 0x65c, 0x660, 0x664, 0x66c, 0x674, 0x678, 0x67c,
            0x680,
        ] {
            Addr(p_7(c.g[S0][0] as u32, off)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int InSeq_StartSequence(InSeq* seq, void* src)` (0x272938): take the first
/// free channel node, detach it from the free list, set up the instance and
/// push it onto the used list; returns the channel id.
pub fn in_seq_start_sequence(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        sqsp(c, 16, 0x20);
        c.g[S0][0] = Addr(p_8(r32(c, S1), 0x18)).lw(c);
        c.g[V0][0] = Addr(p_8(c.g[S0][0] as u32, 0x50)).lw(c);
        Addr(p_8(r32(c, S1), 0x18)).sw(c, c.g[V0][0]);
        Addr(p_8(r32(c, S0), 0x50)).sw(c, 0);
        c.g[V0][0] = Addr(p_8(r32(c, S1), 0x18)).lw(c);
        if c.g[V0][0] != 0 {
            Addr(p_8(c.g[V0][0] as u32, 0x54)).sw(c, 0);
        }
        c.cycles += 9;
        c.g[A3][0] = Addr(p_8(r32(c, S0), 4)).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A2][0] = c.g[S1][0];
        callee::IN_SEQ_INSTANCE_SETUP.call(c, 0x0027_2978)?;
        c.g[V1][0] = Addr(p_8(r32(c, S1), 0x1c)).lw(c);
        Addr(p_8(r32(c, S0), 0x50)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(p_8(r32(c, S1), 0x1c)).lw(c);
        if c.g[V0][0] != 0 {
            Addr(p_8(c.g[V0][0] as u32, 0x54)).sw(c, c.g[S0][0]);
        }
        c.cycles += 7;
        Addr(p_8(r32(c, S1), 0x1c)).sw(c, c.g[S0][0]);
        lqsp(c, 17, 0x10);
        c.g[V0][0] = Addr(p_8(r32(c, S0), 4)).lw(c);
        lqsp(c, 16, 0x20);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `void Replay_SeekToEnd(self)` (0x270378): find the oldest snapshot (frame
/// limit 100000000), record it and its position, then seek the two stream
/// run lists to that position.
pub fn replay_seek_to_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.g[A1][0] = 0x5f5_0000;
        c.sq(16, sp_2(c).wrapping_add(0x30));
        c.g[A1][0] = c.g[A1][0] | 0xe100;
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, sp_2(c).wrapping_add(0x20));
        c.sq(18, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S1][0] = sx(low(c, S0).wrapping_add(0x48c));
        sw_r(c, S0, 0x604, 0);
        c.g[S2][0] = 0x1;
        callee::REPLAY_FIND_SNAPSHOT_2.call(c, 0x0027_03a8)?;
        sw_r(c, S0, 0x488, c.g[V0][0]);
        c.g[V1][0] = lw_r(c, V0, 0x30);
        sw_r(c, S0, 0x484, c.g[V1][0]);
        c.g[V0][0] = Addr(low(c, S1)).lw(c);
        loop {
            c.cycles += 4;
            c.g[S2][0] = sx(low(c, S2).wrapping_sub(1));
            c.g[S1][0] = sx(low(c, S1).wrapping_add(4));
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !none {
                c.cycles += 2;
                c.g[A1][0] = lw_r(c, S0, 0x484);
                callee::RUN_LIST_SEEK.call(c, 0x0027_03d0)?;
            }
            c.cycles += 2;
            if (c.g[S2][0] as i64) >= 0 {
                c.g[V0][0] = Addr(low(c, S1)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_03b8);
                }
                continue;
            }
            c.cycles += 6;
            c.lq(16, sp_2(c).wrapping_add(0x30));
            c.lq(17, sp_2(c).wrapping_add(0x20));
            c.lq(18, sp_2(c).wrapping_add(0x10));
            c.g[RA][0] = Addr(sp_2(c)).ld(c);
            let t = low(c, RA);
            c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
            c.pc = t;
            break;
        }
        Ok(())
    })
}

/// `void Replay_CheckInstantReplay(self, obj, alt)` (0x2705b0): when a request
/// is pending and the race manager has exactly one rider, fire an instant
/// replay once the recorded span reaches 1000 ms, otherwise clear the request.
pub fn replay_check_instant_replay(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[A3][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[V0][0] = lw_r(c, A3, 0x3d4);
        let idle = c.g[V0][0] == 0;
        c.g[T0][0] = c.g[A1][0];
        if idle {
            c.cycles += 1;
            c.g[RA][0] = Addr(sp_2(c)).ld(c);
        } else {
            c.cycles += 7;
            c.g[A0][0] = Addr(gpo(c, -0x848)).lw(c);
            c.g[A1][0] = 0x1;
            c.g[V0][0] = lw_r(c, A0, 0x84);
            c.g[V1][0] = lw_r(c, V0, 0xc);
            c.g[A0][0] = lw_r(c, V1, 0x7c);
            let multi = c.g[A0][0] != c.g[A1][0];
            c.g[RA][0] = Addr(sp_2(c)).ld(c);
            if !multi {
                let now;
                let start;
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    c.g[V0][0] = lw_r(c, T0, 0xa4);
                    c.cycles += 1;
                    c.g[V1][0] = lw_r(c, A3, 0x47c);
                    now = c.g[V0][0];
                    start = c.g[V1][0];
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw_r(c, T0, 0x9c);
                    c.g[V1][0] = lw_r(c, A3, 0x474);
                    now = c.g[V0][0];
                    start = c.g[V1][0];
                }
                c.cycles += 4;
                c.g[V0][0] = sx((now as u32).wrapping_sub(start as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 1000) as u64;
                if c.g[V0][0] != 0 {
                    sw_r(c, A3, 0x3d4, 0);
                    c.cycles += 1;
                    c.g[RA][0] = Addr(sp_2(c)).ld(c);
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[A3][0];
                    callee::REPLAY_REQUEST_INSTANT_REPLAY.call(c, 0x0027_0614)?;
                    c.g[RA][0] = Addr(sp_2(c)).ld(c);
                }
            }
        }
        c.cycles += 2;
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// In-sequence channels.

/// `void* InSeq_ChannelSetListener(InSeqChannel* self, void* listener)`
/// (0x274240): release the previous listener, store the new one and hand it
/// its owner; returns the previous listener.
pub fn in_seq_channel_set_listener(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = Addr(p_9(r32(c, S0), 0x2c)).lw(c);
        let null = c.g[S2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 8;
            c.g[V0][0] = Addr(r32(c, S2)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = Addr(p_9(r32(c, S0), 0x1c)).lw(c);
            c.g[A0][0] = Addr(p_9(r32(c, V0), 0x10)).lh(c);
            c.g[A3][0] = Addr(p_9(r32(c, V0), 0x14)).lw(c);
            c.g[A2][0] = Addr(p_9(r32(c, V1), 3)).lbu(c);
            let target = r32(c, A3);
            c.g[RA][0] = sx(0x0027_4284);
            c.g[A0][0] = sx(p_9(r32(c, S2), r32(c, A0)));
            call_ptr(c, target, 0x0027_4284)?;
        }
        c.cycles += 2;
        let none = c.g[S1][0] == 0;
        Addr(p_9(r32(c, S0), 0x2c)).sw(c, c.g[S1][0]);
        if !none {
            c.cycles += 8;
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = Addr(p_9(r32(c, S0), 0x1c)).lw(c);
            c.g[A0][0] = Addr(p_9(r32(c, V0), 8)).lh(c);
            c.g[A3][0] = Addr(p_9(r32(c, V0), 0xc)).lw(c);
            c.g[A2][0] = Addr(p_9(r32(c, V1), 3)).lbu(c);
            let target = r32(c, A3);
            c.g[RA][0] = sx(0x0027_42ac);
            c.g[A0][0] = sx(p_9(r32(c, S1), r32(c, A0)));
            call_ptr(c, target, 0x0027_42ac)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay curve tracks.

/// `int InSeq_CurveTrackStart(void* self, CurveTrack* data, void* owner)`
/// (0x2725f0): attach a curve track (its key/value table and first segment)
/// to the in-sequence object and evaluate the first value.
pub fn in_seq_curve_track_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = sx(r32(c, A1).wrapping_add(4));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p_10(r32(c, S0), 0xc)).sw(c, c.g[A2][0]);
        Addr(p_10(r32(c, S0), 8)).sw(c, c.g[A1][0]);
        Addr(p_10(r32(c, S0), 0x10)).sw(c, c.g[A1][0]);
        Addr(p_10(r32(c, S0), 0x14)).sw(c, c.g[V1][0]);
        Addr(p_10(r32(c, S0), 0x18)).sw(c, 0);
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
        if c.g[V0][0] != 0 {
            Addr(p_10(r32(c, S0), 0x24)).sw(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_10(r32(c, A1), 4)).lhu(c);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            Addr(p_10(r32(c, S0), 0x24)).sw(c, c.g[V0][0]);
        }
        c.cycles += 10;
        c.g[A0][0] = Addr(p_10(r32(c, S0), 0x14)).lw(c);
        c.mtc1(0, 12);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        callee::MATH_EVAL_CUBIC.call(c, 0x0027_2644)?;
        c.g[V1][0] = Addr(p_10(r32(c, S0), 0x14)).lw(c);
        c.swc1(0, p_10(r32(c, S0), 0x1c));
        c.g[V0][0] = Addr(r32(c, V1)).lhu(c);
        c.g[V0][0] = (c.g[V0][0] < 0x15u64) as u64;
        if c.g[V0][0] != 0 {
            Addr(p_10(r32(c, S0), 0x20)).sw(c, 0);
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x14));
            Addr(p_10(r32(c, S0), 0x20)).sw(c, c.g[V0][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(p_10(r32(c, S0), 0x24)).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `Highlight* Replay_FindWeakestHighlight(Replay* self, int i, Highlight*
/// best, int positive)` (0x270c58): scan the highlight list for the entry
/// with the weakest value in column `i` (`positive`: the strongest).
pub fn replay_find_weakest_highlight(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T0][0] = c.lw(p_11(r32(c, A0), 0x3b0));
    let empty = c.g[T0][0] == 0;
    c.g[T1][0] = c.g[A2][0];
    if !empty {
        c.cycles += 1;
        c.g[V0][0] = c.lbu(at(c, T0, 0x1e));
        loop {
            c.cycles += 4;
            let shift = r32(c, A1) & 31;
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> shift) as u32);
            c.g[V0][0] = c.g[V0][0] & 1;
            if c.g[V0][0] == 0 {
                c.g[T0][0] = c.lw(at(c, T0, 0x14));
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lbu(at(c, T0, 0x1f));
                if c.g[V0][0] != 0 {
                    c.g[T0][0] = c.lw(at(c, T0, 0x14));
                } else {
                    // Take the element or skip it: `land` is where the
                    // original's `break` goes (1: re-test V0, 2: skip).
                    let next = c.lw(at(c, T0, 0x14));
                    c.cycles += 2;
                    let negative = c.g[A3][0] == 0;
                    c.g[V1][0] = sx(r32(c, A1) << 2);
                    let land: u8;
                    if negative {
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
                        c.g[A0][0] = c.lw(at(c, V0, 0x20));
                        if (c.g[A0][0] as i64) >= 0 {
                            land = 2;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
                            c.g[V0][0] = c.lw(at(c, V1, 0x20));
                            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            land = 1;
                        }
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
                        c.g[A0][0] = c.lw(at(c, V0, 0x20));
                        let not_positive = (c.g[A0][0] as i64) <= 0;
                        c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
                        if not_positive {
                            land = 2;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(at(c, V1, 0x20));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                            land = 1;
                        }
                    }
                    if land == 1 {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.g[T0][0] = next;
                        } else {
                            c.cycles += 2;
                            c.g[T1][0] = c.g[T0][0];
                            c.g[A2][0] = c.g[T1][0];
                            c.cycles += 1;
                            c.g[T0][0] = next;
                        }
                    } else if land == 2 {
                        c.cycles += 1;
                        c.g[T0][0] = next;
                    } else {
                        c.g[T0][0] = next;
                    }
                }
            }
            c.cycles += 2;
            if c.g[T0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lbu(at(c, T0, 0x1e));
            if event_due(c) {
                deliver_event_here(c, 0x0027_0c68);
            }
        }
    }
    c.cycles += 2;
    let ra = r32(c, RA);
    c.g[V0][0] = c.g[T1][0];
    c.pc = ra;
}

/// `void Replay_RecycleOldestSnapshot(Replay* self)` (0x2701a0): free the
/// oldest unlocked, unused snapshot in a two-pass walk of the ring.
pub fn replay_recycle_oldest_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0027_01a0 => {
                    c.cycles += 4;
                    let frame = Frame::open(c, 0x10);
                    c.g[A2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[A3][0] = 0;
                    pc = 0x0027_01b0;
                }
                0x0027_01b0 => {
                    c.cycles += 2;
                    c.g[V0][0] = lw_2(c, A2, 0x3ac);
                    pc = 0x0027_01f4;
                }
                0x0027_01b8 => {
                    c.cycles += 4;
                    c.g[A1][0] = lw_2(c, V0, 0x14);
                    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x1f).lbu(c);
                    let used = c.g[V1][0] != 0;
                    c.g[A0][0] = c.g[V0][0];
                    pc = if used { 0x0027_01ec } else { 0x0027_01c8 };
                }
                0x0027_01c8 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x1e).lbu(c);
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw_2(c, A0, 0x14);
                        pc = 0x0027_01f0;
                    } else {
                        pc = 0x0027_01d4;
                    }
                }
                0x0027_01d4 => {
                    c.cycles += 6;
                    c.g[V0][0] = lw_2(c, A1, 0x14);
                    c.g[A0][0] = c.g[A2][0];
                    Addr(c.g[A2][0] as u32).off(0x3ac).sw(c, c.g[V0][0]);
                    callee::REPLAY_FREE_SNAPSHOT.call(c, 0x0027_01e4)?;
                    c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
                    pc = 0x0027_022c;
                }
                0x0027_01ec => {
                    c.cycles += 1;
                    c.g[V0][0] = lw_2(c, A0, 0x14);
                    pc = 0x0027_01f0;
                }
                0x0027_01f0 => {
                    c.cycles += 1;
                    Addr(c.g[A2][0] as u32).off(0x3ac).sw(c, c.g[V0][0]);
                    pc = 0x0027_01f4;
                }
                0x0027_01f4 => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = lw_2(c, A2, 0x3b0);
                        pc = 0x0027_0218;
                    } else {
                        pc = 0x0027_01fc;
                    }
                }
                0x0027_01fc => {
                    c.cycles += 3;
                    c.g[V0][0] = lw_2(c, V0, 0x14);
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = lw_2(c, A2, 0x3b0);
                        pc = 0x0027_0218;
                    } else {
                        pc = 0x0027_0208;
                    }
                }
                0x0027_0208 => {
                    c.cycles += 3;
                    c.g[V0][0] = lw_2(c, V0, 0x14);
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw_2(c, A2, 0x3ac);
                        if back_edge_unwinds(c, 0x0027_01b8) {
                            return Ok(());
                        }
                    }
                    pc = 0x0027_01b8;
                }
                0x0027_0214 => {
                    c.cycles += 1;
                    c.g[V1][0] = lw_2(c, A2, 0x3b0);
                    pc = 0x0027_0218;
                }
                0x0027_0218 => {
                    c.cycles += 4;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
                    let again = c.g[V0][0] != 0;
                    Addr(c.g[A2][0] as u32).off(0x3ac).sw(c, c.g[V1][0]);
                    if again && back_edge_unwinds(c, 0x0027_01b0) {
                        return Ok(());
                    }
                    pc = if again { 0x0027_01b0 } else { 0x0027_0228 };
                }
                0x0027_0228 => {
                    c.cycles += 1;
                    c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
                    pc = 0x0027_022c;
                }
                0x0027_022c => {
                    c.cycles += 2;
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0027_01a0, 0x0027_0234);
                    return Ok(());
                }
            }
        }
    })
}

/// `Replay_FlushSaveOutput` (0x270fc0): append the pending replay snapshot
/// bytes to the file buffer (at most 0x4000 per call) and advance the source
/// and count fields.
pub fn replay_flush_save_output(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(p_12(this, 0x644)).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 3;
            c.g[A2][0] = Addr(p_12(this, 0x648)).lw(c);
            let empty = (c.g[A2][0] as i64) <= 0;
            c.g[V1][0] = 0x4000;
            if empty {
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr(p_12(this, 0x64c)).lw(c);
                c.g[V1][0] = sx(0x4000u32.wrapping_sub(r(c, V0)));
                c.g[A0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                c.g[S1][0] = c.g[V1][0];
                if c.g[A0][0] == 0 {
                    c.g[S1][0] = c.g[A2][0];
                }
                if (c.g[S1][0] as i64) <= 0 {
                    frame.restore(c, S0, 0x20);
                } else {
                    c.cycles += 15;
                    c.g[A0][0] = Addr(p_12(this, 0x650)).lw(c);
                    c.g[RA][0] = sx(0x0027_1014);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, callee::MEMCPY_2, 0x0027_1014)?;
                    c.g[V1][0] = Addr(p_12(this, 0x650)).lw(c);
                    c.g[A0][0] = Addr(p_12(this, 0x644)).lw(c);
                    c.g[A1][0] = Addr(p_12(this, 0x648)).lw(c);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, S1)));
                    c.g[V0][0] = Addr(p_12(this, 0x64c)).lw(c);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S1)));
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(r(c, S1)));
                    Addr(p_12(this, 0x650)).sw(c, c.g[V1][0]);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
                    Addr(p_12(this, 0x644)).sw(c, c.g[A0][0]);
                    Addr(p_12(this, 0x64c)).sw(c, c.g[V0][0]);
                    Addr(p_12(this, 0x648)).sw(c, c.g[A1][0]);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                }
            }
        } else {
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_SaveBuildHeader` (0x271228): write the replay header (magic, build
/// id, world/rider fields) into the save buffer and reset the write cursor.
pub fn replay_save_build_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x20);
        c.g[RA][0] = sx(0x0027_1244);
        c.g[S0][0] = Addr(p_12(this, 0x640)).lw(c);
        callee::REPLAY_COUNT_SAVE_SNAPSHOTS.call(c, 0x0027_1244)?;
        let buf = r(c, S0);
        Addr(p_12(buf, 4)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0xffff_ffff_abcd_0000;
        c.g[A3][0] = Addr(gp_2(c, -0x848)).lw(c);
        c.g[A2][0] = c.g[A2][0] | 2;
        c.g[V1][0] = Addr(p_12(this, 0x18)).lw(c);
        c.g[A1][0] = sx(buf.wrapping_add(0x20));
        Addr(p_12(buf, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(p_12(this, 0x610)).lw(c);
        Addr(p_12(buf, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(p_12(this, 0x3c8)).lw(c);
        Addr(p_12(buf, 0x10)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(p_12(this, 0x3cc)).lw(c);
        Addr(p_12(buf, 0x14)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(p_12(r(c, A3), 0x84)).lw(c);
        c.g[V0][0] = Addr(p_12(r(c, V1), 0xc)).lw(c);
        c.g[A0][0] = Addr(p_12(r(c, V0), 0x7c)).lw(c);
        Addr(buf).sw(c, c.g[A2][0]);
        Addr(p_12(buf, 0x18)).sw(c, c.g[A0][0]);
        c.g[V0][0] = Addr(p_12(r(c, A3), 0x84)).lw(c);
        c.g[RA][0] = sx(0x0027_1298);
        c.g[A0][0] = Addr(p_12(r(c, V0), 0xc)).lw(c);
        call_ptr(c, callee::REPLAY_GET_WORLD, 0x0027_1298)?;
        c.g[V0][0] = Addr(p_12(r(c, S1), 0x638)).lw(c);
        Addr(p_12(r(c, S1), 0x644)).sw(c, 0);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
        Addr(p_12(r(c, S1), 0x648)).sw(c, 0);
        Addr(p_12(r(c, S1), 0x638)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_InstanceGetLength` (0x274c70): total length of the instance's
/// channels (sum over the channel records), cached at `+0x30`.
pub fn in_seq_instance_get_length(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p_12(this, 0x30)).lw(c);
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 5;
            c.g[V1][0] = Addr(p_12(this, 0x10)).lw(c);
            c.g[S3][0] = 0;
            c.g[V0][0] = Addr(p_12(r(c, V1), 1)).lbu(c);
            let no_channels = c.g[V0][0] == 0;
            c.g[S0][0] = 0;
            if !no_channels {
                c.cycles += 1;
                c.g[S2][0] = 0;
                loop {
                    c.cycles += 11;
                    c.g[A0][0] = Addr(p_12(this, 0x18)).lw(c);
                    c.g[S3][0] = sx(r(c, S3).wrapping_add(1));
                    c.g[RA][0] = sx(0x0027_4cc0);
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S2)));
                    callee::IN_SEQ_CHANNEL_GET_LENGTH.call(c, 0x0027_4cc0)?;
                    c.g[S2][0] = sx(r(c, S2).wrapping_add(0x3c));
                    c.g[A1][0] = Addr(p_12(this, 0x10)).lw(c);
                    c.g[A0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[V1][0] = Addr(p_12(r(c, A1), 1)).lbu(c);
                    c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    if c.g[A0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_4cb0);
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p_12(this, 0x20)).lw(c);
            if c.g[V0][0] != 0 {
                Addr(p_12(this, 0x30)).sw(c, c.g[S0][0]);
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[S0][0];
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

// MenuSys_SetActive's direct callee is defined in `callee` above.

/// `int InSeq_ChannelFlushEvents(self)` (0x273d20): tell the channel's two
/// listeners to drop their events and clear the queued event count.
pub fn in_seq_channel_flush_events(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[T1][0] = lw_3(c, S0, 0x2c);
        let has = c.g[T1][0] != 0;
        c.g[S1][0] = lw_3(c, S0, 0x30);
        if has {
            c.cycles += 3;
            c.g[A3][0] = lw_3(c, S0, 0x34);
            let handler = c.g[A3][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if handler {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
                c.g[V1][0] = lw_3(c, S0, 0x1c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lh(c);
                c.g[T0][0] = lw_3(c, V0, 0x1c);
                c.g[A2][0] = Addr(c.g[V1][0] as u32).off(3).lbu(c);
                c.mtc1(0, 12);
                let target = c.g[T0][0] as u32;
                c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_3d6c)?;
            }
            c.cycles += 12;
            c.g[A3][0] = lw_3(c, S0, 0x2c);
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = lw_3(c, S0, 0x1c);
            c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
            c.mtc1(0, 12);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
            c.g[V1][0] = lw_3(c, V0, 0x24);
            c.mov_s(13, 12);
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A2][0] = Addr(c.g[T0][0] as u32).off(3).lbu(c);
            let target = c.g[V1][0] as u32;
            c.g[A3][0] = lw_3(c, S0, 0x34);
            call_ptr(c, target, 0x0027_3d9c)?;
        }
        c.cycles += 9;
        sw_2(c, S0, 0x38, 0);
        c.g[V0][0] = c.g[S1][0];
        sw_2(c, S0, 0x30, 0);
        sw_2(c, S0, 0x34, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InSeq: unlink and free a sequence instance.

/// `void InSeq_FreeSequence(mgr, handle)` (0x2729a8): stop the sequence,
/// release its instance and unlink it from the active and idle lists.
pub fn in_seq_free_sequence(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'outer: {
            'mid: {
                'inner: {
                    c.cycles += 16;
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save_ra(c, 0);
                    c.g[RA][0] = sx(0x0027_29c4);
                    c.g[S1][0] = c.g[A0][0];
                    callee::INSEQ_STOP.call(c, 0x0027_29c4)?;
                    c.g[V0][0] = 0x58;
                    c.g[V1][0] = lw(c, at_3(c.g[S1][0], 0x14));
                    c.mult(0, 16, 16, 2);
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[RA][0] = sx(0x0027_29dc);
                    c.g[A0][0] = sx(at_3(c.g[S0][0], 8));
                    callee::INSEQ_INSTANCE_RELEASE.call(c, 0x0027_29dc)?;
                    c.g[V1][0] = lw(c, at_3(c.g[S0][0], 0x54));
                    let t = c.g[V1][0] != 0;
                    c.g[V0][0] = lw(c, at_3(c.g[S0][0], 0x50));
                    if t {
                        break 'inner;
                    }
                    c.cycles += 5;
                    Addr(at_3(c.g[S1][0], 0x1c)).sw(c, c.g[V0][0]);
                    Addr(at_3(c.g[S0][0], 0x50)).sw(c, 0);
                    c.g[V0][0] = lw(c, at_3(c.g[S1][0], 0x1c));
                    if c.g[V0][0] != 0 {
                        Addr(at_3(c.g[V0][0], 0x54)).sw(c, 0);
                        break 'mid;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = lw(c, at_3(c.g[S1][0], 0x18));
                    break 'outer;
                }
                {
                    c.cycles += 4;
                    Addr(at_3(c.g[V1][0], 0x50)).sw(c, c.g[V0][0]);
                    c.g[V1][0] = lw(c, at_3(c.g[S0][0], 0x50));
                    if c.g[V1][0] == 0 {
                        Addr(at_3(c.g[S0][0], 0x50)).sw(c, 0);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, at_3(c.g[S0][0], 0x54));
                        Addr(at_3(c.g[V1][0], 0x54)).sw(c, c.g[V0][0]);
                        Addr(at_3(c.g[S0][0], 0x50)).sw(c, 0);
                    }
                }
                c.cycles += 1;
                Addr(at_3(c.g[S0][0], 0x54)).sw(c, 0);
                break 'mid;
            }
            c.cycles += 1;
            c.g[V0][0] = lw(c, at_3(c.g[S1][0], 0x18));
            break 'outer;
        }
        {
            c.cycles += 4;
            Addr(at_3(c.g[S0][0], 0x50)).sw(c, c.g[V0][0]);
            c.g[V0][0] = lw(c, at_3(c.g[S1][0], 0x18));
            if c.g[V0][0] != 0 {
                Addr(at_3(c.g[V0][0], 0x54)).sw(c, c.g[S0][0]);
            }
        }
        c.cycles += 6;
        Addr(at_3(c.g[S1][0], 0x18)).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `InSeq_EventTrackStart` (0x00273208).
//
// `int InSeq_EventTrackStart(EventTrack* t, void* seq, void* source, void* owner)`:
// start firing the sequence's keys.

pub fn in_seq_event_track_start(c: &mut Cpu) {
    guest(c, |c| {
        'b_2732b0: {
            'b_2732ac: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
                let t = c.g[V0][0] == 0u64;
                c.g[S1][0] = c.g[A3][0];
                if t {
                    break 'b_2732ac;
                } else {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
                    c.g[RA][0] = sx(0x0027324c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                    call_ptr(c, 0x00272308, 0x0027324c)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    if t {
                        break 'b_2732ac;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x6));
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            break 'b_2732b0;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                            'l_273278: loop {
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[V0][0] = c.g[V0][0] | 0x10;
                                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                                c.g[RA][0] = sx(0x0027328c);
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                call_ptr(c, 0x00273418, 0x0027328c)?;
                                c.g[RA][0] = sx(0x00273294);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00273468, 0x00273294)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                let t = c.g[V0][0] == 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
                                if t {
                                    break 'b_2732ac;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                        if crate::events::event_due(c) {
                                            crate::events::deliver_event_here(c, 0x00273278);
                                        }
                                        continue 'l_273278;
                                    } else {
                                        break 'b_2732ac;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            break 'b_2732b0;
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sequencing and script processes.

/// `int InSeq_SpanTrackStart(SpanTrack *self, u16 *key, void *owner,
/// u32 *flags)` (0x272df0): start playing a span track, allocating and
/// activating its first key. Returns whether the track has a first key.
pub fn inseq_span_track_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p_14(c.g[S0][0], 0x8)).sw(c, c.g[A1][0]);
        Addr(p_14(c.g[S0][0], 0xc)).sw(c, c.g[A2][0]);
        Addr(p_14(c.g[S0][0], 0x10)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lhu(c);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A3][0];
        let mut extra = true;
        if !empty {
            c.cycles += 14;
            c.g[V1][0] = Addr(p_14(c.g[A1][0], 0x2)).lhu(c);
            c.g[A1][0] = 0x1c;
            c.g[V0][0] = Addr(p_14(c.g[A2][0], 0x8)).lw(c);
            abi::mult(c, c.g[V1][0] as u32, c.g[A1][0] as u32);
            c.g[A0][0] = Addr(p_14(c.g[V0][0], 0x8)).lw(c);
            c.g[A1][0] = c.lo[0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
            callee::ISEQ_HEAP_ALLOC.call(c, 0x0027_2e3c)?;
            c.g[V1][0] = Addr(p_14(c.g[S0][0], 0x10)).lw(c);
            Addr(p_14(c.g[S0][0], 0x20)).sw(c, c.g[V0][0]);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            Addr(p_14(c.g[S0][0], 0x18)).sw(c, 0);
            let none = c.g[V0][0] == 0;
            Addr(p_14(c.g[S0][0], 0x14)).sw(c, c.g[V0][0]);
            if !none {
                c.cycles += 3;
                c.g[V0][0] = Addr(p_14(c.g[V1][0], 0x8)).lhu(c);
                if c.g[V0][0] != 0 {
                    extra = false;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    loop {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[V0][0] |= 0x4;
                        Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
                        c.g[A1][0] = Addr(p_14(c.g[S0][0], 0x14)).lw(c);
                        callee::ISEQ_SPAN_TRACK_ACTIVATE_KEY.call(c, 0x0027_2e7c)?;
                        c.g[A0][0] = c.g[S0][0];
                        callee::ISEQ_SPAN_TRACK_NEXT_KEY.call(c, 0x0027_2e84)?;
                        c.g[V0][0] = Addr(p_14(c.g[S0][0], 0x14)).lw(c);
                        if c.g[V0][0] == 0 {
                            extra = false;
                            break;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(p_14(c.g[V0][0], 0x4)).lhu(c);
                        if c.g[V0][0] != 0 {
                            break;
                        }
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_2e68);
                        }
                    }
                }
            }
        }
        if extra {
            c.cycles += 1;
        }
        c.g[V1][0] = Addr(p_14(c.g[S0][0], 0x10)).lw(c);
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lhu(c);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = (c.g[V0][0] > 0) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay hooks.

/// `void Replay_OnRiderEvent(Replay* self, int rider, int kind)` (0x2709d8):
/// ignore the event while the replay is busy; otherwise find the rider and
/// highlight the matching checkpoint bit, or empty the rider hook.
pub fn replay_on_rider_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[V0][0] = Addr(p_15(r32(c, S0), 0x61c)).lw(c);
        let busy = c.g[V0][0] != 0;
        let mut empty = false;
        let mut highlight = false;
        let mut ret = false;
        if !busy {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_15(r32(c, S0), 0x620)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(p_15(r32(c, S0), 0x610)).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p_15(r32(c, S0), 0x60c)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                        if c.g[V0][0] == 0 {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x0027_0730, 0x0027_0a38)?;
                            c.g[A3][0] = c.g[V0][0];
                            let negative = (c.g[A3][0] as i64) < 0;
                            c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
                            if negative {
                                ret = true;
                            } else {
                                c.cycles += 6;
                                c.g[A1][0] = sx(r32(c, S1) << 2);
                                c.g[V1][0] = Addr(p_15(r32(c, A0), 0x84)).lw(c);
                                c.g[V0][0] = Addr(p_15(r32(c, V1), 0xc)).lw(c);
                                c.g[V0][0] = sx(p_15(r32(c, V0), r32(c, A1)));
                                let cond = c.g[S2][0] == 0;
                                c.g[A0][0] = Addr(p_15(r32(c, V0), 0x28)).lw(c);
                                if cond {
                                    empty = true;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 1;
                                    if c.g[S2][0] == c.g[V0][0] {
                                        c.g[A2][0] = Addr(p_15(r32(c, A0), 0x790)).lw(c);
                                        highlight = true;
                                    } else {
                                        c.cycles += 2;
                                        ret = true;
                                    }
                                }
                            }
                        } else {
                            ret = true;
                        }
                    } else {
                        ret = true;
                    }
                } else {
                    ret = true;
                }
            } else {
                ret = true;
            }
        } else {
            ret = true;
        }
        if empty {
            c.cycles += 6;
            c.g[A1][0] = Addr(p_15(r32(c, A0), 0x790)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(p_15(r32(c, A1), 0xfc));
            call_ptr(c, 0x0027_05a8, 0x0027_0a80)?;
            ret = true;
        } else if highlight {
            c.cycles += 4;
            c.g[A1][0] = c.g[A3][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = sx(p_15(r32(c, A2), 0xfc));
            call_ptr(c, 0x0027_0b88, 0x0027_0a98)?;
            ret = true;
        }
        if !ret {
            // (every remaining arm already loaded s0 and returned)
        }
        if busy || highlight {
            c.cycles += 1;
        }
        c.lq(S0, frame.at(c, 0x30));
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_OnRiderCheckpoint(Replay* self, int rider, int bit)` (0x270ab0):
/// set the rider's checkpoint bit unless the replay is busy, the rider is
/// invalid or the bit is already set; clears the "pending" flag.
pub fn replay_on_rider_checkpoint(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p_15(r32(c, S0), 0x61c)).lw(c);
        c.g[S1][0] = c.g[A2][0];
        let mut plus_one = false;
        'checks: {
            if c.g[V0][0] != 0 {
                plus_one = true;
                break 'checks;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p_15(r32(c, S0), 0x620)).lw(c);
            if c.g[V0][0] != 0 {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p_15(r32(c, S0), 0x610)).lw(c);
            if c.g[V0][0] != 0 {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p_15(r32(c, S0), 0x60c)).lw(c);
            if c.g[V0][0] != 0 {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[V0][0] != 0 {
                break 'checks;
            }
            c.cycles += 5;
            call_ptr(c, 0x0027_0730, 0x0027_0b0c)?;
            c.g[A2][0] = c.g[V0][0];
            if (c.g[A2][0] as i64) < 0 {
                break 'checks;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) < 0 {
                break 'checks;
            }
            c.cycles += 4;
            c.g[V0][0] = Addr(p_15(r32(c, S0), 0x10)).lw(c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let in_range = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r32(c, A2) << 2);
            if !in_range {
                break 'checks;
            }
            c.cycles += 8;
            c.g[V1][0] = sx(p_15(r32(c, S0), 0x5fc));
            c.g[A3][0] = sx(p_15(r32(c, V1), r32(c, A0)));
            c.g[V0][0] = Addr(r32(c, A3)).lw(c);
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> (r32(c, S1) & 31)) as u32);
            c.g[V0][0] = c.g[V0][0] ^ 1;
            c.g[V0][0] = c.g[V0][0] & 1;
            let clear = c.g[V0][0] == 0;
            c.g[A0][0] = 1;
            if clear {
                break 'checks;
            }
            c.cycles += 9;
            c.g[A1][0] = Addr(p_15(r32(c, S0), 0x3b4)).lw(c);
            c.g[V1][0] = sx(r32(c, A0) << (r32(c, A2) & 31));
            c.g[V0][0] = Addr(p_15(r32(c, A1), 0x1f)).lbu(c);
            c.g[A0][0] = sx(r32(c, A0) << (r32(c, S1) & 31));
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            Addr(p_15(r32(c, A1), 0x1f)).sb(c, c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, A3)).lw(c);
            c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
            Addr(r32(c, A3)).sw(c, c.g[V1][0]);
            plus_one = true;
        }
        if plus_one {
            c.cycles += 1;
        }
        c.lq(S0, frame.at(c, 0x20));
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float InSeq_CurveTrackUpdate(InSeqCurveTrack* self, float t)`:
/// advance the curve track's playhead by `t` keys, evaluate the cubic through
/// the four surrounding keys and return the value; 0 when the track ended.
pub fn in_seq_curve_track_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_272778: {
            c.cycles += 8;
            c.g[SP][0] = sub(c.g[SP][0], 0x20);
            c.mov_s(1, 12);
            c.sq(16, p_16(c.g[SP][0] as u32, 0x10));
            c.sd(p_16(c.g[SP][0] as u32, 0), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[A0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::CURVE));
            let ended = c.g[A0][0] == 0;
            c.g[V0][0] = 0;
            if ended {
                break 'b_272778;
            }
            'b_272758: {
                c.cycles += 7;
                c.g[V0][0] = c.lhu(p_16(c.g[A0][0] as u32, 2));
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.c_le_s(0, 1);
                if !c.fcc() {
                    c.g[A0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::SEGMENT));
                    break 'b_272758;
                }
                'b_272714: {
                    c.cycles += 9;
                    c.g[V1][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::KEY));
                    sw_4(
                        c,
                        p_16(c.g[S0][0] as u32, in_seq_curve::SEGMENT),
                        c.g[A0][0],
                    );
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    sw_4(c, p_16(c.g[S0][0] as u32, in_seq_curve::CURSOR), 0);
                    sw_4(c, p_16(c.g[S0][0] as u32, in_seq_curve::KEY), c.g[V1][0]);
                    c.g[V0][0] = c.lhu(c.g[A0][0] as u32);
                    c.g[V0][0] = (c.g[V0][0] < 0x15) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::LENGTH));
                        break 'b_272714;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                    sw_4(c, p_16(c.g[S0][0] as u32, in_seq_curve::CURSOR), c.g[V0][0]);
                    c.g[V0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::LENGTH));
                    break 'b_272714;
                }
                'b_272748: {
                    c.cycles += 5;
                    c.g[A0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::KEY));
                    c.g[V1][0] = lw(c, c.g[V0][0] as u32);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                    if c.g[A0][0] != c.g[V1][0] {
                        c.g[V0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::SEGMENT));
                        break 'b_272748;
                    }
                    c.cycles += 8;
                    c.g[A0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::SEGMENT));
                    sw_4(c, p_16(c.g[S0][0] as u32, in_seq_curve::CURVE), 0);
                    c.mtc1(0, 12);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                    call_ptr(c, 0x0027_2570, 0x0027_273c)?;
                    c.swc1(0, p_16(c.g[S0][0] as u32, in_seq_curve::VALUE));
                    c.g[V0][0] = 0;
                    break 'b_272778;
                }
                c.cycles += 4;
                c.g[V1][0] = c.lhu(c.g[V0][0] as u32);
                c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
                sw_4(c, p_16(c.g[S0][0] as u32, in_seq_curve::CURVE), c.g[V0][0]);
                c.g[A0][0] = lw(c, p_16(c.g[S0][0] as u32, in_seq_curve::SEGMENT));
                break 'b_272758;
            }
            c.cycles += 8;
            c.g[V0][0] = c.lhu(p_16(c.g[A0][0] as u32, 2));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.mtc1(2, 12);
            c.cvt_s_w(12, 12);
            c.sub_s(12, 1, 12);
            call_ptr(c, 0x0027_2570, 0x0027_2770)?;
            c.swc1(0, p_16(c.g[S0][0] as u32, in_seq_curve::VALUE));
            c.g[V0][0] = 1;
            break 'b_272778;
        }
        c.cycles += 4;
        c.lq(16, p_16(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p_16(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// InSeq_HeapFree (0x00002723a8).

/// `InSeq_HeapFree`: return a block to an InSeq heap, coalescing with the neighbouring free blocks.
pub fn in_seq_heap_free(c: &mut Cpu) {
    guest(c, |c| {
        'b_272480: {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;

            if t {
                break 'b_272480;
            } else {
                'b_2723bc: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x8));
                    if t {
                        c.cycles += 3;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[T0][0] = 0u64;
                        c.g[V0][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
                        'l_2723d0: loop {
                            'b_272478: {
                                'b_272470: {
                                    'b_27241c: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                            break 'b_27241c;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[A3][0] as u32));
                                            let t = c.g[V1][0] != c.g[A2][0];

                                            if t {
                                                c.cycles += 2;
                                                if c.g[T0][0] == 0u64 {
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                                    break 'b_272470;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                                    if t {
                                                        break 'b_272478;
                                                    } else {
                                                        break 'b_27241c;
                                                    }
                                                }
                                            } else {
                                                c.cycles += 6;
                                                c.g[V1][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                                c.sw(
                                                    (c.g[A1][0] as u32).wrapping_add(0x4),
                                                    c.g[V1][0],
                                                );
                                                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                                c.g[V0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                let t = c.g[T0][0] == 0u64;
                                                c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                                                if t {
                                                    break 'b_2723bc;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[RA][0] as u32;
                                                    c.sw(
                                                        (c.g[T0][0] as u32).wrapping_add(0x4),
                                                        c.g[A1][0],
                                                    );
                                                    c.pc = t;
                                                    return Ok(());
                                                }
                                            }
                                        }
                                    }
                                    'b_27245c: {
                                        'b_272458: {
                                            c.cycles += 4;
                                            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A3][0] as u32));
                                            if c.g[V1][0] != c.g[A1][0] {
                                                c.g[T0][0] = c.g[A2][0];
                                                break 'b_272458;
                                            } else {
                                                'b_272450: {
                                                    c.cycles += 7;
                                                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                                    c.g[A1][0] = c.g[A2][0];
                                                    c.g[V0][0] = sx((c.g[A3][0] as u32)
                                                        .wrapping_add(c.g[V0][0] as u32));
                                                    c.sw(c.g[A2][0] as u32, c.g[V0][0]);
                                                    c.g[A2][0] =
                                                        c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                                    let t = c.g[T0][0] != 0u64;
                                                    c.sw(
                                                        (c.g[A1][0] as u32).wrapping_add(0x4),
                                                        0u64,
                                                    );
                                                    if t {
                                                        break 'b_272450;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x8),
                                                            c.g[A2][0],
                                                        );
                                                        if t {
                                                            break 'b_27245c;
                                                        } else {
                                                            break 'b_272450;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.sw(
                                                    (c.g[T0][0] as u32).wrapping_add(0x4),
                                                    c.g[A2][0],
                                                );
                                                if t {
                                                    break 'b_27245c;
                                                } else {
                                                    break 'b_272458;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                        break 'b_27245c;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[A2][0] != 0u64;
                                    c.g[V0][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002723d0);
                                        }
                                        continue 'l_2723d0;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[T0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                                            break 'b_272478;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                            break 'b_272470;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                if t {
                                    break 'b_2723bc;
                                } else {
                                    break 'b_272478;
                                }
                            }
                            c.cycles += 2;
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                            break 'b_272480;
                        }
                    } else {
                        break 'b_2723bc;
                    }
                }
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.pc = t;
                return Ok(());
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    });
}

/// `void func_002703F0(void* self, int reset)` (0x2703f0): step the replay
/// player one frame: on reset re-find the first snapshot and skip a finished
/// one, re-seek both streams, then either restore the current snapshot (reset)
/// or advance the position, and at the end of the replay loop back to the
/// start or stop at the end.
pub fn replay_step_frame(c: &mut Cpu) {
    let frame = Frame::open(c, 0x40);
    'b_27041c: {
        c.cycles += 8;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        let t = c.g[S1][0] == 0u64;
        sd(c, r32(c, SP), c.g[RA][0]);
        if t {
            break 'b_27041c;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x00270418);
            c.g[A1][0] = lw_5(c, at(c, S0, 0x484));
            if callee::C0026E448.call(c, 0x00270418).is_err() {
                return;
            }
            sw_5(c, at(c, S0, 0x488), c.g[V0][0]);
            break 'b_27041c;
        }
    }
    'b_27043c: {
        c.cycles += 3;
        c.g[V1][0] = lw_5(c, at(c, S0, 0x488));
        let t = c.g[V1][0] == 0u64;

        if t {
            break 'b_27043c;
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw_5(c, at(c, S0, 0x3d0));
            let t = c.g[V1][0] != c.g[V0][0];

            if t {
                break 'b_27043c;
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw_5(c, at(c, V1, 0x18));
                sw_5(c, at(c, S0, 0x488), c.g[V0][0]);
                break 'b_27043c;
            }
        }
    }
    'b_270520: {
        'b_27048c: {
            c.cycles += 2;
            let t = c.g[S1][0] == 0u64;
            c.g[S1][0] = sx(at(c, S0, 0x48c));
            if t {
                break 'b_27048c;
            } else {
                c.cycles += 5;
                c.g[V1][0] = lw_5(c, at(c, S0, 0x488));
                c.g[S2][0] = 0x1;
                c.g[V0][0] = lw_5(c, at(c, V1, 0x30));
                sw_5(c, at(c, S0, 0x484), c.g[V0][0]);
                'l_270458: loop {
                    'b_270470: {
                        c.cycles += 3;
                        c.g[A0][0] = lw_5(c, r32(c, S1));
                        if c.g[A0][0] == 0u64 {
                            c.g[S2][0] = sx(r32(c, S2).wrapping_sub(0x1));
                            break 'b_270470;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x0027046c);
                            c.g[A1][0] = lw_5(c, at(c, S0, 0x484));
                            if callee::C0026D4D8.call(c, 0x0027046c).is_err() {
                                return;
                            }
                            c.g[S2][0] = sx(r32(c, S2).wrapping_sub(0x1));
                            break 'b_270470;
                        }
                    }
                    c.cycles += 2;
                    let t = (c.g[S2][0] as i64) >= 0;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00270458);
                        }
                        continue 'l_270458;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x00270484);
                        c.g[A1][0] = lw_5(c, at(c, A0, 0x488));
                        if callee::C0026DB88.call(c, 0x00270484).is_err() {
                            return;
                        }
                        let t = true;
                        frame.restore(c, S0, 0x30);
                        if t {
                            break 'b_270520;
                        } else {
                            break 'b_27048c;
                        }
                    }
                }
            }
        }
        'b_270518: {
            c.cycles += 5;
            c.g[V1][0] = lw_5(c, at(c, S0, 0x484));
            c.g[A0][0] = lw_5(c, at(c, S0, 0x3cc));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = sx(at(c, V1, 0x1));
            if t {
                break 'b_270518;
            } else {
                'b_270504: {
                    c.cycles += 3;
                    c.g[V0][0] = lw_5(c, at(c, S0, 0x61c));
                    if c.g[V0][0] == 0u64 {
                        sw_5(c, at(c, S0, 0x484), c.g[A0][0]);
                        break 'b_270504;
                    } else {
                        c.cycles += 22;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x002704b8);
                        c.g[A1][0] = 0u64;
                        if callee::C002702F8.call(c, 0x002704b8).is_err() {
                            return;
                        }
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x002704c4);
                        c.g[A1][0] = 0x1;
                        if callee::C002703F0.call(c, 0x002704c4).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw_5(c, r32(c, GP).wrapping_sub(0x848));
                        c.g[V1][0] = lw_5(c, at(c, V0, 0x84));
                        c.g[S1][0] = lw_5(c, at(c, V1, 0x84));
                        c.g[V0][0] = lw_5(c, at(c, S1, 0x4));
                        c.g[S0][0] = lw_5(c, at(c, V0, 0xa8));
                        c.g[RA][0] = sx(0x002704e0);
                        c.g[A0][0] = c.g[S0][0];
                        if call_ptr(c, 0x001620d0, 0x002704e0).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw_5(c, at(c, S0, 0x14));
                        c.g[A0][0] = lh(c, at(c, V0, 0x20));
                        c.g[V1][0] = lw_5(c, at(c, V0, 0x24));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002704f4);
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x002704f4).is_err() {
                            return;
                        }
                        c.g[RA][0] = sx(0x002704fc);
                        c.g[A0][0] = c.g[S1][0];
                        if call_ptr(c, 0x0022e840, 0x002704fc).is_err() {
                            return;
                        }
                        let t = true;
                        frame.restore(c, S0, 0x30);
                        if t {
                            break 'b_270520;
                        } else {
                            break 'b_270504;
                        }
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = 0x3;
                c.g[RA][0] = sx(0x00270510);
                c.g[A0][0] = c.g[S0][0];
                if callee::C0026F4A0.call(c, 0x00270510).is_err() {
                    return;
                }
                let t = true;
                frame.restore(c, S0, 0x30);
                if t {
                    break 'b_270520;
                } else {
                    break 'b_270518;
                }
            }
        }
        c.cycles += 2;
        sw_5(c, at(c, S0, 0x484), c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        break 'b_270520;
    }
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x40));
    c.pc = t;
    return;
}

/// `a + off` on a guest pointer.
#[inline(always)]
pub(crate) fn pa(a: u64, off: u32) -> u32 {
    (a as u32).wrapping_add(off)
}

/// The speeds [`script_speed_for_rate`] wrote: instance → (its own speed,
/// the speed written for the tick).
pub(crate) struct ScriptSpeed {
    speed: u32,
    tick_speed: u32,
}

thread_local! {
    static SCRIPT_SPEEDS: std::cell::RefCell<std::collections::HashMap<u32, ScriptSpeed>> = Default::default();
}

/// Script time follows the tick rate: the per-tick advance of a sequence
/// (`InSeq_InstanceTick`: `pos += speed`, the channels advanced to `pos`) is
/// authored in 60 Hz frames, so above 60 the tick runs with `speed · 60 /
/// rate` in `+0x24` (the seek above does the same with its partial steps).
/// Returns the instance's own speed and the one written, for
/// [`script_speed_restore`]; `None` at 60, where nothing changes.
///
/// The instance's own speed is also kept here: if the tick unwinds (the
/// generated code resumes after it, without the restore), the next tick
/// finds the written speed still in place and takes the kept one.
pub(crate) fn script_speed_for_rate(c: &mut Cpu, this: u32) -> Option<(u32, u32)> {
    let r = crate::tick::rate(c);
    if r == 60 {
        return None;
    }
    let at = this.wrapping_add(0x24);
    let now = c.bus.read32(at);
    let speed = SCRIPT_SPEEDS.with(|m| match m.borrow().get(&this) {
        Some(k) if k.tick_speed == now => k.speed,
        _ => now,
    });
    let tick_speed = (f32::from_bits(speed) * 60.0 / r as f32).to_bits();
    c.bus.write32(at, tick_speed);
    SCRIPT_SPEEDS.with(|m| {
        let mut m = m.borrow_mut();
        if m.len() > 1024 {
            m.clear();
        }
        m.insert(this, ScriptSpeed { speed, tick_speed });
    });
    Some((speed, tick_speed))
}

/// After the tick: the instance's own speed back, unless the tick set a new
/// one (a script command).
pub(crate) fn script_speed_restore(c: &mut Cpu, this: u32, speed: u32, tick_speed: u32) {
    let at = this.wrapping_add(0x24);
    if c.bus.read32(at) == tick_speed {
        c.bus.write32(at, speed);
    }
    SCRIPT_SPEEDS.with(|m| m.borrow_mut().remove(&this));
}

/// `void InSeq_InstanceUpdate(InSeq_Instance* self)` (0x274a30): apply a
/// pending seek (`+0x34`) or a step count (`+0x38`), else advance exactly one
/// script frame; keeps the short step budget (`+0x2c`) and pauses when it runs
/// out.
///
/// `+0x1c` loop, `+0x20` playing, `+0x24` speed, `+0x28` pos (frames),
/// `+0x2c` frame budget, `+0x34` seek target, `+0x38` pending steps.
pub fn in_seq_instance_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x60);
        frame.save_fpr(c, 21, 0x58);
        frame.save_fpr(c, 20, 0x50);
        c.g[V1][0] = c.lw(pa(this, 0x34));
        // `need` (`s2`): the final one-frame tick is due unless a seek or a
        // step run already handled the frame. It stays in `s2` because the
        // callees save that register to the stack.
        c.g[S2][0] = 1;
        if (c.g[V1][0] as i64) >= 0 {
            // 0x274a64: a pending seek.
            c.cycles += 9;
            c.g[V0][0] = u64::MAX;
            c.g[S1][0] = c.g[V1][0];
            c.sw(pa(this, 0x34), c.g[V0][0]);
            callee::GET_LENGTH.call(c, 0x0027_4a74)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[S3][0] = c.lw(pa(this, 0x2c)); // the saved frame budget
            c.g[V0][0] = c.lw(pa(this, 0x1c));
            let no_loop = c.g[V0][0] == 0;
            c.sw(pa(this, 0x2c), 0);
            if !no_loop {
                // 0x274a88: target %= length.
                c.cycles += 3;
                c.div(0, S1, V1);
                c.cycles += 2;
                c.g[V0][0] = c.hi[0];
                c.g[S1][0] = c.g[V0][0];
            }
            // 0x274a9c: f20 = (float)target.
            c.cycles += 5;
            c.g[V0][0] = c.lw(pa(this, 0x20));
            c.mtc1(17, 20);
            c.cvt_s_w(20, 20);
            if c.g[V0][0] != 1 {
                c.lwc1(0, pa(this, 0x28));
            } else {
                // 0x274ab0: one tick before the seek comparison.
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::TICK.call(c, 0x0027_4ab8)?;
                c.lwc1(0, pa(this, 0x28));
            }
            // 0x274abc: already at the target?
            c.cycles += 4;
            c.c_eq_s(20, 0);
            if c.fcc() {
                // 0x274b6c: keep the budget while playing.
                c.g[V0][0] = c.lw(pa(this, 0x20));
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(pa(this, 0x2c), c.g[S3][0]);
                }
            } else {
                // 0x274acc: target < pos: rewind by restarting.
                c.cycles += 4;
                c.c_lt_s(20, 0);
                if c.fcc() {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    callee::STOP.call(c, 0x0027_4ae4)?;
                    c.g[A0][0] = c.g[S0][0];
                    callee::RESTART.call(c, 0x0027_4aec)?;
                    c.g[A0][0] = c.g[S0][0];
                    callee::TICK.call(c, 0x0027_4af4)?;
                }
                c.lwc1(0, pa(this, 0x28));
                // 0x274af8: advance in 1.0 steps until the target is reached.
                c.cycles += 12;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 21); // f21 = 1.0
                c.cvt_w_s(1, 0); // f1 = (int)pos
                c.mfc1(2, 1);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.mtc1(2, 1);
                c.cvt_s_w(1, 1); // f1 = (float)((int)pos + 1)
                c.sub_s(1, 1, 0); // f1 = fraction to the next frame
                c.c_lt_s(1, 21); // fraction < 1.0
                let partial = c.fcc();
                c.lwc1(22, pa(this, 0x24)); // f22 = speed
                if partial {
                    // 0x274b28: finish the current frame at the partial step.
                    c.cycles += 3;
                    c.swc1(1, pa(this, 0x24));
                    c.g[A0][0] = c.g[S0][0];
                    callee::TICK.call(c, 0x0027_4b34)?;
                }
                // 0x274b34: full steps until pos reaches the target.
                c.cycles += 3;
                c.g[V0][0] = c.lw(pa(this, 0x20));
                c.swc1(21, pa(this, 0x24));
                loop {
                    // 0x274b4c
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.swc1(22, pa(this, 0x24));
                        break;
                    }
                    // 0x274b54: pos < target?
                    c.cycles += 4;
                    c.lwc1(0, pa(this, 0x28));
                    c.c_lt_s(0, 20);
                    if !c.fcc() {
                        c.cycles += 1;
                        c.swc1(22, pa(this, 0x24));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_4b40);
                    }
                    // 0x274b40
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    callee::TICK.call(c, 0x0027_4b48)?;
                    c.g[V0][0] = c.lw(pa(this, 0x20));
                }
                // 0x274b68: keep the budget while playing.
                c.cycles += 1;
                c.g[V0][0] = c.lw(pa(this, 0x20));
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(pa(this, 0x2c), c.g[S3][0]);
                }
            }
            // 0x274b74
            c.cycles += 1;
            c.g[S2][0] = 0;
        }
        // 0x274b78: pending steps.
        c.cycles += 3;
        let mut steps = c.lw(pa(this, 0x38));
        c.g[V0][0] = steps;
        c.g[S1][0] = steps;
        if steps != 0 {
            // 0x274b84
            c.cycles += 5;
            let saved_f2c = c.lw(pa(this, 0x2c));
            c.g[S2][0] = saved_f2c;
            c.g[V0][0] = c.lw(pa(this, 0x20));
            c.sw(pa(this, 0x38), 0);
            let playing = c.g[V0][0];
            c.sw(pa(this, 0x2c), 0);
            if playing != 0 {
                // 0x274b98
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                loop {
                    // 0x274ba0
                    c.cycles += 5;
                    steps = sx((steps as u32).wrapping_sub(1));
                    c.g[S1][0] = steps;
                    callee::TICK.call(c, 0x0027_4ba8)?;
                    c.g[V0][0] = c.lw(pa(this, 0x20));
                    if c.g[V0][0] == 0 {
                        // The stop path jumps straight to 0x274bc4.
                        c.g[V0][0] = c.lw(pa(this, 0x2c));
                        break;
                    }
                    // 0x274bb4
                    c.cycles += 2;
                    let more = c.g[S1][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !more {
                        // 0x274bbc: the steps ran out while still playing.
                        c.cycles += 1;
                        c.sw(pa(this, 0x2c), c.g[S2][0]);
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(pa(this, 0x2c));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_4ba0);
                    }
                }
            } else {
                // 0x274bc0
                c.cycles += 1;
                c.g[V0][0] = c.lw(pa(this, 0x2c));
            }
            // 0x274bc4: a positive budget keeps playing.
            c.cycles += 2;
            let positive = (c.g[V0][0] as i64) > 0;
            c.g[S2][0] = 0;
            if !positive {
                // 0x274bcc: the budget ran out: pause.
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::PAUSE.call(c, 0x0027_4bd4)?;
                c.g[S2][0] = 0;
            }
        }
        // 0x274bd8: the final one-frame tick (skipped after a seek or steps).
        c.cycles += 2;
        if c.g[S2][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            let scaled = script_speed_for_rate(c, this as u32);
            callee::TICK.call(c, 0x0027_4be8)?;
            if let Some((speed, tick_speed)) = scaled {
                script_speed_restore(c, this as u32, speed, tick_speed);
            }
        }
        c.lq(S0, frame.at(c, 0x40));
        // 0x274bec
        c.cycles += 9;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x60);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InSeq_UpdateAll (0x2728f8)

/// Input sequencer: the first sequence node (+0x1c). Node: the instance
/// (+8) and the next node (+0x50).
pub(crate) const INSEQ_FIRST: u32 = 0x1c;

pub(crate) const INSEQ_NODE_INSTANCE: u32 = 0x8;

pub(crate) const INSEQ_NODE_NEXT: u32 = 0x50;

/// `InSeq_InstanceUpdate` (0x274a30).
pub(crate) const INSEQ_INSTANCE_UPDATE: Callee = Callee {
    addr: 0x0027_4a30,
    f: ee::s07::InSeq_InstanceUpdate,
};

/// `void InSeq_UpdateAll(InSeq *seq)` (0x2728f8): updates the instance of
/// every node in the sequencer's list. Frame 0x20 (ra at 0, s0 at 0x10).
pub fn in_seq_update_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = Addr(r32(c, A0)).off(INSEQ_FIRST).lw(c);
        if c.g[S0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(INSEQ_NODE_INSTANCE));
                INSEQ_INSTANCE_UPDATE.call(c, 0x0027_2918)?;
                c.g[S0][0] = Addr(r32(c, S0)).off(INSEQ_NODE_NEXT).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_2910);
                }
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The world pointer (`gp-0x848`).
pub(crate) const REPLAY_FIND_WORLD_GP: u32 = 0x848;

/// World -> rider list (+0x84), list -> rider table (+0xc), table -> rider
/// count (+0x7c) and rider pointers (+0x40, one word each).
pub(crate) const REPLAY_FIND_WORLD_RIDERS: u32 = 0x84;

pub(crate) const REPLAY_FIND_LIST_TABLE: u32 = 0xc;

pub(crate) const REPLAY_FIND_TABLE_COUNT: u32 = 0x7c;

pub(crate) const REPLAY_FIND_TABLE_RIDERS: u32 = 0x40;

/// Rider -> object (+0x18); the replay id object is embedded at +0x6c0,
/// its id getter is vtable slot 0x38.
pub(crate) const REPLAY_FIND_RIDER_OBJ: u32 = 0x18;

pub(crate) const REPLAY_FIND_OBJ_ID: u32 = 0x6c0;

pub(crate) const REPLAY_FIND_ID_SLOT: u32 = 0x38;

/// `world` as `lw gp-0x848` leaves it.
pub(crate) fn replay_find_world(c: &mut Cpu) -> u64 {
    Addr((c.g[GP][0] as u32).wrapping_sub(REPLAY_FIND_WORLD_GP)).lw(c)
}

/// The world's rider count (`world->riders->table->count`), leaving the
/// table pointer in v1 as the original does.
pub(crate) fn replay_find_rider_count(c: &mut Cpu, world: u64) -> u64 {
    let list = Addr((world as u32).wrapping_add(REPLAY_FIND_WORLD_RIDERS)).lw(c);
    c.g[A0][0] = list;
    let table = Addr((list as u32).wrapping_add(REPLAY_FIND_LIST_TABLE)).lw(c);
    table
}

/// `int Replay_FindRiderIndex(int id)` (0x270730): the index of the world
/// rider whose replay id object (rider object +0x6c0, virtual getter at slot
/// 0x38) returns `id`, or -1. The loop checks events at its head 0x270768.
pub fn replay_find_rider_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        let world = replay_find_world(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[A1][0] = world;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let table = replay_find_rider_count(c, world);
        c.g[V0][0] = table;
        let count = Addr((table as u32).wrapping_add(REPLAY_FIND_TABLE_COUNT)).lw(c);
        c.g[V1][0] = count;
        c.g[S0][0] = 0;
        let mut found = false;
        if (count as i64) > 0 {
            c.cycles += 2;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(REPLAY_FIND_WORLD_RIDERS)).lw(c);
            loop {
                c.cycles += 13;
                let index = c.g[S0][0] as u32;
                c.g[A0][0] = sx(index << 2);
                let table = Addr((c.g[V1][0] as u32).wrapping_add(REPLAY_FIND_LIST_TABLE)).lw(c);
                let slot = sx((table as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = slot;
                let rider = Addr((slot as u32).wrapping_add(REPLAY_FIND_TABLE_RIDERS)).lw(c);
                c.g[V1][0] = rider;
                let obj = Addr((rider as u32).wrapping_add(REPLAY_FIND_RIDER_OBJ)).lw(c);
                let vt = Addr((obj as u32).wrapping_add(REPLAY_FIND_OBJ_ID)).lw(c);
                c.g[A1][0] = vt;
                let id_obj = sx((obj as u32).wrapping_add(REPLAY_FIND_OBJ_ID));
                let delta = Addr((vt as u32).wrapping_add(REPLAY_FIND_ID_SLOT)).lh(c);
                c.g[V1][0] = delta;
                let getter = Addr((vt as u32).wrapping_add(REPLAY_FIND_ID_SLOT + 4)).lw(c);
                c.g[V0][0] = getter;
                c.g[A0][0] = sx((id_obj as u32).wrapping_add(delta as u32));
                call_ptr(c, getter as u32, 0x0027_0794)?;
                let differs = c.g[V0][0] != c.g[S1][0];
                c.g[V0][0] = replay_find_world(c);
                if !differs {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    found = true;
                    break;
                }
                c.cycles += 8;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                let world = c.g[V0][0];
                let table = replay_find_rider_count(c, world);
                c.g[A1][0] = world;
                c.g[V1][0] = table;
                let count = Addr((table as u32).wrapping_add(REPLAY_FIND_TABLE_COUNT)).lw(c);
                let more = (c.g[S0][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(REPLAY_FIND_WORLD_RIDERS)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_0768);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay_OnRiderLaunch (0x2707e0)

/// `Replay` busy flags: any non-zero one blocks new highlights (`+0x61c`,
/// `+0x620`, `+0x610`, `+0x60c`, and the playing state at `+0`).
pub(crate) const OC300_REPLAY_BUSY_FLAGS: [u32; 4] = [0x61c, 0x620, 0x610, 0x60c];

pub(crate) const OC300_REPLAY_PLAYING: u32 = 0x0;

/// `Replay_FindRiderIndex` (0x270730).
pub(crate) const OC300_REPLAY_FIND_RIDER_INDEX: Callee = Callee {
    addr: 0x0027_0730,
    f: ee::s07::Replay_FindRiderIndex,
};

/// `Replay_BeginHighlight` (0x270b88).
pub(crate) const OC300_REPLAY_BEGIN_HIGHLIGHT: Callee = Callee {
    addr: 0x0027_0b88,
    f: ee::s07::Replay_BeginHighlight,
};

/// `Replay_EmptyRiderHook` (0x2705a8).
pub(crate) const OC300_REPLAY_EMPTY_RIDER_HOOK: Callee = Callee {
    addr: 0x0027_05a8,
    f: ee::s07::Replay_EmptyRiderHook,
};

/// `void Replay_OnRiderLaunch(Replay* self, int riderId, void* data)`
/// (0x2707e0): unless the replay is busy (any of the flags at
/// `+0x61c/+0x620/+0x610/+0x60c` or playing at `+0`), find the rider's index
/// (`Replay_FindRiderIndex(riderId)`) and, when found, begin a highlight
/// for it (`Replay_BeginHighlight(self, index, data)`) and run the (empty)
/// rider hook.
pub fn replay_on_rider_launch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).off(OC300_REPLAY_BUSY_FLAGS[0]).lw(c);
        c.g[S1][0] = c.g[A2][0];
        // The first flag set: the tail charges one more cycle.
        let mut tail = c.g[V0][0] != 0;
        if !tail {
            let mut busy = false;
            for off in OC300_REPLAY_BUSY_FLAGS[1..]
                .iter()
                .copied()
                .chain([OC300_REPLAY_PLAYING])
            {
                c.cycles += 3;
                c.g[V0][0] = Addr(this).off(off).lw(c);
                if c.g[V0][0] != 0 {
                    busy = true;
                    break;
                }
            }
            if !busy {
                tail = true;
                c.cycles += 4;
                OC300_REPLAY_FIND_RIDER_INDEX.call(c, 0x0027_083c)?;
                let found = (c.g[V0][0] as i64) >= 0;
                c.g[A0][0] = c.g[S0][0];
                if found {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S1][0];
                    OC300_REPLAY_BEGIN_HIGHLIGHT.call(c, 0x0027_0850)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    OC300_REPLAY_EMPTY_RIDER_HOOK.call(c, 0x0027_085c)?;
                }
            }
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay_OnRiderLand (0x270870)

/// `Replay` flags that block highlight bookkeeping on landing: `+0x61c`
/// (playing back), `+0x620` (instant replay), then the replay state at `+0`.
pub(crate) const OC305_REPLAY_LAND_BUSY_FLAGS: [u32; 3] = [0x61c, 0x620, 0x0];

/// `Replay_FindRiderIndex` (0x270730).
pub(crate) const OC305_REPLAY_FIND_RIDER_INDEX: Callee = Callee {
    addr: 0x0027_0730,
    f: ee::s07::Replay_FindRiderIndex,
};

/// `Replay_EndHighlight` (0x270ce8).
pub(crate) const OC305_REPLAY_END_HIGHLIGHT: Callee = Callee {
    addr: 0x0027_0ce8,
    f: ee::s07::Replay_EndHighlight,
};

/// `Replay_CheckInstantReplay` (0x2705b0).
pub(crate) const OC305_REPLAY_CHECK_INSTANT_REPLAY: Callee = Callee {
    addr: 0x0027_05b0,
    f: ee::s07::Replay_CheckInstantReplay,
};

/// `void Replay_OnRiderLand(Replay* self, int riderId, void* data)`
/// (0x270870): when the replay is idle (none of `+0x61c`, `+0x620`, `+0`
/// set), find the rider's index and, when found, close its highlight
/// (`Replay_EndHighlight(self, index, data, 1)`) and check whether the
/// landing earns an instant replay (`Replay_CheckInstantReplay(self, data,
/// 1)`).
pub fn replay_on_rider_land(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).off(OC305_REPLAY_LAND_BUSY_FLAGS[0]).lw(c);
        c.g[S1][0] = c.g[A2][0];
        // The first flag set (or the full path): one more cycle at the tail.
        let mut tail = c.g[V0][0] != 0;
        if !tail {
            let mut busy = false;
            for off in OC305_REPLAY_LAND_BUSY_FLAGS[1..].iter().copied() {
                c.cycles += 3;
                c.g[V0][0] = Addr(this).off(off).lw(c);
                if c.g[V0][0] != 0 {
                    busy = true;
                    break;
                }
            }
            if !busy {
                tail = true;
                c.cycles += 4;
                OC305_REPLAY_FIND_RIDER_INDEX.call(c, 0x0027_08b4)?;
                let found = (c.g[V0][0] as i64) >= 0;
                c.g[A1][0] = c.g[V0][0];
                if found {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = 1;
                    OC305_REPLAY_END_HIGHLIGHT.call(c, 0x0027_08cc)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = 1;
                    OC305_REPLAY_CHECK_INSTANT_REPLAY.call(c, 0x0027_08dc)?;
                }
            }
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay_PruneSnapshots (0x270538)

/// `Replay` live snapshot list head (`+0x3b0`).
pub(crate) const OC305_REPLAY_LIVE_SNAPSHOTS: u32 = 0x3b0;

/// Snapshot node: next (`+0x14`), and the keep markers: a halfword at
/// `+0x1c` and bytes at `+0x1f` and `+0x1e`.
pub(crate) const OC305_SNAPSHOT_NEXT: u32 = 0x14;

pub(crate) const OC305_SNAPSHOT_KEEP_HALF: u32 = 0x1c;

pub(crate) const OC305_SNAPSHOT_KEEP_B: u32 = 0x1f;

pub(crate) const OC305_SNAPSHOT_KEEP_A: u32 = 0x1e;

/// `Replay_FreeSnapshot` (0x270238).
pub(crate) const OC305_REPLAY_FREE_SNAPSHOT: Callee = Callee {
    addr: 0x0027_0238,
    f: ee::s07::Replay_FreeSnapshot,
};

/// `void Replay_PruneSnapshots(Replay* self)` (0x270538): walk the live
/// snapshot list and free (`Replay_FreeSnapshot(self, node)`) every snapshot
/// nothing keeps (its halfword at `+0x1c` and bytes at `+0x1f`/`+0x1e` all
/// zero). The next pointer is read before the free unlinks the node.
pub fn replay_prune_snapshots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, S1)).off(OC305_REPLAY_LIVE_SNAPSHOTS).lw(c);
        frame.restore(c, S0, 0x20);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, A1)).off(OC305_SNAPSHOT_KEEP_HALF).lhu(c);
            loop {
                // The node and its keep halfword live in a1/v0 (an event at
                // the loop head sees them there).
                let node = Addr(r32(c, A1));
                c.cycles += 2;
                c.g[S0][0] = node.off(OC305_SNAPSHOT_NEXT).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = node.off(OC305_SNAPSHOT_KEEP_B).lbu(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = node.off(OC305_SNAPSHOT_KEEP_A).lbu(c);
                        if c.g[V0][0] == 0 {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S1][0];
                            OC305_REPLAY_FREE_SNAPSHOT.call(c, 0x0027_0588)?;
                            c.cycles += 1;
                        }
                    }
                }
                c.g[A1][0] = c.g[S0][0];
                c.cycles += 2;
                if c.g[A1][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                    break;
                }
                c.g[V0][0] = Addr(r32(c, A1)).off(OC305_SNAPSHOT_KEEP_HALF).lhu(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_0560);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replay: the free-snapshot list (head +0x3bc, tail +0x3c0, count +0x3c4).
pub(crate) const REPLAY_FREE_LIST: u32 = 0x3bc;

pub(crate) const REPLAY_FREE_COUNT: u32 = 0x3c4;

/// Snapshot fields reset on allocation.
pub(crate) const SNAPSHOT_DESC: u32 = 0x1c;

pub(crate) const SNAPSHOT_FRAME_A: u32 = 0x20;

pub(crate) const SNAPSHOT_FRAME_B: u32 = 0x24;

pub(crate) const SNAPSHOT_USED: u32 = 0x30;

pub(crate) const SNAPSHOT_INDEX: u32 = 0x34;

pub(crate) const REPLAY_RECYCLE_OLDEST_SNAPSHOT: Callee = Callee {
    addr: 0x0027_01a0,
    f: ee::s07::Replay_RecycleOldestSnapshot,
};

pub(crate) const SNAPSHOT_LIST_POP_FRONT: Callee = Callee {
    addr: 0x0026_e670,
    f: ee::s07::SnapshotList_PopFront,
};

/// `Snapshot* Replay_AllocSnapshot(Replay* self)` (0x270130): take a
/// snapshot from the free list (recycling the oldest one first when the list
/// is empty) and reset it: the 4-byte descriptor at +0x1c cleared, the frame
/// links (+0x20, +0x24) and index (+0x34) set to -1, +0x30 cleared.
pub fn replay_alloc_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let replay = r32(c, A0);
        c.g[V0][0] = c.lw(replay.wrapping_add(REPLAY_FREE_COUNT));
        let have_free = (c.g[V0][0] as i64) > 0;
        c.g[S0][0] = sx(replay.wrapping_add(REPLAY_FREE_LIST));
        if !have_free {
            c.cycles += 2;
            REPLAY_RECYCLE_OLDEST_SNAPSHOT.call(c, 0x0027_0150)?;
        }
        c.cycles += 19;
        c.g[A0][0] = c.g[S0][0];
        SNAPSHOT_LIST_POP_FRONT.call(c, 0x0027_0158)?;
        c.g[V1][0] = c.g[V0][0];
        let snapshot = r32(c, V1);
        // The zeroed 4-byte descriptor is built on the stack and copied
        // with an unaligned word move (lwl/lwr, swl/swr), as compiled.
        let tmp = sp(c);
        c.sh(tmp, 0);
        c.sb(tmp.wrapping_add(2), 0);
        c.g[A0][0] = u64::MAX;
        c.sb(tmp.wrapping_add(3), 0);
        c.lwl(A1, tmp.wrapping_add(3));
        c.lwr(A1, tmp);
        c.swl(A1, snapshot.wrapping_add(SNAPSHOT_DESC + 3));
        c.swr(A1, snapshot.wrapping_add(SNAPSHOT_DESC));
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        c.sw(snapshot.wrapping_add(SNAPSHOT_INDEX), u64::MAX);
        c.sw(snapshot.wrapping_add(SNAPSHOT_FRAME_A), u64::MAX);
        c.sw(snapshot.wrapping_add(SNAPSHOT_FRAME_B), u64::MAX);
        c.sw(snapshot.wrapping_add(SNAPSHOT_USED), 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay_EndHighlight (0x270ce8)

/// Size of one highlight slot in the `Replay` and the offset of the slot
/// array.
pub(crate) const REPLAY_END_HIGHLIGHT_SLOT_SIZE: u32 = 0xb4;

pub(crate) const REPLAY_END_HIGHLIGHT_SLOTS: u32 = 0x494;

/// Offset of a slot's snapshot node pointer from the slot array start
/// (`slot + 0xb0`, seen from the replay as `+0x544`).
pub(crate) const REPLAY_END_HIGHLIGHT_SLOT_NODE: u32 = 0x544;

/// Snapshot node flag byte: one "highlighted in slot n" bit per slot.
pub(crate) const REPLAY_END_HIGHLIGHT_NODE_FLAGS: u32 = 0x1e;

pub(crate) const REPLAY_END_HIGHLIGHT_SCORE: Callee = Callee {
    addr: 0x0026_e838,
    f: ee::s07::Replay_HighlightSlotScore,
};

pub(crate) const REPLAY_END_HIGHLIGHT_CLEAR: Callee = Callee {
    addr: 0x0026_e800,
    f: ee::s07::Replay_HighlightSlotClear,
};

pub(crate) const REPLAY_END_HIGHLIGHT_COUNT: Callee = Callee {
    addr: 0x0027_0bd8,
    f: ee::s07::Replay_CountHighlights,
};

pub(crate) const REPLAY_END_HIGHLIGHT_GET_MAX: Callee = Callee {
    addr: 0x0027_0c40,
    f: ee::s07::Replay_GetMaxHighlights,
};

pub(crate) const REPLAY_END_HIGHLIGHT_FIND_WEAKEST: Callee = Callee {
    addr: 0x0027_0c58,
    f: ee::s07::Replay_FindWeakestHighlight,
};

pub(crate) const REPLAY_END_HIGHLIGHT_DETACH: Callee = Callee {
    addr: 0x0026_e7f8,
    f: ee::s07::Replay_HighlightSlotDetach,
};

/// `void Replay_EndHighlight(Replay* replay, int slot, Player* player, int
/// mode)` (0x270ce8): close the highlight being recorded in `slot`, if the
/// slot has a snapshot node. Score it (`Replay_HighlightSlotScore`); a
/// worthless one is dropped (`Replay_HighlightSlotClear`). A kept one that
/// brings the slot's highlight count to the maximum evicts the weakest
/// highlight (`Replay_FindWeakestHighlight`): that node loses the slot's bit
/// in its flag byte. Finally the slot lets go of its node
/// (`Replay_HighlightSlotDetach`).
pub fn replay_end_highlight(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A0][0]; // replay
        c.g[S3][0] = c.g[A1][0]; // slot
        c.g[A0][0] = REPLAY_END_HIGHLIGHT_SLOT_SIZE as u64;
        c.mult(0, V0, S3, A0);
        frame.save(c, S2, 0x30);
        frame.save(c, S0, 0x50);
        c.g[A1][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(REPLAY_END_HIGHLIGHT_SLOT_NODE));
        c.g[S2][0] = c.g[A3][0]; // mode
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            c.mult(0, S4, S3, A0); // slot offset
            c.g[A2][0] = c.g[S2][0];
            c.g[V0][0] = sx(r32(c, S4).wrapping_add(REPLAY_END_HIGHLIGHT_SLOTS));
            c.g[S0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0))); // the slot
            c.g[A0][0] = c.g[S0][0];
            REPLAY_END_HIGHLIGHT_SCORE.call(c, 0x0027_0d40)?;
            c.g[A1][0] = c.g[S3][0];
            if c.g[V0][0] == 0 {
                // Not worth keeping.
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                REPLAY_END_HIGHLIGHT_CLEAR.call(c, 0x0027_0dac)?;
                c.cycles += 1;
                c.g[A0][0] = REPLAY_END_HIGHLIGHT_SLOT_SIZE as u64;
            } else {
                c.cycles += 10;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                REPLAY_END_HIGHLIGHT_COUNT.call(c, 0x0027_0d54)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                REPLAY_END_HIGHLIGHT_GET_MAX.call(c, 0x0027_0d64)?;
                c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[S0][0] != 0 {
                    c.g[A0][0] = REPLAY_END_HIGHLIGHT_SLOT_SIZE as u64;
                } else {
                    // Full: evict the weakest highlight from this slot.
                    c.cycles += 13;
                    c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, S1)));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(REPLAY_END_HIGHLIGHT_SLOT_NODE));
                    c.g[A3][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S3][0];
                    REPLAY_END_HIGHLIGHT_FIND_WEAKEST.call(c, 0x0027_0d88)?;
                    let node = r32(c, V0).wrapping_add(REPLAY_END_HIGHLIGHT_NODE_FLAGS);
                    c.g[A0][0] = c.lbu(node);
                    c.g[V1][0] = sx(1u32 << (r32(c, S3) & 31));
                    c.g[V1][0] = !c.g[V1][0];
                    c.g[A0][0] &= c.g[V1][0];
                    c.sb(node, c.g[A0][0]);
                    c.cycles += 1;
                    c.g[A0][0] = REPLAY_END_HIGHLIGHT_SLOT_SIZE as u64;
                }
            }
            c.cycles += 4;
            c.mult(0, A0, S3, A0);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(REPLAY_END_HIGHLIGHT_SLOTS));
            c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
            REPLAY_END_HIGHLIGHT_DETACH.call(c, 0x0027_0dc0)?;
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
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::ai::ai::tests::leaf;

    #[test]
    fn world_state_stream_reset_matches() {
        check(
            ssx3_ee::s07::f_0026d628,
            world_state_stream_reset,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c13_000b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..16 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn in_seq_heap_init_matches() {
        check(
            ssx3_ee::s07::f_002722e0,
            in_seq_heap_init,
            100,
            &|c, seed| {
                let mut r = Rng(0x722e0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = if seed % 3 == 0 {
                    0xffff_fff8
                } else {
                    r.next_u32()
                } as u64;
            },
        );
    }

    #[test]
    fn in_seq_channel_get_track_matches() {
        check(
            ssx3_ee::s07::f_002742c8,
            in_seq_channel_get_track,
            100,
            &|c, seed| {
                let mut r = Rng(0x742c8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let index = seed % 4;
                c.g[A1][0] = index as u64;
                let table = DATA + 0x1000;
                c.bus.write32(DATA + 0x24, table);
                c.bus.write32(table + index * 4, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_channel_get_length_matches() {
        check(
            ssx3_ee::s07::f_00274348,
            in_seq_channel_get_length,
            100,
            &|c, seed| {
                let mut r = Rng(0x74348 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let track = DATA + 0x100;
                c.bus.write32(DATA + 0x1c, track);
                c.bus.write32(track + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_channel_get_id_matches() {
        check(
            ssx3_ee::s07::f_002743c8,
            in_seq_channel_get_id,
            100,
            &|c, seed| {
                let mut r = Rng(0x743c8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let track = DATA + 0x100;
                c.bus.write32(DATA + 0x1c, track);
                c.bus.write32(track, r.next_u32());
            },
        );
    }

    #[test]
    fn world_state_stream_reset_matches_2() {
        check(
            ssx3_ee::s07::f_0026d628,
            world_state_stream_reset_2,
            64,
            &|c, seed| {
                let mut r = Rng(0x26d6 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn replay_empty_rider_hook_matches() {
        leaf(
            ssx3_ee::s07::f_002705a8__generated,
            replay_empty_rider_hook,
            14,
        );
    }

    use crate::difftest::check_lohi;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::ai::ai::tests::uf;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4;
    pub(crate) use crate::game::bx::bxstringctor::tests::game;
    pub(crate) use crate::game::bx::bxstringctor::tests::SCRATCH;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::dtor_setup as dtor_setup_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;
    pub(crate) use crate::game::fe::debugfe::tests::FEW;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_6 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::fe::festateloadscreen::tests::r32_seed;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::w as w_3;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_6;
    pub(crate) use crate::game::main::ssxapp::tests::strict_check;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w as w_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::soundsys::tests::base_9;
    pub(crate) use crate::game::sound::soundsys::tests::w as w_8;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn replay_request_instant_replay_matches() {
        check(
            ssx3_ee::s07::f_00270658,
            replay_request_instant_replay,
            50,
            &|c, seed| {
                let mut r = Rng(0x0c13_0014 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(
                    DATA + 0x620,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(DATA + 0x624, r.next_u32());
            },
        );
    }

    #[test]
    fn replay_save_begin_snapshots_matches() {
        check(
            ssx3_ee::s07::f_00271348,
            replay_save_begin_snapshots,
            200,
            &|c, seed| {
                let mut r = Rng(0x71348 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let a = DATA + 0x100;
                let b = if seed % 2 == 0 { a } else { DATA + 0x140 };
                c.bus.write32(DATA + 0x3b0, a);
                c.bus.write32(DATA + 0x3d0, b);
                c.bus.write32(a + 0x14, r.next_u32());
                c.bus.write32(DATA + 0x638, r.next_u32());
            },
        );
    }

    #[test]
    fn replay_save_begin_caches_matches() {
        check(
            ssx3_ee::s07::f_002714e0,
            replay_save_begin_caches,
            100,
            &|c, seed| {
                let mut r = Rng(0x714e0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x638, r.next_u32());
            },
        );
    }

    #[test]
    fn math_eval_cubic_matches() {
        check(
            ssx3_ee::s07::f_00272570,
            math_eval_cubic,
            300,
            &|c, seed| {
                let mut r = Rng(0x72570 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + i * 4, r.edge_f32());
                }
                // Cover t == 0, a denormal (compares as zero) and ordinary values.
                c.f[12] = match seed % 4 {
                    0 => 0,
                    1 => r.next_u32() | 0x3f80_0000,
                    2 => r.next_u32() & 0x007f_ffff,
                    _ => r.edge_f32(),
                };
            },
        );
    }

    #[test]
    fn in_seq_span_track_activate_key_matches() {
        check_lohi(
            ssx3_ee::s07::f_002730c0,
            in_seq_span_track_activate_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x730c0 ^ seed as u64);
                scramble(c, &mut r);
                let track = DATA + 0x200;
                let key = DATA;
                let base = DATA + 0x1000;
                c.g[A0][0] = track as u64;
                c.g[A1][0] = key as u64;
                c.bus.write32(track + 0x20, base);
                c.bus.write32(track + 0x24, r.next_u32());
                c.bus.write32(track + 0x28, r.next_u32());
                c.bus.write32(key + 0x8, r.next_u32());
                c.bus.write32(key + 0xc, r.next_u32());
                let index = seed % 4;
                c.bus.write16(key + 2, index as u16);
                c.bus.write16(key + 4, (r.next_u32() & 0xffff) as u16);
                c.bus.write16(key + 6, (r.next_u32() & 0xffff) as u16);
                let kind = if seed % 2 == 0 {
                    seed % 0x11
                } else {
                    0x11 + seed % 8
                };
                c.bus.write16(key, kind as u16);
            },
        );
    }

    #[test]
    fn in_seq_span_track_next_key_matches() {
        check(
            ssx3_ee::s07::f_00273140,
            in_seq_span_track_next_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x73140 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let keys = DATA + 0x100;
                let next = DATA + 0x200;
                c.bus.write32(DATA + 0x10, keys);
                c.bus.write32(DATA + 0x14, next);
                let count = (seed % 7) as u32 + 1;
                c.bus.write16(keys, count as u16);
                c.bus.write16(next, (r.next_u32() & 0x3f) as u16);
                let cur = if seed % 2 == 0 { count - 1 } else { count + 3 };
                c.bus.write32(DATA + 0x18, cur);
            },
        );
    }

    #[test]
    fn in_seq_event_track_fire_key_matches() {
        check(
            ssx3_ee::s07::f_00273418,
            in_seq_event_track_fire_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x73418 ^ seed as u64);
                scramble(c, &mut r);
                let track = DATA;
                let key = DATA + 0x100;
                c.g[A0][0] = track as u64;
                c.g[A1][0] = key as u64;
                c.bus.write32(track + 0x1c, DATA + 0x1000);
                c.bus.write32(track + 0x20, (seed % 4) as u32);
                c.bus.write32(key + 0x4, r.next_u32());
                let kind = if seed % 3 == 0 {
                    seed % 9
                } else {
                    9 + seed % 8
                };
                c.bus.write16(key, kind as u16);
            },
        );
    }

    #[test]
    fn in_seq_event_track_next_key_matches() {
        check(
            ssx3_ee::s07::f_00273468,
            in_seq_event_track_next_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x73468 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let keys = DATA + 0x100;
                let next = DATA + 0x200;
                c.bus.write32(DATA + 0x10, keys);
                c.bus.write32(DATA + 0x14, next);
                let count = (seed % 7) as u32 + 1;
                c.bus.write16(keys, count as u16);
                c.bus.write16(next, (r.next_u32() & 0x3f) as u16);
                let cur = if seed % 2 == 0 { count - 1 } else { count + 3 };
                c.bus.write32(DATA + 0x18, cur);
            },
        );
    }

    #[test]
    fn in_seq_ramp_track_activate_key_matches() {
        check_lohi(
            ssx3_ee::s07::f_00273860,
            in_seq_ramp_track_activate_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x73860 ^ seed as u64);
                scramble(c, &mut r);
                let track = DATA + 0x200;
                let key = DATA;
                let base = DATA + 0x1000;
                c.g[A0][0] = track as u64;
                c.g[A1][0] = key as u64;
                c.bus.write32(track + 0x1c, base);
                c.bus.write32(track + 0x20, r.next_u32());
                c.bus.write32(track + 0x24, r.next_u32());
                c.bus.write32(key + 0x8, r.next_u32());
                c.bus.write32(key + 0xc, r.next_u32());
                c.bus.write32(key + 0x10, r.edge_f32());
                c.bus.write32(key + 0x14, r.next_u32());
                let index = seed % 4;
                c.bus.write16(key + 2, index as u16);
                c.bus.write16(key + 4, (r.next_u32() & 0xffff) as u16);
                c.bus.write16(key + 6, (r.next_u32() & 0xffff) as u16);
                let kind = if seed % 2 == 0 {
                    seed % 0x19
                } else {
                    0x19 + seed % 8
                };
                c.bus.write16(key, kind as u16);
            },
        );
    }

    #[test]
    fn in_seq_ramp_track_next_key_matches() {
        check(
            ssx3_ee::s07::f_002738f8,
            in_seq_ramp_track_next_key,
            200,
            &|c, seed| {
                let mut r = Rng(0x738f8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let keys = DATA + 0x100;
                let next = DATA + 0x200;
                c.bus.write32(DATA + 0x10, keys);
                c.bus.write32(DATA + 0x14, next);
                let count = (seed % 7) as u32 + 1;
                c.bus.write16(keys, count as u16);
                c.bus.write16(next, (r.next_u32() & 0x3f) as u16);
                let cur = if seed % 2 == 0 { count - 1 } else { count + 3 };
                c.bus.write32(DATA + 0x18, cur);
            },
        );
    }

    #[test]
    fn in_seq_channel_ctor_matches() {
        check(
            ssx3_ee::s07::f_00273938,
            in_seq_channel_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x73938 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..12 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn in_seq_channel_setup_matches() {
        check(
            ssx3_ee::s07::f_002739b8,
            in_seq_channel_setup,
            200,
            &|c, seed| {
                let mut r = Rng(0x739b8 ^ seed as u64);
                scramble(c, &mut r);
                let data = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = data as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                let extra = if seed % 2 == 0 { 0 } else { 0x10 };
                c.bus.write32(data + 8, extra);
                c.bus.write32(data, r.next_u32());
                let track = data + extra;
                let kind = if seed % 4 == 0 {
                    seed % 9
                } else {
                    9 + seed % 8
                };
                c.bus.write16(track, kind as u16);
            },
        );
    }

    #[test]
    fn in_seq_instance_ctor_matches() {
        check(
            ssx3_ee::s07::f_00274518,
            in_seq_instance_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x74518 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..0x12 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn in_seq_instance_start_matches() {
        check(
            ssx3_ee::s07::f_00274830,
            in_seq_instance_start,
            300,
            &|c, seed| {
                let mut r = Rng(0x74830 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                // Path 0: not active. Path 1: active, no pending start. Path 2:
                // active with a pending start request.
                let path = seed % 3;
                c.bus
                    .write32(DATA + 0x20, if path == 0 { 0 } else { r.next_u32() | 1 });
                c.bus
                    .write32(DATA + 0x3c, if path == 2 { r.next_u32() | 1 } else { 0 });
            },
        );
    }

    #[test]
    fn in_seq_instance_pause_matches() {
        check(
            ssx3_ee::s07::f_002749e8,
            in_seq_instance_pause,
            200,
            &|c, seed| {
                let mut r = Rng(0x749e8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(
                    DATA + 0x20,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(DATA + 0x2c, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_instance_resume_matches() {
        check(
            ssx3_ee::s07::f_00274a08,
            in_seq_instance_resume,
            300,
            &|c, seed| {
                let mut r = Rng(0x74a08 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let path = seed % 3;
                c.bus
                    .write32(DATA + 0x20, if path == 0 { 0 } else { r.next_u32() | 1 });
                let count = match path {
                    0 => r.next_u32(),
                    1 => 1 + r.next_u32() % 100,
                    _ => 0,
                };
                c.bus.write32(DATA + 0x2c, count);
            },
        );
    }

    #[test]
    fn in_seq_instance_get_channel_matches() {
        check_lohi(
            ssx3_ee::s07::f_00274c20,
            in_seq_instance_get_channel,
            200,
            &|c, seed| {
                let mut r = Rng(0x274c_20 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 12) as u64;
                c.bus.write32(DATA + 0x18, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_instance_get_listener_matches() {
        check(
            ssx3_ee::s07::f_00274d70,
            in_seq_instance_get_listener,
            200,
            &|c, seed| {
                let mut r = Rng(0x274d_70 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 0x44, if seed % 3 == 0 { 0 } else { r.next_u32() });
                let link = DATA + 0x200;
                c.bus.write32(DATA + 0x8, link);
                c.bus.write32(link, r.next_u32());
            },
        );
    }

    #[test]
    fn replay_get_max_highlights_matches() {
        check(
            ee::s07::f_00270c40__generated,
            replay_get_max_highlights,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0027_0c40, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let v = r.next_u32();
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    mixed(&mut r, seed, v)
                };
                c.bus.write32(DATA + 0xc, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_get_header_extra_matches() {
        check(
            ee::s07::f_00274c38__generated,
            in_seq_get_header_extra,
            200,
            &|c, seed| {
                machine(c, seed, 0x27_4c38, 0);
                c.g[A0][0] = DATA as u64;
                let version = (seed % 20) as u16;
                c.bus.write16(DATA + 2, version);
            },
        );
    }

    // ------------------------------------------------------------ destructors

    /// `flags = seed % 2` so both paths run; on the deleting path `this` is a
    /// plain data pointer (the allocator tolerates it, as in its own test).
    pub(crate) fn dtor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = game(c, seed, salt, 6);
        c.g[A0][0] = if seed % 8 == 0 { 0 } else { ptr(&mut r) as u64 };
        c.g[A1][0] = (seed % 2) as u64;
    }

    #[test]
    fn in_seq_dtor_matches() {
        check(
            ee::s07::f_002728d0__generated,
            in_seq_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x2728d0),
        );
    }

    // ----------------------------------------------- sequence / audio helpers

    #[test]
    fn in_seq_stop_matches() {
        check(
            ee::s07::f_00272b58__generated,
            in_seq_stop,
            100,
            &|c, seed| {
                let _r = game(c, seed, 0x272b58, 2);
                let array = SCRATCH + 0x1000;
                c.g[A0][0] = SCRATCH as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write32(SCRATCH + 0x14, array);
                for i in 0..4u32 {
                    // Half the instances exist, half have no instance at all.
                    c.bus
                        .write32(array + 0x58 * i + 0xc, ((seed + i) % 2) as u32);
                    // `+0x20 == 0` stops the instance immediately.
                    c.bus.write32(array + 0x58 * i + 0x28, 0);
                }
            },
        );
    }

    #[test]
    fn replay_stream_dtor_matches() {
        check(
            ee::s07::f_002721d0__generated,
            replay_stream_dtor,
            60,
            &|c, seed| dtor_setup_2(c, seed, 0x2721d0),
        );
    }

    #[test]
    fn world_state_stream_dtor_matches() {
        check(
            ee::s07::f_00272258__generated,
            world_state_stream_dtor,
            60,
            &|c, seed| dtor_setup_2(c, seed, 0x272258),
        );
    }

    // ------------------------------------------------------- script / sequence

    /// An instrument-sequence channel at `SCRATCH` whose `+0x14` table has one
    /// live record (so `InSeq_GetInstance` returns `record + 8`).
    pub(crate) fn in_seq_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let _r = machine(c, seed, salt, 2);
        let table = SCRATCH + 0x1000;
        let record = table + (seed % 4) as u32 * 0x58;
        c.g[A0][0] = SCRATCH as u64;
        c.g[A1][0] = (seed % 4) as u64;
        c.g[A2][0] = if seed % 2 == 0 {
            0
        } else {
            (SCRATCH + 0x800) as u64
        };
        c.bus.write32(SCRATCH + 0x14, table);
        c.bus.write32(record + 0xc, 1);
    }

    #[test]
    fn in_seq_play_matches() {
        check(
            ee::s07::f_00272ad0__generated,
            in_seq_play,
            100,
            &|c, seed| in_seq_setup(c, seed, 0x272ad0),
        );
    }

    #[test]
    fn replay_highlight_slot_init_matches() {
        check(
            ssx3_ee::s07::f_00272288__generated,
            replay_highlight_slot_init,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x272288, 0);
                let slot = DATA + 0x100;
                fill(c, &mut r, slot, 0x40);
                c.g[A0][0] = sx(slot);
            },
        );
    }

    // ------------------------------------------------------------------
    // InSeq track constructors.

    #[test]
    fn in_seq_curve_track_ctor_matches() {
        check(
            ssx3_ee::s07::f_002725b8__generated,
            in_seq_curve_track_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2725b8, 0);
                let track = DATA + 0x100;
                fill(c, &mut r, track, 0x20);
                c.g[A0][0] = sx(track);
            },
        );
    }

    #[test]
    fn in_seq_span_track_ctor_matches() {
        check(
            ssx3_ee::s07::f_00272d68__generated,
            in_seq_span_track_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x272d68, 0);
                let track = DATA + 0x100;
                fill(c, &mut r, track, 0x20);
                c.g[A0][0] = sx(track);
            },
        );
    }

    #[test]
    fn in_seq_event_track_ctor_matches() {
        check(
            ssx3_ee::s07::f_00273180__generated,
            in_seq_event_track_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x273180, 0);
                let track = DATA + 0x100;
                fill(c, &mut r, track, 0x20);
                c.g[A0][0] = sx(track);
            },
        );
    }

    #[test]
    fn in_seq_ramp_track_ctor_matches() {
        check(
            ssx3_ee::s07::f_002734a8__generated,
            in_seq_ramp_track_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2734a8, 0);
                let track = DATA + 0x100;
                fill(c, &mut r, track, 0x20);
                c.g[A0][0] = sx(track);
            },
        );
    }

    #[test]
    fn replay_open_menu_matches() {
        check(
            ee::s07::f_002706b8__generated,
            replay_open_menu,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2706_b8, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x700);
                c.g[A0][0] = sx(self_);
                c.bus
                    .write32(self_ + 0x61c, if seed % 3 == 0 { 0 } else { 1 });
                let state = if seed % 2 == 0 { 9 } else { seed % 9 };
                c.bus.write32(self_, state);
            },
        );
    }

    #[test]
    fn replay_cancel_highlight_matches() {
        check(
            ee::s07::f_00270de0__generated,
            replay_cancel_highlight,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x270d_e0, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x800);
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx((seed % 4) as u32);
                c.bus.write32(self_, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn replay_save_write_header_matches() {
        check(
            ee::s07::f_002712c0__generated,
            replay_save_write_header,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2712_c0, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x700);
                fill(c, &mut r, DATA + 0x600, 0x10);
                c.g[A0][0] = sx(self_);
                c.bus.write32(self_ + 0x638, r.next_u32());
                c.bus
                    .write32(self_ + 0x640, if seed % 2 == 0 { 0 } else { DATA + 0x600 });
            },
        );
    }

    #[test]
    fn replay_end_instant_replay_matches() {
        check(
            ssx3_ee::s07::f_00270670__generated,
            replay_end_instant_replay,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x270670, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                // Only the idle path: an active playback runs the whole replay
                // end-of-playback chain, which needs a live replay object.
                c.bus.write32(this_ + 0x620, 0);
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn replay_free_save_buffers_matches() {
        check(
            ssx3_ee::s07::f_00270f78__generated,
            replay_free_save_buffers,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x270f78, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x63c, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                c.bus
                    .write32(this_ + 0x640, if seed % 2 == 0 { 0 } else { DATA + 0x2400 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn in_seq_channel_dtor_matches() {
        check(
            ssx3_ee::s07::f_00273970__generated,
            in_seq_channel_dtor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x273970, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus.write32(this_ + 0x14, DATA + 0x2000);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn in_seq_channel_select_block_matches() {
        check(
            ssx3_ee::s07::f_00273a60__generated,
            in_seq_channel_select_block,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x273a60, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let offsets = DATA + 0x2000;
                let blocks = DATA + 0x3000;
                c.bus.write32(this_ + 0x04, blocks);
                c.bus.write32(this_ + 0x14, offsets);
                // The selected block's first event record has a small/large count.
                let idx: u32 = seed % 4;
                c.bus.write32(offsets + 4 * idx, 0x40 * idx);
                c.bus
                    .write16(blocks + 0x40 * idx, if seed % 3 == 0 { 0 } else { 12 });
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = idx as u64;
            },
        );
    }

    #[test]
    fn replay_save_emit_matches() {
        check(
            ee::s07::f_00271058__generated,
            replay_save_emit,
            SEEDS_3,
            &|c, seed| {
                let mut r = base(c, seed, 0x2710_58);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let data = DATA + 0x400;
                for i in 0..0x40u32 {
                    c.bus.write8(data + i, r.next_u32() as u8);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(data);
                c.g[A2][0] = [0u64, 1, 8, 0x40][(seed % 4) as usize];
                c.bus.write32(this + 0x680, r.next_u32());
                c.bus.write32(this + 0x684, r.next_u32());
                c.bus.write32(this + 0x678, r.next_u32());
            },
        );
    }

    #[test]
    fn replay_count_save_snapshots_matches() {
        check(
            ee::s07::f_002712f8__generated,
            replay_count_save_snapshots,
            SEEDS_3,
            &|c, seed| {
                let mut r = base(c, seed, 0x2712_f8);
                let this = DATA + 0x100;
                clear(c, this, 0x700);
                // A chain of 0..4 snapshots; `skip` is one of them.
                let n = seed % 5;
                let nodes: Vec<u32> = (0..n).map(|i| DATA + 0x200 + 0x20 * i).collect();
                for (i, &node) in nodes.iter().enumerate() {
                    let next = nodes.get(i + 1).copied().unwrap_or(0);
                    c.bus.write32(node + 0x14, next);
                    c.bus.write32(node + 4, r.next_u32());
                }
                c.bus
                    .write32(this + 0x3b0, nodes.first().copied().unwrap_or(0));
                c.bus.write32(
                    this + 0x3d0,
                    nodes
                        .get((seed as usize) % (n as usize + 1))
                        .copied()
                        .unwrap_or(0),
                );
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn replay_save_write_footer_matches() {
        check(
            ee::s07::f_00271708__generated,
            replay_save_write_footer,
            SEEDS_3,
            &|c, seed| {
                let mut r = base(c, seed, 0x2717_08);
                let this = DATA + 0x100;
                clear(c, this, 0x700);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x638, r.next_u32());
                c.bus
                    .write32(this + 0x640, if seed % 3 == 0 { 0 } else { DATA + 0x900 });
                clear(c, DATA + 0x900, 0x40);
            },
        );
    }

    #[test]
    fn replay_clear_all_highlights_matches() {
        check(
            ee::s07::f_00270e18__generated,
            replay_clear_all_highlights,
            60,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x270e);
                let s = DATA;
                c.bus.write32(s, 1 + r.next_u32() % 3);
                for i in 0..2u32 {
                    c.bus.write32(s + 0x494 + 0xb4 * i, r.next_u32());
                    c.bus.write32(s + 0x494 + 0xb4 * i + 4, r.next_u32());
                }
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn inseq_span_track_dtor_matches() {
        check(
            ee::s07::f_00272da0__generated,
            inseq_span_track_dtor,
            60,
            &|c, seed| {
                base_2(c, seed, 0x272d);
                let flags = if seed % 3 == 0 { 1u32 } else { 0 };
                // The delete path frees `self`: use a small (pooled) address.
                let obj = if flags == 1 { 0x100 } else { DATA };
                c.bus.write32(gp_at(0x398), 0);
                // No live track data: the stop routine takes its early exit.
                c.bus.write32(obj + 0xc, 0);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(flags);
            },
        );
    }

    #[test]
    fn inseq_event_track_dtor_matches() {
        check(
            ee::s07::f_002731b8__generated,
            inseq_event_track_dtor,
            60,
            &|c, seed| {
                base_2(c, seed, 0x2731);
                let flags = if seed % 3 == 0 { 1u32 } else { 0 };
                let obj = if flags == 1 { 0x100 } else { DATA };
                c.bus.write32(gp_at(0x398), 0);
                c.bus.write32(obj + 0xc, 0);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(flags);
            },
        );
    }

    #[test]
    fn inseq_ramp_track_dtor_matches() {
        check(
            ee::s07::f_002734e0__generated,
            inseq_ramp_track_dtor,
            60,
            &|c, seed| {
                base_2(c, seed, 0x2734);
                let flags = if seed % 3 == 0 { 1u32 } else { 0 };
                let obj = if flags == 1 { 0x100 } else { DATA };
                c.bus.write32(gp_at(0x398), 0);
                c.bus.write32(obj + 0xc, 0);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(flags);
            },
        );
    }

    #[test]
    fn player_free_replay_cache_matches() {
        check(
            ssx3_ee::s07::f_002700d8__generated,
            player_free_replay_cache,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x2700d8, 1);
                heap_init(c);
                let player = DATA;
                let cache = alloc(c, 0x40);
                c.bus.write32(cache + 0xc, 0);
                let slot1 = if seed % 2 == 0 { cache } else { DATA + 0x300 };
                c.bus.write32(player + 0x48c, slot1);
                c.bus
                    .write32(player + 0x490, if seed % 3 == 0 { cache } else { 0 });
                c.g[A0][0] = player as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => cache as u64,
                    _ => (DATA + 0x300) as u64,
                };
            },
        );
    }

    #[test]
    fn replay_alloc_save_buffers_matches() {
        check(
            ssx3_ee::s07::f_00270f20__generated,
            replay_alloc_save_buffers,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x270f20, 2);
                heap_init(c);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.bus.write32(DATA + 0x400, seed);
            },
        );
    }

    /// Build `n` linked event nodes for the span/ramp track queries.
    pub(crate) fn build_nodes(
        c: &mut Cpu,
        r: &mut Rng,
        addr: u32,
        n: u32,
        time_off: u32,
        next_off: u32,
    ) {
        for i in 0..n {
            let node = addr + i * 0x40;
            c.bus.write32(node + time_off, r.next_u32());
            c.bus
                .write32(node + next_off, if i + 1 < n { node + 0x40 } else { 0 });
        }
    }

    #[test]
    fn in_seq_span_track_get_next_event_time_matches() {
        check_events(
            ssx3_ee::s07::f_00273040__generated,
            in_seq_span_track_get_next_event_time,
            120,
            &|c, seed| {
                let mut r = Rng(0x2730_4000 ^ seed as u64);
                scramble(c, &mut r);
                let track = DATA;
                c.g[A0][0] = track as u64;
                let has_table = seed % 5 != 0;
                c.bus.write32(track + 0x8, if has_table { 1 } else { 0 });
                c.bus.write32(track + 0x10, DATA + 0x100);
                c.bus
                    .write16(DATA + 0x100, if seed % 7 == 0 { 0 } else { 1 });
                let n = match seed % 4 {
                    0 => 0,
                    1 | 2 => 2,
                    _ => 4,
                };
                let head = if n == 0 { 0 } else { DATA + 0x1000 };
                c.bus.write32(track + 0x24, head);
                build_nodes(c, &mut r, DATA + 0x1000, n, 0xc, 0x18);
                let current = if seed % 3 == 0 { 0 } else { DATA + 0x2000 };
                c.bus.write32(track + 0x14, current);
                if current != 0 {
                    c.bus.write16(current + 0x4, r.next_u32() as u16);
                }
                let mode = schedule(c, &mut r, seed, 0x0027_3040);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn in_seq_event_track_get_next_event_time_matches() {
        check(
            ssx3_ee::s07::f_002733e0__generated,
            in_seq_event_track_get_next_event_time,
            160,
            &|c, seed| {
                let mut r = Rng(0x2733_e000 ^ seed as u64);
                scramble(c, &mut r);
                let track = DATA;
                c.g[A0][0] = track as u64;
                c.bus
                    .write32(track + 0x8, if seed % 5 == 0 { 0 } else { DATA + 0x100 });
                c.bus.write16(
                    DATA + 0x100,
                    if seed % 3 == 0 {
                        0
                    } else {
                        r.next_u32() as u16
                    },
                );
                let current = if seed % 4 == 0 { 0 } else { DATA + 0x200 };
                c.bus.write32(track + 0x14, current);
                if current != 0 {
                    c.bus.write16(current + 0x2, r.next_u32() as u16);
                }
            },
        );
    }

    #[test]
    fn in_seq_ramp_track_get_next_event_time_matches() {
        check_events(
            ssx3_ee::s07::f_002737e0__generated,
            in_seq_ramp_track_get_next_event_time,
            120,
            &|c, seed| {
                let mut r = Rng(0x2737_e000 ^ seed as u64);
                scramble(c, &mut r);
                let track = DATA;
                c.g[A0][0] = track as u64;
                let has_table = seed % 5 != 0;
                c.bus.write32(track + 0x8, if has_table { 1 } else { 0 });
                c.bus.write32(track + 0x10, DATA + 0x100);
                c.bus
                    .write16(DATA + 0x100, if seed % 7 == 0 { 0 } else { 1 });
                let n = match seed % 4 {
                    0 => 0,
                    1 | 2 => 2,
                    _ => 4,
                };
                let head = if n == 0 { 0 } else { DATA + 0x1000 };
                c.bus.write32(track + 0x20, head);
                build_nodes(c, &mut r, DATA + 0x1000, n, 0x8, 0x20);
                let current = if seed % 3 == 0 { 0 } else { DATA + 0x2000 };
                c.bus.write32(track + 0x14, current);
                if current != 0 {
                    c.bus.write16(current + 0x4, r.next_u32() as u16);
                }
                let mode = schedule(c, &mut r, seed, 0x0027_37e0);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn in_seq_channel_release_matches() {
        check(
            ee::s07::f_00273a00__generated,
            in_seq_channel_release,
            SEEDS_4,
            &|c, seed| {
                let _ = base_3(c, seed, 0x273a_00);
                let s = DATA + 0x100;
                clear(c, s, 0x80);
                w(c, s + 4, if seed % 2 == 0 { 0 } else { 1 });
                // `InSeq_ChannelStop` returns at once with no bound stream.
                w(c, s + 0x28, 0);
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn replay_save_compute_padding_matches() {
        check(
            ee::s07::f_00271620__generated,
            replay_save_compute_padding,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_1620, 2);
                be_library(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x640, DATA + 0x1_0000);
                c.bus.write32(DATA + 0x638, r.next_u32());
                c.bus.write32(DATA + 0x678, r.next_u32() & 0xffff);
            },
        );
    }

    #[test]
    fn replay_save_write_padding_matches() {
        check(
            ee::s07::f_00271688__generated,
            replay_save_write_padding,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_1688, 1);
                be_library(c);
                c.g[A0][0] = DATA as u64;
                let left: i32 = match seed % 5 {
                    0 => -1,
                    1 => 0,
                    2 => 1 + (r.next_u32() % 0x3fff) as i32,
                    3 => 0x4000,
                    _ => 0x4000 + (r.next_u32() % 0x1000) as i32,
                };
                c.bus.write32(DATA + 0x67c, left as u32);
                c.bus.write32(DATA + 0x640, DATA + 0x1_0000);
                c.bus.write32(DATA + 0x638, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_instance_get_remaining_steps_matches() {
        check(
            ee::s07::f_00274d08__generated,
            in_seq_instance_get_remaining_steps,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_4d08, 1);
                be_library(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x24, r.f32(8.0) | 0x0080_0000);
                c.bus.write32(DATA + 0x28, r.f32(4.0));
                c.bus.write32(DATA + 0x30, 0); // length path returns at once
            },
        );
    }

    /// The shipped replay state table at [`REPLAY_TABLE`].
    pub(crate) const REPLAY_HANDLERS: [u32; 16] = [
        0x0027_02ec,
        0x0027_02b0,
        0x0027_02ec,
        0x0027_02e8,
        0x0027_02d4,
        0x0027_02e8,
        0x0027_02e8,
        0x0027_02b0,
        0x0027_02e8,
        0x0027_02e8,
        0x0027_02ec,
        0x0027_02ec,
        0x0027_02ec,
        0x0027_02ec,
        0x0027_02ec,
        0x0027_02ec,
    ];

    #[test]
    fn replay_is_paused_matches() {
        strict_check(
            ssx3_ee::s07::f_00270280__generated,
            replay_is_paused,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2702_80, 0);
                c.pc = 0x0027_0280;
                let replay = DATA + 0x1000;
                c.g[A0][0] = sx(replay);
                put_words(c, REPLAY_TABLE, &REPLAY_HANDLERS);
                // States 0..15 reach every handler kind; 0x10 skips the lookup.
                c.bus.write32(replay, (seed % 17) as u32);
                c.bus.write32(replay + 0x3cc, r.next_u32() % 4);
                c.bus.write32(replay + 0x484, r.next_u32() % 4);
                c.bus
                    .write32(replay + 0x604, if seed % 3 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    pub(crate) fn in_seq_test(generated: GuestFn, hand: GuestFn, salt: u64, field: u32) {
        check(generated, hand, 20, &|c, seed| {
            fakes::machine(c, seed, salt, 0);
            globals(c);
            c.g[A0][0] = OBJ as u64;
            if seed % 3 == 0 {
                w(c, OBJ + 0xc, 0);
            } else {
                w(c, OBJ + 0xc, OBJ2);
                w(c, OBJ2 + 8, OBJ3);
                w(c, OBJ3 + 8, OBJ4);
                w(c, OBJ4 + 8, 0);
                w(c, OBJ + field, if seed % 2 == 0 { 0 } else { OBJ4 + 0x100 });
            }
        });
    }

    #[test]
    fn in_seq_span_track_stop_matches() {
        in_seq_test(
            ee::s07::f_00272ec0__generated,
            in_seq_span_track_stop,
            0x272ec0,
            0x20,
        );
    }

    #[test]
    fn in_seq_event_track_stop_matches() {
        in_seq_test(
            ee::s07::f_002732c8__generated,
            in_seq_event_track_stop,
            0x2732c8,
            0x1c,
        );
    }

    #[test]
    fn in_seq_ramp_track_stop_matches() {
        in_seq_test(
            ee::s07::f_00273630__generated,
            in_seq_ramp_track_stop,
            0x273630,
            0x1c,
        );
    }

    #[test]
    fn player_alloc_replay_cache_matches() {
        check(
            ee::s07::f_00270070__generated,
            player_alloc_replay_cache,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x270070, 0);
                globals(c);
                pool_allocator_2(c);
                c.g[A0][0] = OBJ as u64;
                let k = seed % 3;
                w(c, OBJ + 0x48c, if k == 0 { 0 } else { OBJ2 });
                w(c, OBJ + 0x490, if k == 2 { OBJ3 } else { 0 });
            },
        );
    }

    #[test]
    fn replay_count_highlights_matches() {
        check(
            ee::s07::f_00270bd8__generated,
            replay_count_highlights,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x270bd8, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                if seed % 5 == 0 {
                    w(c, OBJ + 0x3b0, 0);
                } else {
                    w(c, OBJ + 0x3b0, OBJ2);
                    w(c, OBJ2 + 0x1e, if seed % 3 == 0 { 1 } else { 0xff });
                    w(c, OBJ2 + 0x14, if seed % 4 == 0 { 0 } else { OBJ3 });
                    w(c, OBJ3 + 0x1e, 0xff);
                    w(c, OBJ3 + 0x14, 0);
                    for i in 0..4u32 {
                        w(
                            c,
                            OBJ2 + 0x20 + 4 * i,
                            if seed % 3 == 0 { 0xffff_ffff } else { 1 },
                        );
                        w(c, OBJ3 + 0x20 + 4 * i, 0x8000_0000);
                    }
                }
            },
        );
    }

    #[test]
    fn replay_begin_save_matches() {
        check(
            ee::s07::f_00270e68__generated,
            replay_begin_save,
            4,
            &|c, seed| {
                let _ = base_4(c, seed, 0x0027_0e68, 2);
                c.g[A0][0] = OBJ_2 as u64;
            },
        );
    }

    #[test]
    fn in_seq_start_sequence_matches() {
        check(
            ssx3_ee::s07::f_00272938__generated,
            in_seq_start_sequence,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x272938, 0);
                let seq = DATA + 0x1000;
                let src = DATA + 0x2000;
                c.g[A0][0] = seq as u64;
                c.g[A1][0] = src as u64;
                // Free node at seq+0x18, used list at seq+0x1c.
                let node = DATA + 0x3000;
                let used = DATA + 0x4000;
                w32(c, seq + 0x18, node);
                w32(c, seq + 0x1c, if seed % 2 == 0 { 0 } else { used });
                w32(c, node + 0x50, if seed % 3 == 0 { 0 } else { node + 0x100 });
                w32(c, node + 0x4, r.next_u32());
                w32(c, node + 0x54, r.next_u32());
                w32(c, used + 0x54, r.next_u32());
                // InSeq_InstanceSetup needs the source header to look benign.
                w16(c, src + 2, 0);
                w32(c, src + 4, 0);
                w32(c, src + 8, 0);
                w32(c, node, 0);
                w32(c, node + 8, 0);
                w32(c, node + 0x34, 0);
            },
        );
    }

    #[test]
    fn replay_seek_to_end_matches() {
        check(
            ee::s07::f_00270378__generated,
            replay_seek_to_end,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x270378, 2);
                be_library_2(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                w_3(
                    c,
                    obj + 0x48c,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                w_3(
                    c,
                    obj + 0x490,
                    if seed % 4 == 0 { 0 } else { DATA + 0x2000 },
                );
                w_3(c, obj + 0x3b0, 0);
                w_3(c, obj + 0x484, r.next_u32() % 0x1000);
            },
        );
    }

    #[test]
    fn replay_check_instant_replay_matches() {
        check(
            ee::s07::f_002705b0__generated,
            replay_check_instant_replay,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2705b0, 1);
                be_library_2(c);
                let obj = DATA;
                let alt = DATA + 0x800;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = alt as u64;
                c.g[A2][0] = (seed % 2) as u64;
                w_3(c, obj + 0x3d4, if seed % 3 == 0 { 0 } else { 1 });
                let wld = DATA + 0x1000;
                let race = DATA + 0x1200;
                let mgr = DATA + 0x1400;
                w_3(c, GAME_GP.wrapping_sub(0x848), wld);
                w_3(c, wld + 0x84, race);
                w_3(c, race + 0xc, mgr);
                w_3(c, mgr + 0x7c, if seed % 4 == 0 { 2 } else { 1 });
                w_3(c, alt + 0x9c, r.next_u32() % 0x1000);
                w_3(c, alt + 0xa4, r.next_u32() % 0x1000);
                w_3(c, obj + 0x474, r.next_u32() % 0x1000);
                w_3(c, obj + 0x47c, r.next_u32() % 0x1000);
            },
        );
    }

    // ------------------------------------------------------------------
    // In-sequence channels.

    #[test]
    fn in_seq_channel_set_listener_matches() {
        check(
            ssx3_ee::s07::f_00274240__generated,
            in_seq_channel_set_listener,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274240, 4);
                be_library_3(c);
                world_2(c);
                let ch = DATA + 0x100;
                c.g[A0][0] = ch as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    (DATA + 0x300) as u64
                };
                w_4(c, ch + 0x1c, DATA + 0x400);
                b(c, DATA + 0x400 + 3, (r.next_u32() % 4) as u8);
                if seed % 3 != 0 {
                    let old = DATA + 0x500;
                    let vt = DATA + 0x600;
                    w_4(c, ch + 0x2c, old);
                    w_4(c, old, vt);
                    slot(c, vt, 0x10, 0, FAKE);
                    slot(c, vt, 0x14, 0, FAKE + 0x10);
                } else {
                    w_4(c, ch + 0x2c, 0);
                }
                let new = DATA + 0x300;
                w_4(c, new, if seed % 4 == 0 { 0 } else { DATA + 0x700 });
                let vt = DATA + 0x700;
                slot(c, vt, 8, 0, FAKE + 0x20);
                slot(c, vt, 0xc, 0, FAKE + 0x30);
            },
        );
    }

    #[test]
    fn in_seq_curve_track_start_matches() {
        check(
            ee::s07::f_002725f0__generated,
            in_seq_curve_track_start,
            150,
            &|c, seed| {
                let mut r = base(c, seed, 0x27_25f0);
                globals_3(c);
                const DATA_PTR: u32 = DATA + 0x2000;
                w_5(c, DATA_PTR, [0, 1, 2][seed as usize % 3]);
                w_5(c, DATA_PTR + 4, [0, 0x10, 0x20][seed as usize % 3]);
                // A couple of curve keys for Math_EvalCubic.
                for i in 0..6u32 {
                    w_5(c, DATA_PTR + 8 + 4 * i, r.next_u32() & 0x3f80_3f80);
                }
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA_PTR);
                c.g[A2][0] = sx(DATA + 0x3000);
            },
        );
    }

    #[test]
    fn replay_flush_save_output_matches() {
        check(
            ee::s07::f_00270fc0__generated,
            replay_flush_save_output,
            SEEDS_8,
            &|c, seed| {
                let _ = base_5(c, seed, 0x0fc0, 2);
                let this = DATA;
                match seed % 4 {
                    0 => w_6(c, this + 0x644, 0),
                    1 => {
                        w_6(c, this + 0x644, DATA + 0x1000);
                        w_6(c, this + 0x648, 0);
                    }
                    2 => {
                        w_6(c, this + 0x644, DATA + 0x1000);
                        w_6(c, this + 0x648, 0x100);
                        w_6(c, this + 0x64c, 0x4000);
                        w_6(c, this + 0x650, DATA + 0x8000);
                    }
                    _ => {
                        w_6(c, this + 0x644, DATA + 0x1000);
                        w_6(c, this + 0x648, 0x3000);
                        w_6(c, this + 0x64c, 0x100);
                        w_6(c, this + 0x650, DATA + 0x8000);
                    }
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn replay_save_build_header_matches() {
        check(
            ee::s07::f_00271228__generated,
            replay_save_build_header,
            SEEDS_8,
            &|c, seed| {
                let _ = base_5(c, seed, 0x1228, 3);
                let this = DATA;
                w_6(c, this + 0x640, DATA + 0x1000);
                w_6(c, this + 0x18, 0x1234);
                w_6(c, this + 0x610, 0x5678);
                w_6(c, this + 0x3c8, 0x9abc);
                w_6(c, this + 0x3cc, 0xdef0);
                w_6(c, this + 0x3b0, 0);
                w_6(c, this + 0x3d0, 0);
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn in_seq_instance_get_length_matches() {
        check(
            ee::s07::f_00274c70__generated,
            in_seq_instance_get_length,
            SEEDS_8,
            &|c, seed| {
                let _ = base_5(c, seed, 0x4c70, 2);
                let this = DATA;
                if seed & 1 == 0 {
                    w_6(c, this + 0x30, 0x1234);
                } else {
                    w_6(c, this + 0x30, 0xffff_ffff);
                    w_6(c, this + 0x10, DATA + 0x1000);
                    wb(c, DATA + 0x1000 + 1, (seed % 3) as u8);
                    w_6(c, this + 0x18, DATA + 0x1100);
                    for i in 0..3u32 {
                        let ch = DATA + 0x1100 + 0x3c * i;
                        w_6(c, ch + 0x1c, DATA + 0x1300 + 0x20 * i);
                        w_6(c, DATA + 0x1300 + 0x20 * i + 4, 0x100 + i);
                    }
                    w_6(c, this + 0x20, (seed & 2) as u32);
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn in_seq_channel_flush_events_matches() {
        check(
            ssx3_ee::s07::f_00273d20__generated,
            in_seq_channel_flush_events,
            30,
            &|c, seed| {
                let _r = base_6(c, seed, 0x273d20, 2);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                w(
                    c,
                    this + 0x2c,
                    if seed % 2 == 0 { 0 } else { DATA + 0x2000 },
                );
                w(c, this + 0x30, r32_seed(seed));
                w(
                    c,
                    this + 0x34,
                    if seed % 3 == 0 { 0 } else { DATA + 0x3000 },
                );
                w(c, this + 0x1c, DATA + 0x4000);
                w(c, DATA + 0x2000, DATA + 0x2100);
                w(c, DATA + 0x2100 + 0x18, 0);
                w(c, DATA + 0x2100 + 0x1c, 0);
                w(c, DATA + 0x2100 + 0x20, 0);
                w(c, DATA + 0x2100 + 0x24, 0);
            },
        );
    }

    // ------------------------------------------------------------------ InSeq

    #[test]
    fn in_seq_free_sequence_matches() {
        check(
            ee::s07::f_002729a8__generated,
            in_seq_free_sequence,
            FEW,
            &|c, seed| {
                let _ = base_7(c, seed, 0x2729_a8, 4);
                let mgr = SCRATCH_2;
                let handle = 0x58 * (seed % 4) as u32;
                c.g[A0][0] = mgr as u64;
                c.g[A1][0] = handle as u64;
                w_7(c, mgr + 0x14, mgr + 0x2000);
                let seq = mgr + 0x2000 + handle;
                // Linked-list links: next (+0x50), prev (+0x54), instance
                // (+0x58) and the manager's active/idle head pointers.
                w_7(c, seq + 0x50, if seed % 3 == 0 { 0 } else { seq + 0x100 });
                w_7(c, seq + 0x54, if seed % 3 == 0 { 0 } else { seq + 0x200 });
                w_7(c, seq + 0x58, 0);
                w_7(c, seq + 0x100 + 0x50, seq);
                w_7(c, seq + 0x100 + 0x54, 0);
                w_7(c, seq + 0x200 + 0x50, 0);
                w_7(c, seq + 0x200 + 0x54, seq);
                w_7(c, mgr + 0x1c, seq);
                w_7(c, mgr + 0x18, seq + 0x100);
            },
        );
    }

    #[test]
    fn in_seq_event_track_start_matches() {
        strict(
            ee::s07::f_00273208__generated,
            in_seq_event_track_start,
            30,
            &|c, seed| {
                let mut r = Rng(0x0027_3208 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let seq = DATA + 0x100;
                c.bus.write16(seq, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write16(seq + 2, (r.next_u32() % 4) as u16);
                c.bus.write16(seq + 6, (r.next_u32() % 4) as u16);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(seq);
                c.g[A2][0] = sx(DATA + 0x200);
                c.g[A3][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn inseq_span_track_start_matches() {
        check(
            ee::s07::f_00272df0__generated,
            inseq_span_track_start,
            60,
            &|c, seed| {
                let _r = base_8(c, seed, 0x272d);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.g[A3][0] = (DATA + 0x3000) as u64;
                // An empty key list: the short path (the full path allocates
                // and walks a heap that cannot be bounded on test data).
                c.bus.write16(DATA + 0x1000, 0);
                c.bus.write32(DATA + 0x3000, seed);
            },
        );
    }

    #[test]
    fn replay_on_rider_checkpoint_matches() {
        check(
            ee::s07::f_00270ab0__generated,
            replay_on_rider_checkpoint,
            16,
            &|c, seed| {
                let _ = base_9(c, seed, 0x270ab0, 2);
                let this = DATA;
                clear_2(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = if seed % 4 == 0 {
                    0xffff_fffe
                } else {
                    (seed % 4) as u64
                };
                w_8(c, this + 0x61c, if seed % 5 == 0 { 1 } else { 0 });
                w_8(c, this + 0x620, if seed % 5 == 1 { 1 } else { 0 });
                w_8(c, this + 0x610, if seed % 5 == 2 { 1 } else { 0 });
                w_8(c, this + 0x60c, if seed % 5 == 3 { 1 } else { 0 });
                w_8(c, this + 0x10, 8);
                w_8(c, GAME_GP.wrapping_sub(0x848), WORLD);
            },
        );
    }

    #[test]
    fn in_seq_curve_track_update_matches() {
        diff(
            ee::s07::f_002726a8__generated,
            in_seq_curve_track_update,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27_26a8, 0);
                globals_6(c);
                let mode = seed % 4;
                let curve = if mode == 0 { 0 } else { DATA + 0x2000 };
                c.g[A0][0] = sx(DATA);
                c.f[12] = uf(&mut r, 10.0);
                w_9(c, DATA + in_seq_curve::CURVE, curve);
                w_9(c, DATA + in_seq_curve::VALUE, r.next_u32());
                w_9(c, DATA + in_seq_curve::LENGTH, DATA + 0x2100);
                w_9(
                    c,
                    DATA + in_seq_curve::SEGMENT,
                    if mode == 3 { 0 } else { DATA + 0x2200 },
                );
                w_9(c, DATA + in_seq_curve::KEY, (seed % 3) as u32);
                if curve != 0 {
                    c.bus.write16(curve, (seed % 0x30) as u16);
                    c.bus.write16(curve + 2, if mode == 1 { 2 } else { 0x20 });
                    fill(c, &mut r, curve + 4, 4);
                }
                w_9(c, DATA + 0x2100, (seed % 3) as u32);
                c.bus.write16(DATA + 0x2200, (seed % 0x30) as u16);
                c.bus.write16(DATA + 0x2202, (seed % 0x30) as u16);
                for i in 0..4 {
                    w_9(c, DATA + 0x2204 + 4 * i, uf(&mut r, 2.0));
                }
            },
        );
    }

    #[test]
    fn in_seq_heap_free_matches() {
        diff(
            ee::s07::f_002723a8__generated,
            in_seq_heap_free,
            40,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x27_23a8);
                c.g[A0][0] = sx(DATA);
                w_9(c, DATA + 8, if seed % 2 == 0 { 0 } else { 0 });
                w_9(c, DATA, r.next_u32());
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x1008) };
                w_9(c, DATA + 0x1000, 0x20); // the block's size header (payload at +8)
                w_9(c, DATA + 0x1008, 0x30);
                w_9(c, DATA + 0x100c, 0);
            },
        );
    }

    /// One `InSeq_Instance` at `obj` with a one-second channel block (the
    /// cached length avoids the channel walk) and seed-picked seek/step/play
    /// state.
    pub(crate) fn fixture(c: &mut Cpu, seed: u32) {
        fakes::machine(c, seed, 0x274a30, 0);
        c.pc = 0x0027_4a30;
        let obj = DATA + 0x1000;
        let chan = DATA + 0x2000;
        c.g[A0][0] = obj as u64;
        c.bus.write8(chan + 1, 0);
        c.bus.write32(obj + 0x10, chan);
        c.bus.write32(obj + 0x18, 0);
        c.bus.write32(obj + 0x1c, if seed % 4 == 0 { 0 } else { 1 });
        c.bus
            .write32(obj + 0x20, [0, 1, 1, 2, 4][(seed % 5) as usize]);
        c.bus.write32(
            obj + 0x24,
            if seed % 2 == 0 {
                0x3f80_0000
            } else {
                0x3f00_0000
            },
        );
        c.bus.write32(
            obj + 0x28,
            [0, 0x3f00_0000, 0x4020_0000][(seed % 3) as usize],
        );
        c.bus.write32(obj + 0x2c, if seed % 2 == 0 { 0 } else { 1 });
        c.bus.write32(obj + 0x30, 100);
        c.bus
            .write32(obj + 0x34, [0xffff_ffff, 0, 3][(seed % 3) as usize]);
        c.bus.write32(obj + 0x38, [0, 1, 3, 0][(seed % 4) as usize]);
        c.bus.write32(obj + 0x3c, 0);
        c.bus.write32(obj + 0x40, 0);
    }

    #[test]
    fn in_seq_instance_update_matches() {
        check(
            ee::s07::f_00274a30__generated,
            in_seq_instance_update,
            24,
            &|c, seed| {
                fixture(c, seed);
            },
        );
    }

    /// A playing sequence (speed 1, then 0.5) moves the same script time per
    /// second at 60, 120, 144 and 240 Hz, and keeps its own speed.
    #[test]
    fn in_seq_instance_update_rate() {
        for speed in [1.0f32, 0.5] {
            for hz in [60u32, 120, 144, 240] {
                let mut c = crate::difftest::fresh();
                fixture(&mut c, 2);
                crate::tick::set_for_test(&mut c, hz);
                let obj = DATA + 0x1000;
                c.bus.write32(obj + 0x1c, 1);
                c.bus.write32(obj + 0x20, 3);
                c.bus.write32(obj + 0x24, speed.to_bits());
                c.bus.write32(obj + 0x28, 0);
                c.bus.write32(obj + 0x2c, 0);
                c.bus.write32(obj + 0x30, 1000);
                c.bus.write32(obj + 0x34, 0xffff_ffff);
                c.bus.write32(obj + 0x38, 0);
                for _ in 0..hz {
                    c.g[A0][0] = obj as u64;
                    c.g[RA][0] = SENTINEL_RA as u64;
                    c.pc = 0x0027_4a30;
                    in_seq_instance_update(&mut c);
                    assert_eq!(c.pc, SENTINEL_RA, "{hz} Hz returned");
                }
                let pos = f32::from_bits(c.bus.read32(obj + 0x28));
                assert!(
                    (pos - 60.0 * speed).abs() < 0.01,
                    "{hz} Hz speed {speed}: pos {pos}"
                );
                assert_eq!(
                    f32::from_bits(c.bus.read32(obj + 0x24)),
                    speed,
                    "{hz} Hz keeps its speed"
                );
            }
        }
    }

    /// InSeq_UpdateAll: an empty list or one to five nodes whose instances
    /// take the real InSeq_InstanceUpdate's short path (no restart pending,
    /// no queued ticks) into InSeq_InstanceTick's early returns (idle, or
    /// waiting on a delay).
    #[test]
    fn in_seq_update_all_matches() {
        check(
            ee::s07::f_002728f8__generated,
            in_seq_update_all,
            200,
            &|c, seed| {
                let mut r = Rng(0x0027_28f8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0027_28f8;
                let seq = DATA;
                let n = if seed % 5 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 5
                };
                let mut next = 0u32;
                for i in (0..n).rev() {
                    let node = DATA + 0x1000 + 0x100 * i;
                    let inst = node + INSEQ_NODE_INSTANCE;
                    c.bus.write32(node + INSEQ_NODE_NEXT, next);
                    c.bus.write32(inst + 0x34, 0xffff_ffff);
                    c.bus.write32(inst + 0x38, 0);
                    let idle = r.next_u32() % 2 == 0;
                    c.bus
                        .write32(inst + 0x20, if idle { 0 } else { 1 + r.next_u32() % 4 });
                    c.bus.write32(inst + 0x2c, 1 + r.next_u32() % 100);
                    next = node;
                }
                c.bus.write32(seq + INSEQ_FIRST, next);
                c.g[A0][0] = sx(seq);
            },
        );
    }

    /// A world with 0..5 riders (or a negative count), each rider object's
    /// replay id getter a fake method; the fake for rider `hit` (if any)
    /// returns the id asked for, the others a pointer.
    pub(crate) fn replay_find_rider_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        use crate::difftest::GAME_GP;
        let count = match seed % 6 {
            0 => 0u32,
            1 => (-(1 + (seed / 6 % 3) as i32)) as u32,
            _ => 1 + seed / 6 % 5,
        };
        let calls = if (count as i32) > 0 { count } else { 0 };
        let mut r = machine(c, seed, 0x2707_3000, calls);
        let world = DATA;
        let list = DATA + 0x100;
        let table = DATA + 0x200;
        c.bus.write32(GAME_GP - REPLAY_FIND_WORLD_GP, world);
        c.bus.write32(world + REPLAY_FIND_WORLD_RIDERS, list);
        c.bus.write32(list + REPLAY_FIND_LIST_TABLE, table);
        c.bus.write32(table + REPLAY_FIND_TABLE_COUNT, count);
        for i in 0..calls {
            let rider = DATA + 0x1000 + i * 0x100;
            let obj = DATA + 0x4000 + i * 0x1000;
            let vt = DATA + 0x10000 + i * 0x100;
            c.bus
                .write32(table + REPLAY_FIND_TABLE_RIDERS + 4 * i, rider);
            c.bus.write32(rider + REPLAY_FIND_RIDER_OBJ, obj);
            c.bus.write32(obj + REPLAY_FIND_OBJ_ID, vt);
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + REPLAY_FIND_ID_SLOT, delta as u16);
            c.bus.write32(vt + REPLAY_FIND_ID_SLOT + 4, FAKE + 0x10 * i);
        }
        let id = 1 + r.next_u32() % 0x1000;
        // Rider `hit` answers with the id (none for seed % 4 == 0).
        if calls > 0 && seed % 4 != 0 {
            let hit = r.next_u32() % calls;
            c.bus.write32(LOG_RESULTS + 4 * hit, id);
        }
        c.g[A0][0] = if seed % 7 == 3 {
            (r.next_u32() as u64) << 32 | id as u64
        } else {
            id as u64
        };
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0027_0730;
    }

    /// `Replay_FindRiderIndex`: found at any index, not found, no riders,
    /// fakes unwinding.
    #[test]
    fn replay_find_rider_index_matches() {
        check(
            ee::s07::f_00270730__generated,
            replay_find_rider_index,
            300,
            &replay_find_rider_fixture,
        );
    }

    /// The world and riders of `Replay_FindRiderIndex`'s test (found at any
    /// index, not found, no riders; fake id getters, some unwinding), the
    /// busy flags each set or all clear, and the highlight slot of the found
    /// rider in use (early out) or free (the real `HighlightSlotBegin`:
    /// snapshot bit, value, and the 0xa0-byte copy from `data`, aligned or
    /// not).
    #[test]
    fn replay_on_rider_launch_matches() {
        use crate::game::replay::replaycache::tests::replay_find_rider_fixture;
        check(
            ee::s07::f_002707e0__generated,
            replay_on_rider_launch,
            300,
            &|c, seed| {
                replay_find_rider_fixture(c, seed);
                let id = c.g[A0][0];
                let mut r = Rng(0x2707_e000 ^ seed as u64);
                let this = DATA + 0x3_0000;
                let snapshot = DATA + 0x3_8000;
                let data = DATA + 0x3_9000 + (r.next_u32() % 2) * 4;
                crate::difftest::fill(c, &mut r, data, 0x30);
                crate::difftest::fill(c, &mut r, snapshot, 0x40);
                let busy = seed / 2 % 7;
                for (i, off) in OC300_REPLAY_BUSY_FLAGS
                    .into_iter()
                    .chain([OC300_REPLAY_PLAYING])
                    .enumerate()
                {
                    let set = busy as usize == i + 1;
                    c.bus
                        .write32(this + off, if set { 1 + r.next_u32() % 4 } else { 0 });
                }
                c.bus.write32(this + 0x3b4, snapshot);
                for k in 0..5u32 {
                    let slot = this + 0x494 + 0xb4 * k;
                    c.bus.write32(slot, r.next_u32() % 8);
                    let used = r.next_u32() % 3 == 0;
                    c.bus.write32(slot + 0xb0, if used { 1 } else { 0 });
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = id;
                c.g[A2][0] = sx(data);
                c.pc = 0x0027_07e0;
            },
        );
    }

    /// `Replay_OnRiderLand`: the find-rider world of `Replay_FindRiderIndex`'s
    /// test (found, not found, no riders, fakes unwinding), each busy flag
    /// set or all clear, the rider's highlight slot free (the real
    /// `Replay_EndHighlight` returns early), and the real
    /// `Replay_CheckInstantReplay` with instant replays armed or not, one or
    /// more riders, the landing time close to or past the last one.
    #[test]
    fn replay_on_rider_land_matches() {
        use crate::game::replay::replaycache::tests::replay_find_rider_fixture;
        crate::difftest::check(
            ee::s07::f_00270870__generated,
            replay_on_rider_land,
            300,
            &|c, seed| {
                replay_find_rider_fixture(c, seed);
                let id = c.g[A0][0];
                let mut r = crate::difftest::Rng(0x2708_7000 ^ seed as u64);
                let this = crate::difftest::DATA + 0x3_0000;
                let data = crate::difftest::DATA + 0x3_9000;
                let busy = seed / 2 % 5;
                for (i, off) in OC305_REPLAY_LAND_BUSY_FLAGS.into_iter().enumerate() {
                    let set = busy as usize == i + 1;
                    c.bus
                        .write32(this + off, if set { 1 + r.next_u32() % 4 } else { 0 });
                }
                // Highlight slots: all free.
                for k in 0..5u32 {
                    c.bus.write32(this + 0x494 + 0xb4 * k + 0xb0, 0);
                }
                // Instant replay armed (`+0x3d4`), last times `+0x474/+0x47c`.
                c.bus.write32(this + 0x3d4, (seed % 3 != 0) as u32);
                let last = r.next_u32() % 5000;
                c.bus.write32(this + 0x474, last);
                c.bus.write32(this + 0x47c, last);
                let now = last.wrapping_add(if seed % 4 < 2 { 500 } else { 1500 });
                c.bus.write32(data + 0x9c, now);
                c.bus.write32(data + 0xa4, now);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = id;
                c.g[A2][0] = sx(data);
                c.pc = 0x0027_0870;
            },
        );
    }

    /// `Replay_PruneSnapshots`: a live list of 0..6 snapshots (doubly
    /// linked, with count) and a free list, each node kept by its halfword,
    /// either byte, or nothing (freed through the real `Replay_FreeSnapshot`:
    /// unlink, stream reset, push onto the free list); events off, delivered,
    /// or unwinding at the loop head.
    #[test]
    fn replay_prune_snapshots_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        check_events(
            ee::s07::f_00270538__generated,
            replay_prune_snapshots,
            240,
            &|c, seed| {
                let mut r = crate::difftest::Rng(0x2705_3800 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                let this = crate::difftest::DATA;
                let n = seed / 3 % 7;
                let node_at = |i: u32| crate::difftest::DATA + 0x1000 + 0x100 * i;
                for i in 0..n {
                    let node = node_at(i);
                    crate::difftest::fill(c, &mut r, node, 0x40);
                    let next = if i + 1 < n { node_at(i + 1) } else { 0 };
                    let prev = if i > 0 { node_at(i - 1) } else { 0 };
                    c.bus.write32(node + OC305_SNAPSHOT_NEXT, next);
                    c.bus.write32(node + 0x18, prev);
                    let keep = r.next_u32() % 4;
                    let v = 1 + r.next_u32() % 0xff;
                    c.bus.write16(
                        node + OC305_SNAPSHOT_KEEP_HALF,
                        if keep == 1 { v as u16 } else { 0 },
                    );
                    c.bus.write8(
                        node + OC305_SNAPSHOT_KEEP_B,
                        if keep == 2 { v as u8 } else { 0 },
                    );
                    c.bus.write8(
                        node + OC305_SNAPSHOT_KEEP_A,
                        if keep == 3 { v as u8 } else { 0 },
                    );
                }
                let live = this + OC305_REPLAY_LIVE_SNAPSHOTS;
                c.bus.write32(live, if n > 0 { node_at(0) } else { 0 });
                c.bus
                    .write32(live + 4, if n > 0 { node_at(n - 1) } else { 0 });
                c.bus.write32(live + 8, n);
                // Free list at +0x3bc: empty or one node.
                let spare = crate::difftest::DATA + 0x2000;
                let has_spare = seed % 2 == 0;
                c.bus
                    .write32(this + 0x3bc, if has_spare { spare } else { 0 });
                c.bus
                    .write32(this + 0x3c0, if has_spare { spare } else { 0 });
                c.bus.write32(this + 0x3c4, has_spare as u32);
                c.bus.write32(spare + OC305_SNAPSHOT_NEXT, 0);
                c.bus.write32(spare + 0x18, 0);
                c.g[A0][0] = sx(this);
                let mode = schedule(c, &mut r, seed, 0x0027_0538);
                if mode != Events::Off {
                    c.next_event = c.cycles + (r.next_u32() % 20) as u64;
                }
                mode
            },
        );
    }

    /// Free list with one snapshot (count > 0: popped directly) or a
    /// non-positive count (the recycle pass runs first; its ring is empty, so
    /// it only walks twice and returns).
    #[test]
    fn replay_alloc_snapshot_matches() {
        crate::difftest::check(
            ee::s07::f_00270130__generated,
            replay_alloc_snapshot,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27_0130, 0);
                let replay = DATA;
                let node = DATA + 0x1000;
                let count = match seed % 3 {
                    0 => 0,
                    1 => (r.next_u32() % 5) + 1,
                    _ => r.next_u32() | 0x8000_0000,
                };
                c.bus.write32(replay + REPLAY_FREE_COUNT, count);
                c.bus.write32(replay + REPLAY_FREE_LIST, node);
                c.bus.write32(replay + REPLAY_FREE_LIST + 4, node);
                // Recycle ring: current (+0x3ac) and start (+0x3b0) empty.
                c.bus.write32(replay + 0x3ac, 0);
                c.bus.write32(replay + 0x3b0, 0);
                crate::difftest::fill(c, &mut r, node, 0x10);
                c.bus.write32(node + 0x14, 0);
                c.bus.write32(node + 0x18, 0);
                c.g[A0][0] = sx(replay);
                c.pc = 0x0027_0130;
            },
        );
    }

    /// `Replay_EndHighlight` with the real callees (all short leaves): a
    /// replay with a snapshot list of 0..4 nodes (random flag bytes and
    /// per-slot values), slot 0..5 whose node pointer is null (nothing to
    /// end), a list node or a lone node; score inputs that give 0 (clear) or
    /// 1 (kept), maximum counts that make the count below or at the limit
    /// (evict the weakest), both modes. LO/HI are compared (`mult`).
    #[test]
    fn replay_end_highlight_matches() {
        use crate::difftest::{check_lohi, fill, scramble, Rng, DATA};
        check_lohi(
            ee::s07::f_00270ce8__generated,
            replay_end_highlight,
            300,
            &|c, seed| {
                let mut r = Rng(0x0270_ce80 ^ (seed as u64) << 4);
                scramble(c, &mut r);
                let replay = DATA + 0x1000;
                let player = DATA + 0x3000;
                let nodes = DATA + 0x4000;
                let lone = DATA + 0x5000;
                fill(c, &mut r, replay, 0x300);
                fill(c, &mut r, player, 0x40);
                let slot_n = seed % 6;
                let slot =
                    replay + REPLAY_END_HIGHLIGHT_SLOTS + slot_n * REPLAY_END_HIGHLIGHT_SLOT_SIZE;
                // Snapshot list: node+0x14 next, +0x1e slot bits, +0x1f skip,
                // +0x20 + 4 * slot value.
                let count = r.next_u32() % 5;
                let init_node = |c: &mut Cpu, r: &mut Rng, n: u32| {
                    fill(c, r, n, 0x20);
                    c.bus
                        .write8(n + REPLAY_END_HIGHLIGHT_NODE_FLAGS, r.next_u32() as u8);
                    c.bus.write8(n + 0x1f, (r.next_u32() % 4 == 0) as u8);
                    for i in 0..8 {
                        c.bus
                            .write32(n + 0x20 + 4 * i, (r.next_u32() % 7).wrapping_sub(3));
                    }
                };
                for i in 0..count {
                    let n = nodes + i * 0x80;
                    init_node(c, &mut r, n);
                    c.bus
                        .write32(n + 0x14, if i + 1 < count { n + 0x80 } else { 0 });
                }
                init_node(c, &mut r, lone);
                c.bus
                    .write32(replay + 0x3b0, if count == 0 { 0 } else { nodes });
                let node = match seed % 5 {
                    0 => 0,
                    1 => lone,
                    _ if count > 0 => nodes + (r.next_u32() % count) * 0x80,
                    _ => lone,
                };
                c.bus.write32(slot + 0xb0, node);
                c.bus.write32(slot, r.next_u32() % 8); // slot index
                c.bus.write32(replay + 0xc, r.next_u32() % 4); // max highlights
                                                               // Score inputs: zero/near/far progress, distance gate at 5.0.
                let base = r.next_u32() % 5000;
                c.bus.write32(slot + 0xa0, base);
                c.bus.write32(
                    player + 0x9c,
                    base + [0, 400, 1500][(r.next_u32() % 3) as usize],
                );
                let t = r.next_u32() % 100;
                c.bus.write32(slot + 0xa8, t);
                c.bus.write32(
                    player + 0xa4,
                    if r.next_u32() % 3 == 0 {
                        t
                    } else {
                        t + 1 + r.next_u32() % 50
                    },
                );
                c.bus.write32(
                    player + 0x44,
                    (if r.next_u32() % 2 == 0 {
                        4.0f32
                    } else {
                        6.0f32
                    })
                    .to_bits(),
                );
                let mode = match (seed / 6) % 3 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32() as u64 | 1,
                };
                c.pc = 0x0027_0ce8;
                c.g[A0][0] = sx(replay);
                c.g[A1][0] = slot_n as u64;
                c.g[A2][0] = sx(player);
                c.g[A3][0] = mode;
            },
        );
    }
}
