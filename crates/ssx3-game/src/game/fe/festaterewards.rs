//! `src/fe/festaterewards.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::set_fpr;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
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
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::fe::ovstatelodge::fe_state_world;
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
    (
        0x001c_f270,
        c_fe_state_reward_gallery_handle_input,
        "cFEStateRewardGallery_HandleInput",
    ),
    (
        0x001c_f820,
        c_fe_state_reward_gallery_on_widget_created,
        "cFEStateRewardGallery_OnWidgetCreated",
    ),
    (
        0x001c_fdd0,
        reward_gallery_find_texture_by_name,
        "cFEStateRewardGallery_FindTextureByName",
    ),
    (
        0x001c_fe60,
        reward_gallery_request_shape_load,
        "cFEStateRewardGallery_RequestShapeLoad",
    ),
    (
        0x001d_00c0,
        reward_gallery_release_textures,
        "cFEStateRewardGallery_ReleaseTextures",
    ),
    (
        0x001d_0180,
        reward_gallery_init_pages,
        "cFEStateRewardGallery_InitPages",
    ),
    (
        0x001d_0320,
        reward_gallery_purchase_item,
        "cFEStateRewardGallery_PurchaseItem",
    ),
    (
        0x001d_03f8,
        fe_state_reward_gallery_view_item,
        "cFEStateRewardGallery_ViewItem",
    ),
    (
        0x001d_0648,
        c_fe_state_reward_gallery_update_help_text,
        "cFEStateRewardGallery_UpdateHelpText",
    ),
    (
        0x001d_0890,
        c_fe_state_reward_gallery_refresh_row_icons,
        "cFEStateRewardGallery_RefreshRowIcons",
    ),
    (
        0x001d_0bc8,
        c_fe_state_reward_gallery_update_page_text,
        "cFEStateRewardGallery_UpdatePageText",
    ),
    (
        0x001d_0d30,
        fe_state_reward_gallery_update_item_info,
        "cFEStateRewardGallery_UpdateItemInfo",
    ),
    (
        0x001d_0f30,
        fe_state_reward_gallery_update_selection,
        "cFEStateRewardGallery_UpdateSelection",
    ),
    (
        0x001d_12c8,
        fe_state_reward_gallery_trading_cards_ctor,
        "cFEStateRewardGalleryTradingCards_ctor",
    ),
    (
        0x001d_1460,
        fe_state_reward_gallery_concept_arts_ctor,
        "cFEStateRewardGalleryConceptArts_ctor",
    ),
    (
        0x001d_15f8,
        fe_state_reward_gallery_videos_ctor,
        "cFEStateRewardGalleryVideos_ctor",
    ),
    (
        0x001d_1790,
        fe_state_reward_gallery_toys_ctor,
        "cFEStateRewardGalleryToys_ctor",
    ),
    (
        0x001d_1928,
        fe_state_reward_gallery_cheat_chars_ctor,
        "cFEStateRewardGalleryCheatChars_ctor",
    ),
    (
        0x001d_1c98,
        c_fe_state_preview_reward_on_widget_created,
        "cFEStatePreviewReward_OnWidgetCreated",
    ),
    (
        0x001d_2150,
        cfe_state_preview_reward_request_image,
        "cFEStatePreviewReward_RequestImage",
    ),
    (
        0x001d_2198,
        cfe_state_preview_reward_poll_image_load,
        "cFEStatePreviewReward_PollImageLoad",
    ),
    (
        0x001d_23e0,
        cfe_state_video_player_init,
        "cFEStateVideoPlayer_Init",
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

    pub(crate) use crate::game::dirtysock::tagsunk::callee::AUDIO_APPLY_OUTPUT_MODE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::BE_REWARD_INT_IS_UNLOCKED;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::GET_HASH_VALUE32;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SOUND_PLAYER_START;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::VID_ENGINE_PLAYER_DTOR;

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

    pub(crate) use crate::game::fe::festatetrophyroom::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::UI_STATE_UPDATE as UI_STATE_UPDATE_2;

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

/// `int cFEStateRewardGallery_HandleInput(cFEStateRewardGallery* self,
/// cUIEvent* ev)` (0x1cf270): the reward gallery's pad handling. Back (event
/// vtable +0x34) hides the page's cells, starts the screen's exit thread and
/// switches state; confirm (+0x2c) buys or opens the item under the cursor
/// (self +0x64 page start, +0x5c columns, +0x68 item count, +0x8f0 reward kind);
/// left/right/up/down (+0x9c, +0xa4, +0x8c, +0x94) move the cursor inside the
/// page grid (+0x58 rows, wrapping) and the page up/down events (+0xdc, +0xcc)
/// move the page start by one row block (+0x60 rows in all). A changed cursor
/// stores the new cell (+0x8fc), notifies the state's vtable (+0x144, +0x10c)
/// and sends a small code (1 move, 2 horizontal move, 4 refused, 3 back, 6 buy;
/// probably a UI sound or animation id) to the default object (`self->+0x10->+0x14`, else gp+0x2968).
/// Returns 1 when the event was handled, 0 otherwise. The body keeps the
/// generated block structure (labels are the generated block addresses).
pub fn c_fe_state_reward_gallery_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 16, 0x80);
        frame.save(c, 18, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 22, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x70);
        c.g[S6][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
        frame.save(c, 19, 0x50);
        frame.save(c, 20, 0x40);
        frame.save(c, 21, 0x30);
        frame.save(c, 23, 0x10);
        frame.save_ra(c, 0);
        'b_1cf7f4: {
            'b_1cf3a8: {
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x34));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                let t = c.g[V0][0] as u32;
                if c.g[V1][0] != 0 {
                    c.g[S6][0] = c.g[V1][0];
                }
                call_ptr(c, t, 0x001cf2c4)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x3;
                if t {
                    break 'b_1cf3a8;
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x924));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001cf2e4)?;
                    c.g[V1][0] = 0u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 5);
                    'l_1cf2f0: loop {
                        c.cycles += 4;
                        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                        c.g[S1][0] = 0x7;
                        'l_1cf300: loop {
                            'b_1cf320: {
                                c.cycles += 3;
                                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1cf320;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001cf320)?;
                                    break 'b_1cf320;
                                }
                            }
                            c.cycles += 3;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[S1][0] as i64) >= 0;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001cf300);
                                }
                                continue 'l_1cf300;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.g[S3][0];
                                c.g[V0][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 5);
                                    if event_due(c) {
                                        deliver_event_here(c, 0x001cf2f0);
                                    }
                                    continue 'l_1cf2f0;
                                } else {
                                    c.cycles += 27;
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x23a0));
                                    call_ptr(c, 0x00317670, 0x001cf348)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039c778, 0x001cf354)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                    c.g[A2][0] = 0x1;
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039c870, 0x001cf364)?;
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8f0));
                                    c.g[A3][0] = c.lw(c.g[V1][0] as u32);
                                    c.g[A2][0] = !(0u64 | c.g[A2][0]);
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[A2][0] = sx((c.g[A2][0] as u32) >> 31);
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001cf390)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f400, 0x001cf3a0)?;
                                    c.g[V0][0] = 0x1;
                                    break 'b_1cf7f4;
                                }
                            }
                        }
                    }
                }
            }
            'b_1cf3b8: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8fc));
                let t = c.g[V0][0] == 0u64;
                c.g[S7][0] = 0u64;
                if t {
                    break 'b_1cf3b8;
                } else {
                    c.cycles += 1;
                    c.g[S7][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    break 'b_1cf3b8;
                }
            }
            'b_1cf3cc: {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                c.g[S5][0] = 0u64;
                c.div(0, 23, 2);
                break 'b_1cf3cc;
            }
            'b_1cf7dc: {
                'b_1cf7d8: {
                    'b_1cf54c: {
                        c.cycles += 12;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[S3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[A1][0] = c.lo[0];
                        c.g[A2][0] = c.hi[0];
                        c.g[S1][0] = c.g[A1][0];
                        let t = c.g[V0][0] as u32;
                        c.g[S4][0] = c.g[A2][0];
                        call_ptr(c, t, 0x001cf3f4)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1cf54c;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                            c.mult(0, 5, 2, 3);
                            let t = (c.g[A0][0] as i64) <= 0;
                            c.g[S3][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S7][0] as u32));
                            if t {
                                break 'b_1cf7d8;
                            } else {
                                'b_1cf52c: {
                                    'b_1cf510: {
                                        'b_1cf50c: {
                                            'b_1cf4fc: {
                                                'b_1cf468: {
                                                    c.cycles += 8;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = c.g[S3][0];
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V1][0] as u32).wrapping_add(0x130)
                                                        );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x134)
                                                        );
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001cf42c)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x8;
                                                    if t {
                                                        break 'b_1cf468;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x4c));
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[A1][0] = 0x6;
                                                        if t {
                                                            break 'b_1cf50c;
                                                        } else {
                                                            c.cycles += 10;
                                                            c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A0][0] = sx((c.g[S6][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x001cf454)?;
                                                            c.g[V0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = c.g[S3][0];
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0xf8));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xfc));
                                                            break 'b_1cf4fc;
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8f0));
                                                let t = (c.g[V0][0] as i64) < 0;
                                                c.g[A1][0] = c.g[S3][0];
                                                if t {
                                                    break 'b_1cf52c;
                                                } else {
                                                    c.cycles += 15;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V1][0] as u32).wrapping_add(0x110)
                                                        );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x114)
                                                        );
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001cf488)?;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                    c.g[S1][0] = c.g[V0][0];
                                                    c.g[S0][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x8f0)
                                                        );
                                                    c.g[A1][0] = c.g[S1][0];
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V1][0] as u32).wrapping_add(0x120)
                                                        );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x124)
                                                        );
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001cf4a8)?;
                                                    if c.g[S0][0] != c.g[V0][0] {
                                                        c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                                                        break 'b_1cf510;
                                                    } else {
                                                        c.cycles += 10;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[A1][0] = c.g[S1][0];
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x118));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x11c));
                                                        let t = c.g[V0][0] as u32;
                                                        c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        call_ptr(c, t, 0x001cf4c8)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x54));
                                                        c.g[V1][0] = ((c.g[V1][0] as i64)
                                                            < (c.g[V0][0] as i64))
                                                            as u64;
                                                        let t = c.g[V1][0] != 0u64;
                                                        c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                                                        if t {
                                                            break 'b_1cf510;
                                                        } else {
                                                            c.cycles += 9;
                                                            c.g[A1][0] = 0x6;
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A0][0] = sx((c.g[S6][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x001cf4ec)?;
                                                            c.g[V0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = c.g[S3][0];
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x100));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x104));
                                                            break 'b_1cf4fc;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001cf504)?;
                                            c.g[V0][0] = 0x1;
                                            break 'b_1cf7f4;
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                                        break 'b_1cf510;
                                    }
                                    c.cycles += 7;
                                    c.g[A1][0] = 0x4;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001cf524)?;
                                    c.g[V0][0] = 0x1;
                                    break 'b_1cf7f4;
                                }
                                c.cycles += 8;
                                c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                                c.g[A1][0] = 0x4;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001cf544)?;
                                c.g[V0][0] = 0x1;
                                break 'b_1cf7f4;
                            }
                        }
                    }
                    'b_1cf6c8: {
                        'b_1cf57c: {
                            c.cycles += 6;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001cf55c)?;
                            if c.g[V0][0] == 0u64 {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                break 'b_1cf57c;
                            } else {
                                c.cycles += 3;
                                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                                let t = (c.g[S4][0] as i64) >= 0;
                                c.g[S5][0] = 0x1;
                                if t {
                                    break 'b_1cf6c8;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    break 'b_1cf6c8;
                                }
                            }
                        }
                        'b_1cf698: {
                            'b_1cf690: {
                                'b_1cf5d0: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001cf58c)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        break 'b_1cf5d0;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[S4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[S4][0] = 0u64;
                                            break 'b_1cf690;
                                        } else {
                                            c.cycles += 8;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S1][0] as u32));
                                            c.mult(0, 5, 2, 4);
                                            c.g[V0][0] =
                                                sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[S4][0] as u32));
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[S5][0] = 0x1;
                                            if t {
                                                break 'b_1cf6c8;
                                            } else {
                                                c.cycles += 2;
                                                c.g[S4][0] = 0u64;
                                                break 'b_1cf6c8;
                                            }
                                        }
                                    }
                                }
                                'b_1cf600: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001cf5e0)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        break 'b_1cf600;
                                    } else {
                                        c.cycles += 3;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                        let t = (c.g[S1][0] as i64) >= 0;
                                        c.g[S5][0] = 0x1;
                                        if t {
                                            break 'b_1cf6c8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                            break 'b_1cf6c8;
                                        }
                                    }
                                }
                                'b_1cf64c: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001cf610)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        break 'b_1cf64c;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[S1][0] = 0u64;
                                            break 'b_1cf690;
                                        } else {
                                            c.cycles += 6;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S1][0] as u32));
                                            c.g[V1][0] =
                                                ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[S5][0] = 0x1;
                                            if t {
                                                break 'b_1cf6c8;
                                            } else {
                                                c.cycles += 2;
                                                c.g[S1][0] = 0u64;
                                                break 'b_1cf6c8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xd8));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xdc));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001cf65c)?;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    break 'b_1cf698;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                                    c.g[S3][0] =
                                        sx((c.g[S3][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                    let t = (c.g[S3][0] as i64) >= 0;
                                    c.g[S5][0] = 0x1;
                                    if t {
                                        break 'b_1cf6c8;
                                    } else {
                                        'b_1cf680: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                                            break 'b_1cf680;
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        c.div(0, 2, 3);
                                        c.g[V0][0] = c.lo[0];
                                        c.mult(0, 19, 2, 3);
                                        break 'b_1cf690;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[S5][0] = 0x1;
                            break 'b_1cf6c8;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc8));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xcc));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001cf6a8)?;
                        let t = c.g[V0][0] == 0u64;
                        if t {
                            break 'b_1cf6c8;
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                            c.g[S5][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                c.g[S3][0] = 0u64;
                            }
                            break 'b_1cf6c8;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S5][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_1cf7f4;
                    } else {
                        'b_1cf710: {
                            c.cycles += 12;
                            c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                            c.mult(0, 3, 17, 6);
                            c.mult(0, 4, 19, 6);
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                            c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                c.g[S1][0] = c.g[V1][0];
                            }
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                            if c.g[S7][0] != c.g[A1][0] {
                                c.div(0, 5, 6);
                                break 'b_1cf710;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                                if c.g[S3][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                                    break 'b_1cf7dc;
                                } else {
                                    c.cycles += 1;
                                    c.div(0, 5, 6);
                                    break 'b_1cf710;
                                }
                            }
                        }
                        'b_1cf71c: {
                            c.cycles += 3;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x64), c.g[S3][0]);
                            break 'b_1cf71c;
                        }
                        'b_1cf7bc: {
                            'b_1cf7a0: {
                                c.cycles += 26;
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lo[0];
                                c.g[V0][0] = c.hi[0];
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.g[V0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x924));
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x8fc), c.g[V1][0]);
                                c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x140));
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x144));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001cf750)?;
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x108));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001cf768)?;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001cf77c)?;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                                    break 'b_1cf7a0;
                                } else {
                                    c.cycles += 7;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001cf798)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                                    if t {
                                        break 'b_1cf7bc;
                                    } else {
                                        break 'b_1cf7a0;
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.g[A1][0] = 0x2;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001cf7b4)?;
                            c.g[V0][0] = 0x1;
                            break 'b_1cf7f4;
                        }
                        c.cycles += 7;
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001cf7d0)?;
                        c.g[V0][0] = 0x1;
                        break 'b_1cf7f4;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                break 'b_1cf7dc;
            }
            c.cycles += 6;
            c.g[A1][0] = 0x4;
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001cf7f0)?;
            c.g[V0][0] = 0x1;
            break 'b_1cf7f4;
        }
        c.cycles += 11;
        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore(c, 20, 0x40);
        frame.restore(c, 21, 0x30);
        frame.restore(c, 22, 0x20);
        frame.restore(c, 23, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRewardGallery_UpdatePageText` (0x001d0bc8): puts "page n of m" into the
/// gallery's page text widget (format string from the screen's text object,
/// `per = self+0x58`, `pages = ceil(self+0x60 / per)`), and shows or hides the
/// two page arrows (`pages > 1`). Without a format string it sets the plain
/// fallback text.
pub fn c_fe_state_reward_gallery_update_page_text(c: &mut Cpu) {
    if c.pc != 0x001d0bc8 {
        return ee::s04::f_001d0bc8__generated(c);
    }
    guest(c, |c| {
        'b_1d0d18: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x904));
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            if t {
                break 'b_1d0d18;
            } else {
                'b_1d0d04: {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x460000;
                    if t {
                        break 'b_1d0d04;
                    } else {
                        c.cycles += 12;
                        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7060));
                        c.g[S0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x001d0c18);
                        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        call_ptr(c, 0x00317670, 0x001d0c18)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A1][0] = c.g[V0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001d0c28);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, t, 0x001d0c28)?;
                        c.g[A1][0] = c.g[V0][0];
                        let t = c.g[A1][0] == 0u64;
                        c.g[A0][0] = c.g[SP][0];
                        if t {
                            break 'b_1d0d04;
                        } else {
                            'b_1d0c44: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                                if c.g[V0][0] == 0u64 {
                                    break 'b_1d0c44;
                                } else {
                                    break 'b_1d0c44;
                                }
                            }
                            'b_1d0cbc: {
                                c.cycles += 24;
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                c.div(0, 3, 2);
                                c.g[A2][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                c.g[S0][0] = c.lo[0];
                                c.div(0, 6, 2);
                                c.g[A3][0] = c.g[S0][0];
                                c.g[A2][0] = c.lo[0];
                                c.g[RA][0] = sx(0x001d0c74);
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                call_ptr(c, 0x002c26d0, 0x001d0c74)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x904));
                                c.g[RA][0] = sx(0x001d0c80);
                                c.g[A1][0] = c.g[SP][0];
                                call_ptr(c, 0x003a0e90, 0x001d0c80)?;
                                c.g[A0][0] = 0x4a0000;
                                c.g[RA][0] = sx(0x001d0c8c);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ef8));
                                call_ptr(c, 0x00317670, 0x001d0c8c)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001d0c98);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001d0c98)?;
                                c.g[A2][0] = c.g[V0][0];
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                                if t {
                                    break 'b_1d0cbc;
                                } else {
                                    c.cycles += 6;
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = c.g[A1][0] ^ 0x1;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001d0cbc);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001d0cbc)?;
                                    break 'b_1d0cbc;
                                }
                            }
                            c.cycles += 9;
                            c.g[A0][0] = 0x4a0000;
                            c.g[RA][0] = sx(0x001d0cc8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f00));
                            call_ptr(c, 0x00317670, 0x001d0cc8)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001d0cd4);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001d0cd4)?;
                            c.g[A2][0] = c.g[V0][0];
                            if c.g[A2][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                                break 'b_1d0d18;
                            } else {
                                c.cycles += 9;
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
                                c.g[A1][0] = c.g[A1][0] ^ 0x1;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001d0cfc);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001d0cfc)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                                if t {
                                    break 'b_1d0d18;
                                } else {
                                    break 'b_1d0d04;
                                }
                            }
                        }
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x904));
                c.g[RA][0] = sx(0x001d0d14);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                call_ptr(c, 0x003a0c50, 0x001d0d14)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                break 'b_1d0d18;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRewardGallery::OnWidgetCreated(self, UIWidget* w)` (0x001cf820):
/// a widget of the reward gallery screen was created. Compare its name hash
/// (`w+0x38`) with the screen's widget names in order (help text, arrows, select
/// bar, max/num rewards, slash, Menu0000, poster icon, screen title, page text,
/// reward name/cost, you-have-amount, then the cell and frame grids) and store
/// it in the matching state field; titles, icons and texts depend on the
/// reward type (`self+0x4c`, jump tables in the game image).
///
/// Converted from the generated body (a `match pc` body; every register, cycle
/// and call kept). Any other `pc` (a resume) runs the generated body.
pub fn c_fe_state_reward_gallery_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001cf820 {
        return ee::s04::f_001cf820__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001cf820;
        loop {
            match pc {
                0x001cf820 => {
                    c.cycles += 19;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[A0][0] = 0x460000;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001cf864);
                    c.g[S7][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x001cf864)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001cf874;
                        continue;
                    }
                    pc = 0x001cf86c;
                    continue;
                }
                0x001cf86c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x900), c.g[S1][0]);
                    if t {
                        pc = 0x001cf8b8;
                        continue;
                    }
                    pc = 0x001cf874;
                    continue;
                }
                0x001cf874 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf87c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6da0));
                    call_ptr(c, 0x00317670, 0x001cf87c)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001cf88c;
                        continue;
                    }
                    pc = 0x001cf884;
                    continue;
                }
                0x001cf884 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x91c), c.g[S1][0]);
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cf88c;
                    continue;
                }
                0x001cf88c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf894);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6db0));
                    call_ptr(c, 0x00317670, 0x001cf894)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001cf8a4;
                        continue;
                    }
                    pc = 0x001cf89c;
                    continue;
                }
                0x001cf89c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x920), c.g[S1][0]);
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cf8a4;
                    continue;
                }
                0x001cf8a4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf8ac);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6dc0));
                    call_ptr(c, 0x00317670, 0x001cf8ac)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001cf8d8;
                        continue;
                    }
                    pc = 0x001cf8b4;
                    continue;
                }
                0x001cf8b4 => {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8f8), c.g[S1][0]);
                    pc = 0x001cf8b8;
                    continue;
                }
                0x001cf8b8 => {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001cf8d0);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001cf8d0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cf8d8;
                    continue;
                }
                0x001cf8d8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf8e0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6dd8));
                    call_ptr(c, 0x00317670, 0x001cf8e0)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cf908;
                        continue;
                    }
                    pc = 0x001cf8e8;
                    continue;
                }
                0x001cf8e8 => {
                    c.cycles += 8;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x918), c.g[S1][0]);
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                    c.g[RA][0] = sx(0x001cf900);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00417828, 0x001cf900)?;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x918));
                    if t {
                        pc = 0x001cf934;
                        continue;
                    }
                    pc = 0x001cf908;
                    continue;
                }
                0x001cf908 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf910);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6de8));
                    call_ptr(c, 0x00317670, 0x001cf910)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x001cf944;
                        continue;
                    }
                    pc = 0x001cf918;
                    continue;
                }
                0x001cf918 => {
                    c.cycles += 7;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x914), c.g[S1][0]);
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                    c.g[RA][0] = sx(0x001cf930);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00417828, 0x001cf930)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x914));
                    pc = 0x001cf934;
                    continue;
                }
                0x001cf934 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf93c);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x003a0c50, 0x001cf93c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cf944;
                    continue;
                }
                0x001cf944 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf94c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ee8));
                    call_ptr(c, 0x00317670, 0x001cf94c)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cf96c;
                        continue;
                    }
                    pc = 0x001cf954;
                    continue;
                }
                0x001cf954 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001cf964);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ef0));
                    call_ptr(c, 0x003a0c50, 0x001cf964)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cf96c;
                    continue;
                }
                0x001cf96c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf974);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2470));
                    call_ptr(c, 0x00317670, 0x001cf974)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001cf984;
                        continue;
                    }
                    pc = 0x001cf97c;
                    continue;
                }
                0x001cf97c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8f4), c.g[S1][0]);
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cf984;
                    continue;
                }
                0x001cf984 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf98c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6df8));
                    call_ptr(c, 0x00317670, 0x001cf98c)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cfa30;
                        continue;
                    }
                    pc = 0x001cf994;
                    continue;
                }
                0x001cf994 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                    c.g[V1][0] = (c.g[A0][0] < 0x5u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cf9a8;
                    continue;
                }
                0x001cf9a8 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6f40));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001cf9c0 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e08));
                    if t {
                        pc = 0x001cfa0c;
                        continue;
                    }
                    pc = 0x001cf9d0;
                    continue;
                }
                0x001cf9d0 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e18));
                    if t {
                        pc = 0x001cfa0c;
                        continue;
                    }
                    pc = 0x001cf9e0;
                    continue;
                }
                0x001cf9e0 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e30));
                    if t {
                        pc = 0x001cfa0c;
                        continue;
                    }
                    pc = 0x001cf9f0;
                    continue;
                }
                0x001cf9f0 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e40));
                    if t {
                        pc = 0x001cfa0c;
                        continue;
                    }
                    pc = 0x001cfa00;
                    continue;
                }
                0x001cfa00 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e50));
                    pc = 0x001cfa0c;
                    continue;
                }
                0x001cfa0c => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001cfa14);
                    call_ptr(c, 0x00317670, 0x001cfa14)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x001cfa20);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00398380, 0x001cfa20)?;
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x78), c.g[V1][0]);
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cfa30;
                    continue;
                }
                0x001cfa30 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfa38);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e60));
                    call_ptr(c, 0x00317670, 0x001cfa38)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cfab4;
                        continue;
                    }
                    pc = 0x001cfa40;
                    continue;
                }
                0x001cfa40 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                    c.g[V1][0] = (c.g[A0][0] < 0x6u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cfa54;
                    continue;
                }
                0x001cfa54 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6f60));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001cfa6c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e70));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfa78;
                    continue;
                }
                0x001cfa78 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e88));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfa84;
                    continue;
                }
                0x001cfa84 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ea0));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfa90;
                    continue;
                }
                0x001cfa90 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6eb0));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfa9c;
                    continue;
                }
                0x001cfa9c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ec0));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfaa8;
                    continue;
                }
                0x001cfaa8 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ed0));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfab4;
                    continue;
                }
                0x001cfab4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfabc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ee8));
                    call_ptr(c, 0x00317670, 0x001cfabc)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cfad0;
                        continue;
                    }
                    pc = 0x001cfac4;
                    continue;
                }
                0x001cfac4 => {
                    c.cycles += 3;
                    c.g[A1][0] = 0x4a0000;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x904), c.g[S1][0]);
                    if t {
                        pc = 0x001cfbac;
                        continue;
                    }
                    pc = 0x001cfad0;
                    continue;
                }
                0x001cfad0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfad8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ef8));
                    call_ptr(c, 0x00317670, 0x001cfad8)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cfb78;
                        continue;
                    }
                    pc = 0x001cfae0;
                    continue;
                }
                0x001cfae0 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                    c.g[V0][0] = 0x3;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4;
                    if t {
                        pc = 0x001cfafc;
                        continue;
                    }
                    pc = 0x001cfaf0;
                    continue;
                }
                0x001cfaf0 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6210));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfafc;
                    continue;
                }
                0x001cfafc => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x001cfb10;
                        continue;
                    }
                    pc = 0x001cfb04;
                    continue;
                }
                0x001cfb04 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6228));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfb10;
                    continue;
                }
                0x001cfb10 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x6;
                    if t {
                        pc = 0x001cfb24;
                        continue;
                    }
                    pc = 0x001cfb18;
                    continue;
                }
                0x001cfb18 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6240));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfb24;
                    continue;
                }
                0x001cfb24 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x7;
                    if t {
                        pc = 0x001cfb38;
                        continue;
                    }
                    pc = 0x001cfb2c;
                    continue;
                }
                0x001cfb2c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6258));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfb38;
                    continue;
                }
                0x001cfb38 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x8;
                    if t {
                        pc = 0x001cfb4c;
                        continue;
                    }
                    pc = 0x001cfb40;
                    continue;
                }
                0x001cfb40 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6270));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfb4c;
                    continue;
                }
                0x001cfb4c => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cfb54;
                    continue;
                }
                0x001cfb54 => {
                    c.cycles += 2;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6288));
                    pc = 0x001cfb5c;
                    continue;
                }
                0x001cfb5c => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x001cfb64);
                    call_ptr(c, 0x00317670, 0x001cfb64)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001cfb70);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001cfb70)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cfb78;
                    continue;
                }
                0x001cfb78 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfb80);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6f10));
                    call_ptr(c, 0x00317670, 0x001cfb80)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cfb94;
                        continue;
                    }
                    pc = 0x001cfb88;
                    continue;
                }
                0x001cfb88 => {
                    c.cycles += 3;
                    c.g[A1][0] = 0x4a0000;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x908), c.g[S1][0]);
                    if t {
                        pc = 0x001cfbac;
                        continue;
                    }
                    pc = 0x001cfb94;
                    continue;
                }
                0x001cfb94 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfb9c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6f20));
                    call_ptr(c, 0x00317670, 0x001cfb9c)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cfbc0;
                        continue;
                    }
                    pc = 0x001cfba4;
                    continue;
                }
                0x001cfba4 => {
                    c.cycles += 2;
                    c.g[A1][0] = 0x4a0000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x90c), c.g[S1][0]);
                    pc = 0x001cfbac;
                    continue;
                }
                0x001cfbac => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001cfbb8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    call_ptr(c, 0x003a0c50, 0x001cfbb8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cfbc0;
                    continue;
                }
                0x001cfbbc => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    pc = 0x001cfbc0;
                    continue;
                }
                0x001cfbc0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfbc8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6f30));
                    call_ptr(c, 0x00317670, 0x001cfbc8)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                        pc = 0x001cfbf0;
                        continue;
                    }
                    pc = 0x001cfbd0;
                    continue;
                }
                0x001cfbd0 => {
                    c.cycles += 8;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x910), c.g[S1][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x158));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x15c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001cfbe8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001cfbe8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cfbf0;
                    continue;
                }
                0x001cfbf0 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[FP][0] = 0u64;
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cfbf8;
                    continue;
                }
                0x001cfbf8 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    pc = 0x001cfc00;
                    continue;
                }
                0x001cfc00 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1));
                    c.g[S2][0] = 0u64;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                    if t {
                        pc = 0x001cfcd0;
                        continue;
                    }
                    pc = 0x001cfc10;
                    continue;
                }
                0x001cfc10 => {
                    c.cycles += 12;
                    c.g[V1][0] = sx((c.g[FP][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x924));
                    c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = 0x440000;
                    c.g[V0][0] = 0x440000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14c0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1440));
                    c.g[S5][0] = c.g[V1][0];
                    c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9a4));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    pc = 0x001cfc40;
                    continue;
                }
                0x001cfc40 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfc48);
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    call_ptr(c, 0x00317670, 0x001cfc48)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001cfc7c;
                        continue;
                    }
                    pc = 0x001cfc50;
                    continue;
                }
                0x001cfc50 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.mult(0, 4, 30, 2);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001cfc74);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001cfc74)?;
                    let t = true;
                    c.sw(c.g[S6][0] as u32, c.g[S1][0]);
                    if t {
                        pc = 0x001cfcb0;
                        continue;
                    }
                    pc = 0x001cfc7c;
                    continue;
                }
                0x001cfc7c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfc84);
                    c.g[A0][0] = c.lw(c.g[S4][0] as u32);
                    call_ptr(c, 0x00317670, 0x001cfc84)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        pc = 0x001cfcb4;
                        continue;
                    }
                    pc = 0x001cfc8c;
                    continue;
                }
                0x001cfc8c => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0u64;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.sw(c.g[V1][0] as u32, c.g[S1][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001cfcb0);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001cfcb0)?;
                    pc = 0x001cfcb0;
                    continue;
                }
                0x001cfcb0 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    pc = 0x001cfcb4;
                    continue;
                }
                0x001cfcb4 => {
                    c.cycles += 7;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001cfc40) {
                            return Ok(());
                        }
                        pc = 0x001cfc40;
                        continue;
                    }
                    pc = 0x001cfcd0;
                    continue;
                }
                0x001cfcd0 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[FP][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = ((c.g[FP][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        if back_edge_unwinds(c, 0x001cfc00) {
                            return Ok(());
                        }
                        pc = 0x001cfc00;
                        continue;
                    }
                    pc = 0x001cfce4;
                    continue;
                }
                0x001cfce4 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    pc = 0x001cfce8;
                    continue;
                }
                0x001cfce8 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001cf820, 0x001cfd14);
                    return Ok(());
                }
            }
        }
    });
}

