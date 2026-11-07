//! `src/fe/festaterewards.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_3;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::movie::movieplayer::add32;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::wscript::wscriptman::gp as gp_2;
pub(crate) use crate::game::wscript::wscriptman::lw_4;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001c_f0b0,
        fe_state_reward_gallery_init,
        "cFEStateRewardGallery_Init",
    ),
    (
        0x001c_f168,
        cfe_state_reward_gallery_on_activate,
        "cFEStateRewardGallery_OnActivate",
    ),
    (
        0x001c_f238,
        fe_reward_gallery_update,
        "cFEStateRewardGallery_Update",
    ),
    (
        0x001c_fd18,
        fe_state_reward_gallery_on_child_event,
        "cFEStateRewardGallery_OnChildEvent",
    ),
    (
        0x001c_ff10,
        cfe_state_reward_gallery_poll_shape_load,
        "cFEStateRewardGallery_PollShapeLoad",
    ),
    (
        0x001c_fff8,
        cfe_state_reward_gallery_register_textures,
        "cFEStateRewardGallery_RegisterTextures",
    ),
    (
        0x001d_0238,
        c_fe_state_reward_gallery_check_rewards_ready,
        "cFEStateRewardGallery_CheckRewardsReady",
    ),
    (
        0x001d_0510,
        cfestaterewardgallery_open_buy_popup,
        "cFEStateRewardGallery_OpenBuyPopup",
    ),
    (
        0x001d_05f0,
        c_fe_state_reward_gallery_is_item_owned,
        "cFEStateRewardGallery_IsItemOwned",
    ),
    (
        0x001d_0b00,
        cfe_state_reward_gallery_refresh_page,
        "cFEStateRewardGallery_RefreshPage",
    ),
    (
        0x001d_0e78,
        c_fe_state_reward_gallery_update_cash_text,
        "cFEStateRewardGallery_UpdateCashText",
    ),
    (
        0x001d_0eb8,
        c_fe_state_reward_gallery_update_count_text,
        "cFEStateRewardGallery_UpdateCountText",
    ),
    (
        0x001d_10a8,
        c_fe_state_reward_gallery_update_scroll_arrows,
        "cFEStateRewardGallery_UpdateScrollArrows",
    ),
    (
        0x001d_1128,
        c_fe_state_reward_gallery_posters_ctor,
        "cFEStateRewardGalleryPosters_ctor",
    ),
    (
        0x001d_11a8,
        c_fe_state_reward_gallery_posters_v26,
        "cFEStateRewardGalleryPosters_v26",
    ),
    (
        0x001d_11f8,
        cfe_state_reward_gallery_posters_init_buttons,
        "cFEStateRewardGalleryPosters_InitButtons",
    ),
    (
        0x001d_1290,
        reward_gallery_posters_clear_item_flags,
        "cFEStateRewardGalleryPosters_ClearItemFlags",
    ),
    (
        0x001d_1340,
        c_fe_state_reward_gallery_trading_cards_v26,
        "cFEStateRewardGalleryTradingCards_v26",
    ),
    (
        0x001d_1390,
        cfe_state_reward_gallery_trading_cards_init_buttons,
        "cFEStateRewardGalleryTradingCards_InitButtons",
    ),
    (
        0x001d_1428,
        reward_gallery_trading_cards_clear_item_flags,
        "cFEStateRewardGalleryTradingCards_ClearItemFlags",
    ),
    (
        0x001d_14d8,
        c_fe_state_reward_gallery_concept_arts_v26,
        "cFEStateRewardGalleryConceptArts_v26",
    ),
    (
        0x001d_1528,
        cfe_state_reward_gallery_concept_arts_init_buttons,
        "cFEStateRewardGalleryConceptArts_InitButtons",
    ),
    (
        0x001d_15c0,
        reward_gallery_concept_arts_clear_item_flags,
        "cFEStateRewardGalleryConceptArts_ClearItemFlags",
    ),
    (
        0x001d_1670,
        c_fe_state_reward_gallery_videos_v26,
        "cFEStateRewardGalleryVideos_v26",
    ),
    (
        0x001d_16c0,
        cfe_state_reward_gallery_videos_init_buttons,
        "cFEStateRewardGalleryVideos_InitButtons",
    ),
    (
        0x001d_1758,
        reward_gallery_videos_clear_item_flags,
        "cFEStateRewardGalleryVideos_ClearItemFlags",
    ),
    (
        0x001d_1808,
        c_fe_state_reward_gallery_toys_v26,
        "cFEStateRewardGalleryToys_v26",
    ),
    (
        0x001d_1858,
        cfe_state_reward_gallery_toys_init_buttons,
        "cFEStateRewardGalleryToys_InitButtons",
    ),
    (
        0x001d_18f0,
        reward_gallery_toys_clear_item_flags,
        "cFEStateRewardGalleryToys_ClearItemFlags",
    ),
    (
        0x001d_1990,
        cfe_state_reward_gallery_cheat_chars_init_buttons,
        "cFEStateRewardGalleryCheatChars_InitButtons",
    ),
    (
        0x001d_1a28,
        reward_gallery_cheat_chars_clear_item_flags,
        "cFEStateRewardGalleryCheatChars_ClearItemFlags",
    ),
    (
        0x001d_1a60,
        c_fe_state_reward_gallery_cheat_chars_v27,
        "cFEStateRewardGalleryCheatChars_v27",
    ),
    (
        0x001d_1aa0,
        cfe_state_preview_reward_ctor,
        "cFEStatePreviewReward_ctor",
    ),
    (
        0x001d_1b68,
        c_fe_state_preview_reward_dtor,
        "cFEStatePreviewReward_dtor",
    ),
    (
        0x001d_1c08,
        c_fe_state_preview_reward_init,
        "cFEStatePreviewReward_Init",
    ),
    (
        0x001d_1f78,
        preview_reward_on_widget_notify,
        "cFEStatePreviewReward_OnWidgetNotify",
    ),
    (
        0x001d_1fb0,
        fe_state_preview_reward_update,
        "cFEStatePreviewReward_Update",
    ),
    (
        0x001d_1ff0,
        cfe_state_preview_reward_handle_input,
        "cFEStatePreviewReward_HandleInput",
    ),
    (
        0x001d_20d8,
        c_fe_state_preview_reward_set_item,
        "cFEStatePreviewReward_SetItem",
    ),
    (
        0x001d_2380,
        c_fe_state_video_player_dtor,
        "cFEStateVideoPlayer_dtor",
    ),
    (
        0x001d_2430,
        cfe_state_video_player_shutdown,
        "cFEStateVideoPlayer_Shutdown",
    ),
    (
        0x001d_2468,
        c_fe_state_video_player_render,
        "cFEStateVideoPlayer_Render",
    ),
    (
        0x001d_24a8,
        c_fe_state_video_player_update,
        "cFEStateVideoPlayer_Update",
    ),
    (
        0x001d_2518,
        c_fe_state_video_player_handle_input,
        "cFEStateVideoPlayer_HandleInput",
    ),
    (
        0x001d_25e8,
        cfe_state_video_player_start_playback,
        "cFEStateVideoPlayer_StartPlayback",
    ),
    (
        0x001d_2638,
        c_fe_state_video_player_finish,
        "cFEStateVideoPlayer_Finish",
    ),
];

// ---------------------------------------------------------------------------
// Direct-call targets (the generated body's `c.pc = addr; Name(c)` calls).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_INSTANCE;

    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;

    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_CHECK_REWARDS_READY;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_POLL_SHAPE_LOAD;

    pub(crate) use crate::game::be::beintmission::callee::UI_STATE_UPDATE;

    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;

    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;

    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;

    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;

    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4 as BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE_3;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;

    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_3;

    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;

    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE as UI_STATE_UPDATE_2;

    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;

    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;

    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;

    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;

    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES as UI_STATE_MGR_CLOSE_TOP_STATES_2;

    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;

    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;

    pub(crate) use crate::game::main::gamemode::callee::SPRINTF as SPRINTF_3;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REQUEST_IMAGE;

    pub const C_FE_STATE_REWARD_GALLERY_FIND_TEXTURE_BY_NAME: Callee = Callee {
        addr: 0x001c_fdd0,
        f: ee::s04::f_001cfdd0,
    };

    pub const C_FE_STATE_VIDEO_PLAYER_START_PLAYBACK: Callee = Callee {
        addr: 0x001d_25e8,
        f: ee::s04::f_001d25e8,
    };

    pub const SEQ_LISTENER_ON_START: Callee = Callee {
        addr: 0x0028_29d0,
        f: ee::s07::f_002829d0,
    };

    pub const SEQ_LISTENER_ON_STOP: Callee = Callee {
        addr: 0x0028_2a18,
        f: ee::s07::f_00282a18,
    };

    pub const GET_SEQ_PROCESS_ID: Callee = Callee {
        addr: 0x0028_2be0,
        f: ee::s07::f_00282be0,
    };

    pub const SCRIPT_LIST_ON_SEQ_START: Callee = Callee {
        addr: 0x0027_72c0,
        f: ee::s07::f_002772c0,
    };

    pub const SCRIPT_LIST_ON_SEQ_STOPPED: Callee = Callee {
        addr: 0x0027_7400,
        f: ee::s07::f_00277400,
    };

    pub const SCRIPT_BINDING_CLEAR_LISTENER: Callee = Callee {
        addr: 0x0028_2f90,
        f: ee::s07::f_00282f90,
    };

    pub const MENU_ITEM_GET_INPUT_CODE: Callee = Callee {
        addr: 0x002c_a3e8,
        f: ee::s09::f_002ca3e8,
    };

    pub const MENU_ITEM_IS_DISABLED: Callee = Callee {
        addr: 0x002c_a3b0,
        f: ee::s09::f_002ca3b0,
    };

    pub const MENU_ITEM_HANDLE_INPUT: Callee = Callee {
        addr: 0x002c_a2b0,
        f: ee::s09::f_002ca2b0,
    };

    pub const MENU_PAGE_VALIDATE_SELECTION: Callee = Callee {
        addr: 0x002c_aab0,
        f: ee::s09::f_002caab0,
    };

    pub const MENU_PUSH_PAGE: Callee = Callee {
        addr: 0x002c_c3b8,
        f: ee::s09::f_002cc3b8,
    };

    pub const MENU_FORMAT_FLOAT: Callee = Callee {
        addr: 0x002c_dfd8,
        f: ee::s09::f_002cdfd8,
    };

    pub const MENU_ITEM_DRAW_ROW: Callee = Callee {
        addr: 0x002c_a988,
        f: ee::s09::f_002ca988,
    };

    pub const IN_SEQ_INSTANCE_SETUP: Callee = Callee {
        addr: 0x0027_45c0,
        f: ee::s07::f_002745c0,
    };

    pub const LIGHT_MAN_EMPTY_HOOK: Callee = Callee {
        addr: 0x0038_dc60,
        f: ee::s12::f_0038dc60,
    };

    pub const UI_OBJECT_CTOR: Callee = Callee {
        addr: 0x0039_fb30,
        f: ee::s13::f_0039fb30,
    };

    pub const UI_LIST_CTOR: Callee = Callee {
        addr: 0x0039_77e8,
        f: ee::s13::f_003977e8,
    };

    pub const CUI_TEXT_SCROLL_SET_NAV_BUTTONS: Callee = Callee {
        addr: 0x003a_1310,
        f: ee::s13::f_003a1310,
    };

    pub const FE_STATE_CTOR: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::f_0039e2a0,
    };

    pub const REWARD_GALLERY_CTOR: Callee = Callee {
        addr: 0x001c_eea0,
        f: ee::s04::f_001ceea0,
    };

    pub const AI_VISUAL_EFFECTS_MAN_CTOR: Callee = Callee {
        addr: 0x002f_0368,
        f: ee::s09::f_002f0368,
    };

    pub const IS_DJ_SPEECH_ON_2: Callee = Callee {
        addr: 0x0029_f0d8,
        f: ee::s08::SSXAudioSystem_IsDJSpeechOn,
    };

    pub const IS_ARCADE_SPEECH_ON: Callee = Callee {
        addr: 0x0029_f160,
        f: ee::s08::SSXAudioSystem_IsArcadeSpeechOn,
    };

    pub const GET_LANGUAGE_MASK_2: Callee = Callee {
        addr: 0x0029_f198,
        f: ee::s08::SSXAudioSystem_GetLanguageMask,
    };

    pub const SPEECH_STOP_INSTANCE: Callee = Callee {
        addr: 0x002b_11b0,
        f: ee::s08::SpeechSystem_StopInstance,
    };

    pub const SPEECH_SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::SpeechSystem_SetEventContext,
    };

    pub const SCRIPT_POP_FREE_NODE: Callee = Callee {
        addr: 0x0027_bd78,
        f: ee::s07::ScriptEngine_PopFreeNode,
    };

    pub const SCRIPT_PUSH_FREE_NODE: Callee = Callee {
        addr: 0x0027_bd90,
        f: ee::s07::ScriptEngine_PushFreeNode,
    };

    pub const SCRIPT_START_SCRIPT: Callee = Callee {
        addr: 0x0028_21a0,
        f: ee::s07::ScriptEngine_StartScript,
    };

    pub const MOD_OBJ_RUN_SCRIPT_EVENT: Callee = Callee {
        addr: 0x0034_fe00,
        f: ee::s11::ModObj_RunScriptEvent19b8,
    };

    pub const REWARD_GALLERY_REGISTER_TEXTURES: Callee = Callee {
        addr: 0x001c_fff8,
        f: ee::s04::cFEStateRewardGallery_RegisterTextures,
    };

    pub const GAME_ERROR_SERVER_TIMEOUT: Callee = Callee {
        addr: 0x001f_b180,
        f: ee::s04::OVScreenInGameError_UpdateServerTimeoutText,
    };

    pub const GAME_ERROR_OPPONENT_DISCONNECTED: Callee = Callee {
        addr: 0x001f_b218,
        f: ee::s04::OVScreenInGameError_UpdateOpponentDisconnectedText,
    };

    pub const GAME_ERROR_SHOW_CANCEL: Callee = Callee {
        addr: 0x001f_aff8,
        f: ee::s04::OVScreenInGameError_ShowCancelGame,
    };

    pub const BE_LIBRARY_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };

    pub const BE_LIBRARY_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };

    pub const BE_REWARD_SET_BIT: Callee = Callee {
        addr: 0x0015_73f0,
        f: ee::s01::BERewardInt_SetBit,
    };

    pub const BE_REWARD_GET_FILE: Callee = Callee {
        addr: 0x0015_a478,
        f: ee::s01::BERewardInt_GetRewardsFile,
    };

    pub const BE_ECON_SPEND_MONEY: Callee = Callee {
        addr: 0x0015_0b48,
        f: ee::s01::BEEconInt_SpendMoney,
    };

    pub const BE_ECON_GET_MONEY: Callee = Callee {
        addr: 0x0015_0928,
        f: ee::s01::BEEconInt_GetMoney,
    };

    pub const TAG_FIELD_COPY_TAG: Callee = Callee {
        addr: 0x003e_b198,
        f: ee::s14::TagFieldCopyTag,
    };

    pub const TAG_FIELD_SET_RAW: Callee = Callee {
        addr: 0x003e_b1e0,
        f: ee::s14::TagFieldSetRaw,
    };

    pub const BRAGGING_RIGHTS_UPDATE_ROW_COLORS: Callee = Callee {
        addr: 0x0019_27b0,
        f: ee::s02::cFEStateBraggingRights_UpdateRowColors,
    };

    pub const BENEWPLAYER_IS_SHARING_CHAR_RECORD: Callee = Callee {
        addr: 0x0014_77e8,
        f: ee::s01::BENewPlayerInt_IsSharingCharRecord,
    };

    pub const BECHAR_GET_OVERALL_RATING: Callee = Callee {
        addr: 0x0014_8950,
        f: ee::s01::BECharInt_GetOverallRating,
    };

    pub const MESSAGE_IS_FLAG_SET: Callee = Callee {
        addr: 0x001e_3a30,
        f: ee::s04::MessageCenter_IsFlagSet,
    };

    pub const MESSAGE_SET_FLAG: Callee = Callee {
        addr: 0x001e_3a78,
        f: ee::s04::MessageCenter_SetFlag,
    };

    pub const MESSAGE_POST: Callee = Callee {
        addr: 0x001e_2fe0,
        f: ee::s04::MessageCenter_PostMessage,
    };

    pub const TEXFLIP_SET_FRAME: Callee = Callee {
        addr: 0x0035_f378,
        f: ee::s12::TexFlipModifier_SetFrame,
    };

    pub const BE_GET_RIDER_PROFILE_2: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::BE_GetRiderProfile,
    };

    pub const BENEWPLAYER_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::BENewPlayerInt_GetSlot,
    };

    pub const VFX_PLANTS_GET_SWAY_MATRIX: Callee = Callee {
        addr: 0x0038_f2a8,
        f: ee::s12::VfxPlants_GetSwayMatrix,
    };
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

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

    pub const UPLOAD_TEXTURE_PIXELS: u32 = 0x003e_6574;

    pub const MUSIC_SLOT_RELEASE: u32 = 0x0019_dc20;

    pub const UI_STATE_DTOR: u32 = 0x0039_e390;

    pub const BXSTRING_CTOR: u32 = 0x0031_82d8;

    pub const BXSTRING_DTOR: u32 = 0x0031_8260;

    pub const SECTION_TABLE_CTOR: u32 = 0x002b_b1d0;
}

