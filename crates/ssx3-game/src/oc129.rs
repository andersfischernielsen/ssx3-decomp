//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold front-end / network / front-end-state functions,
//! each with a differential test against the generated body.
//!
//! The functions here are mostly UI-state handlers: they look an object up on
//! a screen, build a popup, and hand a vtable call or a formatted string to a
//! helper. The ports keep the original's frame, block cycle charges, register
//! dance at every call (the fake-call log in the tests compares all of them)
//! and branch order; calls go through [`crate::call`] so an unwind in a callee
//! returns at once exactly as the compiled caller does.

#![allow(dead_code)]

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_3258,
        cfe_state_mp_circuit_on_create_screen,
        "cFEStateMPCircuit_onCreateScreen",
    ),
    (
        0x0019_1228,
        cfe_state_rider_bio_fill_bio_info,
        "cFEStateRiderBio_fillBioInfo",
    ),
    (
        0x001a_4ce8,
        net_fe_state_show_user_found,
        "cFEStateNetMessenger_ShowUserFound",
    ),
    (
        0x001a_9668,
        net_fe_state_on_disconnected_popup_event,
        "NetFEState_OnDisconnectedPopupEvent",
    ),
    (
        0x001a_9710,
        net_fe_state_on_net_interrupted_popup_event,
        "NetFEState_OnNetInterruptedPopupEvent",
    ),
    (
        0x001a_c2b8,
        cfe_use_existing_account_check_password,
        "cFEUseExistingAccount_CheckPassword",
    ),
    (
        0x001a_c360,
        cfe_use_existing_account_check_account_name,
        "cFEUseExistingAccount_CheckAccountName",
    ),
    (
        0x001b_38d0,
        cfe_state_chat_room_handle_input,
        "cFEStateChatRoom_HandleInput",
    ),
    (
        0x0025_a650,
        lobby_man_create_peer_table,
        "LobbyMan_CreatePeerTable",
    ),
    (
        0x0027_9528,
        script_engine_spawn_chosen_script,
        "ScriptEngine_SpawnChosenScript",
    ),(0x002c_c3b8, menu_sys_push_page, "MenuSys_PushPage"),
    (
        0x002d_0500,
        menu_sys_vector_label_item_draw,
        "MenuSysVectorLabelItem_Draw",
    ),
    (0x0027_29a8, in_seq_free_sequence, "InSeq_FreeSequence"),
    (0x002c_2718, ustr_sprintf_ascii_fmt, "UStr_SprintfAsciiFmt"),
    (
        0x002c_27c0,
        ustr_sprintf_ascii_fmt_2,
        "UStr_SprintfAsciiFmt_2",
    ),
    (
        0x0023_f050,
        mc_manager_state0b_format,
        "MCManager_State0B_Format",
    ),
    (
        0x0024_0b88,
        mc_manager_state28_format,
        "MCManager_State28_Format",
    ),
    (0x001c_5b88, cfe_popup_confirm_v06, "cFEPopupConfirm_v06"),
    (
        0x0017_a220,
        debug_menu_find_next_item,
        "DebugMenu_FindNextItem",
    ),
    (0x0017_e118, lobby_menu_login, "LobbyMenu_Login"),
    (
        0x0028_6200,
        ssx_audio_system_unload_front_end_audio,
        "SSXAudioSystem_UnloadFrontEndAudio",
    ),(
        0x0018_e2c0,
        cfe_state_options_device_select_v38,
        "cFEStateOptionsDeviceSelect_v38",
    ),
    (
        0x0025_3938,
        vid_engine_player_update,
        "VidEnginePlayer_Update",
    ),
    (0x0039_e390, ui_state_dtor, "UIState_dtor"),
    (
        0x002e_cfa0,
        world_painter_free_view_queries,
        "WorldPainter_FreeViewQueries",
    ),
    (
        0x001d_4b20,
        cfe_state_trophy_room_release_trophy_images,
        "cFEStateTrophyRoom_ReleaseTrophyImages",
    ),
    (
        0x0023_32e8,
        c_game_load_screen_unload,
        "cGameLoadScreen_Unload",
    ),
    (
        0x0039_9d80,
        ui_list_box_set_item_wide_string,
        "UIListBox_SetItemWideString",
    ),(0x0023_0e98, game_module_try_exit, "GameModule_TryExit"),
    (0x0014_ebf8, be_backup_player_setup, "BE_BackupPlayerSetup"),
    (
        0x0015_19e0,
        be_reset_char_rivalries,
        "BE_ResetCharRivalries",
    ),
    (
        0x0026_e838,
        replay_highlight_slot_score,
        "Replay_HighlightSlotScore",
    ),
    (0x003a_0348, ui_text_ctor, "UIText_ctor"),
    (
        0x0019_4600,
        cfe_state_transition_v16,
        "cFEStateTransition_v16",
    ),
    (
        0x001e_6668,
        ov_screen_final_standings_init,
        "OVScreenFinalStandings_Init",
    ),(
        0x0027_d0c0,
        script_engine_set_obj_binding_filter,
        "ScriptEngine_SetObjBindingFilter",
    ),
    (0x002b_0e60, spch_create_instance, "SPCH_CreateInstance"),
    (
        0x002d_0450,
        menu_sys_vector_label_item_measure,
        "MenuSysVectorLabelItem_Measure",
    ),
    (
        0x0012_c5c8,
        rider_finish_line_sub_enter,
        "RiderFinishLineSub_Enter",
    ),
    (
        0x0012_d9d8,
        rider_wipeout_sub_update_phase4,
        "RiderWipeoutSub_UpdatePhase4",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const STR_HASH: Callee = Callee {
        addr: 0x0031_7670,
        f: ee::s10::f_00317670,
    };
    pub const UIENGINE_CREATE_SCREEN: Callee = Callee {
        addr: 0x0039_7cc0,
        f: ee::s13::f_00397cc0,
    };
    pub const UISCREEN_START_THREAD: Callee = Callee {
        addr: 0x0039_c870,
        f: ee::s13::f_0039c870,
    };
    pub const UISCREEN_FIND_OBJECT: Callee = Callee {
        addr: 0x0039_d860,
        f: ee::s13::f_0039d860,
    };
    pub const BE_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };
    pub const BE_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };
    pub const BE_GET_CHAR_DISPLAY_NAME: Callee = Callee {
        addr: 0x0014_eec8,
        f: ee::s01::f_0014eec8,
    };
    pub const SPRINTF: Callee = Callee {
        addr: 0x0041_7828,
        f: ee::s15::f_00417828,
    };
    pub const UITEXTSCROLL_CLEAR_LINES: Callee = Callee {
        addr: 0x003a_18c0,
        f: ee::s13::f_003a18c0,
    };
    pub const UITEXTSCROLL_SET_TEXT_ID: Callee = Callee {
        addr: 0x003a_1f18,
        f: ee::s13::f_003a1f18,
    };
    pub const UIENGINE_CLOSE_ALL_POPUPS: Callee = Callee {
        addr: 0x0039_f698,
        f: ee::s13::f_0039f698,
    };
    pub const UIENGINE_GET_ACTIVE_STATE: Callee = Callee {
        addr: 0x0039_fa88,
        f: ee::s13::f_0039fa88,
    };
    pub const NETFE_STATE_GET_POPUP: Callee = Callee {
        addr: 0x001a_9c50,
        f: ee::s03::f_001a9c50,
    };
    pub const NETFE_STATE_SHOW_POPUP: Callee = Callee {
        addr: 0x001a_9bc0,
        f: ee::s03::f_001a9bc0,
    };
    pub const POPUP_SET_OPTION_STRING: Callee = Callee {
        addr: 0x001c_5780,
        f: ee::s03::f_001c5780,
    };
    pub const POPUP_SET_OPTION_TEXT_ID: Callee = Callee {
        addr: 0x001c_57c0,
        f: ee::s03::f_001c57c0,
    };
    pub const NETFE_STATE_ON_CHILD_EVENT_DEFAULT: Callee = Callee {
        addr: 0x001a_93d0,
        f: ee::s03::f_001a93d0,
    };
    pub const NETFE_STATE_OPEN_KEYBOARD_TEXT_ID: Callee = Callee {
        addr: 0x001a_9150,
        f: ee::s03::f_001a9150,
    };
    pub const NETFE_STATE_GOTO_NEXT_STATE: Callee = Callee {
        addr: 0x001a_8a40,
        f: ee::s03::f_001a8a40,
    };
    pub const CFE_STATE_LOGIN_CTOR: Callee = Callee {
        addr: 0x001b_ee30,
        f: ee::s03::f_001bee30,
    };
    pub const CFE_STATE_NET_CONFIG_CTOR: Callee = Callee {
        addr: 0x001b_3f78,
        f: ee::s03::f_001b3f78,
    };
    pub const CBXSTRING_ASSIGN: Callee = Callee {
        addr: 0x0031_8448,
        f: ee::s10::f_00318448,
    };
    pub const MENU_SYS_SET_ACTIVE: Callee = Callee {
        addr: 0x002c_c0b8,
        f: ee::s09::f_002cc0b8,
    };
    pub const MENU_SYS_ITEM_DRAW_ROW: Callee = Callee {
        addr: 0x002c_a988,
        f: ee::s09::f_002ca988,
    };
    pub const MENU_SYS_ITEM_MEASURE_ROW: Callee = Callee {
        addr: 0x002c_a4c8,
        f: ee::s09::f_002ca4c8,
    };
    pub const SCRIPT_ENGINE_CHOOSE_SCRIPT: Callee = Callee {
        addr: 0x0027_b0c0,
        f: ee::s07::f_0027b0c0,
    };
    pub const SCRIPT_ENGINE_POP_FREE_NODE: Callee = Callee {
        addr: 0x0027_bd78,
        f: ee::s07::f_0027bd78,
    };
    pub const SCRIPT_ENGINE_START_SCRIPT: Callee = Callee {
        addr: 0x0028_21a0,
        f: ee::s07::f_002821a0,
    };
    pub const SCRIPT_ENGINE_PUSH_FREE_NODE: Callee = Callee {
        addr: 0x0027_bd90,
        f: ee::s07::f_0027bd90,
    };
    pub const SCRIPT_ENGINE_ADD_SCRIPT_OBJ: Callee = Callee {
        addr: 0x0027_cea8,
        f: ee::s07::f_0027cea8,
    };
    pub const SCRIPT_VIEW_OBJ_ADD_CAM: Callee = Callee {
        addr: 0x0028_1190,
        f: ee::s07::f_00281190,
    };
    pub const SCRIPT_VIEW_OBJ_SET_TRANSITION: Callee = Callee {
        addr: 0x0028_12e0,
        f: ee::s07::f_002812e0,
    };
    pub const SCRIPT_ENGINE_SET_BINDING_FILTER: Callee = Callee {
        addr: 0x0027_d540,
        f: ee::s07::f_0027d540,
    };
    pub const INSEQ_STOP: Callee = Callee {
        addr: 0x0027_2b58,
        f: ee::s07::f_00272b58,
    };
    pub const INSEQ_INSTANCE_RELEASE: Callee = Callee {
        addr: 0x0027_46f8,
        f: ee::s07::f_002746f8,
    };
    pub const INSEQ_EVENT_TRACK_FIRE_KEY: Callee = Callee {
        addr: 0x0027_3418,
        f: ee::s07::f_00273418,
    };
    pub const INSEQ_EVENT_TRACK_NEXT_KEY: Callee = Callee {
        addr: 0x0027_3468,
        f: ee::s07::f_00273468,
    };
    pub const USTR_VSPRINTF: Callee = Callee {
        addr: 0x003d_c450,
        f: ee::s14::f_003dc450,
    };
    pub const USTR_LEN: Callee = Callee {
        addr: 0x003d_bca0,
        f: ee::s14::f_003dbca0,
    };
    pub const USTR_CPY: Callee = Callee {
        addr: 0x003d_be20,
        f: ee::s14::f_003dbe20,
    };
    pub const EA_MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::f_00317d70,
    };
    pub const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };
    pub const EA_MEM_FREE: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };
    pub const EAFILE_GET_PATH_SIZE_CHECKED: Callee = Callee {
        addr: 0x0031_7f60,
        f: ee::s10::f_00317f60,
    };
    pub const EAFILE_LOAD_FILE_INTO: Callee = Callee {
        addr: 0x003e_1bb8,
        f: ee::s14::f_003e1bb8,
    };
    pub const BOLT_FILE_ACQUIRE: Callee = Callee {
        addr: 0x0019_da10,
        f: ee::s03::f_0019da10,
    };
    pub const REWARDS_FILE_PARSE: Callee = Callee {
        addr: 0x0015_a818,
        f: ee::s01::f_0015a818,
    };
    pub const BE_BACKUP_CHARACTER: Callee = Callee {
        addr: 0x0015_6950,
        f: ee::s01::f_00156950,
    };
    pub const RIDER_METRIX_ON_RIDER_RESET: Callee = Callee {
        addr: 0x0011_9368,
        f: ee::s00::f_00119368,
    };
    pub const RIDER_ADD_WIPEOUT_RECOVERY_BOOST: Callee = Callee {
        addr: 0x0010_f280,
        f: ee::s00::f_0010f280,
    };
    pub const RIDER_CHECK_RESET_PATH: Callee = Callee {
        addr: 0x0011_6120,
        f: ee::s00::f_00116120,
    };
    pub const ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED: Callee = Callee {
        addr: 0x0031_2ae8,
        f: ee::s10::f_00312ae8,
    };
    pub const AV_PLAYER_IS_FRAME_DUE: Callee = Callee {
        addr: 0x003a_e780,
        f: ee::s13::f_003ae780,
    };
    pub const AV_PLAYER_DECODE_FRAME: Callee = Callee {
        addr: 0x003a_e6c8,
        f: ee::s13::f_003ae6c8,
    };
    pub const AV_PLAYER_IS_AUDIO_DONE: Callee = Callee {
        addr: 0x003a_e860,
        f: ee::s13::f_003ae860,
    };
    pub const UI_LIST_DELETE_ALL: Callee = Callee {
        addr: 0x0039_7948,
        f: ee::s13::f_00397948,
    };
    pub const UI_LIST_NODE_DTOR_BODY: Callee = Callee {
        addr: 0x003a_4868,
        f: ee::s13::f_003a4868,
    };
    pub const UI_ENGINE_DTOR: Callee = Callee {
        addr: 0x0039_7b70,
        f: ee::s13::f_00397b70,
    };
    pub const UI_OBJECT_CTOR: Callee = Callee {
        addr: 0x0039_fb30,
        f: ee::s13::f_0039fb30,
    };
    pub const UI_STATE_MGR_FIND_STATE: Callee = Callee {
        addr: 0x0039_f9d8,
        f: ee::s13::f_0039f9d8,
    };
    pub const SSX_AUDIO_SYSTEM_SET_MODE: Callee = Callee {
        addr: 0x0028_5be0,
        f: ee::s07::f_00285be0,
    };
    pub const SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN: Callee = Callee {
        addr: 0x0028_7930,
        f: ee::s07::f_00287930,
    };
    pub const SND_BANK_MAN_UNLOAD_BANK: Callee = Callee {
        addr: 0x0028_bce8,
        f: ee::s07::f_0028bce8,
    };
    pub const BANK_MONITOR_KILL_ALL_TIMERS: Callee = Callee {
        addr: 0x002a_dda0,
        f: ee::s08::f_002adda0,
    };
    pub const MUSIC_PLAYER_STOP_SONG: Callee = Callee {
        addr: 0x002b_3ac0,
        f: ee::s08::f_002b3ac0,
    };
    pub const MUSIC_PLAYER_FADE_OUT: Callee = Callee {
        addr: 0x002b_3d48,
        f: ee::s08::f_002b3d48,
    };
    pub const MUSIC_PLAYER_PLAY_SONG: Callee = Callee {
        addr: 0x002b_35a0,
        f: ee::s08::f_002b35a0,
    };
    pub const SPEECH_INSTANCE_CTOR: Callee = Callee {
        addr: 0x002a_fa68,
        f: ee::s08::f_002afa68,
    };
    pub const APP_MODULE_TRY_EXIT: Callee = Callee {
        addr: 0x0023_1cf0,
        f: ee::s06::f_00231cf0,
    };
    pub const SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT: Callee = Callee {
        addr: 0x0027_8718,
        f: ee::s07::f_00278718,
    };
    pub const LOAD_INDICATOR_CLEAR: Callee = Callee {
        addr: 0x001f_3170,
        f: ee::s04::f_001f3170,
    };
    pub const GRAPHICS_MAN_FREE_ALL_TEXTURES: Callee = Callee {
        addr: 0x0036_a020,
        f: ee::s12::f_0036a020,
    };
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::f_0028b180,
    };
    pub const SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP: Callee = Callee {
        addr: 0x0028_fa98,
        f: ee::s07::f_0028fa98,
    };
    pub const LOBBY_API_GET_RECORD: Callee = Callee {
        addr: 0x003e_8a70,
        f: ee::s14::f_003e8a70,
    };
    pub const NET_GAME_UTIL_CREATE: Callee = Callee {
        addr: 0x003e_6e08,
        f: ee::s14::f_003e6e08,
    };
    pub const NET_GAME_UTIL_ADVERT: Callee = Callee {
        addr: 0x003e_7120,
        f: ee::s14::f_003e7120,
    };
    pub const C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK: Callee = Callee {
        addr: 0x001a_34a0,
        f: ee::s03::f_001a34a0,
    };
    pub const DEBUG_MENU_DISPATCH_ITEM_OP: Callee = Callee {
        addr: 0x0017_a158,
        f: ee::s02::f_0017a158,
    };
    pub const LOBBY_LIST_GET_AT: Callee = Callee {
        addr: 0x003e_e560,
        f: ee::s14::f_003ee560,
    };
    pub const ATOI: Callee = Callee {
        addr: 0x0041_87c0,
        f: ee::s15::f_004187c0,
    };
    pub const TAG_FIELD_GET_BINARY: Callee = Callee {
        addr: 0x003e_d678,
        f: ee::s14::f_003ed678,
    };
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `base + off` (32-bit).
#[inline(always)]
fn at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `lw` of a 32-bit object field.
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

