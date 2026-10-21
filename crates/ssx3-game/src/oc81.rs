//! Phase 2 wave 20 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for fifty small game functions, each with a differential test
//! against the generated body.
//!
//! Groups:
//! - **Front end**: child-event dispatch and transitions of the online
//!   character-select, chat-room, net-account, quick-messages, stats and
//!   keyboard-popup states; popup selection clamping; the reward-gallery help
//!   text; the trophy room and career stats teardown/setup; the Lodge rider
//!   detail callback.
//! - **Message center / memcards**: unread count, the memcard state reset and
//!   the PS2 memcard formatting/list-dir/save-name steps.
//! - **Sections / buddies / net events**: section dequeue, the card manager
//!   state, buddy roster lookup and disconnect, and the net-event destructors.
//! - **Replay / script engine**: resume snapshots, aux-list stopping, camera
//!   add, and the script database constructor.
//! - **Audio**: output-mode mapping, bank teardown, ambience/loop stopping,
//!   track-change query, crowd sample choice, stream destroy, resume-all and
//!   the track-volume query.
//! - **Menu / world**: page selection validation, end-draw, row measuring,
//!   avalanche baking, snow boxes, WScript local tables and pool stats.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001b_1ec0,
        c_fe_state_online_character_select_on_child_event,
        "cFEStateOnlineCharacterSelect_OnChildEvent",
    ),
    (
        0x001b_2320,
        c_fe_state_chat_room_update,
        "cFEStateChatRoom_Update",
    ),
    (
        0x001b_7030,
        c_fe_state_net_account_mem_dtor,
        "cFEStateNetAccountMem_dtor",
    ),
    (
        0x001b_bd08,
        c_fe_state_create_duplicate_ctor,
        "cFEStateCreateDuplicate_ctor",
    ),
    (
        0x001c_23c0,
        c_fe_state_net_quick_messages_transition_in,
        "cFEStateNetQuickMessages_TransitionIn",
    ),
    (
        0x001c_32b8,
        c_fe_state_stats_transition_in,
        "cFEStateStats_TransitionIn",
    ),
    (
        0x001c_75c8,
        c_fe_popup_confirm_select_option,
        "cFEPopupConfirm_SelectOption",
    ),
    (
        0x001c_d2f0,
        c_keyboard_popup_on_activate,
        "cKeyboardPopup_OnActivate",
    ),
    (
        0x001c_e4c8,
        c_keyboard_popup_apply_font_scale,
        "cKeyboardPopup_ApplyFontScale",
    ),
    (
        0x001d_05f0,
        c_fe_state_reward_gallery_is_item_owned,
        "cFEStateRewardGallery_IsItemOwned",
    ),
    (
        0x001d_8be0,
        mem_card_state_base_reset_state,
        "MemCardStateBase_ResetState",
    ),
    (
        0x001e_0210,
        c_fe_state_trophy_room_dtor,
        "cFEStateTrophyRoom_dtor",
    ),
    (
        0x001e_3408,
        message_center_get_unread_count,
        "MessageCenter_GetUnreadCount",
    ),
    (
        0x001f_3ff8,
        c_fe_state_lodge_rider_detail_ctor,
        "cFEStateLodgeRiderDetail_ctor",
    ),
    (
        0x001f_4380,
        c_fe_state_lodge_rider_detail_on_render,
        "cFEStateLodgeRiderDetail_OnRender",
    ),
    (
        0x001f_54b0,
        c_fe_state_career_stats_ctor,
        "cFEStateCareerStats_ctor",
    ),
    (
        0x0022_e360,
        section_man_dequeue_section,
        "SectionMan_DequeueSection",
    ),
    (0x0023_f0f8, mc_manager_state_0c, "MCManager_State0C"),
    (0x0026_1408, buddy_man_disconnect, "BuddyMan_Disconnect"),
    (
        0x0026_25c8,
        buddy_man_get_roster_entry,
        "BuddyMan_GetRosterEntry",
    ),
    (
        0x0026_5cb0,
        c_shared_login_man_is_card_unformatted,
        "cSharedLoginMan_IsCardUnformatted",
    ),
    (
        0x0026_5fb8,
        c_shared_login_man_mc_list_dir,
        "cSharedLoginMan_McListDir",
    ),
    (0x0026_8158, net_event_e2_dtor, "NetEventE2_dtor"),
    (0x0026_8398, net_event102_down_dtor, "NetEvent102Down_dtor"),
    (0x0026_8588, net_event_e5_dtor, "NetEventE5_dtor"),
    (
        0x0026_f7f8,
        replay_save_resume_snapshot,
        "Replay_SaveResumeSnapshot",
    ),
    (
        0x0027_9488,
        script_engine_stop_aux_lists,
        "ScriptEngine_StopAuxLists",
    ),
    (0x0028_1190, script_view_obj_add_cam, "ScriptViewObj_AddCam"),
    (
        0x0028_1e80,
        script_engine_script_db_ctor,
        "ScriptEngine_ScriptDbCtor",
    ),
    (
        0x0028_9d08,
        ssx_audio_system_option_to_output_mode,
        "SSXAudioSystem_OptionToOutputMode",
    ),
    (0x0028_b278, snd_bank_dtor, "SndBank_dtor"),
    (
        0x0029_13d8,
        ssx_audio_system_stop_ambience_loop,
        "SSXAudioSystem_StopAmbienceLoop",
    ),
    (
        0x0029_d678,
        ssx_audio_system_stop_loop_sfx,
        "SSXAudioSystem_StopLoopSfx",
    ),
    (
        0x002a_4718,
        ssx_audio_system_has_track_changed,
        "SSXAudioSystem_HasTrackChanged",
    ),
    (
        0x002a_4ca0,
        crowd_audio_pick_random_sample,
        "CrowdAudio_PickRandomSample",
    ),
    (
        0x002a_abd0,
        stream_array_destroy_stream,
        "StreamArray_DestroyStream",
    ),
    (
        0x002a_d300,
        bank_monitor_resume_all,
        "BankMonitor_ResumeAll",
    ),
    (
        0x002a_f428,
        path_plat_get_track_volume,
        "PATH_PlatGetTrackVolume",
    ),
    (
        0x002c_2210,
        mem_card_ascii_to_sjis_string,
        "MemCard_AsciiToSjisString",
    ),
    (
        0x002c_4388,
        c_ps2_mem_card_do_format,
        "cPS2MemCard_DoFormat",
    ),
    (0x002c_4520, c_ps2_mem_card_v34, "cPS2MemCard_v34"),
    (0x002c_4578, c_ps2_mem_card_v35, "cPS2MemCard_v35"),
    (0x002c_4800, c_ps2_mem_card_v29, "cPS2MemCard_v29"),
    (
        0x002c_aab0,
        menu_sys_page_validate_selection,
        "MenuSysPage_ValidateSelection",
    ),
    (0x002c_b2f8, menu_sys_page_end_draw, "MenuSysPage_EndDraw"),
    (
        0x002c_d090,
        menu_sys_action_item_measure,
        "MenuSysActionItem_Measure",
    ),
    // `Avalanche_BakeAll` (0x2d96e0) is ported below but has no differential
    // test/override: its unconditional file export/reload enters EAFile's
    // request queue, which cannot be bounded on a test machine.
    (0x002e_62c8, snow_fall_reset_boxes, "SnowFall_ResetBoxes"),
    (
        0x0030_8a48,
        w_script_man_free_local_tables,
        "WScriptMan_FreeLocalTables",
    ),
    (0x0031_a200, mem_report_pool_stats, "MEM_ReportPoolStats"),
];

// ---------------------------------------------------------------------------
// Callees (same-shard direct calls; cross-shard ones stay `call_ptr`s).

mod callee {
    use super::{ee, Callee};