/// `void cFEStateRewardGallery_Update(cFEStateRewardGallery* self)`
/// (0x1cf238): poll the reward readiness / shape loads and run the base UI
/// state update.
pub fn fe_reward_gallery_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::REWARD_GALLERY_CHECK_REWARDS_READY.call(c, 0x001c_f24c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REWARD_GALLERY_POLL_SHAPE_LOAD.call(c, 0x001c_f254)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::UI_STATE_UPDATE, 0x001c_f25c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateVideoPlayer_Shutdown(cFEStateVideoPlayer* self)` (0x1d2430):
/// stop and release the video player at `+0x248`.
pub fn cfe_state_video_player_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x248).lw(c);
        if c.g[A0][0] == 0 {
            let s0 = c.g[S0][0];
            Addr(s0 as u32).off(0x248).sw(c, 0);
        } else {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::VID_ENGINE_PLAYER_DTOR.call(c, 0x001d_2454)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).off(0x248).sw(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStatePreviewReward_Update(cFEStatePreviewReward* self)`
/// (0x1D1FB0): poll the reward image while one is loading (handle >= 0), then
/// run the UI state update.
pub fn fe_state_preview_reward_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let handle = c.lw_r_2(V0, r(c, S0).wrapping_add(0x6c));
        if (handle as i64) >= 0 {
            c.cycles += 2;
            callee::PREVIEW_REWARD_POLL_IMAGE_LOAD.call(c, 0x001d_1fd4)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::UI_STATE_UPDATE_2.call(c, 0x001d_1fdc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardGallery_UpdateCashText(State* s)` (0x1d0e78): the
/// text at `+0x910`, the value at `+0x54`.
pub fn c_fe_state_reward_gallery_update_cash_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0x910);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            c.g[A0][0] = lw(c, this, 0x54);
            callee::FE_FORMAT_MONEY.call(c, 0x001d_0e9c)?;
            let this = crate::abi::r32(c, S0);
            c.g[A0][0] = lw(c, this, 0x910);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_STRING_2.call(c, 0x001d_0ea8)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardGalleryCheatChars_v27(State* s, int i)` (0x1d1a60):
/// take cheat entry `i` (`+0xa28 + 4i`), hash its name and look the texture
/// up in the set at `*(s+0x10) + 0x58`.
pub fn c_fe_state_reward_gallery_cheat_chars_v27(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx(crate::abi::r32(c, A1) << 2);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(crate::abi::r32(c, A0).wrapping_add(crate::abi::r32(c, A1)));
        frame.save(c, S0, 0x10);
        let this = crate::abi::r32(c, A0);
        c.g[S0][0] = lw(c, this, 0x10);
        let idx = crate::abi::r32(c, A1);
        c.g[V0][0] = lw(c, idx, 0xa28);
        let entry = crate::abi::r32(c, V0);
        c.g[A0][0] = lw(c, entry, 4);
        callee::STR_HASH.call(c, 0x001d_1a84)?;
        c.g[A0][0] = sx(p(crate::abi::r32(c, S0), 0x58));
        c.g[A1][0] = c.g[V0][0];
        callee::UI_TEXTURE_SET_FIND.call(c, 0x001d_1a90)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateVideoPlayer_Render(Player* p)` (0x1d2468): draw the video
/// frame when a stream is open (and mark it playing), then draw the UI.
pub fn c_fe_state_video_player_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[A0][0] = lw(c, this, 0x248);
        let idle = c.g[A0][0] == 0;
        c.g[V0][0] = 1;
        if !idle {
            c.cycles += 3;
            c.g[A1][0] = 0;
            Addr(p(this, 0x278)).sw(c, c.g[V0][0]);
            callee::VID_ENGINE_PLAYER_RENDER.call(c, 0x001d_2490)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::UI_STATE_RENDER.call(c, 0x001d_2498)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateVideoPlayer_Finish(Player* p)` (0x1d2638): resume the
/// music, clear the "playing" flag and reopen the UI list.
pub fn c_fe_state_video_player_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SSX_AUDIO_SYSTEM_GET_INSTANCE.call(c, 0x001d_264c)?;
        c.g[A0][0] = sx(p(crate::abi::r32(c, V0), 0x118));
        callee::MUSIC_PLAYER_RESUME.call(c, 0x001d_2654)?;
        let this = crate::abi::r32(c, S0);
        c.g[A0][0] = lw(c, this, 0x10);
        c.g[A1][0] = 1;
        Addr(p(crate::abi::r32(c, S0), 0x27c)).sw(c, 0);
        c.g[A0][0] = sx(p(crate::abi::r32(c, A0), 0x18));
        callee::UI_STATE_MGR_CLOSE_TOP_STATES_2.call(c, 0x001d_2668)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStatePreviewReward_RequestImage(self, id)` (0x1d2150): ask the
/// front end's image loader (`world+0x7c` tagged with 0x1a70) for `self+0x64`
/// through the six-argument request call, and keep the handle at `+0x6c`.
pub fn c_fe_state_preview_reward_request_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A3][0] = 9;
        c.g[T0][0] = 1;
        let world = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(world.wrapping_add(0x7c)).lw(c);
        c.g[T1][0] = 0;
        let self_ = abi::r32(c, S0);
        c.g[A1][0] = Addr(self_.wrapping_add(0x64)).lw(c);
        let owner = (c.g[A0][0] as u32).wrapping_add(0x1a70);
        c.g[A0][0] = sx(owner);
        call_ptr(c, 0x0019_da10, 0x001d_2184)?;
        Addr(self_.wrapping_add(0x6c)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the five `cFEStateRewardGallery*_InitButtons`
/// handlers: copy the first four characters of the `slot`-th entry's name
/// (`(self + slot*4 + 0xa28)` -> `+8`) into a stack buffer and look the
/// texture up by that name.
pub(crate) fn reward_gallery_init_button(c: &mut Cpu, slot: u32, ret1: u32, ret2: u32) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx(slot << 2);
        frame.save(c, S0, 0x20);
        c.g[A2][0] = 0x4;
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[A1][0] = sx(this.wrapping_add(slot << 2));
        c.g[A0][0] = c.g[SP][0];
        c.g[V0][0] = Addr((this.wrapping_add(slot << 2)).wrapping_add(0xa28)).lw(c);
        c.g[A1][0] = Addr(c.g[V0][0] as u32).off(8).lw(c);
        call_ptr(c, 0x0041_6ce0, ret1)?;
        c.g[A0][0] = c.g[S0][0];
        Addr((c.g[SP][0] as u32).wrapping_add(4)).sb(c, 0);
        c.g[A1][0] = c.g[SP][0];
        C_FE_STATE_REWARD_GALLERY_FIND_TEXTURE_BY_NAME.call(c, ret2)?;
        frame.restore(c, S0, 0x20);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardGalleryPosters_InitButtons(self, slot)` (0x1d11a8).
pub fn c_fe_state_reward_gallery_posters_v26(c: &mut Cpu) {
    let slot = c.g[A1][0] as u32;
    reward_gallery_init_button(c, slot, 0x001d_11d4, 0x001d_11e4);
}

/// `void cFEStateRewardGalleryTradingCards_InitButtons(self, slot)`
/// (0x1d1340).
pub fn c_fe_state_reward_gallery_trading_cards_v26(c: &mut Cpu) {
    let slot = c.g[A1][0] as u32;
    reward_gallery_init_button(c, slot, 0x001d_136c, 0x001d_137c);
}

/// `void cFEStateRewardGalleryConceptArts_InitButtons(self, slot)` (0x1d14d8).
pub fn c_fe_state_reward_gallery_concept_arts_v26(c: &mut Cpu) {
    let slot = c.g[A1][0] as u32;
    reward_gallery_init_button(c, slot, 0x001d_1504, 0x001d_1514);
}

/// `void cFEStateRewardGalleryVideos_InitButtons(self, slot)` (0x1d1670).
pub fn c_fe_state_reward_gallery_videos_v26(c: &mut Cpu) {
    let slot = c.g[A1][0] as u32;
    reward_gallery_init_button(c, slot, 0x001d_169c, 0x001d_16ac);
}

/// `void cFEStateRewardGalleryToys_InitButtons(self, slot)` (0x1d1808).
pub fn c_fe_state_reward_gallery_toys_v26(c: &mut Cpu) {
    let slot = c.g[A1][0] as u32;
    reward_gallery_init_button(c, slot, 0x001d_1834, 0x001d_1844);
}

/// `void cFEStateVideoPlayer_Init(self)` (0x1d23e0): allocate and construct
/// the 0x44-byte video engine (`0x461320`) into `+0x248`, then start playback.
pub fn c_fe_state_video_player_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x0046_0000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0x1320));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x44;
        call_ptr(c, 0x0031_7d70, 0x001d_2408)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0025_33c8, 0x001d_2410)?;
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x248).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        C_FE_STATE_VIDEO_PLAYER_START_PLAYBACK.call(c, 0x001d_241c)?;
        frame.restore(c, S0, 0x10);
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

