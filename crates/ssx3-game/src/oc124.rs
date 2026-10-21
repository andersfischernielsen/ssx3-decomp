//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path front-end/online state handlers, lobby and
//! buddy helpers, a few UI render/update routines and small subsystem
//! destructors, each with a differential test against the generated body.
//!
//! The replacements follow the call protocol of [`crate::call`] (stack frame,
//! registers at each call, unwind check, cycles between calls) and the event
//! rules of [`crate::events`]; VU0 transfers are issued as the original's
//! `lqc2`/`sqc2` ops.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_d930,
        cfe_state_select_persona_on_save_account_popup_event,
        "cFEStateSelectPersona_OnSaveAccountPopupEvent",
    ),
    (
        0x001b_0fb0,
        cfe_state_online_character_select_init,
        "cFEStateOnlineCharacterSelect_Init",
    ),(
        0x001b_8a08,
        cfe_state_create_step1_transition_in,
        "cFEStateCreateStep1_TransitionIn",
    ),(
        0x001e_4168,
        ov_screen_message_center_on_activate,
        "OVScreenMessageCenter_OnActivate",
    ),(
        0x0026_33f8,
        buddy_man_is_same_product_at,
        "BuddyMan_IsSameProductAt",
    ),
    (
        0x0029_2a50,
        ssx_audio_system_start_rider_board_loops,
        "SSXAudioSystem_StartRiderBoardLoops",
    ),
    (
        0x0029_dbb0,
        ssx_audio_system_on_mission_abort,
        "SSXAudioSystem_OnMissionAbort",
    ),
    (0x002a_77f8, ini_parser_ctor, "IniParser_ctor"),
    (0x0027_d5e8, script_obj_cam_ctor, "ScriptObjCam_ctor"),
    (
        0x0027_d8d8,
        script_obj_cam_deactivate,
        "ScriptObjCam_Deactivate",
    ),(0x0023_f3a0, mc_manager_state0e, "MCManager_State0E"),
    (
        0x0024_0768,
        mc_manager_state19_delete,
        "MCManager_State19_Delete",
    ),
    (
        0x001c_8050,
        cfe_popup_confirm_set_text_from_desc,
        "cFEPopupConfirm_SetTextFromDesc",
    ),
    (
        0x001c_a980,
        cuistate_buy_popup_set_music_track_params,
        "cUIStateBuyPopup_SetMusicTrackParams",
    ),
    (
        0x001c_ab10,
        cuistate_buy_popup_set_item_params,
        "cUIStateBuyPopup_SetItemParams",
    ),
    (
        0x003b_0fb8,
        ps2_sony_codec_internal_get_picture,
        "PS2_SONY_CODEC_INTERNAL_GetPicture",
    ),(
        0x001d_11f8,
        cfe_state_reward_gallery_posters_init_buttons,
        "cFEStateRewardGalleryPosters_InitButtons",
    ),
    (
        0x001d_1390,
        cfe_state_reward_gallery_trading_cards_init_buttons,
        "cFEStateRewardGalleryTradingCards_InitButtons",
    ),
    (
        0x001d_1528,
        cfe_state_reward_gallery_concept_arts_init_buttons,
        "cFEStateRewardGalleryConceptArts_InitButtons",
    ),
    (
        0x001d_16c0,
        cfe_state_reward_gallery_videos_init_buttons,
        "cFEStateRewardGalleryVideos_InitButtons",
    ),
    (
        0x001d_1858,
        cfe_state_reward_gallery_toys_init_buttons,
        "cFEStateRewardGalleryToys_InitButtons",
    ),
    (
        0x001d_1990,
        cfe_state_reward_gallery_cheat_chars_init_buttons,
        "cFEStateRewardGalleryCheatChars_InitButtons",
    ),
    (
        0x0019_c7e8,
        cfe_state_char_equip_open_buy_popup,
        "cFEStateCharEquip_OpenBuyPopup",
    ),
    (0x0023_4e20, game_state_race_enter, "GameStateRace_Enter"),
    (
        0x0012_2e50,
        rider_estimate_final_score,
        "Rider_EstimateFinalScore",
    ),(
        0x001f_b180,
        ov_screen_in_game_error_update_server_timeout_text,
        "OVScreenInGameError_UpdateServerTimeoutText",
    ),
    (
        0x001f_e5b0,
        ov_screen_save_replay_v38,
        "OVScreenSaveReplay_v38",
    ),
    (
        0x001f_f278,
        ov_screen_save_records_v38,
        "OVScreenSaveRecords_v38",
    ),
    (
        0x0028_22a0,
        script_engine_run_process,
        "ScriptEngine_RunProcess",
    ),
    (
        0x0014_5d38,
        benew_player_int_is_char_profile_flag_clear,
        "BENewPlayerInt_IsCharProfileFlagClear",
    ),
    (0x0034_3b28, crowd_man3d_dtor, "CrowdMan3d_dtor"),
    (
        0x001d_4bc8,
        cfe_state_rewards_room_ctor,
        "cFEStateRewardsRoom_ctor",
    ),(0x0035_8668, rail_man_dtor, "RailMan_dtor"),
    (0x0039_c130, ui_pair_render, "UIPair_Render"),
    (0x0039_c300, cui_progress_render, "cUIProgress_Render"),(
        0x0021_1970,
        ov_screen_profile_save_pda_v38,
        "OVScreenProfileSavePDA_v38",
    ),
    (
        0x0020_cc60,
        game_ui_open_finish_screen,
        "GameUI_OpenFinishScreen",
    ),
    (
        0x001d_28f8,
        cfe_state_mountain_room_dtor,
        "cFEStateMountainRoom_dtor",
    ),];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };
    pub const NET_ACCOUNT_MEM_CTOR: Callee = Callee {
        addr: 0x001b_6ed0,
        f: ee::s03::cFEStateNetAccountMem_ctor,
    };
    pub const SHOW_CHAR_STATS: Callee = Callee {
        addr: 0x001b_1b98,
        f: ee::s03::cFEStateOnlineCharacterSelect_ShowCharStats,
    };
    pub const START_CARD_SCAN: Callee = Callee {
        addr: 0x001b_5268,
        f: ee::s03::cFEStateNetConfig_StartCardScan,
    };
    pub const SHOW_CHECKING_CARD_POPUP: Callee = Callee {
        addr: 0x001b_47c0,
        f: ee::s03::cFEStateNetConfig_ShowCheckingCardPopup,
    };
    pub const CREATE_STEP1_SET_ACCOUNT_NAME: Callee = Callee {
        addr: 0x001b_95c0,
        f: ee::s03::cFEStateCreateStep1_SetAccountNameText,
    };
    pub const CREATE_STEP1_SET_PASSWORD: Callee = Callee {
        addr: 0x001b_9610,
        f: ee::s03::cFEStateCreateStep1_SetPasswordText,
    };
    pub const CREATE_STEP1_SET_CONFIRM_PASSWORD: Callee = Callee {
        addr: 0x001b_9678,
        f: ee::s03::cFEStateCreateStep1_SetConfirmPasswordText,
    };
    pub const CREATE_STEP1_SET_EMAIL: Callee = Callee {
        addr: 0x001b_96e0,
        f: ee::s03::cFEStateCreateStep1_SetEmailText,
    };
    pub const CREATE_STEP1_SET_SHARE_INFO: Callee = Callee {
        addr: 0x001b_97b0,
        f: ee::s03::cFEStateCreateStep1_SetShareInfoSelection,
    };
    pub const CREATE_STEP1_SET_INFORM: Callee = Callee {
        addr: 0x001b_9740,
        f: ee::s03::cFEStateCreateStep1_SetInformSelection,
    };
    pub const CREATE_STEP1_BACKUP_ACCOUNT: Callee = Callee {
        addr: 0x001b_a178,
        f: ee::s03::cFEStateCreateStep1_BackupAccountInfo,
    };
    pub const CREATE_STEP1_ALL_FIELDS_FILLED: Callee = Callee {
        addr: 0x001b_9558,
        f: ee::s03::cFEStateCreateStep1_AllFieldsFilled,
    };
    pub const LOGIN_ON_ACCOUNT_MEM_POPUP: Callee = Callee {
        addr: 0x001b_f438,
        f: ee::s03::cFEStateLogin_OnAccountMemPopupEvent,
    };
    pub const LOGIN_ON_DISCONNECT_POPUP: Callee = Callee {
        addr: 0x001b_f5a8,
        f: ee::s03::cFEStateLogin_OnDisconnectPopupEvent,
    };
    pub const MESSAGE_CENTER_GET_LIST_INDEX: Callee = Callee {
        addr: 0x001e_3680,
        f: ee::s04::MessageCenter_GetListIndex,
    };
    pub const MESSAGE_CENTER_OPEN_VIEW_POPUP: Callee = Callee {
        addr: 0x001e_50c8,
        f: ee::s04::OVScreenMessageCenter_OpenViewPopup,
    };
    pub const LOBBY_MAN_SEND_REQUEST: Callee = Callee {
        addr: 0x0025_b608,
        f: ee::s06::LobbyMan_SendRequest,
    };
    pub const LOBBY_MAN_GET_LANGUAGE: Callee = Callee {
        addr: 0x0025_9198,
        f: ee::s06::LobbyMan_GetLanguage,
    };
    pub const BUDDY_MAN_GET_ROSTER_COUNT: Callee = Callee {
        addr: 0x0026_2598,
        f: ee::s07::BuddyMan_GetRosterCount,
    };
    pub const BUDDY_MAN_GET_ROSTER_ENTRY: Callee = Callee {
        addr: 0x0026_25c8,
        f: ee::s07::BuddyMan_GetRosterEntry,
    };
    pub const SSX_AUDIO_GET_SURFACE_SOUND_TYPE: Callee = Callee {
        addr: 0x0029_1710,
        f: ee::s07::SSXAudioSystem_GetSurfaceSoundType,
    };
    pub const SSX_AUDIO_START_BOARD_BASE_LOOP: Callee = Callee {
        addr: 0x0029_17d0,
        f: ee::s07::SSXAudioSystem_StartBoardBaseLoop,
    };
    pub const SSX_AUDIO_START_BOARD_SURFACE_LOOP: Callee = Callee {
        addr: 0x0029_1c88,
        f: ee::s07::SSXAudioSystem_StartBoardSurfaceLoop,
    };
    pub const SSX_AUDIO_START_BOARD_EDGE_LOOP: Callee = Callee {
        addr: 0x0029_2508,
        f: ee::s07::SSXAudioSystem_StartBoardEdgeLoop,
    };
    pub const SSX_AUDIO_PLAY_FE_SOUND: Callee = Callee {
        addr: 0x0029_4f78,
        f: ee::s08::SSXAudioSystem_PlayFESound,
    };
    pub const SSX_AUDIO_STOP_WIPEOUT_LOOP: Callee = Callee {
        addr: 0x0029_6e20,
        f: ee::s08::SSXAudioSystem_StopWipeoutLoop,
    };
    pub const SSX_AUDIO_STOP_WIPEOUT_LOOP2: Callee = Callee {
        addr: 0x0029_7438,
        f: ee::s08::SSXAudioSystem_StopWipeoutLoop2,
    };
    pub const SSX_AUDIO_STOP_TRICK_METER_SOUND: Callee = Callee {
        addr: 0x0029_b3c0,
        f: ee::s08::SSXAudioSystem_StopTrickMeterSound,
    };
    pub const SCRIPT_OBJ_CTOR: Callee = Callee {
        addr: 0x0028_3200,
        f: ee::s07::ScriptObj_ctor,
    };
    pub const SCRIPT_OBJ_DEACTIVATE: Callee = Callee {
        addr: 0x0028_3440,
        f: ee::s07::ScriptObj_Deactivate,
    };
    pub const REPLAY_FREE_SNAPSHOT: Callee = Callee {
        addr: 0x0027_0238,
        f: ee::s07::Replay_FreeSnapshot,
    };
    pub const CODEC_POP_FREE_FRAME: Callee = Callee {
        addr: 0x003b_10d0,
        f: ee::s13::PS2_SONY_CODEC_INTERNAL_PopFreeFrame,
    };
    pub const MPEG2_GET_BITS: Callee = Callee {
        addr: 0x003b_1300,
        f: ee::s13::MPEG2_Get_Bits,
    };
    pub const MPEG2_MARKER_BIT: Callee = Callee {
        addr: 0x003b_2360,
        f: ee::s13::MPEG2_marker_bit,
    };
    pub const BUY_POPUP_INIT_BUY_BOLT: Callee = Callee {
        addr: 0x001c_ab10,
        f: ee::s04::cUIStateBuyPopup_SetItemParams,
    };
    pub const RIDER_GET_START_DIST_TO_FINISH: Callee = Callee {
        addr: 0x0011_3130,
        f: ee::s00::Rider_GetStartDistToFinish,
    };
    pub const BE_GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::BE_GetRiderProfile,
    };
    pub const BENEW_PLAYER_INT_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::BENewPlayerInt_GetSlot,
    };
    pub const SCRIPT_ENGINE_GET_PROCESS: Callee = Callee {
        addr: 0x0028_2cb0,
        f: ee::s07::ScriptEngine_GetProcess,
    };
    pub const IN_SEQ_PLAY: Callee = Callee {
        addr: 0x0027_2ad0,
        f: ee::s07::InSeq_Play,
    };
    pub const CROWD_MAN_GROUP_FREE: Callee = Callee {
        addr: 0x0034_4348,
        f: ee::s11::CrowdMan3d_GroupFree,
    };
    pub const RAIL_MAN_ENABLE_RAIL: Callee = Callee {
        addr: 0x0035_8998,
        f: ee::s11::RailMan_EnableRail,
    };
    pub const SYNC_PROCESS_DTOR: Callee = Callee {
        addr: 0x0035_4920,
        f: ee::s11::SyncProcess_dtor,
    };
    pub const TEX_MAN_EVICT_TEXTURE: Callee = Callee {
        addr: 0x0036_9098,
        f: ee::s12::TexMan_EvictTexture,
    };
    pub const VRAM_HEAP_FREE_TEXTURE_BLOCKS: Callee = Callee {
        addr: 0x0036_6e30,
        f: ee::s12::VramHeap_FreeTextureBlocks,
    };
    pub const GAME_UI_OPEN_SCREEN: Callee = Callee {
        addr: 0x0020_ab50,
        f: ee::s05::GameUI_OpenScreen,
    };
    pub const WORLD_CACHE_RESET: Callee = Callee {
        addr: 0x003a_7f90,
        f: ee::s13::cWorldCache_Reset,
    };
    pub const WORLD_CACHE_LOAD: Callee = Callee {
        addr: 0x003a_7fc0,
        f: ee::s13::cWorldCache_Load,
    };
    pub const STREAMER_GET_SECTION_COUNT: Callee = Callee {
        addr: 0x003a_9800,
        f: ee::s13::cWorldCache_StreamerGetSectionCount,
    };
    pub const WSCRIPT_CACHE_ALLOC: Callee = Callee {
        addr: 0x003a_c800,
        f: ee::s13::cWScriptCache_Alloc,
    };
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
mod table_callee {
    pub const GET_HASH_VALUE32: u32 = 0x0031_7670;
    pub const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    pub const STRLEN: u32 = 0x0031_7f98;
    pub const OPERATOR_NEW: u32 = 0x0031_7e30;
    pub const STRCPY: u32 = 0x003e_1b68;
    pub const UI_ENGINE_ADD_SCREEN_BY_HASH_NAME: u32 = 0x0039_7cc0;
    pub const UI_SCREEN_PLAY_FRAME: u32 = 0x0039_c870;
    pub const UI_OBJECT_PUSH: u32 = 0x0039_f290;
    pub const UI_SCREEN_ADD_OBJECT: u32 = 0x0039_f4c0;
    pub const UI_ENGINE_GET_INTERFACE: u32 = 0x0025_6c50;
    pub const UI_MENU_SET_SELECTED_BY_INDEX: u32 = 0x0039_b960;
    pub const TAG_FIELD_SET_STRING: u32 = 0x003e_c180;
    pub const TAG_FIELD_FIND: u32 = 0x003e_b6d8;
    pub const TAG_FIELD_GET_STRING4: u32 = 0x003e_d378;
    pub const BUDDY_PRODUCT_COMPARE: u32 = 0x0041_65a8;
    pub const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;
    pub const SSX_AUDIO_GET_RACE_MAN: u32 = 0x0028_b1d8;
    pub const RIDER_REMOVE_SCRIPT_OBJECT: u32 = 0x0016_92d8;
    pub const RIDER_SET_SCRIPT_STATE: u32 = 0x0015_dfd8;
    pub const RIDER_SCRIPT_CLEANUP: u32 = 0x0016_2170;
    pub const UI_TEXT_SET_RICH_STRING: u32 = 0x003a_0e90;
    pub const UI_TEXT_SET_ASCII_STRING: u32 = 0x003a_0c50;
    pub const UI_TEXT_SET_UNICODE_STRING_BY_ID: u32 = 0x003a_1000;
    pub const CODEC_DECODE_FRAME: u32 = 0x0040_2a10;
    pub const BE_GET: u32 = 0x0014_dc80;
    pub const BE_GET_INTERFACE_FV: u32 = 0x0014_dd58;
    pub const REWARD_GALLERY_GET_BASE: u32 = 0x0015_a478;
    pub const BUY_POPUP_CTOR: u32 = 0x001c_aba8;
    pub const REWARD_GALLERY_POSTERS: u32 = 0x0015_72b0;
    pub const REWARD_GALLERY_TRADING_CARDS: u32 = 0x0015_72d0;
    pub const REWARD_GALLERY_CONCEPT_ARTS: u32 = 0x0015_72f0;
    pub const REWARD_GALLERY_VIDEOS: u32 = 0x0015_7330;
    pub const REWARD_GALLERY_TOYS: u32 = 0x0015_7310;
    pub const REWARD_GALLERY_CHEAT_CHARS: u32 = 0x0015_7370;
    pub const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;
    pub const UI_OPEN_FINISH_SCREEN: u32 = 0x0039_f840;
    pub const OPERATOR_DELETE: u32 = 0x0031_7e50;
    pub const AI_SET_AI_STATE: u32 = 0x0011_3b10;
    pub const BE_SET_STATE: u32 = 0x0014_de68;
    pub const FE_STATE_CTOR: u32 = 0x0039_e2a0;
    pub const BE_GET_TEXT_FROM_ID: u32 = 0x001a_1cd0;
    pub const BENEW_PLAYER_INT_GET_CHAR_ID: u32 = 0x0014_7488;
    pub const FE_MODULE_SMALL_CTOR: u32 = 0x0017_f7b8;
    pub const FE_MODULE_BIG_CTOR: u32 = 0x001a_1ce8;
    pub const APP_MAN_SET_NEXT_MODULE: u32 = 0x0031_6ef8;
    pub const UPLOAD_TEXTURE_PIXELS: u32 = 0x003e_6574;
    pub const MUSIC_SLOT_RELEASE: u32 = 0x0019_dc20;
    pub const UI_STATE_DTOR: u32 = 0x0039_e390;
    pub const BXSTRING_CTOR: u32 = 0x0031_82d8;
    pub const BXSTRING_DTOR: u32 = 0x0031_8260;
    pub const SECTION_TABLE_CTOR: u32 = 0x002b_b1d0;
}

