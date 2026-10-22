//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for a batch of small, cold-path functions — world / camera
//! trigger setup, FE screen constructors and child-event handlers, the DJ
//! speech wrappers, process-list and replay helpers, and the VU0 microcode
//! uploaders — each with a differential test against the generated body.
//!
//! These are literal ports: every block charge, every register the generated
//! code leaves behind (notably `v0`/`v1` and the scratch values a callee may
//! observe) and the call protocol of [`crate::call`] are preserved. Loop back
//! edges deliver events with [`event_due`] + [`deliver_event_here`];
//! `BuddyMan_HasChallenger` keeps the generated dispatcher's resume points
//! (`pc` = loop head) because it can be re-entered after an unwind.

use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_6688, world_ctor, "World_ctor"),
    (0x002e_a480, c_board_trail_fx_render, "cBoardTrailFX_Render"),
    (
        0x0019_4080,
        fe_state_bragging_rights_start_wait_anim,
        "cFEStateBraggingRights_StartWaitAnim",
    ),
    (0x0019_5eb8, fe_locale_v04, "FELocale_v04"),
    (
        0x001f_4050,
        fe_state_lodge_rider_detail_init,
        "cFEStateLodgeRiderDetail_Init",
    ),
    (0x0016_c170, camera_trigger_man_load_file, "CameraTriggerMan_LoadFile"),
    (
        0x0035_51a8,
        process_list_delete_section_objects,
        "ProcessList_DeleteSectionObjects",
    ),(0x0018_1090, fe_state_char_select_ctor, "cFEStateCharSelect_ctor"),
    (0x0015_ca50, algorithm_list_remove, "AlgorithmList_Remove"),
    (
        0x001a_c4f0,
        fe_use_existing_account_v25,
        "cFEUseExistingAccount_v25",
    ),
    (
        0x001a_ddd0,
        fe_state_challenge_popup_ctor,
        "cFEStateChallengePopup_ctor",
    ),
    (
        0x001b_14d8,
        fe_state_online_character_select_v25,
        "cFEStateOnlineCharacterSelect_v25",
    ),
    (
        0x001b_33d8,
        fe_state_chat_room_on_keyboard_done,
        "cFEStateChatRoom_OnKeyboardDone",
    ),
    (
        0x001b_3978,
        fe_state_chat_room_select_player_by_name,
        "cFEStateChatRoom_SelectPlayerByName",
    ),(
        0x001b_a428,
        fe_state_create_step2_init,
        "cFEStateCreateStep2_Init",
    ),
    (0x001c_5540, fe_state_net_news_v25, "cFEStateNetNews_v25"),
    (
        0x001e_2370,
        message_center_set_peak_foreshadow_done,
        "MessageCenter_SetPeakForeshadowDone",
    ),
    (
        0x0035_d288,
        roller_modifier_update_momentum,
        "RollerModifier_UpdateMomentum",
    ),
    (0x0026_3128, buddy_man_has_challenger, "BuddyMan_HasChallenger"),
    (0x002a_2018, dj_say_20c8, "SSXAudioSystem_DJSay20C8"),
    (0x002a_20d0, dj_say_20c9, "SSXAudioSystem_DJSay20C9"),
    (0x002a_2260, dj_say_20e7, "SSXAudioSystem_DJSay20E7"),
    (0x002a_2318, dj_say_20e8, "SSXAudioSystem_DJSay20E8"),
    (0x002a_2638, dj_say_20ba, "SSXAudioSystem_DJSay20BA"),
    (0x002a_2ad0, dj_say_212b, "SSXAudioSystem_DJSay212B"),(0x002a_f608, path_plat_fade_volume, "PATH_PlatFadeVolume"),(0x0023_fbb8, mc_manager_state12_save, "MCManager_State12_Save"),
    (0x0024_05d0, mc_manager_state21_load, "MCManager_State21_Load"),
    (
        0x0017_14e8,
        camera_trigger_read_action_type1,
        "CameraTrigger_ReadActionType1",
    ),
    (
        0x0017_21c0,
        camera_sphere_trigger_get_bounds,
        "CameraSphereTrigger_GetBounds",
    ),
    (
        0x0017_2840,
        camera_box_trigger_get_bounds,
        "CameraBoxTrigger_GetBounds",
    ),
    (0x0026_9188, setup_man_client_recv_msg5, "cSetupMan_ClientRecvMsg5"),
    (0x0026_93c0, setup_man_client_recv_msg4, "cSetupMan_ClientRecvMsg4"),
    (0x0026_9890, setup_man_client_recv_msg7, "cSetupMan_ClientRecvMsg7"),
    (0x001c_f0b0, fe_state_reward_gallery_init, "cFEStateRewardGallery_Init"),
    (
        0x001c_fd18,
        fe_state_reward_gallery_on_child_event,
        "cFEStateRewardGallery_OnChildEvent",
    ),
    (
        0x0018_c1b8,
        fe_state_options_hud_on_child_event,
        "cFEStateOptionsHUD_OnChildEvent",
    ),
    (0x001f_d268, ov_screen_rival_pre_init, "OVScreenRivalPre_Init"),
    (0x0011_1890, rider_base_reset_fx, "RiderBase_ResetFX"),
    (0x0028_30c8, script_cam_binding_bind, "ScriptCamBinding_Bind"),
    (0x002b_58d0, world_trigger_manager_ctor, "WorldTriggerManager_ctor"),
    (
        0x0032_65c0,
        input_preproc_skip_line_comment,
        "InputPreproc_SkipLineComment",
    ),
    (0x0031_89d0, cbx_string_get_buffer, "cBXString_GetBuffer"),
    (0x0039_12a8, vu0_upload_microcode_a, "Vu0_UploadMicrocodeA"),
    (0x0039_1360, vu0_upload_microcode_b, "Vu0_UploadMicrocodeB"),
    (0x0026_d420, replay_cache_read_frame, "ReplayCache_ReadFrame"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    /// The engine allocator (`cMemMan_alloc`), reached as a table call.
    pub const MALLOC: u32 = 0x0031_7d70;
    /// `cMemMan_free`.
    pub const FREE: u32 = 0x0031_7e50;
    /// `cMemMan_free2` (the second form used by the FE states).
    pub const FREE2: u32 = 0x0031_7e98;
    /// `GetHashValue32` (FE string name -> hash).
    pub const FE_STRING_HASH: u32 = 0x0031_7670;

    pub const VEC3_MAX_COMPONENT: Callee = Callee {
        addr: 0x0017_31c0,
        f: ee::s02::Vec3_MaxComponent,
    };
    pub const SONG_INSTANCE_CTOR: Callee = Callee {
        addr: 0x002b_1c50,
        f: ee::s08::SongInstance_ctor,
    };
    pub const NET_FE_STATE_CTOR: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    };
    pub const NET_FE_STATE_DTOR: Callee = Callee {
        addr: 0x001a_85d0,
        f: ee::s03::NetFEState_dtor,
    };
    pub const NET_FE_STATE_INIT: Callee = Callee {
        addr: 0x001a_8768,
        f: ee::s03::NetFEState_Init,
    };
    pub const NET_FE_STATE_GET_SELECTED_USER: Callee = Callee {
        addr: 0x001a_98b0,
        f: ee::s03::NetFEState_GetSelectedUser,
    };
    pub const WORLD_CACHE_CTOR: Callee = Callee {
        addr: 0x003a_7dd0,
        f: ee::s13::cWorldCache_ctor,
    };
    pub const WSCRIPT_CACHE_CTOR: Callee = Callee {
        addr: 0x003a_c7c8,
        f: ee::s13::cWScriptCache_ctor,
    };
    pub const WORLD_TRIGGER_INSTANCE_CTOR: Callee = Callee {
        addr: 0x002b_4b70,
        f: ee::s08::WorldTriggerInstance_ctor,
    };
    pub const WORLD_TRIGGER_MANAGER_LOAD: Callee = Callee {
        addr: 0x002b_5a18,
        f: ee::s08::WorldTriggerManager_LoadTriggerInfo,
    };
    pub const PROCESS_LIST_FIRST: Callee = Callee {
        addr: 0x0035_4ea8,
        f: ee::s11::ProcessList_First,
    };
    pub const PROCESS_LIST_NEXT: Callee = Callee {
        addr: 0x0035_4ef8,
        f: ee::s11::ProcessList_Next,
    };
    pub const CAMERA_TRIGGER_MAN_FREE_TRIGGERS: Callee = Callee {
        addr: 0x0016_bf40,
        f: ee::s02::CameraTriggerMan_FreeTriggers,
    };
    pub const CAMERA_TRIGGER_MAN_PARSE: Callee = Callee {
        addr: 0x0016_c228,
        f: ee::s02::CameraTriggerMan_Parse,
    };
    pub const CREATE_STEP2_SET_MONTH: Callee = Callee {
        addr: 0x001b_b2d0,
        f: ee::s03::cFEStateCreateStep2_SetMonthSelection,
    };
    pub const CREATE_STEP2_SET_DAY: Callee = Callee {
        addr: 0x001b_b340,
        f: ee::s03::cFEStateCreateStep2_SetDaySelection,
    };
    pub const CREATE_STEP2_SET_YEAR: Callee = Callee {
        addr: 0x001b_b3b0,
        f: ee::s03::cFEStateCreateStep2_SetYearSelection,
    };
    pub const CREATE_STEP2_SET_GENDER: Callee = Callee {
        addr: 0x001b_b440,
        f: ee::s03::cFEStateCreateStep2_SetGenderSelection,
    };
    pub const CREATE_STEP2_SET_LEGAL: Callee = Callee {
        addr: 0x001b_b4b0,
        f: ee::s03::cFEStateCreateStep2_SetLegalSelection,
    };
    pub const REWARD_GALLERY_INIT_PAGES: Callee = Callee {
        addr: 0x001d_0180,
        f: ee::s04::cFEStateRewardGallery_InitPages,
    };
    pub const REWARD_GALLERY_REQUEST_SHAPE: Callee = Callee {
        addr: 0x001c_fe60,
        f: ee::s04::cFEStateRewardGallery_RequestShapeLoad,
    };
    pub const OV_STATE_OVERLAY_INIT: Callee = Callee {
        addr: 0x0020_a380,
        f: ee::s05::cOVStateOverlay_Init,
    };
    pub const OPTIONS_HUD_REFRESH_SELECTION: Callee = Callee {
        addr: 0x0018_c478,
        f: ee::s02::cFEStateOptionsHUD_RefreshSelection,
    };
    pub const MESSAGE_CENTER_SET_FLAG: Callee = Callee {
        addr: 0x001e_3a78,
        f: ee::s04::MessageCenter_SetFlag,
    };
    pub const SCRIPT_BINDING_BIND: Callee = Callee {
        addr: 0x0028_2fb8,
        f: ee::s07::ScriptBinding_Bind,
    };
    pub const INPUT_PREPROC_WRITE: Callee = Callee {
        addr: 0x0032_6360,
        f: ee::s10::InputPreproc_Write,
    };
    pub const INPUT_PREPROC_NEXT_CHAR: Callee = Callee {
        addr: 0x0032_6478,
        f: ee::s10::InputPreproc_NextChar,
    };
    pub const INPUT_PREPROC_EMPTY_STUB: Callee = Callee {
        addr: 0x0032_6a20,
        f: ee::s10::InputPreproc_EmptyStub,
    };
    pub const BXSTRING_CTOR_COPY: Callee = Callee {
        addr: 0x0031_7fe8,
        f: ee::s10::cBXString_ctor_copy,
    };
    pub const BXSTRING_RELEASE: Callee = Callee {
        addr: 0x0031_80b8,
        f: ee::s10::cBXString_Release,
    };
    pub const BXSTRING_ALLOC_BUFFER: Callee = Callee {
        addr: 0x0031_8040,
        f: ee::s10::cBXString_AllocBuffer,
    };
    pub const BXSTRING_DTOR: Callee = Callee {
        addr: 0x0031_8260,
        f: ee::s10::cBXString_dtor,
    };
    /// `SSXAudioSystem_IsDJSpeechOn` — returns non-zero when DJ speech plays.
    pub const IS_DJ_SPEECH_ON: Callee = Callee {
        addr: 0x0029_f0d8,
        f: ee::s08::f_0029f0d8,
    };
    pub const GET_LANGUAGE_MASK: Callee = Callee {
        addr: 0x0029_f198,
        f: ee::s08::f_0029f198,
    };
    pub const SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::f_002b1458,
    };
    pub const TRACK_TO_CATEGORY: Callee = Callee {
        addr: 0x002a_1e20,
        f: ee::s08::f_002a1e20,
    };

    // `call_indirect` targets the generator left as table calls.
    pub const AUDIO_GET_GAME_MODULE: u32 = 0x0028_b1e8;
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const BE_GET_SOMETHING: u32 = 0x0014_4bc0;
    pub const SPEECH_NEW: u32 = 0x003d_8008;
}

// ---------------------------------------------------------------------------
// Small helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// The low word of a GPR.
#[inline(always)]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

// ---------------------------------------------------------------------------
// Roller modifier.

/// `void RollerModifier_UpdateMomentum(RollerModifier* m)` (0x35d288): one
/// Verlet-style momentum step for the roller coaster's board trail. The
/// node's linear velocity (`+0x54`..`+0x70`) is scaled by the frame delta
/// (`+0x30`,`+0x34`), damping (`+0x38`) and an acceleration (`+0x3c`); the
/// result is summed with the node's accumulated force (`+0x58`..`+0x6c`) and
/// the previous position vector (`+0x20`, copied into a VU0 register) is
/// added before the trail's current positions (`+0x80`, `+0x90`) are
/// written.
pub fn roller_modifier_update_momentum(c: &mut Cpu) {
    c.cycles += 46;
    let m = Addr(r(c, A0));
    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x20));

    // vf5 = (m+0x20).xyz * (m+0x40), stored to the stack scratch vector.
    c.lwc1(0, p(m.0, 0x40));
    c.mfc1(2, 0);
    c.lqc2(4, p(m.0, 0x20));
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, ssx3_ee_rt::vu0::Src::Bc(3, ssx3_ee_rt::vu0::X));
    let v = p(r(c, SP), 0x10);
    c.sqc2(5, v);

    // velocity += force * delta and velocity += accel * damping.
    c.lwc1(4, p(m.0, 0x30));
    c.lwc1(5, p(m.0, 0x34));
    c.lwc1(2, p(m.0, 0x50));
    c.lwc1(8, p(m.0, 0x5c));
    c.lwc1(1, p(m.0, 0x54));
    c.mul_s(2, 2, 4);
    c.lwc1(6, p(m.0, 0x60));
    c.mul_s(8, 8, 5);
    c.lwc1(0, p(m.0, 0x58));
    c.mul_s(1, 1, 4);
    c.lwc1(3, p(m.0, 0x64));
    c.mul_s(6, 6, 5);
    c.mul_s(0, 0, 4);
    c.lwc1(7, p(m.0, 0x38));
    c.mul_s(3, 3, 5);
    c.lwc1(9, p(m.0, 0x68));
    c.lwc1(5, p(m.0, 0x6c));
    c.add_s(2, 2, 8);
    c.lwc1(4, p(m.0, 0x70));
    c.add_s(1, 1, 6);
    c.add_s(0, 0, 3);
    c.lwc1(6, p(m.0, 0x3c));
    c.mul_s(4, 4, 7);
    c.lq(2, v);
    c.mul_s(9, 9, 7);
    c.swc1(6, p(r(c, SP), 0x1c));
    c.mul_s(5, 5, 7);
    c.sq(2, p(m.0, 0x80));
    c.add_s(0, 0, 4);
    c.add_s(2, 2, 9);
    c.add_s(1, 1, 5);
    c.swc1(0, p(r(c, SP), 0x18));
    c.swc1(2, p(r(c, SP), 0x10));
    c.swc1(1, p(r(c, SP), 0x14));
    c.lq(2, p(r(c, SP), 0x10));
    c.sq(2, p(m.0, 0x90));
    c.sq(2, r(c, SP));

    let ra = r(c, RA);
    c.g[SP][0] = sx(r(c, SP).wrapping_add(0x20));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Camera trigger bounds.