// ---------------------------------------------------------------------------
// Reward gallery (fe/festaterewardgallery.cpp).

/// Shared body of the six `ClearItemFlags` resets: clear the state word at
/// `+0x68` and the "flags dirty" word at `+0xa24`, then zero `count + 1`
/// item-flag words ending at `+base` (the array is walked downwards).
pub(crate) fn clear_item_flags(c: &mut Cpu, base: u32, count: u32, head: u32) {
    const STATE: u32 = 0x68;
    const FLAGS_DIRTY: u32 = 0xa24;
    c.cycles += 4;
    let this = r32(c, A0);
    c.sw(this.wrapping_add(STATE), 0);
    c.g[V1][0] = count as u64;
    c.sw(this.wrapping_add(FLAGS_DIRTY), 0);
    c.g[V0][0] = sx(this.wrapping_add(base));
    loop {
        c.cycles += 7;
        c.sw(r32(c, V0), 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void cFEStateRewardGalleryPosters_ClearItemFlags(cFEState* self)`
/// (0x1d1290): 43 poster flags at `+0xad0..+0xa28`.
pub fn reward_gallery_posters_clear_item_flags(c: &mut Cpu) {
    clear_item_flags(c, 0xad0, 0x2a, 0x001d_12a0);
}

/// `void cFEStateRewardGalleryTradingCards_ClearItemFlags(cFEState* self)`
/// (0x1d1428): 116 trading-card slot ids at `+0xbf4..+0xa28`.
pub fn reward_gallery_trading_cards_clear_item_flags(c: &mut Cpu) {
    clear_item_flags(c, 0xbf4, 0x73, 0x001d_1438);
}

/// `void cFEStateRewardGalleryConceptArts_ClearItemFlags(cFEState* self)`
/// (0x1d15c0): 100 concept-art ids at `+0xbb4..+0xa28`.
pub fn reward_gallery_concept_arts_clear_item_flags(c: &mut Cpu) {
    clear_item_flags(c, 0xbb4, 0x63, 0x001d_15d0);
}

/// `void cFEStateRewardGalleryVideos_ClearItemFlags(cFEState* self)`
/// (0x1d1758): the two video ids at `+0xa2c/+0xa28`.
pub fn reward_gallery_videos_clear_item_flags(c: &mut Cpu) {
    clear_item_flags(c, 0xa2c, 0x1, 0x001d_1768);
}

/// `void cFEStateRewardGalleryToys_ClearItemFlags(cFEState* self)`
/// (0x1d18f0): 28 toy ids at `+0xa94..+0xa28`.
pub fn reward_gallery_toys_clear_item_flags(c: &mut Cpu) {
    clear_item_flags(c, 0xa94, 0x1b, 0x001d_1900);
}

/// `void cFEStateRewardGalleryCheatChars_ClearItemFlags(cFEState* self)`
/// (0x1d1a28): 28 cheat-character ids at `+0xa94..+0xa28`.
pub fn reward_gallery_cheat_chars_clear_item_flags(c: &mut Cpu) {
    clear_item_flags(c, 0xa94, 0x1b, 0x001d_1a38);
}

/// `int cFEStatePreviewReward_OnWidgetNotify(cFEState*, int, int key)`
/// (0x1d1f78): the reward preview answers the back/cancel keys (6, 8, 9)
/// with 0x100 (handled) and everything else with 0x101 (ignored), matching
/// the generated code's nested range tests.
pub fn preview_reward_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x6;
    let is6 = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
    if !is6 {
        c.cycles += 2;
        let below6 = c.g[V0][0] != 0;
        c.g[V0][0] = (c.g[A2][0] < 0xau64) as u64;
        if !below6 {
            c.cycles += 2;
            let above10 = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[A2][0] < 0x8u64) as u64;
            if !above10 {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    // 8 or 9: handled.
                    c.cycles += 2;
                    c.g[V0][0] = 0x100;
                    abi::ret(c);
                    return;
                }
            }
        }
    } else {
        // key 6: handled.
        c.cycles += 2;
        c.g[V0][0] = 0x100;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0x101;
    abi::ret(c);
}

/// `void cFEStateVideoPlayer_dtor(cFEStateVideoPlayer* self, int free)`
/// (0x1d2380): restore the vtable, free the decoder, then chain to the base
/// destructor.
pub fn c_fe_state_video_player_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0046_9928);
        frame.save(c, S1, 0x10);
        let self_ = c.g[A0][0];
        let free = c.g[A1][0];
        c.g[S0][0] = self_;
        frame.save_ra(c, 0);
        Addr(at(c, S0, 0x8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(at(c, S0, 0x248)).lw(c);
        let none = c.g[A0][0] == 0;
        c.g[S1][0] = free;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0025_3418, 0x001d_23b4)?;
        }
        c.cycles += 11;
        c.g[V0][0] = 0x47_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx(0x0046_d0d0);
        c.g[A1][0] = c.g[S1][0];
        Addr(at(c, S0, 0x8)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x001d_23cc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Video player, AI riders and renderer misc.

/// `cFEStateVideoPlayer::Update(self)` (0x1d24a8): advance the movie player and
/// finish the state when the movie ends.
pub fn c_fe_state_video_player_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x248)).lw(c);
        let absent = c.g[A0][0] == 0;
        c.g[V0][0] = 1;
        if !absent {
            c.cycles += 5;
            Addr(p(c.g[S0][0] as u32, 0x274)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0025_3938, 0x001d_24cc)?;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x27c)).lw(c);
            let finish;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x248)).lw(c);
                c.cycles += 3;
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                finish = c.g[V1][0] != 0;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x278)).lw(c);
                if c.g[V0][0] != 0 {
                    finish = true;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x248)).lw(c);
                    c.cycles += 3;
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    finish = c.g[V1][0] != 0;
                }
            }
            if finish {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001d_2638, 0x001d_24fc)?;
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_e510, 0x001d_2504)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRewardGalleryPosters* cFEStateRewardGalleryPosters_ctor(...)`
/// (0x1d1128): construct the reward-gallery posters state (43 poster slots).
pub fn c_fe_state_reward_gallery_posters_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
        sqsp(c, 16, 0x10);
        let a2 = c.g[A2][0] as u32;
        c.g[A2][0] = sx(((a2 as i32) >> 24) as u32);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[T0][0] = 3;
        callee::REWARD_GALLERY_CTOR.call(c, 0x001d_1148)?;
        c.g[V1][0] = 0x0046_a248;
        Addr(p_2(r32(c, S0), 0x68)).sw(c, 0);
        Addr(p_2(r32(c, S0), 0xa24)).sw(c, 0);
        Addr(p_2(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xad0));
        c.g[V1][0] = 0x2a;
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_1168);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        lqsp(c, 16, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `cFEStatePreviewReward_SetItem` (0x1D20D8): store the reward item kind and
/// data on the state and request the item's image for kinds 3, 4, 5 and 7.
pub fn c_fe_state_preview_reward_set_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 4;
        frame.save_ra(c, 0);
        sw_at(c, A0, 0x68, c.g[A1][0]);
        let is4 = c.g[A1][0] == c.g[V0][0];
        sw_at(c, A0, 0x60, c.g[A2][0]);
        // Where the switch ends: 4 = the 4-cycle request at 0x1d2130,
        // 3 = the 3-cycle request at 0x1d2140, 0 = no request.
        //   kind 4        -> 4
        //   kind 3        -> 4
        //   kind 5        -> 4
        //   kind 7        -> 3
        //   anything else -> none
        let mut request = 0u32;
        if is4 {
            request = 4;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 5) as u64;
            let at_least5 = c.g[V0][0] == 0;
            c.g[V0][0] = 3;
            if at_least5 {
                // `break 'b_1d210c` jumps past the inner block to the second
                // comparison chain (the 5/7 arm), which charges 3 more.
                c.cycles += 3;
                c.g[V0][0] = 5;
                let is5 = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 7;
                if is5 {
                    request = 4;
                } else {
                    c.cycles += 2;
                    let is7 = c.g[A1][0] == c.g[V0][0];
                    c.g[RA][0] = Addr(sp(c)).ld(c);
                    if is7 {
                        request = 3;
                    } else {
                        c.cycles += 2;
                    }
                }
            } else {
                c.cycles += 2;
                let is3 = c.g[A1][0] == c.g[V0][0];
                c.g[RA][0] = Addr(sp(c)).ld(c);
                if is3 {
                    request = 4;
                } else {
                    c.cycles += 2;
                }
            }
        }
        if request == 4 {
            c.cycles += 4;
            c.g[A1][0] = lw_at(c, A2, 0x4);
            callee::REQUEST_IMAGE.call(c, 0x001d_2130)?;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        } else if request == 3 {
            c.cycles += 3;
            c.g[A1][0] = lw_at(c, A2, 0x4);
            callee::REQUEST_IMAGE.call(c, 0x001d_2140)?;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardGallery_UpdateCountText(self)` (0x1d0eb8): re-render
/// the two count labels from the stored totals.
pub fn c_fe_state_reward_gallery_update_count_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.sq(16, sp_2(c).wrapping_add(0x30));
        c.sd(sp_2(c).wrapping_add(0x20), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_r(c, S0, 0x918);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = lw_r(c, S0, 0x914);
        } else {
            c.cycles += 9;
            c.g[A1][0] = 0x4a_0000;
            c.g[A2][0] = lw_r(c, S0, 0x6c);
            c.g[A1][0] = sx(low(c, A1).wrapping_add(0x12c0));
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, callee::SPRINTF_3, 0x001d_0ee8)?;
            c.g[A0][0] = lw_r(c, S0, 0x918);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0c50, 0x001d_0ef4)?;
            c.g[V0][0] = lw_r(c, S0, 0x914);
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lq(16, sp_2(c).wrapping_add(0x30));
        } else {
            c.cycles += 9;
            c.g[A1][0] = 0x4a_0000;
            c.g[A2][0] = lw_r(c, S0, 0x68);
            c.g[A1][0] = sx(low(c, A1).wrapping_add(0x12c0));
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, callee::SPRINTF_3, 0x001d_0f14)?;
            c.g[A0][0] = lw_r(c, S0, 0x914);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0c50, 0x001d_0f20)?;
            c.lq(16, sp_2(c).wrapping_add(0x30));
        }
        c.cycles += 3;
        c.g[RA][0] = Addr(sp_2(c).wrapping_add(0x20)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
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
        c.g[A2][0] = Addr(at_2(c, S0, 0x91c)).lw(c);
        if c.g[A2][0] == 0 {
            c.g[T0][0] = Addr(at_2(c, S0, 0x920)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = Addr(at_2(c, A2, 0x8)).lw(c);
            c.g[A1][0] = Addr(at_2(c, S0, 0x64)).lw(c);
            c.g[A0][0] = Addr(at_2(c, V0, 0x48)).lh(c);
            c.g[V1][0] = Addr(at_2(c, V0, 0x4c)).lw(c);
            c.g[A1][0] = ((0u64 as i64) < (c.g[A1][0] as i64)) as u64;
            c.g[A0][0] = add32(word(c, A2), word(c, A0));
            call_ptr(c, word(c, V1), 0x001d_10e0)?;
            c.g[T0][0] = Addr(at_2(c, S0, 0x920)).lw(c);
        }
        c.cycles += 2;
        if c.g[T0][0] != 0 {
            c.cycles += 11;
            c.g[A2][0] = Addr(at_2(c, T0, 0x8)).lw(c);
            c.g[A1][0] = Addr(at_2(c, S0, 0x64)).lw(c);
            c.g[V0][0] = Addr(at_2(c, S0, 0x58)).lw(c);
            c.g[A3][0] = Addr(at_2(c, S0, 0x60)).lw(c);
            c.g[A0][0] = Addr(at_2(c, A2, 0x48)).lh(c);
            c.g[A1][0] = sx(word(c, A1).wrapping_add(word(c, V0)));
            c.g[V1][0] = Addr(at_2(c, A2, 0x4c)).lw(c);
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
        c.g[V0][0] = Addr(at_2(c, S1, 0x270)).lw(c);
        let closed = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if closed {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr(at_2(c, S0, 0x8)).lw(c);
            c.g[A0][0] = Addr(at_2(c, V1, 0x28)).lh(c);
            c.g[V0][0] = Addr(at_2(c, V1, 0x2c)).lw(c);
            c.g[A0][0] = add32(word(c, S0), word(c, A0));
            call_ptr(c, word(c, V0), 0x001d_254c)?;
            let handled = c.g[V0][0] != 0;
            c.g[V1][0] = 1;
            if !handled {
                c.cycles += 7;
                c.g[V1][0] = Addr(at_2(c, S0, 0x8)).lw(c);
                c.g[A0][0] = Addr(at_2(c, V1, 0x30)).lh(c);
                c.g[V0][0] = Addr(at_2(c, V1, 0x34)).lw(c);
                c.g[A0][0] = add32(word(c, S0), word(c, A0));
                call_ptr(c, word(c, V0), 0x001d_2568)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    Addr(at_2(c, S1, 0x27c)).sw(c, c.g[V1][0]);
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 1;
                Addr(at_2(c, S1, 0x27c)).sw(c, c.g[V1][0]);
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

/// `void cFEStatePreviewReward_Init(self)` (0x1d1c08): add the reward preview
/// screen by hash name, play it, then flag the preview object.
pub fn c_fe_state_preview_reward_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(p(r32(c, A0), 0x7140));
        c.g[S0][0] = c.lw(at_3(c, S1, 0x10));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001d_1c2c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME, 0x001d_1c40)?;
        let no_screen = c.g[V0][0] == 0;
        c.sw(at_3(c, S1, 0x40), c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, callee::UI_SCREEN_PLAY_FRAME, 0x001d_1c58)?;
        }
        c.cycles += 9;
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(p(r32(c, A0), 0x1398));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001d_1c64)?;
        c.g[A0][0] = c.lw(at_3(c, S1, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x001d_1c70)?;
        c.g[V1][0] = c.g[V0][0];
        let missing = c.g[V1][0] == 0;
        frame.restore(c, S0, 0x20);
        if !missing {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at_3(c, V1, 0x90));
            c.g[V0][0] = c.g[V0][0] | 8;
            c.sw(at_3(c, V1, 0x90), c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Small helpers.

/// The address `reg + off`.
#[inline(always)]
pub(crate) fn at_4(c: &Cpu, reg: usize, off: u32) -> u32 {
    abi::r32(c, reg).wrapping_add(off)
}

/// `lw_3 reg + off`.
#[inline(always)]
pub(crate) fn lw_3(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(at_4(c, reg, off)).lw(c)
}

/// Shared body of the six `cFEStateRewardGallery*_InitButtons` helpers: gather
/// the page's entry count and pointer table into the button list.
pub(crate) fn reward_gallery_init_buttons(
    c: &mut Cpu,
    entry: u32,
    getter: u32,
    base_off: u32,
) -> Flow {
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
            c.g[A0][0] = lw_3(c, S0, 0x68);
            c.g[A1][0] = sx((c.g[A3][0] as u32) << 4);
            c.g[V1][0] = lw_3(c, S0, 0xa24);
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

// ---------------------------------------------------------------------------
// Reward preview.

/// `void cFEStatePreviewReward_dtor(cFEStatePreviewReward* self, int flags)`
/// (0x1d1b68): release the previewed bolt file when one is set, reset the two
/// handles and run the base state destructor.
pub fn c_fe_state_preview_reward_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6608));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sw(c, p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        c.g[V1][0] = lw_4(c, p(c.g[S0][0] as u32, 0x70));
        let no_bolt = (c.g[V1][0] as i64) < 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_bolt {
            c.cycles += 7;
            c.g[A2][0] = lw_4(c, gp_2(c, -0x854));
            c.g[A1][0] = c.g[V1][0];
            c.g[V0][0] = lw_4(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x190)).lh(c);
            c.g[V1][0] = lw_4(c, p(c.g[V0][0] as u32, 0x194));
            c.g[V1][0] = sx(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001d_1bb0)?;
        }
        c.cycles += 3;
        c.g[A2][0] = lw_4(c, p(c.g[S0][0] as u32, 0x6c));
        let no_rider = (c.g[A2][0] as i64) < 0;
        c.g[V0][0] = lw_4(c, gp_2(c, -0x848));
        if !no_rider {
            c.cycles += 4;
            c.g[A1][0] = lw_4(c, p(c.g[S0][0] as u32, 0x64));
            c.g[A0][0] = lw_4(c, p(c.g[V0][0] as u32, 0x7c));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
            call_ptr(c, 0x0019_dc20, 0x001d_1bcc)?; // BoltFile_Release
        }
        c.cycles += 14;
        c.g[V0][0] = 0x0047_0000;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
        sw(c, p(c.g[S0][0] as u32, 0x70), c.g[V1][0]);
        sw(c, p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        sw(c, p(c.g[S0][0] as u32, 0x6c), c.g[V1][0]);
        call_ptr(c, 0x0039_e390, 0x001d_1bf0)?; // UIState_dtor
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Ports kept for later (no differential test yet; not in OVERRIDES).

/// `cFEStateRewardGallery_RequestShapeLoad` (0x1cfe60): ask the bolt manager
/// for the gallery's shape, or mark the request failed. Not listed: the
/// acquired path runs `BoltFile_Acquire`, which is not boundable on test data.
pub fn cfe_state_reward_gallery_request_shape_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0xc8));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xcc));
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a70));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001c_fe94)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            c.cycles += 4;
            c.g[V1][0] = 0xffff_ffff_ffff_fffb;
            c.g[V0][0] = c.g[V0][0] | 0x8;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
        } else {
            c.cycles += 19;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xcc));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_feb0)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A3][0] = 0x9;
            c.g[T0][0] = 0x1;
            c.g[T1][0] = 0u64;
            call_ptr(c, 0x0019_da10, 0x001c_fecc)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            c.g[A0][0] = 0xffff_ffff_ffff_fff7;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8ec), c.g[V0][0]);
            c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
            c.g[V1][0] = c.g[V1][0] | 0x4;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V1][0]);
        }
        c.cycles += 5;
        c.lq(16, frame.at(c, 0x20));
        c.lq(17, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardGallery_InitPages(self)` (0x1d0180): set the
/// "has next / has previous" bits of the reward gallery from the loaded
/// rewards and compute the page count.
pub fn cfe_reward_gallery_init_pages(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        call_ptr(c, 0x0014_dc80, 0x001d_0198)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x001d_01a4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0015_6a38, 0x001d_01b0)?;
        let ready = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x48)).lw(c);
        if !ready {
            c.cycles += 8;
            c.g[V1][0] = 0xffff_ffff_ffff_fffd;
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x1;
            c.sw(p(c.g[S0][0] as u32, 0x48), c.g[V0][0]);
            call_ptr(c, 0x0015_6a10, 0x001d_01d0)?;
        } else {
            c.cycles += 13;
            c.g[V1][0] = 0xffff_ffff_ffff_fffe;
            c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x2;
            c.sw(p(c.g[S0][0] as u32, 0x48), c.g[V0][0]);
            c.g[A0][0] = Addr(p(c.g[A1][0] as u32, 0xe0)).lh(c);
            c.g[V0][0] = Addr(p(c.g[A1][0] as u32, 0xe4)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001d_01fc)?;
            c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x5c)).lw(c);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x68)).lw(c);
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.div(0, 2, 3);
            c.g[V0][0] = c.lo[0];
            c.sw(p(c.g[S0][0] as u32, 0x60), c.g[V0][0]);
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
// FE state: reward gallery.