/// `void cFEStateRewardGallery::UpdateHelpText(self, int idx)` (0x001d0648):
/// hide the help text widget (`self+0x900`), look up reward `idx` through the
/// state's vtable (item, its state, whether it is owned) and show the help text
/// that fits: not owned, wrong mode, can afford or not, etc. Fetches the text by
/// string hash and sets it on the widget.
///
/// Converted from the generated labelled-block body (every register, cycle and
/// call kept). Any other `pc` (a resume) runs the generated body.
pub fn c_fe_state_reward_gallery_update_help_text(c: &mut Cpu) {
    if c.pc != 0x001d0648 {
        return ee::s04::f_001d0648__generated(c);
    }
    guest(c, |c| {
        'b_1d0874: {
            'b_1d0870: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x900));
                let t = c.g[A2][0] == 0u64;
                c.g[S3][0] = c.g[A1][0];
                if t {
                    break 'b_1d0870;
                } else {
                    c.cycles += 15;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d0688);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001d0688)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x110));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x114));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001d06a0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001d06a0)?;
                    c.g[S2][0] = c.g[V0][0];
                    if c.g[S2][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        break 'b_1d0874;
                    } else {
                        'b_1d085c: {
                            'b_1d0840: {
                                'b_1d0838: {
                                    'b_1d07a0: {
                                        c.cycles += 18;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = c.g[S2][0];
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x120));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001d06c4);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001d06c4)?;
                                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x900));
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x1;
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001d06e4);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001d06e4)?;
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                                        c.g[V0][0] = 0x8;
                                        let t = c.g[A0][0] != c.g[V0][0];
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_1d07a0;
                                        } else {
                                            'b_1d0718: {
                                                c.cycles += 7;
                                                c.g[A1][0] = c.g[S3][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x130));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x134));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001d0708);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x001d0708)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_1d0718;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x6f90));
                                                    if t {
                                                        break 'b_1d0840;
                                                    } else {
                                                        break 'b_1d0718;
                                                    }
                                                }
                                            }
                                            'b_1d0728: {
                                                c.cycles += 2;
                                                if (c.g[S1][0] as i64) >= 0 {
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x8f0)
                                                        );
                                                    break 'b_1d0728;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                    if t {
                                                        break 'b_1d0840;
                                                    } else {
                                                        break 'b_1d0728;
                                                    }
                                                }
                                            }
                                            'b_1d0798: {
                                                c.cycles += 2;
                                                let t = (c.g[V0][0] as i64) < 0;
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_1d0798;
                                                } else {
                                                    'b_1d0768: {
                                                        c.cycles += 2;
                                                        let t = c.g[S1][0] == c.g[V0][0];
                                                        c.g[V0][0] = 0x1;
                                                        if t {
                                                            break 'b_1d0768;
                                                        } else {
                                                            'b_1d074c: {
                                                                c.cycles += 2;
                                                                let t = c.g[S1][0] != c.g[V0][0];
                                                                c.g[V0][0] = 0x2;
                                                                if t {
                                                                    break 'b_1d074c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A0][0] = 0x460000;
                                                                    let t = true;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x6fb0));
                                                                    if t {
                                                                        break 'b_1d0840;
                                                                    } else {
                                                                        break 'b_1d074c;
                                                                    }
                                                                }
                                                            }
                                                            'b_1d0760: {
                                                                c.cycles += 2;
                                                                if c.g[S1][0] != c.g[V0][0] {
                                                                    c.g[A0][0] = 0x460000;
                                                                    break 'b_1d0760;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A0][0] = 0x460000;
                                                                    let t = true;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x6fc8));
                                                                    if t {
                                                                        break 'b_1d0840;
                                                                    } else {
                                                                        break 'b_1d0760;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x6fe0));
                                                            if t {
                                                                break 'b_1d0840;
                                                            } else {
                                                                break 'b_1d0768;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 10;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = c.g[S2][0];
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V1][0] as u32).wrapping_add(0x118)
                                                        );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x11c)
                                                        );
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[RA][0] = sx(0x001d0780);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001d0780)?;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x54));
                                                    c.g[V1][0] = ((c.g[V1][0] as i64)
                                                        < (c.g[V0][0] as i64))
                                                        as u64;
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[A0][0] = 0x460000;
                                                    if t {
                                                        break 'b_1d0838;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x958));
                                                        if t {
                                                            break 'b_1d0840;
                                                        } else {
                                                            break 'b_1d0798;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x7010));
                                            if t {
                                                break 'b_1d0840;
                                            } else {
                                                break 'b_1d07a0;
                                            }
                                        }
                                    }
                                    'b_1d07c4: {
                                        c.cycles += 7;
                                        c.g[A1][0] = c.g[S3][0];
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x130));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x134));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001d07b4);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001d07b4)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = 0x460000;
                                        if t {
                                            break 'b_1d07c4;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x7028));
                                            if t {
                                                break 'b_1d0840;
                                            } else {
                                                break 'b_1d07c4;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8f0));
                                    let t = (c.g[V0][0] as i64) < 0;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1d085c;
                                    } else {
                                        'b_1d0808: {
                                            c.cycles += 2;
                                            let t = c.g[S1][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_1d0808;
                                            } else {
                                                'b_1d07ec: {
                                                    c.cycles += 2;
                                                    let t = c.g[S1][0] != c.g[V0][0];
                                                    c.g[V0][0] = 0x2;
                                                    if t {
                                                        break 'b_1d07ec;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = 0x460000;
                                                        let t = true;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x6fb0));
                                                        if t {
                                                            break 'b_1d0840;
                                                        } else {
                                                            break 'b_1d07ec;
                                                        }
                                                    }
                                                }
                                                'b_1d0800: {
                                                    c.cycles += 2;
                                                    if c.g[S1][0] != c.g[V0][0] {
                                                        c.g[A0][0] = 0x460000;
                                                        break 'b_1d0800;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = 0x460000;
                                                        let t = true;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x6fc8));
                                                        if t {
                                                            break 'b_1d0840;
                                                        } else {
                                                            break 'b_1d0800;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x6fe0));
                                                if t {
                                                    break 'b_1d0840;
                                                } else {
                                                    break 'b_1d0808;
                                                }
                                            }
                                        }
                                        c.cycles += 10;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = c.g[S2][0];
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x118));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x11c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001d0820);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001d0820)?;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                                        c.g[V1][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[A0][0] = 0x460000;
                                        if t {
                                            break 'b_1d0838;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x958));
                                            if t {
                                                break 'b_1d0840;
                                            } else {
                                                break 'b_1d0838;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 0x460000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ff8));
                                break 'b_1d0840;
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001d0848);
                            call_ptr(c, 0x00317670, 0x001d0848)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x900));
                            c.g[RA][0] = sx(0x001d0854);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001d0854)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_1d0874;
                            } else {
                                break 'b_1d085c;
                            }
                        }
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001d0864);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7010));
                        call_ptr(c, 0x00317670, 0x001d0864)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x900));
                        c.g[RA][0] = sx(0x001d0870);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001d0870)?;
                        break 'b_1d0870;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1d0874;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRewardGallery::RefreshRowIcons(self, int row)` (0x001d0890):