/// `void CameraSphereTrigger_GetBounds(CameraTrigger*, Bounds*)` (0x1721c0):
/// the trigger's local box (`+0` min, `+8` max, `+0x18` centre) grown by the
/// sphere radius; `out+0` receives the min corner, `out+0x10` the max corner
/// and `out+0x30`/`+0x40` their padding words.
fn camera_trigger_bounds(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 46;
    let frame = Frame::open(c, 0x80);
    frame.save(c, S0, 0x70);
    frame.save(c, S1, 0x60);
    c.g[S0][0] = c.g[A1][0];
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0x50);
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = sx(r(c, S0).wrapping_add(0x18));
    callee::VEC3_MAX_COMPONENT.call(c, ret)?;

    // radius f0 -> the bounds.
    c.lwc1(3, p(r(c, S0), 0x8));
    c.g[V0][0] = c.g[S1][0];
    c.lwc1(2, r(c, S0));
    c.lwc1(1, p(r(c, S0), 0x4));
    c.add_s(7, 3, 0);
    c.add_s(6, 2, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 4);
    c.add_s(5, 1, 0);
    c.swc1(0, r(c, SP));
    c.sub_s(2, 2, 0);
    c.swc1(7, p(r(c, SP), 0x38));
    c.sub_s(1, 1, 0);
    c.swc1(6, p(r(c, SP), 0x30));
    c.sub_s(3, 3, 0);
    c.swc1(5, p(r(c, SP), 0x34));
    c.swc1(2, p(r(c, SP), 0x10));
    c.swc1(1, p(r(c, SP), 0x14));
    c.swc1(3, p(r(c, SP), 0x18));
    c.swc1(4, p(r(c, SP), 0x3c));
    c.swc1(4, p(r(c, SP), 0x1c));
    c.lq(3, p(r(c, SP), 0x30));
    c.lq(4, p(r(c, SP), 0x10));
    c.sq(3, p(r(c, S1), 0x10));
    c.sq(4, r(c, S1));
    frame.restore(c, 16, 0x70);
    frame.restore(c, 17, 0x60);
    frame.restore_ra(c, 0x50);
    c.swc1(0, p(r(c, SP), 0x4));
    c.swc1(0, p(r(c, SP), 0x8));
    c.swc1(2, p(r(c, SP), 0x20));
    c.swc1(1, p(r(c, SP), 0x24));
    c.swc1(3, p(r(c, SP), 0x28));
    c.swc1(6, p(r(c, SP), 0x40));
    c.swc1(5, p(r(c, SP), 0x44));
    c.swc1(7, p(r(c, SP), 0x48));
    frame.close(c);
    Ok(())
}

/// `void CameraSphereTrigger_GetBounds(CameraTrigger*, Bounds*)` (0x1721c0).
pub fn camera_sphere_trigger_get_bounds(c: &mut Cpu) {
    guest(c, |c| camera_trigger_bounds(c, 0x0017_21e0))
}

/// `void CameraBoxTrigger_GetBounds(CameraTrigger*, Bounds*)` (0x172840).
pub fn camera_box_trigger_get_bounds(c: &mut Cpu) {
    guest(c, |c| camera_trigger_bounds(c, 0x0017_2860))
}

// ---------------------------------------------------------------------------
// Song instance allocation.

/// `SongInstance* SongInstance_Create(...)` (0x2b1b98): allocate a 0xc4-byte
/// song instance from the named pool and construct it from the caller's nine
/// arguments (the ninth is passed on the stack).
pub fn song_instance_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S3, 0x60);
        let a0 = r(c, A0);
        let a1 = r(c, A1);
        let a2 = r(c, A2);
        let a3 = r(c, A3);
        let t0 = r(c, T0);
        let t1 = r(c, T1);
        let t2 = r(c, T2);
        let t3 = r(c, T3);
        frame.save(c, S0, 0x90);
        // The low byte of t0, sign-extended.
        c.g[S0][0] = sx(((t0 as u32) << 24) as i32 as u32);
        frame.save(c, S2, 0x70);
        frame.save(c, S4, 0x50);
        c.g[S2][0] = sx(a0);
        frame.save(c, S5, 0x40);
        c.g[S4][0] = sx(a2);
        frame.save(c, S1, 0x80);
        c.g[S5][0] = sx(a3);
        frame.save(c, S7, 0x20);
        frame.save(c, S6, 0x30);
        c.g[S6][0] = sx(t1);
        c.g[S7][0] = sx(t2);
        c.g[S3][0] = sx(a1);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = sx(t3);
        c.g[S0][0] = sx((r(c, S0) as i32 >> 24) as u32);

        c.g[A1][0] = sx(0x0048_33e0);
        c.g[A0][0] = 0xc4;
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x002b_1bfc);
        c.g[A3][0] = 0;
        call_ptr(c, callee::MALLOC, 0x002b_1bfc)?;

        c.sw(r(c, SP), c.g[S1][0]);
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S4][0];
        c.g[T0][0] = c.g[S5][0];
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = c.g[S6][0];
        c.g[T3][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x002b_1c24);
        c.g[A0][0] = c.g[V0][0];
        callee::SONG_INSTANCE_CTOR.call(c, 0x002b_1c24)?;

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
    })
}

// ---------------------------------------------------------------------------
// PATH platform.

/// `void PATH_PlatInit(PATH_Plat* p, void* a, void* b)` (0x2ae048): clear the
/// 24 platform path records (`+0x18`, `+8`, `+4`, `0` set to -1), the `+8`
/// field of the parallel table to -1, publish both tables, install the tick
/// callback and create the PATH sound streams.
pub fn path_plat_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
        c.sd(r(c, SP), c.g[RA][0]);
        // v0 = 0x4d3ef8 (record table), v1 = 0x4d3e98 (parallel table).
        c.g[V0][0] = sx(0x004d_3ef8);
        c.g[V1][0] = sx(0x004d_3e98);
        c.g[A3][0] = c.g[A0][0];
        c.g[A2][0] = u64::MAX;
        c.g[A0][0] = 0x17;
        loop {
            c.cycles += 9;
            c.sw(r(c, V1), 0);
            c.g[A0][0] = sx(r(c, A0).wrapping_sub(1));
            c.sw(p(r(c, V0), 0x8), c.g[A2][0]);
            c.g[V1][0] = sx(r(c, V1).wrapping_add(4));
            c.sw(p(r(c, V0), 0x4), c.g[A2][0]);
            c.sw(r(c, V0), c.g[A2][0]);
            c.sw(p(r(c, V0), 0x18), 0);
            let more = (c.g[A0][0] as i64) >= 0;
            c.g[V0][0] = sx(r(c, V0).wrapping_add(0x28));
            if !more {
                c.cycles += 27;
                c.g[A0][0] = c.g[A1][0];
                c.sw(gp(c, 0x6a8), c.g[A3][0]);
                c.g[RA][0] = sx(0x002a_e0a4);
                c.sw(gp(c, 0x6ac), c.g[A1][0]);
                call_ptr(c, 0x003b_5910, 0x002a_e0a4)?;
                c.g[A0][0] = sx(0x002a_dfe0);
                c.g[A1][0] = 0x5;
                c.g[RA][0] = sx(0x002a_e0b8);
                c.g[A2][0] = 0x64;
                call_ptr(c, 0x003e_57f8, 0x002a_e0b8)?;
                c.g[A2][0] = sx(0x003e_6070);
                c.g[A1][0] = sx(0x0041_75c8);
                c.g[A0][0] = sx(0x0025_2fa0);
                c.g[V1][0] = sx(0x0025_26b8);
                c.g[V0][0] = sx(0x002a_e000);
                c.g[RA][0] = c.ld(r(c, SP));
                c.sw(gp(c, 0x690), c.g[A2][0]);
                c.sw(gp(c, 0x694), c.g[A1][0]);
                c.sw(gp(c, 0x69c), c.g[A0][0]);
                c.sw(gp(c, 0x6a0), c.g[V1][0]);
                c.sw(gp(c, 0x6a4), c.g[V0][0]);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_e070);
            }
        }
        frame_less_return(c, 0x10);
        Ok(())
    })
}

/// The `addiu sp; jr ra` tail of a frame whose `ra` was reloaded from
/// `off(sp)` by the body.
#[inline(always)]
fn frame_less_return(c: &mut Cpu, size: u32) {
    let ra = r(c, RA);
    c.g[SP][0] = sx(r(c, SP).wrapping_add(size));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// cSetupMan message handlers.

/// The shared body of `cSetupMan_ClientRecvMsg4`/`Msg5` (0x2693c0/0x269188):
/// allocate a small network message holding the received payload word, chain
/// it onto the client's queue and store the queue head.
fn setup_man_client_recv_msg_alloc(c: &mut Cpu, ret1: u32, ret2: u32) -> Flow {
    c.cycles += 31;
    let frame = Frame::open(c, 0x60);
    let a0 = r(c, A0);
    let a1 = r(c, A1);
    c.g[A2][0] = 0x2000_0000;
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    c.g[S0][0] = 0xf7;
    frame.save(c, S2, 0x30);
    c.g[A3][0] = 0;
    frame.save_ra(c, 0x20);
    c.sw(a0, a1 as u64);
    c.g[A1][0] = sx(0x0048_0488);
    c.g[S1][0] = sx(Addr(gp(c, -0xc8)).lw(c) as u32);
    c.g[RA][0] = sx(ret1);
    c.g[A0][0] = 0x8;
    call_ptr(c, callee::MALLOC, ret1)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = 0x0048_0000;
    c.g[V0][0] = 0x0048_0000;
    c.sw(r(c, S2), c.g[S0][0]);
    c.g[V0][0] = sx(0x0048_12b0);
    c.g[A1][0] = sx(0x0048_04a8);
    c.sw(p(r(c, S2), 4), c.g[V0][0]);
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[V0][0] = Addr(p(r(c, S1), 0xf0)).lw(c);
    c.g[RA][0] = sx(ret2);
    c.sw(p(r(c, SP), 0x10), c.g[V0][0]);
    call_ptr(c, callee::MALLOC, ret2)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx(r(c, A1).wrapping_add(8));
    if c.g[V0][0] != 0 {
        c.sw(p(r(c, A1), 8), c.g[S2][0]);
    }
    c.cycles += 15;
    c.g[V0][0] = Addr(p(r(c, SP), 0x10)).lw(c);
    frame.restore(c, 16, 0x50);
    c.sw(r(c, A1), c.g[V0][0]);
    frame.restore(c, 17, 0x40);
    c.g[V1][0] = Addr(p(r(c, V0), 4)).lw(c);
    frame.restore(c, 18, 0x30);
    c.sw(p(r(c, A1), 4), c.g[V1][0]);
    c.g[RA][0] = c.ld(p(r(c, SP), 0x20));
    c.g[A0][0] = Addr(p(r(c, V0), 4)).lw(c);
    c.sw(r(c, A0), c.g[A1][0]);
    c.g[V0][0] = Addr(p(r(c, SP), 0x10)).lw(c);
    c.sw(p(r(c, V0), 4), c.g[A1][0]);
    c.sw(r(c, SP), c.g[A1][0]);
    frame_less_return(c, 0x60);
    Ok(())
}

/// `void cSetupMan_ClientRecvMsg5(void* q, int payload)` (0x269188).
pub fn setup_man_client_recv_msg5(c: &mut Cpu) {
    guest(c, |c| setup_man_client_recv_msg_alloc(c, 0x0026_91c0, 0x0026_91f4))
}

/// `void cSetupMan_ClientRecvMsg4(void* q, int payload)` (0x2693c0).
pub fn setup_man_client_recv_msg4(c: &mut Cpu) {
    guest(c, |c| setup_man_client_recv_msg_alloc(c, 0x0026_93f8, 0x0026_942c))
}

/// `void cSetupMan_ClientRecvMsg7(void* q, int payload)` (0x269890): build a
/// 0x10-byte host message with the payload at `+0xc` and queue it.
pub fn setup_man_client_recv_msg7(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x50);
        c.g[A0][0] = 0x10;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[A1][0] = sx(0x0048_0f18);
        frame.save_ra(c, 0x20);
        c.g[A3][0] = 0;
        call_ptr(c, callee::MALLOC, 0x0026_98bc)?;
        c.g[V1][0] = 0x0048_0000;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = sx(0x0048_1150);
        c.g[V0][0] = 0xfb;
        c.sw(p(r(c, S1), 4), c.g[V1][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.sw(r(c, S1), c.g[V0][0]);
        c.sw(p(r(c, S1), 0xc), c.g[S0][0]);
        c.g[A0][0] = 0xc;
        c.sw(p(r(c, S1), 8), 0);
        c.g[A2][0] = 0x2000_0000;
        c.g[V1][0] = Addr(gp(c, -0xc8)).lw(c);
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(p(r(c, V1), 0xf0)).lw(c);
        c.g[RA][0] = sx(0x0026_9900);
        c.sw(p(r(c, SP), 0x10), c.g[V0][0]);
        call_ptr(c, callee::MALLOC, 0x0026_9900)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r(c, A1).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw(p(r(c, A1), 8), c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = Addr(p(r(c, SP), 0x10)).lw(c);
        frame.restore(c, 16, 0x40);
        c.sw(r(c, A1), c.g[V0][0]);
        frame.restore(c, 17, 0x30);
        c.g[V1][0] = Addr(p(r(c, V0), 4)).lw(c);
        c.g[RA][0] = c.ld(p(r(c, SP), 0x20));
        c.sw(p(r(c, A1), 4), c.g[V1][0]);
        c.g[A0][0] = Addr(p(r(c, V0), 4)).lw(c);
        c.sw(r(c, A0), c.g[A1][0]);
        c.g[V0][0] = Addr(p(r(c, SP), 0x10)).lw(c);
        c.sw(p(r(c, V0), 4), c.g[A1][0]);
        c.sw(r(c, SP), c.g[A1][0]);
        frame_less_return(c, 0x50);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World.

/// `cWorld* cWorld::cWorld()` (0x3a6688): construct the world singleton — the
/// object cache, the script cache, the trigger manager hook object and the
/// path network — and publish it at `gp+0x16c8`.
pub fn world_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[A1][0] = sx(0x0049_4d58);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x1c00;
        c.g[RA][0] = sx(0x003a_66b4);
        Addr(gp(c, 0x16c8)).sw(c, c.g[S0][0]);
        call_ptr(c, callee::MALLOC, 0x003a_66b4)?;

        c.g[RA][0] = sx(0x003a_66bc);
        c.g[A0][0] = c.g[V0][0];
        callee::WORLD_CACHE_CTOR.call(c, 0x003a_66bc)?;

        c.g[A1][0] = sx(0x0049_4d68);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        Addr(r(c, S0)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x003a_66d8);
        c.g[A0][0] = 0x10;
        call_ptr(c, callee::MALLOC, 0x003a_66d8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003a_66e4);
        c.g[A0][0] = c.g[V0][0];
        callee::WSCRIPT_CACHE_CTOR.call(c, 0x003a_66e4)?;

        c.g[A1][0] = sx(0x0049_4d78);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        Addr(p(r(c, S0), 4)).sw(c, c.g[V0][0]);
        c.g[RA][0] = sx(0x003a_6700);
        c.g[A0][0] = 8;
        call_ptr(c, callee::MALLOC, 0x003a_6700)?;

        c.g[RA][0] = sx(0x003a_6708);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002b_b130, 0x003a_6708)?;

        Addr(p(r(c, S0), 0x10)).sw(c, 0);
        c.g[A0][0] = 0xa;
        Addr(p(r(c, S0), 0x14)).sw(c, 0);
        c.g[A1][0] = 4;
        Addr(p(r(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 2;
        c.g[RA][0] = sx(0x003a_6728);
        c.g[A3][0] = 5;
        call_ptr(c, 0x0032_dc10, 0x003a_6728)?;

        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Board trail FX.

/// `void cBoardTrailFX_Render(cBoardTrailFX* fx)` (0x2ea480): flush the trail's
/// queued nodes when the effect is enabled and the current render pass owns
/// it, otherwise drop the oldest entry when the queue is at least two deep.
pub fn c_board_trail_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        'outer: {
            'mid: {
                frame.save(c, S0, 0x10);
                c.g[V0][0] = Addr(gp(c, 0x1428)).lw(c);
                let this = r(c, A0);
                frame.save_ra(c, 0);
                let enabled = c.g[V0][0] != 0;
                c.g[S0][0] = sx(this);
                if enabled {
                    break 'mid;
                }
                'inner: {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(gp(c, 0x142c)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(p(r(c, S0), 0x90)).lw(c);
                        break 'inner;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = Addr(r(c, S0)).lw(c);
                    c.g[V1][0] = Addr(p(r(c, A0), 0x6c0)).lw(c);
                    c.g[A0][0] = sx(p(r(c, A0), 0x6c0));
                    c.g[A1][0] = Addr(p(r(c, V1), 0x48)).lh(c);
                    c.g[V0][0] = Addr(p(r(c, V1), 0x4c)).lw(c);
                    let target = r(c, V0);
                    c.g[RA][0] = sx(0x002e_a4c0);
                    c.g[A0][0] = sx(p(r(c, A0), r(c, A1)));
                    call_ptr(c, target, 0x002e_a4c0)?;
                    if c.g[V0][0] != 0 {
                        frame.restore(c, 16, 0x10);
                        break 'outer;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r(c, S0), 0x90)).lw(c);
                    break 'inner;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
                let few = c.g[V0][0] != 0;
                frame.restore(c, 16, 0x10);
                if few {
                    break 'outer;
                }
                // Push the oldest node: shift 0x14 bytes of the node up by
                // 0x14 and drop its last word into the new slot.
                c.cycles += 19;
                c.g[A0][0] = Addr(gp(c, -0x854)).lw(c);
                c.g[V0][0] = Addr(p(r(c, A0), 0xe84)).lw(c);
                c.ldl(3, p(r(c, V0), 7));
                c.ldr(3, r(c, V0));
                c.ldl(5, p(r(c, V0), 0xf));
                c.ldr(5, p(r(c, V0), 8));
                c.g[A2][0] = Addr(p(r(c, V0), 0x10)).lw(c);
                c.sdl(3, p(r(c, V0), 0x1b));
                c.sdr(3, p(r(c, V0), 0x14));
                c.sdl(5, p(r(c, V0), 0x23));
                c.sdr(5, p(r(c, V0), 0x1c));
                c.sw(p(r(c, V0), 0x24), c.g[A2][0]);
                c.g[V1][0] = Addr(p(r(c, A0), 0xe84)).lw(c);
                c.g[A1][0] = Addr(gp(c, -0x854)).lw(c);
                c.g[V1][0] = sx(r(c, V1).wrapping_add(0x14));
                c.sw(p(r(c, A0), 0xe84), c.g[V1][0]);
                c.g[V0][0] = Addr(p(r(c, A1), 0xe84)).lw(c);
                c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x14));
                c.sw(p(r(c, A1), 0xe84), c.g[V0][0]);
                break 'mid;
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
// FE state: bragging rights.

/// `void cFEStateBraggingRights_StartWaitAnim(cFEStateBraggingRights* s)`
/// (0x194080): start the idle "wait" animation of the screen's primary thread
/// — either the widget stored at `+0x6f8`/`+0x72c`, or the screen's `+0x40`
/// thread object, looking the animation up by hash.
pub fn fe_state_bragging_rights_start_wait_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        'outer: {
            'mid2: {
                'mid: {
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    let this = r(c, A0);
                    c.g[S0][0] = sx(this);
                    c.g[V0][0] = Addr(p(this, 0x6f8)).lw(c);
                    let has_anim = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(p(this, 0x72c)).lw(c);
                    if has_anim {
                        break 'mid;
                    }
                    'inner: {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = Addr(p(r(c, S0), 0x6f8)).lw(c);
                            break 'inner;
                        }
                        c.cycles += 10;
                        c.g[A0][0] = sx(0x004a_1790);
                        c.g[RA][0] = sx(0x0019_40b0);
                        call_ptr(c, callee::FE_STRING_HASH, 0x0019_40b0)?;
                        c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                        c.g[RA][0] = sx(0x0019_40bc);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x0019_40bc)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let missing = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] = c.g[A1][0] & 0xffff;
                        if missing {
                            break 'mid2;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                        c.g[RA][0] = sx(0x0019_40d8);
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0039_c870, 0x0019_40d8)?;
                        frame.restore(c, 16, 0x10);
                        break 'outer;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        frame.restore(c, 16, 0x10);
                        break 'outer;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r(c, S0), 0x72c)).lw(c);
                    break 'mid;
                }
                c.cycles += 2;
                let idle = c.g[V0][0] == 0;
                c.g[A0][0] = sx(0x004a_0000);
                if idle {
                    break 'mid2;
                }
                c.cycles += 9;
                c.g[RA][0] = sx(0x0019_40fc);
                c.g[A0][0] = sx(0x004a_1798);
                call_ptr(c, callee::FE_STRING_HASH, 0x0019_40fc)?;
                c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                c.g[RA][0] = sx(0x0019_4108);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_c778, 0x0019_4108)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let missing = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if missing {
                    break 'mid2;
                }
                c.cycles += 3;
                c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                c.g[RA][0] = sx(0x0019_4124);
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x0019_4124)?;
                break 'mid2;
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
// FE locale.