// ---------------------------------------------------------------------------
// Small helpers.

/// The address `reg + off`.
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    abi::r32(c, reg).wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    c.g[GP][0] as u32 + off as u32
}

/// `lw reg + off`.
#[inline(always)]
fn lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(at(c, reg, off)).lw(c)
}

// ---------------------------------------------------------------------------
// Front-end / online state handlers.

/// `void func_001AD930(void* self, void* child, int event)` (0x1ad930): when
/// the save-account popup closes with event 0xF, allocate and post a
/// `cFEStateNetAccountMem` (0xB10 bytes) to the screen's object queue;
/// otherwise forward the child event.
pub fn cfe_state_select_persona_on_save_account_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 25;
            c.g[A2][0] = 0xf;
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001a_d950)?;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0xb10;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d10));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, table_callee::EA_MEM_ALLOC_NAMED, 0x001a_d968)?;
            let mem = c.g[V0][0];
            c.g[A1][0] = lw(c, S0, 0x10);
            c.g[A0][0] = mem;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = 0;
            callee::NET_ACCOUNT_MEM_CTOR.call(c, 0x001a_d984)?;
            c.g[V1][0] = 0x832;
            c.g[A1][0] = c.g[V0][0];
            Addr(c.g[V0][0] as u32).off(0x18).sw(c, c.g[V1][0]);
            c.g[V0][0] = lw(c, S0, 0x40);
            c.g[V1][0] = lw(c, V0, 0xd0);
            c.g[A0][0] = lw(c, V1, 0x10);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, table_callee::UI_OBJECT_PUSH, 0x001a_d9a4)?;
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001a_d9b4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOnlineCharacterSelect_Init(self)` (0x1b0fb0): add the screen
/// by hash, play its first frame and show the stat line of the 30 characters.
pub fn cfe_state_online_character_select_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x460000;
        Addr(c.g[S1][0] as u32).off(0x6e0).sw(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3800));
        c.g[S0][0] = lw(c, S1, 0x10);
        call_ptr(c, table_callee::GET_HASH_VALUE32, 0x001b_0fd8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(
            c,
            table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME,
            0x001b_0fec,
        )?;
        let screen = c.g[V0][0];
        Addr(c.g[S1][0] as u32).off(0x40).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, table_callee::UI_SCREEN_PLAY_FRAME, 0x001b_1004)?;
        }
        c.cycles += 3;
        c.g[S0][0] = 0;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 1;
            c.g[A3][0] = 0;
            callee::SHOW_CHAR_STATS.call(c, 0x001b_1020)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 30) as u64;
            c.g[A1][0] = c.g[S0][0];
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001b_1010);
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

/// `void cFEStateNetConfig_Init(self)` (0x1b40d0): add the screen by hash,
/// optionally drop the network adapter interface, then start the card scan
/// and the network config module.
pub fn cfe_state_net_config_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ed0));
        c.g[S0][0] = lw(c, S1, 0x10);
        call_ptr(c, table_callee::GET_HASH_VALUE32, 0x001b_40f4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(
            c,
            table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME,
            0x001b_4108,
        )?;
        let screen = c.g[V0][0];
        Addr(c.g[S1][0] as u32).off(0x40).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, table_callee::UI_SCREEN_PLAY_FRAME, 0x001b_4120)?;
        }
        // A loaded network adapter gets dropped before the scan restarts.
        c.cycles += 3;
        c.g[V0][0] = Addr(gp(c, -0x204)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, table_callee::UI_ENGINE_GET_INTERFACE, 0x001b_4134)?;
            c.g[A0][0] = c.g[S1][0];
        }
        c.cycles += 11;
        c.g[A1][0] = 0;
        callee::START_CARD_SCAN.call(c, 0x001b_4140)?;
        call_ptr(c, 0x0028_b180, 0x001b_4148)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_fc38, 0x001b_4150)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateNetConfig_OnRetryPopupEvent(self, child, event)` (0x1b4c40):
