//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold front-end, script, memcard, modifier, stream and
//! overlay helpers, each with a differential test against the generated body.
//!
//! The bodies are transcribed from the generated Rust (the authority for
//! registers, cycles, memory and call order). Calls go through
//! [`crate::call`] with the original return address and register setup, so a
//! replaced callee and the fake hooks the differential tests install are both
//! reached. Structured loops make the original's back-edge event checks with
//! [`crate::events::event_due`] and [`crate::events::deliver_event_here`].

use crate::abi::r32;
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001a_86e8, fe_set_online_mode, "FE_SetOnlineMode"),
    (
        0x001a_fd88,
        net_fe_show_player_conn_quality,
        "NetFE_ShowPlayerConnQuality",
    ),
    (
        0x001b_d848,
        c_fe_state_lobby_menu_on_net_result,
        "cFEStateLobbyMenu_OnNetResult",
    ),
    (
        0x001c_0f40,
        c_fe_state_net_quick_play_popup_ctor,
        "cFEStateNetQuickPlayPopup_ctor",
    ),
    (
        0x001c_1c90,
        c_fe_state_net_ssx_world_init,
        "cFEStateNetSSXWorld_Init",
    ),
    (
        0x0025_9310,
        tag_field_get_string_unescaped,
        "TagFieldGetStringUnescaped",
    ),
    (
        0x0027_8cd8,
        script_engine_unload_world_scripts,
        "ScriptEngine_UnloadWorldScripts",
    ),
    (
        0x0027_9720,
        script_engine_play_script_instance,
        "ScriptEngine_PlayScriptInstance",
    ),
    (
        0x0028_0730,
        script_audio_obj_activate,
        "ScriptAudioObj_Activate",
    ),
    (
        0x002c_fe78,
        menu_sys_group_item_on_focus,
        "MenuSysGroupItem_OnFocus",
    ),
    (0x0012_b180, ai_reset_rider_stats, "AI_ResetRiderStats"),
    (
        0x0013_13a8,
        rider_board_press_sub_settle_slew,
        "RiderBoardPressSub_SettleSlew",
    ),
    (0x0037_2aa0, path_arrow_ctor, "PathArrow_ctor"),
    (0x0023_f258, mc_manager_state17, "MCManager_State17"),
    (0x0024_0f30, mc_manager_state30, "MCManager_State30"),
    (
        0x0024_1180,
        mc_manager_end_progress,
        "MCManager_EndProgress",
    ),
    (
        0x0024_1fd0,
        mc_manager_on_card_scanned,
        "MCManager_OnCardScanned",
    ),
    (
        0x001c_5dd8,
        c_fe_popup_confirm_rebuild,
        "cFEPopupConfirm_Rebuild",
    ),
    (
        0x001c_a900,
        c_ui_state_buy_popup_set_song_credit_params,
        "cUIStateBuyPopup_SetSongCreditParams",
    ),
    (
        0x0030_b068,
        w_script_process_list_dtor,
        "WScriptProcessList_dtor",
    ),
    (
        0x0035_5978,
        mod_obj_add_parent_modifier,
        "ModObj_AddParentModifier",
    ),
    (
        0x0035_5db8,
        mod_obj_add_roller_modifier,
        "ModObj_AddRollerModifier",
    ),
    (0x0035_6c48, mod_obj_v21, "ModObj_v21"),
    (0x0035_76d0, magnet_modifier_save, "MagnetModifier_Save"),
    (0x0035_7fa0, multi_particle_dtor, "MultiParticle_dtor"),
    (
        0x001d_10a8,
        c_fe_state_reward_gallery_update_scroll_arrows,
        "cFEStateRewardGallery_UpdateScrollArrows",
    ),
    (
        0x001d_2518,
        c_fe_state_video_player_handle_input,
        "cFEStateVideoPlayer_HandleInput",
    ),
    (
        0x0019_b518,
        c_fe_state_char_equip_fill_equip_rows,
        "cFEStateCharEquip_FillEquipRows",
    ),
    (
        0x0019_b598,
        c_fe_state_char_equip_fill_buy_rows,
        "cFEStateCharEquip_FillBuyRows",
    ),
    (
        0x002a_a7f0,
        stream_array_create_stream,
        "StreamArray_CreateStream",
    ),
    (
        0x0030_8be0,
        w_script_man_spawn_processes,
        "WScriptMan_SpawnProcesses",
    ),
    (
        0x001d_9948,
        mem_card_state_base_alloc_read_buffer,
        "MemCardStateBase_AllocReadBuffer",
    ),
    (0x0010_e558, rider_on_split_marker, "Rider_OnSplitMarker"),
    (
        0x0028_2838,
        script_engine_is_process_finished,
        "ScriptEngine_IsProcessFinished",
    ),
    (
        0x0015_78a0,
        be_reward_int_are_all_peaks_complete,
        "BERewardInt_AreAllPeaksComplete",
    ),
    (
        0x002b_69b8,
        world_trigger_manager_check_crossing,
        "WorldTriggerManager_CheckCrossing",
    ),
    (0x0027_5d10, script_list_ctor, "ScriptList_ctor"),
    (0x0031_9028, c_bx_string_right, "cBXString_Right"),
    (0x0039_1708, font_load, "Font_Load"),
    (0x0026_d4d8, run_list_seek, "RunList_Seek"),
    (0x003e_cf30, tag_field_get_number, "TagFieldGetNumber"),
    (0x0039_7c40, ui_engine_load_package, "UIEngine_LoadPackage"),
    (
        0x0035_3e80,
        flex_bridge_springs_load,
        "FlexBridge_SpringsLoad",
    ),
    (0x0020_8498, ov_screen_map_init, "OVScreenMap_Init"),
    (
        0x0034_af38,
        c_flag_manager_group_ctor,
        "cFlagManager_GroupCtor",
    ),
    (
        0x0036_9610,
        graphics_man_set_color_mask,
        "GraphicsMan_SetColorMask",
    ),
    (0x001f_76b8, ov_screen_bc_fail_init, "OVScreenBCFail_Init"),
    (
        0x0021_11a0,
        ov_screen_profile_save_pda_init,
        "OVScreenProfileSavePDA_Init",
    ),
    (
        0x0022_df50,
        section_man_begin_event_switch,
        "SectionMan_BeginEventSwitch",
    ),
    (
        0x0020_d110,
        ov_screen_pda_template_open,
        "OVScreenPDATemplate_Open",
    ),
];

// ---------------------------------------------------------------------------
// Callees (generated shard functions, reached through their override slots).

mod callee {
    use super::{ee, Callee};

    pub const BUDDY_MAN_IS_CONNECTED: Callee = Callee {
        addr: 0x0026_1460,
        f: ee::s06::f_00261460,
    };
    pub const LOBBY_MAN: Callee = Callee {
        addr: 0x0026_0f80,
        f: ee::s06::f_00260f80,
    };
    pub const BUDDY_MAN_RECONNECT: Callee = Callee {
        addr: 0x0026_13c8,
        f: ee::s06::f_002613c8,
    };
    pub const LOBBY_MAN_DISCONNECT: Callee = Callee {
        addr: 0x0025_b800,
        f: ee::s06::f_0025b800,
    };
    pub const BUDDY_MAN_DISCONNECT: Callee = Callee {
        addr: 0x0026_1408,
        f: ee::s06::f_00261408,
    };
    pub const NET_FE_SET_QUALITY_ICONS: Callee = Callee {
        addr: 0x001a_ad50,
        f: ee::s03::f_001aad50,
    };
    pub const GET_HASH_VALUE_32: Callee = Callee {
        addr: 0x0031_7670,
        f: ee::s10::f_00317670,
    };
    pub const UI_SCREEN_FIND_OBJECT: Callee = Callee {
        addr: 0x0039_d860,
        f: ee::s13::f_0039d860,
    };
    pub const UI_TEXT_SET_UNICODE_STRING_BY_ID: Callee = Callee {
        addr: 0x003a_1000,
        f: ee::s13::f_003a1000,
    };
    pub const NET_FE_STATE_HANDLE_NET_EVENT: Callee = Callee {
        addr: 0x001a_8e40,
        f: ee::s03::f_001a8e40,
    };
    pub const LOBBY_MENU_ON_CREATE_ROOM_RESULT: Callee = Callee {
        addr: 0x001b_da88,
        f: ee::s03::f_001bda88,
    };
    pub const LOBBY_MENU_ON_JOIN_ROOM_RESULT: Callee = Callee {
        addr: 0x001b_d8c8,
        f: ee::s03::f_001bd8c8,
    };
    pub const NET_FE_STATE_CTOR: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    };
    pub const UI_ENGINE_ADD_SCREEN_BY_HASH_NAME: Callee = Callee {
        addr: 0x0039_7cc0,
        f: ee::s13::f_00397cc0,
    };
    pub const UI_SCREEN_PLAY_FRAME: Callee = Callee {
        addr: 0x0039_c870,
        f: ee::s13::f_0039c870,
    };
    pub const NET_FE_STATE_SHOW_DISCONNECTED_POPUP: Callee = Callee {
        addr: 0x001a_9930,
        f: ee::s03::f_001a9930,
    };
    pub const TAG_FIELD_GET_STRING: Callee = Callee {
        addr: 0x003e_d378,
        f: ee::s14::f_003ed378,
    };
    pub const STRCPY: Callee = Callee {
        addr: 0x0041_66f4,
        f: ee::s15::f_004166f4,
    };
    pub const ASYNC_FILE_RELEASE_1: Callee = Callee {
        addr: 0x003d_faf0,
        f: ee::s14::f_003dfaf0,
    };
    pub const ASYNC_FILE_RELEASE_2: Callee = Callee {
        addr: 0x003d_f9d8,
        f: ee::s14::f_003df9d8,
    };
    pub const SCRIPT_ENGINE_UNLOAD_BANK: Callee = Callee {
        addr: 0x0027_b410,
        f: ee::s07::f_0027b410,
    };
    pub const MEM_FREE: Callee = Callee {
        addr: 0x0025_23a8,
        f: ee::s06::f_002523a8,
    };
    pub const SCRIPT_ENGINE_GET_PROCESS_IN_SEQ: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::f_00282bf0,
    };
    pub const SCRIPT_ENGINE_GET_INSTANCE_SCRIPT: Callee = Callee {
        addr: 0x0027_a668,
        f: ee::s07::f_0027a668,
    };
    pub const SCRIPT_ENGINE_RUN_PROCESS: Callee = Callee {
        addr: 0x0028_22a0,
        f: ee::s07::f_002822a0,
    };
    pub const SCRIPT_OBJ_ACTIVATE: Callee = Callee {
        addr: 0x0028_3430,
        f: ee::s07::f_00283430,
    };
    pub const SCRIPT_DATA_GET_INFO: Callee = Callee {
        addr: 0x0027_c070,
        f: ee::s07::f_0027c070,
    };
    pub const SCRIPT_ENGINE_FIND_SOUND_BANK: Callee = Callee {
        addr: 0x0027_9ef8,
        f: ee::s07::f_00279ef8,
    };
    pub const MENU_SYS_ITEM_ON_FOCUS: Callee = Callee {
        addr: 0x002c_a368,
        f: ee::s09::f_002ca368,
    };
    pub const MENU_SYS_PAGE_SET_SELECTION: Callee = Callee {
        addr: 0x002c_be68,
        f: ee::s09::f_002cbe68,
    };
    pub const MENU_SYS_PAGE_ON_ACTIVATE: Callee = Callee {
        addr: 0x002c_bf60,
        f: ee::s09::f_002cbf60,
    };
    pub const BE_SCORE_INT_UPDATE_BEST_STATS: Callee = Callee {
        addr: 0x0015_4a58,
        f: ee::s01::f_00154a58,
    };
    pub const RIDER_METRIX_RESET_STATS: Callee = Callee {
        addr: 0x0011_75b8,
        f: ee::s00::f_001175b8,
    };
    pub const MC_MANAGER_ON_OPERATION_DONE: Callee = Callee {
        addr: 0x0023_ea90,
        f: ee::s06::f_0023ea90,
    };
    pub const MC_MANAGER_END_OPERATION: Callee = Callee {
        addr: 0x0023_ea30,
        f: ee::s06::f_0023ea30,
    };
    pub const MC_MANAGER_SET_MODE: Callee = Callee {
        addr: 0x0023_d570,
        f: ee::s06::f_0023d570,
    };
    pub const FE_POPUP_CONFIRM_RESET_LAYOUT: Callee = Callee {
        addr: 0x001c_8a38,
        f: ee::s03::f_001c8a38,
    };
    pub const FE_POPUP_CONFIRM_APPLY_CONFIG: Callee = Callee {
        addr: 0x001c_6b08,
        f: ee::s03::f_001c6b08,
    };
    pub const FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE: Callee = Callee {
        addr: 0x001c_7738,
        f: ee::s03::f_001c7738,
    };
    pub const BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };
    pub const PARENT_MODIFIER_CTOR: Callee = Callee {
        addr: 0x0035_7038,
        f: ee::s11::f_00357038,
    };
    pub const ROLLER_MODIFIER_CTOR: Callee = Callee {
        addr: 0x0035_da70,
        f: ee::s11::f_0035da70,
    };
    pub const MOD_OBJ_SET_TRANSFORM_MODIFIER: Callee = Callee {
        addr: 0x0035_54b0,
        f: ee::s11::f_003554b0,
    };
    pub const MULTI_SPLINE_MODIFIER_ADD_REF: Callee = Callee {
        addr: 0x0035_aad0,
        f: ee::s11::f_0035aad0,
    };
    pub const MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::f_00317d70,
    };
    pub const MEM_FREE_2: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };
    pub const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };
    pub const SCRIPT_ENGINE_GET_PROCESS: Callee = Callee {
        addr: 0x0028_2cb0,
        f: ee::s07::f_00282cb0,
    };
    pub const SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS: Callee = Callee {
        addr: 0x0028_2908,
        f: ee::s07::f_00282908,
    };
    pub const C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW: Callee = Callee {
        addr: 0x0019_b7e0,
        f: ee::s03::f_0019b7e0,
    };
    pub const C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW: Callee = Callee {
        addr: 0x0019_ba60,
        f: ee::s03::f_0019ba60,
    };
    pub const STREAM_BUFFER_CTOR: Callee = Callee {
        addr: 0x002a_99d8,
        f: ee::s08::f_002a99d8,
    };
    pub const WSCRIPT_MAN_CREATE_PROCESS: Callee = Callee {
        addr: 0x0030_94c8,
        f: ee::s10::f_003094c8,
    };
    /// `WScriptMan` wrappers that turn the manager into the cache pointer
    /// (`lw(lw(man+4)+4) + section`) and call the cache accessors.
    pub const WSCRIPT_CACHE_GET_PROCESS_COUNT: Callee = Callee {
        addr: 0x003a_6c08,
        f: ee::s13::f_003a6c08,
    };
    pub const WSCRIPT_CACHE_GET_PROCESSES: Callee = Callee {
        addr: 0x003a_6c38,
        f: ee::s13::f_003a6c38,
    };
    pub const RIDER_METRIX_ON_SPLIT_MARKER: Callee = Callee {
        addr: 0x0011_94c0,
        f: ee::s00::f_001194c0,
    };
    pub const SCRIPT_CMD_QUEUE_CTOR: Callee = Callee {
        addr: 0x0028_3c30,
        f: ee::s07::f_00283c30,
    };
    pub const SCRIPT_CMD_QUEUE_CLEAR: Callee = Callee {
        addr: 0x0028_3c20,
        f: ee::s07::f_00283c20,
    };
    pub const BX_STRING_ALLOC_COPY: Callee = Callee {
        addr: 0x0031_81e8,
        f: ee::s10::f_003181e8,
    };
    pub const BX_STRING_CTOR_COPY: Callee = Callee {
        addr: 0x0031_7fe8,
        f: ee::s10::f_00317fe8,
    };
    pub const BX_STRING_DTOR: Callee = Callee {
        addr: 0x0031_8260,
        f: ee::s10::f_00318260,
    };
    pub const FONT_PARSE_GLYPHS: Callee = Callee {
        addr: 0x0039_1788,
        f: ee::s12::f_00391788,
    };
    pub const SPRINTF: Callee = Callee {
        addr: 0x0041_7828,
        f: ee::s15::f_00417828,
    };
    pub const FILE_LOAD: Callee = Callee {
        addr: 0x003e_18c8,
        f: ee::s14::f_003e18c8,
    };
    pub const UI_ENGINE_STORE_WORD: Callee = Callee {
        addr: 0x0039_7138,
        f: ee::s13::f_00397138,
    };
    pub const UI_ENGINE_LOAD_TEXTURES: Callee = Callee {
        addr: 0x0039_8138,
        f: ee::s13::f_00398138,
    };
    pub const FLEX_BRIDGE_ALLOC_SPRING_NODES: Callee = Callee {
        addr: 0x0035_3f00,
        f: ee::s11::f_00353f00,
    };
    pub const CBE_GET_BE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };
    pub const CBE_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };
    pub const C_OV_STATE_OVERLAY_INIT: Callee = Callee {
        addr: 0x0020_a380,
        f: ee::s05::f_0020a380,
    };
    pub const EVENT_SELECT_INIT: Callee = Callee {
        addr: 0x0020_0700,
        f: ee::s05::f_00200700,
    };
    pub const C_FLAG_MANAGER_GROUP_INIT_KEY: Callee = Callee {
        addr: 0x0034_af20,
        f: ee::s11::f_0034af20,
    };
    pub const OV_SCREEN_PDA_TEMPLATE_OPEN: Callee = Callee {
        addr: 0x0020_d110,
        f: ee::s05::f_0020d110,
    };
    pub const OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS: Callee = Callee {
        addr: 0x0021_2080,
        f: ee::s05::f_00212080,
    };
    pub const GAME_MODULE_REQUEST_STATE: Callee = Callee {
        addr: 0x0023_1250,
        f: ee::s06::f_00231250,
    };
    pub const GAME_MODULE_REQUEST_OVERLAY_STATE: Callee = Callee {
        addr: 0x0023_1278,
        f: ee::s06::f_00231278,
    };
    pub const UI_STATE_CTOR: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::f_0039e2a0,
    };
    pub const UI_STATE_MGR_ADD_BACKGROUND_STATE: Callee = Callee {
        addr: 0x0039_f238,
        f: ee::s13::f_0039f238,
    };
    pub const BEREWARD_INT_IS_PEAK_COMPLETE: Callee = Callee {
        addr: 0x0015_7a78,
        f: ee::s01::f_00157a78,
    };
}