/// `char* FELocale_v04(FELocale* l, int lang)` (0x195eb8): return the locale
/// string for `lang`, looking it up in the table at `+8` (the first byte of
/// the table entry must match `l->kind`) or falling back to the default
/// translation of the key at `+0x5a`.
pub fn fe_locale_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        'outer: {
            'mid: {
                frame.save(c, S1, 0x40);
                frame.save(c, S3, 0x20);
                let this = r(c, A0);
                c.g[S1][0] = sx(this);
                frame.save(c, S0, 0x50);
                frame.save(c, S2, 0x30);
                frame.save(c, S4, 0x10);
                c.g[S3][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[V0][0] = Addr(this).lb(c);
                let cached = c.g[V0][0] != 0;
                if cached {
                    break 'mid;
                }
                c.cycles += 3;
                c.g[S2][0] = 0x0100_0000;
                c.g[S0][0] = sx(this.wrapping_add(8));
                c.g[S4][0] = 0x0100_0000;
                loop {
                    'entry: {
                        c.cycles += 3;
                        c.g[A0][0] = Addr(r(c, S0)).lw(c);
                        let empty = c.g[A0][0] == 0;
                        c.g[V0][0] = c.g[S2][0];
                        if empty {
                            break 'entry;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = Addr(r(c, A0)).lb(c);
                        c.g[V0][0] = Addr(p(r(c, S1), 1)).lb(c);
                        let differs = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = c.g[S2][0];
                        if differs {
                            break 'entry;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0019_5f14);
                        c.g[A1][0] = c.g[S3][0];
                        call_ptr(c, 0x002c_6de0, 0x0019_5f14)?;
                        if c.g[V0][0] != 0 {
                            frame.restore(c, 16, 0x50);
                            break 'outer;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S2][0];
                        break 'entry;
                    }
                    c.cycles += 5;
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                    c.g[V0][0] = sx((r(c, V0) as i32 >> 24) as u32);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 20) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S2][0] = sx(r(c, S2).wrapping_add(r(c, S4)));
                    if !more {
                        break 'mid;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_5ef0);
                    }
                }
            }
            c.cycles += 7;
            c.g[S0][0] = sx(p(r(c, S1), 0x5a));
            c.g[A1][0] = sx(0x004a_1390);
            c.g[RA][0] = sx(0x0019_5f48);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c_27c0, 0x0019_5f48)?;
            c.g[V0][0] = c.g[S0][0];
            frame.restore(c, 16, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE state: lodge rider detail.

/// `void cFEStateLodgeRiderDetail_Init(cFEStateLodgeRiderDetail* s)` (0x1f4050):
/// create the "rider detail" screen from its name hash, play its first frame,
/// then install the screen's rider name and the current peak's description.
pub fn fe_state_lodge_rider_detail_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        'outer: {
            frame.save(c, S1, 0x10);
            let this = r(c, A0);
            c.g[S1][0] = sx(this);
            frame.save_ra(c, 0);
            frame.save(c, S0, 0x20);
            c.g[A0][0] = sx(0x0046_ef20);
            c.g[RA][0] = sx(0x001f_4074);
            c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
            call_ptr(c, callee::FE_STRING_HASH, 0x001f_4074)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001f_4088);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001f_4088)?;
            let none = c.g[V0][0] == 0;
            Addr(p(r(c, S1), 0x40)).sw(c, c.g[V0][0]);
            if none {
                break 'outer;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_40a0);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_40a0)?;
            'mid: {
                c.cycles += 20;
                c.g[V0][0] = 1;
                c.g[RA][0] = sx(0x001f_40ac);
                Addr(p(r(c, S1), 0x48)).sw(c, c.g[V0][0]);
                call_ptr(c, 0x0014_dc80, 0x001f_40ac)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f_40b8);
                c.g[A1][0] = 1;
                call_ptr(c, callee::BE_GET_INTERFACE, 0x001f_40b8)?;
                c.g[A1][0] = Addr(p(r(c, S1), 0x44)).lb(c);
                c.g[RA][0] = sx(0x001f_40c4);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_7488, 0x001f_40c4)?;
                c.g[RA][0] = sx(0x001f_40cc);
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dc80, 0x001f_40cc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f_40d8);
                c.g[A1][0] = 0xd;
                call_ptr(c, callee::BE_GET_INTERFACE, 0x001f_40d8)?;
                c.g[A1][0] = Addr(p(r(c, S1), 0x44)).lb(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f_40e8);
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x0015_77a0, 0x001f_40e8)?;
                if c.g[V0][0] == 0 {
                    Addr(p(r(c, S1), 0x48)).sw(c, 0);
                    break 'mid;
                }
                break 'mid;
            }
            break 'outer;
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
// Camera trigger file loading.

/// `int CameraTriggerMan_LoadFile(CameraTriggerMan* m, const char* name)`
/// (0x16c170): free the old trigger list, load and parse the named file,
/// returning 0 on success, 1 when the file is missing and 2 when the buffer
/// could not be allocated.
pub fn camera_trigger_man_load_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        'outer: {
            frame.save(c, S2, 0x20);
            frame.save(c, S3, 0x10);
            c.g[S2][0] = c.g[A1][0];
            let this = r(c, A0);
            c.g[S3][0] = sx(this);
            frame.save(c, S0, 0x40);
            frame.save(c, S1, 0x30);
            frame.save_ra(c, 0);
            c.g[RA][0] = sx(0x0016_c198);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_7f38, 0x0016_c198)?;
            let missing = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if missing {
                break 'outer;
            }
            'mid: {
                c.cycles += 18;
                c.g[RA][0] = sx(0x0016_c1a8);
                c.g[A0][0] = c.g[S3][0];
                callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS.call(c, 0x0016_c1a8)?;
                c.g[RA][0] = sx(0x0016_c1b0);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0031_7f98, 0x0016_c1b0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = sx(0x0045_bf30);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0x100;
                c.g[RA][0] = sx(0x0016_c1cc);
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x0016_c1cc)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0016_c1e0);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003e_2470, 0x0016_c1e0)?;
                let ok = c.g[S1][0] != 0;
                c.g[A0][0] = c.g[S3][0];
                if ok {
                    break 'mid;
                }
                c.cycles += 2;
                c.g[V0][0] = 2;
                break 'outer;
            }
            c.cycles += 6;
            c.g[RA][0] = sx(0x0016_c1f8);
            c.g[A1][0] = c.g[S1][0];
            callee::CAMERA_TRIGGER_MAN_PARSE.call(c, 0x0016_c1f8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0016_c204);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e98, 0x0016_c204)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Process list.

/// `void ProcessList_DeleteSectionObjects(ProcessList* l, int section, int id)`
/// (0x3551a8): walk the list and delete every section object whose key matches
/// `id`.
pub fn process_list_delete_section_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        'outer: {
            frame.save(c, S2, 0x30);
            frame.save(c, S3, 0x20);
            c.g[S2][0] = c.g[A1][0];
            let this = r(c, A0);
            c.g[S3][0] = sx(this);
            frame.save(c, S4, 0x10);
            frame.save(c, S0, 0x50);
            frame.save(c, S1, 0x40);
            frame.save_ra(c, 0);
            c.g[S4][0] = c.g[A2][0];
            callee::PROCESS_LIST_FIRST.call(c, 0x0035_51d4)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                loop {
                    'entry: {
                        c.cycles += 11;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x0035_51f4);
                        c.g[A2][0] = c.g[S2][0];
                        callee::PROCESS_LIST_NEXT.call(c, 0x0035_51f4)?;
                        c.g[V1][0] = Addr(p(r(c, S0), 0xc)).lw(c);
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A0][0] = Addr(p(r(c, V1), 0x58)).lh(c);
                        c.g[V0][0] = Addr(p(r(c, V1), 0x5c)).lw(c);
                        let target = r(c, V0);
                        c.g[RA][0] = sx(0x0035_520c);
                        c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
                        call_ptr(c, target, 0x0035_520c)?;
                        if c.g[V0][0] != c.g[S4][0] {
                            c.g[S0][0] = c.g[S1][0];
                            break 'entry;
                        }
                        'del: {
                            c.cycles += 2;
                            let gone = c.g[S0][0] == 0;
                            c.g[A1][0] = 3;
                            if gone {
                                break 'del;
                            }
                            c.cycles += 5;
                            c.g[V0][0] = Addr(p(r(c, S0), 0xc)).lw(c);
                            c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                            c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                            let target = r(c, V1);
                            c.g[RA][0] = sx(0x0035_5230);
                            c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
                            call_ptr(c, target, 0x0035_5230)?;
                            break 'del;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = c.g[S1][0];
                        break 'entry;
                    }
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        c.cycles += 1;
                        frame.restore(c, 16, 0x50);
                        break 'outer;
                    }
                    c.g[A0][0] = c.g[S3][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_51e8);
                    }
                }
            }
            frame.restore(c, 16, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World painter query list.

/// `void WorldPainterQuery_dtor(WorldPainterQuery* q, int free)` (0x2c06c0):
/// unlink the query from the painter's list (`gp+0x790`), release its payload
/// and optionally free it.
pub fn world_painter_query_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        'outer: {
            'mid: {
                'first: {
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[V0][0] = sx(0x0048_3e00);
                    let this = r(c, A0);
                    c.g[S0][0] = sx(this);
                    c.g[S1][0] = c.g[A1][0];
                    frame.save_ra(c, 0);
                    Addr(p(this, 4)).sw(c, c.g[V0][0]);
                    c.g[RA][0] = sx(0x002c_06ec);
                    c.g[A0][0] = Addr(this).lw(c);
                    call_ptr(c, callee::FREE, 0x002c_06ec)?;
                    c.g[V1][0] = Addr(gp(c, 0x790)).lw(c);
                    if c.g[V1][0] != c.g[S0][0] {
                        c.g[V0][0] = Addr(p(r(c, V1), 0x14)).lw(c);
                        break 'first;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(r(c, S0), 0x14)).lw(c);
                    c.g[A1][0] = c.g[S1][0] & 1;
                    c.g[A0][0] = sx(0x0048_0000);
                    Addr(gp(c, 0x790)).sw(c, c.g[V0][0]);
                    break 'mid;
                }
                'second: {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S1][0] & 1;
                    let hit = c.g[V0][0] == c.g[S0][0];
                    c.g[A0][0] = sx(0x0048_0000);
                    if hit {
                        break 'second;
                    }
                    c.cycles += 2;
                    c.g[V1][0] = Addr(p(r(c, V1), 0x14)).lw(c);
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = Addr(p(r(c, V1), 0x14)).lw(c);
                        if c.g[V0][0] == c.g[S0][0] {
                            break 'second;
                        }
                        c.g[V1][0] = Addr(p(r(c, V1), 0x14)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x002c_0720);
                        }
                    }
                }
                c.cycles += 2;
                let empty = c.g[V1][0] == 0;
                c.g[V0][0] = sx(p(r(c, A0), 0x5d30));
                if empty {
                    break 'outer;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(p(r(c, S0), 0x14)).lw(c);
                Addr(p(r(c, V1), 0x14)).sw(c, c.g[V0][0]);
                break 'mid;
            }
            c.cycles += 1;
            c.g[V0][0] = sx(p(r(c, A0), 0x5d30));
        }
        'tail: {
            c.cycles += 2;
            let keep = c.g[A1][0] == 0;
            Addr(p(r(c, S0), 4)).sw(c, c.g[V0][0]);
            if keep {
                break 'tail;
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x002c_0760);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::FREE, 0x002c_0760)?;
            break 'tail;
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
// FE state: character select.