/// for each column of `row` show or hide the cell and frame widgets
/// (`self+0x924`, `+0x9a4`) depending on whether the reward index
/// `(scroll + row) * cols + col` is below the count, and set the cell's texture
/// from the reward (vtable queries) or the locked/selected icon.
///
/// Converted from the generated labelled-block body (every register, cycle and
/// call kept). Any other `pc` (a resume) runs the generated body.
pub fn c_fe_state_reward_gallery_refresh_row_icons(c: &mut Cpu) {
    if c.pc != 0x001d0890 {
        return ee::s04::f_001d0890__generated(c);
    }
    guest(c, |c| {
        'b_1d0ad0: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S1][0] = c.g[A0][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S7][0] = 0u64;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[FP][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[FP][0] as u32));
            let t = (c.g[V1][0] as i64) <= 0;
            c.mult(0, 19, 2, 3);
            if t {
                break 'b_1d0ad0;
            } else {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x924));
                c.g[V0][0] = sx((c.g[FP][0] as u32) << 5);
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S6][0] = 0u64;
                'l_1d08f0: loop {
                    'b_1d0ab8: {
                        'b_1d0a7c: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_1d0a7c;
                            } else {
                                'b_1d0a50: {
                                    'b_1d0a4c: {
                                        'b_1d09c0: {
                                            c.cycles += 23;
                                            c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                                            c.g[S4][0] = sx((c.g[FP][0] as u32) << 5);
                                            c.g[S5][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x924));
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001d0920);
                                            c.g[A0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001d0920)?;
                                            c.g[A2][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x80));
                                            c.g[A1][0] = 0x1;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001d093c);
                                            c.g[A0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001d093c)?;
                                            c.g[A2][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[A0][0] =
                                                c.lh((c.g[A2][0] as u32).wrapping_add(0x130));
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x134));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001d0954);
                                            c.g[A0][0] =
                                                sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001d0954)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = c.g[S3][0];
                                            if t {
                                                break 'b_1d09c0;
                                            } else {
                                                'b_1d098c: {
                                                    c.cycles += 8;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0xd0));
                                                    c.g[V0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0xd4));
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[RA][0] = sx(0x001d0970);
                                                    c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001d0970)?;
                                                    c.g[V1][0] = c.g[V0][0];
                                                    if (c.g[V1][0] as i64) < 0 {
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                        break 'b_1d098c;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                                        c.sw(
                                                            (c.g[V0][0] as u32).wrapping_add(0x78),
                                                            c.g[V1][0],
                                                        );
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                            0u64,
                                                        );
                                                        if t {
                                                            break 'b_1d0a4c;
                                                        } else {
                                                            break 'b_1d098c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 8;
                                                c.g[A1][0] = c.g[S3][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0xd8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xdc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001d09a0);
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x001d09a0)?;
                                                c.g[V1][0] = c.g[V0][0];
                                                let t = c.g[V1][0] == 0u64;
                                                c.g[V0][0] = sx((c.g[S6][0] as u32)
                                                    .wrapping_add(c.g[S4][0] as u32));
                                                if t {
                                                    break 'b_1d0a50;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[V1][0] = 0xffffffffffffffff;
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x78),
                                                        c.g[V1][0],
                                                    );
                                                    if t {
                                                        break 'b_1d0a4c;
                                                    } else {
                                                        break 'b_1d09c0;
                                                    }
                                                }
                                            }
                                        }
                                        'b_1d0a1c: {
                                            'b_1d0a14: {
                                                c.cycles += 9;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x110));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x114));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001d09d4);
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x001d09d4)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8f0));
                                                if (c.g[V0][0] as i64) < 0 {
                                                    c.g[A0][0] = 0x460000;
                                                    break 'b_1d0a14;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V1][0] as u32).wrapping_add(0x120)
                                                        );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x124)
                                                        );
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[RA][0] = sx(0x001d09f8);
                                                    c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001d09f8)?;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x8f0)
                                                        );
                                                    if c.g[V0][0] != c.g[V1][0] {
                                                        c.g[A0][0] = 0x460000;
                                                        break 'b_1d0a14;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[A0][0] = 0x460000;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                        let t = true;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x7040));
                                                        if t {
                                                            break 'b_1d0a1c;
                                                        } else {
                                                            break 'b_1d0a14;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[S0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x7050));
                                            break 'b_1d0a1c;
                                        }
                                        c.cycles += 12;
                                        c.g[RA][0] = sx(0x001d0a24);
                                        call_ptr(c, 0x00317670, 0x001d0a24)?;
                                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                                        c.g[RA][0] = sx(0x001d0a30);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x00398380, 0x001d0a30)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[V0][0] =
                                            sx((c.g[S6][0] as u32).wrapping_add(c.g[S4][0] as u32));
                                        c.g[V0][0] =
                                            sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), c.g[A0][0]);
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                                        break 'b_1d0a4c;
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] =
                                        sx((c.g[S6][0] as u32).wrapping_add(c.g[S4][0] as u32));
                                    break 'b_1d0a50;
                                }
                                c.cycles += 11;
                                c.g[V1][0] = 0x4c0000;
                                c.g[V0][0] =
                                    sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6818));
                                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001d0a74);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001d0a74)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                                if t {
                                    break 'b_1d0ab8;
                                } else {
                                    break 'b_1d0a7c;
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001d0a98);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001d0a98)?;
                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x80));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001d0ab4);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001d0ab4)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                        break 'b_1d0ab8;
                    }
                    c.cycles += 6;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001d08f0);
                        }
                        continue 'l_1d08f0;
                    } else {
                        break 'b_1d0ad0;
                    }
                }
            }
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStatePreviewReward::OnWidgetCreated(self, UIWidget* w)` (0x001d1c98):
/// null `w` returns at once. Otherwise match its name hash (`w+0x38`): the icon
/// texture or title text by reward type (`self+0x68`), the reward name text,
/// the three frame shapes (hidden, `self+0x54..`) and bitmaps (`self+0x48..`).
/// Afterwards show the frame of the selected shape (`self+0x74`).
///
/// Converted from the generated body (a `match pc` body; every register, cycle
/// and call kept). Any other `pc` (a resume) runs the generated body.
pub fn c_fe_state_preview_reward_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001d1c98 {
        return ee::s04::f_001d1c98__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001d1c98;
        loop {
            match pc {
                0x001d1c98 => {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A0][0];
                    let t = c.g[S1][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        pc = 0x001d1f5c;
                        continue;
                    }
                    pc = 0x001d1cb8;
                    continue;
                }
                0x001d1cb8 => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001d1cc8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6df8));
                    call_ptr(c, 0x00317670, 0x001d1cc8)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1d6c;
                        continue;
                    }
                    pc = 0x001d1cd0;
                    continue;
                }
                0x001d1cd0 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                    c.g[V0][0] = 0x4;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 5i64) as u64;
                    if t {
                        pc = 0x001d1d20;
                        continue;
                    }
                    pc = 0x001d1ce0;
                    continue;
                }
                0x001d1ce0 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x001d1cf8;
                        continue;
                    }
                    pc = 0x001d1ce8;
                    continue;
                }
                0x001d1ce8 => {
                    c.cycles += 2;

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1d14;
                        continue;
                    }
                    pc = 0x001d1cf0;
                    continue;
                }
                0x001d1cf0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    if t {
                        pc = 0x001d1f04;
                        continue;
                    }
                    pc = 0x001d1cf8;
                    continue;
                }
                0x001d1cf8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x5;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x7;
                    if t {
                        pc = 0x001d1d30;
                        continue;
                    }
                    pc = 0x001d1d04;
                    continue;
                }
                0x001d1d04 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001d1d40;
                        continue;
                    }
                    pc = 0x001d1d0c;
                    continue;
                }
                0x001d1d0c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    if t {
                        pc = 0x001d1f04;
                        continue;
                    }
                    pc = 0x001d1d14;
                    continue;
                }
                0x001d1d14 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e08));
                    if t {
                        pc = 0x001d1d48;
                        continue;
                    }
                    pc = 0x001d1d20;
                    continue;
                }
                0x001d1d20 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e18));
                    if t {
                        pc = 0x001d1d48;
                        continue;
                    }
                    pc = 0x001d1d30;
                    continue;
                }
                0x001d1d30 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e30));
                    if t {
                        pc = 0x001d1d48;
                        continue;
                    }
                    pc = 0x001d1d40;
                    continue;
                }
                0x001d1d40 => {
                    c.cycles += 2;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e50));
                    pc = 0x001d1d48;
                    continue;
                }
                0x001d1d48 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001d1d50);
                    call_ptr(c, 0x00317670, 0x001d1d50)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x001d1d5c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00398380, 0x001d1d5c)?;
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x78), c.g[V1][0]);
                    if t {
                        pc = 0x001d1f00;
                        continue;
                    }
                    pc = 0x001d1d6c;
                    continue;
                }
                0x001d1d6c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1d74);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e60));
                    call_ptr(c, 0x00317670, 0x001d1d74)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001d1dfc;
                        continue;
                    }
                    pc = 0x001d1d7c;
                    continue;
                }
                0x001d1d7c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                    c.g[V1][0] = (c.g[A0][0] < 0x5u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001d1f00;
                        continue;
                    }
                    pc = 0x001d1d90;
                    continue;
                }
                0x001d1d90 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x71d0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001d1da8 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e70));
                    if t {
                        pc = 0x001d1de0;
                        continue;
                    }
                    pc = 0x001d1db4;
                    continue;
                }
                0x001d1db4 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e88));
                    if t {
                        pc = 0x001d1de0;
                        continue;
                    }
                    pc = 0x001d1dc0;
                    continue;
                }
                0x001d1dc0 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ea0));
                    if t {
                        pc = 0x001d1de0;
                        continue;
                    }
                    pc = 0x001d1dcc;
                    continue;
                }
                0x001d1dcc => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7150));
                    if t {
                        pc = 0x001d1de0;
                        continue;
                    }
                    pc = 0x001d1dd8;
                    continue;
                }
                0x001d1dd8 => {
                    c.cycles += 2;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ec0));
                    pc = 0x001d1de0;
                    continue;
                }
                0x001d1de0 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x001d1de8);
                    call_ptr(c, 0x00317670, 0x001d1de8)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001d1df4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001d1df4)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    if t {
                        pc = 0x001d1f04;
                        continue;
                    }
                    pc = 0x001d1dfc;
                    continue;
                }
                0x001d1dfc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1e04);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6f10));
                    call_ptr(c, 0x00317670, 0x001d1e04)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001d1e5c;
                        continue;
                    }
                    pc = 0x001d1e0c;
                    continue;
                }
                0x001d1e0c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                    c.g[V1][0] = (c.g[A0][0] < 0x5u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001d1f00;
                        continue;
                    }
                    pc = 0x001d1e20;
                    continue;
                }
                0x001d1e20 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x71f0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001d1e38 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001d1ea8;
                        continue;
                    }
                    pc = 0x001d1e44;
                    continue;
                }
                0x001d1e44 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001d1e54);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    call_ptr(c, 0x003a0c50, 0x001d1e54)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    if t {
                        pc = 0x001d1f04;
                        continue;
                    }
                    pc = 0x001d1e5c;
                    continue;
                }
                0x001d1e5c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1e64);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7160));
                    call_ptr(c, 0x00317670, 0x001d1e64)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1e74;
                        continue;
                    }
                    pc = 0x001d1e6c;
                    continue;
                }
                0x001d1e6c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x54), c.g[S1][0]);
                    if t {
                        pc = 0x001d1ea0;
                        continue;
                    }
                    pc = 0x001d1e74;
                    continue;
                }
                0x001d1e74 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1e7c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7170));
                    call_ptr(c, 0x00317670, 0x001d1e7c)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1e8c;
                        continue;
                    }
                    pc = 0x001d1e84;
                    continue;
                }
                0x001d1e84 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x58), c.g[S1][0]);
                    if t {
                        pc = 0x001d1ea0;
                        continue;
                    }
                    pc = 0x001d1e8c;
                    continue;
                }
                0x001d1e8c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1e94);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7188));
                    call_ptr(c, 0x00317670, 0x001d1e94)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1ec0;
                        continue;
                    }
                    pc = 0x001d1e9c;
                    continue;
                }
                0x001d1e9c => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x5c), c.g[S1][0]);
                    pc = 0x001d1ea0;
                    continue;
                }
                0x001d1ea0 => {
                    c.cycles += 2;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    pc = 0x001d1ea8;
                    continue;
                }
                0x001d1ea8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d1eb8);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001d1eb8)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    if t {
                        pc = 0x001d1f04;
                        continue;
                    }
                    pc = 0x001d1ec0;
                    continue;
                }
                0x001d1ec0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1ec8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7198));
                    call_ptr(c, 0x00317670, 0x001d1ec8)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1ed8;
                        continue;
                    }
                    pc = 0x001d1ed0;
                    continue;
                }
                0x001d1ed0 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x48), c.g[S1][0]);
                    if t {
                        pc = 0x001d1f00;
                        continue;
                    }
                    pc = 0x001d1ed8;
                    continue;
                }
                0x001d1ed8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1ee0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x71a8));
                    call_ptr(c, 0x00317670, 0x001d1ee0)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1ef0;
                        continue;
                    }
                    pc = 0x001d1ee8;
                    continue;
                }
                0x001d1ee8 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[S1][0]);
                    if t {
                        pc = 0x001d1f00;
                        continue;
                    }
                    pc = 0x001d1ef0;
                    continue;
                }
                0x001d1ef0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1ef8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x71c0));
                    call_ptr(c, 0x00317670, 0x001d1ef8)?;

                    if c.g[S0][0] == c.g[V0][0] {
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x50), c.g[S1][0]);
                        pc = 0x001d1f00;
                        continue;
                    }
                    pc = 0x001d1f00;
                    continue;
                }
                0x001d1f00 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    pc = 0x001d1f04;
                    continue;
                }
                0x001d1f04 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x001d1f5c;
                        continue;
                    }
                    pc = 0x001d1f0c;
                    continue;
                }
                0x001d1f0c => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                    let t = c.g[A0][0] == 0u64;
                    c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x54));
                    if t {
                        pc = 0x001d1f5c;
                        continue;
                    }
                    pc = 0x001d1f1c;
                    continue;
                }
                0x001d1f1c => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[V1][0] == 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        pc = 0x001d1f5c;
                        continue;
                    }
                    pc = 0x001d1f2c;
                    continue;
                }
                0x001d1f2c => {
                    c.cycles += 12;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x70));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x7c), 0u64);
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x78), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001d1f5c);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001d1f5c)?;
                    pc = 0x001d1f5c;
                    continue;
                }
                0x001d1f5c => {
                    c.cycles += 6;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001d1c98, 0x001d1f74);
                    return Ok(());
                }
            }
        }
    });
}

/// `cFEStateRewardGallery::FindTextureByName(this, name)` (0x1cfdd0): the
/// texture word of the first slot whose inline name matches, or -1. The
/// slots are 12-byte records from `this+0x7c` (name, 8 bytes; texture word at
/// +8) and the count is at `this+0x78`.
pub fn reward_gallery_find_texture_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let f = Frame::open(c, 0x60);
        f.save(c, 18, 0x30);
        f.save(c, 19, 0x20);
        c.g[S2][0] = 0;
        f.save(c, 20, 0x10);
        c.g[S3][0] = c.g[A0][0];
        f.save(c, 16, 0x50);
        f.save(c, 17, 0x40);
        f.save_ra(c, 0);
        let this = c.g[S3][0] as u32;
        c.g[V0][0] = Addr(this.wrapping_add(0x78)).lw(c);
        c.g[S4][0] = c.g[A1][0];
        'done: {
            if (c.g[V0][0] as i64) <= 0 {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
                break 'done;
            }
            c.cycles += 2;
            c.g[S1][0] = sx(this.wrapping_add(0x84));
            c.g[S0][0] = sx(this.wrapping_add(0x7c));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S4][0];
                call_ptr(c, 0x0041_65a8, 0x001c_fe14)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    break 'done;
                }
                c.g[V0][0] = Addr(this.wrapping_add(0x78)).lw(c);
                c.cycles += 5;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = u64::MAX;
                    break 'done;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001c_fe08);
                }
            }
        }
        c.cycles += 8;
        f.restore(c, 16, 0x50);
        f.restore(c, 17, 0x40);
        f.restore(c, 18, 0x30);
        f.restore(c, 19, 0x20);
        f.restore(c, 20, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `cFEStateRewardGallery::RequestShapeLoad(this)` (0x1cfe60): asks the
/// shape loader for the current item's shape, or, when the loader answers
/// with a handle, queues the load and records its state in `this+0x48`
/// (bits 2 and 3 set, bit 0 cleared; the handle at +0x8ec).
pub fn reward_gallery_request_shape_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let f = Frame::open(c, 0x30);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        f.save(c, 16, 0x20);
        f.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        f.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.g[A1][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xc8)).lh(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xcc)).lw(c);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a70));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001c_fe94)?;
        let loaded = c.g[V0][0] != 0;
        if !loaded {
            c.g[V0][0] = Addr(this.wrapping_add(0x48)).lw(c);
        } else {
            c.cycles += 19;
            c.g[V1][0] = Addr(this.wrapping_add(0x8)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc8)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xcc)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001c_feb0);
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_feb0)?;
            c.g[A1][0] = Addr(this.wrapping_add(0x50)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A3][0] = 0x9;
            c.g[T0][0] = 0x1;
            c.g[T1][0] = 0;
            call_ptr(c, 0x0019_da10, 0x001c_fecc)?;
            c.g[V1][0] = Addr(this.wrapping_add(0x48)).lw(c);
            c.g[A0][0] = 0xffff_ffff_ffff_fff7;
            let handle = c.g[V0][0];
            Addr(this.wrapping_add(0x8ec)).sw(c, handle);
            c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
            c.g[V1][0] = c.g[V1][0] | 0x4;
            let state = c.g[V1][0];
            Addr(this.wrapping_add(0x48)).sw(c, state);
            return finish_request_shape_load(c, f);
        }
        c.cycles += 4;
        c.g[V1][0] = 0xffff_ffff_ffff_fffb;
        c.g[V0][0] = c.g[V0][0] | 0x8;
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        let state = c.g[V0][0];
        Addr(this.wrapping_add(0x48)).sw(c, state);
        finish_request_shape_load(c, f)
    })
}

/// The epilogue of `RequestShapeLoad`.
pub(crate) fn finish_request_shape_load(c: &mut Cpu, f: Frame) -> Flow {
    c.cycles += 5;
    f.restore(c, 16, 0x20);
    f.restore(c, 17, 0x10);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `cFEStateRewardGallery::ReleaseTextures(this)` (0x1d00c0): releases every
/// texture slot (the loaded word at `+0x84 + 12*i` is cleared to -1 and the
/// slot's name byte is zeroed), then the shape handle at +0x8ec, and drops
/// the bit-3 flag of `this+0x48`.
pub fn reward_gallery_release_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let f = Frame::open(c, 0x50);
        f.save(c, 17, 0x30);
        f.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        f.save(c, 16, 0x40);
        f.save(c, 19, 0x10);
        f.save_ra(c, 0);
        let this = c.g[S1][0] as u32;
        'b_1d012c: {
            c.g[V0][0] = Addr(this.wrapping_add(0x78)).lw(c);
            let empty = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = 0;
            if empty {
                break 'b_1d012c;
            }
            c.cycles += 2;
            c.g[S3][0] = u64::MAX;
            c.g[S0][0] = sx(this.wrapping_add(0x84));
            loop {
                'b_1d0110: {
                    c.cycles += 3;
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                    let negative = (c.g[A1][0] as i64) < 0;
                    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
                    if negative {
                        break 'b_1d0110;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x10d8)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x190)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x194)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001d_0110)?;
                    break 'b_1d0110;
                }
                c.cycles += 7;
                let s0 = c.g[S0][0] as u32;
                let slot = c.g[S3][0];
                Addr(s0).sw(c, slot);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                Addr(s0.wrapping_sub(0x8)).sb(c, 0);
                c.g[V0][0] = Addr(this.wrapping_add(0x78)).lw(c);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                if !more {
                    break 'b_1d012c;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001d_00f0);
                }
            }
        }
        {
            c.cycles += 3;
            c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x8ec).lw(c);
            let pending = (c.g[A2][0] as i64) < 0;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            if !pending {
                c.cycles += 6;
                c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x50).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
                c.g[RA][0] = sx(0x001d_0148);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
                call_ptr(c, 0x0019_dc20, 0x001d_0148)?;
                c.g[V0][0] = u64::MAX;
                Addr(c.g[S1][0] as u32).off(0x8ec).sw(c, u64::MAX);
            }
        }
        c.cycles += 11;
        let this = c.g[S1][0] as u32;
        c.g[V0][0] = Addr(this.wrapping_add(0x48)).lw(c);
        c.g[V1][0] = 0xffff_ffff_ffff_fff7;
        f.restore(c, 16, 0x40);
        c.g[V0][0] &= c.g[V1][0];
        f.restore(c, 18, 0x20);
        let flags = c.g[V0][0];
        Addr(this.wrapping_add(0x48)).sw(c, flags);
        f.restore(c, 17, 0x30);
        f.restore(c, 19, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `cFEStateRewardGallery::InitPages(this)` (0x1d0180): (re)creates the
/// page layout; when the layout call answers non-zero, the pages are set up
/// (bit 1 set, bit 0 cleared) and the page count is divided into `+0x60`.
pub fn reward_gallery_init_pages(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let f = Frame::open(c, 0x30);
        f.save(c, 16, 0x20);
        f.save(c, 17, 0x10);
        f.save_ra(c, 0);
        c.g[RA][0] = sx(0x001d_0198);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        call_ptr(c, 0x0014_dc80, 0x001d_0198)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d_01a4);
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x001d_01a4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d_01b0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0015_6a38, 0x001d_01b0)?;
        let laid_out = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(this.wrapping_add(0x48)).lw(c);
        if !laid_out {
            c.cycles += 8;
            c.g[V1][0] = 0xffff_ffff_ffff_fffd;
            c.g[A0][0] = c.g[S1][0];
            c.g[V0][0] &= c.g[V1][0];
            c.g[V0][0] |= 0x1;
            c.g[RA][0] = sx(0x001d_01d0);
            let state = c.g[V0][0];
            Addr(this.wrapping_add(0x48)).sw(c, state);
            call_ptr(c, 0x0015_6a10, 0x001d_01d0)?;
            f.restore(c, 16, 0x20);
            return init_pages_epilogue(c, f);
        }
        // The layout call answered non-zero: second setup, then the divide.
        c.cycles += 13;
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        c.g[A1][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= 0x2;
        let state = c.g[V0][0];
        Addr(this.wrapping_add(0x48)).sw(c, state);
        c.g[A0][0] = Addr(c.g[A1][0] as u32).off(0xe0).lh(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xe4)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x001d_01fc);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_01fc)?;
        c.g[V1][0] = Addr(this.wrapping_add(0x5c)).lw(c);
        c.g[V0][0] = Addr(this.wrapping_add(0x68)).lw(c);
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.div(0, V0, V1);
        c.g[V0][0] = c.lo[0];
        Addr(this.wrapping_add(0x60)).sw(c, c.g[V0][0]);
        f.restore(c, 16, 0x20);
        init_pages_epilogue(c, f)
    })
}

/// The epilogue of `InitPages` (the `s0` pair is restored by the caller's
/// path already: only `s1` / the return address remain).
pub(crate) fn init_pages_epilogue(c: &mut Cpu, f: Frame) -> Flow {
    c.cycles += 4;
    f.restore(c, 17, 0x10);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `cFEStateRewardGallery::PurchaseItem(this, item)` (0x1d0320): refreshes
/// the reward's labels, prices and icons through the menu helpers, with the
/// item index in `a1`.
pub fn reward_gallery_purchase_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 53;
        let f = Frame::open(c, 0x30);
        f.save(c, 16, 0x20);
        f.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        f.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.g[RA][0] = sx(0x001d_033c);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0014_dc80, 0x001d_033c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d_0348);
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x001d_0348)?;
        c.g[A3][0] = Addr(this.wrapping_add(0x4c)).lw(c);
        c.g[T0][0] = c.g[S1][0];
        c.g[A2][0] = Addr(this.wrapping_add(0x50)).lw(c);
        c.g[T1][0] = 0x1;
        c.g[A1][0] = Addr(this.wrapping_add(0x44)).lb(c);
        c.g[RA][0] = sx(0x001d_0364);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0015_7f20, 0x001d_0364)?;
        c.g[RA][0] = sx(0x001d_036c);
        call_ptr(c, 0x0014_dc80, 0x001d_036c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d_0378);
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x001d_0378)?;
        c.g[A2][0] = Addr(this.wrapping_add(0x50)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001d_0388);
        c.g[A1][0] = Addr(this.wrapping_add(0x44)).lb(c);
        call_ptr(c, 0x0015_0928, 0x001d_0388)?;
        Addr(this.wrapping_add(0x54)).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x150)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x154)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001d_03a0);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_03a0)?;
        c.g[V0][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x108)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001d_03b8);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_03b8)?;
        c.g[V0][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x158)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x15c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001d_03cc);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_03cc)?;
        c.g[V0][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x168)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x16c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001d_03e0);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_03e0)?;
        f.restore(c, 16, 0x20);
        f.restore(c, 17, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `s0` (the gallery object), read from the register each time: a callee
/// can change it, so no Rust copy is kept across a call.
#[inline]
pub(crate) fn reward_gallery_self(c: &Cpu) -> u32 {
    r32(c, S0)
}

// ---------------------------------------------------------------------------
// cFEStateRewardGallery_ViewItem (0x1d03f8).

/// `void cFEStateRewardGallery_ViewItem(cFEStateRewardGallery* self)`
/// (0x1d03f8, `cFEStateRewardGalleryBase::showReward`): open the viewer of the
/// selected reward. Types 3, 4, 5 and 7 (posters, cards, art, toys) get a
/// `cFEStatePreviewReward`, type 6 a `cFEStateVideoPlayer` (640 bytes); any
/// other type opens nothing. A new state is pushed as a popup on the engine's
/// state stack.
pub fn fe_state_reward_gallery_view_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);

        // vt[34]: the reward's data, `vt[34].fn(this + delta)`.
        c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vt.wrapping_add(0x110)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(0x114)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_0428)?;
        c.g[S2][0] = c.g[V0][0];

        // The reward type decides the state.
        c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(0x4c)).lw(c);
        let ty = c.g[V1][0];
        c.g[V0][0] = 6;
        let is_video = ty == c.g[V0][0];
        c.g[V0][0] = ((ty as i64) < 7) as u64;
        if is_video {
            view_item_video(c)?;
        } else if (ty as i64) < 7 {
            c.cycles += 4;
            if (ty as i64) < 3 {
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                view_item_preview(c)?;
            }
        } else {
            c.cycles += 5;
            c.g[V0][0] = 7;
            if ty == 7 {
                c.cycles += 1;
                view_item_preview(c)?;
            }
        }

        // 'b_1d04f8: push the new state as a popup when there is one.
        c.cycles += 2;
        c.g[A1][0] = c.g[S1][0];
        if c.g[S1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x10)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f290, 0x001d_04f8)?;
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

/// The video branch of ViewItem: a `cFEStateVideoPlayer` with its movie set
/// from the reward data in `s2` (the movie name is at +4).
pub(crate) fn view_item_video(c: &mut Cpu) -> Flow {
    c.cycles += 16;
    c.g[A0][0] = 0x280;
    c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0x1d8));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x001d_04bc)?;
    c.g[A1][0] = Addr(reward_gallery_self(c).wrapping_add(0x10)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A3][0] = Addr(reward_gallery_self(c).wrapping_add(0x44)).lb(c);
    c.g[A2][0] = c.g[S0][0];
    call_ptr(c, 0x001d_22f0, 0x001d_04d0)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(4)).lw(c);
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = 0;
    call_ptr(c, 0x001d_2598, 0x001d_04e4)?;
    Ok(())
}

/// The preview branch of ViewItem: a `cFEStatePreviewReward` (120 bytes)
/// showing the reward data in `s2`, with the reward type.
pub(crate) fn view_item_preview(c: &mut Cpu) -> Flow {
    c.cycles += 16;
    c.g[A0][0] = 0x78;
    c.g[A1][0] = sx(0x0046_6f78);
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x001d_0474)?;
    c.g[A1][0] = Addr(reward_gallery_self(c).wrapping_add(0x10)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A3][0] = Addr(reward_gallery_self(c).wrapping_add(0x44)).lb(c);
    c.g[A2][0] = c.g[S0][0];
    call_ptr(c, 0x001d_1aa0, 0x001d_0488)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = Addr(reward_gallery_self(c).wrapping_add(0x4c)).lw(c);
    c.g[A2][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x001d_20d8, 0x001d_049c)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// cFEStateRewardGallery_UpdateItemInfo (0x1d0d30).

/// `void cFEStateRewardGallery_UpdateItemInfo(cFEStateRewardGallery* self)`
/// (0x1d0d30): refresh the two item texts (+0x908 the name, +0x90c the price)
/// for the selected grid cell. Without a grid both texts are blanked.
pub fn fe_state_reward_gallery_update_item_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);

        c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
        item_info_body(c, frame)?;

        // 'b_1d0e64: the epilogue.
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body of UpdateItemInfo up to its epilogue (`a0` = the grid).
pub(crate) fn item_info_body(c: &mut Cpu, frame: Frame) -> Flow {
    if c.g[A0][0] != 0 {
        c.g[V0][0] = Addr(reward_gallery_self(c).wrapping_add(0x5c)).lw(c);
        return item_info_grid(c, frame);
    }
    // No grid: blank the name and the price.
    c.cycles += 3;
    c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x908)).lw(c);
    if c.g[A0][0] == 0 {
        c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x90c)).lw(c);
    } else {
        c.cycles += 4;
        c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x1408));
        call_ptr(c, 0x003a_0c50, 0x001d_0d6c)?;
        c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x90c)).lw(c);
    }
    c.cycles += 2;
    let no_price = c.g[A0][0] == 0;
    frame.restore(c, S0, 0x30);
    if no_price {
        return Ok(());
    }
    c.cycles += 5;
    c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x1408));
    call_ptr(c, 0x003a_0c50, 0x001d_0d84)?;
    frame.restore(c, S0, 0x30);
    Ok(())
}

/// The grid case of UpdateItemInfo: the item index, the item and its name,
/// then its price (or the empty text).
pub(crate) fn item_info_grid(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 14;
    c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(0x64)).lw(c);
    c.g[A1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
    c.mult(0, 3, 3, 2);
    c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x18)).lw(c);
    let vt = c.g[A1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x110)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0x114)).lw(c);
    c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    let target = c.g[V0][0] as u32;
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, target, 0x001d_0db4)?;
    c.g[S1][0] = c.g[V0][0];

    // vt[37]: the name of the item, set on the name text.
    c.g[V0][0] = Addr(reward_gallery_self(c).wrapping_add(0x908)).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 10;
        c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vt.wrapping_add(0x128)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(0x12c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_0ddc)?;
        c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x908)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_0c50, 0x001d_0de8)?;
    }
    c.g[V0][0] = Addr(reward_gallery_self(c).wrapping_add(0x90c)).lw(c);

    c.cycles += 2;
    if c.g[V0][0] == 0 {
        frame.restore(c, S0, 0x30);
        return Ok(());
    }

    // vt[35]: the price. A positive price the item can be bought for is
    // formatted; anything else (or a shown item) gets the empty text.
    c.cycles += 9;
    c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
    c.g[A1][0] = c.g[S1][0];
    let vt = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x118)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0x11c)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001d_0e0c)?;
    c.g[S1][0] = c.g[V0][0];
    let not_positive = (c.g[S1][0] as i64) <= 0;
    c.g[A1][0] = 0x004a_0000;
    if !not_positive {
        c.cycles += 8;
        c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vt.wrapping_add(0x130)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(0x134)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_0e30)?;
        let shown = c.g[V0][0] != 0;
        c.g[A1][0] = 0x004a_0000;
        if !shown {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0019_8af0, 0x001d_0e40)?;
            c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x90c)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001d_0e4c)?;
            frame.restore(c, S0, 0x30);
            return Ok(());
        }
    }

    // Blank the price text.
    c.cycles += 4;
    c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x90c)).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
    call_ptr(c, 0x003a_0c50, 0x001d_0e60)?;
    frame.restore(c, S0, 0x30);
    Ok(())
}

// ---------------------------------------------------------------------------
// cFEStateRewardGallery_UpdateSelection (0x1d0f30).

/// `void cFEStateRewardGallery_UpdateSelection(cFEStateRewardGallery* self)`
/// (0x1d0f30): move the selection's icon and the grid's selection state. The
/// icon is placed at the grid cell (offset of the grid's (x, y) plus the
/// icon's own (x, y) at +0x70/+0x74); a selected item sets the grid's
/// (+0x78, +0x7c) selection pair, and the grid's own vtable is told the
/// selection frame.
pub fn fe_state_reward_gallery_update_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);

        c.g[A2][0] = Addr(reward_gallery_self(c).wrapping_add(0x8f8)).lw(c);
        let no_icon = c.g[A2][0] == 0;
        c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
        if !no_icon {
            selection_icon(c)?;
        }
        selection_grid(c, frame)?;

        // Epilogue (after 'b_1d1098).
        c.cycles += 4;
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The icon half of UpdateSelection (`a0` = the grid, `a2` = the icon).
pub(crate) fn selection_icon(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        // No grid: the icon's slot 9 with 0.
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
        c.cycles += 6;
        c.g[A1][0] = 0;
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt.wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr(vt.wrapping_add(0x4c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_0fe0)?;
        c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
        return Ok(());
    }

    // The grid's cell: the grid's centre (a) plus the icon's offset (b).
    c.cycles += 29;
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, 0x003a_0290, 0x001d_0f60)?;
    c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8f8)).lw(c);
    let s = sp(c);
    c.g[A1][0] = sx(s.wrapping_add(0x10));
    call_ptr(c, 0x003a_0290, 0x001d_0f6c)?;

    let f2 = Addr(reward_gallery_self(c).wrapping_add(0x74)).read_fpu(c);
    c.g[A1][0] = 1;
    let f3 = Addr(reward_gallery_self(c).wrapping_add(0x70)).read_fpu(c);
    let a_y = Addr(s.wrapping_add(4)).read_fpu(c);
    let a_x = Addr(s).read_fpu(c);
    let b_y = a_y + f2;
    c.g[V0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8f8)).lw(c);
    let b_x = a_x + f3;
    set_fpr(c, 3, f3);
    set_fpr(c, 2, f2);
    set_fpr(c, 1, b_y);
    set_fpr(c, 0, b_x);
    Addr(s.wrapping_add(0x14)).write_fpu(c, b_y);
    Addr(s.wrapping_add(0x10)).write_fpu(c, b_x);

    // The 12-byte vector b is copied to the icon at +0x44.
    c.ldl(3, s.wrapping_add(0x17));
    c.ldr(3, s.wrapping_add(0x10));
    c.g[A0][0] = Addr(s.wrapping_add(0x18)).lw(c);
    let icon = c.g[V0][0] as u32;
    c.sdl(3, icon.wrapping_add(0x4b));
    c.sdr(3, icon.wrapping_add(0x44));
    let z = c.g[A0][0];
    Addr(icon.wrapping_add(0x4c)).sw(c, z);

    // vt[9] of the icon, with a 1.
    c.g[A2][0] = Addr(reward_gallery_self(c).wrapping_add(0x8f8)).lw(c);
    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
    let vt = c.g[V0][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001d_0fc4)?;
    c.g[A0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
    Ok(())
}

/// The grid half of UpdateSelection: the selection pair of the grid cell
/// (`a0` = the grid when this starts), then the grid's own vt[11] call.
pub(crate) fn selection_grid(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        frame.restore(c, S0, 0x40);
        return Ok(());
    }
    selection_cell(c)?;

    // Tail: the grid's vt[11] with the selection frame colour (0x4c6808).
    c.cycles += 9;
    c.g[A2][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
    c.g[A1][0] = sx(0x004c_0000u32.wrapping_add(0x6808));
    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
    let vt = c.g[V0][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x58)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(0x5c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001d_1094)?;
    frame.restore(c, S0, 0x40);
    Ok(())
}

/// The cell lookup of UpdateSelection: the item index, the selection pair
/// of the grid (+0x78, +0x7c), set from the item's vt[26] and vt[27].
pub(crate) fn selection_cell(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    c.g[V0][0] = Addr(reward_gallery_self(c).wrapping_add(0x5c)).lw(c);
    c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(0x64)).lw(c);
    c.g[A1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
    c.mult(0, 3, 3, 2);
    c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x18)).lw(c);
    let vt = c.g[A1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0x130)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0x134)).lw(c);
    c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    let target = c.g[V0][0] as u32;
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, target, 0x001d_1018)?;
    let no_item = c.g[V0][0] == 0;
    c.g[A1][0] = c.g[S1][0];
    if no_item {
        return Ok(());
    }

    // vt[26]: the item's row; a row at or after 0 is stored as is.
    c.cycles += 8;
    c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
    let vt = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0xd0)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0xd4)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001d_1034)?;
    c.g[V1][0] = c.g[V0][0];
    if (c.g[V1][0] as i64) >= 0 {
        c.g[V0][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
        c.cycles += 2;
        let grid = c.g[V0][0] as u32;
        let row = c.g[V1][0];
        Addr(grid.wrapping_add(0x78)).sw(c, row);
        Addr(grid.wrapping_add(0x7c)).sw(c, 0);
        return Ok(());
    }

    // vt[27]: a negative row, the item's index is stored at +0x7c.
    c.cycles += 11;
    c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(8)).lw(c);
    c.g[A1][0] = c.g[S1][0];
    let vt = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(vt.wrapping_add(0xd8)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(0xdc)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx(reward_gallery_self(c).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001d_1058)?;
    c.g[V1][0] = Addr(reward_gallery_self(c).wrapping_add(0x8fc)).lw(c);
    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
    let grid = c.g[V1][0] as u32;
    let count = c.g[V0][0];
    Addr(grid.wrapping_add(0x7c)).sw(c, count);
    Addr(grid.wrapping_add(0x78)).sw(c, c.g[A0][0]);
    Ok(())
}

// ---------------------------------------------------------------------------
// The five page constructors (0x1d12c8, 0x1d1460, 0x1d15f8, 0x1d1790, 0x1d1928).

/// What the gallery page constructors differ in.
pub(crate) struct RewardGalleryPage {
    /// The base constructor's page type (`t0`).
    kind: u64,
    /// The page's vtable is `0x470000 - vt_delta`.
    vt_delta: u32,
    /// Counts down from here: the item slots cleared are `last + 1`.
    last: u64,
    /// Offset of the first cleared slot (the slots are 4 bytes, going down).
    first: u32,
    /// Return address after the base constructor call.
    ret: u32,
    /// Loop head (the event check's pc).
    head: u32,
}

pub(crate) const RG_TRADING_CARDS: RewardGalleryPage = RewardGalleryPage {
    kind: 4,
    vt_delta: 0x5f38,
    last: 0x73,
    first: 0xbf4,
    ret: 0x001d_12e8,
    head: 0x001d_1300,
};

pub(crate) const RG_CONCEPT_ARTS: RewardGalleryPage = RewardGalleryPage {
    kind: 5,
    vt_delta: 0x60b8,
    last: 0x63,
    first: 0xbb4,
    ret: 0x001d_1480,
    head: 0x001d_1498,
};

pub(crate) const RG_VIDEOS: RewardGalleryPage = RewardGalleryPage {
    kind: 6,
    vt_delta: 0x6238,
    last: 0x1,
    first: 0xa2c,
    ret: 0x001d_1618,
    head: 0x001d_1630,
};

pub(crate) const RG_TOYS: RewardGalleryPage = RewardGalleryPage {
    kind: 7,
    vt_delta: 0x63b8,
    last: 0x1b,
    first: 0xa94,
    ret: 0x001d_17b0,
    head: 0x001d_17c8,
};

pub(crate) const RG_CHEAT_CHARS: RewardGalleryPage = RewardGalleryPage {
    kind: 8,
    vt_delta: 0x6538,
    last: 0x1b,
    first: 0xa94,
    ret: 0x001d_1948,
    head: 0x001d_1960,
};

/// `cFEStateRewardGalleryTradingCards::ctor(self, owner, a, b)` (0x1d12c8).
pub fn fe_state_reward_gallery_trading_cards_ctor(c: &mut Cpu) {
    reward_gallery_page_ctor(c, &RG_TRADING_CARDS);
}

/// `cFEStateRewardGalleryConceptArts::ctor` (0x1d1460).
pub fn fe_state_reward_gallery_concept_arts_ctor(c: &mut Cpu) {
    reward_gallery_page_ctor(c, &RG_CONCEPT_ARTS);
}

/// `cFEStateRewardGalleryVideos::ctor` (0x1d15f8).
pub fn fe_state_reward_gallery_videos_ctor(c: &mut Cpu) {
    reward_gallery_page_ctor(c, &RG_VIDEOS);
}

/// `cFEStateRewardGalleryToys::ctor` (0x1d1790).
pub fn fe_state_reward_gallery_toys_ctor(c: &mut Cpu) {
    reward_gallery_page_ctor(c, &RG_TOYS);
}

/// `cFEStateRewardGalleryCheatChars::ctor` (0x1d1928).
pub fn fe_state_reward_gallery_cheat_chars_ctor(c: &mut Cpu) {
    reward_gallery_page_ctor(c, &RG_CHEAT_CHARS);
}

/// The shared body of the page constructors: the base gallery constructor
/// (its `a2` is sign-extended from a byte), the page's vtable, a cleared
/// pointer at +0xa24 and the item slots from `first` down, cleared to 0.
pub(crate) fn reward_gallery_page_ctor(c: &mut Cpu, p: &RewardGalleryPage) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 24) as u32);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);

        c.g[T0][0] = p.kind;
        call_ptr(c, 0x001c_eea0, p.ret)?;

        let this = reward_gallery_self(c);
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(p.vt_delta));
        Addr(this.wrapping_add(0xa24)).sw(c, 0);
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(8)).sw(c, vtable);
        c.g[A0][0] = p.last;
        c.g[V0][0] = sx(this.wrapping_add(p.first));
        loop {
            c.cycles += 7;
            let slot = c.g[V0][0] as u32;
            Addr(slot).sw(c, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            let more = (c.g[A0][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, p.head);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// Offsets of the preview-reward state (`self`): the bolt bank and handle of its
// texture, the texture id, the load state (0 equal, 1 b < a, 2 a < b, which
// also indexes the two per-state arrays at +0x48 and +0x54).
pub(crate) const RWD_BANK: u32 = 0x64;

pub(crate) const RWD_HANDLE: u32 = 0x6c;

pub(crate) const RWD_TEX: u32 = 0x70;

pub(crate) const RWD_STATE: u32 = 0x74;

pub(crate) const RWD_SLOTS: u32 = 0x48;

pub(crate) const RWD_OBJS: u32 = 0x54;

/// `void cFEStatePreviewReward_RequestImage(self, id)` (0x1d2150): ask the
/// bolt manager of the world (`world->bolt + 0x1a70`) for bank `self->bank`
/// item `id`; the handle goes to `self->handle`.
pub fn cfe_state_preview_reward_request_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = fe_state_world(c);
        frame.save(c, S0, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A3][0] = 9;
        c.g[T0][0] = 1;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[T1][0] = 0;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_BANK));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
        call_ptr(c, 0x0019_da10, 0x001d_2184)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(RWD_HANDLE), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `'b_1d2224` part of `cFEStatePreviewReward_PollImageLoad`: fetch the