// ---------------------------------------------------------------------------
// Small register helpers.

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// The address `reg + off`.
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// A 32-bit `arg` (the low word of a GPR).
#[inline(always)]
fn word(c: &Cpu, reg: usize) -> u32 {
    r32(c, reg)
}

/// `addu dst, a, b` of two low words, sign-extended.
#[inline(always)]
fn add32(a: u32, b: u32) -> u64 {
    sx(a.wrapping_add(b))
}

// ---------------------------------------------------------------------------
// Front end / net.

/// `void FE_SetOnlineMode(int mode)` (0x001a86e8): record the mode at
/// `gp-0x1680`; going offline disconnects the lobby and buddy session.
pub fn fe_set_online_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        Addr(gp(c, -0x1680)).sw(c, c.g[A0][0]);
        let online = c.g[A0][0] != 0;
        frame.save_ra(c, 0);
        if online {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
            let absent = c.g[A0][0] == 0;
            frame.restore_ra(c, 0);
            if absent {
                // Nothing to disconnect.
            } else {
                c.cycles += 4;
                callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x001a_870c)?;
                let connected = c.g[V0][0];
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                if connected != 0 {
                    // The session is up: make sure the lobby manager exists,
                    // reconnect the buddy session if it dropped out.
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        callee::LOBBY_MAN.call(c, 0x001a_8724)?;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                    callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x001a_872c)?;
                    let still = c.g[V0][0];
                    c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                    if still == 0 {
                        c.cycles += 3;
                        c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                        callee::BUDDY_MAN_RECONNECT.call(c, 0x001a_873c)?;
                        c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                        callee::LOBBY_MAN_DISCONNECT.call(c, 0x001a_8750)?;
                    }
                    c.cycles += 2;
                    c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                    callee::BUDDY_MAN_DISCONNECT.call(c, 0x001a_8758)?;
                }
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void NetFE_ShowPlayerConnQuality(cUIScreen* screen, player*)`
/// (0x001afd88): fill the connection-quality text and icons.
pub fn net_fe_show_player_conn_quality(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[A0][0] = sx(0x460000);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = sx(0x460000u32.wrapping_add(0x3538));
        callee::GET_HASH_VALUE_32.call(c, 0x001a_fdb0)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::UI_SCREEN_FIND_OBJECT.call(c, 0x001a_fdbc)?;
        c.g[S0][0] = c.g[V0][0];
        let absent = c.g[S0][0] == 0;
        c.g[A0][0] = sx(0x460000);
        if !absent {
            c.cycles += 9;
            c.g[A0][0] = sx(0x460000u32.wrapping_add(0x35c0));
            callee::GET_HASH_VALUE_32.call(c, 0x001a_fdd0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_UNICODE_STRING_BY_ID.call(c, 0x001a_fddc)?;
            c.g[A2][0] = Addr(at(c, S2, 0xc)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            callee::NET_FE_SET_QUALITY_ICONS.call(c, 0x001a_fdec)?;
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

/// `void cFEStateLobbyMenu_OnNetResult(self, msg, a2)` (0x001bd848): dispatch
/// a net result to the create/join room handlers or the generic handler.
pub fn c_fe_state_lobby_menu_on_net_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, A0, 0x1c)).lw(c);
        c.g[V0][0] &= 6;
        let none = c.g[V0][0] == 0;
        c.g[V1][0] = 0x828;
        if none {
            c.cycles += 3;
            // a0 still holds the message (self), as at the original call.
            callee::NET_FE_STATE_HANDLE_NET_EVENT.call(c, 0x001b_d8b8)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(at(c, A0, 0x6a0)).lw(c);
            c.g[A3][0] = Addr(at(c, V0, 0x18)).lw(c);
            c.g[V0][0] = 0x829;
            if c.g[A3][0] == 0x828 {
                c.cycles += 4;
                c.g[A1][0] = c.g[A2][0];
                callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT.call(c, 0x001b_d888)?;
                frame.restore_ra(c, 0);
            } else if c.g[A3][0] == 0x829 {
                c.cycles += 2;
                c.cycles += 4;
                c.g[A1][0] = c.g[A2][0];
                callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT.call(c, 0x001b_d898)?;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                c.cycles += 2;
                c.cycles += 4;
                callee::NET_FE_STATE_HANDLE_NET_EVENT.call(c, 0x001b_d8a8)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetQuickPlayPopup* ctor(self)` (0x001c0f40): the net quick-play