/// `cFEStateCharSelect* cFEStateCharSelect_ctor(cFEStateCharSelect* s, void*
/// screen, int count)` (0x181090): install the character-select screen and
/// pre-fill every rider slot it owns.
pub fn fe_state_char_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        'outer: {
            frame.save(c, S1, 0x40);
            let this = r(c, A0);
            c.g[S1][0] = sx(this);
            c.g[A2][0] = sx((r(c, A2) as u32) << 24);
            frame.save(c, S2, 0x30);
            frame.save(c, S0, 0x50);
            c.g[S2][0] = sx((r(c, A2) as i32 >> 24) as u32);
            frame.save(c, S3, 0x20);
            frame.save(c, S4, 0x10);
            c.g[S0][0] = 0;
            frame.save_ra(c, 0);
            c.g[S4][0] = 0xff;
            call_ptr(c, 0x0039_e2a0, 0x0018_10c4)?;

            c.g[V0][0] = sx(0x0046_d000);
            c.g[A0][0] = 9;
            c.g[V1][0] = 1;
            Addr(p(r(c, S1), 8)).sw(c, c.g[V0][0]);
            Addr(p(r(c, S1), 0xc)).sw(c, c.g[A0][0]);
            Addr(p(r(c, S1), 0x54)).sw(c, c.g[V1][0]);
            Addr(p(r(c, S1), 0x44)).sb(c, c.g[S2][0]);
            Addr(p(r(c, S1), 0x48)).sw(c, 0);
            Addr(p(r(c, S1), 0x4c)).sw(c, 0);
            c.g[V0][0] = Addr(p(r(c, S1), 0x10)).lw(c);
            let none = (c.g[S2][0] as i64) <= 0;
            c.g[S3][0] = Addr(r(c, V0)).lw(c);
            if none {
                break 'outer;
            }
            loop {
                c.cycles += 9;
                c.g[A1][0] = sx((r(c, S0) as u32) << 24);
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = sx((r(c, A1) as i32 >> 24) as u32);
                c.g[RA][0] = sx(0x0018_110c);
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                call_ptr(c, 0x001a_1cd0, 0x0018_110c)?;
                c.g[V0][0] = !(0u64 | c.g[V0][0]);
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                c.g[S4][0] = c.g[S4][0] & c.g[V0][0];
                if !more {
                    break 'outer;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0018_10f8);
                }
            }
        }
        c.cycles += 10;
        Addr(p(r(c, S1), 0x15)).sb(c, c.g[S4][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Algorithm list.

/// `void AlgorithmList_Remove(AlgorithmList* list, AlgorithmList::Node* n)`
/// (0x15ca50): unlink a list node, drop its held object when it is the head
/// and free the node; return the list's new length minus one.
pub fn algorithm_list_remove(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        'outer: {
            'prev: {
                frame.save(c, S0, 0x30);
                frame.save(c, S2, 0x10);
                c.g[S0][0] = c.g[A1][0];
                frame.save(c, S1, 0x20);
                let list = r(c, A0);
                c.g[S2][0] = sx(list);
                frame.save_ra(c, 0);
                c.g[V0][0] = Addr(p(r(c, S0), 0x10)).lw(c);
                let has_prev = c.g[V0][0] != 0;
                c.g[S1][0] = Addr(p(r(c, S0), 0x14)).lw(c);
                if has_prev {
                    break 'prev;
                }
                c.cycles += 2;
                Addr(r(c, S2)).sw(c, c.g[S1][0]);
                break 'outer;
            }
            c.cycles += 1;
            Addr(p(r(c, V0), 0x14)).sw(c, c.g[S1][0]);
        }
        'next: {
            c.cycles += 3;
            c.g[V1][0] = Addr(p(r(c, S0), 0x14)).lw(c);
            let last = c.g[V1][0] == 0;
            if last {
                break 'next;
            }
            c.cycles += 2;
            c.g[V0][0] = Addr(p(r(c, S0), 0x10)).lw(c);
            Addr(p(r(c, V1), 0x10)).sw(c, c.g[V0][0]);
        }
        'held: {
            c.cycles += 2;
            if c.g[S0][0] != 0 {
                'owned: {
                    c.cycles += 3;
                    c.g[A2][0] = Addr(r(c, S0)).lw(c);
                    if c.g[A2][0] == 0 {
                        Addr(r(c, S0)).sw(c, 0);
                        break 'owned;
                    }
                    'release: {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(p(r(c, S0), 0x18)).lw(c);
                        let plain = c.g[V0][0] == 0;
                        c.g[A1][0] = 3;
                        if plain {
                            break 'release;
                        }
                        c.cycles += 5;
                        c.g[V0][0] = Addr(p(r(c, A2), 0x10)).lw(c);
                        c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                        c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                        let target = r(c, V1);
                        c.g[RA][0] = sx(0x0015_cacc);
                        c.g[A0][0] = sx(p(r(c, A2), r(c, A0)));
                        call_ptr(c, target, 0x0015_cacc)?;
                        break 'release;
                    }
                    c.cycles += 1;
                    Addr(r(c, S0)).sw(c, 0);
                    break 'owned;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0015_cadc);
                Addr(p(r(c, S0), 0x18)).sw(c, 0);
                call_ptr(c, callee::FREE, 0x0015_cadc)?;
                c.g[V1][0] = Addr(p(r(c, S2), 4)).lw(c);
                break 'held;
            }
            c.g[V1][0] = Addr(p(r(c, S2), 4)).lw(c);
            break 'held;
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x30);
        c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
        frame.restore(c, 17, 0x20);
        Addr(p(r(c, S2), 4)).sw(c, c.g[V1][0]);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE state: two-row popup screens (the `_v25` handlers).

/// The shared body of the three `_v25` handlers below: set the popup's state,
/// then three rows of (label hash, text hash, icon) built from the FE string
/// table at `0x460000`.
fn fe_v25_populate(c: &mut Cpu, base: u32, strings: [u32; 5], icons: [u64; 3]) -> Flow {
    c.cycles += 46;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = 3;
    c.g[RA][0] = sx(base.wrapping_add(0x1c));
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x001d_a528, base.wrapping_add(0x1c))?;

    c.g[A0][0] = sx(0x0046_0000);
    c.g[RA][0] = sx(base.wrapping_add(0x28));
    c.g[A0][0] = sx(strings[0]);
    call_ptr(c, callee::FE_STRING_HASH, base.wrapping_add(0x28))?;
    c.g[A0][0] = sx(0x0046_0000);
    Addr(p(r(c, S0), 0x54)).sw(c, c.g[V0][0]);
    c.g[RA][0] = sx(base.wrapping_add(0x38));
    c.g[A0][0] = sx(strings[1]);
    call_ptr(c, callee::FE_STRING_HASH, base.wrapping_add(0x38))?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[RA][0] = sx(base.wrapping_add(0x48));
    c.g[A1][0] = 0;
    call_ptr(c, 0x001d_a530, base.wrapping_add(0x48))?;

    c.g[A0][0] = sx(0x0046_0000);
    c.g[RA][0] = sx(base.wrapping_add(0x54));
    c.g[A0][0] = sx(strings[2]);
    call_ptr(c, callee::FE_STRING_HASH, base.wrapping_add(0x54))?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[RA][0] = sx(base.wrapping_add(0x68));
    c.g[A3][0] = icons[0];
    call_ptr(c, 0x001d_a510, base.wrapping_add(0x68))?;

    c.g[A0][0] = sx(0x0046_0000);
    c.g[RA][0] = sx(base.wrapping_add(0x74));
    c.g[A0][0] = sx(strings[3]);
    call_ptr(c, callee::FE_STRING_HASH, base.wrapping_add(0x74))?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    c.g[RA][0] = sx(base.wrapping_add(0x88));
    c.g[A3][0] = icons[1];
    call_ptr(c, 0x001d_a510, base.wrapping_add(0x88))?;

    c.g[A0][0] = sx(0x0046_0000);
    c.g[RA][0] = sx(base.wrapping_add(0x94));
    c.g[A0][0] = sx(strings[4]);
    call_ptr(c, callee::FE_STRING_HASH, base.wrapping_add(0x94))?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 2;
    c.g[RA][0] = sx(base.wrapping_add(0xa8));
    c.g[A3][0] = icons[2];
    call_ptr(c, 0x001d_a510, base.wrapping_add(0xa8))?;

    frame.restore(c, 16, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cFEUseExistingAccount_v25(cFEState* s, void* tooltip)` (0x1ac4f0).
pub fn fe_use_existing_account_v25(c: &mut Cpu) {
    guest(c, |c| {
        fe_v25_populate(
            c,
            0x001a_c4f0,
            [0x0046_2c08, 0x0046_2c28, 0x0045_fe60, 0x0046_2c50, 0x0046_14e8],
            [3, 0, 1],
        )
    })
}

/// `void cFEStateOnlineCharacterSelect_v25(cFEState* s, void* tooltip)`
/// (0x1b14d8).
pub fn fe_state_online_character_select_v25(c: &mut Cpu) {
    guest(c, |c| {
        fe_v25_populate(
            c,
            0x001b_14d8,
            [0x0046_3848, 0x0046_3868, 0x0046_3890, 0x0046_38b0, 0x0046_14e8],
            [4, 0, 1],
        )
    })
}

/// `void cFEStateNetNews_v25(cFEState* s, void* tooltip)` (0x1c5540).
pub fn fe_state_net_news_v25(c: &mut Cpu) {
    guest(c, |c| {
        fe_v25_populate(
            c,
            0x001c_5540,
            [0x0046_5f58, 0x0046_5f70, 0x0046_5f90, 0x0046_14e8, 0x0046_1458],
            [3, 1, 2],
        )
    })
}

// ---------------------------------------------------------------------------
// FE state: challenge popup.

/// `cFEStateChallengePopup* cFEStateChallengePopup_ctor(cFEStateChallengePopup*
/// s)` (0x1addd0): construct the network sub-state, install the popup vtable
/// and the two option rows.
pub fn fe_state_challenge_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 1;
        callee::NET_FE_STATE_CTOR.call(c, 0x001a_ddf0)?;

        c.g[V0][0] = sx(0x0046_8e38);
        c.g[V1][0] = sx(0x0044_1268);
        c.g[A0][0] = 0x3f;
        Addr(p(r(c, S1), 0x6d8)).sw(c, c.g[V1][0]);
        Addr(p(r(c, S1), 0xc)).sw(c, c.g[A0][0]);
        Addr(p(r(c, S1), 8)).sw(c, c.g[V0][0]);
        Addr(p(r(c, S1), 0x6d0)).sw(c, 0);
        Addr(p(r(c, S1), 0x6dc)).sw(c, 0);
        c.g[RA][0] = sx(0x001a_de20);
        Addr(gp(c, 0x1d98)).sw(c, 0);
        call_ptr(c, 0x0014_dc80, 0x001a_de20)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001a_de2c);
        c.g[A1][0] = 0;
        call_ptr(c, callee::BE_GET_INTERFACE, 0x001a_de2c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[V0][0] = sx(0x0053_0000);
        c.g[A0][0] = c.g[S0][0];
        Addr(gp(c, -0x1f38)).sw(c, c.g[S2][0]);
        c.g[RA][0] = sx(0x001a_de48);
        Addr(p(r(c, V0), 0x5c08)).sw(c, 0);
        call_ptr(c, 0x0014_4df0, 0x001a_de48)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001a_de54);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_51e8, 0x001a_de54)?;
        c.g[V1][0] = Addr(p(r(c, S0), 0xc)).lw(c);
        c.g[A0][0] = Addr(p(r(c, V1), 8)).lh(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0xc)).lw(c);
        let target = r(c, V0);
        c.g[RA][0] = sx(0x001a_de68);
        c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
        call_ptr(c, target, 0x001a_de68)?;
        Addr(p(r(c, S1), 0x6d4)).sw(c, c.g[S2][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE state: chat room.

/// `void cFEStateChatRoom_OnKeyboardDone(cFEStateChatRoom* s)` (0x1b33d8):
/// when the keyboard popup was accepted, post the edited text to the lobby.
pub fn fe_state_chat_room_on_keyboard_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        'outer: {
            'accepted: {
                c.g[V0][0] = 2;
                frame.save(c, S1, 0x10);
                frame.save(c, S0, 0x20);
                let this = r(c, A0);
                c.g[S1][0] = sx(this);
                frame.save_ra(c, 0);
                c.g[A1][0] = Addr(p(this, 0x69c)).lw(c);
                c.g[V1][0] = Addr(p(r(c, A1), 0x18)).lw(c);
                let ok = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 4;
                if ok {
                    break 'accepted;
                }
                c.cycles += 2;
                let cancel = c.g[V1][0] != c.g[V0][0];
                frame.restore(c, 16, 0x20);
                if cancel {
                    break 'outer;
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                c.g[RA][0] = sx(0x001b_3414);
                c.g[A1][0] = sx(p(r(c, A1), 0x74));
                call_ptr(c, 0x0025_aae0, 0x001b_3414)?;
                frame.restore(c, 16, 0x20);
                break 'outer;
            }
            'lobby: {
                c.cycles += 3;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                let present = c.g[V0][0] != 0;
                if present {
                    break 'lobby;
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x001b_3430);
                call_ptr(c, 0x0026_0f80, 0x001b_3430)?;
                break 'lobby;
            }
            'update: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001b_3438);
                c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                call_ptr(c, 0x0026_1460, 0x001b_3438)?;
                let busy = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                if busy {
                    break 'update;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x001b_3448);
                c.g[A0][0] = Addr(gp(c, 0x238)).lw(c);
                call_ptr(c, 0x0026_13c8, 0x001b_3448)?;
                c.g[V0][0] = Addr(gp(c, 0x238)).lw(c);
                break 'update;
            }
            'bind: {
                c.cycles += 2;
                let present = c.g[V0][0] != 0;
                c.g[S0][0] = Addr(gp(c, 0x238)).lw(c);
                if present {
                    break 'bind;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x001b_345c);
                c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                call_ptr(c, 0x0025_b800, 0x001b_345c)?;
                c.g[S0][0] = Addr(gp(c, 0x238)).lw(c);
                break 'bind;
            }
            c.cycles += 8;
            c.g[RA][0] = sx(0x001b_3468);
            c.g[A0][0] = c.g[S1][0];
            callee::NET_FE_STATE_GET_SELECTED_USER.call(c, 0x001b_3468)?;
            c.g[A2][0] = Addr(p(r(c, S1), 0x69c)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001b_347c);
            c.g[A2][0] = sx(p(r(c, A2), 0x74));
            call_ptr(c, 0x0026_2690, 0x001b_347c)?;
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
// FE state: chat room player selection.

/// `void cFEStateChatRoom_SelectPlayerByName(cFEStateChatRoom* s, const char*
/// name)` (0x1b3978): find the room user whose name matches and select that
/// player in the chat room's screen.
pub fn fe_state_chat_room_select_player_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        'outer: {
            frame.save(c, S3, 0x20);
            frame.save(c, S4, 0x10);
            frame.save(c, S0, 0x50);
            frame.save(c, S1, 0x40);
            frame.save(c, S2, 0x30);
            frame.save_ra(c, 0);
            let this = r(c, A0);
            c.g[S4][0] = sx(this);
            c.g[V0][0] = Addr(p(this, 0x40)).lw(c);
            let no_screen = c.g[V0][0] == 0;
            c.g[S3][0] = c.g[A1][0];
            if no_screen {
                break 'outer;
            }
            c.cycles += 5;
            c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
            c.g[S2][0] = u64::MAX;
            c.g[RA][0] = sx(0x001b_39b4);
            c.g[S0][0] = 0;
            call_ptr(c, 0x0025_f1b0, 0x001b_39b4)?;
            c.g[S1][0] = c.g[V0][0];
            'scan: loop {
                'search: {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let end = c.g[V0][0] == 0;
                    c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                    if end {
                        break 'search;
                    }
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001b_39d0);
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0025_f1e0, 0x001b_39d0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001b_39dc);
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x0041_65a8, 0x001b_39dc)?;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                        if event_due(c) {
                            deliver_event_here(c, 0x001b_39b8);
                        }
                        continue 'scan;
                    }
                    c.cycles += 1;
                    c.g[S2][0] = c.g[S0][0];
                    break 'search;
                }
                c.cycles += 2;
                let found = (c.g[S2][0] as i64) < 0;
                c.g[A0][0] = sx(0x0046_0000);
                if found {
                    break 'outer;
                }
                c.cycles += 8;
                c.g[RA][0] = sx(0x001b_39f8);
                c.g[A0][0] = sx(0x0046_3a38);
                call_ptr(c, callee::FE_STRING_HASH, 0x001b_39f8)?;
                c.g[A0][0] = Addr(p(r(c, S4), 0x40)).lw(c);
                c.g[RA][0] = sx(0x001b_3a04);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001b_3a04)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001b_3a10);
                c.g[A1][0] = c.g[S2][0] & 0xff;
                call_ptr(c, 0x0039_b960, 0x001b_3a10)?;
                break 'outer;
            }
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE state: network config teardown.

/// `void cFEStateNetConfig_dtor(cFEStateNetConfig* s, int free)` (0x1b4018):
/// release the four optional sub-objects, then the net state itself.
pub fn fe_state_net_config_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0046_8b00);
        frame.save(c, S0, 0x20);
        let this = r(c, A0);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        Addr(p(this, 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p(this, 0x710)).lw(c);
        let absent = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !absent {
            c.cycles += 3;
            c.g[RA][0] = sx(0x001b_404c);
            call_ptr(c, callee::FREE2, 0x001b_404c)?;
            Addr(p(r(c, S0), 0x710)).sw(c, 0);
        }
        'first: {
            c.cycles += 3;
            c.g[A0][0] = Addr(p(r(c, S0), 0x714)).lw(c);
            if c.g[A0][0] == 0 {
                c.g[A0][0] = Addr(p(r(c, S0), 0x718)).lw(c);
                break 'first;
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x001b_4064);
            call_ptr(c, callee::FREE2, 0x001b_4064)?;
            Addr(p(r(c, S0), 0x714)).sw(c, 0);
            c.g[A0][0] = Addr(p(r(c, S0), 0x718)).lw(c);
        }
        'second: {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.g[A0][0] = Addr(p(r(c, S0), 0x704)).lw(c);
                break 'second;
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x001b_407c);
            call_ptr(c, callee::FREE2, 0x001b_407c)?;
            Addr(p(r(c, S0), 0x718)).sw(c, 0);
            c.g[A0][0] = Addr(p(r(c, S0), 0x704)).lw(c);
        }
        'third: {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.g[A0][0] = Addr(gp(c, -0x1578)).lw(c);
                break 'third;
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x001b_4094);
            call_ptr(c, callee::FREE2, 0x001b_4094)?;
            Addr(p(r(c, S0), 0x704)).sw(c, 0);
            c.g[A0][0] = Addr(gp(c, -0x1578)).lw(c);
        }
        'fourth: {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.g[A0][0] = c.g[S0][0];
                break 'fourth;
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x001b_40ac);
            call_ptr(c, 0x002c_2300, 0x001b_40ac)?;
            Addr(gp(c, -0x1578)).sw(c, 0);
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x001b_40bc);
        c.g[A1][0] = c.g[S1][0];
        callee::NET_FE_STATE_DTOR.call(c, 0x001b_40bc)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE state: create step 2.

