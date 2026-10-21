//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small MPEG-2 decoder wrappers, rider metrics and AI
//! setup helpers, front-end state constructors and hooks, overlay-screen
//! hooks and a few manager setters, each with a differential test against
//! the generated body.
//!
//! Most of these are one-call wrappers: open the compiler's 0x10/0x20 frame,
//! save `ra` (and `s0` where the argument is needed twice), charge the
//! block's cycles, load a member or a vtable, and call one method. The ports
//! keep the frame, the cycle charge of every block, the values the original
//! leaves in `v0`/`v1` at each return, and the unwind check through
//! [`crate::call`].
//!
//! Four groups stand out:
//! - the `RCMP_DECODER_*` wrappers call a codec's virtual method through the
//!   object stored at `decoder + 0x14`;
//! - `Rider_GetSyncChecksum` is a pure FPU leaf that sums the two position
//!   and rotation vectors;
//! - `FEModule_UnloadRiderPreviews` / `FEModule_UpdateRiderPreviews` walk two
//!   rider-preview slots (0xce0 bytes apart) with a back-edge event check;
//! - the front-end `*_UpdateMoneyText` hooks format a value with
//!   `FE_FormatMoney` and hand the string to a `cUIText`.

use crate::call::{guest, Callee, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    // MPEG-2 decoder wrappers (s13).
    (0x003b_04f8, rcmp_decoder_set_codec, "RCMP_DECODER_SetCodec"),
    (
        0x003b_0580,
        rcmp_decoder_get_frame_number,
        "RCMP_DECODER_GetFrameNumber",
    ),
    (
        0x003b_05c0,
        rcmp_decoder_get_frame_rate,
        "RCMP_DECODER_GetFrameRate",
    ),
    (0x003b_1300, mpeg2_get_bits, "MPEG2_Get_Bits"),
    // Rider metrics and AI setup (s00/s01).
    (
        0x0011_75b8,
        rider_metrix_reset_stats,
        "RiderMetrix_ResetStats",
    ),
    (
        0x0011_99f8,
        rider_metrix_start_board_press,
        "RiderMetrix_StartBoardPress",
    ),
    (
        0x0012_5ad8,
        rider_get_sync_checksum,
        "Rider_GetSyncChecksum",
    ),
    (
        0x0012_8958,
        ai_setup_computer_riders,
        "AI_SetupComputerRiders",
    ),
    (
        0x0013_5eb0,
        rider_spin_sub_is_trick_anim12,
        "RiderSpinSub_IsTrickAnim12",
    ),
    (
        0x0013_5ef0,
        rider_spin_sub_is_trick_anim13,
        "RiderSpinSub_IsTrickAnim13",
    ),
    (
        0x0013_5f30,
        rider_spin_sub_is_trick_anim14,
        "RiderSpinSub_IsTrickAnim14",
    ),
    // Front-end / BE wrappers (s01/s02).
    (
        0x0014_73d0,
        benew_player_int_set_alt_character,
        "BENewPlayerInt_SetAltCharacter",
    ),
    (
        0x0015_87b8,
        be_reward_int_count_enabled_songs,
        "BERewardInt_CountEnabledSongs",
    ),
    (
        0x0016_0760,
        matrix_chase_camera_algorithm_ctor,
        "MatrixChaseCameraAlgorithm_ctor",
    ),
    (
        0x0016_1a60,
        chase_cam_controller_set_target,
        "ChaseCamController_SetTarget",
    ),
    (
        0x0018_05d8,
        keyboard_screen_set_text,
        "KeyboardScreen_SetText",
    ),
    (
        0x0018_6998,
        c_fe_state_event_select_post_input,
        "cFEStateEventSelect_PostInput",
    ),
    (
        0x0018_7230,
        c_fe_state_mem_card_reset_scan,
        "cFEStateMemCard_ResetScan",
    ),
    // Front-end state constructors (s02).
    (
        0x0018_a258,
        c_fe_state_options_sound_ctor,
        "cFEStateOptionsSound_ctor",
    ),
    (
        0x0018_c0e8,
        c_fe_state_options_hud_ctor,
        "cFEStateOptionsHUD_ctor",
    ),
    (
        0x0018_d240,
        c_fe_state_options_save_load_ctor,
        "cFEStateOptionsSaveLoad_ctor",
    ),
    (
        0x001b_fbb8,
        c_fe_state_online_main_menu_ctor,
        "cFEStateOnlineMainMenu_ctor",
    ),
    (
        0x001c_1c50,
        c_fe_state_net_ssx_world_ctor,
        "cFEStateNetSSXWorld_ctor",
    ),
    (
        0x001c_20f8,
        c_fe_state_net_quick_messages_ctor,
        "cFEStateNetQuickMessages_ctor",
    ),
    (
        0x001c_5018,
        c_fe_state_net_news_ctor,
        "cFEStateNetNews_ctor",
    ),
    // Front-end money/cash text hooks (s03/s04).
    (
        0x0019_85b0,
        c_fe_state_request_line_update_money,
        "cFEStateRequestLine_UpdateMoney",
    ),
    (
        0x0019_bba0,
        c_fe_state_char_equip_update_money_text,
        "cFEStateCharEquip_UpdateMoneyText",
    ),
    (
        0x001a_04c8,
        fe_module_unload_rider_previews,
        "FEModule_UnloadRiderPreviews",
    ),
    (
        0x001a_0608,
        fe_module_update_rider_previews,
        "FEModule_UpdateRiderPreviews",
    ),
    (
        0x001a_c408,
        c_fe_use_existing_account_validate_fields,
        "cFEUseExistingAccount_ValidateFields",
    ),
    (
        0x001b_4c00,
        c_fe_state_net_config_on_popup_event_passthrough,
        "cFEStateNetConfig_OnPopupEventPassthrough",
    ),
    (
        0x001c_e520,
        c_keyboard_popup_ensure_cursor_on_enabled_key,
        "cKeyboardPopup_EnsureCursorOnEnabledKey",
    ),
    (
        0x001d_0e78,
        c_fe_state_reward_gallery_update_cash_text,
        "cFEStateRewardGallery_UpdateCashText",
    ),
    (
        0x001d_1a60,
        c_fe_state_reward_gallery_cheat_chars_v27,
        "cFEStateRewardGalleryCheatChars_v27",
    ),
    (
        0x001d_2468,
        c_fe_state_video_player_render,
        "cFEStateVideoPlayer_Render",
    ),
    (
        0x001d_2638,
        c_fe_state_video_player_finish,
        "cFEStateVideoPlayer_Finish",
    ),
    (
        0x001d_3120,
        c_fe_state_mountain_room_update,
        "cFEStateMountainRoom_Update",
    ),
    (
        0x001d_9368,
        mem_card_state_base_set_mode_menu,
        "MemCardStateBase_SetModeMenu",
    ),
    (
        0x001f_49f8,
        c_fe_state_buy_attrib_on_activate,
        "cFEStateBuyAttrib_OnActivate",
    ),
    // Overlay screens (s04/s05).
    (
        0x001f_bce0,
        ov_screen_race_pre_update,
        "OVScreenRacePre_Update",
    ),
    (
        0x001f_d0d8,
        ov_screen_multi_freestyle_pre_on_activate,
        "OVScreenMultiFreestylePre_OnActivate",
    ),
    (
        0x001f_d150,
        ov_screen_multi_freestyle_pre_update,
        "OVScreenMultiFreestylePre_Update",
    ),
    (
        0x001f_dbf0,
        ov_screen_save_replay_ctor,
        "OVScreenSaveReplay_ctor",
    ),
    (
        0x001f_df00,
        ov_screen_save_replay_v33,
        "OVScreenSaveReplay_v33",
    ),
    (
        0x001f_e968,
        ov_screen_save_records_ctor,
        "OVScreenSaveRecords_ctor",
    ),
    (
        0x001f_ec60,
        ov_screen_save_records_v33,
        "OVScreenSaveRecords_v33",
    ),
    (
        0x0020_1868,
        event_select_hide_map_pic,
        "EventSelect_HideMapPic",
    ),
    (
        0x0020_8b78,
        ov_screen_session_shutdown,
        "OVScreenSession_Shutdown",
    ),
    (0x0021_24c8, ov_screen_autosave_v33, "OVScreenAutosave_v33"),
    (
        0x0022_de58,
        section_man_request_sky,
        "SectionMan_RequestSky",
    ),
];

