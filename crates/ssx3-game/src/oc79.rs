//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small front-end, script, replay, audio and render
//! helpers, each with a differential test against the generated body.
//!
//! The functions here are the small leaves and wrappers of wave 19: emitter
//! teardown, the FX trail ring, list-box/screen lookups, the AV player's
//! decode step, rider-gate timing, camera-controller message handling, a
//! family of front-end state activate/refresh/shutdown hooks, the keyboard
//! popup's enabled-key steppers, video-player movie setup, widget actions,
//! the replay stream's int peek, the sequencer track destructors, and the
//! script engine's process table / object table helpers.
//!
//! Every body keeps the original's stack frame, call protocol (direct calls
//! through [`Callee`], `jalr`/cross-shard calls through
//! [`crate::call::call_ptr`]), cycle charges and the registers the original
//! leaves behind at return.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, VSlot};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0037_14f8, memitter_dtor, "mEmitter_dtor"),
    (0x0037_15b0, memitter_free_data, "mEmitter_FreeData"),
    (0x0039_2d90, fxtrail_add_point, "FXTrail_AddPoint"),
    (0x0039_a8d8, uilistbox_set_mode, "UIListBox_SetMode"),
    (0x0039_c728, uiscreen_find_keyframe, "UIScreen_FindKeyframe"),
    (0x003a_e6c8, av_player_decode_frame, "AV_Player_DecodeFrame"),
    (0x0012_c028, ridergate_sub_is_start_time_reached, "RiderGateSub_IsStartTimeReached"),
    (0x0015_1368, be_char_profile_dtor, "BE_CharProfile_dtor"),
    (0x0015_cf80, manual_cam_controller_select_algorithm, "ManualCamController_SelectAlgorithm"),
    (0x0017_e368, lobby_menu_leave_occupants, "LobbyMenu_LeaveOccupants"),
    (0x0018_2870, cfe_state_cheat_char_select_apply_outfit, "cFEStateCheatCharSelect_ApplyOutfit"),
    (0x0019_6f00, cfe_state_request_line_on_activate, "cFEStateRequestLine_OnActivate"),
    (0x0019_9350, cfe_state_char_equip_shutdown, "cFEStateCharEquip_Shutdown"),
    (0x0019_a9b8, cfe_state_char_equip_refresh_item_info, "cFEStateCharEquip_RefreshItemInfo"),
    (0x0019_c880, bolt_file_index_manager_alloc_node, "BoltFileIndexManager_AllocNode"),
    (0x001a_3648, feasync_manager_init, "FEAsyncManager_Init"),
    (0x001a_4170, cfe_state_net_messenger_handle_input, "cFEStateNetMessenger_HandleInput"),
    (0x001a_c068, cfe_use_existing_account_dtor, "cFEUseExistingAccount_dtor"),
    (0x001b_95c0, cfe_state_create_step1_set_account_name_text, "cFEStateCreateStep1_SetAccountNameText"),
    (0x001c_4cd8, cfe_state_stats_v31, "cFEStateStats_v31"),
    (0x001c_c090, ckeyboard_popup_find_enabled_in_column_from_top, "cKeyboardPopup_FindEnabledInColumnFromTop"),
    (0x001c_c140, ckeyboard_popup_step_up_to_enabled_key, "cKeyboardPopup_StepUpToEnabledKey"),
    (0x001c_c190, ckeyboard_popup_step_down_to_enabled_key, "cKeyboardPopup_StepDownToEnabledKey"),
    (0x001d_2598, cfe_state_video_player_set_movie, "cFEStateVideoPlayer_SetMovie"),
    (0x001d_25e8, cfe_state_video_player_start_playback, "cFEStateVideoPlayer_StartPlayback"),
    (0x001e_3a78, message_center_set_flag, "MessageCenter_SetFlag"),
    (0x001e_8160, ov_screen_finish_ov_on_widget_action, "OVScreenFinishOv_OnWidgetAction"),
    (0x001e_8880, ov_screen_finish_ov2_on_widget_action, "OVScreenFinishOv2_OnWidgetAction"),
    (0x001f_6840, cfe_state_career_stats_set_text, "cFEStateCareerStats_SetText"),
    (0x001f_8118, ov_screen_top_times_v26, "OVScreenTopTimes_v26"),
    (0x0022_9788, crowd_man2d_add, "CrowdMan2d_Add"),
    (0x0023_6c88, game_state_pre_race_menu_exit, "GameStatePreRaceMenu_Exit"),
    (0x0024_1000, mc_manager_write_save_by_name_w, "MCManager_WriteSaveByNameW"),
    (0x0025_b688, lobby_man_destroy, "LobbyMan_Destroy"),
    (0x0026_5140, csetup_man_cancel_character, "cSetupMan_CancelCharacter"),
    (0x0026_59e8, cshared_login_man_request_card_info_if_idle, "cSharedLoginMan_RequestCardInfoIfIdle"),
    (0x0026_7e48, net_event_clone, "NetEvent_Clone"),
    (0x0026_ea68, replay_stream_peek_int, "ReplayStream_PeekInt"),
    (0x0027_0e18, replay_clear_all_highlights, "Replay_ClearAllHighlights"),
    (0x0027_2da0, inseq_span_track_dtor, "InSeq_SpanTrack_dtor"),
    (0x0027_31b8, inseq_event_track_dtor, "InSeq_EventTrack_dtor"),
    (0x0027_34e0, inseq_ramp_track_dtor, "InSeq_RampTrack_dtor"),
    (0x0027_72c0, script_list_on_seq_start, "ScriptList_OnSeqStart"),
    (0x0027_8308, script_engine_unload_master, "ScriptEngine_UnloadMaster"),
    (0x0027_ce58, script_engine_obj_table_dtor, "ScriptEngine_ObjTableDtor"),
    (0x0028_28b8, script_engine_reset_process_table, "ScriptEngine_ResetProcessTable"),
    (0x0028_2c38, script_engine_alloc_process, "ScriptEngine_AllocProcess"),
    (0x0028_31b0, script_cam_binding_accept_command, "ScriptCamBinding_AcceptCommand"),
    (0x0028_37c8, script_movie_dtor, "ScriptMovie_dtor"),
    (0x0028_54a8, ssx_audio_system_set_output_mode, "SSXAudioSystem_SetOutputMode"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    pub const MEMITTER_FREE_DATA: Callee = Callee { addr: 0x0037_15b0, f: ee::s12::f_003715b0 };
    pub const DYNAMIC_EMITTER_DTOR: Callee = Callee { addr: 0x0037_0c08, f: ee::s12::f_00370c08 };
    pub const DYNAMIC_EMITTER_FREE_DATA: Callee =
        Callee { addr: 0x0037_0cf8, f: ee::s12::f_00370cf8 };
    pub const RCMP_DECODE_FRAME: Callee = Callee { addr: 0x003b_0600, f: ee::s13::f_003b0600 };
    pub const RCMP_FRAME_NUMBER: Callee = Callee { addr: 0x003b_0580, f: ee::s13::f_003b0580 };
    pub const CAM_CONTROLLER_SET_ALGORITHM: Callee =
        Callee { addr: 0x0015_cf28, f: ee::s01::f_0015cf28 };
    pub const FE_STATE_ON_ACTIVATE: Callee = Callee { addr: 0x0018_6518, f: ee::s02::f_00186518 };
    pub const REQUEST_LINE_REFRESH_LIST: Callee =
        Callee { addr: 0x0019_7b88, f: ee::s02::f_00197b88 };
    pub const REQUEST_LINE_UPDATE_NOW_PLAYING: Callee =
        Callee { addr: 0x0019_7e70, f: ee::s02::f_00197e70 };
    pub const REQUEST_LINE_UPDATE_HELP_TEXT: Callee =
        Callee { addr: 0x0019_81f8, f: ee::s02::f_001981f8 };
    pub const FEMODULE_GET_RIDER_PREVIEW: Callee =
        Callee { addr: 0x001a_0548, f: ee::s03::f_001a0548 };
    pub const CHAR_EQUIP_REFRESH_BUY_INFO: Callee =
        Callee { addr: 0x0019_ab78, f: ee::s03::f_0019ab78 };
    pub const CHAR_EQUIP_REFRESH_EQUIP_INFO: Callee =
        Callee { addr: 0x0019_aa08, f: ee::s03::f_0019aa08 };
    pub const CHAR_EQUIP_UPDATE_HELP_TEXT: Callee =
        Callee { addr: 0x0019_a798, f: ee::s03::f_0019a798 };
    pub const BOLT_FILE_INDEX_GET_NEXT_NODE: Callee =
        Callee { addr: 0x0019_c8f8, f: ee::s03::f_0019c8f8 };
    pub const NET_MESSENGER_SET_PLAYER_MENU_SELECTION: Callee =
        Callee { addr: 0x001a_6c40, f: ee::s03::f_001a6c40 };
    pub const NET_FE_STATE_DTOR: Callee = Callee { addr: 0x001a_85d0, f: ee::s03::f_001a85d0 };
    pub const NET_FE_STATE_HANDLE_NET_EVENT: Callee =
        Callee { addr: 0x001a_8e40, f: ee::s03::f_001a8e40 };
    pub const FE_STATE_STATS_REFRESH_RESULTS: Callee =
        Callee { addr: 0x001c_3310, f: ee::s03::f_001c3310 };
    pub const KEYBOARD_POPUP_IS_KEY_ENABLED: Callee =
        Callee { addr: 0x001c_e560, f: ee::s04::f_001ce560 };
    pub const CROWD_MAN2D_INIT_SLOT: Callee = Callee { addr: 0x0022_9910, f: ee::s05::f_00229910 };
    pub const GAME_STATE_PRE_RACE_MENU_RESUME: Callee =
        Callee { addr: 0x0023_6cd8, f: ee::s06::f_00236cd8 };
    pub const LOBBY_MAN_DISCONNECT: Callee = Callee { addr: 0x0025_b800, f: ee::s06::f_0025b800 };
    pub const LOBBY_MAN_DTOR: Callee = Callee { addr: 0x0025_9628, f: ee::s06::f_00259628 };
    pub const GAME_COMM_DESTROY: Callee = Callee { addr: 0x0025_6c50, f: ee::s06::f_00256c50 };
    pub const SETUP_MAN_SEND_CHARACTER_MSG: Callee =
        Callee { addr: 0x0026_4e50, f: ee::s07::f_00264e50 };
    pub const SCRIPT_LIST_IS_PLAYING_SCRIPT: Callee =
        Callee { addr: 0x0027_71c8, f: ee::s07::f_002771c8 };
    pub const SCRIPT_DATA_GET_INFO: Callee = Callee { addr: 0x0027_c070, f: ee::s07::f_0027c070 };
    pub const SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE: Callee =
        Callee { addr: 0x0028_2020, f: ee::s07::f_00282020 };
    pub const SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS: Callee =
        Callee { addr: 0x0027_d050, f: ee::s07::f_0027d050 };
    pub const SCRIPT_ENGINE_GET_INSTANCE_SCRIPT: Callee =
        Callee { addr: 0x0027_a668, f: ee::s07::f_0027a668 };
    pub const SCRIPT_MOVIE_STOP: Callee = Callee { addr: 0x0028_3818, f: ee::s07::f_00283818 };
    pub const REPLAY_HIGHLIGHT_SLOT_CLEAR: Callee =
        Callee { addr: 0x0026_e800, f: ee::s07::f_0026e800 };
    pub const INSEQ_SPAN_TRACK_STOP: Callee = Callee { addr: 0x0027_2ec0, f: ee::s07::f_00272ec0 };
    pub const INSEQ_EVENT_TRACK_STOP: Callee = Callee { addr: 0x0027_32c8, f: ee::s07::f_002732c8 };
    pub const INSEQ_RAMP_TRACK_STOP: Callee = Callee { addr: 0x0027_3630, f: ee::s07::f_00273630 };
    pub const INSEQ_HEAP_DELETE: Callee = Callee { addr: 0x0027_24e8, f: ee::s07::f_002724e8 };

    // Cross-shard targets the generator emits as `call_indirect` (they go
    // through the function table like the original).
    pub const OPERATOR_DELETE: u32 = 0x0031_7e50;
    pub const EA_MEM_FREE: u32 = 0x0031_7e98;
    pub const FLOAT_TO_UINT: u32 = 0x0041_39f8;
    pub const STRCPY: u32 = 0x0041_66f4;
    pub const GET_HASH_VALUE32: u32 = 0x0031_7670;
    pub const UI_SCREEN_GET_OBJECT_BY_HASH_NAME: u32 = 0x0039_d860;
    pub const UI_TEXT_SET_ASCII_STRING: u32 = 0x003a_0c50;
    pub const UI_MENU_SET_SELECTED_BY_INDEX: u32 = 0x0039_b960;
    pub const UI_OBJECT_SET_VISIBLE: u32 = 0x0039_e4a0;
    pub const UI_WIDGET_SLOT_0: u32 = 0x0039_f840;
    pub const UI_WIDGET_SLOT_1: u32 = 0x0023_1320;
    pub const FE_STATE_UPDATE_BUTTONS_TEXT: u32 = 0x0019_8340;
    pub const MESSAGE_CENTER_MESSAGE: u32 = 0x002e_f378;
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    pub const AUDIO_APPLY_OUTPUT_MODE: u32 = 0x002b_3a70;
    pub const SOUND_PLAYER_SET_BY_INDEX: u32 = 0x0025_34a8;
    pub const SOUND_PLAYER_START: u32 = 0x0025_3860;
    pub const BE_GET_BE: u32 = 0x0014_dc80;
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const BE_INTERFACE_CALL: u32 = 0x0014_7a30;
    pub const CBXSTRING_CTOR: u32 = 0x0031_8260;
    pub const KEYBOARD_POPUP_SHOW: u32 = 0x0020_ab50;
    pub const MEMORY_FREE: u32 = 0x0025_23a8;
    pub const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    pub const AUDIO_SET_OUTPUT: u32 = 0x002a_df80;
}