/// `void cFEStateCreateStep2_Init(cFEStateCreateStep2* s)` (0x1ba428): create
/// the "create rider, step 2" screen, play its first frame, clear the date
/// selection array and hand the six option widgets their initial state.
pub fn fe_state_create_step2_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        'outer: {
            c.g[V0][0] = Addr(gp(c, -0xc8)).lw(c);
            frame.save(c, S1, 0x10);
            let this = r(c, A0);
            c.g[S1][0] = sx(this);
            frame.save(c, S0, 0x20);
            frame.save_ra(c, 0);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(0x260));
            Addr(p(this, 0x6d4)).sw(c, 0);
            c.g[A0][0] = sx(0x0046_49f8);
            c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
            c.g[RA][0] = sx(0x001b_a45c);
            Addr(p(this, 0x6d0)).sw(c, c.g[V0][0]);
            call_ptr(c, callee::FE_STRING_HASH, 0x001b_a45c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001b_a470);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001b_a470)?;
            let none = c.g[V0][0] == 0;
            Addr(p(r(c, S1), 0x40)).sw(c, c.g[V0][0]);
            if none {
                break 'outer;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001b_a488);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001b_a488)?;
            break 'outer;
        }
        c.cycles += 22;
        c.g[V0][0] = Addr(p(r(c, S1), 0x6d0)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        Addr(p(r(c, V0), 0x40)).sw(c, 0);
        c.g[RA][0] = sx(0x001b_a49c);
        Addr(p(r(c, S1), 0x6dc)).sw(c, 0);
        callee::CREATE_STEP2_SET_MONTH.call(c, 0x001b_a49c)?;
        c.g[RA][0] = sx(0x001b_a4a4);
        c.g[A0][0] = c.g[S1][0];
        callee::CREATE_STEP2_SET_DAY.call(c, 0x001b_a4a4)?;
        c.g[RA][0] = sx(0x001b_a4ac);
        c.g[A0][0] = c.g[S1][0];
        callee::CREATE_STEP2_SET_YEAR.call(c, 0x001b_a4ac)?;
        c.g[RA][0] = sx(0x001b_a4b4);
        c.g[A0][0] = c.g[S1][0];
        callee::CREATE_STEP2_SET_GENDER.call(c, 0x001b_a4b4)?;
        c.g[RA][0] = sx(0x001b_a4bc);
        c.g[A0][0] = c.g[S1][0];
        callee::CREATE_STEP2_SET_LEGAL.call(c, 0x001b_a4bc)?;
        c.g[RA][0] = sx(0x001b_a4c4);
        c.g[A0][0] = c.g[S1][0];
        callee::NET_FE_STATE_INIT.call(c, 0x001b_a4c4)?;
        c.g[V0][0] = 1;
        frame.restore(c, 16, 0x20);
        Addr(p(r(c, S1), 0x6d4)).sw(c, c.g[V0][0]);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay cache.

/// `int ReplayCache_ReadFrame(ReplayCache* c, void* out)` (0x26d420): pop one
/// recorded frame — a 4-byte word and a 16-byte state block — from the ring at
/// `+0xc`; returns 1 on success and 0 when the cache is empty.
pub fn replay_cache_read_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        'outer: {
            'count: {
                c.g[A2][0] = 8;
                frame.save(c, S0, 0x20);
                frame.save(c, S1, 0x10);
                let cache = r(c, A0);
                c.g[S0][0] = sx(cache);
                c.g[S1][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0026_d448);
                c.g[A1][0] = 0;
                call_ptr(c, 0x003e_6448, 0x0026_d448)?;
                c.g[V1][0] = Addr(p(r(c, S0), 4)).lw(c);
                c.g[V0][0] = Addr(r(c, S0)).lw(c);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(p(r(c, S0), 8)).lw(c);
                    break 'count;
                }
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'outer;
            }
            'head: {
                c.cycles += 2;
                if (c.g[V0][0] as i64) > 0 {
                    c.g[V0][0] = Addr(p(r(c, S0), 4)).lw(c);
                    break 'head;
                }
                c.cycles += 7;
                c.g[V0][0] = Addr(p(r(c, S0), 0xc)).lw(c);
                c.g[V1][0] = sx((r(c, V1) as u32) << 3);
                c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                c.g[A0][0] = Addr(r(c, V1)).lw(c);
                c.g[A0][0] = c.g[A0][0] & 0xfff;
                Addr(p(r(c, S0), 8)).sw(c, c.g[A0][0]);
                c.g[V0][0] = Addr(p(r(c, S0), 4)).lw(c);
            }
            'copy: {
                c.cycles += 11;
                c.g[V1][0] = Addr(p(r(c, S0), 0xc)).lw(c);
                c.g[V0][0] = sx((r(c, V0) as u32) << 3);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                c.ldl(3, p(r(c, V0), 7));
                c.ldr(3, r(c, V0));
                c.sdl(3, p(r(c, S1), 7));
                c.sdr(3, r(c, S1));
                c.g[V1][0] = Addr(p(r(c, S0), 8)).lw(c);
                c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
                let more = (c.g[V1][0] as i64) > 0;
                Addr(p(r(c, S0), 8)).sw(c, c.g[V1][0]);
                if more {
                    break 'copy;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r(c, S0), 4)).lw(c);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
                Addr(p(r(c, S0), 4)).sw(c, c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
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
// PATH platform volume.

/// `int PATH_PlatFadeVolume(int idx, int target, int time, int flags)`
/// (0x2af608): lock the platform path, write the target volume and either
/// fade or set it, then unlock; returns the unlock result.
pub fn path_plat_fade_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = sx(0x004d_3e98);
        frame.save(c, S1, 0x30);
        c.g[A0][0] = sx((r(c, A0) as u32) << 2);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A2][0];
        c.g[S0][0] = Addr(r(c, A0)).lw(c);
        c.g[A3][0] = sx((r(c, A3) as u32) << 24);
        let immediate = c.g[A3][0] != 0;
        c.g[S2][0] = 0;
        'outer: {
            if immediate {
                break 'outer;
            }
            'done: {
                'stream: {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x002a_f650);
                    call_ptr(c, 0x003b_58a0, 0x002a_f650)?;
                    c.g[V0][0] = Addr(p(r(c, S0), 0x64)).lb(c);
                    let plain = c.g[V0][0] == 0;
                    Addr(p(r(c, S0), 8)).sw(c, c.g[S3][0]);
                    if plain {
                        break 'stream;
                    }
                    'fade: {
                        c.cycles += 5;
                        c.g[V0][0] = 0xa;
                        c.g[V1][0] = Addr(p(r(c, S0), 0x6c)).lw(c);
                        c.div(0, 17, 2);
                        let _ = c.g[V0][0] == 0;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = Addr(p(r(c, V1), 4)).lw(c);
                    c.g[A1][0] = c.lo[0];
                    c.g[RA][0] = sx(0x002a_f680);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x003b_8530, 0x002a_f680)?;
                    break 'done;
                }
                c.cycles += 4;
                c.g[A0][0] = Addr(r(c, S0)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x002a_f698);
                c.g[A2][0] = c.g[S3][0];
                call_ptr(c, 0x003b_7eb0, 0x002a_f698)?;
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x002a_f6a0);
            c.g[S2][0] = c.g[V0][0];
            call_ptr(c, 0x003b_58d8, 0x002a_f6a0)?;
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Media control manager state transitions.

/// `void MCManager_State12_Save(MCManager* m)` (0x23fbb8): drop the pending
/// action count, ask the device whether it is busy and otherwise mark the
/// save finished.
pub fn mc_manager_state12_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = Addr(p(this, 0x108)).lw(c);
        c.g[A1][0] = Addr(p(this, 0x434)).lw(c);
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
        Addr(p(this, 0x108)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(r(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r(c, V1), 0x58)).lh(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0x5c)).lw(c);
        let target = r(c, V0);
        c.g[RA][0] = sx(0x0023_fbec);
        c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
        call_ptr(c, target, 0x0023_fbec)?;
        'outer: {
            if c.g[V0][0] != 0 {
                frame.restore(c, 16, 0x10);
                break 'outer;
            }
            'busy: {
                c.cycles += 8;
                c.g[A1][0] = Addr(p(r(c, S0), 0x434)).lw(c);
                c.g[V1][0] = Addr(r(c, A1)).lw(c);
                c.g[A0][0] = Addr(p(r(c, V1), 0x248)).lh(c);
                c.g[V0][0] = Addr(p(r(c, V1), 0x24c)).lw(c);
                let target = r(c, V0);
                c.g[RA][0] = sx(0x0023_fc0c);
                c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x0023_fc0c)?;
                let idle = c.g[V0][0] == 0;
                c.g[A1][0] = 0x14;
                if idle {
                    break 'busy;
                }
                c.cycles += 7;
                c.g[V0][0] = Addr(p(r(c, S0), 0x748)).lw(c);
                c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                let target = r(c, V1);
                c.g[RA][0] = sx(0x0023_fc28);
                c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
                call_ptr(c, target, 0x0023_fc28)?;
                frame.restore(c, 16, 0x10);
                break 'outer;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r(c, S0), 0x108)).lw(c);
            if (c.g[V0][0] as i64) > 0 {
                frame.restore(c, 16, 0x10);
                break 'outer;
            }
            c.cycles += 10;
            c.g[V0][0] = Addr(p(r(c, S0), 0xc)).lw(c);
            let target = r(c, V0);
            c.g[RA][0] = sx(0x0023_fc48);
            call_ptr(c, target, 0x0023_fc48)?;
            c.g[V1][0] = Addr(p(r(c, S0), 0x748)).lw(c);
            c.g[A1][0] = 0x13;
            c.g[A0][0] = Addr(p(r(c, V1), 8)).lh(c);
            c.g[V0][0] = Addr(p(r(c, V1), 0xc)).lw(c);
            let target = r(c, V0);
            c.g[RA][0] = sx(0x0023_fc60);
            c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
            call_ptr(c, target, 0x0023_fc60)?;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State21_Load(MCManager* m)` (0x2405d0): the load-side twin
/// of [`mc_manager_state12_save`].
pub fn mc_manager_state21_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = Addr(p(this, 0x108)).lw(c);
        c.g[A1][0] = Addr(p(this, 0x434)).lw(c);
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
        Addr(p(this, 0x108)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(r(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r(c, V1), 0x58)).lh(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0x5c)).lw(c);
        let target = r(c, V0);
        c.g[RA][0] = sx(0x0024_0604);
        c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
        call_ptr(c, target, 0x0024_0604)?;
        'outer: {
            if c.g[V0][0] != 0 {
                frame.restore(c, 16, 0x10);
                break 'outer;
            }
            'busy: {
                c.cycles += 8;
                c.g[A1][0] = Addr(p(r(c, S0), 0x434)).lw(c);
                c.g[V1][0] = Addr(r(c, A1)).lw(c);
                c.g[A0][0] = Addr(p(r(c, V1), 0x248)).lh(c);
                c.g[V0][0] = Addr(p(r(c, V1), 0x24c)).lw(c);
                let target = r(c, V0);
                c.g[RA][0] = sx(0x0024_0624);
                c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x0024_0624)?;
                let idle = c.g[V0][0] == 0;
                c.g[A1][0] = 0x24;
                if idle {
                    break 'busy;
                }
                c.cycles += 10;
                c.g[V0][0] = Addr(p(r(c, S0), 0x748)).lw(c);
                c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                let target = r(c, V1);
                c.g[RA][0] = sx(0x0024_0640);
                c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
                call_ptr(c, target, 0x0024_0640)?;
                c.g[V0][0] = Addr(p(r(c, S0), 0x2c)).lw(c);
                let target = r(c, V0);
                c.g[RA][0] = sx(0x0024_064c);
                call_ptr(c, target, 0x0024_064c)?;
                frame.restore(c, 16, 0x10);
                break 'outer;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r(c, S0), 0x108)).lw(c);
            if (c.g[V0][0] as i64) > 0 {
                frame.restore(c, 16, 0x10);
                break 'outer;
            }
            c.cycles += 7;
            c.g[V0][0] = Addr(p(r(c, S0), 0x748)).lw(c);
            c.g[A1][0] = 0x22;
            c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
            let target = r(c, V1);
            c.g[RA][0] = sx(0x0024_0678);
            c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
            call_ptr(c, target, 0x0024_0678)?;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
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
// FE overlay: rival pre-screen.

/// `void OVScreenRivalPre_Init(OVScreenRivalPre* s)` (0x1fd268): create the
/// overlay's two screens, play them, initialise the overlay base and start the
/// rival-entry speech.
pub fn ov_screen_rival_pre_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x0046_ff58);
        c.g[RA][0] = sx(0x001f_d28c);
        c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
        call_ptr(c, callee::FE_STRING_HASH, 0x001f_d28c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001f_d2a0);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_d2a0)?;
        let none = c.g[V0][0] == 0;
        Addr(p(r(c, S1), 0x40)).sw(c, c.g[V0][0]);
        if !none {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_d2b8);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_d2b8)?;
        }
        'second: {
            c.cycles += 11;
            c.g[A0][0] = sx(0x0047_0000);
            c.g[S0][0] = Addr(p(r(c, S1), 0x10)).lw(c);
            c.g[RA][0] = sx(0x001f_d2c8);
            c.g[A0][0] = sx(0x0046_e378);
            call_ptr(c, callee::FE_STRING_HASH, 0x001f_d2c8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001f_d2dc);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001f_d2dc)?;
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if none {
                break 'second;
            }
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_d2f0);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_d2f0)?;
        }
        c.cycles += 12;
        c.g[RA][0] = sx(0x001f_d2f8);
        c.g[A0][0] = c.g[S1][0];
        callee::OV_STATE_OVERLAY_INIT.call(c, 0x001f_d2f8)?;
        c.g[RA][0] = sx(0x001f_d300);
        call_ptr(c, 0x0028_b180, 0x001f_d300)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001f_d30c);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x0029_4f78, 0x001f_d30c)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider FX.

/// `void RiderBase_ResetFX(RiderBase* r)` (0x111890): reset every FX
/// sub-object hanging off the rider, then notify the game module that the
/// rider's FX were reset.
pub fn rider_base_reset_fx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        // The FX sub-objects, in order.
        for (off, target, ret) in [
            (0x3b0, 0x002d_cf28, 0x0011_18a8),
            (0x470, 0x002d_aa78, 0x0011_18b0),
            (0x520, 0x002e_8560, 0x0011_18b8),
            (0x610, 0x002e_6640, 0x0011_18c0),
            (0x9c0, 0x002e_adc0, 0x0011_18c8),
            (0xad0, 0x002e_f6a0, 0x0011_18d0),
            (0xaf0, 0x002d_4be0, 0x0011_18d8),
            (0xb00, 0x002e_3930, 0x0011_18e0),
            (0xb40, 0x002d_f3b0, 0x0011_18e8),
            (0xc70, 0x002f_1148, 0x0011_18f0),
            (0xd20, 0x002f_64e8, 0x0011_18f8),
        ] {
            c.g[RA][0] = sx(ret);
            c.g[A0][0] = sx(p(r(c, S0), off));
            call_ptr(c, target, ret)?;
        }
        c.g[RA][0] = sx(0x0011_1900);
        call_ptr(c, 0x002c_03e8, 0x0011_1900)?;
        'outer: {
            'notify: {
                c.g[V0][0] = Addr(p(r(c, S0), 0x18)).lw(c);
                c.g[A1][0] = Addr(p(r(c, V0), 0x870)).lw(c);
                let skip = (c.g[A1][0] as i64) < 0;
                c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
                if skip {
                    break 'notify;
                }
                c.cycles += 5;
                c.g[RA][0] = sx(0x0011_1918);
                c.g[A0][0] = Addr(p(r(c, V0), 0x84)).lw(c);
                call_ptr(c, 0x0023_06a8, 0x0011_1918)?;
                c.g[A1][0] = c.g[V0][0];
                let none = c.g[A1][0] == 0;
                frame.restore(c, 16, 0x10);
                if none {
                    break 'outer;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r(c, A1), 0xc)).lw(c);
                c.g[A0][0] = Addr(p(r(c, V0), 0x70)).lh(c);
                c.g[V1][0] = Addr(p(r(c, V0), 0x74)).lw(c);
                let target = r(c, V1);
                c.g[RA][0] = sx(0x0011_1938);
                c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x0011_1938)?;
                break 'notify;
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
// Input preprocessor.

/// `void InputPreproc_SkipLineComment(InputPreproc* p, Cursor* cur, void* a,
/// void* b)` (0x3265c0): copy the rest of a `//` comment line to the output,
/// stopping at the newline (which is written) or at end of input.
pub fn input_preproc_skip_line_comment(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        let pre = r(c, A0);
        c.g[S0][0] = sx(pre);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0xa;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0];
        c.g[V0][0] = Addr(p(pre, 0xc)).lw(c);
        'outer_scan: loop {
            'body: {
                'newline: {
                    c.cycles += 2;
                    let is_nl = c.g[V0][0] != c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    if is_nl {
                        break 'newline;
                    }
                    c.cycles += 10;
                    c.g[V0][0] = Addr(p(r(c, S1), 0xc)).lw(c);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
                    Addr(p(r(c, S1), 0xc)).sw(c, c.g[V0][0]);
                    c.g[RA][0] = sx(0x0032_6614);
                    c.g[A1][0] = Addr(p(r(c, S0), 0xc)).lb(c);
                    callee::INPUT_PREPROC_WRITE.call(c, 0x0032_6614)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0032_6620);
                    c.g[A1][0] = c.g[S1][0];
                    callee::INPUT_PREPROC_NEXT_CHAR.call(c, 0x0032_6620)?;
                    frame.restore(c, 16, 0x50);
                    break 'body;
                }
                'eof: {
                    c.cycles += 2;
                    let at_end = (c.g[V0][0] as i64) < 0;
                    c.g[T0][0] = sx(0x0049_0000);
                    if at_end {
                        break 'eof;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0032_6638);
                    c.g[A1][0] = c.g[S1][0];
                    callee::INPUT_PREPROC_NEXT_CHAR.call(c, 0x0032_6638)?;
                    c.g[V0][0] = Addr(p(r(c, S0), 0xc)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_65f8);
                    }
                    continue 'outer_scan;
                }
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x0032_6658);
                c.g[T0][0] = sx(r(c, T0).wrapping_sub(0x1c28));
                callee::INPUT_PREPROC_EMPTY_STUB.call(c, 0x0032_6658)?;
                frame.restore(c, 16, 0x50);
                break 'body;
            }
            c.cycles += 7;
            frame.restore(c, 17, 0x40);
            frame.restore(c, 18, 0x30);
            frame.restore(c, 19, 0x20);
            frame.restore(c, 20, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

// ---------------------------------------------------------------------------
// cBXString.

/// `char* cBXString_GetBuffer(cBXString* s, int min_len)` (0x3189d0): return a
/// writable buffer of at least `min_len` bytes, copying a shared buffer and
/// growing a short one first.
pub fn cbx_string_get_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        'outer: {
            'shared: {
                frame.save(c, S1, 0x30);
                frame.save(c, S2, 0x20);
                let s = r(c, A0);
                c.g[S1][0] = sx(s);
                frame.save(c, S0, 0x40);
                frame.save_ra(c, 0x10);
                c.g[V1][0] = Addr(s).lw(c);
                c.g[V0][0] = Addr(p(r(c, V1), (-0xc_i32) as u32)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
                let shared = c.g[V0][0] == 0;
                c.g[S2][0] = c.g[A1][0];
                if shared {
                    break 'shared;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr(p(r(c, V1), (-4i32) as u32)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(r(c, S1)).lw(c);
                    break 'outer;
                }
            }
            c.cycles += 25;
            c.g[A0][0] = sx(r(c, SP));
            c.g[RA][0] = sx(0x0031_8a18);
            c.g[A1][0] = c.g[S1][0];
            callee::BXSTRING_CTOR_COPY.call(c, 0x0031_8a18)?;
            c.g[V1][0] = Addr(r(c, SP)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0031_8a28);
            c.g[S0][0] = Addr(p(r(c, V1), (-8i32) as u32)).lw(c);
            callee::BXSTRING_RELEASE.call(c, 0x0031_8a28)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            c.g[A1][0] = c.g[S2][0];
            if c.g[V0][0] == 0 {
                c.g[A1][0] = c.g[S0][0];
            }
            c.g[RA][0] = sx(0x0031_8a3c);
            c.g[A0][0] = c.g[S1][0];
            callee::BXSTRING_ALLOC_BUFFER.call(c, 0x0031_8a3c)?;
            c.g[A0][0] = Addr(r(c, S1)).lw(c);
            c.g[A2][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0031_8a4c);
            c.g[A1][0] = Addr(r(c, SP)).lw(c);
            call_ptr(c, 0x0041_605c, 0x0031_8a4c)?;
            c.g[V1][0] = Addr(r(c, S1)).lw(c);
            c.g[A0][0] = sx(r(c, SP));
            c.g[A1][0] = 0x2;
            Addr(p(r(c, V1), (-8i32) as u32)).sw(c, c.g[S0][0]);
            c.g[V0][0] = Addr(r(c, S1)).lw(c);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S0)));
            c.g[RA][0] = sx(0x0031_8a6c);
            Addr(r(c, V0)).sb(c, 0);
            callee::BXSTRING_DTOR.call(c, 0x0031_8a6c)?;
            c.g[V0][0] = Addr(r(c, S1)).lw(c);
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// VU0 microcode upload.