// ---------------------------------------------------------------------------
// Callees (the generated crate's readable aliases, which dispatch through
// their override slot exactly as the generated `jal`/`call_indirect` does).

mod callee {
    use super::{ee, Callee};

    pub const MPEG2_SHOW_BITS: Callee = Callee {
        addr: 0x003b_1258,
        f: ee::s13::MPEG2_Show_Bits,
    };
    pub const MPEG2_FLUSH_BUFFER: Callee = Callee {
        addr: 0x003b_1290,
        f: ee::s13::MPEG2_Flush_Buffer,
    };
    pub const AI_DESTROY_MISSION_RIDERS: Callee = Callee {
        addr: 0x0012_af68,
        f: ee::s00::AI_DestroyMissionRiders,
    };
    pub const AI_CREATE_COMPUTER_RIDERS: Callee = Callee {
        addr: 0x0012_9e20,
        f: ee::s00::AI_CreateComputerRiders,
    };
    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY: Callee = Callee {
        addr: 0x0031_1ae8,
        f: ee::s10::AnimController_GetLayerAnimEntry,
    };
    pub const BE_NEW_PLAYER_INT_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::BENewPlayerInt_GetSlot,
    };
    pub const BE_REWARD_INT_COUNT_SET_BITS: Callee = Callee {
        addr: 0x0015_7390,
        f: ee::s01::BERewardInt_CountSetBits,
    };
    pub const CHASE_CAMERA_ALGORITHM_CTOR: Callee = Callee {
        addr: 0x0016_2318,
        f: ee::s01::ChaseCameraAlgorithm_ctor,
    };
    pub const BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };
    pub const RIDER_METRIX_CLEAR_PEAK_A4: Callee = Callee {
        addr: 0x0011_76f8,
        f: ee::s00::RiderMetrix_ClearPeakA4,
    };
    pub const EA_MEM_SET: Callee = Callee {
        addr: 0x003e_6448,
        f: ee::s14::EA_MemSet,
    };
    pub const MC_MANAGER_GET: Callee = Callee {
        addr: 0x0022_7f80,
        f: ee::s05::MCManager_Get,
    };
    pub const MC_MANAGER_BEGIN_LOAD: Callee = Callee {
        addr: 0x0023_c8f0,
        f: ee::s06::MCManager_BeginLoad,
    };
    pub const UI_STATE_CTOR: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::UIState_ctor,
    };
    pub const FE_FORMAT_MONEY: Callee = Callee {
        addr: 0x0019_8af0,
        f: ee::s03::FE_FormatMoney,
    };
    pub const UI_TEXT_SET_STRING: Callee = Callee {
        addr: 0x003a_0c50,
        f: ee::s13::UIText_SetString,
    };
    pub const FE_MODULE_RIDER_PREVIEW_UNLOAD: Callee = Callee {
        addr: 0x0019_eba0,
        f: ee::s03::FEModule_RiderPreviewUnload,
    };
    pub const FE_MODULE_RIDER_PREVIEW_UPDATE: Callee = Callee {
        addr: 0x0019_f138,
        f: ee::s03::FEModule_RiderPreviewUpdate,
    };
    pub const CHECK_ACCOUNT_NAME: Callee = Callee {
        addr: 0x001a_c360,
        f: ee::s03::cFEUseExistingAccount_CheckAccountName,
    };
    pub const CHECK_PASSWORD: Callee = Callee {
        addr: 0x001a_c2b8,
        f: ee::s03::cFEUseExistingAccount_CheckPassword,
    };
    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };
    pub const NET_FE_STATE_CTOR: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::NetFEState_ctor,
    };
    pub const KEYBOARD_POPUP_IS_KEY_ENABLED: Callee = Callee {
        addr: 0x001c_e560,
        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
    };
    pub const KEYBOARD_POPUP_MOVE_DOWN: Callee = Callee {
        addr: 0x001c_b9b0,
        f: ee::s04::cKeyboardPopup_MoveDown,
    };
    pub const EVENT_SELECT_POST_INPUT: Callee = Callee {
        addr: 0x0020_2770,
        f: ee::s05::EventSelect_PostInput,
    };
    pub const UI_STATE_POST_INPUT: Callee = Callee {
        addr: 0x0039_e508,
        f: ee::s13::UIState_PostInput,
    };
    pub const STR_HASH: Callee = Callee {
        addr: 0x0031_7670,
        f: ee::s10::StrHash,
    };
    pub const UI_TEXTURE_SET_FIND: Callee = Callee {
        addr: 0x0039_8380,
        f: ee::s13::UITextureSet_Find,
    };
    pub const VID_ENGINE_PLAYER_RENDER: Callee = Callee {
        addr: 0x0025_3890,
        f: ee::s06::VidEnginePlayer_Render,
    };
    pub const UI_STATE_RENDER: Callee = Callee {
        addr: 0x0039_e6b8,
        f: ee::s13::UIState_Render,
    };
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::SSXAudioSystem_GetInstance,
    };
    pub const MUSIC_PLAYER_RESUME: Callee = Callee {
        addr: 0x002b_3a98,
        f: ee::s08::MusicPlayer_Resume,
    };
    pub const MOUNTAIN_ROOM_POLL_IMAGE_LOAD: Callee = Callee {
        addr: 0x001d_31c0,
        f: ee::s04::cFEStateMountainRoom_PollImageLoad,
    };
    pub const UI_STATE_UPDATE: Callee = Callee {
        addr: 0x0039_e510,
        f: ee::s13::UIState_Update,
    };
    pub const C_FE_STATE_ON_ACTIVATE: Callee = Callee {
        addr: 0x0018_6518,
        f: ee::s02::cFEState_OnActivate,
    };
    pub const C_OV_STATE_OVERLAY_UPDATE: Callee = Callee {
        addr: 0x0020_e900,
        f: ee::s05::cOVStateOverlay_Update,
    };
    pub const UI_STATE_ON_ACTIVATE: Callee = Callee {
        addr: 0x0039_e4c0,
        f: ee::s13::UIState_OnActivate,
    };
    pub const PLAY_FE_SOUND: Callee = Callee {
        addr: 0x0029_4f78,
        f: ee::s08::SSXAudioSystem_PlayFESound,
    };
    pub const MEM_CARD_STATE_BASE_CTOR_2: Callee = Callee {
        addr: 0x001d_5330,
        f: ee::s04::MemCardStateBase_ctor_2,
    };
    pub const UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS: Callee = Callee {
        addr: 0x0039_f718,
        f: ee::s13::UIStateMgr_CloseTopStateAndPopups,
    };
    pub const UI_STATE_MGR_CLOSE_TOP_STATES: Callee = Callee {
        addr: 0x0039_f190,
        f: ee::s13::UIStateMgr_CloseTopStates,
    };
    pub const OV_SCREEN_SESSION_DESTROY_INDICATORS: Callee = Callee {
        addr: 0x0020_a088,
        f: ee::s05::OVScreenSession_DestroyIndicators,
    };
    pub const C_OV_STATE_OVERLAY_SHUTDOWN: Callee = Callee {
        addr: 0x0020_a430,
        f: ee::s05::cOVStateOverlay_Shutdown,
    };
    pub const FE_ASYNC_MANAGER_CANCEL: Callee = Callee {
        addr: 0x001a_38c0,
        f: ee::s03::FEAsyncManager_Cancel,
    };
    pub const SECTION_MAN_REQUEST_ACTIVATE: Callee = Callee {
        addr: 0x0022_d478,
        f: ee::s05::SectionMan_RequestActivate,
    };
    pub const STRCPY: Callee = Callee {
        addr: 0x0041_66f4,
        f: ee::s15::strcpy,
    };
    pub const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::strlen,
    };
}

/// `lw` at `base + off` (sign-extended), the generators' most common load.
#[inline(always)]
fn lw(c: &mut Cpu, base: u32, off: u32) -> u64 {
    Addr(base.wrapping_add(off)).lw(c)
}

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

// ---------------------------------------------------------------------------
// MPEG-2 decoder wrappers (s13): the codec is the object at `decoder + 0x14`
/// and the wrappers call one of its virtual methods.

