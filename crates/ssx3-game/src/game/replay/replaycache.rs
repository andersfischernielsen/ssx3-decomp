//! `src/replay/replaycache.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
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
pub(crate) use crate::game::input::inputparse::o406_lw;
pub(crate) use crate::game::input::inputparse::o406_sw;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::main::ssxapp::oc429_at;
pub(crate) use crate::game::scripter::datamanager::oc538_field;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_10;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p_3;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_add;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_lbu;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_lh;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_lw;
pub(crate) use crate::game::seg::seg_1fbe38::oc417_sw0;
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
    (
        0x0027_08f0,
        replay_on_rider_wipeout,
        "Replay_OnRiderWipeout",
    ),
    (0x0027_0b88, replay_begin_highlight, "Replay_BeginHighlight"),
    (
        0x0027_3698,
        in_seq_ramp_track_update,
        "InSeq_RampTrackUpdate",
    ),
    (0x0027_2308, in_seq_heap_alloc, "InSeq_HeapAlloc"),
    (
        0x0027_2f28,
        in_seq_span_track_update,
        "InSeq_SpanTrackUpdate",
    ),
    (
        0x0027_3330,
        in_seq_event_track_update,
        "InSeq_EventTrackUpdate",
    ),
    (0x0027_3dc0, in_seq_channel_stop, "InSeq_ChannelStop"),
    (0x0027_4100, in_seq_channel_advance, "InSeq_ChannelAdvance"),
    (
        0x0027_43e8,
        in_seq_channel_update_tracks,
        "InSeq_ChannelUpdateTracks",
    ),
    (0x0027_4de0, in_seq_instance_tick, "InSeq_InstanceTick"),
    (
        0x0027_51a0,
        in_seq_instance_advance,
        "InSeq_InstanceAdvance",
    ),
    (0x0027_2488, in_seq_heap_new, "InSeq_HeapNew"),
    (0x0027_24e8, in_seq_heap_delete, "InSeq_HeapDelete"),
    (0x0027_3530, in_seq_ramp_track_start, "InSeq_RampTrackStart"),
    (0x0027_3aa8, in_seq_channel_start, "InSeq_ChannelStart"),
    (0x0027_4918, in_seq_instance_stop, "InSeq_InstanceStop"),
    (0x0027_45c0, in_seq_instance_setup, "InSeq_InstanceSetup"),
    (
        0x0027_46f8,
        in_seq_instance_release,
        "InSeq_InstanceRelease",
    ),
    (0x0027_2788, in_seq_ctor, "InSeq_ctor"),
    (
        0x0027_24c8,
        oc436_in_seq_heap_new_array,
        "InSeq_HeapNewArray",
    ),
    (
        0x0027_2510,
        inseq_heap_delete_array,
        "InSeq_HeapDeleteArray",
    ),
    (
        0x0027_4388,
        oc436_in_seq_channel_get_header_field4,
        "InSeq_ChannelGetHeaderField4",
    ),
    (
        0x0027_4c10,
        inseq_instance_set_user_data,
        "InSeq_InstanceSetUserData",
    ),
    (0x0027_0ed8, replay_end_save, "Replay_EndSave"),
    (0x0027_0ef8, replay_reset_on_load, "Replay_ResetOnLoad"),
    (0x0027_0f18, replay_empty_load_hook, "Replay_EmptyLoadHook"),
    (
        0x0027_10a8,
        replay_write_save_block,
        "Replay_WriteSaveBlock",
    ),
    (
        0x0027_1380,
        replay_save_write_snapshot,
        "Replay_SaveWriteSnapshot",
    ),
    (
        0x0027_1510,
        replay_save_write_cache,
        "Replay_SaveWriteCache",
    ),
    (0x0027_16f0, replay_next_save_block, "Replay_NextSaveBlock"),
    (
        0x0027_18a8,
        replay_debug_printf_stub,
        "Replay_DebugPrintfStub",
    ),
    (
        0x0027_18e8,
        replay_static_init_destroy,
        "Replay_StaticInitDestroy",
    ),
    (0x0027_22c0, replay_global_ctors, "Replay_GlobalCtors"),
    (
        0x0027_2530,
        insseq_heap_delete_thunk,
        "InSeq_HeapDelete_Thunk",
    ),
    (
        0x0027_2550,
        insseq_heap_delete_thunk2,
        "InSeq_HeapDelete_Thunk2",
    ),
    (0x0027_2a50, insseq_free_all, "InSeq_FreeAll"),
    (0x0027_2b00, insseq_play_all, "InSeq_PlayAll"),
    (0x0027_2b80, insseq_stop_all, "InSeq_StopAll"),
    (0x0027_2bd0, insseq_pause, "InSeq_Pause"),
    (0x0027_2bf8, insseq_pause_all, "InSeq_PauseAll"),
    (0x0027_2c48, insseq_resume, "InSeq_Resume"),
    (0x0027_2c70, insseq_resume_all, "InSeq_ResumeAll"),
    (
        0x0027_2ce0,
        insseq_get_instance_data,
        "InSeq_GetInstanceData",
    ),
    (
        0x0027_2d00,
        insseq_get_instance_owner,
        "InSeq_GetInstanceOwner",
    ),
    (
        0x0027_2d20,
        insseq_get_channel_count,
        "InSeq_GetSeqChannelCount",
    ),
    (
        0x0027_2d40,
        insseq_get_channel_block_field4,
        "InSeq_GetChannelBlockField4",
    ),
    (
        0x0027_4318,
        in_seq_get_block_payload,
        "InSeq_GetBlockPayload",
    ),
    (0x0027_4398, in_seq_block_get_field4, "InSeq_BlockGetField4"),
    (0x0027_43c0, in_seq_block_get_size, "InSeq_BlockGetSize"),
    (0x0027_4578, in_seq_instance_dtor, "InSeq_Instance_dtor"),
    (
        0x0027_4808,
        in_seq_instance_set_speed,
        "InSeq_InstanceSetSpeed",
    ),
    (
        0x0027_4810,
        in_seq_instance_restart,
        "InSeq_InstanceRestart",
    ),
    (
        0x0027_4868,
        in_seq_instance_run_to_end,
        "InSeq_InstanceRunToEnd",
    ),
    (
        0x0027_4c58,
        in_seq_instance_get_data,
        "InSeq_InstanceGetData",
    ),
    (
        0x0027_4c60,
        in_seq_instance_get_owner,
        "InSeq_InstanceGetOwner",
    ),
    (
        0x0027_4c68,
        in_seq_get_channel_count,
        "InSeq_GetChannelCount",
    ),
    (
        0x0027_4d90,
        in_seq_find_channel_data,
        "InSeq_FindChannelData",
    ),
    (
        0x0027_0c58,
        replay_find_weakest_highlight_2,
        "Replay_FindWeakestHighlight",
    ),
    (0x0027_2aa0, in_seq_run_to_end, "InSeq_RunToEnd"),
    (0x0027_3f68, in_seq_channel_restart, "InSeq_ChannelRestart"),
    (
        0x0027_01a0,
        replay_recycle_oldest_snapshot_2,
        "Replay_RecycleOldestSnapshot",
    ),
    (0x0027_03f0, replay_step_frame_3, "Replay_StepFrame"),
    (0x0027_09d8, replay_on_rider_event_3, "Replay_OnRiderEvent"),
    (0x0027_1758, replay_load_saved_2, "Replay_LoadSaved"),
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

/// Speech sentence entries base: `0x450000 + 0x6c`.
pub const SPCH_SENTENCE_BASE: u32 = 0x0045_006c;

// ---------------------------------------------------------------------------
// Replay_OnRiderWipeout (0x2708f0)

/// `Replay` flags that block the wipeout bookkeeping: `+0x61c` (playing
/// back), `+0x620` (instant replay), then the replay state at `+0`.
pub const OC331_REPLAY_WIPEOUT_BUSY: [u32; 3] = [0x61c, 0x620, 0x0];

/// `Replay_FindRiderIndex` (0x270730).
pub const OC331_REPLAY_FIND_RIDER_INDEX: Callee = Callee {
    addr: 0x0027_0730,
    f: ee::s07::Replay_FindRiderIndex,
};

/// `Replay_EndHighlight` (0x270ce8).
pub const OC331_REPLAY_END_HIGHLIGHT: Callee = Callee {
    addr: 0x0027_0ce8,
    f: ee::s07::Replay_EndHighlight,
};

/// `Replay_CheckInstantReplay` (0x2705b0).
pub const OC331_REPLAY_CHECK_INSTANT_REPLAY: Callee = Callee {
    addr: 0x0027_05b0,
    f: ee::s07::Replay_CheckInstantReplay,
};

/// `void Replay_OnRiderWipeout(Replay* self, int key, void* data)`
/// (0x2708f0): when the replay is idle (none of `+0x61c`, `+0x620`, `+0`
/// set), find the rider's index and, when found, close its highlight
/// (`Replay_EndHighlight(self, index, data, 0)`) and run the wipeout
/// instant-replay check (`Replay_CheckInstantReplay(self, data, 0)`).
pub fn replay_on_rider_wipeout(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let selfp = c.g[A0][0];
        c.g[S0][0] = selfp;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).off(OC331_REPLAY_WIPEOUT_BUSY[0]).lw(c);
        c.g[S1][0] = c.g[A2][0];
        // The first flag set, or the full path: one more cycle at the tail.
        let mut tail = c.g[V0][0] != 0;
        if !tail {
            let mut busy = false;
            for off in OC331_REPLAY_WIPEOUT_BUSY[1..].iter().copied() {
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
                OC331_REPLAY_FIND_RIDER_INDEX.call(c, 0x0027_0934)?;
                let found = (c.g[V0][0] as i64) >= 0;
                c.g[A1][0] = c.g[V0][0];
                if found {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = 0;
                    OC331_REPLAY_END_HIGHLIGHT.call(c, 0x0027_094c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = 0;
                    OC331_REPLAY_CHECK_INSTANT_REPLAY.call(c, 0x0027_095c)?;
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

/// The bound box written by `WorldObj_GetBoundBox` (8 floats: box, then the
/// radius at `+0x20`).
pub const MODOBJ_V48_BOX: u32 = 0x0;

// ---------------------------------------------------------------------------
// Replay_BeginHighlight (0x270b88)

/// One replay highlight slot: 0xb4 bytes, `state` at +0xb0.
pub(crate) const REPLAY_SLOT_STRIDE: u64 = 0xb4;

/// The slot array base relative to the cache (+0x494); the busy word is at
/// `slot + 0xb0` and the rider at +0x3b4.
pub(crate) const REPLAY_SLOTS: u64 = 0x494;

/// The rider handle the highlight copies from.
pub(crate) const REPLAY_RIDER: u64 = 0x3b4;

/// `Replay_HighlightSlotBegin` (0x26e6e0), same shard.
pub(crate) const HIGHLIGHT_SLOT_BEGIN: Callee = Callee {
    addr: 0x0026_e6e0,
    f: ee::s07::Replay_HighlightSlotBegin,
};

/// `void Replay_BeginHighlight(ReplayCache* self, int slot, int id)`
/// (0x270b88): start the replay highlight in `slot` unless the cache is
/// already recording (`+0`) or the slot is still busy (+0xb0).
pub fn replay_begin_highlight(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A3)).lw(c);
        let recording = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !recording {
            c.cycles += 6;
            c.g[A0][0] = REPLAY_SLOT_STRIDE;
            c.mult(0, V1, A1, A0);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x544)).lw(c);
            let busy = c.g[V1][0] != 0;
            if !busy {
                c.cycles += 6;
                c.mult(0, A0, A1, A0);
                c.g[A1][0] = Addr(r32(c, A3).wrapping_add(REPLAY_RIDER as u32)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(REPLAY_SLOTS as u32));
                c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                HIGHLIGHT_SLOT_BEGIN.call(c, 0x0027_0bcc)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InSeq_RampTrackUpdate (0x273698)

pub(crate) const RAMP_TRACK_ACTIVATE_KEY: Callee = Callee {
    addr: 0x0027_3860,
    f: ee::s07::InSeq_RampTrackActivateKey,
};

pub(crate) const RAMP_TRACK_NEXT_KEY: Callee = Callee {
    addr: 0x0027_38f8,
    f: ee::s07::InSeq_RampTrackNextKey,
};

/// Ramp track fields: the next key record, the active ramps (a list linked
/// at +0x20: end time int at +8, value at +0x10, speed at +0x14) and their
/// count.
pub(crate) const RAMP_TRACK_KEY: u32 = 0x14;

pub(crate) const RAMP_TRACK_ACTIVE: u32 = 0x20;

pub(crate) const RAMP_TRACK_COUNT: u32 = 0x24;

/// `bool InSeq_RampTrackUpdate(RampTrack* self, uint* flags, float t, float
/// dt)` (0x273698): advance the track to time `t`. Ramps that ended by `t`
/// are unlinked (flags |= 2); the others move by `speed * dt`. Then every key
/// due by `t` starts its ramp (flags |= 1), advanced by the time since the
/// key. Returns whether anything is left (active ramps or keys).
pub fn in_seq_ramp_track_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x38);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.mov_s(21, 12);
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 20, 0x30);
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = c.lw(this.wrapping_add(RAMP_TRACK_ACTIVE));
        c.g[A0][0] = 0; // the previous ramp
        if c.g[V1][0] != 0 {
            c.cycles += 1;
            loop {
                let e = c.g[V1][0] as u32;
                c.cycles += 6;
                c.lwc1(0, e.wrapping_add(0x8));
                c.cvt_s_w(0, 0);
                c.c_le_s(0, 21);
                if !c.fcc() {
                    // Still running: value += speed * dt.
                    c.lwc1(1, e.wrapping_add(0x14));
                    c.cycles += 6;
                    c.g[A0][0] = c.g[V1][0];
                    c.lwc1(0, e.wrapping_add(0x10));
                    c.mul_s(1, 1, 13);
                    c.add_s(0, 0, 1);
                    c.swc1(0, e.wrapping_add(0x10));
                    c.g[V1][0] = c.lw(e.wrapping_add(0x20));
                } else {
                    // Ended: unlink it.
                    c.cycles += 4;
                    let s1 = c.g[S1][0] as u32;
                    c.g[V0][0] = c.lw(s1) | 0x2;
                    let has_prev = c.g[A0][0] != 0;
                    c.sw(s1, c.g[V0][0]);
                    if has_prev {
                        c.cycles += 2;
                        c.g[V1][0] = c.lw(e.wrapping_add(0x20));
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(e.wrapping_add(0x20));
                        c.g[V1][0] = c.g[V0][0];
                        c.sw(this.wrapping_add(RAMP_TRACK_ACTIVE), c.g[V0][0]);
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(this.wrapping_add(RAMP_TRACK_COUNT));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.sw(this.wrapping_add(RAMP_TRACK_COUNT), c.g[V0][0]);
                }
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_36d0);
                }
            }
        }
        // Start the keys that are due.
        c.cycles += 2;
        c.mtc1(0, 22);
        c.g[V1][0] = c.lw(this.wrapping_add(RAMP_TRACK_KEY));
        loop {
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RAMP_TRACK_ACTIVE));
                break;
            }
            c.cycles += 8;
            c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.sub_s(20, 21, 0);
            c.c_le_s(22, 20);
            let due = c.fcc();
            c.g[A0][0] = c.g[S0][0];
            if !due {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RAMP_TRACK_ACTIVE));
                break;
            }
            c.cycles += 14;
            let s1 = c.g[S1][0] as u32;
            c.g[V0][0] = c.lw(s1) | 0x1;
            c.sw(s1, c.g[V0][0]);
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RAMP_TRACK_KEY));
            RAMP_TRACK_ACTIVATE_KEY.call(c, 0x0027_3784)?;
            let ramp = c.g[V0][0] as u32;
            c.lwc1(0, ramp.wrapping_add(0x14));
            c.g[A0][0] = c.g[S0][0];
            c.lwc1(1, ramp.wrapping_add(0x10));
            c.mul_s(0, 20, 0);
            c.add_s(1, 1, 0);
            c.swc1(1, ramp.wrapping_add(0x10));
            RAMP_TRACK_NEXT_KEY.call(c, 0x0027_37a0)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RAMP_TRACK_KEY));
            if event_due(c) {
                deliver_event_here(c, 0x0027_3748);
            }
        }
        c.cycles += 2;
        let active = c.g[V0][0] != 0;
        c.g[V0][0] = 0x1;
        if !active {
            c.cycles += 2;
            let no_key = c.g[V1][0] == 0;
            c.g[V0][0] = 0;
            if !no_key {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ InSeq_ChannelUpdateTracks

/// `bool InSeq_ChannelUpdateTracks(InSeqChannel *self, float frame, float dt)`
/// (0x2743e8): update every track player of the channel (virtual call at
/// vtable +0x1c with the frame, dt and an output flag word), OR the
/// flags together and tell the entity communicator (`self+0x2c`), if any,
/// about them. Returns whether any track reported progress.
pub fn in_seq_channel_update_tracks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0;
        frame.save_fpr(c, 21, 0x68);
        c.g[S3][0] = 0;
        frame.save_fpr(c, 20, 0x60);
        c.mov_s(21, 13);
        frame.save_ra(c, 0x10);
        c.g[V1][0] = o406_lw(c, S0, 0x1c);
        let a = (c.g[V1][0] as u32).wrapping_add(2);
        c.g[V0][0] = c.lbu(a);
        c.mov_s(20, 12);
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            loop {
                c.cycles += 22;
                c.g[V1][0] = o406_lw(c, S0, 0x24);
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                let a = frame.at(c, 0);
                c.sw(a, 0);
                c.mov_s(12, 20);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.mov_s(13, 21);
                c.g[A2][0] = o406_lw(c, V0, 0);
                c.g[A1][0] = c.g[SP][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V1][0] = o406_lw(c, A2, 4);
                let a = (c.g[V1][0] as u32).wrapping_add(0x18);
                c.g[A0][0] = c.lh(a);
                c.g[V0][0] = o406_lw(c, V1, 0x1c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_4468)?;
                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                c.g[A0][0] = o406_lw(c, S0, 0x1c);
                c.g[S3][0] |= c.g[V0][0];
                c.g[V1][0] = c.lw(frame.at(c, 0));
                let a = (c.g[A0][0] as u32).wrapping_add(2);
                c.g[V0][0] = c.lbu(a);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let again = c.g[V0][0] != 0;
                c.g[S2][0] |= c.g[V1][0];
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_4430);
                }
            }
        }
        c.cycles += 3;
        c.g[T1][0] = o406_lw(c, S0, 0x2c);
        c.g[V0][0] = c.g[S3][0];
        if c.g[T1][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            if c.g[S2][0] != 0 {
                c.cycles += 9;
                c.g[V1][0] = o406_lw(c, T1, 0);
                c.g[T0][0] = o406_lw(c, S0, 0x1c);
                c.mov_s(12, 20);
                let a = (c.g[V1][0] as u32).wrapping_add(0x18);
                c.g[A0][0] = c.lh(a);
                c.g[A3][0] = c.g[S2][0];
                c.g[V0][0] = o406_lw(c, V1, 0x1c);
                let a = (c.g[T0][0] as u32).wrapping_add(3);
                c.g[A2][0] = c.lbu(a);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_44c0)?;
            }
            c.cycles += 13;
            c.g[T0][0] = o406_lw(c, S0, 0x2c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = o406_lw(c, S0, 0x1c);
            c.mov_s(12, 20);
            c.g[V1][0] = o406_lw(c, T0, 0);
            c.mov_s(13, 21);
            let a = (c.g[V0][0] as u32).wrapping_add(3);
            c.g[A2][0] = c.lbu(a);
            c.g[A3][0] = c.g[S2][0];
            let a = (c.g[V1][0] as u32).wrapping_add(0x20);
            c.g[A0][0] = c.lh(a);
            c.g[V0][0] = o406_lw(c, V1, 0x24);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_44f0)?;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- InSeq_EventTrackUpdate

/// `bool InSeq_EventTrackUpdate(EventTrack *self, float time, u32 *flags)`
/// (0x273330): fire every key (`self+0x14`, a key with its time in the
/// half-word at +2) that `time` has reached: set bit 0x10 in `*flags`,
/// fire the key and advance to the next one. Returns whether keys remain.
pub fn in_seq_event_track_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x38);
        frame.save_ra(c, 0);
        c.mov_s(21, 12);
        frame.save_fpr(c, 20, 0x30);
        let a = (c.g[S0][0] as u32).wrapping_add(0x24);
        c.lwc1(0, a);
        c.c_eq_s(0, 21);
        let same_time = c.fcc();
        c.g[S1][0] = c.g[A1][0];
        if !same_time {
            c.cycles += 2;
            o406_sw(c, S0, 0x20, ZERO);
        }
        loop {
            c.cycles += 3;
            c.g[V0][0] = o406_lw(c, S0, 0x14);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x20);
                break;
            }
            c.cycles += 7;
            let a = (c.g[V0][0] as u32).wrapping_add(2);
            c.g[V0][0] = c.lhu(a);
            c.mtc1(V0, 20);
            c.cvt_s_w(20, 20);
            c.c_lt_s(21, 20);
            if !c.fcc() {
                c.g[V0][0] = o406_lw(c, S1, 0);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_3370);
                }
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[V0][0] |= 0x10;
                o406_sw(c, S1, 0, V0);
                c.g[A1][0] = o406_lw(c, S0, 0x14);
                call_ptr(c, 0x0027_3418, 0x0027_3384)?;
                let a = (c.g[S0][0] as u32).wrapping_add(0x24);
                c.swc1(20, a);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0027_3468, 0x0027_3390)?;
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = o406_lw(c, S0, 0x14);
            frame.restore(c, S0, 0x20);
            break;
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ----------------------------------------------------- InSeq_ChannelAdvance

/// `bool InSeq_ChannelAdvance(InSeqChannel *self, float frame, float dt,
/// ..., int step)` (0x274100): advance the channel by `dt`. With a
/// non-zero `step` the time is first cut to the nearest key time any track
/// player (vtable +0x34, returns a frame count) says is ahead, and
/// `UpdateTracks` runs until the whole time is used or a call reports no
/// progress. Returns 1 when the channel is finished (`+0x28 == 0`), else
/// the last `UpdateTracks` result.
pub fn in_seq_channel_advance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x10);
        frame.save_fpr(c, 22, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x38);
        c.mov_s(22, 13);
        frame.save(c, S0, 0x20);
        c.mov_s(21, 12);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 23, 0x48);
        frame.save_fpr(c, 20, 0x30);
        c.g[V0][0] = o406_lw(c, S1, 0x28);
        let running = c.g[V0][0] != 0;
        c.mov_s(13, 14);
        'end: {
            if !running {
                c.cycles += 2;
                c.g[V0][0] = 1;
                break 'end;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            let to_last_update = if c.g[A1][0] == 0 {
                // 0x274200.
                c.cycles += 1;
                c.mov_s(12, 22);
                true
            } else {
                c.cycles += 1;
                c.g[V1][0] = o406_lw(c, S1, 0x1c);
                'step: loop {
                    // 0x274158: the smallest positive key distance of any track.
                    c.cycles += 4;
                    c.mov_s(20, 22);
                    let a = (c.g[V1][0] as u32).wrapping_add(2);
                    c.g[V0][0] = c.lbu(a);
                    c.g[S0][0] = 0;
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.mtc1(0, 23);
                        c.g[V1][0] = o406_lw(c, S1, 0x24);
                        loop {
                            c.cycles += 14;
                            c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[A0][0] = o406_lw(c, V0, 0);
                            c.g[V1][0] = o406_lw(c, A0, 4);
                            let a = (c.g[V1][0] as u32).wrapping_add(0x30);
                            c.g[A1][0] = c.lh(a);
                            c.g[V0][0] = o406_lw(c, V1, 0x34);
                            let target = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            call_ptr(c, target, 0x0027_4190)?;
                            c.mtc1(2, 0);
                            c.cvt_s_w(0, 0);
                            c.c_le_s(23, 0);
                            if !c.fcc() {
                                c.g[V1][0] = o406_lw(c, S1, 0x1c);
                            } else {
                                c.cycles += 4;
                                c.c_lt_s(0, 20);
                                if c.fcc() {
                                    c.mov_s(20, 0);
                                }
                                c.cycles += 1;
                                c.g[V1][0] = o406_lw(c, S1, 0x1c);
                            }
                            c.cycles += 5;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                            let a = (c.g[V1][0] as u32).wrapping_add(2);
                            c.g[V0][0] = c.lbu(a);
                            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                break;
                            }
                            c.g[V1][0] = o406_lw(c, S1, 0x24);
                            if event_due(c) {
                                deliver_event_here(c, 0x0027_4170);
                            }
                        }
                    }
                    // 0x2741d0.
                    c.cycles += 4;
                    c.c_eq_s(20, 22);
                    let nothing_left = c.fcc();
                    c.g[A0][0] = c.g[S1][0];
                    if nothing_left {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_4140);
                        }
                        // 0x274140.
                        c.cycles += 3;
                        c.mov_s(12, 22);
                        c.sub_s(13, 12, 21);
                        break 'step true;
                    }
                    c.cycles += 6;
                    c.sub_s(13, 20, 21);
                    c.mov_s(21, 20);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x0027_43e8, 0x0027_41f0)?;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = o406_lw(c, S1, 0x1c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_4158);
                        }
                        continue 'step;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'step false;
                }
            };
            if to_last_update {
                c.cycles += 2;
                call_ptr(c, 0x0027_43e8, 0x0027_420c)?;
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x48);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------- InSeq_InstanceTick