    // s03
    pub const ON_QUIT_POPUP_EVENT: Callee = Callee {
        addr: 0x001b_1f18,
        f: ee::s03::f_001b1f18,
    };
    pub const EMPTY_CHILD_HANDLER: Callee = Callee {
        addr: 0x001a_94d8,
        f: ee::s03::f_001a94d8,
    };
    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::f_001a97b8,
    };
    pub const NET_FE_STATE_UPDATE: Callee = Callee {
        addr: 0x001a_8770,
        f: ee::s03::f_001a8770,
    };
    pub const NET_FE_STATE_CTOR: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    };
    pub const NET_FE_STATE_DTOR: Callee = Callee {
        addr: 0x001a_85d0,
        f: ee::s03::f_001a85d0,
    };
    pub const CHAT_ROOM_UPDATE_PLAYER_QUALITY: Callee = Callee {
        addr: 0x001b_3bf0,
        f: ee::s03::f_001b3bf0,
    };
    pub const STATS_SELECT_CATEGORY: Callee = Callee {
        addr: 0x001c_3ee8,
        f: ee::s03::f_001c3ee8,
    };

    // s04
    pub const KEYBOARD_POPUP_UPDATE_CURSOR: Callee = Callee {
        addr: 0x001c_cc30,
        f: ee::s04::f_001ccc30,
    };
    pub const MEM_CARD_SET_USE_SAVE_TITLE: Callee = Callee {
        addr: 0x001d_9308,
        f: ee::s04::f_001d9308,
    };
    pub const TROPHY_ROOM_RELEASE_IMAGES: Callee = Callee {
        addr: 0x001d_4b20,
        f: ee::s04::f_001d4b20,
    };
    pub const MESSAGE_CENTER_GET_INBOX: Callee = Callee {
        addr: 0x001e_39f8,
        f: ee::s04::f_001e39f8,
    };

    // s05
    pub const SECTION_MAN_PUSH_FREE_NODE: Callee = Callee {
        addr: 0x0022_e288,
        f: ee::s05::f_0022e288,
    };

    // s06
    pub const MC_MANAGER_END_OPERATION: Callee = Callee {
        addr: 0x0023_ea30,
        f: ee::s06::f_0023ea30,
    };
    pub const MC_MANAGER_REQUEST_CARD_INFO: Callee = Callee {
        addr: 0x0023_fb20,
        f: ee::s06::f_0023fb20,
    };
    pub const BUDDY_MAN_SET_PRESENCE_ADDR: u32 = 0x0026_2768;
    pub const BUDDY_MAN_DESTROY_API: Callee = Callee {
        addr: 0x0026_1530,
        f: ee::s06::f_00261530,
    };

    // s07
    pub const BUDDY_MAN_GET_ROSTER_COUNT: Callee = Callee {
        addr: 0x0026_2598,
        f: ee::s07::f_00262598,
    };
    pub const SCRIPT_LIST_STOP: Callee = Callee {
        addr: 0x0027_6868,
        f: ee::s07::f_00276868,
    };
    pub const SCRIPT_OBJ_CAM_SET_OWNER: Callee = Callee {
        addr: 0x0027_d680,
        f: ee::s07::f_0027d680,
    };
    pub const SCRIPT_ENGINE_OBJ_TABLE_CTOR: Callee = Callee {
        addr: 0x0027_cdd0,
        f: ee::s07::f_0027cdd0,
    };
    pub const SCRIPT_ENGINE_RESET_PROCESS_TABLE: Callee = Callee {
        addr: 0x0028_28b8,
        f: ee::s07::f_002828b8,
    };
    pub const SND_BANK_UNLOAD: Callee = Callee {
        addr: 0x0028_b528,
        f: ee::s07::f_0028b528,
    };
    pub const SND_BANK_UNLINK: Callee = Callee {
        addr: 0x0028_b730,
        f: ee::s07::f_0028b730,
    };
    pub const SND_BANK_FREE_MEMORY: Callee = Callee {
        addr: 0x0028_b650,
        f: ee::s07::f_0028b650,
    };
    pub const REPLAY_RECORD_SNAPSHOT: Callee = Callee {
        addr: 0x0026_d760,
        f: ee::s07::f_0026d760,
    };
    pub const REPLAY_ADD_SNAPSHOT: Callee = Callee {
        addr: 0x0026_d740,
        f: ee::s07::f_0026d740,
    };

    // s08
    pub const BANK_MONITOR_STOP_HANDLE: Callee = Callee {
        addr: 0x002a_d5f0,
        f: ee::s08::f_002ad5f0,
    };
    pub const STREAM_BUFFER_DTOR: Callee = Callee {
        addr: 0x002a_9df0,
        f: ee::s08::f_002a9df0,
    };
    pub const BANK_MONITOR_INSTANCE_RESUME: Callee = Callee {
        addr: 0x002a_cb30,
        f: ee::s08::f_002acb30,
    };
    pub const PATH_PLAT_REFRESH_STATUS: Callee = Callee {
        addr: 0x002a_efa8,
        f: ee::s08::f_002aefa8,
    };
    pub const MEM_CARD_ASCII_CHAR_TO_SJIS: Callee = Callee {
        addr: 0x002c_2338,
        f: ee::s08::f_002c2338,
    };
    pub const C_PS2_MEM_CARD_SET_STATE: Callee = Callee {
        addr: 0x002c_48c0,
        f: ee::s08::f_002c48c0,
    };
    pub const USTR_TO_ASCII: Callee = Callee {
        addr: 0x002c_2580,
        f: ee::s08::f_002c2580,
    };
    pub const C_PS2_MEM_CARD_GET_SAVE_NAME: Callee = Callee {
        addr: 0x002c_54d0,
        f: ee::s08::f_002c54d0,
    };
    pub const C_PS2_MEM_CARD_REQUEST_READ_SAVE: Callee = Callee {
        addr: 0x002c_4788,
        f: ee::s08::f_002c4788,
    };

    // s09
    pub const MENU_SYS_PAGE_SELECT_NEXT: Callee = Callee {
        addr: 0x002c_ab08,
        f: ee::s09::f_002cab08,
    };
    pub const MENU_SYS_ITEM_MEASURE_ROW: Callee = Callee {
        addr: 0x002c_a4c8,
        f: ee::s09::f_002ca4c8,
    };
    pub const AVALANCHE_FREE_HISTORIES: Callee = Callee {
        addr: 0x002d_9a80,
        f: ee::s09::f_002d9a80,
    };
    pub const AVALANCHE_BAKE_GROUP: Callee = Callee {
        addr: 0x002d_9660,
        f: ee::s09::f_002d9660,
    };
    pub const AVALANCHE_EXPORT_HISTORY_FILES: Callee = Callee {
        addr: 0x002d_8258,
        f: ee::s09::f_002d8258,
    };
    pub const AVALANCHE_LOAD_HISTORY_FILE: Callee = Callee {
        addr: 0x002d_87d0,
        f: ee::s09::f_002d87d0,
    };
    pub const SNOW_BOX_RESET: Callee = Callee {
        addr: 0x002e_4f10,
        f: ee::s09::f_002e4f10,
    };

    // s10
    pub const MEM_REPORT_POOL_STAT0: Callee = Callee {
        addr: 0x0032_0518,
        f: ee::s10::f_00320518,
    };
    pub const MEM_REPORT_POOL_STAT1: Callee = Callee {
        addr: 0x0032_0520,
        f: ee::s10::f_00320520,
    };
    pub const MEM_REPORT_POOL_STAT2: Callee = Callee {
        addr: 0x0032_0528,
        f: ee::s10::f_00320528,
    };
    pub const MEM_REPORT_POOL_STAT3: Callee = Callee {
        addr: 0x0032_0530,
        f: ee::s10::f_00320530,
    };
    pub const MEM_REPORT_POOL_STAT4: Callee = Callee {
        addr: 0x0032_0538,
        f: ee::s10::f_00320538,
    };
    pub const MEM_REPORT_POOL_STAT5: Callee = Callee {
        addr: 0x0032_0540,
        f: ee::s10::f_00320540,
    };
    pub const MEM_REPORT_POOL_STAT6: Callee = Callee {
        addr: 0x0032_0548,
        f: ee::s10::f_00320548,
    };

    // Fixed cross-shard `call_indirect` targets.
    pub const FE_MODULE_GET_PLAYER_PAD_MASK: u32 = 0x001a_1cd0;
    pub const UIMENU_SET_SELECTION: u32 = 0x0039_b960;
    pub const UI_STATE_ON_ACTIVATE: u32 = 0x0039_e4c0;
    pub const UI_TEXT_GET_FONT: u32 = 0x003a_04f0;
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const BE_REWARD_INT_IS_UNLOCKED: u32 = 0x0015_7468;
    pub const UI_STATE_CTOR: u32 = 0x0039_e2a0;
    pub const UI_STATE_DTOR: u32 = 0x0039_e390;
    pub const EA_MEMSET: u32 = 0x003e_6448;
    pub const C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL: u32 = 0x002c_6810;
    pub const C_BX_STRING_DTOR: u32 = 0x0031_8260;
    pub const BUILTIN_DELETE: u32 = 0x0031_7e50;
    pub const LOBBY_LIST_GET_AT: u32 = 0x003e_e560;
    pub const SLOGIN_MC_GET_PORT_SLOT: u32 = 0x003f_a1d0;
    pub const SCE_MC_FORMAT: u32 = 0x0040_a998;
    pub const STRCPY: u32 = 0x0041_66f4;
    pub const SPRINTF: u32 = 0x0041_7828;
    pub const RAND_NEXT_MAIN: u32 = 0x0031_77f0;
    pub const C_SETUP_MAN_SEND_QUIT: u32 = 0x0026_5768;
    pub const UI_OBJECT_SHOW_CLAMP_ALPHA_HALF: u32 = 0x0019_4498;
    pub const C_LUNO_TABLE_DTOR: u32 = 0x0022_4df0;
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lw` (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

// ---------------------------------------------------------------------------
// Front end.

/// `void cFEStateOnlineCharacterSelect_OnChildEvent(self, child, event)`
/// (0x1b1ec0): dispatch the character-select screen's child events: 0x838 is
/// the quit popup, 0x835 empties the child, anything else forwards.
pub fn c_fe_state_online_character_select_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x835;
        frame.save_ra(c, 0);
        let child = r32(c, A1);
        c.g[V1][0] = Addr(p(child, 0x18)).lw(c);
        let is_empty = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x838;
        if is_empty {
            c.cycles += 4;
            callee::EMPTY_CHILD_HANDLER.call(c, 0x001b_1ef8)?;
        } else {
            c.cycles += 2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 4;
                callee::ON_QUIT_POPUP_EVENT.call(c, 0x001b_1ee8)?;
            } else {
                c.cycles += 3;
                callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_1f08)?;
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateChatRoom_Update(self)` (0x1b2320): every 10000th tick update
/// the player quality, then run the net state's update.
pub fn c_fe_state_chat_room_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0x2710;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = lw(c, gp(c, 0x2a74));
        c.cycles += 5;
        c.g[V0][0] = Addr(r32(c, A1)).off(0x18).lw(c);
        c.div(0, V0, V1);
        c.g[A0][0] = c.hi[0];
        if c.g[A0][0] == 0 {
            c.cycles += 2;
            callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY.call(c, 0x001b_235c)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::NET_FE_STATE_UPDATE.call(c, 0x001b_2364)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetAccountMem_dtor(self, flags)` (0x1b7030): destroy the
/// shared login manager, install the net-state base vtable and run its dtor.
pub fn c_fe_state_net_account_mem_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7830));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x001b_7058);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0026_58b8, 0x001b_7058)?;
        c.g[V0][0] = 0x47_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4908));
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001b_7070);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        callee::NET_FE_STATE_DTOR.call(c, 0x001b_7070)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateCreateDuplicate_ctor(self, a1, a2)` (0x1bbd08): run the net
/// state ctor, install the create-duplicate class and store the arguments.
pub fn c_fe_state_create_duplicate_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001b_bd28);
        c.g[A2][0] = 0;
        callee::NET_FE_STATE_CTOR.call(c, 0x001b_bd28)?;
        c.g[V1][0] = 0x47_0000;
        c.g[A0][0] = 0x4f;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7b60));
        Addr(p(r32(c, S0), 0x6d0)).sw(c, c.g[S1][0]);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[A0][0]);
        Addr(p(r32(c, S0), 0x6a8)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetQuickMessages_TransitionIn(self, arg)` (0x1c23c0): call
/// the state's transition hook, then refresh all messages when requested.
pub fn c_fe_state_net_quick_messages_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S1)).off(8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x100).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x104).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x001c_23ec);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x001c_23ec)?;
        let no_refresh = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !no_refresh {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001c_23fc);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x001c_2418, 0x001c_23fc)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 5;
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateStats_TransitionIn(self, arg)` (0x1c32b8): call the state's
/// transition hook, then select the first category when requested.
pub fn c_fe_state_stats_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S1)).off(8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x100).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x104).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x001c_32e4);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x001c_32e4)?;
        c.g[A0][0] = c.g[S1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x001c_32f4);
            c.g[A1][0] = 0;
            callee::STATS_SELECT_CATEGORY.call(c, 0x001c_32f4)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupConfirm_SelectOption(menu, option, idx, max)` (0x1c75c8):
/// clamp `idx + 1` into `[lo, max]` (with `max == -1` meaning `[0, 1]`) and
/// select it in the menu.
pub fn c_fe_popup_confirm_select_option(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = u64::MAX;
        let has_max = c.g[A3][0] != c.g[V0][0];
        let idx = sx(r32(c, A2).wrapping_add(1));
        let lo;
        let mut hi = c.g[A3][0];
        if has_max {
            c.cycles += 1;
            c.g[V1][0] = 1;
            lo = 1u64;
        } else {
            c.cycles += 3;
            c.g[V1][0] = 0;
            c.g[A3][0] = 1;
            lo = 0;
            hi = 1;
        }
        c.cycles += 3;
        c.g[V0][0] = ((idx as i64) < (lo as i64)) as u64;
        let below = c.g[V0][0] != 0;
        c.g[V0][0] = ((hi as i64) < (idx as i64)) as u64;
        c.g[A1][0] = sx(idx as u32);
        if below {
            c.cycles += 2;
            c.g[A1][0] = sx(lo as u32);
        } else {
            c.cycles += 1;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = hi;
            }
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x001c_7610);
        c.g[A1][0] = c.g[A1][0] & 0xff;
        call_ptr(c, callee::UIMENU_SET_SELECTION, 0x001c_7610)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cKeyboardPopup_OnActivate(self, arg)` (0x1cd2f0): step the cursor