/// `lh` (sign-extended) of a 16-bit field.
#[inline(always)]
fn lh(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lh(c)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// FE state machine: multiplayer circuit screen creation.

/// `void cFEStateMPCircuit_onCreateScreen(cFEStateMPCircuit* self)` (0x1a3258):
/// create the screen for the state's stored name hash, start its thread and
/// find the state object on it, clearing the "not ready" bit.
pub fn cfe_state_mp_circuit_on_create_screen(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        c.cycles += 16;
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x0046_13c8);
        c.g[RA][0] = sx(0x001a_327c);
        c.g[S0][0] = lw(c, at(c.g[S1][0], 0x10));
        call_ptr(c, 0x0031_7670, 0x001a_327c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001a_3290);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001a_3290)?;
        let screen = c.g[V0][0];
        Addr(at(c.g[S1][0], 0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001a_32a8);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001a_32a8)?;
        }
        c.cycles += 9;
        c.g[A0][0] = sx(0x004a_19e8);
        c.g[RA][0] = sx(0x001a_32b4);
        call_ptr(c, 0x0031_7670, 0x001a_32b4)?;
        c.g[A0][0] = lw(c, at(c.g[S1][0], 0x40));
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001a_32c0);
        call_ptr(c, 0x0039_d860, 0x001a_32c0)?;
        let obj = c.g[V0][0];
        let t = obj == 0;
        c.g[V1][0] = sx(0xffff_fffe);
        if !t {
            c.cycles += 7;
            c.g[V0][0] = lw(c, at(obj, 0x14));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = obj;
            c.g[V0][0] &= c.g[V1][0];
            c.g[V0][0] |= 0x80;
            c.g[RA][0] = sx(0x001a_32e8);
            Addr(at(obj, 0x14)).sw(c, c.g[V0][0]);
            callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK.call(c, 0x001a_32e8)?;
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
// Rider bio: format the character's display name into the bio's text object.

/// `void cFEStateRiderBio_fillBioInfo(cFEStateRiderBio* self)` (0x191228):
/// find the scrolling-text object on the screen, format the character's
/// display name into a stack buffer and set it as the text (by hash).
pub fn cfe_state_rider_bio_fill_bio_info(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        c.cycles += 14;
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        c.g[A0][0] = sx(0x0046_0000);
        frame.save_ra(c, 0x40);
        c.g[RA][0] = sx(0x0019_1248);
        c.g[A0][0] = sx(0x0045_e438);
        call_ptr(c, 0x0031_7670, 0x0019_1248)?;
        c.g[A0][0] = lw(c, at(c.g[S0][0], 0x40));
        c.g[RA][0] = sx(0x0019_1254);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0019_1254)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] == 0 {
            c.lq(16, frame.at(c, 0x60));
        } else {
            c.cycles += 23;
            c.g[RA][0] = sx(0x0019_1268);
            call_ptr(c, 0x0014_dc80, 0x0019_1268)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0019_1274);
            c.g[A1][0] = 2;
            call_ptr(c, 0x0014_dd58, 0x0019_1274)?;
            c.g[A1][0] = lw(c, at(c.g[S0][0], 0x48));
            c.g[A0][0] = c.g[V0][0];
            c.g[S0][0] = sx(0x0046_0000);
            c.g[A2][0] = 0;
            c.g[RA][0] = sx(0x0019_128c);
            c.g[S0][0] = sx(0x0045_f7d0);
            call_ptr(c, 0x0014_eec8, 0x0019_128c)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(frame.at(c, 0));
            c.g[RA][0] = sx(0x0019_129c);
            c.g[A2][0] = c.g[V0][0];
            call_ptr(c, 0x0041_7828, 0x0019_129c)?;
            c.g[RA][0] = sx(0x0019_12a4);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a_18c0, 0x0019_12a4)?;
            c.g[RA][0] = sx(0x0019_12ac);
            c.g[A0][0] = sx(frame.at(c, 0));
            call_ptr(c, 0x0031_7670, 0x0019_12ac)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0019_12b8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1f18, 0x0019_12b8)?;
            c.lq(16, frame.at(c, 0x60));
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Net messenger: "user found" popup.

/// `void cFEStateNetMessenger_ShowUserFound(self, src)` (0x1a4ce8): copy the
/// found user's name into the state's string, configure a three-option popup
/// and show it.
pub fn net_fe_state_show_user_found(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 42;
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[A1][0] = sx(at(c.g[S0][0], 8));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(at(c.g[S2][0], 0x6d0));
        callee::CBXSTRING_ASSIGN.call(c, 0x001a_4d14)?;
        c.g[S3][0] = 3;
        c.g[V0][0] = lw(c, at(c.g[S0][0], 0x10));
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001a_4d28);
        Addr(at(c.g[S2][0], 0x6d8)).sw(c, c.g[V0][0]);
        callee::NETFE_STATE_GET_POPUP.call(c, 0x001a_4d28)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sx(0x0046_1648);
        c.g[V0][0] = 1;
        c.g[A0][0] = sx(0x0046_1648);
        Addr(at(c.g[S0][0], 0xbc)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x001a_4d44);
        c.g[S1][0] = sx(at(c.g[S0][0], 0xbc));
        call_ptr(c, 0x0031_7670, 0x001a_4d44)?;
        Addr(at(c.g[S0][0], 0xcc)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        Addr(at(c.g[S0][0], 0xc8)).sw(c, c.g[S3][0]);
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x001a_4d5c);
        c.g[A2][0] = lw(c, at(c.g[S2][0], 0x6d0));
        callee::POPUP_SET_OPTION_STRING.call(c, 0x001a_4d5c)?;
        Addr(at(c.g[S0][0], 0x14c)).sw(c, c.g[S3][0]);
        c.g[V0][0] = sx(0x809);
        Addr(at(c.g[S0][0], 0x150)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001a_4d74);
        c.g[A1][0] = c.g[S0][0];
        callee::NETFE_STATE_SHOW_POPUP.call(c, 0x001a_4d74)?;
        c.lq(16, frame.at(c, 0x40));
        c.lq(17, frame.at(c, 0x30));
        c.lq(18, frame.at(c, 0x20));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Net FE state: disconnected / interrupted popup events.

/// `void NetFEState_OnDisconnectedPopupEvent(self, a1, event)` (0x1a9668):
/// for popup events 15 and 20 close every popup and go to the login state,
/// for any other event fall through to the default child-event handler.
pub fn net_fe_state_on_disconnected_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'end: {
            'default: {
                'body: {
                    c.cycles += 6;
                    c.g[V0][0] = 0xf;
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        break 'body;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x14;
                    if c.g[A2][0] != c.g[V0][0] {
                        break 'default;
                    }
                    break 'body;
                }
                c.cycles += 27;
                c.g[A0][0] = c.g[S0][0];
                callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_9694)?;
                c.g[A0][0] = lw(c, at(c.g[S0][0], 0x10));
                c.g[RA][0] = sx(0x001a_96a0);
                c.g[A0][0] = sx(at(c.g[A0][0], 0x18));
                call_ptr(c, 0x0039_f698, 0x001a_96a0)?;
                c.g[A0][0] = lw(c, at(c.g[S0][0], 0x10));
                c.g[V0][0] = 1;
                Addr(at(c.g[S0][0], 0x58)).sw(c, c.g[V0][0]);
                c.g[RA][0] = sx(0x001a_96b4);
                c.g[A0][0] = sx(at(c.g[A0][0], 0x18));
                call_ptr(c, 0x0039_fa88, 0x001a_96b4)?;
                c.g[V1][0] = lw(c, at(c.g[V0][0], 0x1c));
                c.g[A1][0] = sx(0x0046_0000);
                c.g[A3][0] = 0;
                c.g[A2][0] = 0;
                c.g[V1][0] |= 0x80;
                c.g[A1][0] = sx(0x0046_1cd8);
                Addr(at(c.g[V0][0], 0x1c)).sw(c, c.g[V1][0]);
                c.g[RA][0] = sx(0x001a_96d8);
                c.g[A0][0] = 0x6d4;
                call_ptr(c, 0x0031_7d70, 0x001a_96d8)?;
                c.g[A1][0] = lw(c, at(c.g[S0][0], 0x10));
                c.g[RA][0] = sx(0x001a_96e4);
                c.g[A0][0] = c.g[V0][0];
                callee::CFE_STATE_LOGIN_CTOR.call(c, 0x001a_96e4)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a_96f0);
                c.g[A0][0] = c.g[S0][0];
                callee::NETFE_STATE_GOTO_NEXT_STATE.call(c, 0x001a_96f0)?;
                Addr(at(c.g[S0][0], 0x6ac)).sw(c, 0);
                break 'end;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_9700)?;
            break 'end;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void NetFEState_OnNetInterruptedPopupEvent(self, a1, event)` (0x1a9710):
/// the same for a network interruption, but to the net-config state.
pub fn net_fe_state_on_net_interrupted_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'end: {
            'default: {
                'body: {
                    c.cycles += 6;
                    c.g[V0][0] = 0xf;
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        break 'body;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x14;
                    if c.g[A2][0] != c.g[V0][0] {
                        break 'default;
                    }
                    break 'body;
                }
                c.cycles += 27;
                c.g[A0][0] = c.g[S0][0];
                callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_973c)?;
                c.g[A0][0] = lw(c, at(c.g[S0][0], 0x10));
                c.g[RA][0] = sx(0x001a_9748);
                c.g[A0][0] = sx(at(c.g[A0][0], 0x18));
                call_ptr(c, 0x0039_f698, 0x001a_9748)?;
                c.g[A0][0] = lw(c, at(c.g[S0][0], 0x10));
                c.g[V0][0] = 1;
                Addr(at(c.g[S0][0], 0x58)).sw(c, c.g[V0][0]);
                c.g[RA][0] = sx(0x001a_975c);
                c.g[A0][0] = sx(at(c.g[A0][0], 0x18));
                call_ptr(c, 0x0039_fa88, 0x001a_975c)?;
                c.g[V1][0] = lw(c, at(c.g[V0][0], 0x1c));
                c.g[A1][0] = sx(0x0046_0000);
                c.g[A3][0] = 0;
                c.g[A2][0] = 0;
                c.g[V1][0] |= 0x80;
                c.g[A1][0] = sx(0x0046_0bf0);
                Addr(at(c.g[V0][0], 0x1c)).sw(c, c.g[V1][0]);
                c.g[RA][0] = sx(0x001a_9780);
                c.g[A0][0] = 0x91c;
                call_ptr(c, 0x0031_7d70, 0x001a_9780)?;
                c.g[A1][0] = lw(c, at(c.g[S0][0], 0x10));
                c.g[RA][0] = sx(0x001a_978c);
                c.g[A0][0] = c.g[V0][0];
                callee::CFE_STATE_NET_CONFIG_CTOR.call(c, 0x001a_978c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a_9798);
                c.g[A0][0] = c.g[S0][0];
                callee::NETFE_STATE_GOTO_NEXT_STATE.call(c, 0x001a_9798)?;
                Addr(at(c.g[S0][0], 0x6ac)).sw(c, 0);
                break 'end;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT.call(c, 0x001a_97a8)?;
            break 'end;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Use-existing-account: password / account-name validation popups.

/// `int cFEUseExistingAccount_CheckPassword(self)` (0x1ac2b8): if the typed
/// password is empty, show the "password missing" popup and return 0.
pub fn cfe_use_existing_account_check_password(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 11;
        c.g[V0][0] = lw(c, gp(c, -0xc8));
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001a_c2dc);
        c.g[A0][0] = lw(c, at(c.g[V0][0], 0x4c));
        call_ptr(c, 0x0041_6810, 0x001a_c2dc)?;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !t {
            c.cycles += 25;
            c.g[A0][0] = c.g[S2][0];
            callee::NETFE_STATE_GET_POPUP.call(c, 0x001a_c2ec)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = sx(0x0046_0000);
            c.g[V0][0] = 1;
            c.g[A0][0] = sx(0x0046_2bb0);
            Addr(at(c.g[S0][0], 0xbc)).sw(c, c.g[V0][0]);
            c.g[RA][0] = sx(0x001a_c308);
            c.g[S1][0] = sx(at(c.g[S0][0], 0xbc));
            call_ptr(c, 0x0031_7670, 0x001a_c308)?;
            c.g[V1][0] = 3;
            c.g[A0][0] = sx(0x0046_0000);
            Addr(at(c.g[S0][0], 0xc0)).sw(c, c.g[V1][0]);
            c.g[A0][0] = sx(0x0046_1618);
            c.g[RA][0] = sx(0x001a_c320);
            Addr(at(c.g[S0][0], 0xc4)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0031_7670, 0x001a_c320)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001a_c330);
            c.g[A1][0] = 0;
            callee::POPUP_SET_OPTION_TEXT_ID.call(c, 0x001a_c330)?;
            c.g[V0][0] = sx(0x80d);
            c.g[A0][0] = c.g[S2][0];
            Addr(at(c.g[S0][0], 0x150)).sw(c, c.g[V0][0]);
            c.g[RA][0] = sx(0x001a_c344);
            c.g[A1][0] = c.g[S0][0];
            callee::NETFE_STATE_SHOW_POPUP.call(c, 0x001a_c344)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        c.lq(16, frame.at(c, 0x30));
        c.lq(17, frame.at(c, 0x20));
        c.lq(18, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEUseExistingAccount_CheckAccountName(self)` (0x1ac360): the same
/// for the account name (option text from the interface's `+0x48` string).
pub fn cfe_use_existing_account_check_account_name(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 11;
        c.g[V0][0] = lw(c, gp(c, -0xc8));
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001a_c384);
        c.g[A0][0] = lw(c, at(c.g[V0][0], 0x48));
        call_ptr(c, 0x0041_6810, 0x001a_c384)?;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !t {
            c.cycles += 25;
            c.g[A0][0] = c.g[S2][0];
            callee::NETFE_STATE_GET_POPUP.call(c, 0x001a_c394)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = sx(0x0046_0000);
            c.g[V0][0] = 1;
            c.g[A0][0] = sx(0x0046_2bd0);
            Addr(at(c.g[S0][0], 0xbc)).sw(c, c.g[V0][0]);
            c.g[RA][0] = sx(0x001a_c3b0);
            c.g[S1][0] = sx(at(c.g[S0][0], 0xbc));
            call_ptr(c, 0x0031_7670, 0x001a_c3b0)?;
            c.g[V1][0] = 3;
            c.g[A0][0] = sx(0x0046_0000);
            Addr(at(c.g[S0][0], 0xc0)).sw(c, c.g[V1][0]);
            c.g[A0][0] = sx(0x0046_1618);
            c.g[RA][0] = sx(0x001a_c3c8);
            Addr(at(c.g[S0][0], 0xc4)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0031_7670, 0x001a_c3c8)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001a_c3d8);
            c.g[A1][0] = 0;
            callee::POPUP_SET_OPTION_TEXT_ID.call(c, 0x001a_c3d8)?;
            c.g[V0][0] = sx(0x80d);
            c.g[A0][0] = c.g[S2][0];
            Addr(at(c.g[S0][0], 0x150)).sw(c, c.g[V0][0]);
            c.g[RA][0] = sx(0x001a_c3ec);
            c.g[A1][0] = c.g[S0][0];
            callee::NETFE_STATE_SHOW_POPUP.call(c, 0x001a_c3ec)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        c.lq(16, frame.at(c, 0x30));
        c.lq(17, frame.at(c, 0x20));
        c.lq(18, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Chat room: open the keyboard for the message field.

/// `int cFEStateChatRoom_HandleInput(self, ev)` (0x1b38d0): if the event
/// target's virtual "is pressed" says so, open the text-entry keyboard with
/// the config popup's current option, then enable the field.
pub fn cfe_state_chat_room_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        c.cycles += 11;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = lw(c, at(c.g[A1][0], 8));
        c.g[A0][0] = lh(c, at(c.g[V1][0], 0xd8));
        c.g[V0][0] = lw(c, at(c.g[V1][0], 0xdc));
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x001b_38f4);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_38f4)?;
        let t = c.g[V0][0] != 0;
        c.g[A0][0] = sx(0x0046_0000);
        if !t {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 25;
            c.g[RA][0] = sx(0x001b_390c);
            c.g[A0][0] = sx(0x0046_3db8);
            call_ptr(c, 0x0031_7670, 0x001b_390c)?;
            c.g[A3][0] = sx(0x004a_0000);
            c.g[A2][0] = c.g[V0][0];
            c.g[T0][0] = 0;
            c.g[T1][0] = 4;
            c.g[T2][0] = 0x3f;
            c.g[A3][0] = sx(0x004a_1408);
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001b_3930);
            c.g[A1][0] = 0;
            callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID.call(c, 0x001b_3930)?;
            c.g[V0][0] = lw(c, at(c.g[S0][0], 0x69c));
            c.g[A1][0] = 1;
            Addr(at(c.g[V0][0], 0x43c)).sw(c, 0);
            c.g[RA][0] = sx(0x001b_3944);
            c.g[A0][0] = lw(c, at(c.g[S0][0], 0x69c));
            call_ptr(c, 0x001c_e468, 0x001b_3944)?;
            c.g[A0][0] = lw(c, at(c.g[S0][0], 0x69c));
            c.g[A1][0] = 0x4b;
            c.g[A2][0] = 1;
            c.g[RA][0] = sx(0x001b_3958);
            c.g[A3][0] = 2;
            call_ptr(c, 0x001c_e3c8, 0x001b_3958)?;
            c.g[A0][0] = lw(c, at(c.g[S0][0], 0x69c));
            c.g[V1][0] = 1;
            c.g[V0][0] = 1;
            Addr(at(c.g[A0][0], 0x438)).sw(c, c.g[V1][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// Lobby manager: create the peer table.

/// `void LobbyMan_CreatePeerTable(LobbyMan* self)` (0x25a650): if the network
/// record exists and is non-empty, create the game-util object and advertise
/// the record on it.
pub fn lobby_man_create_peer_table(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'outer: {
            'mid: {
                c.cycles += 8;
                frame.save(c, S1, 0x10);
                frame.save(c, S0, 0x20);
                c.g[S1][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[V0][0] = lw(c, at(c.g[S1][0], 0xc));
                let t = c.g[V0][0] == 0;
                frame.restore(c, S0, 0x20);
                if t {
                    break 'mid;
                }
                c.cycles += 3;
                c.g[V0][0] = lw(c, at(c.g[S1][0], 0xf4));
                if c.g[V0][0] != 0 {
                    frame.restore(c, S1, 0x10);
                    break 'outer;
                }
                'give_up: {
                    c.cycles += 6;
                    c.g[A1][0] = 0x7065_0000;
                    c.g[A0][0] = lw(c, at(c.g[S1][0], 0x54));
                    c.g[RA][0] = sx(0x0025_a68c);
                    c.g[A1][0] |= 0x7273;
                    call_ptr(c, 0x003e_8a70, 0x0025_a68c)?;
                    let t = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x7065_0000;
                    if t {
                        break 'give_up;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = lw(c, at(c.g[S1][0], 0x54));
                    c.g[RA][0] = sx(0x0025_a6a0);
                    c.g[A1][0] |= 0x7273;
                    call_ptr(c, 0x003e_8a70, 0x0025_a6a0)?;
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
                    let t = c.g[V1][0] == 0;
                    frame.restore(c, S0, 0x20);
                    if t {
                        break 'mid;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0025_a6b4);
                    call_ptr(c, 0x003e_6e08, 0x0025_a6b4)?;
                    let t = c.g[V0][0] == 0;
                    Addr(at(c.g[S1][0], 0xf4)).sw(c, c.g[V0][0]);
                    if t {
                        break 'give_up;
                    }
                    c.cycles += 10;
                    c.g[A0][0] = lw(c, at(c.g[S1][0], 0x54));
                    c.g[S0][0] = sx(0x0048_0000);
                    c.g[A1][0] = 0x7065_0000;
                    c.g[S0][0] = sx(0x0048_13a0);
                    c.g[RA][0] = sx(0x0025_a6d4);
                    c.g[A1][0] |= 0x7273;
                    call_ptr(c, 0x003e_8a70, 0x0025_a6d4)?;
                    c.g[A0][0] = lw(c, at(c.g[S1][0], 0xf4));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0025_a6e4);
                    c.g[A2][0] = c.g[V0][0];
                    call_ptr(c, 0x003e_7120, 0x0025_a6e4)?;
                    break 'give_up;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
                break 'mid;
            }
            c.cycles += 1;
            frame.restore(c, S1, 0x10);
            break 'outer;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script engine: spawn a chosen script.

/// `int ScriptEngine_SpawnChosenScript(engine, category, a, b, c)` (0x279528):
/// choose the script, take a free node and start it; on failure push the node
/// back and return the error.
pub fn script_engine_spawn_chosen_script(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'end: {
            c.cycles += 16;
            frame.save(c, S2, 0x40);
            frame.save(c, S3, 0x30);
            c.g[S2][0] = c.g[A0][0];
            frame.save(c, S4, 0x20);
            c.g[S3][0] = c.g[A2][0];
            frame.save(c, S5, 0x10);
            c.g[S4][0] = c.g[A3][0];
            frame.save(c, S1, 0x50);
            frame.save(c, S0, 0x60);
            frame.save_ra(c, 0);
            c.g[S5][0] = c.g[T0][0];
            callee::SCRIPT_ENGINE_CHOOSE_SCRIPT.call(c, 0x0027_955c)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S1][0] == 0;
            c.g[V0][0] = sx(0xffff_ffff);
            if t {
                break 'end;
            }
            c.cycles += 13;
            c.g[RA][0] = sx(0x0027_9570);
            c.g[A0][0] = c.g[S2][0];
            callee::SCRIPT_ENGINE_POP_FREE_NODE.call(c, 0x0027_9570)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            Addr(c.g[S0][0] as u32).sw(c, c.g[S3][0]);
            c.g[A2][0] = c.g[S0][0];
            Addr(at(c.g[S0][0], 4)).sw(c, c.g[S4][0]);
            Addr(at(c.g[S0][0], 8)).sw(c, c.g[S5][0]);
            c.g[RA][0] = sx(0x0027_9590);
            c.g[A1][0] = lw(c, c.g[S1][0] as u32);
            callee::SCRIPT_ENGINE_START_SCRIPT.call(c, 0x0027_9590)?;
            c.g[S1][0] = c.g[V0][0];
            let t = (c.g[S1][0] as i64) >= 0;
            c.g[V0][0] = c.g[S1][0];
            if !t {
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0027_95a8);
                c.g[A1][0] = c.g[S0][0];
                callee::SCRIPT_ENGINE_PUSH_FREE_NODE.call(c, 0x0027_95a8)?;
                c.g[V0][0] = c.g[S1][0];
            }
            break 'end;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_v02(engine, obj)` (0x279d00): add the script object and
/// either attach its camera or set its transition, returning the object.
pub fn script_engine_v02(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        'end: {
            'plain: {
                c.cycles += 13;
                frame.save(c, S0, 0x30);
                frame.save(c, S1, 0x20);
                c.g[S0][0] = c.g[A1][0];
                frame.save(c, S2, 0x10);
                frame.save_ra(c, 0);
                c.g[RA][0] = sx(0x0027_9d20);
                c.g[S1][0] = c.g[A0][0];
                callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ.call(c, 0x0027_9d20)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V1][0] = lw(c, c.g[S0][0] as u32);
                c.g[V0][0] = 1;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 2;
                if !t {
                    c.cycles += 9;
                    c.g[A0][0] = lw(c, at(c.g[S0][0], 0x7c));
                    c.g[V0][0] = 0x120;
                    c.g[A1][0] = c.g[S0][0];
                    c.mult(0, 4, 4, 2);
                    c.g[A0][0] = sx(at(c.g[A0][0], 0x6d0));
                    c.g[RA][0] = sx(0x0027_9d50);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    callee::SCRIPT_VIEW_OBJ_ADD_CAM.call(c, 0x0027_9d50)?;
                    c.g[V0][0] = c.g[S2][0];
                    break 'end;
                }
                break 'plain;
            }
            c.cycles += 2;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[S2][0];
            if t {
                break 'end;
            }
            c.cycles += 12;
            c.g[V1][0] = lw(c, at(c.g[S0][0], 0xc));
            c.g[A0][0] = lh(c, at(c.g[V1][0], 0x48));
            c.g[V0][0] = lw(c, at(c.g[V1][0], 0x4c));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0027_9d74);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_9d74)?;
            c.g[V1][0] = 0x120;
            c.g[A1][0] = c.g[S0][0];
            c.mult(0, 2, 2, 3);
            c.g[V0][0] = sx(at(c.g[V0][0], 0x6d0));
            c.g[RA][0] = sx(0x0027_9d8c);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            callee::SCRIPT_VIEW_OBJ_SET_TRANSITION.call(c, 0x0027_9d8c)?;
            c.g[V0][0] = c.g[S2][0];
            break 'end;
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