/// The two per-channel loops of `InSeq_InstanceTick` that share one shape:
/// `flush` true calls `InSeq_ChannelFlushEvents` on every channel (counter
/// s1, previous result s5, 0x274fc0), false calls `InSeq_ChannelRestart`
/// (counter s4, previous result s1, 0x275078). A channel reports "progress"
/// in s2 when its call returns non-zero or the previous one did.
pub(crate) fn o406_inseq_channel_loop(c: &mut Cpu, flush: bool) -> Flow {
    // `flush`: the FlushEvents loop (counter s1, offset s3, previous s5, flag
    // s2) at 0x274fc0; otherwise the Restart loop (counter s4, offset s3,
    // previous s1, flag s2) at 0x275078.
    let (counter, prev, call, ret, head) = if flush {
        (S1, S5, 0x0027_3d20u32, 0x0027_4f90u32, 0x0027_4f80u32)
    } else {
        (S4, S1, 0x0027_3f68u32, 0x0027_5048u32, 0x0027_5038u32)
    };
    if flush {
        c.cycles += 4;
    } else {
        c.cycles += 5;
    }
    c.g[V1][0] = o406_lw(c, S0, 0x10);
    let a = (c.g[V1][0] as u32).wrapping_add(1);
    c.g[V0][0] = c.lbu(a);
    let none = c.g[V0][0] == 0;
    if !flush {
        c.g[S4][0] = 0;
    }
    c.g[S1][0] = 0;
    if none {
        return Ok(());
    }
    c.cycles += if flush { 1 } else { 2 };
    c.g[S3][0] = 0;
    loop {
        c.cycles += 6;
        c.g[A0][0] = o406_lw(c, S0, 0x18);
        c.g[S2][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
        call_ptr(c, call, ret)?;
        if c.g[V0][0] != 0 {
            c.g[S2][0] = 1;
            c.cycles += 1;
            c.g[V1][0] = o406_lw(c, S0, 0x10);
        } else {
            c.cycles += 2;
            if c.g[prev][0] == 0 {
                c.g[V1][0] = o406_lw(c, S0, 0x10);
            } else {
                c.cycles += 1;
                c.g[S2][0] = 1;
                c.cycles += 1;
                c.g[V1][0] = o406_lw(c, S0, 0x10);
            }
        }
        c.cycles += 6;
        c.g[counter][0] = sx((c.g[counter][0] as u32).wrapping_add(1));
        c.g[prev][0] = c.g[S2][0];
        let a = (c.g[V1][0] as u32).wrapping_add(1);
        c.g[V0][0] = c.lbu(a);
        c.g[V0][0] = ((c.g[counter][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let again = c.g[V0][0] != 0;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3c));
        if !again {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// Call `listener->vtable[off]` (`lh +off` this adjust, `lw +off+4`
/// function) with `a1 = inst` (the instance), through the listener `s4`:
/// the notification the tick sends on a state change.
pub(crate) fn o406_inseq_notify(c: &mut Cpu, voff: u32, ret: u32) -> Flow {
    c.g[V0][0] = o406_lw(c, S4, 0);
    c.g[A1][0] = c.g[S0][0];
    let a = (c.g[V0][0] as u32).wrapping_add(voff);
    c.g[A0][0] = c.lh(a);
    c.g[V1][0] = o406_lw(c, V0, voff + 4);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `bool InSeq_InstanceTick(InSeqInstance *self, ...)` (0x274de0): advance a
/// playing sequence instance (state at +0x20: 0 idle, 1 starting, 2
/// playing, 3 stopping, 4 finished). Tells the listener about the start,
/// runs `InstanceAdvance` over the time left (state 3 and 4 clamp to the
/// length), starts / flushes / restarts the channels, loops the time and
/// finally stops the instance or reports the end to the listener.
pub fn in_seq_instance_tick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        c.g[V0][0] = o406_lw(c, S0, 0x20);
        'exit: {
            if c.g[V0][0] == 0 {
                break 'exit;
            }
            c.cycles += 3;
            c.g[V0][0] = o406_lw(c, S0, 0x2c);
            if (c.g[V0][0] as i64) > 0 {
                break 'exit;
            }
            c.cycles += 7;
            call_ptr(c, 0x0027_4d70, 0x0027_4e30)?;
            c.g[S4][0] = c.g[V0][0];
            c.g[V1][0] = o406_lw(c, S0, 0x20);
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = o406_lw(c, S0, 0x2c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                o406_sw(c, S0, 0x20, V0);
                if c.g[S4][0] != 0 {
                    c.cycles += 6;
                    o406_inseq_notify(c, 8, 0x0027_4e68)?;
                }
                c.cycles += 1;
                c.g[V0][0] = o406_lw(c, S0, 0x2c);
            }
            c.cycles += 2;
            if (c.g[V0][0] as i64) > 0 {
                break 'exit;
            }
            c.cycles += 3;
            c.g[V0][0] = o406_lw(c, S0, 0x20);
            if c.g[V0][0] == 0 {
                break 'exit;
            }
            'stop: {
                'to_j: {
                    'g: {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[S5][0] = 0;
                        call_ptr(c, 0x0027_4c70, 0x0027_4e8c)?;
                        c.mtc1(V0, 20);
                        c.cvt_s_w(20, 20);
                        c.g[V1][0] = o406_lw(c, S0, 0x20);
                        c.mtc1(0, 21);
                        c.g[V0][0] = 4;
                        c.g[S6][0] = 0;
                        if c.g[V1][0] == 4 {
                            c.cycles += 4;
                            let a = (c.g[S0][0] as u32).wrapping_add(0x28);
                            c.lwc1(0, a);
                            c.c_le_s(20, 0);
                            if c.fcc() {
                                break 'stop;
                            }
                        }
                        break 'g;
                    }
                    'finish: {
                        'check: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(3));
                            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                            let other_state = c.g[V0][0] == 0;
                            c.g[V0][0] = 4;
                            if !other_state {
                                // State 3 or 4: advance to the end time.
                                c.cycles += 5;
                                let a = (c.g[S0][0] as u32).wrapping_add(0x24);
                                c.lwc1(14, a);
                                let a = (c.g[S0][0] as u32).wrapping_add(0x28);
                                c.lwc1(13, a);
                                c.mov_s(0, 14);
                                let not_four = c.g[V1][0] != c.g[V0][0];
                                c.add_s(12, 13, 14);
                                if !not_four {
                                    c.cycles += 1;
                                    c.mov_s(12, 20);
                                }
                                c.cycles += 4;
                                c.c_le_s(20, 12);
                                let within = !c.fcc();
                                c.g[V0][0] = 1;
                                if !within {
                                    c.cycles += 4;
                                    c.sub_s(14, 12, 13);
                                    c.g[S6][0] = 1;
                                    c.mov_s(12, 20);
                                    c.sub_s(21, 0, 14);
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                o406_sw(c, S0, 0x3c, V0);
                                call_ptr(c, 0x0027_51a0, 0x0027_4f0c)?;
                                o406_sw(c, S0, 0x3c, ZERO);
                                break 'finish;
                            }
                            break 'check;
                        }
                        // 0x274f28: any other state starts / flushes the channels.
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_eq_s(20, 0);
                        if c.fcc() {
                            c.g[S6][0] = 1;
                        }
                        c.cycles += 6;
                        c.g[V0][0] = 1;
                        c.g[V1][0] = o406_lw(c, S0, 0x10);
                        o406_sw(c, S0, 0x3c, V0);
                        let a = (c.g[V1][0] as u32).wrapping_add(1);
                        c.g[V0][0] = c.lbu(a);
                        let none = c.g[V0][0] == 0;
                        c.g[S1][0] = 0;
                        if !none {
                            c.cycles += 2;
                            c.g[S2][0] = 0;
                            loop {
                                c.cycles += 9;
                                c.g[A0][0] = o406_lw(c, S0, 0x18);
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                call_ptr(c, 0x0027_3aa8, 0x0027_4f58)?;
                                c.g[V1][0] = o406_lw(c, S0, 0x10);
                                let a = (c.g[V1][0] as u32).wrapping_add(1);
                                c.g[V0][0] = c.lbu(a);
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let again = c.g[V0][0] != 0;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3c));
                                if !again {
                                    break;
                                }
                                if event_due(c) {
                                    deliver_event_here(c, 0x0027_4f48);
                                }
                            }
                        }
                        o406_inseq_channel_loop(c, true)?;
                        c.cycles += 4;
                        c.g[V0][0] = 3;
                        o406_sw(c, S0, 0x3c, ZERO);
                        let no_listener = c.g[S4][0] == 0;
                        o406_sw(c, S0, 0x20, V0);
                        if !no_listener {
                            c.cycles += 8;
                            c.g[V1][0] = o406_lw(c, S4, 0);
                            c.g[A1][0] = c.g[S0][0];
                            c.mtc1(0, 12);
                            let a = (c.g[V1][0] as u32).wrapping_add(0x10);
                            c.g[A0][0] = c.lh(a);
                            c.g[V0][0] = o406_lw(c, V1, 0x14);
                            c.mov_s(13, 12);
                            let target = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x0027_4ff0)?;
                        }
                    }
                    // 0x274ff0.
                    c.cycles += 3;
                    c.g[V0][0] = o406_lw(c, S0, 0x40);
                    let has_flag = c.g[V0][0] != 0;
                    c.g[V0][0] = 4;
                    if has_flag {
                        break 'stop;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = o406_lw(c, S0, 0x20);
                    if c.g[V1][0] == c.g[V0][0] {
                        break 'stop;
                    }
                    c.cycles += 2;
                    if c.g[S6][0] == 0 {
                        let a = (c.g[S0][0] as u32).wrapping_add(0x28);
                        c.lwc1(0, a);
                        break 'to_j;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = o406_lw(c, S0, 0x1c);
                    if c.g[V0][0] == 0 {
                        let a = (c.g[S0][0] as u32).wrapping_add(0x28);
                        c.lwc1(0, a);
                        break 'to_j;
                    }
                    // Loop the sequence: restart the channels and wrap the time.
                    o406_inseq_channel_loop(c, false)?;
                    c.cycles += 4;
                    c.c_lt_s(20, 21);
                    let skip = !c.fcc();
                    o406_sw(c, S0, 0x28, ZERO);
                    if !skip {
                        c.cycles += 2;
                        c.sub_s(21, 21, 20);
                        loop {
                            c.cycles += 5;
                            c.c_lt_s(20, 21);
                            if !c.fcc() {
                                break;
                            }
                            c.sub_s(21, 21, 20);
                            if event_due(c) {
                                deliver_event_here(c, 0x0027_5090);
                            }
                        }
                    }
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_eq_s(21, 0);
                    if c.fcc() {
                        c.g[V0][0] = o406_lw(c, S0, 0x40);
                    } else {
                        c.cycles += 9;
                        let a = (c.g[S0][0] as u32).wrapping_add(0x28);
                        c.lwc1(13, a);
                        c.g[V0][0] = 1;
                        o406_sw(c, S0, 0x3c, V0);
                        c.mov_s(14, 21);
                        c.add_s(12, 13, 21);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0027_51a0, 0x0027_50d4)?;
                        o406_sw(c, S0, 0x3c, ZERO);
                        c.g[V0][0] = o406_lw(c, S0, 0x40);
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        break 'exit;
                    }
                    break 'stop;
                }
                // 0x2750f4: the time is within the length: end, or keep going.
                c.cycles += 6;
                let a = (c.g[S0][0] as u32).wrapping_add(0x24);
                c.lwc1(1, a);
                c.add_s(0, 0, 1);
                c.c_le_s(20, 0);
                if !c.fcc() {
                    break 'exit;
                }
                c.cycles += 3;
                c.g[V1][0] = o406_lw(c, S0, 0x1c);
                let looping = c.g[V1][0] == 0;
                c.g[V0][0] = 4;
                if !looping {
                    c.cycles += 2;
                    let none = c.g[S4][0] == 0;
                    c.g[A1][0] = c.g[S0][0];
                    if !none {
                        c.cycles += 6;
                        c.g[V1][0] = o406_lw(c, S4, 0);
                        let a = (c.g[V1][0] as u32).wrapping_add(0x28);
                        c.g[A0][0] = c.lh(a);
                        c.g[V0][0] = o406_lw(c, V1, 0x2c);
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0027_5134)?;
                        c.g[V1][0] = c.g[V0][0];
                    }
                    c.cycles += 3;
                    c.g[V0][0] = o406_lw(c, S0, 0x20);
                    if c.g[V0][0] == 0 {
                        break 'exit;
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != 0 {
                        break 'exit;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 4;
                }
                // 0x275150: the sequence has ended: state 4 and tell the listener.
                c.cycles += 2;
                let none = c.g[S4][0] == 0;
                o406_sw(c, S0, 0x20, V0);
                if !none {
                    c.cycles += 6;
                    o406_inseq_notify(c, 0x18, 0x0027_5170)?;
                }
                c.cycles += 1;
                break 'exit;
            }
            // 0x2750e4: stop the instance.
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0027_4918, 0x0027_50ec)?;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InSeq_SpanTrackUpdate (0x00272f28)

/// `InSeq_SpanTrackActivateKey` (0x002730c0).
pub(crate) const IN_SEQ_SPAN_TRACK_ACTIVATE_KEY_CALL: Callee = Callee {
    addr: 0x0027_30c0,
    f: ee::s07::InSeq_SpanTrackActivateKey,
};

/// `InSeq_SpanTrackNextKey` (0x00273140).
pub(crate) const IN_SEQ_SPAN_TRACK_NEXT_KEY_CALL: Callee = Callee {
    addr: 0x0027_3140,
    f: ee::s07::InSeq_SpanTrackNextKey,
};

/// `int InSeq::SpanTrackUpdate(this, flags*, time)` (0x272f28): retire the expired cached events (`end <= time`: unlink, flag 8, decrement the count at +0x28), then activate and advance every key record whose time has come (flag 4) until the next one lies in the future. Returns 1 while events or a pending record remain, else 0. `time` is passed in f12.
pub fn in_seq_span_track_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        'b_272fb4: {
            c.cycles += 12;
            Frame::open(c, 0x40);
            frame.save(c, S0, 0x20);
            frame.save(c, S1, 0x10);
            c.g[S0][0] = c.g[A0][0];
            frame.save_fpr(c, 20, 0x30);
            c.g[S1][0] = c.g[A1][0];
            frame.save_ra(c, 0);
            c.mov_s(20, 12);
            frame.save_fpr(c, 21, 0x38);
            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x24));
            let t = c.g[V1][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_272fb4;
            } else {
                'l_272f58: loop {
                    'b_272fac: {
                        'b_272fa8: {
                            c.cycles += 6;
                            c.lwc1(0, r32(c, V1).wrapping_add(0xc));
                            c.cvt_s_w(0, 0);
                            c.c_le_s(0, 20);
                            if !c.fcc() {
                                c.g[A0][0] = c.g[V1][0];
                                break 'b_272fa8;
                            } else {
                                'b_272f98: {
                                    'b_272f90: {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw(r32(c, S1));
                                        c.g[V0][0] = c.g[V0][0] | 0x8;
                                        let t = c.g[A0][0] != 0u64;
                                        c.sw(r32(c, S1), c.g[V0][0]);
                                        if t {
                                            break 'b_272f90;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x18));
                                            c.g[V1][0] = c.g[V0][0];
                                            let t = true;
                                            c.sw(r32(c, S0).wrapping_add(0x24), c.g[V0][0]);
                                            if t {
                                                break 'b_272f98;
                                            } else {
                                                break 'b_272f90;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[V1][0] = c.lw(r32(c, V1).wrapping_add(0x18));
                                    c.sw(r32(c, A0).wrapping_add(0x18), c.g[V1][0]);
                                    break 'b_272f98;
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x28));
                                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                                let t = true;
                                c.sw(r32(c, S0).wrapping_add(0x28), c.g[V0][0]);
                                if t {
                                    break 'b_272fac;
                                } else {
                                    break 'b_272fa8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(r32(c, V1).wrapping_add(0x18));
                        break 'b_272fac;
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] != 0u64;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_2f58);
                        }
                        continue 'l_272f58;
                    } else {
                        break 'b_272fb4;
                    }
                }
            }
        }
        c.cycles += 3;
        c.mtc1(0, 21);
        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x14));
        'l_272fc0: loop {
            'b_273010: {
                c.cycles += 2;
                if c.g[V1][0] == 0u64 {
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x24));
                    break 'b_273010;
                } else {
                    'b_27300c: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lhu(r32(c, V1).wrapping_add(0x4));
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        c.sub_s(0, 20, 0);
                        c.c_le_s(21, 0);
                        let t = !c.fcc();
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_27300c;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw(r32(c, S1));
                            c.g[V0][0] = c.g[V0][0] | 0x4;
                            c.sw(r32(c, S1), c.g[V0][0]);
                            c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x14));
                            IN_SEQ_SPAN_TRACK_ACTIVATE_KEY_CALL.call(c, 0x0027_2ffc)?;
                            c.g[A0][0] = c.g[S0][0];
                            IN_SEQ_SPAN_TRACK_NEXT_KEY_CALL.call(c, 0x0027_3004)?;
                            let t = true;
                            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x14));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0027_2fc0);
                                }
                                continue 'l_272fc0;
                            } else {
                                break 'b_27300c;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x24));
                    break 'b_273010;
                }
            }
            'b_273024: {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_273024;
                } else {
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_273024;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0x1;
                        break 'b_273024;
                    }
                }
            }
            c.cycles += 7;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.restore_fpr(c, 21, 0x38);
            frame.restore_fpr(c, 20, 0x30);
            frame.close(c);
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// InSeq_InstanceAdvance (0x002751a0)

/// `InSeq_ChannelAdvance` (0x00274100).
pub(crate) const IN_SEQ_CHANNEL_ADVANCE_CALL: Callee = Callee {
    addr: 0x0027_4100,
    f: ee::s07::InSeq_ChannelAdvance,
};

/// `InSeq_InstanceGetListener` (0x00274d70).
pub(crate) const IN_SEQ_INSTANCE_GET_LISTENER_CALL: Callee = Callee {
    addr: 0x0027_4d70,
    f: ee::s07::InSeq_InstanceGetListener,
};

/// `int InSeq::InstanceAdvance(this, time, prev, speed)` (0x2751a0): store the new time (f12, at +0x28), decide whether the play head jumped (speed above 1 or the integer part of the time changed), advance every channel (0x3c bytes each), and notify the listener (vtable +0x14) of the new time and speed. Returns 1 when any channel changed.
pub fn in_seq_instance_advance(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x80 };
        'b_27522c: {
            'b_275228: {
                'b_2751f8: {
                    c.cycles += 20;
                    Frame::open(c, 0x80);
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    frame.save_fpr(c, 21, 0x78);
                    c.mov_s(21, 14);
                    frame.save(c, S0, 0x60);
                    c.c_lt_s(0, 21);
                    frame.save(c, S4, 0x20);
                    frame.save(c, S5, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_fpr(c, 20, 0x70);
                    c.g[S5][0] = 0u64;
                    frame.save(c, S1, 0x50);
                    c.mov_s(20, 13);
                    frame.save(c, S2, 0x40);
                    c.g[S4][0] = 0u64;
                    frame.save(c, S3, 0x30);
                    frame.save_ra(c, 0);
                    let t = !c.fcc();
                    c.swc1(12, r32(c, S0).wrapping_add(0x28));
                    if t {
                        break 'b_2751f8;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[S4][0] = 0x1;
                        if t {
                            break 'b_275228;
                        } else {
                            break 'b_2751f8;
                        }
                    }
                }
                c.cycles += 10;
                c.cvt_w_s(0, 12);
                c.mfc1(2, 0);
                c.cvt_w_s(0, 20);
                c.mfc1(3, 0);
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.c_eq_s(0, 12);
                if c.fcc() {
                    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x10));
                    break 'b_27522c;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0] ^ c.g[V0][0];
                    c.g[S4][0] = (0u64 < c.g[V0][0]) as u64;
                    break 'b_275228;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x10));
            break 'b_27522c;
        }
        'b_275290: {
            c.cycles += 3;
            c.g[V0][0] = c.lbu(r32(c, V1).wrapping_add(0x1));
            let t = c.g[V0][0] == 0u64;
            c.g[S1][0] = 0u64;
            if t {
                break 'b_275290;
            } else {
                c.cycles += 2;
                c.g[S3][0] = 0u64;
                'l_275240: loop {
                    'b_275278: {
                        'b_275274: {
                            c.cycles += 10;
                            c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x18));
                            c.g[S2][0] = 0u64;
                            c.lwc1(13, r32(c, S0).wrapping_add(0x28));
                            c.mov_s(12, 20);
                            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
                            c.mov_s(14, 21);
                            c.g[A1][0] = c.g[S4][0];
                            IN_SEQ_CHANNEL_ADVANCE_CALL.call(c, 0x0027_5260)?;
                            if c.g[V0][0] != 0u64 {
                                c.g[S2][0] = 0x1;
                                break 'b_275274;
                            } else {
                                c.cycles += 2;
                                if c.g[S5][0] == 0u64 {
                                    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x10));
                                    break 'b_275278;
                                } else {
                                    c.cycles += 1;
                                    c.g[S2][0] = 0x1;
                                    break 'b_275274;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x10));
                        break 'b_275278;
                    }
                    c.cycles += 6;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x1));
                    c.g[S5][0] = c.g[S2][0];
                    c.g[V0][0] = c.lbu(r32(c, V1).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x3c));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_5240);
                        }
                        continue 'l_275240;
                    } else {
                        break 'b_275290;
                    }
                }
            }
        }
        'b_2752c0: {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            IN_SEQ_INSTANCE_GET_LISTENER_CALL.call(c, 0x0027_5298)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S0][0];
            if t {
                break 'b_2752c0;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw(r32(c, A2));
                c.lwc1(12, r32(c, A1).wrapping_add(0x28));
                c.mov_s(13, 21);
                c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x10));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x14));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0027_52c0)?;
                break 'b_2752c0;
            }
        }
        c.cycles += 12;
        c.g[V0][0] = c.g[S5][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `void* InSeq_HeapAlloc(Heap* heap, uint size)` (0x272308): first-fit
/// allocation from the free list at +8 (size rounded up to 4; a block is
/// split when it leaves 16 bytes more); returns the block's data or null.
pub fn in_seq_heap_alloc(c: &mut Cpu) {
    'b_27239c: {
        c.cycles += 10;
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[T1][0] = c.g[A0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3));
        c.g[V0][0] = c.g[V0][0] | 0xfffc;
        c.g[A1][0] = c.g[A1][0] & c.g[V0][0];
        c.g[A2][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8));
        c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[A3][0] = 0u64;
        let t = c.g[A2][0] == 0u64;
        c.g[A0][0] = 0u64;
        if t {
            break 'b_27239c;
        } else {
            c.cycles += 2;
            c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
            'l_272338: loop {
                'b_272390: {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V1][0] < c.g[A1][0]) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A3][0] = c.g[A2][0];
                        break 'b_272390;
                    } else {
                        'b_27236c: {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[T2][0] < c.g[V1][0]) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                            if t {
                                break 'b_27236c;
                            } else {
                                c.cycles += 7;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
                                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                                c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                                c.sw(c.g[A2][0] as u32, c.g[A1][0]);
                                break 'b_27236c;
                            }
                        }
                        'b_272388: {
                            'b_272380: {
                                c.cycles += 2;
                                let t = c.g[A3][0] != 0u64;
                                c.g[A0][0] = c.g[A2][0];
                                if t {
                                    break 'b_272380;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                    let t = true;
                                    c.sw((c.g[T1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                    if t {
                                        break 'b_272388;
                                    } else {
                                        break 'b_272380;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                            c.sw((c.g[A3][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            break 'b_272388;
                        }
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[A2][0] as u32).wrapping_add(0x4), 0u64);
                        if t {
                            break 'b_27239c;
                        } else {
                            break 'b_272390;
                        }
                    }
                }
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                if c.g[A2][0] != 0u64 {
                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x00272338);
                    }
                    continue 'l_272338;
                } else {
                    break 'b_27239c;
                }
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// InSeq_ChannelStop (0x273dc0)

/// `InSeq_HeapDelete(obj)` (0x2724e8).
pub(crate) const CHANNEL_STOP_HEAP_DELETE: Callee = Callee {
    addr: 0x0027_24e8,
    f: ee::s07::InSeq_HeapDelete,
};

/// `InSeq_HeapFree(heap, block)` (0x2723a8).
pub(crate) const CHANNEL_STOP_HEAP_FREE: Callee = Callee {
    addr: 0x0027_23a8,
    f: ee::s07::InSeq_HeapFree,
};

/// Which way `InSeq_ChannelStop` disposes of one channel player.
#[derive(Clone, Copy)]
pub(crate) enum ChannelStopCase {
    /// Kind 0: `InSeq_HeapDelete(player)`.
    Delete,
    /// Kinds 1 and 2: `player->vfunc(3)` when the player is not null.
    ReleaseA,
    /// Kind 3: the same virtual call (its own copy of the code).
    ReleaseB,
    /// Any other kind: nothing.
    Skip,
}

/// `void cInSeqChannel::stop()` (0x273dc0): when playing (`+0x28`), tell the
/// entity comm object (`+0x2c`, a vtable call with the system header's byte
/// 3), clear it, then for each of the header's channel count (byte 2) stop
/// the channel player (vtable `+0x4`, entries `0x10/0x14`: stop; the kind
/// virtual at `0x38/0x3c`: 0 deletes the player, 1-3 call its release
/// virtual with 3), free the player array from the owner's heap and clear
/// the playing flag.
pub fn in_seq_channel_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = oc417_lw(c, S1, 0x28);
        let idle = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        'stopped: {
            if idle {
                break 'stopped;
            }
            c.cycles += 3;
            c.g[T0][0] = oc417_lw(c, S1, 0x2c);
            let no_comm = c.g[T0][0] == 0;
            c.g[A1][0] = c.g[S1][0];
            if !no_comm {
                c.cycles += 7;
                c.g[V0][0] = oc417_lw(c, T0, 0);
                c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                c.g[A0][0] = oc417_lh(c, V0, 0x10);
                c.g[A3][0] = oc417_lw(c, V0, 0x14);
                c.g[A2][0] = oc417_lbu(c, V1, 3);
                let target = c.g[A3][0] as u32;
                c.g[A0][0] = oc417_add(c.g[T0][0], c.g[A0][0]);
                call_ptr(c, target, 0x0027_3e0c)?;
            }
            c.cycles += 5;
            oc417_sw0(c, S1, 0x2c);
            c.g[V1][0] = oc417_lw(c, S1, 0x1c);
            c.g[V0][0] = oc417_lbu(c, V1, 2);
            if c.g[V0][0] == 0 {
                oc417_sw0(c, S1, 0x28);
            } else {
                c.cycles += 2;
                c.g[S2][0] = 0;
                c.cycles += 2;
                c.g[V0][0] = oc417_lw(c, S1, 0x24);
                loop {
                    // Stop the player.
                    c.cycles += 20;
                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[V0][0] = oc417_add(c.g[S0][0], c.g[V0][0]);
                    c.g[A2][0] = oc417_lw(c, V0, 0);
                    c.g[V1][0] = oc417_lw(c, A2, 4);
                    c.g[A0][0] = oc417_lh(c, V1, 0x10);
                    c.g[A1][0] = oc417_lw(c, V1, 0x14);
                    let target = c.g[A1][0] as u32;
                    c.g[A0][0] = oc417_add(c.g[A2][0], c.g[A0][0]);
                    call_ptr(c, target, 0x0027_3e50)?;
                    // Ask its kind.
                    c.g[V0][0] = oc417_lw(c, S1, 0x24);
                    c.g[V0][0] = oc417_add(c.g[S0][0], c.g[V0][0]);
                    c.g[A1][0] = oc417_lw(c, V0, 0);
                    c.g[V1][0] = oc417_lw(c, A1, 4);
                    c.g[A0][0] = oc417_lh(c, V1, 0x38);
                    c.g[V0][0] = oc417_lw(c, V1, 0x3c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = oc417_add(c.g[A1][0], c.g[A0][0]);
                    call_ptr(c, target, 0x0027_3e70)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = 1;
                    let is_one = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
                    let case = if is_one {
                        ChannelStopCase::ReleaseA
                    } else {
                        c.cycles += 2;
                        let at_least_two = c.g[V0][0] == 0;
                        c.g[V0][0] = 2;
                        if !at_least_two {
                            c.cycles += 2;
                            if c.g[V1][0] == 0 {
                                c.g[V0][0] = oc417_lw(c, S1, 0x24);
                                ChannelStopCase::Delete
                            } else {
                                c.cycles += 2;
                                c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                                ChannelStopCase::Skip
                            }
                        } else {
                            c.cycles += 2;
                            let is_two = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 3;
                            if is_two {
                                ChannelStopCase::ReleaseA
                            } else {
                                c.cycles += 2;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[V0][0] = oc417_lw(c, S1, 0x24);
                                    ChannelStopCase::ReleaseB
                                } else {
                                    c.cycles += 2;
                                    c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                                    ChannelStopCase::Skip
                                }
                            }
                        }
                    };
                    match case {
                        ChannelStopCase::Delete => {
                            c.cycles += 5;
                            c.g[V0][0] = oc417_add(c.g[S0][0], c.g[V0][0]);
                            c.g[A0][0] = oc417_lw(c, V0, 0);
                            CHANNEL_STOP_HEAP_DELETE.call(c, 0x0027_3ebc)?;
                            c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                        }
                        ChannelStopCase::ReleaseA => {
                            c.cycles += 5;
                            c.g[V0][0] = oc417_lw(c, S1, 0x24);
                            c.g[V0][0] = oc417_add(c.g[S0][0], c.g[V0][0]);
                            c.g[A2][0] = oc417_lw(c, V0, 0);
                            let null = c.g[A2][0] == 0;
                            c.g[A1][0] = 3;
                            if null {
                                c.cycles += 1;
                                c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = oc417_lw(c, A2, 4);
                                c.g[A0][0] = oc417_lh(c, V0, 0x50);
                                c.g[V1][0] = oc417_lw(c, V0, 0x54);
                                let target = c.g[V1][0] as u32;
                                c.g[A0][0] = oc417_add(c.g[A2][0], c.g[A0][0]);
                                call_ptr(c, target, 0x0027_3eec)?;
                                c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                            }
                        }
                        ChannelStopCase::ReleaseB => {
                            c.cycles += 4;
                            c.g[V0][0] = oc417_add(c.g[S0][0], c.g[V0][0]);
                            c.g[A2][0] = oc417_lw(c, V0, 0);
                            let null = c.g[A2][0] == 0;
                            c.g[A1][0] = 3;
                            if !null {
                                c.cycles += 5;
                                c.g[V0][0] = oc417_lw(c, A2, 4);
                                c.g[A0][0] = oc417_lh(c, V0, 0x50);
                                c.g[V1][0] = oc417_lw(c, V0, 0x54);
                                let target = c.g[V1][0] as u32;
                                c.g[A0][0] = oc417_add(c.g[A2][0], c.g[A0][0]);
                                call_ptr(c, target, 0x0027_3f18)?;
                            }
                            c.cycles += 1;
                            c.g[V1][0] = oc417_lw(c, S1, 0x1c);
                        }
                        ChannelStopCase::Skip => {}
                    }
                    // Next player.
                    c.cycles += 5;
                    c.g[S2][0] = oc417_add(c.g[S2][0], 1);
                    c.g[V0][0] = oc417_lbu(c, V1, 2);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = oc417_lw(c, S1, 0x24);
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_3e30);
                        }
                        continue;
                    }
                    break;
                }
                // Free the player array.
                c.cycles += 7;
                c.g[V0][0] = oc417_lw(c, S1, 8);
                c.g[A1][0] = oc417_lw(c, S1, 0x24);
                c.g[A0][0] = oc417_lw(c, V0, 8);
                c.g[A0][0] = oc417_add(c.g[A0][0], 8);
                CHANNEL_STOP_HEAP_FREE.call(c, 0x0027_3f44)?;
                oc417_sw0(c, S1, 0x24);
                oc417_sw0(c, S1, 0x28);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_HeapAlloc` (0x272308).
pub(crate) const OC420_INSEQ_HEAP_ALLOC: Callee = Callee {
    addr: 0x0027_2308,
    f: ee::s07::f_00272308,
};

/// `void* InSeq_HeapNew(uint base, Heap* heap)` (0x272488): allocates
/// `base + gp[0x398]` bytes from `heap` (`InSeq_HeapAlloc(heap_arg, size)`
/// with the arguments in that order), stores `heap` at the block's first word, and
/// returns it advanced by `gp[0x398]`.
pub fn in_seq_heap_new(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x398)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        OC420_INSEQ_HEAP_ALLOC.call(c, 0x0027_24a8)?;
        Addr(c.g[V0][0] as u32).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x398)).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InSeq_ChannelStart.

/// Fields of the in-sequence player instance used by `InSeq_ChannelStart`.
pub(crate) const OC420_INST_OWNER: u32 = 0x08;

pub(crate) const OC420_INST_HDR: u32 = 0x1c;

pub(crate) const OC420_INST_CHANNELS: u32 = 0x24;

pub(crate) const OC420_INST_READY: u32 = 0x28;

pub(crate) const OC420_INST_PLAYER: u32 = 0x2c;

pub(crate) const OC420_INST_DONE: u32 = 0x30;

pub(crate) const OC420_INST_FLAGS: u32 = 0x34;

pub(crate) const OC420_INST_STARTED: u32 = 0x38;

pub(crate) const OC420_INSEQ_HEAP_NEW: Callee = Callee {
    addr: 0x0027_2488,
    f: ee::s07::f_00272488,
};

/// `bool InSeq_ChannelStart(Instance* self)` (0x273aa8): builds the
/// instance's channel objects (type 0 curve, 1 ramp, 2 span, 3 event track,
/// from the header's per-channel records, each allocated with
/// `InSeq_HeapNew`) and starts each through its virtual at vtable+4
/// (`done`/`flags` accumulate), then marks the instance ready, asks the
/// owner's listener for a player and starts it. Returns `self->ready`.
pub fn in_seq_channel_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0x10);
        let this = c.g[S1][0] as u32;
        Addr(this + OC420_INST_DONE).sw(c, 0);
        c.g[A0][0] = Addr(this + OC420_INST_HDR).lw(c);
        Addr(this + OC420_INST_FLAGS).sw(c, 0);
        // Header byte +2: channel count.
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(2)).lbu(c);
        c.g[A1][0] = c.g[V0][0];
        'channels: {
            if c.g[V0][0] == 0 {
                break 'channels;
            }
            c.cycles += 13;
            c.g[V1][0] = Addr(this + OC420_INST_OWNER).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[S2][0] = 0;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
            c.g[A0][0] = c.g[S5][0];
            OC420_INSEQ_HEAP_ALLOC.call(c, 0x0027_3b08)?;
            Addr(this + OC420_INST_CHANNELS).sw(c, c.g[V0][0]);
            c.g[V0][0] = Addr(this + OC420_INST_HDR).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(2)).lbu(c);
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            if c.g[A0][0] == 0 {
                break 'channels;
            }
            c.cycles += 2;
            c.g[S7][0] = 0;
            loop {
                // Channel record: type (u16) and size (u16) at s3.
                c.cycles += 5;
                c.g[S6][0] = c.g[S3][0];
                c.g[V0][0] = 1;
                c.g[V1][0] = Addr(c.g[S6][0] as u32).lhu(c);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(4));
                let kind = c.g[V1][0];
                // (ctor, HeapNew size, HeapNew return, ctor return) per type.
                let mut ctor: Option<(Callee, u64, u32, u32)> = None;
                let mut tail = true;
                if kind == 1 {
                    c.cycles += 10;
                    c.g[V0][0] = Addr(this + OC420_INST_CHANNELS).lw(c);
                    ctor = Some((
                        Callee {
                            addr: 0x0027_34a8,
                            f: ee::s07::f_002734a8,
                        },
                        0x28,
                        0x0027_3bac,
                        0x0027_3bb4,
                    ));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
                    c.g[V0][0] = 2;
                    if kind < 2 {
                        c.cycles += 2;
                        c.g[V0][0] = Addr(this + OC420_INST_CHANNELS).lw(c);
                        if kind == 0 {
                            c.cycles += 9;
                            ctor = Some((
                                Callee {
                                    addr: 0x0027_25b8,
                                    f: ee::s07::f_002725b8,
                                },
                                0x28,
                                0x0027_3b84,
                                0x0027_3b8c,
                            ));
                        } else {
                            c.cycles += 2;
                            tail = false;
                        }
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 3;
                        if kind == 2 {
                            c.cycles += 10;
                            c.g[V0][0] = Addr(this + OC420_INST_CHANNELS).lw(c);
                            ctor = Some((
                                Callee {
                                    addr: 0x0027_2d68,
                                    f: ee::s07::f_00272d68,
                                },
                                0x2c,
                                0x0027_3bd4,
                                0x0027_3bdc,
                            ));
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = Addr(this + OC420_INST_CHANNELS).lw(c);
                            if kind == 3 {
                                c.cycles += 8;
                                ctor = Some((
                                    Callee {
                                        addr: 0x0027_3180,
                                        f: ee::s07::f_00273180,
                                    },
                                    0x28,
                                    0x0027_3bf8,
                                    0x0027_3c00,
                                ));
                            } else {
                                c.cycles += 2;
                                tail = false;
                            }
                        }
                    }
                }
                if let Some((ctor, size, heap_ret, ctor_ret)) = ctor {
                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[A0][0] = size;
                    c.g[A1][0] = c.g[S5][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    OC420_INSEQ_HEAP_NEW.call(c, heap_ret)?;
                    c.g[A0][0] = c.g[V0][0];
                    ctor.call(c, ctor_ret)?;
                    Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                }
                if tail {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(this + OC420_INST_CHANNELS).lw(c);
                }
                // Start the channel object: vtable slot 1.
                c.g[S0][0] = sx(this + OC420_INST_DONE);
                c.cycles += 14;
                Addr(sp(c)).sw(c, 0);
                c.g[S4][0] = 0;
                c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = c.g[S3][0];
                c.g[T0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[SP][0];
                c.g[V1][0] = Addr(c.g[T0][0] as u32).off(4).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(8).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_3c3c)?;
                let started = if c.g[V0][0] != 0 {
                    c.g[S4][0] = 1;
                    c.cycles += 1;
                    1
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(this + OC420_INST_DONE).lw(c);
                    if c.g[V0][0] == 0 {
                        0
                    } else {
                        // This `+1` and the shared one of the started path.
                        c.cycles += 2;
                        c.g[S4][0] = 1;
                        1
                    }
                };
                Addr(c.g[S0][0] as u32).sw(c, started);
                c.cycles += 12;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(4));
                c.g[V1][0] = Addr(sp(c)).lw(c);
                c.g[V0][0] = Addr(this + OC420_INST_FLAGS).lw(c);
                c.g[A0][0] = Addr(this + OC420_INST_HDR).lw(c);
                c.g[V0][0] |= c.g[V1][0];
                Addr(this + OC420_INST_FLAGS).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr(c.g[S6][0] as u32).off(2).lhu(c);
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(2).lbu(c);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if c.g[V0][0] == 0 {
                    break 'channels;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_3b28);
                }
            }
        }
        // Mark ready, ask the owner's listener for a player and start it.
        c.cycles += 7;
        c.g[S0][0] = 1;
        c.g[A0][0] = Addr(this + OC420_INST_OWNER).lw(c);
        Addr(this + OC420_INST_READY).sw(c, c.g[S0][0]);
        Callee {
            addr: 0x0027_4d70,
            f: ee::s07::f_00274d70,
        }
        .call(c, 0x0027_3c98)?;
        c.g[T0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        if c.g[T0][0] != 0 {
            c.cycles += 8;
            c.g[V1][0] = Addr(c.g[T0][0] as u32).lw(c);
            c.g[A3][0] = Addr(this + OC420_INST_HDR).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x30).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x34).lw(c);
            c.g[A2][0] = Addr(c.g[A3][0] as u32).off(3).lbu(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_3cc0)?;
            Addr(this + OC420_INST_PLAYER).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[T0][0] = Addr(this + OC420_INST_PLAYER).lw(c);
        c.g[A1][0] = c.g[S1][0];
        if c.g[T0][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
            c.g[V1][0] = Addr(this + OC420_INST_HDR).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[A3][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            c.g[A2][0] = Addr(c.g[V1][0] as u32).off(3).lbu(c);
            let target = c.g[A3][0] as u32;
            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_3cec)?;
        }
        c.cycles += 13;
        Addr(this + OC420_INST_STARTED).sw(c, c.g[S0][0]);
        c.g[V0][0] = Addr(this + OC420_INST_READY).lw(c);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const OC423_INSEQ_HEAP_ALLOC: Callee = Callee {
    addr: 0x0027_2308,
    f: ee::s07::InSeq_HeapAlloc,
};

pub(crate) const OC423_RAMP_TRACK_ACTIVATE_KEY: Callee = Callee {
    addr: 0x0027_3860,
    f: ee::s07::InSeq_RampTrackActivateKey,
};

pub(crate) const OC423_RAMP_TRACK_NEXT_KEY: Callee = Callee {
    addr: 0x0027_38f8,
    f: ee::s07::InSeq_RampTrackNextKey,
};

/// `bool InSeq_RampTrackStart(RampTrack *track, RampKeys *keys, Seq *seq,
/// uint *flags)` (0x273530, "cScriptPlayerChannelObject_Play"): starts a ramp
/// track. Stores `keys` (+0x10, also +8), `seq` (+0xc), allocates
/// `keys->count * 0x24` bytes of ramp records from the sequence heap
/// (`*(*(seq + 8) + 8) + 8`) into +0x1c, resets the key index (+0x18) and sets
/// the cursor (+0x14) to the first key (`keys + 4`). Then, as long as the
/// first key has no flags, it sets bit 0 of `*flags`, activates the key under
/// the cursor and advances to the next key until a key with flags (+4) or the
/// end. Returns whether the key list is non-empty.
pub fn in_seq_ramp_track_start(c: &mut Cpu) {
    /// Key list header: `u16 key_count` (+0), `u16 record_count` (+2); keys
    /// follow at +4.
    const KEYS_HEADER: u32 = 4;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
        c.g[S1][0] = c.g[A3][0];
        // 'done_load: reload keys into v1 and return; 'epilogue: return with v1.
        'epilogue: {
            'reload: {
                if c.g[V0][0] == 0 {
                    break 'reload;
                }
                c.cycles += 13;
                c.g[V1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(2));
                c.g[A1][0] = 0x24;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.mult(0, 0, V1, A1);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.lo[0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                OC423_INSEQ_HEAP_ALLOC.call(c, 0x0027_357c)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(2));
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                } else {
                    // Empty countdown over the record count.
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    loop {
                        c.cycles += 7;
                        let more = c.g[V0][0] != 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_3598);
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                }
                c.cycles += 4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(KEYS_HEADER));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                if c.g[V0][0] == 0 {
                    break 'reload;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(8));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    break 'epilogue;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                loop {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] |= 1;
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    OC423_RAMP_TRACK_ACTIVATE_KEY.call(c, 0x0027_35ec)?;
                    c.g[A0][0] = c.g[S0][0];
                    OC423_RAMP_TRACK_NEXT_KEY.call(c, 0x0027_35f4)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        break 'epilogue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(4));
                    if c.g[V0][0] != 0 {
                        break 'reload;
                    }
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_35d8);
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = c.lhu(c.g[V1][0] as u32);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