/// entry of the loaded item and store the state at `self + 0x74`: 1 when the
/// second 16-bit value is below the first, 2 when the first is below the
/// second, 0 when they are equal.
pub(crate) fn rwd_poll_state(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_BANK));
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_HANDLE));
    call_ptr(c, 0x0019_e2b0, 0x001d_21e4)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x6));
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
    let b_below_a = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if b_below_a {
        c.cycles += 2;
        c.sw((c.g[S0][0] as u32).wrapping_add(RWD_STATE), c.g[V0][0]);
    } else {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let a_below_b = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if a_below_b {
            c.cycles += 2;
            c.sw((c.g[S0][0] as u32).wrapping_add(RWD_STATE), c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(RWD_STATE), 0);
        }
    }
    Ok(())
}

/// The `'b_1d22d4` part of `cFEStatePreviewReward_PollImageLoad`: create the
/// render texture for the entry (3b3d00 fills the name, the render vtable
/// call makes the texture), release the bolt handle and attach the texture
/// to the per-state object, if there is one.
pub(crate) fn rwd_poll_attach(c: &mut Cpu) -> Flow {
    c.cycles += 28;
    c.g[A2][0] = c.g[SP][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x003b_3d00, 0x001d_2230)?;
    c.sb((c.g[SP][0] as u32).wrapping_add(0x4), 0);
    c.g[A2][0] = c.g[SP][0];
    c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
    c.g[A3][0] = 0;
    c.g[T0][0] = 1;
    c.g[T1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001d_2260)?;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_BANK));
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_HANDLE));
    c.sw((c.g[S0][0] as u32).wrapping_add(RWD_TEX), c.g[V0][0]);
    call_ptr(c, 0x0019_dc20, 0x001d_2274)?;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_STATE));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.sw((c.g[S0][0] as u32).wrapping_add(RWD_HANDLE), c.g[V0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(RWD_SLOTS));
    let no_slot = c.g[A0][0] == 0;
    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(RWD_OBJS));
    if no_slot {
        return Ok(());
    }
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    let no_obj = c.g[V1][0] == 0;
    c.g[A1][0] = 1;
    if no_obj {
        return Ok(());
    }
    c.cycles += 12;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_TEX));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x7c), 0);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x78), c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_STATE));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = c.lw(c.g[V0][0] as u32);
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, 0x001d_22d4)?;
    Ok(())
}