/// The FPU constant the rider-gate timing multiplies the race clock by
/// (1/60: seconds per frame), at `gp - 0x7840`.
const FRAME_SECONDS_GP: i32 = -0x7840;
/// The game-module global (`D_004A28A8`, the world/app pointer).
const GAME_MODULE_GP: i32 = -0x848;
/// `D_004A26FC`: the "top times visible" flag.
const TOP_TIMES_VISIBLE_GP: i32 = -0x9f4;
/// `D_004A34DC`: the script-engine object table's live flag.
const SCRIPT_OBJ_TABLE_GP: i32 = 0x3ec;
/// `D_004A28A4`: the script-engine singleton.
const SCRIPT_ENGINE_GP: i32 = -0x84c;
/// `D_004A2A50` / `D_004A2A54`: the pre-race menu stack top and depth.
const PRE_RACE_STACK_TOP_GP: i32 = -0x6a0;
const PRE_RACE_STACK_DEPTH_GP: i32 = -0x69c;
/// `D_004A3028`: the lobby instance; `D_004A2EEC`: the game comm object.
const LOBBY_INSTANCE_GP: i32 = -0xc8;
const GAME_COMM_GP: i32 = -0x204;

// ---------------------------------------------------------------------------
// Small register and memory helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global address.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `a0 = a0 + off` (the `addiu` the generated code uses for arguments).
#[inline(always)]
fn add_arg(c: &mut Cpu, reg: usize, off: u32) {
    c.g[reg][0] = sx(r32(c, reg).wrapping_add(off));
}

// ---------------------------------------------------------------------------
// Emitter teardown.

/// `void mEmitter_dtor(mEmitter* self, int flags)` (0x3714f8): install the
/// class vtable at +0x1f8, release the emitter's data and destroy the
/// dynamic-emitter base. 19 cycles plus the callees.
pub fn memitter_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0049_30d0);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw(p(self_, 0x1f8), c.g[V0][0]);
        c.g[A1][0] = 0;
        callee::MEMITTER_FREE_DATA.call(c, 0x0037_1524)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::DYNAMIC_EMITTER_DTOR.call(c, 0x0037_1530)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void mEmitter_FreeData(mEmitter* self, int flags)` (0x3715b0): free the
/// owned point list at +0x204 and pass the flags to the base class. 8 (+3)
/// + 8 cycles plus the callees.
pub fn memitter_free_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let owned = c.lw(p(self_, 0x204));
        c.g[A0][0] = owned;
        c.g[S1][0] = c.g[A1][0];
        if owned != 0 {
            c.cycles += 3;
            call_ptr(c, callee::EA_MEM_FREE, 0x0037_15d8)?;
            c.sw(p(self_, 0x204), 0);
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::DYNAMIC_EMITTER_FREE_DATA.call(c, 0x0037_15e8)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FX trail.

/// `void FXTrail_AddPoint(FXTrail* t, const Quat* rot, const Vec* pos)`
/// (0x392d90): append one point — 12 bytes at `t+0x10+12*count` and the
/// quaternion at `t+0x50+16*count` — then bump the count. 19 cycles.
pub fn fxtrail_add_point(c: &mut Cpu) {
    c.cycles += 19;
    let t = r32(c, A0);
    c.g[V0][0] = c.lw(p(t, 0xc));
    c.g[V1][0] = 0xc;
    c.mult(0, A3, V0, V1);
    c.g[V0][0] = sx(r32(c, A3).wrapping_add(t));
    let pos = r32(c, A2);
    c.ldl(8, p(pos, 7));
    c.ldr(8, pos);
    c.g[T1][0] = c.lw(p(pos, 8));
    let slot = r32(c, V0);
    c.sdl(8, p(slot, 0x17));
    c.sdr(8, p(slot, 0x10));
    c.sw(p(slot, 0x18), c.g[T1][0]);
    c.g[V1][0] = c.lw(p(t, 0xc));
    let rot = r32(c, A1);
    c.lq(6, rot);
    c.g[V1][0] = sx(r32(c, V1) << 4);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(t));
    c.sq(6, p(r32(c, V1), 0x50));
    c.g[V0][0] = c.lw(p(r32(c, A0), 0xc));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    c.sw(p(t, 0xc), c.g[V0][0]);
    abi::ret(c);
}

/// `void UIListBox_SetMode(UIListBox* box, int mode)` (0x39a8d8): store the
/// column count, clear the "dirty layout" byte, flag the object dirty and
/// tell the owner (`box->+0x5c->+0xd0`, slot 0x98) to rebuild with 9.
/// 19 cycles plus the call.
pub fn uilistbox_set_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 9;
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        Addr(p(self_, 0x318)).sb(c, c.g[A1][0]);
        let mut flags = c.lw(p(self_, 0x74));
        c.g[V0][0] = flags;
        c.g[A1][0] = c.g[A0][0];
        Addr(p(self_, 0x319)).sb(c, 0);
        flags |= 0x10;
        c.sw(p(self_, 0x74), flags);
        let owner = c.lw(p(self_, 0x5c)) as u32;
        let obj = c.lw(p(owner, 0xd0));
        c.g[A3][0] = obj;
        let vt = c.lw(p(obj as u32, 8));
        c.g[V1][0] = vt;
        c.g[V0][0] = c.lw(p(vt as u32, 0x9c));
        let m = VSlot::read(c, vt, 0x98);
        c.g[A0][0] = m.this(obj);
        call_ptr(c, m.func as u32, 0x0039_a918)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIKeyframe* UIScreen_FindKeyframe(UIScreen* s, unsigned short frame)`