/// `void RCMP_DECODER_SetCodec(Decoder* d, Codec* codec, int a2)`
/// (0x3b04f8): remember the codec and `a2`, then `codec->v02(codec, a2)`.
pub fn rcmp_decoder_set_codec(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let decoder = c.g[A0][0];
        c.g[V0][0] = decoder;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        // The arguments are moved, not truncated: `a0 = a1`.
        c.g[A0][0] = c.g[A1][0];
        let codec = c.g[A0][0];
        Addr((decoder as u32).wrapping_add(0x14)).sw(c, c.g[A0][0]);
        c.g[A1][0] = c.g[V0][0];
        Addr(decoder as u32).sw(c, c.g[A2][0]);
        let vt = Addr(codec as u32).lw(c);
        c.g[V1][0] = vt;
        let m = VSlot::read(c, vt, 0x10);
        c.g[A3][0] = m.delta;
        c.g[V0][0] = m.func;
        c.g[A0][0] = m.this(codec);
        crate::call::call_ptr(c, m.func as u32, 0x003b_0528)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int RCMP_DECODER_GetFrameNumber(Decoder* d)` (0x3b0580): the codec's
/// frame number, or 0 before a codec is set.
pub fn rcmp_decoder_get_frame_number(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let dec = crate::abi::arg(c, 0);
        let obj = lw(c, dec, 0x14);
        c.g[A1][0] = obj;
        let missing = obj == 0;
        c.g[V0][0] = 0;
        if missing {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 7;
            let vt = Addr(obj as u32).lw(c);
            c.g[V1][0] = vt;
            let m = VSlot::read(c, vt, 0x20);
            c.g[A0][0] = m.delta;
            c.g[V0][0] = m.func;
            c.g[A0][0] = m.this(obj);
            crate::call::call_ptr(c, m.func as u32, 0x003b_05a8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `float RCMP_DECODER_GetFrameRate(Decoder* d)` (0x3b05c0): the codec's
/// frame rate, or 0.0 before a codec is set (the result stays in `v0` when
/// there is none, as the original leaves it).
pub fn rcmp_decoder_get_frame_rate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let dec = crate::abi::arg(c, 0);
        let obj = lw(c, dec, 0x14);
        c.g[A1][0] = obj;
        let missing = obj == 0;
        frame.restore_ra(c, 0);
        if missing {
            c.cycles += 1;
            c.mtc1(0, 0);
        } else {
            c.cycles += 7;
            let vt = Addr(obj as u32).lw(c);
            c.g[V0][0] = vt;
            let m = VSlot::read(c, vt, 0x28);
            c.g[A0][0] = m.delta;
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(obj);
            crate::call::call_ptr(c, m.func as u32, 0x003b_05e8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `unsigned MPEG2_Get_Bits(reader* r)` (0x3b1300): peek at the next bits
/// (`MPEG2_Show_Bits`), then consume them (`MPEG2_Flush_Buffer`) and return
/// the peeked value.
pub fn mpeg2_get_bits(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MPEG2_SHOW_BITS.call(c, 0x003b_1318)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::MPEG2_FLUSH_BUFFER.call(c, 0x003b_1324)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider metrics and AI setup.

/// `void RiderMetrix_ResetStats(RiderMetrix* m)` (0x1175b8): clear the 0xac
/// byte stat block at `+0xfc`, mark `+0x18c` as "no trick" (−1) and carry
/// `+0x1c8` into `+0x198`.
pub fn rider_metrix_reset_stats(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0xac;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(crate::abi::r32(c, S0), 0xfc));
        callee::EA_MEM_SET.call(c, 0x0011_75d8)?;
        let this = crate::abi::r32(c, S0);
        c.g[V1][0] = lw(c, this, 0x1c8);
        c.g[V0][0] = u64::MAX;
        Addr(p(this, 0x18c)).sw(c, c.g[V0][0]);
        Addr(p(this, 0x198)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float RiderMetrix_StartBoardPress(RiderMetrix* m, int a1)` (0x1199f8):
/// record the press (`+0x10`, `+0x28`), keep the peak at `+0x2c` at least
/// zero, clear the peak tracker and return 0.0 (f0 and `f20`).
pub fn rider_metrix_start_board_press(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x10);
        c.mtc1(0, 20);
        let this = crate::abi::r32(c, V0);
        c.lwc1(0, p(this, 0x2c));
        Addr(p(this, 0x28)).sw(c, c.g[A1][0]);
        c.max_s(0, 0, 20);
        Addr(p(this, 0x10)).sw(c, c.g[A1][0]);
        c.swc1(0, p(this, 0x2c));
        callee::RIDER_METRIX_CLEAR_PEAK_A4.call(c, 0x0011_9a24)?;
        c.mov_s(0, 20);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int Rider_GetSyncChecksum(const Rider* r)` (0x125ad8): the bit pattern
/// of `sum(position) + sum(rotation)` (the two vectors at `+0x110` and
/// `+0x120`, w included). Leaf: no frame, no calls.
pub fn rider_get_sync_checksum(c: &mut Cpu) {
    c.cycles += 16;
    let r = crate::abi::arg(c, 0);
    let f1 = Addr(p(r, 0x124)).read_fpu(c);
    c.f[1] = f1.0;
    let mut f0 = Addr(p(r, 0x120)).read_fpu(c);
    c.f[0] = f0.0;
    let mut f2 = Addr(p(r, 0x110)).read_fpu(c);
    c.f[2] = f2.0;
    f0 = f0 + f1;
    c.f[0] = f0.0;
    let f3 = Addr(p(r, 0x114)).read_fpu(c);
    c.f[3] = f3.0;
    let f4 = Addr(p(r, 0x128)).read_fpu(c);
    c.f[4] = f4.0;
    f2 = f2 + f3;
    c.f[2] = f2.0;
    let f5 = Addr(p(r, 0x12c)).read_fpu(c);
    c.f[5] = f5.0;
    f0 = f0 + f4;
    c.f[0] = f0.0;
    let f1 = Addr(p(r, 0x118)).read_fpu(c);
    c.f[1] = f1.0;
    f2 = f2 + f1;
    c.f[2] = f2.0;
    f0 = f0 + f5;
    c.f[0] = f0.0;
    let f2 = f2 + f0;
    c.f[2] = f2.0;
    c.mfc1(V0, 2);
    crate::abi::ret(c);
}

/// `void AI_SetupComputerRiders(cAI* ai)` (0x128958): kill the previous
/// computer riders when there were any, then create a fresh set.
pub fn ai_setup_computer_riders(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        Addr(p(this, 0x94)).sw(c, 0);
        c.g[V0][0] = lw(c, this, 0x88);
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            callee::AI_DESTROY_MISSION_RIDERS.call(c, 0x0012_8980)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::AI_CREATE_COMPUTER_RIDERS.call(c, 0x0012_8988)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The three `RiderSpinSub_IsTrickAnim1x` predicates: `kind` is the spin
/// sub-state (`+0x4`) and `trick` the animation id they test for. The
/// animation entry comes from the rider's controller at `+0x58` → `+0x784`.
fn rider_spin_sub_is_trick_anim(c: &mut Cpu, ret: u32, kind: u64, trick: u64) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = crate::abi::arg(c, 0);
        c.g[V0][0] = kind;
        c.g[V1][0] = lw(c, this, 4);
        let same = c.g[V1][0] == c.g[V0][0];
        c.g[A1][0] = 0;
        if same {
            c.cycles += 6;
            c.g[V0][0] = lw(c, this, 0x58);
            c.g[A1][0] = 2;
            let ctrl = crate::abi::r32(c, V0);
            c.g[A0][0] = lw(c, ctrl, 0x784);
            callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY.call(c, ret)?;
            c.g[V0][0] ^= trick;
            c.g[A1][0] = (c.g[V0][0] < 1) as u64;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[A1][0];
        frame.close(c);
        Ok(())
    })
}

/// `int RiderSpinSub_IsTrickAnim12(SpinSub* s)` (0x135eb0): sub-state 2 with
/// animation entry 0x12. Returns 0 instead of `gcc`'s `1` when the entry is
/// not a trick (`v0` is set from `v1`, as the original does).
pub fn rider_spin_sub_is_trick_anim12(c: &mut Cpu) {
    rider_spin_sub_is_trick_anim(c, 0x0013_5ed8, 2, 0x12)
}

/// `int RiderSpinSub_IsTrickAnim13(SpinSub* s)` (0x135ef0): sub-state 4 with
/// animation entry 0x13.
pub fn rider_spin_sub_is_trick_anim13(c: &mut Cpu) {
    rider_spin_sub_is_trick_anim(c, 0x0013_5f18, 4, 0x13)
}

/// `int RiderSpinSub_IsTrickAnim14(SpinSub* s)` (0x135f30): sub-state 4 with
/// animation entry 0x14.
pub fn rider_spin_sub_is_trick_anim14(c: &mut Cpu) {
    rider_spin_sub_is_trick_anim(c, 0x0013_5f58, 4, 0x14)
}

// ---------------------------------------------------------------------------
// Front end and BE wrappers.

/// `void BENewPlayerInt_SetAltCharacter(int index, int value)` (0x1473d0):
/// store `value` in alternates' character byte (`D_00534FE0[id].+0x12`).
pub fn benew_player_int_set_alt_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        callee::BE_NEW_PLAYER_INT_GET_SLOT.call(c, 0x0014_73e8)?;
        c.g[A0][0] = 0x1c;
        c.g[V1][0] = 0x53_0000;
        // `mult v0, v0, a0`: the per-player stride.
        c.mult(0, 2, 2, 4);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_add(0x4fe0));
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(crate::abi::r32(c, V0).wrapping_add(crate::abi::r32(c, V1)));
        Addr(p(crate::abi::r32(c, V0), 0x12)).sb(c, c.g[S0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int BERewardInt_CountEnabledSongs(BERewardInt* r, int a1, int a2)`
/// (0x1587b8): count the set bits of the 0x40-byte song mask of reward
/// `a1 * 0x9b50 + a2 * 0xf88 + 0xf78` in `D_004A6CA8`.
pub fn be_reward_int_count_enabled_songs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[V0][0] = 0xf88;
        c.g[A3][0] = 0x9b50;
        // `mult a2, a2, v0` and `mult v1, a1, a3`.
        c.mult(0, 6, 6, 2);
        c.mult(0, 3, 5, 7);
        c.g[V0][0] = 0x4a_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(crate::abi::r32(c, V0).wrapping_add(0x6ca8));
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x40;
        c.g[A2][0] = sx(crate::abi::r32(c, A2).wrapping_add(crate::abi::r32(c, V0)));
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_add(crate::abi::r32(c, A2)));
        c.g[A2][0] = sx(crate::abi::r32(c, V1).wrapping_add(0xf78));
        callee::BE_REWARD_INT_COUNT_SET_BITS.call(c, 0x0015_87ec)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* MatrixChaseCameraAlgorithm_ctor(void* self)` (0x160760): build
/// the base chase-camera algorithm, then the Matrix variant's class id,
/// vtable and reset flag.
pub fn matrix_chase_camera_algorithm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CHASE_CAMERA_ALGORITHM_CTOR.call(c, 0x0016_0774)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = 0x29;
        c.g[V1][0] = 0x46_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_sub(0x45a8));
        Addr(p(this, 0x10)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(this, 0x390)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ChaseCamController_SetTarget(Ctrl* c, void* p)` (0x161a60): delete