pub(crate) const OC423_INSEQ_HEAP_FREE: Callee = Callee {
    addr: 0x0027_23a8,
    f: ee::s07::InSeq_HeapFree,
};

/// `void InSeq_HeapDelete(void *p)` (0x2724e8): frees an object allocated by
/// `InSeq_HeapNew`: the heap pointer sits in the word `gp[0x398]` bytes before
/// `p`; `InSeq_HeapFree(*header, header)`.
pub fn in_seq_heap_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x398));
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[A1][0] as u32);
        OC423_INSEQ_HEAP_FREE.call(c, 0x0027_2500)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InSeq::InstanceStop(this)` (0x274918, replay cache script player): does
/// nothing when the play state (+0x20) is 0; when an entity update is running
/// (+0x3c) only flags a deferred stop (+0x40 = 1); otherwise stops each
/// entity player (the header's byte +1 entries of 0x3c bytes at +0x18), clears
/// the play state, frame, pause and step counters (+0x20..+0x38, the
/// script length and jump frame to -1) and tells the in-sequence listener
/// (`GetListener`) through its vtable slot +0x20/+0x24.
pub fn in_seq_instance_stop(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x00274918);
        'b_2749d4: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_2749d4;
            } else {
                'b_2749d0: {
                    'b_274950: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_274950;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                            if t {
                                break 'b_2749d0;
                            } else {
                                break 'b_274950;
                            }
                        }
                    }
                    'b_27498c: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                        let t = c.g[V0][0] == 0u64;
                        c.g[S1][0] = 0u64;
                        if t {
                            break 'b_27498c;
                        } else {
                            c.cycles += 2;
                            c.g[S2][0] = 0u64;
                            'l_274968: loop {
                                c.cycles += 9;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                Callee {
                                    addr: 0x00273dc0,
                                    f: ee::s07::InSeq_ChannelStop,
                                }
                                .call(c, 0x00274978)?;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3c));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00274968);
                                    }
                                    continue 'l_274968;
                                } else {
                                    break 'b_27498c;
                                }
                            }
                        }
                    }
                    c.cycles += 12;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0u64);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x38), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    Callee {
                        addr: 0x00274d70,
                        f: ee::s07::InSeq_InstanceGetListener,
                    }
                    .call(c, 0x002749b0)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        break 'b_2749d0;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002749d0)?;
                        break 'b_2749d0;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_2749d4;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

/// `int InSeq_InstanceSetup(InSeq* self, SeqData* data, Owner* owner, int
/// playerId)` (0x2745c0, `a0..a3`): initialise a sequence instance. Stores
/// `playerId` / `owner` / `data` (twice: `+4`, `+0x10`) and the defaults
/// (`+0x34 = -1`, `+0x38 = +0x1c = 0`), the extension block at `data + 0xc`
/// when the header is 13 bytes or more, and, for the `data->+1` channels,
/// allocates the channel array (`InSeq_HeapNewArray(owner + 8, 0x3c * n +
/// 0x10)`, the count stored in front, `InSeq_Channel_ctor` for each) and sets
/// each up from the entity data after the header (`InSeq_ChannelSetup`
/// returns the bytes it consumed). Returns `data->+8`.
pub fn in_seq_instance_setup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x50);
        frame.save(c, S5, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x70);
        c.g[S5][0] = u64::MAX;
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, 22, 0x10);
        frame.save_ra(c, 0);
        c.sw(c.g[S2][0] as u32, c.g[A3][0]);
        c.sw(oc429_at(c.g[S2][0], 0x8), c.g[A2][0]);
        c.sw(oc429_at(c.g[S2][0], 0x4), c.g[A1][0]);
        c.sw(oc429_at(c.g[S2][0], 0x10), c.g[A1][0]);
        c.sw(oc429_at(c.g[S2][0], 0x34), c.g[S5][0]);
        c.sw(oc429_at(c.g[S2][0], 0x38), 0);
        c.sw(oc429_at(c.g[S2][0], 0x1c), 0);
        c.g[V0][0] = c.lhu(oc429_at(c.g[A1][0], 0x2));
        c.g[V0][0] = (c.g[V0][0] < 0xd) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = sx(oc429_at(c.g[A1][0], 0xc));
            c.sw(oc429_at(c.g[S2][0], 0x14), c.g[V0][0]);
        }
        c.g[A0][0] = c.lw(oc429_at(c.g[S2][0], 0x10));
        'done: {
            c.cycles += 4;
            c.g[V0][0] = c.lhu(oc429_at(c.g[A0][0], 0x2));
            c.g[V1][0] = c.lbu(oc429_at(c.g[A0][0], 0x1));
            let no_channels = c.g[V1][0] == 0;
            c.g[S3][0] = sx(oc429_at(c.g[A1][0], c.g[V0][0] as u32));
            if no_channels {
                break 'done;
            }
            'array: {
                c.cycles += 12;
                c.g[S0][0] = c.g[V1][0];
                c.g[A0][0] = 0x3c;
                c.mult(0, A0, S0, A0);
                c.g[A1][0] = sx(oc429_at(c.g[A2][0], 0x8));
                c.g[S1][0] = sx(oc429_at(c.g[S0][0], 0u32.wrapping_sub(1)));
                c.g[S6][0] = sx(oc429_at(c.g[S2][0], 0x18));
                c.g[A0][0] = sx(oc429_at(c.g[A0][0], 0x10));
                call_ptr(c, 0x0027_24c8, 0x0027_4654)?; // InSeq_HeapNewArray
                c.g[S4][0] = sx(oc429_at(c.g[V0][0], 0x10));
                c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                let one = c.g[S1][0] == c.g[S5][0];
                c.g[S0][0] = c.g[S4][0];
                if one {
                    break 'array;
                }
                c.cycles += 1;
                c.g[S5][0] = u64::MAX;
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx(oc429_at(c.g[S1][0], 0u32.wrapping_sub(1)));
                    call_ptr(c, 0x0027_3938, 0x0027_4674)?; // InSeq_Channel_ctor
                    let more = c.g[S1][0] != c.g[S5][0];
                    c.g[S0][0] = sx(oc429_at(c.g[S0][0], 0x3c));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_4668);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 5;
            c.sw(c.g[S6][0] as u32, c.g[S4][0]);
            c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], 0x10));
            c.g[V1][0] = c.lbu(oc429_at(c.g[V0][0], 0x1));
            let none = c.g[V1][0] == 0;
            c.g[S0][0] = 0;
            if none {
                break 'done;
            }
            c.cycles += 2;
            c.g[S1][0] = 0;
            loop {
                c.cycles += 12;
                c.g[A0][0] = c.lw(oc429_at(c.g[S2][0], 0x18));
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[S0][0] = sx(oc429_at(c.g[S0][0], 1));
                c.g[A0][0] = sx(oc429_at(c.g[A0][0], c.g[S1][0] as u32));
                call_ptr(c, 0x0027_39b8, 0x0027_46b0)?; // InSeq_ChannelSetup
                c.g[V1][0] = c.lw(oc429_at(c.g[S2][0], 0x10));
                c.g[S3][0] = sx(oc429_at(c.g[S3][0], c.g[V0][0] as u32));
                c.g[V0][0] = c.lbu(oc429_at(c.g[V1][0], 0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(oc429_at(c.g[S1][0], 0x3c));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_4698);
                    }
                    continue;
                }
                break 'done;
            }
        }
        c.cycles += 12;
        c.g[V1][0] = c.lw(oc429_at(c.g[S2][0], 0x10));
        frame.restore(c, S0, 0x70);
        c.g[V0][0] = c.lw(oc429_at(c.g[V1][0], 0x8));
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InSeq_InstanceRelease(InSeqInstance* self)` (0x2746f8): if the
/// instance is live (`+4`), stops it, releases every channel (`+0x18` array of
/// 0x3c-byte `InSeq_Channel`s, count in the byte at `(+0x10)+1`), destroys the
/// channel array backwards and deletes it, then resets the instance to its
/// idle state (`-1` ids, `1.0f` rate, everything else 0).
pub fn in_seq_instance_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let idle = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if !idle {
            c.cycles += 6;
            c.g[S0][0] = 0;
            call_ptr(c, 0x0027_4918, 0x0027_4724)?; // InSeq_InstanceStop
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(1));
            if c.g[V1][0] == 0 {
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            } else {
                c.cycles += 1;
                c.g[S2][0] = 0;
                loop {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    call_ptr(c, 0x0027_3a00, 0x0027_4748)?; // InSeq_ChannelRelease
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3c));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_4738);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                    break;
                }
            }
            c.cycles += 2;
            let no_array = c.g[A1][0] == 0;
            c.g[V0][0] = 0x3c;
            if !no_array {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x10));
                c.mult(0, A0, V1, V0);
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                if c.g[A1][0] == c.g[S0][0] {
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                } else {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3c));
                    loop {
                        c.cycles += 6;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0027_3970, 0x0027_478c)?; // InSeq_Channel_dtor
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        let more = c.g[V0][0] != c.g[S0][0];
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3c));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0027_4780);
                            }
                            continue;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        break;
                    }
                }
                c.cycles += 2;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
                call_ptr(c, 0x0027_2510, 0x0027_47a4)?; // InSeq_HeapDeleteArray
            }
            c.cycles += 19;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            let this = c.g[S1][0] as u32;
            c.sw(this.wrapping_add(0x34), c.g[V0][0]);
            c.swc1(0, this.wrapping_add(0x24));
            c.sw(this.wrapping_add(0x38), 0);
            c.sw(this, c.g[V0][0]);
            for off in [0x4u32, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x28, 0x2c] {
                c.sw(this.wrapping_add(off), 0);
            }
            c.sw(this.wrapping_add(0x30), c.g[V0][0]);
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq* InSeq_ctor(self, pHeap, heapSize, count, tag)` (0x272788): sets up
/// an instance pool. Initialises the heap (`self + 8`), stores `tag` (+0) and
/// `count` (+4), clears +0x14/+0x18, allocates `count * 0x58 + 0x10` bytes
/// (the first word holds the count, the items start at +0x10), constructs
/// every item (`InSeq_Instance_ctor(item + 8)`), stores the items at +0x14,
/// links them in a free list (`item[i].index = i`, `item[i].next = &item[i +
/// 1]`, `item[i].prev = &item[i - 1]`), terminates both ends (the first item's
/// prev and the last one's next = 0), clears the used list (+0x1c) and points
/// the free list (+0x18) at the first item. Returns `self`. Frame 0x70 (s0-s5).
pub fn in_seq_ctor(c: &mut Cpu) {
    const HEAP_INIT: Callee = Callee {
        addr: 0x0027_22e0,
        f: ee::s07::f_002722e0,
    };
    const HEAP_NEW_ARRAY: Callee = Callee {
        addr: 0x0027_24c8,
        f: ee::s07::f_002724c8,
    };
    const INSTANCE_CTOR: Callee = Callee {
        addr: 0x0027_4518,
        f: ee::s07::f_00274518,
    };
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S3, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        let this = r32(c, S3);
        c.g[S2][0] = sx(this.wrapping_add(8));
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A3][0];
        c.g[S1][0] = c.g[T0][0];
        c.g[A0][0] = c.g[S2][0];
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        c.g[S4][0] = sx(r32(c, S0).wrapping_sub(1));
        frame.save_ra(c, 0);
        c.g[S5][0] = sx(this.wrapping_add(0x14));
        HEAP_INIT.call(c, 0x0027_27c8)?;
        c.g[A0][0] = 0x58;
        c.sw(this, c.g[S1][0]);
        c.mult(0, A0, S0, A0);
        c.sw(this.wrapping_add(4), c.g[S0][0]);
        c.sw(this.wrapping_add(0x14), 0);
        c.g[A1][0] = c.g[S2][0];
        c.sw(this.wrapping_add(0x18), 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        HEAP_NEW_ARRAY.call(c, 0x0027_27ec)?;
        c.g[S2][0] = sx(r32(c, V0).wrapping_add(0x10));
        c.sw(r32(c, V0), c.g[S0][0]);
        let empty = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[S2][0];
        // Construct the items.
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = u64::MAX;
            loop {
                c.cycles += 5;
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(8));
                c.g[S4][0] = sx(r32(c, S4).wrapping_sub(1));
                INSTANCE_CTOR.call(c, 0x0027_280c)?;
                let more = c.g[S4][0] != c.g[S0][0];
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x58));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_2800);
                }
            }
        }
        // Link the items: index, next (+0x50), prev (+0x54).
        c.cycles += 4;
        c.sw(r32(c, S5), c.g[S2][0]);
        c.g[V0][0] = c.lw(this.wrapping_add(4));
        let none = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[A3][0] = 0xffff_ffff_ffff_ffa8;
            c.g[A2][0] = 0x58;
            c.g[A1][0] = 0;
            loop {
                c.cycles += 18;
                c.g[V0][0] = c.lw(this.wrapping_add(0x14));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.sw(r32(c, V0).wrapping_add(4), c.g[A0][0]);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                c.g[V0][0] = c.lw(this.wrapping_add(0x14));
                c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x58));
                c.sw(r32(c, V0).wrapping_add(0x50), c.g[V1][0]);
                c.g[V0][0] = c.lw(this.wrapping_add(0x14));
                c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x58));
                c.sw(r32(c, V0).wrapping_add(0x54), c.g[V1][0]);
                c.g[V0][0] = c.lw(this.wrapping_add(4));
                c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x58));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_2830);
                }
            }
        }
        c.cycles += 21;
        c.g[A0][0] = c.lw(this.wrapping_add(0x14));
        c.g[A1][0] = 0x58;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x60);
        c.sw(r32(c, A0).wrapping_add(0x54), 0);
        frame.restore(c, S1, 0x50);
        c.g[V1][0] = c.lw(this.wrapping_add(4));
        c.g[A0][0] = c.lw(this.wrapping_add(0x14));
        c.mult(0, V1, V1, A1);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        c.sw(r32(c, V1).wrapping_sub(8), 0);
        c.sw(this.wrapping_add(0x1c), 0);
        c.g[V1][0] = c.lw(this.wrapping_add(0x14));
        c.sw(this.wrapping_add(0x18), c.g[V1][0]);
        frame.restore(c, S3, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void* InSeq_ChannelGetHeaderField4(InSeqChannel* ch)` (0x274388): the
/// word at +4 of the header the channel points to (`ch->header` at +0x10).
pub fn oc436_in_seq_channel_get_header_field4(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(OC436_CHANNEL_HEADER));
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
    c.pc = t;
}

/// `InSeqChannel.header`.
pub(crate) const OC436_CHANNEL_HEADER: u32 = 0x10;

/// `void* InSeq_HeapNewArray(...)` (0x2724c8): tail call of `InSeq_HeapNew`.
pub fn oc436_in_seq_heap_new_array(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0027_2488), 0x0027_24d8)
}

/// `InSeq_HeapDeleteArray(p)` (0x272510): `InSeq_HeapDelete(p)`.
pub fn inseq_heap_delete_array(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(Callee {
            addr: 0x0027_24e8,
            f: ee::s07::InSeq_HeapDelete,
        }),
        0x0027_2520,
    )
}

/// `void* InSeq_InstanceSetUserData(inst, data)` (0x274c10): swaps
/// `inst->userData` (+0xc) for `data`, returning the old value.
pub fn inseq_instance_set_user_data(c: &mut Cpu) {
    c.cycles += 3;
    let inst = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(inst.wrapping_add(0xc)).lw(c);
    c.sw(inst.wrapping_add(0xc), c.g[A1][0]);
    ret(c);
}

/// `Replay_EndSave` (0x270ed8): frees the save buffers.
pub fn replay_end_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0027_0f78, 0x0027_0ee8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_ResetOnLoad` (0x270ef8): resets the replay state on a load.
pub fn replay_reset_on_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0026_f228, 0x0027_0f08)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_EmptyLoadHook` (0x270f18): an empty hook.
pub fn replay_empty_load_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

// Callees. Installed callees are reached through their override slots by the
// generated alias; the rest run as generated.
pub(crate) const CALLEE_REPLAY_SAVE_EMIT: Callee = Callee {
    addr: 0x0027_1058,
    f: ee::s07::f_00271058,
};

pub(crate) const CALLEE_REPLAY_STATIC_INIT_DESTROY: Callee = Callee {
    addr: 0x0027_18e8,
    f: ee::s07::f_002718e8,
};

pub(crate) const CALLEE_INSEQ_HEAP_DELETE: Callee = Callee {
    addr: 0x0027_24e8,
    f: ee::s07::f_002724e8,
};

pub(crate) const CALLEE_INSEQ_FREE_SEQUENCE: Callee = Callee {
    addr: 0x0027_29a8,
    f: ee::s07::f_002729a8,
};

pub(crate) const CALLEE_INSEQ_GET_INSTANCE: Callee = Callee {
    addr: 0x0027_2cc0,
    f: ee::s07::f_00272cc0,
};

pub(crate) const CALLEE_INSEQ_PLAY: Callee = Callee {
    addr: 0x0027_2ad0,
    f: ee::s07::f_00272ad0,
};

pub(crate) const CALLEE_INSEQ_STOP: Callee = Callee {
    addr: 0x0027_2b58,
    f: ee::s07::f_00272b58,
};

pub(crate) const CALLEE_INSEQ_PAUSE: Callee = Callee {
    addr: 0x0027_2bd0,
    f: ee::s07::f_00272bd0,
};

pub(crate) const CALLEE_INSEQ_RESUME: Callee = Callee {
    addr: 0x0027_2c48,
    f: ee::s07::f_00272c48,
};

pub(crate) const CALLEE_INSEQ_INSTANCE_PAUSE: Callee = Callee {
    addr: 0x0027_49e8,
    f: ee::s07::f_002749e8,
};

pub(crate) const CALLEE_INSEQ_INSTANCE_RESUME: Callee = Callee {
    addr: 0x0027_4a08,
    f: ee::s07::f_00274a08,
};

pub(crate) const CALLEE_INSEQ_INSTANCE_GET_DATA: Callee = Callee {
    addr: 0x0027_4c58,
    f: ee::s07::f_00274c58,
};

pub(crate) const CALLEE_INSEQ_INSTANCE_GET_OWNER: Callee = Callee {
    addr: 0x0027_4c60,
    f: ee::s07::f_00274c60,
};

pub(crate) const CALLEE_INSEQ_GET_CHANNEL_COUNT: Callee = Callee {
    addr: 0x0027_4c68,
    f: ee::s07::f_00274c68,
};

pub(crate) const CALLEE_INSEQ_FIND_CHANNEL_DATA: Callee = Callee {
    addr: 0x0027_4d90,
    f: ee::s07::f_00274d90,
};

pub(crate) const CALLEE_INSEQ_BLOCK_GET_FIELD4: Callee = Callee {
    addr: 0x0027_4398,
    f: ee::s07::f_00274398,
};

/// `Replay_NextSaveBlock(Replay* r)` (0x2716f0): clears the two save words
/// at +0x644/+0x648 and advances the save block index at +0x638.
pub fn replay_next_save_block(c: &mut Cpu) {
    c.cycles += 6;
    let r = Addr(c.g[A0][0] as u32);
    c.g[V0][0] = r.off(0x638).lw(c);
    r.off(0x644).sw(c, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    r.off(0x648).sw(c, 0);
    let v0 = c.g[V0][0];
    r.off(0x638).sw(c, v0);
    ret(c);
}

/// `Replay_DebugPrintfStub(a0..a3, t0..t3, f12..f18)` (0x2718a8): a varargs
/// spill of the argument registers into the caller's save area, then
/// returns `a0` in v0.
pub fn replay_debug_printf_stub(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x80);
    c.g[V0][0] = c.g[A0][0];
    let a1 = c.g[A1][0];
    c.sd(frame.at(c, 0x48), a1);
    let a2 = c.g[A2][0];
    c.sd(frame.at(c, 0x50), a2);
    let a3 = c.g[A3][0];
    c.sd(frame.at(c, 0x58), a3);
    let t0 = c.g[T0][0];
    c.sd(frame.at(c, 0x60), t0);
    let t1 = c.g[T1][0];
    c.sd(frame.at(c, 0x68), t1);
    let t2 = c.g[T2][0];
    c.sd(frame.at(c, 0x70), t2);
    let t3 = c.g[T3][0];
    c.sd(frame.at(c, 0x78), t3);
    c.swc1(12, frame.at(c, 0x38));
    c.swc1(14, frame.at(c, 0x3c));
    c.swc1(16, frame.at(c, 0x40));
    c.swc1(18, frame.at(c, 0x44));
    frame.close(c);
}

/// `Replay_GlobalCtors` (0x2722c0): the static constructor entry, calling
/// `Replay_StaticInitDestroy(1, 0xffff)` (initialise the replay's globals).
pub fn replay_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 1;
        c.g[A1][0] = 0xffff;
        CALLEE_REPLAY_STATIC_INIT_DESTROY.call(c, 0x0027_22d4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Replay_SaveWriteCache(Replay* r)` (0x271510): one step of the replay