/// `void cFEStatePreviewReward_PollImageLoad(self)` (0x1d2198): while the
/// texture request is pending, ask the bolt manager whether it has finished
/// (0x19e238). When it has, read the entry's state, attach the texture, and
/// clear the handle.
pub fn cfe_state_preview_reward_poll_image_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = fe_state_world(c);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_BANK));
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a70));
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RWD_HANDLE));
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0019_e238, 0x001d_21cc)?;
        if c.g[V0][0] != 0 {
            rwd_poll_state(c)?;
            rwd_poll_attach(c)?;
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateVideoPlayer_Init(self)` (0x1d23e0): open the movie player
/// (0x317d70 for the 0x44-byte allocation, 0x2533c8 for the player object),
/// store the player at `+0x248`, and start playback.
pub fn cfe_state_video_player_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0x1320));
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x44;
        call_ptr(c, 0x0031_7d70, 0x001d_2408)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0025_33c8, 0x001d_2410)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x248), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_25e8, 0x001d_241c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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
    pub(crate) use crate::game::dirtysock::tagsunk::tests::alloc_machine;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::run_2 as run;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::string;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::world_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::MEM_HEAP;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS_2;
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

    pub(crate) use crate::game::fe::festatecharequipdetail::tests::money_text_test;
    pub(crate) use crate::game::fe::festatecharequipdetail::tests::world;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;

    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
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

    use crate::difftest::elf_image;
    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::fakes::LOG_COUNT;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_START;
    use crate::difftest::fill;
    use crate::difftest::fresh;
    use crate::difftest::run_generated;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD;
    pub(crate) use crate::game::fe::ovstatelodge::tests::oc942_cell;
    pub(crate) use crate::game::fe::ovstatelodge::tests::oc942_entry;
    pub(crate) use crate::game::fe::ovstatelodge::tests::oc942_install_base;
    pub(crate) use crate::game::fe::ovstatelodge::tests::ENTRY;

    pub(crate) use crate::game::fe::ovstatelodge::tests::OBJ2_2;
    pub(crate) use crate::game::fe::ovstatelodge::tests::OBJ_2 as OBJ_3;
    pub(crate) use crate::game::fe::ovstatelodge::tests::OC942_GATE;
    pub(crate) use crate::game::fe::ovstatelodge::tests::OC942_MAGIC;
    pub(crate) use crate::game::fe::ovstatelodge::tests::OC942_RES;

    pub(crate) use crate::game::fe::ovstatelodge::tests::SELF;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (the machine's RAM, never
    /// `DATA+0x100000`).
    pub(crate) const OC928_CTRL: u32 = DATA + 0x0020_0000;

    pub(crate) const OC928_MAGIC: u32 = 0x0928_c0de;

    // Fixture layout (the state `self` is DATA, 0xa00 bytes).
    pub(crate) const OC928_EV: u32 = DATA + 0x1000;

    pub(crate) const OC928_EV_VT: u32 = DATA + 0x1100;

    pub(crate) const OC928_HOLDER: u32 = DATA + 0x1200;

    pub(crate) const OC928_DEF_OBJ: u32 = DATA + 0x1300;

    pub(crate) const OC928_DEF_VT: u32 = DATA + 0x1400;

    pub(crate) const OC928_KID_VT: u32 = DATA + 0x1500;

    pub(crate) const OC928_CELL: u32 = DATA + 0x1600;

    pub(crate) const OC928_SCREEN: u32 = DATA + 0x1700;

    pub(crate) const OC928_SELF_VT: u32 = DATA + 0x2000;

    pub(crate) const OC928_KIDS: u32 = DATA + 0x3000;

    pub(crate) fn oc928_open(c: &mut Cpu) -> bool {
        c.bus.read32(OC928_CTRL) == OC928_MAGIC
    }

    /// `StrHash`, `UIScreen_LookupValue`, `UIScreen_StartThread` and
    /// `UIStateMgr_SwitchState` are faked while the gate word is set (they run
    /// real UI code on fixture data otherwise).
    pub(crate) fn oc928_fake_317670(c: &mut Cpu) {
        if !oc928_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc928_fake_317670),
                ee::s10::f_00317670__generated,
            );
        }
        fakes::fake_call(c, 0x0031_7670);
    }

    pub(crate) fn oc928_fake_39c778(c: &mut Cpu) {
        if !oc928_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc928_fake_39c778),
                ee::s13::f_0039c778__generated,
            );
        }
        fakes::fake_call(c, 0x0039_c778);
    }

    pub(crate) fn oc928_fake_39c870(c: &mut Cpu) {
        if !oc928_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc928_fake_39c870),
                ee::s13::f_0039c870__generated,
            );
        }
        fakes::fake_call(c, 0x0039_c870);
    }

    pub(crate) fn oc928_fake_39f400(c: &mut Cpu) {
        if !oc928_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc928_fake_39f400),
                ee::s13::f_0039f400__generated,
            );
        }
        fakes::fake_call(c, 0x0039_f400);
    }

    pub(crate) fn oc928_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc928_fake_317670,
            fakes::prev_cell(oc928_fake_317670),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039C778),
            oc928_fake_39c778,
            fakes::prev_cell(oc928_fake_39c778),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc928_fake_39c870,
            fakes::prev_cell(oc928_fake_39c870),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039F400),
            oc928_fake_39f400,
            fakes::prev_cell(oc928_fake_39f400),
        );
    }

    pub(crate) fn oc928_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A reward gallery with its event: `seed % 9` picks the pad event that is
    /// "pressed" (back, confirm, left, right, up, down, page up, page down,
    /// none); the grid, cursor and reward kind are random. `kids` fills the
    /// 4 x 8 cell table (the back path calls each cell), `calls` bounds the
    /// call index at which a fake unwinds.
    pub(crate) fn oc928_fix(c: &mut Cpu, seed: u32, calls: u32, kids: bool) {
        oc928_install();
        let mut r: Rng = base_9(c, seed, 0x1cf270, calls);
        oc928_w(c, OC928_CTRL, OC928_MAGIC);
        let me = DATA;
        c.g[A0][0] = me as u64;
        c.g[A1][0] = OC928_EV as u64;

        // The state, its vtable and the default object.
        oc928_w(c, me + 8, OC928_SELF_VT);
        oc928_w(c, me + 0x10, OC928_HOLDER);
        oc928_w(
            c,
            OC928_HOLDER + 0x14,
            if seed % 4 == 0 { 0 } else { OC928_DEF_OBJ },
        );
        oc928_w(c, OC928_DEF_OBJ, OC928_DEF_VT);
        oc928_w(c, GAME_GP + 0x2968, OC928_DEF_VT);
        c.bus.write16(OC928_DEF_VT + 0x10, 0);
        oc928_w(c, OC928_DEF_VT + 0x14, FAKE + 0x100);
        oc928_w(c, me + 0x40, OC928_SCREEN);

        let cols = 1 + r.next_u32() % 4;
        let rows = 1 + r.next_u32() % 3;
        oc928_w(c, me + 0x5c, cols);
        oc928_w(c, me + 0x58, rows);
        oc928_w(c, me + 0x60, 1 + r.next_u32() % 8);
        oc928_w(c, me + 0x64, r.next_u32() % 4);
        oc928_w(c, me + 0x68, r.next_u32() % 31);
        oc928_w(c, me + 0x54, r.next_u32() % 10);
        let k4c = r.next_u32();
        oc928_w(c, me + 0x4c, if k4c % 2 == 0 { 8 } else { k4c % 8 });
        let kind = if seed % 3 == 0 {
            (-5i32) as u32
        } else {
            r.next_u32() % 7
        };
        oc928_w(c, me + 0x8f0, kind);
        oc928_w(c, me + 0x8fc, if seed % 5 == 0 { 0 } else { OC928_CELL });
        oc928_w(c, OC928_CELL + 0x18, r.next_u32() % (cols * rows + 3));

        // The 4 x 8 cell table at +0x924 (row stride 0x20), each cell with a
        // vtable slot +0x4c.
        oc928_w(c, OC928_KID_VT + 0x48, 0);
        oc928_w(c, OC928_KID_VT + 0x4c, FAKE + 0x110);
        if kids {
            for row in 0..4u32 {
                for col in 0..8u32 {
                    let k = OC928_KIDS + 0x40 * (row * 8 + col);
                    let here = r.next_u32() % 3 != 0;
                    oc928_w(
                        c,
                        me + 0x924 + 0x20 * row + 4 * col,
                        if here { k } else { 0 },
                    );
                    oc928_w(c, k + 8, OC928_KID_VT);
                }
            }
        }

        // The event's vtable: pad queries (delta, function) at off, off+4.
        oc928_w(c, OC928_EV + 8, OC928_EV_VT);
        const EV_SLOTS: [u32; 8] = [0x30, 0x28, 0x98, 0xa0, 0x88, 0x90, 0xd8, 0xc8];
        let active = seed % 9;
        for (i, &off) in EV_SLOTS.iter().enumerate() {
            let target = FAKE + 0x10 * i as u32;
            oc928_w(c, OC928_EV_VT + off, 0);
            oc928_w(c, OC928_EV_VT + off + 4, target);
            let i = i as u32;
            let v = if i < active {
                0
            } else if i == active {
                1
            } else {
                r.next_u32() & 1
            };
            force(c, i, target, v, KEEP_F0);
        }

        // The state's vtable: the item queries and notifications.
        const SELF_SLOTS: [u32; 8] = [0x130, 0x110, 0x120, 0x118, 0xf8, 0x100, 0x140, 0x108];
        for (i, &off) in SELF_SLOTS.iter().enumerate() {
            let target = FAKE + 0x80 + 0x10 * i as u32;
            oc928_w(c, OC928_SELF_VT + off, 0);
            oc928_w(c, OC928_SELF_VT + off + 4, target);
        }
        let same_kind = r.next_u32() & 1 != 0;
        force(c, 8, FAKE + 0x80, r.next_u32() & 1, KEEP_F0);
        force(c, 9, FAKE + 0x90, r.next_u32() % 40, KEEP_F0);
        force(
            c,
            10,
            FAKE + 0xa0,
            if same_kind && (kind as i32) >= 0 {
                kind
            } else {
                r.next_u32() % 7
            },
            KEEP_F0,
        );
        force(c, 11, FAKE + 0xb0, r.next_u32() % 10, KEEP_F0);
    }

    /// The pad handling matches the generated body: every event, cursor
    /// wrap, confirm outcome, and unwinds inside the call chains.
    #[test]
    fn c_fe_state_reward_gallery_handle_input_matches() {
        strict(
            ee::s04::f_001cf270__generated,
            c_fe_state_reward_gallery_handle_input,
            180,
            &|c, seed| oc928_fix(c, seed, 12, true),
        );
    }

    /// The fixture reaches many distinct paths (total cycle costs of the
    /// generated body over the seeds: no unwinds, no cell table).
    #[test]
    fn c_fe_state_reward_gallery_handle_input_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..400u32 {
            let mut c = fresh();
            oc928_fix(&mut c, seed, 0, false);
            let before = c.cycles;
            run_generated(&mut c, ee::s04::f_001cf270__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 16, "only {} paths: {seen:?}", seen.len());
    }

    pub(crate) const OC931_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC931_GATE: u32 = OC931_CTRL + 4;

    pub(crate) const OC931_FOUND: u32 = OC931_CTRL + 8;

    pub(crate) const OC931_SEEN: u32 = OC931_CTRL + 12;

    pub(crate) const OC931_MAGIC: u32 = 0x4f43_3933;

    pub(crate) const OC931_SELF: u32 = DATA + 0x1_0000;

    pub(crate) const OC931_SCREEN: u32 = DATA + 0x2_0000;

    pub(crate) const OC931_OBJ: u32 = DATA + 0x3_0000;

    pub(crate) const OC931_VT: u32 = DATA + 0x4_0000;

    pub(crate) const OC931_TEXT: u32 = DATA + 0x5_0000;

    pub(crate) const OC931_WIDGET: u32 = DATA + 0x6_0000;

    pub(crate) const OC931_WVT: u32 = DATA + 0x7_0000;

    /// The fake callee: logs like the plain fake; `getObjectByHashName`
    /// answers with a widget or none, as the fixture's `OC931_FOUND` bits say.
    pub(crate) fn oc931_hook(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc != ra || target != 0x0039_d860 {
            return;
        }
        let k = c.bus.read32(OC931_SEEN);
        c.bus.write32(OC931_SEEN, k + 1);
        let found = c.bus.read32(OC931_FOUND) >> (k & 31) & 1;
        c.g[V0][0] = if found == 1 { OC931_WIDGET as u64 } else { 0 };
    }

    macro_rules! oc931_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC931_GATE) != OC931_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc931_hook(c, $addr);
                }
            )*
            pub(crate) fn oc931_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc931_fakes!(
        (
            oc931_fk_00317670,
            OC931_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc931_fk_002c26d0,
            OC931_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002C26D0
        ),
        (
            oc931_fk_003a0e90,
            OC931_PREV_003A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003A0E90
        ),
        (
            oc931_fk_0039d860,
            OC931_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039D860
        ),
        (
            oc931_fk_003a0c50,
            OC931_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003A0C50
        ),
    );

    /// Fixture: seed picks the path (no text widget; no screen text object;
    /// format string method returning none; full path with both, one or no
    /// arrow widgets found), the page numbers (including per = 0, 1 page and
    /// many) and, through `machine`, the unwinds of the calls.
    pub(crate) fn oc931_fixture(c: &mut Cpu, seed: u32, unwind: bool) {
        let mut r = machine(c, seed, 0x0931_1d0b, 10);
        if !unwind {
            c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        }
        oc931_install_fakes();
        c.bus.write32(OC931_GATE, OC931_MAGIC);
        c.pc = 0x001d_0bc8;
        c.g[A0][0] = sx(OC931_SELF);
        let class = seed % 9;
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        w(
            c,
            OC931_SELF + 0x904,
            if class == 0 { 0 } else { OC931_TEXT },
        );
        w(c, OC931_SELF + 0x40, OC931_SCREEN);
        w(c, OC931_SCREEN + 0xd0, OC931_SCREEN + 0x100);
        w(c, OC931_SCREEN + 0x100 + 0x10, OC931_SCREEN + 0x200);
        w(
            c,
            OC931_SCREEN + 0x200 + 0x10,
            if class == 1 { 0 } else { OC931_OBJ },
        );
        w(c, OC931_OBJ + 4, OC931_VT);
        c.bus.write16(OC931_VT + 0x20, (r.next_u32() & 0x3c) as u16);
        // Format string method: a fake (non-zero result) or a real leaf that
        // returns 0.
        w(
            c,
            OC931_VT + 0x24,
            if class == 2 { 0x0014_06c0 } else { fakes::FAKE },
        );
        let (per, total, cur) = match seed / 9 % 6 {
            0 => (4, 10, 3),
            1 => (4, 3, 1),
            2 => (1, 7, 6),
            3 => (0, 5, 2),
            4 => (3, 0, 0),
            _ => (8, 9, 100),
        };
        w(c, OC931_SELF + 0x58, per);
        w(c, OC931_SELF + 0x60, total);
        w(c, OC931_SELF + 0x64, cur);
        w(c, OC931_FOUND, if class < 6 { 0 } else { (seed / 9) % 4 });
        w(c, OC931_SEEN, 0);
        // The arrow widget: vtable with the visibility method at slot 9.
        w(c, OC931_WIDGET + 8, OC931_WVT);
        c.bus.write16(OC931_WVT + 0x48, 0);
        w(c, OC931_WVT + 0x4c, fakes::FAKE + 0x10);
    }

    /// `cFEStateRewardGallery_UpdatePageText`: every path (no text widget, no
    /// text object, no format string, widgets found or missing), page counts
    /// and the unwinds of its calls; LO/HI of the divisions too.
    #[test]
    fn c_fe_state_reward_gallery_update_page_text_matches() {
        check_lohi(
            ee::s04::f_001d0bc8__generated,
            c_fe_state_reward_gallery_update_page_text,
            540,
            &|c, seed| oc931_fixture(c, seed, true),
        );
    }

    /// The fixture reaches every path of the generated body (distinct fake
    /// call counts without unwinds: 0, 1, 2 and the full path with 0, 1 or 2
    /// arrows found).
    #[test]
    fn c_fe_state_reward_gallery_update_page_text_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..108 {
            let mut c = fresh();
            oc931_fixture(&mut c, seed, false);
            ee::s04::f_001d0bc8__generated(&mut c);
            seen.insert(c.bus.read32(fakes::LOG_COUNT));
        }
        assert!(seen.len() >= 6, "fake call counts {seen:?}");
    }

    /// Gate word of this module's fakes (in the machine's RAM: off in every
    /// other test) and the magic that turns them on.
    pub(crate) const OC935_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC935_MAGIC: u32 = 0x0935_c0de;

    /// Number of `GetHashValue32` calls so far, and the call that matches.
    pub(crate) const OC935_HASH_N: u32 = OC935_CTRL + 4;

    pub(crate) const OC935_HASH_TARGET: u32 = OC935_CTRL + 8;

    /// Result mailboxes (0x20 bytes per fake callee index).
    pub(crate) const OC935_MAIL: u32 = OC935_CTRL + 0x100;

    /// The hash every widget carries at `+0x38` (what a matching call returns).
    pub(crate) const OC935_HASH: u32 = 0x8765_4321;

    pub(crate) const OC935_ZERO: u32 = 1;

    pub(crate) const OC935_NEG1: u32 = 2;

    pub(crate) const OC935_SELF: u32 = DATA + 0x1000;

    pub(crate) const OC935_WIDGET: u32 = DATA + 0x3000;

    pub(crate) const OC935_VT_SELF: u32 = DATA + 0x6000;

    pub(crate) const OC935_VT_W: u32 = DATA + 0x6800;

    /// Mailbox word returning `v` (0 leaves the fake's own result).
    pub(crate) fn oc935_val(v: u32) -> u32 {
        v + 3
    }

    /// The fake callee `idx` answers its calls from these four words in turn.
    pub(crate) fn oc935_mail(c: &mut Cpu, idx: u32, words: [u32; 4]) {
        for (i, w) in words.iter().enumerate() {
            c.bus.write32(OC935_MAIL + 0x20 * idx + 4 * i as u32, *w);
        }
    }

    pub(crate) fn oc935_apply_mail(c: &mut Cpu, idx: u32) {
        let base = OC935_MAIL + 0x20 * idx;
        let k = c.bus.read32(base + 0x10);
        c.bus.write32(base + 0x10, k + 1);
        match c.bus.read32(base + 4 * (k % 4)) {
            0 => {}
            OC935_ZERO => c.g[V0][0] = 0,
            OC935_NEG1 => c.g[V0][0] = !0,
            m => c.g[V0][0] = sx(m - 3),
        }
    }

    /// `miss` hook: the vtable fakes (`FAKE + 0x10 * idx`) answer from their
    /// mailbox once the gate is on.
    pub(crate) fn oc935_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc == ra
            && c.bus.read32(OC935_CTRL) == OC935_MAGIC
            && (FAKE..FAKE + 0x200).contains(&target)
        {
            oc935_apply_mail(c, (target - FAKE) / 0x10);
        }
    }

    /// `GetHashValue32` fake: call number `OC935_HASH_TARGET` returns the
    /// widgets' hash (a match), every other call something else.
    pub(crate) fn oc935_fake_hash(c: &mut Cpu) {
        if c.bus.read32(OC935_CTRL) != OC935_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc935_fake_hash),
                ee::s10::f_00317670__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(OC935_HASH_N);
        c.bus.write32(OC935_HASH_N, n + 1);
        fakes::fake(c, 0x0031_7670, ra);
        if c.pc == ra {
            let hit = n == c.bus.read32(OC935_HASH_TARGET);
            c.g[V0][0] = sx(if hit { OC935_HASH } else { OC935_HASH ^ 1 });
        }
    }

    /// Gated fakes in the override slots of the direct callees: each logs its
    /// call like `fakes::fake`, then answers from its mailbox; gate off, the
    /// slot runs the body it held.
    macro_rules! oc935_gated {
        ($($fake:ident => $addr:literal, $idx:literal, $slot:path, $generated:path;)*) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC935_CTRL) != OC935_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $addr, ra);
                    if c.pc == ra {
                        oc935_apply_mail(c, $idx);
                    }
                }
            )*

            pub(crate) fn oc935_install_fakes() {
                fakes::hook(
                    addr_of_mut!(ee::s10::OVERRIDE_00317670),
                    oc935_fake_hash,
                    fakes::prev_cell(oc935_fake_hash),
                );
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc935_gated! {
        oc935_fake_398380 => 0x0039_8380, 24, ee::s13::OVERRIDE_00398380, ee::s13::f_00398380__generated;
        oc935_fake_3a0c50 => 0x003a_0c50, 25, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated;
        oc935_fake_3a1000 => 0x003a_1000, 26, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated;
        oc935_fake_417828 => 0x0041_7828, 27, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
        oc935_fake_145f90 => 0x0014_5f90, 28, ee::s01::OVERRIDE_00145F90, ee::s01::f_00145f90__generated;
        oc935_fake_147488 => 0x0014_7488, 29, ee::s01::OVERRIDE_00147488, ee::s01::f_00147488__generated;
        oc935_fake_148950 => 0x0014_8950, 29, ee::s01::OVERRIDE_00148950, ee::s01::f_00148950__generated;
        oc935_fake_14a130 => 0x0014_a130, 29, ee::s01::OVERRIDE_0014A130, ee::s01::f_0014a130__generated;
        oc935_fake_14dc80 => 0x0014_dc80, 29, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc935_fake_14dd58 => 0x0014_dd58, 29, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc935_fake_156b28 => 0x0015_6b28, 29, ee::s01::OVERRIDE_00156B28, ee::s01::f_00156b28__generated;
        oc935_fake_1a1cd0 => 0x001a_1cd0, 29, ee::s03::OVERRIDE_001A1CD0, ee::s03::f_001a1cd0__generated;
        oc935_fake_1d3160 => 0x001d_3160, 29, ee::s04::OVERRIDE_001D3160, ee::s04::f_001d3160__generated;
        oc935_fake_39e2a0 => 0x0039_e2a0, 29, ee::s13::OVERRIDE_0039E2A0, ee::s13::f_0039e2a0__generated;
    }

    /// Turn the fakes on: gate, mailbox-aware `miss` hook, and the call
    /// number of `GetHashValue32` that matches the widget's hash.
    pub(crate) fn oc935_arm(c: &mut Cpu, hash_target: u32) {
        oc935_install_fakes();
        c.hooks.miss = oc935_miss;
        c.bus.write32(OC935_CTRL, OC935_MAGIC);
        c.bus.write32(OC935_HASH_N, 0);
        c.bus.write32(OC935_HASH_TARGET, hash_target);
    }

    /// A vtable slot `{delta, FAKE + 0x10 * idx}`.
    pub(crate) fn oc935_vslot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, idx: u32) {
        c.bus.write16(vt + off, ((r.next_u32() % 8) * 4) as u16);
        c.bus.write32(vt + off + 4, FAKE + 0x10 * idx);
    }

    /// The widget vtable (set visible `+0x48`, set value `+0x58`) and the
    /// state vtable (`+0x158`, `+0x110`, `+0x120`, `+0x130`, `+0x118`, `+0xd0`,
    /// `+0xd8`), `self+8` and `widget+8` pointing at them.
    pub(crate) fn oc935_vtables(c: &mut Cpu, r: &mut Rng) {
        oc935_vslot(c, r, OC935_VT_W, 0x48, 0);
        oc935_vslot(c, r, OC935_VT_W, 0x58, 1);
        for (off, idx) in [
            (0x158, 2),
            (0x110, 3),
            (0x120, 4),
            (0x130, 5),
            (0x118, 6),
            (0xd0, 7),
            (0xd8, 8),
        ] {
            oc935_vslot(c, r, OC935_VT_SELF, off, idx);
        }
        c.bus.write32(OC935_SELF + 8, OC935_VT_SELF);
    }

    /// An object that has the widget vtable.
    pub(crate) fn oc935_widget(c: &mut Cpu, obj: u32) {
        c.bus.write32(obj + 8, OC935_VT_W);
        c.bus.write32(obj + 0x38, OC935_HASH);
    }

    /// Distinct call-target sequences the generated body makes over `seeds`
    /// (a path count: the setups must reach at least `at_least` of them).
    pub(crate) fn oc935_require_sequences(
        generated: GuestFn,
        seeds: u32,
        at_least: usize,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            run_generated(&mut c, generated);
            let n = c.bus.read32(LOG_COUNT).min(64);
            let seq: Vec<u32> = (0..n)
                .map(|i| c.bus.read32(LOG_START + 0x220 * i))
                .collect();
            seen.insert(seq);
        }
        assert!(
            seen.len() >= at_least,
            "only {} distinct call sequences (want {at_least})",
            seen.len()
        );
    }

    /// Reward types seen by the type switches (3..8 in the tables, others out of range).
    pub(crate) const OC935_TYPES: [u32; 10] = [3, 4, 5, 6, 7, 8, 9, 0, 2, 12];

    /// Fixture for `cFEStateRewardGallery_OnWidgetCreated`: the widget's hash
    /// matches the k-th `GetHashValue32` call (even seeds favour the three
    /// type-dependent branches, odd seeds walk the chain and the cell grid).
    pub(crate) fn oc935_gallery_created_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1cf820, 8);
        elf_image(c);
        let k = seed / 2;
        let (target, ty) = if seed % 2 == 0 {
            (
                [8, 9, 11][(k % 3) as usize],
                OC935_TYPES[(k / 3 % 10) as usize],
            )
        } else {
            (k % 41, OC935_TYPES[(r.next_u32() % 10) as usize])
        };
        oc935_arm(c, target);
        c.pc = 0x001c_f820;
        oc935_vtables(c, &mut r);
        let this = OC935_SELF;
        c.bus.write32(this + 0x4c, ty);
        c.bus.write32(this + 0x58, 1 + r.next_u32() % 3);
        c.bus.write32(this + 0x5c, 1 + r.next_u32() % 3);
        c.bus.write32(this + 0x68, r.next_u32() % 100);
        c.bus.write32(this + 0x6c, r.next_u32() % 100);
        c.bus.write32(this + 0x10, DATA + 0x4000);
        oc935_widget(c, OC935_WIDGET);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(OC935_WIDGET);
    }

    /// Every widget name of the screen, the type-dependent icons, titles and
    /// captions, the cell/frame grid, and unwinds of the calls.
    #[test]
    fn c_fe_state_reward_gallery_on_widget_created_matches() {
        check(
            ee::s04::f_001cf820__generated,
            c_fe_state_reward_gallery_on_widget_created,
            216,
            &oc935_gallery_created_setup,
        );
        oc935_require_sequences(
            ee::s04::f_001cf820__generated,
            216,
            25,
            &oc935_gallery_created_setup,
        );
    }

    /// Fixture for `cFEStateRewardGallery_UpdateHelpText`: no help widget,
    /// no item, item state below/equal/above the mode, owned or not, both
    /// screen types.
    pub(crate) fn oc935_help_text_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1d0648, 9);
        elf_image(c);
        oc935_arm(c, 99);
        c.pc = 0x001d_0648;
        oc935_vtables(c, &mut r);
        let this = OC935_SELF;
        let txt = if seed % 7 == 0 { 0 } else { OC935_WIDGET };
        c.bus.write32(this + 0x900, txt);
        oc935_widget(c, OC935_WIDGET);
        c.bus
            .write32(this + 0x4c, if (seed / 25) % 2 == 0 { 8 } else { 5 });
        let sel = [0xffff_ffff, 0, 1, 2, 5][((seed / 5) % 5) as usize];
        c.bus.write32(this + 0x8f0, sel);
        c.bus.write32(this + 0x54, 50);
        let item = if seed % 9 == 0 {
            OC935_ZERO
        } else {
            oc935_val(DATA + 0x2000)
        };
        oc935_mail(c, 3, [item; 4]);
        let state = [
            OC935_NEG1,
            oc935_val(0),
            oc935_val(1),
            oc935_val(2),
            oc935_val(5),
        ][(seed % 5) as usize];
        oc935_mail(c, 4, [state; 4]);
        let has = if (seed / 50) % 2 == 0 {
            OC935_ZERO
        } else {
            oc935_val(1)
        };
        oc935_mail(c, 5, [has; 4]);
        let cost = if (seed / 100) % 2 == 0 {
            oc935_val(0)
        } else {
            oc935_val(100)
        };
        oc935_mail(c, 6, [cost; 4]);
        c.bus.write32(DATA + 0x2008, r.next_u32());
        c.g[A0][0] = sx(this);
        c.g[A1][0] = (seed % 12) as u64;
    }

    #[test]
    fn c_fe_state_reward_gallery_update_help_text_matches() {
        check(
            ee::s04::f_001d0648__generated,
            c_fe_state_reward_gallery_update_help_text,
            400,
            &oc935_help_text_setup,
        );
        oc935_require_sequences(
            ee::s04::f_001d0648__generated,
            400,
            8,
            &oc935_help_text_setup,
        );
    }

    /// Fixture for `cFEStateRewardGallery_RefreshRowIcons`: rows 0..2, one to
    /// four columns, scroll offsets, shown/hidden cells (count), owned and
    /// unowned rewards with the three icon sources, selected or not.
    pub(crate) fn oc935_row_icons_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1d0890, 12);
        elf_image(c);
        oc935_arm(c, 99);
        c.pc = 0x001d_0890;
        oc935_vtables(c, &mut r);
        let this = OC935_SELF;
        let row = seed % 3;
        let cols = 1 + (seed / 9) % 4;
        c.bus.write32(this + 0x64, (seed / 3) % 3);
        c.bus.write32(this + 0x5c, cols);
        c.bus.write32(this + 0x68, r.next_u32() % 15);
        let sel = [0xffff_ffff, 5, 6][((seed / 36) % 3) as usize];
        c.bus.write32(this + 0x8f0, sel);
        c.bus.write32(this + 0x10, DATA + 0x4000);
        for rr in 0..3 {
            for col in 0..8 {
                let cell = DATA + 0x8000 + 0x100 * (rr * 8 + col);
                let frame = DATA + 0xa000 + 0x100 * (rr * 8 + col);
                oc935_widget(c, cell);
                oc935_widget(c, frame);
                c.bus.write32(this + 0x924 + 0x20 * rr + 4 * col, cell);
                c.bus.write32(this + 0x9a4 + 0x20 * rr + 4 * col, frame);
            }
        }
        let pick = |r: &mut Rng, f: &dyn Fn(u32) -> u32| -> [u32; 4] {
            [
                f(r.next_u32()),
                f(r.next_u32()),
                f(r.next_u32()),
                f(r.next_u32()),
            ]
        };
        let owned = pick(&mut r, &|x| {
            if x % 2 == 0 {
                OC935_ZERO
            } else {
                oc935_val(1)
            }
        });
        oc935_mail(c, 5, owned);
        let a = pick(&mut r, &|x| {
            if x % 2 == 0 {
                OC935_NEG1
            } else {
                oc935_val(x % 3)
            }
        });
        oc935_mail(c, 7, a);
        let b = pick(&mut r, &|x| {
            if x % 2 == 0 {
                OC935_ZERO
            } else {
                oc935_val(7 + x % 5)
            }
        });
        oc935_mail(c, 8, b);
        let id = pick(&mut r, &|x| oc935_val(x % 8));
        oc935_mail(c, 3, id);
        let state = pick(&mut r, &|x| oc935_val(5 + x % 2));
        oc935_mail(c, 4, state);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = row as u64;
    }

    #[test]
    fn c_fe_state_reward_gallery_refresh_row_icons_matches() {
        check(
            ee::s04::f_001d0890__generated,
            c_fe_state_reward_gallery_refresh_row_icons,
            324,
            &oc935_row_icons_setup,
        );
        oc935_require_sequences(
            ee::s04::f_001d0890__generated,
            324,
            15,
            &oc935_row_icons_setup,
        );
    }

    /// Fixture for `cFEStatePreviewReward_OnWidgetCreated`: null widget,
    /// every name of the chain with every reward type, frames and bitmaps
    /// stored, and the selected-frame update (missing items or frames,
    /// no selection).
    pub(crate) fn oc935_preview_created_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1d1c98, 8);
        elf_image(c);
        let k = seed / 2;
        let (target, ty) = if seed % 2 == 0 {
            (
                [0, 1, 2][(k % 3) as usize],
                OC935_TYPES[(k / 3 % 10) as usize],
            )
        } else {
            (k % 12, OC935_TYPES[(r.next_u32() % 10) as usize])
        };
        oc935_arm(c, target);
        c.pc = 0x001d_1c98;
        oc935_vtables(c, &mut r);
        let this = OC935_SELF;
        c.bus.write32(this + 0x68, ty);
        let sel = [0xffff_ffff, 0, 1, 2][((seed / 3) % 4) as usize];
        c.bus.write32(this + 0x74, sel);
        c.bus.write32(this + 0x70, r.next_u32());
        c.bus.write32(this + 0x10, DATA + 0x4000);
        c.bus.write32(this + 0x60, DATA + 0x4400);
        c.bus.write32(DATA + 0x4400, DATA + 0x4500);
        for i in 0..3u32 {
            let item = DATA + 0x8000 + 0x100 * i;
            let frame = DATA + 0x9000 + 0x100 * i;
            oc935_widget(c, item);
            oc935_widget(c, frame);
            let have_item = (seed >> (i + 3)) % 4 != 0;
            let have_frame = (seed >> (i + 5)) % 4 != 0;
            c.bus
                .write32(this + 0x48 + 4 * i, if have_item { item } else { 0 });
            c.bus
                .write32(this + 0x54 + 4 * i, if have_frame { frame } else { 0 });
        }
        oc935_widget(c, OC935_WIDGET);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = if seed % 13 == 0 { 0 } else { sx(OC935_WIDGET) };
    }

    #[test]
    fn c_fe_state_preview_reward_on_widget_created_matches() {
        check(
            ee::s04::f_001d1c98__generated,
            c_fe_state_preview_reward_on_widget_created,
            180,
            &oc935_preview_created_setup,
        );
        oc935_require_sequences(
            ee::s04::f_001d1c98__generated,
            180,
            12,
            &oc935_preview_created_setup,
        );
    }

    #[test]
    fn reward_gallery_find_texture_by_name_matches() {
        check(
            ee::s04::f_001cfdd0__generated,
            reward_gallery_find_texture_by_name,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x1c_fdd0, 0);
                let this = DATA + 0x100;
                let count = seed % 5;
                c.bus.write32(this + 0x78, count);
                for i in 0..count {
                    let entry = this + 0x7c + 12 * i;
                    c.bus.write8(entry, b'a' + i as u8);
                    c.bus.write8(entry + 1, 0);
                    c.bus.write32(entry + 8, 0x1000 + i);
                }
                c.bus.write8(DATA + 0x400, b'a' + (seed % 6) as u8);
                c.bus.write8(DATA + 0x401, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x400);
            },
        );
    }

    #[test]
    fn reward_gallery_request_shape_load_matches() {
        check(
            ee::s04::f_001cfe60__generated,
            reward_gallery_request_shape_load,
            120,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1c_fe60, 3);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
                c.bus.write32(WORLD + 0x7c, DATA + 0x3000);
                let this = DATA + 0x100;
                let obj = DATA + 0x200;
                c.bus.write32(this + 0x8, obj);
                c.bus.write32(this + 0x48, seed.wrapping_mul(0x9e37));
                c.bus.write16(obj + 0xc8, 0);
                // Seeds pick the leaf (returns 0) or a fake (returns non-zero).
                let target = if seed % 2 == 0 { 0x0014_06c0 } else { FAKE };
                c.bus.write32(obj + 0xcc, target);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn reward_gallery_release_textures_matches() {
        check(
            ee::s04::f_001d00c0__generated,
            reward_gallery_release_textures,
            120,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1d_00c0, 3);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
                c.bus.write32(WORLD + 0x7c, DATA + 0x3000);
                let ctx = DATA + 0x5000;
                c.bus.write32(GAME_GP.wrapping_sub(0x854), ctx);
                let vt = DATA + 0x5100;
                c.bus.write32(ctx + 0x10d8, vt);
                c.bus.write16(vt + 0x190, 0);
                c.bus.write32(vt + 0x194, FAKE + 0x20);
                let this = DATA + 0x100;
                let count = seed % 4;
                c.bus.write32(this + 0x78, count);
                for i in 0..count {
                    // Negative words skip the release call, others make it.
                    let word = if (seed + i) % 2 == 0 {
                        u32::MAX
                    } else {
                        0x1000 + i
                    };
                    c.bus.write32(this + 0x84 + 12 * i, word);
                }
                c.bus
                    .write32(this + 0x8ec, if seed % 2 == 0 { u32::MAX } else { 0 });
                c.bus.write32(this + 0x48, 0xf);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn reward_gallery_init_pages_matches() {
        check(
            ee::s04::f_001d0180__generated,
            reward_gallery_init_pages,
            120,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1d_0180, 3);
                let this = DATA + 0x100;
                c.bus.write32(this + 0x48, seed.wrapping_mul(0x51));
                c.bus.write32(this + 0x5c, seed % 7);
                c.bus.write32(this + 0x68, 3 + seed % 5);
                c.bus.write32(this + 0x8, DATA + 0x200);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn reward_gallery_purchase_item_matches() {
        check(
            ee::s04::f_001d0320__generated,
            reward_gallery_purchase_item,
            120,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1d_0320, 3);
                let this = DATA + 0x100;
                let obj = DATA + 0x200;
                c.bus.write32(this + 0x8, obj);
                c.bus.write32(this + 0x44, seed & 0xff);
                c.bus.write32(this + 0x4c, seed.wrapping_mul(7));
                c.bus.write32(this + 0x50, seed.wrapping_mul(11));
                for off in [0x150u32, 0x154, 0x108, 0x10c, 0x158, 0x15c, 0x168, 0x16c] {
                    c.bus.write32(obj + off, FAKE + 0x30);
                }
                c.bus.write16(obj + 0x150, 0);
                c.bus.write16(obj + 0x108, 0);
                c.bus.write16(obj + 0x158, 0);
                c.bus.write16(obj + 0x168, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = seed as u64;
            },
        );
    }

    /// The fake callee with sentinel results. Fakes zero-extend the forced
    /// results, so 0xffff_fffe forces 0 and 0xffff_ffff forces -1 here.
    pub(crate) fn oc941_sentinel_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(LOG_COUNT);
        let forced = if n < 8 {
            c.bus.read32(LOG_RESULTS + 4 * n)
        } else {
            0
        };
        fake(c, target, ra);
        if c.pc == ra {
            match forced {
                0xffff_fffe => c.g[V0][0] = 0,
                0xffff_ffff => c.g[V0][0] = u64::MAX,
                _ => {}
            }
        }
    }

    /// Forced results for the first eight fake calls, chosen by the seed:
    /// zero, -1, a hash (0 = not forced) or a data pointer.
    pub(crate) fn oc941_force_results(c: &mut Cpu, seed: u32) {
        for n in 0..8u32 {
            let v = match (seed >> (2 * n)) & 3 {
                0 => 0xffff_fffe,
                1 => 0xffff_ffff,
                2 => 0,
                _ => DATA + 0x4000,
            };
            c.bus.write32(LOG_RESULTS + 4 * n, v);
        }
    }

    /// The reward types the viewer branches on (6 = video, 7 = toy, -1 = other).
    pub(crate) const OC941_TYPES: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, (-1i32) as u32];

    #[test]
    fn fe_state_reward_gallery_view_item_matches() {
        check(
            ee::s04::f_001d03f8__generated,
            fe_state_reward_gallery_view_item,
            200,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1d03f8, 6);
                c.hooks.miss = oc941_sentinel_miss;
                oc941_force_results(c, seed);
                let this = DATA + 0x1000;
                let vt = vtable(c, &mut r, &[0x110, 0x128]);
                c.bus.write32(this + 8, vt);
                c.bus.write32(this + 0x10, DATA + 0x3000);
                c.bus.write8(this + 0x44, r.next_u32() as u8);
                c.bus
                    .write32(this + 0x4c, OC941_TYPES[(seed % 10) as usize]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn fe_state_reward_gallery_update_item_info_matches() {
        check(
            ee::s04::f_001d0d30__generated,
            fe_state_reward_gallery_update_item_info,
            200,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1d0d30, 6);
                c.hooks.miss = oc941_sentinel_miss;
                oc941_force_results(c, seed);
                let this = DATA + 0x1000;
                let grid = DATA + 0x5000;
                let vt = vtable(c, &mut r, &[0x110, 0x118, 0x128, 0x130]);
                c.bus.write32(this + 8, vt);
                c.bus.write32(this + 0x5c, r.next_u32() & 0xf);
                c.bus.write32(this + 0x64, r.next_u32() & 0xf);
                c.bus
                    .write32(this + 0x8fc, if seed % 4 == 0 { 0 } else { grid });
                c.bus
                    .write32(this + 0x908, if seed % 3 == 0 { 0 } else { DATA + 0x6000 });
                c.bus
                    .write32(this + 0x90c, if seed % 5 == 0 { 0 } else { DATA + 0x6100 });
                c.bus.write32(grid + 0x18, r.next_u32() & 0x1f);
                fill(c, &mut r, DATA + 0x6000, 0x10);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn fe_state_reward_gallery_update_selection_matches() {
        check(
            ee::s04::f_001d0f30__generated,
            fe_state_reward_gallery_update_selection,
            200,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1d0f30, 6);
                c.hooks.miss = oc941_sentinel_miss;
                oc941_force_results(c, seed);
                let this = DATA + 0x1000;
                let icon = DATA + 0x5000;
                let grid = DATA + 0x5400;
                let vt_self = vtable(c, &mut r, &[0xd0, 0xd8, 0x130]);
                let vt_icon = vtable(c, &mut r, &[0x48]);
                let vt_grid = vtable(c, &mut r, &[0x58]);
                c.bus.write32(this + 8, vt_self);
                c.bus.write32(this + 0x5c, r.next_u32() & 0xf);
                c.bus.write32(this + 0x64, r.next_u32() & 0xf);
                c.bus.write32(this + 0x70, r.f32(10.0));
                c.bus.write32(this + 0x74, r.f32(10.0));
                c.bus
                    .write32(this + 0x8f8, if seed % 4 == 0 { 0 } else { icon });
                c.bus
                    .write32(this + 0x8fc, if seed % 3 == 0 { 0 } else { grid });
                c.bus.write32(icon + 8, vt_icon);
                fill(c, &mut r, icon + 0x40, 4);
                c.bus.write32(grid + 8, vt_grid);
                c.bus.write32(grid + 0x18, r.next_u32() & 0x1f);
                c.bus.write32(grid + 0x78, r.next_u32());
                c.bus.write32(grid + 0x7c, r.next_u32());
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// The page constructors' fixture: a zeroed gallery object with random
    /// junk in the arguments (the sign-extended byte in `a2`).
    pub(crate) fn oc941_ctor_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = fakes::machine_rt(c, seed, salt, 6);
        c.hooks.miss = oc941_sentinel_miss;
        oc941_force_results(c, seed);
        let this = DATA + 0x2000;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(DATA + 0x3000);
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = r.next_u32() as u64;
    }

    #[test]
    fn fe_state_reward_gallery_trading_cards_ctor_matches() {
        check(
            ee::s04::f_001d12c8__generated,
            fe_state_reward_gallery_trading_cards_ctor,
            200,
            &|c, seed| oc941_ctor_fixture(c, seed, 0x1d12c8),
        );
    }

    #[test]
    fn fe_state_reward_gallery_concept_arts_ctor_matches() {
        check(
            ee::s04::f_001d1460__generated,
            fe_state_reward_gallery_concept_arts_ctor,
            200,
            &|c, seed| oc941_ctor_fixture(c, seed, 0x1d1460),
        );
    }

    #[test]
    fn fe_state_reward_gallery_videos_ctor_matches() {
        check(
            ee::s04::f_001d15f8__generated,
            fe_state_reward_gallery_videos_ctor,
            200,
            &|c, seed| oc941_ctor_fixture(c, seed, 0x1d15f8),
        );
    }

    #[test]
    fn fe_state_reward_gallery_toys_ctor_matches() {
        check(
            ee::s04::f_001d1790__generated,
            fe_state_reward_gallery_toys_ctor,
            200,
            &|c, seed| oc941_ctor_fixture(c, seed, 0x1d1790),
        );
    }

    #[test]
    fn fe_state_reward_gallery_cheat_chars_ctor_matches() {
        check(
            ee::s04::f_001d1928__generated,
            fe_state_reward_gallery_cheat_chars_ctor,
            200,
            &|c, seed| oc941_ctor_fixture(c, seed, 0x1d1928),
        );
    }

    /// The fake callee body: log the call, then return the cell `idx` (only
    /// when it returns to its caller).
    pub(crate) fn oc942_call(c: &mut Cpu, target: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc == ra {
            c.g[V0][0] = sx(c.bus.read32(OC942_RES + 4 * idx));
        }
    }

    macro_rules! oc942_fakes {
        ($(($fake:ident, $prev:ident, $slot:path, $gen:path, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC942_GATE) != OC942_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc942_call(c, $addr, $idx);
                }
            )*
            pub(crate) fn oc942_install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc942_fakes!(
        (
            oc942_fk_0019da10,
            OC942_PREV_0019DA10,
            ee::s03::OVERRIDE_0019DA10,
            ee::s03::f_0019da10__generated,
            0x0019_da10,
            0
        ),
        (
            oc942_fk_0019e238,
            OC942_PREV_0019E238,
            ee::s03::OVERRIDE_0019E238,
            ee::s03::f_0019e238__generated,
            0x0019_e238,
            1
        ),
        (
            oc942_fk_0019e2b0,
            OC942_PREV_0019E2B0,
            ee::s03::OVERRIDE_0019E2B0,
            ee::s03::f_0019e2b0__generated,
            0x0019_e2b0,
            2
        ),
        (
            oc942_fk_0019dc20,
            OC942_PREV_0019DC20,
            ee::s03::OVERRIDE_0019DC20,
            ee::s03::f_0019dc20__generated,
            0x0019_dc20,
            3
        ),
        (
            oc942_fk_003b3d00,
            OC942_PREV_003B3D00,
            ee::s13::OVERRIDE_003B3D00,
            ee::s13::f_003b3d00__generated,
            0x003b_3d00,
            4
        ),
        (
            oc942_fk_00317670,
            OC942_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            5
        ),
        (
            oc942_fk_00397cc0,
            OC942_PREV_00397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0,
            6
        ),
        (
            oc942_fk_0039c870,
            OC942_PREV_0039C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870,
            7
        ),
        (
            oc942_fk_0039d860,
            OC942_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            8
        ),
        (
            oc942_fk_003a0c50,
            OC942_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50,
            9
        ),
        (
            oc942_fk_0028b180,
            OC942_PREV_0028B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180,
            10
        ),
        (
            oc942_fk_0028f140,
            OC942_PREV_0028F140,
            ee::s07::OVERRIDE_0028F140,
            ee::s07::f_0028f140__generated,
            0x0028_f140,
            11
        ),
        (
            oc942_fk_001d25e8,
            OC942_PREV_001D25E8,
            ee::s04::OVERRIDE_001D25E8,
            ee::s04::f_001d25e8__generated,
            0x001d_25e8,
            12
        ),
        (
            oc942_fk_001d32e0,
            OC942_PREV_001D32E0,
            ee::s04::OVERRIDE_001D32E0,
            ee::s04::f_001d32e0__generated,
            0x001d_32e0,
            13
        ),
        (
            oc942_fk_0014dc80,
            OC942_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            14
        ),
        (
            oc942_fk_0014dd58,
            OC942_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            15
        ),
        (
            oc942_fk_00147488,
            OC942_PREV_00147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488,
            16
        ),
        (
            oc942_fk_001a1cd0,
            OC942_PREV_001A1CD0,
            ee::s03::OVERRIDE_001A1CD0,
            ee::s03::f_001a1cd0__generated,
            0x001a_1cd0,
            17
        ),
        (
            oc942_fk_0039e2a0,
            OC942_PREV_0039E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039_e2a0,
            18
        ),
        (
            oc942_fk_0039e4c0,
            OC942_PREV_0039E4C0,
            ee::s13::OVERRIDE_0039E4C0,
            ee::s13::f_0039e4c0__generated,
            0x0039_e4c0,
            19
        ),
        (
            oc942_fk_00317d70,
            OC942_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70,
            20
        ),
        (
            oc942_fk_002533c8,
            OC942_PREV_002533C8,
            ee::s06::OVERRIDE_002533C8,
            ee::s06::f_002533c8__generated,
            0x0025_33c8,
            21
        ),
    );

    #[test]
    fn cfe_state_preview_reward_request_image_matches() {
        check(
            ee::s04::f_001d2150__generated,
            cfe_state_preview_reward_request_image,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_2150);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = (seed % 7) as u64;
                c.bus.write32(SELF + RWD_BANK, seed % 3);
                oc942_cell(c, 0, 0x55 + seed);
            },
        );
    }

    #[test]
    fn cfe_state_preview_reward_poll_image_load_matches() {
        check(
            ee::s04::f_001d2198__generated,
            cfe_state_preview_reward_poll_image_load,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_2198);
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + RWD_BANK, seed % 3);
                c.bus.write32(SELF + RWD_HANDLE, 0x55 + seed);
                oc942_entry(c, seed);
                oc942_cell(c, 1, if seed % 4 == 0 { 0 } else { 1 });
                oc942_cell(c, 2, ENTRY);
                oc942_cell(c, 3, seed);
                for k in 0..3 {
                    let slot = k * 4;
                    if seed % 5 == 0 {
                        c.bus.write32(SELF + RWD_SLOTS + slot, 0);
                    } else {
                        c.bus.write32(SELF + RWD_SLOTS + slot, OBJ_3);
                        c.bus.write32(SELF + RWD_OBJS + slot, OBJ2_2);
                    }
                }
                c.bus.write32(SELF + RWD_STATE, seed % 3);
            },
        );
    }

    #[test]
    fn cfe_state_video_player_init_matches() {
        check(
            ee::s04::f_001d23e0__generated,
            cfe_state_video_player_init,
            60,
            &|c, seed| {
                let _r = oc942_install_base(c, seed, 0x1d_23e0);
                c.g[A0][0] = sx(SELF);
                oc942_cell(c, 20, 0x0009_0000 + seed);
                oc942_cell(c, 21, 0x0009_1000 + seed);
            },
        );
    }
}