/// `void Vu0_UploadMicrocodeA(void)` (0x3912a8): wait for the VU0 micro
/// program to stop, install the microcode entry point at `0x43bd00`, start it
/// and flush the instruction cache.
pub fn vu0_upload_microcode_a(c: &mut Cpu) {
    guest(c, |c| {
        'first: {
            c.cycles += 8;
            c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
            c.g[V0][0] = sx(0x1000_8000);
            c.sd(r(c, SP), c.g[RA][0]);
            c.g[V1][0] = Addr(r(c, V0)).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x100;
            let idle = c.g[V1][0] == 0;
            c.g[V1][0] = sx(0x1000_0000);
            if idle {
                break 'first;
            }
            c.cycles += 2;
            c.g[V1][0] = sx(0x1000_8000);
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0;
                if !busy {
                    break 'first;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_12d0);
                }
            }
        }
        'second: {
            c.cycles += 16;
            c.g[A0][0] = sx(0x1000_8030);
            c.g[V0][0] = sx(0x0043_bd00);
            c.g[V1][0] = sx(0x1000_0000);
            Addr(r(c, A0)).sw(c, c.g[V0][0]);
            c.g[V1][0] = sx(0x1000_8020);
            c.g[A0][0] = sx(0x1000_0000);
            Addr(r(c, V1)).sw(c, 0);
            c.g[A0][0] = sx(0x1000_8000);
            c.g[V1][0] = 0x104;
            Addr(r(c, A0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(r(c, A0)).lw(c);
            c.g[V0][0] = c.g[V0][0] & 0x100;
            let idle = c.g[V0][0] == 0;
            c.g[V1][0] = sx(0x1000_0000);
            if idle {
                break 'second;
            }
            c.cycles += 1;
            c.g[V1][0] = sx(0x1000_8000);
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0;
                if !busy {
                    break 'second;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_1330);
                }
            }
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0039_1354);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0042_4020, 0x0039_1354)?;
        c.g[RA][0] = c.ld(r(c, SP));
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `void Vu0_UploadMicrocodeB(void)` (0x391360): the second half of the VU0
/// upload — install the entry point at `0x43cca0` and start the program.
pub fn vu0_upload_microcode_b(c: &mut Cpu) {
    guest(c, |c| {
        'first: {
            c.cycles += 8;
            c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
            c.g[V0][0] = sx(0x1000_8000);
            c.sd(r(c, SP), c.g[RA][0]);
            c.g[V1][0] = Addr(r(c, V0)).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x100;
            let idle = c.g[V1][0] == 0;
            c.g[V1][0] = sx(0x1000_0000);
            if idle {
                break 'first;
            }
            c.cycles += 2;
            c.g[V1][0] = sx(0x1000_8000);
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0;
                if !busy {
                    break 'first;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_1388);
                }
            }
        }
        'second: {
            c.cycles += 16;
            c.g[A0][0] = sx(0x1000_8030);
            c.g[V0][0] = sx(0x0043_cca0);
            c.g[V1][0] = sx(0x1000_0000);
            Addr(r(c, A0)).sw(c, c.g[V0][0]);
            c.g[V1][0] = sx(0x1000_8020);
            c.g[A0][0] = sx(0x1000_0000);
            Addr(r(c, V1)).sw(c, 0);
            c.g[A0][0] = sx(0x1000_8000);
            c.g[V1][0] = 0x104;
            Addr(r(c, A0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(r(c, A0)).lw(c);
            c.g[V0][0] = c.g[V0][0] & 0x100;
            let idle = c.g[V0][0] == 0;
            c.g[V1][0] = sx(0x1000_0000);
            if idle {
                break 'second;
            }
            c.cycles += 1;
            c.g[V1][0] = sx(0x1000_8000);
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r(c, V1)).lw(c);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0;
                if !busy {
                    break 'second;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_13e8);
                }
            }
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0039_140c);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0042_4020, 0x0039_140c)?;
        c.g[RA][0] = c.ld(r(c, SP));
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Script camera binding.

/// `void ScriptCamBinding_Bind(ScriptCamBinding* b, void* channel)` (0x2830c8):
/// bind the script camera to the channel and copy the camera flags appropriate
/// to its parameter type (`+0x21` for manual/target/subject, `+0x1c` for an
/// algorithmic camera).
pub fn script_cam_binding_bind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let channel = r(c, A1);
        c.g[S0][0] = sx(channel);
        let this = r(c, A0);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(this);
        callee::SCRIPT_BINDING_BIND.call(c, 0x0028_30e4)?;
        c.g[RA][0] = sx(0x0028_30ec);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0027_43c8, 0x0028_30ec)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        let manual = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        'outer: {
            'store: {
                'alg: {
                    'subj: {
                        'target: {
                            if manual {
                                break 'target;
                            }
                            'zero: {
                                'cmp: {
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0 {
                                        c.g[V0][0] = 2;
                                        break 'cmp;
                                    }
                                    c.cycles += 2;
                                    let is_zero = c.g[V1][0] == 0;
                                    if is_zero {
                                        break 'zero;
                                    }
                                    c.cycles += 2;
                                    frame.restore(c, 16, 0x20);
                                    break 'outer;
                                }
                                c.cycles += 2;
                                let is_two = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 3;
                                if is_two {
                                    break 'subj;
                                }
                                c.cycles += 2;
                                let is_three = c.g[V1][0] == c.g[V0][0];
                                if is_three {
                                    break 'alg;
                                }
                                c.cycles += 2;
                                frame.restore(c, 16, 0x20);
                                break 'outer;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0028_3134);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0027_c098, 0x0028_3134)?;
                            c.g[V1][0] = Addr(p(r(c, V0), 0x21)).lb(c);
                            break 'store;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0028_3144);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0027_c2a8, 0x0028_3144)?;
                        c.g[V1][0] = Addr(p(r(c, V0), 0x21)).lb(c);
                        break 'store;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0028_3154);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0027_c4b8, 0x0028_3154)?;
                    c.g[V1][0] = Addr(p(r(c, V0), 0x21)).lb(c);
                    break 'store;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x0028_3164);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0027_c6a0, 0x0028_3164)?;
                c.g[V1][0] = Addr(p(r(c, V0), 0x1c)).lb(c);
                break 'store;
            }
            c.cycles += 2;
            Addr(p(r(c, S1), 0xc)).sw(c, c.g[V1][0]);
            frame.restore(c, 16, 0x20);
            break 'outer;
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World trigger manager.

/// `cWorldTriggerManager* cWorldTriggerManager_ctor(cWorldTriggerManager* m)`
/// (0x2b58d0): allocate and construct the trigger instance array (0x28
/// entries), publish it and load the trigger definitions.
pub fn world_trigger_manager_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S1, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x50);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[A1][0] = sx(0x0048_35a0);
        frame.save(c, S4, 0x10);
        c.g[A0][0] = 0x1190;
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        Addr(p(this, 0x324)).sw(c, 0);
        c.g[A3][0] = 0;
        c.g[S2][0] = 0x27;
        Addr(p(this, 0x330)).sw(c, 0);
        Addr(p(this, 0x32c)).sw(c, c.g[V1][0]);
        c.g[S4][0] = u64::MAX;
        Addr(p(this, 0x328)).sw(c, c.g[V1][0]);
        c.g[RA][0] = sx(0x002b_592c);
        Addr(p(this, 0x334)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x002b_592c)?;
        c.g[V1][0] = 0x28;
        c.g[S3][0] = sx(r(c, V0).wrapping_add(0x10));
        Addr(r(c, V0)).sw(c, c.g[V1][0]);
        c.g[S0][0] = c.g[S3][0];
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002b_594c);
            c.g[S2][0] = sx(r(c, S2).wrapping_sub(1));
            callee::WORLD_TRIGGER_INSTANCE_CTOR.call(c, 0x002b_594c)?;
            let more = c.g[S2][0] != c.g[S4][0];
            c.g[S0][0] = sx(r(c, S0).wrapping_add(0x70));
            if !more {
                c.cycles += 13;
                Addr(r(c, S1)).sw(c, c.g[S3][0]);
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x002b_5964);
                Addr(p(r(c, S1), 0x338)).sw(c, 0);
                callee::WORLD_TRIGGER_MANAGER_LOAD.call(c, 0x002b_5964)?;
                c.g[V0][0] = c.g[S1][0];
                frame.restore(c, 16, 0x50);
                frame.restore(c, 17, 0x40);
                frame.restore(c, 18, 0x30);
                frame.restore(c, 19, 0x20);
                frame.restore(c, 20, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_5940);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Camera trigger action reader.

/// `int CameraTrigger_ReadActionType1(CameraTrigger*, ActionBoundedCam*)`
/// (0x1714e8): read the six serialized fields of a bounded-camera action and
/// return their sum.
pub fn camera_trigger_read_action_type1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        let out = r(c, A1);
        c.g[S1][0] = sx(out);
        frame.save(c, S0, 0x30);
        let this = r(c, A0);
        c.g[S2][0] = sx(this);
        frame.save_ra(c, 0);
        Addr(out).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(p(out, 0xc));
        c.g[RA][0] = sx(0x0017_1514);
        call_ptr(c, 0x002d_2510, 0x0017_1514)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p(r(c, S1), 0x10));
        c.g[RA][0] = sx(0x0017_1524);
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x002d_2510, 0x0017_1524)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p(r(c, S1), 0x14));
        c.g[RA][0] = sx(0x0017_1534);
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        call_ptr(c, 0x002d_2510, 0x0017_1534)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p(r(c, S1), 0x18));
        c.g[RA][0] = sx(0x0017_1544);
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        call_ptr(c, 0x002d_2510, 0x0017_1544)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p(r(c, S1), 0x1c));
        c.g[RA][0] = sx(0x0017_1554);
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        call_ptr(c, 0x002d_2510, 0x0017_1554)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p(r(c, S1), 0x20));
        c.g[RA][0] = sx(0x0017_1564);
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        call_ptr(c, 0x002d_2510, 0x0017_1564)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p(r(c, S1), 0x24));
        c.g[RA][0] = sx(0x0017_1574);
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        call_ptr(c, 0x002d_2558, 0x0017_1574)?;
        c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0017_1584);
        c.g[A1][0] = sx(p(r(c, S1), 0x28));
        call_ptr(c, 0x002d_25a0, 0x0017_1584)?;
        c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
        frame.restore(c, 17, 0x20);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Buddy manager.