/// the previous target and install `p`.
pub fn chase_cam_controller_set_target(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = crate::abi::r32(c, S0);
        c.g[A0][0] = lw(c, this, 0x20);
        let old = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !old {
            c.cycles += 2;
            callee::BUILTIN_DELETE.call(c, 0x0016_1a88)?;
        }
        c.cycles += 6;
        Addr(p(crate::abi::r32(c, S0), 0x20)).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void KeyboardScreen_SetText(Screen* s, const char* text)` (0x1805d8):
/// copy `text` into the 0x80-byte buffer at `+0xec` and cache its length at
/// `+0x16c`.
pub fn keyboard_screen_set_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx(p(crate::abi::r32(c, S1), 0xec));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::STRCPY.call(c, 0x0018_05f8)?;
        c.g[A0][0] = c.g[S0][0];
        callee::STRLEN.call(c, 0x0018_0600)?;
        Addr(p(crate::abi::r32(c, S1), 0x16c)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateEventSelect_PostInput(State* s, int a1)` (0x186998):
/// forward the input to the event-select panel, then to the UI state.
pub fn c_fe_state_event_select_post_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(crate::abi::r32(c, S0), 0x48));
        callee::EVENT_SELECT_POST_INPUT.call(c, 0x0018_69b8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::UI_STATE_POST_INPUT.call(c, 0x0018_69c4)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMemCard_ResetScan(void)` (0x187230): restart the memory
/// card manager's scan (mode 2) with no slot selected.
pub fn c_fe_state_mem_card_reset_scan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        let g = crate::abi::r32(c, GP);
        c.g[A0][0] = lw(c, g, 0xfffff7b8); // gp - 0x848
        frame.save_ra(c, 0);
        callee::MC_MANAGER_GET.call(c, 0x0018_7244)?;
        let g = crate::abi::r32(c, GP);
        let mgr = lw(c, g, 0xffffe3c8); // gp - 0x1c38
        c.g[A2][0] = mgr;
        c.g[V1][0] = u64::MAX;
        Addr(crate::abi::r32(c, GP).wrapping_sub(0x1c34)).sw(c, 0);
        c.g[A0][0] = c.g[V0][0];
        Addr(p(mgr as u32, 0x22c)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 2;
        Addr(p(mgr as u32, 0x234)).sw(c, 0);
        callee::MC_MANAGER_BEGIN_LOAD.call(c, 0x0018_7264)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end state constructors.

/// `void* cFEStateOptionsSound_ctor(void* self)` (0x18a258): UI state with
/// class id 0x1d, its vtable, and the global "sound changed" flag cleared.
pub fn c_fe_state_options_sound_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_STATE_CTOR.call(c, 0x0018_a26c)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = 0x1d;
        c.g[V1][0] = 0x47_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_sub(0x39e0));
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(crate::abi::r32(c, GP).wrapping_sub(0x1c1c)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateOptionsHUD_ctor(void* self)` (0x18c0e8): class id 0x1e.
pub fn c_fe_state_options_hud_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_STATE_CTOR.call(c, 0x0018_c0fc)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = 0x1e;
        c.g[V1][0] = 0x47_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_sub(0x3ab0));
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(crate::abi::r32(c, GP).wrapping_sub(0x1c18)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateOptionsSaveLoad_ctor(void* self)` (0x18d240): class id
/// 0x20, `+0x48` (the "scanning" flag) cleared.
pub fn c_fe_state_options_save_load_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_STATE_CTOR.call(c, 0x0018_d254)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = 0x20;
        c.g[V1][0] = 0x47_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_sub(0x3c50));
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(this, 0x48)).sb(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The four `NetFEState` constructors: base constructor, class id, vtable,
/// and (for the online main menu) an explicit "no parent" argument.
fn net_fe_state_ctor(c: &mut Cpu, id: u32, vt: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::NET_FE_STATE_CTOR.call(c, ret)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = id as u64;
        c.g[V1][0] = 0x47_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(vt);
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateOnlineMainMenu_ctor(void* self)` (0x1bfbb8): class 0x3b.
pub fn c_fe_state_online_main_menu_ctor(c: &mut Cpu) {
    net_fe_state_ctor(c, 0x3b, 0x46_ad48, 0x001b_fbd0)
}

/// `void* cFEStateNetSSXWorld_ctor(void* self)` (0x1c1c50): class 0x4a.
pub fn c_fe_state_net_ssx_world_ctor(c: &mut Cpu) {
    net_fe_state_ctor(c, 0x4a, 0x46_aa18, 0x001c_1c68)
}

/// `void* cFEStateNetQuickMessages_ctor(void* self)` (0x1c20f8): class 0x4b.
pub fn c_fe_state_net_quick_messages_ctor(c: &mut Cpu) {
    net_fe_state_ctor(c, 0x4b, 0x46_a908, 0x001c_2110)
}

/// `void* cFEStateNetNews_ctor(void* self)` (0x1c5018): class 0x43.
pub fn c_fe_state_net_news_ctor(c: &mut Cpu) {
    net_fe_state_ctor(c, 0x43, 0x46_8168, 0x001c_5030)
}

// ---------------------------------------------------------------------------
// Money / cash text hooks: format the value and hand the string to a text.

/// `void cFEStateRequestLine_UpdateMoney(State* s)` (0x1985b0): set the
/// money text (`+0x1e0`) to the formatted value at `+0x168`.
pub fn c_fe_state_request_line_update_money(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0x1e0);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            c.g[A0][0] = lw(c, this, 0x168);
            callee::FE_FORMAT_MONEY.call(c, 0x0019_85d4)?;
            let this = crate::abi::r32(c, S0);
            c.g[A0][0] = lw(c, this, 0x1e0);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_STRING.call(c, 0x0019_85e0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_UpdateMoneyText(State* s)` (0x19bba0): the same
/// with the text at `+0x64` and the value at `+0xc0`.
pub fn c_fe_state_char_equip_update_money_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0x64);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            c.g[A0][0] = lw(c, this, 0xc0);
            callee::FE_FORMAT_MONEY.call(c, 0x0019_bbc4)?;
            let this = crate::abi::r32(c, S0);
            c.g[A0][0] = lw(c, this, 0x64);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_STRING.call(c, 0x0019_bbd0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
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
            callee::UI_TEXT_SET_STRING.call(c, 0x001d_0ea8)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider previews: two slots 0xce0 bytes apart.

/// `void FEModule_UnloadRiderPreviews(FEModule* m)` (0x1a04c8): unload the
/// two rider preview slots at `+0x19c0` and `+0x26a0` (the loop the compiler
/// left, with an event check at the back edge).
pub fn fe_module_unload_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(p(crate::abi::r32(c, S0), 0x19c0));
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            callee::FE_MODULE_RIDER_PREVIEW_UNLOAD.call(c, 0x001a_04e8)?;
            c.g[S0][0] = sx(p(crate::abi::r32(c, S0), 0xce0));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            frame.restore_ra(c, 0);
            if !more {
                c.cycles += 4;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_04e0);
            }
        }
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_UpdateRiderPreviews(FEModule* m)` (0x1a0608): the same
/// walk with `FEModule_RiderPreviewUpdate`.
pub fn fe_module_update_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(p(crate::abi::r32(c, S0), 0x19c0));
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            callee::FE_MODULE_RIDER_PREVIEW_UPDATE.call(c, 0x001a_0628)?;
            c.g[S0][0] = sx(p(crate::abi::r32(c, S0), 0xce0));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            frame.restore_ra(c, 0);
            if !more {
                c.cycles += 4;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_0620);
            }
        }
        frame.close(c);
        Ok(())
    })
}