/// `void cFEStateRewardGallery_Init(cFEStateRewardGallery* s)` (0x1cf0b0):
/// create the gallery screen, clear its per-peak slots and load the first
/// page (or request the shape it needs).
pub fn fe_state_reward_gallery_init(c: &mut Cpu) {
    guest(c, |c| {
        // The generated body adjusts sp once at the entry and keeps the frame
        // open across the early `break` into the page loop; the epilogue at the
        // end closes it.
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        'outer: {
            c.cycles += 16;
            let this = r(c, A0);
            c.g[S1][0] = sx(this);
            frame.save_ra(c, 0);
            frame.save(c, S0, 0x20);
            c.g[A0][0] = sx(0x0046_6d88);
            c.g[RA][0] = sx(0x001c_f0d4);
            c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
            call_ptr(c, callee::FE_STRING_HASH, 0x001c_f0d4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001c_f0e8);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001c_f0e8)?;
            let none = c.g[V0][0] == 0;
            Addr(p(r(c, S1), 0x40)).sw(c, c.g[V0][0]);
            if none {
                break 'outer;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001c_f100);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001c_f100)?;
        }
        c.cycles += 4;
        c.g[A2][0] = u64::MAX;
        c.g[V1][0] = sx(p(r(c, S1), 0x84));
        c.g[A0][0] = sx(p(r(c, S1), 0x7c));
        c.g[A1][0] = sx(p(r(c, S1), 0x8f4));
        loop {
            c.cycles += 7;
            Addr(r(c, A0)).sb(c, 0);
            Addr(r(c, V1)).sw(c, c.g[A2][0]);
            c.g[V1][0] = sx(r(c, V1).wrapping_add(0xc));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r(c, A0).wrapping_add(0xc));
            if !more {
                'pages: {
                    c.cycles += 8;
                    Addr(p(r(c, S1), 0x78)).sw(c, 0);
                    c.g[RA][0] = sx(0x001c_f138);
                    c.g[A0][0] = c.g[S1][0];
                    callee::REWARD_GALLERY_INIT_PAGES.call(c, 0x001c_f138)?;
                    c.g[V0][0] = Addr(p(r(c, S1), 0x48)).lw(c);
                    c.g[V0][0] = sx((r(c, V0) as i32 >> 3) as u32);
                    c.g[V0][0] = c.g[V0][0] & 1;
                    let have = c.g[V0][0] != 0;
                    frame.restore(c, 16, 0x20);
                    if have {
                        break 'pages;
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001c_f154);
                    c.g[A0][0] = c.g[S1][0];
                    callee::REWARD_GALLERY_REQUEST_SHAPE.call(c, 0x001c_f154)?;
                    frame.restore(c, 16, 0x20);
                    break 'pages;
                }
                c.cycles += 4;
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_f110);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// FE state: child events.

/// `void cFEStateRewardGallery_OnChildEvent(cFEStateRewardGallery* s, void*
/// child, int event)` (0x1cfd18): when the child's action fires, toggle the
/// gallery's "dirty" bit and, for a completed preview, refresh the page.
pub fn fe_state_reward_gallery_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'outer: {
            'dirty: {
                'selected: {
                    'notify: {
                        c.cycles += 5;
                        let _frame = Frame::open(c, 0x10);
                        let this = r(c, A0);
                        c.g[T0][0] = sx(this);
                        c.g[V0][0] = 0x15;
                        let is_open = c.g[A2][0] == c.g[V0][0];
                        c.sd(r(c, SP), c.g[RA][0]);
                        if is_open {
                            break 'notify;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x16;
                        let is_close = c.g[A2][0] == c.g[V0][0];
                        c.g[RA][0] = c.ld(r(c, SP));
                        if is_close {
                            break 'selected;
                        }
                        c.cycles += 2;
                        break 'outer;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = Addr(p(r(c, A1), 0xc)).lw(c);
                    c.g[V0][0] = 7;
                    let done = c.g[V1][0] == c.g[V0][0];
                    c.g[V1][0] = sx(0xffff_fff7);
                    if done {
                        break 'dirty;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(p(r(c, T0), 0x1c)).lw(c);
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    Addr(p(r(c, T0), 0x1c)).sw(c, c.g[V0][0]);
                    break 'dirty;
                }
                'refresh: {
                    c.cycles += 5;
                    c.g[V1][0] = Addr(p(r(c, T0), 0x1c)).lw(c);
                    c.g[V0][0] = sx((r(c, V1) as i32 >> 3) as u32);
                    c.g[V0][0] = c.g[V0][0] & 1;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = Addr(p(r(c, A1), 0xc)).lw(c);
                        break 'refresh;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] | 8;
                    Addr(p(r(c, T0), 0x1c)).sw(c, c.g[V0][0]);
                    c.g[V1][0] = Addr(p(r(c, A1), 0xc)).lw(c);
                }
                c.cycles += 3;
                c.g[V0][0] = 7;
                let busy = c.g[V1][0] != c.g[V0][0];
                c.g[RA][0] = c.ld(r(c, SP));
                if busy {
                    break 'outer;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r(c, A1), 0x6c)).lw(c);
                let empty = c.g[V0][0] == 0;
                if empty {
                    break 'outer;
                }
                c.cycles += 11;
                c.g[V0][0] = Addr(p(r(c, T0), 0x5c)).lw(c);
                c.g[A1][0] = Addr(p(r(c, T0), 0x64)).lw(c);
                c.g[A3][0] = Addr(p(r(c, T0), 8)).lw(c);
                c.mult(0, 5, 5, 2);
                c.g[V1][0] = Addr(p(r(c, T0), 0x8fc)).lw(c);
                c.g[A0][0] = Addr(p(r(c, A3), 0xf0)).lh(c);
                c.g[V0][0] = Addr(p(r(c, V1), 0x18)).lw(c);
                c.g[A2][0] = Addr(p(r(c, A3), 0xf4)).lw(c);
                c.g[A0][0] = sx(p(r(c, T0), r(c, A0)));
                let target = r(c, A2);
                c.g[RA][0] = sx(0x001c_fdc4);
                c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
                call_ptr(c, target, 0x001c_fdc4)?;
                break 'dirty;
            }
            c.cycles += 1;
            c.g[RA][0] = c.ld(r(c, SP));
        }
        c.cycles += 2;
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `cFEStatePreviewReward::cFEStatePreviewReward(this, engine, owner, player)`
/// (0x1d1aa0): construct the base state, install the vtable, store the player
/// index, fetch the player's pad mask and zero the reward slots.
pub fn cfe_state_preview_reward_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let this = abi::arg(c, 0);
        let player = abi::arg64(c, 3);
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = sx((player as u32) << 24);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);

        c.g[S0][0] = sx((((c.g[S0][0] as u32) as i32) >> 24) as u32);
        callee::UI_STATE_CTOR_2_3.call(c, 0x001d_1ac0)?;
        c.g[V0][0] = 0x30;
        c.g[V1][0] = 0x0047_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6608));
        Addr(this).off(0xc).sw(c, c.g[V0][0]);
        Addr(this).off(8).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        Addr(this).off(0x44).sb(c, c.g[S0][0]);
        c.g[V0][0] = Addr(this).off(0x10).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        callee::FE_MODULE_GET_PLAYER_PAD_MASK_2.call(c, 0x001d_1ae8)?;
        Addr(this).off(0x15).sb(c, c.g[V0][0]);

        c.g[V1][0] = 2;
        c.g[V0][0] = sx(this.wrapping_add(0x54));
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).off(0xffff_fff4).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            Addr(c.g[V0][0] as u32).sw(c, 0);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_1af8);
            }
        }

        c.cycles += 20;
        Addr(this).off(0x60).sw(c, 0);
        callee::BE_LIBRARY_GET_INSTANCE_3.call(c, 0x001d_1b1c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        callee::BE_LIBRARY_GET_INTERFACE_3.call(c, 0x001d_1b28)?;
        c.g[A1][0] = Addr(this).off(0x44).lb(c);
        c.g[A0][0] = c.g[V0][0];
        callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT.call(c, 0x001d_1b34)?;
        c.g[V1][0] = u64::MAX;
        Addr(this).off(0x64).sw(c, c.g[V0][0]);
        Addr(this).off(0x74).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S1][0];
        Addr(this).off(0x68).sw(c, c.g[V1][0]);
        Addr(this).off(0x6c).sw(c, c.g[V1][0]);
        Addr(this).off(0x70).sw(c, c.g[V1][0]);

        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardGallery_RegisterTextures(cFEStateRewardGallery *self,
/// const GalleryData *data)` (0x1cfff8): publish the data's entry count and,
/// for each entry, copy its name stub into `self->entries[i]` and resolve the
/// texture handle through `cGame`'s vtable slot 46.
pub fn cfe_state_reward_gallery_register_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        if c.g[S2][0] == 0 {
            c.cycles += 2;
            Addr(p_3(c.g[S4][0], 0x78)).sw(c, 0);
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(p_3(c.g[S2][0], 0x8)).lw(c);
            c.g[S1][0] = 0;
            Addr(p_3(c.g[S4][0], 0x78)).sw(c, c.g[V0][0]);
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 2;
                c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
                c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x7c));
                loop {
                    c.cycles += 24;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    call_ptr(c, 0x003b_3d00, 0x001d_0054)?;
                    Addr(p_3(c.g[S0][0], 0x4)).sb(c, 0);
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A3][0] = 0;
                    c.g[T0][0] = 1;
                    c.g[T2][0] = Addr(gp_3(c, -0x854)).lw(c);
                    c.g[T1][0] = u64::MAX;
                    c.g[A1][0] = Addr(c.g[S3][0] as u32).lw(c);
                    c.g[V1][0] = Addr(p_3(c.g[T2][0], 0x10d8)).lw(c);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A0][0] = Addr(p_3(c.g[V1][0], 0x170)).lh(c);
                    c.g[V0][0] = Addr(p_3(c.g[V1][0], 0x174)).lw(c);
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let target = c.g[V0][0] as u32;
                    call_ptr(c, target, 0x001d_008c)?;
                    Addr(p_3(c.g[S0][0], 0x8)).sw(c, c.g[V0][0]);
                    c.g[V0][0] = Addr(p_3(c.g[S4][0], 0x78)).lw(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x001d_0040);
                    }
                }
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
    })
}