/// write-out. Emits a pending setup block, or Huffman-encodes the pending
/// command frame and emits it, or moves on to the next input cache.
pub fn replay_save_write_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        'done: {
            c.g[S0][0] = c.g[A0][0];
            let me = c.g[S0][0] as u32;
            c.g[A1][0] = Addr(me).off(0x674).lw(c);
            if c.g[A1][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = 8;
                CALLEE_REPLAY_SAVE_EMIT.call(c, 0x0027_1534)?;
                Addr(me).off(0x674).sw(c, 0);
                break 'done;
            }
            c.g[A0][0] = Addr(me).off(0x66c).lw(c);

            // A pending command frame: Huffman-encode it into the temporary.
            c.cycles += 2;
            let no_frame = c.g[A0][0] == 0;
            c.g[A3][0] = c.g[SP][0];
            if !no_frame {
                c.cycles += 13;
                c.g[A2][0] = Addr(me).off(0x640).lw(c);
                c.g[A1][0] = Addr(me).off(0x670).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                Addr(sp(c)).sw(c, 0);
                call_ptr(c, 0x002c_a100, 0x0027_1558)?;
                c.g[V1][0] = Addr(me).off(0x640).lw(c);
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[S0][0];
                let v0 = c.g[V0][0];
                Addr(c.g[V1][0] as u32).sw(c, v0);
                c.g[A1][0] = Addr(me).off(0x640).lw(c);
                CALLEE_REPLAY_SAVE_EMIT.call(c, 0x0027_1570)?;
                Addr(me).off(0x66c).sw(c, 0);
                break 'done;
            }

            // No command frame: the next input cache, if any.
            c.cycles += 5;
            c.g[V1][0] = Addr(me).off(0x654).lw(c);
            c.g[V0][0] = Addr(me).off(0x658).lw(c);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let past_end = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            if past_end {
                // Past the last index: count the block and finish.
                c.cycles += 5;
                c.g[V0][0] = Addr(me).off(0x638).lw(c);
                Addr(me).off(0x644).sw(c, 0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                Addr(me).off(0x648).sw(c, 0);
                let v0 = c.g[V0][0];
                Addr(me).off(0x638).sw(c, v0);
                break 'done;
            }
            c.cycles += 7;
            Addr(me).off(0x66c).sw(c, 0);
            Addr(me).off(0x674).sw(c, 0);
            c.g[V0][0] = sx(me.wrapping_add(c.g[V0][0] as u32));
            Addr(me).off(0x670).sw(c, 0);
            c.g[A2][0] = Addr(c.g[V0][0] as u32).off(0x48c).lw(c);
            let empty_slot = c.g[A2][0] == 0;
            c.g[A0][0] = 0x8000;
            if !empty_slot {
                // The slot holds a cache: start its command frame.
                c.cycles += 15;
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
                c.g[A1][0] = Addr(me).off(0x640).lw(c);
                c.g[V0][0] = 0x1111_0000;
                let v1 = c.g[V1][0];
                Addr(me).off(0x66c).sw(c, v1);
                c.g[V0][0] = c.g[V0][0] | 0x1112;
                let a1 = c.g[A1][0];
                Addr(me).off(0x674).sw(c, a1);
                let a0 = c.g[A0][0];
                Addr(me).off(0x670).sw(c, a0);
                let v0 = c.g[V0][0];
                Addr(c.g[A1][0] as u32).sw(c, v0);
                c.g[A0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr(me).off(0x674).lw(c);
                let a0 = c.g[A0][0];
                Addr((c.g[V1][0] as u32).wrapping_add(0x4)).sw(c, a0);
                c.g[V0][0] = Addr(me).off(0x654).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                let v0 = c.g[V0][0];
                Addr(me).off(0x654).sw(c, v0);
                break 'done;
            }
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            Addr(me).off(0x644).sw(c, 0);
            let v0 = c.g[V0][0];
            Addr(me).off(0x654).sw(c, v0);
            Addr(me).off(0x648).sw(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_HeapDelete_Thunk(p)` (0x272530): tail call of `InSeq_HeapDelete`.
pub fn insseq_heap_delete_thunk(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(CALLEE_INSEQ_HEAP_DELETE),
        0x0027_2540,
    );
}

/// `InSeq_HeapDelete_Thunk2(p)` (0x272550): the second copy of the thunk.
pub fn insseq_heap_delete_thunk2(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(CALLEE_INSEQ_HEAP_DELETE),
        0x0027_2560,
    );
}

/// `InSeq_FreeAll(self)` (0x272a50): frees every sequence of the channel list
/// (the list head at +0x1c, next at +0x50). The next pointer is loaded
/// before each `InSeq_FreeSequence(self, node)` call.
pub fn insseq_free_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x1c).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[S0][0] = Addr(c.g[S0][0] as u32).off(0x50).lw(c);
                CALLEE_INSEQ_FREE_SEQUENCE.call(c, 0x0027_2a84)?;
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_2a78);
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_PlayAll(self, a1, a2)` (0x272b00): `InSeq_Play(self, node, a2)` for
/// every node of the channel list.
pub fn insseq_play_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x1c).lw(c);
        c.g[S2][0] = c.g[A1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                CALLEE_INSEQ_PLAY.call(c, 0x0027_2b34)?;
                c.g[S0][0] = Addr(c.g[S0][0] as u32).off(0x50).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_2b28);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The loop shared by `InSeq_StopAll`, `PauseAll` and `ResumeAll`: calls
/// `callee(self, node)` for every node of the channel list. The next pointer
/// is loaded after the call.
pub(crate) fn insseq_each_channel(c: &mut Cpu, callee: Callee, ret: u32, head: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x20);
    c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x1c).lw(c);
    if c.g[S0][0] != 0 {
        c.cycles += 2;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            callee.call(c, ret)?;
            c.g[S0][0] = Addr(c.g[S0][0] as u32).off(0x50).lw(c);
            if c.g[S0][0] == 0 {
                c.cycles += 1;
                break;
            }
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `InSeq_StopAll(self)` (0x272b80): `InSeq_Stop(self, node)` for each node.
pub fn insseq_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        insseq_each_channel(c, CALLEE_INSEQ_STOP, 0x0027_2bb0, 0x0027_2ba8)
    });
}

/// `InSeq_PauseAll(self)` (0x272bf8): `InSeq_Pause(self, node)` for each node.
pub fn insseq_pause_all(c: &mut Cpu) {
    guest(c, |c| {
        insseq_each_channel(c, CALLEE_INSEQ_PAUSE, 0x0027_2c28, 0x0027_2c20)
    });
}

/// `InSeq_ResumeAll(self)` (0x272c70): `InSeq_Resume(self, node)` for each node.
pub fn insseq_resume_all(c: &mut Cpu) {
    guest(c, |c| {
        insseq_each_channel(c, CALLEE_INSEQ_RESUME, 0x0027_2ca0, 0x0027_2c98)
    });
}

/// `InSeq_Pause(self, id)` (0x272bd0): pauses the player of channel `id`
/// (`InSeq_GetInstance`, then `InstancePause` on it).
pub fn insseq_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        CALLEE_INSEQ_GET_INSTANCE.call(c, 0x0027_2be0)?;
        c.g[A0][0] = c.g[V0][0];
        CALLEE_INSEQ_INSTANCE_PAUSE.call(c, 0x0027_2be8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_Resume(self, id)` (0x272c48): resumes the player of channel `id`.
pub fn insseq_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        CALLEE_INSEQ_GET_INSTANCE.call(c, 0x0027_2c58)?;
        c.g[A0][0] = c.g[V0][0];
        CALLEE_INSEQ_INSTANCE_RESUME.call(c, 0x0027_2c60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_GetInstanceData(self, id)` (0x272ce0): tail call of
/// `cScriptPlayerMain_GetProjectID`-style accessor (`+4` of the instance).
pub fn insseq_get_instance_data(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(CALLEE_INSEQ_INSTANCE_GET_DATA),
        0x0027_2cf0,
    );
}

/// `InSeq_GetInstanceOwner(self, id)` (0x272d00): tail call of the owner
/// accessor (`+8` of the instance).
pub fn insseq_get_instance_owner(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(CALLEE_INSEQ_INSTANCE_GET_OWNER),
        0x0027_2d10,
    );
}

/// `InSeq_GetSeqChannelCount(self)` (0x272d20): tail call of the channel
/// count accessor (byte at +1 of the instance).
pub fn insseq_get_channel_count(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(CALLEE_INSEQ_GET_CHANNEL_COUNT),
        0x0027_2d30,
    );
}

/// `InSeq_GetChannelBlockField4(self, id)` (0x272d40): the field-4 word of
/// the channel block that `InSeq_FindChannelData` finds.
pub fn insseq_get_channel_block_field4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        CALLEE_INSEQ_FIND_CHANNEL_DATA.call(c, 0x0027_2d50)?;
        c.g[A0][0] = c.g[V0][0];
        CALLEE_INSEQ_BLOCK_GET_FIELD4.call(c, 0x0027_2d58)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `InSeq_GetBlockPayload` (0x274318, `cScriptPlayerEntity_GetHeader1`): the payload of block
/// `index` of a replay sequence. The block's table entry is `seq + 4*index`; its word at +8 is
/// the offset of the block header from `seq`. A header whose 16-bit kind is below 9 has no
/// payload (returns 0); any other returns `header + 8`.
pub fn in_seq_get_block_payload(c: &mut Cpu) {
    c.cycles += 8;
    let scaled = (c.g[A1][0] as u32) << 2;
    let entry = (c.g[A0][0] as u32).wrapping_add(scaled);
    c.g[A1][0] = sx(entry);
    c.g[V1][0] = Addr(entry.wrapping_add(0x8)).lw(c);
    let header = (c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32);
    c.g[A0][0] = sx(header);
    c.g[V0][0] = Addr(header).lhu(c);
    let kind_small = c.g[V0][0] < 0x9;
    c.g[V0][0] = 0;
    c.cycles += 2;
    if !kind_small {
        c.g[V0][0] = sx(header.wrapping_add(0x8));
    }
    ret(c);
}

/// `InSeq_BlockGetField4` (0x274398): the word at +4 of a sequence block.
pub fn in_seq_block_get_field4(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
    ret(c);
}

/// `InSeq_BlockGetSize` (0x2743c0): the size word at +0 of a sequence block.
pub fn in_seq_block_get_size(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    ret(c);
}

/// `InSeq_InstanceSetSpeed` (0x274808): stores the float in f12 as the instance speed at +0x24.
pub fn in_seq_instance_set_speed(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x24));
    ret(c);
}

/// `InSeq_InstanceRestart` (0x274810): restarts an instance, starting it again on the track
/// word at +0x1c (`InSeq_InstanceStart(inst, inst->track)`).
pub fn in_seq_instance_restart(c: &mut Cpu) {
    const INSTANCE_START: Callee = Callee {
        addr: 0x0027_4830,
        f: ee::s07::f_00274830,
    };
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).lw(c);
        INSTANCE_START.call(c, 0x0027_4820)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `InSeq_InstanceGetData` (0x274c58): the data pointer at +4 of an instance.
pub fn in_seq_instance_get_data(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
    ret(c);
}

/// `InSeq_InstanceGetOwner` (0x274c60): the owner pointer at +8 of an instance.
pub fn in_seq_instance_get_owner(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c);
    ret(c);
}

/// `InSeq_GetChannelCount` (0x274c68): the byte at +1 of a sequence header.
pub fn in_seq_get_channel_count(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1)).lbu(c);
    ret(c);
}

/// `InSeq_FindChannelData` (0x274d90): walks `count` blocks of a sequence. The first block
/// is at `seq + u16 at +2`; each step adds the block's size word (`InSeq_BlockGetSize`).
/// Returns the address after the last block walked. The loop checks for due events at its
/// back edge, as the generated body does.
pub fn in_seq_find_channel_data(c: &mut Cpu) {
    const BLOCK_GET_SIZE: Callee = Callee {
        addr: 0x0027_43c0,
        f: ee::s07::f_002743c0,
    };
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x2)).lhu(c);
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if c.g[S0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                BLOCK_GET_SIZE.call(c, 0x0027_4dbc)?;
                let more = c.g[S0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_4db0);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cScriptPlayerMain::~cScriptPlayerMain(this, flags)` (0x274578, `InSeq_Instance_dtor`):
/// releases the instance (`InSeq_InstanceRelease`), then frees `this` through
/// `operator delete` (0x317e50) when bit 0 of `flags` is set.
pub fn in_seq_instance_dtor(c: &mut Cpu) {
    const RELEASE: Callee = Callee {
        addr: 0x0027_46f8,
        f: ee::s07::f_002746f8,
    };
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        RELEASE.call(c, 0x0027_4594)?;
        c.g[S0][0] &= 1;
        let free_this = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if free_this {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0027_45a8)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cScriptPlayerMain_Preview(this, flag)` (0x274868, `InSeq_InstanceRunToEnd`): stores
/// `flag` as the preview comm interface (+0x44), starts the instance at speed 1.0 and
/// updates it until it stops, then restores the saved speed (+0x24) and loop script
/// (+0x1c) and clears the interface. The update loop checks for due events at its back
/// edge.
pub fn in_seq_instance_run_to_end(c: &mut Cpu) {
    const START: Callee = Callee {
        addr: 0x0027_4830,
        f: ee::s07::f_00274830,
    };
    const UPDATE: Callee = Callee {
        addr: 0x0027_4a30,
        f: ee::s07::f_00274a30,
    };
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save_fpr(c, 20, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x38);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20);
        c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x24));
        let flag = c.g[A1][0];
        oc538_field(c, S0, 0x44).sw(c, flag);
        c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[A1][0] = 0;
        c.g[S1][0] = oc538_field(c, S0, 0x1c).lw(c);
        START.call(c, 0x0027_48a4)?;
        c.g[V0][0] = oc538_field(c, S0, 0x20).lw(c);
        if c.g[V0][0] == 0 {
            c.swc1(21, (c.g[S0][0] as u32).wrapping_add(0x24));
        } else {
            c.cycles += 2;
            c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x24));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                UPDATE.call(c, 0x0027_48c0)?;
                c.g[V0][0] = oc538_field(c, S0, 0x20).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.swc1(21, (c.g[S0][0] as u32).wrapping_add(0x24));
                    break;
                }
                c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x24));
                if event_due(c) {
                    deliver_event_here(c, 0x0027_48b8);
                }
            }
        }
        c.cycles += 9;
        let saved_script = c.g[S1][0];
        oc538_field(c, S0, 0x1c).sw(c, saved_script);
        oc538_field(c, S0, 0x44).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Field address `base + off`, `base` being a 64-bit register value.
#[inline]
pub(crate) fn replay_field_ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// The epilogue of `Replay_SaveWriteSnapshot` (frame 0x30, `ra` at +0x10).
pub(crate) fn write_snapshot_return(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

/// `void cReplay_writeFrames(cReplay* self)` (0x271380): write the next piece
/// of the save stream. A pending encoded block is Huffman-encoded and emitted;
/// otherwise the next frame of the write queue is emitted as a header (with its
/// block queued); with no frame left the write state advances.
pub fn replay_save_write_snapshot(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x660));
        c.g[A3][0] = c.g[SP][0];
        if c.g[A0][0] != 0 {
            // A block is pending: encode it into WriteTmp + 4, store the size
            // word, emit the block and clear WriteBlockData.
            c.cycles += 13;
            c.g[A2][0] = c.lw(replay_field_ea(c.g[S0][0], 0x640));
            c.g[A1][0] = c.lw(replay_field_ea(c.g[S0][0], 0x664));
            c.g[A2][0] = sx(replay_field_ea(c.g[A2][0], 4));
            c.g[RA][0] = sx(0x0027_13b0);
            c.sw(c.g[SP][0] as u32, 0);
            call_ptr(c, 0x002c_a100, 0x0027_13b0)?;
            c.g[V1][0] = c.lw(replay_field_ea(c.g[S0][0], 0x640));
            c.g[A2][0] = sx(replay_field_ea(c.g[V0][0], 4));
            c.g[A0][0] = c.g[S0][0];
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            c.g[RA][0] = sx(0x0027_13c8);
            c.g[A1][0] = c.lw(replay_field_ea(c.g[S0][0], 0x640));
            Callee {
                addr: 0x0027_1058,
                f: ee::s07::f_00271058,
            }
            .call(c, 0x0027_13c8)?;
            c.sw(replay_field_ea(c.g[S0][0], 0x660), 0);
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            write_snapshot_return(c, frame);
            return Ok(());
        }

        // No block pending: the write queue (WriteFrame).
        c.cycles += 3;
        c.g[A0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[A1][0] = 1;
        if c.g[A0][0] == 0 {
            // Queue empty: advance the write state and clear the source.
            c.cycles += 5;
            c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x638));
            c.sw(replay_field_ea(c.g[S0][0], 0x644), 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sw(replay_field_ea(c.g[S0][0], 0x648), 0);
            c.sw(replay_field_ea(c.g[S0][0], 0x638), c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            write_snapshot_return(c, frame);
            return Ok(());
        }

        // A frame: count it, queue its block, and emit its header.
        c.cycles += 50;
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x668));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.g[RA][0] = sx(0x0027_13ec);
        c.sw(replay_field_ea(c.g[S0][0], 0x668), c.g[V0][0]);
        Callee {
            addr: 0x0026_ed90,
            f: ee::s07::f_0026ed90,
        }
        .call(c, 0x0027_13ec)?;
        c.g[A3][0] = c.lw(replay_field_ea(c.g[S0][0], 0x640));
        c.g[V0][0] = 0x1111_1113;
        c.g[T1][0] = 0x1111_1111;
        c.sw(c.g[A3][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[A3][0];
        c.g[V1][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[A2][0] = 0x28;
        c.g[V0][0] = c.lw(replay_field_ea(c.g[V1][0], 0x30));
        c.sw(replay_field_ea(c.g[A3][0], 0x8), c.g[V0][0]);
        c.g[V1][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V0][0] = c.lw(replay_field_ea(c.g[V1][0], 0x34));
        c.sw(replay_field_ea(c.g[A3][0], 0xc), c.g[V0][0]);
        c.g[V1][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[T0][0] = c.lw(replay_field_ea(c.g[V1][0], 0x10));
        c.sw(replay_field_ea(c.g[A3][0], 0x24), c.g[T1][0]);
        c.sw(replay_field_ea(c.g[A3][0], 0x4), c.g[T0][0]);
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V1][0] = c.lw(replay_field_ea(c.g[V0][0], 0x20));
        c.sw(replay_field_ea(c.g[A3][0], 0x14), c.g[V1][0]);
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V1][0] = c.lw(replay_field_ea(c.g[V0][0], 0x24));
        c.sw(replay_field_ea(c.g[A3][0], 0x18), c.g[V1][0]);
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V1][0] = c.lw(replay_field_ea(c.g[V0][0], 0x28));
        c.sw(replay_field_ea(c.g[A3][0], 0x1c), c.g[V1][0]);
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V1][0] = c.lw(replay_field_ea(c.g[V0][0], 0x2c));
        c.sw(replay_field_ea(c.g[A3][0], 0x20), c.g[V1][0]);
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V1][0] = c.lw(replay_field_ea(c.g[V0][0], 0x28));
        c.sw(replay_field_ea(c.g[A3][0], 0x1c), c.g[V1][0]);
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V1][0] = c.lw(replay_field_ea(c.g[V0][0], 0x2c));
        Callee {
            addr: 0x0027_1058,
            f: ee::s07::f_00271058,
        }
        .call(c, 0x0027_1484)?;
        // The frame's block becomes the pending one; the queue moves on.
        c.g[A0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x65c));
        c.g[V0][0] = c.lw(replay_field_ea(c.g[A0][0], 0x4));
        c.sw(replay_field_ea(c.g[S0][0], 0x660), c.g[V0][0]);
        c.g[V1][0] = c.lw(replay_field_ea(c.g[A0][0], 0x10));
        c.sw(replay_field_ea(c.g[S0][0], 0x664), c.g[V1][0]);
        c.g[V1][0] = c.lw(replay_field_ea(c.g[A0][0], 0x14));
        let no_next = c.g[V1][0] == 0;
        c.sw(replay_field_ea(c.g[S0][0], 0x65c), c.g[V1][0]);
        if no_next {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            write_snapshot_return(c, frame);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(replay_field_ea(c.g[S0][0], 0x3d0));
        if c.g[V1][0] != c.g[V0][0] {
            // The next frame is not the save frame: leave without the +1.
            frame.restore(c, S0, 0x20);
            write_snapshot_return(c, frame);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(replay_field_ea(c.g[V1][0], 0x14));
        c.sw(replay_field_ea(c.g[S0][0], 0x65c), c.g[V0][0]);
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        write_snapshot_return(c, frame);
        Ok(())
    });
}

/// `Replay_StaticInitDestroy` (0x002718e8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn replay_static_init_destroy(c: &mut Cpu) {
    // no resume points: the generated body ignores `c.pc` and always starts at the
    // entry, so every pc runs the body below (the guard is not needed).
    guest(c, |c| {
        'b_2721a0: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.g[V0][0] = 0xffff;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
            let t = c.g[A1][0] != c.g[V0][0];
            c.sq(30, c.g[SP][0] as u32);
            if t {
                break 'b_2721a0;
            } else {
                c.cycles += 2;
                let t = c.g[A0][0] == 0u64;
                c.g[T5][0] = 0x4d0000;
                if t {
                    break 'b_2721a0;
                } else {
                    c.cycles += 544;
                    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4918));
                    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4914));
                    c.g[T6][0] = 0x4d0000;
                    c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4910));
                    c.g[T7][0] = 0x4d0000;
                    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x490c));
                    c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_add(0x3420));
                    c.swc1(9, (c.g[T5][0] as u32).wrapping_add(0x3400));
                    c.g[S1][0] = 0x4d0000;
                    c.swc1(10, (c.g[T6][0] as u32).wrapping_add(0x3410));
                    c.g[S2][0] = 0x4d0000;
                    c.swc1(11, (c.g[T7][0] as u32).wrapping_add(0x3420));
                    c.g[S3][0] = 0x4d0000;
                    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4908));
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3450));
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4904));
                    c.g[S5][0] = 0x4d0000;
                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4900));
                    c.g[S4][0] = 0x4d0000;
                    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[T3][0] = 0x4d0000;
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x48fc));
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x33f0));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_add(0x3400));
                    c.swc1(8, (c.g[S1][0] as u32).wrapping_add(0x3430));
                    c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_add(0x3410));
                    c.swc1(4, (c.g[S2][0] as u32).wrapping_add(0x3440));
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3430));
                    c.swc1(5, (c.g[S3][0] as u32).wrapping_add(0x3450));
                    c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3440));
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x48f8));
                    c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3460));
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x48f4));
                    c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_add(0x3470));
                    c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[T9][0] = 0x4d0000;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 6);
                    c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_add(0x3480));
                    c.swc1(3, (c.g[S4][0] as u32).wrapping_add(0x3460));
                    c.g[S6][0] = 0x4d0000;
                    c.swc1(1, (c.g[T3][0] as u32).wrapping_add(0x3470));
                    c.g[T7][0] = sx((c.g[S6][0] as u32).wrapping_add(0x34b0));
                    c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x33f0));
                    c.g[S2][0] = 0x4d0000;
                    c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x34d0));
                    c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[S3][0] = 0x4d0000;
                    c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x48f0));
                    c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x34e0));
                    c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x48ec));
                    c.g[T8][0] = 0x4d0000;
                    c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[FP][0] = 0x4d0000;
                    c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x48e8));
                    c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_add(0x34f0));
                    c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x48e4));
                    c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_add(0x3490));
                    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[S7][0] = 0x4d0000;
                    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[FP][0] = 0x4d0000;
                    c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = 0x4d0000;
                    c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3500));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_add(0x34a0));
                    c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x48e0));
                    c.g[V1][0] = 0x4d0000;
                    c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x48dc));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3540));
                    c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3560));
                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x48d8));
                    c.g[S0][0] = 0x4d0000;
                    c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
                    c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = 0x4d0000;
                    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3550));
                    c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = 0x4d0000;
                    c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3570));
                    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = 0x4d0000;
                    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3580));
                    c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x4d0000;
                    c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3590));
                    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = 0x4d0000;
                    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                    c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x48d4));
                    c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x35c0));
                    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = 0x4d0000;
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x48d0));
                    c.g[A2][0] = 0x4d0000;
                    c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x35a0));
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x48cc));
                    c.g[A2][0] = 0x4d0000;
                    c.swc1(2, (c.g[T9][0] as u32).wrapping_add(0x3480));
                    c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x35d0));
                    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x48c8));
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x35f0));
                    c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x48c4));
                    c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0xc));
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x48c0));
                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x48bc));
                    c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                    c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
                    c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x3490));
                    c.g[T2][0] = 0x4d0000;
                    c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
                    c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_add(0x3650));
                    c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0x34a0));
                    c.g[T2][0] = 0x4d0000;
                    c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x48b8));
                    c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x34b0));
                    c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x34c0));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34c0));
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x48b4));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = 0x4d0000;
                    c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x34d0));
                    c.g[T4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3670));
                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = 0x4d0000;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x48b0));
                    c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3680));
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x48ac));
                    c.g[S0][0] = 0x4d0000;
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x48a8));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x34e0));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[S0][0] = 0x4d0000;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x34f0));
                    c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = 0x4d0000;
                    c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[T5][0] = 0x4d0000;
                    c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x34c0));
                    c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
                    c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                    c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[T5][0] = 0x4d0000;
                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x36b0));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[S0][0] = 0x4d0000;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[S2][0] = sx((c.g[T5][0] as u32).wrapping_add(0x36d0));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[T7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x36f0));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[T5][0] = 0x4d0000;
                    c.g[S3][0] = 0x4d0000;
                    c.g[T6][0] = sx((c.g[T5][0] as u32).wrapping_add(0x3700));
                    c.g[S0][0] = 0x4d0000;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3530));
                    c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[T5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3710));
                    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = 0x4d0000;
                    c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x3560));
                    c.g[S0][0] = 0x4d0000;
                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x48a4));
                    c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3720));
                    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x48a0));
                    c.g[S0][0] = 0x4d0000;
                    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x489c));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x3500));
                    c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = 0x4d0000;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x3510));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3510));
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4898));
                    c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = 0x4d0000;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4894));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x3520));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3520));
                    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4890));
                    c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = 0x4d0000;
                    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x488c));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x3530));
                    c.g[S0][0] = 0x4d0000;
                    c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4888));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x3540));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = 0x4d0000;
                    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = 0x4d0000;
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4884));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3550));
                    c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = 0x4d0000;
                    c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3510));
                    c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x4));
                    c.swc1(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[V1][0] = 0x4d0000;
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4880));
                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x3570));
                    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = 0x4d0000;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x3580));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3520));
                    c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = 0x4d0000;
                    c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3530));
                    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = 0x4d0000;
                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = 0x4d0000;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = 0x4d0000;
                    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                    c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                    c.swc1(2, (c.g[A1][0] as u32).wrapping_add(0xc));
                    c.swc1(1, (c.g[T0][0] as u32).wrapping_add(0xc));
                    c.swc1(3, (c.g[FP][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x487c));
                    c.g[FP][0] = 0x4d0000;
                    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x3590));
                    c.swc1(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[FP][0] = 0x4d0000;
                    c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4878));
                    c.g[A0][0] = c.g[V1][0];
                    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4874));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x35b0));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x35a0));
                    c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[FP][0];
                    c.swc1(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35e0));
                    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4870));
                    c.g[A1][0] = 0x4d0000;
                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x35b0));
                    c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0x4d0000;
                    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                    c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x35c0));
                    c.g[A0][0] = 0x4d0000;
                    c.swc1(9, (c.g[T0][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x4d0000;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x35d0));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x35b0));
                    c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = 0x4d0000;
                    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x35e0));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[FP][0] = 0x4d0000;
                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x35f0));
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3660));
                    c.swc1(10, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.swc1(11, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x486c));
                    c.g[A2][0] = c.g[A0][0];
                    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3600));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3600));
                    c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = 0x4d0000;
                    c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[A0][0];
                    c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3620));
                    c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = 0x4d0000;
                    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.g[V0][0];
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4868));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3610));
                    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4864));
                    c.g[A2][0] = 0x4d0000;
                    c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x3640));
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4860));
                    c.g[S0][0] = c.g[A2][0];
                    c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3630));
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x485c));
                    c.swc1(2, (c.g[FP][0] as u32).wrapping_add(0xc));
                    c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                    c.g[FP][0] = 0x4d0000;
                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4858));
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3640));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3610));
                    c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = 0x4d0000;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x3620));
                    c.g[V1][0] = 0x4d0000;
                    c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.g[V1][0];
                    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x3630));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3660));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[A1][0] = 0x4d0000;
                    c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[A2][0] = 0x4d0000;
                    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4854));
                    c.g[S0][0] = 0x4d0000;
                    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4850));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3610));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3650));
                    c.g[FP][0] = 0x4d0000;
                    c.swc1(6, (c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3620));
                    c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = 0x4d0000;
                    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x484c));
                    c.g[A3][0] = 0x4d0000;
                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x3660));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3630));
                    c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = 0x4d0000;
                    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4848));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3640));
                    c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x3670));
                    c.swc1(9, (c.g[T4][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x4d0000;
                    c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x36a0));
                    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4844));
                    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x3680));
                    c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = 0x4d0000;
                    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x3690));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x36e0));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = 0x4d0000;
                    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[FP][0] = c.g[S0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x36c0));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V0][0] = 0x4d0000;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0x4d0000;
                    c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[A3][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3690));
                    c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = 0x4d0000;
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4840));
                    c.g[T2][0] = 0x4d0000;
                    c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0xc));
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x483c));
                    c.g[A1][0] = 0x4d0000;
                    c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                    c.swc1(1, (c.g[T7][0] as u32).wrapping_add(0xc));
                    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x36a0));
                    c.g[A0][0] = 0x4d0000;
                    c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x36b0));
                    c.g[A3][0] = 0x4d0000;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3690));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x36c0));
                    c.g[T2][0] = 0x4d0000;
                    c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x36a0));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x36d0));
                    c.g[S0][0] = c.g[V1][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x36e0));
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4838));
                    c.g[V0][0] = 0x3;
                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x36e0));
                    c.swc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4834));
                    c.g[A2][0] = 0x4d0000;
                    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x36f0));
                    c.swc1(4, (c.g[T7][0] as u32).wrapping_add(0x8));
                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4830));
                    c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x3700));
                    c.swc1(5, (c.g[T6][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x482c));
                    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4828));
                    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x3710));
                    c.swc1(6, (c.g[T5][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = 0x4d0000;
                    c.swc1(7, (c.g[T5][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3730));
                    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                    c.swc1(12, (c.g[T2][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = 0x4d0000;
                    c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3740));
                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[T2][0] = 0x4d0000;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[FP][0] as u32).wrapping_add(0x8), 0u64);
                    c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[FP][0] = 0x2;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[S2][0] = 0x1;
                    c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = 0x4d0000;
                    c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x2090), c.g[V0][0]);
                    c.g[T5][0] = 0x4d0000;
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x2078), 0u64);
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4824));
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x2080), c.g[S2][0]);
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x2088), c.g[FP][0]);
                    c.g[A1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2078));
                    c.g[V0][0] = c.g[S2][0] & 0xff;
                    c.g[A0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2090));
                    c.g[S2][0] = 0x4d0000;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4820));
                    c.g[V1][0] = c.g[FP][0] & 0xff;
                    c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[FP][0] = 0x4d0000;
                    c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] | 0x100;
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x481c));
                    c.g[V1][0] = c.g[V1][0] | 0x200;
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4818));
                    c.g[A0][0] = c.g[A0][0] | 0x300;
                    c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x3720));
                    c.swc1(3, (c.g[S3][0] as u32).wrapping_add(0x4));
                    c.g[T2][0] = 0x5;
                    c.swc1(4, (c.g[S3][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4814));
                    c.g[S3][0] = c.g[S2][0];
                    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4810));
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3750));
                    c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x3730));
                    c.swc1(5, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.g[T5][0] = 0x6;
                    c.swc1(6, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x480c));
                    c.g[A2][0] = c.g[FP][0];
                    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4808));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3760));
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x3740));
                    c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = 0x7;
                    c.swc1(8, (c.g[A3][0] as u32).wrapping_add(0x8));
                    c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x3750));
                    c.g[A3][0] = 0x4;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                    c.g[S2][0] = 0x8;
                    c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x3760));
                    c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.g[FP][0] = 0xa;
                    c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x2078), c.g[A1][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x2080), c.g[V0][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x2088), c.g[V1][0]);
                    c.g[V0][0] = 0xb;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V1][0] = 0xc;
                    c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[S3][0] = 0x9;
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x2074), 0u64);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x2070), 0u64);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x2090), c.g[A0][0]);
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x2098), c.g[A3][0]);
                    c.g[A0][0] = 0xd;
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x20a0), c.g[T2][0]);
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x20d0), c.g[V0][0]);
                    c.g[T0][0] = c.g[A3][0] & 0xff;
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x20e0), c.g[A0][0]);
                    c.g[T2][0] = c.g[T2][0] & 0xff;
                    c.g[A0][0] = c.g[V0][0] & 0xff;
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x20a8), c.g[T5][0]);
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x20b0), c.g[S0][0]);
                    c.g[T0][0] = c.g[T0][0] | 0x400;
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x20b8), c.g[S2][0]);
                    c.g[T3][0] = c.g[T5][0] & 0xff;
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x20c0), c.g[S3][0]);
                    c.g[T1][0] = c.g[S0][0] & 0xff;
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x20c8), c.g[FP][0]);
                    c.g[A3][0] = c.g[S2][0] & 0xff;
                    c.sb((c.g[GP][0] as u32).wrapping_add(0x20d8), c.g[V1][0]);
                    c.g[A2][0] = c.g[S3][0] & 0xff;
                    c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x20e0));
                    c.g[A1][0] = c.g[FP][0] & 0xff;
                    c.g[V1][0] = c.g[V1][0] & 0xff;
                    c.g[T2][0] = c.g[T2][0] | 0x500;
                    c.g[T3][0] = c.g[T3][0] | 0x600;
                    c.g[T1][0] = c.g[T1][0] | 0x700;
                    c.g[A3][0] = c.g[A3][0] | 0x800;
                    c.g[A2][0] = c.g[A2][0] | 0x900;
                    c.g[A1][0] = c.g[A1][0] | 0xa00;
                    c.g[A0][0] = c.g[A0][0] | 0xb00;
                    c.g[V1][0] = c.g[V1][0] | 0xc00;
                    c.g[V0][0] = c.g[V0][0] | 0xd00;
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x2098), c.g[T0][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x20a0), c.g[T2][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x20a8), c.g[T3][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x20b0), c.g[T1][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x20b8), c.g[A3][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x20c0), c.g[A2][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x20c8), c.g[A1][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x20d0), c.g[A0][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x20d8), c.g[V1][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x20e0), c.g[V0][0]);
                    break 'b_2721a0;
                }
            }
        }
        c.cycles += 11;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(30, c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `Replay_WriteSaveBlock` (0x002710a8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn replay_write_save_block(c: &mut Cpu) {
    if c.pc != 0x002710a8 {
        return replay_write_save_block_resume(c);
    }
    guest(c, |c| {
        'b_27120c: {
            'b_2710e0: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[V0][0] = 0xb;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A2][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x638));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[S2][0] = c.g[A1][0];
                if t {
                    break 'b_2710e0;
                } else {
                    c.cycles += 3;
                    c.sw(c.g[S1][0] as u32, 0u64);
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_27120c;
                    } else {
                        break 'b_2710e0;
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x63c));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x64c), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x650), c.g[V0][0]);
            'l_2710f0: loop {
                'b_2711fc: {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x002710f8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00270fc0,
                        f: ee::s07::Replay_FlushSaveOutput,
                    }
                    .call(c, 0x002710f8)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 16384i64) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                        break 'b_2711fc;
                    } else {
                        'b_2711e0: {
                            'b_2711dc: {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x638));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = (c.g[V0][0] < 0xcu64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x480000;
                                if t {
                                    break 'b_2711dc;
                                } else {
                                    'b_2711d4: {
                                        'b_2711c4: {
                                            'b_2711b4: {
                                                'b_2711a4: {
                                                    'b_271194: {
                                                        'b_271184: {
                                                            'b_271174: {
                                                                'b_271164: {
                                                                    'b_271154: {
                                                                        'b_271144: {
                                                                            c.cycles += 6;
                                                                            c.g[V1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    << 2);
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1820,
                                                                                    ));
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        c.g[V0][0]
                                                                                            as u32,
                                                                                    ));
                                                                            c.g[A0][0] = c
                                                                                .lw(c.g[V1][0]
                                                                                    as u32);
                                                                            let t =
                                                                                c.g[A0][0] as u32;
                                                                            match t {
                                                                                0x00271134 => {
                                                                                    c.cycles += 4;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x0027113c,
                                                                                    );
                                                                                    c.g[A0][0] =
                                                                                        c.g[S0][0];
                                                                                    Callee { addr: 0x00271228, f: ee::s07::Replay_SaveBuildHeader }.call(c, 0x0027113c)?;
                                                                                    let t = true;
                                                                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                                                                                    if t {
                                                                                        break 'b_2711e0;
                                                                                    } else {
                                                                                        break 'b_271144;
                                                                                    }
                                                                                }
                                                                                0x00271144 => {
                                                                                    break 'b_271144;
                                                                                }
                                                                                0x00271154 => {
                                                                                    break 'b_271154;
                                                                                }
                                                                                0x00271164 => {
                                                                                    break 'b_271164;
                                                                                }
                                                                                0x00271174 => {
                                                                                    break 'b_271174;
                                                                                }
                                                                                0x00271184 => {
                                                                                    break 'b_271184;
                                                                                }
                                                                                0x00271194 => {
                                                                                    break 'b_271194;
                                                                                }
                                                                                0x002711a4 => {
                                                                                    break 'b_2711a4;
                                                                                }
                                                                                0x002711b4 => {
                                                                                    break 'b_2711b4;
                                                                                }
                                                                                0x002711c4 => {
                                                                                    break 'b_2711c4;
                                                                                }
                                                                                0x002711d4 => {
                                                                                    break 'b_2711d4;
                                                                                }
                                                                                0x002711dc => {
                                                                                    break 'b_2711dc;
                                                                                }
                                                                                _ => {
                                                                                    c.jump_out(
                                                                                        t,
                                                                                        0x002710a8,
                                                                                        0x00271224,
                                                                                    );
                                                                                    return Ok(());
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 4;
                                                                        c.g[RA][0] = sx(0x0027114c);
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        Callee { addr: 0x002712c0, f: ee::s07::Replay_SaveWriteHeader }.call(c, 0x0027114c)?;
                                                                        let t = true;
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x64c));
                                                                        if t {
                                                                            break 'b_2711e0;
                                                                        } else {
                                                                            break 'b_271154;
                                                                        }
                                                                    }
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x0027115c);
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    Callee { addr: 0x00271348, f: ee::s07::Replay_SaveBeginSnapshots }.call(c, 0x0027115c)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x64c));
                                                                    if t {
                                                                        break 'b_2711e0;
                                                                    } else {
                                                                        break 'b_271164;
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x0027116c);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                Callee { addr: 0x00271380, f: ee::s07::Replay_SaveWriteSnapshot }.call(c, 0x0027116c)?;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x64c));
                                                                if t {
                                                                    break 'b_2711e0;
                                                                } else {
                                                                    break 'b_271174;
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x0027117c);
                                                            c.g[A0][0] = c.g[S0][0];
                                                            Callee {
                                                                addr: 0x002714e0,
                                                                f: ee::s07::Replay_SaveBeginCaches,
                                                            }
                                                            .call(c, 0x0027117c)?;
                                                            let t = true;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x64c));
                                                            if t {
                                                                break 'b_2711e0;
                                                            } else {
                                                                break 'b_271184;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x0027118c);
                                                        c.g[A0][0] = c.g[S0][0];
                                                        Callee {
                                                            addr: 0x00271510,
                                                            f: ee::s07::Replay_SaveWriteCache,
                                                        }
                                                        .call(c, 0x0027118c)?;
                                                        let t = true;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x64c));
                                                        if t {
                                                            break 'b_2711e0;
                                                        } else {
                                                            break 'b_271194;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x0027119c);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    Callee {
                                                        addr: 0x00271620,
                                                        f: ee::s07::Replay_SaveComputePadding,
                                                    }
                                                    .call(c, 0x0027119c)?;
                                                    let t = true;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x64c)
                                                        );
                                                    if t {
                                                        break 'b_2711e0;
                                                    } else {
                                                        break 'b_2711a4;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x002711ac);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x00271688,
                                                    f: ee::s07::Replay_SaveWritePadding,
                                                }
                                                .call(c, 0x002711ac)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                                                if t {
                                                    break 'b_2711e0;
                                                } else {
                                                    break 'b_2711b4;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x002711bc);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x002716f0,
                                                f: ee::s07::Replay_NextSaveBlock,
                                            }
                                            .call(c, 0x002711bc)?;
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                                            if t {
                                                break 'b_2711e0;
                                            } else {
                                                break 'b_2711c4;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x002711cc);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x00271708,
                                            f: ee::s07::Replay_SaveWriteFooter,
                                        }
                                        .call(c, 0x002711cc)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                                        if t {
                                            break 'b_2711e0;
                                        } else {
                                            break 'b_2711d4;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x638), c.g[V0][0]);
                                    break 'b_2711dc;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                            break 'b_2711e0;
                        }
                        'b_2711f8: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 16384i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0xb;
                            if t {
                                break 'b_2711f8;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x638));
                                let t = c.g[V1][0] != c.g[V0][0];

                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002710f0);
                                    }
                                    continue 'l_2710f0;
                                } else {
                                    break 'b_2711f8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                        break 'b_2711fc;
                    }
                }
                c.cycles += 4;
                c.g[V0][0] = 0x1;
                c.sw(c.g[S1][0] as u32, c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x63c));
                c.sw(c.g[S2][0] as u32, c.g[V1][0]);
                break 'b_27120c;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// Native resume of `replay_write_save_block` (converted from the `entries` body): any entry