// ---------------------------------------------------------------------------
// Menu system: push a page item.

/// `void MenuSys_PushPage(MenuSys* menu, item)` (0x2cc3b8): notify the item
/// below the page, append the item to the page's array, notify it with the new
/// index and mark the menu active.
pub fn menu_sys_push_page(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        c.cycles += 8;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, c.g[S0][0] as u32);
        let t = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !t {
            c.cycles += 9;
            c.g[V0][0] = sx(at(c.g[V0][0], 0xffff_ffff));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = lw(c, at(c.g[V0][0], 4));
            c.g[V1][0] = lw(c, at(c.g[A1][0], 0x12c));
            c.g[A0][0] = lh(c, at(c.g[V1][0], 0x38));
            c.g[V0][0] = lw(c, at(c.g[V1][0], 0x3c));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x002c_c3fc);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002c_c3fc)?;
        }
        c.cycles += 25;
        c.g[A0][0] = lw(c, c.g[S0][0] as u32);
        c.g[A2][0] = sx(at(c.g[S0][0], 4));
        c.g[A1][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx(at(c.g[A0][0], 1));
        Addr(c.g[V0][0] as u32).sw(c, c.g[S1][0]);
        c.g[V1][0] = sx(at(c.g[A0][0], 0xffff_ffff));
        Addr(c.g[S0][0] as u32).sw(c, c.g[A0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A3][0] = lw(c, c.g[A2][0] as u32);
        c.g[V0][0] = lw(c, at(c.g[A3][0], 0x12c));
        c.g[A0][0] = lh(c, at(c.g[V0][0], 0x30));
        c.g[V1][0] = lw(c, at(c.g[V0][0], 0x34));
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x002c_c440);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_c440)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x002c_c44c);
        c.g[A1][0] = 1;
        callee::MENU_SYS_SET_ACTIVE.call(c, 0x002c_c44c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu system: vector-label item draw.

/// `void MenuSysVectorLabelItem_Draw(item, ctx)` (0x2d0500): convert the
/// label's position to three doubles for `sprintf`, then draw the row.
pub fn menu_sys_vector_label_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0xe0);
        c.cycles += 42;
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        c.g[S0][0] = sx(0x0048_0000);
        frame.save(c, S2, 0xb0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0xa0);
        c.g[S0][0] = sx(0x0048_6720);
        frame.save(c, S4, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S4][0] = c.g[A1][0];
        c.g[V0][0] = lw(c, at(c.g[S1][0], 0x14));
        c.g[RA][0] = sx(0x002d_0538);
        c.lwc1(12, c.g[V0][0] as u32);
        call_ptr(c, 0x0041_3af8, 0x002d_0538)?;
        c.g[V1][0] = lw(c, at(c.g[S1][0], 0x14));
        c.g[S3][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002d_0548);
        c.lwc1(12, at(c.g[V1][0], 4));
        call_ptr(c, 0x0041_3af8, 0x002d_0548)?;
        c.g[V1][0] = lw(c, at(c.g[S1][0], 0x14));
        c.g[S2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002d_0558);
        c.lwc1(12, at(c.g[V1][0], 8));
        call_ptr(c, 0x0041_3af8, 0x002d_0558)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = sx(frame.at(c, 0));
        c.g[RA][0] = sx(0x002d_0570);
        c.g[T0][0] = c.g[V0][0];
        call_ptr(c, 0x0041_7828, 0x002d_0570)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = lw(c, at(c.g[A0][0], 0x18));
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x002d_0588);
        c.g[T0][0] = sx(frame.at(c, 0));
        callee::MENU_SYS_ITEM_DRAW_ROW.call(c, 0x002d_0588)?;
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore_ra(c, 0x80);
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
                    c.g[V1][0] = lw(c, at(c.g[S1][0], 0x14));
                    c.mult(0, 16, 16, 2);
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[RA][0] = sx(0x0027_29dc);
                    c.g[A0][0] = sx(at(c.g[S0][0], 8));
                    callee::INSEQ_INSTANCE_RELEASE.call(c, 0x0027_29dc)?;
                    c.g[V1][0] = lw(c, at(c.g[S0][0], 0x54));
                    let t = c.g[V1][0] != 0;
                    c.g[V0][0] = lw(c, at(c.g[S0][0], 0x50));
                    if t {
                        break 'inner;
                    }
                    c.cycles += 5;
                    Addr(at(c.g[S1][0], 0x1c)).sw(c, c.g[V0][0]);
                    Addr(at(c.g[S0][0], 0x50)).sw(c, 0);
                    c.g[V0][0] = lw(c, at(c.g[S1][0], 0x1c));
                    if c.g[V0][0] != 0 {
                        Addr(at(c.g[V0][0], 0x54)).sw(c, 0);
                        break 'mid;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = lw(c, at(c.g[S1][0], 0x18));
                    break 'outer;
                }
                {
                    c.cycles += 4;
                    Addr(at(c.g[V1][0], 0x50)).sw(c, c.g[V0][0]);
                    c.g[V1][0] = lw(c, at(c.g[S0][0], 0x50));
                    if c.g[V1][0] == 0 {
                        Addr(at(c.g[S0][0], 0x50)).sw(c, 0);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, at(c.g[S0][0], 0x54));
                        Addr(at(c.g[V1][0], 0x54)).sw(c, c.g[V0][0]);
                        Addr(at(c.g[S0][0], 0x50)).sw(c, 0);
                    }
                }
                c.cycles += 1;
                Addr(at(c.g[S0][0], 0x54)).sw(c, 0);
                break 'mid;
            }
            c.cycles += 1;
            c.g[V0][0] = lw(c, at(c.g[S1][0], 0x18));
            break 'outer;
        }
        {
            c.cycles += 4;
            Addr(at(c.g[S0][0], 0x50)).sw(c, c.g[V0][0]);
            c.g[V0][0] = lw(c, at(c.g[S1][0], 0x18));
            if c.g[V0][0] != 0 {
                Addr(at(c.g[V0][0], 0x54)).sw(c, c.g[S0][0]);
            }
        }
        c.cycles += 6;
        Addr(at(c.g[S1][0], 0x18)).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ASCII sprintf wrappers (UTF-8 argument to UTF-16 output).

/// `void UStr_SprintfAsciiFmt(char* buf, const char* fmt, ...)` (0x2c2718):
/// widen the (signed-char) format bytes into a stack UTF-16 buffer and run
/// `UStr_vsprintf` with the vararg save area.
pub fn ustr_sprintf_ascii_fmt(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x8b0);
        'end: {
            c.cycles += 22;
            frame.save(c, S1, 0x820);
            frame.save(c, S2, 0x810);
            c.g[S1][0] = c.g[A1][0];
            frame.save(c, S0, 0x830);
            c.g[S2][0] = c.g[A0][0];
            Addr(frame.at(c, 0x880)).sd(c, c.g[A2][0]);
            c.g[S0][0] = sp_of(c);
            frame.save_ra(c, 0x800);
            c.g[A0][0] = c.g[S1][0];
            Addr(frame.at(c, 0x888)).sd(c, c.g[A3][0]);
            Addr(frame.at(c, 0x890)).sd(c, c.g[T0][0]);
            Addr(frame.at(c, 0x898)).sd(c, c.g[T1][0]);
            Addr(frame.at(c, 0x8a0)).sd(c, c.g[T2][0]);
            Addr(frame.at(c, 0x8a8)).sd(c, c.g[T3][0]);
            c.swc1(12, frame.at(c, 0x870));
            c.swc1(14, frame.at(c, 0x874));
            c.swc1(16, frame.at(c, 0x878));
            c.g[RA][0] = sx(0x002c_2768);
            c.swc1(18, frame.at(c, 0x87c));
            call_ptr(c, 0x0041_6810, 0x002c_2768)?;
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A2][0] = sx(frame.at(c, 0x880));
            if t {
                break 'end;
            }
            c.cycles += 2;
            c.g[V1][0] = c.g[V0][0];
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lbu(c);
                c.g[V1][0] = sx(at(c.g[V1][0], 0xffff_ffff));
                c.g[S1][0] = sx(at(c.g[S1][0], 1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                Addr(c.g[S0][0] as u32).sh(c, c.g[V0][0]);
                let t = c.g[V1][0] != 0;
                c.g[S0][0] = sx(at(c.g[S0][0], 2));
                if !t {
                    break 'end;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x002c_2778);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S2][0];
        Addr(c.g[S0][0] as u32).sh(c, 0);
        c.g[RA][0] = sx(0x002c_27a8);
        c.g[A1][0] = sp_of(c);
        call_ptr(c, 0x003d_c450, 0x002c_27a8)?;
        frame.restore(c, S0, 0x830);
        frame.restore(c, S1, 0x820);
        frame.restore(c, S2, 0x810);
        frame.restore_ra(c, 0x800);
        frame.close(c);
        Ok(())
    })
}

/// `int UStr_SprintfAsciiFmt_2(char* buf, const char* fmt, ...)` (0x2c27c0):
/// the same as [`ustr_sprintf_ascii_fmt`].
pub fn ustr_sprintf_ascii_fmt_2(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x8b0);
        'end: {
            c.cycles += 22;
            frame.save(c, S1, 0x820);
            frame.save(c, S2, 0x810);
            c.g[S1][0] = c.g[A1][0];
            frame.save(c, S0, 0x830);
            c.g[S2][0] = c.g[A0][0];
            Addr(frame.at(c, 0x880)).sd(c, c.g[A2][0]);
            c.g[S0][0] = sp_of(c);
            frame.save_ra(c, 0x800);
            c.g[A0][0] = c.g[S1][0];
            Addr(frame.at(c, 0x888)).sd(c, c.g[A3][0]);
            Addr(frame.at(c, 0x890)).sd(c, c.g[T0][0]);
            Addr(frame.at(c, 0x898)).sd(c, c.g[T1][0]);
            Addr(frame.at(c, 0x8a0)).sd(c, c.g[T2][0]);
            Addr(frame.at(c, 0x8a8)).sd(c, c.g[T3][0]);
            c.swc1(12, frame.at(c, 0x870));
            c.swc1(14, frame.at(c, 0x874));
            c.swc1(16, frame.at(c, 0x878));
            c.g[RA][0] = sx(0x002c_2810);
            c.swc1(18, frame.at(c, 0x87c));
            call_ptr(c, 0x0041_6810, 0x002c_2810)?;
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[A2][0] = sx(frame.at(c, 0x880));
            if t {
                break 'end;
            }
            c.cycles += 2;
            c.g[V1][0] = c.g[V0][0];
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lbu(c);
                c.g[V1][0] = sx(at(c.g[V1][0], 0xffff_ffff));
                c.g[S1][0] = sx(at(c.g[S1][0], 1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                Addr(c.g[S0][0] as u32).sh(c, c.g[V0][0]);
                let t = c.g[V1][0] != 0;
                c.g[S0][0] = sx(at(c.g[S0][0], 2));
                if !t {
                    break 'end;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x002c_2820);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S2][0];
        Addr(c.g[S0][0] as u32).sh(c, 0);
        c.g[RA][0] = sx(0x002c_2850);
        c.g[A1][0] = sp_of(c);
        call_ptr(c, 0x003d_c450, 0x002c_2850)?;
        frame.restore(c, S0, 0x830);
        frame.restore(c, S1, 0x820);
        frame.restore(c, S2, 0x810);
        frame.restore_ra(c, 0x800);
        frame.close(c);
        Ok(())
    })
}

/// The current stack pointer.
#[inline(always)]
fn sp_of(c: &Cpu) -> u64 {
    sx(crate::call::sp(c))
}
// ---------------------------------------------------------------------------
// Memory-card manager formatting helpers.

/// `void MCManager_State0B_Format(MCManager* self)` (0x23f050): count the
/// outstanding formats down; at zero, rebuild the status text and the prompt.
pub fn mc_manager_state0b_format(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'outer: {
            'mid: {
                c.cycles += 9;
                frame.save(c, S0, 0x10);
                frame.save_ra(c, 0);
                c.g[S0][0] = c.g[A0][0];
                c.g[V0][0] = lw(c, at(c.g[S0][0], 0x108));
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = sx(at(c.g[V0][0], 0xffff_ffff));
                let t = (c.g[V1][0] as i64) > 0;
                Addr(at(c.g[S0][0], 0x108)).sw(c, c.g[V0][0]);
                if t {
                    break 'mid;
                }
                c.cycles += 8;
                c.g[A1][0] = lw(c, at(c.g[S0][0], 0x434));
                c.g[V1][0] = lw(c, c.g[A1][0] as u32);
                c.g[A0][0] = lh(c, at(c.g[V1][0], 0x58));
                c.g[V0][0] = lw(c, at(c.g[V1][0], 0x5c));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0023_f08c);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_f08c)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'outer;
                }
                'retry: {
                    c.cycles += 8;
                    c.g[A1][0] = lw(c, at(c.g[S0][0], 0x434));
                    c.g[V1][0] = lw(c, c.g[A1][0] as u32);
                    c.g[A0][0] = lh(c, at(c.g[V1][0], 0x48));
                    c.g[V0][0] = lw(c, at(c.g[V1][0], 0x4c));
                    let target = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0023_f0ac);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_f0ac)?;
                    let t = c.g[V0][0] == 0;
                    c.g[A1][0] = 0xc;
                    if t {
                        break 'retry;
                    }
                    c.cycles += 7;
                    c.g[V0][0] = lw(c, at(c.g[S0][0], 0x748));
                    c.g[A0][0] = lh(c, at(c.g[V0][0], 8));
                    c.g[V1][0] = lw(c, at(c.g[V0][0], 0xc));
                    let target = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0023_f0c8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_f0c8)?;
                    frame.restore(c, S0, 0x10);
                    break 'outer;
                }
                c.cycles += 6;
                c.g[V0][0] = lw(c, at(c.g[S0][0], 0x748));
                c.g[A1][0] = 0xd;
                c.g[A0][0] = lh(c, at(c.g[V0][0], 8));
                c.g[V1][0] = lw(c, at(c.g[V0][0], 0xc));
                let target = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0023_f0e8);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_f0e8)?;
                break 'mid;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
            break 'outer;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State28_Format(MCManager* self)` (0x240b88): the same for
/// the second card state (prompt ids 0x29 / 0x2a).
pub fn mc_manager_state28_format(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'outer: {
            'mid: {
                c.cycles += 9;
                frame.save(c, S0, 0x10);
                frame.save_ra(c, 0);
                c.g[S0][0] = c.g[A0][0];
                c.g[V0][0] = lw(c, at(c.g[S0][0], 0x108));
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = sx(at(c.g[V0][0], 0xffff_ffff));
                let t = (c.g[V1][0] as i64) > 0;
                Addr(at(c.g[S0][0], 0x108)).sw(c, c.g[V0][0]);
                if t {
                    break 'mid;
                }
                c.cycles += 8;
                c.g[A1][0] = lw(c, at(c.g[S0][0], 0x434));
                c.g[V1][0] = lw(c, c.g[A1][0] as u32);
                c.g[A0][0] = lh(c, at(c.g[V1][0], 0x58));
                c.g[V0][0] = lw(c, at(c.g[V1][0], 0x5c));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0024_0bc4);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0bc4)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'outer;
                }
                'retry: {
                    c.cycles += 8;
                    c.g[A1][0] = lw(c, at(c.g[S0][0], 0x434));
                    c.g[V1][0] = lw(c, c.g[A1][0] as u32);
                    c.g[A0][0] = lh(c, at(c.g[V1][0], 0x48));
                    c.g[V0][0] = lw(c, at(c.g[V1][0], 0x4c));
                    let target = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0024_0be4);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0024_0be4)?;
                    let t = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x29;
                    if t {
                        break 'retry;
                    }
                    c.cycles += 7;
                    c.g[V0][0] = lw(c, at(c.g[S0][0], 0x748));
                    c.g[A0][0] = lh(c, at(c.g[V0][0], 8));
                    c.g[V1][0] = lw(c, at(c.g[V0][0], 0xc));
                    let target = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0024_0c00);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0024_0c00)?;
                    frame.restore(c, S0, 0x10);
                    break 'outer;
                }
                c.cycles += 6;
                c.g[V0][0] = lw(c, at(c.g[S0][0], 0x748));
                c.g[A1][0] = 0x2a;
                c.g[A0][0] = lh(c, at(c.g[V0][0], 8));
                c.g[V1][0] = lw(c, at(c.g[V0][0], 0xc));
                let target = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0024_0c20);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0c20)?;
                break 'mid;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
            break 'outer;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Popup confirm.