/// the checking-card popup's outcome selects whether the config state is
/// flagged complete (`0x70C`/`0x708`) or the child event is forwarded.
pub fn cfe_state_net_config_on_retry_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x10;
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let event = c.g[A2][0];
        let at_1b4ca8 = event == c.g[V0][0];
        frame.save_ra(c, 0);
        // 1 = case 0x0F, 2 = fall through the first switch arm,
        // 3 = the 0x10/0x14 arm, 4 = the default (forward) arm.
        let mut stage = 0u8;
        if at_1b4ca8 {
            stage = 3;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (event < 0x11) as u64;
            let below = c.g[V0][0] == 0;
            c.g[V0][0] = 0xf;
            if below {
                stage = 2;
            } else if event == c.g[V0][0] {
                Addr(gp(c, -0x1570)).sw(c, 0);
                stage = 1;
            } else {
                c.cycles += 2;
                stage = 4;
            }
        }
        if stage == 2 {
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            if event == c.g[V0][0] {
                stage = 3;
            } else {
                c.cycles += 2;
                stage = 4;
            }
        }
        match stage {
            1 => {
                c.cycles += 7;
                c.g[S0][0] = 1;
                Addr(c.g[S1][0] as u32).off(0x70c).sw(c, c.g[S0][0]);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 1;
                callee::SHOW_CHECKING_CARD_POPUP.call(c, 0x001b_4ca0)?;
                Addr(c.g[S1][0] as u32).off(0x708).sw(c, c.g[S0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            3 => {
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_4cc0)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
            _ => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_4cb0)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCreateStep1_TransitionIn(self, enabled)` (0x1b8a08): refresh
/// the account text fields and, if the state owns a create-step popup, tell
/// it whether every field is filled.
pub fn cfe_state_create_step1_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let enabled = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if enabled {
            c.cycles += 17;
            callee::CREATE_STEP1_SET_ACCOUNT_NAME.call(c, 0x001b_8a28)?;
            c.g[A0][0] = c.g[S1][0];
            callee::CREATE_STEP1_SET_PASSWORD.call(c, 0x001b_8a30)?;
            c.g[A0][0] = c.g[S1][0];
            callee::CREATE_STEP1_SET_CONFIRM_PASSWORD.call(c, 0x001b_8a38)?;
            c.g[A0][0] = c.g[S1][0];
            callee::CREATE_STEP1_SET_EMAIL.call(c, 0x001b_8a40)?;
            c.g[A0][0] = c.g[S1][0];
            callee::CREATE_STEP1_SET_SHARE_INFO.call(c, 0x001b_8a48)?;
            c.g[A0][0] = c.g[S1][0];
            callee::CREATE_STEP1_SET_INFORM.call(c, 0x001b_8a50)?;
            c.g[A0][0] = c.g[S1][0];
            callee::CREATE_STEP1_BACKUP_ACCOUNT.call(c, 0x001b_8a58)?;
            c.g[V0][0] = lw(c, S1, 0x6f0);
            c.g[A0][0] = c.g[S1][0];
            if c.g[V0][0] != 0 {
                c.cycles += 8;
                c.g[S0][0] = lw(c, V0, 0x8);
                c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x40).lh(c);
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                callee::CREATE_STEP1_ALL_FIELDS_FILLED.call(c, 0x001b_8a74)?;
                c.g[V1][0] = lw(c, S0, 0x44);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[V0][0] ^ 1;
                call_ptr(c, c.g[V1][0] as u32, 0x001b_8a84)?;
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateLogin_OnChildEvent(self, child, event)` (0x1bf6c0): dispatch
/// the login state's child events (account-memory popup, disconnect popup,
/// keyboard popup) and forward everything else.
pub fn cfe_state_login_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x82c;
        let event = Addr(c.g[A1][0] as u32).off(0x18).lw(c);
        c.g[V1][0] = event;
        let is_82c = event == c.g[V0][0];
        c.g[A3][0] = c.g[A0][0];
        // 0 = default (forward), 1 = 0x82C, 2 = 0x80E, 3 = 0x848.
        let mut stage = 0u8;
        if is_82c {
            stage = 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (event < 0x82d) as u64;
            let above = c.g[V0][0] == 0;
            c.g[V0][0] = 0x80e;
            if above {
                c.cycles += 3;
                c.g[V0][0] = 0x848;
                if event == c.g[V0][0] {
                    stage = 3;
                } else {
                    c.cycles += 2;
                    stage = 0;
                }
            } else if event == c.g[V0][0] {
                c.g[V0][0] = lw(c, A3, 0x8);
                stage = 2;
            } else {
                c.cycles += 2;
                stage = 0;
            }
        }
        match stage {
            1 => {
                c.cycles += 4;
                c.g[A0][0] = c.g[A3][0];
                callee::LOGIN_ON_ACCOUNT_MEM_POPUP.call(c, 0x001b_f728)?;
            }
            2 => {
                c.cycles += 6;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x108).lh(c);
                c.g[V1][0] = lw(c, V0, 0x10c);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x001b_f718)?;
            }
            3 => {
                c.cycles += 4;
                c.g[A0][0] = c.g[A3][0];
                callee::LOGIN_ON_DISCONNECT_POPUP.call(c, 0x001b_f738)?;
            }
            _ => {
                c.cycles += 3;
                c.g[A0][0] = c.g[A3][0];
                callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_f748)?;
            }
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMessageCenter_OnActivate(self)` (0x1e4168): when the message
/// centre opens (state 1) select the first entry of the message list, refresh
/// the menu and open the view popup; otherwise just refresh the menu.
pub fn ov_screen_message_center_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = lw(c, S0, 0x18);
        c.g[V0][0] = 1;
        let mut open = c.g[V1][0] == 0;
        if !open {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                // The non-reset arm re-reads the menu pointer.
                c.g[A2][0] = lw(c, S0, 0x144);
            } else {
                open = true;
            }
        }
        if open {
            c.cycles += 18;
            c.g[A0][0] = 0x25;
            callee::MESSAGE_CENTER_GET_LIST_INDEX.call(c, 0x001e_4194)?;
            c.g[A0][0] = lw(c, S0, 0x144);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, table_callee::UI_MENU_SET_SELECTED_BY_INDEX, 0x001e_41a0)?;
            c.g[A2][0] = lw(c, S0, 0x144);
            c.g[A1][0] = 1;
            c.g[V0][0] = lw(c, A2, 0x8);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x38).lh(c);
            c.g[V1][0] = lw(c, V0, 0x3c);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001e_41bc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x25;
            c.g[A2][0] = u64::MAX;
            callee::MESSAGE_CENTER_OPEN_VIEW_POPUP.call(c, 0x001e_41cc)?;
        } else {
            c.cycles += 7;
            c.g[A1][0] = 1;
            c.g[V0][0] = lw(c, A2, 0x8);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x38).lh(c);
            c.g[V1][0] = lw(c, V0, 0x3c);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001e_41ec)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_MoveToRoom(self, room, name)` (0x25d860): build the
/// dirtysock tag list for a "move to room" request and send it.
pub fn lobby_man_move_to_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        frame.save_ra(c, 0x100);
        c.g[S1][0] = c.g[A0][0];
        let room = c.g[A1][0];
        let name = c.g[A2][0];
        c.g[S0][0] = name;
        if lw(c, S1, 0) != 0 {
            c.cycles += 9;
            c.g[A2][0] = 0x4a0000;
            c.g[A3][0] = room;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f58));
            Addr(c.g[SP][0] as u32).sb(c, 0);
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            call_ptr(c, table_callee::TAG_FIELD_SET_STRING, 0x0025_d89c)?;
            c.g[A2][0] = 0x4a0000;
            if c.g[S0][0] != 0 {
                c.cycles += 5;
                c.g[A3][0] = c.g[S0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3040));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0x100;
                call_ptr(c, table_callee::TAG_FIELD_SET_STRING, 0x0025_d8b8)?;
            }
            c.cycles += 10;
            c.g[T0][0] = 0x270000;
            c.g[A2][0] = 0x6d6f0000;
            c.g[AT][0] = 0x41a00000;
            c.mtc1(AT, 12);
            c.g[A0][0] = c.g[S1][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x72c8));
            c.g[A1][0] = 0xc;
            c.g[A2][0] = c.g[A2][0] | 0x7665;
            c.g[A3][0] = c.g[SP][0];
            callee::LOBBY_MAN_SEND_REQUEST.call(c, 0x0025_d8e0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_ReportUser(self, name)` (0x25df00): tag the player's name
/// and the console language and send a "report user" request.
pub fn lobby_man_report_user(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S1, 0x110);
        frame.save(c, S0, 0x120);
        frame.save_ra(c, 0x100);
        c.g[S1][0] = c.g[A0][0];
        let name = c.g[A1][0];
        c.g[A2][0] = 0x4a0000;
        if lw(c, S1, 0) != 0 {
            c.cycles += 24;
            c.g[A3][0] = name;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3108));
            Addr(c.g[SP][0] as u32).sb(c, 0);
            c.g[A1][0] = 0x100;
            call_ptr(c, table_callee::TAG_FIELD_SET_STRING, 0x0025_df38)?;
            c.g[S0][0] = 0x4a0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fe0));
            callee::LOBBY_MAN_GET_LANGUAGE.call(c, 0x0025_df44)?;
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            call_ptr(c, table_callee::TAG_FIELD_SET_STRING, 0x0025_df58)?;
            c.g[T0][0] = 0x270000;
            c.g[A2][0] = 0x72650000;
            c.g[AT][0] = 0x41a00000;
            c.mtc1(AT, 12);
            c.g[A0][0] = c.g[S1][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7250));
            c.g[A1][0] = 0x10;
            c.g[A2][0] = c.g[A2][0] | 0x7074;
            c.g[A3][0] = c.g[SP][0];
            callee::LOBBY_MAN_SEND_REQUEST.call(c, 0x0025_df80)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `int BuddyMan_IsSameProductAt(BuddyMan* self, int idx)` (0x2633f8): true if
/// roster entry `idx`'s product tag equals this console's product.
pub fn buddy_man_is_same_product_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let idx_negative = (c.g[S0][0] as i64) < 0;
        c.g[S1][0] = c.g[A0][0];
        let mut zero = false;
        if idx_negative {
            zero = true;
        } else {
            c.cycles += 5;
            callee::BUDDY_MAN_GET_ROSTER_COUNT.call(c, 0x0026_341c)?;
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            let out_of_range = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if !out_of_range {
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S1][0];
                callee::BUDDY_MAN_GET_ROSTER_ENTRY.call(c, 0x0026_3434)?;
                c.g[A1][0] = 0x4a0000;
                if c.g[V0][0] == 0 {
                    zero = true;
                } else {
                    c.cycles += 14;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x54));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x31f8));
                    call_ptr(c, table_callee::TAG_FIELD_FIND, 0x0026_3448)?;
                    c.g[A3][0] = 0x4a0000;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = 0x10;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f40));
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, table_callee::TAG_FIELD_GET_STRING4, 0x0026_3460)?;
                    c.g[A0][0] = lw(c, S1, 0xec);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, table_callee::BUDDY_PRODUCT_COMPARE, 0x0026_346c)?;
                    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
                }
            }
        }
        if zero {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StartRiderBoardLoops(self, rider)` (0x292a50): cache
/// the rider's main-state and surface type and start the three board loops.
pub fn ssx_audio_system_start_rider_board_loops(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, table_callee::RIDER_GET_MAIN_STATE, 0x0029_2a78)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE.call(c, 0x0029_2a88)?;
        c.g[S1][0] = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x764).sw(c, c.g[S3][0]);
        Addr(c.g[S0][0] as u32).off(0x760).sw(c, c.g[S3][0]);
        c.g[A0][0] = c.g[S2][0];
        Addr(c.g[S0][0] as u32).off(0x768).sw(c, c.g[S1][0]);
        c.g[A1][0] = c.g[S0][0];
        Addr(c.g[S0][0] as u32).off(0x76c).sw(c, c.g[S1][0]);
        callee::SSX_AUDIO_START_BOARD_BASE_LOOP.call(c, 0x0029_2aa8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP.call(c, 0x0029_2ab8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        callee::SSX_AUDIO_START_BOARD_EDGE_LOOP.call(c, 0x0029_2ac8)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_OnMissionAbort(self, keep)` (0x29dbb0): if the mission
/// run is live, stop the wipeout/trick sounds, send event 0x27 and clear the
/// mission flags.
pub fn ssx_audio_system_on_mission_abort(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, S0, 0x5fd4);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            if c.g[A1][0] == 0 {
                c.cycles += 2;
                c.g[A1][0] = 0xe;
                callee::SSX_AUDIO_PLAY_FE_SOUND.call(c, 0x0029_dbdc)?;
            }
            c.cycles += 23;
            call_ptr(c, table_callee::SSX_AUDIO_GET_RACE_MAN, 0x0029_dbe4)?;
            c.g[A1][0] = lw(c, V0, 0x28);
            c.g[A0][0] = c.g[S0][0];
            callee::SSX_AUDIO_STOP_WIPEOUT_LOOP.call(c, 0x0029_dbf0)?;
            call_ptr(c, table_callee::SSX_AUDIO_GET_RACE_MAN, 0x0029_dbf8)?;
            c.g[A1][0] = lw(c, V0, 0x28);
            c.g[A0][0] = c.g[S0][0];
            callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2.call(c, 0x0029_dc04)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            callee::SSX_AUDIO_STOP_TRICK_METER_SOUND.call(c, 0x0029_dc10)?;
            c.g[V1][0] = lw(c, S0, 0x5558);
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = 0x27;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
            c.g[V0][0] = lw(c, V1, 0x24);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0029_dc2c)?;
            Addr(c.g[S0][0] as u32).off(0x5fd4).sw(c, 0);
            Addr(c.g[S0][0] as u32).off(0x5fd0).sw(c, 0);
            frame.restore(c, S0, 0x10);
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `IniParser* IniParser_ctor(IniParser* self, char* text, u32 flags)`
/// (0x2a77f8): measure the text, copy it into a fresh allocation and point the
/// cursor at the copy's start.
pub fn ini_parser_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        Addr(c.g[S0][0] as u32).off(0x4).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr(c.g[S0][0] as u32).off(0x8).sw(c, 0);
        call_ptr(c, table_callee::STRLEN, 0x002a_7828)?;
        let size = c.g[V0][0];
        Addr(c.g[S0][0] as u32).sw(c, size);
        if size != 0 {
            c.cycles += 16;
            c.g[A2][0] = c.g[S2][0];
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0;
            call_ptr(c, table_callee::OPERATOR_NEW, 0x002a_7844)?;
            let text = c.g[V0][0];
            c.g[A2][0] = lw(c, S0, 0);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = text;
            Addr(c.g[S0][0] as u32).off(0x4).sw(c, text);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            call_ptr(c, table_callee::STRCPY, 0x002a_785c)?;
            c.g[V1][0] = lw(c, S0, 0x4);
            c.g[V0][0] = lw(c, S0, 0);
            Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V1][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            Addr(c.g[V1][0] as u32).sb(c, 0);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptObjCam* ScriptObjCam_ctor(ScriptObjCam* self, u32 owner)` (0x27d5e8):
/// build the scripted camera object, copy the default 4x4 matrix and clear the
/// follow target.
pub fn script_obj_cam_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        callee::SCRIPT_OBJ_CTOR.call(c, 0x0027_d608)?;
        c.g[V1][0] = 0x480000;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20b8));
        c.g[V0][0] = 0x500000;
        Addr(c.g[S0][0] as u32).off(0xc).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
        Addr(c.g[S0][0] as u32).off(0x20).sw(c, c.g[A0][0]);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
        Addr(c.g[S0][0] as u32).off(0x10).sw(c, 0);
        Addr(c.g[S0][0] as u32).off(0x14).sw(c, 0);
        Addr(c.g[S0][0] as u32).off(0x18).sw(c, 0);
        c.lqc2(1, c.g[V0][0] as u32);
        c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
        c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.sqc2(1, c.g[V1][0] as u32);
        c.sqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.sqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
        c.sqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
        Addr(c.g[S0][0] as u32).off(0x7c).sw(c, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(c.g[S0][0] as u32).off(0x70).sw(c, 0);
        Addr(c.g[S0][0] as u32).off(0x74).sw(c, 0);
        Addr(c.g[S0][0] as u32).off(0x78).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptObjCam_Deactivate(self)` (0x27d8d8): detach the camera from its
/// rider and release the rider's scripted object when nothing else holds it.
pub fn script_obj_cam_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_OBJ_DEACTIVATE.call(c, 0x0027_d8ec)?;
        let world = Addr(gp(c, -0x848)).lw(c);
        c.g[A0][0] = world;
        c.g[V0][0] = Addr(world as u32).off(0x84).lw(c);
        c.g[A1][0] = c.g[S0][0];
        if c.g[V0][0] != 0 {
            c.cycles += 13;
            Addr(c.g[S0][0] as u32).off(0x18).sw(c, 0);
            c.g[V1][0] = lw(c, S0, 0x7c);
            c.g[A0][0] = Addr(world as u32).off(0x84).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = lw(c, A0, 0x84);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S0][0] = lw(c, V0, 0x4);
            c.g[A0][0] = lw(c, S0, 0xac);
            call_ptr(c, table_callee::RIDER_REMOVE_SCRIPT_OBJECT, 0x0027_d920)?;
            c.g[V0][0] = lw(c, S0, 0xac);
            c.g[V1][0] = lw(c, V0, 0x18);
            if c.g[V1][0] == 0 {
                c.cycles += 11;
                c.g[A1][0] = 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, table_callee::RIDER_SET_SCRIPT_STATE, 0x0027_d93c)?;
                c.g[A0][0] = lw(c, S0, 0xa8);
                call_ptr(c, table_callee::RIDER_SCRIPT_CLEANUP, 0x0027_d944)?;
                c.g[A1][0] = lw(c, S0, 0xa0);
                c.g[V0][0] = lw(c, A1, 0x14);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
                c.g[V1][0] = lw(c, V0, 0x24);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0027_d95c)?;
            }
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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
                    c.g[V0][0] = lw(c, A2, 0x3ac);
                    pc = 0x0027_01f4;
                }
                0x0027_01b8 => {
                    c.cycles += 4;
                    c.g[A1][0] = lw(c, V0, 0x14);
                    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x1f).lbu(c);
                    let used = c.g[V1][0] != 0;
                    c.g[A0][0] = c.g[V0][0];
                    pc = if used { 0x0027_01ec } else { 0x0027_01c8 };
                }
                0x0027_01c8 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x1e).lbu(c);
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw(c, A0, 0x14);
                        pc = 0x0027_01f0;
                    } else {
                        pc = 0x0027_01d4;
                    }
                }
                0x0027_01d4 => {
                    c.cycles += 6;
                    c.g[V0][0] = lw(c, A1, 0x14);
                    c.g[A0][0] = c.g[A2][0];
                    Addr(c.g[A2][0] as u32).off(0x3ac).sw(c, c.g[V0][0]);
                    callee::REPLAY_FREE_SNAPSHOT.call(c, 0x0027_01e4)?;
                    c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
                    pc = 0x0027_022c;
                }
                0x0027_01ec => {
                    c.cycles += 1;
                    c.g[V0][0] = lw(c, A0, 0x14);
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
                        c.g[V1][0] = lw(c, A2, 0x3b0);
                        pc = 0x0027_0218;
                    } else {
                        pc = 0x0027_01fc;
                    }
                }
                0x0027_01fc => {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, V0, 0x14);
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = lw(c, A2, 0x3b0);
                        pc = 0x0027_0218;
                    } else {
                        pc = 0x0027_0208;
                    }
                }
                0x0027_0208 => {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, V0, 0x14);
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw(c, A2, 0x3ac);
                        if back_edge_unwinds(c, 0x0027_01b8) {
                            return Ok(());
                        }
                    }
                    pc = 0x0027_01b8;
                }
                0x0027_0214 => {
                    c.cycles += 1;
                    c.g[V1][0] = lw(c, A2, 0x3b0);
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

/// Shared body of the two `MCManager_State*` movie checks: if the movie
/// finished and the second flag is set, send the "next" event.
fn mc_manager_state_check(c: &mut Cpu, entry: u32, first: u32, second: u32) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A1][0] = lw(c, S0, 0x434);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
    c.g[V0][0] = lw(c, V1, 0x5c);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, c.g[V0][0] as u32, entry + 0x28)?;
    let mut late = false;
    if c.g[V0][0] == 0 {
        c.cycles += 8;
        c.g[A1][0] = lw(c, S0, 0x434);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x248).lh(c);
        c.g[V0][0] = lw(c, V1, 0x24c);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, entry + 0x48)?;
        c.g[A1][0] = sx(first);
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = lw(c, S0, 0x748);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
            c.g[V1][0] = lw(c, V0, 0xc);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, entry + 0x64)?;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        late = true;
    }
    if late {
        c.cycles += 7;
        c.g[V0][0] = lw(c, S0, 0x748);
        c.g[A1][0] = sx(second);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
        c.g[V1][0] = lw(c, V0, 0xc);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, entry + 0x84)?;
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void MCManager_State0E(self, void* unused)` (0x23f3a0): after the movie
/// finishes, ask the movie object for its sub-state and post event 0x14 or
/// 0xF.
pub fn mc_manager_state0e(c: &mut Cpu) {
    guest(c, |c| mc_manager_state_check(c, 0x0023_f3a0, 0x14, 0xf))
}

/// `void MCManager_State19_Delete(self, void* unused)` (0x240768): the delete
/// step's version of the same movie check (events 0x1C and 0x1A).
pub fn mc_manager_state19_delete(c: &mut Cpu) {
    guest(c, |c| mc_manager_state_check(c, 0x0024_0768, 0x1c, 0x1a))
}

/// `void cFEPopupConfirm_SetTextFromDesc(cUIText* text, void* unused,
/// const int* desc)` (0x1c8050): set the popup text from the description
/// record (0 = default string, 1 = rich text, 2 = ASCII, 3 = string id).
pub fn cfe_popup_confirm_set_text_from_desc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        frame.save_ra(c, 0);
        c.g[V0][0] = 1;
        let kind = c.g[V1][0];
        c.g[A0][0] = c.g[A1][0];
        // 0,1,2,3 = the switch cases, 4 = no matching case.
        let mut case = 4u8;
        if kind == c.g[V0][0] {
            case = 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (kind < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let is_two = kind == c.g[V0][0];
                c.g[V0][0] = 3;
                if is_two {
                    case = 2;
                } else {
                    c.cycles += 2;
                    if kind == c.g[V0][0] {
                        case = 3;
                    } else {
                        c.cycles += 2;
                    }
                }
            } else {
                c.cycles += 2;
                if kind == 0 {
                    case = 0;
                } else {
                    c.cycles += 2;
                }
            }
        }
        match case {
            1 => {
                c.cycles += 4;
                c.g[A1][0] = lw(c, A2, 0x4);
                call_ptr(c, table_callee::UI_TEXT_SET_RICH_STRING, 0x001c_80a4)?;
            }
            2 => {
                c.cycles += 4;
                c.g[A1][0] = lw(c, A2, 0x4);
                call_ptr(c, table_callee::UI_TEXT_SET_ASCII_STRING, 0x001c_80b4)?;
            }
            3 => {
                c.cycles += 4;
                c.g[A1][0] = lw(c, A2, 0x4);
                call_ptr(
                    c,
                    table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID,
                    0x001c_80c4,
                )?;
            }
            0 => {
                c.cycles += 4;
                c.g[A1][0] = 0x4a0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                call_ptr(c, table_callee::UI_TEXT_SET_ASCII_STRING, 0x001c_80d8)?;
            }
            _ => {}
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Shared body of the two `cUIStateBuyPopup_Set*Params` helpers: fill the five
/// hash/parameter slots of the popup state.
fn ui_state_buy_popup_set_params(
    c: &mut Cpu,
    entry: u32,
    first_hash: u32,
    second_hash: u32,
) -> Flow {
    c.cycles += 37;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, 0x40);
    frame.save(c, S3, 0x20);
    c.g[A0][0] = 0x460000;
    frame.save(c, S4, 0x10);
    c.g[S1][0] = c.g[A1][0];
    c.g[S4][0] = c.g[A2][0];
    c.g[S3][0] = c.g[A3][0];
    frame.save(c, S2, 0x30);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(first_hash));
    frame.save_ra(c, 0);
    c.g[S2][0] = 3;
    call_ptr(c, table_callee::GET_HASH_VALUE32, entry + 0x3c)?;
    Addr(c.g[S0][0] as u32).off(0x10).sw(c, c.g[S1][0]);
    c.g[V1][0] = 2;
    Addr(c.g[S0][0] as u32).off(0x14).sw(c, c.g[S2][0]);
    c.g[A0][0] = 0x460000;
    Addr(c.g[S0][0] as u32).off(0xc).sw(c, c.g[V1][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(second_hash));
    Addr(c.g[S0][0] as u32).off(0x18).sw(c, c.g[V0][0]);
    call_ptr(c, table_callee::GET_HASH_VALUE32, entry + 0x5c)?;
    c.g[V1][0] = u64::MAX;
    Addr(c.g[S0][0] as u32).off(0x4).sw(c, c.g[S3][0]);
    Addr(c.g[S0][0] as u32).off(0x1c).sw(c, c.g[S2][0]);
    Addr(c.g[S0][0] as u32).sw(c, c.g[S4][0]);
    Addr(c.g[S0][0] as u32).off(0x20).sw(c, c.g[V0][0]);
    Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V1][0]);
    frame.restore(c, S0, 0x50);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cUIStateBuyPopup_SetMusicTrackParams(self, a1, a2, a3)` (0x1ca980).
pub fn cuistate_buy_popup_set_music_track_params(c: &mut Cpu) {
    guest(c, |c| {
        ui_state_buy_popup_set_params(c, 0x001c_a980, 0x61b8, 0x61e0)
    })
}

/// `void cUIStateBuyPopup_SetItemParams(self, a1, a2, a3)` (0x1cab10).
pub fn cuistate_buy_popup_set_item_params(c: &mut Cpu) {
    guest(c, |c| {
        ui_state_buy_popup_set_params(c, 0x001c_ab10, 0x62b8, 0x61f8)
    })
}

/// `sWSItem* PS2_SONY_CODEC_INTERNAL_GetPicture(sWSPool* self, ...)`
/// (0x3b0fb8): pop the first free frame, decode into the work buffer and bump
/// the frame counter.
pub fn ps2_sony_codec_internal_get_picture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CODEC_POP_FREE_FRAME.call(c, 0x003b_0fd0)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = lw(c, S1, 0x4);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0xc).lw(c);
        c.g[V0][0] = c.g[V0][0] & 0x1000;
        if c.g[V0][0] == 0 {
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 2;
            c.g[V0][0] = lw(c, A1, 0x10);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
        c.cycles += 23;
        c.g[A0][0] = lw(c, S0, 0x8);
        c.g[A3][0] = u64::MAX;
        c.g[A2][0] = lw(c, S0, 0xc);
        c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf));
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A0][0];
        }
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
        c.mult(0, 2, 2, 6);
        c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf));
        if c.g[A3][0] != 0 {
            c.g[A2][0] = c.g[V0][0];
        }
        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 4) as u32);
        call_ptr(c, table_callee::CODEC_DECODE_FRAME, 0x003b_1028)?;
        c.g[V1][0] = lw(c, S0, 0x10);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S1, 0x10);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        frame.restore_ra(c, 0);
        Addr(c.g[S0][0] as u32).off(0x10).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void MPEG2_sequence_display_extension(void)` (0x3b1cc0): read the display