/// `void cFEStateRewardGallery_RefreshPage(cFEStateRewardGallery *self)`
/// (0x1d0b00): re-lay-out every page entry (`vtable[39]`), then refresh the
/// five text/scroll widgets (`vtable[46]`, `[45]`, `[44]`, `[41]`, `[42]`).
pub fn cfe_state_reward_gallery_refresh_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p_3(c.g[S1][0], 0x58)).lw(c);
        c.g[S0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(p_3(c.g[S1][0], 0x8)).lw(c);
            loop {
                c.cycles += 10;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[A0][0] = Addr(p_3(c.g[V0][0], 0x138)).lh(c);
                c.g[V1][0] = Addr(p_3(c.g[V0][0], 0x13c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001d_0b40)?;
                c.g[V0][0] = Addr(p_3(c.g[S1][0], 0x58)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(p_3(c.g[S1][0], 0x8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x001d_0b28);
                }
            }
        }
        c.cycles += 30;
        for (delta_off, fn_off, ret) in [
            (0x170u32, 0x174u32, 0x001d_0b64u32),
            (0x168, 0x16c, 0x001d_0b78),
            (0x160, 0x164, 0x001d_0b8c),
            (0x148, 0x14c, 0x001d_0ba0),
            (0x150, 0x154, 0x001d_0bb4),
        ] {
            c.g[V0][0] = Addr(p_3(c.g[S1][0], 0x8)).lw(c);
            c.g[A0][0] = Addr(p_3(c.g[V0][0], delta_off)).lh(c);
            c.g[V1][0] = Addr(p_3(c.g[V0][0], fn_off)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardGallery_OnActivate(cFEStateRewardGallery *self)`
/// (0x1cf168): take the reward object's screen position into the state's
/// scroll anchor, then update the selected page.
pub fn cfe_state_reward_gallery_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p_3(c.g[S0][0], 0x924)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[V1][0] = Addr(p_3(c.g[S0][0], 0x48)).lw(c);
        } else {
            c.cycles += 5;
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0290, 0x001c_f18c)?;
            c.g[A0][0] = Addr(p_3(c.g[S0][0], 0x8f8)).lw(c);
            if c.g[A0][0] == 0 {
                c.g[V1][0] = Addr(p_3(c.g[S0][0], 0x48)).lw(c);
            } else {
                c.cycles += 11;
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x003a_0290, 0x001c_f1a0)?;
                c.lwc1(1, p_3(c.g[SP][0], 0x14));
                c.lwc1(3, p_3(c.g[SP][0], 0x4));
                c.lwc1(0, p_3(c.g[SP][0], 0x10));
                c.lwc1(2, c.g[SP][0] as u32);
                c.sub_s(1, 1, 3);
                c.sub_s(0, 0, 2);
                c.swc1(1, p_3(c.g[S0][0], 0x74));
                c.swc1(0, p_3(c.g[S0][0], 0x70));
                c.g[V1][0] = Addr(p_3(c.g[S0][0], 0x48)).lw(c);
            }
        }
        c.cycles += 4;
        c.g[V0][0] = sx((((c.g[V1][0] as u32) as i32) >> 3) as u32);
        c.g[V0][0] &= 1;
        let no_page = c.g[V0][0] == 0;
        Addr(p_3(c.g[S0][0], 0x64)).sw(c, 0);
        if !no_page {
            c.cycles += 4;
            c.g[V0][0] = sx((((c.g[V1][0] as u32) as i32) >> 1) as u32);
            c.g[V0][0] &= 1;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x30);
                c.cycles += 3;
                frame.restore_ra(c, 0x20);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p_3(c.g[S0][0], 0x8fc)).lw(c);
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p_3(c.g[S0][0], 0x8)).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p_3(c.g[S0][0], 0x924)).lw(c);
                Addr(p_3(c.g[S0][0], 0x8fc)).sw(c, c.g[V0][0]);
                c.g[V0][0] = Addr(p_3(c.g[S0][0], 0x8)).lw(c);
            }
            c.cycles += 10;
            c.g[A0][0] = Addr(p_3(c.g[V0][0], 0x140)).lh(c);
            c.g[V1][0] = Addr(p_3(c.g[V0][0], 0x144)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_f20c)?;
            c.g[V0][0] = Addr(p_3(c.g[S0][0], 0x8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(p_3(c.g[V0][0], 0x108)).lh(c);
            c.g[V1][0] = Addr(p_3(c.g[V0][0], 0x10c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_f224)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
        c.cycles += 3;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Reward gallery.

/// `void cFEStateRewardGallery_OpenBuyPopup(self)` (0x1d0510): create the
/// buy popup for the selected reward and its item parameters.
pub fn cfestaterewardgallery_open_buy_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 55;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x110)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x114)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, f, 0x001d_0540)?;
        c.g[A1][0] = 0x460000;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x790));
        c.g[A0][0] = 0x70;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x001d_055c)?;
        c.g[A3][0] = Addr(p(r32(c, S0), 0x44)).lb(c);
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x001c_aba8, 0x001d_0570)?;
        c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[S3][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr(p(r32(c, V1), 0x128)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x12c)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, f, 0x001d_058c)?;
        c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr(p(r32(c, V1), 0x118)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x11c)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, f, 0x001d_05a8)?;
        c.g[T0][0] = Addr(p(r32(c, S0), 0x54)).lw(c);
        c.g[A0][0] = sx(p(r32(c, S3), 0x48));
        c.g[A1][0] = Addr(p(r32(c, S0), 0x4c)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[V0][0];
        call_ptr(c, 0x001c_aa18, 0x001d_05c0)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x10)).lw(c);
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = sx(p(r32(c, A0), 0x18));
        call_ptr(c, 0x0039_f290, 0x001d_05d0)?;
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
// cFEStateRewardGallery_CheckRewardsReady (0x00001d0238).

/// `cFEStateRewardGallery_CheckRewardsReady`: check whether all reward icons in the gallery have finished loading.
pub fn c_fe_state_reward_gallery_check_rewards_ready(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d0310: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_1d0310;
            } else {
                'b_1d030c: {
                    c.cycles += 9;
                    call_ptr(c, 0x0014dc80, 0x001d0264)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xd;
                    call_ptr(c, 0x0014dd58, 0x001d0270)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00156a38, 0x001d0278)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0xfffffffffffffffe;
                    if t {
                        break 'b_1d030c;
                    } else {
                        'b_1d02b4: {
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            c.g[V0][0] = c.g[V0][0] | 0x2;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                            c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0xe0));
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe4));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[RA][0] = sx(0x001d02a4);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x001d02a4 {
                                return Err(crate::call::Unwound);
                            }
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                            if c.g[A0][0] == 0u64 {
                                break 'b_1d02b4;
                            } else {
                                break 'b_1d02b4;
                            }
                        }
                        c.cycles += 9;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.div(0, 2, 4);
                        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                        c.g[V1][0] = c.g[V1][0] & 0x1;
                        c.g[V0][0] = c.lo[0];
                        let t = c.g[V1][0] == 0u64;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[V0][0]);
                        if t {
                            break 'b_1d030c;
                        } else {
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x924));
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8fc), c.g[V0][0]);
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x140));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x144));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[RA][0] = sx(0x001d02f4);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x001d02f4 {
                                return Err(crate::call::Unwound);
                            }
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x108));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[RA][0] = sx(0x001d030c);
                            c.pc = t;
                            ssx3_ee_rt::funcs::call_indirect(c, t);
                            if c.pc != 0x001d030c {
                                return Err(crate::call::Unwound);
                            }
                            break 'b_1d030c;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_1d0310;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    });
}

/// A `gp_4`-relative global address.
#[inline(always)]
pub(crate) fn gp_4(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Front-end screens.

/// `void cFEStateRewardGallery_PollShapeLoad(cFEStateRewardGallery* self)`
/// (0x1cff10): while the screen waits for the reward shape's file (bit 3 of
/// `+0x48` clear), poll the shape's load state; once loaded, register the
/// textures, release the file and mark the screen "done" (`+0x48` bit 3),
/// firing the two completion slots when bit 1 is set.
pub fn cfe_state_reward_gallery_poll_shape_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);

        c.g[V0][0] = Addr(r(c, S0)).off(0x48).lw(c);
        c.g[V0][0] = sx((r(c, V0) as i32 >> 3) as u32);
        c.g[V0][0] = c.g[V0][0] & 1;
        if c.g[V0][0] != 0 {
            frame.restore(c, 16, 0x20);
            return cfe_poll_tail(c, frame);
        }

        c.cycles += 9;
        c.g[V0][0] = Addr(gp_4(c, -0x848)).lw(c);
        c.g[A1][0] = Addr(r(c, S0)).off(0x50).lw(c);
        c.g[V1][0] = Addr(r(c, V0)).off(0x7c).lw(c);
        c.g[A2][0] = Addr(r(c, S0)).off(0x8ec).lw(c);
        c.g[S1][0] = sx(p(r(c, V1), 0x1a70));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0019_e238, 0x001c_ff54)?;
        if c.g[V0][0] == 0 {
            frame.restore(c, 16, 0x20);
            return cfe_poll_tail(c, frame);
        }

        'done: {
            c.cycles += 21;
            c.g[A2][0] = Addr(r(c, S0)).off(0x8ec).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = Addr(r(c, S0)).off(0x50).lw(c);
            call_ptr(c, 0x0019_e2b0, 0x001c_ff6c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            callee::REWARD_GALLERY_REGISTER_TEXTURES.call(c, 0x001c_ff78)?;
            c.g[A1][0] = Addr(r(c, S0)).off(0x50).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = Addr(r(c, S0)).off(0x8ec).lw(c);
            call_ptr(c, 0x0019_dc20, 0x001c_ff88)?;

            c.g[V1][0] = Addr(r(c, S0)).off(0x48).lw(c);
            c.g[V0][0] = 0xffff_ffff_ffff_fffb;
            c.g[A0][0] = u64::MAX;
            c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
            c.sw(Addr(r(c, S0)).off(0x8ec).0, c.g[A0][0]);
            c.g[V1][0] = c.g[V1][0] | 0x8;
            c.g[V0][0] = sx((r(c, V1) as i32 >> 1) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let quiet = c.g[V0][0] == 0;
            c.sw(Addr(r(c, S0)).off(0x48).0, c.g[V1][0]);
            if quiet {
                break 'done;
            }
            c.cycles += 13;
            c.g[V0][0] = Addr(r(c, S0)).off(0x924).lw(c);
            c.g[V1][0] = Addr(r(c, S0)).off(0x8).lw(c);
            c.sw(Addr(r(c, S0)).off(0x8fc).0, c.g[V0][0]);
            c.g[A0][0] = Addr(r(c, V1)).off(0x140).lh(c);
            c.g[V0][0] = Addr(r(c, V1)).off(0x144).lw(c);
            let target = r(c, V0);
            c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
            call_ptr(c, target, 0x001c_ffcc)?;

            c.g[V1][0] = Addr(r(c, S0)).off(0x8).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(r(c, V1)).off(0x108).lh(c);
            c.g[V0][0] = Addr(r(c, V1)).off(0x10c).lw(c);
            let target = r(c, V0);
            c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
            call_ptr(c, target, 0x001c_ffe4)?;
        }
        c.cycles += 1;
        frame.restore(c, 16, 0x20);
        cfe_poll_tail(c, frame)
    })
}

/// The `lq s1; ld ra; addiu sp; jr ra` tail.
pub(crate) fn cfe_poll_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    frame.restore(c, 17, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int cFEStatePreviewReward_HandleInput(cFEStatePreviewReward* self,
/// cUIEvent* ev)` (0x1d1ff0): when the event is a confirm on the preview, close
/// the state's objects and play the reward's "confirm" frame; returns 1 when
/// handled, 0 otherwise.
pub fn cfe_state_preview_reward_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);

        c.g[V1][0] = Addr(r(c, A1)).off(0x8).lw(c);
        c.g[A0][0] = Addr(r(c, V1)).off(0x30).lh(c);
        c.g[V0][0] = Addr(r(c, V1)).off(0x34).lw(c);
        let target = r(c, V0);
        c.g[A0][0] = sx(p(r(c, A1), r(c, A0)));
        call_ptr(c, target, 0x001d_201c)?;
        let not_confirm = r(c, V0) == 0;
        c.g[V0][0] = 0;
        if not_confirm {
            return preview_reward_tail(c, frame);
        }

        'objects: {
            c.cycles += 6;
            c.g[V0][0] = Addr(r(c, S2)).off(0x10).lw(c);
            c.g[V1][0] = sx(p(r(c, GP), 0x2968));
            c.g[A2][0] = Addr(r(c, V0)).off(0x14).lw(c);
            if c.g[A2][0] == 0 {
                c.g[A2][0] = c.g[V1][0];
            }
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 3;
            if none {
                break 'objects;
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(r(c, A2)).lw(c);
            c.g[A0][0] = Addr(r(c, V0)).off(0x10).lh(c);
            c.g[V1][0] = Addr(r(c, V0)).off(0x14).lw(c);
            let target = r(c, V1);
            c.g[A0][0] = sx(p(r(c, A2), r(c, A0)));
            call_ptr(c, target, 0x001d_2050)?;
        }

        c.cycles += 2;
        c.g[S0][0] = sx(p(r(c, S2), 0x48));
        c.g[S1][0] = 2;
        loop {
            'slot: {
                c.cycles += 3;
                c.g[A2][0] = Addr(r(c, S0)).lw(c);
                let empty = c.g[A2][0] == 0;
                c.g[A1][0] = 0;
                if empty {
                    break 'slot;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(r(c, A2)).off(0x8).lw(c);
                c.g[A0][0] = Addr(r(c, V0)).off(0x48).lh(c);
                c.g[V1][0] = Addr(r(c, V0)).off(0x4c).lw(c);
                let target = r(c, V1);
                c.g[A0][0] = sx(p(r(c, A2), r(c, A0)));
                call_ptr(c, target, 0x001d_2078)?;
            }
            c.cycles += 3;
            c.g[S1][0] = sx(r(c, S1).wrapping_sub(0x1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(p(r(c, S0), 0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_2058);
            }
        }

        c.cycles += 15;
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x23a0));
        call_ptr(c, 0x0031_7670, 0x001d_2090)?;
        c.g[A0][0] = Addr(r(c, S2)).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_c778, 0x001d_209c)?;
        c.g[A0][0] = Addr(r(c, S2)).off(0x40).lw(c);
        c.g[A2][0] = 1;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_c870, 0x001d_20ac)?;
        c.g[A0][0] = Addr(r(c, S2)).off(0x10).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = sx(p(r(c, A0), 0x18));
        call_ptr(c, 0x0039_f190, 0x001d_20bc)?;
        c.g[V0][0] = 1;
        preview_reward_tail(c, frame)
    })
}