/// `void cFEPopupConfirm_v06(self)` (0x1c5b88): with a live popup, fade it
/// out, select its zero-th option and hand its result to the screen.
pub fn cfe_popup_confirm_v06(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        c.cycles += 7;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, at(c.g[S0][0], 0x6c));
        if c.g[V0][0] != 0 {
            c.cycles += 32;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[RA][0] = sx(0x001c_5bb4);
            call_ptr(c, 0x001c_5f20, 0x001c_5bb4)?;
            c.g[A2][0] = lw(c, at(c.g[S0][0], 0x6c));
            c.g[A1][0] = 0;
            c.g[V0][0] = lw(c, at(c.g[A2][0], 8));
            c.g[A0][0] = lh(c, at(c.g[V0][0], 0x40));
            c.g[V1][0] = lw(c, at(c.g[V0][0], 0x44));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c_5bd0);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_5bd0)?;
            c.g[A2][0] = lw(c, at(c.g[S0][0], 0x2c4));
            c.g[A0][0] = c.g[S0][0];
            c.g[A3][0] = lw(c, at(c.g[S0][0], 0x30c));
            c.g[RA][0] = sx(0x001c_5be4);
            c.g[A1][0] = lw(c, at(c.g[S0][0], 0x6c));
            call_ptr(c, 0x001c_75c8, 0x001c_5be4)?;
            c.g[A2][0] = lw(c, at(c.g[S0][0], 0x6c));
            c.g[A1][0] = 1;
            c.g[V0][0] = lw(c, at(c.g[A2][0], 8));
            c.g[A0][0] = lh(c, at(c.g[V0][0], 0x38));
            c.g[V1][0] = lw(c, at(c.g[V0][0], 0x3c));
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c_5c00);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_5c00)?;
            c.g[T0][0] = lw(c, at(c.g[S0][0], 8));
            c.g[A2][0] = 5;
            c.g[A1][0] = lw(c, at(c.g[S0][0], 0x6c));
            c.g[A3][0] = 0;
            c.g[A0][0] = lh(c, at(c.g[T0][0], 0xa0));
            c.g[V0][0] = lw(c, at(c.g[T0][0], 0xa4));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001c_5c20);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_5c20)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DebugMenu_FindNextItem(menu, index, op)` (0x17a220): starting at
/// `(index + 1) % count`, dispatch the op on each item until one accepts it.
pub fn debug_menu_find_next_item(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        'end: {
            'found: {
                c.cycles += 14;
                c.g[A1][0] = sx(at(c.g[A1][0], 1));
                frame.save(c, S0, 0x40);
                frame.save(c, S1, 0x30);
                frame.save(c, S2, 0x20);
                frame.save(c, S3, 0x10);
                c.g[S1][0] = 0;
                c.g[S2][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[S3][0] = c.g[A2][0];
                c.g[V0][0] = lw(c, c.g[S2][0] as u32);
                c.div(0, 5, 2);
                c.cycles += 3;
                c.g[V1][0] = c.hi[0];
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S0][0] = c.g[V1][0];
                if t {
                    break 'found;
                }
                c.cycles += 1;
                loop {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0017_a278);
                    c.g[A2][0] = c.g[S3][0];
                    callee::DEBUG_MENU_DISPATCH_ITEM_OP.call(c, 0x0017_a278)?;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S0][0];
                    if t {
                        break 'end;
                    }
                    {
                        c.cycles += 6;
                        c.g[V0][0] = lw(c, c.g[S2][0] as u32);
                        c.g[V1][0] = sx(at(c.g[S0][0], 1));
                        c.g[S1][0] = sx(at(c.g[S1][0], 1));
                        c.div(0, 3, 2);
                    }
                    c.cycles += 4;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[A0][0] = c.hi[0];
                    let t = c.g[V0][0] != 0;
                    c.g[S0][0] = c.g[A0][0];
                    if !t {
                        break 'found;
                    }
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x0017_a268);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx(0xffff_ffff);
            break 'end;
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

/// `void LobbyMenu_Login(menu, a1, a2)` (0x17e118): pop the menu's pages,
/// copy the two account strings into the interface and log in.
pub fn lobby_menu_login(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 12;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(at(c.g[S1][0], 8));
        c.g[V0][0] = lw(c, at(c.g[S1][0], 8));
        let t = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A2][0];
        if !t {
            loop {
                c.cycles += 5;
                c.g[RA][0] = sx(0x0017_e150);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_c460, 0x0017_e150)?;
                c.g[V0][0] = lw(c, c.g[S0][0] as u32);
                let t = c.g[V0][0] != 0;
                if !t {
                    break;
                }
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0017_e148);
                }
            }
        }
        c.cycles += 25;
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0017_e168);
        c.g[A0][0] = sx(at(c.g[S1][0], 0x1c04));
        call_ptr(c, 0x0031_84e8, 0x0017_e168)?;
        c.g[A0][0] = sx(at(c.g[S1][0], 0x1c08));
        c.g[RA][0] = sx(0x0017_e174);
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0031_84e8, 0x0017_e174)?;
        c.g[A0][0] = lw(c, gp(c, -0xc8));
        c.g[V0][0] = 5;
        c.g[A1][0] = lw(c, at(c.g[S1][0], 0x1c04));
        c.g[A0][0] = sx(at(c.g[A0][0], 0x48));
        c.g[RA][0] = sx(0x0017_e18c);
        Addr(at(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_84e8, 0x0017_e18c)?;
        c.g[A0][0] = lw(c, gp(c, -0xc8));
        c.g[A1][0] = lw(c, at(c.g[S1][0], 0x1c08));
        c.g[RA][0] = sx(0x0017_e19c);
        c.g[A0][0] = sx(at(c.g[A0][0], 0x4c));
        call_ptr(c, 0x0031_84e8, 0x0017_e19c)?;
        c.g[RA][0] = sx(0x0017_e1a4);
        c.g[A0][0] = lw(c, gp(c, -0xc8));
        call_ptr(c, 0x0025_c610, 0x0017_e1a4)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio system: unload front-end audio / song changed.

/// `void SSXAudioSystem_UnloadFrontEndAudio(self)` (0x286200): stop the
/// front-end music (depending on the two song slots), kill bank timers and
/// unload the front-end bank.
pub fn ssx_audio_system_unload_front_end_audio(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'end: {
            'second: {
                'first: {
                    c.cycles += 8;
                    c.g[V1][0] = 2;
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = lw(c, at(c.g[S0][0], 0x608c));
                    let t = c.g[V0][0] == c.g[V1][0];
                    if t {
                        break 'first;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, at(c.g[S0][0], 0x6090));
                    if c.g[V0][0] != c.g[V1][0] {
                        break 'second;
                    }
                    break 'first;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x0028_6234);
                c.g[A0][0] = sx(at(c.g[S0][0], 0x118));
                call_ptr(c, 0x002b_3ac0, 0x0028_6234)?;
                c.g[A1][0] = 0;
                break 'end;
            }
            c.cycles += 5;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[RA][0] = sx(0x0028_624c);
            c.g[A0][0] = sx(at(c.g[S0][0], 0x118));
            call_ptr(c, 0x002b_3d48, 0x0028_624c)?;
            c.g[A1][0] = 0;
            break 'end;
        }
        c.cycles += 22;
        c.g[RA][0] = sx(0x0028_6258);
        c.g[A0][0] = c.g[S0][0];
        callee::SSX_AUDIO_SYSTEM_SET_MODE.call(c, 0x0028_6258)?;
        c.g[V0][0] = lw(c, at(c.g[S0][0], 0x118));
        c.g[A0][0] = lw(c, c.g[V0][0] as u32);
        c.g[RA][0] = sx(0x0028_6268);
        c.g[A0][0] = sx(at(c.g[A0][0], 0x1d8));
        call_ptr(c, 0x002a_dda0, 0x0028_6268)?;
        c.g[V0][0] = lw(c, at(c.g[S0][0], 0x118));
        c.g[A0][0] = lw(c, c.g[V0][0] as u32);
        c.g[V1][0] = lw(c, at(c.g[A0][0], 0xab0));
        c.g[A0][0] = sx(at(c.g[A0][0], 0x1d8));
        c.g[V0][0] = lh(c, at(c.g[V1][0], 0x10));
        c.g[A1][0] = lw(c, at(c.g[V1][0], 0x14));
        let target = c.g[A1][0] as u32;
        c.g[RA][0] = sx(0x0028_6288);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, target, 0x0028_6288)?;
        c.g[V0][0] = lw(c, at(c.g[S0][0], 0x118));
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0028_6298);
        c.g[A0][0] = lw(c, c.g[V0][0] as u32);
        callee::SND_BANK_MAN_UNLOAD_BANK.call(c, 0x0028_6298)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_OnSongChanged(self)` (0x28f478): play the new song,
/// and if its gain is audible, push the song title to the active music state.
pub fn ssx_audio_system_on_song_changed(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'outer: {
            'inner: {
                c.cycles += 15;
                frame.save(c, S0, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[RA][0] = sx(0x0028_f490);
                c.g[A0][0] = sx(at(c.g[S0][0], 0x118));
                callee::MUSIC_PLAYER_PLAY_SONG.call(c, 0x0028_f490)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x0028_f4a0);
                c.g[A2][0] = 0;
                callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN.call(c, 0x0028_f4a0)?;
                c.mtc1(0, 1);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = lw(c, gp(c, -0x848));
                if t {
                    break 'inner;
                }
                c.cycles += 3;
                c.g[V0][0] = lw(c, at(c.g[V0][0], 0x84));
                let t = c.g[V0][0] == 0;
                frame.restore(c, S0, 0x10);
                if t {
                    break 'outer;
                }
                c.cycles += 3;
                c.g[A0][0] = lw(c, at(c.g[V0][0], 0x48));
                let t = c.g[A0][0] == 0;
                c.g[S0][0] = sx(at(c.g[A0][0], 0x18));
                if t {
                    break 'inner;
                }
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    frame.restore(c, S0, 0x10);
                    break 'outer;
                }
                c.cycles += 9;
                c.g[A0][0] = sx(0x004a_3660);
                c.g[RA][0] = sx(0x0028_f4e0);
                call_ptr(c, 0x0031_7670, 0x0028_f4e0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0028_f4ec);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_f9d8, 0x0028_f4ec)?;
                c.g[A3][0] = c.g[V0][0];
                let t = c.g[A3][0] == 0;
                c.g[A1][0] = 5;
                if !t {
                    c.cycles += 6;
                    c.g[V1][0] = lw(c, at(c.g[A3][0], 8));
                    c.g[A2][0] = 0;
                    c.g[A0][0] = lh(c, at(c.g[V1][0], 0xc0));
                    c.g[V0][0] = lw(c, at(c.g[V1][0], 0xc4));
                    let target = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0028_f510);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0028_f510)?;
                }
                break 'inner;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
            break 'outer;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOptionsDeviceSelect_v38(self, flag)` (0x18e2c0): if the