/// other than 0x002710a8, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn replay_write_save_block_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002710a8 => {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V0][0] = 0xb;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A2][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x638));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[S2][0] = c.g[A1][0];
                    if t {
                        pc = 0x002710e0;
                        continue;
                    }
                    pc = 0x002710d4;
                    continue;
                }
                0x002710d4 => {
                    c.cycles += 3;
                    c.sw((c.g[S1][0] as u32), 0u64);
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0027120c;
                        continue;
                    }
                    pc = 0x002710e0;
                    continue;
                }
                0x002710e0 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x63c));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x64c), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x650), c.g[V0][0]);
                    pc = 0x002710f0;
                    continue;
                }
                0x002710f0 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x002710f8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00270fc0,
                        f: ee::s07::Replay_FlushSaveOutput,
                    }
                    .call(c, 0x002710f8)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 16384i64) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                        pc = 0x002711fc;
                        continue;
                    }
                    pc = 0x00271108;
                    continue;
                }
                0x00271108 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x638));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = (c.g[V0][0] < 0xcu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002711dc;
                        continue;
                    }
                    pc = 0x0027111c;
                    continue;
                }
                0x0027111c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1820));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x00271134 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0027113c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00271228,
                        f: ee::s07::Replay_SaveBuildHeader,
                    }
                    .call(c, 0x0027113c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    if t {
                        pc = 0x002711e0;
                        continue;
                    }
                    pc = 0x00271144;
                    continue;
                }
                0x00271144 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0027114c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002712c0,
                        f: ee::s07::Replay_SaveWriteHeader,
                    }
                    .call(c, 0x0027114c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    if t {
                        pc = 0x002711e0;
                        continue;
                    }
                    pc = 0x00271154;
                    continue;
                }
                0x00271154 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0027115c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00271348,
                        f: ee::s07::Replay_SaveBeginSnapshots,
                    }
                    .call(c, 0x0027115c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    if t {
                        pc = 0x002711e0;
                        continue;
                    }
                    pc = 0x00271164;
                    continue;
                }
                0x00271164 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0027116c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00271380,
                        f: ee::s07::Replay_SaveWriteSnapshot,
                    }
                    .call(c, 0x0027116c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    if t {
                        pc = 0x002711e0;
                        continue;
                    }
                    pc = 0x00271174;
                    continue;
                }
                0x00271174 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0027117c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002714e0,
                        f: ee::s07::Replay_SaveBeginCaches,
                    }
                    .call(c, 0x0027117c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    if t {
                        pc = 0x002711e0;
                        continue;
                    }
                    pc = 0x00271184;
                    continue;
                }
                0x00271184 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0027118c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00271510,
                        f: ee::s07::Replay_SaveWriteCache,
                    }
                    .call(c, 0x0027118c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    if t {
                        pc = 0x002711e0;
                        continue;
                    }
                    pc = 0x00271194;
                    continue;
                }
                0x00271194 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0027119c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00271620,
                        f: ee::s07::Replay_SaveComputePadding,
                    }
                    .call(c, 0x0027119c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    if t {
                        pc = 0x002711e0;
                        continue;
                    }
                    pc = 0x002711a4;
                    continue;
                }
                0x002711a4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002711ac);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00271688,
                        f: ee::s07::Replay_SaveWritePadding,
                    }
                    .call(c, 0x002711ac)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    if t {
                        pc = 0x002711e0;
                        continue;
                    }
                    pc = 0x002711b4;
                    continue;
                }
                0x002711b4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002711bc);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002716f0,
                        f: ee::s07::Replay_NextSaveBlock,
                    }
                    .call(c, 0x002711bc)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    if t {
                        pc = 0x002711e0;
                        continue;
                    }
                    pc = 0x002711c4;
                    continue;
                }
                0x002711c4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002711cc);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00271708,
                        f: ee::s07::Replay_SaveWriteFooter,
                    }
                    .call(c, 0x002711cc)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    if t {
                        pc = 0x002711e0;
                        continue;
                    }
                    pc = 0x002711d4;
                    continue;
                }
                0x002711d4 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x638), c.g[V0][0]);
                    pc = 0x002711dc;
                    continue;
                }
                0x002711dc => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    pc = 0x002711e0;
                    continue;
                }
                0x002711e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 16384i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0xb;
                    if t {
                        pc = 0x002711f8;
                        continue;
                    }
                    pc = 0x002711ec;
                    continue;
                }
                0x002711ec => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x638));
                    let t = c.g[V1][0] != c.g[V0][0];

                    if t {
                        if back_edge_unwinds(c, 0x002710f0) {
                            return Ok(());
                        }
                        pc = 0x002710f0;
                        continue;
                    }
                    pc = 0x002711f8;
                    continue;
                }
                0x002711f8 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64c));
                    pc = 0x002711fc;
                    continue;
                }
                0x002711fc => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S1][0] as u32), c.g[A0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x63c));
                    c.sw((c.g[S2][0] as u32), c.g[V1][0]);
                    pc = 0x0027120c;
                    continue;
                }
                0x0027120c => {
                    c.cycles += 6;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002710a8, 0x00271224);
                    return Ok(());
                }
            }
        }
    });
}

/// `Highlight* Replay_FindWeakestHighlight(Replay* self, int i, Highlight*
/// best, int positive)` (0x270c58): scan the highlight list for the entry
/// with the weakest value in column `i` (`positive`: the strongest). Leaf: no
/// frame, no calls. Returns `best` (or the element found) in v0.
pub fn replay_find_weakest_highlight_2(c: &mut Cpu) {
    'b_270ce0: {
        c.cycles += 3;
        c.g[T0][0] = Addr(r32(c, A0).wrapping_add(0x3b0)).lw(c);
        let empty = c.g[T0][0] == 0;
        c.g[T1][0] = c.g[A2][0];
        if empty {
            break 'b_270ce0;
        }
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, T0).wrapping_add(0x1e)).lbu(c);
        'l_270c68: loop {
            'b_270cd8: {
                c.cycles += 4;
                c.g[V0][0] =
                    sx((((c.g[V0][0] as u32) as i32) >> ((c.g[A1][0] as u32) & 31)) as u32);
                c.g[V0][0] &= 1;
                if c.g[V0][0] == 0 {
                    c.g[T0][0] = Addr(r32(c, T0).wrapping_add(0x14)).lw(c);
                    break 'b_270cd8;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, T0).wrapping_add(0x1f)).lbu(c);
                if c.g[V0][0] != 0 {
                    c.g[T0][0] = Addr(r32(c, T0).wrapping_add(0x14)).lw(c);
                    break 'b_270cd8;
                }
                'b_270cd4: {
                    'b_270cc4: {
                        'b_270ca8: {
                            c.cycles += 2;
                            let positive = c.g[A3][0] == 0;
                            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                            if positive {
                                break 'b_270ca8;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = sx(r32(c, T0).wrapping_add(c.g[V1][0] as u32));
                            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x20)).lw(c);
                            c.g[V1][0] = sx(r32(c, A2).wrapping_add(c.g[V1][0] as u32));
                            if (c.g[A0][0] as i64) <= 0 {
                                break 'b_270cd4;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x20)).lw(c);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                            break 'b_270cc4;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, T0).wrapping_add(c.g[V1][0] as u32));
                        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x20)).lw(c);
                        if (c.g[A0][0] as i64) >= 0 {
                            c.g[T0][0] = Addr(r32(c, T0).wrapping_add(0x14)).lw(c);
                            break 'b_270cd8;
                        }
                        c.cycles += 3;
                        c.g[V1][0] = sx(r32(c, A2).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x20)).lw(c);
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[T0][0] = Addr(r32(c, T0).wrapping_add(0x14)).lw(c);
                        break 'b_270cd8;
                    }
                    c.cycles += 2;
                    c.g[T1][0] = c.g[T0][0];
                    c.g[A2][0] = c.g[T1][0];
                }
                c.cycles += 1;
                c.g[T0][0] = Addr(r32(c, T0).wrapping_add(0x14)).lw(c);
            }
            c.cycles += 2;
            if c.g[T0][0] != 0 {
                c.g[V0][0] = Addr(r32(c, T0).wrapping_add(0x1e)).lbu(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_0c68);
                }
                continue 'l_270c68;
            }
            break 'b_270ce0;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T1][0];
    abi::ret(c);
}

// Callees, by their generated shard functions (each goes through its override
// slot, so a replaced callee is reached too).

/// `InSeq_GetInstance` (0x272cc0): the singleton sequencer.
pub(crate) const IS_GET_INSTANCE: Callee = Callee {
    addr: 0x0027_2cc0,
    f: ee::s07::f_00272cc0,
};

/// `InSeq_InstanceRunToEnd` (0x274868).
pub(crate) const IS_INSTANCE_RUN_TO_END: Callee = Callee {
    addr: 0x0027_4868,
    f: ee::s07::f_00274868,
};

/// `InSeq_RunToEnd(a0, a1, flag)` (0x272aa0): runs the singleton's `RunToEnd`
/// on `InSeq_GetInstance(a0, a1)`, with `flag` (a2) passed through as the
/// last argument. Frame 0x20: saves ra and s0.
pub fn in_seq_run_to_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        IS_GET_INSTANCE.call(c, 0x0027_2ab4)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        IS_INSTANCE_RUN_TO_END.call(c, 0x0027_2ac0)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The entity's `mIsPlaying` flag (0x28), `mpSystemHeader` (0x1c),
/// `mpChannelPlayers` (0x24) and `mpEntityComm` (0x2c) of the channel restart.
pub(crate) const ICR_HEADER: u32 = 0x1c;

pub(crate) const ICR_PLAYERS: u32 = 0x24;

pub(crate) const ICR_PLAYING: u32 = 0x28;

pub(crate) const ICR_COMM: u32 = 0x2c;

/// The sequencer `self` pointer, as the register holds it (s1, read after
/// each call).
#[inline(always)]
pub(crate) fn icr_self(c: &Cpu) -> u32 {
    c.g[S1][0] as u32
}

/// `u32 InSeq_ChannelRestart(self)` (0x273f68): restarts a playing entity.
/// Stops each channel player, plays its channel record again, and tells the
/// entity comm about the events the channels raised. Returns whether any
/// channel changed (0 if the entity is not playing). Frame 0xa0.
pub fn in_seq_channel_restart(c: &mut Cpu) {
    guest(c, icr_body)
}

pub(crate) fn icr_body(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0xa0);
    frame.save(c, 17, 0x80);
    frame.save(c, 16, 0x90);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, 18, 0x70);
    frame.save(c, 19, 0x60);
    frame.save(c, 20, 0x50);
    frame.save(c, 21, 0x40);
    frame.save(c, 22, 0x30);
    frame.save(c, 23, 0x20);
    frame.save_ra(c, 0x10);

    c.g[V0][0] = Addr(icr_self(c).wrapping_add(ICR_PLAYING)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(icr_self(c).wrapping_add(ICR_HEADER)).lw(c);
        icr_channels(c)?;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }

    c.cycles += 11;
    frame.restore(c, 16, 0x90);
    frame.restore(c, 17, 0x80);
    frame.restore(c, 18, 0x70);
    frame.restore(c, 19, 0x60);
    frame.restore(c, 20, 0x50);
    frame.restore(c, 21, 0x40);
    frame.restore(c, 22, 0x30);
    frame.restore(c, 23, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// The channel loop and the entity callbacks, from the system header in V0.
/// The loop stops and replays each channel (`S3` = index, `S2` = channel
/// record, `S6` = event mask, `S7` = changed); then `OnInSeqKeyEvent` when
/// there are events and `OnInSeqUpdate` for the entity comm.
pub(crate) fn icr_channels(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    c.g[S6][0] = 0;
    c.g[S3][0] = 0;
    c.g[S7][0] = 0;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(2)).lbu(c);
    let no_channels = c.g[A0][0] == 0;
    c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));

    if !no_channels {
        c.cycles += 1;
        loop {
            c.cycles += 26;
            c.g[V1][0] = Addr(icr_self(c).wrapping_add(ICR_PLAYERS)).lw(c);
            c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
            c.g[S4][0] = c.g[S2][0];
            c.g[S5][0] = 0;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
            c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(4)).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
            let stop = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, stop, 0x0027_3ff8)?; // player->Stop()

            c.g[V0][0] = Addr(icr_self(c).wrapping_add(ICR_PLAYERS)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            Addr(sp(c)).sw(c, 0); // channelEvents = 0
            c.g[A2][0] = c.g[S1][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A3][0] = c.g[SP][0];
            c.g[T0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(4)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
            let play = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, play, 0x0027_4028)?; // player->Play(data, self, &events)

            if c.g[V0][0] != 0 {
                c.g[S5][0] = 1;
                c.cycles += 1;
                c.g[A0][0] = Addr(icr_self(c).wrapping_add(ICR_HEADER)).lw(c);
            } else {
                c.cycles += 2;
                if c.g[S7][0] == 0 {
                    c.g[A0][0] = Addr(icr_self(c).wrapping_add(ICR_HEADER)).lw(c);
                } else {
                    c.cycles += 1;
                    c.g[S5][0] = 1;
                    c.cycles += 1;
                    c.g[A0][0] = Addr(icr_self(c).wrapping_add(ICR_HEADER)).lw(c);
                }
            }

            c.cycles += 9;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            c.g[A1][0] = Addr((c.g[S4][0] as u32).wrapping_add(2)).lhu(c);
            c.g[S7][0] = c.g[S5][0];
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(2)).lbu(c);
            c.g[V1][0] = Addr(sp(c)).lw(c);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S6][0] |= c.g[V1][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0027_3fc8);
                }
                continue;
            }
            break;
        }
    }

    c.cycles += 3;
    c.g[T1][0] = Addr(icr_self(c).wrapping_add(ICR_COMM)).lw(c);
    let no_comm = c.g[T1][0] == 0;
    c.g[V0][0] = c.g[S7][0];
    if !no_comm {
        c.cycles += 2;
        c.g[A1][0] = c.g[S1][0];
        if c.g[S6][0] != 0 {
            c.cycles += 9;
            c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
            c.g[V1][0] = Addr(icr_self(c).wrapping_add(ICR_HEADER)).lw(c);
            c.g[A3][0] = c.g[S6][0];
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lh(c);
            c.g[T0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1c)).lw(c);
            c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(3)).lbu(c);
            c.mtc1(0, 12);
            let key_event = c.g[T0][0] as u32;
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, key_event, 0x0027_409c)?; // comm->OnInSeqKeyEvent
        }
        c.cycles += 13;
        c.g[T0][0] = Addr(icr_self(c).wrapping_add(ICR_COMM)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[T1][0] = Addr(icr_self(c).wrapping_add(ICR_HEADER)).lw(c);
        c.g[A3][0] = c.g[S6][0];
        c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
        c.mtc1(0, 12);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
        c.mov_s(13, 12);
        c.g[A2][0] = Addr((c.g[T1][0] as u32).wrapping_add(3)).lbu(c);
        let update = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, update, 0x0027_40cc)?; // comm->OnInSeqUpdate
        c.g[V0][0] = c.g[S7][0];
    }
    Ok(())
}

/// `void Replay_StepFrame(Replay* self, int reset)` (0x2703f0): step the
/// replay player one frame. With `reset` it re-finds the current snapshot
/// (`+0x488`), skips a finished one, re-seeks every run list at `+0x48c` and
/// restores the snapshot; without it the tick (`+0x484`) advances, and past
/// the last tick (`+0x3cc`) the replay either goes to state 3 or rewinds,
/// steps once more with `reset = 1` and redraws the camera and views.
pub fn replay_step_frame_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        let no_step = c.g[S1][0] == 0;
        frame.save_ra(c, 0);
        'b_27041c: {
            if no_step {
                break 'b_27041c;
            }
            c.cycles += 3;
            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x484)).lw(c);
            Callee {
                addr: 0x0026_e448,
                f: ee::s07::f_0026e448,
            }
            .call(c, 0x0027_0418)?;
            let v = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(0x488)).sw(c, v);
        }
        'b_27043c: {
            c.cycles += 3;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x488)).lw(c);
            if c.g[V1][0] == 0 {
                break 'b_27043c;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x3d0)).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                break 'b_27043c;
            }
            c.cycles += 2;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
            let v = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(0x488)).sw(c, v);
        }
        'b_270520: {
            'b_27048c: {
                c.cycles += 2;
                let flag_zero = c.g[S1][0] == 0;
                c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x48c));
                if flag_zero {
                    break 'b_27048c;
                }
                c.cycles += 5;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x488)).lw(c);
                c.g[S2][0] = 1;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x30)).lw(c);
                let v = c.g[V0][0];
                Addr(r32(c, S0).wrapping_add(0x484)).sw(c, v);
                'l_270458: loop {
                    'b_270470: {
                        c.cycles += 3;
                        c.g[A0][0] = Addr(r32(c, S1)).lw(c);
                        if c.g[A0][0] == 0 {
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                            break 'b_270470;
                        }
                        c.cycles += 3;
                        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x484)).lw(c);
                        Callee {
                            addr: 0x0026_d4d8,
                            f: ee::s07::f_0026d4d8,
                        }
                        .call(c, 0x0027_046c)?;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    }
                    c.cycles += 2;
                    let go_on = (c.g[S2][0] as i64) >= 0;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                    if go_on {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_0458);
                        }
                        continue 'l_270458;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = Addr(r32(c, A0).wrapping_add(0x488)).lw(c);
                    Callee {
                        addr: 0x0026_db88,
                        f: ee::s07::f_0026db88,
                    }
                    .call(c, 0x0027_0484)?;
                    frame.restore(c, S0, 0x30);
                    break 'b_270520;
                }
            }
            'b_270518: {
                c.cycles += 5;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x484)).lw(c);
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x3cc)).lw(c);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let below = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                if below {
                    break 'b_270518;
                }
                'b_270504: {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x61c)).lw(c);
                    if c.g[V0][0] == 0 {
                        let v = c.g[A0][0];
                        Addr(r32(c, S0).wrapping_add(0x484)).sw(c, v);
                        break 'b_270504;
                    }
                    c.cycles += 22;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    Callee {
                        addr: 0x0027_02f8,
                        f: ee::s07::f_002702f8,
                    }
                    .call(c, 0x0027_04b8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 1;
                    Callee {
                        addr: 0x0027_03f0,
                        f: ee::s07::f_002703f0,
                    }
                    .call(c, 0x0027_04c4)?;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[S1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                    c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa8)).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0016_20d0, 0x0027_04e0)?;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x14)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0027_04f4)?;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0022_e840, 0x0027_04fc)?;
                    frame.restore(c, S0, 0x30);
                    break 'b_270520;
                }
                c.cycles += 5;
                c.g[A1][0] = 3;
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0026_f4a0,
                    f: ee::s07::f_0026f4a0,
                }
                .call(c, 0x0027_0510)?;
                frame.restore(c, S0, 0x30);
                break 'b_270520;
            }
            c.cycles += 2;
            let v = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(0x484)).sw(c, v);
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_OnRiderEvent(Replay* self, int rider, int kind)` (0x2709d8): a
/// rider event while a replay is recorded. Ignored while the replay is busy
/// (`+0x61c`, `+0x620`, `+0x610`, `+0x60c` or the state word at 0 set);
/// otherwise the rider's index is looked up and kind 0 runs the empty rider
/// hook and kind 1 begins a highlight (with the rider's `+0x790` field).
pub fn replay_on_rider_event_3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        'b_270a9c: {
            'b_270a98: {
                frame.save(c, S0, 0x30);
                frame.save(c, S1, 0x20);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x10);
                c.g[S1][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x61c));
                let busy = c.g[V0][0] != 0;
                c.g[S2][0] = c.g[A2][0];
                if busy {
                    break 'b_270a98;
                }
                for off in [0x620u32, 0x610, 0x60c, 0] {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(off));
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x30);
                        break 'b_270a9c;
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0027_0730,
                    f: ee::s07::f_00270730,
                }
                .call(c, 0x0027_0a38)?;
                c.g[A3][0] = c.g[V0][0];
                let negative = (c.g[A3][0] as i64) < 0;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if negative {
                    break 'b_270a98;
                }
                'b_270a88: {
                    'b_270a70: {
                        c.cycles += 6;
                        c.g[A1][0] = sx(r32(c, S1) << 2);
                        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x84));
                        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                        let kind_zero = c.g[S2][0] == 0;
                        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x28));
                        if kind_zero {
                            break 'b_270a70;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        if c.g[S2][0] == c.g[V0][0] {
                            c.g[A2][0] = c.lw(r32(c, A0).wrapping_add(0x790));
                            break 'b_270a88;
                        }
                        c.cycles += 2;
                        frame.restore(c, S0, 0x30);
                        break 'b_270a9c;
                    }
                    // kind 0: the empty rider hook.
                    c.cycles += 6;
                    c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(0x790));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0xfc));
                    Callee {
                        addr: 0x0027_05a8,
                        f: ee::s07::f_002705a8,
                    }
                    .call(c, 0x0027_0a80)?;
                    frame.restore(c, S0, 0x30);
                    break 'b_270a9c;
                }
                // kind 1: begin the rider's highlight.
                c.cycles += 4;
                c.g[A1][0] = c.g[A3][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0xfc));
                Callee {
                    addr: 0x0027_0b88,
                    f: ee::s07::f_00270b88,
                }
                .call(c, 0x0027_0a98)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int Replay_LoadSaved(Replay* self, ReplaySave* save)` (0x271758): rebuild