/// extension bit fields into the decoder's parameter block.
pub fn mpeg2_sequence_display_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 3;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1cd0)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 1;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ac8)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1ce0)?;
        c.g[V1][0] = 0x510000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ac4)).sw(c, c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 12;
            c.g[A0][0] = 8;
            callee::MPEG2_GET_BITS.call(c, 0x003b_1cf4)?;
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 8;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6ac0)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1d04)?;
            c.g[V1][0] = 0x510000;
            c.g[A0][0] = 8;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6abc)).sw(c, c.g[V0][0]);
            callee::MPEG2_GET_BITS.call(c, 0x003b_1d14)?;
            c.g[V1][0] = 0x510000;
            Addr((c.g[V1][0] as u32).wrapping_sub(0x6ab8)).sw(c, c.g[V0][0]);
        }
        c.cycles += 14;
        c.g[A0][0] = 0xe;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1d24)?;
        c.g[V1][0] = 0x510000;
        c.g[A0][0] = 0x490000;
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ab4)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x54d0));
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_1d38)?;
        c.g[A0][0] = 0xe;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1d40)?;
        c.g[V1][0] = 0x510000;
        frame.restore_ra(c, 0);
        Addr((c.g[V1][0] as u32).wrapping_sub(0x6ab0)).sw(c, c.g[V0][0]);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Shared body of the six `cFEStateRewardGallery*_InitButtons` helpers: gather