/// counter down at its limit, update the cursor, then activate.
pub fn c_keyboard_popup_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S0)).off(0x134).lw(c);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x60).lw(c);
        let at_limit = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if at_limit {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, V1).wrapping_sub(1));
            Addr(p(r32(c, S0), 0x134)).sw(c, c.g[V0][0]);
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x001c_d324);
        c.g[A0][0] = c.g[S0][0];
        callee::KEYBOARD_POPUP_UPDATE_CURSOR.call(c, 0x001c_d324)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001c_d330);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, callee::UI_STATE_ON_ACTIVATE, 0x001c_d330)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cKeyboardPopup_ApplyFontScale(obj, scale)` (0x1ce4c8): save the
/// 12-byte scale vector at `+0x50`, scale the font's x/y size into its
/// `+0x38`/`+0x3c` size.
pub fn c_keyboard_popup_apply_font_scale(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        let obj = r32(c, A1);
        c.ldl(2, p(obj, 0x57));
        c.ldr(2, p(obj, 0x50));
        c.g[V1][0] = Addr(p(obj, 0x58)).lw(c);
        let sp = c.g[SP][0] as u32;
        c.sdl(2, p(sp, 7));
        c.sdr(2, sp);
        c.sw(p(sp, 8), c.g[V1][0]);
        c.g[RA][0] = sx(0x001c_e4f0);
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, callee::UI_TEXT_GET_FONT, 0x001c_e4f0)?;
        let font = r32(c, V0);
        c.lwc1(2, c.g[SP][0] as u32);
        c.lwc1(3, p(c.g[SP][0] as u32, 4));
        c.lwc1(0, Addr(font).off(0x30).0);
        c.lwc1(1, Addr(font).off(0x34).0);
        c.mul_s(0, 0, 2);
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        c.mul_s(1, 1, 3);
        c.swc1(0, Addr(font).off(0x38).0);
        c.swc1(1, Addr(font).off(0x3c).0);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(p(c.g[SP][0] as u32, 0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateRewardGallery_IsItemOwned(self, arg)` (0x1d05f0): ask the
/// reward interface whether the selected reward is unlocked.
pub fn c_fe_state_reward_gallery_is_item_owned(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001d_060c);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x001d_060c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d_0618);
        c.g[A1][0] = 0xd;
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x001d_0618)?;
        c.g[A3][0] = Addr(r32(c, S0)).off(0x4c).lw(c);
        c.g[T0][0] = c.g[S1][0];
        c.g[A1][0] = Addr(r32(c, S0)).off(0x44).lb(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d_0630);
        c.g[A2][0] = Addr(r32(c, S0)).off(0x50).lw(c);
        call_ptr(c, callee::BE_REWARD_INT_IS_UNLOCKED, 0x001d_0630)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_ResetState(self)` (0x1d8be0): clear the save-title
/// and dialog fields and set the default use-save-title mode.
pub fn mem_card_state_base_reset_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        Addr(p(this, 0x1a8)).sw(c, 0);
        Addr(p(this, 0x1f0)).sw(c, 0);
        Addr(p(this, 0x1ec)).sw(c, 0);
        Addr(p(this, 0x1b0)).sw(c, 0);
        c.g[RA][0] = sx(0x001d_8c10);
        Addr(p(this, 0x1e4)).sw(c, c.g[V0][0]);
        callee::MEM_CARD_SET_USE_SAVE_TITLE.call(c, 0x001d_8c10)?;
        c.g[V0][0] = 2;
        Addr(p(this, 0x1dc)).sw(c, 0);
        Addr(p(this, 0x1c0)).sw(c, c.g[V0][0]);
        Addr(p(this, 0x1e0)).sw(c, 0);
        Addr(p(this, 0x1e8)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateTrophyRoom_dtor(self, flags)` (0x1e0210): release the trophy