/// state's object exists, tell both device-select items the new selected
/// flag and store the inverted flag.
pub fn cfe_state_options_device_select_v38(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 9;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, at(c.g[S1][0], 0x22c));
        let t = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !t {
            c.cycles += 25;
            c.g[A0][0] = sx(0x004a_0000);
            c.g[S0][0] = c.g[S2][0] ^ 1;
            c.g[RA][0] = sx(0x0018_e2f4);
            c.g[A0][0] = sx(0x004a_1458);
            call_ptr(c, 0x0031_7670, 0x0018_e2f4)?;
            c.g[A0][0] = lw(c, at(c.g[S1][0], 0x40));
            c.g[RA][0] = sx(0x0018_e300);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_e300)?;
            c.g[V1][0] = lw(c, at(c.g[V0][0], 8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = lh(c, at(c.g[V1][0], 0x48));
            c.g[A2][0] = lw(c, at(c.g[V1][0], 0x4c));
            let target = c.g[A2][0] as u32;
            c.g[RA][0] = sx(0x0018_e318);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_e318)?;
            c.g[A0][0] = sx(0x004a_0000);
            c.g[RA][0] = sx(0x0018_e324);
            c.g[A0][0] = sx(0x004a_1460);
            call_ptr(c, 0x0031_7670, 0x0018_e324)?;
            c.g[A0][0] = lw(c, at(c.g[S1][0], 0x40));
            c.g[RA][0] = sx(0x0018_e330);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_e330)?;
            c.g[V1][0] = lw(c, at(c.g[V0][0], 8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = lh(c, at(c.g[V1][0], 0x48));
            c.g[A2][0] = lw(c, at(c.g[V1][0], 0x4c));
            let target = c.g[A2][0] as u32;
            c.g[RA][0] = sx(0x0018_e348);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_e348)?;
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0] ^ 1;
        frame.restore(c, S0, 0x30);
        Addr(at(c.g[S1][0], 0x220)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Update(player)` (0x253938): wait for the current
/// frame, decode it, and when both video and audio are done restart.
pub fn vid_engine_player_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        {
            'first: {
                'due: {
                    c.cycles += 7;
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = lw(c, at(c.g[S0][0], 0x40));
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if t {
                        break 'due;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0025_395c);
                    c.g[A0][0] = lw(c, at(c.g[S0][0], 0x38));
                    call_ptr(c, 0x003a_e780, 0x0025_395c)?;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = 1;
                    if t {
                        break 'due;
                    }
                    c.cycles += 14;
                    c.g[A1][0] = lw(c, at(c.g[S0][0], 0x3c));
                    c.g[V0][0] = lw(c, at(c.g[A1][0], 4));
                    c.g[A0][0] = lh(c, at(c.g[V0][0], 0x30));
                    c.g[V1][0] = lw(c, at(c.g[V0][0], 0x34));
                    let target = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0025_397c);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0025_397c)?;
                    c.g[A0][0] = lw(c, at(c.g[S0][0], 0x38));
                    c.g[RA][0] = sx(0x0025_3988);
                    c.lwc1(12, at(c.g[A0][0], 0x48));
                    call_ptr(c, 0x003a_e6c8, 0x0025_3988)?;
                    c.g[V1][0] = 1;
                    Addr(at(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
                    Addr(at(c.g[S0][0], 0x40)).sw(c, c.g[V1][0]);
                    Addr(at(c.g[S0][0], 4)).sw(c, 0);
                    break 'first;
                }
                c.cycles += 1;
                Addr(at(c.g[S0][0], 4)).sw(c, c.g[V0][0]);
                break 'first;
            }
            'sec: {
                c.cycles += 3;
                c.g[V0][0] = lw(c, at(c.g[S0][0], 0xc));
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'sec;
                }
                c.cycles += 3;
                c.g[V0][0] = lw(c, c.g[S0][0] as u32);
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'sec;
                }
                c.cycles += 7;
                c.g[V0][0] = 1;
                Addr(at(c.g[S0][0], 0x40)).sw(c, 0);
                c.g[A0][0] = lw(c, at(c.g[S0][0], 0x38));
                c.g[RA][0] = sx(0x0025_39cc);
                Addr(at(c.g[S0][0], 4)).sw(c, c.g[V0][0]);
                call_ptr(c, 0x003a_e860, 0x0025_39cc)?;
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                frame.restore(c, S0, 0x10);
                break 'sec;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void UIState_dtor(UIState* self, int flags)` (0x39e390): run the list and
/// node destructors, then free the object when `flags & 1`.
pub fn ui_state_dtor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        'end: {
            c.cycles += 33;
            c.g[V0][0] = sx(0x0049_0000);
            frame.save(c, S3, 0x10);
            c.g[V0][0] = sx(0x0049_46c8);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S1, 0x30);
            frame.save(c, S0, 0x40);
            c.g[S1][0] = sx(at(c.g[S3][0], 0x24));
            frame.save(c, S2, 0x20);
            c.g[A0][0] = c.g[S1][0];
            frame.save_ra(c, 0);
            c.g[S2][0] = c.g[A1][0];
            c.g[RA][0] = sx(0x0039_e3c8);
            Addr(at(c.g[S3][0], 8)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0039_7948, 0x0039_e3c8)?;
            c.g[V0][0] = sx(0x0049_0000);
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] = sx(0x0049_4c98);
            c.g[RA][0] = sx(0x0039_e3dc);
            Addr(at(c.g[S3][0], 0x3c)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0039_7948, 0x0039_e3dc)?;
            c.g[S0][0] = sx(0x0049_0000);
            c.g[A0][0] = sx(at(c.g[S3][0], 0x30));
            c.g[S0][0] = sx(0x0049_4cc0);
            c.g[RA][0] = sx(0x0039_e3f0);
            Addr(at(c.g[S3][0], 0x38)).sw(c, c.g[S0][0]);
            call_ptr(c, 0x003a_4868, 0x0039_e3f0)?;
            Addr(at(c.g[S3][0], 0x2c)).sw(c, c.g[S0][0]);
            c.g[RA][0] = sx(0x0039_e3fc);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a_4868, 0x0039_e3fc)?;
            Addr(at(c.g[S3][0], 8)).sw(c, c.g[S0][0]);
            c.g[RA][0] = sx(0x0039_e408);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x003a_4868, 0x0039_e408)?;
            c.g[S2][0] &= 1;
            let t = c.g[S2][0] == 0;
            frame.restore(c, S0, 0x40);
            if t {
                break 'end;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x0039_e41c);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0031_7e50, 0x0039_e41c)?;
            frame.restore(c, S0, 0x40);
            break 'end;
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

/// `void WorldPainter_FreeViewQueries()` (0x2ecfa0): release the 8 view
/// queries of every painter (the table is at 0x9fa370).
pub fn world_painter_free_view_queries(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        'end: {
            c.cycles += 9;
            c.g[V0][0] = lw(c, gp(c, 0xa64));
            frame.save(c, S0, 0x50);
            frame.save(c, S1, 0x40);
            frame.save(c, S2, 0x30);
            frame.save(c, S3, 0x20);
            frame.save(c, S4, 0x10);
            let t = c.g[V0][0] == 0;
            frame.save_ra(c, 0);
            if t {
                break 'end;
            }
            c.cycles += 5;
            c.g[V0][0] = sx(0x0050_0000);
            c.g[V1][0] = 0;
            c.g[S4][0] = sx(0x004f_a370);
            c.g[S3][0] = 0xf0;
            c.mult(0, 2, 3, 19);
            loop {
                c.cycles += 4;
                c.g[S2][0] = sx(at(c.g[V1][0], 1));
                c.g[S1][0] = 8;
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                loop {
                    {
                        c.cycles += 3;
                        c.g[A2][0] = lw(c, c.g[S0][0] as u32);
                        let t = c.g[A2][0] == 0;
                        c.g[A1][0] = 3;
                        if !t {
                            c.cycles += 5;
                            c.g[V0][0] = lw(c, at(c.g[A2][0], 4));
                            c.g[A0][0] = lh(c, at(c.g[V0][0], 8));
                            c.g[V1][0] = lw(c, at(c.g[V0][0], 0xc));
                            let target = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x002e_d008);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x002e_d008)?;
                        }
                    }
                    c.cycles += 3;
                    c.g[S1][0] = sx(at(c.g[S1][0], 0xffff_ffff));
                    let t = (c.g[S1][0] as i64) >= 0;
                    c.g[S0][0] = sx(at(c.g[S0][0], 4));
                    if !t {
                        c.cycles += 4;
                        c.g[V1][0] = c.g[S2][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 8i64) as u64;
                        let t = c.g[V0][0] != 0;
                        c.mult(0, 2, 3, 19);
                        if !t {
                            break 'end;
                        }
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x002e_cfd8);
                        }
                        break;
                    }
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x002e_cfe8);
                    }
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        Addr(gp(c, 0xa64)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateTrophyRoom_ReleaseTrophyImages(self)` (0x1d4b20): for the
/// five trophy images, unlink the image object and free it.
pub fn cfe_state_trophy_room_release_trophy_images(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        c.cycles += 14;
        c.g[V1][0] = lw(c, gp(c, -0x848));
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(at(c.g[S2][0], 0x74));
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, at(c.g[V1][0], 0x7c));
        c.g[S4][0] = sx(at(c.g[V0][0], 0x1a70));
        loop {
            {
                c.cycles += 3;
                c.g[A1][0] = lw(c, at(c.g[S0][0], 0xffff_ffec));
                let t = (c.g[A1][0] as i64) < 0;
                c.g[A2][0] = lw(c, gp(c, -0x854));
                if !t {
                    c.cycles += 5;
                    c.g[V0][0] = lw(c, at(c.g[A2][0], 0x10d8));
                    c.g[A0][0] = lh(c, at(c.g[V0][0], 0x190));
                    c.g[V1][0] = lw(c, at(c.g[V0][0], 0x194));
                    let target = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d_4b78);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001d_4b78)?;
                }
            }
            {
                c.cycles += 4;
                c.g[S1][0] = sx(0xffff_ffff);
                c.g[A2][0] = lw(c, c.g[S0][0] as u32);
                let t = (c.g[A2][0] as i64) < 0;
                Addr(at(c.g[S0][0], 0xffff_ffec)).sw(c, c.g[S1][0]);
                if !t {
                    c.cycles += 3;
                    c.g[A1][0] = lw(c, at(c.g[S2][0], 0x50));
                    c.g[RA][0] = sx(0x001d_4b94);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x0019_dc20, 0x001d_4b94)?;
                }
            }
            c.cycles += 5;
            Addr(c.g[S0][0] as u32).sw(c, c.g[S1][0]);
            c.g[S3][0] = sx(at(c.g[S3][0], 1));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 5i64) as u64;
            let t = c.g[V0][0] != 0;
            c.g[S0][0] = sx(at(c.g[S0][0], 4));
            if !t {
                c.cycles += 8;
                frame.restore(c, S0, 0x50);
                frame.restore(c, S1, 0x40);
                frame.restore(c, S2, 0x30);
                frame.restore(c, S3, 0x20);
                frame.restore(c, S4, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x001d_4b58);
            }
        }
    })
}

/// `void cGameLoadScreen_Unload(self)` (0x2332e8): destroy the screen and
/// audio of the load screen, then release every remaining sub-object.
pub fn c_game_load_screen_unload(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        c.cycles += 14;
        c.g[A1][0] = lw(c, gp(c, -0x854));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, at(c.g[A1][0], 0x10d8));
        c.g[A0][0] = lh(c, at(c.g[V0][0], 0x98));
        c.g[V1][0] = lw(c, at(c.g[V0][0], 0x9c));
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0023_3314);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_3314)?;
        c.g[A0][0] = lw(c, at(c.g[S0][0], 0xc));
        if c.g[A0][0] == 0 {
            c.g[S0][0] = sx(at(c.g[S0][0], 0x10));
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0023_3328);
            c.g[A1][0] = 3;
            call_ptr(c, 0x0039_7b70, 0x0023_3328)?;
            c.g[S0][0] = sx(at(c.g[S0][0], 0x10));
        }
        c.cycles += 1;
        c.g[S1][0] = 1;
        loop {
            {
                c.cycles += 3;
                c.g[A1][0] = lw(c, c.g[S0][0] as u32);
                let t = c.g[A1][0] == 0;
                c.g[A2][0] = lw(c, gp(c, -0x854));
                if !t {
                    c.cycles += 5;
                    c.g[V0][0] = lw(c, at(c.g[A2][0], 0x10d8));
                    c.g[A0][0] = lh(c, at(c.g[V0][0], 0x190));
                    c.g[V1][0] = lw(c, at(c.g[V0][0], 0x194));
                    let target = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0023_3350);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_3350)?;
                }
            }
            c.cycles += 3;
            c.g[S1][0] = sx(at(c.g[S1][0], 0xffff_ffff));
            let t = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(at(c.g[S0][0], 4));
            if !t {
                c.cycles += 13;
                c.g[RA][0] = sx(0x0023_3364);
                c.g[A0][0] = lw(c, gp(c, -0x854));
                callee::GRAPHICS_MAN_FREE_ALL_TEXTURES.call(c, 0x0023_3364)?;
                c.g[RA][0] = sx(0x0023_336c);
                callee::SSX_AUDIO_SYSTEM_GET_INSTANCE.call(c, 0x0023_336c)?;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 12);
                c.g[RA][0] = sx(0x0023_337c);
                c.g[A0][0] = c.g[V0][0];
                callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP.call(c, 0x0023_337c)?;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0023_3330);
            }
        }
    })
}

/// `void UIListBox_SetItemWideString(listbox, index, str)` (0x399d80): replace
/// item `index`'s wide string with a copy of `str`.
pub fn ui_list_box_set_item_wide_string(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 13;
        c.g[A3][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[A1][0] &= 0xff;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = lw(c, at(c.g[A3][0], 0x5c));
        c.g[V0][0] = lw(c, at(c.g[A0][0], 0xd0));
        c.g[V1][0] = lw(c, at(c.g[V0][0], 0x10));
        c.g[A0][0] = lw(c, at(c.g[V1][0], 0x10));
        let t = c.g[A0][0] == 0;
        c.g[S2][0] = c.g[A2][0];
        if t {
            c.cycles += 6;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        {
            c.cycles += 7;
            c.g[V0][0] = 0x14;
            c.g[V1][0] = sx(at(c.g[A3][0], 0x9c));
            c.mult(0, 2, 5, 2);
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = lw(c, c.g[S1][0] as u32);
            let t = c.g[A0][0] == 0;
            if !t {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0039_9dd8);
                call_ptr(c, 0x0031_7e98, 0x0039_9dd8)?;
            }
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x0039_9de0);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x003d_bca0, 0x0039_9de0)?;
        c.g[V0][0] = sx(at(c.g[V0][0], 1));
        c.g[A1][0] = sx(0x0049_0000);
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[RA][0] = sx(0x0039_9dfc);
        c.g[A1][0] = sx(0x0049_3dc0);
        call_ptr(c, 0x0031_7e30, 0x0039_9dfc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0039_9e0c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003d_be20, 0x0039_9e0c)?;
        Addr(c.g[S1][0] as u32).sw(c, c.g[S0][0]);
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// Reward database.

/// `void cBERewardDB_init(cBERewardDB* db, int use_file)` (0x15ad58): with a
/// bolt-file name given, acquire the bolt file for that rider; otherwise load
/// and parse the rewards file. Store the result in the database's first word.
pub fn rewards_file_load(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        'end: {
            'named: {
                c.cycles += 7;
                frame.save(c, S2, 0x30);
                frame.save(c, S0, 0x20);
                c.g[S2][0] = c.g[A0][0];
                frame.save(c, S1, 0x10);
                let t = c.g[A1][0] != 0;
                frame.save_ra(c, 0);
                if t {
                    break 'named;
                }
                c.cycles += 19;
                c.g[RA][0] = sx(0x0015_ad7c);
                c.g[A0][0] = lw(c, gp(c, -0x1e80));
                call_ptr(c, 0x0031_7f60, 0x0015_ad7c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = sx(0x0046_0000);
                c.g[A3][0] = 0;
                c.g[A1][0] = sx(0x0045_ab78);
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0015_ad98);
                c.g[A2][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x0015_ad98)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = lw(c, gp(c, -0x1e80));
                c.g[A2][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0015_adac);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003e_1bb8, 0x0015_adac)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0015_adb8);
                c.g[A0][0] = c.g[S2][0];
                callee::REWARDS_FILE_PARSE.call(c, 0x0015_adb8)?;
                c.g[V0][0] = sx(0xffff_ffff);
                break 'end;
            }
            c.cycles += 9;
            c.g[V0][0] = lw(c, gp(c, -0x848));
            c.g[T0][0] = c.g[A1][0];
            c.g[A2][0] = lw(c, gp(c, -0x1e80));
            c.g[A1][0] = 0;
            c.g[A0][0] = lw(c, at(c.g[V0][0], 0x7c));
            c.g[A3][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[RA][0] = sx(0x0015_ade4);
            c.g[A0][0] = sx(at(c.g[A0][0], 0x1a70));
            callee::BOLT_FILE_ACQUIRE.call(c, 0x0015_ade4)?;
            break 'end;
        }
        c.cycles += 7;
        Addr(c.g[S2][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int GameModule_TryExit(self)` (0x230e98): if the module's virtual "can
/// exit" lets it, shut the app module down and report success.
pub fn game_module_try_exit(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'end: {
            c.cycles += 12;
            frame.save(c, S0, 0x20);
            frame.save_ra(c, 0x10);
            c.g[S0][0] = c.g[A0][0];
            c.g[A1][0] = lw(c, at(c.g[S0][0], 0x98));
            c.g[V1][0] = lw(c, c.g[A1][0] as u32);
            c.g[A0][0] = lh(c, at(c.g[V1][0], 0x18));
            c.g[V0][0] = lw(c, at(c.g[V1][0], 0x1c));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0023_0ec0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_0ec0)?;
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if !t {
                'attempt: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0023_0ed0);
                    c.g[A0][0] = c.g[S0][0];
                    callee::APP_MODULE_TRY_EXIT.call(c, 0x0023_0ed0)?;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = lw(c, gp(c, -0x854));
                    if t {
                        break 'attempt;
                    }
                    c.cycles += 21;
                    c.g[V1][0] = lw(c, at(c.g[V0][0], 0x10d8));
                    c.g[A0][0] = lh(c, at(c.g[V1][0], 0x78));
                    Addr(frame.at(c, 0)).sw(c, 0);
                    Addr(frame.at(c, 4)).sw(c, 0);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Addr(frame.at(c, 8)).sw(c, 0);
                    c.g[V0][0] = lw(c, at(c.g[V1][0], 0x7c));
                    let target = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0023_0efc);
                    c.g[A1][0] = sx(frame.at(c, 0));
                    call_ptr(c, target, 0x0023_0efc)?;
                    c.g[V1][0] = lw(c, gp(c, -0x854));
                    c.g[V0][0] = 1;
                    c.g[A0][0] = lw(c, gp(c, -0x84c));
                    Addr(at(c.g[V1][0], 0x270)).sw(c, c.g[V0][0]);
                    Addr(gp(c, 0x1234)).sw(c, 0);
                    Addr(gp(c, 0x14e8)).sw(c, 0);
                    c.g[RA][0] = sx(0x0023_0f1c);
                    Addr(gp(c, 0x12d8)).sw(c, 0);
                    call_ptr(c, 0x0027_8718, 0x0023_0f1c)?;
                    c.g[RA][0] = sx(0x0023_0f24);
                    c.g[A0][0] = lw(c, at(c.g[S0][0], 0x94));
                    call_ptr(c, 0x001f_3170, 0x0023_0f24)?;
                    c.g[V0][0] = 1;
                    break 'end;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            break 'end;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BE character database backup.

/// `void BE_BackupPlayerSetup()` (0x14ebf8): back up the local player's
/// character and copy the six rival records into the backup table.
pub fn be_backup_player_setup(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        c.cycles += 20;
        c.g[A0][0] = c.g[A1][0];
        c.g[V1][0] = 0x1c;
        c.mult(0, 3, 4, 3);
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x9b50;
        c.mult(0, 4, 4, 2);
        c.g[S0][0] = sx(0x0053_0000);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(0x0053_5b20);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V0][0] = sx(0x004a_0000);
        c.g[V0][0] = sx(0x004a_6ca8);
        c.g[A1][0] = Addr(at(c.g[V1][0], 0x11)).lb(c);
        c.g[RA][0] = sx(0x0014_ec38);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        callee::BE_BACKUP_CHARACTER.call(c, 0x0014_ec38)?;
        c.g[V0][0] = sx(0x0053_0000);
        c.g[V1][0] = sx(0x0053_4a88);
        c.g[A0][0] = sx(at(c.g[V1][0], 0xa8));
        loop {
            c.cycles += 18;
            c.ldl(2, at(c.g[S0][0], 7));
            c.ldr(2, c.g[S0][0] as u32);
            c.ldl(5, at(c.g[S0][0], 0xf));
            c.ldr(5, at(c.g[S0][0], 8));
            c.ldl(6, at(c.g[S0][0], 0x17));
            c.ldr(6, at(c.g[S0][0], 0x10));
            c.g[A3][0] = lw(c, at(c.g[S0][0], 0x18));
            c.sdl(2, at(c.g[V1][0], 7));
            c.sdr(2, c.g[V1][0] as u32);
            c.sdl(5, at(c.g[V1][0], 0xf));
            c.sdr(5, at(c.g[V1][0], 8));
            c.sdl(6, at(c.g[V1][0], 0x17));
            c.sdr(6, at(c.g[V1][0], 0x10));
            Addr(at(c.g[V1][0], 0x18)).sw(c, c.g[A3][0]);
            c.g[V1][0] = sx(at(c.g[V1][0], 0x1c));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let t = c.g[V0][0] != 0;
            c.g[S0][0] = sx(at(c.g[S0][0], 0x1c));
            if !t {
                c.cycles += 4;
                frame.restore(c, S0, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0014_ec48);
            }
        }
    })
}

/// `void BE_ResetCharRivalries(p, index)` (0x1519e0): write the three record
/// bytes for `index` into the rivalry table (10 entries of 3 bytes).
pub fn be_reset_char_rivalries(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        c.cycles += 20;
        c.g[V1][0] = 0xc;
        c.mult(0, 5, 5, 3);
        c.g[V0][0] = sx(0x0044_0000);
        c.g[V1][0] = sx(0x004a_0000);
        c.g[V0][0] = sx(0x0044_06fc);
        c.g[T2][0] = sx(0x004a_1218);
        c.g[A3][0] = Addr(c.g[T2][0] as u32).lb(c);
        c.g[T0][0] = Addr(at(c.g[T2][0], 1)).lb(c);
        c.g[T1][0] = Addr(at(c.g[T2][0], 2)).lb(c);
        Addr(frame.at(c, 0)).sb(c, c.g[A3][0]);
        Addr(frame.at(c, 1)).sb(c, c.g[T0][0]);
        Addr(frame.at(c, 2)).sb(c, c.g[T1][0]);
        c.g[A2][0] = sx(at(c.g[V0][0], 4));
        c.g[A0][0] = sx(at(c.g[A0][0], 0xbc2));
        c.g[V1][0] = 0;
        c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T0][0] = 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A2][0] = 2;
        loop {
            'next: {
                'emit: {
                    'zero_or_match: {
                        c.cycles += 9;
                        c.g[T3][0] = Addr(frame.at(c, 0)).lb(c);
                        c.g[V0][0] = Addr(frame.at(c, 1)).lb(c);
                        c.g[T1][0] = Addr(frame.at(c, 2)).lb(c);
                        Addr(at(c.g[A0][0], 0xffff_ffff)).sb(c, c.g[T3][0]);
                        Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
                        Addr(at(c.g[A0][0], 1)).sb(c, c.g[T1][0]);
                        c.g[V0][0] = lw(c, c.g[A3][0] as u32);
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = lw(c, c.g[A1][0] as u32);
                            break 'zero_or_match;
                        }
                        c.cycles += 3;
                        Addr(at(c.g[A0][0], 0xffff_ffff)).sb(c, 0);
                        Addr(c.g[A0][0] as u32).sb(c, 0);
                        break 'emit;
                    }
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = sx(at(c.g[V1][0], 1));
                        break 'next;
                    }
                    c.cycles += 2;
                    Addr(at(c.g[A0][0], 0xffff_ffff)).sb(c, c.g[T0][0]);
                    Addr(c.g[A0][0] as u32).sb(c, c.g[A2][0]);
                    break 'emit;
                }
                c.cycles += 1;
                c.g[V1][0] = sx(at(c.g[V1][0], 1));
                break 'next;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
            let t = c.g[V0][0] != 0;
            c.g[A0][0] = sx(at(c.g[A0][0], 3));
            if !t {
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x0015_1a30);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Replay highlight.