/// (0x39c728): walk the variable-length keyframe list at `s+0x34` until an
/// entry's `+8` matches `frame`; null when the list ends. 6 (+2, then
/// 3 (+4) per miss, 2 on a hit) + 2 cycles.
pub fn uiscreen_find_keyframe(c: &mut Cpu) {
    c.cycles += 6;
    let list = c.lw(p(r32(c, A0), 0x34)) as u32;
    c.g[A1][0] = c.g[A1][0] & 0xffff;
    c.g[A2][0] = 0;
    let count = c.lw(list);
    c.g[V1][0] = count;
    let mut entry = p(list, 4);
    c.g[A0][0] = sx(entry);
    if count != 0 {
        c.cycles += 2;
        c.g[A3][0] = count;
        loop {
            c.cycles += 3;
            let frame = Addr(p(entry, 8)).lhu(c);
            c.g[V0][0] = frame;
            if frame != c.g[A1][0] {
                let stride = c.lw(p(entry, 4));
                c.g[V1][0] = stride;
                c.cycles += 4;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                c.g[V0][0] = (c.g[A2][0] < c.g[A3][0]) as u64;
                entry = p(entry, stride as u32);
                c.g[A0][0] = sx(entry);
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_c748);
                    }
                    continue;
                }
                break;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx(entry);
                abi::ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// AV player.

/// `int AV_Player_DecodeFrame(AV_Player* p, float time)` (0x3ae6c8): store
/// the requested time at +0x48, decode that frame of the stream at +0x64,
/// remember the decoded index at +0x74 and the stream's frame number at
/// +0x44; return the decoded index. 19 cycles plus the callees.
pub fn av_player_decode_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.mov_s(0, 12);
        frame.save(c, S0, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.swc1(0, p(self_, 0x48));
        call_ptr(c, callee::FLOAT_TO_UINT, 0x003a_e6e4)?;
        c.g[A0][0] = c.lw(p(self_, 0x64));
        c.g[A1][0] = c.g[V0][0];
        callee::RCMP_DECODE_FRAME.call(c, 0x003a_e6f0)?;
        c.g[A0][0] = c.lw(p(self_, 0x64));
        c.sw(p(self_, 0x74), c.g[V0][0]);
        callee::RCMP_FRAME_NUMBER.call(c, 0x003a_e6fc)?;
        c.sw(p(self_, 0x44), c.g[V0][0]);
        c.g[V0][0] = c.lw(p(self_, 0x74));
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider gate timing.

/// `bool RiderGateSub_IsStartTimeReached(RiderGateSub* gate)` (0x12c028):
/// only while the race manager's state is 5, true when the race clock
/// (frames × 1/60) has reached the gate's start time (`gate->+0x14 ->
/// +0xb30`). 8 (+10 when racing) + 2 cycles.
pub fn ridergate_sub_is_start_time_reached(c: &mut Cpu) {
    c.cycles += 8;
    let module = c.lw(gp(c, GAME_MODULE_GP));
    c.g[A1][0] = c.g[A0][0];
    c.g[A0][0] = 5;
    let world = c.lw(p(module as u32, 0x84));
    let race = c.lw(p(world as u32, 0xc));
    let state = c.lw(race as u32);
    c.g[V1][0] = state;
    c.g[A2][0] = 0;
    let mut reached = 0u64;
    if state == c.g[A0][0] {
        c.cycles += 10;
        let frames = Addr(p(race as u32, 0xc)).read_fpu(c);
        c.f[0] = frames.0;
        c.cvt_s_w(0, 0);
        let seconds = Addr(gp(c, FRAME_SECONDS_GP)).read_fpu(c);
        c.f[1] = seconds.0;
        let rider = c.lw(p(r32(c, A1), 0x14));
        c.g[V1][0] = rider;
        c.mul_s(0, 0, 1);
        c.g[A0][0] = c.lw(p(rider as u32, 0xb30));
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
        reached = c.g[V0][0] ^ 1;
    }
    c.cycles += 2;
    c.g[A2][0] = reached;
    c.g[V0][0] = reached;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// BE character profile.

/// `void BE_CharProfile_dtor(BE_CharProfile* p, int flags)` (0x151368):
/// free the owned block at +0x288 and, when bit 0 of `flags` is set, delete
/// the object. 8 (+2) + 3 (+2) + 5 cycles plus the callees.
pub fn be_char_profile_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let owned = c.lw(p(self_, 0x288));
        c.g[A0][0] = owned;
        c.g[S1][0] = c.g[A1][0];
        if owned != 0 {
            c.cycles += 2;
            call_ptr(c, callee::EA_MEM_FREE, 0x0015_1390)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        c.sw(p(self_, 0x288), 0);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::OPERATOR_DELETE, 0x0015_13a4)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Camera controller.

/// `void ManualCamController_SelectAlgorithm(CamController* c, int msg)`
/// (0x15cf80): a camera message 0x50/0x51 switches the controller to the
/// manual algorithm's field at +0x1c/+0x18. 4 (+5, +4 or +2) + 2 cycles plus
/// the callee.
pub fn manual_cam_controller_select_algorithm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x50;
        let msg = c.g[A1][0];
        frame.save_ra(c, 0);
        if msg == c.g[V0][0] {
            c.cycles += 5;
            c.g[A1][0] = c.lw(p(r32(c, A0), 0x1c));
            c.g[A2][0] = 0;
            callee::CAM_CONTROLLER_SET_ALGORITHM.call(c, 0x0015_cfb0)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x51;
            if msg == c.g[V0][0] {
                c.cycles += 4;
                c.g[A1][0] = c.lw(p(r32(c, A0), 0x18));
                c.g[A2][0] = 0;
                callee::CAM_CONTROLLER_SET_ALGORITHM.call(c, 0x0015_cfc4)?;
            } else {
                c.cycles += 2;
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby menu.

/// `void LobbyMenu_LeaveOccupants(LobbyMenu* menu)` (0x17e368): destroy every
/// occupant in the list at +0x8, then mark the menu left (state 0xd at +0x4).
/// 8 (+5 per occupant) + 7 cycles plus the callees.
pub fn lobby_menu_leave_occupants(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        let mut cur = p(self_, 8);
        let first = c.lw(p(self_, 8));
        c.g[V0][0] = first;
        c.g[S0][0] = sx(cur);
        if first != 0 {
            loop {
                c.cycles += 5;
                c.g[A0][0] = sx(cur);
                call_ptr(c, 0x002c_c460, 0x0017_e390)?;
                cur = c.g[S0][0] as u32;
                c.g[V0][0] = c.lw(cur);
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0017_e388);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 0xd;
        frame.restore(c, 16, 0x20);
        c.sw(p(self_, 4), c.g[V0][0]);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end states: activate / refresh / shutdown hooks.

/// `void cFEStateCheatCharSelect_ApplyOutfit(cFEStateCheatCharSelect* s)`
/// (0x182870): for the six outfit slots at +0x54, point each non-null item's
/// +0x78 at -1 and its +0x7c at the current outfit table entry
/// (`s+0xf4+4*index`). 8 + 6×(3 (+4) + 3) + 2 cycles.
pub fn cfe_state_cheat_char_select_apply_outfit(c: &mut Cpu) {
    c.cycles += 8;
    let self_ = r32(c, A0);
    let owner = c.lw(p(self_, 0x6c));
    c.g[V1][0] = owner;
    c.g[A1][0] = sx(p(self_, 0xf4));
    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
    c.g[A0][0] = sx(p(self_, 0x54));
    let index = c.lbu(p(owner as u32, 0x98));
    c.g[V0][0] = index;
    c.g[A2][0] = 5;
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
    loop {
        c.cycles += 3;
        let item = c.lw(r32(c, A0));
        c.g[V1][0] = item;
        let next = p(r32(c, A1), 0);
        if item == 0 {
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(next);
            c.sw(p(item as u32, 0x78), c.g[A3][0]);
            c.sw(p(item as u32, 0x7c), c.g[V0][0]);
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
        }
        c.cycles += 3;
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0018_2890);
            }
            continue;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
}

/// `void cFEStateRequestLine_OnActivate(cFEStateRequestLine* s)` (0x196f00):
/// run the base activation, reset the request list, refresh the list and the
/// now-playing line, then update the button and help texts. 20 cycles plus
/// the callees.
pub fn cfe_state_request_line_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::FE_STATE_ON_ACTIVATE.call(c, 0x0019_6f14)?;
        c.sw(p(self_, 0x150), 0);
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_REFRESH_LIST.call(c, 0x0019_6f20)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_UPDATE_NOW_PLAYING.call(c, 0x0019_6f28)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::FE_STATE_UPDATE_BUTTONS_TEXT, 0x0019_6f34)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::REQUEST_LINE_UPDATE_HELP_TEXT.call(c, 0x0019_6f40)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_Shutdown(cFEStateCharEquipDetail* s)` (0x199350):
/// invalidate the rider preview's card (-1 at +0xc2c) when a module manager is
/// present, then run the base shutdown. 8 (+6) + 6 cycles plus the callees.
pub fn cfe_state_char_equip_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let module = c.lw(gp(c, GAME_MODULE_GP));
        c.g[V0][0] = module;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        let manager = c.lw(p(module as u32, 0x7c));
        c.g[A0][0] = manager;
        if manager != 0 {
            c.cycles += 6;
            c.g[A1][0] = Addr(p(self_, 0x44)).lb(c);
            add_arg(c, A0, 0xb0);
            callee::FEMODULE_GET_RIDER_PREVIEW.call(c, 0x0019_937c)?;
            c.g[V1][0] = 0xffff_ffff_ffff_ffff;
            c.sw(p(r32(c, V0), 0xc2c), c.g[V1][0]);
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::UI_OBJECT_SET_VISIBLE, 0x0019_9390)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_RefreshItemInfo(cFEStateCharEquipDetail* s)`
/// (0x19a9b8): refresh the buy panel when the detail flags word at +0xc8 is
/// 1, else the equip panel, then the help text. 8 (+4, +2) + 6 cycles plus
/// the callees.
pub fn cfe_state_char_equip_refresh_item_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(p(self_, 0xc8));
        c.g[V0][0] ^= 1;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            callee::CHAR_EQUIP_REFRESH_BUY_INFO.call(c, 0x0019_a9e0)?;
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::CHAR_EQUIP_REFRESH_EQUIP_INFO.call(c, 0x0019_a9f0)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::CHAR_EQUIP_UPDATE_HELP_TEXT.call(c, 0x0019_a9f8)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BoltFileIndexNode* BoltFileIndexManager_AllocNode(BoltFileIndexManager* m)`
/// (0x19c880): pop the free list at +0x1f40 and mark both the popped node's
/// and the new head's link words -1; return the popped node. 14 + 6 cycles
/// plus the callee.
pub fn bolt_file_index_manager_alloc_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        let node = c.lw(p(self_, 0x1f40));
        c.g[S1][0] = node;
        c.g[A1][0] = node;
        callee::BOLT_FILE_INDEX_GET_NEXT_NODE.call(c, 0x0019_c8a0)?;
        c.sw(p(self_, 0x1f40), c.g[V0][0]);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        Addr(p(node as u32, 0xe)).sh(c, c.g[V1][0]);
        c.g[V0][0] = c.lw(p(self_, 0x1f40));
        if c.g[V0][0] != 0 {
            Addr(p(r32(c, V0), 0xc)).sh(c, c.g[V1][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEAsyncManager_Init(FEAsyncManager* m)` (0x1a3648): initialise the
/// five 0x11c-byte async-file slots at +0, +0x11c, ... +0x470: name byte from
/// `D_004A1408`, +0x100 = 0, +0x104 = +0x110 = +0x118 = -1, +0x108 = 1,
/// +0x10c = +0x114 = 0. 6 + 5×12 + 2 cycles.
pub fn feasync_manager_init(c: &mut Cpu) {
    c.cycles += 6;
    let first = r32(c, A0);
    let end = p(first, 0x58c);
    c.g[A1][0] = sx(end);
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.g[A2][0] = 1;
    c.g[V0][0] = Addr(0x004a_1408).lbu(c);
    let mut cur = first;
    loop {
        c.cycles += 12;
        c.sw(p(cur, 0x100), 0);
        Addr(cur).sb(c, c.g[V0][0]);
        c.sw(p(cur, 0x104), c.g[V1][0]);
        c.sw(p(cur, 0x108), c.g[A2][0]);
        c.sw(p(cur, 0x10c), 0);
        c.sw(p(cur, 0x110), c.g[V1][0]);
        c.sw(p(cur, 0x114), 0);
        c.sw(p(cur, 0x118), c.g[V1][0]);
        c.g[A0][0] = sx(cur.wrapping_add(0x11c));
        cur = r32(c, A0);
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(0x004a_1408).lbu(c);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x001a_3660);
            }
            continue;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
}

/// `int cFEStateNetMessenger_HandleInput(cFEStateNetMessenger* s, void* menu)`
/// (0x1a4170): when the menu's `+8` vtable slot 0xd8 reports that it handled
/// the input, push the messenger's selection (+0x6f8) into the player menu and
/// return 1; otherwise return 0. 11 (+3, +2) + 4 cycles plus the callees.
pub fn cfe_state_net_messenger_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        let arg = r32(c, A1);
        let vt = c.lw(p(arg, 8));
        c.g[V1][0] = vt;
        c.g[V0][0] = c.lw(p(vt as u32, 0xdc));
        let m = VSlot::read(c, vt, 0xd8);
        c.g[A0][0] = m.this(sx(arg));
        call_ptr(c, m.func as u32, 0x001a_4194)?;
        let handled = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];
        let mut result = 0u64;
        if handled {
            c.cycles += 3;
            c.g[A1][0] = c.lw(p(self_, 0x6f8));
            callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION.call(c, 0x001a_41ac)?;
            c.g[V0][0] = 1;
            result = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        c.g[V0][0] = result;
        Ok(())
    })
}

/// `void cFEUseExistingAccount_dtor(cFEUseExistingAccount* s, int flags)`
/// (0x1ac068): install the vtable at +8, destruct the account-name string at
/// +0x6dc and run the base destructor. 20 cycles plus the callees.
pub fn cfe_use_existing_account_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0046_9068);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw(p(self_, 8), c.g[V0][0]);
        c.g[A0][0] = sx(p(self_, 0x6dc));
        c.g[A1][0] = 2;
        call_ptr(c, callee::CBXSTRING_CTOR, 0x001a_c098)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::NET_FE_STATE_DTOR.call(c, 0x001a_c0a4)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCreateStep1_SetAccountNameText(cFEStateCreateStep1* s)`
/// (0x1b95c0): look the account-name text object up by hash in the screen at
/// +0x40 and set it to the string held at `s+0x6d0`. 13 (+4, +3) + 3 cycles
/// plus the callees.
pub fn cfe_state_create_step1_set_account_name_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_2b88);
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001b_95dc)?;
        c.g[A0][0] = c.lw(p(self_, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME, 0x001b_95e8)?;
        c.g[A0][0] = c.g[V0][0];
        if c.g[A0][0] == 0 {
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(p(self_, 0x6d0));
            c.g[A1][0] = c.lw(r32(c, V0));
            call_ptr(c, callee::UI_TEXT_SET_ASCII_STRING, 0x001b_9600)?;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateStats_v31(cFEStateStats* s, int msg)` (0x1c4cd8): message
/// 0xe1 refreshes the results, selects the first entry of the table at +0x6dc
/// and returns 0; every other message goes to the net front-end handler.
/// 6 (+7 or +4) + 3 cycles plus the callees.
pub fn cfe_state_stats_v31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xe1;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let msg = c.g[A1][0];
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        if msg == c.g[V0][0] {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            callee::FE_STATE_STATS_REFRESH_RESULTS.call(c, 0x001c_4d08)?;
            c.g[A0][0] = c.lw(p(self_, 0x6dc));
            c.g[A1][0] = 0;
            call_ptr(c, callee::UI_MENU_SET_SELECTED_BY_INDEX, 0x001c_4d14)?;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            callee::NET_FE_STATE_HANDLE_NET_EVENT.call(c, 0x001c_4cf8)?;
        }
        c.cycles += 3;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Keyboard popup: enabled-key steppers.

/// The body of the three steppers below: start at `cur + first`, add `step`
/// while `KeyboardPopup_IsKeyEnabled(popup, index, 0)` is false and store the
/// found index at +0x140. `head` is the event-check address the original
/// resumes at; `call_ret` the return address after the enabled check.
fn keyboard_popup_step(c: &mut Cpu, first: i32, step: u32, head: u32, call_ret: u32) -> Flow {
    if c.pc != head {
        c.cycles += 8;
        let sp = sp(c).wrapping_sub(0x30);
        c.g[SP][0] = sx(sp);
        c.sq(16, p(sp, 0x20));
        c.sq(17, p(sp, 0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd(p(sp, 0), c.g[RA][0]);
        let self_ = r32(c, A0);
        c.g[V0][0] = c.lw(p(self_, 0x140));
        c.g[S0][0] = sx(r32(c, V0).wrapping_add(first as u32));
    } else {
        c.cycles += 1;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(step));
    }
    loop {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        callee::KEYBOARD_POPUP_IS_KEY_ENABLED.call(c, call_ret)?;
        let disabled = c.g[V0][0] == 0;
        let sp = sp(c);
        c.g[RA][0] = c.ld(p(sp, 0));
        if disabled {
            if back_edge_unwinds(c, head) {
                return Ok(());
            }
            c.cycles += 1;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(step));
            continue;
        }
        c.cycles += 5;
        c.sw(p(r32(c, S1), 0x140), c.g[S0][0]);
        c.lq(16, p(sp, 0x20));
        c.lq(17, p(sp, 0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp.wrapping_add(0x30));
        c.pc = ra;
        return Ok(());
    }
}

/// `void cKeyboardPopup_FindEnabledInColumnFromTop(cKeyboardPopup* p)`
/// (0x1cc090): from 0x38 keys above the current index, step down 14 keys at a
/// time to the first enabled one.
pub fn ckeyboard_popup_find_enabled_in_column_from_top(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_step(c, -0x38, 0xe, 0x001c_c0b0, 0x001c_c0c4))
}