/// `int BuddyMan_HasChallenger(BuddyMan* m, const char* name)` (0x263128):
/// walk the challenger list comparing names; returns 1 and bumps the entry's
/// flag on a match, 0 at the end of the list.
pub fn buddy_man_has_challenger(c: &mut Cpu) {
    // Dispatcher-shaped: an event at a back edge can unwind and the trampoline
    // resumes at the loop head, so `pc` selects the stage and the loop state
    // lives in s0-s3 exactly as the generated code keeps it.
    const ENTRY: u32 = 0x0026_3128;
    const MATCHED: u32 = 0x0026_3164;
    const NEXT: u32 = 0x0026_316c;
    const CHECK: u32 = 0x0026_3188;
    const ADVANCE: u32 = 0x0026_31a8;
    const DONE: u32 = 0x0026_31c4;
    guest(c, |c| {
        // The frame lives on the guest stack, so a resumed call finds it at
        // the same sp-relative offsets; only `sp` itself is set in ENTRY.
        let frame = Frame { size: 0x60 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 13;
                    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x60));
                    frame.save(c, S2, 0x30);
                    frame.save(c, S3, 0x20);
                    frame.save(c, S0, 0x50);
                    let this = r(c, A0);
                    c.g[S3][0] = sx(this);
                    frame.save(c, S1, 0x40);
                    frame.save_ra(c, 0x10);
                    c.g[V1][0] = Addr(p(this, 0xc0)).lw(c);
                    c.g[S0][0] = Addr(p(this, 0xbc)).lw(c);
                    c.g[V0][0] = sx(r(c, V1).wrapping_sub(r(c, S0)));
                    c.g[V0][0] = sx((r(c, V0) as i32 >> 2) as u32);
                    let empty = c.g[V0][0] == 0;
                    c.g[S2][0] = c.g[A1][0];
                    if empty {
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        pc = DONE;
                    } else {
                        pc = 0x0026_3174;
                    }
                    continue;
                }
                0x0026_3174 => {
                    c.cycles += 3;
                    c.g[S1][0] = c.g[S0][0];
                    let at_end = c.g[S1][0] == c.g[V1][0];
                    c.g[V0][0] = 0;
                    if at_end {
                        pc = DONE;
                    } else {
                        pc = 0x0026_3180;
                    }
                    continue;
                }
                0x0026_3180 => {
                    c.cycles += 2;
                    c.g[S0][0] = Addr(r(c, S1)).lw(c);
                    pc = CHECK;
                    continue;
                }
                MATCHED => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(NEXT);
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_8260, NEXT)?;
                    c.g[V0][0] = 1;
                    pc = DONE;
                    continue;
                }
                CHECK => {
                    c.cycles += 8;
                    c.g[A0][0] = sx(r(c, SP));
                    c.g[RA][0] = sx(0x0026_3194);
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0031_82d8, 0x0026_3194)?;
                    c.g[A0][0] = Addr(r(c, S0)).lw(c);
                    c.g[RA][0] = sx(0x0026_31a0);
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0041_65a8, 0x0026_31a0)?;
                    let same = c.g[V0][0] == 0;
                    c.g[A0][0] = sx(r(c, SP));
                    if same {
                        if event_due(c) {
                            if crate::events::deliver_event(c, MATCHED) {
                                return Ok(());
                            }
                        }
                        pc = MATCHED;
                        continue;
                    }
                    pc = ADVANCE;
                    continue;
                }
                ADVANCE => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x0026_31b0);
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_8260, 0x0026_31b0)?;
                    c.g[S1][0] = sx(r(c, S1).wrapping_add(4));
                    c.g[V0][0] = Addr(p(r(c, S3), 0xc0)).lw(c);
                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[S0][0] = Addr(r(c, S1)).lw(c);
                        pc = CHECK;
                        if event_due(c) {
                            if crate::events::deliver_event(c, CHECK) {
                                return Ok(());
                            }
                        }
                        continue;
                    }
                    pc = 0x0026_31c0;
                    continue;
                }
                0x0026_31c0 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    pc = DONE;
                    continue;
                }
                DONE => {
                    c.cycles += 7;
                    frame.restore(c, 16, 0x50);
                    frame.restore(c, 17, 0x40);
                    frame.restore(c, 18, 0x30);
                    frame.restore(c, 19, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENTRY, 0x0026_31e0);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// DJ speech wrappers.

/// The shared body of the six `SSXAudioSystem_DJSayXXXX` entry points: when DJ
/// speech is on, play the speech event with the current language mask and
/// track/category, then hand it to the speech system.
fn dj_say(c: &mut Cpu, base: u32, event: u64) -> Flow {
    let r1 = base.wrapping_add(0x18);
    let r2 = base.wrapping_add(0x40);
    let r3 = base.wrapping_add(0x50);
    let r4 = base.wrapping_add(0x58);
    let r5 = base.wrapping_add(0x64);
    let r6 = base.wrapping_add(0x6c);
    let r7 = base.wrapping_add(0x78);
    let r8 = base.wrapping_add(0x8c);
    let r9 = base.wrapping_add(0xa4);
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    'outer: {
        'skip: {
            frame.save(c, S0, 0x20);
            frame.save(c, S1, 0x10);
            frame.save_ra(c, 0);
            let this = r(c, A0);
            c.g[S0][0] = sx(this);
            c.g[RA][0] = sx(r1);
            callee::IS_DJ_SPEECH_ON.call(c, r1)?;
            let off = c.g[V0][0] == 0;
            c.g[A0][0] = sx(p(this, 0x5560));
            if off {
                break 'skip;
            }
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = event;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[RA][0] = sx(r2);
            c.g[T2][0] = 0;
            callee::SET_EVENT_CONTEXT.call(c, r2)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, 16, 0x20);
                break 'outer;
            }
            c.cycles += 23;
            c.g[RA][0] = sx(r3);
            c.g[A0][0] = c.g[S0][0];
            callee::GET_LANGUAGE_MASK.call(c, r3)?;
            c.g[RA][0] = sx(r4);
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, callee::AUDIO_GET_GAME_MODULE, r4)?;
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(r5);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_GET_INTERFACE, r5)?;
            c.g[RA][0] = sx(r6);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_GET_SOMETHING, r6)?;
            c.g[A1][0] = Addr(r(c, V0)).lw(c);
            c.g[RA][0] = sx(r7);
            c.g[A0][0] = c.g[S0][0];
            callee::TRACK_TO_CATEGORY.call(c, r7)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 1;
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(r8);
            c.g[A2][0] = event;
            call_ptr(c, callee::SPEECH_NEW, r8)?;
            c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            let target = r(c, V1);
            c.g[RA][0] = sx(r9);
            c.g[A1][0] = 2;
            call_ptr(c, target, r9)?;
            break 'skip;
        }
        c.cycles += 1;
        frame.restore(c, 16, 0x20);
    }
    c.cycles += 4;
    frame_restore_s1(c);
    Ok(())
}

/// The `lq s1; ld ra; addiu sp; jr ra` tail of a 0x30-byte frame.
#[inline(always)]
fn frame_restore_s1(c: &mut Cpu) {
    c.lq(17, p(r(c, SP), 0x10));
    c.g[RA][0] = c.ld(r(c, SP));
    let ra = r(c, RA);
    c.g[SP][0] = sx(r(c, SP).wrapping_add(0x30));
    c.pc = ra;
}

/// `void SSXAudioSystem_DJSay20C8(void* self)` (0x2a2018).
pub fn dj_say_20c8(c: &mut Cpu) {
    guest(c, |c| dj_say(c, 0x002a_2018, 0x20c8))
}

/// `void SSXAudioSystem_DJSay20C9(void* self)` (0x2a20d0).
pub fn dj_say_20c9(c: &mut Cpu) {
    guest(c, |c| dj_say(c, 0x002a_20d0, 0x20c9))
}

/// `void SSXAudioSystem_DJSay20E7(void* self)` (0x2a2260).
pub fn dj_say_20e7(c: &mut Cpu) {
    guest(c, |c| dj_say(c, 0x002a_2260, 0x20e7))
}

/// `void SSXAudioSystem_DJSay20E8(void* self)` (0x2a2318).
pub fn dj_say_20e8(c: &mut Cpu) {
    guest(c, |c| dj_say(c, 0x002a_2318, 0x20e8))
}

/// `void SSXAudioSystem_DJSay20BA(void* self)` (0x2a2638).
pub fn dj_say_20ba(c: &mut Cpu) {
    guest(c, |c| dj_say(c, 0x002a_2638, 0x20ba))
}

/// `void SSXAudioSystem_DJSay212B(void* self)` (0x2a2ad0).
pub fn dj_say_212b(c: &mut Cpu) {
    guest(c, |c| dj_say(c, 0x002a_2ad0, 0x212b))
}

// ---------------------------------------------------------------------------
// Message center.