/// images, install the rewards-room vtable and run the base dtor.
pub fn c_fe_state_trophy_room_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6948));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x001e_0238);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        callee::TROPHY_ROOM_RELEASE_IMAGES.call(c, 0x001e_0238)?;
        c.g[V0][0] = 0x47_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001e_0250);
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        call_ptr(c, callee::UI_STATE_DTOR, 0x001e_0250)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MessageCenter_GetUnreadCount(void)` (0x1e3408): count the inbox bits
/// that are not set.
pub fn message_center_get_unread_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MESSAGE_CENTER_GET_INBOX.call(c, 0x001e_3418)?;
        let inbox = r32(c, V0);
        c.g[A3][0] = Addr(inbox).off(0xcc).lbu(c);
        c.g[A2][0] = 0;
        c.g[A1][0] = 0;
        if c.g[A3][0] != 0 {
            c.cycles += 2;
            c.g[T0][0] = Addr(inbox).off(0xc8).lw(c);
            c.g[T1][0] = 1;
            loop {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[T1][0] as u32) << ((c.g[A2][0] as u32) & 31));
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V0][0] = c.g[T0][0] & c.g[V0][0];
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.g[A0][0];
                }
                if c.g[V1][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001e_3430);
                }
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[A1][0];
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(p(c.g[SP][0] as u32, 0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void* cFEStateLodgeRiderDetail_ctor(self)` (0x1f3ff8): run the UI state
/// ctor, install the class and copy the pad mask byte.
pub fn c_fe_state_lodge_rider_detail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x001f_400c);
        call_ptr(c, callee::UI_STATE_CTOR, 0x001f_400c)?;
        c.g[V0][0] = 0x28;
        c.g[V1][0] = 0x47_0000;
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x39d8));
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 0;
        Addr(p(r32(c, S0), 0x44)).sb(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x10).lw(c);
        c.g[RA][0] = sx(0x001f_4034);
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        call_ptr(c, callee::FE_MODULE_GET_PLAYER_PAD_MASK, 0x001f_4034)?;
        Addr(p(r32(c, S0), 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateLodgeRiderDetail_OnRender(self)` (0x1f4380): when the detail
/// screen is not open, find its widget by name and reveal it.
pub fn c_fe_state_lodge_rider_detail_on_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x48).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = 0x47_0000;
            c.g[RA][0] = sx(0x001f_43a8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1090));
            call_ptr(c, 0x0031_7670, 0x001f_43a8)?;
            c.g[A0][0] = Addr(r32(c, S0)).off(0x40).lw(c);
            c.g[RA][0] = sx(0x001f_43b4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_43b4)?;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001f_43c4);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF, 0x001f_43c4)?;
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateCareerStats_ctor(self)` (0x1f54b0): run the UI state ctor,
/// install the career stats class and copy the pad mask byte.
pub fn c_fe_state_career_stats_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x001f_54c4);
        call_ptr(c, callee::UI_STATE_CTOR, 0x001f_54c4)?;
        c.g[V0][0] = 0x2a;
        c.g[V1][0] = 0x47_0000;
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3838));
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 0;
        Addr(p(r32(c, S0), 0x44)).sb(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x10).lw(c);
        c.g[RA][0] = sx(0x001f_54ec);
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        call_ptr(c, callee::FE_MODULE_GET_PLAYER_PAD_MASK, 0x001f_54ec)?;
        Addr(p(r32(c, S0), 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionMan_DequeueSection(self, node)` (0x22e360): unlink a section
/// node from its list or record it as the new head, push it free and count it.
pub fn section_man_dequeue_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if c.g[V1][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A1)).off(4).lw(c);
            Addr(p(r32(c, S0), 0x19c)).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, A1)).off(4).lw(c);
            Addr(p(r32(c, V1), 4)).sw(c, c.g[V0][0]);
        }
        c.cycles += 9;
        c.g[RA][0] = sx(0x0022_e398);
        c.g[A0][0] = c.g[S0][0];
        callee::SECTION_MAN_PUSH_FREE_NODE.call(c, 0x0022_e398)?;
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).off(0x1a0).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        Addr(p(this, 0x1a0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State0C(self)` (0x23f0f8): when the operation has finished,
/// end it, request the card info and move to state 1.
pub fn mc_manager_state_0c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x6c).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0023_f114);
        call_ptr(c, target, 0x0023_f114)?;
        if c.g[V0][0] != 0 {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0023_f128);
            c.g[A1][0] = 0;
            callee::MC_MANAGER_END_OPERATION.call(c, 0x0023_f128)?;
            c.g[A1][0] = Addr(r32(c, S0)).off(0x428).lw(c);
            c.g[RA][0] = sx(0x0023_f134);
            c.g[A0][0] = c.g[S0][0];
            callee::MC_MANAGER_REQUEST_CARD_INFO.call(c, 0x0023_f134)?;
            c.g[V0][0] = 1;
            Addr(p(r32(c, S0), 0x338)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_Disconnect(self)` (0x261408): clear the presence state and
/// destroy the API connections and queues.
pub fn buddy_man_disconnect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 1;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[RA][0] = sx(0x0026_1430);
        c.g[T1][0] = 0;
        call_ptr(c, callee::BUDDY_MAN_SET_PRESENCE_ADDR, 0x0026_1430)?;
        c.g[RA][0] = sx(0x0026_1438);
        c.g[A0][0] = c.g[S0][0];
        callee::BUDDY_MAN_DESTROY_API.call(c, 0x0026_1438)?;
        let this = r32(c, S0);
        Addr(p(this, 0xe8)).sw(c, 0);
        Addr(p(this, 0xd8)).sw(c, 0);
        Addr(p(this, 0xd0)).sw(c, 0);
        Addr(p(this, 0xdc)).sw(c, 0);
        Addr(p(this, 0xe0)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* BuddyMan_GetRosterEntry(self, idx)` (0x2625c8): 0 for an invalid
/// index, otherwise the buddy at that roster position.
pub fn buddy_man_get_roster_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let negative = (c.g[S0][0] as i64) < 0;
        c.g[S1][0] = c.g[A0][0];
        if negative {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0026_25ec);
            callee::BUDDY_MAN_GET_ROSTER_COUNT.call(c, 0x0026_25ec)?;
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = Addr(r32(c, S1)).off(0xcc).lw(c);
                c.cycles += 2;
                c.g[RA][0] = sx(0x0026_2608);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, callee::LOBBY_LIST_GET_AT, 0x0026_2608)?;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cSharedLoginMan_IsCardUnformatted(void)` (0x265cb0): true when the
/// global card object is formatted but not write-protected.
pub fn c_shared_login_man_is_card_unformatted(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = lw(c, gp(c, 0x2e8));
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw(r32(c, A1));
        c.g[A0][0] = Addr(r32(c, V1)).off(0x1a8).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x1ac).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0026_5cd0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0026_5cd0)?;
        let formatted = c.g[V0][0] != 0;
        c.g[A1][0] = lw(c, gp(c, 0x2e8));
        if !formatted {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[V1][0] = c.lw(r32(c, A1));
            c.g[A0][0] = Addr(r32(c, V1)).off(0x1b8).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x1bc).lw(c);
            let target = r32(c, V0);
            c.g[RA][0] = sx(0x0026_5cf4);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0026_5cf4)?;
            c.g[V0][0] = c.g[V0][0] ^ 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_McListDir(code, arg)` (0x265fb8): query the port
/// slot, clear the error buffer and start listing the directory.
pub fn c_shared_login_man_mc_list_dir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0026_5fd4);
        c.g[A1][0] = sx(c.g[SP][0] as u32);
        call_ptr(c, callee::SLOGIN_MC_GET_PORT_SLOT, 0x0026_5fd4)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A2][0] = 0x40;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7190));
        c.g[RA][0] = sx(0x0026_5fe8);
        c.g[A1][0] = 0;
        call_ptr(c, callee::EA_MEMSET, 0x0026_5fe8)?;
        c.g[A0][0] = lw(c, gp(c, 0x2e8));
        c.g[RA][0] = sx(0x0026_5ff4);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL, 0x0026_5ff4)?;
        c.g[V0][0] = 2;
        frame.restore(c, S0, 0x20);
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        Addr(gp(c, 0x2e4)).sw(c, c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(p(c.g[SP][0] as u32, 0x30));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Net-event destructors, replay and script engine.

/// The body shared by the three `NetEvent*_dtor`s: destroy the string at
/// `string_off` and free `self` when bit 0 of `flags` is set.
fn net_event_dtor_body(c: &mut Cpu, string_off: u32, ret_string: u32, ret_free: u32) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(string_off));
    c.g[RA][0] = sx(ret_string);
    c.g[A1][0] = 2;
    call_ptr(c, callee::C_BX_STRING_DTOR, ret_string)?;
    c.g[V0][0] = 0x48_0000;
    c.g[S0][0] = c.g[S0][0] & 1;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    let free = c.g[S0][0] == 0;
    Addr(p(r32(c, S1), 4)).sw(c, c.g[V0][0]);
    if !free {
        c.cycles += 2;
        c.g[RA][0] = sx(ret_free);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::BUILTIN_DELETE, ret_free)?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void NetEventE2_dtor(self, flags)` (0x268158).
pub fn net_event_e2_dtor(c: &mut Cpu) {
    guest(c, |c| net_event_dtor_body(c, 8, 0x0026_817c, 0x0026_8198))
}

/// `void NetEvent102Down_dtor(self, flags)` (0x268398).
pub fn net_event102_down_dtor(c: &mut Cpu) {
    guest(c, |c| net_event_dtor_body(c, 0xc, 0x0026_83bc, 0x0026_83d8))
}

/// `void NetEventE5_dtor(self, flags)` (0x268588).
pub fn net_event_e5_dtor(c: &mut Cpu) {
    guest(c, |c| net_event_dtor_body(c, 8, 0x0026_85ac, 0x0026_85c8))
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
            c.sb(p(c.g[SP][0] as u32, 2), 0);
            c.g[RA][0] = sx(0x0026_f82c);
            c.sb(p(c.g[SP][0] as u32, 3), 0);
            callee::REPLAY_RECORD_SNAPSHOT.call(c, 0x0026_f82c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0026_f83c);
            Addr(p(r32(c, S0), 0x3d0)).sw(c, c.g[V0][0]);
            callee::REPLAY_ADD_SNAPSHOT.call(c, 0x0026_f83c)?;
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_StopAuxLists(self)` (0x279488): stop both auxiliary
/// script lists of the rider.
pub fn script_engine_stop_aux_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 1;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = Addr(r32(c, S2)).off(0x624).lw(c);
            c.g[A1][0] = 1;
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
            c.g[RA][0] = sx(0x0027_94bc);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
            callee::SCRIPT_LIST_STOP.call(c, 0x0027_94bc)?;
            let more = (c.g[S0][0] as i64) >= 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xcc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_94a8);
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

/// `int ScriptViewObj_AddCam(self, cam)` (0x281190): put the camera in the
/// first free slot and make the view own it; 0 when both slots are taken.
pub fn script_view_obj_add_cam(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[A2][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[V1][0] = sx(r32(c, A2).wrapping_add(0xd0));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, V1));
            let occupied = c.g[V0][0] != 0;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            if !occupied {
                c.cycles += 5;
                Addr(r32(c, V1)).sw(c, c.g[A0][0]);
                c.g[RA][0] = sx(0x0028_11c0);
                c.g[A1][0] = c.g[A2][0];
                callee::SCRIPT_OBJ_CAM_SET_OWNER.call(c, 0x0028_11c0)?;
                c.g[V0][0] = 1;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_11a8);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* ScriptEngine_ScriptDbCtor(self)` (0x281e80): build the object table,
/// install the script-db vtables, register it globally and reset the process
/// table.
pub fn script_engine_script_db_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x0028_1e94);
        callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR.call(c, 0x0028_1e94)?;
        c.g[V1][0] = 0x48_0000;
        c.g[V0][0] = 0x48_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2450));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x24b8));
        let this = r32(c, S0);
        Addr(p(this, 0x2ac)).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.g[S0][0];
        Addr(p(this, 0x2a8)).sw(c, c.g[V0][0]);
        Addr(gp(c, 0x3f8)).sw(c, c.g[S0][0]);
        Addr(p(this, 0x2b0)).sw(c, 0);
        c.g[RA][0] = sx(0x0028_1ec0);
        Addr(p(this, 0x2b4)).sw(c, 0);
        callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE.call(c, 0x0028_1ec0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// `int SSXAudioSystem_OptionToOutputMode(option)` (0x289d08): map the audio
/// option byte to its output mode.
pub fn ssx_audio_system_option_to_output_mode(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = c.g[A1][0] & 0xff;
    c.g[V1][0] = 1;
    let is1 = c.g[A1][0] == c.g[V1][0];
    c.g[V0][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
    let mut to_289d50 = true;
    if is1 {
        c.cycles += 2;
        c.g[V1][0] = 2;
    } else {
        c.cycles += 2;
        let small = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0];
        if small {
            to_289d50 = false; // break b_289d54 with v0 = 1
        } else {
            c.cycles += 3;
            c.g[V0][0] = 2;
            let is2 = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if is2 {
                c.cycles += 2;
                c.g[V1][0] = 0;
            } else {
                c.cycles += 2;
                let is3 = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = c.g[V1][0];
                if is3 {
                    c.cycles += 1;
                    c.g[V1][0] = 3;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    to_289d50 = false;
                }
            }
        }
    }
    if to_289d50 {
        c.cycles += 1;
        c.g[V0][0] = c.g[V1][0];
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SndBank_dtor(self, flags)` (0x28b278): unload, unlink and free the
/// bank's memory, then free the bank when asked.
pub fn snd_bank_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0028_b294);
        callee::SND_BANK_UNLOAD.call(c, 0x0028_b294)?;
        c.g[RA][0] = sx(0x0028_b29c);
        c.g[A0][0] = c.g[S1][0];
        callee::SND_BANK_UNLINK.call(c, 0x0028_b29c)?;
        c.g[RA][0] = sx(0x0028_b2a4);
        c.g[A0][0] = c.g[S1][0];
        callee::SND_BANK_FREE_MEMORY.call(c, 0x0028_b2a4)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let free = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !free {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0028_b2b8);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0028_b2b8)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopAmbienceLoop(self)` (0x2913d8): stop the ambience
/// handle through the sound-system interface and clear the loop fields.
pub fn ssx_audio_system_stop_ambience_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x4a_0000;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.ldl(5, p(r32(c, V0), 0x3687));
        c.ldr(5, p(r32(c, V0), 0x3680));
        c.g[V1][0] = Addr(r32(c, S0)).off(0x118).lw(c);
        c.g[A0][0] = c.lw(r32(c, V1));
        c.g[V0][0] = Addr(r32(c, A0)).off(0xab0).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
        c.g[V1][0] = Addr(r32(c, V0)).off(0x18).lh(c);
        c.g[A2][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
        let target = r32(c, A2);
        c.g[RA][0] = sx(0x0029_1414);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        call_ptr(c, target, 0x0029_1414)?;
        Addr(p(r32(c, S0), 0x5834)).sw(c, 0);
        Addr(p(r32(c, S0), 0x5830)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopLoopSfx(self)` (0x29d678): stop the current loop