/// the replay from a saved one. Returns 0 when the save's world mode
/// (`save+0x18`) is not the game's; else resets the snapshot list (`+0x3b0`),
/// reads the save's snapshots, then each cache slot (`+0x48c`..) that is set,
/// enters state 8 and returns 1.
pub fn replay_load_saved_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        c.g[S4][0] = c.g[SP][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x84));
        c.g[A2][0] = c.lw(r32(c, S3).wrapping_add(0x18));
        let a1 = c.g[A1][0];
        c.sw(frame.at(c, 0), a1);
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x7c));
        let mismatch = c.g[A2][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        'b_271880: {
            if mismatch {
                break 'b_271880;
            }
            'b_271838: {
                c.cycles += 22;
                c.g[V0][0] = 1;
                c.g[A0][0] = sx(r32(c, S2).wrapping_add(0x3b0));
                let v = c.g[V0][0];
                c.sw(r32(c, S2).wrapping_add(0x608), v);
                c.g[S5][0] = c.g[A0][0];
                c.g[S1][0] = 0;
                c.g[V1][0] = c.lw(r32(c, S3).wrapping_add(0x8));
                let v = c.g[V1][0];
                c.sw(r32(c, S2).wrapping_add(0x18), v);
                c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0xc));
                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                let v = c.g[V0][0];
                c.sw(r32(c, S2).wrapping_add(0x610), v);
                c.g[V1][0] = c.lw(r32(c, S3).wrapping_add(0x10));
                let v = c.g[V1][0];
                c.sw(r32(c, S2).wrapping_add(0x3c8), v);
                c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x14));
                c.sw(r32(c, S2).wrapping_add(0x3d0), 0);
                c.sw(r32(c, S2).wrapping_add(0x484), 0);
                let v = c.g[V0][0];
                c.sw(r32(c, S2).wrapping_add(0x3cc), v);
                Callee {
                    addr: 0x0026_e5c0,
                    f: ee::s07::f_0026e5c0,
                }
                .call(c, 0x0027_17e8)?;
                c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x28));
                let v = c.g[V0][0];
                c.sw(frame.at(c, 0), v);
                c.g[V1][0] = c.lw(r32(c, S3).wrapping_add(0x4));
                if (c.g[V1][0] as i64) <= 0 {
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x18));
                    break 'b_271838;
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[S2][0];
                // The saved snapshots, one allocation each.
                loop {
                    c.cycles += 13;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    Callee {
                        addr: 0x0027_0130,
                        f: ee::s07::f_00270130,
                    }
                    .call(c, 0x0027_1808)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0026_d678,
                        f: ee::s07::f_0026d678,
                    }
                    .call(c, 0x0027_1818)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0026_e5d0,
                        f: ee::s07::f_0026e5d0,
                    }
                    .call(c, 0x0027_1824)?;
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.g[S2][0];
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_1800);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x18));
                    break 'b_271838;
                }
            }
            'b_271870: {
                c.cycles += 2;
                let none = (c.g[V0][0] as i64) <= 0;
                c.g[S1][0] = 0;
                if none {
                    break 'b_271870;
                }
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S2).wrapping_add(0x48c));
                // The cache slots that are set.
                loop {
                    'b_271860: {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw(r32(c, S0));
                        if c.g[A0][0] == 0 {
                            c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x18));
                            break 'b_271860;
                        }
                        c.cycles += 3;
                        c.g[A1][0] = c.g[S4][0];
                        Callee {
                            addr: 0x0026_d558,
                            f: ee::s07::f_0026d558,
                        }
                        .call(c, 0x0027_185c)?;
                        c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x18));
                    }
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_1848);
                        }
                        continue;
                    }
                    break 'b_271870;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = 1;
            c.g[V1][0] = 8;
            let v = c.g[V0][0];
            c.sw(r32(c, S2).wrapping_add(0x618), v);
            let v = c.g[V1][0];
            c.sw(r32(c, S2), v);
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `Replay_RecycleOldestSnapshot` (0x002701a0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn replay_recycle_oldest_snapshot_2(c: &mut Cpu) {
    if c.pc != 0x002701a0 {
        return replay_recycle_oldest_snapshot_2_resume(c);
    }
    guest(c, |c| {
        c.cycles += 4;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[A2][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A3][0] = 0u64;
        // 0x2701b0: two passes over the list head at +0x3ac.
        loop {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3ac));
            // 0x2701f4: walk while the node's chain is at least three deep.
            loop {
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3b0));
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3b0));
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                if c.g[V0][0] == 0u64 {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3b0));
                    break;
                }
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3ac));
                if back_edge_unwinds(c, 0x002701b8) {
                    return Ok(());
                }
                // 0x2701b8
                c.cycles += 4;
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1f));
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.g[V0][0];
                if t {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1e));
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                    } else {
                        // 0x2701d4: free the snapshot and leave.
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                        c.g[A0][0] = c.g[A2][0];
                        c.g[RA][0] = sx(0x002701e4);
                        c.sw((c.g[A2][0] as u32).wrapping_add(0x3ac), c.g[V0][0]);
                        Callee {
                            addr: 0x00270238,
                            f: ee::s07::Replay_FreeSnapshot,
                        }
                        .call(c, 0x002701e4)?;
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.pc = t;
                        return Ok(());
                    }
                }
                // 0x2701f0
                c.cycles += 1;
                c.sw((c.g[A2][0] as u32).wrapping_add(0x3ac), c.g[V0][0]);
            }
            // 0x270218
            c.cycles += 4;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[A3][0] as i64) < 2i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.sw((c.g[A2][0] as u32).wrapping_add(0x3ac), c.g[V1][0]);
            if !t {
                break;
            }
            if back_edge_unwinds(c, 0x002701b0) {
                return Ok(());
            }
        }
        c.cycles += 1;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    });
}