/// popup state's constructor.
pub fn c_fe_state_net_quick_play_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR.call(c, 0x001c_0f54)?;
        c.g[V0][0] = 0x48;
        c.g[V1][0] = 0x470000;
        Addr(at(c, S0, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(0x470000u32.wrapping_sub(0x54d8));
        Addr(at(c, S0, 0x8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        for off in [
            0x6ec, 0x720, 0x724, 0x728, 0x72c, 0x730, 0x6f8, 0x6fc, 0x700, 0x704, 0x708, 0x70c,
            0x710, 0x714, 0x718, 0x71c,
        ] {
            Addr(at(c, S0, off)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetSSXWorld_Init(self)` (0x001c1c90): create the net SSX
/// World screen and (when disconnected) its popup.
pub fn c_fe_state_net_ssx_world_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x460000);
        c.g[A0][0] = sx(0x460000u32.wrapping_add(0x5a38));
        c.g[S0][0] = Addr(at(c, S1, 0x10)).lw(c);
        callee::GET_HASH_VALUE_32.call(c, 0x001c_1cb4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME.call(c, 0x001c_1cc8)?;
        let absent = c.g[V0][0] == 0;
        Addr(at(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_SCREEN_PLAY_FRAME.call(c, 0x001c_1ce0)?;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[V1][0] = Addr(at(c, V0, 0)).lw(c);
        let online = c.g[V1][0] != 0;
        c.lq(16, frame.at(c, 0x20));
        if !online {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP.call(c, 0x001c_1cf8)?;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int TagFieldGetStringUnescaped(const char* tag, char* dst, int size)`
/// (0x00259310): parse a tag field into a stack buffer and copy it, dropping
/// a leading backtick.
pub fn tag_field_get_string_unescaped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[A2][0] = sx(word(c, A2).wrapping_add(1));
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        c.g[V0][0] = sx(word(c, A2) << 3);
        c.sq(30, at(c, SP, 0x30));
        c.g[V0][0] = sx(word(c, V0) >> 3);
        c.sq(16, at(c, SP, 0x40));
        c.g[V0][0] = sx(word(c, V0).wrapping_add(0xf));
        c.sd(at(c, SP, 0x20), c.g[RA][0]);
        c.g[V0][0] = sx(word(c, V0) >> 4);
        c.g[FP][0] = c.g[SP][0];
        c.g[V0][0] = sx(word(c, V0) << 4);
        c.g[S0][0] = c.g[A1][0];
        c.g[SP][0] = sx(sp(c).wrapping_sub(word(c, V0)));
        c.g[RA][0] = sx(0x0025_934c);
        c.g[A1][0] = c.g[SP][0];
        callee::TAG_FIELD_GET_STRING.call(c, 0x0025_934c)?;
        c.g[V1][0] = Addr(sp(c)).lb(c);
        c.g[V0][0] = 0x60;
        let plain = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if plain {
            c.cycles += 2;
            c.g[A1][0] = c.g[SP][0];
            callee::STRCPY.call(c, 0x0025_9374)?;
        } else {
            c.cycles += 4;
            c.g[A1][0] = sx(word(c, SP) | 1);
            callee::STRCPY.call(c, 0x0025_9364)?;
        }
        c.cycles += 6;
        c.g[SP][0] = c.g[FP][0];
        frame_restore(c, 16, 0x40);
        frame_restore(c, 30, 0x30);
        c.g[RA][0] = c.ld(at(c, SP, 0x20));
        let ra = word(c, RA);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    })
}

/// `lq reg, off(sp)` (a bare restore, no frame object).
#[inline(always)]
fn frame_restore(c: &mut Cpu, reg: usize, off: u32) {
    let a = at(c, SP, off);
    c.lq(reg, a);
}

/// `void ScriptEngine_UnloadWorldScripts(self)` (0x00278cd8): release the
/// world script file and unload the script bank.
pub fn script_engine_unload_world_scripts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(at(c, S0, 0x538)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 10;
            callee::ASYNC_FILE_RELEASE_1.call(c, 0x0027_8cfc)?;
            c.g[A0][0] = Addr(at(c, S0, 0x538)).lw(c);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::ASYNC_FILE_RELEASE_2.call(c, 0x0027_8d0c)?;
            c.g[V1][0] = Addr(at(c, S0, 0x620)).lw(c);
            c.g[V1][0] = sx(word(c, V1).wrapping_sub(1));
            Addr(at(c, S0, 0x620)).sw(c, c.g[V1][0]);
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        c.g[A1][0] = 0x1;
        callee::SCRIPT_ENGINE_UNLOAD_BANK.call(c, 0x0027_8d24)?;
        c.g[A0][0] = Addr(at(c, S0, 0x53c)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            callee::MEM_FREE.call(c, 0x0027_8d38)?;
        }
        Addr(at(c, S0, 0x540)).sw(c, 0);
        c.cycles += 6;
        Addr(at(c, S0, 0x538)).sw(c, 0);
        Addr(at(c, S0, 0x53c)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_PlayScriptInstance(self, key, val, b, c)` (0x00279720):
/// look the process up, set its instance script's field and run it.
pub fn script_engine_play_script_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A3][0];
        c.g[S4][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0027_9758);
        c.g[A1][0] = c.g[S1][0];
        callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ.call(c, 0x0027_9758)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT.call(c, 0x0027_9764)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S4][0];
        c.g[RA][0] = sx(0x0027_977c);
        Addr(at(c, V0, 0xc)).sw(c, c.g[S2][0]);
        callee::SCRIPT_ENGINE_RUN_PROCESS.call(c, 0x0027_977c)?;
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

/// `void ScriptAudioObj_Activate(self)` (0x00280730): activate the script
/// object, resolve its data/bank and clear the four sound slots.
pub fn script_audio_obj_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(word(c, S0).wrapping_add(0xc));
        callee::SCRIPT_OBJ_ACTIVATE.call(c, 0x0028_0748)?;
        Addr(at(c, S0, 0xec)).sw(c, 0);
        Addr(at(c, S0, 0xf0)).sw(c, 0);
        Addr(at(c, S0, 0xf4)).sw(c, 0);
        c.g[V0][0] = Addr(at(c, S0, 0x8)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 0x8)).lw(c);
        callee::SCRIPT_DATA_GET_INFO.call(c, 0x0028_0760)?;
        c.g[A1][0] = Addr(at(c, V0, 0x1c)).lw(c);
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        callee::SCRIPT_ENGINE_FIND_SOUND_BANK.call(c, 0x0028_076c)?;
        Addr(at(c, S0, 0xf8)).sw(c, c.g[V0][0]);
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[S0][0] = sx(word(c, S0).wrapping_add(0xbc));
        c.g[V0][0] = 0x3;
        loop {
            c.cycles += 7;
            Addr(at(c, S0, 0x8)).sw(c, c.g[V1][0]);
            c.g[V0][0] = sx(word(c, V0).wrapping_sub(1));
            Addr(at(c, S0, 0)).sw(c, c.g[V1][0]);
            Addr(at(c, S0, 0x4)).sw(c, 0);
            let more = (c.g[V0][0] as i64) >= 0;
            c.g[S0][0] = sx(word(c, S0).wrapping_add(0xc));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0028_0780);
                }
                continue;
            }
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            break;
        }
        Ok(())
    })
}

/// `void MenuSysGroupItem_OnFocus(self, mode)` (0x002cfe78): select the
/// group's page (0 = first, 1 = last) and activate it.
pub fn menu_sys_group_item_on_focus(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::MENU_SYS_ITEM_ON_FOCUS.call(c, 0x002c_fe94)?;
        let mut select: Option<u64> = None;
        let s0 = c.g[S0][0];
        c.g[V0][0] = 0x1;
        if s0 == 0 {
            c.cycles += 4;
            c.g[S0][0] = sx(word(c, S1).wrapping_add(0x18));
            select = Some(0);
        } else {
            c.cycles += 2;
            c.g[A0][0] = sx(word(c, S1).wrapping_add(0x18));
            if s0 == 1 {
                c.cycles += 3;
                c.g[S0][0] = sx(word(c, S1).wrapping_add(0x18));
                select = Some(0xffffffffffffffff);
            } else {
                c.cycles += 2;
                c.g[V0][0] = Addr(at(c, S1, 0)).lw(c);
            }
        }
        if let Some(arg) = select {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = arg;
            callee::MENU_SYS_PAGE_SET_SELECTION.call(c, 0x002c_fed0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = Addr(at(c, S1, 0)).lw(c);
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x002c_fee0);
        c.g[A1][0] = Addr(at(c, V0, 0x124)).lw(c);
        callee::MENU_SYS_PAGE_ON_ACTIVATE.call(c, 0x002c_fee0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_ResetRiderStats(race*)` (0x0012b180): reset the score and metric
/// stats of every rider in the race.
pub fn ai_reset_rider_stats(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, S2, 0x78)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[S0][0] = sx(word(c, S2).wrapping_add(0x28));
            c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
            loop {
                c.cycles += 13;
                c.g[A0][0] = c.g[S1][0];
                c.g[S1][0] = sx(word(c, S1).wrapping_add(1));
                c.g[A1][0] = Addr(at(c, V0, 0x790)).lw(c);
                c.g[RA][0] = sx(0x0012_b1c4);
                c.g[A1][0] = sx(word(c, A1).wrapping_add(0xfc));
                callee::BE_SCORE_INT_UPDATE_BEST_STATS.call(c, 0x0012_b1c4)?;
                c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
                c.g[S0][0] = sx(word(c, S0).wrapping_add(4));
                c.g[RA][0] = sx(0x0012_b1d4);
                c.g[A0][0] = Addr(at(c, V0, 0x790)).lw(c);
                callee::RIDER_METRIX_RESET_STATS.call(c, 0x0012_b1d4)?;
                c.g[V0][0] = Addr(at(c, S2, 0x78)).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0012_b1b0);
                    }
                    continue;
                }
                break;
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

/// `void RiderBoardPressSub_SettleSlew(rider)` (0x001313a8): clamp the board
/// press sub-state's slew against ±0.5, or snap it to the 1.0/0.3 targets.
pub fn rider_board_press_sub_settle_slew(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = Addr(at(c, A0, 0x14)).lw(c);
    c.g[AT][0] = 0xffff_ffff_bf00_0000;
    c.mtc1(AT, 0);
    c.lwc1(1, at(c, V0, 0x280));
    c.c_lt_s(1, 0);
    if !c.fcc() {
        c.cycles += 6;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.c_lt_s(0, 1);
        let below = !c.fcc();
        c.lwc1(0, at(c, V0, 0x300));
        if below {
            c.cycles += 5;
            c.lwc1(1, gp(c, -0x75f8));
            Addr(at(c, V0, 0x288)).sw(c, 0);
            c.mul_s(0, 0, 1);
            let ra = word(c, RA);
            c.swc1(0, at(c, V0, 0x284));
            c.pc = ra;
            return;
        }
        c.cycles += 5;
        c.lwc1(1, gp(c, -0x75fc));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        c.mul_s(0, 0, 1);
    } else {
        c.cycles += 5;
        c.lwc1(0, at(c, V0, 0x300));
        c.lwc1(1, gp(c, -0x7600));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 2);
        c.mul_s(0, 0, 1);
    }
    c.cycles += 3;
    c.swc1(2, at(c, V0, 0x288));
    let ra = word(c, RA);
    c.swc1(0, at(c, V0, 0x284));
    c.pc = ra;
}

/// `void* PathArrow_ctor(self)` (0x00372aa0): set the arrow's vtable and
/// initialise the global 8-entry arrow table once.
pub fn path_arrow_ctor(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = sx(0x490000);
    c.g[V1][0] = Addr(gp(c, 0x114c)).lw(c);
    c.g[V0][0] = sx(0x490000u32.wrapping_add(0x3038));
    let done = c.g[V1][0] != 0;
    Addr(at(c, A0, 0)).sw(c, c.g[V0][0]);
    if done {
        c.cycles += 2;
        let ra = word(c, RA);
        c.g[V0][0] = c.g[A0][0];
        c.pc = ra;
        return;
    }
    c.cycles += 11;
    c.g[V0][0] = sx(0x500000);
    c.g[T0][0] = Addr(gp(c, 0x1124)).lw(c);
    c.g[V1][0] = sx(0x500000u32.wrapping_add(0x4560));
    c.g[A3][0] = Addr(gp(c, 0x1118)).lw(c);
    c.g[A2][0] = Addr(gp(c, 0x111c)).lw(c);
    c.g[T1][0] = sx(word(c, V1).wrapping_add(0x180));
    c.g[A1][0] = Addr(gp(c, 0x1120)).lw(c);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    Addr(at(c, V1, 0x1c)).sw(c, c.g[T0][0]);
    loop {
        c.cycles += 11;
        Addr(at(c, V1, 0x10)).sw(c, c.g[A3][0]);
        Addr(at(c, V1, 0x14)).sw(c, c.g[A2][0]);
        Addr(at(c, V1, 0x18)).sw(c, c.g[A1][0]);
        c.swc1(0, at(c, V1, 0x8));
        Addr(at(c, V1, 0xc)).sw(c, 0);
        Addr(at(c, V1, 0)).sw(c, 0);
        Addr(at(c, V1, 0x4)).sw(c, 0);
        c.g[V1][0] = sx(word(c, V1).wrapping_add(0x30));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            Addr(at(c, V1, 0x1c)).sw(c, c.g[T0][0]);
            if event_due(c) {
                deliver_event_here(c, 0x0037_2ae0);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        Addr(gp(c, 0x114c)).sw(c, c.g[V0][0]);
        break;
    }
    c.cycles += 2;
    let ra = word(c, RA);
    c.g[V0][0] = c.g[A0][0];
    c.pc = ra;
}

/// The shared tail of `MCManager_State17`/`State30`: probe the card operation
/// (three virtual calls) and finish it with `MCManager_EndOperation(self, a1)`.
fn mc_manager_finish(c: &mut Cpu, end_arg: u64, ret: u32, done: u32, probe: u32, end: u32) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A1][0] = Addr(at(c, S0, 0x434)).lw(c);
    c.g[V1][0] = Addr(at(c, A1, 0)).lw(c);
    c.g[A0][0] = Addr(at(c, V1, 0x58)).lh(c);
    c.g[V0][0] = Addr(at(c, V1, 0x5c)).lw(c);
    c.g[A0][0] = add32(word(c, A1), word(c, A0));
    call_ptr(c, word(c, V0), ret)?;
    if c.g[V0][0] != 0 {
        frame.restore(c, S0, 0x10);
    } else {
        c.cycles += 5;
        c.g[V0][0] = Addr(at(c, S0, 0x84)).lw(c);
        call_ptr(c, word(c, V0), probe)?;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::MC_MANAGER_ON_OPERATION_DONE.call(c, done)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[V0][0] = Addr(at(c, S0, 0x6c)).lw(c);
            c.cycles += 4;
            call_ptr(c, word(c, V0), probe.wrapping_add(0x20))?;
            let finished = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !finished {
                c.cycles += 2;
                c.g[A1][0] = end_arg;
                callee::MC_MANAGER_END_OPERATION.call(c, end)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void MCManager_State17(self)` (0x0023f258): tick the memcard manager's
/// operation 17.
pub fn mc_manager_state17(c: &mut Cpu) {
    guest(c, |c| {
        mc_manager_finish(c, 0, 0x0023_f280, 0x0023_f2a4, 0x0023_f294, 0x0023_f2c4)
    })
}

/// `void MCManager_State30(self)` (0x00240f30): tick the memcard manager's
/// operation 30.
pub fn mc_manager_state30(c: &mut Cpu) {
    guest(c, |c| {
        mc_manager_finish(c, 1, 0x0024_0f58, 0x0024_0f7c, 0x0024_0f6c, 0x0024_0f9c)
    })
}

/// `void MCManager_EndProgress(self, flag)` (0x00241180): notify the card
/// operation's owner and switch to mode 0 with the progress text cleared.
pub fn mc_manager_end_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(at(c, S0, 0x118)).lw(c);
        let absent = c.g[V0][0] == 0;
        Addr(gp(c, -0x478)).sw(c, 0);
        if !absent {
            c.cycles += 6;
            c.g[A1][0] = Addr(at(c, S0, 0x434)).lw(c);
            c.g[V0][0] = Addr(at(c, A1, 0)).lw(c);
            c.g[A0][0] = Addr(at(c, V0, 0x160)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0x164)).lw(c);
            c.g[A0][0] = add32(word(c, A1), word(c, A0));
            call_ptr(c, word(c, V1), 0x0024_11bc)?;
        }
        c.cycles += 2;
        if c.g[S1][0] != 0 {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            callee::MC_MANAGER_SET_MODE.call(c, 0x0024_11d0)?;
            c.g[V0][0] = Addr(at(c, S0, 0x748)).lw(c);
            c.g[A1][0] = 0x14;
            c.g[A0][0] = Addr(at(c, V0, 0x8)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
            c.g[A0][0] = add32(word(c, S0), word(c, A0));
            call_ptr(c, word(c, V1), 0x0024_11e8)?;
        }
        c.lq(16, frame.at(c, 0x20));
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_OnCardScanned(self)` (0x00241fd0): after a card scan,
/// update the manager's 0x33/0x35 messages.
pub fn mc_manager_on_card_scanned(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(at(c, S0, 0xac)).lw(c);
        call_ptr(c, word(c, V0), 0x0024_1fec)?;
        c.g[A1][0] = Addr(at(c, S0, 0x434)).lw(c);
        c.g[V1][0] = Addr(at(c, A1, 0)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x58)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0x5c)).lw(c);
        c.g[A0][0] = add32(word(c, A1), word(c, A0));
        call_ptr(c, word(c, V0), 0x0024_2004)?;
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(at(c, S0, 0x748)).lw(c);
        if !done {
            c.cycles += 7;
            c.g[A1][0] = 0x33;
            c.g[A0][0] = Addr(at(c, V0, 0x8)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
            c.g[A0][0] = add32(word(c, S0), word(c, A0));
            call_ptr(c, word(c, V1), 0x0024_2020)?;
            c.lq(16, frame.at(c, 0x10));
        } else {
            c.cycles += 6;
            c.g[A1][0] = 0x35;
            c.g[A0][0] = Addr(at(c, V0, 0x8)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
            c.g[A0][0] = add32(word(c, S0), word(c, A0));
            call_ptr(c, word(c, V1), 0x0024_203c)?;
            c.lq(16, frame.at(c, 0x10));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupConfirm_Rebuild(self)` (0x001c5dd8): re-lay out the confirm
/// popup when it has a background, then scale and show it.
pub fn c_fe_popup_confirm_rebuild(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(at(c, S0, 0x2bc)).lw(c);
        if c.g[V0][0] == 0 {
            c.lq(16, frame.at(c, 0x20));
            c.cycles += 3;
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 21;
        callee::FE_POPUP_CONFIRM_RESET_LAYOUT.call(c, 0x001c_5dfc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(word(c, S0).wrapping_add(0xbc));
        callee::FE_POPUP_CONFIRM_APPLY_CONFIG.call(c, 0x001c_5e08)?;
        c.g[A0][0] = c.g[S0][0];
        c.ldl(2, at(c, S0, 0x27f));
        c.ldr(2, at(c, S0, 0x278));
        c.g[V1][0] = Addr(at(c, S0, 0x280)).lw(c);
        c.sdl(2, at(c, SP, 0x7));
        c.sdr(2, at(c, SP, 0));
        Addr(at(c, SP, 0x8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[SP][0];
        callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE.call(c, 0x001c_5e2c)?;
        c.g[V0][0] = Addr(at(c, S0, 0x8)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(at(c, V0, 0x30)).lh(c);
        c.g[V1][0] = Addr(at(c, V0, 0x34)).lw(c);
        c.g[A0][0] = add32(word(c, S0), word(c, A0));
        call_ptr(c, word(c, V1), 0x001c_5e44)?;
        c.lq(16, frame.at(c, 0x20));
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIStateBuyPopup_SetSongCreditParams(self, song, credit)`
/// (0x001ca900): fill the buy-popup info with the song's credit price.
pub fn c_ui_state_buy_popup_set_song_credit_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[A0][0] = sx(0x460000);
        c.g[S1][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[A0][0] = sx(0x460000u32.wrapping_add(0x61b8));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x3;
        callee::GET_HASH_VALUE_32.call(c, 0x001c_a934)?;
        Addr(at(c, S0, 0x10)).sw(c, c.g[S1][0]);
        c.g[V1][0] = 0x2;
        Addr(at(c, S0, 0x14)).sw(c, c.g[S2][0]);
        c.g[A0][0] = sx(0x460000);
        Addr(at(c, S0, 0xc)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(0x460000u32.wrapping_add(0x61c8));
        c.g[RA][0] = sx(0x001c_a954);
        Addr(at(c, S0, 0x18)).sw(c, c.g[V0][0]);
        callee::GET_HASH_VALUE_32.call(c, 0x001c_a954)?;
        Addr(at(c, S0, 0x8)).sw(c, c.g[S3][0]);
        Addr(at(c, S0, 0x1c)).sw(c, c.g[S2][0]);
        Addr(at(c, S0, 0x20)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptProcessList_dtor(self, flags)` (0x0030b068): tell every
/// process in the list to unlink itself, then free the list when asked to.
pub fn w_script_process_list_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = Addr(at(c, S1, 0)).lw(c);
        let empty = c.g[S0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !empty {
            c.cycles += 1;
            c.g[A2][0] = c.g[S0][0];
            loop {
                c.cycles += 2;
                let null = c.g[A2][0] == 0;
                c.g[S0][0] = Addr(at(c, S0, 0x18)).lw(c);
                if !null {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(at(c, A2, 0x5c)).lw(c);
                    c.g[A1][0] = 0x3;
                    c.g[A0][0] = Addr(at(c, V0, 0x8)).lh(c);
                    c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
                    c.g[A0][0] = add32(word(c, A2), word(c, A0));
                    call_ptr(c, word(c, V1), 0x0030_b0b0)?;
                }
                c.cycles += 2;
                let more = c.g[S0][0] != 0;
                c.g[A2][0] = c.g[S0][0];
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_b090);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 1;
        let keep = c.g[V0][0] == 0;
        c.lq(16, frame.at(c, 0x30));
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE.call(c, 0x0030_b0cc)?;
            c.lq(16, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ModObj_AddParentModifier(self, a1, a2, pos)` (0x00355978): allocate a
/// parent modifier, build it and install it as the transform modifier.
pub fn mod_obj_add_parent_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = sx(0x490000);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xb0;
        c.g[A1][0] = sx(0x490000u32.wrapping_sub(0x1720));
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        callee::MEM_ALLOC_NAMED.call(c, 0x0035_59b8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        callee::PARENT_MODIFIER_CTOR.call(c, 0x0035_59cc)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[V0][0];
        callee::MOD_OBJ_SET_TRANSFORM_MODIFIER.call(c, 0x0035_59d8)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ModObj_AddRollerModifier(self, a1, a2, a3)` (0x00355db8): allocate a
/// roller modifier, build it and install it as the transform modifier.
pub fn mod_obj_add_roller_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = sx(0x490000);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x2d0;
        c.g[A1][0] = sx(0x490000u32.wrapping_sub(0x16f8));
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        callee::MEM_ALLOC_NAMED.call(c, 0x0035_5df8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        callee::ROLLER_MODIFIER_CTOR.call(c, 0x0035_5e0c)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[V0][0];
        callee::MOD_OBJ_SET_TRANSFORM_MODIFIER.call(c, 0x0035_5e18)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ModObj_v21(self, a1)` (0x00356c48): when the object's multi-spline
/// modifier is in state 2, add a reference to it.
pub fn mod_obj_v21(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(c, S0, 0x1c)).lw(c);
        let none = c.g[V1][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if none {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            c.cycles += 4;
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A1][0] = Addr(at(c, V1, 0)).lw(c);
        let null = c.g[A1][0] == 0;
        c.g[V0][0] = 0;
        if null {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            c.cycles += 4;
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 8;
        c.g[V1][0] = Addr(at(c, A1, 0)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0xd0)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0xd4)).lw(c);
        c.g[A0][0] = add32(word(c, A1), word(c, A0));
        call_ptr(c, word(c, V0), 0x0035_6c88)?;
        c.g[V1][0] = 0x2;
        let not_state2 = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        if not_state2 {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            c.cycles += 4;
            frame.restore(c, S1, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V0][0] = Addr(at(c, S0, 0x1c)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(at(c, V0, 0)).lw(c);
        callee::MULTI_SPLINE_MODIFIER_ADD_REF.call(c, 0x0035_6ca4)?;
        c.lq(16, frame.at(c, 0x20));
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MagnetModifier_Save(self, stream)` (0x003576d0): write the magnet's
/// three parameter blocks through the stream's virtual writer.
pub fn magnet_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
        c.g[A1][0] = Addr(at(c, S1, 0xa0)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 0x28)).lh(c);
        c.g[V1][0] = Addr(at(c, V0, 0x2c)).lw(c);
        c.g[A0][0] = add32(word(c, S0), word(c, A0));
        call_ptr(c, word(c, V1), 0x0035_7700)?;
        c.g[A3][0] = Addr(at(c, S0, 0)).lw(c);
        c.g[A1][0] = sx(word(c, S1).wrapping_add(0x10));
        c.g[A2][0] = 0x50;
        c.g[A0][0] = Addr(at(c, A3, 0x8)).lh(c);
        c.g[V0][0] = Addr(at(c, A3, 0xc)).lw(c);
        c.g[A0][0] = add32(word(c, S0), word(c, A0));
        call_ptr(c, word(c, V0), 0x0035_771c)?;
        c.g[V1][0] = Addr(at(c, S0, 0)).lw(c);
        c.g[A1][0] = sx(word(c, S1).wrapping_add(0x60));
        c.g[A2][0] = 0x40;
        c.g[A0][0] = Addr(at(c, V1, 0x8)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0xc)).lw(c);
        c.g[A0][0] = add32(word(c, S0), word(c, A0));
        call_ptr(c, word(c, V0), 0x0035_7738)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MultiParticle_dtor(self, flags)` (0x00357fa0): free the particle
/// arrays, tell the owner, then free the object when asked to.
pub fn multi_particle_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(at(c, S0, 0x8)).lw(c);
        callee::BUILTIN_DELETE.call(c, 0x0035_7fc0)?;
        c.g[A0][0] = Addr(at(c, S0, 0xc)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A2][0] = Addr(at(c, S0, 0x10)).lw(c);
        } else {
            c.cycles += 3;
            callee::MEM_FREE_2.call(c, 0x0035_7fd4)?;
            c.g[A2][0] = Addr(at(c, S0, 0x10)).lw(c);
        }
        c.cycles += 2;
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(at(c, A2, 0x18c)).lw(c);
            c.g[A0][0] = Addr(at(c, V0, 0x8)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
            c.g[A0][0] = add32(word(c, A2), word(c, A0));
            call_ptr(c, word(c, V1), 0x0035_7ff4)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            c.lq(16, frame.at(c, 0x20));
        } else {
            // `a0 = s0` still needs the object pointer: the original reloads
            // s0 only after `__builtin_delete` returns.
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::BUILTIN_DELETE.call(c, 0x0035_8008)?;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardGallery_UpdateScrollArrows(self)`
/// (0x001d10a8): drive the gallery's two scroll-arrow widgets.
pub fn c_fe_state_reward_gallery_update_scroll_arrows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = Addr(at(c, S0, 0x91c)).lw(c);
        if c.g[A2][0] == 0 {
            c.g[T0][0] = Addr(at(c, S0, 0x920)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = Addr(at(c, A2, 0x8)).lw(c);
            c.g[A1][0] = Addr(at(c, S0, 0x64)).lw(c);
            c.g[A0][0] = Addr(at(c, V0, 0x48)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0x4c)).lw(c);
            c.g[A1][0] = ((0u64 as i64) < (c.g[A1][0] as i64)) as u64;
            c.g[A0][0] = add32(word(c, A2), word(c, A0));
            call_ptr(c, word(c, V1), 0x001d_10e0)?;
            c.g[T0][0] = Addr(at(c, S0, 0x920)).lw(c);
        }
        c.cycles += 2;
        if c.g[T0][0] != 0 {
            c.cycles += 11;
            c.g[A2][0] = Addr(at(c, T0, 0x8)).lw(c);
            c.g[A1][0] = Addr(at(c, S0, 0x64)).lw(c);
            c.g[V0][0] = Addr(at(c, S0, 0x58)).lw(c);
            c.g[A3][0] = Addr(at(c, S0, 0x60)).lw(c);
            c.g[A0][0] = Addr(at(c, A2, 0x48)).lh(c);
            c.g[A1][0] = sx(word(c, A1).wrapping_add(word(c, V0)));
            c.g[V1][0] = Addr(at(c, A2, 0x4c)).lw(c);
            c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[A3][0] as i64)) as u64;
            c.g[A0][0] = add32(word(c, T0), word(c, A0));
            call_ptr(c, word(c, V1), 0x001d_1114)?;
            c.lq(16, frame.at(c, 0x10));
        } else {
            c.lq(16, frame.at(c, 0x10));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool cFEStateVideoPlayer_HandleInput(self, input)` (0x001d2518): when the
/// video player is open, drive its two input widgets.
pub fn c_fe_state_video_player_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(at(c, S1, 0x270)).lw(c);
        let closed = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if closed {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr(at(c, S0, 0x8)).lw(c);
            c.g[A0][0] = Addr(at(c, V1, 0x28)).lh(c);
            c.g[V0][0] = Addr(at(c, V1, 0x2c)).lw(c);
            c.g[A0][0] = add32(word(c, S0), word(c, A0));
            call_ptr(c, word(c, V0), 0x001d_254c)?;
            let handled = c.g[V0][0] != 0;
            c.g[V1][0] = 1;
            if !handled {
                c.cycles += 7;
                c.g[V1][0] = Addr(at(c, S0, 0x8)).lw(c);
                c.g[A0][0] = Addr(at(c, V1, 0x30)).lh(c);
                c.g[V0][0] = Addr(at(c, V1, 0x34)).lw(c);
                c.g[A0][0] = add32(word(c, S0), word(c, A0));
                call_ptr(c, word(c, V0), 0x001d_2568)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    Addr(at(c, S1, 0x27c)).sw(c, c.g[V1][0]);
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 1;
                Addr(at(c, S1, 0x27c)).sw(c, c.g[V1][0]);
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

/// `void cFEStateCharEquip_FillEquipRows(self)` (0x0019b518): fill up to six
/// equip rows for the selected profile.
pub fn c_fe_state_char_equip_fill_equip_rows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(c, S2, 0x5c)).lw(c);
        c.g[V0][0] = Addr(at(c, S2, 0x958)).lw(c);
        c.g[S0][0] = Addr(at(c, V1, 0x98)).lbu(c);
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW.call(c, 0x0019_b558)?;
                c.g[S1][0] = sx(word(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
                let six = c.g[V0][0] == 0;
                c.g[S0][0] = sx(word(c, S0).wrapping_add(1));
                if six {
                    break;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr(at(c, S2, 0x958)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_b550);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_FillBuyRows(self)` (0x0019b598): fill up to six
/// buy rows for the selected profile.
pub fn c_fe_state_char_equip_fill_buy_rows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(c, S2, 0x5c)).lw(c);
        c.g[V0][0] = Addr(at(c, S2, 0x958)).lw(c);
        c.g[S0][0] = Addr(at(c, V1, 0x98)).lbu(c);
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW.call(c, 0x0019_b5d8)?;
                c.g[S1][0] = sx(word(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
                let six = c.g[V0][0] == 0;
                c.g[S0][0] = sx(word(c, S0).wrapping_add(1));
                if six {
                    break;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr(at(c, S2, 0x958)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_b5d0);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* StreamArray_CreateStream(self, idx, heap, extra)` (0x002aa7f0):
/// allocate a stream buffer and store it in slot `idx`.
pub fn stream_array_create_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx(word(c, A1) << 2);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        c.g[A1][0] = sx(0x480000);
        c.g[V0][0] = Addr(at(c, S1, 0x8)).lw(c);
        c.g[A1][0] = sx(0x480000u32.wrapping_add(0x3228));
        c.g[A0][0] = 0x84;
        c.g[A3][0] = 0;
        c.g[S0][0] = sx(word(c, S0).wrapping_add(word(c, V0)));
        callee::MEM_ALLOC_NAMED.call(c, 0x002a_a834)?;
        c.g[A1][0] = Addr(at(c, S1, 0)).lw(c);
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx(word(c, A1).wrapping_add(0x1d8));
        callee::STREAM_BUFFER_CTOR.call(c, 0x002a_a84c)?;
        Addr(at(c, S0, 0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_SpawnProcesses(self, section)` (0x00308be0): spawn a
/// script process for every entry in the section.
pub fn w_script_man_spawn_processes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(c, S3, 0x28c)).lw(c);
        callee::WSCRIPT_CACHE_GET_PROCESS_COUNT.call(c, 0x0030_8c08)?;
        c.g[A0][0] = Addr(at(c, S3, 0x28c)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        callee::WSCRIPT_CACHE_GET_PROCESSES.call(c, 0x0030_8c18)?;
        c.g[S1][0] = c.g[V0][0];
        if (c.g[S0][0] as i64) > 0 {
            loop {
                c.cycles += 8;
                c.g[A2][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A3][0] = 0;
                callee::WSCRIPT_MAN_CREATE_PROCESS.call(c, 0x0030_8c34)?;
                c.g[S0][0] = sx(word(c, S0).wrapping_sub(1));
                let more = c.g[S0][0] != 0;
                c.g[S1][0] = sx(word(c, S1).wrapping_add(0x40));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_8c20);
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

/// `void MemCardStateBase_AllocReadBuffer(self, size)` (0x001d9948):
/// (re)allocate the memcard state's read buffer.
pub fn mem_card_state_base_alloc_read_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(c, S0, 0x228)).lw(c);
        let fresh = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if fresh {
            c.cycles += 15;
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(at(c, S0, 0x1d4)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                callee::MEM_FREE_2.call(c, 0x001d_9980)?;
            }
            Addr(at(c, S0, 0x1d4)).sw(c, 0);
            c.cycles += 1;
            Addr(at(c, S0, 0x228)).sw(c, 0);
            c.cycles += 15;
        }
        c.g[A1][0] = sx(0x460000);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(0x460000u32.wrapping_add(0x7990));
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        callee::OPERATOR_NEW_NAMED.call(c, 0x001d_99a0)?;
        c.g[V1][0] = 1;
        Addr(at(c, S0, 0x1d4)).sw(c, c.g[V0][0]);
        Addr(at(c, S0, 0x228)).sw(c, c.g[V1][0]);
        Addr(at(c, S0, 0x1d0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Rider_OnSplitMarker(rider, marker)` (0x0010e558): route a split
/// marker to the rider's metrics when the feature is enabled.
pub fn rider_on_split_marker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x530000);
        c.g[V1][0] = Addr(at(c, S0, 0xec)).lw(c);
        Addr(at(c, S0, 0xe8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(at(c, A0, 0x8d0)).lw(c);
        c.g[V0][0] = sx(((word(c, V0) as i32) >> 9) as u32);
        c.g[V0][0] &= 1;
        let disabled = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !disabled {
            c.cycles += 10;
            callee::CBE_GET_BE.call(c, 0x0010_e594)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::CBE_GET_INTERFACE.call(c, 0x0010_e5a0)?;
            c.g[V1][0] = sx(0x530000);
            c.g[A0][0] = 0x1;
            c.g[V0][0] = Addr(at(c, V1, 0x5c12)).lb(c);
            if c.g[V0][0] != c.g[A0][0] {
                c.lq(16, frame.at(c, 0x20));
            } else {
                c.cycles += 3;
                c.g[A0][0] = Addr(at(c, S0, 0x790)).lw(c);
                c.g[A1][0] = Addr(at(c, S1, 0)).lw(c);
                callee::RIDER_METRIX_ON_SPLIT_MARKER.call(c, 0x0010_e5c0)?;
                c.cycles += 1;
                c.lq(16, frame.at(c, 0x20));
            }
        } else {
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_IsProcessFinished(self, a1)` (0x00282838): after a
/// process reaches its end state, launch it and report completion.
pub fn script_engine_is_process_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_PROCESS.call(c, 0x0028_2850)?;
        c.g[S0][0] = c.g[V0][0];
        let absent = c.g[S0][0] == 0;
        c.g[V0][0] = 1;
        if !absent {
            c.cycles += 3;
            c.g[V1][0] = Addr(at(c, S0, 0x4)).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = Addr(at(c, S0, 0x4)).lw(c);
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS.call(c, 0x0028_2874)?;
                c.g[V0][0] = Addr(at(c, S0, 0x4)).lw(c);
            }
            c.cycles += 4;
            c.g[V0][0] = sx(word(c, V0).wrapping_sub(4));
            c.g[V0][0] = (c.g[V0][0] < 2u64) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 5;
                c.g[V1][0] = Addr(at(c, S0, 0)).lw(c);
                c.g[V0][0] = Addr(at(c, V1, 0x20)).lw(c);
                c.g[V0][0] ^= 4;
                c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool BERewardInt_AreAllPeaksComplete(self, profile, c)` (0x001578a0):
/// true when every peak of every (up to three) rider is complete.
pub fn be_reward_int_are_all_peaks_complete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        c.g[A0][0] = c.g[S3][0];
        loop {
            c.cycles += 6;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            callee::BEREWARD_INT_IS_PEAK_COMPLETE.call(c, 0x0015_78e0)?;
            let complete = c.g[V0][0] != 0;
            c.g[S0][0] = sx(word(c, S0).wrapping_add(1));
            if !complete {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S3][0];
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 1;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_78d0);
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

/// `int WorldTriggerManager_CheckCrossing(self, trigger, state)`
/// (0x002b69b8): test a trigger against the world position and report a change.
pub fn world_trigger_manager_check_crossing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.swc1(20, frame.at(c, 0x20));
        c.g[A1][0] = Addr(at(c, A1, 0xc)).lw(c);
        let absent = c.g[A1][0] == 0;
        c.g[V0][0] = 0;
        if !absent {
            c.cycles += 10;
            c.g[V0][0] = Addr(at(c, A1, 0xc)).lw(c);
            c.lwc1(20, at(c, S0, 0x68));
            c.g[A0][0] = Addr(at(c, V0, 0xf0)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0xf4)).lw(c);
            c.g[A0][0] = add32(word(c, A1), word(c, A0));
            call_ptr(c, word(c, V1), 0x002b_69f0)?;
            c.c_lt_s(0, 20);
            c.g[V1][0] = c.fcc() as u64;
            if !c.fcc() {
                c.cycles += 1;
                c.g[V1][0] = 0;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(at(c, S0, 0x6c)).lw(c);
            let was = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            if !was {
                c.cycles += 1;
                c.g[A0][0] = c.g[V1][0] ^ 1;
            }
            c.cycles += 3;
            Addr(at(c, S0, 0x6c)).sw(c, c.g[V1][0]);
            c.g[V0][0] = c.g[A0][0];
            c.swc1(0, at(c, S0, 0x68));
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        c.lwc1(20, frame.at(c, 0x20));
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptList_ctor(self)` (0x00275d10): the script list constructor.
pub fn script_list_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[S1][0] = sx(word(c, S0).wrapping_add(0xc));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::SCRIPT_CMD_QUEUE_CTOR.call(c, 0x0027_5d30)?;
        c.g[A0][0] = c.g[S1][0];
        callee::SCRIPT_CMD_QUEUE_CLEAR.call(c, 0x0027_5d38)?;
        c.g[V0][0] = 0xffffffffffffffff;
        c.g[V1][0] = 0x1;
        Addr(at(c, S0, 0xbc)).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x4)).sw(c, c.g[V1][0]);
        for off in [0xa4, 0xa8, 0xc8, 0xb0, 0xac, 0xb8, 0xc0, 0xc4] {
            Addr(at(c, S0, off)).sw(c, 0);
        }
        Addr(at(c, S0, 0)).sw(c, 0);
        Addr(at(c, S0, 0xb4)).sw(c, 0);
        Addr(at(c, S0, 0x8)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cBXString_Right(dst, src, n)` (0x00319028): dst = the last `n`
/// characters of the BX string `src` (all of it when `n` is negative).
pub fn c_bx_string_right(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        if (c.g[A2][0] as i64) < 0 {
            c.cycles += 2;
            c.g[A2][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(at(c, A1, 0)).lw(c);
            c.g[A0][0] = Addr(word(c, V0).wrapping_sub(8)).lw(c);
            c.g[V1][0] = (c.g[A0][0] < c.g[A2][0]) as u64;
            if c.g[V1][0] != 0 {
                c.g[A2][0] = c.g[A0][0];
            }
        }
        c.cycles += 20;
        c.g[V0][0] = Addr(gp(c, 0xda0)).lw(c);
        c.g[A0][0] = c.g[A1][0];
        c.g[V1][0] = Addr(at(c, A1, 0)).lw(c);
        c.g[T0][0] = 0;
        Addr(at(c, SP, 0)).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = Addr(word(c, V1).wrapping_sub(8)).lw(c);
        c.g[A3][0] = sx(word(c, A3).wrapping_sub(word(c, A2)));
        callee::BX_STRING_ALLOC_COPY.call(c, 0x0031_9078)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        callee::BX_STRING_CTOR_COPY.call(c, 0x0031_9084)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x2;
        callee::BX_STRING_DTOR.call(c, 0x0031_9090)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.lq(16, frame.at(c, 0x20));
        frame.close(c);
        Ok(())
    })
}

/// `void Font_Load(font, name, flags)` (0x00391708): load the font file and
/// parse its glyphs, freeing the file image afterwards.
pub fn font_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = sx(0x490000);
        c.g[A2][0] = c.g[S0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x490000u32.wrapping_add(0x2f30));
        c.g[A0][0] = sx(word(c, S1).wrapping_add(0x64));
        callee::SPRINTF.call(c, 0x0039_173c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0] ^ 0x100;
        callee::FILE_LOAD.call(c, 0x0039_1748)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        callee::FONT_PARSE_GLYPHS.call(c, 0x0039_175c)?;
        if c.g[S0][0] == 0 {
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::MEM_FREE_2.call(c, 0x0039_176c)?;
            c.lq(16, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int RunList_Seek(list, pos)` (0x0026d4d8): find the run containing byte
/// `pos`, returning its index and the offset inside it.
pub fn run_list_seek(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = Addr(at(c, A0, 0)).lw(c);
    c.g[T0][0] = 0;
    let empty = (c.g[A2][0] as i64) <= 0;
    c.g[A3][0] = Addr(at(c, A0, 0xc)).lw(c);
    if !empty {
        c.cycles += 6;
        c.g[V1][0] = Addr(at(c, A3, 0)).lw(c);
        c.g[T2][0] = c.g[A2][0];
        c.g[A2][0] = sx(word(c, V1) & 0xfff);
        c.g[V0][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
        c.g[A3][0] = sx(word(c, A3).wrapping_add(8));
        // The loop's "ran out of entries" exit breaks all the way out to the
        // `v0 = 0` tail (skipping the +3 block); finding the run falls through.
        let mut exhausted = false;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[T1][0] = c.g[T2][0];
            loop {
                c.cycles += 4;
                c.g[T0][0] = sx(word(c, T0).wrapping_add(1));
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    exhausted = true;
                    break;
                }
                c.cycles += 6;
                c.g[V0][0] = Addr(at(c, A3, 0)).lw(c);
                c.g[V0][0] = sx(word(c, V0) & 0xfff);
                c.g[A2][0] = sx(word(c, A2).wrapping_add(word(c, V0)));
                c.g[V1][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
                let not_yet = c.g[V1][0] == 0;
                c.g[A3][0] = sx(word(c, A3).wrapping_add(8));
                if not_yet {
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_d508);
                    }
                    continue;
                }
                break;
            }
        }
        if exhausted {
            c.cycles += 2;
            let ra = word(c, RA);
            c.g[V0][0] = 0;
            c.pc = ra;
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T2][0] as i64)) as u64;
        let found = c.g[V0][0] != 0;
        c.g[V0][0] = sx(word(c, A2).wrapping_sub(word(c, A1)));
        if found {
            c.cycles += 4;
            Addr(at(c, A0, 0x4)).sw(c, c.g[T0][0]);
            Addr(at(c, A0, 0x8)).sw(c, c.g[V0][0]);
            let ra = word(c, RA);
            c.g[V0][0] = 0x1;
            c.pc = ra;
            return;
        }
    }
    c.cycles += 2;
    let ra = word(c, RA);
    c.g[V0][0] = 0;
    c.pc = ra;
}

/// `int TagFieldGetNumber(const char* tag, int defval)` (0x003ecf30): parse a
/// signed decimal number from a tag field.
pub fn tag_field_get_number(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = c.g[A0][0];
    let empty = c.g[A2][0] == 0;
    c.g[V0][0] = c.g[A1][0];
    if !empty {
        c.cycles += 4;
        c.g[V1][0] = Addr(at(c, A2, 0)).lb(c);
        c.g[V0][0] = 0x2b;
        let not_plus = c.g[V1][0] != c.g[V0][0];
        c.g[T0][0] = 0x1;
        if not_plus {
            c.cycles += 3;
            c.g[V0][0] = 0x2d;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A1][0] = Addr(at(c, A2, 0)).lbu(c);
            } else {
                c.cycles += 2;
                c.g[A2][0] = sx(word(c, A2).wrapping_add(1));
                c.g[T0][0] = 0xffffffffffffffff;
                c.cycles += 1;
                c.g[A1][0] = Addr(at(c, A2, 0)).lbu(c);
            }
        } else {
            c.cycles += 2;
            c.g[A2][0] = sx(word(c, A2).wrapping_add(1));
            c.cycles += 1;
            c.g[A1][0] = Addr(at(c, A2, 0)).lbu(c);
        }
        c.cycles += 4;
        c.g[V0][0] = sx(word(c, A1).wrapping_sub(0x30));
        c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
        let not_digit = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !not_digit {
            c.cycles += 1;
            c.g[A3][0] = 0xa;
            loop {
                c.cycles += 8;
                c.g[A0][0] = sx(word(c, A1) & 0xf);
                c.mult(0, V1, V1, A3);
                c.g[A2][0] = sx(word(c, A2).wrapping_add(1));
                c.g[A1][0] = Addr(at(c, A2, 0)).lbu(c);
                c.g[V0][0] = sx(word(c, A1).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                let digit = c.g[V0][0] != 0;
                c.g[V1][0] = sx(word(c, V1).wrapping_add(word(c, A0)));
                if !digit {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003e_cf80);
                }
            }
        }
        c.cycles += 1;
        c.mult(0, V0, T0, V1);
    }
    c.cycles += 2;
    let ra = word(c, RA);
    c.pc = ra;
}

/// `int UIEngine_LoadPackage(engine, name)` (0x00397c40): load a UI package
/// file, store its handle and load its textures.
pub fn ui_engine_load_package(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = sx(0x4a0000);
        frame.save_ra(c, 0x70);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = sx(0x4a0000u32.wrapping_add(0x4770));
        callee::SPRINTF.call(c, 0x0039_7c6c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0;
        callee::FILE_LOAD.call(c, 0x0039_7c78)?;
        Addr(at(c, S0, 0x4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(word(c, S0).wrapping_add(0x50));
        c.g[A1][0] = Addr(at(c, V0, 0x8)).lw(c);
        c.g[A1][0] = sx(word(c, V0).wrapping_add(word(c, A1)));
        callee::UI_ENGINE_STORE_WORD.call(c, 0x0039_7c8c)?;
        c.g[V0][0] = Addr(at(c, S0, 0x4)).lw(c);
        c.g[A0][0] = sx(word(c, S0).wrapping_add(0x58));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = Addr(at(c, V0, 0x10)).lw(c);
        c.g[A2][0] = sx(word(c, V0).wrapping_add(word(c, A2)));
        callee::UI_ENGINE_LOAD_TEXTURES.call(c, 0x0039_7ca4)?;
        c.lq(16, frame.at(c, 0x90));
        c.g[V0][0] = 0x1;
        c.lq(17, frame.at(c, 0x80));
        c.g[RA][0] = c.ld(frame.at(c, 0x70));
        frame.close(c);
        Ok(())
    })
}

/// `void* FlexBridge_SpringsLoad(self, stream)` (0x00353e80): load the spring
/// nodes and tell the owner.
pub fn flex_bridge_springs_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x24;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(at(c, S1, 0)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x10)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0x14)).lw(c);
        c.g[A0][0] = add32(word(c, S1), word(c, A0));
        call_ptr(c, word(c, V0), 0x0035_3eb4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::FLEX_BRIDGE_ALLOC_SPRING_NODES.call(c, 0x0035_3ebc)?;
        c.g[V0][0] = Addr(at(c, S0, 0)).lw(c);
        c.g[A2][0] = 0x50;
        c.g[V1][0] = Addr(at(c, S1, 0)).lw(c);
        c.mult(0, V0, V0, A2);
        c.g[A1][0] = Addr(at(c, S0, 0x24)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x10)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0x14)).lw(c);
        c.g[A2][0] = c.lo[0];
        c.g[A0][0] = add32(word(c, S1), word(c, A0));
        call_ptr(c, word(c, V0), 0x0035_3ee4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMap_Init(self)` (0x00208498): create the map screen and
/// initialise the overlay's event select.
pub fn ov_screen_map_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x4a0000);
        c.g[A0][0] = sx(0x4a0000u32.wrapping_add(0x26e8));
        c.g[S0][0] = Addr(at(c, S1, 0x10)).lw(c);
        callee::GET_HASH_VALUE_32.call(c, 0x0020_84bc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME.call(c, 0x0020_84d0)?;
        let absent = c.g[V0][0] == 0;
        Addr(at(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_SCREEN_PLAY_FRAME.call(c, 0x0020_84e8)?;
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        callee::C_OV_STATE_OVERLAY_INIT.call(c, 0x0020_84f0)?;
        c.g[A1][0] = Addr(at(c, S1, 0x40)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx(word(c, S1).wrapping_add(0x9c));
        callee::EVENT_SELECT_INIT.call(c, 0x0020_8500)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFlagManager_GroupCtor(self)` (0x0034af38): construct a flag group
/// and clear its 60-entry key table.
pub fn c_flag_manager_group_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::C_FLAG_MANAGER_GROUP_INIT_KEY.call(c, 0x0034_af4c)?;
        c.g[V1][0] = sx(0x490000);
        Addr(at(c, S0, 0x60)).sw(c, 0);
        c.g[V1][0] = sx(0x490000u32.wrapping_sub(0x408));
        Addr(at(c, S0, 0x74)).sw(c, 0);
        Addr(at(c, S0, 0x184)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(word(c, S0).wrapping_add(0x180));
        c.g[V1][0] = 0x3b;
        loop {
            c.cycles += 7;
            Addr(at(c, V0, 0)).sw(c, 0);
            c.g[V1][0] = sx(word(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(word(c, V0).wrapping_sub(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_af68);
                }
                continue;
            }
            c.cycles += 6;
            c.g[V0][0] = Addr(gp(c, 0xf20)).lw(c);
            c.g[V1][0] = sx(word(c, V0).wrapping_add(1));
            Addr(at(c, S0, 0x90)).sw(c, c.g[V0][0]);
            c.g[A0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
            let keep = c.g[A0][0] != 0;
            Addr(gp(c, 0xf20)).sw(c, c.g[V1][0]);
            if !keep {
                c.cycles += 1;
                Addr(gp(c, 0xf20)).sw(c, 0);
            }
            c.cycles += 5;
            c.g[V0][0] = c.g[S0][0];
            c.g[RA][0] = c.ld(frame.at(c, 0));
            c.lq(16, frame.at(c, 0x10));
            frame.close(c);
            break;
        }
        Ok(())
    })
}

/// `void GraphicsMan_SetColorMask(man, mask)` (0x00369610): invert the colour
/// mask into four words and set the two blend words.
pub fn graphics_man_set_color_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(word(c, A3).wrapping_add(0xed8));
        c.g[V1][0] = c.g[A1][0];
        c.g[A2][0] = 0x3;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(at(c, V1, 0)).lw(c);
            c.g[A2][0] = sx(word(c, A2).wrapping_sub(1));
            c.g[V1][0] = sx(word(c, V1).wrapping_add(4));
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            Addr(at(c, A0, 0)).sw(c, c.g[V0][0]);
            let more = (c.g[A2][0] as i64) >= 0;
            c.g[A0][0] = sx(word(c, A0).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0036_9628);
                }
                continue;
            }
            c.cycles += 3;
            c.g[A0][0] = 0;
            c.g[A2][0] = 0x7fff;
            c.g[V1][0] = sx(word(c, A3).wrapping_add(0xee8));
            loop {
                c.cycles += 7;
                Addr(at(c, V1, 0)).sh(c, c.g[A2][0]);
                c.g[A0][0] = sx(word(c, A0).wrapping_add(1));
                c.g[V0][0] = (c.g[A0][0] < 2u64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx(word(c, V1).wrapping_add(2));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_9650);
                    }
                    continue;
                }
                c.cycles += 8;
                c.g[V0][0] = Addr(at(c, A3, 0x10d8)).lw(c);
                c.g[A0][0] = Addr(at(c, V0, 0x1f0)).lh(c);
                c.g[V1][0] = Addr(at(c, V0, 0x1f4)).lw(c);
                c.g[A0][0] = add32(word(c, A3), word(c, A0));
                call_ptr(c, word(c, V1), 0x0036_9680)?;
                c.g[RA][0] = c.ld(frame.at(c, 0));
                frame.close(c);
                break;
            }
            break;
        }
        Ok(())
    })
}

/// `void OVScreenBCFail_Init(self)` (0x001f76b8): create the BC-fail screen
/// and initialise the overlay.
pub fn ov_screen_bc_fail_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(0x470000);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx(0x470000u32.wrapping_sub(0xb00));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::GET_HASH_VALUE_32.call(c, 0x001f_76dc)?;
        c.g[A0][0] = c.g[S0][0];
        Addr(at(c, S1, 0xc)).sw(c, c.g[V0][0]);
        c.g[S0][0] = Addr(at(c, S1, 0x10)).lw(c);
        callee::GET_HASH_VALUE_32.call(c, 0x001f_76ec)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME.call(c, 0x001f_7700)?;
        let absent = c.g[V0][0] == 0;
        Addr(at(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_SCREEN_PLAY_FRAME.call(c, 0x001f_7718)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        callee::C_OV_STATE_OVERLAY_INIT.call(c, 0x001f_7720)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenProfileSavePDA_Init(self)` (0x002111a0): create the profile
/// save PDA screen, open the template and label its slots.
pub fn ov_screen_profile_save_pda_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x470000);
        c.g[A0][0] = sx(0x470000u32.wrapping_add(0x1d50));
        c.g[S0][0] = Addr(at(c, S1, 0x10)).lw(c);
        callee::GET_HASH_VALUE_32.call(c, 0x0021_11c4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME.call(c, 0x0021_11d8)?;
        let absent = c.g[V0][0] == 0;
        Addr(at(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_SCREEN_PLAY_FRAME.call(c, 0x0021_11f0)?;
        }
        c.cycles += 11;
        callee::OV_SCREEN_PDA_TEMPLATE_OPEN.call(c, 0x0021_11f8)?;
        c.g[A0][0] = c.g[S1][0];
        callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS.call(c, 0x0021_1200)?;
        c.g[A0][0] = c.g[S1][0];
        callee::C_OV_STATE_OVERLAY_INIT.call(c, 0x0021_1208)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionMan_BeginEventSwitch(self)` (0x0022df50): request the section
/// state and overlay, then tell the interface to begin the switch.
pub fn section_man_begin_event_switch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0xb;
        c.g[A0][0] = Addr(at(c, S0, 0x1b0)).lw(c);
        callee::GAME_MODULE_REQUEST_STATE.call(c, 0x0022_df74)?;
        callee::CBE_GET_BE.call(c, 0x0022_df7c)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        callee::CBE_GET_INTERFACE.call(c, 0x0022_df88)?;
        c.g[A1][0] = Addr(at(c, S0, 0x1bc)).lw(c);
        c.g[V1][0] = sx(0x530000);
        c.g[A0][0] = 0x1;
        Addr(at(c, V1, 0x5c08)).sw(c, c.g[A1][0]);
        Addr(gp(c, -0x1f38)).sw(c, c.g[A0][0]);
        c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x8)).lh(c);
        c.g[A1][0] = Addr(at(c, V1, 0xc)).lw(c);
        c.g[A0][0] = add32(word(c, V0), word(c, A0));
        call_ptr(c, word(c, A1), 0x0022_dfb0)?;
        c.g[A0][0] = Addr(at(c, S0, 0x1b0)).lw(c);
        c.g[A1][0] = 0xa;
        callee::GAME_MODULE_REQUEST_OVERLAY_STATE.call(c, 0x0022_dfbc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenPDATemplate_Open(void)` (0x0020d110): once per process, add
/// the PDA template background state.
pub fn ov_screen_pda_template_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(gp(c, -0x9e4)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let already = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !already {
            c.cycles += 20;
            c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[A1][0] = sx(0x470000);
            c.g[A1][0] = sx(0x470000u32.wrapping_add(0x1a10));
            c.g[A0][0] = 0x48;
            c.g[V0][0] = Addr(at(c, V1, 0x84)).lw(c);
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            c.g[S1][0] = Addr(at(c, V0, 0x48)).lw(c);
            callee::MEM_ALLOC_NAMED.call(c, 0x0020_d14c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            callee::UI_STATE_CTOR.call(c, 0x0020_d15c)?;
            c.g[V1][0] = sx(0x470000);
            c.g[A0][0] = sx(word(c, S1).wrapping_add(0x18));
            c.g[V1][0] = sx(0x470000u32.wrapping_add(0x43f0));
            c.g[A1][0] = c.g[S0][0];
            Addr(at(c, S0, 0x8)).sw(c, c.g[V1][0]);
            Addr(gp(c, -0x9e4)).sw(c, c.g[S0][0]);
            callee::UI_STATE_MGR_ADD_BACKGROUND_STATE.call(c, 0x0020_d178)?;
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
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, Rng, DATA, GAME_GP, RA as SENTINEL_RA};
    use ssx3_ee_rt::funcs::GuestFn;

    /// The zeroed BELibrary dummy and the world.
    const BE: u32 = DATA + 0x1_0000;
    const W: u32 = DATA + 0x1_4000;
    /// The dlmalloc arena the game's `operator new`/`EA_MemAlloc` uses.
    const MEM_HEAP: u32 = 0x004f_f250;
    /// Tiny translated functions used as virtual-method targets.
    const RET_ZERO: u32 = 0x001a_8828;
    const RET_ONE: u32 = 0x001c_5dd0;
    const RET_TWO: u32 = 0x0014_5854;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    fn wh(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }
    fn wb(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }
    fn put_str(c: &mut Cpu, a: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            wb(c, a + i as u32, *b);
        }
        wb(c, a + s.len() as u32, 0);
    }
    /// A vtable slot `{ short delta = 0; void (*fn)(); }`.
    fn ret_slot(c: &mut Cpu, vt: u32, off: u32, target: u32) {
        wh(c, vt + off, 0);
        w(c, vt + off + 4, target);
    }

    /// EAFile's request and file-handle pools (counts `0x519ab0`/`0x519ab4`,
    /// tables `0x519ad8`/`0x519adc`, entries 0x30/0x110 bytes): with no free
    /// entry the allocators spin forever (`0x3de768`).
    fn eafile_pools(c: &mut Cpu) {
        for (count, table, at, size) in [
            (0x0051_9ab0, 0x0051_9ad8, 0xf_0000, 0x30),
            (0x0051_9ab4, 0x0051_9adc, 0xf_1000, 0x110),
        ] {
            let base = DATA + at;
            c.bus.write32(count, 4);
            c.bus.write32(table, base);
            for i in 0..size {
                c.bus.write32(base + 4 * i, 0);
            }
        }
    }

    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = addr;
        f(c);
        // A setup call may unwind through a fake hook; that is fine for setup,
        // the port under test is compared separately.
        c.g[V0][0]
    }

    fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    /// The engine's size-class pool allocator, enabled through `gp + 0xdd8`.
    fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes;
            for n in 0..16u32 {
                let node = base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// A real heap block of `size` bytes.
    fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0, 0],
        ) as u32
    }

    /// Scrambled registers, the fake hooks, a game-like `gp`, the BE singleton
    /// and a world with no riders.
    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x1efc), 0);
        w(c, GAME_GP.wrapping_sub(0x848), W);
        let game = W + 0x1000;
        w(c, W + 0x84, game);
        let race = W + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
        r
    }

    /// [`base`] plus a working pool allocator and dlmalloc arena.
    fn alloc_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = base(c, seed, salt, calls);
        pool_allocator(c);
        heap_init(c);
        r
    }

    // ------------------------------------------------------------------ front end

    #[test]
    fn fe_set_online_mode_matches() {
        check(ee::s03::f_001a86e8__generated, fe_set_online_mode, 24, &|c, seed| {
            let _ = base(c, seed, 0x1a86_e8, 6);
            c.g[A0][0] = if seed % 3 == 0 {
                0
            } else {
                (1 + seed % 2) as u64
            };
            match seed % 4 {
                0 => w(c, GAME_GP.wrapping_add(0x238), 0),
                _ => {
                    let bm = DATA + 0x1000;
                    w(c, GAME_GP.wrapping_add(0x238), bm);
                    w(c, bm + 0xc8, (seed % 3) as u32);
                    w(c, bm + 0xd8, ((seed / 3) % 2) as u32);
                    w(c, bm + 0xe0, ((seed / 5) % 2) as u32);
                }
            }
            w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x2000);
        });
    }

    #[test]
    fn net_fe_show_player_conn_quality_matches() {
        check(
            ee::s03::f_001afd88__generated,
            net_fe_show_player_conn_quality,
            16,
            &|c, seed| {
                let mut r = base(c, seed, 0x1afd_88, 6);
                let screen = DATA;
                let player = DATA + 0x1000;
                w(c, player + 0xc, r.next_u32() % 8);
                if seed % 2 == 0 {
                    w(c, screen + 0x3c, 0);
                    w(c, screen + 0x38, 0);
                } else {
                    let node = DATA + 0x2000;
                    w(c, screen + 0x3c, 1);
                    w(c, screen + 0x38, DATA + 0x1800);
                    w(c, DATA + 0x1800, node);
                    w(c, node + 0x38, 0);
                }
                c.g[A0][0] = screen as u64;
                c.g[A1][0] = player as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_lobby_menu_on_net_result_matches() {
        check(
            ee::s03::f_001bd848__generated,
            c_fe_state_lobby_menu_on_net_result,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x1bd8_48, 8);
                let msg = DATA + 0x1800;
                w(c, DATA + 0x1c, if seed % 2 == 0 { 0 } else { 6 });
                let result = match seed % 4 {
                    0 => 0x828,
                    1 => 0x829,
                    2 => 0x700,
                    _ => [0x828, 0x829, 0x700][(seed as usize / 4) % 3],
                };
                let obj = DATA + 0x1000;
                w(c, DATA + 0x6a0, obj);
                w(c, obj + 0x18, result);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (0xdc + seed % 0x3b) as u64;
                c.g[A2][0] = msg as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_net_quick_play_popup_ctor_matches() {
        check(
            ee::s03::f_001c0f40__generated,
            c_fe_state_net_quick_play_popup_ctor,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x1c0f_40, 2);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_net_ssx_world_init_matches() {
        check(
            ee::s03::f_001c1c90__generated,
            c_fe_state_net_ssx_world_init,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x1c1c_90, 4);
                let engine = DATA + 0x1000;
                w(c, DATA + 0x10, engine);
                w(c, engine, DATA + 0x1100);
                w(c, engine + 4, 0);
                w(c, DATA + 0x1100, 0);
                let lobby = DATA + 0x2000;
                w(c, GAME_GP.wrapping_sub(0xc8), lobby);
                w(c, lobby, (seed % 2) as u32);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn tag_field_get_string_unescaped_matches() {
        check(
            ee::s06::f_00259310__generated,
            tag_field_get_string_unescaped,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x2593_10, 2);
                let tag = DATA + 0x1000;
                let dst = DATA + 0x2000;
                let text: &[u8] = match seed % 4 {
                    0 => b"`abcdef",
                    1 => b"abcdef",
                    2 => b"",
                    _ => b"`",
                };
                put_str(c, tag, text);
                c.g[A0][0] = tag as u64;
                c.g[A1][0] = dst as u64;
                c.g[A2][0] = (4 + seed % 13) as u64;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
            },
        );
    }

    #[test]
    fn script_engine_unload_world_scripts_matches() {
        check(
            ee::s07::f_00278cd8__generated,
            script_engine_unload_world_scripts,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x278c_d8, 6);
                let self_ = DATA;
                w(
                    c,
                    self_ + 0x538,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                let blk = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
                w(c, self_ + 0x53c, blk);
                w(c, self_ + 0x620, 1 + seed % 4 as u32);
                match seed % 4 {
                    0 => w(c, self_ + 0x638, 0),
                    _ => w(c, self_ + 0x638, 0),
                }
                w(c, self_ + 0x63c, 0);
                c.g[A0][0] = self_ as u64;
            },
        );
    }

    #[test]
    fn script_engine_play_script_instance_matches() {
        check(
            ee::s07::f_00279720__generated,
            script_engine_play_script_instance,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x2797_20, 4);
                let self_ = DATA;
                let obj = DATA + 0x1000;
                w(c, self_ + 0x2c0 + 0x20, (seed % 2) as u32);
                w(c, self_ + 0x2c0 + 0x24, 0);
                w(c, self_ + 0x2c0 + 0x4, (seed % 4) as u32);
                w(c, obj, 0);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (r.next_u32() % 8) as u64;
                c.g[A2][0] = (r.next_u32() % 8) as u64;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
            },
        );
    }

    #[test]
    fn script_audio_obj_activate_matches() {
        check(
            ee::s07::f_00280730__generated,
            script_audio_obj_activate,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2807_30, 6);
                let self_ = DATA;
                w(
                    c,
                    self_ + 0x8,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                w(c, DATA + 0x1000 + 0x8, 0);
                w(c, DATA + 0x1000 + 0x1c, 0);
                w(c, GAME_GP.wrapping_sub(0x84c), 0);
                c.g[A0][0] = self_ as u64;
            },
        );
    }

    #[test]
    fn menu_sys_group_item_on_focus_matches() {
        check(
            ee::s09::f_002cfe78__generated,
            menu_sys_group_item_on_focus,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2cfe_78, 4);
                let self_ = DATA;
                let page = DATA + 0x18;
                w(c, self_, 0);
                w(c, self_ + 0x18, 0);
                let mode = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + (seed % 7) as u64,
                };
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = mode;
                let _ = page;
            },
        );
    }

    #[test]
    fn ai_reset_rider_stats_matches() {
        check(ee::s00::f_0012b180__generated, ai_reset_rider_stats, 24, &|c, seed| {
            let _ = base(c, seed, 0x12b1_80, 2);
            let self_ = DATA;
            let n = (seed % 4) as u32;
            w(c, self_ + 0x78, n);
            for i in 0..n {
                let rider = DATA + 0x1000 + 0x100 * i;
                w(c, self_ + 0x28 + 4 * i, rider);
                w(c, rider + 0x790, DATA + 0x2000 + 0x100 * i);
            }
            c.g[A0][0] = self_ as u64;
        });
    }

    #[test]
    fn rider_board_press_sub_settle_slew_matches() {
        use crate::difftest::put_words;
        check(
            ee::s00::f_001313a8__generated,
            rider_board_press_sub_settle_slew,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x1313_a8, 0);
                let st = DATA + 0x1000;
                w(c, DATA + 0x14, st);
                let slew = match seed % 4 {
                    0 => (-1.0f32).to_bits(),
                    1 => 0.0f32.to_bits(),
                    2 => 1.0f32.to_bits(),
                    _ => r.edge_f32(),
                };
                w(c, st + 0x280, slew);
                w(c, st + 0x300, r.edge_f32());
                put_words(
                    c,
                    GAME_GP.wrapping_sub(0x7600),
                    &[r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn path_arrow_ctor_matches() {
        check(ee::s12::f_00372aa0__generated, path_arrow_ctor, 12, &|c, seed| {
            let _ = base(c, seed, 0x372a_a0, 0);
            w(c, GAME_GP.wrapping_add(0x114c), (seed % 2) as u32);
            for (i, off) in [0x1118u32, 0x111c, 0x1120, 0x1124].into_iter().enumerate() {
                w(c, GAME_GP.wrapping_add(off), 0x11_0000 + i as u32);
            }
            c.g[A0][0] = (DATA + 0x1000) as u64;
        });
    }

    // ------------------------------------------------------------- memcard

    fn mc_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = alloc_base(c, seed, salt, 4);
        let self_ = DATA;
        let obj = DATA + 0x1000;
        let vtable = DATA + 0x1200;
        w(c, self_ + 0x434, obj);
        w(c, obj, vtable);
        ret_slot(
            c,
            vtable,
            0x58,
            if seed % 4 == 0 { RET_ONE } else { RET_ZERO },
        );
        w(
            c,
            self_ + 0x84,
            match seed % 4 {
                1 => RET_ONE,
                _ => RET_ZERO,
            },
        );
        w(
            c,
            self_ + 0x6c,
            if seed % 4 == 3 { RET_ONE } else { RET_ZERO },
        );
        c.g[A0][0] = self_ as u64;
    }

    #[test]
    fn mc_manager_state17_matches() {
        check(ee::s06::f_0023f258__generated, mc_manager_state17, 12, &|c, seed| {
            mc_setup(c, seed, 0x23f2_58);
        });
    }

    #[test]
    fn mc_manager_state30_matches() {
        check(ee::s06::f_00240f30__generated, mc_manager_state30, 12, &|c, seed| {
            mc_setup(c, seed, 0x240f_30);
        });
    }

    #[test]
    fn mc_manager_end_progress_matches() {
        check(
            ee::s06::f_00241180__generated,
            mc_manager_end_progress,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2411_80, 4);
                let self_ = DATA;
                let obj = DATA + 0x1000;
                let widget = DATA + 0x2000;
                let vtable = DATA + 0x1200;
                w(c, self_ + 0x118, (seed % 2) as u32);
                w(c, self_ + 0x434, obj);
                w(c, obj, vtable);
                ret_slot(c, vtable, 0x160, RET_ZERO);
                w(c, self_ + 0x748, widget);
                ret_slot(c, widget, 0x8, RET_ZERO);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    #[test]
    fn mc_manager_on_card_scanned_matches() {
        check(
            ee::s06::f_00241fd0__generated,
            mc_manager_on_card_scanned,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x241f_d0, 4);
                let self_ = DATA;
                let obj = DATA + 0x1000;
                let widget = DATA + 0x2000;
                let vtable = DATA + 0x1200;
                w(
                    c,
                    self_ + 0xac,
                    match seed % 3 {
                        0 => RET_ONE,
                        _ => RET_ZERO,
                    },
                );
                w(c, self_ + 0x434, obj);
                w(c, obj, vtable);
                ret_slot(
                    c,
                    vtable,
                    0x58,
                    if seed % 2 == 0 { RET_ZERO } else { RET_ONE },
                );
                w(c, self_ + 0x748, widget);
                ret_slot(c, widget, 0x8, RET_ZERO);
                c.g[A0][0] = self_ as u64;
            },
        );
    }

    // ------------------------------------------------------------ front end II

    #[test]
    fn c_fe_popup_confirm_rebuild_matches() {
        check(
            ee::s03::f_001c5dd8__generated,
            c_fe_popup_confirm_rebuild,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x1c5d_d8, 6);
                let self_ = DATA;
                let child = DATA + 0x1000;
                let vtable = DATA + 0x1200;
                w(c, self_ + 0x2bc, (seed % 2) as u32);
                w(c, self_ + 0x8, child);
                ret_slot(c, child, 0x30, RET_ZERO);
                let _ = vtable;
                w(c, self_ + 0x278, 0x1122_3344);
                w(c, self_ + 0x27c, 0x5566_7788);
                w(c, self_ + 0x280, 0x99aa_bbcc);
                w(c, self_ + 0xbc, 0);
                c.g[A0][0] = self_ as u64;
            },
        );
    }

    #[test]
    fn c_ui_state_buy_popup_set_song_credit_params_matches() {
        check(
            ee::s04::f_001ca900__generated,
            c_ui_state_buy_popup_set_song_credit_params,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x1ca9_00, 2);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 16) as u64;
                c.g[A2][0] = (r.next_u32() % 16) as u64;
            },
        );
    }

    #[test]
    fn w_script_process_list_dtor_matches() {
        check(
            ee::s10::f_0030b068__generated,
            w_script_process_list_dtor,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x30b0_68, 4);
                let self_ = DATA;
                let n0 = DATA + 0x1000;
                let n1 = DATA + 0x1100;
                if seed % 2 == 0 {
                    w(c, self_, 0);
                } else {
                    w(c, self_, n0);
                    w(c, n0 + 0x18, if seed % 4 == 3 { n1 } else { 0 });
                    w(c, n1 + 0x18, 0);
                    for n in [n0, n1] {
                        w(c, n + 0x5c, DATA + 0x2000);
                        ret_slot(c, DATA + 0x2000, 0x8, RET_ZERO);
                    }
                }
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    // -------------------------------------------------------------- modifiers

    #[test]
    fn mod_obj_add_parent_modifier_matches() {
        check(
            ee::s11::f_00355978__generated,
            mod_obj_add_parent_modifier,
            8,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x3559_78, 6);
                let self_ = DATA;
                let vtable = DATA + 0x1200;
                w(c, self_ + 0xc, 0);
                w(c, self_, vtable);
                ret_slot(c, vtable, 0x188, RET_ZERO);
                ret_slot(c, vtable, 0x180, RET_ZERO);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x1400) as u64;
                c.g[A3][0] = (DATA + 0x1800) as u64;
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn mod_obj_add_roller_modifier_matches() {
        check(
            ee::s11::f_00355db8__generated,
            mod_obj_add_roller_modifier,
            8,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x355d_b8, 6);
                let self_ = DATA;
                let vtable = DATA + 0x1200;
                w(c, self_ + 0xc, 0);
                w(c, self_, vtable);
                ret_slot(c, vtable, 0x188, RET_ZERO);
                ret_slot(c, vtable, 0x180, RET_ZERO);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x1400) as u64;
                c.g[A3][0] = (DATA + 0x1800) as u64;
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn mod_obj_v21_matches() {
        check(ee::s11::f_00356c48__generated, mod_obj_v21, 16, &|c, seed| {
            let _ = base(c, seed, 0x356c_48, 4);
            let self_ = DATA;
            let obj = DATA + 0x1000;
            let inner = DATA + 0x1100;
            let vtable = DATA + 0x1200;
            match seed % 4 {
                0 => {
                    w(c, self_ + 0x1c, 0);
                }
                1 => {
                    w(c, self_ + 0x1c, obj);
                    w(c, obj, 0);
                }
                2 => {
                    w(c, self_ + 0x1c, obj);
                    w(c, obj, inner);
                    w(c, inner, vtable);
                    ret_slot(c, vtable, 0xd0, RET_ONE);
                }
                _ => {
                    w(c, self_ + 0x1c, obj);
                    w(c, obj, inner);
                    w(c, inner, vtable);
                    ret_slot(c, vtable, 0xd0, RET_TWO);
                }
            }
            c.g[A0][0] = self_ as u64;
            c.g[A1][0] = (DATA + 0x1800) as u64;
        });
    }

    #[test]
    fn magnet_modifier_save_matches() {
        check(ee::s11::f_003576d0__generated, magnet_modifier_save, 12, &|c, seed| {
            let _ = base(c, seed, 0x3576_d0, 4);
            let self_ = DATA;
            let stream = DATA + 0x1000;
            let vtable = DATA + 0x1200;
            let nested = DATA + 0x1400;
            w(c, self_ + 0xa0, 0);
            w(c, stream, vtable);
            w(c, nested, vtable);
            ret_slot(c, vtable, 0x28, RET_ZERO);
            ret_slot(c, vtable, 0x8, RET_ZERO);
            ret_slot(c, vtable, 0xc, RET_ZERO);
            let _ = nested;
            c.g[A0][0] = self_ as u64;
            c.g[A1][0] = stream as u64;
        });
    }

    #[test]
    fn multi_particle_dtor_matches() {
        check(ee::s11::f_00357fa0__generated, multi_particle_dtor, 12, &|c, seed| {
            let _ = alloc_base(c, seed, 0x357f_a0, 4);
            let self_ = DATA;
            let a8 = if seed % 3 == 0 { 0 } else { alloc(c, 0x40) };
            let ac = if seed % 2 == 0 { 0 } else { alloc(c, 0x40) };
            w(c, self_ + 0x8, a8);
            w(c, self_ + 0xc, ac);
            let owner = if seed % 4 == 0 { 0 } else { DATA + 0x1000 };
            w(c, self_ + 0x10, owner);
            if owner != 0 {
                w(c, owner + 0x18c, DATA + 0x1200);
                ret_slot(c, DATA + 0x1200, 0x8, RET_ZERO);
            }
            c.g[A0][0] = self_ as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }

    // ---------------------------------------------------------------- gallery

    #[test]
    fn c_fe_state_reward_gallery_update_scroll_arrows_matches() {
        check(
            ee::s04::f_001d10a8__generated,
            c_fe_state_reward_gallery_update_scroll_arrows,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x1d10_a8, 4);
                let self_ = DATA;
                let left = DATA + 0x1000;
                let right = DATA + 0x1100;
                let vtable = DATA + 0x1200;
                w(c, self_ + 0x91c, if seed % 2 == 0 { 0 } else { left });
                w(c, self_ + 0x920, if seed % 3 == 0 { 0 } else { right });
                for o in [left, right] {
                    w(c, o + 8, vtable);
                }
                ret_slot(c, vtable, 0x48, RET_ZERO);
                w(c, self_ + 0x64, r.next_u32() % 8);
                w(c, self_ + 0x58, r.next_u32() % 8);
                w(c, self_ + 0x60, r.next_u32() % 16);
                c.g[A0][0] = self_ as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_video_player_handle_input_matches() {
        check(
            ee::s04::f_001d2518__generated,
            c_fe_state_video_player_handle_input,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x1d25_18, 4);
                let self_ = DATA;
                let input = DATA + 0x1000;
                let vtable = DATA + 0x1200;
                w(c, self_ + 0x270, (seed % 2) as u32);
                w(c, self_ + 0x27c, 0);
                w(c, input + 8, vtable);
                ret_slot(
                    c,
                    vtable,
                    0x28,
                    if seed % 3 == 0 { RET_ONE } else { RET_ZERO },
                );
                ret_slot(
                    c,
                    vtable,
                    0x30,
                    if seed % 4 == 3 { RET_ONE } else { RET_ZERO },
                );
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = input as u64;
            },
        );
    }

    fn fill_rows_test(gen: GuestFn, hand: GuestFn, salt: u64, row_target: u32) {
        check(gen, hand, 8, &|c, seed| {
            let _ = alloc_base(c, seed, salt, 4);
            let self_ = DATA;
            let profile = DATA + 0x1000;
            w(c, self_ + 0x5c, profile);
            wb(c, profile + 0x98, (seed % 4) as u8);
            w(c, self_ + 0x958, (seed % 10) as u32);
            let _ = row_target;
            c.g[A0][0] = self_ as u64;
        });
    }

    #[test]
    fn c_fe_state_char_equip_fill_equip_rows_matches() {
        fill_rows_test(
            ee::s03::f_0019b518__generated,
            c_fe_state_char_equip_fill_equip_rows,
            0x19b5_18,
            0x0019_b7e0,
        );
    }

    #[test]
    fn c_fe_state_char_equip_fill_buy_rows_matches() {
        fill_rows_test(
            ee::s03::f_0019b598__generated,
            c_fe_state_char_equip_fill_buy_rows,
            0x19b5_98,
            0x0019_ba60,
        );
    }

    // ----------------------------------------------------------------- streams

    #[test]
    fn stream_array_create_stream_matches() {
        check(
            ee::s08::f_002aa7f0__generated,
            stream_array_create_stream,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2aa7_f0, 4);
                let self_ = DATA;
                w(c, self_ + 0x8, DATA + 0x2000);
                w(c, self_, DATA + 0x2100);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn w_script_man_spawn_processes_matches() {
        check(
            ee::s10::f_00308be0__generated,
            w_script_man_spawn_processes,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x308b_e0, 8);
                eafile_pools(c);
                let self_ = DATA;
                // The wrappers do `lw(lw(man+4)+4) + section` before reading
                // the cache; section 0 keeps the cache pointer aligned.
                let man = DATA + 0x1000;
                let mid = DATA + 0x1100;
                let cache = DATA + 0x1200;
                let table = DATA + 0x1300;
                w(c, self_ + 0x28c, man);
                w(c, man + 4, mid);
                w(c, mid + 4, cache);
                wh(c, cache, 3);
                w(c, cache + 8, table);
                let n = (seed % 4) as u32;
                w(c, table + 0x28, n);
                w(c, table + 0x2c, DATA + 0x3000);
                for i in 0..n {
                    w(c, DATA + 0x3000 + 0x40 * i, 0);
                }
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn mem_card_state_base_alloc_read_buffer_matches() {
        check(
            ee::s04::f_001d9948__generated,
            mem_card_state_base_alloc_read_buffer,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x1d99_48, 4);
                let self_ = DATA;
                w(c, self_ + 0x228, (seed % 2) as u32);
                let old = if seed % 3 == 0 { 0 } else { alloc(c, 0x100) };
                w(c, self_ + 0x1d4, old);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = 0x100;
            },
        );
    }

    #[test]
    fn rider_on_split_marker_matches() {
        check(
            ee::s00::f_0010e558__generated,
            rider_on_split_marker,
            16,
            &|c, seed| {
                let _ = base(c, seed, 0x10e5_58, 6);
                let rider = DATA;
                let marker = DATA + 0x1000;
                w(c, rider + 0xec, 0x1234);
                w(c, rider + 0x790, DATA + 0x2000);
                w(c, marker, 0x55);
                wb(c, 0x0053_5c12, (seed % 2) as u8);
                w(c, 0x0053_08d0, (seed % 4 == 0) as u32 * 0x200);
                c.g[A0][0] = rider as u64;
                c.g[A1][0] = marker as u64;
            },
        );
    }

    // ----------------------------------------------------------------- scripts

    #[test]
    fn script_engine_is_process_finished_matches() {
        check(
            ee::s07::f_00282838__generated,
            script_engine_is_process_finished,
            16,
            &|c, seed| {
                let _ = base(c, seed, 0x2828_38, 4);
                let self_ = DATA;
                let idx = (seed % 6) as u32;
                let proc = self_ + 0x2c0 + idx * 0x24;
                w(c, proc + 0x20, if seed % 5 == 0 { 0 } else { 1 });
                w(c, proc + 0x4, (seed % 8) as u32);
                w(c, proc, DATA + 0x1000);
                w(c, DATA + 0x1000 + 0x20, (seed % 5) as u32);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = idx as u64;
            },
        );
    }

    #[test]
    fn be_reward_int_are_all_peaks_complete_matches() {
        check(
            ee::s01::f_001578a0__generated,
            be_reward_int_are_all_peaks_complete,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x1578_a0, 6);
                let self_ = DATA;
                let profile = DATA + 0x1000;
                let ctx = DATA + 0x2000;
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = profile as u64;
                c.g[A2][0] = ctx as u64;
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn world_trigger_manager_check_crossing_matches() {
        check(
            ee::s08::f_002b69b8__generated,
            world_trigger_manager_check_crossing,
            16,
            &|c, seed| {
                let mut r = base(c, seed, 0x2b69_b8, 4);
                let trigger = DATA + 0x1000;
                let obj = DATA + 0x1100;
                let vtable = DATA + 0x1200;
                let state = DATA + 0x2000;
                w(c, trigger + 0xc, obj);
                w(c, obj + 0xc, vtable);
                ret_slot(c, vtable, 0xf0, RET_ZERO);
                w(c, state + 0x68, r.edge_f32());
                w(c, state + 0x6c, (seed % 2) as u32);
                c.f[0] = r.edge_f32();
                c.g[A0][0] = (1 + seed) as u64;
                c.g[A1][0] = trigger as u64;
                c.g[A2][0] = state as u64;
            },
        );
    }

    #[test]
    fn script_list_ctor_matches() {
        check(ee::s07::f_00275d10__generated, script_list_ctor, 12, &|c, seed| {
            let _ = alloc_base(c, seed, 0x275d_10, 4);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn c_bx_string_right_matches() {
        check(ee::s10::f_00319028__generated, c_bx_string_right, 12, &|c, seed| {
            let mut r = alloc_base(c, seed, 0x3190_28, 4);
            let dst = DATA;
            let src = DATA + 0x1000;
            let buf = DATA + 0x2000;
            let len = 1 + r.next_u32() % 12;
            w(c, src, buf);
            w(c, buf - 8, len);
            for i in 0..len {
                wb(c, buf + i, b'a' + (i % 26) as u8);
            }
            c.g[A0][0] = dst as u64;
            c.g[A1][0] = src as u64;
            c.g[A2][0] = match seed % 3 {
                0 => (r.next_u32() % 16) as u64,
                1 => 0,
                _ => (-1i64) as u64,
            };
        });
    }

    #[test]
    fn font_load_matches() {
        check(ee::s12::f_00391708__generated, font_load, 8, &|c, seed| {
            let _ = alloc_base(c, seed, 0x3917_08, 8);
            eafile_pools(c);
            let font = DATA;
            let name = DATA + 0x1000;
            put_str(c, name, b"font.x");
            c.g[A0][0] = font as u64;
            c.g[A1][0] = name as u64;
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { 0x100 };
        });
    }

    #[test]
    fn run_list_seek_matches() {
        check(ee::s07::f_0026d4d8__generated, run_list_seek, 16, &|c, seed| {
            let mut r = base(c, seed, 0x26d4_d8, 0);
            let list = DATA;
            let entries = DATA + 0x1000;
            let n = (seed % 5) as u32;
            w(c, list, n);
            w(c, list + 0xc, entries);
            let mut pos = 0u32;
            for i in 0..n {
                let size = 1 + r.next_u32() % 0x100;
                w(c, entries + 8 * i, size);
                w(c, entries + 8 * i + 4, pos);
                pos = pos.wrapping_add(size);
            }
            c.g[A0][0] = list as u64;
            c.g[A1][0] = match seed % 4 {
                0 => 0,
                1 => pos as u64,
                2 => pos as u64 + 1,
                _ => (r.next_u32() % (pos + 1).max(1)) as u64,
            };
        });
    }

    #[test]
    fn tag_field_get_number_matches() {
        check(ee::s14::f_003ecf30__generated, tag_field_get_number, 24, &|c, seed| {
            let _ = base(c, seed, 0x3ecf_30, 0);
            let tag = DATA + 0x1000;
            let text: &[u8] = match seed % 6 {
                0 => b"123",
                1 => b"-45",
                2 => b"+7",
                3 => b"abc",
                4 => b"",
                _ => b"0x10",
            };
            put_str(c, tag, text);
            c.g[A0][0] = tag as u64;
            c.g[A1][0] = 77;
        });
    }

    #[test]
    fn ui_engine_load_package_matches() {
        check(
            ee::s13::f_00397c40__generated,
            ui_engine_load_package,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x397c_40, 8);
                eafile_pools(c);
                let engine = DATA;
                let name = DATA + 0x1000;
                put_str(c, name, b"package");
                c.g[A0][0] = engine as u64;
                c.g[A1][0] = name as u64;
            },
        );
    }

    #[test]
    fn flex_bridge_springs_load_matches() {
        check(
            ee::s11::f_00353e80__generated,
            flex_bridge_springs_load,
            8,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x353e_80, 6);
                let self_ = DATA;
                let stream = DATA + 0x1000;
                let vtable = DATA + 0x1200;
                w(c, self_, 1 + r.next_u32() % 4);
                w(c, self_ + 0x24, DATA + 0x2000);
                w(c, stream, vtable);
                ret_slot(c, vtable, 0x10, RET_ZERO);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = stream as u64;
            },
        );
    }

    // ---------------------------------------------------------------- overlay

    #[test]
    fn ov_screen_map_init_matches() {
        check(ee::s05::f_00208498__generated, ov_screen_map_init, 8, &|c, seed| {
            let _ = alloc_base(c, seed, 0x2084_98, 4);
            let engine = DATA + 0x1000;
            w(c, DATA + 0x10, engine);
            w(c, engine, DATA + 0x1100);
            w(c, engine + 4, 0);
            w(c, DATA + 0x1100, 0);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn c_flag_manager_group_ctor_matches() {
        check(
            ee::s11::f_0034af38__generated,
            c_flag_manager_group_ctor,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x34af_38, 4);
                w(c, GAME_GP.wrapping_add(0xf20), (seed % 3) as u32);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn graphics_man_set_color_mask_matches() {
        check(
            ee::s12::f_00369610__generated,
            graphics_man_set_color_mask,
            12,
            &|c, seed| {
                let _ = base(c, seed, 0x3696_10, 4);
                let man = DATA;
                let mask = DATA + 0x1000;
                let vtable = DATA + 0x1200;
                for i in 0..4 {
                    w(c, mask + 4 * i, 0x0101_0101 * (i + 1));
                }
                w(c, man + 0x10d8, DATA + 0x1100);
                w(c, DATA + 0x1100, vtable);
                ret_slot(c, vtable, 0x1f0, RET_ZERO);
                c.g[A0][0] = man as u64;
                c.g[A1][0] = mask as u64;
            },
        );
    }

    #[test]
    fn ov_screen_bc_fail_init_matches() {
        check(
            ee::s04::f_001f76b8__generated,
            ov_screen_bc_fail_init,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x1f76_b8, 4);
                let engine = DATA + 0x1000;
                w(c, DATA + 0x10, engine);
                w(c, engine, DATA + 0x1100);
                w(c, engine + 4, 0);
                w(c, DATA + 0x1100, 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_init_matches() {
        check(
            ee::s05::f_002111a0__generated,
            ov_screen_profile_save_pda_init,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2111_a0, 6);
                let engine = DATA + 0x1000;
                w(c, DATA + 0x10, engine);
                w(c, engine, DATA + 0x1100);
                w(c, engine + 4, 0);
                w(c, DATA + 0x1100, 0);
                w(c, GAME_GP.wrapping_sub(0x9e4), 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn section_man_begin_event_switch_matches() {
        check(
            ee::s05::f_0022df50__generated,
            section_man_begin_event_switch,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x22df_50, 4);
                let self_ = DATA;
                let module = DATA + 0x1000;
                w(c, self_ + 0x1b0, module);
                w(c, self_ + 0x1bc, r.next_u32());
                c.g[A0][0] = self_ as u64;
            },
        );
    }

    #[test]
    fn ov_screen_pda_template_open_matches() {
        check(
            ee::s05::f_0020d110__generated,
            ov_screen_pda_template_open,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x20d1_10, 4);
                w(
                    c,
                    GAME_GP.wrapping_sub(0x9e4),
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                let game = W + 0x1000;
                w(c, game + 0x48, DATA + 0x2000);
                c.g[A0][0] = (seed + 1) as u64;
            },
        );
    }
}