/// `int cFEUseExistingAccount_ValidateFields(State* s)` (0x1ac408): true
/// when both the account name and the password are non-empty.
pub fn c_fe_use_existing_account_validate_fields(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CHECK_ACCOUNT_NAME.call(c, 0x001a_c41c)?;
        let have_name = c.g[V0][0] != 0;
        c.g[V1][0] = 0;
        if have_name {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::CHECK_PASSWORD.call(c, 0x001a_c42c)?;
            c.g[V1][0] = 1;
            if c.g[V0][0] == 0 {
                c.g[V1][0] = 0;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateNetConfig_OnPopupEventPassthrough(State* s, int a1, int a2)`
/// (0x1b4c00): forward child events, except that the two popup event ids
/// (0xf and 0x14) take the "own popup" path with one extra cycle.
pub fn c_fe_state_net_config_on_popup_event_passthrough(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xf;
        let popup = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if !popup {
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            if c.g[A2][0] != c.g[V0][0] {
                // The passthrough path.
                c.cycles += 3;
                callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_4c34)?;
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 4;
        callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001b_4c24)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cKeyboardPopup_EnsureCursorOnEnabledKey(Popup* p)` (0x1ce520):
/// when the key under the cursor is disabled, move down one row.
pub fn c_keyboard_popup_ensure_cursor_on_enabled_key(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[A1][0] = lw(c, this, 0x140);
        callee::KEYBOARD_POPUP_IS_KEY_ENABLED.call(c, 0x001c_e53c)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            callee::KEYBOARD_POPUP_MOVE_DOWN.call(c, 0x001c_e550)?;
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
        callee::UI_STATE_MGR_CLOSE_TOP_STATES.call(c, 0x001d_2668)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMountainRoom_Update(State* s)` (0x1d3120): poll the room
/// image load unless `+0xbc` is all ones, then update the UI.
pub fn c_fe_state_mountain_room_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0xbc);
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD.call(c, 0x001d_3148)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::UI_STATE_UPDATE.call(c, 0x001d_3150)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_SetModeMenu(State* s)` (0x1d9368): enter mode 1
/// once (`+0x214`), calling the state's own `v` slot (+0x118) with 1.
pub fn mem_card_state_base_set_mode_menu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        let this = crate::abi::r32(c, A0);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 1;
        c.g[V0][0] = lw(c, this, 0x214);
        let already = c.g[V0][0] == c.g[V1][0];
        c.g[A1][0] = 1;
        if !already {
            c.cycles += 6;
            Addr(p(this, 0x214)).sw(c, c.g[V1][0]);
            c.g[V0][0] = lw(c, this, 8);
            let vt = c.g[V0][0];
            let m = VSlot::read(c, vt, 0x118);
            c.g[A0][0] = m.delta;
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(c.g[A2][0]);
            crate::call::call_ptr(c, m.func as u32, 0x001d_939c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_OnActivate(State* s)` (0x1f49f8): activate the
/// state, then the selected attribute's "show" method (slot +0x38 of the
/// object at `+0xac`).
pub fn c_fe_state_buy_attrib_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::C_FE_STATE_ON_ACTIVATE.call(c, 0x001f_4a0c)?;
        let this = crate::abi::r32(c, S0);
        let obj = lw(c, this, 0xac);
        c.g[A2][0] = obj;
        c.g[A1][0] = 1;
        let vt = lw(c, obj as u32, 8);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 0x38);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = m.this(obj);
        crate::call::call_ptr(c, m.func as u32, 0x001f_4a28)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Overlay screens.

/// The shared tail of `OVScreenRacePre_Update` / `OVScreenMultiFreestylePre_Update`:
/// update the overlay base, then flag the loaded race "finished" once.
fn ov_screen_pre_update_common(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::C_OV_STATE_OVERLAY_UPDATE.call(c, ret)?;
        let g = crate::abi::r32(c, GP);
        let race = lw(c, g, 0xfffffdfc); // gp - 0x204
        c.g[A0][0] = race;
        let missing = race == 0;
        frame.restore_ra(c, 0);
        if !missing {
            c.cycles += 3;
            c.g[V0][0] = lw(c, race as u32, 0x64);
            let done = c.g[V0][0] != 0;
            c.g[V0][0] = u64::MAX;
            if !done {
                c.cycles += 4;
                c.g[V1][0] = 1;
                Addr(p(race as u32, 0x64)).sw(c, c.g[V0][0]);
                Addr(crate::abi::r32(c, GP).wrapping_sub(0x9f4)).sw(c, c.g[V1][0]);
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenRacePre_Update(Screen* s)` (0x1fbce0).
pub fn ov_screen_race_pre_update(c: &mut Cpu) {
    ov_screen_pre_update_common(c, 0x001f_bcf0)
}

/// `void OVScreenMultiFreestylePre_Update(Screen* s)` (0x1fd150).
pub fn ov_screen_multi_freestyle_pre_update(c: &mut Cpu) {
    ov_screen_pre_update_common(c, 0x001f_d160)
}

/// `void OVScreenMultiFreestylePre_OnActivate(Screen* s)` (0x1fd0d8):
/// activate the overlay, mark the loaded race with state 0x4b0, and play the
/// front-end "activate" sound (0xe).
pub fn ov_screen_multi_freestyle_pre_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::UI_STATE_ON_ACTIVATE.call(c, 0x001f_d0e8)?;
        let g = crate::abi::r32(c, GP);
        let race = lw(c, g, 0xfffffdfc); // gp - 0x204
        c.g[V1][0] = race;
        c.g[V0][0] = 0x4b0;
        if race != 0 {
            c.cycles += 1;
            Addr((race as u32).wrapping_add(0x64)).sw(c, c.g[V0][0]);
        }
        c.cycles += 8;
        callee::SSX_AUDIO_SYSTEM_GET_INSTANCE.call(c, 0x001f_d100)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xe;
        callee::PLAY_FE_SOUND.call(c, 0x001f_d10c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* OVScreenSaveReplay_ctor(void* self, int a1, int a2)` (0x1fdbf0):
/// memory-card base state (variant 1), class vtable, `+0x22c` cleared.
pub fn ov_screen_save_replay_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 1;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MEM_CARD_STATE_BASE_CTOR_2.call(c, 0x001f_dc0c)?;
        let this = crate::abi::r32(c, S0);
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_add(0x2440));
        Addr(p(this, 0x22c)).sw(c, 0);
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* OVScreenSaveRecords_ctor(void* self, int a1, int a2)` (0x1fe968):
/// the same with variant 2 and the records vtable.
pub fn ov_screen_save_records_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 2;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MEM_CARD_STATE_BASE_CTOR_2.call(c, 0x001f_e984)?;
        let this = crate::abi::r32(c, S0);
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_add(0x22b8));
        Addr(p(this, 0x22c)).sw(c, 0);
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The three "close the top state once" hooks: when `+0x1ac` is clear, close
/// the state at `*(self + 0x10) + 0x18`, then set `+0x1ac`.
fn close_top_state_once(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0x1ac);
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !done {
            c.cycles += 4;
            c.g[A0][0] = lw(c, this, 0x10);
            c.g[A0][0] = sx(p(crate::abi::r32(c, A0), 0x18));
            callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS.call(c, ret)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        Addr(p(crate::abi::r32(c, S0), 0x1ac)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSaveReplay_v33(Screen* s)` (0x1fdf00).
pub fn ov_screen_save_replay_v33(c: &mut Cpu) {
    close_top_state_once(c, 0x001f_df28)
}

/// `void OVScreenSaveRecords_v33(Screen* s)` (0x1fec60).
pub fn ov_screen_save_records_v33(c: &mut Cpu) {
    close_top_state_once(c, 0x001f_ec88)
}

/// `void OVScreenAutosave_v33(Screen* s)` (0x2124c8).
pub fn ov_screen_autosave_v33(c: &mut Cpu) {
    close_top_state_once(c, 0x0021_24f0)
}

/// `void EventSelect_HideMapPic(EventSelect* s)` (0x201868): hide the map
/// picture (`+0x2dc = 0`), flag `+0x2f0`, and call the picture object's
/// "hide" slot (+0x48) when there is one.
pub fn event_select_hide_map_pic(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        let this = crate::abi::arg(c, 0);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        Addr(p(this, 0x2f0)).sw(c, c.g[V0][0]);
        c.g[A2][0] = lw(c, this, 0x2e4);
        let missing = c.g[A2][0] == 0;
        Addr(p(this, 0x2dc)).sw(c, 0);
        if !missing {
            c.cycles += 6;
            let obj = c.g[A2][0];
            let vt = lw(c, obj as u32, 8);
            c.g[V0][0] = vt;
            c.g[A1][0] = 0;
            let m = VSlot::read(c, vt, 0x48);
            c.g[A0][0] = m.delta;
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(obj);
            crate::call::call_ptr(c, m.func as u32, 0x0020_189c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSession_Shutdown(Session* s)` (0x208b78): cancel the
/// session's async file, destroy the indicators and shut the overlay down.
pub fn ov_screen_session_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        let g = crate::abi::r32(c, GP);
        let world = lw(c, g, 0xfffff7b8); // gp - 0x848
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[A1][0] = lw(c, this, 0xc4);
        c.g[A0][0] = lw(c, world as u32, 0x11c);
        callee::FE_ASYNC_MANAGER_CANCEL.call(c, 0x0020_8b98)?;
        c.g[A0][0] = c.g[S0][0];
        callee::OV_SCREEN_SESSION_DESTROY_INDICATORS.call(c, 0x0020_8ba0)?;
        c.g[A0][0] = c.g[S0][0];
        callee::C_OV_STATE_OVERLAY_SHUTDOWN.call(c, 0x0020_8ba8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionMan_RequestSky(SectionMan* m, int v)` (0x22de58): switch the
/// requested sky when it changes, activate the section and mark it
/// "requested".
pub fn section_man_request_sky(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0x1ac);
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 5;
            Addr(p(this, 0x1ac)).sw(c, c.g[V1][0]);
            callee::SECTION_MAN_REQUEST_ACTIVATE.call(c, 0x0022_de80)?;
            c.g[V0][0] = 1;
            Addr(p(crate::abi::r32(c, S0), 0x1a8)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
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
    use crate::difftest::fakes::{machine, vtable};
    use crate::difftest::{check, put_words, Rng, DATA, GAME_GP};

    /// A non-null, zeroed stand-in for the BE-library singleton at
    /// `gp - 0x1f04` (its lazy constructor would otherwise load from disc).
    const BE_LIBRARY: u32 = DATA + 0x1_f000;
    /// A small zeroed world (`world -> +0x84 -> +0xc`, rider count `+0x7c`).
    const WORLD: u32 = DATA + 0x2_0000;

    fn globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
        c.bus.write32(GAME_GP - 0x848, WORLD);
        let game = WORLD + 0x1000;
        c.bus.write32(WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        c.bus.write32(game + 0xc, race);
        c.bus.write32(race + 0x7c, 0);
    }

    /// A vtable whose one slot `off` is the fake, installed at `*(obj + 8)`.
    fn vtable_at(c: &mut Cpu, r: &mut Rng, obj: u32, off: u32) -> u32 {
        let vt = vtable(c, r, &[off]);
        c.bus.write32(obj + 8, vt);
        vt
    }

    /// A NUL-terminated ASCII string at `addr`.
    fn put_str(c: &mut Cpu, addr: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    // ---------------------------------------------------------------- MPEG-2

    #[test]
    fn rcmp_decoder_set_codec_matches() {
        check(
            ssx3_ee::s13::f_003b04f8__generated,
            rcmp_decoder_set_codec,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x01, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = 0x1000 + seed as u64;
                // `*(codec)` is the codec's vtable; slot 0x10 is its v02.
                let vt = vtable(c, &mut r, &[0x10]);
                c.bus.write32(DATA + 0x2000, vt);
            },
        );
    }

    #[test]
    fn rcmp_decoder_get_frame_number_matches() {
        check(
            ssx3_ee::s13::f_003b0580__generated,
            rcmp_decoder_get_frame_number,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x02, 1);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x20]);
                    c.bus.write32(DATA + 0x14, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2000, vt);
                } else {
                    c.bus.write32(DATA + 0x14, 0);
                }
            },
        );
    }

    #[test]
    fn rcmp_decoder_get_frame_rate_matches() {
        check(
            ssx3_ee::s13::f_003b05c0__generated,
            rcmp_decoder_get_frame_rate,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x03, 1);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    let vt = vtable(c, &mut r, &[0x28]);
                    c.bus.write32(DATA + 0x14, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2000, vt);
                } else {
                    c.bus.write32(DATA + 0x14, 0);
                }
            },
        );
    }

    #[test]
    fn mpeg2_get_bits_matches() {
        check(ssx3_ee::s13::f_003b1300__generated, mpeg2_get_bits, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x04, 1);
            c.g[A0][0] = sx(DATA + 0x3000);
            // The peek/consume pair runs on the (zeroed) reader state; give
            // the shift count a value that keeps it in range.
            c.bus.write32(DATA + 0x3000, r.next_u32() % 32);
        });
    }

    // ------------------------------------------------- rider metrics and AI

    #[test]
    fn rider_metrix_reset_stats_matches() {
        check(
            ssx3_ee::s00::f_001175b8__generated,
            rider_metrix_reset_stats,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x05, 1);
                c.g[A0][0] = sx(DATA);
                put_words(
                    c,
                    DATA + 0x1c8,
                    &[if seed % 2 == 0 { 0 } else { r.next_u32() }],
                );
                put_words(c, DATA + 0xfc, &[r.next_u32(), r.next_u32()]);
            },
        );
    }

    #[test]
    fn rider_metrix_start_board_press_matches() {
        check(
            ssx3_ee::s00::f_001199f8__generated,
            rider_metrix_start_board_press,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x06, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                // The press peak `+0x2c` can be negative or positive.
                c.bus.write32(
                    DATA + 0x2c,
                    if seed % 3 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(1.0)
                    },
                );
            },
        );
    }

    #[test]
    fn rider_get_sync_checksum_matches() {
        check(
            ssx3_ee::s00::f_00125ad8__generated,
            rider_get_sync_checksum,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x07, 0);
                c.g[A0][0] = sx(DATA);
                let mut words = [0u32; 8];
                for (i, w) in words.iter_mut().enumerate() {
                    *w = if i % 2 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(100.0)
                    };
                }
                put_words(c, DATA + 0x110, &words);
            },
        );
    }

    #[test]
    fn ai_setup_computer_riders_matches() {
        check(
            ssx3_ee::s00::f_00128958__generated,
            ai_setup_computer_riders,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x08, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // No previous computer riders when `+0x88` is not positive.
                c.bus.write32(
                    DATA + 0x88,
                    if seed % 2 == 0 {
                        0
                    } else {
                        r.next_u32() | 0x8000_0000
                    },
                );
                c.bus.write32(DATA + 0x80, 0);
                c.bus.write32(DATA + 0x78, 0);
            },
        );
    }

    /// The three spin-sub predicates share a fixture: either the sub-state
    /// does not match (early 0) or the animation entry is read through the
    /// controller (`+0x58` → `+0x784`).
    fn spin_sub_test(generated: GuestFn, hand: GuestFn, kind: u32, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            c.g[A0][0] = sx(DATA);
            let matching = seed % 2 == 0;
            let sub = if matching { kind } else { kind ^ 1 };
            c.bus.write32(DATA + 4, sub);
            c.bus.write32(DATA + 0x58, DATA + 0x2000);
            // The controller's animation table pointer, then the entry slot.
            c.bus.write32(DATA + 0x2784, DATA + 0x3000);
            c.bus.write32(DATA + 0x3008, 0x1b6);
        });
    }

    #[test]
    fn rider_spin_sub_is_trick_anim12_matches() {
        spin_sub_test(
            ssx3_ee::s01::f_00135eb0__generated,
            rider_spin_sub_is_trick_anim12,
            2,
            0x09,
        );
    }

    #[test]
    fn rider_spin_sub_is_trick_anim13_matches() {
        spin_sub_test(
            ssx3_ee::s01::f_00135ef0__generated,
            rider_spin_sub_is_trick_anim13,
            4,
            0x0a,
        );
    }

    #[test]
    fn rider_spin_sub_is_trick_anim14_matches() {
        spin_sub_test(
            ssx3_ee::s01::f_00135f30__generated,
            rider_spin_sub_is_trick_anim14,
            4,
            0x0b,
        );
    }

    // ----------------------------------------------------------- front end

    #[test]
    fn benew_player_int_set_alt_character_matches() {
        check(
            ssx3_ee::s01::f_001473d0__generated,
            benew_player_int_set_alt_character,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0c, 1);
                c.g[A0][0] = 0;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (r.next_u32() & 0xff) as u64;
            },
        );
    }

    #[test]
    fn be_reward_int_count_enabled_songs_matches() {
        check(
            ssx3_ee::s01::f_001587b8__generated,
            be_reward_int_count_enabled_songs,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0d, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                c.bus.write32(DATA + 0x200, r.next_u32());
            },
        );
    }

    #[test]
    fn matrix_chase_camera_algorithm_ctor_matches() {
        check(
            ssx3_ee::s01::f_00160760__generated,
            matrix_chase_camera_algorithm_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x0e, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn chase_cam_controller_set_target_matches() {
        check(
            ssx3_ee::s01::f_00161a60__generated,
            chase_cam_controller_set_target,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x0f, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
                // A previous target is deleted; the null case covers the skip.
                // The allocator's mutex looks held by this thread, so `delete`
                // takes its fast path (the `dispatch.rs` tests' recipe).
                c.bus.write32(0x0053_8b00 + 4, 0x42);
                c.bus.write32(0x0090_0010, 0x42);
                c.bus.write32(0x0053_8b00 + 8, seed % 4);
                c.bus
                    .write32(DATA + 0x20, if seed % 2 == 0 { 0 } else { DATA + 0x4000 });
            },
        );
    }

    #[test]
    fn keyboard_screen_set_text_matches() {
        check(
            ssx3_ee::s02::f_001805d8__generated,
            keyboard_screen_set_text,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x10, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x5000);
                put_str(
                    c,
                    DATA + 0x5000,
                    ["A", "hello", "a longer text value"][seed as usize % 3],
                );
            },
        );
    }

    #[test]
    fn c_fe_state_event_select_post_input_matches() {
        check(
            ssx3_ee::s02::f_00186998__generated,
            c_fe_state_event_select_post_input,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
                // The event panel's vtable (slot +0x88) resolves to the fake.
                vtable_at(c, &mut r, DATA + 0x3000, 0x88);
                // Mode 3 keeps `EventSelect_PostInput` on its short path.
                c.bus.write32(GAME_GP - 0xb54, 3);
            },
        );
    }

    #[test]
    fn c_fe_state_mem_card_reset_scan_matches() {
        check(
            ssx3_ee::s02::f_00187230__generated,
            c_fe_state_mem_card_reset_scan,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12, 1);
                globals(c);
                let mgr = DATA + 0x4000;
                // The manager singleton and its scan object, both zeroed
                // dummies (the lazy constructor would call the allocator).
                c.bus.write32(GAME_GP - 0x47c, mgr);
                c.bus.write32(GAME_GP - 0x1c38, mgr);
                c.bus.write32(GAME_GP - 0x1c34, r.next_u32());
                // The card object the manager scans (`+0x434`) and its vtable.
                c.bus.write32(mgr + 0x434, DATA + 0x5000);
                vtable_at(c, &mut r, DATA + 0x5000, 0x98);
            },
        );
    }

    // --------------------------------------------------------- state ctors

    #[test]
    fn c_fe_state_options_sound_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018a258__generated,
            c_fe_state_options_sound_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x13, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
            },
        );
    }

    #[test]
    fn c_fe_state_options_hud_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018c0e8__generated,
            c_fe_state_options_hud_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x14, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
            },
        );
    }

    #[test]
    fn c_fe_state_options_save_load_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018d240__generated,
            c_fe_state_options_save_load_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x15, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
            },
        );
    }

    fn net_fe_state_ctor_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 30, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x3000);
        });
    }

    #[test]
    fn c_fe_state_online_main_menu_ctor_matches() {
        net_fe_state_ctor_test(
            ssx3_ee::s03::f_001bfbb8__generated,
            c_fe_state_online_main_menu_ctor,
            0x16,
        );
    }

    #[test]
    fn c_fe_state_net_ssx_world_ctor_matches() {
        net_fe_state_ctor_test(
            ssx3_ee::s03::f_001c1c50__generated,
            c_fe_state_net_ssx_world_ctor,
            0x17,
        );
    }

    #[test]
    fn c_fe_state_net_quick_messages_ctor_matches() {
        net_fe_state_ctor_test(
            ssx3_ee::s03::f_001c20f8__generated,
            c_fe_state_net_quick_messages_ctor,
            0x18,
        );
    }

    #[test]
    fn c_fe_state_net_news_ctor_matches() {
        net_fe_state_ctor_test(ssx3_ee::s03::f_001c5018__generated, c_fe_state_net_news_ctor, 0x19);
    }

    // ---------------------------------------------------------- money text

    fn money_text_test(
        generated: GuestFn,
        hand: GuestFn,
        text_off: u32,
        value_off: u32,
        salt: u64,
    ) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            // The money formatter asks the BE library; a zeroed singleton
            // keeps it from lazily loading the rewards file from disc.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
            c.g[A0][0] = sx(DATA);
            if seed % 3 == 0 {
                c.bus.write32(DATA + text_off, 0);
            } else {
                c.bus.write32(DATA + text_off, DATA + 0x3000);
            }
            c.bus.write32(
                DATA + value_off,
                if seed % 4 == 0 {
                    0
                } else {
                    r.next_u32() % 100_000
                },
            );
            // The formatted string is a static buffer in the game image; the
            // text object at DATA + 0x3000 is only a target of the setter.
            c.bus.write32(DATA + 0x3000, 0);
        });
    }

    #[test]
    fn c_fe_state_request_line_update_money_matches() {
        money_text_test(
            ssx3_ee::s03::f_001985b0__generated,
            c_fe_state_request_line_update_money,
            0x1e0,
            0x168,
            0x1a,
        );
    }

    #[test]
    fn c_fe_state_char_equip_update_money_text_matches() {
        money_text_test(
            ssx3_ee::s03::f_0019bba0__generated,
            c_fe_state_char_equip_update_money_text,
            0x64,
            0xc0,
            0x1b,
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
    fn fe_module_unload_rider_previews_matches() {
        check(
            ssx3_ee::s03::f_001a04c8__generated,
            fe_module_unload_rider_previews,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn fe_module_update_rider_previews_matches() {
        check(
            ssx3_ee::s03::f_001a0608__generated,
            fe_module_update_rider_previews,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn c_fe_use_existing_account_validate_fields_matches() {
        check(
            ssx3_ee::s03::f_001ac408__generated,
            c_fe_use_existing_account_validate_fields,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x1f, 1);
                c.g[A0][0] = sx(DATA);
                // The BE library holds the account name (`+0x48`) and password
                // (`+0x4c`) strings; both non-empty keeps the popup path out.
                c.bus.write32(GAME_GP - 0xc8, DATA + 0x3000);
                c.bus.write32(DATA + 0x3048, DATA + 0x5000);
                c.bus.write32(DATA + 0x304c, DATA + 0x5100);
                put_str(c, DATA + 0x5000, if seed % 2 == 0 { "player" } else { "x" });
                put_str(c, DATA + 0x5100, "secret");
            },
        );
    }

    #[test]
    fn c_fe_state_net_config_on_popup_event_passthrough_matches() {
        check(
            ssx3_ee::s03::f_001b4c00__generated,
            c_fe_state_net_config_on_popup_event_passthrough,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x20, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0;
                c.g[A2][0] = match seed % 3 {
                    0 => 0xf,
                    1 => 0x14,
                    _ => 0x30,
                };
                // Bypass the keyboard-popup shortcut so the child event goes
                // through the dispatcher (which lands in its default case).
                c.bus.write32(DATA + 0x69c, 0x1234);
            },
        );
    }

    #[test]
    fn c_keyboard_popup_ensure_cursor_on_enabled_key_matches() {
        check(
            ssx3_ee::s04::f_001ce520__generated,
            c_keyboard_popup_ensure_cursor_on_enabled_key,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x21, 1);
                c.g[A0][0] = sx(DATA);
                // `cKeyboardPopup_IsKeyEnabled` answers from the per-key words
                // at `popup + index * 4 + 0x148` (shifted grid) and `+0x298`.
                // On a zeroed object every key reads "disabled", and then the
                // cursor walk (`cKeyboardPopup_MoveDown` -> its own recursive
                // tail / `cKeyboardPopup_StepDownToEnabledKey`) keeps stepping
                // down the grid until the guest stack overflows. Enable every
                // key, and disable only the cursor's own slot so that the
                // "move down" call is still exercised.
                for i in 0..0x100u32 {
                    c.bus.write32(DATA + 0x148 + 4 * i, 1);
                    c.bus.write32(DATA + 0x298 + 4 * i, 1);
                }
                // A plain-grid cursor key (>= 0xe skips the top-row walk; the
                // step below is a single `+0xe` into enabled keys).
                let key = 0x20 + seed % 0x10;
                c.bus.write32(DATA + 0x140, key);
                if seed % 2 == 1 {
                    c.bus.write32(DATA + 0x148 + 4 * key, 0);
                    c.bus.write32(DATA + 0x298 + 4 * key, 0);
                }
                // The grid state the walk consults (row marker and shift flag
                // in `+0x68`/`+0x6c`, the "current row" at `+0x430`/`+0x434`).
                c.bus.write32(DATA + 0x68, 0);
                c.bus.write32(DATA + 0x6c, 0);
                c.bus.write32(DATA + 0x430, 0);
                c.bus.write32(DATA + 0x434, 1);
                c.bus.write32(DATA + 0x144, (seed % 3) as u32);
            },
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

    #[test]
    fn c_fe_state_mountain_room_update_matches() {
        check(
            ssx3_ee::s04::f_001d3120__generated,
            c_fe_state_mountain_room_update,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x25, 1);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0xbc, if seed % 2 == 0 { 0 } else { 0xffff_ffff });
            },
        );
    }

    #[test]
    fn mem_card_state_base_set_mode_menu_matches() {
        check(
            ssx3_ee::s04::f_001d9368__generated,
            mem_card_state_base_set_mode_menu,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26, 1);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x214, if seed % 2 == 0 { 0 } else { 1 });
                vtable_at(c, &mut r, DATA, 0x118);
            },
        );
    }

    #[test]
    fn c_fe_state_buy_attrib_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001f49f8__generated,
            c_fe_state_buy_attrib_on_activate,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27, 1);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0xac, DATA + 0x3000);
                vtable_at(c, &mut r, DATA + 0x3000, 0x38);
            },
        );
    }

    // ------------------------------------------------------ overlay screens

    fn ov_pre_update_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            // Zeroed BE library: no disc load on first use.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
            c.g[A0][0] = sx(DATA);
            match seed % 3 {
                0 => c.bus.write32(GAME_GP - 0x204, 0),
                1 => {
                    c.bus.write32(GAME_GP - 0x204, DATA + 0x3000);
                    c.bus.write32(DATA + 0x3064, 1);
                }
                _ => {
                    c.bus.write32(GAME_GP - 0x204, DATA + 0x3000);
                    c.bus.write32(DATA + 0x3064, 0);
                }
            }
        });
    }

    #[test]
    fn ov_screen_race_pre_update_matches() {
        ov_pre_update_test(ssx3_ee::s04::f_001fbce0__generated, ov_screen_race_pre_update, 0x28);
    }

    #[test]
    fn ov_screen_multi_freestyle_pre_update_matches() {
        ov_pre_update_test(
            ssx3_ee::s04::f_001fd150__generated,
            ov_screen_multi_freestyle_pre_update,
            0x29,
        );
    }

    #[test]
    fn ov_screen_multi_freestyle_pre_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001fd0d8__generated,
            ov_screen_multi_freestyle_pre_on_activate,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x2a, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    c.bus.write32(GAME_GP - 0x204, 0);
                } else {
                    c.bus.write32(GAME_GP - 0x204, DATA + 0x3000);
                }
            },
        );
    }

    fn save_screen_ctor_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 30, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            // Zeroed BE library: no disc load on first use.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x3000);
            c.g[A2][0] = (seed % 5) as u64;
        });
    }

    #[test]
    fn ov_screen_save_replay_ctor_matches() {
        save_screen_ctor_test(ssx3_ee::s05::f_001fdbf0__generated, ov_screen_save_replay_ctor, 0x2b);
    }

    #[test]
    fn ov_screen_save_records_ctor_matches() {
        save_screen_ctor_test(ssx3_ee::s05::f_001fe968__generated, ov_screen_save_records_ctor, 0x2c);
    }

    fn close_top_state_once_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            c.g[A0][0] = sx(DATA);
            c.bus
                .write32(DATA + 0x1ac, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(DATA + 0x10, DATA + 0x3000);
        });
    }

    #[test]
    fn ov_screen_save_replay_v33_matches() {
        close_top_state_once_test(ssx3_ee::s05::f_001fdf00__generated, ov_screen_save_replay_v33, 0x2d);
    }

    #[test]
    fn ov_screen_save_records_v33_matches() {
        close_top_state_once_test(ssx3_ee::s05::f_001fec60__generated, ov_screen_save_records_v33, 0x2e);
    }

    #[test]
    fn ov_screen_autosave_v33_matches() {
        close_top_state_once_test(ssx3_ee::s05::f_002124c8__generated, ov_screen_autosave_v33, 0x2f);
    }

    #[test]
    fn event_select_hide_map_pic_matches() {
        check(
            ssx3_ee::s05::f_00201868__generated,
            event_select_hide_map_pic,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30, 1);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x2e4, 0);
                } else {
                    c.bus.write32(DATA + 0x2e4, DATA + 0x3000);
                    vtable_at(c, &mut r, DATA + 0x3000, 0x48);
                }
            },
        );
    }

    #[test]
    fn ov_screen_session_shutdown_matches() {
        check(
            ssx3_ee::s05::f_00208b78__generated,
            ov_screen_session_shutdown,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x31, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0xc4, (seed % 8) as u32);
                c.bus.write32(WORLD + 0x11c, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn section_man_request_sky_matches() {
        check(
            ssx3_ee::s05::f_0022de58__generated,
            section_man_request_sky,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x32, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                let v = (seed % 4) as u64;
                c.g[A1][0] = v;
                c.bus
                    .write32(DATA + 0x1ac, if seed % 3 == 0 { v as u32 } else { 0 });
            },
        );
    }
}