/// the page's entry count and pointer table into the button list.
fn reward_gallery_init_buttons(c: &mut Cpu, entry: u32, getter: u32, base_off: u32) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, table_callee::BE_GET, entry + 0x18)?;
    c.g[A1][0] = 0xd;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, table_callee::BE_GET_INTERFACE_FV, entry + 0x24)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, getter, entry + 0x2c)?;
    Addr(c.g[S0][0] as u32).off(0x68).sw(c, 0);
    c.g[S1][0] = c.g[V0][0];
    call_ptr(c, table_callee::REWARD_GALLERY_GET_BASE, entry + 0x38)?;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(base_off).lw(c);
    c.g[A3][0] = 0;
    Addr(c.g[S0][0] as u32).off(0xa24).sw(c, c.g[V1][0]);
    if (c.g[S1][0] as i64) > 0 {
        c.cycles += 2;
        c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa28));
        let head = entry + 0x50;
        loop {
            c.cycles += 12;
            c.g[A0][0] = lw(c, S0, 0x68);
            c.g[A1][0] = sx((c.g[A3][0] as u32) << 4);
            c.g[V1][0] = lw(c, S0, 0xa24);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A2][0] = ((c.g[A3][0] as i64) < (c.g[S1][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            let more = c.g[A2][0] != 0;
            Addr(c.g[S0][0] as u32).off(0x68).sw(c, c.g[A0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cFEStateRewardGalleryPosters_InitButtons(self)` (0x1d11f8).
pub fn cfe_state_reward_gallery_posters_init_buttons(c: &mut Cpu) {
    guest(c, |c| {
        reward_gallery_init_buttons(c, 0x001d_11f8, table_callee::REWARD_GALLERY_POSTERS, 0x14)
    })
}

/// `void cFEStateRewardGalleryTradingCards_InitButtons(self)` (0x1d1390).
pub fn cfe_state_reward_gallery_trading_cards_init_buttons(c: &mut Cpu) {
    guest(c, |c| {
        reward_gallery_init_buttons(
            c,
            0x001d_1390,
            table_callee::REWARD_GALLERY_TRADING_CARDS,
            0x18,
        )
    })
}

/// `void cFEStateRewardGalleryConceptArts_InitButtons(self)` (0x1d1528).
pub fn cfe_state_reward_gallery_concept_arts_init_buttons(c: &mut Cpu) {
    guest(c, |c| {
        reward_gallery_init_buttons(
            c,
            0x001d_1528,
            table_callee::REWARD_GALLERY_CONCEPT_ARTS,
            0x1c,
        )
    })
}

/// `void cFEStateRewardGalleryVideos_InitButtons(self)` (0x1d16c0).
pub fn cfe_state_reward_gallery_videos_init_buttons(c: &mut Cpu) {
    guest(c, |c| {
        reward_gallery_init_buttons(c, 0x001d_16c0, table_callee::REWARD_GALLERY_VIDEOS, 0x24)
    })
}

/// `void cFEStateRewardGalleryToys_InitButtons(self)` (0x1d1858).
pub fn cfe_state_reward_gallery_toys_init_buttons(c: &mut Cpu) {
    guest(c, |c| {
        reward_gallery_init_buttons(c, 0x001d_1858, table_callee::REWARD_GALLERY_TOYS, 0x20)
    })
}

/// `void cFEStateRewardGalleryCheatChars_InitButtons(self)` (0x1d1990).
pub fn cfe_state_reward_gallery_cheat_chars_init_buttons(c: &mut Cpu) {
    guest(c, |c| {
        reward_gallery_init_buttons(
            c,
            0x001d_1990,
            table_callee::REWARD_GALLERY_CHEAT_CHARS,
            0x28,
        )
    })
}

/// `void cFEStateCharEquip_OpenBuyPopup(self, item)` (0x19c7e8): allocate a buy
/// popup, fill it from the item's price/id and post it to the screen.
pub fn cfe_state_char_equip_open_buy_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x30);
        c.g[A3][0] = 0;
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x460000;
        frame.save(c, S2, 0x10);
        c.g[A0][0] = 0x70;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x790));
        call_ptr(c, table_callee::EA_MEM_ALLOC_NAMED, 0x0019_c81c)?;
        c.g[A1][0] = lw(c, S1, 0x10);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = Addr(c.g[S1][0] as u32).off(0x44).lb(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, table_callee::BUY_POPUP_CTOR, 0x0019_c830)?;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0xe).lh(c);
        c.g[S2][0] = c.g[V0][0];
        let no_price = (c.g[A2][0] as i64) <= 0;
        c.g[A1][0] = lw(c, S0, 0x14);
        if !no_price {
            c.cycles += 2;
            c.g[V0][0] = 0xa;
            c.mult(0, 6, 6, 2);
        }
        c.cycles += 13;
        c.g[A3][0] = lw(c, S1, 0xc0);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x48));
        callee::BUY_POPUP_INIT_BUY_BOLT.call(c, 0x0019_c854)?;
        c.g[A0][0] = lw(c, S1, 0x10);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, table_callee::UI_OBJECT_PUSH, 0x0019_c864)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateRace_Enter(self)` (0x234e20): put the AI into state 5, reset
/// the race manager unless the game was already in mode 4, then open the
/// finish screen if the rider count looks wrong.
pub fn game_state_race_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp(c, -0x488)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 5;
        c.g[A0][0] = lw(c, V0, 0xc);
        call_ptr(c, table_callee::AI_SET_AI_STATE, 0x0023_4e38)?;
        call_ptr(c, table_callee::BE_GET, 0x0023_4e40)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, table_callee::BE_GET_INTERFACE_FV, 0x0023_4e4c)?;
        c.g[V1][0] = 0x530000;
        c.g[A0][0] = 4;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c10)).lb(c);
        let mode_four = c.g[V0][0] == c.g[A0][0];
        c.g[A1][0] = Addr(gp(c, -0x488)).lw(c);
        if !mode_four {
            c.cycles += 5;
            c.g[A0][0] = 1;
            c.g[V0][0] = lw(c, A1, 0xc);
            Addr(c.g[V0][0] as u32).off(0x14).sw(c, c.g[A0][0]);
            c.g[V1][0] = lw(c, A1, 0xc);
            Addr(c.g[V1][0] as u32).off(0x8c).sw(c, 0);
        }
        c.cycles += 7;
        c.g[A0][0] = 1;
        call_ptr(c, table_callee::BE_SET_STATE, 0x0023_4e7c)?;
        c.g[V1][0] = Addr(gp(c, -0x488)).lw(c);
        c.g[V0][0] = lw(c, V1, 0x28);
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[A0][0] = ((c.g[V0][0] as i64) < 10) as u64;
        }
        c.cycles += 2;
        let ok = c.g[A0][0] != 0;
        frame.restore_ra(c, 0);
        if !ok {
            c.cycles += 4;
            c.g[A0][0] = lw(c, V1, 0x48);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, table_callee::UI_OPEN_FINISH_SCREEN, 0x0023_4ea8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int Rider_EstimateFinalScore(Rider* r)` (0x122e50): extrapolate the final
/// score from the distance still to go and the time so far.
pub fn rider_estimate_final_score(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = lw(c, S0, 0x790);
        c.g[V0][0] = lw(c, V1, 0x198);
        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        callee::RIDER_GET_START_DIST_TO_FINISH.call(c, 0x0012_2e74)?;
        c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x4d0));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.sub_s(2, 0, 3);
        c.c_lt_s(2, 1);
        if c.fcc() {
            c.mov_s(2, 1);
        }
        c.cycles += 11;
        c.g[AT][0] = 0x447a_0000;
        c.mtc1(AT, 0);
        c.mtc1(S1, 1);
        c.cvt_s_w(1, 1);
        c.mtc1(0, 4);
        c.sub_s(0, 3, 0);
        c.div_s(1, 1, 2);
        c.c_lt_s(0, 4);
        if c.fcc() {
            c.mov_s(0, 4);
        }
        c.cycles += 9;
        c.mul_s(0, 0, 1);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        c.cvt_w_s(1, 0);
        c.mfc1(2, 1);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore(c, S1, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOptionsDeviceSelect_v33(self)` (0x18e478): open the device
/// select popup once, with the mode selected from the four option flags.
pub fn cfe_state_options_device_select_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, S0, 0x1ac);
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !done {
            c.cycles += 21;
            c.g[V1][0] = lw(c, S0, 0x10);
            c.g[T2][0] = 2;
            c.g[A2][0] = lw(c, S0, 0x1bc);
            c.g[A1][0] = c.g[S0][0];
            c.g[T1][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[V0][0] = lw(c, S0, 0x1e4);
            c.g[A2][0] = c.g[A2][0] ^ 1;
            c.g[T0][0] = lw(c, T1, 0x4);
            c.g[A2][0] = (c.g[A2][0] < 1) as u64;
            c.g[V0][0] = c.g[V0][0] ^ 1;
            c.g[V1][0] = lw(c, S0, 0x1b4);
            if c.g[V0][0] == 0 {
                c.g[A2][0] = 0;
            }
            c.g[A3][0] = lw(c, S0, 0x230);
            c.g[A0][0] = Addr(c.g[T0][0] as u32).off(0x28).lh(c);
            if c.g[V1][0] == 0 {
                c.g[A2][0] = 0;
            }
            c.g[V0][0] = lw(c, T0, 0x2c);
            if c.g[A3][0] != 0 {
                c.g[A2][0] = c.g[T2][0];
            }
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0018_e4e0)?;
            let created = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !created {
                c.cycles += 3;
                c.g[A0][0] = lw(c, S0, 0x10);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, table_callee::UI_SCREEN_ADD_OBJECT, 0x0018_e4f4)?;
            }
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        Addr(c.g[S0][0] as u32).off(0x1ac).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of the `OVScreen*_v38` visibility helpers: hide/show two named
/// UI objects and record the flag at +0x220. `entry` is needed for the return
/// addresses the generated code uses.
fn ov_screen_set_two_visible(c: &mut Cpu, entry: u32, first_string: u32) -> Flow {
    c.cycles += 37;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x20);
    c.g[A0][0] = 0x470000;
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A1][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(first_string));
    call_ptr(c, table_callee::GET_HASH_VALUE32, entry + 0x24)?;
    c.g[S0][0] = c.g[S0][0] ^ 1;
    c.g[A0][0] = lw(c, S1, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, table_callee::UI_SCREEN_FIND_OBJECT, entry + 0x34)?;
    c.g[V1][0] = lw(c, V0, 0x8);
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
    c.g[A2][0] = lw(c, V1, 0x4c);
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, c.g[A2][0] as u32, entry + 0x4c)?;
    c.g[A0][0] = 0x4a0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20a0));
    call_ptr(c, table_callee::GET_HASH_VALUE32, entry + 0x58)?;
    c.g[A0][0] = lw(c, S1, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, table_callee::UI_SCREEN_FIND_OBJECT, entry + 0x64)?;
    c.g[V1][0] = lw(c, V0, 0x8);
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
    c.g[A2][0] = lw(c, V1, 0x4c);
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, c.g[A2][0] as u32, entry + 0x7c)?;
    Addr(c.g[S1][0] as u32).off(0x220).sw(c, c.g[S0][0]);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void OVScreenSaveReplay_v38(self, on)` (0x1fe5b0).
pub fn ov_screen_save_replay_v38(c: &mut Cpu) {
    guest(c, |c| ov_screen_set_two_visible(c, 0x001f_e5b0, 0x188))
}

/// `void OVScreenSaveRecords_v38(self, on)` (0x1ff278).
pub fn ov_screen_save_records_v38(c: &mut Cpu) {
    guest(c, |c| ov_screen_set_two_visible(c, 0x001f_f278, 0x188))
}

/// `void OVScreenProfileSavePDA_v38(self, on)` (0x211970).
pub fn ov_screen_profile_save_pda_v38(c: &mut Cpu) {
    guest(c, |c| ov_screen_set_two_visible(c, 0x0021_1970, 0x1db0))
}

/// `void OVScreenInGameError_UpdateServerTimeoutText(self)` (0x1fb180): update
/// the timeout text and its object's countdown value.
pub fn ov_screen_in_game_error_update_server_timeout_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, S1, 0x40);
        if c.g[V0][0] != 0 {
            // The text object (S0) is looked up by hash; the hash of the
            // unloaded string is 0 on the test machine.
            c.cycles += 9;
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e8));
            call_ptr(c, table_callee::GET_HASH_VALUE32, 0x001f_b1ac)?;
            c.g[A0][0] = lw(c, S1, 0x40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, table_callee::UI_SCREEN_FIND_OBJECT, 0x001f_b1b8)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                c.g[V0][0] = lw(c, S0, 0x8);
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, S1, 0xa0);
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = lw(c, S0, 0x8);
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4a0));
                    call_ptr(c, table_callee::GET_HASH_VALUE32, 0x001f_b1dc)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(
                        c,
                        table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID,
                        0x001f_b1e8,
                    )?;
                    c.g[V0][0] = lw(c, S0, 0x8);
                }
            }
            c.cycles += 6;
            c.g[A1][0] = lw(c, S1, 0xa0);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = lw(c, V0, 0x4c);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001f_b200)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_RunProcess(self, arg, value, unused)` (0x2822a0): run a