/// `void cKeyboardPopup_StepUpToEnabledKey(cKeyboardPopup* p)` (0x1cc140):
/// step up (14 keys) from the current index to the first enabled key.
pub fn ckeyboard_popup_step_up_to_enabled_key(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_step(c, -0xe, 0xffff_fff2, 0x001c_c160, 0x001c_c174))
}

/// `void cKeyboardPopup_StepDownToEnabledKey(cKeyboardPopup* p)` (0x1cc190):
/// step down (14 keys) from the current index to the first enabled key.
pub fn ckeyboard_popup_step_down_to_enabled_key(c: &mut Cpu) {
    guest(c, |c| keyboard_popup_step(c, 0xe, 0xe, 0x001c_c1b0, 0x001c_c1c4))
}

// ---------------------------------------------------------------------------
// Video player.

/// `void cFEStateVideoPlayer_SetMovie(s, const char* name, const char* movie)`
/// (0x1d2598): copy `name` into +0x48 and, when non-null, `movie` into +0x148
/// (else clear the first byte there). 10 (+4) + 2 (+4) cycles plus the copies.
pub fn cfe_state_video_player_set_movie(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(self_, 0x48));
        call_ptr(c, callee::STRCPY, 0x001d_25b8)?;
        let movie = c.g[S1][0];
        c.g[A0][0] = sx(p(self_, 0x148));
        if movie == 0 {
            c.cycles += 2;
            Addr(p(self_, 0x148)).sb(c, 0);
            frame.restore(c, 16, 0x20);
        } else {
            c.cycles += 4;
            c.g[A1][0] = movie;
            call_ptr(c, callee::STRCPY, 0x001d_25c8)?;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateVideoPlayer_StartPlayback(cFEStateVideoPlayer* s)`
/// (0x1d25e8): flush the audio output mode, bind the movie player, set the
/// playing flag at +0x254, start the sound and clear the two seek words.
/// 20 cycles plus the callees.
pub fn cfe_state_video_player_start_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::AUDIO_GET_INSTANCE, 0x001d_25fc)?;
        c.g[A0][0] = sx(p(r32(c, V0), 0x118));
        call_ptr(c, callee::AUDIO_APPLY_OUTPUT_MODE, 0x001d_2604)?;
        c.g[A0][0] = c.lw(p(self_, 0x248));
        c.g[V0][0] = 1;
        c.g[A1][0] = sx(p(self_, 0x24c));
        c.sw(p(self_, 0x254), c.g[V0][0]);
        call_ptr(c, callee::SOUND_PLAYER_SET_BY_INDEX, 0x001d_2618)?;
        c.g[A0][0] = c.lw(p(self_, 0x248));
        call_ptr(c, callee::SOUND_PLAYER_START, 0x001d_2620)?;
        c.sw(p(self_, 0x274), 0);
        c.sw(p(self_, 0x278), 0);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Message center.

/// `void MessageCenter_SetFlag(int a, int b)` (0x1e3a78): get the BE
/// interface and post the message through its handler with the two arguments.
/// 20 cycles plus the callees.
pub fn message_center_set_flag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let a = c.g[A0][0];
        c.g[S0][0] = a;
        frame.save_ra(c, 0);
        let b = c.g[A1][0];
        c.g[S1][0] = b;
        call_ptr(c, callee::BE_GET_BE, 0x001e_3a94)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, callee::BE_GET_INTERFACE, 0x001e_3aa0)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::BE_INTERFACE_CALL, 0x001e_3ab4)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Overlay screens.

/// The shared body of the two `OnWidgetAction` handlers (0x1e8160 and
/// 0x1e8880): widget 5 with a live source and a module hides the current
/// widget and refreshes the world HUD. `ret0`/`ret1` are the return addresses
/// of the two calls.
fn widget_action_finish(c: &mut Cpu, ret0: u32, ret1: u32) -> Flow {
    c.cycles += 4;
    let frame = Frame::open(c, 0x20);
    frame.save(c, 16, 0x10);
    let source_null = c.g[A1][0] == 0;
    frame.save_ra(c, 0);
    let mut extra = source_null;
    if !source_null {
        c.cycles += 3;
        c.g[V0][0] = 5;
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 3;
            let field = c.lw(p(r32(c, A0), 0xa0));
            c.g[V0][0] = field;
            let module = c.lw(gp(c, GAME_MODULE_GP));
            c.g[V0][0] = module;
            if field != 0 {
                c.cycles += 6;
                let world = c.lw(p(module as u32, 0x84));
                c.g[S0][0] = world;
                let widget = c.lw(p(world as u32, 0x48));
                c.g[A0][0] = sx((widget as u32).wrapping_add(0x18));
                call_ptr(c, callee::UI_WIDGET_SLOT_0, ret0)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::UI_WIDGET_SLOT_1, ret1)?;
                extra = true;
            }
        }
    }
    if extra {
        c.cycles += 1;
    }
    frame.restore(c, 16, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void OVScreenFinishOv_OnWidgetAction(s, void* source, int action)`
/// (0x1e8160).
pub fn ov_screen_finish_ov_on_widget_action(c: &mut Cpu) {
    guest(c, |c| widget_action_finish(c, 0x001e_8198, 0x001e_81a0))
}

/// `void OVScreenFinishOv2_OnWidgetAction(s, void* source, int action)`
/// (0x1e8880).
pub fn ov_screen_finish_ov2_on_widget_action(c: &mut Cpu) {
    guest(c, |c| widget_action_finish(c, 0x001e_88b8, 0x001e_88c0))
}

/// `void cFEStateCareerStats_SetText(s, const char* name, const char* text)`
/// (0x1f6840): find the text object called `name` in the screen at +0x40 and
/// set it to `text`. 13 (+2) + 5 cycles plus the callees.
pub fn cfe_state_career_stats_set_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001f_6860)?;
        c.g[A0][0] = c.lw(p(self_, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME, 0x001f_686c)?;
        let text = c.g[V0][0];
        c.g[A0][0] = text;
        if text != 0 {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, callee::UI_TEXT_SET_ASCII_STRING, 0x001f_687c)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenTopTimes_v26(s, void* widget)` (0x1f8118): once the screen's
/// +0xa0 word is clear, interpret the widget's `+0x18` selection: 0 shows the
/// top times (setting the flag at +0xa0 and `D_004A26FC`), 1 plays the
/// confirmation sound. 5 (+3, +2, +3) + 2 (+2) + 1 + 2 cycles.
pub fn ov_screen_top_times_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        let busy = c.lw(p(self_, 0xa0));
        c.g[V0][0] = busy;
        frame.restore_ra(c, 0);
        if busy == 0 {
            c.cycles += 3;
            let sel = c.lw(p(r32(c, A1), 0x18));
            c.g[A1][0] = sel;
            let none = sel == 0;
            c.g[V0][0] = 1;
            if none {
                c.cycles += 3;
                c.sw(p(self_, 0xa0), c.g[V0][0]);
                c.sw(gp(c, TOP_TIMES_VISIBLE_GP), c.g[V0][0]);
                c.cycles += 1;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                if sel == c.g[V0][0] {
                    c.cycles += 2;
                    c.sw(p(self_, 0xa0), c.g[A1][0]);
                    c.cycles += 2;
                    c.g[A0][0] = 0x13;
                    call_ptr(c, callee::KEYBOARD_POPUP_SHOW, 0x001f_815c)?;
                    c.cycles += 1;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Crowd manager.

/// `void CrowdMan2d_Add(CrowdMan2d* m, int mode)` (0x229788): find the first
/// slot of the 64 entries at +8 whose word is -1 and initialise it with
/// `mode` (and the caller's second argument). 8 + 3 (+3 (+1), +3) per slot +
/// 2 cycles plus the callee. The event check resumes at 0x2297a8.
pub fn crowd_man2d_add(c: &mut Cpu) {
    guest(c, |c| {
        let resume = c.pc == 0x0022_97a8;
        if !resume {
            c.cycles += 8;
            let frame = Frame::open(c, 0x10);
            c.g[A3][0] = 0xffff_ffff_ffff_ffff;
            c.g[A2][0] = c.g[A1][0];
            frame.save_ra(c, 0);
            c.g[A1][0] = 0;
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(8));
        } else {
            c.cycles += 1;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        }
        let frame = Frame { size: 0x10 };
        loop {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 64i64) as u64;
            let exhausted = c.g[V0][0] == 0;
            frame.restore_ra(c, 0);
            if exhausted {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, V1));
            let taken = c.g[V0][0] != c.g[A3][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            if taken {
                if back_edge_unwinds(c, 0x0022_97a8) {
                    return Ok(());
                }
                c.cycles += 1;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                continue;
            }
            c.cycles += 3;
            callee::CROWD_MAN2D_INIT_SLOT.call(c, 0x0022_97cc)?;
            frame.restore_ra(c, 0);
            break;
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Game state.

/// `void GameStatePreRaceMenu_Exit(GameStatePreRaceMenu* s)` (0x236c88): pop
/// the pre-race menu stack (`D_004A2A54` deep, `D_005366E8` indexes into
/// `D_004428F0`) into the current-menu global and resume the race.
/// 20 cycles plus the callee.
pub fn game_state_pre_race_menu_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let depth = c.lw(gp(c, PRE_RACE_STACK_DEPTH_GP));
        c.g[V0][0] = depth;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(0x0053_66e8);
        frame.save_ra(c, 0);
        let new_depth = sx(r32(c, V0).wrapping_sub(1));
        c.g[V0][0] = new_depth;
        c.g[V1][0] = sx(r32(c, V0) << 2);
        c.sw(gp(c, PRE_RACE_STACK_DEPTH_GP), c.g[V0][0]);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[A2][0] = sx(0x0044_28f0);
        c.g[V0][0] = c.lw(r32(c, V1));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
        c.g[V1][0] = c.lw(r32(c, V0));
        c.sw(gp(c, PRE_RACE_STACK_TOP_GP), c.g[V1][0]);
        callee::GAME_STATE_PRE_RACE_MENU_RESUME.call(c, 0x0023_6ccc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Save-game writing.

/// `void MCManager_WriteSaveByNameW(MCManager* m, int a1, int a2, int name)`
/// (0x241000): mark the save in progress (`D_004A2C78 = 1`) and forward to the
/// memory-card manager object at `m+0x434` (vtable slot 0x128); remember the
/// name word at +0xe4. 20 cycles plus the call.
pub fn mc_manager_write_save_by_name_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let arg3 = c.g[A3][0];
        c.g[S0][0] = arg3;
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        let obj = c.lw(p(self_, 0x434));
        c.g[T0][0] = obj;
        c.sw(gp(c, -0x478), c.g[V0][0]);
        let vt = c.lw(obj as u32);
        c.g[V1][0] = vt;
        c.g[V0][0] = c.lw(p(vt as u32, 0x12c));
        let m = VSlot::read(c, vt, 0x128);
        c.g[A0][0] = m.this(obj);
        call_ptr(c, m.func as u32, 0x0024_1038)?;
        c.sw(p(self_, 0xe4), c.g[S0][0]);
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby / setup managers.

/// `void LobbyMan_Destroy()` (0x25b688): tear the lobby manager down (and, if
/// it is still alive, delete it), then the game comm object.
/// 4 (+5, +3) + 3 (+3) + 2 cycles plus the callees.
pub fn lobby_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let lobby = c.lw(gp(c, LOBBY_INSTANCE_GP));
        c.g[A0][0] = lobby;
        let none = lobby == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 5;
            callee::LOBBY_MAN_DISCONNECT.call(c, 0x0025_b6a0)?;
            let alive = c.lw(gp(c, LOBBY_INSTANCE_GP));
            c.g[A0][0] = alive;
            if alive == 0 {
                c.sw(gp(c, LOBBY_INSTANCE_GP), 0);
            } else {
                c.cycles += 3;
                c.g[A1][0] = 3;
                callee::LOBBY_MAN_DTOR.call(c, 0x0025_b6b4)?;
                c.sw(gp(c, LOBBY_INSTANCE_GP), 0);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(gp(c, GAME_COMM_GP));
        let no_comm = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !no_comm {
            c.cycles += 3;
            callee::GAME_COMM_DESTROY.call(c, 0x0025_b6cc)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_CancelCharacter(cSetupMan* m)` (0x265140): send the
/// character message that cancels the pending selection: 5 (event 4 for a
/// live character, else 5; the state word at +0 moves 4 → 2) is passed with
/// the current state as the last argument. 5 (+3 (+2 (+3) or +1)) + 3 + 3
/// cycles plus the callee.
pub fn csetup_man_cancel_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        let self_ = r32(c, A0);
        let state = c.lw(self_);
        c.g[V1][0] = state;
        c.g[V0][0] = 0xc;
        let idle = state == c.g[V0][0];
        frame.save_ra(c, 0);
        if !idle {
            c.cycles += 3;
            let pending = c.lw(p(self_, 0x124));
            c.g[V0][0] = pending;
            let none = pending == 0;
            c.g[V0][0] = 4;
            if none {
                c.cycles += 1;
                c.g[A1][0] = 5;
            } else {
                c.cycles += 2;
                c.g[A1][0] = 4;
                let four = c.g[V0][0];
                if state == four {
                    c.cycles += 3;
                    c.g[V0][0] = 2;
                    c.sw(self_, c.g[V0][0]);
                }
            }
            c.cycles += 3;
            c.g[A3][0] = c.lw(self_);
            c.g[A2][0] = 0xffff_ffff_ffff_ffff;
            callee::SETUP_MAN_SEND_CHARACTER_MSG.call(c, 0x0026_5184)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_RequestCardInfoIfIdle()` (0x2659e8): when the card
/// manager's slot 0x58 says it is busy, return; else request the card info
/// held at `D_004A33DC` through slot 0x28. 10 (+8) + 2 cycles plus the calls.
pub fn cshared_login_man_request_card_info_if_idle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw(gp(c, 0x2e8));
        frame.save_ra(c, 0);
        let obj = r32(c, A1);
        let vt = c.lw(obj);
        c.g[V1][0] = vt;
        c.g[V0][0] = c.lw(p(vt as u32, 0x5c));
        let m = VSlot::read(c, vt, 0x58);
        c.g[A0][0] = m.this(sx(obj));
        call_ptr(c, m.func as u32, 0x0026_5a08)?;
        let busy = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if busy {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 8;
        let obj2 = c.lw(gp(c, 0x2e8)) as u32;
        c.g[A2][0] = c.lw(gp(c, 0x2e8));
        c.g[A1][0] = c.lw(gp(c, 0x2ec));
        let vt2 = c.lw(obj2);
        c.g[V0][0] = vt2;
        c.g[A0][0] = Addr(p(vt2 as u32, 0x28)).lh(c);
        c.g[V1][0] = c.lw(p(vt2 as u32, 0x2c));
        c.g[A0][0] = sx(obj2.wrapping_add(r32(c, A0)));
        call_ptr(c, r32(c, V1), 0x0026_5a2c)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Network events.

/// `NetEvent* NetEvent_Clone(NetEvent* e)` (0x267e48): allocate 8 bytes from
/// the network heap (tagged with `D_00480488`) and copy the event's first
/// word and the class vtable at `D_004812B0`. 20 cycles plus the allocation.
pub fn net_event_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx(0x0048_0488);
        frame.save(c, 16, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0026_7e70)?;
        c.g[A1][0] = c.lw(self_);
        c.g[V1][0] = sx(0x0048_12b0);
        c.g[A0][0] = c.g[V0][0];
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        c.sw(p(r32(c, A0), 4), c.g[V1][0]);
        c.sw(r32(c, A0), c.g[A1][0]);
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
        c.sd(p(frame.at(c, 0), 0x10), c.g[RA][0]);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        let pos = c.lw(p(self_, 0xc));
        c.g[S1][0] = pos;
        let vt = c.lw(self_);
        c.g[V1][0] = vt;
        c.g[V0][0] = c.lw(p(vt as u32, 0x14));
        let m = VSlot::read(c, vt, 0x10);
        c.g[A0][0] = m.this(sx(self_));
        call_ptr(c, m.func as u32, 0x0026_ea9c)?;
        c.sw(p(self_, 0xc), c.g[S1][0]);
        frame.restore(c, 16, 0x30);
        c.g[V0][0] = Addr(frame.at(c, 0)).lh(c);
        frame.restore(c, 17, 0x20);
        c.g[RA][0] = c.ld(p(frame.at(c, 0), 0x10));
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
            let mut slot = p(self_, 0x494);
            c.g[S0][0] = sx(slot);
            c.g[S1][0] = 1;
            loop {
                c.cycles += 5;
                c.g[A0][0] = sx(slot);
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                callee::REPLAY_HIGHLIGHT_SLOT_CLEAR.call(c, 0x0027_0e4c)?;
                let more = c.g[S1][0] as i64 >= 0;
                slot = p(slot, 0xb4);
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
fn inseq_track_dtor(c: &mut Cpu, vt: u32, stop: Callee, ret: u32) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(vt);
    c.sq(17, p(sp(c), 0x10));
    c.sq(16, p(sp(c), 0x20));
    c.g[S1][0] = c.g[A0][0];
    c.sd(p(sp(c), 0), c.g[RA][0]);
    let flags = c.g[A1][0];
    c.g[S0][0] = flags;
    c.sw(p(r32(c, S1), 4), c.g[V0][0]);
    stop.call(c, ret)?;
    c.g[S0][0] = c.g[S0][0] & 1;
    let keep = c.g[S0][0] == 0;
    c.lq(16, p(sp(c), 0x20));
    if !keep {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        callee::INSEQ_HEAP_DELETE.call(c, ret + 0x14)?;
        c.lq(16, p(sp(c), 0x20));
    }
    c.cycles += 4;
    c.lq(17, p(sp(c), 0x10));
    c.g[RA][0] = c.ld(p(sp(c), 0));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
    c.pc = ra;
    Ok(())
}

/// `void InSeq_SpanTrack_dtor(InSeq_SpanTrack* t, int flags)` (0x272da0).
pub fn inseq_span_track_dtor(c: &mut Cpu) {
    guest(c, |c| inseq_track_dtor(c, 0x0048_19b0, callee::INSEQ_SPAN_TRACK_STOP, 0x0027_2dc8))
}

/// `void InSeq_EventTrack_dtor(InSeq_EventTrack* t, int flags)` (0x2731b8).
pub fn inseq_event_track_dtor(c: &mut Cpu) {
    guest(c, |c| inseq_track_dtor(c, 0x0048_1950, callee::INSEQ_EVENT_TRACK_STOP, 0x0027_31e0))
}

/// `void InSeq_RampTrack_dtor(InSeq_RampTrack* t, int flags)` (0x2734e0).
pub fn inseq_ramp_track_dtor(c: &mut Cpu) {
    guest(c, |c| inseq_track_dtor(c, 0x0048_18f0, callee::INSEQ_RAMP_TRACK_STOP, 0x0027_3508))
}

// ---------------------------------------------------------------------------
// Script list / data manager.

/// `void ScriptList_OnSeqStart(ScriptList* list, void* seq, ScriptObj* obj)`
/// (0x2772c0): when the sequence's script is playing and it is a named script,
/// post the "sequence started" message from `D_004A34A8`.
/// 8 (+6, +2) + 1 + 3 cycles plus the callees.
pub fn script_list_on_seq_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        let obj = c.g[A2][0];
        c.g[S0][0] = obj;
        frame.save_ra(c, 0);
        c.g[A1][0] = obj;
        callee::SCRIPT_LIST_IS_PLAYING_SCRIPT.call(c, 0x0027_72d8)?;
        if c.g[V0][0] == 0 {
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            callee::SCRIPT_DATA_GET_INFO.call(c, 0x0027_72e8)?;
            c.g[V1][0] = Addr(r32(c, V0)).lb(c);
            c.g[V0][0] = sx(0x004a_0000);
            let unnamed = c.g[V1][0] == 0;
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x34a8));
            if !unnamed {
                c.cycles += 2;
                call_ptr(c, callee::MESSAGE_CENTER_MESSAGE, 0x0027_7300)?;
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_UnloadMaster(ScriptEngine* e)` (0x278308): destroy the
/// script database, free the two object-table blocks at +0x52c and +0x534 and
/// clear them (and the +0x554 word). 10 (+3) + 7 cycles plus the callees.
pub fn script_engine_unload_master(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE.call(c, 0x0027_831c)?;
        c.g[A0][0] = c.lw(p(self_, 0x52c));
        call_ptr(c, callee::MEMORY_FREE, 0x0027_8324)?;
        let second = c.lw(p(self_, 0x534));
        c.g[A0][0] = second;
        if second == 0 {
            c.sw(p(self_, 0x554), 0);
        } else {
            c.cycles += 3;
            call_ptr(c, callee::MEMORY_FREE, 0x0027_8338)?;
            c.sw(p(self_, 0x554), 0);
        }
        c.cycles += 7;
        c.sw(p(self_, 0x52c), 0);
        c.sw(p(self_, 0x530), 0);
        c.sw(p(self_, 0x534), 0);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the two engine-object destructors below: install the
/// class vtable at `vt_off`, remove all children and, when bit 0 of `flags` is
/// set, delete the object. 13 (+2) + 5 cycles plus the callees.
fn script_obj_dtor(
    c: &mut Cpu,
    vt: u32,
    vt_off: u32,
    clear_global: bool,
    remove: Callee,
    remove_ret: u32,
    tail: u32,
) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(vt);
    c.sq(17, p(sp(c), 0x10));
    c.sq(16, p(sp(c), 0x20));
    c.g[S1][0] = c.g[A0][0];
    c.sd(p(sp(c), 0), c.g[RA][0]);
    c.g[S0][0] = c.g[A1][0];
    let self_ = r32(c, S1);
    c.sw(p(self_, vt_off), c.g[V0][0]);
    remove.call(c, remove_ret)?;
    c.g[S0][0] = c.g[S0][0] & 1;
    let keep = c.g[S0][0] == 0;
    if clear_global {
        c.sw(gp(c, SCRIPT_OBJ_TABLE_GP), 0);
    }
    if !keep {
        c.cycles += 2;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::OPERATOR_DELETE, remove_ret + 0x14)?;
    }
    c.cycles += tail as u64;
    frame.restore(c, 16, 0x20);
    frame.restore(c, 17, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void ScriptEngine_ObjTableDtor(ScriptEngine* e, int flags)` (0x27ce58).
pub fn script_engine_obj_table_dtor(c: &mut Cpu) {
    guest(c, |c| {
        script_obj_dtor(
            c,
            0x0048_2528,
            0x2a8,
            true,
            callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS,
            0x0027_ce80,
            5,
        )
    })
}

/// `void ScriptMovie_dtor(ScriptMovie* m, int flags)` (0x2837c8).
pub fn script_movie_dtor(c: &mut Cpu) {
    guest(c, |c| {
        script_obj_dtor(c, 0x0048_2228, 0x18, false, callee::SCRIPT_MOVIE_STOP, 0x0028_37f0, 4)
    })
}

/// `void ScriptEngine_ResetProcessTable(ScriptEngine* e)` (0x2828b8): clear
/// the process count at +0x2b8/+0x2bc and the sixteen 0x24-byte process
/// entries at +0x2c0: id = i, +0x2d4 = -1, everything else 0.
/// 6 + 16×12 + 2 cycles.
pub fn script_engine_reset_process_table(c: &mut Cpu) {
    c.cycles += 6;
    let base = r32(c, A0);
    c.sw(p(base, 0x2b8), 0);
    c.g[V1][0] = sx(base);
    c.sw(p(base, 0x2bc), 0);
    c.g[A1][0] = 0;
    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
    let mut slot = base;
    loop {
        c.cycles += 12;
        c.sw(p(slot, 0x2dc), c.g[A1][0]);
        c.sw(p(slot, 0x2e0), 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        c.sw(p(slot, 0x2c0), 0);
        c.g[V0][0] = ((c.g[A1][0] as i64) < 16i64) as u64;
        c.sw(p(slot, 0x2c4), 0);
        Addr(p(slot, 0x2d0)).sh(c, 0);
        Addr(p(slot, 0x2d2)).sh(c, 0);
        c.sw(p(slot, 0x2d8), 0);
        c.sw(p(slot, 0x2d4), c.g[A0][0]);
        let more = c.g[V0][0] != 0;
        slot += 0x24;
        c.g[V1][0] = sx(slot);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0028_28d0);
            }
            continue;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
}

/// `ScriptThread* ScriptEngine_AllocProcess(ScriptEngine* e, int type)`
/// (0x282c38): find the first of the sixteen process entries at +0x2c0 whose
/// type word (+0x2e0) is 0, set it to `type` (a type-1 process gets handle -1)
/// and return the entry; 0 when the table is full.
/// 6 + 3 (+4) + 2 / 2 (+1) + 2 cycles.
pub fn script_engine_alloc_process(c: &mut Cpu) {
    c.cycles += 6;
    let base = r32(c, A0);
    let mut type_slot = p(base, 0x2e0);
    c.g[V1][0] = sx(type_slot);
    let ty = c.g[A1][0];
    c.g[A2][0] = 0;
    c.g[T0][0] = 1;
    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
    c.g[A0][0] = sx(p(base, 0x2c0));
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw(type_slot);
        if c.g[V0][0] != 0 {
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            c.cycles += 4;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x24));
            c.g[V0][0] = ((c.g[A2][0] as i64) < 16i64) as u64;
            let more = c.g[V0][0] != 0;
            type_slot += 0x24;
            c.g[V1][0] = sx(type_slot);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0028_2c50);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        let plain = ty != c.g[T0][0];
        c.sw(type_slot, c.g[A1][0]);
        if !plain {
            c.cycles += 1;
            c.sw(type_slot.wrapping_sub(0x18), c.g[A3][0]);
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        abi::ret(c);
        return;
    }
}

/// `bool ScriptCamBinding_AcceptCommand(ScriptCamBinding* b, void* cmd, int n)`
/// (0x2831b0): true when `n < 4` and the instance script found for
/// `cmd->+8` has the same owner word (+0xc) as the binding's `+0x10 -> 0xf8`.
/// 6 (+9) + 1 or +4 cycles plus the callee.
pub fn script_cam_binding_accept_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = ((c.g[A2][0] as i64) < 4i64) as u64;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let too_many = c.g[A2][0] == 0;
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        if too_many {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
            c.g[A1][0] = c.lw(p(r32(c, A1), 8));
            callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT.call(c, 0x0028_31d4)?;
            c.g[V1][0] = c.lw(p(self_, 0x10));
            c.g[V0][0] = c.lw(p(r32(c, V0), 0xc));
            c.g[A0][0] = c.lw(p(r32(c, V1), 0xf8));
            c.g[V0][0] = c.g[V0][0] ^ c.g[A0][0];
            c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_SetOutputMode(SSXAudioSystem* sys, int mode)`
/// (0x2854a8): apply `mode` through the audio object at `sys+0x118`'s slot
/// 0x1d8 field, and when that reports 2 remember the mode at +0x62b0.
/// 14 + 6 cycles plus the callee.
pub fn ssx_audio_system_set_output_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let mode = c.g[A1][0];
        c.g[S0][0] = mode;
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        let obj = c.lw(p(self_, 0x118));
        c.g[V0][0] = obj;
        let vt = c.lw(obj as u32);
        c.g[V1][0] = vt;
        c.g[A0][0] = c.lw(p(vt as u32, 0x1d8));
        call_ptr(c, callee::AUDIO_SET_OUTPUT, 0x0028_54d0)?;
        let result = c.g[V0][0];
        c.g[V1][0] = result;
        c.g[V0][0] = 2;
        if c.g[V1][0] == c.g[V0][0] {
            c.sw(p(self_, 0x62b0), c.g[S0][0]);
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x20);
        c.g[V0][0] = c.g[V1][0];
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    /// Seeds per test (deep front-end callees run fewer).
    const SEEDS: u32 = 100;

    /// A real generated function that returns 0: a vtable slot for the
    /// "nothing handled" path.
    const ZERO_FN: u32 = 0x003c_51e0;

    /// `gp`-relative global address.
    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// Scrambled registers, the game's `gp`, the fake-call hooks and a zeroed
    /// BELibrary dummy.
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(gp_at(-0x1f04), DATA + 0x3_e000);
        r
    }

    /// A vtable at `at` whose entries are `{delta, fake}` for each slot offset.
    fn put_vtable(c: &mut Cpu, r: &mut Rng, at: u32, slots: &[u32]) -> u32 {
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(at + off, delta as u16);
            c.bus.write32(at + off + 4, FAKE + 0x10 * i as u32);
        }
        at
    }

    /// The main heap (0x4ff250) over 256 KiB of test RAM, so `EA_MemAllocNamed`
    /// has a real arena.
    fn put_arena(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = 0x004f_f250;
        c.g[A1][0] = (DATA + 0x6_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ssx3_ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, crate::difftest::RA as u32, "arena init did not return");
    }

    // ------------------------------------------------------------- emitters

    #[test]
    fn memitter_dtor_matches() {
        check(ee::s12::f_003714f8__generated, memitter_dtor, 60, &|c, seed| {
            base(c, seed, 0x3714);
            let flags = if seed % 5 == 0 { 1u32 } else { 0 };
            // The delete path frees `self`: use a small (pooled) address.
            let obj = if flags == 1 { 0x100 } else { DATA };
            c.bus.write32(obj + 0x204, 0);
            c.bus.write32(obj + 0x1a0, 0);
            c.bus.write32(obj + 0x1a4, 0);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(flags);
        });
    }

    #[test]
    fn memitter_free_data_matches() {
        check(ee::s12::f_003715b0__generated, memitter_free_data, 60, &|c, seed| {
            base(c, seed, 0x3715);
            let obj = DATA;
            let owned = if seed % 3 == 0 { 0 } else { 0x40 + 8 * (seed % 16) };
            c.bus.write32(obj + 0x204, owned);
            c.bus.write32(obj + 0x1a0, 0);
            c.bus.write32(obj + 0x1a4, 0);
            c.bus.write32(obj + 0x170, 0xffff_ffff);
            c.bus.write32(obj + 0x178, 0);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (seed / 2 % 2) as u64;
        });
    }

    // ------------------------------------------------------------ FX trail

    #[test]
    fn fxtrail_add_point_matches() {
        check(ee::s13::f_00392d90__generated, fxtrail_add_point, 80, &|c, seed| {
            let mut r = Rng(0x392d_7a11 ^ seed as u64);
            scramble(c, &mut r);
            let t = DATA;
            c.bus.write32(t + 0xc, r.next_u32() % 4);
            fill(c, &mut r, t + 0x10, 32);
            fill(c, &mut r, t + 0x50, 64);
            let rot = DATA + 0x200;
            let pos = DATA + 0x300;
            fill(c, &mut r, rot, 4);
            fill(c, &mut r, pos, 4);
            c.g[A0][0] = sx(t);
            c.g[A1][0] = sx(rot);
            c.g[A2][0] = sx(pos);
        });
    }

    // --------------------------------------------------------------- UI list

    #[test]
    fn uilistbox_set_mode_matches() {
        check(ee::s13::f_0039a8d8__generated, uilistbox_set_mode, 80, &|c, seed| {
            let mut r = base(c, seed, 0x39a8);
            let obj = DATA;
            let owner = DATA + 0x400;
            let target = DATA + 0x600;
            let vt = put_vtable(c, &mut r, DATA + 0x800, &[0x98]);
            c.bus.write32(obj + 0x5c, owner);
            c.bus.write16(obj + 0x319, 0xffff);
            c.bus.write32(obj + 0x74, r.next_u32());
            c.bus.write32(owner + 0xd0, target);
            c.bus.write32(target + 8, vt);
            fill(c, &mut r, target, 16);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(r.next_u32());
        });
    }

    // --------------------------------------------------------------- UI screen

    #[test]
    fn uiscreen_find_keyframe_matches() {
        check(ee::s13::f_0039c728__generated, uiscreen_find_keyframe, 80, &|c, seed| {
            let mut r = Rng(0x39c7_2811 ^ seed as u64);
            scramble(c, &mut r);
            let screen = DATA;
            let list = DATA + 0x100;
            let n = 1 + seed % 4;
            c.bus.write32(list, n);
            let mut at = list + 4;
            for i in 0..n {
                c.bus.write32(at + 4, 0x10);
                c.bus.write16(at + 8, (i * 7 % 5) as u16);
                fill(c, &mut r, at, 4);
                at += 0x10;
            }
            c.bus.write32(screen + 0x34, list);
            c.g[A0][0] = sx(screen);
            // 0..4 hits an entry, 5 never does.
            c.g[A1][0] = ((seed / 4) % 6) as u64;
        });
    }

    // ------------------------------------------------------------- AV player

    #[test]
    fn av_player_decode_frame_matches() {
        check(ee::s13::f_003ae6c8__generated, av_player_decode_frame, 60, &|c, seed| {
            let mut r = base(c, seed, 0x3ae6);
            let p = DATA;
            let stream = DATA + 0x200;
            c.bus.write32(p + 0x64, stream);
            c.bus.write32(p + 0x74, r.next_u32());
            c.bus.write32(p + 0x44, r.next_u32());
            c.bus.write32(stream + 0x1c, 1 + r.next_u32() % 4);
            fill(c, &mut r, stream, 0x40);
            c.g[A0][0] = sx(p);
            c.f[12] = r.edge_f32();
        });
    }

    // ----------------------------------------------------------- rider gate

    #[test]
    fn ridergate_sub_is_start_time_reached_matches() {
        check(ee::s00::f_0012c028__generated, ridergate_sub_is_start_time_reached, 80, &|c, seed| {
            let mut r = Rng(0x12c0_2844 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let (module, world, race) = (DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
            let gate = DATA + 0x4000;
            let rider = DATA + 0x5000;
            c.bus.write32(gp_at(-0x848), module);
            c.bus.write32(module + 0x84, world);
            c.bus.write32(world + 0xc, race);
            c.bus.write32(race, if seed % 3 == 0 { 5 } else { seed % 7 });
            c.bus.write32(race + 0xc, r.next_u32() % 100_000);
            c.bus.write32(gp_at(-0x7840), 0.016_666_668f32.to_bits());
            c.bus.write32(gate + 0x14, rider);
            c.bus.write32(rider + 0xb30, r.next_u32() % 2000);
            c.g[A0][0] = sx(gate);
        });
    }

    // ---------------------------------------------------------- BE profile

    #[test]
    fn be_char_profile_dtor_matches() {
        check(ee::s01::f_00151368__generated, be_char_profile_dtor, 60, &|c, seed| {
            base(c, seed, 0x1513);
            let flags = if seed % 4 == 0 { 1u32 } else { 0 };
            let obj = if flags == 1 { 0x100 } else { DATA };
            let owned = if seed % 3 == 0 { 0 } else { 0x40 + 8 * (seed % 8) };
            c.bus.write32(obj + 0x288, owned);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(flags);
        });
    }

    // ------------------------------------------------------ camera controller

    #[test]
    fn manual_cam_controller_select_algorithm_matches() {
        check(ee::s01::f_0015cf80__generated, manual_cam_controller_select_algorithm, 80, &|c, seed| {
            let mut r = base(c, seed, 0x15cf);
            let cam = DATA;
            c.bus.write32(cam + 0x18, r.next_u32() % 4);
            c.bus.write32(cam + 0x1c, r.next_u32() % 4);
            fill(c, &mut r, cam, 4);
            let msg = match seed % 3 {
                0 => 0x50u64,
                1 => 0x51,
                _ => r.next_u32() as u64,
            };
            c.g[A0][0] = sx(cam);
            c.g[A1][0] = msg;
        });
    }

    // ---------------------------------------------------------- lobby menu

    #[test]
    fn lobby_menu_leave_occupants_matches() {
        check(ee::s02::f_0017e368__generated, lobby_menu_leave_occupants, 60, &|c, seed| {
            let mut r = base(c, seed, 0x17e3);
            let menu = DATA;
            let n = if seed % 5 == 0 { 0 } else { 1 + seed % 3 };
            c.bus.write32(menu + 8, n);
            let vt = put_vtable(c, &mut r, DATA + 0x2000, &[0x30, 0x38]);
            for i in 0..3u32 {
                let item = DATA + 0x1000 + 0x200 * i;
                c.bus.write32(menu + 0xc + 4 * i, if i < n { item } else { 0 });
                c.bus.write32(item + 0x12c, vt);
            }
            c.g[A0][0] = sx(menu);
        });
    }

    // ---------------------------------------------------- front-end states

    #[test]
    fn cfe_state_cheat_char_select_apply_outfit_matches() {
        check(ee::s02::f_00182870__generated, cfe_state_cheat_char_select_apply_outfit, 80, &|c, seed| {
            let mut r = Rng(0x1828_7042 ^ seed as u64);
            scramble(c, &mut r);
            let s = DATA;
            let owner = DATA + 0x400;
            c.bus.write32(s + 0x6c, owner);
            c.bus.write8(owner + 0x98, (r.next_u32() % 4) as u8);
            for k in 0..6u32 {
                let item = if k % 3 == 0 || seed % 3 == 0 { 0 } else { DATA + 0x1000 + 0x100 * k };
                c.bus.write32(s + 0x54 + 4 * k, item);
            }
            fill(c, &mut r, s + 0xf4, 8);
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn cfe_state_request_line_on_activate_matches() {
        check(ee::s02::f_00196f00__generated, cfe_state_request_line_on_activate, 24, &|c, seed| {
            let _r = base(c, seed, 0x196f);
            let s = DATA;
            c.bus.write32(s + 0x150, 0xdead);
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn cfe_state_char_equip_shutdown_matches() {
        check(ee::s03::f_00199350__generated, cfe_state_char_equip_shutdown, 60, &|c, seed| {
            let _r = base(c, seed, 0x1993);
            let module = DATA + 0x1000;
            let manager = DATA + 0x2000;
            c.bus.write32(gp_at(-0x848), if seed % 3 == 0 { 0 } else { module });
            c.bus.write32(module + 0x7c, manager);
            let preview = DATA + 0x3000;
            c.bus.write32(manager + 0xb0, preview);
            c.bus.write8(DATA + 0x44, (seed % 4) as u8);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn cfe_state_char_equip_refresh_item_info_matches() {
        check(ee::s03::f_0019a9b8__generated, cfe_state_char_equip_refresh_item_info, 24, &|c, seed| {
            let _r = base(c, seed, 0x19a9);
            c.bus.write32(DATA + 0xc8, (seed % 3) as u32);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn bolt_file_index_manager_alloc_node_matches() {
        check(ee::s03::f_0019c880__generated, bolt_file_index_manager_alloc_node, 60, &|c, seed| {
            base(c, seed, 0x19c8);
            let m = DATA;
            let n = seed % 3;
            let head = if n == 0 { 0 } else { DATA + 0x1000 };
            c.bus.write32(m + 0x1f40, head);
            if n != 0 {
                // The next-node link at +0xe is either -1 (empty) or 0 (the
                // manager's own node, so a new head exists).
                c.bus.write16(head + 0xe, if seed % 2 == 0 { 0xbeef } else { 0 });
                c.bus.write16(head + 0xc, 0xdead);
                fill(c, &mut Rng(0x19c8_0000 ^ seed as u64), head, 3);
            }
            c.g[A0][0] = sx(m);
        });
    }

    #[test]
    fn feasync_manager_init_matches() {
        check(ee::s03::f_001a3648__generated, feasync_manager_init, 80, &|c, seed| {
            let mut r = Rng(0x1a36_48a1 ^ seed as u64);
            scramble(c, &mut r);
            c.bus.write8(0x004a_1408, (r.next_u32() % 5) as u8);
            fill(c, &mut r, DATA, 0x600 / 4);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn cfe_state_net_messenger_handle_input_matches() {
        check(ee::s03::f_001a4170__generated, cfe_state_net_messenger_handle_input, 80, &|c, seed| {
            let mut r = base(c, seed, 0x1a41);
            let s = DATA;
            let menu = DATA + 0x400;
            let vt = put_vtable(c, &mut r, DATA + 0x800, &[0xd8]);
            if seed % 3 == 0 {
                c.bus.write32(vt + 0xdc, ZERO_FN);
                c.bus.write16(vt + 0xd8, 0);
            }
            c.bus.write32(menu + 8, vt);
            c.bus.write32(s + 0x6f8, r.next_u32() % 8);
            fill(c, &mut r, menu, 16);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = sx(menu);
        });
    }

    #[test]
    fn cfe_use_existing_account_dtor_matches() {
        check(ee::s03::f_001ac068__generated, cfe_use_existing_account_dtor, 24, &|c, seed| {
            let _r = base(c, seed, 0x1ac0);
            let flags = if seed % 4 == 0 { 1u32 } else { 0 };
            let obj = if flags == 1 { 0x100 } else { DATA };
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(flags);
        });
    }

    #[test]
    fn cfe_state_create_step1_set_account_name_text_matches() {
        check(ee::s03::f_001b95c0__generated, cfe_state_create_step1_set_account_name_text, 60, &|c, seed| {
            base(c, seed, 0x1b95);
            let s = DATA;
            let screen = DATA + 0x400;
            let name = DATA + 0x1000;
            c.bus.write32(s + 0x40, screen);
            c.bus.write32(s + 0x6d0, name);
            c.bus.write32(name, name + 0x10);
            for k in 0..8u32 {
                c.bus.write8(name + 0x10 + k, b'a' + ((seed + k) % 26) as u8);
            }
            c.bus.write8(name + 0x18, 0);
            fill(c, &mut Rng(0x1b95_0000 ^ seed as u64), screen, 4);
            c.bus.write32(s + 0x40, screen);
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn cfe_state_stats_v31_matches() {
        check(ee::s03::f_001c4cd8__generated, cfe_state_stats_v31, 24, &|c, seed| {
            let _r = base(c, seed, 0x1c4c);
            c.bus.write32(DATA + 0x6dc, DATA + 0x1000);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 2 == 0 { 0xe1u64 } else { (seed % 5) as u64 };
        });
    }

    // ------------------------------------------------------ keyboard popup

    /// Enable exactly one key of the popup steppers: the popup's own flags
    /// (+0x68/+0x6c) stay 0 and `index` is enabled in both key tables.
    fn enable_key(c: &mut Cpu, index: i32) {
        let off = (index as u32).wrapping_mul(4);
        c.bus.write32(DATA.wrapping_add(0x148).wrapping_add(off), 1);
        c.bus.write32(DATA.wrapping_add(0x298).wrapping_add(off), 1);
    }

    #[test]
    fn ckeyboard_popup_find_enabled_in_column_from_top_matches() {
        check(ee::s04::f_001cc090__generated, ckeyboard_popup_find_enabled_in_column_from_top, 80, &|c, seed| {
            let mut r = base(c, seed, 0x1cc0);
            let cur = (r.next_u32() % 40) as i32;
            let found = cur - 0x38 + 0xe * (seed % 3) as i32;
            c.bus.write32(DATA + 0x140, cur as u32);
            enable_key(c, found);
            c.g[A0][0] = sx(DATA);
            c.pc = 0x001c_c090;
        });
    }

    #[test]
    fn ckeyboard_popup_step_up_to_enabled_key_matches() {
        check(ee::s04::f_001cc140__generated, ckeyboard_popup_step_up_to_enabled_key, 80, &|c, seed| {
            let mut r = base(c, seed, 0x1cc1);
            let cur = 0x50 + (r.next_u32() % 40) as i32;
            let found = cur - 0xe - 0xe * (seed % 3) as i32;
            c.bus.write32(DATA + 0x140, cur as u32);
            enable_key(c, found);
            c.g[A0][0] = sx(DATA);
            c.pc = 0x001c_c140;
        });
    }

    #[test]
    fn ckeyboard_popup_step_down_to_enabled_key_matches() {
        check(ee::s04::f_001cc190__generated, ckeyboard_popup_step_down_to_enabled_key, 80, &|c, seed| {
            let mut r = base(c, seed, 0x1cc2);
            let cur = (r.next_u32() % 40) as i32;
            let found = cur + 0xe + 0xe * (seed % 3) as i32;
            c.bus.write32(DATA + 0x140, cur as u32);
            enable_key(c, found);
            c.g[A0][0] = sx(DATA);
            c.pc = 0x001c_c190;
        });
    }

    // -------------------------------------------------------- video player

    #[test]
    fn cfe_state_video_player_set_movie_matches() {
        check(ee::s04::f_001d2598__generated, cfe_state_video_player_set_movie, 80, &|c, seed| {
            let mut r = base(c, seed, 0x1d25);
            let s = DATA;
            let name = DATA + 0x200;
            let movie = DATA + 0x300;
            for (i, base_at) in [name, movie].into_iter().enumerate() {
                let n = (r.next_u32() % 16) as usize;
                for k in 0..n {
                    c.bus.write8(base_at + k as u32, b'a' + ((seed + i as u32 + k as u32) % 26) as u8);
                }
                c.bus.write8(base_at + n as u32, 0);
            }
            c.g[A0][0] = sx(s);
            c.g[A1][0] = sx(name);
            c.g[A2][0] = if seed % 4 == 0 { 0 } else { sx(movie) };
        });
    }

    #[test]
    fn cfe_state_video_player_start_playback_matches() {
        check(ee::s04::f_001d25e8__generated, cfe_state_video_player_start_playback, 24, &|c, seed| {
            let _r = base(c, seed, 0x1d26);
            c.bus.write32(gp_at(0x410), DATA + 0x1000);
            c.bus.write32(DATA + 0x248, DATA + 0x2000);
            c.bus.write32(DATA + 0x254, 0xdead);
            c.bus.write32(DATA + 0x274, 0xdead);
            c.bus.write32(DATA + 0x278, 0xdead);
            c.g[A0][0] = sx(DATA);
        });
    }

    // ------------------------------------------------------- message center

    #[test]
    fn message_center_set_flag_matches() {
        check(ee::s04::f_001e3a78__generated, message_center_set_flag, 24, &|c, seed| {
            let mut r = base(c, seed, 0x1e3a);
            c.bus.write32(gp_at(-0x1f04), DATA + 0x3_e000);
            c.bus.write32(DATA + 0x3_e000, 0);
            c.g[A0][0] = sx(r.next_u32());
            c.g[A1][0] = sx(r.next_u32());
        });
    }

    // ------------------------------------------------------ overlay screens

    #[test]
    fn ov_screen_finish_ov_on_widget_action_matches() {
        check(ee::s04::f_001e8160__generated, ov_screen_finish_ov_on_widget_action, 80, &|c, seed| {
            let mut r = base(c, seed, 0x1e81);
            let s = DATA;
            let world = DATA + 0x1000;
            let module = DATA + 0x2000;
            c.bus.write32(gp_at(-0x848), if seed % 5 == 0 { 0 } else { module });
            c.bus.write32(module + 0x84, world);
            c.bus.write32(world + 0x48, DATA + 0x3000);
            c.bus.write32(s + 0xa0, if seed % 3 == 0 { 0 } else { 1 });
            c.g[A0][0] = sx(s);
            c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x4000) };
            c.g[A2][0] = match seed % 4 {
                0 => 0,
                1 => 5,
                _ => (seed % 9) as u64,
            };
        });
    }

    #[test]
    fn ov_screen_finish_ov2_on_widget_action_matches() {
        check(ee::s04::f_001e8880__generated, ov_screen_finish_ov2_on_widget_action, 80, &|c, seed| {
            let mut r = base(c, seed, 0x1e88);
            let s = DATA;
            let world = DATA + 0x1000;
            let module = DATA + 0x2000;
            c.bus.write32(gp_at(-0x848), if seed % 5 == 0 { 0 } else { module });
            c.bus.write32(module + 0x84, world);
            c.bus.write32(world + 0x48, DATA + 0x3000);
            c.bus.write32(s + 0xa0, if seed % 3 == 0 { 0 } else { 1 });
            c.g[A0][0] = sx(s);
            c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x4000) };
            c.g[A2][0] = match seed % 4 {
                0 => 0,
                1 => 5,
                _ => (seed % 9) as u64,
            };
        });
    }

    #[test]
    fn cfe_state_career_stats_set_text_matches() {
        check(ee::s04::f_001f6840__generated, cfe_state_career_stats_set_text, 60, &|c, seed| {
            let mut r = base(c, seed, 0x1f68);
            let s = DATA;
            let screen = DATA + 0x400;
            let name = DATA + 0x1000;
            let n = (r.next_u32() % 12) as usize;
            for k in 0..n {
                c.bus.write8(name + k as u32, b'A' + ((seed as usize + k) % 26) as u8);
            }
            c.bus.write8(name + n as u32, 0);
            fill(c, &mut Rng(0x1f68_0000 ^ seed as u64), screen, 4);
            c.bus.write32(s + 0x40, screen);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = sx(name);
            c.g[A2][0] = sx(DATA + 0x2000);
        });
    }

    #[test]
    fn ov_screen_top_times_v26_matches() {
        check(ee::s04::f_001f8118__generated, ov_screen_top_times_v26, 80, &|c, seed| {
            let mut r = base(c, seed, 0x1f81);
            let s = DATA;
            let widget = DATA + 0x400;
            c.bus.write32(s + 0xa0, if seed % 3 == 0 { 0 } else { 1 });
            c.bus.write32(widget + 0x18, match seed % 4 {
                0 => 0,
                _ => 2 + r.next_u32() % 6,
            });
            c.bus.write32(gp_at(-0x9f4), 0);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = sx(widget);
        });
    }

    // -------------------------------------------------------------- crowd

    #[test]
    fn crowd_man2d_add_matches() {
        check(ee::s05::f_00229788__generated, crowd_man2d_add, 80, &|c, seed| {
            let mut r = base(c, seed, 0x2297);
            let m = DATA;
            for i in 0..64u32 {
                let v = if i == seed % 64 { r.next_u32() & 0xffff_fffe } else { 0xffff_ffff };
                c.bus.write32(m + 8 + 4 * i, v);
            }
            c.g[A0][0] = sx(m);
            c.g[A1][0] = sx(r.next_u32() % 8);
            c.pc = 0x0022_9788;
        });
    }

    // -------------------------------------------------------- game states

    #[test]
    fn game_state_pre_race_menu_exit_matches() {
        check(ee::s06::f_00236c88__generated, game_state_pre_race_menu_exit, 60, &|c, seed| {
            let mut r = base(c, seed, 0x236c);
            let depth = 1 + r.next_u32() % 4;
            c.bus.write32(gp_at(-0x69c), depth);
            for i in 0..4u32 {
                c.bus.write32(0x0053_66e8 + 4 * i, (i * 3 + seed) % 16);
            }
            for i in 0..16u32 {
                c.bus.write32(0x0044_28f0 + 4 * i, r.next_u32());
            }
            c.bus.write32(gp_at(-0x6a0), 0xdead);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn mc_manager_write_save_by_name_w_matches() {
        check(ee::s06::f_00241000__generated, mc_manager_write_save_by_name_w, 60, &|c, seed| {
            let mut r = base(c, seed, 0x2410);
            let s = DATA;
            let card = DATA + 0x1000;
            let vt = put_vtable(c, &mut r, DATA + 0x2000, &[0x128]);
            c.bus.write32(s + 0x434, card);
            c.bus.write32(card, vt);
            c.bus.write32(s + 0xe4, 0xdead);
            fill(c, &mut r, card + 4, 16);
            c.g[A0][0] = sx(s);
            c.g[A1][0] = sx(r.next_u32());
            c.g[A2][0] = sx(r.next_u32());
            c.g[A3][0] = sx(r.next_u32());
        });
    }

    // ------------------------------------------------------------- lobby

    #[test]
    fn lobby_man_destroy_matches() {
        check(ee::s06::f_0025b688__generated, lobby_man_destroy, 24, &|c, seed| {
            let _r = base(c, seed, 0x25b6);
            let lobby = if seed % 3 == 0 { 0 } else { DATA + 0x1000 };
            c.bus.write32(gp_at(-0xc8), lobby);
            c.bus.write32(gp_at(-0x204), if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn csetup_man_cancel_character_matches() {
        check(ee::s07::f_00265140__generated, csetup_man_cancel_character, 24, &|c, seed| {
            let mut r = base(c, seed, 0x2651);
            let s = DATA;
            let state = match seed % 4 {
                0 => 0xc,
                1 => 4,
                _ => r.next_u32() % 16,
            };
            c.bus.write32(s, state);
            c.bus.write32(s + 0x124, if seed % 3 == 0 { 0 } else { 1 });
            c.g[A0][0] = sx(s);
            c.g[A1][0] = sx(r.next_u32() % 8);
        });
    }

    #[test]
    fn cshared_login_man_request_card_info_if_idle_matches() {
        check(ee::s07::f_002659e8__generated, cshared_login_man_request_card_info_if_idle, 80, &|c, seed| {
            let mut r = base(c, seed, 0x2659);
            let obj = DATA + 0x1000;
            let vt = put_vtable(c, &mut r, DATA + 0x2000, &[0x28, 0x58]);
            if seed % 2 == 0 {
                c.bus.write16(vt + 0x58, 0);
                c.bus.write32(vt + 0x5c, FAKE);
            } else {
                c.bus.write16(vt + 0x58, 0);
                c.bus.write32(vt + 0x5c, ZERO_FN);
            }
            c.bus.write32(vt + 0x28, 0);
            c.bus.write32(vt + 0x2c, FAKE + 0x20);
            c.bus.write32(obj, vt);
            c.bus.write32(gp_at(0x2e8), obj);
            c.bus.write32(gp_at(0x2ec), r.next_u32());
            fill(c, &mut r, obj + 4, 0x100);
            c.g[A0][0] = sx(DATA);
        });
    }

    // -------------------------------------------------------------- network

    #[test]
    fn net_event_clone_matches() {
        check(ee::s07::f_00267e48__generated, net_event_clone, 40, &|c, seed| {
            let mut r = base(c, seed, 0x267e);
            put_arena(c);
            c.bus.write32(DATA, r.next_u32());
            fill(c, &mut r, DATA + 4, 16);
            c.g[A0][0] = sx(DATA);
        });
    }

    // --------------------------------------------------------------- replay

    #[test]
    fn replay_stream_peek_int_matches() {
        check(ee::s07::f_0026ea68__generated, replay_stream_peek_int, 60, &|c, seed| {
            let mut r = base(c, seed, 0x26ea);
            let s = DATA;
            let vt = put_vtable(c, &mut r, DATA + 0x2000, &[0x10]);
            c.bus.write32(s, vt);
            c.bus.write32(s + 0xc, r.next_u32());
            fill(c, &mut r, DATA + 0x40, 16);
            c.g[A0][0] = sx(s);
            c.g[A2][0] = 4;
        });
    }

    #[test]
    fn replay_clear_all_highlights_matches() {
        check(ee::s07::f_00270e18__generated, replay_clear_all_highlights, 60, &|c, seed| {
            let mut r = base(c, seed, 0x270e);
            let s = DATA;
            c.bus.write32(s, 1 + r.next_u32() % 3);
            for i in 0..2u32 {
                c.bus.write32(s + 0x494 + 0xb4 * i, r.next_u32());
                c.bus.write32(s + 0x494 + 0xb4 * i + 4, r.next_u32());
            }
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn inseq_span_track_dtor_matches() {
        check(ee::s07::f_00272da0__generated, inseq_span_track_dtor, 60, &|c, seed| {
            base(c, seed, 0x272d);
            let flags = if seed % 3 == 0 { 1u32 } else { 0 };
            // The delete path frees `self`: use a small (pooled) address.
            let obj = if flags == 1 { 0x100 } else { DATA };
            c.bus.write32(gp_at(0x398), 0);
            // No live track data: the stop routine takes its early exit.
            c.bus.write32(obj + 0xc, 0);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(flags);
        });
    }

    #[test]
    fn inseq_event_track_dtor_matches() {
        check(ee::s07::f_002731b8__generated, inseq_event_track_dtor, 60, &|c, seed| {
            base(c, seed, 0x2731);
            let flags = if seed % 3 == 0 { 1u32 } else { 0 };
            let obj = if flags == 1 { 0x100 } else { DATA };
            c.bus.write32(gp_at(0x398), 0);
            c.bus.write32(obj + 0xc, 0);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(flags);
        });
    }

    #[test]
    fn inseq_ramp_track_dtor_matches() {
        check(ee::s07::f_002734e0__generated, inseq_ramp_track_dtor, 60, &|c, seed| {
            base(c, seed, 0x2734);
            let flags = if seed % 3 == 0 { 1u32 } else { 0 };
            let obj = if flags == 1 { 0x100 } else { DATA };
            c.bus.write32(gp_at(0x398), 0);
            c.bus.write32(obj + 0xc, 0);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(flags);
        });
    }

    // ----------------------------------------------------------- script list

    #[test]
    fn script_list_on_seq_start_matches() {
        check(ee::s07::f_002772c0__generated, script_list_on_seq_start, 40, &|c, seed| {
            let mut r = base(c, seed, 0x2772);
            let list = DATA;
            let obj = DATA + 0x1000;
            c.bus.write32(list + 0xa4, if seed % 3 == 0 { 0 } else { 3 });
            c.bus.write32(obj + 0x14, DATA + 0x2000);
            c.bus.write8(DATA + 0x2000, if seed % 4 == 0 { 0 } else { b'x' });
            c.bus.write32(DATA + 0x2004, r.next_u32());
            c.bus.write8(0x004a_34a8, 0);
            c.g[A0][0] = sx(list);
            c.g[A1][0] = sx(DATA + 0x3000);
            c.g[A2][0] = sx(obj);
        });
    }

    // --------------------------------------------------------- script engine

    #[test]
    fn script_engine_unload_master_matches() {
        check(ee::s07::f_00278308__generated, script_engine_unload_master, 60, &|c, seed| {
            let mut r = base(c, seed, 0x2783);
            let e = DATA;
            // The database destroy is a no-op without a live database.
            c.bus.write32(e + 0x2b0, 0);
            fill(c, &mut r, e + 0x400, 0x100);
            c.bus.write32(e + 0x52c, if seed % 3 == 0 { 0 } else { 0x40 });
            c.bus.write32(e + 0x534, if seed % 2 == 0 { 0 } else { 0x80 });
            c.g[A0][0] = sx(e);
        });
    }

    #[test]
    fn script_engine_obj_table_dtor_matches() {
        check(ee::s07::f_0027ce58__generated, script_engine_obj_table_dtor, 60, &|c, seed| {
            base(c, seed, 0x27ce);
            let flags = if seed % 3 == 0 { 1u32 } else { 0 };
            // The delete path frees `self`: use a small (pooled) address.
            let obj = if flags == 1 { 0x100 } else { DATA };
            c.bus.write32(gp_at(0x3ec), 1);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(flags);
        });
    }

    #[test]
    fn script_movie_dtor_matches() {
        check(ee::s07::f_002837c8__generated, script_movie_dtor, 60, &|c, seed| {
            base(c, seed, 0x2837);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = 0;
        });
    }

    #[test]
    fn script_engine_reset_process_table_matches() {
        check(ee::s07::f_002828b8__generated, script_engine_reset_process_table, 80, &|c, seed| {
            let mut r = Rng(0x2828_b811 ^ seed as u64);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x300);
            c.g[A0][0] = sx(DATA);
        });
    }

    #[test]
    fn script_engine_alloc_process_matches() {
        check(ee::s07::f_00282c38__generated, script_engine_alloc_process, 80, &|c, seed| {
            let mut r = Rng(0x282c_3811 ^ seed as u64);
            scramble(c, &mut r);
            let e = DATA;
            fill(c, &mut r, e + 0x2c0, 16 * 0x24 / 4);
            // Leave a few slots occupied so the search walks the table; make
            // slot 0 occupied for some seeds so the walk exhausts.
            let used = seed % 6;
            for i in 0..used {
                c.bus.write32(e + 0x2e0 + 0x24 * i, 1 + i);
            }
            c.bus.write32(e + 0x300, 0);
            c.g[A0][0] = sx(e);
            c.g[A1][0] = match seed % 3 {
                0 => 1u64,
                1 => 0,
                _ => (seed % 5) as u64,
            };
        });
    }

    #[test]
    fn script_cam_binding_accept_command_matches() {
        check(ee::s07::f_002831b0__generated, script_cam_binding_accept_command, 80, &|c, seed| {
            let mut r = base(c, seed, 0x2831);
            let b = DATA;
            let cmd = DATA + 0x400;
            c.bus.write32(b + 0x10, DATA + 0x800);
            c.bus.write32(DATA + 0x800 + 0xf8, r.next_u32());
            c.bus.write32(cmd + 8, r.next_u32());
            c.bus.write32(gp_at(-0x84c), DATA + 0x1000);
            fill(c, &mut r, DATA + 0x1000, 0x40);
            c.g[A0][0] = sx(b);
            c.g[A1][0] = sx(cmd);
            c.g[A2][0] = (seed % 6) as u64;
        });
    }

    #[test]
    fn ssx_audio_system_set_output_mode_matches() {
        check(ee::s07::f_002854a8__generated, ssx_audio_system_set_output_mode, 80, &|c, seed| {
            let mut r = base(c, seed, 0x2854);
            let sys = DATA;
            let obj = DATA + 0x1000;
            c.bus.write32(sys + 0x118, obj);
            c.bus.write32(obj, DATA + 0x2000);
            c.bus.write32(DATA + 0x2000 + 0x1d8, DATA + 0x3000);
            fill(c, &mut r, DATA + 0x3000, 0x20);
            c.bus.write32(sys + 0x62b0, 0xdead);
            c.g[A0][0] = sx(sys);
            c.g[A1][0] = sx(r.next_u32() % 8);
        });
    }
}