/// SFX handle through the bank monitor and clear the slot.
pub fn ssx_audio_system_stop_loop_sfx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x5fc4).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = Addr(r32(c, S0)).off(0x5fc8).lw(c);
            let negative = (c.g[A1][0] as i64) < 0;
            c.g[A2][0] = 1;
            if !negative {
                c.cycles += 6;
                c.g[V0][0] = Addr(r32(c, S0)).off(0x118).lw(c);
                c.g[A0][0] = c.lw(r32(c, V0));
                c.g[RA][0] = sx(0x0029_d6b0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
                callee::BANK_MONITOR_STOP_HANDLE.call(c, 0x0029_d6b0)?;
                c.g[V0][0] = u64::MAX;
                Addr(p(r32(c, S0), 0x5fc8)).sw(c, c.g[V0][0]);
            }
            c.cycles += 2;
            Addr(p(r32(c, S0), 0x5fc4)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_HasTrackChanged(self)` (0x2a4718): whether the current
/// track id differs from the system's track (and is not the "none" id 0x17).
pub fn ssx_audio_system_has_track_changed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x002a_472c);
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x002a_472c)?;
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x002a_4738);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x002a_4738)?;
        c.g[RA][0] = sx(0x002a_4740);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x002a_4740)?;
        c.g[A0][0] = Addr(r32(c, S0)).off(0x5814).lw(c);
        c.g[V1][0] = c.lw(r32(c, V0));
        let same = c.g[V1][0] == c.g[A0][0];
        c.g[V0][0] = 0x17;
        if same {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[V0][0] = 1;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* CrowdAudio_PickRandomSample(list)` (0x2a4ca0): a random entry of
/// the sample list, or 0 when the list is empty.
pub fn crowd_audio_pick_random_sample(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(r32(c, S0));
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[RA][0] = sx(0x002a_4cc4);
            call_ptr(c, callee::RAND_NEXT_MAIN, 0x002a_4cc4)?;
            c.g[V1][0] = c.lw(r32(c, S0));
            c.divu(0, V0, V1);
            c.cycles += 4;
            c.g[A0][0] = c.hi[0];
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            c.g[V0][0] = Addr(r32(c, A0)).off(4).lw(c);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamArray_DestroyStream(self, i)` (0x2aabd0): destroy and clear
/// entry `i` of the stream array.
pub fn stream_array_destroy_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V0));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002a_ac04);
            c.g[A1][0] = 3;
            callee::STREAM_BUFFER_DTOR.call(c, 0x002a_ac04)?;
            c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
        } else {
            c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, V0)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_ResumeAll(self)` (0x2ad300): resume all 64 bank monitor
/// instances and clear the pending count.
pub fn bank_monitor_resume_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = 0x3f;
        frame.save(c, S1, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S2)).off(0x8ec).lw(c);
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
            c.g[RA][0] = sx(0x002a_d330);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
            callee::BANK_MONITOR_INSTANCE_RESUME.call(c, 0x002a_d330)?;
            let more = (c.g[S0][0] as i64) >= 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xc0));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_d320);
            }
        }
        c.cycles += 7;
        Addr(p(r32(c, S2), 0x8e8)).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatGetTrackVolume(idx)` (0x2af428): the track handle's volume,
/// or -8 when the slot is empty.
pub fn path_plat_get_track_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x4d_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V0][0] = c.lw(r32(c, S0));
        if c.g[V0][0] == 0 {
            c.g[V0][0] = u64::MAX.wrapping_sub(7);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, V0)).off(0x65).lb(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a_f464);
                c.g[A0][0] = 0;
                callee::PATH_PLAT_REFRESH_STATUS.call(c, 0x002a_f464)?;
                c.g[V1][0] = c.lw(r32(c, S0));
            } else {
                c.g[V1][0] = c.lw(r32(c, S0));
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memcards, menus and world.

/// `void MemCard_AsciiToSjisString(src, dst)` (0x2c2210): convert a
/// NUL-terminated ASCII string to Shift-JIS.
pub fn mem_card_ascii_to_sjis_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            loop {
                c.cycles += 7;
                c.g[A0][0] = Addr(r32(c, S0)).lbu(c);
                c.g[RA][0] = sx(0x002c_223c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                callee::MEM_CARD_ASCII_CHAR_TO_SJIS.call(c, 0x002c_223c)?;
                Addr(r32(c, S1)).sh(c, c.g[V0][0]);
                c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_2230);
                }
            }
        }
        c.cycles += 6;
        Addr(r32(c, S1)).sh(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoFormat(self)` (0x2c4388): format the card and go to
/// the done/failed state.
pub fn c_ps2_mem_card_do_format(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x002c_43a4);
        c.g[A0][0] = Addr(r32(c, S0)).off(0xc).lw(c);
        call_ptr(c, callee::SCE_MC_FORMAT, 0x002c_43a4)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        Addr(gp(c, 0x850)).sw(c, c.g[V0][0]);
        if ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002c_43cc);
            c.g[A1][0] = 0x12;
            callee::C_PS2_MEM_CARD_SET_STATE.call(c, 0x002c_43cc)?;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002c_43b8);
            c.g[A1][0] = 1;
            callee::C_PS2_MEM_CARD_SET_STATE.call(c, 0x002c_43b8)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_v34(self, name)` (0x2c4520): copy the save name, format
/// the full path and go to state 0x14.
pub fn c_ps2_mem_card_v34(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(0xb9));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002c_4540);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::STRCPY, 0x002c_4540)?;
        c.g[A1][0] = 0x4a_0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0xf9));
        c.g[RA][0] = sx(0x002c_4554);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3950));
        call_ptr(c, callee::SPRINTF, 0x002c_4554)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002c_4560);
        c.g[A1][0] = 0x14;
        callee::C_PS2_MEM_CARD_SET_STATE.call(c, 0x002c_4560)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_v35(self, name)` (0x2c4578): convert the wide save name
/// to ASCII, format the full path and go to state 0x14.
pub fn c_ps2_mem_card_v35(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(0xb9));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002c_4598);
        c.g[A0][0] = c.g[S0][0];
        callee::USTR_TO_ASCII.call(c, 0x002c_4598)?;
        c.g[A1][0] = 0x4a_0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0xf9));
        c.g[RA][0] = sx(0x002c_45ac);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3950));
        call_ptr(c, callee::SPRINTF, 0x002c_45ac)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002c_45b8);
        c.g[A1][0] = 0x14;
        callee::C_PS2_MEM_CARD_SET_STATE.call(c, 0x002c_45b8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_v29(self, a1, a2, a3)` (0x2c4800): build the save name
/// and request a read of the save.
pub fn c_ps2_mem_card_v29(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A3][0];
        c.g[RA][0] = sx(0x002c_4828);
        c.g[A2][0] = sx(c.g[SP][0] as u32);
        callee::C_PS2_MEM_CARD_GET_SAVE_NAME.call(c, 0x002c_4828)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x002c_483c);
        c.g[A1][0] = sx(c.g[SP][0] as u32);
        callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE.call(c, 0x002c_483c)?;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_ValidateSelection(self)` (0x2caab0): when the selected
/// item is not selectable, move to the next one.
pub fn menu_sys_page_validate_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        c.g[A1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).off(0x10).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x18).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x002c_aae4);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002c_aae4)?;
        if c.g[V0][0] != 0 {
            // no-op: return
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002c_aaf4);
            c.g[A0][0] = c.g[S0][0];
            callee::MENU_SYS_PAGE_SELECT_NEXT.call(c, 0x002c_aaf4)?;
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MenuSysPage_EndDraw(void)` (0x2cb2f8): end the current menu's draw,
/// end its draw list and undo the 0x14 draw offset.
pub fn menu_sys_page_end_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw(c, gp(c, -0x854));
        c.g[V0][0] = Addr(r32(c, A1)).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x100).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x104).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x002c_b318);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002c_b318)?;
        c.g[A1][0] = lw(c, gp(c, -0x854));
        c.g[V0][0] = Addr(r32(c, A1)).off(0x10d8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0xb0).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0xb4).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x002c_b330);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x002c_b330)?;
        c.g[A0][0] = lw(c, gp(c, -0x854));
        c.g[V0][0] = Addr(r32(c, A0)).off(0xe84).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
        Addr(p(r32(c, A0), 0xe84)).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* MenuSysActionItem_Measure(self, item)` (0x2cd090): measure an