/// `int Replay_HighlightSlotScore(self, replay, mode)` (0x26e838): compare a
/// replay slot's distance (or time) against the current highlight and set it
/// when the slot is better.
pub fn replay_highlight_slot_score(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        'end: {
            'mid: {
                'obj: {
                    'inner: {
                        'dist: {
                            'cmp: {
                                c.cycles += 3;
                                let t = c.g[A2][0] == 0;
                                frame.save_ra(c, 0);
                                if t {
                                    break 'cmp;
                                }
                                c.cycles += 5;
                                c.g[V1][0] = lw(c, at(c.g[A1][0], 0x9c));
                                c.g[V0][0] = lw(c, at(c.g[A0][0], 0xa0));
                                c.g[A2][0] =
                                    sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                                let t = c.g[A2][0] == 0;
                                c.g[V0][0] = 0;
                                if t {
                                    break 'end;
                                }
                                'gate: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A2][0] as i64) < 1000i64) as u64;
                                    if c.g[V0][0] != 0 {
                                        c.lwc1(1, at(c.g[A1][0], 0x44));
                                        break 'gate;
                                    }
                                    c.cycles += 2;
                                    c.g[A1][0] = lw(c, c.g[A0][0] as u32);
                                    break 'inner;
                                }
                                c.cycles += 6;
                                c.g[AT][0] = 0x40a0_0000;
                                c.mtc1(1, 0);
                                c.c_le_s(0, 1);
                                if !c.fcc() {
                                    c.g[V1][0] = lw(c, at(c.g[A1][0], 0xa4));
                                    break 'dist;
                                }
                                c.cycles += 8;
                                c.g[AT][0] = 0x447a_0000;
                                c.mtc1(1, 0);
                                c.g[A1][0] = lw(c, c.g[A0][0] as u32);
                                c.mul_s(0, 1, 0);
                                c.cvt_w_s(1, 0);
                                c.mfc1(6, 1);
                                c.g[A0][0] = lw(c, at(c.g[A0][0], 0xb0));
                                break 'obj;
                            }
                            c.cycles += 1;
                            c.g[V1][0] = lw(c, at(c.g[A1][0], 0xa4));
                            break 'dist;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = lw(c, at(c.g[A0][0], 0xa8));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        let t = c.g[V1][0] == 0;
                        c.g[A2][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
                        if t {
                            break 'mid;
                        }
                        c.cycles += 1;
                        c.g[A1][0] = lw(c, c.g[A0][0] as u32);
                        break 'inner;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = lw(c, at(c.g[A0][0], 0xb0));
                    break 'obj;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x0026_e8c8);
                call_ptr(c, 0x0026_d730, 0x0026_e8c8)?;
                c.g[V0][0] = 1;
                break 'end;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
            break 'end;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI text constructor.

/// `UIText* UIText_ctor(UIText* self)` (0x3a0348): run the UI object
/// constructor and install the two text vtables and the default flags.
pub fn ui_text_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        c.cycles += 42;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_CTOR.call(c, 0x003a_035c)?;
        c.g[V1][0] = Addr(at(c.g[S0][0], 0x74)).lbu(c);
        c.g[V0][0] = sx(0x0049_0000);
        c.g[V0][0] = sx(0x0049_4588);
        Addr(at(c.g[S0][0], 0xa8)).sw(c, 0);
        c.g[V1][0] |= 4;
        Addr(at(c.g[S0][0], 8)).sw(c, c.g[V0][0]);
        c.g[V1][0] &= 0x87;
        Addr(at(c.g[S0][0], 0xac)).sw(c, 0);
        Addr(at(c.g[S0][0], 0x74)).sb(c, c.g[V1][0]);
        c.g[A0][0] = sx(0x004c_0000);
        Addr(at(c.g[S0][0], 0xb0)).sw(c, 0);
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c.g[S0][0], 0xb4)).sw(c, 0);
        Addr(at(c.g[S0][0], 0xb8)).sh(c, 0);
        Addr(at(c.g[S0][0], 0xba)).sh(c, 0);
        c.g[A3][0] = sx(0x004c_66c8);
        c.ldl(3, at(c.g[A3][0], 7));
        c.ldr(3, c.g[A3][0] as u32);
        c.ldl(5, at(c.g[A3][0], 0xf));
        c.ldr(5, at(c.g[A3][0], 8));
        c.sdl(3, at(c.g[S0][0], 0x8f));
        c.sdr(3, at(c.g[S0][0], 0x88));
        c.sdl(5, at(c.g[S0][0], 0x97));
        c.sdr(5, at(c.g[S0][0], 0x90));
        c.g[A3][0] = sx(0x004c_66c8);
        c.ldl(3, at(c.g[A3][0], 7));
        c.ldr(3, c.g[A3][0] as u32);
        c.ldl(5, at(c.g[A3][0], 0xf));
        c.ldr(5, at(c.g[A3][0], 8));
        c.sdl(3, at(c.g[S0][0], 0x9f));
        c.sdr(3, at(c.g[S0][0], 0x98));
        c.sdl(5, at(c.g[S0][0], 0xa7));
        c.sdr(5, at(c.g[S0][0], 0xa0));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateTransition_v16(self, a1, a2)` (0x194600): if the transition
/// id matches the state's, clear or set the pending bits on the two objects.
pub fn cfe_state_transition_v16(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        'end: {
            'mid: {
                'inner: {
                    c.cycles += 18;
                    frame.save(c, S1, 0x30);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S2, 0x20);
                    frame.save(c, S3, 0x10);
                    c.g[A0][0] = sx(0x004a_0000);
                    c.g[A0][0] = sx(0x004a_1360);
                    frame.save(c, S0, 0x40);
                    c.g[S2][0] = c.g[A1][0];
                    frame.save_ra(c, 0);
                    c.g[RA][0] = sx(0x0019_4630);
                    c.g[S3][0] = c.g[A2][0];
                    call_ptr(c, 0x0031_7670, 0x0019_4630)?;
                    c.g[A0][0] = sx(0x004a_0000);
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0019_4640);
                    c.g[A0][0] = sx(0x004a_17b8);
                    call_ptr(c, 0x0031_7670, 0x0019_4640)?;
                    let t = c.g[S3][0] != c.g[S0][0];
                    c.g[A0][0] = sx(0xffff_ffbf);
                    if t {
                        break 'inner;
                    }
                    c.cycles += 8;
                    c.g[V1][0] = lw(c, at(c.g[S1][0], 0x1c));
                    c.g[V0][0] = sx(0xffff_fff7);
                    c.g[V1][0] &= c.g[V0][0];
                    Addr(at(c.g[S1][0], 0x1c)).sw(c, c.g[V1][0]);
                    c.g[V0][0] = lw(c, at(c.g[S2][0], 0x54));
                    c.g[V0][0] &= c.g[A0][0];
                    Addr(at(c.g[S2][0], 0x54)).sw(c, c.g[V0][0]);
                    break 'mid;
                }
                c.cycles += 2;
                let t = c.g[S3][0] != c.g[V0][0];
                frame.restore(c, S0, 0x40);
                if t {
                    break 'end;
                }
                c.cycles += 6;
                c.g[V0][0] = lw(c, at(c.g[S1][0], 0x1c));
                c.g[V0][0] |= 8;
                Addr(at(c.g[S1][0], 0x1c)).sw(c, c.g[V0][0]);
                c.g[V1][0] = lw(c, at(c.g[S2][0], 0x54));
                c.g[V1][0] |= 0x40;
                Addr(at(c.g[S2][0], 0x54)).sw(c, c.g[V1][0]);
                break 'mid;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
            break 'end;
        }
        c.cycles += 7;
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int OVScreenFinalStandings_Init(self)` (0x1e6668): create the two screens
/// of the final-standings state, start their threads and mark the state in.
pub fn ov_screen_final_standings_init(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        {
            c.cycles += 16;
            frame.save(c, S1, 0x10);
            c.g[S1][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            frame.save(c, S0, 0x20);
            c.g[A0][0] = sx(0x0046_e520);
            c.g[RA][0] = sx(0x001e_668c);
            c.g[S0][0] = lw(c, at(c.g[S1][0], 0x10));
            call_ptr(c, 0x0031_7670, 0x001e_668c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001e_66a0);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001e_66a0)?;
            let t = c.g[V0][0] == 0;
            Addr(at(c.g[S1][0], 0x40)).sw(c, c.g[V0][0]);
            if !t {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                c.g[RA][0] = sx(0x001e_66b8);
                c.g[A2][0] = 0;
                call_ptr(c, 0x0039_c870, 0x001e_66b8)?;
            }
        }
        {
            c.cycles += 11;
            c.g[A0][0] = sx(0x0046_0000);
            c.g[S0][0] = lw(c, at(c.g[S1][0], 0x10));
            c.g[RA][0] = sx(0x001e_66c8);
            c.g[A0][0] = sx(0x0046_e378);
            call_ptr(c, 0x0031_7670, 0x001e_66c8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001e_66dc);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001e_66dc)?;
            let t = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !t {
                c.cycles += 3;
                c.g[A1][0] = 0;
                c.g[RA][0] = sx(0x001e_66f0);
                c.g[A2][0] = 0;
                call_ptr(c, 0x0039_c870, 0x001e_66f0)?;
            }
        }
        c.cycles += 9;
        c.g[RA][0] = sx(0x001e_66f8);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x001e_66f8)?;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x20);
        Addr(at(c.g[S1][0], 0x9c)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby manager: list-changed handlers.

/// `void LobbyMan_OnRoomListChanged(self)` (0x2599b0): allocate a 0xe0-byte
/// link node with the room vtable and insert it at the head of the list.
pub fn lobby_man_on_room_list_changed(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 29;
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S0, 0x40);
        c.g[A1][0] = sx(0x0048_0488);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 8;
        c.g[RA][0] = sx(0x0025_99dc);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_99dc)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A2][0] = lw(c, at(c.g[S0][0], 0xf0));
        c.g[V0][0] = sx(0x0048_0000);
        c.g[V1][0] = 0xe0;
        c.g[V0][0] = sx(0x0048_12b0);
        Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
        Addr(at(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000);
        Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(0x0025_9a14);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_9a14)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(at(c.g[A1][0], 8));
        if c.g[V0][0] != 0 {
            Addr(at(c.g[A1][0], 8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw(c, frame.at(c, 0x10));
        frame.restore(c, S0, 0x40);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = lw(c, c.g[V0][0] as u32);
        frame.restore_ra(c, 0x20);
        Addr(at(c.g[A1][0], 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = lw(c, at(c.g[V0][0], 4));
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = lw(c, frame.at(c, 0x10));
        Addr(at(c.g[V0][0], 4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnRankListChanged(self)` (0x259a60): the same with the rank
/// vtable (id 0xe1).
pub fn lobby_man_on_rank_list_changed(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 29;
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S0, 0x40);
        c.g[A1][0] = sx(0x0048_0488);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 8;
        c.g[RA][0] = sx(0x0025_9a8c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_9a8c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A2][0] = lw(c, at(c.g[S0][0], 0xf0));
        c.g[V0][0] = sx(0x0048_0000);
        c.g[V1][0] = 0xe1;
        c.g[V0][0] = sx(0x0048_12b0);
        Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
        Addr(at(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000);
        Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(0x0025_9ac4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_9ac4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(at(c.g[A1][0], 8));
        if c.g[V0][0] != 0 {
            Addr(at(c.g[A1][0], 8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw(c, frame.at(c, 0x10));
        frame.restore(c, S0, 0x40);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = lw(c, c.g[V0][0] as u32);
        frame.restore_ra(c, 0x20);
        Addr(at(c.g[A1][0], 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = lw(c, at(c.g[V0][0], 4));
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = lw(c, frame.at(c, 0x10));
        Addr(at(c.g[V0][0], 4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnUserListChanged(self)` (0x25d6c0): the same with the user
/// vtable (id 0xdf).
pub fn lobby_man_on_user_list_changed(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 29;
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S0, 0x40);
        c.g[A1][0] = sx(0x0048_0488);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[A0][0] = 8;
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(0x0025_d6ec);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_d6ec)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = lw(c, at(c.g[S0][0], 0xf0));
        c.g[V0][0] = sx(0x0048_0000);
        c.g[V1][0] = 0xdf;
        c.g[V0][0] = sx(0x0048_12b0);
        Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
        Addr(at(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000);
        Addr(frame.at(c, 0x10)).sw(c, c.g[A0][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(0x0025_d724);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_d724)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(at(c.g[A1][0], 8));
        if c.g[V0][0] != 0 {
            Addr(at(c.g[A1][0], 8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw(c, frame.at(c, 0x10));
        frame.restore(c, S0, 0x40);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = lw(c, c.g[V0][0] as u32);
        frame.restore_ra(c, 0x20);
        Addr(at(c.g[A1][0], 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = lw(c, at(c.g[V0][0], 4));
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = lw(c, frame.at(c, 0x10));
        Addr(at(c.g[V0][0], 4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_GetRoomUserStats(self, index, out)` (0x25f288): look the
/// user up in the room list and fill in its id, level and total score.
pub fn lobby_man_get_room_user_stats(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'end: {
            c.cycles += 7;
            frame.save(c, S0, 0x60);
            frame.save(c, S1, 0x50);
            frame.save_ra(c, 0x40);
            c.g[V0][0] = lw(c, c.g[A0][0] as u32);
            let t = c.g[V0][0] == 0;
            c.g[S0][0] = c.g[A2][0];
            if t {
                break 'end;
            }
            'lookup: {
                'found: {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0025_f2ac);
                    c.g[A0][0] = lw(c, at(c.g[A0][0], 0x60));
                    call_ptr(c, 0x003e_e560, 0x0025_f2ac)?;
                    c.g[A0][0] = c.g[V0][0];
                    let t = c.g[A0][0] == 0;
                    c.g[V1][0] = sx(at(c.g[A0][0], 0x38));
                    if t {
                        break 'found;
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0;
                    c.g[S1][0] = c.g[V1][0];
                    if t {
                        break 'found;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = lw(c, at(c.g[A0][0], 0x34));
                    c.g[A0][0] = sx(at(c.g[A0][0], 0x28));
                    c.g[RA][0] = sx(0x0025_f2d0);
                    Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                    callee::ATOI.call(c, 0x0025_f2d0)?;
                    Addr(at(c.g[S0][0], 0x10)).sw(c, c.g[V0][0]);
                    break 'lookup;
                }
                c.cycles += 3;
                Addr(c.g[S0][0] as u32).sw(c, 0);
                c.g[S1][0] = 0;
                Addr(at(c.g[S0][0], 0x10)).sw(c, 0);
                break 'lookup;
            }
            c.cycles += 15;
            c.g[A3][0] = sx(0x004a_0000);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx(frame.at(c, 0));
            c.g[A3][0] = sx(0x004a_2f30);
            c.g[RA][0] = sx(0x0025_f2fc);
            c.g[A2][0] = 0x40;
            call_ptr(c, 0x003e_d678, 0x0025_f2fc)?;
            c.g[A1][0] = lw(c, frame.at(c, 8));
            c.g[V0][0] = lw(c, frame.at(c, 0xc));
            c.g[V1][0] = lw(c, frame.at(c, 0x10));
            c.g[A0][0] = lw(c, frame.at(c, 0x18));
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            Addr(at(c.g[S0][0], 4)).sw(c, c.g[A1][0]);
            Addr(at(c.g[S0][0], 0xc)).sw(c, c.g[A0][0]);
            Addr(at(c.g[S0][0], 8)).sw(c, c.g[V0][0]);
            break 'end;
        }
        c.cycles += 5;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Buddy manager: roster replies.

/// `void BuddyMan_OnRosterAddReply(self)` (0x2618c0): allocate the 0x10a link
/// node for the buddy's roster and insert it.
pub fn buddy_man_on_roster_add_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 29;
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S0, 0x40);
        c.g[A1][0] = sx(0x0048_0488);
        frame.save(c, S1, 0x30);
        c.g[A0][0] = 8;
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = lw(c, gp(c, -0xc8));
        c.g[RA][0] = sx(0x0026_18ec);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_18ec)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0x10a;
        c.g[V0][0] = sx(0x0048_0000);
        Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(0x0048_12b0);
        c.g[A1][0] = sx(0x0048_0000);
        Addr(at(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = lw(c, at(c.g[S0][0], 0xf0));
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0026_1924);
        Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_1924)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(at(c.g[A1][0], 8));
        if c.g[V0][0] != 0 {
            Addr(at(c.g[A1][0], 8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw(c, frame.at(c, 0x10));
        frame.restore(c, S0, 0x40);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = lw(c, c.g[V0][0] as u32);
        frame.restore_ra(c, 0x20);
        Addr(at(c.g[A1][0], 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = lw(c, at(c.g[V0][0], 4));
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = lw(c, frame.at(c, 0x10));
        Addr(at(c.g[V0][0], 4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_OnRosterRemoveReply(self)` (0x261970): the same with the
/// remove id 0x10b.
pub fn buddy_man_on_roster_remove_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 29;
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S0, 0x40);
        c.g[A1][0] = sx(0x0048_0488);
        frame.save(c, S1, 0x30);
        c.g[A0][0] = 8;
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = lw(c, gp(c, -0xc8));
        c.g[RA][0] = sx(0x0026_199c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_199c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0x10b;
        c.g[V0][0] = sx(0x0048_0000);
        Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(0x0048_12b0);
        c.g[A1][0] = sx(0x0048_0000);
        Addr(at(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = lw(c, at(c.g[S0][0], 0xf0));
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0026_19d4);
        Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_19d4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(at(c.g[A1][0], 8));
        if c.g[V0][0] != 0 {
            Addr(at(c.g[A1][0], 8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw(c, frame.at(c, 0x10));
        frame.restore(c, S0, 0x40);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = lw(c, c.g[V0][0] as u32);
        frame.restore_ra(c, 0x20);
        Addr(at(c.g[A1][0], 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = lw(c, at(c.g[V0][0], 4));
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = lw(c, frame.at(c, 0x10));
        Addr(at(c.g[V0][0], 4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// Script engine: binding filter.

/// `void ScriptEngine_SetObjBindingFilter(engine, index, filter)` (0x27d0c0):
/// apply the filter to every binding of the indexed script object.
pub fn script_engine_set_obj_binding_filter(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        'end: {
            c.cycles += 21;
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
            frame.save(c, S3, 0x20);
            frame.save(c, S0, 0x50);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S2, 0x30);
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A1][0] as u32));
            frame.save(c, S4, 0x10);
            c.g[S0][0] = 0;
            frame.save_ra(c, 0);
            c.g[S4][0] = c.g[A2][0];
            frame.save(c, S1, 0x40);
            c.g[S1][0] = lw(c, c.g[A1][0] as u32);
            c.g[V1][0] = lw(c, at(c.g[S1][0], 0xc));
            c.g[A0][0] = lh(c, at(c.g[V1][0], 0x30));
            c.g[V0][0] = lw(c, at(c.g[V1][0], 0x34));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0027_d108);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_d108)?;
            c.g[S2][0] = c.g[V0][0];
            if (c.g[S2][0] as i64) <= 0 {
                frame.restore(c, S0, 0x50);
                break 'end;
            }
            c.cycles += 1;
            c.g[V1][0] = lw(c, at(c.g[S1][0], 0xc));
            loop {
                c.cycles += 13;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx(at(c.g[S0][0], 1));
                c.g[A0][0] = lh(c, at(c.g[V1][0], 0x38));
                c.g[V0][0] = lw(c, at(c.g[V1][0], 0x3c));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0027_d130);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_d130)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x0027_d140);
                c.g[A2][0] = c.g[S4][0];
                callee::SCRIPT_ENGINE_SET_BINDING_FILTER.call(c, 0x0027_d140)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x50);
                    break 'end;
                }
                c.g[V1][0] = lw(c, at(c.g[S1][0], 0xc));
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0027_d118);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Speech instance creation.

/// `SpeechInstance* SPCH_CreateInstance(owner, id, a2, a3, t0, t1, t2)`
/// (0x2b0e60): allocate a 0x904-byte speech instance, run its constructor and
/// store it in the owner's instance table.
pub fn spch_create_instance(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x80);
        c.cycles += 43;
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x50);
        c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
        frame.save(c, S3, 0x40);
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S4, 0x30);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S5, 0x20);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S6, 0x10);
        c.g[S6][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T1][0];
        c.g[S4][0] = c.g[T2][0];
        c.g[V0][0] = lw(c, at(c.g[A0][0], 4));
        c.g[A1][0] = sx(0x0048_33b8);
        c.g[A0][0] = 0x904;
        c.g[A2][0] = 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[RA][0] = sx(0x002b_0ec0);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x002b_0ec0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S6][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[T1][0] = c.g[S3][0];
        c.g[T2][0] = c.g[S4][0];
        c.g[RA][0] = sx(0x002b_0ee0);
        c.g[A0][0] = c.g[V0][0];
        callee::SPEECH_INSTANCE_CTOR.call(c, 0x002b_0ee0)?;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Menu system: vector-label item measure.

/// `void MenuSysVectorLabelItem_Measure(item, ctx)` (0x2d0450): format the
/// label's position and measure the row.
pub fn menu_sys_vector_label_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0xe0);
        c.cycles += 43;
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        c.g[S0][0] = sx(0x0048_0000);
        frame.save(c, S2, 0xb0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0xa0);
        c.g[S0][0] = sx(0x0048_6720);
        frame.save(c, S4, 0x90);
        frame.save_ra(c, 0x80);
        c.g[S4][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, at(c.g[S1][0], 0x14));
        c.g[RA][0] = sx(0x002d_0488);
        c.lwc1(12, c.g[V0][0] as u32);
        call_ptr(c, 0x0041_3af8, 0x002d_0488)?;
        c.g[V1][0] = lw(c, at(c.g[S1][0], 0x14));
        c.g[S3][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002d_0498);
        c.lwc1(12, at(c.g[V1][0], 4));
        call_ptr(c, 0x0041_3af8, 0x002d_0498)?;
        c.g[V1][0] = lw(c, at(c.g[S1][0], 0x14));
        c.g[S2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002d_04a8);
        c.lwc1(12, at(c.g[V1][0], 8));
        call_ptr(c, 0x0041_3af8, 0x002d_04a8)?;
        c.g[A0][0] = sx(frame.at(c, 0));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x002d_04c0);
        c.g[T0][0] = c.g[V0][0];
        call_ptr(c, 0x0041_7828, 0x002d_04c0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = lw(c, at(c.g[A1][0], 0x18));
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x002d_04d8);
        c.g[T0][0] = sx(frame.at(c, 0));
        callee::MENU_SYS_ITEM_MEASURE_ROW.call(c, 0x002d_04d8)?;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider finish line / wipeout.

/// `void RiderFinishLineSub_Enter(self)` (0x12c5c8): reset the rider metrics,
/// compute the finish direction and mark whether the finish line has been
/// crossed.
pub fn rider_finish_line_sub_enter(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'end: {
            c.cycles += 24;
            c.g[A1][0] = 1;
            frame.save(c, S0, 0x10);
            frame.save_ra(c, 0);
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = lw(c, at(c.g[S0][0], 4));
            c.g[RA][0] = sx(0x0012_c5e8);
            c.g[A0][0] = lw(c, at(c.g[V0][0], 0x790));
            callee::RIDER_METRIX_ON_RIDER_RESET.call(c, 0x0012_c5e8)?;
            c.g[V1][0] = lw(c, at(c.g[S0][0], 4));
            c.lwc1(1, gp(c, 0x1b9c));
            c.lqc2(3, at(c.g[V1][0], 0x1e0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 0);
            c.c_lt_s(1, 0);
            if c.fcc() {
                Addr(c.g[S0][0] as u32).sw(c, 0);
                break 'end;
            }
            c.cycles += 6;
            c.lwc1(1, at(c.g[V1][0], 0x2dc));
            c.mtc1(0, 0);
            c.c_eq_s(1, 0);
            if !c.fcc() {
                Addr(c.g[S0][0] as u32).sw(c, 0);
                break 'end;
            }
            'check: {
                c.cycles += 6;
                c.lwc1(1, at(c.g[V1][0], 0x378));
                c.lwc1(0, gp(c, -0x77fc));
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = 1;
                if t {
                    break 'check;
                }
                c.cycles += 2;
                Addr(c.g[S0][0] as u32).sw(c, 0);
                break 'end;
            }
            c.cycles += 1;
            Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            break 'end;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderWipeoutSub_UpdatePhase4(self)` (0x12d9d8): advance the wipeout
/// recovery phase; once the animation is over (or the timer expired) apply the
/// recovery boost and re-check the reset path.
pub fn rider_wipeout_sub_update_phase4(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'first: {
            'restart: {
                c.cycles += 12;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 1);
                frame.save(c, S0, 0x20);
                frame.save(c, S1, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.lwc1(0, at(c.g[S0][0], 0x70));
                c.c_le_s(1, 0);
                let t = c.fcc();
                c.g[S1][0] = 0;
                if t {
                    break 'restart;
                }
                {
                    c.cycles += 7;
                    c.g[V0][0] = lw(c, at(c.g[S0][0], 0x80));
                    c.mtc1(0, 1);
                    c.lwc1(0, at(c.g[V0][0], 0x470));
                    c.c_le_s(1, 0);
                    let t = c.fcc();
                    c.g[V1][0] = 1;
                    if !t {
                        c.cycles += 1;
                        c.g[V1][0] = 0;
                    }
                }
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    c.g[A0][0] = lw(c, at(c.g[V0][0], 0x784));
                    break 'first;
                }
                break 'restart;
            }
            c.cycles += 3;
            c.g[V0][0] = lw(c, at(c.g[S0][0], 0x80));
            c.g[S1][0] = 1;
            c.g[A0][0] = lw(c, at(c.g[V0][0], 0x784));
            break 'first;
        }
        'second: {
            'play: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0012_da44);
                c.g[A1][0] = 2;
                callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED.call(c, 0x0012_da44)?;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = lw(c, at(c.g[S0][0], 0x80));
                    break 'play;
                }
                c.cycles += 2;
                if c.g[S1][0] == 0 {
                    frame.restore(c, S0, 0x20);
                    break 'second;
                }
                c.cycles += 1;
                c.g[A0][0] = lw(c, at(c.g[S0][0], 0x80));
                break 'play;
            }
            c.cycles += 7;
            c.g[RA][0] = sx(0x0012_da60);
            c.g[A1][0] = c.g[S1][0];
            callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST.call(c, 0x0012_da60)?;
            c.g[A0][0] = lw(c, at(c.g[S0][0], 0x80));
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x0012_da70);
            c.g[A2][0] = 2;
            callee::RIDER_CHECK_RESET_PATH.call(c, 0x0012_da70)?;
            frame.restore(c, S0, 0x20);
            break 'second;
        }
        c.cycles += 4;
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
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 40;
    /// Fewer seeds for tests whose real callees are heavy.
    const FEW: u32 = 16;
    /// The zeroed BELibrary / interface dummy.
    const BE: u32 = DATA + 0x2000;
    const IFACE: u32 = DATA + 0x4000;
    /// The world and its game / race manager chain.
    const W: u32 = DATA + 0x3000;
    /// General scratch (objects, vtables) below the pool area.
    const SCRATCH: u32 = DATA + 0x5000;
    /// The dlmalloc arena the game's `operator new`/`EA_MemAlloc` uses.
    const MEM_HEAP: u32 = 0x004f_f250;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    fn wh(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }
    fn wb(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }
    fn wf(c: &mut Cpu, a: u32, v: f32) {
        c.bus.write32(a, v.to_bits());
    }

    /// A vtable slot `{ short delta; void (*fn)(); }` pointing at fake `i`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, i: u32) {
        wh(c, vt + off, 0);
        w(c, vt + off + 4, fakes::FAKE + 0x10 * i);
    }

    /// Run a generated function for setup (entering at its own address, so a
    /// dispatcher-shaped one starts at its entry), returning `v0`.
    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.pc = addr;
        f(c);
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

    /// A real heap block of `size` bytes.
    fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0x2000_0000, 0],
        ) as u32
    }

    /// The engine's size-class pool allocator, enabled through `gp + 0xdd8`.
    fn pool_allocator(c: &mut Cpu) {
        w(c, GAME_GP + 0xdd8, 1);
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
                w(c, node, if n == 15 { 0 } else { node + block });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// Scrambled registers, the fake hooks, a game-like `gp`, the BE/interface
    /// singleton, a world with no riders, a working heap and pool allocator.
    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        pool_allocator(c);
        heap_init(c);
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), W);
        w(c, GAME_GP.wrapping_sub(0xc8), IFACE);
        w(c, GAME_GP.wrapping_sub(0x854), IFACE);
        // world -> +0x84 game -> +0xc race -> +0x7c riders (none)
        w(c, W + 0x84, W + 0x1000);
        w(c, W + 0x1000 + 0xc, W + 0x2000);
        w(c, W + 0x2000 + 0x7c, 0);
        // world +0x7c: the rider slot table (empty) the bolt file scans.
        w(c, W + 0x7c, DATA + 0x15000);
        // The interface's character-name strings and list head.
        w(c, IFACE + 0x48, SCRATCH + 0x1000);
        w(c, IFACE + 0x4c, SCRATCH + 0x1010);
        wb(c, SCRATCH + 0x1000, 0);
        wb(c, SCRATCH + 0x1010, 0);
        w(c, IFACE + 0xf0, SCRATCH + 0x1100);
        w(c, SCRATCH + 0x1100, 0);
        w(c, SCRATCH + 0x1104, 0);
        r
    }

    // --------------------------------------------------------------- FE states

    #[test]
    fn cfe_state_mp_circuit_on_create_screen_matches() {
        check(
            ee::s03::f_001a3258__generated,
            cfe_state_mp_circuit_on_create_screen,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1a32_58, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                // The engine the screen is created on, with an empty class
                // registry (so no class matches and no screen comes back).
                w(c, obj + 0x10, obj + 0x400);
                w(c, obj + 0x40, 0);
                if seed % 3 == 0 {
                    // A registry with one class entry.
                    w(c, obj + 0x404, obj + 0x500);
                    w(c, obj + 0x500 + 0xc, 0);
                }
            },
        );
    }

    #[test]
    fn cfe_state_rider_bio_fill_bio_info_matches() {
        check(
            ee::s02::f_00191228__generated,
            cfe_state_rider_bio_fill_bio_info,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1912_28, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                // The screen and (for the found branch) the text object.
                let screen = obj + 0x800;
                w(c, obj + 0x40, if seed % 3 == 0 { 0 } else { screen });
                w(c, obj + 0x48, seed % 8);
                // UIScreen_FindObject: count at +0x3c, array at +0x38,
                // entries whose +0x38 is the hash.
                w(c, screen + 0x3c, if seed % 2 == 0 { 0 } else { 1 });
                w(c, screen + 0x38, screen + 0x900);
                let entry = screen + 0xa00;
                w(c, screen + 0x900, entry);
                // The hash of the (zeroed) bio-title string.
                let hash = run(c, 0x0031_7670, ee::s10::f_00317670, &[0x0045_e438u64]);
                w(c, entry + 0x38, hash as u32);
                // The text-scroll object: no lines, no popup chain.
                w(c, entry + 0xbc, 0);
                wh(c, entry + 0xc8, 0);
            },
        );
    }

    #[test]
    fn net_fe_state_show_user_found_matches() {
        check(
            ee::s03::f_001a4ce8__generated,
            net_fe_state_show_user_found,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1a4c_e8, 8);
                let obj = SCRATCH;
                let src = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = src as u64;
                // The popup pre-exists so GetPopup does not allocate.
                let popup = obj + 0x2000;
                w(c, obj + 0x6a0, popup);
                for i in 0..0xa0u32 {
                    w(c, popup + 0xbc + 4 * i, 0);
                }
                // The source record: a name pointer at +8 and an id at +0x10.
                w(c, src + 8, SCRATCH + 0x1000);
                w(c, src + 0x10, seed.wrapping_mul(7));
                w(c, obj + 0x6d0, 0);
            },
        );
    }

    #[test]
    fn net_fe_state_on_disconnected_popup_event_matches() {
        check(
            ee::s03::f_001a9668__generated,
            net_fe_state_on_disconnected_popup_event,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1a96_68, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A2][0] = (seed % 4) as u64;
                // The state's UI manager.
                let ui = obj + 0x400;
                w(c, obj + 0x10, ui);
                let state = ui + 0x18;
                w(c, ui + 0xd0, obj + 0x500);
                w(c, obj + 0x500 + 0x10, state);
                w(c, state + 0x18, obj + 0x600);
                slot(c, obj + 0x600, 0, 0);
                w(c, obj + 0x600 + 0x1c, 0);
            },
        );
    }

    #[test]
    fn net_fe_state_on_net_interrupted_popup_event_matches() {
        check(
            ee::s03::f_001a9710__generated,
            net_fe_state_on_net_interrupted_popup_event,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1a97_10, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A2][0] = (seed % 4) as u64;
                let ui = obj + 0x400;
                w(c, obj + 0x10, ui);
                let state = ui + 0x18;
                w(c, ui + 0xd0, obj + 0x500);
                w(c, obj + 0x500 + 0x10, state);
                w(c, state + 0x18, obj + 0x600);
                slot(c, obj + 0x600, 0, 1);
                w(c, obj + 0x600 + 0x1c, 0);
            },
        );
    }

    #[test]
    fn cfe_use_existing_account_check_password_matches() {
        check(
            ee::s03::f_001ac2b8__generated,
            cfe_use_existing_account_check_password,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1ac2_b8, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                // Empty password for even seeds.
                w(
                    c,
                    IFACE + 0x4c,
                    if seed % 2 == 0 {
                        SCRATCH + 0x1010
                    } else {
                        SCRATCH + 0x1020
                    },
                );
                wb(c, SCRATCH + 0x1020, b'x');
                wb(c, SCRATCH + 0x1021, 0);
                let popup = obj + 0x2000;
                w(c, obj + 0x6a0, popup);
                for i in 0..0xa0u32 {
                    w(c, popup + 0xbc + 4 * i, 0);
                }
            },
        );
    }

    #[test]
    fn cfe_use_existing_account_check_account_name_matches() {
        check(
            ee::s03::f_001ac360__generated,
            cfe_use_existing_account_check_account_name,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1ac3_60, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                w(
                    c,
                    IFACE + 0x48,
                    if seed % 2 == 0 {
                        SCRATCH + 0x1000
                    } else {
                        SCRATCH + 0x1030
                    },
                );
                wb(c, SCRATCH + 0x1030, b'y');
                wb(c, SCRATCH + 0x1031, 0);
                let popup = obj + 0x2000;
                w(c, obj + 0x6a0, popup);
                for i in 0..0xa0u32 {
                    w(c, popup + 0xbc + 4 * i, 0);
                }
            },
        );
    }

    #[test]
    fn cfe_state_chat_room_handle_input_matches() {
        check(
            ee::s03::f_001b38d0__generated,
            cfe_state_chat_room_handle_input,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1b38_d0, 8);
                let obj = SCRATCH;
                let ev = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = ev as u64;
                // The event target's vtable (`+8` object, slot 0xd8).
                let target = obj + 0x1100;
                w(c, ev + 8, target);
                slot(c, target, 0xd8, 0);
                // The keyboard object and its two helper calls.
                let kb = obj + 0x2000;
                w(c, obj + 0x69c, kb);
                w(c, kb + 0x38, 0);
                w(c, kb + 0x3c, 0);
                slot(c, kb, 0x38, 1);
                slot(c, kb, 0x40, 2);
                w(c, kb + 0x438, 0);
                w(c, kb + 0x43c, 1);
            },
        );
    }

    // ----------------------------------------------------------------- lobby

    #[test]
    fn lobby_man_create_peer_table_matches() {
        check(
            ee::s06::f_0025a650__generated,
            lobby_man_create_peer_table,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x25a6_50, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0xc, if seed % 4 == 0 { 0 } else { 1 });
                w(c, obj + 0xf4, if seed % 3 == 0 { obj + 0x1000 } else { 0 });
                w(c, obj + 0x54, obj + 0x2000);
                // LobbyApi_GetRecord reads the "lobby" object's fields.
                w(c, obj + 0x2000, 0);
                w(c, obj + 0x2004, 0);
            },
        );
    }

    // ---------------------------------------------------------- script engine

    #[test]
    fn script_engine_spawn_chosen_script_matches() {
        check(
            ee::s07::f_00279528__generated,
            script_engine_spawn_chosen_script,
            FEW,
            &|c, seed| {
                let _ = base(c, seed, 0x2795_28, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A2][0] = seed as u64;
                c.g[A3][0] = (seed ^ 0x55) as u64;
                c.g[T0][0] = (seed.wrapping_mul(3)) as u64;
                // A script node chosen for some seeds.
                w(c, obj + 0x100, if seed % 3 == 0 { 0 } else { obj + 0x1000 });
                w(c, obj + 0x1000, 0x100);
            },
        );
    }

        // ----------------------------------------------------------------- menus

    #[test]
    fn menu_sys_push_page_matches() {
        check(
            ee::s09::f_002cc3b8__generated,
            menu_sys_push_page,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x2cc3_b8, 6);
                let obj = SCRATCH;
                let item = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = item as u64;
                // The item list: 0, 1 or 2 pages, each with a vtable at +0x12c.
                w(c, obj, (seed % 3) as u32);
                for i in 0..3u32 {
                    let e = obj + 0x100 + 4 * i;
                    w(c, e, obj + 0x200 + 0x40 * i);
                    let v = obj + 0x200 + 0x40 * i;
                    w(c, v + 0x100, 0);
                    w(c, v + 0x12c, obj + 0x800 + 0x40 * i);
                    slot(c, obj + 0x800 + 0x40 * i, 0x30, i);
                    slot(c, obj + 0x800 + 0x40 * i, 0x38, i + 1);
                }
                w(c, obj + 4, item);
                w(c, item + 0x12c, obj + 0x900);
                slot(c, obj + 0x900, 0x30, 4);
                slot(c, obj + 0x900, 0x38, 5);
            },
        );
    }

    #[test]
    fn menu_sys_vector_label_item_draw_matches() {
        check(
            ee::s09::f_002d0500__generated,
            menu_sys_vector_label_item_draw,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x2d05_00, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (obj + 0x1000) as u64;
                // Three positions converted to doubles, then a formatted row.
                let pos = obj + 0x2000;
                wf(c, pos, (seed as f32) * 0.5);
                wf(c, pos + 4, 1.0);
                wf(c, pos + 8, 2.0);
                w(c, obj + 0x14, pos);
                w(c, obj + 0x18, 0x40);
                w(c, obj + 0x1000 + 0x14, pos);
                w(c, obj + 0x1000 + 0x18, 0);
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
                let _ = base(c, seed, 0x2729_a8, 4);
                let mgr = SCRATCH;
                let handle = 0x58 * (seed % 4) as u32;
                c.g[A0][0] = mgr as u64;
                c.g[A1][0] = handle as u64;
                w(c, mgr + 0x14, mgr + 0x2000);
                let seq = mgr + 0x2000 + handle;
                // Linked-list links: next (+0x50), prev (+0x54), instance
                // (+0x58) and the manager's active/idle head pointers.
                w(c, seq + 0x50, if seed % 3 == 0 { 0 } else { seq + 0x100 });
                w(c, seq + 0x54, if seed % 3 == 0 { 0 } else { seq + 0x200 });
                w(c, seq + 0x58, 0);
                w(c, seq + 0x100 + 0x50, seq);
                w(c, seq + 0x100 + 0x54, 0);
                w(c, seq + 0x200 + 0x50, 0);
                w(c, seq + 0x200 + 0x54, seq);
                w(c, mgr + 0x1c, seq);
                w(c, mgr + 0x18, seq + 0x100);
            },
        );
    }

    // ------------------------------------------------------- ASCII formatting

    fn format_fixture(c: &mut Cpu, seed: u32, buf: u32, fmt: u32) {
        // A short ASCII format, sometimes with one conversion.
        let s: &[u8] = match seed % 4 {
            0 => b"hello",
            1 => b"%d",
            2 => b"x=%d y=%d",
            _ => b"",
        };
        for (i, b) in s.iter().enumerate() {
            wb(c, fmt + i as u32, *b);
        }
        wb(c, fmt + s.len() as u32, 0);
        // The vararg save area the callee reads through a2.
        w(c, buf + 0x80, seed);
        w(c, buf + 0x84, 0xdead);
        w(c, buf + 0x88, 7);
    }

    #[test]
    fn ustr_sprintf_ascii_fmt_matches() {
        check(
            ee::s08::f_002c2718__generated,
            ustr_sprintf_ascii_fmt,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x2c27_18, 4);
                let buf = SCRATCH + 0x2000;
                let fmt = SCRATCH + 0x3000;
                for i in 0..0x1000u32 {
                    w(c, buf + 4 * i, 0);
                }
                format_fixture(c, seed, buf, fmt);
                c.g[A0][0] = buf as u64;
                c.g[A1][0] = fmt as u64;
                c.g[A2][0] = (seed as u64) << 40 | 3;
                c.g[A3][0] = 4;
                c.g[T0][0] = 5;
                c.g[T1][0] = 6;
                c.g[T2][0] = 7;
                c.g[T3][0] = 8;
            },
        );
    }

    #[test]
    fn ustr_sprintf_ascii_fmt_2_matches() {
        check(
            ee::s08::f_002c27c0__generated,
            ustr_sprintf_ascii_fmt_2,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x2c27_c0, 4);
                let buf = SCRATCH + 0x2000;
                let fmt = SCRATCH + 0x3000;
                for i in 0..0x1000u32 {
                    w(c, buf + 4 * i, 0);
                }
                format_fixture(c, seed, buf, fmt);
                c.g[A0][0] = buf as u64;
                c.g[A1][0] = fmt as u64;
                c.g[A2][0] = (seed as u64) << 40 | 3;
                c.g[A3][0] = 4;
                c.g[T0][0] = 5;
                c.g[T1][0] = 6;
                c.g[T2][0] = 7;
                c.g[T3][0] = 8;
            },
        );
    }

    /// A vtable slot pointing at a fixed translated function.
    fn slot_to(c: &mut Cpu, vt: u32, off: u32, target: u32) {
        wh(c, vt + off, 0);
        w(c, vt + off + 4, target);
    }

    const RET_ZERO: u32 = 0x001a_8828;
    const RET_ONE: u32 = 0x001c_5dd0;

    // -------------------------------------------- memory-card format states

    fn mc_manager_fixture(c: &mut Cpu, obj: u32, seed: u32) {
        w(c, obj + 0x108, (seed % 3) as u32);
        let a = obj + 0x1000;
        w(c, obj + 0x434, a);
        let vt = obj + 0x1100;
        w(c, a, vt);
        if seed % 2 == 0 {
            slot_to(c, vt, 0x5c, RET_ZERO);
            slot_to(c, vt, 0x4c, RET_ONE);
        } else {
            slot(c, vt, 0x5c, 0);
            slot(c, vt, 0x4c, 1);
        }
        slot_to(c, vt, 0x58, RET_ZERO);
        slot_to(c, vt, 0x48, RET_ZERO);
        let vt2 = obj + 0x1200;
        w(c, obj + 0x748, vt2);
        slot(c, vt2, 8, 2);
        slot(c, vt2, 0xc, 3);
    }

    #[test]
    fn mc_manager_state0b_format_matches() {
        check(
            ee::s06::f_0023f050__generated,
            mc_manager_state0b_format,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x23f0_50, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                mc_manager_fixture(c, obj, seed);
            },
        );
    }

    #[test]
    fn mc_manager_state28_format_matches() {
        check(
            ee::s06::f_00240b88__generated,
            mc_manager_state28_format,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x240b_88, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                mc_manager_fixture(c, obj, seed);
            },
        );
    }

    // --------------------------------------------------------- popup confirm

    #[test]
    fn cfe_popup_confirm_v06_matches() {
        check(
            ee::s03::f_001c5b88__generated,
            cfe_popup_confirm_v06,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1c5b_88, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                let popup = if seed % 3 == 0 { 0 } else { obj + 0x2000 };
                w(c, obj + 0x6c, popup);
                w(c, obj + 0x2c4, seed as u32);
                w(c, obj + 0x30c, seed.wrapping_mul(3));
                // The popup's two child objects.
                w(c, popup + 8, obj + 0x2100);
                w(c, obj + 0x2100 + 0x38, 0);
                w(c, obj + 0x2100 + 0x3c, 0);
                slot(c, obj + 0x2100, 0x38, 0);
                slot(c, obj + 0x2100, 0x3c, 1);
                slot(c, obj + 0x2100, 0x40, 2);
                slot(c, obj + 0x2100, 0x44, 3);
                // The item object at self+8.
                w(c, obj + 8, obj + 0x2200);
                slot(c, obj + 0x2200, 0xa0, 4);
                slot(c, obj + 0x2200, 0xa4, 5);
            },
        );
    }

    // ----------------------------------------------------------- debug menu

    #[test]
    fn debug_menu_find_next_item_matches() {
        check(
            ee::s02::f_0017a220__generated,
            debug_menu_find_next_item,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x17a2_20, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed ^ 0xa5) as u64;
                w(c, obj, (seed % 6) as u32);
            },
        );
    }

    // ---------------------------------------------------------- lobby login

    #[test]
    fn lobby_menu_login_matches() {
        check(ee::s02::f_0017e118__generated, lobby_menu_login, FEW, &|c, seed| {
            let _ = base(c, seed, 0x17e1_18, 8);
            let obj = SCRATCH;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (obj + 0x1000) as u64;
            c.g[A2][0] = (obj + 0x1100) as u64;
            w(c, obj + 8, if seed % 3 == 0 { 0 } else { obj + 0x2000 });
            w(c, obj + 0x2000, 0);
            w(c, obj + 0x1c04, 0);
            w(c, obj + 0x1c08, 0);
            wb(c, obj + 0x1000, b'a');
            wb(c, obj + 0x1100, b'b');
        });
    }

    // ---------------------------------------------------------------- audio

    #[test]
    fn ssx_audio_system_unload_front_end_audio_matches() {
        check(
            ee::s07::f_00286200__generated,
            ssx_audio_system_unload_front_end_audio,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x2862_00, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                w(
                    c,
                    obj + 0x608c,
                    match seed % 3 {
                        0 => 2,
                        1 => 1,
                        _ => 0,
                    },
                );
                w(c, obj + 0x6090, (seed % 2) as u32);
                let player = obj + 0x2000;
                w(c, obj + 0x118, player);
                w(c, player, obj + 0x2100);
                w(c, obj + 0x2100 + 0xab0, obj + 0x2200);
                slot(c, obj + 0x2200, 0x10, 0);
                slot(c, obj + 0x2200, 0x14, 1);
            },
        );
    }

        // ------------------------------------------------------- device select

    #[test]
    fn cfe_state_options_device_select_v38_matches() {
        check(
            ee::s02::f_0018e2c0__generated,
            cfe_state_options_device_select_v38,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x18e2_c0, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0x22c, if seed % 3 == 0 { 0 } else { obj + 0x2000 });
                let screen = obj + 0x3000;
                w(c, obj + 0x40, screen);
                w(c, screen + 0x3c, 1);
                w(c, screen + 0x38, screen + 0x400);
                let entry = screen + 0x500;
                w(c, screen + 0x400, entry);
                let hash = run(c, 0x0031_7670, ee::s10::f_00317670, &[0x004a_1458u64]);
                w(c, entry + 0x38, hash as u32);
                w(c, entry + 8, screen + 0x600);
                slot(c, screen + 0x600, 0x48, 0);
                slot(c, screen + 0x600, 0x4c, 1);
            },
        );
    }

    // ----------------------------------------------------------- video player

    #[test]
    fn vid_engine_player_update_matches() {
        check(
            ee::s06::f_00253938__generated,
            vid_engine_player_update,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x2539_38, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                let player = obj + 0x2000;
                w(c, obj + 0x38, player);
                w(c, obj + 0x3c, obj + 0x2100);
                w(c, obj + 0x40, (seed % 2) as u32);
                w(c, obj + 0xc, (seed % 3 == 0) as u32);
                w(c, obj, (seed % 4 == 0) as u32);
                // The player's audio track object and its vtables.
                w(c, obj + 0x2100 + 4, obj + 0x2200);
                slot(c, obj + 0x2200, 0x30, 0);
                w(c, player + 0x48, 0);
                w(c, player + 0x470, 0);
            },
        );
    }

    // ------------------------------------------------------------- UI dtor

    #[test]
    fn ui_state_dtor_matches() {
        check(ee::s13::f_0039e390__generated, ui_state_dtor, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0x39e3_90, 4);
            // A real heap object, so the `flags & 1` free is valid.
            let obj = alloc(c, 0x60);
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            // The two lists and the node destructor state.
            w(c, obj + 0x24, 0);
            w(c, obj + 0x28, 0);
            w(c, obj + 0x30, 0);
            w(c, obj + 0x34, 0);
        });
    }

    // ------------------------------------------------------- world painter

    #[test]
    fn world_painter_free_view_queries_matches() {
        check(
            ee::s09::f_002ecfa0__generated,
            world_painter_free_view_queries,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x2ecf_a0, 4);
                w(c, GAME_GP + 0xa64, if seed % 3 == 0 { 0 } else { 1 });
                // The view-query table: 8 painters x (1 + 8) entries.
                for i in 0..0x60u32 {
                    w(c, 0x009f_a370 + 4 * i, 0);
                }
                if seed % 2 == 1 {
                    let obj = SCRATCH + 0x1000;
                    w(c, 0x009f_a370, obj);
                    w(c, obj + 4, obj + 0x1100);
                    slot(c, obj + 0x1100, 8, 0);
                    slot(c, obj + 0x1100, 0xc, 1);
                }
            },
        );
    }

    // ----------------------------------------------------- trophy room images

    #[test]
    fn cfe_state_trophy_room_release_trophy_images_matches() {
        check(
            ee::s04::f_001d4b20__generated,
            cfe_state_trophy_room_release_trophy_images,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1d4b_20, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                // The five image handles: -1 or a live image id.
                for i in 0..6u32 {
                    w(
                        c,
                        obj + 0x74 + 4 * i,
                        if (seed as u32 + i) % 3 == 0 {
                            0xffff_ffff
                        } else {
                            i + 1
                        },
                    );
                }
                w(c, obj + 0x50, obj + 0x2000);
                // The world's rider array (empty) and the interface vtable the
                // first call goes through.
                w(c, W, 0);
                w(c, IFACE + 0x10d8, obj + 0x2100);
                slot(c, obj + 0x2100, 0x190, 0);
            },
        );
    }

    // ------------------------------------------------------ load screen unload

    #[test]
    fn c_game_load_screen_unload_matches() {
        check(
            ee::s06::f_002332e8__generated,
            c_game_load_screen_unload,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x2332_e8, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0xc, if seed % 2 == 0 { 0 } else { obj + 0x2000 });
                for i in 0..3u32 {
                    w(
                        c,
                        obj + 0x10 + 4 * i,
                        if (seed % 4) as u32 == i {
                            obj + 0x3000 + 4 * i
                        } else {
                            0
                        },
                    );
                }
                for i in 0..3u32 {
                    w(c, obj + 0x3000 + 4 * i + 4, obj + 0x2100);
                }
                w(c, IFACE + 0x10d8, obj + 0x2200);
                slot(c, obj + 0x2200, 0x98, 0);
                slot(c, obj + 0x2200, 0x190, 1);
            },
        );
    }

    // ----------------------------------------------------- list box wide str

    #[test]
    fn ui_list_box_set_item_wide_string_matches() {
        check(
            ee::s13::f_00399d80__generated,
            ui_list_box_set_item_wide_string,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x399d_80, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (obj + 0x1000) as u64;
                // The list-box chain that gates the update.
                let a = obj + 0x2000;
                let b = a + 0x100;
                let d = b + 0x100;
                w(c, obj + 0x5c, a);
                w(c, a + 0xd0, b);
                w(c, b + 0x10, d);
                w(c, d + 0x10, if seed % 3 == 0 { 0 } else { obj + 0x2300 });
                // The item's old string (a real block when present, so the
                // replacement's free of it is valid).
                let old = alloc(c, 8);
                w(
                    c,
                    obj + 0x9c + 0x14 * (seed % 4),
                    if seed % 2 == 0 { 0 } else { old },
                );
                // The wide source string.
                wh(c, obj + 0x1000, b'H' as u16);
                wh(c, obj + 0x1002, b'i' as u16);
                wh(c, obj + 0x1004, 0);
            },
        );
    }

    // ------------------------------------------------------ reward database

        // --------------------------------------------------------- game module

    #[test]
    fn game_module_try_exit_matches() {
        check(
            ee::s06::f_00230e98__generated,
            game_module_try_exit,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x230e_98, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                let module = obj + 0x1000;
                w(c, obj + 0x98, module);
                w(c, module, obj + 0x1100);
                if seed % 2 == 0 {
                    slot_to(c, obj + 0x1100, 0x1c, RET_ZERO);
                } else {
                    slot(c, obj + 0x1100, 0x1c, 0);
                }
                slot_to(c, obj + 0x1100, 0x18, RET_ZERO);
                // The world/race manager and the callback object.
                let game = W + 0x1000;
                w(c, game + 0x270, 0);
                w(c, IFACE + 0x10d8, obj + 0x2200);
                slot(c, obj + 0x2200, 0x78, 1);
                w(c, obj + 0x2200 + 0x7c, fakes::FAKE + 0x30);
                w(c, GAME_GP.wrapping_sub(0x84c), obj + 0x3000);
                w(c, obj + 0x94, obj + 0x3100);
            },
        );
    }

    // -------------------------------------------------------------- BE data

    #[test]
    fn be_backup_player_setup_matches() {
        check(
            ee::s01::f_0014ebf8__generated,
            be_backup_player_setup,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x14eb_f8, 4);
                c.g[A0][0] = 0x004a_6ca8u64;
                c.g[A1][0] = (seed % 4) as u64;
                // The character table at 0x535b20 (the source of the copy).
                for i in 0..0x100u32 {
                    w(c, 0x0053_5b20 + 4 * i, seed.wrapping_add(i));
                }
            },
        );
    }

    #[test]
    fn be_reset_char_rivalries_matches() {
        check(
            ee::s01::f_001519e0__generated,
            be_reset_char_rivalries,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1519_e0, 4);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 10) as u64;
                // The character record the loop compares against (`v0` at
                // `0x4406fc + i*0xc`) and the second pointer it reads
                // (`0x440700 + i*0xc`), which decides where the loop stops.
                for i in 0..10u32 {
                    w(c, 0x0044_06fc + 0xc * i, 0);
                    w(c, 0x0044_0700 + 0xc * i, (seed % 9) + 1);
                }
                for i in 0..0x40u32 {
                    w(c, obj + 0xbc2 + 4 * i, 0);
                }
                wb(c, 0x004a_1218, 0);
                wb(c, 0x004a_1219, 1);
                wb(c, 0x004a_121a, 2);
            },
        );
    }

    // ------------------------------------------------------- replay highlight

    #[test]
    fn replay_highlight_slot_score_matches() {
        check(
            ee::s07::f_0026e838__generated,
            replay_highlight_slot_score,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x26e8_38, 4);
                let obj = SCRATCH;
                let rep = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = rep as u64;
                c.g[A2][0] = (seed % 3) as u64;
                let d = (seed % 5) as i32 * 400;
                w(c, rep + 0x9c, (1000 + d) as u32);
                w(c, obj + 0xa0, 1000);
                wf(c, rep + 0x44, if seed % 2 == 0 { 4.0 } else { 6.0 });
                w(c, rep + 0xa4, seed.wrapping_mul(11));
                w(c, obj + 0xa8, (seed % 4) * 3);
                w(c, rep + 0xb0, rep + 0x200);
                w(c, obj, rep);
                w(c, obj + 0xb0, rep + 0x300);
            },
        );
    }

    // ---------------------------------------------------------- UIText ctor

    #[test]
    fn ui_text_ctor_matches() {
        check(ee::s13::f_003a0348__generated, ui_text_ctor, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0x3a03_48, 4);
            let obj = SCRATCH;
            c.g[A0][0] = obj as u64;
            wb(c, obj + 0x74, (seed % 0x100) as u8);
        });
    }

    // --------------------------------------------------- state transition

    #[test]
    fn cfe_state_transition_v16_matches() {
        check(
            ee::s02::f_00194600__generated,
            cfe_state_transition_v16,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1946_00, 4);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (obj + 0x1000) as u64;
                let hash = run(c, 0x0031_7670, ee::s10::f_00317670, &[0x004a_1360u64]);
                c.g[A2][0] = if seed % 2 == 0 { hash } else { hash ^ 0x55 };
                w(c, obj + 0x1c, (seed as u32) & 0xff);
                w(c, obj + 0x1000 + 0x54, (seed.wrapping_mul(3)) & 0xff);
            },
        );
    }

    // -------------------------------------------------- final standings init

    #[test]
    fn ov_screen_final_standings_init_matches() {
        check(
            ee::s04::f_001e6668__generated,
            ov_screen_final_standings_init,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x1e66_68, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x10, obj + 0x400);
                w(c, obj + 0x400 + 4, obj + 0x500);
                w(c, obj + 0x500 + 0xc, 0);
                w(c, obj + 0x9c, 0);
            },
        );
    }

    // ------------------------------------------------------ lobby list adds

                            // ------------------------------------------------------ binding filter

    #[test]
    fn script_engine_set_obj_binding_filter_matches() {
        check(
            ee::s07::f_0027d0c0__generated,
            script_engine_set_obj_binding_filter,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x27d0_c0, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = (seed ^ 0x3c) as u64;
                let so = obj + 0x1000;
                w(c, obj + 4 * (seed % 3), so);
                w(c, so + 0xc, obj + 0x1100);
                slot_to(
                    c,
                    obj + 0x1100,
                    0x30,
                    if seed % 2 == 0 { RET_ZERO } else { RET_ONE },
                );
                slot_to(c, obj + 0x1100, 0x38, RET_ONE);
                slot(c, obj + 0x1100, 0x3c, 0);
            },
        );
    }

    // ------------------------------------------------------- speech instance

    #[test]
    fn spch_create_instance_matches() {
        check(
            ee::s08::f_002b0e60__generated,
            spch_create_instance,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x2b0e_60, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (obj + 0x1000) as u64;
                c.g[A3][0] = 2;
                c.g[T0][0] = 3;
                c.g[T1][0] = 4;
                c.g[T2][0] = 5;
                w(c, obj + 4, obj + 0x2000);
                w(c, obj + 0x1000 + 0x18, 0);
            },
        );
    }

    // ---------------------------------------------------------- label measure

    #[test]
    fn menu_sys_vector_label_item_measure_matches() {
        check(
            ee::s09::f_002d0450__generated,
            menu_sys_vector_label_item_measure,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x2d04_50, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (obj + 0x1000) as u64;
                let pos = obj + 0x2000;
                wf(c, pos, (seed as f32) * 0.25);
                wf(c, pos + 4, 1.5);
                wf(c, pos + 8, 3.0);
                w(c, obj + 0x14, pos);
                w(c, obj + 0x18, 0x48);
                w(c, obj + 0x1000 + 0x14, pos);
                w(c, obj + 0x1000 + 0x18, 0);
            },
        );
    }

    // ------------------------------------------------------ rider finish line

    #[test]
    fn rider_finish_line_sub_enter_matches() {
        check(
            ee::s00::f_0012c5c8__generated,
            rider_finish_line_sub_enter,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x12c5_c8, 4);
                let obj = SCRATCH;
                let rider = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 4, rider);
                w(c, rider + 0x790, rider + 0x2000);
                // The finish-line normal and the two test values.
                for i in 0..4u32 {
                    wf(
                        c,
                        rider + 0x1e0 + 4 * i,
                        match i {
                            0 => 0.0,
                            1 => 0.0,
                            2 => 1.0,
                            _ => 0.0,
                        },
                    );
                }
                wf(c, rider + 0x2dc, if seed % 2 == 0 { 0.0 } else { 0.5 });
                wf(c, rider + 0x378, if seed % 3 == 0 { 0.0 } else { 1.0 });
                wf(
                    c,
                    GAME_GP.wrapping_add(0x1b9c),
                    if seed % 2 == 0 { 0.0 } else { 2.0 },
                );
                wf(
                    c,
                    GAME_GP.wrapping_sub(0x77fc),
                    if seed % 3 == 0 { 5.0 } else { 0.0 },
                );
                w(c, obj, 0);
            },
        );
    }

    // --------------------------------------------------- rider wipeout phase

    #[test]
    fn rider_wipeout_sub_update_phase4_matches() {
        check(
            ee::s00::f_0012d9d8__generated,
            rider_wipeout_sub_update_phase4,
            SEEDS,
            &|c, seed| {
                let _ = base(c, seed, 0x12d9_d8, 6);
                let obj = SCRATCH;
                let rider = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x80, rider);
                w(c, rider + 0x784, rider + 0x2000);
                wf(
                    c,
                    obj + 0x70,
                    match seed % 4 {
                        0 => 0.0,
                        1 => 2.0,
                        _ => 1.0,
                    },
                );
                wf(c, rider + 0x470, if seed % 5 == 0 { 2.0 } else { 0.0 });
                w(c, obj + 0x40, 0);
                w(c, obj + 0xc, 0);
                w(c, obj, 0);
            },
        );
    }
}