/// script process: store the 16-bit value and, if the process is armed, play
/// its sequence.
pub fn script_engine_run_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        callee::SCRIPT_ENGINE_GET_PROCESS.call(c, 0x0028_22c8)?;
        c.g[S0][0] = c.g[V0][0];
        let mut run = false;
        if c.g[S0][0] != 0 {
            c.g[V1][0] = lw(c, S0, 0x4);
            run = true;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        if run {
            c.cycles += 3;
            c.g[V0][0] = 2;
            let wrong_state = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[V1][0];
            if !wrong_state {
                c.cycles += 4;
                c.g[V1][0] = lw(c, S0, 0x20);
                c.g[V0][0] = 1;
                let not_armed = c.g[V1][0] != c.g[V0][0];
                Addr(c.g[S0][0] as u32).off(0x10).sh(c, c.g[S1][0]);
                if !not_armed {
                    c.cycles += 7;
                    c.g[V0][0] = 3;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                    Addr(c.g[S0][0] as u32).off(0x4).sw(c, c.g[V0][0]);
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A0][0] = lw(c, S2, 0x2b8);
                    c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
                    callee::IN_SEQ_PLAY.call(c, 0x0028_2314)?;
                }
                c.cycles += 1;
                c.g[V0][0] = lw(c, S0, 0x4);
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

/// `bool BENewPlayerInt_IsCharProfileFlagClear(void* be, int index, int bit)`
/// (0x145d38): whether the character-profile flag `bit` is clear.
pub fn benew_player_int_is_char_profile_flag_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_5d5c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        callee::BENEW_PLAYER_INT_GET_SLOT.call(c, 0x0014_5d68)?;
        c.g[V1][0] = 0x1c;
        c.g[A2][0] = 0x530000;
        c.mult(0, V0, V0, V1);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4fe0));
        c.g[V1][0] = 0x9b50;
        c.g[A3][0] = 0xf88;
        c.mult(0, S0, S0, V1);
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
        c.g[A1][0] = 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32) << ((c.g[S2][0] as u32) & 31));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x11).lb(c);
        frame.restore(c, S1, 0x20);
        c.mult(0, V1, V1, A3);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xacc).lw(c);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
        c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdMan3d_dtor(self, flags)` (0x343b28): release the five group
/// slots (twice, as the original does) and delete the object.
pub fn crowd_man3d_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[S1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x9c4));
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            callee::CROWD_MAN_GROUP_FREE.call(c, 0x0034_3b58)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_3b50);
            }
        }
        c.cycles += 3;
        c.g[S2][0] = c.g[S3][0] & 1;
        let no_elems = c.g[S1][0] == 0;
        Addr(gp(c, 0xf00)).sw(c, 0);
        if !no_elems {
            c.cycles += 3;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x9c4));
            let empty = c.g[S1][0] == c.g[S0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1f4));
            if !empty {
                loop {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::CROWD_MAN_GROUP_FREE.call(c, 0x0034_3b88)?;
                    let more = c.g[S1][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1f4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_3b80);
                    }
                }
            }
        }
        c.cycles += 2;
        let no_delete = c.g[S2][0] == 0;
        frame.restore(c, S0, 0x40);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table_callee::OPERATOR_DELETE, 0x0034_3ba0)?;
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

/// `cFEStateRewardsRoom* cFEStateRewardsRoom_ctor(self, idx, char, a3)`
/// (0x1d4bc8): build the rewards-room state, remember the player's character
/// id and return the object.
pub fn cfe_state_rewards_room_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, table_callee::FE_STATE_CTOR, 0x001d_4bf0)?;
        c.g[V0][0] = 0x2e;
        c.g[V1][0] = 0x470000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6a18));
        Addr(c.g[S0][0] as u32).off(0xc).sw(c, c.g[V0][0]);
        Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        Addr(c.g[S0][0] as u32).off(0x44).sb(c, c.g[S1][0]);
        c.g[V0][0] = lw(c, S0, 0x10);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        call_ptr(c, table_callee::BE_GET_TEXT_FROM_ID, 0x001d_4c18)?;
        Addr(c.g[S0][0] as u32).off(0x4c).sw(c, c.g[S2][0]);
        Addr(c.g[S0][0] as u32).off(0x15).sb(c, c.g[V0][0]);
        call_ptr(c, table_callee::BE_GET, 0x001d_4c24)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, table_callee::BE_GET_INTERFACE_FV, 0x001d_4c30)?;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x44).lb(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, table_callee::BENEW_PLAYER_INT_GET_CHAR_ID, 0x001d_4c3c)?;
        Addr(c.g[S0][0] as u32).off(0x48).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXApp_StartFEModule(self)` (0x228238): start the front-end module
/// (the big one from +0x7C or the small one from +0x80).
pub fn ssx_app_start_fe_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        Addr(gp(c, -0x10b4)).sw(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, S0, 0x60);
        let small = c.g[V0][0] == 0;
        c.g[A1][0] = 0x480000;
        if small {
            c.cycles += 12;
            c.g[A0][0] = 0x238;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x57e8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, table_callee::EA_MEM_ALLOC_NAMED, 0x0022_82a4)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, table_callee::FE_MODULE_SMALL_CTOR, 0x0022_82ac)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            Addr(c.g[S0][0] as u32).off(0x80).sw(c, c.g[V0][0]);
            call_ptr(c, table_callee::APP_MAN_SET_NEXT_MODULE, 0x0022_82bc)?;
        } else {
            c.cycles += 14;
            c.g[A0][0] = 0xb0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x57e8));
            c.g[A0][0] = c.g[A0][0] | 0x5ae0;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, table_callee::EA_MEM_ALLOC_NAMED, 0x0022_8270)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, table_callee::FE_MODULE_BIG_CTOR, 0x0022_8278)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            Addr(c.g[S0][0] as u32).off(0x7c).sw(c, c.g[V0][0]);
            call_ptr(c, table_callee::APP_MAN_SET_NEXT_MODULE, 0x0022_8288)?;
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RailMan_dtor(self, flags)` (0x358668): free every live rail object and
/// chain to the base destructor.
pub fn rail_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x490000;
        frame.save(c, S2, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1220));
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
        frame.save(c, S3, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0xffff_ffff_ffff_0000;
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        Addr(c.g[S2][0] as u32).off(0xc).sw(c, c.g[V0][0]);
        c.g[S3][0] = c.g[S3][0] | 0xffff;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            let unused = c.g[V0][0] == c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            if !unused {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                callee::RAIL_MAN_ENABLE_RAIL.call(c, 0x0035_86bc)?;
            }
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 128) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_86a8);
            }
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S4][0];
        Addr(gp(c, 0xf50)).sw(c, 0);
        callee::SYNC_PROCESS_DTOR.call(c, 0x0035_86dc)?;
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

/// `void UIPair_Render(self, x, y)` (0x39c130): forward the pair's render to
/// the two child objects at +0x78 and +0x7C, offset by the pair's own position.
pub fn ui_pair_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 21, 0x28);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x20);
        c.mov_s(21, 12);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw(c, S0, 0x78);
        let none = c.g[A1][0] == 0;
        c.mov_s(20, 13);
        if !none {
            c.cycles += 9;
            c.g[V1][0] = lw(c, A1, 0x8);
            c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x44));
            c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x48));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x88).lh(c);
            c.add_s(12, 21, 12);
            c.add_s(13, 20, 13);
            c.g[V0][0] = lw(c, V1, 0x8c);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0039_c17c)?;
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, S0, 0x7c);
        if c.g[A1][0] != 0 {
            c.cycles += 10;
            c.g[V1][0] = lw(c, A1, 0x8);
            c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x48));
            c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x88).lh(c);
            c.add_s(13, 20, 13);
            c.add_s(12, 21, 12);
            c.g[V0][0] = lw(c, V1, 0x8c);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0039_c1ac)?;
            frame.restore(c, S0, 0x10);
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cUIProgress_Render(self, x, y)` (0x39c300): render the progress pair's
/// two child objects when both are present.
pub fn cui_progress_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        c.g[A1][0] = lw(c, S0, 0x74);
        let mut draw = c.g[A1][0] != 0;
        if draw {
            c.cycles += 3;
            c.g[V0][0] = lw(c, S0, 0x78);
            draw = c.g[V0][0] != 0;
        }
        if draw {
            c.cycles += 20;
            c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x44));
            c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x48));
            c.g[V0][0] = lw(c, A1, 0x8);
            c.add_s(20, 12, 20);
            c.add_s(21, 13, 21);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x88).lh(c);
            c.g[V1][0] = lw(c, V0, 0x8c);
            c.mov_s(12, 20);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.mov_s(13, 21);
            call_ptr(c, c.g[V1][0] as u32, 0x0039_c35c)?;
            c.g[A1][0] = lw(c, S0, 0x78);
            c.mov_s(12, 20);
            c.mov_s(13, 21);
            c.g[V0][0] = lw(c, A1, 0x8);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x88).lh(c);
            c.g[V1][0] = lw(c, V0, 0x8c);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x0039_c37c)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void TexMan_UpdateTextureData(self, index, data, clut, unused, upload)`
/// (0x367d20): upload the texture and/or CLUT pixels and then evict the slot.
pub fn tex_man_update_texture_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
        frame.save(c, S0, 0x40);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = Addr(c.g[SP][0] as u32).off(0x58).lw(c);
        c.g[A1][0] = c.g[A2][0];
        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        let no_upload = c.g[V1][0] == 0;
        c.g[S3][0] = Addr(c.g[SP][0] as u32).off(0x50).lw(c);
        if !no_upload {
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = lw(c, S0, 0x1c);
                c.g[A2][0] = lw(c, S0, 0x10);
                call_ptr(c, table_callee::UPLOAD_TEXTURE_PIXELS, 0x0036_7d70)?;
            }
            c.cycles += 2;
            let no_clut = c.g[S3][0] == 0;
            c.g[A2][0] = 4;
            if !no_clut {
                c.cycles += 5;
                c.g[V0][0] = lw(c, S0, 0x18);
                c.g[A0][0] = lw(c, S0, 0x20);
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[V0][0] as u32) & 31));
                call_ptr(c, table_callee::UPLOAD_TEXTURE_PIXELS, 0x0036_7d8c)?;
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        callee::TEX_MAN_EVICT_TEXTURE.call(c, 0x0036_7d98)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void TexMan_EvictTexture(self, index)` (0x369098): free the slot's VRAM
/// blocks and clear the three cached slot indices.
pub fn tex_man_evict_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x28).lw(c);
        let unused = c.g[V1][0] == c.g[A0][0];
        c.g[V1][0] = 1;
        if !unused {
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            c.g[V0][0] = 9;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4350));
                callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS.call(c, 0x0036_90ec)?;
                c.g[A0][0] = u64::MAX;
            } else {
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f60));
                callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS.call(c, 0x0036_90dc)?;
                c.g[A0][0] = u64::MAX;
            }
            c.cycles += 1;
            c.g[V1][0] = 1;
        }
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f50));
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_90f8);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = u64::MAX;
        frame.restore_ra(c, 0);
        Addr(c.g[S0][0] as u32).off(0x1f54).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void GameUI_OpenFinishScreen(void)` (0x20cc60): open the finish screen that
/// matches the current race mode and player count.
pub fn game_ui_open_finish_screen(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, table_callee::BE_GET, 0x0020_cc70)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, table_callee::BE_GET_INTERFACE_FV, 0x0020_cc7c)?;
        c.g[V1][0] = 0x530000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x49).lb(c);
        c.g[V1][0] = Addr(c.g[V1][0] as u32).off(0x48).lb(c);
        let no_mode = c.g[V1][0] == 0;
        c.g[A0][0] = 2;
        let mut stage = 0u8;
        if no_mode {
            stage = 1;
        } else {
            c.cycles += 2;
            let two_players = c.g[V0][0] != c.g[A0][0];
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            stage = if two_players { 2 } else { 1 };
        }
        if stage == 2 {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 3) as u64;
            let one_to_three = c.g[V0][0] != 0;
            c.g[V0][0] = 6;
            if one_to_three {
                stage = 3;
            } else {
                c.cycles += 2;
                let not_six = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 4;
                stage = if not_six { 4 } else { 3 };
            }
        }
        match stage {
            1 => {
                c.cycles += 4;
                c.g[A0][0] = 0xc;
                callee::GAME_UI_OPEN_SCREEN.call(c, 0x0020_cca4)?;
                frame.restore_ra(c, 0);
            }
            3 => {
                c.cycles += 4;
                c.g[A0][0] = 0xd;
                callee::GAME_UI_OPEN_SCREEN.call(c, 0x0020_ccc8)?;
                frame.restore_ra(c, 0);
            }
            _ => {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.cycles += 2;
                    c.g[A0][0] = 0xc;
                    callee::GAME_UI_OPEN_SCREEN.call(c, 0x0020_cce8)?;
                } else {
                    c.cycles += 2;
                    Addr(gp(c, -0x9f4)).sw(c, c.g[A0][0]);
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

/// `void cFEStateMountainRoom_dtor(self, flags)` (0x1d28f8): release the room's
/// music slot and rider, repoint the vtable and run the base destructor.
pub fn cfe_state_mountain_room_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x470000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x67a8));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V0][0]);
        c.g[A2][0] = lw(c, S0, 0xbc);
        let no_slot = (c.g[A2][0] as i64) < 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_slot {
            c.cycles += 7;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[A1][0] = lw(c, S0, 0xb0);
            c.g[A0][0] = lw(c, V0, 0x7c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
            call_ptr(c, table_callee::MUSIC_SLOT_RELEASE, 0x001d_2938)?;
            c.g[V0][0] = u64::MAX;
            Addr(c.g[S0][0] as u32).off(0xbc).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, S0, 0xc4);
        let no_rider = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
        if !no_rider {
            c.cycles += 5;
            c.g[V0][0] = lw(c, A2, 0x10d8);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
            c.g[V1][0] = lw(c, V0, 0x194);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001d_2960)?;
        }
        c.cycles += 11;
        c.g[V0][0] = 0x470000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
        c.g[A1][0] = c.g[S1][0];
        Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V0][0]);
        call_ptr(c, table_callee::UI_STATE_DTOR, 0x001d_2978)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void World_Load(World* self, char* name, u32 flags)` (0x3a6ae0): reset and