/// `void MessageCenter_SetPeakForeshadowDone(int peak)` (0x1e2370): when the
/// current peak is in the first ten, mark the peak's foreshadow pair of
/// message flags as done (flags 0x1c/0x1d, 0x1e/0x1f or 0x20/0x21).
pub fn message_center_set_peak_foreshadow_done(c: &mut Cpu) {
    guest(c, |c| {
        'preamble: {
            c.cycles += 9;
            c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            c.sd(r(c, SP), c.g[RA][0]);
            c.g[A1][0] = c.g[A0][0];
            c.g[V1][0] = Addr(p(r(c, V0), 0x84)).lw(c);
            c.g[A0][0] = Addr(p(r(c, V1), 0x28)).lw(c);
            c.g[A0][0] = Addr(r(c, A0)).lw(c);
            let none = c.g[A0][0] == 0;
            c.g[A2][0] = 0;
            if none {
                break 'preamble;
            }
            c.cycles += 1;
            c.g[A2][0] = ((c.g[A0][0] as i64) < 10) as u64;
        }
        'done: {
            c.cycles += 2;
            let skip = c.g[A2][0] != 0;
            c.g[RA][0] = c.ld(r(c, SP));
            if skip {
                break 'done;
            }
            'tail: {
                'pairs: {
                    'two: {
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        let is_one = c.g[A1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
                        if is_one {
                            break 'two;
                        }
                        'zero: {
                            'cmp: {
                                c.cycles += 2;
                                if c.g[V0][0] == 0 {
                                    c.g[V0][0] = 2;
                                    break 'cmp;
                                }
                                c.cycles += 2;
                                let is_zero = c.g[A1][0] == 0;
                                c.g[A0][0] = 0x1c;
                                if is_zero {
                                    break 'zero;
                                }
                                c.cycles += 2;
                                break 'done;
                            }
                            c.cycles += 2;
                            let is_two = c.g[A1][0] == c.g[V0][0];
                            c.g[RA][0] = c.ld(r(c, SP));
                            if is_two {
                                break 'tail;
                            }
                            c.cycles += 2;
                            break 'done;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e_23dc);
                        c.g[A1][0] = 1;
                        callee::MESSAGE_CENTER_SET_FLAG.call(c, 0x001e_23dc)?;
                        c.g[A0][0] = 0x1d;
                        break 'pairs;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = 0x1e;
                    c.g[RA][0] = sx(0x001e_23f0);
                    c.g[A1][0] = 1;
                    callee::MESSAGE_CENTER_SET_FLAG.call(c, 0x001e_23f0)?;
                    c.g[A0][0] = 0x1f;
                    break 'pairs;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x001e_23fc);
                c.g[A1][0] = 1;
                callee::MESSAGE_CENTER_SET_FLAG.call(c, 0x001e_23fc)?;
                c.g[RA][0] = c.ld(r(c, SP));
                break 'done;
            }
            c.cycles += 7;
            c.g[A0][0] = 0x20;
            c.g[RA][0] = sx(0x001e_2410);
            c.g[A1][0] = 1;
            callee::MESSAGE_CENTER_SET_FLAG.call(c, 0x001e_2410)?;
            c.g[A0][0] = 0x21;
            c.g[RA][0] = sx(0x001e_241c);
            c.g[A1][0] = 1;
            callee::MESSAGE_CENTER_SET_FLAG.call(c, 0x001e_241c)?;
            c.g[RA][0] = c.ld(r(c, SP));
        }
        c.cycles += 2;
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use super::ee;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, check_lohi, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 120;
    /// The zeroed BELibrary dummy and the world singleton.
    const BE: u32 = DATA + 0x1_0000;
    const W: u32 = DATA + 0x1_4000;
    /// The dlmalloc arena.
    const MEM_HEAP: u32 = 0x004f_f250;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    fn wb(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }
    /// `{ short delta; void (*fn)(); }` at `vt + off`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, target: u32) {
        c.bus.write16(vt + off, 0);
        w(c, vt + off + 4, target);
    }

    /// EAFile's request and file-handle pools (see oc117): with no free entry
    /// the allocators spin forever (`0x3de768`).
    fn eafile_pools(c: &mut Cpu) {
        for (count, table, at, size) in [
            (0x0051_9ab0, 0x0051_9ad8, 0xf_0000, 0x30),
            (0x0051_9ab4, 0x0051_9adc, 0xf_1000, 0x110),
        ] {
            let base = DATA + at;
            w(c, count, 4);
            w(c, table, base);
            for i in 0..size {
                w(c, base + 4 * i, 0);
            }
        }
    }

    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = 0x0012_3450;
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

    /// A real heap block of `size` bytes.
    fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0, 0],
        ) as u32
    }

    /// Scrambled registers, a game-like `gp`, the BE singleton and a world
    /// with no riders.
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

    /// [`base`] plus the pool allocator and dlmalloc arena.
    fn alloc_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = base(c, seed, salt, calls);
        eafile_pools(c);
        pool_allocator(c);
        heap_init(c);
        r
    }

    // ------------------------------------------------------- world / triggers

    #[test]
    fn world_ctor_matches() {
        check(ee::s13::f_003a6688__generated, world_ctor, 60, &|c, seed| {
            alloc_base(c, seed, 0x3a66, 5);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn c_board_trail_fx_render_matches() {
        check(ee::s09::f_002ea480__generated, c_board_trail_fx_render, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x2ea4, 0);
            w(c, GAME_GP + 0x1428, 0);
            w(c, GAME_GP + 0x142c, 0);
            let holder = DATA + 0x2_000;
            w(c, GAME_GP.wrapping_sub(0x854), holder);
            w(c, holder + 0xe84, DATA + 0x900);
            for i in 0..0x30u32 {
                w(c, DATA + 0x900 + 4 * i, r.next_u32());
            }
            w(c, DATA + 0x90, seed % 4);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn world_trigger_manager_ctor_matches() {
        check(ee::s08::f_002b58d0__generated, world_trigger_manager_ctor, 30, &|c, seed| {
            alloc_base(c, seed, 0x2b58, 5);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn fe_state_reward_gallery_init_matches() {
        check(ee::s04::f_001cf0b0__generated, fe_state_reward_gallery_init, 40, &|c, seed| {
            alloc_base(c, seed, 0x1cf0, 4);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x10, DATA + 0x400);
            w(c, DATA + 0x48, (seed % 4) << 3);
        });
    }

    #[test]
    fn ov_screen_rival_pre_init_matches() {
        check(ee::s05::f_001fd268__generated, ov_screen_rival_pre_init, 40, &|c, seed| {
            alloc_base(c, seed, 0x1fd2, 5);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x10, DATA + 0x400);
            w(c, DATA + 0x400, DATA + 0x500);
        });
    }

    #[test]
    fn fe_state_lodge_rider_detail_init_matches() {
        check(ee::s04::f_001f4050__generated, fe_state_lodge_rider_detail_init, 40, &|c, seed| {
            alloc_base(c, seed, 0x1f40, 5);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x10, DATA + 0x400);
            wb(c, DATA + 0x44, (seed % 4) as u8);
        });
    }

    #[test]
    fn fe_state_create_step2_init_matches() {
        check(ee::s03::f_001ba428__generated, fe_state_create_step2_init, 40, &|c, seed| {
            alloc_base(c, seed, 0x1ba4, 5);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x10, DATA + 0x400);
            w(c, DATA + 0x6d0, DATA + 0x500);
        });
    }

    #[test]
    fn fe_state_challenge_popup_ctor_matches() {
        check(ee::s03::f_001addd0__generated, fe_state_challenge_popup_ctor, 40, &|c, seed| {
            alloc_base(c, seed, 0x1add, 5);
            c.g[A0][0] = DATA as u64;
        });
    }

    #[test]
    fn fe_state_char_select_ctor_matches() {
        check(ee::s02::f_00181090__generated, fe_state_char_select_ctor, 60, &|c, seed| {
            base(c, seed, 0x1810, 1);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = (seed % 4) as u64;
            w(c, DATA + 0x10, DATA + 0x400);
            w(c, DATA + 0x400, DATA + 0x500);
        });
    }

    #[test]
    fn camera_trigger_man_load_file_matches() {
        check(ee::s02::f_0016c170__generated, camera_trigger_man_load_file, 30, &|c, seed| {
            alloc_base(c, seed, 0x16c1, 4);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            for (i, b) in b"triggers.bin\0".iter().enumerate() {
                wb(c, DATA + 0x800 + i as u32, *b);
            }
        });
    }

    #[test]
    fn camera_trigger_read_action_type1_matches() {
        check(ee::s02::f_001714e8__generated, camera_trigger_read_action_type1, 30, &|c, seed| {
            let mut r = base(c, seed, 0x1714, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            for i in 0..0x30u32 {
                w(c, DATA + 0x800 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn camera_sphere_trigger_get_bounds_matches() {
        check(ee::s02::f_001721c0__generated, camera_sphere_trigger_get_bounds, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x1721, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            for i in 0..8u32 {
                w(c, DATA + 4 * i, r.f32(100.0));
            }
            w(c, DATA + 0x18, r.f32(100.0));
            w(c, DATA + 0x1c, r.f32(100.0));
            w(c, DATA + 0x20, r.f32(100.0));
        });
    }

    #[test]
    fn camera_box_trigger_get_bounds_matches() {
        check(ee::s02::f_00172840__generated, camera_box_trigger_get_bounds, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x1728, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            for i in 0..8u32 {
                w(c, DATA + 4 * i, r.f32(100.0));
            }
            w(c, DATA + 0x18, r.f32(100.0));
            w(c, DATA + 0x1c, r.f32(100.0));
            w(c, DATA + 0x20, r.f32(100.0));
        });
    }

    // ------------------------------------------------------------- momentum

    #[test]
    fn roller_modifier_update_momentum_matches() {
        check(ee::s11::f_0035d288__generated, roller_modifier_update_momentum, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x35d2, 0);
            c.g[A0][0] = DATA as u64;
            for off in (0x20..0x74).step_by(4) {
                w(c, DATA + off, r.edge_f32());
            }
        });
    }

    // ---------------------------------------------------------- buddy list

    #[test]
    fn buddy_man_has_challenger_matches() {
        check(ee::s07::f_00263128__generated, buddy_man_has_challenger, 60, &|c, seed| {
            let mut r = base(c, seed, 0x2631, 3);
            c.g[A0][0] = DATA as u64;
            c.pc = 0x0026_3128;
            // Two challengers; the second one's name matches.
            w(c, DATA + 0xbc, DATA + 0x200);
            w(c, DATA + 0xc0, DATA + 0x208);
            w(c, DATA + 0x200, DATA + 0x300);
            w(c, DATA + 0x204, DATA + 0x320);
            for (i, b) in b"alice\0".iter().enumerate() {
                wb(c, DATA + 0x300 + i as u32, *b);
            }
            for (i, b) in b"bob\0".iter().enumerate() {
                wb(c, DATA + 0x320 + i as u32, *b);
            }
            let want: &[u8] = match seed % 3 {
                0 => b"bob\0",
                1 => b"alice\0",
                _ => b"carol\0",
            };
            for (i, b) in want.iter().enumerate() {
                wb(c, DATA + 0x900 + i as u32, *b);
            }
            let _ = r.next_u32();
            c.g[A1][0] = (DATA + 0x900) as u64;
        });
    }

    // ------------------------------------------------------------ audio DJ

    fn dj_setup(c: &mut Cpu, seed: u32, salt: u64) {
        base(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
    }

    #[test]
    fn dj_say_20c8_matches() {
        check(ee::s08::f_002a2018__generated, dj_say_20c8, 30, &|c, seed| {
            dj_setup(c, seed, 0x2a20)
        });
    }

    #[test]
    fn dj_say_20c9_matches() {
        check(ee::s08::f_002a20d0__generated, dj_say_20c9, 30, &|c, seed| {
            dj_setup(c, seed, 0x2a21)
        });
    }

    #[test]
    fn dj_say_20e7_matches() {
        check(ee::s08::f_002a2260__generated, dj_say_20e7, 30, &|c, seed| {
            dj_setup(c, seed, 0x2a22)
        });
    }

    #[test]
    fn dj_say_20e8_matches() {
        check(ee::s08::f_002a2318__generated, dj_say_20e8, 30, &|c, seed| {
            dj_setup(c, seed, 0x2a23)
        });
    }

    #[test]
    fn dj_say_20ba_matches() {
        check(ee::s08::f_002a2638__generated, dj_say_20ba, 30, &|c, seed| {
            dj_setup(c, seed, 0x2a26)
        });
    }

    #[test]
    fn dj_say_212b_matches() {
        check(ee::s08::f_002a2ad0__generated, dj_say_212b, 30, &|c, seed| {
            dj_setup(c, seed, 0x2a2a)
        });
    }

    // -------------------------------------------------------- PATH platform

        #[test]
    fn path_plat_fade_volume_matches() {
        check_lohi(ee::s08::f_002af608__generated, path_plat_fade_volume, 40, &|c, seed| {
            let mut r = base(c, seed, 0x2af6, 0);
            let idx = seed % 3;
            let obj = DATA + 0x1_000;
            w(c, 0x004d_3e98 + 4 * idx, obj);
            w(c, obj, DATA + 0x900);
            w(c, obj + 8, r.next_u32());
            w(c, obj + 0x6c, r.next_u32());
            wb(c, obj + 0x64, (seed % 3) as u8);
            c.g[A0][0] = idx as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = if seed % 4 == 0 { 1 } else { 0 };
        });
    }

    // ------------------------------------------------------ song allocation

        // ---------------------------------------------------- media control mgr

    fn mc_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = alloc_base(c, seed, salt, 4);
        let holder = DATA + 0x2_000;
        let vt = DATA + 0x2_100;
        w(c, holder, vt);
        slot(c, vt, 0x58, FAKE);
        slot(c, vt, 0x5c, FAKE);
        w(c, DATA + 0x434, holder);
        let mgr = DATA + 0x2_200;
        let mvt = DATA + 0x2_300;
        w(c, mgr, 0);
        w(c, DATA + 0x748, mvt);
        slot(c, mvt, 8, FAKE);
        slot(c, mvt, 0xc, FAKE);
        w(c, DATA + 0x108, seed % 3);
        let _ = r.next_u32();
        c.g[A0][0] = DATA as u64;
    }

    #[test]
    fn mc_manager_state12_save_matches() {
        check(ee::s06::f_0023fbb8__generated, mc_manager_state12_save, 60, &|c, seed| {
            mc_setup(c, seed, 0x23fb)
        });
    }

    #[test]
    fn mc_manager_state21_load_matches() {
        check(ee::s06::f_002405d0__generated, mc_manager_state21_load, 60, &|c, seed| {
            mc_setup(c, seed, 0x2405)
        });
    }

    // ------------------------------------------------------- replay cache

    #[test]
    fn replay_cache_read_frame_matches() {
        check(ee::s07::f_0026d420__generated, replay_cache_read_frame, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x26d4, 0);
            let ring = DATA + 0x1_000;
            for i in 0..0x20u32 {
                w(c, ring + 4 * i, r.next_u32());
            }
            w(c, DATA, r.next_u32() % 4);
            w(c, DATA + 4, r.next_u32() % 4);
            w(c, DATA + 8, r.next_u32() % 4);
            w(c, DATA + 0xc, ring);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
        });
    }

    // --------------------------------------------------------- message center

    #[test]
    fn message_center_set_peak_foreshadow_done_matches() {
        check(
            ee::s04::f_001e2370__generated,
            message_center_set_peak_foreshadow_done,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x1e23, 0);
                let game = W + 0x1000;
                w(c, game + 0x28, DATA + 0x300);
                w(c, DATA + 0x300, (seed % 13) as u32);
                c.g[A0][0] = (seed % 4) as u64;
            },
        );
    }

    // ------------------------------------------------------------ FE states

    #[test]
    fn fe_state_bragging_rights_start_wait_anim_matches() {
        check(
            ee::s02::f_00194080__generated,
            fe_state_bragging_rights_start_wait_anim,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x1940, 2);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x6f8, seed & 1);
                w(c, DATA + 0x72c, (seed >> 1) & 1);
                w(c, DATA + 0x40, 0);
            },
        );
    }

    #[test]
    fn fe_locale_v04_matches() {
        check(ee::s02::f_00195eb8__generated, fe_locale_v04, 40, &|c, seed| {
            let mut r = base(c, seed, 0x195e, 0);
            let this = DATA;
            wb(c, this, (seed % 4 == 0) as u8);
            wb(c, this + 1, 0x41);
            let lang = r.next_u32();
            for i in 0..20u32 {
                let entry = DATA + 0x400 + 4 * i;
                w(c, this + 8 + 4 * i, entry);
                if i == 0 && seed % 3 == 1 {
                    wb(c, entry, 0x41);
                } else {
                    w(c, entry, 0);
                }
            }
            w(c, this + 0x5a, 0);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = lang as u64;
        });
    }

    #[test]
    fn fe_state_options_hud_on_child_event_matches() {
        check(
            ee::s02::f_0018c1b8__generated,
            fe_state_options_hud_on_child_event,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x18c1, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 0x15,
                    1 => 0x16,
                    _ => 0x17,
                };
                w(c, DATA + 0x800 + 0xc, r.next_u32());
                w(c, DATA + 0x800 + 8, DATA + 0x900);
                w(c, DATA + 0x900 + 0xb8, 0);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn fe_use_existing_account_v25_matches() {
        check(
            ee::s03::f_001ac4f0__generated,
            fe_use_existing_account_v25,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x1ac4, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
            },
        );
    }

    #[test]
    fn fe_state_online_character_select_v25_matches() {
        check(
            ee::s03::f_001b14d8__generated,
            fe_state_online_character_select_v25,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x1b14, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
            },
        );
    }

    #[test]
    fn fe_state_net_news_v25_matches() {
        check(ee::s03::f_001c5540__generated, fe_state_net_news_v25, SEEDS, &|c, seed| {
            base(c, seed, 0x1c55, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
        });
    }

    #[test]
    fn fe_state_chat_room_on_keyboard_done_matches() {
        check(
            ee::s03::f_001b33d8__generated,
            fe_state_chat_room_on_keyboard_done,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x1b33, 2);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x69c, DATA + 0x800);
                w(c, DATA + 0x800 + 0x18, (seed % 5) as u32);
                // The buddy-manager teardown paths need a live manager whose
                // lists are intact; keep them on the empty path.
                w(c, GAME_GP + 0x238, 0);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn fe_state_chat_room_select_player_by_name_matches() {
        check(
            ee::s03::f_001b3978__generated,
            fe_state_chat_room_select_player_by_name,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x1b39, 2);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x900) as u64;
                w(c, DATA + 0x40, if seed % 3 == 0 { 0 } else { DATA + 0x800 });
                w(c, GAME_GP.wrapping_sub(0xc8), if seed % 2 == 0 { 0 } else { DATA + 0xa00 });
                w(c, DATA + 0xa00, 0);
                let _ = r.next_u32();
            },
        );
    }

        #[test]
    fn fe_state_reward_gallery_on_child_event_matches() {
        check_lohi(
            ee::s04::f_001cfd18__generated,
            fe_state_reward_gallery_on_child_event,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x1cfd, 0);
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

    // -------------------------------------------------------- algorithm list

    #[test]
    fn algorithm_list_remove_matches() {
        check(ee::s01::f_0015ca50__generated, algorithm_list_remove, SEEDS, &|c, seed| {
            let mut r = alloc_base(c, seed, 0x15ca, 3);
            let list = DATA;
            let node = DATA + 0x100;
            w(c, list + 4, r.next_u32() % 4);
            w(c, node + 0x10, if seed % 2 == 0 { 0 } else { DATA + 0x200 });
            w(c, node + 0x14, if seed & 2 == 0 { 0 } else { DATA + 0x300 });
            w(c, node, if seed & 4 == 0 { 0 } else { DATA + 0x400 });
            w(c, node + 0x18, 0);
            let vt = DATA + 0x500;
            w(c, DATA + 0x400, vt);
            slot(c, vt, 0x10, FAKE);
            slot(c, vt, 0x14, FAKE);
            c.g[A0][0] = list as u64;
            c.g[A1][0] = node as u64;
        });
    }

    // ------------------------------------------------------ world painter

        // --------------------------------------------------------- process list

    #[test]
    fn process_list_delete_section_objects_matches() {
        check(
            ee::s11::f_003551a8__generated,
            process_list_delete_section_objects,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x3551, 2);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 3) as u64;
                w(c, DATA, 0);
            },
        );
    }

    // ------------------------------------------------------ rider FX reset

    #[test]
    fn rider_base_reset_fx_matches() {
        check(ee::s00::f_00111890__generated, rider_base_reset_fx, 40, &|c, seed| {
            alloc_base(c, seed, 0x1118, 3);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x18, DATA + 0x400);
            w(c, DATA + 0x400 + 0x870, if seed % 2 == 0 { 0 } else { 5 });
        });
    }

    // ---------------------------------------------------------- script cam

    #[test]
    fn script_cam_binding_bind_matches() {
        check(ee::s07::f_002830c8__generated, script_cam_binding_bind, 30, &|c, seed| {
            let mut r = base(c, seed, 0x2830, 2);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            for i in 0..0x40u32 {
                w(c, DATA + 0x800 + 4 * i, r.next_u32());
            }
        });
    }

    // ------------------------------------------------------ input preproc

    #[test]
    fn input_preproc_skip_line_comment_matches() {
        check(
            ee::s10::f_003265c0__generated,
            input_preproc_skip_line_comment,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x3265, 0);
                let pre = DATA;
                let cur = DATA + 0x100;
                if seed % 3 == 0 {
                    // End of input: the reader reports -1.
                    w(c, pre + 0xc, 0);
                    w(c, cur, DATA + 0x400);
                    w(c, cur + 4, 0);
                    w(c, cur + 8, DATA + 0x400);
                } else {
                    // Newline: the comment is written and the cursor moves.
                    w(c, pre + 0xc, 0xa);
                    w(c, cur, DATA + 0x400);
                    w(c, cur + 4, 4);
                    w(c, cur + 8, DATA + 0x400);
                    wb(c, DATA + 0x400, 0xa);
                }
                w(c, pre, r.next_u32());
                c.g[A0][0] = pre as u64;
                c.g[A1][0] = cur as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    // ---------------------------------------------------------- cBXString

    #[test]
    fn cbx_string_get_buffer_matches() {
        check(ee::s10::f_003189d0__generated, cbx_string_get_buffer, 40, &|c, seed| {
            alloc_base(c, seed, 0x3189, 4);
            let s = DATA;
            let buf = DATA + 0x1_000;
            for i in 0..0x40u32 {
                w(c, buf + 4 * i, 0x41);
            }
            w(c, s, buf);
            // 3 = shared buffer (copy path), 1 = unique with room (fast path).
            let shared = seed % 2 == 0;
            w(c, buf.wrapping_sub(0xc), if shared { 3 } else { 1 });
            w(c, buf.wrapping_sub(4), 0x100);
            w(c, buf.wrapping_sub(8), 4);
            c.g[A0][0] = s as u64;
            c.g[A1][0] = (if shared { 8 } else { 2 }) as u64;
        });
    }

    // ------------------------------------------------------------ VU0

    #[test]
    fn vu0_upload_microcode_a_matches() {
        check(ee::s12::f_003912a8__generated, vu0_upload_microcode_a, 20, &|c, seed| {
            base(c, seed, 0x3912, 1);
            w(c, 0x1000_8000, 0);
            w(c, 0x1000_8020, 0);
            w(c, 0x1000_8030, 0);
        });
    }

    #[test]
    fn vu0_upload_microcode_b_matches() {
        check(ee::s12::f_00391360__generated, vu0_upload_microcode_b, 20, &|c, seed| {
            base(c, seed, 0x3913, 1);
            w(c, 0x1000_8000, 0);
            w(c, 0x1000_8020, 0);
            w(c, 0x1000_8030, 0);
        });
    }

    // ------------------------------------------------------ cSetupMan queue

    #[test]
    fn setup_man_client_recv_msg5_matches() {
        check(ee::s07::f_00269188__generated, setup_man_client_recv_msg5, 40, &|c, seed| {
            let mut r = alloc_base(c, seed, 0x2691, 3);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64;
            w(c, GAME_GP.wrapping_sub(0xc8), if seed % 2 == 0 { 0 } else { DATA + 0x800 });
        });
    }

    #[test]
    fn setup_man_client_recv_msg4_matches() {
        check(ee::s07::f_002693c0__generated, setup_man_client_recv_msg4, 40, &|c, seed| {
            let mut r = alloc_base(c, seed, 0x2693, 3);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64;
            w(c, GAME_GP.wrapping_sub(0xc8), if seed % 2 == 0 { 0 } else { DATA + 0x800 });
        });
    }

    #[test]
    fn setup_man_client_recv_msg7_matches() {
        check(ee::s07::f_00269890__generated, setup_man_client_recv_msg7, 40, &|c, seed| {
            let mut r = alloc_base(c, seed, 0x2698, 3);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64;
            w(c, GAME_GP.wrapping_sub(0xc8), if seed % 2 == 0 { 0 } else { DATA + 0x800 });
        });
    }
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
                        let frame = Frame::open(c, 0x10);
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

/// `void cFEStateOptionsHUD_OnChildEvent(cFEStateOptionsHUD* s, void* child,
/// int event)` (0x18c1b8): apply the option row's change when the HUD's child
/// screen reports the selection event.
pub fn fe_state_options_hud_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        'outer: {
            'done: {
                frame.save(c, S0, 0x40);
                frame.save(c, S1, 0x30);
                let event = r(c, A2);
                c.g[S0][0] = sx(event);
                frame.save(c, S2, 0x20);
                c.g[S1][0] = c.g[A1][0];
                let this = r(c, A0);
                frame.save(c, S3, 0x10);
                frame.save_ra(c, 0);
                c.g[S3][0] = sx(this);
                call_ptr(c, 0x0014_dc80, 0x0018_c1e0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0018_c1ec);
                c.g[A1][0] = 4;
                call_ptr(c, callee::BE_GET_INTERFACE, 0x0018_c1ec)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = 0x15;
                let open = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0x16;
                if open {
                    break 'done;
                }
                c.cycles += 2;
                let not_close = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, 16, 0x40);
                if not_close {
                    break 'outer;
                }
                c.cycles += 6;
                c.g[A0][0] = sx(0x0046_0000);
                c.g[S0][0] = Addr(p(r(c, S1), 0xc)).lw(c);
                c.g[RA][0] = sx(0x0018_c214);
                c.g[A0][0] = sx(0x0045_dd20);
                call_ptr(c, callee::FE_STRING_HASH, 0x0018_c214)?;
                let differs = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, 16, 0x40);
                if differs {
                    break 'outer;
                }
                c.cycles += 8;
                c.g[V1][0] = Addr(p(r(c, S1), 8)).lw(c);
                c.g[A1][0] = 2;
                c.g[A0][0] = Addr(p(r(c, V1), 0xb8)).lh(c);
                c.g[V0][0] = Addr(p(r(c, V1), 0xbc)).lw(c);
                let target = r(c, V0);
                c.g[RA][0] = sx(0x0018_c234);
                c.g[A0][0] = sx(p(r(c, S1), r(c, A0)));
                call_ptr(c, target, 0x0018_c234)?;
                let empty = c.g[V0][0] == 0;
                frame.restore(c, 16, 0x40);
                if empty {
                    break 'outer;
                }
                c.cycles += 6;
                c.g[RA][0] = sx(0x0018_c244);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0014_f600, 0x0018_c244)?;
                c.g[RA][0] = sx(0x0018_c24c);
                c.g[A0][0] = c.g[S3][0];
                callee::OPTIONS_HUD_REFRESH_SELECTION.call(c, 0x0018_c24c)?;
                c.g[V0][0] = 1;
                Addr(gp(c, -0x1c18)).sw(c, c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