/// Native resume of `replay_recycle_oldest_snapshot_2` (converted from the `generated` body): any entry
/// other than 0x002701a0, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn replay_recycle_oldest_snapshot_2_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002701a0 => {
                    c.cycles += 4;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.g[A2][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[A3][0] = 0u64;
                    pc = 0x002701b0;
                    continue;
                }
                0x002701b0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3ac));
                    if t {
                        pc = 0x002701f4;
                        continue;
                    }
                    pc = 0x002701b8;
                    continue;
                }
                0x002701b8 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1f));
                    let t = c.g[V1][0] != 0u64;
                    c.g[A0][0] = c.g[V0][0];
                    if t {
                        pc = 0x002701ec;
                        continue;
                    }
                    pc = 0x002701c8;
                    continue;
                }
                0x002701c8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1e));

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                        pc = 0x002701f0;
                        continue;
                    }
                    pc = 0x002701d4;
                    continue;
                }
                0x002701d4 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = c.g[A2][0];
                    c.g[RA][0] = sx(0x002701e4);
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x3ac), c.g[V0][0]);
                    Callee {
                        addr: 0x00270238,
                        f: ee::s07::Replay_FreeSnapshot,
                    }
                    .call(c, 0x002701e4)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    if t {
                        pc = 0x0027022c;
                        continue;
                    }
                    pc = 0x002701ec;
                    continue;
                }
                0x002701ec => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                    pc = 0x002701f0;
                    continue;
                }
                0x002701f0 => {
                    c.cycles += 1;
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x3ac), c.g[V0][0]);
                    pc = 0x002701f4;
                    continue;
                }
                0x002701f4 => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3b0));
                        pc = 0x00270218;
                        continue;
                    }
                    pc = 0x002701fc;
                    continue;
                }
                0x002701fc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3b0));
                        pc = 0x00270218;
                        continue;
                    }
                    pc = 0x00270208;
                    continue;
                }
                0x00270208 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3ac));
                        if back_edge_unwinds(c, 0x002701b8) {
                            return Ok(());
                        }
                        pc = 0x002701b8;
                        continue;
                    }
                    pc = 0x00270214;
                    continue;
                }
                0x00270214 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x3b0));
                    pc = 0x00270218;
                    continue;
                }
                0x00270218 => {
                    c.cycles += 4;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 2i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x3ac), c.g[V1][0]);
                    if t {
                        if back_edge_unwinds(c, 0x002701b0) {
                            return Ok(());
                        }
                        pc = 0x002701b0;
                        continue;
                    }
                    pc = 0x00270228;
                    continue;
                }
                0x00270228 => {
                    c.cycles += 1;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    pc = 0x0027022c;
                    continue;
                }
                0x0027022c => {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002701a0, 0x00270234);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::game::intersect::riderspheretree::tests::oc430_install_fakes;

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

    pub(crate) use crate::game::animation::rideranim::tests::CTRL;
    pub(crate) use crate::game::animation::rideranim::tests::MAGIC;

    pub(crate) use crate::game::animation::rideranim::tests::active;
    pub(crate) use crate::game::animation::rideranim::tests::chain;
    pub(crate) use crate::game::animation::rideranim::tests::fake_ret;
    pub(crate) use crate::game::animation::rideranim::tests::install;

    /// `Replay_OnRiderWipeout`: the find-rider world of
    /// `Replay_FindRiderIndex`'s test, each busy flag set or all clear, the
    /// rider's highlight slot free (the real `Replay_EndHighlight` returns
    /// early), and the real `Replay_CheckInstantReplay` with instant replays
    /// armed or not, the wipeout time close to or past the last one.
    #[test]
    fn replay_on_rider_wipeout_matches() {
        use crate::game::replay::replaycache::tests::replay_find_rider_fixture;
        check(
            ee::s07::f_002708f0__generated,
            replay_on_rider_wipeout,
            300,
            &|c, seed| {
                replay_find_rider_fixture(c, seed);
                let key = c.g[A0][0];
                let mut r = Rng(0x2708_f000 ^ seed as u64);
                let this = DATA + 0x3_0000;
                let data = DATA + 0x3_9000;
                let busy = seed / 2 % 5;
                for (i, off) in OC331_REPLAY_WIPEOUT_BUSY.into_iter().enumerate() {
                    let set = busy as usize == i + 1;
                    c.bus
                        .write32(this + off, if set { 1 + r.next_u32() % 4 } else { 0 });
                }
                for k in 0..5u32 {
                    c.bus.write32(this + 0x494 + 0xb4 * k + 0xb0, 0);
                }
                c.bus.write32(this + 0x3d4, (seed % 3 != 0) as u32);
                let last = r.next_u32() % 5000;
                c.bus.write32(this + 0x47c, last);
                let now = last.wrapping_add(if seed % 4 < 2 { 500 } else { 1500 });
                c.bus.write32(data + 0xa4, now);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = key;
                c.g[A2][0] = sx(data);
                c.pc = 0x0027_08f0;
            },
        );
    }

    // -- Replay_BeginHighlight ----------------------------------------------

    pub(crate) fn fake_highlight_slot_begin(c: &mut Cpu) {
        if !active(c) {
            return chain(c, 7, fake_highlight_slot_begin);
        }
        fake_ret(c, 10);
    }

    #[test]
    fn replay_begin_highlight_matches() {
        install(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026E6E0),
            fake_highlight_slot_begin,
            7,
        );
        check(
            ee::s07::f_00270b88__generated,
            replay_begin_highlight,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_0b88, 2);
                let cache = DATA + 0x1000;
                let slot = (seed % 4) as u64;
                c.g[A0][0] = cache as u64;
                c.g[A1][0] = slot;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                c.bus
                    .write32(cache, if seed % 3 == 0 { r.next_u32() } else { 0 });
                let slot_addr = cache + (slot * REPLAY_SLOT_STRIDE) as u32;
                c.bus
                    .write32(slot_addr + 0x544, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(cache + REPLAY_RIDER as u32, DATA + 0x2000);
                c.bus.write32(CTRL, MAGIC);
                c.pc = 0x0027_0b88;
            },
        );
    }

    /// A ramp track with up to three running ramps (some ended by `t`) and
    /// up to four keys (some due), with the real key activation.
    #[test]
    fn in_seq_ramp_track_update_matches() {
        check(
            ee::s07::f_00273698__generated,
            in_seq_ramp_track_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_3698, 0);
                c.pc = 0x0027_3698;
                let this = DATA + 0x100;
                let header = DATA + 0x200;
                let keys = DATA + 0x400;
                let pool = DATA + 0x1000;
                let flags = DATA + 0x80;
                c.bus.write32(flags, r.next_u32() & 0xfc);
                let t = (r.next_u32() % 20) as f32;
                c.f[12] = t.to_bits();
                c.f[13] = r.f32(2.0);
                // Running ramps.
                let n = r.next_u32() % 4;
                let mut head = 0;
                for k in (0..n).rev() {
                    let e = DATA + 0x1800 + 0x40 * k;
                    put_words(
                        c,
                        e,
                        &[
                            0,
                            0,
                            r.next_u32() % 30,
                            0,
                            r.f32(10.0),
                            r.f32(3.0),
                            0,
                            0,
                            head,
                        ],
                    );
                    head = e;
                }
                c.bus.write32(this + RAMP_TRACK_ACTIVE, head);
                c.bus.write32(this + RAMP_TRACK_COUNT, n);
                // Keys (size 0x18 or 0x20), sorted by time.
                let nk = r.next_u32() % 5;
                c.bus.write16(header, nk.max(1) as u16);
                let mut at = keys;
                let mut time = 0;
                for k in 0..nk {
                    let size = if r.next_u32() % 2 == 0 { 0x18 } else { 0x20 };
                    time += r.next_u32() % 8;
                    c.bus.write16(at, size as u16);
                    c.bus.write16(at + 2, k as u16);
                    c.bus.write16(at + 4, time as u16);
                    c.bus.write16(at + 6, r.next_u32() as u16);
                    put_words(
                        c,
                        at + 8,
                        &[
                            r.next_u32() % 40,
                            r.next_u32() % 100,
                            r.f32(3.0),
                            r.next_u32(),
                        ],
                    );
                    at += size;
                }
                c.bus.write32(this + 0x10, header);
                c.bus
                    .write32(this + RAMP_TRACK_KEY, if nk == 0 { 0 } else { keys });
                c.bus.write32(this + 0x18, 0);
                c.bus.write32(this + 0x1c, pool);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(flags);
            },
        );
    }

    pub(crate) use crate::game::input::inputparse::tests::o406_w;

    pub(crate) use crate::game::input::inputparse::tests::o406_base;
    pub(crate) use crate::game::input::inputparse::tests::o406_results;

    pub(crate) use crate::game::ai::rider::tests::w32_2;
    pub(crate) use crate::game::world::worldview::tests::oc410_machine;
    pub(crate) use crate::game::world::worldview::tests::oc410_out;
    pub(crate) use crate::game::world::worldview::tests::oc410_result;

    pub(crate) use crate::game::seg::seg_1fbe38::tests::oc417_machine;

    // ------------------------------------------------ InSeq channel updates

    /// A channel at `DATA`: header (+0x1c: byte +2 track count, byte +3
    /// type), the track player array (+0x24), a communicator (+0x2c) whose
    /// vtable entries (+0x1c, +0x24) are fakes. Each track player's vtable
    /// (+0x1c tick, +0x34 key distance) points at `FAKE`/`FAKE + 0x30`.
    pub(crate) fn o406_channel(c: &mut Cpu, r: &mut Rng, tracks: u32) {
        let hdr = DATA + 0x100;
        let array = DATA + 0x200;
        let vtbl = DATA + 0x500;
        let comm_vt = DATA + 0x600;
        o406_w(c, DATA + 0x1c, hdr);
        c.bus.write8(hdr + 2, tracks as u8);
        c.bus.write8(hdr + 3, (r.next_u32() % 4) as u8);
        o406_w(c, DATA + 0x24, array);
        for i in 0..tracks {
            let player = DATA + 0x300 + 0x40 * i;
            o406_w(c, array + 4 * i, player);
            o406_w(c, player + 4, vtbl);
        }
        c.bus.write16(vtbl + 0x18, 0x10);
        o406_w(c, vtbl + 0x1c, fakes::FAKE);
        c.bus.write16(vtbl + 0x30, 0x20);
        o406_w(c, vtbl + 0x34, fakes::FAKE + 0x30);
        let comm = if r.next_u32() % 4 == 0 {
            0
        } else {
            DATA + 0x580
        };
        o406_w(c, DATA + 0x2c, comm);
        o406_w(c, DATA + 0x580, comm_vt);
        c.bus.write16(comm_vt + 0x18, 0x8);
        o406_w(c, comm_vt + 0x1c, fakes::FAKE + 0x10);
        c.bus.write16(comm_vt + 0x20, 0x8);
        o406_w(c, comm_vt + 0x24, fakes::FAKE + 0x20);
    }

    /// Tick results the track players give: progress or none, and flag words.
    pub(crate) fn o406_tick_results(c: &mut Cpu, r: &mut Rng) {
        let mut values = vec![];
        for _ in 0..10 {
            values.push([0u64, 1, 1, 5][(r.next_u32() % 4) as usize]);
            values.push([0u64, 0, 1, 0x10][(r.next_u32() % 4) as usize]);
        }
        o406_results(c, &values);
    }

    /// Run every track's tick (vtable call), OR the flags and notify the
    /// communicator (0 to 3 tracks, with and without a communicator).
    #[test]
    fn in_seq_channel_update_tracks_matches() {
        fakes::covered(
            ee::s07::f_002743e8__generated,
            in_seq_channel_update_tracks,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x2743e8, 6, 0x0027_43e8);
                let tracks = r.next_u32() % 4;
                o406_channel(c, &mut r, tracks);
                o406_tick_results(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.f32(100.0);
                c.f[13] = r.f32(2.0);
            },
        );
    }

    /// Advance a channel: stopped (`+0x28 == 0`), without a step, and with
    /// one: the key distances (fake `FAKE + 0x30`) clamp the time, the real
    /// `UpdateTracks` runs until the time is used or reports no progress.
    #[test]
    fn in_seq_channel_advance_matches() {
        fakes::covered(
            ee::s07::f_00274100__generated,
            in_seq_channel_advance,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x274100, 12, 0x0027_4100);
                let tracks = r.next_u32() % 3;
                o406_channel(c, &mut r, tracks);
                o406_tick_results(c, &mut r);
                // Key distances after the tick answers: small counts.
                let mut values = vec![];
                for i in 0..24 {
                    values.push(if i % 3 == 0 {
                        [0u64, 1, 2, 3, 6][(r.next_u32() % 5) as usize]
                    } else if i % 3 == 1 {
                        r.next_u32() as u64 % 2
                    } else {
                        r.next_u32() as u64 % 0x20
                    });
                }
                o406_results(c, &values);
                o406_w(c, DATA + 0x28, if seed % 7 == 0 { 0 } else { 1 });
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 5 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
                c.f[12] = r.f32(10.0);
                c.f[13] = r.f32(5.0);
                c.f[14] = r.f32(5.0);
            },
        );
    }

    /// Event track: the keys (halfword time at +2) at `DATA + 0x300`.
    pub(crate) fn o406_event_track(c: &mut Cpu, r: &mut Rng) {
        let n = 1 + r.next_u32() % 4;
        let mut t = r.next_u32() % 3;
        let mut next = vec![];
        for i in 0..n {
            let key = DATA + 0x300 + 0x10 * i;
            c.bus.write16(key + 2, t as u16);
            t += r.next_u32() % 3;
            next.push((key + 0x10) as u64);
        }
        // The chain ends (the key after the last is null).
        let last = next.len() - 1;
        next[last] = 0;
        o406_results(c, &next);
        o406_w(c, DATA + 0x14, DATA + 0x300);
        if r.next_u32() % 6 == 0 {
            o406_w(c, DATA + 0x14, 0);
        }
        c.f[12] = ((r.next_u32() % 12) as f32).to_bits();
        let t = c.f[12];
        o406_w(
            c,
            DATA + 0x24,
            if r.next_u32() % 3 == 0 { t } else { r.f32(5.0) },
        );
    }

    /// Fire the keys the time has reached (the fire/advance callees are
    /// fakes; the next-key fake feeds the chain).
    #[test]
    fn in_seq_event_track_update_matches() {
        fakes::covered(
            ee::s07::f_00273330__generated,
            in_seq_event_track_update,
            200,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x273330, 6, 0x0027_3330);
                o406_event_track(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x80) as u64;
                o406_w(c, DATA + 0x80, r.next_u32() & !0x10);
            },
        );
    }

    // ------------------------------------------------------ InSeq_InstanceTick

    /// An instance at `DATA`: state at +0x20, delay +0x2c, channel table
    /// header +0x10 (byte +1 = channel count) with channels (0x3c each) at
    /// +0x18, flag +0x1c, times +0x24/+0x28, +0x40, listener from the fake.
    #[test]
    fn in_seq_instance_tick_matches() {
        fakes::covered(
            ee::s07::f_00274de0__generated,
            in_seq_instance_tick,
            300,
            &|c, seed| {
                let mut r = o406_base(c, seed, 0x274de0, 8, 0x0027_4de0);
                let listener = DATA + 0x700;
                let l_vt = DATA + 0x780;
                o406_w(c, listener, l_vt);
                for off in [8u32, 0x10, 0x18, 0x28] {
                    c.bus.write16(l_vt + off, 0x4);
                    o406_w(c, l_vt + off + 4, fakes::FAKE + 0x10 + off);
                }
                // Results: listener, length, then channel flush answers.
                let mut values = vec![];
                values.push(if r.next_u32() % 4 == 0 {
                    0
                } else {
                    listener as u64
                });
                values.push([4u64, 4, 8, 12][(r.next_u32() % 4) as usize]);
                for _ in 0..6 {
                    values.push(r.next_u32() as u64 % 2);
                }
                o406_results(c, &values);
                let states = [0u32, 1, 2, 3, 4, 2, 3, 1];
                o406_w(c, DATA + 0x20, states[(seed % 8) as usize]);
                o406_w(c, DATA + 0x2c, if seed % 9 == 0 { 3 } else { 0 });
                let hdr = DATA + 0x100;
                o406_w(c, DATA + 0x10, hdr);
                c.bus.write8(hdr + 1, (seed % 3 == 0) as u8);
                o406_w(c, DATA + 0x18, DATA + 0x200);
                o406_w(c, DATA + 0x1c, (seed % 3 == 0) as u32);
                o406_w(c, DATA + 0x24, r.f32(3.0));
                o406_w(c, DATA + 0x28, r.f32(12.0) & 0x7fff_ffff);
                o406_w(c, DATA + 0x40, (seed % 5 == 0) as u32);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// `InSeq_SpanTrackUpdate`: expired events are unlinked, due key records
    /// activated and advanced.
    #[test]
    fn in_seq_span_track_update_matches() {
        const THIS: u32 = DATA + 0x1000;
        const FLAGS: u32 = DATA + 0x500;
        const EVENTS: u32 = DATA + 0x2000;
        const REC0: u32 = DATA + 0x3000;
        const REC1: u32 = DATA + 0x3100;
        check(
            ee::s07::f_00272f28__generated,
            in_seq_span_track_update,
            300,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0027_2f28, 4);
                c.pc = 0x0027_2f28;
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = FLAGS as u64;
                c.f[12] = ((r.next_u32() % 100) as f32 + 0.5).to_bits();
                w32_2(c, FLAGS, r.next_u32());
                let n = seed % 4;
                w32_2(c, THIS + 0x24, if n == 0 { 0 } else { EVENTS });
                w32_2(c, THIS + 0x28, r.next_u32() % 8);
                for i in 0..n {
                    let ev = EVENTS + 0x40 * i;
                    w32_2(c, ev + 0xc, r.next_u32() % 100);
                    w32_2(c, ev + 0x18, if i + 1 < n { ev + 0x40 } else { 0 });
                }
                w32_2(c, THIS + 0x14, if seed % 3 == 0 { 0 } else { REC0 });
                c.bus.write16(REC0 + 4, (r.next_u32() % 100) as u16);
                c.bus.write16(REC1 + 4, 65000);
                // The key reader moves on to a future record, or to none.
                w32_2(c, oc410_out(20), if seed % 2 == 0 { 0 } else { REC1 });
            },
        );
    }

    /// `InSeq_InstanceAdvance`: jump decision, channel loop (real
    /// `InSeq_ChannelAdvance` on empty channels) and the listener call.
    #[test]
    fn in_seq_instance_advance_matches() {
        const THIS: u32 = DATA + 0x1000;
        const HDR: u32 = DATA + 0x1100;
        const TRACKS: u32 = DATA + 0x2000;
        const EMPTY_HDR: u32 = DATA + 0x5000;
        const LISTENER: u32 = DATA + 0x6000;
        check(
            ee::s07::f_002751a0__generated,
            in_seq_instance_advance,
            400,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x0027_51a0, 2);
                c.pc = 0x0027_51a0;
                c.g[A0][0] = THIS as u64;
                let frac = |r: &mut Rng| if r.next_u32() & 1 == 0 { 0.0 } else { 0.5 };
                let time = (r.next_u32() % 50) as f32 + frac(&mut r);
                let prev = (r.next_u32() % 50) as f32 + frac(&mut r);
                c.f[12] = time.to_bits();
                c.f[13] = prev.to_bits();
                c.f[14] = [0.5f32, 1.0, 2.0][(seed % 3) as usize].to_bits();
                w32_2(c, THIS + 0x10, HDR);
                let channels = (seed / 3) % 4;
                c.bus.write8(HDR + 1, channels as u8);
                w32_2(c, THIS + 0x18, TRACKS);
                for i in 0..channels {
                    let ch = TRACKS + 0x3c * i;
                    w32_2(c, ch + 0x28, if r.next_u32() & 1 == 0 { 0 } else { 1 });
                    w32_2(c, ch + 0x1c, EMPTY_HDR);
                    w32_2(c, ch + 0x2c, 0);
                }
                let listener = if seed % 5 == 0 { 0 } else { LISTENER };
                w32_2(c, oc410_result(24), listener);
                let vt = fakes::vtable(c, &mut r, &[0x10]);
                w32_2(c, LISTENER, vt);
            },
        );
    }

    /// `InSeq_HeapAlloc`: free lists of 0..3 blocks of assorted sizes against
    /// requests that fit exactly, split a block, or fit nowhere.
    #[test]
    fn in_seq_heap_alloc_matches() {
        check(
            ee::s07::f_00272308__generated,
            in_seq_heap_alloc,
            120,
            &|c, seed| {
                let mut r = Rng(0x2723_0800 ^ seed as u64);
                scramble(c, &mut r);
                let heap = DATA;
                let blocks = [DATA + 0x100, DATA + 0x200, DATA + 0x300];
                let n = (seed % 4) as usize;
                c.bus.write32(heap + 8, if n == 0 { 0 } else { blocks[0] });
                for i in 0..n {
                    let size = [8u32, 16, 40, 100, 200, 4][r.next_u32() as usize % 6];
                    c.bus.write32(blocks[i], size);
                    c.bus
                        .write32(blocks[i] + 4, if i + 1 < n { blocks[i + 1] } else { 0 });
                }
                let want = [1u32, 4, 8, 13, 24, 40, 90, 150][r.next_u32() as usize % 8];
                c.g[A0][0] = sx(heap);
                c.g[A1][0] = if seed % 5 == 0 {
                    want as u64 | ((r.next_u32() as u64) << 32)
                } else {
                    want as u64
                };
            },
        );
    }

    // ----------------------------------------------------------------------
    // InSeq_ChannelStop

    /// A channel with up to three players of every kind (0 delete, 1-3
    /// release, others nothing; the kind virtual is `0x27c958`, which reads
    /// `obj+0x20`, so the fixture chooses the kind), with or without the
    /// entity comm object, playing or idle, 0-3 players.
    #[test]
    fn in_seq_channel_stop_matches() {
        const CHAN: u32 = DATA + 0x1000;
        const COMM: u32 = DATA + 0x1200;
        const HEADER: u32 = DATA + 0x1400;
        const PLAYERS: u32 = DATA + 0x1600;
        const OWNER: u32 = DATA + 0x1800;
        const HEAP: u32 = DATA + 0x1900;
        const COMM_VT: u32 = DATA + 0x3200;
        const PLAYER_VT: u32 = DATA + 0x3000;
        check(
            ee::s07::f_00273dc0__generated,
            in_seq_channel_stop,
            300,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0006, 8);
                c.pc = 0x0027_3dc0;
                c.bus
                    .write32(CHAN + 0x28, if seed % 7 == 6 { 0 } else { 1 });
                c.bus
                    .write32(CHAN + 0x2c, if seed % 3 == 0 { 0 } else { COMM });
                c.bus.write32(COMM, COMM_VT);
                c.bus.write16(COMM_VT + 0x10, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(COMM_VT + 0x14, fakes::FAKE + 0x40);
                c.bus.write32(CHAN + 0x1c, HEADER);
                c.bus.write8(HEADER + 2, (seed / 3 % 4) as u8);
                c.bus.write8(HEADER + 3, r.next_u32() as u8);
                c.bus.write32(CHAN + 0x24, PLAYERS);
                c.bus.write32(CHAN + 8, OWNER);
                c.bus.write32(OWNER + 8, HEAP);
                c.bus.write32(HEAP + 8, r.next_u32());
                c.bus.write16(PLAYER_VT + 0x10, 0);
                c.bus.write32(PLAYER_VT + 0x14, fakes::FAKE);
                c.bus.write16(PLAYER_VT + 0x38, 0);
                c.bus.write32(PLAYER_VT + 0x3c, 0x0027_c958);
                c.bus.write16(PLAYER_VT + 0x50, 0);
                c.bus.write32(PLAYER_VT + 0x54, fakes::FAKE + 0x10);
                for i in 0..3u32 {
                    let obj = DATA + 0x2000 + 0x100 * i;
                    c.bus.write32(PLAYERS + 4 * i, obj);
                    c.bus.write32(obj + 4, PLAYER_VT);
                    c.bus
                        .write32(obj + 0x20, (seed / 12 + i * 2 + r.next_u32() % 3) % 6);
                }
                c.g[A0][0] = CHAN as u64;
            },
        );
    }

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake;

    use std::sync::atomic::AtomicUsize;

    pub(crate) use crate::game::ai::ai::tests::OC420_CTRL;
    pub(crate) use crate::game::ai::ai::tests::OC420_MAGIC;
    pub(crate) use crate::game::ai::ai::tests::OC420_RET;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w_5 as w_2;

    pub(crate) use crate::game::main::gamemode::tests::oc427_vtable;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_queue;

    /// A logging fake while the control word is set (its result is the word
    /// the fixture stored for it, sign-extended), otherwise whatever was
    /// installed before (chained through `prev`).
    pub(crate) fn oc420_fake_gated(
        c: &mut Cpu,
        addr: u32,
        ret_idx: u32,
        prev: &AtomicUsize,
        fallback: fn(&mut Cpu),
    ) {
        if c.bus.read32(OC420_CTRL) == OC420_MAGIC {
            let ra = c.g[RA][0] as u32;
            fake(c, addr, ra);
            if c.pc == ra {
                c.g[V0][0] = sx(c.bus.read32(OC420_RET + 4 * ret_idx));
            }
            return;
        }
        crate::difftest::fakes::fall_back(c, prev, fallback)
    }

    /// Stamps out a gated fake for the override slot of `$addr` and the
    /// function `$install` that hooks it (once per process).
    macro_rules! oc420_fake {
        ($fake:ident, $prev:ident, $install:ident, $addr:expr, $ret:expr, $shard:ident, $slot:ident, $gen:ident) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                oc420_fake_gated(c, $addr, $ret, &$prev, ee::$shard::$gen);
            }
            pub(crate) fn $install() {
                crate::difftest::fakes::hook(
                    std::ptr::addr_of_mut!(ee::$shard::$slot),
                    $fake,
                    &$prev,
                );
            }
        };
    }

    oc420_fake!(
        oc420_fake_section,
        OC420_PREV_SECTION,
        oc420_hook_section,
        0x0022_e0e0,
        0,
        s05,
        OVERRIDE_0022E0E0,
        f_0022e0e0__generated
    );

    oc420_fake!(
        oc420_fake_get_be,
        OC420_PREV_GET_BE,
        oc420_hook_get_be,
        0x0014_dc80,
        1,
        s01,
        OVERRIDE_0014DC80,
        f_0014dc80__generated
    );

    oc420_fake!(
        oc420_fake_get_iface,
        OC420_PREV_GET_IFACE,
        oc420_hook_get_iface,
        0x0014_dd58,
        2,
        s01,
        OVERRIDE_0014DD58,
        f_0014dd58__generated
    );

    oc420_fake!(
        oc420_fake_path_free,
        OC420_PREV_PATH_FREE,
        oc420_hook_path_free,
        0x0026_af00,
        3,
        s07,
        OVERRIDE_0026AF00,
        f_0026af00__generated
    );

    oc420_fake!(
        oc420_fake_path_set,
        OC420_PREV_PATH_SET,
        oc420_hook_path_set,
        0x0026_ad70,
        4,
        s07,
        OVERRIDE_0026AD70,
        f_0026ad70__generated
    );

    oc420_fake!(
        oc420_fake_init_paths,
        OC420_PREV_INIT_PATHS,
        oc420_hook_init_paths,
        0x0011_2180,
        5,
        s00,
        OVERRIDE_00112180,
        f_00112180__generated
    );

    oc420_fake!(
        oc420_fake_rand_range,
        OC420_PREV_RAND_RANGE,
        oc420_hook_rand_range,
        0x0031_7830,
        6,
        s10,
        OVERRIDE_00317830,
        f_00317830__generated
    );

    oc420_fake!(
        oc420_fake_luno_as_float,
        OC420_PREV_LUNO_AS_FLOAT,
        oc420_hook_luno_as_float,
        0x0022_6618,
        7,
        s05,
        OVERRIDE_00226618,
        f_00226618__generated
    );

    oc420_fake!(
        oc420_fake_audio_instance,
        OC420_PREV_AUDIO_INSTANCE,
        oc420_hook_audio_instance,
        0x0028_b180,
        8,
        s07,
        OVERRIDE_0028B180,
        f_0028b180__generated
    );

    oc420_fake!(
        oc420_fake_audio_stop_loop,
        OC420_PREV_AUDIO_STOP_LOOP,
        oc420_hook_audio_stop_loop,
        0x0029_7eb8,
        9,
        s08,
        OVERRIDE_00297EB8,
        f_00297eb8__generated
    );

    oc420_fake!(
        oc420_fake_add_parent,
        OC420_PREV_ADD_PARENT,
        oc420_hook_add_parent,
        0x0035_5978,
        10,
        s11,
        OVERRIDE_00355978,
        f_00355978__generated
    );

    oc420_fake!(
        oc420_fake_rider_script_off,
        OC420_PREV_SCRIPT_OFF,
        oc420_hook_script_off,
        0x0028_03f0,
        11,
        s07,
        OVERRIDE_002803F0,
        f_002803f0__generated
    );

    oc420_fake!(
        oc420_fake_rider_reset,
        OC420_PREV_RIDER_RESET,
        oc420_hook_rider_reset,
        0x0011_d390,
        12,
        s00,
        OVERRIDE_0011D390,
        f_0011d390__generated
    );

    oc420_fake!(
        oc420_fake_hide_part,
        OC420_PREV_HIDE_PART,
        oc420_hook_hide_part,
        0x0030_ea80,
        13,
        s10,
        OVERRIDE_0030EA80,
        f_0030ea80__generated
    );

    oc420_fake!(
        oc420_fake_show_part,
        OC420_PREV_SHOW_PART,
        oc420_hook_show_part,
        0x0030_e9e0,
        14,
        s10,
        OVERRIDE_0030E9E0,
        f_0030e9e0__generated
    );

    oc420_fake!(
        oc420_fake_bolts_db,
        OC420_PREV_BOLTS_DB,
        oc420_hook_bolts_db,
        0x0014_bdb8,
        15,
        s01,
        OVERRIDE_0014BDB8,
        f_0014bdb8__generated
    );

    oc420_fake!(
        oc420_fake_profile,
        OC420_PREV_PROFILE,
        oc420_hook_profile,
        0x0014_ad50,
        16,
        s01,
        OVERRIDE_0014AD50,
        f_0014ad50__generated
    );

    oc420_fake!(
        oc420_fake_get_character,
        OC420_PREV_GET_CHARACTER,
        oc420_hook_get_character,
        0x0012_3128,
        17,
        s00,
        OVERRIDE_00123128,
        f_00123128__generated
    );

    oc420_fake!(
        oc420_fake_bolt_count,
        OC420_PREV_BOLT_COUNT,
        oc420_hook_bolt_count,
        0x0014_d988,
        18,
        s01,
        OVERRIDE_0014D988,
        f_0014d988__generated
    );

    oc420_fake!(
        oc420_fake_bolt_list,
        OC420_PREV_BOLT_LIST,
        oc420_hook_bolt_list,
        0x0014_d998,
        19,
        s01,
        OVERRIDE_0014D998,
        f_0014d998__generated
    );

    oc420_fake!(
        oc420_fake_stop_handle,
        OC420_PREV_STOP_HANDLE,
        oc420_hook_stop_handle,
        0x002a_d5f0,
        20,
        s08,
        OVERRIDE_002AD5F0,
        f_002ad5f0__generated
    );

    oc420_fake!(
        oc420_fake_rider_char,
        OC420_PREV_RIDER_CHAR,
        oc420_hook_rider_char,
        0x0014_ac30,
        21,
        s01,
        OVERRIDE_0014AC30,
        f_0014ac30__generated
    );

    oc420_fake!(
        oc420_fake_alt_char,
        OC420_PREV_ALT_CHAR,
        oc420_hook_alt_char,
        0x0014_a0b0,
        22,
        s01,
        OVERRIDE_0014A0B0,
        f_0014a0b0__generated
    );

    oc420_fake!(
        oc420_fake_rider_profile,
        OC420_PREV_RIDER_PROFILE,
        oc420_hook_rider_profile,
        0x0014_ac50,
        23,
        s01,
        OVERRIDE_0014AC50,
        f_0014ac50__generated
    );

    oc420_fake!(
        oc420_fake_bolt_texture,
        OC420_PREV_BOLT_TEXTURE,
        oc420_hook_bolt_texture,
        0x0014_b988,
        24,
        s01,
        OVERRIDE_0014B988,
        f_0014b988__generated
    );

    oc420_fake!(
        oc420_fake_rider_init,
        OC420_PREV_RIDER_INIT,
        oc420_hook_rider_init,
        0x0014_0738,
        25,
        s01,
        OVERRIDE_00140738,
        f_00140738__generated
    );

    oc420_fake!(
        oc420_fake_script_obj_rider,
        OC420_PREV_SCRIPT_OBJ_RIDER,
        oc420_hook_script_obj_rider,
        0x0027_f968,
        26,
        s07,
        OVERRIDE_0027F968,
        f_0027f968__generated
    );

    oc420_fake!(
        oc420_fake_mem_alloc,
        OC420_PREV_MEM_ALLOC,
        oc420_hook_mem_alloc,
        0x0031_7d70,
        27,
        s10,
        OVERRIDE_00317D70,
        f_00317d70__generated
    );

    oc420_fake!(
        oc420_fake_emitter_fx,
        OC420_PREV_EMITTER_FX,
        oc420_hook_emitter_fx,
        0x002f_1850,
        28,
        s09,
        OVERRIDE_002F1850,
        f_002f1850__generated
    );

    oc420_fake!(
        oc420_fake_bolt_on_fx,
        OC420_PREV_BOLT_ON_FX,
        oc420_hook_bolt_on_fx,
        0x002e_c9e0,
        29,
        s09,
        OVERRIDE_002EC9E0,
        f_002ec9e0__generated
    );

    oc420_fake!(
        oc420_fake_shadow_ctor,
        OC420_PREV_SHADOW_CTOR,
        oc420_hook_shadow_ctor,
        0x0037_58f8,
        30,
        s12,
        OVERRIDE_003758F8,
        f_003758f8__generated
    );

    oc420_fake!(
        oc420_fake_script_register,
        OC420_PREV_SCRIPT_REGISTER,
        oc420_hook_script_register,
        0x0028_3298,
        31,
        s07,
        OVERRIDE_00283298,
        f_00283298__generated
    );

    oc420_fake!(
        oc420_fake_mem_free,
        OC420_PREV_MEM_FREE,
        oc420_hook_mem_free,
        0x0031_7e98,
        32,
        s10,
        OVERRIDE_00317E98,
        f_00317e98__generated
    );

    /// Heap allocator wrapper against free lists of 0..3 blocks (fits,
    /// splits, none), with a random `gp[0x398]` bias.
    #[test]
    fn in_seq_heap_new_matches() {
        check_lohi(
            ee::s07::f_00272488__generated,
            in_seq_heap_new,
            160,
            &|c, seed| {
                let mut r = Rng(0x2724_8800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0027_2488;
                c.g[GP][0] = GAME_GP as u64;
                c.bus
                    .write32(GAME_GP + 0x398, [0u32, 4, 8, 12][(seed / 4 % 4) as usize]);
                let heap = DATA;
                let blocks = [DATA + 0x100, DATA + 0x200, DATA + 0x300];
                let n = (seed % 4) as usize;
                c.bus.write32(heap + 8, if n == 0 { 0 } else { blocks[0] });
                for i in 0..n {
                    let size = [8u32, 16, 40, 100, 200, 4][r.next_u32() as usize % 6];
                    c.bus.write32(blocks[i], size);
                    c.bus
                        .write32(blocks[i] + 4, if i + 1 < n { blocks[i + 1] } else { 0 });
                }
                let want = [1u32, 4, 8, 13, 24, 40, 90, 150][r.next_u32() as usize % 8];
                c.g[A0][0] = sx(want);
                c.g[A1][0] = sx(heap);
            },
        );
    }

    // ------------------------------------------------------- InSeq_ChannelStart

    pub(crate) const OC420_THIS: u32 = DATA + 0x1000;

    pub(crate) const OC420_OWNER: u32 = DATA + 0x1100;

    pub(crate) const OC420_HEAP: u32 = DATA + 0x1200;

    pub(crate) const OC420_HDR: u32 = DATA + 0x1400;

    pub(crate) const OC420_LISTENER: u32 = DATA + 0x1800;

    pub(crate) const OC420_LISTENER_VT: u32 = DATA + 0x1900;

    pub(crate) const OC420_PLAYER: u32 = DATA + 0x1a00;

    pub(crate) const OC420_PLAYER_VT: u32 = DATA + 0x1b00;

    pub(crate) const OC420_DUMMY: u32 = DATA + 0x1c00;

    pub(crate) const OC420_DUMMY_VT: u32 = DATA + 0x1d00;

    pub(crate) const OC420_BLOCK: u32 = DATA + 0x4000;

    /// An instance with 0..4 channel records (types 0-3 build a track
    /// object with the real constructor; type 4 builds nothing), channel
    /// virtuals that are fakes (non-zero) or the zero leaf, an optional
    /// listener that hands out a player, and an optional stale player.
    pub(crate) fn oc420_channel_start_setup(c: &mut Cpu, seed: u32, min_channels: u32) -> Rng {
        let mut r = machine(c, seed, 0x27_3aa8, 4);
        c.pc = 0x0027_3aa8;
        c.g[A0][0] = OC420_THIS as u64;
        c.bus
            .write32(GAME_GP + 0x398, [0u32, 4][(seed % 2) as usize]);
        c.bus.write32(OC420_THIS + 8, OC420_OWNER);
        c.bus.write32(OC420_OWNER + 8, OC420_HEAP);
        c.bus.write32(OC420_HEAP + 0x10, OC420_BLOCK);
        c.bus.write32(OC420_BLOCK, 0x800);
        c.bus.write32(OC420_BLOCK + 4, 0);
        for k in 0..4 {
            c.bus.write32(OC420_BLOCK + 8 + 4 * k, OC420_DUMMY);
        }
        c.bus.write32(OC420_DUMMY + 4, OC420_DUMMY_VT);
        c.bus.write16(OC420_DUMMY_VT + 8, 0);
        c.bus.write32(OC420_DUMMY_VT + 0xc, FAKE + 0xf0);
        // The four track vtables the constructors point objects at.
        let mut fakes_used = 0;
        let mut vt_fn = [0u32; 4];
        for (i, vt) in [0x0048_1a10u32, 0x0048_18f0, 0x0048_19b0, 0x0048_1950]
            .into_iter()
            .enumerate()
        {
            let f = if r.next_u32() % 4 == 0 {
                0x0014_06c0
            } else {
                FAKE + 0x10 * (i as u32 + 1)
            };
            vt_fn[i] = f;
            c.bus
                .write16(vt + 8, [0u16, 0x10][(r.next_u32() % 2) as usize]);
            c.bus.write32(vt + 0xc, f);
        }
        let n = min_channels + (seed / 2) % (5 - min_channels);
        c.bus.write32(OC420_THIS + 0x1c, OC420_HDR);
        c.bus.write16(OC420_HDR, 0x10);
        c.bus.write8(OC420_HDR + 2, n as u8);
        c.bus.write8(OC420_HDR + 3, r.next_u32() as u8);
        let mut rec = OC420_HDR + 0x10;
        for _ in 0..n {
            let kind = r.next_u32() % 5;
            let size = 4 * (r.next_u32() % 3);
            c.bus.write16(rec, kind as u16);
            c.bus.write16(rec + 2, 4 + size as u16);
            // Fake channels: the track kinds 0 (curve), 1 (ramp), 2 (span), 3 (event).
            let used = match kind {
                0 => vt_fn[0],
                1 => vt_fn[1],
                2 => vt_fn[2],
                3 => vt_fn[3],
                _ => FAKE + 0xf0,
            };
            if used >= FAKE && used < FAKE + 0x100 {
                fakes_used += 1;
            }
            rec += 4 + 4 + size;
        }
        // Listener (cached at +0x44 or through the owner) and player.
        let listener = [0u32, OC420_LISTENER][(r.next_u32() % 4 != 0) as usize];
        if r.next_u32() % 2 == 0 {
            c.bus.write32(OC420_THIS + 0x44, listener);
        } else {
            c.bus.write32(OC420_THIS + 0x44, 0);
            c.bus.write32(OC420_OWNER, listener);
        }
        c.bus.write32(OC420_LISTENER, OC420_LISTENER_VT);
        c.bus.write16(
            OC420_LISTENER_VT + 0x30,
            [0u16, 8][(r.next_u32() % 2) as usize],
        );
        c.bus.write32(OC420_LISTENER_VT + 0x34, FAKE + 0xe0);
        c.bus.write32(OC420_PLAYER, OC420_PLAYER_VT);
        c.bus
            .write16(OC420_PLAYER_VT + 8, [0u16, 4][(r.next_u32() % 2) as usize]);
        c.bus.write32(OC420_PLAYER_VT + 0xc, FAKE + 0xd0);
        c.bus.write32(
            OC420_THIS + 0x2c,
            [0u32, OC420_PLAYER][(r.next_u32() % 2) as usize],
        );
        // The listener's call is the next fake after the channel starts
        // (only the fake-backed ones count); it hands out the player.
        c.bus.write32(
            crate::difftest::fakes::LOG_RESULTS + 4 * fakes_used,
            OC420_PLAYER,
        );
        c.bus.write32(OC420_THIS + 0x24, 0);
        r
    }

    #[test]
    fn in_seq_channel_start_matches() {
        use crate::difftest::events::{check_events, schedule};
        covered(
            ee::s07::f_00273aa8__generated,
            in_seq_channel_start,
            300,
            &|c, seed| {
                oc420_channel_start_setup(c, seed, 0);
            },
        );
        check_events(
            ee::s07::f_00273aa8__generated,
            in_seq_channel_start,
            150,
            &|c, seed| {
                let mut r = oc420_channel_start_setup(c, seed, 2);
                schedule(c, &mut r, seed, 0x0027_3aa8)
            },
        );
    }

    #[test]
    fn in_seq_ramp_track_start_matches() {
        check(
            ee::s07::f_00273530__generated,
            in_seq_ramp_track_start,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_3530, 0);
                let track = DATA;
                let keys = DATA + 0x1000;
                let seq = DATA + 0x200;
                let flags = DATA + 0x500;
                w_2(c, flags, r.next_u32());
                // seq -> +8 -> heap object -> +8 -> heap head; free block at +0x418.
                w_2(c, seq + 8, DATA + 0x300);
                w_2(c, DATA + 0x308, DATA + 0x400);
                let block = DATA + 0x2000;
                w_2(c, DATA + 0x418, block);
                w_2(c, block, 0x400);
                w_2(c, block + 4, 0);
                let count = if seed % 5 == 0 { 0 } else { 1 + seed % 3 };
                c.bus.write16(keys, count as u16);
                c.bus.write16(keys + 2, count.max(1) as u16);
                for i in 0..4u32 {
                    let key = keys + 4 + 0x18 * i;
                    c.bus.write16(key, 0x18);
                    c.bus.write16(key + 2, (i % count.max(1)) as u16);
                    let stop = match (seed / 5) % 4 {
                        0 => i == 0 && seed % 7 == 0,
                        1 => i == 1,
                        _ => false,
                    };
                    c.bus.write16(key + 4, stop as u16);
                    w_2(c, key + 8, r.next_u32());
                    w_2(c, key + 0xc, r.next_u32());
                    w_2(c, key + 0x10, r.next_u32());
                    w_2(c, key + 0x14, r.next_u32());
                }
                c.g[A0][0] = track as u64;
                c.g[A1][0] = keys as u64;
                c.g[A2][0] = seq as u64;
                c.g[A3][0] = flags as u64;
            },
        );
    }

    #[test]
    fn in_seq_heap_delete_matches() {
        check(
            ee::s07::f_002724e8__generated,
            in_seq_heap_delete,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_24e8, 0);
                let header_size = 0x10u32;
                w_2(c, GAME_GP + 0x398, header_size);
                let heap = DATA;
                // Empty free list; the block's size header at header - 8.
                w_2(c, heap + 8, 0);
                w_2(c, heap, r.next_u32());
                let header = DATA + 0x1010;
                w_2(c, header, heap);
                w_2(c, header - 8, 0x30);
                w_2(c, header - 4, 0);
                // Every third call frees a null header.
                c.g[A0][0] = if seed % 3 == 0 {
                    header_size as u64
                } else {
                    (header + header_size) as u64
                };
            },
        );
    }

    /// `InSeq_InstanceStop`: stopped, update-in-progress and running
    /// instances with 0..3 entity players; the listener is absent or has a
    /// vtable entry (a fake method) at +0x20.
    #[test]
    fn in_seq_instance_stop_matches() {
        covered(
            ee::s07::f_00274918__generated,
            in_seq_instance_stop,
            160,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x274918, 6, 0x0027_4918);
                let this = DATA + 0x1000;
                let header = DATA + 0x2000;
                let listener = DATA + 0x3000;
                let vt = DATA + 0x3100;
                c.bus.write32(this + 0x10, header);
                c.bus.write32(this + 0x18, DATA + 0x2400);
                c.bus
                    .write32(this + 0x20, [0, 1, 2][r.next_u32() as usize % 3]);
                c.bus.write32(this + 0x3c, (r.next_u32() % 3 == 0) as u32);
                c.bus.write8(header + 1, (r.next_u32() % 4) as u8);
                c.bus.write32(listener, vt);
                oc427_vtable(c, vt, 0x20, (r.next_u32() as u16) & !3);
                oc427_queue(
                    c,
                    0x0027_4d70,
                    &[if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        listener as u64
                    }],
                );
                c.g[A0][0] = sx(this);
            },
        );
    }

    pub(crate) use crate::game::intersect::riderspheretree::tests::OC430_CTRL;
    pub(crate) use crate::game::intersect::riderspheretree::tests::OC430_MAGIC;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_force;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_machine;

    pub(crate) use crate::game::main::ssxapp::tests::oc429_w;

    pub(crate) use crate::game::main::ssxapp::tests::OC429_FX;

    /// `InSeq_InstanceSetup`: header sizes below / above 13 (extension
    /// block), 0..3 channels (array created and each channel set up), unwinds
    /// in any call.
    #[test]
    fn in_seq_instance_setup_matches() {
        covered(
            ee::s07::f_002745c0__generated,
            in_seq_instance_setup,
            160,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x2745c0, 8, 0x0027_45c0);
                let this = OC429_FX + 0x400;
                let data = OC429_FX + 0x600;
                let owner = OC429_FX + 0x800;
                c.bus.write8(data + 1, (seed % 4) as u8);
                c.bus
                    .write16(data + 2, if seed % 3 == 0 { 0x10 } else { 0xc });
                oc429_w(c, data + 8, r.next_u32());
                oc429_force(c, 0x0027_2488, &[(OC429_FX + 0xa00) as u64]);
                let sizes: Vec<u64> = (0..3).map(|_| 4 + (r.next_u32() % 8) as u64).collect();
                oc429_force(c, 0x0027_39b8, &sizes);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = data as u64;
                c.g[A2][0] = owner as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// Fixture for `InSeq_InstanceRelease`: the instance at `DATA + 0x1000`
    /// (live or idle, 0..3 channels, an array with a count word before it or
    /// none).
    pub(crate) fn oc430_inseq_instance(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let this = DATA + 0x1000;
        for k in 0..0x40 {
            c.bus.write32(this + 4 * k, r.next_u32());
        }
        c.bus
            .write32(this + 4, if seed % 5 == 0 { 0 } else { r.next_u32() | 1 });
        let desc = DATA + 0x2000;
        c.bus.write32(this + 0x10, desc);
        // the channel count is the byte at `desc + 1`
        c.bus
            .write32(desc, (r.next_u32() % 4) << 8 | (r.next_u32() & 0xff));
        let array = DATA + 0x4010;
        c.bus
            .write32(this + 0x18, if seed % 7 == 3 { 0 } else { array });
        c.bus.write32(array - 0x10, r.next_u32() % 4);
        c.g[A0][0] = sx(this);
    }

    /// `InSeq_InstanceRelease`: idle and live instances, no / some channels,
    /// no / empty / non-empty channel array (LO/HI of the count multiply too).
    #[test]
    fn in_seq_instance_release_matches() {
        oc430_install_fakes();
        check_lohi(
            ee::s07::f_002746f8__generated,
            in_seq_instance_release,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_46f8, 8);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                c.pc = 0x0027_46f8;
                oc430_inseq_instance(c, &mut r, seed);
            },
        );
    }

    /// The same with event delivery at the two loop back edges.
    #[test]
    fn in_seq_instance_release_events_match() {
        oc430_install_fakes();
        check_events(
            ee::s07::f_002746f8__generated,
            in_seq_instance_release,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_46f9, 0);
                c.bus.write32(OC430_CTRL, OC430_MAGIC);
                oc430_inseq_instance(c, &mut r, seed | 1);
                let mode = schedule(c, &mut r, seed, 0x0027_46f8);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[RA][0] = SENTINEL as u64;
                mode
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::oc432_fake;
    pub(crate) use crate::game::ai::ai::tests::OC432_CTRL;
    pub(crate) use crate::game::ai::ai::tests::OC432_MAGIC;
    use std::ptr::addr_of_mut;

    oc432_fake!(
        fake_in_seq_heap_init,
        PREV_IN_SEQ_HEAP_INIT,
        ee::s07::f_002722e0__generated,
        0x0027_22e0
    );

    oc432_fake!(
        fake_in_seq_heap_new,
        PREV_IN_SEQ_HEAP_NEW,
        ee::s07::f_00272488__generated,
        0x0027_2488,
        |c: &mut Cpu| {
            c.g[V0][0] = (DATA + 0x5_8000) as u64;
        }
    );

    oc432_fake!(
        fake_in_seq_instance_ctor,
        PREV_IN_SEQ_INSTANCE_CTOR,
        ee::s07::f_00274518__generated,
        0x0027_4518
    );

    /// `InSeq_ctor`: 0..4 items (including the empty pool); the heap init,
    /// the array allocation and the item constructor are fakes.
    #[test]
    fn in_seq_ctor_matches() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002722E0),
            fake_in_seq_heap_init,
            &PREV_IN_SEQ_HEAP_INIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00272488),
            fake_in_seq_heap_new,
            &PREV_IN_SEQ_HEAP_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00274518),
            fake_in_seq_instance_ctor,
            &PREV_IN_SEQ_INSTANCE_CTOR,
        );
        fakes::covered(
            ee::s07::f_00272788__generated,
            in_seq_ctor,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x272788, 6);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.g[A0][0] = (DATA + 0x5_0000) as u64;
                c.g[A3][0] = (r.next_u32() % 5) as u64;
                let _ = seed;
            },
        );
    }

    pub(crate) use crate::game::camera::camera::tests::SEEDS_5 as SEEDS_9;
    pub(crate) use crate::game::camera::cameracontroller::tests::oc437_checked;

    /// A random word with the edge values zero and negative mixed in.
    pub(crate) fn oc436_word(r: &mut Rng) -> u32 {
        match r.next_u32() % 4 {
            0 => 0,
            1 => 0xffff_fff0,
            _ => r.next_u32(),
        }
    }

    /// `InSeq_ChannelGetHeaderField4`: a channel whose header (+0x10) points
    /// at a record with a random word at +4.
    #[test]
    fn oc436_in_seq_channel_get_header_field4_matches() {
        check(
            ee::s07::f_00274388__generated,
            oc436_in_seq_channel_get_header_field4,
            SEEDS_9,
            &|c, seed| {
                let mut r = Rng(0x2743_88 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let ch = DATA + 0x100;
                let header = DATA + 0x200;
                c.g[A0][0] = sx(ch);
                c.bus.write32(ch + OC436_CHANNEL_HEADER, header);
                let w = oc436_word(&mut r);
                c.bus.write32(header + 4, w);
            },
        );
    }

    #[test]
    fn inseq_heap_delete_array_matches() {
        oc437_checked(
            ee::s07::f_00272510__generated,
            inseq_heap_delete_array,
            0x0027_2510,
            &|_, _, _| {},
        );
    }

    #[test]
    fn inseq_instance_set_user_data_matches() {
        check(
            ee::s07::f_00274c10__generated,
            inseq_instance_set_user_data,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274c10, 0);
                c.pc = 0x0027_4c10;
                let obj = ptr(&mut r);
                c.g[A0][0] = obj as u64;
                c.bus.write32(obj + 0xc, r.next_u32());
            },
        );
    }

    pub(crate) use crate::game::scripter::datamanager::tests::gated;
    pub(crate) use crate::game::scripter::datamanager::tests::install_fakes;

    // The save-buffer free and the replay reset are gated log-only fakes
    // (the gate word in RAM), as in `Replay_BeginPlayback`.
    pub(crate) const LOAD_GATE: u32 = DATA + 0x20_de00;

    pub(crate) const LOAD_MAGIC: u32 = 0x4c4f_4144;

    pub(crate) static OC532_LOAD_PREV_FREESAVE: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) static OC532_LOAD_PREV_RESET: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn load_fake_freesave(c: &mut Cpu) {
        if c.bus.read32(LOAD_GATE) != LOAD_MAGIC {
            return fakes::fall_back(c, &OC532_LOAD_PREV_FREESAVE, ee::s07::f_00270f78__generated);
        }
        fakes::fake_call(c, 0x0027_0f78);
    }

    pub(crate) fn load_fake_reset(c: &mut Cpu) {
        if c.bus.read32(LOAD_GATE) != LOAD_MAGIC {
            return fakes::fall_back(c, &OC532_LOAD_PREV_RESET, ee::s07::f_0026f228__generated);
        }
        fakes::fake_call(c, 0x0026_f228);
    }

    pub(crate) fn load_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00270F78),
            load_fake_freesave,
            &OC532_LOAD_PREV_FREESAVE,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_0026F228),
            load_fake_reset,
            &OC532_LOAD_PREV_RESET,
        );
    }

    pub(crate) fn load_setup(c: &mut Cpu, seed: u32) {
        fakes::machine(c, seed, 0x27_0ed8, 0);
        load_install();
        c.bus.write32(LOAD_GATE, LOAD_MAGIC);
    }

    #[test]
    fn replay_end_save_matches() {
        check(
            ee::s07::f_00270ed8__generated,
            replay_end_save,
            20,
            &load_setup,
        );
    }

    #[test]
    fn replay_reset_on_load_matches() {
        check(
            ee::s07::f_00270ef8__generated,
            replay_reset_on_load,
            20,
            &load_setup,
        );
    }

    #[test]
    fn replay_empty_load_hook_matches() {
        check(
            ee::s07::f_00270f18__generated,
            replay_empty_load_hook,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x27_0f18, 0);
            },
        );
    }

    /// Gate word for this module's callee fakes (test machines only). The
    /// fakes run their gate-off body unless the machine holds this magic.
    pub(crate) const OC534_GATE: u32 = DATA + 0x200000;

    pub(crate) const OC534_MAGIC: u32 = 0x0534_0001;

    /// A gated fake for an installed callee: logs the call (`fakes::fake`)
    /// and returns to `ra`, or runs the body it replaced when the gate is off.
    macro_rules! gated_fake {
        ($fake:ident, $prev:ident, $gen:path, $target:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC534_GATE) != OC534_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    gated_fake!(
        oc534_fake_save_emit,
        OC534_PREV_SAVE_EMIT,
        ee::s07::f_00271058__generated,
        0x0027_1058
    );

    gated_fake!(
        oc534_fake_huff,
        OC534_PREV_HUFF,
        ee::s09::f_002ca100__generated,
        0x002c_a100
    );

    gated_fake!(
        oc534_fake_heap_delete,
        OC534_PREV_HEAP_DELETE,
        ee::s07::f_002724e8__generated,
        0x0027_24e8
    );

    gated_fake!(
        oc534_fake_free_sequence,
        OC534_PREV_FREE_SEQUENCE,
        ee::s07::f_002729a8__generated,
        0x0027_29a8
    );

    gated_fake!(
        oc534_fake_get_instance,
        OC534_PREV_GET_INSTANCE,
        ee::s07::f_00272cc0__generated,
        0x0027_2cc0
    );

    gated_fake!(
        oc534_fake_play,
        OC534_PREV_PLAY,
        ee::s07::f_00272ad0__generated,
        0x0027_2ad0
    );

    gated_fake!(
        oc534_fake_stop,
        OC534_PREV_STOP,
        ee::s07::f_00272b58__generated,
        0x0027_2b58
    );

    gated_fake!(
        oc534_fake_instance_pause,
        OC534_PREV_INSTANCE_PAUSE,
        ee::s07::f_002749e8__generated,
        0x0027_49e8
    );

    gated_fake!(
        oc534_fake_instance_resume,
        OC534_PREV_INSTANCE_RESUME,
        ee::s07::f_00274a08__generated,
        0x0027_4a08
    );

    /// Hooks every fake of this module (each once per process).
    pub(crate) fn hook_fakes(which: &[&str]) {
        for w in which {
            match *w {
                "save_emit" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_00271058),
                    oc534_fake_save_emit,
                    &OC534_PREV_SAVE_EMIT,
                ),
                "huff" => fakes::hook(
                    addr_of_mut!(ee::s09::OVERRIDE_002CA100),
                    oc534_fake_huff,
                    &OC534_PREV_HUFF,
                ),
                "heap_delete" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_002724E8),
                    oc534_fake_heap_delete,
                    &OC534_PREV_HEAP_DELETE,
                ),
                "free_sequence" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_002729A8),
                    oc534_fake_free_sequence,
                    &OC534_PREV_FREE_SEQUENCE,
                ),
                "get_instance" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_00272CC0),
                    oc534_fake_get_instance,
                    &OC534_PREV_GET_INSTANCE,
                ),
                "play" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_00272AD0),
                    oc534_fake_play,
                    &OC534_PREV_PLAY,
                ),
                "stop" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_00272B58),
                    oc534_fake_stop,
                    &OC534_PREV_STOP,
                ),
                "instance_pause" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_002749E8),
                    oc534_fake_instance_pause,
                    &OC534_PREV_INSTANCE_PAUSE,
                ),
                "instance_resume" => fakes::hook(
                    addr_of_mut!(ee::s07::OVERRIDE_00274A08),
                    oc534_fake_instance_resume,
                    &OC534_PREV_INSTANCE_RESUME,
                ),
                other => panic!("unknown fake {other}"),
            }
        }
    }

    /// Machine state with the fakes gated on, registers scrambled.
    pub(crate) fn gate_on(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine(c, seed, salt, 0);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(OC534_GATE, OC534_MAGIC);
        r
    }

    /// A channel list of `seed % 4` nodes behind the self object at DATA+0x1000
    /// (head at +0x1c, next at +0x50, data word at +4).
    pub(crate) fn chain_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let mut r = gate_on(c, seed, salt);
        let me = DATA + 0x1000;
        let n = seed % 4;
        let node = |i: u32| DATA + 0x2000 + 0x100 * i;
        for i in 0..n {
            let next = if i + 1 < n { node(i + 1) } else { 0 };
            c.bus.write32(node(i) + 0x50, next);
            c.bus.write32(node(i) + 0x4, r.next_u32());
        }
        c.bus.write32(me + 0x1c, if n > 0 { node(0) } else { 0 });
        c.g[A0][0] = sx(me);
        c.g[A1][0] = r.next_u32() as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.pc = entry;
        r
    }

    #[test]
    fn replay_next_save_block_matches() {
        check(
            ee::s07::f_002716f0__generated,
            replay_next_save_block,
            200,
            &|c, seed| {
                let mut r = Rng(0x0027_16f0 ^ seed as u64);
                scramble(c, &mut r);
                let base = DATA + 0x1000 * (seed % 8);
                c.g[A0][0] = sx(base);
                c.bus.write32(base + 0x638, r.next_u32());
                c.bus.write32(base + 0x644, r.next_u32());
                c.bus.write32(base + 0x648, r.next_u32());
                c.pc = 0x0027_16f0;
            },
        );
    }

    #[test]
    fn replay_debug_printf_stub_matches() {
        check(
            ee::s07::f_002718a8__generated,
            replay_debug_printf_stub,
            200,
            &|c, seed| {
                let mut r = Rng(0x0027_18a8 ^ seed as u64);
                scramble(c, &mut r);
                for f in 12..=18 {
                    c.f[f] = r.next_u32();
                }
                c.pc = 0x0027_18a8;
            },
        );
    }

    #[test]
    fn replay_global_ctors_matches() {
        check(
            ee::s07::f_002722c0__generated,
            replay_global_ctors,
            60,
            &|c, seed| {
                let mut r = Rng(0x0027_22c0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0027_22c0;
            },
        );
    }

    #[test]
    fn replay_save_write_cache_matches() {
        hook_fakes(&["save_emit", "huff"]);
        check(
            ee::s07::f_00271510__generated,
            replay_save_write_cache,
            200,
            &|c, seed| {
                let mut r = gate_on(c, seed, 0x0027_1510);
                let me = DATA + 0x1000;
                c.g[A0][0] = sx(me);
                c.bus.write32(me + 0x640, DATA + 0xb000);
                c.bus.write32(me + 0x670, 0x100);
                // The five branches, by seed.
                let (frame, emit, idx, max) = match seed % 5 {
                    0 => (0, DATA + 0xd000, 0, 4),
                    1 => (DATA + 0xc000, 0, 0, 4),
                    2 | 3 => (0, 0, 0, 4),
                    _ => (0, 0, 5, 2),
                };
                c.bus.write32(me + 0x66c, frame);
                c.bus.write32(me + 0x674, emit);
                c.bus.write32(me + 0x654, idx);
                c.bus.write32(me + 0x658, max);
                c.bus.write32(me + 0x638, r.next_u32() & 0xff);
                if seed % 5 == 2 {
                    // A cache in the first slot: its command frame and record cursor.
                    let cache = DATA + 0xe000;
                    c.bus.write32(me + 0x48c, cache);
                    c.bus.write32(cache, r.next_u32());
                    c.bus.write32(cache + 0xc, r.next_u32());
                }
                c.pc = 0x0027_1510;
            },
        );
    }

    #[test]
    fn insseq_heap_delete_thunk_matches() {
        hook_fakes(&["heap_delete"]);
        check(
            ee::s07::f_00272530__generated,
            insseq_heap_delete_thunk,
            200,
            &|c, seed| {
                gate_on(c, seed, 0x0027_2530);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.pc = 0x0027_2530;
            },
        );
    }

    #[test]
    fn insseq_heap_delete_thunk2_matches() {
        hook_fakes(&["heap_delete"]);
        check(
            ee::s07::f_00272550__generated,
            insseq_heap_delete_thunk2,
            200,
            &|c, seed| {
                gate_on(c, seed, 0x0027_2550);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.pc = 0x0027_2550;
            },
        );
    }

    #[test]
    fn insseq_free_all_matches() {
        hook_fakes(&["free_sequence"]);
        check(
            ee::s07::f_00272a50__generated,
            insseq_free_all,
            200,
            &|c, seed| {
                chain_setup(c, seed, 0x0027_2a50, 0x0027_2a50);
            },
        );
    }

    #[test]
    fn insseq_play_all_matches() {
        hook_fakes(&["play"]);
        check(
            ee::s07::f_00272b00__generated,
            insseq_play_all,
            200,
            &|c, seed| {
                chain_setup(c, seed, 0x0027_2b00, 0x0027_2b00);
            },
        );
    }

    #[test]
    fn insseq_stop_all_matches() {
        hook_fakes(&["stop"]);
        check(
            ee::s07::f_00272b80__generated,
            insseq_stop_all,
            200,
            &|c, seed| {
                chain_setup(c, seed, 0x0027_2b80, 0x0027_2b80);
            },
        );
    }

    #[test]
    fn insseq_pause_matches() {
        hook_fakes(&["get_instance", "instance_pause"]);
        check(
            ee::s07::f_00272bd0__generated,
            insseq_pause,
            200,
            &|c, seed| {
                gate_on(c, seed, 0x0027_2bd0);
                // The instance `InSeq_GetInstance` returns (fake call 0).
                c.bus.write32(fakes::LOG_RESULTS, DATA + 0x3000);
                c.pc = 0x0027_2bd0;
            },
        );
    }

    #[test]
    fn insseq_pause_all_matches() {
        // The generated `InSeq_Pause` runs for real (unported): both sides
        // call it, so the fakes below serve its two callees.
        hook_fakes(&["get_instance", "instance_pause"]);
        check(
            ee::s07::f_00272bf8__generated,
            insseq_pause_all,
            200,
            &|c, seed| {
                chain_setup(c, seed, 0x0027_2bf8, 0x0027_2bf8);
                c.bus.write32(fakes::LOG_RESULTS, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn insseq_resume_matches() {
        hook_fakes(&["get_instance", "instance_resume"]);
        check(
            ee::s07::f_00272c48__generated,
            insseq_resume,
            200,
            &|c, seed| {
                gate_on(c, seed, 0x0027_2c48);
                c.bus.write32(fakes::LOG_RESULTS, DATA + 0x3000);
                c.pc = 0x0027_2c48;
            },
        );
    }

    #[test]
    fn insseq_resume_all_matches() {
        hook_fakes(&["get_instance", "instance_resume"]);
        check(
            ee::s07::f_00272c70__generated,
            insseq_resume_all,
            200,
            &|c, seed| {
                chain_setup(c, seed, 0x0027_2c70, 0x0027_2c70);
                c.bus.write32(fakes::LOG_RESULTS, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn insseq_get_instance_data_matches() {
        check(
            ee::s07::f_00272ce0__generated,
            insseq_get_instance_data,
            200,
            &|c, seed| {
                let mut r = gate_on(c, seed, 0x0027_2ce0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1004, r.next_u32());
                c.pc = 0x0027_2ce0;
            },
        );
    }

    #[test]
    fn insseq_get_instance_owner_matches() {
        check(
            ee::s07::f_00272d00__generated,
            insseq_get_instance_owner,
            200,
            &|c, seed| {
                let mut r = gate_on(c, seed, 0x0027_2d00);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1008, r.next_u32());
                c.pc = 0x0027_2d00;
            },
        );
    }

    #[test]
    fn insseq_get_channel_count_matches() {
        check(
            ee::s07::f_00272d20__generated,
            insseq_get_channel_count,
            200,
            &|c, seed| {
                let mut r = gate_on(c, seed, 0x0027_2d20);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, r.next_u32());
                c.pc = 0x0027_2d20;
            },
        );
    }

    #[test]
    fn insseq_get_channel_block_field4_matches() {
        check(
            ee::s07::f_00272d40__generated,
            insseq_get_channel_block_field4,
            200,
            &|c, seed| {
                let mut r = gate_on(c, seed, 0x0027_2d40);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = (seed % 3) as u64;
                c.bus.write32(DATA + 0x1000, r.next_u32() & 0xff);
                c.pc = 0x0027_2d40;
            },
        );
    }

    #[test]
    fn in_seq_get_block_payload_matches() {
        check(
            ee::s07::f_00274318__generated,
            in_seq_get_block_payload,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274318, 0);
                let seq = DATA + 0x400;
                let index = seed % 16;
                c.g[A0][0] = sx(seq);
                c.g[A1][0] = index as u64;
                let header_off = 0x200 + (seed % 8) * 0x10;
                c.bus.write32(seq + index * 4 + 8, header_off);
                // Kinds below 9 take the no-payload path, others return header + 8.
                let kind = if seed % 2 == 0 {
                    r.next_u32() % 9
                } else {
                    9 + r.next_u32() % 200
                };
                c.bus.write16(seq + header_off, kind as u16);
            },
        );
    }

    #[test]
    fn in_seq_block_get_field4_matches() {
        check(
            ee::s07::f_00274398__generated,
            in_seq_block_get_field4,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274398, 0);
                let block = DATA + 0x500;
                c.g[A0][0] = sx(block);
                c.bus.write32(block + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_block_get_size_matches() {
        check(
            ee::s07::f_002743c0__generated,
            in_seq_block_get_size,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2743c0, 0);
                let block = DATA + 0x500;
                c.g[A0][0] = sx(block);
                c.bus.write32(block, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_instance_set_speed_matches() {
        check(
            ee::s07::f_00274808__generated,
            in_seq_instance_set_speed,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274808, 0);
                let inst = DATA + 0x600;
                c.g[A0][0] = sx(inst);
                c.f[12] = r.f32(400.0);
            },
        );
    }

    #[test]
    fn in_seq_instance_restart_matches() {
        check(
            ee::s07::f_00274810__generated,
            in_seq_instance_restart,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274810, 0);
                let inst = DATA + 0x600;
                c.g[A0][0] = sx(inst);
                // Path: inactive (start now) or active; a pending start request or none.
                c.bus.write32(inst + 0x1c, r.next_u32());
                c.bus.write32(inst + 0x20, seed % 2);
                c.bus
                    .write32(inst + 0x3c, if seed % 3 == 0 { 1 } else { 0 });
            },
        );
    }

    #[test]
    fn in_seq_instance_get_data_matches() {
        check(
            ee::s07::f_00274c58__generated,
            in_seq_instance_get_data,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274c58, 0);
                let inst = DATA + 0x600;
                c.g[A0][0] = sx(inst);
                c.bus.write32(inst + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_instance_get_owner_matches() {
        check(
            ee::s07::f_00274c60__generated,
            in_seq_instance_get_owner,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274c60, 0);
                let inst = DATA + 0x600;
                c.g[A0][0] = sx(inst);
                c.bus.write32(inst + 8, r.next_u32());
            },
        );
    }

    #[test]
    fn in_seq_get_channel_count_matches() {
        check(
            ee::s07::f_00274c68__generated,
            in_seq_get_channel_count,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274c68, 0);
                let header = DATA + 0x600;
                c.g[A0][0] = sx(header);
                c.bus.write8(header + 1, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn in_seq_find_channel_data_matches() {
        check(
            ee::s07::f_00274d90__generated,
            in_seq_find_channel_data,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x274d90, 0);
                let seq = DATA + 0x100;
                c.g[A0][0] = sx(seq);
                // Zero to three blocks; the first block is at seq + 0x40.
                c.g[A1][0] = (seed % 4) as u64;
                c.bus.write16(seq + 2, 0x40);
                for i in 0..0x100 {
                    c.bus
                        .write32(seq + 0x40 + i * 4, (r.next_u32() % 0x20) * 4 + 4);
                }
            },
        );
    }

    #[test]
    fn in_seq_instance_dtor_matches() {
        install_fakes();
        check(
            ee::s07::f_00274578__generated,
            in_seq_instance_dtor,
            200,
            &|c, seed| {
                gated(c, seed, 0x274578);
                let inst = DATA + 0x600;
                c.g[A0][0] = sx(inst);
                c.g[A1][0] = (seed % 4) as u64;
                c.pc = 0x0027_4578;
            },
        );
    }

    #[test]
    fn in_seq_instance_run_to_end_matches() {
        install_fakes();
        check(
            ee::s07::f_00274868__generated,
            in_seq_instance_run_to_end,
            200,
            &|c, seed| {
                let mut r = gated(c, seed, 0x274868);
                let inst = DATA + 0x600;
                c.g[A0][0] = sx(inst);
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(inst + 0x1c, r.next_u32());
                c.bus.write32(inst + 0x24, r.next_u32());
                c.f[20] = r.next_u32();
                c.f[21] = r.next_u32();
                c.pc = 0x0027_4868;
            },
        );
    }

    /// `Replay_RecycleOldestSnapshot` (existing hand-written body in
    /// `replaycache.rs`): a short free list with used and locked nodes; the
    /// generated dispatcher is entered at its head.

    /// `Replay_StepFrame` (existing hand-written body in `replaycache.rs`)
    /// with `reset == 0` and the tick before the end of the record: the frame
    /// cursor (skipping the save frame) and the tick advance.
    #[test]
    fn oc541_step_frame_matches() {
        check(
            ee::s07::f_002703f0,
            crate::game::replay::replaycache::replay_step_frame,
            200,
            &|c, seed| {
                let mut r = Rng(0x2703_f000 ^ seed as u64);
                scramble(c, &mut r);
                let save = DATA + 0x2000;
                let frame = match seed % 3 {
                    0 => 0,
                    1 => save,
                    _ => DATA + 0x3000,
                };
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                let tick = r.next_u32() % 1000;
                c.bus.write32(DATA + 0x3d0, save);
                c.bus.write32(DATA + 0x488, frame);
                c.bus.write32(save + 0x18, DATA + 0x4000);
                c.bus.write32(DATA + 0x484, tick);
                c.bus.write32(DATA + 0x3cc, tick + 1 + seed % 2);
            },
        );
    }

    /// `Replay_OnRiderEvent` (existing hand-written body in `replaycache.rs`):
    /// each busy flag leaves at once; otherwise `Replay_FindRiderIndex` runs
    /// (the rider fixture, with the world at gp-0x848) and the event's mode
    /// decides the callee. The self object lives away from the world.

    /// `Replay_FindWeakestHighlight` (existing hand-written body in
    /// `replaycache.rs`): a leaf scan over 0-4 nodes, bit `i` in 0..3, the
    /// positive and negative branches, busy and clear nodes, a best of 0 or a
    /// node.

    /// `Replay_SaveWriteSnapshot` with no pending block and an empty write
    /// queue: the write state advances and the source is cleared (the queue
    /// and block paths need the encoder and the emit, covered by their own tests).
    #[test]
    fn oc541_save_write_snapshot_matches() {
        check(
            ee::s07::f_00271380__generated,
            replay_save_write_snapshot,
            200,
            &|c, seed| {
                let mut r = Rng(0x2713_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x660, 0);
                c.bus.write32(DATA + 0x65c, 0);
                c.bus.write32(DATA + 0x638, r.next_u32());
                c.bus.write32(DATA + 0x644, r.next_u32());
                c.bus.write32(DATA + 0x648, r.next_u32());
            },
        );
    }

    #[test]
    fn replay_static_init_destroy_matches() {
        check(
            ee::s07::f_002718e8__generated,
            replay_static_init_destroy,
            200,
            &|c, seed| {
                machine(c, seed, 0x18e8, 0);
                // init (a0) non-zero / zero; priority (a1) 0xffff runs the body.
                c.g[A0][0] = (seed % 2) as u64;
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { 0xffff };
                c.pc = 0x002718e8;
            },
        );
    }

    #[test]
    fn replay_write_save_block_matches() {
        check(
            ee::s07::f_002710a8__generated,
            replay_write_save_block,
            200,
            &|c, seed| {
                machine(c, seed, 0x10a8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x002710a8;
            },
        );
    }

    use crate::difftest::fakes::fake_call;

    pub(crate) use crate::game::replay::frameptr::tests::RET;

    pub(crate) use crate::game::wscript::wscriptman::tests::OC655_MAN;

    pub(crate) use crate::game::wscript::wscriptman::tests::oc655_gates;
    pub(crate) use crate::game::wscript::wscriptman::tests::oc655_hook_all;
    pub(crate) use crate::game::wscript::wscriptman::tests::oc655_slot;

    #[test]
    fn replay_find_weakest_highlight_matches() {
        const SELF: u32 = DATA + 0x30_2000;
        const LIST: u32 = DATA + 0x30_2100;
        const BEST: u32 = DATA + 0x30_2800;
        check(
            ee::s07::f_00270c58__generated,
            replay_find_weakest_highlight_2,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x27_0c58, 0);
                c.pc = 0x0027_0c58;
                c.g[RA][0] = RET;
                let n = seed % 5;
                for k in 0..n {
                    let e = LIST + 0x40 * k;
                    let next = if k + 1 < n { e + 0x40 } else { 0 };
                    c.bus.write32(e + 0x14, next);
                    c.bus.write8(e + 0x1e, r.next_u32() as u8);
                    c.bus.write8(e + 0x1f, (r.next_u32() % 3) as u8 & 1);
                    c.bus.write32(e + 0x20, r.next_u32());
                }
                c.bus.write32(SELF + 0x3b0, if n == 0 { 0 } else { LIST });
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = (seed % 32) as u64;
                c.g[A2][0] = BEST as u64;
                c.g[A3][0] = (seed % 2) as u64;
            },
        );
    }

    /// The channel mask the channel player's `Play` writes to its events word.
    pub(crate) const OC655_CHANNEL_MASK: u32 = DATA + 0x4_f018;

    #[test]
    fn in_seq_run_to_end_matches() {
        oc655_hook_all();
        covered(
            ee::s07::f_00272aa0__generated,
            in_seq_run_to_end,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_2aa0, 2);
                oc655_gates(c, &mut r);
            },
        );
    }

    #[test]
    fn in_seq_channel_restart_matches() {
        oc655_hook_all();
        covered(
            ee::s07::f_00273f68__generated,
            in_seq_channel_restart,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_3f68, 6);
                oc655_gates(c, &mut r);
                c.hooks.miss = oc655_channel_miss;
                let me = OC655_MAN;
                let hdr = me + 0x100;
                let arr = me + 0x200;
                let comm = me + 0x300;
                let player_vt = me + 0x400;
                let comm_vt = me + 0x500;
                let players = me + 0x600;
                c.bus
                    .write32(me + ICR_PLAYING, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(me + ICR_HEADER, hdr);
                c.bus.write32(me + ICR_PLAYERS, arr);
                c.bus
                    .write32(me + ICR_COMM, if seed % 4 == 0 { 0 } else { comm });
                // System header: headerSize 0x10, numChannels 0..3, the entity type.
                let nch = seed % 4;
                c.bus.write16(hdr, 0x10);
                c.bus.write8(hdr + 2, nch as u8);
                c.bus.write8(hdr + 3, r.next_u32() as u8);
                let mut rec = hdr + 0x10;
                for _ in 0..nch {
                    let size = (r.next_u32() % 6) * 2;
                    c.bus.write16(rec + 2, size as u16);
                    rec += 4 + size;
                }
                for i in 0..4 {
                    let p = players + 0x40 * i;
                    c.bus.write32(arr + 4 * i, p);
                    c.bus.write32(p + 4, player_vt);
                }
                // Player vtable: Play at slot 0 (+8), Stop at slot 1 (+0x10).
                oc655_slot(c, &mut r, player_vt, 0x8, FAKE);
                oc655_slot(c, &mut r, player_vt, 0x10, FAKE + 0x10);
                // Entity comm vtable: key event at slot 2 (+0x18), update at slot 3 (+0x20).
                c.bus.write32(comm, comm_vt);
                oc655_slot(c, &mut r, comm_vt, 0x18, FAKE + 0x20);
                oc655_slot(c, &mut r, comm_vt, 0x20, FAKE + 0x30);
                c.bus.write32(OC655_CHANNEL_MASK, r.next_u32() & 7);
                c.g[A0][0] = me as u64;
            },
        );
    }

    /// Channel players' `Play` (slot 0, `FAKE`) writes the test's event mask
    /// through its `events` argument (a3) before the usual fake return.
    pub(crate) fn oc655_channel_miss(c: &mut Cpu, target: u32) {
        if target == FAKE {
            let events = c.g[A3][0] as u32;
            let mask = c.bus.read32(OC655_CHANNEL_MASK);
            c.bus.write32(events, mask);
        }
        fake_call(c, target);
    }

    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;

    use crate::game::scripter::ssxscriptengine::tests::oc423_fake;
    use crate::game::scripter::ssxscriptengine::tests::oc423_fake_call;
    use crate::game::scripter::ssxscriptengine::tests::OC423_CTRL;
    use crate::game::scripter::ssxscriptengine::tests::OC423_MAGIC;
    use crate::game::scripter::ssxscriptengine::tests::OC423_RET;

    pub(crate) use crate::game::animation::animsequencer::tests::oc684_gate;
    pub(crate) use crate::game::animation::animsequencer::tests::oc684_w;

    oc423_fake!(
        oc684_fake_find_snapshot,
        OC684_PREV_FIND_SNAPSHOT,
        0x0026_e448,
        ee::s07::f_0026e448__generated
    );

    oc423_fake!(
        oc684_fake_run_list_seek,
        OC684_PREV_RUN_LIST_SEEK,
        0x0026_d4d8,
        ee::s07::f_0026d4d8__generated
    );

    oc423_fake!(
        oc684_fake_seek_to_snapshot,
        OC684_PREV_SEEK_TO_SNAPSHOT,
        0x0027_02f8,
        ee::s07::f_002702f8__generated
    );

    oc423_fake!(
        oc684_fake_set_state,
        OC684_PREV_SET_STATE,
        0x0026_f4a0,
        ee::s07::f_0026f4a0__generated
    );

    oc423_fake!(
        oc684_fake_find_rider_index,
        OC684_PREV_FIND_RIDER_INDEX,
        0x0027_0730,
        ee::s07::f_00270730__generated
    );

    oc423_fake!(
        oc684_fake_empty_rider_hook,
        OC684_PREV_EMPTY_RIDER_HOOK,
        0x0027_05a8,
        ee::s07::f_002705a8__generated
    );

    oc423_fake!(
        oc684_fake_begin_highlight,
        OC684_PREV_BEGIN_HIGHLIGHT,
        0x0027_0b88,
        ee::s07::f_00270b88__generated
    );

    oc423_fake!(
        oc684_fake_snapshot_clear,
        OC684_PREV_SNAPSHOT_CLEAR,
        0x0026_e5c0,
        ee::s07::f_0026e5c0__generated
    );

    oc423_fake!(
        oc684_fake_cache_deserialize,
        OC684_PREV_CACHE_DESERIALIZE,
        0x0026_d558,
        ee::s07::f_0026d558__generated
    );

    pub(crate) fn oc684_hook_replay_fakes() {
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026E448),
            oc684_fake_find_snapshot,
            &OC684_PREV_FIND_SNAPSHOT,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026D4D8),
            oc684_fake_run_list_seek,
            &OC684_PREV_RUN_LIST_SEEK,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_002702F8),
            oc684_fake_seek_to_snapshot,
            &OC684_PREV_SEEK_TO_SNAPSHOT,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026F4A0),
            oc684_fake_set_state,
            &OC684_PREV_SET_STATE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00270730),
            oc684_fake_find_rider_index,
            &OC684_PREV_FIND_RIDER_INDEX,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_002705A8),
            oc684_fake_empty_rider_hook,
            &OC684_PREV_EMPTY_RIDER_HOOK,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00270B88),
            oc684_fake_begin_highlight,
            &OC684_PREV_BEGIN_HIGHLIGHT,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026E5C0),
            oc684_fake_snapshot_clear,
            &OC684_PREV_SNAPSHOT_CLEAR,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026D558),
            oc684_fake_cache_deserialize,
            &OC684_PREV_CACHE_DESERIALIZE,
        );
    }

    /// `Replay_StepFrame`: reset = 0 with the tick before the last tick (tick
    /// advances; current snapshot unchanged, equal to the saved one, or
    /// another), at the last tick with `+0x61c` clear (state 3 via the fake)
    /// or set (the first fake, SeekToSnapshot, unwinds; the recursive step
    /// and the redraw behind it need the real snapshot restore, which the
    /// fixture cannot bound), and reset = 1 (FindSnapshot unwinds at once:
    /// the run-list loop and `Replay_RestoreSnapshot`, the real
    /// `WorldState_Read`, are not reached).
    #[test]
    fn replay_step_frame_matches() {
        oc684_hook_replay_fakes();
        fakes::covered(
            ee::s07::f_002703f0__generated,
            replay_step_frame_3,
            240,
            &|c, seed| {
                let _ = machine(c, seed, 0x0027_03f0, 1);
                oc684_gate(c);
                let mut r = Rng(0x2703_f0 ^ (seed as u64) << 20);
                let (replay, save) = (DATA, DATA + 0x2000);
                let kind = seed % 6;
                let tick = r.next_u32() % 1000;
                let (last, f61c) = match kind {
                    0 | 5 => (tick + 1 + seed % 2, 0),
                    1 => (tick - tick.min(seed % 2), 0),
                    _ => (tick - tick.min(seed % 2), 1),
                };
                let cur = match seed / 6 % 3 {
                    0 => 0,
                    1 => save,
                    _ => DATA + 0x3000,
                };
                oc684_w(c, replay + 0x3d0, save);
                oc684_w(c, replay + 0x488, cur);
                oc684_w(c, save + 0x18, DATA + 0x4000);
                oc684_w(c, replay + 0x484, tick);
                oc684_w(c, replay + 0x3cc, last);
                oc684_w(c, replay + 0x61c, f61c);
                let unwind = matches!(kind, 2 | 3 | 4);
                oc684_w(c, fakes::LOG_UNWIND_AT, if unwind { 0 } else { u32::MAX });
                c.g[A0][0] = replay as u64;
                c.g[A1][0] = matches!(kind, 3 | 4) as u64;
            },
        );
    }

    /// `Replay_OnRiderEvent`: each busy flag alone, no flag with a negative
    /// or valid rider index, and kinds 0 (empty hook), 1 (highlight) and 2
    /// (ignored).
    #[test]
    fn replay_on_rider_event_matches() {
        oc684_hook_replay_fakes();
        fakes::covered(
            ee::s07::f_002709d8__generated,
            replay_on_rider_event_3,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_09d8, 2);
                oc684_gate(c);
                let replay = DATA;
                // Which busy word is set: 0..=4, or none (5..=7).
                let busy = [0x61cu32, 0x620, 0x610, 0x60c, 0];
                let which = (seed % 8) as usize;
                for off in busy {
                    oc684_w(c, replay + off, 0);
                }
                if which < busy.len() {
                    oc684_w(c, replay + busy[which], 1 + r.next_u32() % 3);
                }
                let rider = seed / 8 % 3;
                let kind = seed / 24 % 3;
                // FindRiderIndex result: negative for one seed in three.
                let index = if seed / 72 % 3 == 0 { u32::MAX } else { rider };
                oc684_w(c, OC423_RET, index);
                // World -> manager -> rider table at +0x28 + 4 * rider.
                let (world, manager, table) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
                oc684_w(c, GAME_GP - 0x848, world);
                oc684_w(c, world + 0x84, manager);
                oc684_w(c, manager + 0xc, table);
                for i in 0..3u32 {
                    let obj = DATA + 0x4400 + 0x1000 * i;
                    oc684_w(c, table + 0x28 + 4 * i, obj);
                    oc684_w(c, obj + 0x790, r.next_u32());
                }
                c.g[A0][0] = replay as u64;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = kind as u64;
            },
        );
    }

    /// `Replay_LoadSaved`: world mode mismatch (returns 0), 0..=2 cache slots
    /// each set or not, no saved snapshots (the snapshot loop calls the
    /// unfakeable `Replay_DeserializeSnapshot` and is not reached).
    #[test]
    fn replay_load_saved_matches() {
        oc684_hook_replay_fakes();
        fakes::covered(
            ee::s07::f_00271758__generated,
            replay_load_saved_2,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_1758, 3);
                oc684_gate(c);
                let (replay, save) = (DATA, DATA + 0x1000);
                let caches = seed % 3;
                let mismatch = seed % 6 == 5;
                let (world, manager, ai) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
                oc684_w(c, GAME_GP - 0x848, world);
                oc684_w(c, world + 0x84, manager);
                oc684_w(c, manager + 0xc, ai);
                oc684_w(c, ai + 0x7c, caches + mismatch as u32);
                oc684_w(c, save + 0x4, 0);
                oc684_w(c, save + 0x8, r.next_u32());
                oc684_w(c, save + 0xc, r.next_u32() % 3);
                oc684_w(c, save + 0x10, r.next_u32());
                oc684_w(c, save + 0x14, r.next_u32());
                oc684_w(c, save + 0x18, caches);
                for i in 0..2u32 {
                    let set = (seed / 3 >> i) & 1 == 1;
                    oc684_w(
                        c,
                        replay + 0x48c + 4 * i,
                        if set { DATA + 0x3000 } else { 0 },
                    );
                }
                c.g[A0][0] = replay as u64;
                c.g[A1][0] = save as u64;
            },
        );
    }

    #[test]
    fn replay_recycle_oldest_snapshot_matches() {
        check(
            ee::s07::f_002701a0__generated,
            replay_recycle_oldest_snapshot_2,
            200,
            &|c, seed| {
                machine(c, seed, 0x1a0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x002701a0;
            },
        );
    }
}