/// action item's row with its icon or its text width.
pub fn menu_sys_action_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[T0][0] = Addr(r32(c, A1)).off(0x18).lw(c);
        let has_icon = c.g[T0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if has_icon {
            c.cycles += 5;
            c.g[A2][0] = Addr(r32(c, A1)).off(0x14).lw(c);
            c.g[RA][0] = sx(0x002c_d0b4);
            c.g[A3][0] = 0;
            callee::MENU_SYS_ITEM_MEASURE_ROW.call(c, 0x002c_d0b4)?;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 6;
            c.g[A3][0] = Addr(r32(c, A1)).off(0x14).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0;
            c.g[RA][0] = sx(0x002c_d0d0);
            c.g[T0][0] = 0;
            callee::MENU_SYS_ITEM_MEASURE_ROW.call(c, 0x002c_d0d0)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Avalanche_BakeAll(void)` (0x2d96e0): free the histories, bake every
/// group, then export and reload the history files.
///
/// Not installed: the unconditional `Avalanche_ExportHistoryFiles` /
/// `Avalanche_LoadHistoryFile` enter the EAFile request queue, which cannot be
/// bounded in a test fixture (no request slots / no disc backing), so the body
/// has no differential test.
pub fn avalanche_bake_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        callee::AVALANCHE_FREE_HISTORIES.call(c, 0x002d_96f4)?;
        c.g[S0][0] = lw(c, gp(c, 0x9c0));
        if c.g[S0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[RA][0] = sx(0x002d_9708);
                c.g[A0][0] = c.g[S0][0];
                callee::AVALANCHE_BAKE_GROUP.call(c, 0x002d_9708)?;
                c.g[S0][0] = Addr(r32(c, S0)).off(8).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9700);
                }
            }
        }
        c.cycles += 8;
        c.g[RA][0] = sx(0x002d_971c);
        callee::AVALANCHE_EXPORT_HISTORY_FILES.call(c, 0x002d_971c)?;
        c.g[RA][0] = sx(0x002d_9724);
        callee::AVALANCHE_LOAD_HISTORY_FILE.call(c, 0x002d_9724)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SnowFall_ResetBoxes(self)` (0x2e62c8): reset the twelve snow boxes.
pub fn snow_fall_reset_boxes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(0x10));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0xb;
        c.g[V0][0] = c.lw(r32(c, S0));
        loop {
            c.cycles += 4;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !empty {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002e_6300);
                callee::SNOW_BOX_RESET.call(c, 0x002e_6300)?;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) < 0 {
                break;
            }
            c.g[V0][0] = c.lw(r32(c, S0));
            if event_due(c) {
                deliver_event_here(c, 0x002e_62e8);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_FreeLocalTables(self)` (0x308a48): destroy and clear the
/// 50 local object tables.
pub fn w_script_man_free_local_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(0x3cc));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x31;
        loop {
            c.cycles += 5;
            c.g[V0][0] = c.lw(r32(c, S0));
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[A1][0] = 3;
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !empty {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0030_8a7c);
                call_ptr(c, callee::C_LUNO_TABLE_DTOR, 0x0030_8a7c)?;
                Addr(r32(c, S0)).sw(c, 0);
            }
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_8a60);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MEM_ReportPoolStats(void)` (0x31a200): ask the seven size-class
/// pools to report themselves, when the pool allocator is initialized.
pub fn mem_report_pool_stats(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, gp(c, 0xdd8));
        let empty = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !empty {
            c.cycles += 14;
            c.g[RA][0] = sx(0x0031_a218);
            c.g[A0][0] = lw(c, gp(c, 0x2a70));
            callee::MEM_REPORT_POOL_STAT0.call(c, 0x0031_a218)?;
            c.g[RA][0] = sx(0x0031_a220);
            c.g[A0][0] = lw(c, gp(c, 0x2a60));
            callee::MEM_REPORT_POOL_STAT1.call(c, 0x0031_a220)?;
            c.g[RA][0] = sx(0x0031_a228);
            c.g[A0][0] = lw(c, gp(c, 0x2a6c));
            callee::MEM_REPORT_POOL_STAT2.call(c, 0x0031_a228)?;
            c.g[RA][0] = sx(0x0031_a230);
            c.g[A0][0] = lw(c, gp(c, 0x2a7c));
            callee::MEM_REPORT_POOL_STAT3.call(c, 0x0031_a230)?;
            c.g[RA][0] = sx(0x0031_a238);
            c.g[A0][0] = lw(c, gp(c, 0x2a84));
            callee::MEM_REPORT_POOL_STAT4.call(c, 0x0031_a238)?;
            c.g[RA][0] = sx(0x0031_a240);
            c.g[A0][0] = lw(c, gp(c, 0x2a78));
            callee::MEM_REPORT_POOL_STAT5.call(c, 0x0031_a240)?;
            c.g[RA][0] = sx(0x0031_a248);
            c.g[A0][0] = lw(c, gp(c, 0x2a68));
            callee::MEM_REPORT_POOL_STAT6.call(c, 0x0031_a248)?;
        }
        c.cycles += 3;
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
    use crate::difftest::fakes::{machine, vtable, FAKE, LOG_RESULTS};
    use crate::difftest::{
        check, check_lohi, elf_image, put_words, scramble, Rng, DATA, GAME_GP, RA as TEST_RA,
    };

    const SEEDS: u32 = 100;

    /// A `gp`-relative global address.
    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// Zero `words` words at `at`.
    fn clear(c: &mut Cpu, at: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(at + 4 * i, 0);
        }
    }

    /// Call a generated helper as a setup step (arguments in a0..t1).
    fn load(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = TEST_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, TEST_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The main heap's arena over 256 KiB of test RAM, so constructors and
    /// destructors that allocate or free take their normal path.
    fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        load(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1);
    }

    /// A block from the test heap (`alloc_ready` must have run first). Used for
    /// objects whose destructor frees them (a static test pointer would send
    /// the heap into its consolidate path).
    fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        const MEM_HEAP: u32 = 0x004f_f250;
        load(
            c,
            0x0032_0058,
            ssx3_ee::s10::f_00320058,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    /// The dummy world at `world_at`: `+0x84` the game module, its `+0xc` the
    /// race manager. Returns the module address.
    fn dummy_world(c: &mut Cpu, world_at: u32) -> u32 {
        let module = DATA + 0x9000;
        let race = DATA + 0x9400;
        clear(c, world_at, 0x40);
        clear(c, module, 0x100);
        clear(c, race, 0x100);
        c.bus.write32(world_at + 0x84, module);
        c.bus.write32(module + 0xc, race);
        c.bus.write32(gp_at(-0x848), world_at);
        module
    }

    /// The usual globals: a game-like `gp`, the BELibrary singleton dummy and
    /// a small world.
    fn common(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(gp_at(-0x1f04), DATA + 0xb000);
        dummy_world(c, DATA + 0x8000);
        clear(c, DATA + 0xb000, 0x100);
        r
    }

    /// A 3-word pseudo-random scale vector at `at`.
    fn put_scale(c: &mut Cpu, r: &mut Rng, at: u32) {
        put_words(c, at, &[r.edge_f32(), r.edge_f32(), r.edge_f32()]);
    }

    // ------------------------------------------------------------------
    // Front end.

    #[test]
    fn c_fe_state_online_character_select_on_child_event_matches() {
        check(
            ee::s03::f_001b1ec0__generated,
            c_fe_state_online_character_select_on_child_event,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1b1ec0, 0);
                let this = DATA + 0x100;
                let child = DATA + 0x200;
                clear(c, this, 0x200);
                clear(c, child, 0x40);
                c.bus.write32(this + 0x69c, 0);
                let kind = [0x835u32, 0x838, 0x800][(seed % 3) as usize];
                c.bus.write32(child + 0x18, kind);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(child);
                c.g[A2][0] = [0xff, 0xf, 0xff][(seed % 3) as usize] as u64;
                c.bus.write32(gp_at(0x2dc), 0);
            },
        );
    }

    #[test]
    fn c_fe_state_chat_room_update_matches() {
        check_lohi(
            ee::s03::f_001b2320__generated,
            c_fe_state_chat_room_update,
            SEEDS,
            &|c, seed| {
                let mut r = common(c, seed, 0x1b2320, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let q = DATA + 0x1000;
                clear(c, q, 0x10);
                c.bus.write32(q + 0x18, 1 + (seed % 5) * 10000 + 1);
                c.bus.write32(gp_at(0x2a74), q);
                let vt = vtable(c, &mut r, &[0xd8, 0xe0]);
                c.bus.write32(this + 8, vt);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_fe_state_net_account_mem_dtor_matches() {
        check(
            ee::s03::f_001b7030__generated,
            c_fe_state_net_account_mem_dtor,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1b7030, 0);
                alloc_ready(c);
                let this = heap_block(c, 0x800);
                clear(c, this, 0x200);
                c.bus.write32(this + 0x6c0, 0x0048_dcd4); // static empty cBXString
                c.bus.write32(gp_at(0x2e0), 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_create_duplicate_ctor_matches() {
        check(
            ee::s03::f_001bbd08__generated,
            c_fe_state_create_duplicate_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = common(c, seed, 0x1bbd08, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x800);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_net_quick_messages_transition_in_matches() {
        check(
            ee::s03::f_001c23c0__generated,
            c_fe_state_net_quick_messages_transition_in,
            SEEDS,
            &|c, seed| {
                let mut r = common(c, seed, 0x1c23c0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x800);
                let vt = vtable(c, &mut r, &[0x100]);
                c.bus.write32(this + 8, vt);
                c.g[A0][0] = sx(this);
                // RefreshAll formats ten messages: take it only on a few seeds.
                c.g[A1][0] = (seed % 5 == 0) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_stats_transition_in_matches() {
        check(
            ee::s03::f_001c32b8__generated,
            c_fe_state_stats_transition_in,
            SEEDS,
            &|c, seed| {
                let mut r = common(c, seed, 0x1c32b8, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x800);
                let vt = vtable(c, &mut r, &[0x100]);
                c.bus.write32(this + 8, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 7 == 0 { 1 } else { 0 };
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_select_option_matches() {
        check(
            ee::s03::f_001c75c8__generated,
            c_fe_popup_confirm_select_option,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1c75c8, 0);
                let menu = DATA + 0x100;
                clear(c, menu, 0x200);
                c.g[A0][0] = sx(DATA + 0x300);
                c.g[A1][0] = sx(menu);
                c.g[A2][0] = (((seed as i32 % 7) - 3) as i64) as u64;
                c.g[A3][0] = match seed % 5 {
                    0 => u64::MAX,
                    1 => 0,
                    n => n as u64,
                };
            },
        );
    }

    #[test]
    fn c_keyboard_popup_on_activate_matches() {
        check(
            ee::s04::f_001cd2f0__generated,
            c_keyboard_popup_on_activate,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1cd2f0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                // The cursor must never go negative: OnActivate decrements it
                // at the limit, and UpdateCursor uses `cursor - length` as the
                // memcpy length. Keep `length` at 0 and the cursor positive.
                let v = 1 + (seed % 15);
                c.bus.write32(this + 0x60, v);
                c.bus
                    .write32(this + 0x134, if seed % 2 == 0 { v } else { v + 1 });
                c.bus.write32(this + 0x138, 0);
                c.bus.write32(this + 0x70, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed as u64) | 1;
            },
        );
    }

    #[test]
    fn c_keyboard_popup_apply_font_scale_matches() {
        check(
            ee::s04::f_001ce4c8__generated,
            c_keyboard_popup_apply_font_scale,
            SEEDS,
            &|c, seed| {
                let mut r = common(c, seed, 0x1ce4c8, 1);
                let obj = DATA + 0x1000;
                clear(c, obj, 0x100);
                put_scale(c, &mut r, obj + 0x50);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(obj);
            },
        );
    }

    #[test]
    fn c_fe_state_reward_gallery_is_item_owned_matches() {
        check(
            ee::s04::f_001d05f0__generated,
            c_fe_state_reward_gallery_is_item_owned,
            SEEDS,
            &|c, seed| {
                let mut r = common(c, seed, 0x1d05f0, 3);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                c.bus.write32(gp_at(-0x1efc), 0);
                c.bus.write32(0x0053_08a8, 0);
                c.bus.write8(this + 0x44, (seed % 3) as u8);
                c.bus
                    .write32(this + 0x4c, if seed % 2 == 0 { 0 } else { 3 + seed % 6 });
                c.bus.write32(this + 0x50, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn mem_card_state_base_reset_state_matches() {
        check(
            ee::s04::f_001d8be0__generated,
            mem_card_state_base_reset_state,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1d8be0, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_fe_state_trophy_room_dtor_matches() {
        check(
            ee::s04::f_001e0210__generated,
            c_fe_state_trophy_room_dtor,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1e0210, 0);
                alloc_ready(c);
                let this = heap_block(c, 0x800);
                clear(c, this, 0x200);
                // The trophy release walks five image slots and hands each
                // handle to BoltFile_Release; a large refcount keeps it on the
                // early-out path.
                c.bus.write32(DATA + 0x8000 + 0x7c, DATA + 0x3000);
                c.bus.write32(DATA + 0x4ca4, 100);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn message_center_get_unread_count_matches() {
        check(
            ee::s04::f_001e3408__generated,
            message_center_get_unread_count,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1e3408, 0);
                c.bus.write32(gp_at(-0x1efc), 0);
                c.bus.write32(0x0053_08a8, 0);
            },
        );
    }

    #[test]
    fn c_fe_state_lodge_rider_detail_ctor_matches() {
        check(
            ee::s04::f_001f3ff8__generated,
            c_fe_state_lodge_rider_detail_ctor,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1f3ff8, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let module = DATA + 0x2000;
                clear(c, module, 0x40);
                c.bus.write32(this + 0x10, module);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_fe_state_lodge_rider_detail_on_render_matches() {
        check(
            ee::s04::f_001f4380__generated,
            c_fe_state_lodge_rider_detail_on_render,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1f4380, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let screen = DATA + 0x2000;
                clear(c, screen, 0x200);
                c.bus
                    .write32(this + 0x48, if seed % 3 == 0 { 1 } else { 0 });
                c.bus.write32(this + 0x40, screen);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_fe_state_career_stats_ctor_matches() {
        check(
            ee::s04::f_001f54b0__generated,
            c_fe_state_career_stats_ctor,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1f54b0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let module = DATA + 0x2000;
                clear(c, module, 0x40);
                c.bus.write32(this + 0x10, module);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn section_man_dequeue_section_matches() {
        check(
            ee::s05::f_0022e360__generated,
            section_man_dequeue_section,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22e360, 0);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let node = DATA + 0x2000;
                let next = DATA + 0x2100;
                clear(c, node, 0x40);
                clear(c, next, 0x40);
                c.bus.write32(next + 4, r.next_u32());
                c.bus.write32(this + 0x1a0, r.next_u32() % 100);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(node) };
                c.g[A2][0] = sx(next);
                let _ = seed;
            },
        );
    }

    #[test]
    fn mc_manager_state_0c_matches() {
        check(
            ee::s06::f_0023f0f8__generated,
            mc_manager_state_0c,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x23f0f8, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x500);
                c.bus.write32(this + 0x6c, FAKE);
                c.bus.write32(this + 0x428, seed % 4);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn buddy_man_disconnect_matches() {
        check(
            ee::s06::f_00261408__generated,
            buddy_man_disconnect,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x261408, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn buddy_man_get_roster_entry_matches() {
        check(
            ee::s07::f_002625c8__generated,
            buddy_man_get_roster_entry,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x2625c8, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let list = DATA + 0x2000;
                clear(c, list, 0x100);
                c.bus.write32(this + 0xcc, list);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = match seed % 5 {
                    0 => u64::MAX,
                    1 => 0,
                    2 => 1,
                    n => n as u64,
                };
            },
        );
    }

    #[test]
    fn c_shared_login_man_is_card_unformatted_matches() {
        check(
            ee::s07::f_00265cb0__generated,
            c_shared_login_man_is_card_unformatted,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x265cb0, 2);
                let holder = DATA + 0x100;
                let obj = DATA + 0x2000;
                clear(c, holder, 0x10);
                clear(c, obj, 0x400);
                c.bus.write32(holder, obj);
                c.bus.write16(obj + 0x1a8, 0x20);
                c.bus.write32(obj + 0x1ac, FAKE);
                c.bus.write16(obj + 0x1b8, 0x30);
                c.bus.write32(obj + 0x1bc, FAKE + 0x10);
                c.bus.write32(gp_at(0x2e8), holder);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_shared_login_man_mc_list_dir_matches() {
        check(
            ee::s07::f_00265fb8__generated,
            c_shared_login_man_mc_list_dir,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x265fb8, 3);
                alloc_ready(c);
                let memcard = DATA + 0x2000;
                clear(c, memcard, 0x200);
                c.bus.write32(gp_at(0x2e8), memcard);
                c.g[A0][0] = (seed % 4) as u64;
                c.g[A1][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn net_event_e2_dtor_matches() {
        check(ee::s07::f_00268158__generated, net_event_e2_dtor, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x268158, 1);
            alloc_ready(c);
            let this = heap_block(c, 0x100);
            clear(c, this, 0x40);
            c.bus.write32(this + 8, 0x0048_dcd4); // static empty cBXString
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn net_event102_down_dtor_matches() {
        check(
            ee::s07::f_00268398__generated,
            net_event102_down_dtor,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x268398, 1);
                alloc_ready(c);
                let this = heap_block(c, 0x100);
                clear(c, this, 0x40);
                c.bus.write32(this + 0xc, 0x0048_dcd4); // static empty cBXString
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn net_event_e5_dtor_matches() {
        check(ee::s07::f_00268588__generated, net_event_e5_dtor, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x268588, 1);
            alloc_ready(c);
            let this = heap_block(c, 0x100);
            clear(c, this, 0x40);
            c.bus.write32(this + 8, 0x0048_dcd4); // static empty cBXString
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn replay_save_resume_snapshot_matches() {
        check(
            ee::s07::f_0026f7f8__generated,
            replay_save_resume_snapshot,
            SEEDS,
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
    fn script_engine_stop_aux_lists_matches() {
        check(
            ee::s07::f_00279488__generated,
            script_engine_stop_aux_lists,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x279488, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x800);
                let lists = DATA + 0x2000;
                clear(c, lists, 0x200);
                c.bus.write32(this + 0x624, lists);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn script_view_obj_add_cam_matches() {
        check(
            ee::s07::f_00281190__generated,
            script_view_obj_add_cam,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x281190, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let cam = DATA + 0x2000;
                clear(c, cam, 0x40);
                let p0 = if seed % 4 == 0 { 0 } else { r.next_u32() };
                let p1 = if seed % 5 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(this + 0xd0, p0);
                c.bus.write32(this + 0xd4, p1);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(cam);
            },
        );
    }

    #[test]
    fn script_engine_script_db_ctor_matches() {
        check(
            ee::s07::f_00281e80__generated,
            script_engine_script_db_ctor,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x281e80, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn ssx_audio_system_option_to_output_mode_matches() {
        check(
            ee::s07::f_00289d08__generated,
            ssx_audio_system_option_to_output_mode,
            160,
            &|c, seed| {
                let mut r = Rng(0x289d08 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn snd_bank_dtor_matches() {
        check(ee::s07::f_0028b278__generated, snd_bank_dtor, SEEDS, &|c, seed| {
            let _ = common(c, seed, 0x28b278, 0);
            alloc_ready(c);
            let this = heap_block(c, 0x400);
            clear(c, this, 0x100);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn ssx_audio_system_stop_ambience_loop_matches() {
        check(
            ee::s07::f_002913d8__generated,
            ssx_audio_system_stop_ambience_loop,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x2913d8, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x6000);
                let sys = DATA + 0x2000;
                let vt = DATA + 0x3000;
                clear(c, sys, 0x1000);
                clear(c, vt, 0x200);
                c.bus.write32(this + 0x118, sys);
                c.bus.write32(sys, DATA + 0x4000);
                c.bus.write32(sys + 0xab0, vt);
                c.bus.write16(vt + 0x18, 0x1d8);
                c.bus.write32(vt + 0x1c, FAKE);
                put_words(c, 0x004a_3680, &[0x1234_5678, 0x9abc_def0]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_loop_sfx_matches() {
        check(
            ee::s08::f_0029d678__generated,
            ssx_audio_system_stop_loop_sfx,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x29d678, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x6100);
                let sys = DATA + 0x3000;
                clear(c, sys, 0x200);
                c.bus.write32(sys, sys);
                c.bus.write32(this + 0x118, sys);
                c.bus
                    .write32(this + 0x5fc4, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(this + 0x5fc8, if seed % 4 == 0 { u32::MAX } else { 3 });
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn ssx_audio_system_has_track_changed_matches() {
        check(
            ee::s08::f_002a4718__generated,
            ssx_audio_system_has_track_changed,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x2a4718, 3);
                let this = DATA + 0x100;
                clear(c, this, 0x6000);
                let iface = DATA + 0x2000;
                clear(c, iface, 0x100);
                let vt = DATA + 0x3000;
                clear(c, vt, 0x100);
                c.bus.write32(gp_at(-0x1efc), 0);
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, vt);
                c.bus.write16(vt + 8, 0x10);
                c.bus.write32(vt + 0xc, FAKE);
                c.bus
                    .write32(this + 0x5814, [0x17u32, 0, 5, 0x17][(seed % 4) as usize]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn crowd_audio_pick_random_sample_matches() {
        check_lohi(
            ee::s08::f_002a4ca0__generated,
            crowd_audio_pick_random_sample,
            SEEDS,
            &|c, seed| {
                let mut r = common(c, seed, 0x2a4ca0, 1);
                let list = DATA + 0x100;
                clear(c, list, 0x40);
                let n = (seed % 5) as u32;
                c.bus.write32(list, n);
                for i in 0..n {
                    c.bus.write32(list + 4 + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(list);
            },
        );
    }

    #[test]
    fn stream_array_destroy_stream_matches() {
        check(
            ee::s08::f_002aabd0__generated,
            stream_array_destroy_stream,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x2aabd0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                let array = DATA + 0x2000;
                clear(c, this, 0x40);
                clear(c, array, 0x100);
                c.bus.write32(this + 8, array);
                let i = seed % 4;
                let buf = if seed % 3 == 0 {
                    0
                } else {
                    heap_block(c, 0x100)
                };
                if buf != 0 {
                    clear(c, buf, 0x40);
                }
                c.bus.write32(array + 4 * i, buf);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = i as u64;
            },
        );
    }

    #[test]
    fn bank_monitor_resume_all_matches() {
        check(
            ee::s08::f_002ad300__generated,
            bank_monitor_resume_all,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x2ad300, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                let inst = DATA + 0x1000;
                clear(c, this, 0x1000);
                clear(c, inst, 0x3000);
                c.bus.write32(this + 0x8ec, inst);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn path_plat_get_track_volume_matches() {
        check(
            ee::s08::f_002af428__generated,
            path_plat_get_track_volume,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2af428, 0);
                let table = 0x004d_3e98u32;
                let obj = DATA + 0x2000;
                clear(c, obj, 0x100);
                let idx = seed % 5;
                let present = seed % 3 != 2;
                c.bus
                    .write32(table + 4 * idx, if present { obj } else { 0 });
                c.bus.write8(obj + 0x65, 0);
                c.bus.write32(obj + 0xc, r.next_u32());
                c.g[A0][0] = idx as u64;
            },
        );
    }

    #[test]
    fn mem_card_ascii_to_sjis_string_matches() {
        check(
            ee::s08::f_002c2210__generated,
            mem_card_ascii_to_sjis_string,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x2c2210, 1);
                let src = DATA + 0x2000;
                let dst = DATA + 0x3000;
                clear(c, src, 0x40);
                clear(c, dst, 0x40);
                let s = match seed % 4 {
                    0 => "",
                    1 => "A",
                    2 => "SSX3",
                    _ => "name",
                };
                for (i, b) in s.bytes().enumerate() {
                    c.bus.write8(src + i as u32, b);
                }
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(dst);
                c.g[A2][0] = sx(src);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_format_matches() {
        check(
            ee::s08::f_002c4388__generated,
            c_ps2_mem_card_do_format,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x2c4388, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                c.bus.write32(this + 0xc, seed % 4);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v34_matches() {
        check(
            ee::s08::f_002c4520__generated,
            c_ps2_mem_card_v34,
            SEEDS,
            &|c, seed| {
                elf_image(c);
                let _ = common(c, seed, 0x2c4520, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let name = DATA + 0x2000;
                clear(c, name, 0x40);
                c.bus.write8(name, b'S');
                c.bus.write8(name + 1, b'S');
                c.bus.write8(name + 2, b'X');
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v35_matches() {
        check(
            ee::s08::f_002c4578__generated,
            c_ps2_mem_card_v35,
            SEEDS,
            &|c, seed| {
                elf_image(c);
                let _ = common(c, seed, 0x2c4578, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let name = DATA + 0x2000;
                clear(c, name, 0x40);
                c.bus.write16(name, b'S' as u16);
                c.bus.write16(name + 2, b'X' as u16);
                c.bus.write16(name + 4, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v29_matches() {
        check(
            ee::s08::f_002c4800__generated,
            c_ps2_mem_card_v29,
            SEEDS,
            &|c, seed| {
                elf_image(c);
                let _ = common(c, seed, 0x2c4800, 3);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = sx(DATA + 0x2100);
            },
        );
    }

    #[test]
    fn menu_sys_page_validate_selection_matches() {
        check(
            ee::s09::f_002caab0__generated,
            menu_sys_page_validate_selection,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x2caab0, 1);
                let this = DATA + 0x100;
                let item = DATA + 0x2000;
                let obj = DATA + 0x3000;
                clear(c, this, 0x200);
                clear(c, item, 0x100);
                clear(c, obj, 0x100);
                let idx = seed % 3;
                c.bus.write32(this + 4, idx);
                c.bus.write32(this + 0xc + 4 * idx, item);
                c.bus.write32(item + 0x10, obj);
                c.bus.write16(obj + 0x18, 0x20);
                c.bus.write32(obj + 0x1c, FAKE);
                if seed % 2 == 0 {
                    c.bus.write32(LOG_RESULTS, 1);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn menu_sys_page_end_draw_matches() {
        check(
            ee::s09::f_002cb2f8__generated,
            menu_sys_page_end_draw,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x2cb2f8, 2);
                let mgr = DATA + 0x100;
                let obj = DATA + 0x2000;
                clear(c, mgr, 0x2000);
                clear(c, obj, 0x200);
                c.bus.write32(mgr + 0x10d8, obj);
                c.bus.write16(obj + 0x100, 0x20);
                c.bus.write32(obj + 0x104, FAKE);
                c.bus.write16(obj + 0xb0, 0x20);
                c.bus.write32(obj + 0xb4, FAKE + 0x10);
                c.bus.write32(mgr + 0xe84, 0x40 + seed % 8);
                c.bus.write32(gp_at(-0x854), mgr);
            },
        );
    }

    #[test]
    fn menu_sys_action_item_measure_matches() {
        check(
            ee::s09::f_002cd090__generated,
            menu_sys_action_item_measure,
            SEEDS,
            &|c, seed| {
                let mut r = common(c, seed, 0x2cd090, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                let item = DATA + 0x2000;
                clear(c, this, 0x400);
                clear(c, item, 0x100);
                c.bus.write32(item + 0x14, r.next_u32() % 100);
                let icon = seed % 3 != 0;
                c.bus
                    .write32(item + 0x18, if icon { DATA + 0x3000 } else { 0 });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(item);
            },
        );
    }

    #[test]
    fn snow_fall_reset_boxes_matches() {
        check(
            ee::s09::f_002e62c8__generated,
            snow_fall_reset_boxes,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x2e62c8, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x100);
                for i in 0..12u32 {
                    let p = if (seed + i) % 3 == 0 {
                        0
                    } else {
                        DATA + 0x2000 + 0x100 * i
                    };
                    c.bus.write32(this + 0x10 + 4 * i, p);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn w_script_man_free_local_tables_matches() {
        check(
            ee::s10::f_00308a48__generated,
            w_script_man_free_local_tables,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x308a48, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x1000);
                for i in 0..50u32 {
                    if (seed + i) % 4 != 0 {
                        let table = heap_block(c, 0x40);
                        clear(c, table, 0x10);
                        c.bus.write32(this + 0x3cc + 4 * i, table);
                    }
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn mem_report_pool_stats_matches() {
        check(
            ee::s10::f_0031a200__generated,
            mem_report_pool_stats,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x31a200, 0);
                c.g[GP][0] = GAME_GP as u64;
                c.bus
                    .write32(gp_at(0xdd8), if seed % 3 == 0 { 0 } else { 1 });
                for off in [0x2a60i32, 0x2a68, 0x2a6c, 0x2a70, 0x2a78, 0x2a7c, 0x2a84] {
                    c.bus.write32(gp_at(off), 0);
                }
            },
        );
    }
}