/// load the world cache, allocate the script cache from the section count and
/// release the name string.
pub fn world_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, table_callee::BXSTRING_CTOR, 0x003a_6b08)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        callee::WORLD_CACHE_RESET.call(c, 0x003a_6b10)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        callee::WORLD_CACHE_LOAD.call(c, 0x003a_6b20)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        callee::STREAMER_GET_SECTION_COUNT.call(c, 0x003a_6b2c)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
        c.g[A1][0] = c.g[V0][0];
        callee::WSCRIPT_CACHE_ALLOC.call(c, 0x003a_6b38)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        callee::STREAMER_GET_SECTION_COUNT.call(c, 0x003a_6b44)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, table_callee::SECTION_TABLE_CTOR, 0x003a_6b50)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, table_callee::BXSTRING_DTOR, 0x003a_6b5c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
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
    use crate::difftest::{check, check_lohi, Rng, DATA, GAME_GP, SP};

    /// A no-op guest function (`jr ra`): an empty vtable slot.
    const EMPTY: u32 = 0x0038_dc60;
    /// `V0 = *(a0 + 0xb0); jr ra` — a vtable slot whose result the test
    /// controls through memory.
    const LOAD_B0: u32 = 0x0027_72b8;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    fn w16(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }
    fn w8(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    /// A `{ short delta; short; void (*fn)(); }` slot at `vt + off`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16, func: u32) {
        w16(c, vt + off, delta as u16);
        w(c, vt + off + 4, func);
    }

    /// The BELibrary singleton (`gp - 0x1f04`, a zeroed dummy so nothing reads
    /// the disc) and the world (`gp - 0x848`, with no riders).
    fn globals(c: &mut Cpu) {
        let be = DATA + 0x30_0000;
        let world = DATA + 0x8_0000;
        w(c, GAME_GP.wrapping_sub(0x1f04), be);
        w(c, GAME_GP.wrapping_sub(0x848), world);
        let game = world + 0x1000;
        w(c, world + 0x84, game);
        let race = world + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// The engine's size-class pool allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class (a zeroed heap walks garbage).
    fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let p = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, p + head, base);
            w(c, GAME_GP + slot, p);
        }
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena, for paths
    /// that allocate beyond the size-class pools.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u64 = 0x0012_3450;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x20_0000;
        c.g[RA][0] = RET;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET as u32, "DLHeap_InitArena did not return");
    }

    /// Scrambled state, fake call hooks, the game `gp`, a working heap and the
    /// standard fixtures.
    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        c.g[GP][0] = GAME_GP as u64;
        heap_init(c);
        let r = machine(c, seed, salt, calls);
        globals(c);
        pool(c);
        r
    }

    /// A screen whose object list holds `node` with hash 0 (`StrHash("")` is 0
    /// on the test machine: the tag strings are not loaded).
    fn hash_object(c: &mut Cpu, screen: u32, node: u32) {
        w(c, screen + 0x38, DATA + 0x32_0100);
        w(c, DATA + 0x32_0100, 1);
        w(c, screen + 0x3c, DATA + 0x32_0000);
        w(c, DATA + 0x32_0000, node);
        w(c, node + 0x38, 0);
    }

    // ------------------------------------------------------------------
    // Front-end / online state handlers.

    #[test]
    fn cfe_state_select_persona_on_save_account_popup_event_matches() {
        check(
            ee::s03::f_001ad930__generated,
            cfe_state_select_persona_on_save_account_popup_event,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1ad930, 3);
                let obj = DATA + 0x1000;
                let child = DATA + 0x2000;
                let screen = DATA + 0x3000;
                let holder = DATA + 0x4000;
                let list = DATA + 0x5000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = child as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    (seed % 0x20) as u64
                };
                w(c, obj + 0x10, holder);
                w(c, obj + 0x40, screen);
                w(c, obj + 0x69c, 0);
                w(c, screen + 0xd0, holder);
                w(c, holder + 0x10, list);
                w(c, child + 0x18, r.next_u32() % 0x900);
            },
        );
    }

    #[test]
    fn cfe_state_online_character_select_init_matches() {
        check(
            ee::s03::f_001b0fb0__generated,
            cfe_state_online_character_select_init,
            30,
            &|c, seed| {
                let mut r = base(c, seed, 0x1b0fb0, 3);
                let obj = DATA + 0x1000;
                let engine = DATA + 0x2000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x10, if seed % 3 == 0 { 0 } else { engine });
                w(c, obj + 0x6e0, r.next_u32());
                w(c, engine + 0x4, engine + 0x100);
                w(c, engine + 0x10c, 0);
            },
        );
    }

            #[test]
    fn cfe_state_create_step1_transition_in_matches() {
        check(
            ee::s03::f_001b8a08__generated,
            cfe_state_create_step1_transition_in,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1b8a08, 3);
                let obj = DATA + 0x1000;
                let popup = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                w(c, obj + 0x6f0, if seed % 3 == 0 { 0 } else { popup });
                w(c, popup + 0x8, vt);
                slot(c, vt, 0x40, 0, FAKE);
                w(c, obj + 0x10, DATA + 0x4000);
                w(c, obj + 0x40, DATA + 0x5000);
                w8(c, obj + 0x15, r.next_u32() as u8);
            },
        );
    }

        // ------------------------------------------------------------------
    // UI / lobby.

    #[test]
    fn ov_screen_message_center_on_activate_matches() {
        check(
            ee::s04::f_001e4168__generated,
            ov_screen_message_center_on_activate,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1e4168, 3);
                let obj = DATA + 0x1000;
                let menu = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x18, (seed % 4) as u32);
                w(c, obj + 0x144, if seed % 7 == 0 { 0 } else { menu });
                w(c, menu + 0x8, vt);
                slot(c, vt, 0x38, 4, FAKE);
                w(c, obj + 0xa0, r.next_u32() % 8);
            },
        );
    }

            #[test]
    fn buddy_man_is_same_product_at_matches() {
        check(
            ee::s07::f_002633f8__generated,
            buddy_man_is_same_product_at,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x2633f8, 3);
                let obj = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = match seed % 3 {
                    0 => (-1i64) as u64,
                    1 => 0x1000,
                    _ => (seed % 8) as u64,
                };
                w(c, obj + 0xec, DATA + 0x2000);
                w(c, DATA + 0x2000 + 0xb0, 0);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn ssx_audio_system_start_rider_board_loops_matches() {
        check(
            ee::s07::f_00292a50__generated,
            ssx_audio_system_start_rider_board_loops,
            16,
            &|c, seed| {
                let mut r = base(c, seed, 0x292a50, 4);
                let sys = DATA + 0x1000;
                let rider = DATA + 0x2000;
                c.g[A0][0] = sys as u64;
                c.g[A1][0] = rider as u64;
                w(c, rider + 0x760, r.next_u32());
                w(c, rider + 0x764, r.next_u32());
                w(c, rider + 0x768, r.next_u32());
                w(c, rider + 0x76c, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_mission_abort_matches() {
        check(
            ee::s08::f_0029dbb0__generated,
            ssx_audio_system_on_mission_abort,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x29dbb0, 4);
                let sys = DATA + 0x1000;
                let vt = DATA + 0x2_0000;
                c.g[A0][0] = sys as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                w(c, sys + 0x5fd4, if seed % 3 == 0 { 0 } else { 1 });
                w(c, sys + 0x5fd0, r.next_u32());
                w(c, sys + 0x5558, vt);
                slot(c, vt, 0x20, 0, FAKE);
            },
        );
    }

    #[test]
    fn ini_parser_ctor_matches() {
        check(ee::s08::f_002a77f8__generated, ini_parser_ctor, 40, &|c, seed| {
            let mut r = base(c, seed, 0x2a77f8, 3);
            let obj = DATA + 0x1000;
            let text = DATA + 0x6000;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = text as u64;
            c.g[A2][0] = r.next_u32() as u64;
            let n = if seed % 3 == 0 { 0 } else { 1 + seed % 40 } as u32;
            for i in 0..n {
                w8(c, text + i, b'a' + (seed as u8 % 26));
            }
            w8(c, text + n, 0);
        });
    }

    #[test]
    fn script_obj_cam_ctor_matches() {
        check(ee::s07::f_0027d5e8__generated, script_obj_cam_ctor, 40, &|c, seed| {
            let mut r = base(c, seed, 0x27d5e8, 2);
            let obj = DATA + 0x1000;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = r.next_u32() as u64;
            for i in 0..4u32 {
                w(c, 0x004f_f1a0 + 0x10 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn script_obj_cam_deactivate_matches() {
        check(
            ee::s07::f_0027d8d8__generated,
            script_obj_cam_deactivate,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x27d8d8, 3);
                let obj = DATA + 0x1000;
                let world = DATA + 0x8_0000;
                let game = world + 0x1000;
                let riders = world + 0x3000;
                let rider = DATA + 0x5000;
                let scr = DATA + 0x6000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x7c, 0);
                w(c, game + 0x84, if seed % 4 == 0 { 0 } else { riders });
                // riders[1] is the rider `idx = 0` selects.
                w(c, riders + 4, rider);
                w(c, rider + 0xac, DATA + 0x7000);
                let sc = scrub(c, scr);
                w(c, rider + 0xa0, sc);
                w(c, DATA + 0x7000 + 0x18, if seed % 2 == 0 { 0 } else { 1 });
                let _ = r.next_u32();
            },
        );
    }

    /// A zeroed script-object dummy with the vtable slot 4 used by
    /// `ScriptObjCam_Deactivate`.
    fn scrub(c: &mut Cpu, at: u32) -> u32 {
        let vt = at + 0x1000;
        w(c, at + 0x14, vt);
        slot(c, vt, 0x20, 0, FAKE);
        at
    }

        #[test]
    fn mc_manager_state0e_matches() {
        check(ee::s06::f_0023f3a0__generated, mc_manager_state0e, 60, &|c, seed| {
            let mut r = base(c, seed, 0x23f3a0, 4);
            let obj = DATA + 0x1000;
            let movie = DATA + 0x2000;
            let vt = DATA + 0x2_0000;
            let ui = DATA + 0x3000;
            let uivt = DATA + 0x3_0000;
            c.g[A0][0] = obj as u64;
            w(c, obj + 0x434, movie);
            w(c, movie, vt);
            slot(c, vt, 0x58, 0, LOAD_B0);
            slot(c, vt, 0x248, 0x40, LOAD_B0);
            w(c, movie + 0xb0, (seed % 2) as u32);
            w(c, movie + 0xf0, if seed % 3 == 0 { 0 } else { 1 });
            w(c, obj + 0x748, ui);
            w(c, ui, uivt);
            slot(c, uivt, 0x8, 0, FAKE);
            w(c, ui + 0xb0, r.next_u32());
        });
    }

    #[test]
    fn mc_manager_state19_delete_matches() {
        check(
            ee::s06::f_00240768__generated,
            mc_manager_state19_delete,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x240768, 4);
                let obj = DATA + 0x1000;
                let movie = DATA + 0x2000;
                let vt = DATA + 0x2_0000;
                let ui = DATA + 0x3000;
                let uivt = DATA + 0x3_0000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x434, movie);
                w(c, movie, vt);
                slot(c, vt, 0x58, 0, LOAD_B0);
                slot(c, vt, 0x248, 0x40, LOAD_B0);
                w(c, movie + 0xb0, (seed % 2) as u32);
                w(c, movie + 0xf0, if seed % 3 == 0 { 0 } else { 1 });
                w(c, obj + 0x748, ui);
                w(c, ui, uivt);
                slot(c, uivt, 0x8, 0, FAKE);
                w(c, ui + 0xb0, r.next_u32());
            },
        );
    }

    #[test]
    fn cfe_popup_confirm_set_text_from_desc_matches() {
        check(
            ee::s03::f_001c8050__generated,
            cfe_popup_confirm_set_text_from_desc,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1c8050, 4);
                let text = DATA + 0x1000;
                let desc = DATA + 0x2000;
                c.g[A0][0] = text as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = desc as u64;
                w(
                    c,
                    desc,
                    match seed % 5 {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        3 => 3,
                        _ => r.next_u32() % 8,
                    },
                );
                w(c, desc + 0x4, r.next_u32());
            },
        );
    }

    #[test]
    fn cuistate_buy_popup_set_music_track_params_matches() {
        check(
            ee::s04::f_001ca980__generated,
            cuistate_buy_popup_set_music_track_params,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1ca980, 2);
                let obj = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn cuistate_buy_popup_set_item_params_matches() {
        check(
            ee::s04::f_001cab10__generated,
            cuistate_buy_popup_set_item_params,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1cab10, 2);
                let obj = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ps2_sony_codec_internal_get_picture_matches() {
        check_lohi(
            ee::s13::f_003b0fb8__generated,
            ps2_sony_codec_internal_get_picture,
            30,
            &|c, seed| {
                let mut r = base(c, seed, 0x3b0fb8, 3);
                let pool = DATA + 0x1000;
                c.g[A0][0] = pool as u64;
                w(
                    c,
                    pool + 0x8,
                    if seed % 3 == 0 {
                        0
                    } else {
                        16 * (1 + seed % 4)
                    },
                );
                w(
                    c,
                    pool + 0xc,
                    if seed % 3 == 0 {
                        0
                    } else {
                        16 * (1 + seed % 4)
                    },
                );
                w(c, pool + 0x10, r.next_u32());
                let _ = r.next_u32();
            },
        );
    }

        /// `BERewardInt_GetRewardsFile()` returns this global block; the page
    /// getters read `count` at +0x3C and the pointer table at `base_off`.
    const REWARDS_FILE: u32 = 0x004c_3e98;

    macro_rules! gallery_test {
        ($name:ident, $addr:expr, $func:expr, $salt:expr, $base_off:expr) => {
            #[test]
            fn $name() {
                check($addr, $func, 40, &|c, seed| {
                    let mut r = base(c, seed, $salt, 1);
                    let obj = DATA + 0x1000;
                    let table = DATA + 0x2_0000;
                    c.g[A0][0] = obj as u64;
                    // -1, 0 and 1..4 entries: skip and loop paths.
                    w(c, REWARDS_FILE + 0x3c, (seed as i32 % 5 - 1) as u32);
                    w(c, REWARDS_FILE + $base_off, table);
                    for i in 0..4u32 {
                        w(c, table + 0x10 * i, r.next_u32());
                    }
                });
            }
        };
    }

    gallery_test!(
        cfe_state_reward_gallery_posters_init_buttons_matches,
        ee::s04::f_001d11f8__generated,
        cfe_state_reward_gallery_posters_init_buttons,
        0x1d11f8,
        0x14
    );
    gallery_test!(
        cfe_state_reward_gallery_trading_cards_init_buttons_matches,
        ee::s04::f_001d1390__generated,
        cfe_state_reward_gallery_trading_cards_init_buttons,
        0x1d1390,
        0x18
    );
    gallery_test!(
        cfe_state_reward_gallery_concept_arts_init_buttons_matches,
        ee::s04::f_001d1528__generated,
        cfe_state_reward_gallery_concept_arts_init_buttons,
        0x1d1528,
        0x1c
    );
    gallery_test!(
        cfe_state_reward_gallery_videos_init_buttons_matches,
        ee::s04::f_001d16c0__generated,
        cfe_state_reward_gallery_videos_init_buttons,
        0x1d16c0,
        0x24
    );
    gallery_test!(
        cfe_state_reward_gallery_toys_init_buttons_matches,
        ee::s04::f_001d1858__generated,
        cfe_state_reward_gallery_toys_init_buttons,
        0x1d1858,
        0x20
    );
    gallery_test!(
        cfe_state_reward_gallery_cheat_chars_init_buttons_matches,
        ee::s04::f_001d1990__generated,
        cfe_state_reward_gallery_cheat_chars_init_buttons,
        0x1d1990,
        0x28
    );

    #[test]
    fn cfe_state_char_equip_open_buy_popup_matches() {
        check_lohi(
            ee::s03::f_0019c7e8__generated,
            cfe_state_char_equip_open_buy_popup,
            30,
            &|c, seed| {
                let mut r = base(c, seed, 0x19c7e8, 4);
                let obj = DATA + 0x1000;
                let item = DATA + 0x2000;
                let screen = DATA + 0x3000;
                let list = DATA + 0x4000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = item as u64;
                w(c, obj + 0x10, screen);
                w(c, obj + 0xc0, r.next_u32());
                w8(c, obj + 0x44, seed as u8);
                w(c, screen + 0x18, list);
                w16(
                    c,
                    item + 0xe,
                    if seed % 3 == 0 {
                        0
                    } else {
                        (seed % 0x100) as u16
                    },
                );
                w(c, item + 0x14, r.next_u32());
            },
        );
    }
    // ------------------------------------------------------------------
    // Game state / world.

    #[test]
    fn game_state_race_enter_matches() {
        check(
            ee::s06::f_00234e20__generated,
            game_state_race_enter,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x234e20, 3);
                let gs = DATA + 0x1000;
                let ai = DATA + 0x2000;
                let riders = DATA + 0x4000;
                let scr = DATA + 0x5000;
                w(c, GAME_GP.wrapping_sub(0x488), gs);
                w(c, gs + 0xc, ai);
                w(c, gs + 0x28, riders);
                w(
                    c,
                    riders,
                    if seed % 3 == 0 { 0 } else { (seed % 12) as u32 },
                );
                w(c, gs + 0x48, scr);
                w(
                    c,
                    0x0053_5c10,
                    if seed % 2 == 0 { 4 } else { (seed % 8) as u32 },
                );
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn rider_estimate_final_score_matches() {
        check(
            ee::s00::f_00122e50__generated,
            rider_estimate_final_score,
            200,
            &|c, seed| {
                let mut r = base(c, seed, 0x122e50, 1);
                let rider = DATA + 0x1000;
                let metrix = DATA + 0x2000;
                c.g[A0][0] = rider as u64;
                w(c, rider + 0x790, metrix);
                w(c, metrix + 0x198, (seed % 32) as u32);
                w(c, rider + 0x4d8, r.edge_f32());
                w(c, rider + 0x4d0, r.edge_f32());
            },
        );
    }

        #[test]
    fn ov_screen_save_replay_v38_matches() {
        check(
            ee::s05::f_001fe5b0__generated,
            ov_screen_save_replay_v38,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1fe5b0, 4);
                let obj = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let node = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0x40, screen);
                hash_object(c, screen, if seed % 5 == 0 { 0 } else { node });
                w(c, node + 0x8, vt);
                slot(c, vt, 0x48, 6, FAKE);
                w(c, obj + 0x220, r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_save_records_v38_matches() {
        check(
            ee::s05::f_001ff278__generated,
            ov_screen_save_records_v38,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1ff278, 4);
                let obj = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let node = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0x40, screen);
                hash_object(c, screen, if seed % 5 == 0 { 0 } else { node });
                w(c, node + 0x8, vt);
                slot(c, vt, 0x48, 6, FAKE);
                w(c, obj + 0x220, r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_profile_save_pda_v38_matches() {
        check(
            ee::s05::f_00211970__generated,
            ov_screen_profile_save_pda_v38,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x211970, 4);
                let obj = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let node = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0x40, screen);
                hash_object(c, screen, if seed % 5 == 0 { 0 } else { node });
                w(c, node + 0x8, vt);
                slot(c, vt, 0x48, 6, FAKE);
                w(c, obj + 0x220, r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_in_game_error_update_server_timeout_text_matches() {
        check(
            ee::s04::f_001fb180__generated,
            ov_screen_in_game_error_update_server_timeout_text,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1fb180, 4);
                let obj = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let node = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x40, if seed % 4 == 0 { 0 } else { screen });
                w(c, obj + 0xa0, (seed % 7) as u32);
                hash_object(c, screen, if seed % 5 == 0 { 0 } else { node });
                w(c, node + 0x8, vt);
                slot(c, vt, 0x48, 6, FAKE);
                let _ = r.next_u32();
            },
        );
    }

    // ------------------------------------------------------------------
    // Script / player / crowd.

    #[test]
    fn script_engine_run_process_matches() {
        check(
            ee::s07::f_002822a0__generated,
            script_engine_run_process,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x2822a0, 3);
                let eng = DATA + 0x1000;
                let id = (seed % 4) as u32;
                let proc = eng + 0x2c0 + 0x24 * id;
                c.g[A0][0] = eng as u64;
                c.g[A1][0] = id as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                w(c, proc + 0x20, (seed / 4 % 2) as u32);
                w(c, proc + 0x4, (seed / 8 % 4) as u32);
                w(c, proc + 0x10, r.next_u32());
                w(c, proc, DATA + 0x2000);
                w(c, DATA + 0x2000, r.next_u32());
                w(c, eng + 0x2b8, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn benew_player_int_is_char_profile_flag_clear_matches() {
        check_lohi(
            ee::s01::f_00145d38__generated,
            benew_player_int_is_char_profile_flag_clear,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x145d38, 2);
                let index = (seed % 4) as u32;
                let player = (seed / 4 % 4) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = index as u64;
                c.g[A2][0] = (seed % 32) as u64;
                // `BE_GetRiderProfile` and `BENewPlayerInt_GetSlot` both read
                // the player id from `0x5305B0 + index*4`; the profile record is
                // at `0x5305B20 + pid*0x1C` and the character table at
                // `0x534FE0 + pid*0x1C`.
                w(c, 0x0053_05b0 + 4 * index, player);
                w(
                    c,
                    0x0053_0520 + 0x1c * player + 0xc,
                    if seed % 3 == 0 { 0xffff_ffff } else { 0 },
                );
                w(c, 0x0053_0520 + 0x1c * player + 0x10, (seed / 3 % 2) as u32);
                w8(c, 0x0053_4fe0 + 0x1c * player + 0x11, (seed % 5) as u8);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn crowd_man3d_dtor_matches() {
        check(ee::s11::f_00343b28__generated, crowd_man3d_dtor, 20, &|c, seed| {
            let mut r = base(c, seed, 0x343b28, 2);
            let obj = DATA + 0x1000;
            c.g[A0][0] = obj as u64;
            // Flags 0: the delete branch would free a pointer that did not come
            // from the test heap, which walks garbage (not a boundable fixture).
            c.g[A1][0] = 0;
            for i in 0..5u32 {
                let e = obj + 0x1f4 * i;
                w8(c, e + 0x1e, (seed >> i) as u8 & 1);
                w(c, e, r.next_u32());
                w(c, e + 4, r.next_u32());
            }
        });
    }

    #[test]
    fn cfe_state_rewards_room_ctor_matches() {
        check(
            ee::s04::f_001d4bc8__generated,
            cfe_state_rewards_room_ctor,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1d4bc8, 3);
                let obj = DATA + 0x1000;
                let engine = DATA + 0x2000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.g[A3][0] = r.next_u32() as u64;
                w(c, obj + 0x10, engine);
                w(c, engine, DATA + 0x3000);
                w(
                    c,
                    0x0053_08a8,
                    if seed % 2 == 0 { 0 } else { DATA + 0x4000 },
                );
                w(c, GAME_GP.wrapping_sub(0x1efc), 0);
            },
        );
    }

        #[test]
    fn rail_man_dtor_matches() {
        check(ee::s11::f_00358668__generated, rail_man_dtor, 16, &|c, seed| {
            let mut r = base(c, seed, 0x358668, 3);
            let obj = DATA + 0x1000;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            // One live rail plus unused slots, so the free loop stays short.
            for i in 0..0x80u32 {
                w(c, obj + 0x18 + 4 * i, if i == 0 { 0 } else { 0xffff_ffff });
            }
            let _ = r.next_u32();
        });
    }

    // ------------------------------------------------------------------
    // UI render / texture manager.

    #[test]
    fn ui_pair_render_matches() {
        check(ee::s13::f_0039c130__generated, ui_pair_render, 100, &|c, seed| {
            let mut r = base(c, seed, 0x39c130, 4);
            let obj = DATA + 0x1000;
            let a = DATA + 0x2000;
            let b = DATA + 0x3000;
            let vt = DATA + 0x4_0000;
            c.g[A0][0] = obj as u64;
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            w(c, obj + 0x78, if seed % 4 == 0 { 0 } else { a });
            w(c, obj + 0x7c, if seed % 3 == 0 { 0 } else { b });
            w(c, obj + 0x44, r.edge_f32());
            w(c, obj + 0x48, r.edge_f32());
            w(c, a + 0x8, vt);
            w(c, b + 0x8, vt);
            slot(c, vt, 0x88, 8, FAKE);
        });
    }

    #[test]
    fn cui_progress_render_matches() {
        check(ee::s13::f_0039c300__generated, cui_progress_render, 100, &|c, seed| {
            let mut r = base(c, seed, 0x39c300, 4);
            let obj = DATA + 0x1000;
            let a = DATA + 0x2000;
            let b = DATA + 0x3000;
            let vt = DATA + 0x4_0000;
            c.g[A0][0] = obj as u64;
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            w(c, obj + 0x74, if seed % 4 == 0 { 0 } else { a });
            w(c, obj + 0x78, if seed % 3 == 0 { 0 } else { b });
            w(c, obj + 0x44, r.edge_f32());
            w(c, obj + 0x48, r.edge_f32());
            w(c, a + 0x8, vt);
            w(c, b + 0x8, vt);
            slot(c, vt, 0x88, 8, FAKE);
        });
    }

            #[test]
    fn game_ui_open_finish_screen_matches() {
        check(
            ee::s05::f_0020cc60__generated,
            game_ui_open_finish_screen,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x20cc60, 4);
                w(c, 0x0053_5bc8 + 0x48, (seed % 8) as u32);
                w(c, 0x0053_5bc8 + 0x49, (seed / 8 % 4) as u32);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn cfe_state_mountain_room_dtor_matches() {
        check(
            ee::s04::f_001d28f8__generated,
            cfe_state_mountain_room_dtor,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1d28f8, 3);
                let obj = DATA + 0x1000;
                let world = DATA + 0x8_0000;
                let game = world + 0x1000;
                let reg = DATA + 0x2_0000;
                let vt = DATA + 0x3_0000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(
                    c,
                    obj + 0xbc,
                    if seed % 2 == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 8) as u32
                    },
                );
                w(c, obj + 0xb0, r.next_u32());
                w(
                    c,
                    obj + 0xc4,
                    if seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 8) as u32
                    },
                );
                w(c, world + 0x7c, game);
                w(c, GAME_GP.wrapping_sub(0x854), reg);
                w(c, reg + 0x10d8, vt);
                slot(c, vt, 0x190, 0, FAKE);
            },
        );
    }

    }