/// The `lq s0-s2; ld ra; addiu sp; jr ra` tail.
pub(crate) fn preview_reward_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    frame.restore(c, 16, 0x30);
    frame.restore(c, 17, 0x20);
    frame.restore(c, 18, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::be::beintplayer::tests::w8;

    pub(crate) use crate::game::camera::cameracontroller::tests::pool as pool_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_machine;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;

    pub(crate) use crate::game::dirtysock::tags2::tests::clear;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;

    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;

    pub(crate) use crate::game::dirtysock::tags2::tests::run_2 as run;
    pub(crate) use crate::game::dirtysock::tags2::tests::string;
    pub(crate) use crate::game::dirtysock::tags2::tests::world_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::dirtysock::tags2::tests::MEM_HEAP;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::fe::debugfe::tests::put_str;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_9 as be_library_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_5;

    pub(crate) use crate::game::fe::festatecharequip2::tests::money_text_test;
    pub(crate) use crate::game::fe::festatecharequipdetail::tests::world;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;

    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2 as globals;

    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_4;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3;
    pub(crate) use crate::game::movie::movieplayer::tests::ret_slot;

    pub(crate) use crate::game::movie::movieplayer::tests::RET_ONE;
    pub(crate) use crate::game::movie::movieplayer::tests::RET_ZERO;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_3;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_8;

    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_5;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn fe_reward_gallery_update_matches() {
        check(
            ssx3_ee::s04::f_001cf238__generated,
            fe_reward_gallery_update,
            SEEDS,
            &|c, seed| {
                let _r = machine(c, seed, 0xcf238, 3);
                world(c);
                let self_ = DATA + 0x100;
                // bits 1 and 3: both poll functions return at once, then the
                // base UI state update runs (its virtual calls land in the hook).
                let _ = seed;
                c.bus.write32(self_ + 0x48, 0xa);
                c.g[A0][0] = sx(self_);
            },
        );
    }

    /// A real block of `size` bytes from the main heap.
    pub(crate) fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0032_0058,
            ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    #[test]
    fn cfe_state_video_player_shutdown_matches() {
        check(
            ee::s04::f_001d2430__generated,
            cfe_state_video_player_shutdown,
            60,
            &|c, seed| {
                let _r = alloc_machine(c, seed, 0x1d24_30);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x300);
                c.g[A0][0] = sx(self_);
                if seed % 2 != 0 {
                    let player = heap_block(c, 0x200);
                    zero(c, player, 0x200);
                    c.bus.write32(self_ + 0x248, player);
                } else {
                    c.bus.write32(self_ + 0x248, 0);
                }
            },
        );
    }

    #[test]
    fn fe_state_preview_reward_update_matches() {
        check(
            ssx3_ee::s04::f_001d1fb0__generated,
            fe_state_preview_reward_update,
            60,
            &|c, seed| {
                machine(c, seed, 0x1d1f_b0, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                let handle = if seed % 2 == 0 { 0xffff_ffffu32 } else { 5 };
                w(c, DATA + 0x6c, handle);
            },
        );
    }

    #[test]
    fn c_fe_state_reward_gallery_update_cash_text_matches() {
        money_text_test(
            ssx3_ee::s04::f_001d0e78__generated,
            c_fe_state_reward_gallery_update_cash_text,
            0x910,
            0x54,
            0x1c,
        );
    }

    #[test]
    fn c_fe_state_reward_gallery_cheat_chars_v27_matches() {
        check(
            ssx3_ee::s04::f_001d1a60__generated,
            c_fe_state_reward_gallery_cheat_chars_v27,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                let i = seed % 4;
                c.bus.write32(DATA + 0x10, DATA + 0x3000);
                c.bus.write32(DATA + i * 4 + 0xa28, DATA + 0x4000);
                c.bus.write32(DATA + 0x4004, DATA + 0x5000);
                put_str(c, DATA + 0x5000, ["", "cheat", "another"][i as usize % 3]);
                c.bus.write32(DATA + 0x5800, r.next_u32());
            },
        );
    }

    #[test]
    fn c_fe_state_video_player_render_matches() {
        check(
            ssx3_ee::s04::f_001d2468__generated,
            c_fe_state_video_player_render,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x23, 1);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x248, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
            },
        );
    }

    #[test]
    fn c_fe_state_video_player_finish_matches() {
        check(
            ssx3_ee::s04::f_001d2638__generated,
            c_fe_state_video_player_finish,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x24, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10, DATA + 0x3000);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x1d11a8 .. 0x1d1808 reward gallery InitButtons.

    pub(crate) fn reward_button_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 1);
        let this = DATA + 0x1000;
        let slot = seed % 8;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = slot as u64;
        let entry = DATA + 0x300;
        let name = DATA + 0x400;
        c.bus.write32(this + 0xa28 + slot * 4, entry);
        c.bus.write32(entry + 8, name);
        for i in 0..4u32 {
            c.bus.write8(name + i, (r.next_u32() % 0x40 + 1) as u8);
        }
        c.bus.write8(name + 4, 0);
        c.bus.write32(this + 0x78, 0); // no texture entries: the lookup ends at -1
        let _ = ptr(&mut r);
    }

    #[test]
    fn c_fe_state_reward_gallery_posters_v26_matches() {
        check(
            ee::s04::f_001d11a8__generated,
            c_fe_state_reward_gallery_posters_v26,
            8,
            &|c, seed| reward_button_setup(c, seed, 0x1d11_a8),
        );
    }

    #[test]
    fn c_fe_state_reward_gallery_trading_cards_v26_matches() {
        check(
            ee::s04::f_001d1340__generated,
            c_fe_state_reward_gallery_trading_cards_v26,
            8,
            &|c, seed| reward_button_setup(c, seed, 0x1d13_40),
        );
    }

    #[test]
    fn c_fe_state_reward_gallery_concept_arts_v26_matches() {
        check(
            ee::s04::f_001d14d8__generated,
            c_fe_state_reward_gallery_concept_arts_v26,
            8,
            &|c, seed| reward_button_setup(c, seed, 0x1d14_d8),
        );
    }

    #[test]
    fn c_fe_state_reward_gallery_videos_v26_matches() {
        check(
            ee::s04::f_001d1670__generated,
            c_fe_state_reward_gallery_videos_v26,
            8,
            &|c, seed| reward_button_setup(c, seed, 0x1d16_70),
        );
    }

    #[test]
    fn c_fe_state_reward_gallery_toys_v26_matches() {
        check(
            ee::s04::f_001d1808__generated,
            c_fe_state_reward_gallery_toys_v26,
            8,
            &|c, seed| reward_button_setup(c, seed, 0x1d18_08),
        );
    }

    #[test]
    fn cfe_state_video_player_start_playback_matches() {
        check(
            ee::s04::f_001d25e8__generated,
            cfe_state_video_player_start_playback,
            24,
            &|c, seed| {
                let _r = base(c, seed, 0x1d26);
                c.bus.write32(gp_at(0x410), DATA + 0x1000);
                c.bus.write32(DATA + 0x248, DATA + 0x2000);
                c.bus.write32(DATA + 0x254, 0xdead);
                c.bus.write32(DATA + 0x274, 0xdead);
                c.bus.write32(DATA + 0x278, 0xdead);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn c_fe_state_reward_gallery_is_item_owned_matches() {
        check(
            ee::s04::f_001d05f0__generated,
            c_fe_state_reward_gallery_is_item_owned,
            SEEDS_2,
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

    // ------------------------------------------------------------------
    // Reward gallery.

    pub(crate) fn clear_flags_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = setup(c, seed, salt);
        c.g[A0][0] = AUX as u64;
        for w in 0..((0xc00u32 - 0xa20) / 4) {
            put(c, AUX + 0xa20 + 4 * w, r.next_u32());
        }
    }

    #[test]
    fn reward_gallery_posters_clear_item_flags_matches() {
        check(
            ee::s04::f_001d1290__generated,
            reward_gallery_posters_clear_item_flags,
            200,
            &|c, seed| clear_flags_fixture(c, seed, 11),
        );
    }

    #[test]
    fn reward_gallery_trading_cards_clear_item_flags_matches() {
        check(
            ee::s04::f_001d1428__generated,
            reward_gallery_trading_cards_clear_item_flags,
            200,
            &|c, seed| clear_flags_fixture(c, seed, 12),
        );
    }

    #[test]
    fn reward_gallery_concept_arts_clear_item_flags_matches() {
        check(
            ee::s04::f_001d15c0__generated,
            reward_gallery_concept_arts_clear_item_flags,
            200,
            &|c, seed| clear_flags_fixture(c, seed, 13),
        );
    }

    #[test]
    fn reward_gallery_videos_clear_item_flags_matches() {
        check(
            ee::s04::f_001d1758__generated,
            reward_gallery_videos_clear_item_flags,
            200,
            &|c, seed| clear_flags_fixture(c, seed, 14),
        );
    }

    #[test]
    fn reward_gallery_toys_clear_item_flags_matches() {
        check(
            ee::s04::f_001d18f0__generated,
            reward_gallery_toys_clear_item_flags,
            200,
            &|c, seed| clear_flags_fixture(c, seed, 15),
        );
    }

    #[test]
    fn reward_gallery_cheat_chars_clear_item_flags_matches() {
        check(
            ee::s04::f_001d1a28__generated,
            reward_gallery_cheat_chars_clear_item_flags,
            200,
            &|c, seed| clear_flags_fixture(c, seed, 16),
        );
    }

    #[test]
    fn preview_reward_on_widget_notify_matches() {
        check(
            ee::s04::f_001d1f78__generated,
            preview_reward_on_widget_notify,
            200,
            &|c, seed| {
                setup(c, seed, 17);
                c.g[A2][0] = (seed % 12) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_video_player_dtor_matches() {
        check(
            ssx3_ee::s04::f_001d2380__generated,
            c_fe_state_video_player_dtor,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x1d2380, 0);
                w_2(c, OBJ + 0x248, if seed % 2 == 0 { 0 } else { OBJ2 });
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    // -------------------------------------------------------- video and misc

    #[test]
    fn c_fe_state_video_player_update_matches() {
        check(
            ee::s04::f_001d24a8__generated,
            c_fe_state_video_player_update,
            8,
            &|c, seed| {
                let _ = base_2(c, seed, 0x001d_24a8, 3);
                c.g[A0][0] = OBJ_2 as u64;
                let player = DATA + 0x3000;
                w(c, OBJ_2 + 0x248, if seed % 2 == 0 { 0 } else { player });
                w(c, OBJ_2 + 0x27c, (seed % 3) as u32);
                w(c, OBJ_2 + 0x278, (seed / 3) as u32);
                w(c, player + 0x40, 0);
                w(c, player + 0x38, 0);
            },
        );
    }

    /// A zeroed BE-library singleton at `gp-0x1f04` (otherwise the UI/FE
    /// helpers fall into the lazy disc load at `0x3de768`).
    pub(crate) fn be_library(c: &mut Cpu) {
        w32(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }

    #[test]
    fn c_fe_state_reward_gallery_posters_ctor_matches() {
        check(
            ssx3_ee::s04::f_001d1128__generated,
            c_fe_state_reward_gallery_posters_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d1128, 0);
                pool_2(c);
                be_library(c);
                let s = DATA + 0x1000;
                let engine = DATA + 0x2000;
                let names = DATA + 0x3000;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = engine as u64;
                c.g[A2][0] = (seed % 4) as u64;
                // Its 4th argument selects the "load the rider bio from disc"
                // branch: keep it null.
                c.g[A3][0] = 0;
                w32(c, s + 0x10, engine);
                w32(c, engine, names);
                for i in 0..8u32 {
                    w8(c, names + i + 8, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn c_fe_state_preview_reward_set_item_matches() {
        check(
            ee::s04::f_001d20d8__generated,
            c_fe_state_preview_reward_set_item,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x1d_20d8, 2);
                world_2(c);
                belib_2(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (seed % 10) as u64;
                c.g[A2][0] = (DATA + 0x2200) as u64;
                // The image request names a bolt slot that already exists in the
                // (otherwise zeroed) bolt file, so the request takes its "slot
                // found" arm instead of allocating a slot and starting a disc
                // read (0x19d780 is a lazy-load loop and runs away on test data).
                string(c, DATA + 0x2300, "BOLT");
                w(c, DATA + 0x2200 + 4, DATA + 0x2300);
                // The file base is `world_2->7c + 0x1a70` = DATA + 0x2f70; the
                // slot names sit at +4, 0x120 apart.
                string(c, DATA + 0x2f74, "BOLT");
                c.pc = 0x001d_20d8;
            },
        );
    }

    #[test]
    fn c_fe_state_reward_gallery_update_count_text_matches() {
        check(
            ee::s04::f_001d0eb8__generated,
            c_fe_state_reward_gallery_update_count_text,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d0eb8, 4);
                be_library_2(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                w(
                    c,
                    obj + 0x918,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                w(c, obj + 0x914, if seed % 4 < 2 { 0 } else { DATA + 0x1100 });
                w(c, obj + 0x6c, r.next_u32());
                w(c, obj + 0x68, r.next_u32());
                for a in [DATA + 0x1000, DATA + 0x1100] {
                    w(c, a, 0);
                    w(c, a + 8, 0);
                }
            },
        );
    }

    // ---------------------------------------------------------------- gallery

    #[test]
    fn c_fe_state_reward_gallery_update_scroll_arrows_matches() {
        check(
            ee::s04::f_001d10a8__generated,
            c_fe_state_reward_gallery_update_scroll_arrows,
            12,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1d10_a8, 4);
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
                let _ = base_3(c, seed, 0x1d25_18, 4);
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

    #[test]
    fn c_fe_state_preview_reward_init_matches() {
        check(
            ee::s04::f_001d1c08__generated,
            c_fe_state_preview_reward_init,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1d1c, 0);
                env(c);
                let this = DATA + 0x400;
                w32(
                    c,
                    this + 0x10,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                w32(c, this + 0x40, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// A `{ short delta; short; void (*fn)(); }` slot_2 at_4 `vt + off`.
    pub(crate) fn slot_2(c: &mut Cpu, vt: u32, off: u32, delta: i16, func: u32) {
        w16(c, vt + off, delta as u16);
        w(c, vt + off + 4, func);
    }

    /// The BELibrary singleton (`gp - 0x1f04`, a zeroed dummy so nothing reads
    /// the disc) and the world (`gp - 0x848`, with no riders).
    pub(crate) fn globals_3(c: &mut Cpu) {
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

    /// The engine's size-class pool_4 allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class (a zeroed heap walks garbage).
    pub(crate) fn pool_4(c: &mut Cpu) {
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
        for (i, (slot_2, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let p = DATA + 0x40_0000 + 0x4_0000 * i;
            let base_4 = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base_4 + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, p + head, base_4);
            w(c, GAME_GP + slot_2, p);
        }
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)` over a test arena, for paths
    /// that allocate beyond the size-class pools.
    pub(crate) fn heap_init_3(c: &mut Cpu) {
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
    pub(crate) fn base_4(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        c.g[GP][0] = GAME_GP as u64;
        heap_init_3(c);
        let r = machine(c, seed, salt, calls);
        globals_3(c);
        pool_4(c);
        r
    }

    /// `BERewardInt_GetRewardsFile()` returns this global block; the page
    /// getters read `count` at_4 +0x3C and the pointer table at_4 `base_off`.
    pub(crate) const REWARDS_FILE: u32 = 0x004c_3e98;

    macro_rules! gallery_test {
        ($name:ident, $addr:expr, $func:expr, $salt:expr, $base_off:expr) => {
            #[test]
            fn $name() {
                check($addr, $func, 40, &|c, seed| {
                    let mut r = base_4(c, seed, $salt, 1);
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

    // ------------------------------------------------------- reward preview

    #[test]
    fn c_fe_state_preview_reward_dtor_matches() {
        check(
            ssx3_ee::s04::f_001d1b68__generated,
            c_fe_state_preview_reward_dtor,
            12,
            &|c, seed| {
                let _ = base_5(c, seed, 0x001d_1b68);
                let state = DATA + 0x100;
                let mgr = DATA + 0x5000;
                let vtable = DATA + 0x6000;
                clear_2(c, state, 0x100);
                clear_2(c, mgr, 0x1200);
                clear_2(c, vtable, 0x200);
                w(
                    c,
                    state + 0x70,
                    if seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 4) as u32
                    },
                );
                w(
                    c,
                    state + 0x6c,
                    if seed % 4 == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 4) as u32
                    },
                );
                w(c, state + 0x64, DATA + 0x7000);
                w(c, GAME_GP.wrapping_sub(0x854), mgr);
                w(c, mgr + 0x10d8, vtable);
                c.bus.write16(vtable + 0x190, 0);
                w(c, vtable + 0x194, crate::difftest::fakes::FAKE);
                w(c, crate::difftest::fakes::LOG_RESULTS, 1);
                w(c, WORLD_3 + 0x7c, 0x10_0000);
                c.g[A0][0] = sx(state);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn fe_state_reward_gallery_init_matches() {
        check(
            ee::s04::f_001cf0b0__generated,
            fe_state_reward_gallery_init,
            40,
            &|c, seed| {
                alloc_base(c, seed, 0x1cf0, 4);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x10, DATA + 0x400);
                w(c, DATA + 0x48, (seed % 4) << 3);
            },
        );
    }

    #[test]
    fn fe_state_reward_gallery_on_child_event_matches() {
        check_lohi(
            ee::s04::f_001cfd18__generated,
            fe_state_reward_gallery_on_child_event,
            SEEDS_5,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x1cfd, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = match seed % 4 {
                    0 => 0x15,
                    1 => 0x16,
                    _ => 0x17,
                };
                w(c, DATA + 0x800 + 0xc, (seed % 8) as u32);
                w(c, DATA + 0x800 + 0x6c, if seed % 2 == 0 { 0 } else { 1 });
                w(c, DATA + 0x1c, r.next_u32());
                w(c, DATA + 0x5c, r.next_u32());
                w(c, DATA + 0x64, r.next_u32());
                w(c, DATA + 0x8, DATA + 0x900);
                w(c, DATA + 0x8fc, DATA + 0xa00);
                w(c, DATA + 0x900 + 0xf0, 0);
                w(c, DATA + 0x900 + 0xf4, FAKE);
                w(c, DATA + 0xa00 + 0x18, r.next_u32());
            },
        );
    }

    #[test]
    fn cfe_state_preview_reward_ctor_matches() {
        check(
            ee::s04::f_001d1aa0__generated,
            cfe_state_preview_reward_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d1a, 3);
                game_globals(c);
                be_library_4(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x2400) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                c.bus.write32(this + 0x10, DATA + 0x2800);
                c.bus.write32(DATA + 0x2800, DATA + 0x2c00);
            },
        );
    }

    #[test]
    fn cfe_state_reward_gallery_on_activate_matches() {
        check(
            ee::s04::f_001cf168__generated,
            cfe_state_reward_gallery_on_activate,
            60,
            &|c, seed| {
                let _r = base_7(c, seed, 0x1cf1);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x48, (seed as u32) | 0x2);
                c.bus
                    .write32(DATA + 0x924, if seed % 4 == 0 { 0 } else { DATA + 0x3000 });
                c.bus
                    .write32(DATA + 0x8f8, if seed % 4 <= 1 { 0 } else { DATA + 0x3100 });
                c.bus
                    .write32(DATA + 0x8fc, if seed % 2 == 0 { 0 } else { DATA + 0x4000 });
                c.bus.write32(DATA + 8, DATA + 0x2000);
                c.bus.write16(DATA + 0x2000 + 0x140, 0);
                c.bus.write32(DATA + 0x2000 + 0x144, FAKE);
                c.bus.write16(DATA + 0x2000 + 0x108, 0);
                c.bus.write32(DATA + 0x2000 + 0x10c, FAKE);
            },
        );
    }

    #[test]
    fn cfe_state_reward_gallery_register_textures_matches() {
        check(
            ee::s04::f_001cfff8__generated,
            cfe_state_reward_gallery_register_textures,
            120,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x1cff);
                let count = seed % 3;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 11 == 0 {
                    0
                } else {
                    (DATA + 0x2000) as u64
                };
                c.bus.write32(DATA + 0x2008, count);
                for i in 0..3 {
                    c.bus.write32(DATA + 0x2014 + 8 * i, r.next_u32() & 0x7ff);
                }
                // `cGame` and its vtable slot 46.
                let game = DATA + 0x3000;
                let vt = DATA + 0x3100;
                c.bus.write32(gp_at(-0x854), game);
                c.bus.write32(game + 0x10d8, vt);
                c.bus.write16(vt + 0x170, 0);
                c.bus.write32(vt + 0x174, FAKE);
                for i in 0..3 {
                    // entry structs at self+0x7c, 0xc bytes each
                    c.bus.write32(DATA + 0x7c + 0xc * i + 4, 0);
                }
            },
        );
    }

    #[test]
    fn cfe_state_reward_gallery_refresh_page_matches() {
        check(
            ee::s04::f_001d0b00__generated,
            cfe_state_reward_gallery_refresh_page,
            120,
            &|c, seed| {
                let _r = base_7(c, seed, 0x1d0b);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x58, (seed % 3) as u32);
                let vt = DATA + 0x3000;
                c.bus.write32(DATA + 0x8, vt);
                for (i, (d, f)) in [
                    (0x138u32, 0x13cu32),
                    (0x170, 0x174),
                    (0x168, 0x16c),
                    (0x160, 0x164),
                    (0x148, 0x14c),
                    (0x150, 0x154),
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write16(vt + d, 0);
                    c.bus.write32(vt + f, FAKE + 0x10 * i as u32);
                }
            },
        );
    }

    #[test]
    fn cfestaterewardgallery_open_buy_popup_matches() {
        check(
            ee::s04::f_001d0510__generated,
            cfestaterewardgallery_open_buy_popup,
            4,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x1d0510, 3);
                let this = DATA;
                clear_3(c, this, 0x400);
                c.g[A0][0] = this as u64;
                let vt = vtable(c, &mut r, &[0x110, 0x128, 0x118]);
                w(c, this + 8, vt);
                w(c, this + 0x10, DATA + 0x200);
                w(c, this + 0x44, (seed % 4) as u32);
                w(c, this + 0x4c, 0);
                w(c, this + 0x54, 0);
            },
        );
    }

    #[test]
    fn reward_gallery_check_rewards_ready_matches() {
        diff(
            ee::s04::f_001d0238__generated,
            c_fe_state_reward_gallery_check_rewards_ready,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1d_0238);
                c.g[A0][0] = sx(DATA);
                w(c, DATA + 0x48, r.next_u32());
                w(c, DATA + 8, 0);
                w(c, DATA + 0x5c, r.next_u32());
                w(c, DATA + 0x68, r.next_u32());
            },
        );
    }

    /// The zeroed BELibrary dummy and the world singleton.
    pub(crate) const BE_7: u32 = DATA + 0x1_0000;

    /// `{ target, v0, f0 }` overrides applied after a fake call; `f0` ==
    /// `KEEP_F0` leaves the fake's value alone.
    pub(crate) const HOOKS: u32 = DATA + 0x7_0000;

    pub(crate) const KEEP_F0: u32 = 0xdead_beef;

    /// The `miss` hook: the shared fake, then the queued per-target results.
    pub(crate) fn hook(c: &mut Cpu, target: u32) {
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, target, ret);
        if c.pc != ret {
            return;
        }
        let mut p = HOOKS;
        loop {
            let t = c.bus.read32(p);
            if t == 0 {
                return;
            }
            if t == target {
                c.g[V0][0] = c.bus.read32(p + 4) as u64;
                let f0 = c.bus.read32(p + 8);
                if f0 != KEEP_F0 {
                    c.f[0] = f0;
                }
                return;
            }
            p += 0xc;
        }
    }

    /// Queue the results an indirect call to `target` returns.
    pub(crate) fn force(c: &mut Cpu, slot: u32, target: u32, v0: u32, f0: u32) {
        let p = HOOKS + 0xc * slot;
        w(c, p, target);
        w(c, p + 4, v0);
        w(c, p + 8, f0);
        w(c, p + 0xc, 0);
    }

    /// Strict comparison: hard, soft and LO/HI must all match.
    pub(crate) fn strict(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..seed_count(seeds) {
            let report = crate::difftest::compare(generated, hand, &|c| setup(c, seed));
            assert!(
                report.hard.is_empty(),
                "seed {seed}: hard {:#?}",
                report.hard
            );
            assert!(
                report.soft.is_empty(),
                "seed {seed}: soft {:#?}",
                report.soft
            );
            assert!(
                report.lohi.is_empty(),
                "seed {seed}: LO/HI {:#?}",
                report.lohi
            );
        }
    }

    /// Scrambled registers, a game-like `gp_4`, the BE_7 singleton, a world with
    /// no riders and the result-queueing miss hook.
    pub(crate) fn base_9(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.hooks.miss = hook;
        w(c, GAME_GP.wrapping_sub(0x1f04), BE_7);
        w(c, GAME_GP.wrapping_sub(0x1efc), 0);
        w(c, GAME_GP.wrapping_sub(0x848), W);
        let game = W + 0x1000;
        w(c, W + 0x84, game);
        let race = W + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x78, 0);
        r
    }

    // -------------------------------------------------------- front-end FE

    #[test]
    fn cfe_state_reward_gallery_poll_shape_load_matches() {
        strict(
            ee::s04::f_001cff10__generated,
            cfe_state_reward_gallery_poll_shape_load,
            90,
            &|c, seed| {
                let _ = base_9(c, seed, 0x1cff, 6);
                let flags = [0u32, 2, 8, 0xa][(seed % 4) as usize];
                let index = (seed % 2) as u32;
                let id = (seed % 3) as u32;
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x48, flags);
                w(c, DATA + 0x50, index);
                w(c, DATA + 0x8ec, seed * 7 + 1);
                // world->+0x7c is the game; the shape manager sits at
                // game+0x1a70.
                w(c, W + 0x7c, W + 0x1000);
                let mgr = W + 0x1000 + 0x1a70;
                let slot = mgr + id * 0x120 + index * 0x12000;
                w(c, slot, if seed % 2 == 0 { 1 } else { 0 });
                // The loaded file's data: a zeroed shape (no textures).
                w(c, slot + 0x104, DATA + 0x7000);
                // self->+8 is the state's vtable, used for the two optional
                // completion callbacks.
                let vt = DATA + 0x2000;
                w(c, DATA + 8, vt);
                w(c, vt + 0x140, 0);
                w(c, vt + 0x144, FAKE + 0x70);
                w(c, vt + 0x108, 0);
                w(c, vt + 0x10c, FAKE + 0x80);
                w(c, DATA + 0x924, 0x1234);
            },
        );
    }

    #[test]
    fn cfe_state_preview_reward_handle_input_matches() {
        strict(
            ee::s04::f_001d1ff0__generated,
            cfe_state_preview_reward_handle_input,
            90,
            &|c, seed| {
                let _ = base_9(c, seed, 0x1d1f, 6);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                let evt_vt = DATA + 0x1000;
                w(c, DATA + 0x808, evt_vt);
                w(c, evt_vt + 0x30, 0);
                w(c, evt_vt + 0x34, FAKE);
                force(c, 0, FAKE, if seed % 4 == 0 { 0 } else { 1 }, KEEP_F0);
                // self->+0x10 points at the screen; its +0x14 at the default
                // object (or zero, which selects the static default).
                let holder = DATA + 0x900;
                w(c, DATA + 0x10, holder);
                w(
                    c,
                    holder + 0x14,
                    if seed % 3 == 0 { 0 } else { DATA + 0xa00 },
                );
                let p = DATA + 0xa00;
                let pvt = DATA + 0x1100;
                w(c, p, pvt);
                w(c, pvt + 0x10, 0);
                w(c, pvt + 0x14, FAKE + 0x10);
                // The three child objects at +0x48.
                for i in 0..3u32 {
                    let o = DATA + 0xb00 + 0x40 * i;
                    let vt = DATA + 0x1200 + 0x20 * i;
                    let present = (seed >> i) & 1 == 0;
                    w(c, DATA + 0x48 + 4 * i, if present { o } else { 0 });
                    w(c, o + 8, vt);
                    w(c, vt + 0x48, 0);
                    w(c, vt + 0x4c, FAKE + 0x20);
                }
                w(c, DATA + 0x40, DATA + 0xc00);
            },
        );
    }
}
