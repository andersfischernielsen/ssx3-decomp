//! Phase 2 wave 44 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for 50 cold-path leaves — camera-algorithm resets, emitter
//! and particle helpers, memory-card / audio / streaming state machines, FE
//! screen helpers and small constructors/destructors — each with a
//! differential test against the generated body.
//!
//! The replacements keep the state in the guest registers the originals use
//! where callers, callees or event handlers can see them, follow the call
//! protocol of [`crate::call`] and the event rules of [`crate::events`].

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow};
use crate::events::{deliver_event, deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0016_8f60,
        mid_left_chase_camera_reset_for_rider,
        "MidLeftChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0037_1600,
        memitter_reset_particles,
        "mEmitter_ResetParticles",
    ),
    (
        0x0037_e098,
        ps2_graphics_man_emit_model_lighting,
        "PS2GraphicsMan_EmitModelLighting",
    ),
    (0x0023_f150, mc_manager_state14, "MCManager_State14"),
    (
        0x001c_aba8,
        c_ui_state_buy_popup_ctor,
        "cUIStateBuyPopup_ctor",
    ),
    (
        0x0017_15f8,
        camera_trigger_read_action_type2,
        "CameraTrigger_ReadActionType2",
    ),
    (
        0x0017_6ce0,
        default_2_chase_camera_reset_for_rider,
        "Default_2ChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0017_6fe0,
        default_3_chase_camera_reset_for_rider,
        "Default_3ChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0017_72e0,
        default_4_chase_camera_reset_for_rider,
        "Default_4ChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0017_7588,
        direction_ne_chase_camera_reset_for_rider,
        "DirectionNeChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0017_8e90,
        post_race_1_chase_camera_reset_for_rider,
        "Post_Race_1ChaseCameraAlgorithm_ResetForRider",
    ),
    (
        0x0030_c3e0,
        wscript_man_collect_set_mark_item,
        "WScriptMan_CollectSetMarkItem",
    ),
    (0x0030_d4b8, rider_model_ctor, "RiderModel_ctor"),(
        0x0012_3da8,
        rider_update_from_script,
        "Rider_UpdateFromScript",
    ),
    (
        0x0028_b930,
        snd_bank_man_init_load_request,
        "SndBankMan_InitLoadRequest",
    ),
    (
        0x0028_f678,
        ssx_audio_system_on_event_select_exit,
        "SSXAudioSystem_OnEventSelectExit",
    ),
    (
        0x001d_93e8,
        mem_card_state_base_play_widget_anim,
        "MemCardStateBase_PlayWidgetAnim",
    ),(0x0010_e830, rider_v15, "Rider_v15"),
    (
        0x0015_a488,
        be_reward_int_new_rewards_ctor,
        "BERewardInt_NewRewardsCtor",
    ),
    (
        0x002d_f448,
        c_dynamic_spray_fx_setup_emitters,
        "cDynamicSprayFX_SetupEmitters",
    ),
    (0x002e_5d18, snow_fall_dtor, "SnowFall_dtor"),
    (
        0x0023_8b70,
        game_mode_man_sort_by_score,
        "GameModeMan_SortByScore",
    ),
    (
        0x0023_8bf8,
        game_mode_man_sort_by_time,
        "GameModeMan_SortByTime",
    ),
    (
        0x0036_9c28,
        ps2_graphics_man_wait_for_render_idle,
        "PS2GraphicsMan_WaitForRenderIdle",
    ),
    (
        0x001d_4890,
        c_fe_state_trophy_room_show_trophy_info,
        "cFEStateTrophyRoom_ShowTrophyInfo",
    ),
    (0x003e_c0f8, tag_field_set_token, "TagFieldSetToken"),
    (
        0x0023_2488,
        pre_start_screen_render,
        "PreStartScreen_Render",
    ),
    (
        0x0023_2650,
        init_load_screen_render,
        "InitLoadScreen_Render",
    ),
    (
        0x0023_2820,
        c_pre_game_load_screen_render,
        "cPreGameLoadScreen_Render",
    ),
    (
        0x0023_28a8,
        c_pre_fe_load_screen_render,
        "cPreFELoadScreen_Render",
    ),
    (
        0x003a_1bf8,
        c_ui_text_scroll_add_line,
        "cUITextScroll_AddLine",
    ),
    (0x0039_86b0, c_ui_group_find_by_id, "cUIGroup_FindById"),
    (
        0x0020_3cb8,
        event_select_set_map_tab_mode,
        "EventSelect_SetMapTabMode",
    ),(
        0x0020_a6f0,
        c_ov_state_overlay_play_object_anim,
        "cOVStateOverlay_PlayObjectAnim",
    ),
    (
        0x0014_fd80,
        bebagt_int_set_uber_trick_button,
        "BEBAGTInt_SetUberTrickButton",
    ),
    (
        0x0022_dff0,
        section_man_finish_event_switch,
        "SectionMan_FinishEventSwitch",
    ),
    (0x003a_6f88, c_stream_man_queue_read, "cStreamMan_QueueRead"),
    (0x0039_7140, ui_anim_set_create_anim, "UIAnimSet_CreateAnim"),
    (
        0x001a_3768,
        fe_async_manager_request_peak_map,
        "FEAsyncManager_RequestPeakMap",
    ),
    (
        0x001a_6618,
        c_fe_state_net_messenger_set_row_name,
        "cFEStateNetMessenger_SetRowName",
    ),
    (0x001a_9bc0, net_fe_state_show_popup, "NetFEState_ShowPopup"),
    (
        0x001b_9cd8,
        c_fe_state_create_step1_is_email_format_valid,
        "cFEStateCreateStep1_IsEmailFormatValid",
    ),
    (
        0x001b_b3b0,
        c_fe_state_create_step2_set_year_selection,
        "cFEStateCreateStep2_SetYearSelection",
    ),
    (0x0036_6fe8, tex_man_ctor, "TexMan_ctor"),
    (0x0025_d770, lobby_man_peek_room, "LobbyMan_PeekRoom"),
    (0x0026_3338, buddy_man_is_online_at, "BuddyMan_IsOnlineAt"),
    (
        0x0026_4cf0,
        c_setup_man_set_local_character,
        "cSetupMan_SetLocalCharacter",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const RESET_FOR_RIDER: Callee = Callee {
        addr: 0x0016_6c60,
        f: ee::s02::f_00166c60,
    };
    pub const INIT_FROM_RIDER: Callee = Callee {
        addr: 0x0016_6550,
        f: ee::s02::f_00166550,
    };
    pub const DYNAMIC_EMITTER_RESET: Callee = Callee {
        addr: 0x0037_0d60,
        f: ee::s12::f_00370d60,
    };
    pub const MC_END_OPERATION: Callee = Callee {
        addr: 0x0023_ea30,
        f: ee::s06::f_0023ea30,
    };
    pub const MC_ON_OPERATION_DONE: Callee = Callee {
        addr: 0x0023_ea90,
        f: ee::s06::f_0023ea90,
    };
    pub const BUY_POPUP_RESET: Callee = Callee {
        addr: 0x001c_a8a0,
        f: ee::s04::f_001ca8a0,
    };
    pub const STOP_LOADING_SCREEN_LOOP: Callee = Callee {
        addr: 0x0028_fa98,
        f: ee::s07::f_0028fa98,
    };
    pub const CLEAR_NEW_REWARDS: Callee = Callee {
        addr: 0x0015_a510,
        f: ee::s01::f_0015a510,
    };
    pub const RIDER_GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::f_0011fe98,
    };
    pub const RIDER_RECORD_BEST_JUMP: Callee = Callee {
        addr: 0x0011_9448,
        f: ee::s00::f_00119448,
    };
    pub const RIDER_ADD_BOOST: Callee = Callee {
        addr: 0x0010_e098,
        f: ee::s00::f_0010e098,
    };
    pub const RIDER_APPLY_SCRIPT_TRACK: Callee = Callee {
        addr: 0x0012_4788,
        f: ee::s00::f_00124788,
    };
    pub const RIDER_UPDATE_SCRIPT_SOUNDS: Callee = Callee {
        addr: 0x0012_5448,
        f: ee::s00::f_00125448,
    };
    pub const SNOW_BOX_DTOR: Callee = Callee {
        addr: 0x002e_4f28,
        f: ee::s09::f_002e4f28,
    };
    pub const UI_ANIM_CTOR: Callee = Callee {
        addr: 0x0039_72a0,
        f: ee::s13::f_003972a0,
    };
    pub const VRAM_HEAP_INIT: Callee = Callee {
        addr: 0x0036_5e40,
        f: ee::s12::f_00365e40,
    };
    pub const TEX_MAN_INIT_TABLE: Callee = Callee {
        addr: 0x0036_70e0,
        f: ee::s12::f_003670e0,
    };
    pub const TAG_FIELD_COPY: Callee = Callee {
        addr: 0x003e_b198,
        f: ee::s14::f_003eb198,
    };
    pub const TAG_FIELD_SET_RAW: Callee = Callee {
        addr: 0x003e_b1e0,
        f: ee::s14::f_003eb1e0,
    };
    pub const GROW_LINES: Callee = Callee {
        addr: 0x003a_1958,
        f: ee::s13::f_003a1958,
    };
    pub const UI_TEXT_SET_EVENT_STAT: Callee = Callee {
        addr: 0x001c_e9e0,
        f: ee::s04::f_001ce9e0,
    };
    pub const UI_STATE_CTOR_2: Callee = Callee {
        addr: 0x0039_e318,
        f: ee::s13::f_0039e318,
    };
    pub const LOAD_INDICATOR_RENDER: Callee = Callee {
        addr: 0x0023_1fc0,
        f: ee::s06::f_00231fc0,
    };
    pub const UI_OBJECT_FIND_BY_ID: Callee = Callee {
        addr: 0x0039_ff60,
        f: ee::s13::f_0039ff60,
    };
    pub const UI_LIST_FIRST: Callee = Callee {
        addr: 0x0039_7840,
        f: ee::s13::f_00397840,
    };
    pub const UI_LIST_NODE_SENTINEL: Callee = Callee {
        addr: 0x0039_7718,
        f: ee::s13::f_00397718,
    };
    pub const SECTION_SELECT_PEAK_LIGHTING: Callee = Callee {
        addr: 0x0022_e180,
        f: ee::s05::f_0022e180,
    };
    pub const INSTANCE_GET_NODE_MATRIX: Callee = Callee {
        addr: 0x0034_fed8,
        f: ee::s11::f_0034fed8,
    };
    pub const AVALANCHE_GET_OBJECT_MATRIX: Callee = Callee {
        addr: 0x002d_9c00,
        f: ee::s09::f_002d9c00,
    };
    pub const GAME_PROCESS_DTOR: Callee = Callee {
        addr: 0x0035_46c8,
        f: ee::s11::f_003546c8,
    };
    pub const WORLD_GET_INSTANCE: Callee = Callee {
        addr: 0x002d_1bd8,
        f: ee::s09::f_002d1bd8,
    };
    pub const BE_GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::f_0014a0e0,
    };
    pub const BE_GET_RIDER_CHARACTER: Callee = Callee {
        addr: 0x0014_a080,
        f: ee::s01::f_0014a080,
    };
    pub const BE_GET_TRACK_INFO: Callee = Callee {
        addr: 0x0014_4bc0,
        f: ee::s01::f_00144bc0,
    };
    pub const BE_SET_CHARACTER_BY_SLOT: Callee = Callee {
        addr: 0x0014_7448,
        f: ee::s01::f_00147448,
    };
    pub const LOBBY_GET_ROOM_NAME: Callee = Callee {
        addr: 0x0025_f150,
        f: ee::s06::f_0025f150,
    };
    pub const BUDDY_GET_ROSTER_COUNT: Callee = Callee {
        addr: 0x0026_2598,
        f: ee::s07::f_00262598,
    };
    pub const BUDDY_GET_ROSTER_ENTRY: Callee = Callee {
        addr: 0x0026_25c8,
        f: ee::s07::f_002625c8,
    };
    pub const FE_ASYNC_SET_STATE: Callee = Callee {
        addr: 0x001a_39d8,
        f: ee::s03::f_001a39d8,
    };
    pub const POPUP_CONFIRM_REBUILD: Callee = Callee {
        addr: 0x001c_5dd8,
        f: ee::s03::f_001c5dd8,
    };
    pub const SSX_AUDIO_PLAY_MUSIC: Callee = Callee {
        addr: 0x0028_cf98,
        f: ee::s07::f_0028cf98,
    };

    // Cross-shard calls the generator emitted through the function table.
    pub const STR_HASH: u32 = 0x0031_7670;
    pub const STREAM_FILE_QUEUE_RANGE: u32 = 0x003e_0c88;
    pub const USTR_LEN: u32 = 0x003d_bca0;
    pub const USTR_CPY: u32 = 0x003d_be20;
    pub const UI_TEXT_SET_TEXT_ID: u32 = 0x003a_1000;
    pub const UI_TEXT_SET_STRING: u32 = 0x003a_0c50;
    pub const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;
    pub const UI_ANIM_SET_CREATE_ANIM: u32 = 0x0039_7140;
    pub const UI_OBJECT_SET_ANIM: u32 = 0x0039_fcc8;
    pub const UI_STATE_MGR_PUSH_POPUP: u32 = 0x0039_f290;
    pub const UI_LISTBOX_SET_ITEM_TEXT: u32 = 0x0039_a7f0;
    pub const SET_ITEM_YEAR: u32 = 0x0039_a7a8;
    pub const UI_TEXT_SET_ASCII: u32 = 0x003a_0c70;
    pub const UI_ANIM_SET_CREATE_ANIM_2: u32 = 0x0039_7168;
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const BE_MISSION_SET_COLLECTED: u32 = 0x0015_3b00;
    pub const MSG_PRINTF: u32 = 0x0041_7828;
    pub const STR_LEN: u32 = 0x0041_6810;
    pub const STRLEN: u32 = 0x0041_6810;
    pub const QSORT: u32 = 0x0041_8ef8;
    pub const EA_TIMER_SERVICE: u32 = 0x003e_5928;
    pub const EA_THREAD_YIELD: u32 = 0x003e_5398;
    pub const GAME_MODULE_REQUEST_STATE: u32 = 0x0023_1250;
    pub const LOBBY_API_SEND: u32 = 0x003e_8b10;
    pub const TAG_FIELD_SET_STRING: u32 = 0x003e_c180;
    pub const SSX_AUDIO_RESUME_SFX: u32 = 0x0029_ce70;
    pub const SSX_AUDIO_IS_TRACK_GROUP5: u32 = 0x002a_3f90;
    pub const RIDER_GATHER_CAM_STATE: u32 = 0x0027_c9b0;
    pub const SSX_AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    pub const MEM_REPORT_STATS_LOCKED: u32 = 0x0031_9cc8;
    pub const OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;
    pub const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    pub const UI_LIST_FIRST_WRAPPER: u32 = 0x0039_fae8;
    pub const SERIALIZE_READ_U32: u32 = 0x002d_2510;
}

// ---------------------------------------------------------------------------
// Address helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// `a + off` sign-extended (`addiu`).
#[inline(always)]
fn ofs(a: u32, off: i32) -> u32 {
    a.wrapping_add(off as u32)
}

/// A `gp`-relative global address.
#[inline(always)]
fn gpa(c: &Cpu, off: i32) -> u32 {
    ofs(c.g[GP][0] as u32, off)
}

/// The current stack pointer.
#[inline(always)]
fn spv(c: &Cpu) -> u32 {
    c.g[SP][0] as u32
}

/// `addiu sp, sp, -n`.
#[inline(always)]
fn open(c: &mut Cpu, n: u32) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(n));
}

/// `lw` of `base + off`.
#[inline(always)]
fn lw_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(at(base as u32, off)).lw(c)
}

/// `lh` of `base + off`.
#[inline(always)]
fn lh_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(at(base as u32, off)).lh(c)
}

/// `lb` of `base + off`.
#[inline(always)]
fn lb_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(at(base as u32, off)).lb(c)
}

/// `lbu` of `base + off`.
#[inline(always)]
fn lbu_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(at(base as u32, off)).lbu(c)
}

/// `lhu` of `base + off`.
#[inline(always)]
fn lhu_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(at(base as u32, off)).lhu(c)
}

/// `sw` at `base + off`.
#[inline(always)]
fn sw_at(c: &mut Cpu, base: u64, off: u32, v: u64) {
    Addr(at(base as u32, off)).sw(c, v);
}

/// `sh` at `base + off`.
#[inline(always)]
fn sh_at(c: &mut Cpu, base: u64, off: u32, v: u64) {
    Addr(at(base as u32, off)).sh(c, v);
}

/// `sb` at `base + off`.
#[inline(always)]
fn sb_at(c: &mut Cpu, base: u64, off: u32, v: u64) {
    Addr(at(base as u32, off)).sb(c, v);
}

/// `lwc1 f, addr`.
#[inline(always)]
fn ldf(c: &mut Cpu, f: usize, addr: u32) -> FpuF32 {
    c.lwc1(f, addr);
    FpuF32(c.f[f])
}

// ---------------------------------------------------------------------------
// Chase-camera algorithm resets.

/// Where a `<X>ChaseCameraAlgorithm_ResetForRider` gets its two distance
/// constants.
#[derive(Clone, Copy)]
enum BackHeight {
    /// `back = *(gp + off)`, `down = 150.0`.
    Gp(i32),
    /// Post-race: `back = down = 100.0`.
    Hundred,
}

/// Shared body of the seven `<X>ChaseCameraAlgorithm_ResetForRider` leaves
/// (0x168f60, 0x176ce0, 0x176fe0, 0x1772e0, 0x177588, 0x178e90): the camera
/// controller at `*(this+0x30)` owns the rider at `+4`; the rider's current
/// algorithm id is asked through the `+0x6c0` vtable slot, and when it is
/// still `rider`, the algorithm is reset for that rider and re-initialised
/// from it (back distance / height), then the target's current-algorithm
/// slot is notified.
fn chase_reset_for_rider(c: &mut Cpu, entry: u32, variant: BackHeight) -> Flow {
    c.cycles += 16;
    open(c, 0x30);
    let sp0 = spv(c);
    c.sq(16, at(sp0, 0x20));
    c.sq(17, at(sp0, 0x10));
    let this = c.g[A0][0];
    c.g[S0][0] = this;
    let rider = c.g[A1][0];
    c.g[S1][0] = rider;
    c.sd(sp0, c.g[RA][0]);

    let target = lw_at(c, this, 0x30);
    c.g[V0][0] = target;
    let obj = lw_at(c, target, 0x4);
    c.g[A0][0] = obj;
    let vt = lw_at(c, obj, 0x6c0);
    c.g[V1][0] = vt;
    c.g[A0][0] = sx(at(obj as u32, 0x6c0));
    c.g[A1][0] = lh_at(c, vt, 0x38);
    c.g[V0][0] = lw_at(c, vt, 0x3c);
    let func = c.g[V0][0] as u32;
    c.g[A0][0] = sx(at(c.g[A0][0] as u32, c.g[A1][0] as u32));
    call_ptr(c, func, entry + 0x38)?;
    if c.g[S1][0] != c.g[V0][0] {
        c.lq(16, at(sp0, 0x20));
    } else {
        c.cycles += 14;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        callee::RESET_FOR_RIDER.call(c, entry + 0x4c)?;
        match variant {
            BackHeight::Gp(off) => {
                let a = gpa(c, off);
                ldf(c, 12, a);
                c.g[AT][0] = 0x4396_0000;
                c.mtc1(1, 13);
            }
            BackHeight::Hundred => {
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 12);
                c.mov_s(13, 12);
            }
        }
        c.g[A0][0] = c.g[S0][0];
        callee::INIT_FROM_RIDER.call(c, entry + 0x60)?;
        let a0 = lw_at(c, c.g[S0][0], 0x10);
        c.g[V0][0] = a0;
        c.g[A0][0] = lh_at(c, a0, 0x28);
        c.g[V1][0] = lw_at(c, a0, 0x2c);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at(c.g[S0][0] as u32, c.g[A0][0] as u32));
        call_ptr(c, func, entry + 0x74)?;
        c.lq(16, at(sp0, 0x20));
    }
    c.cycles += 4;
    c.lq(17, at(sp0, 0x10));
    c.g[RA][0] = c.ld(sp0);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(sp0, 0x30));
    c.pc = ra;
    Ok(())
}

/// `void MidLeftChaseCameraAlgorithm_ResetForRider(algo, int rider)` (0x168f60).
pub fn mid_left_chase_camera_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        chase_reset_for_rider(c, 0x0016_8f60, BackHeight::Gp(-0x654c))
    })
}

/// `void Default_2ChaseCameraAlgorithm_ResetForRider(algo, int rider)` (0x176ce0).
pub fn default_2_chase_camera_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        chase_reset_for_rider(c, 0x0017_6ce0, BackHeight::Gp(-0x6138))
    })
}

/// `void Default_3ChaseCameraAlgorithm_ResetForRider(algo, int rider)` (0x176fe0).
pub fn default_3_chase_camera_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        chase_reset_for_rider(c, 0x0017_6fe0, BackHeight::Gp(-0x607c))
    })
}

/// `void Default_4ChaseCameraAlgorithm_ResetForRider(algo, int rider)` (0x1772e0).
pub fn default_4_chase_camera_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        chase_reset_for_rider(c, 0x0017_72e0, BackHeight::Gp(-0x5fc0))
    })
}

/// `void DirectionNeChaseCameraAlgorithm_ResetForRider(algo, int rider)` (0x177588).
pub fn direction_ne_chase_camera_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        chase_reset_for_rider(c, 0x0017_7588, BackHeight::Gp(-0x5f38))
    })
}

/// `void Post_Race_1ChaseCameraAlgorithm_ResetForRider(algo, int rider)` (0x178e90).
pub fn post_race_1_chase_camera_reset_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        chase_reset_for_rider(c, 0x0017_8e90, BackHeight::Hundred)
    })
}

// ---------------------------------------------------------------------------
// Emitters and graphics.

/// `void mEmitter_ResetParticles(mEmitter* self)` (0x371600): reset the base
/// emitter, then clear the packed colour of every particle (the scratch
/// colour word at `gp+0x28a8` is copied unaligned into each particle slot).
pub fn memitter_reset_particles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x20);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        callee::DYNAMIC_EMITTER_RESET.call(c, 0x0037_1614)?;
        let this = c.g[S0][0];
        c.g[V0][0] = lw_at(c, this, 0x178);
        let count = c.g[V0][0] as i64;
        c.g[A0][0] = 0;
        if count > 0 {
            c.cycles += 2;
            c.g[A1][0] = 1;
            let a = gpa(c, 0x28ac);
            c.g[V0][0] = lw_at(c, a as u64, 0);
            loop {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = lw_at(c, this, 0x204);
                } else {
                    c.cycles += 6;
                    let g = gpa(c, 0x28a8);
                    c.sb(g, 0);
                    c.sb(at(g, 1), 0);
                    c.sb(at(g, 2), 0);
                    c.sb(at(g, 3), 0);
                    let ga = gpa(c, 0x28ac);
                    c.sw(ga, c.g[A1][0]);
                    c.g[V0][0] = lw_at(c, this, 0x204);
                }
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[A0][0] = sx(at(c.g[A0][0] as u32, 1));
                c.g[V1][0] = sx(at(c.g[V1][0] as u32, c.g[V0][0] as u32));
                let t0 = gpa(c, 0x28a8);
                c.g[T0][0] = sx(t0);
                c.lwl(2, at(t0, 3));
                c.lwr(2, t0);
                let dst = c.g[V1][0] as u32;
                c.swl(2, at(dst, 3));
                c.swr(2, dst);
                c.g[V0][0] = lw_at(c, this, 0x178);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                let ga = gpa(c, 0x28ac);
                c.g[V0][0] = lw_at(c, ga as u64, 0);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0037_1628);
                }
            }
        }
        c.cycles += 4;
        c.lq(16, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `void PS2GraphicsMan_EmitModelLighting(void* dst, void* world, u32 model,
/// void** out_dl)` (0x37e098): write a lighting packet (fixed 0x60-byte
/// header, the model's three light quadwords) and append one quadword per
/// light to the display list at `*out_dl`.
pub fn ps2_graphics_man_emit_model_lighting(c: &mut Cpu) {
    c.cycles += 24;
    c.g[T0][0] = lw_at(c, c.g[A3][0], 0);
    c.g[V0][0] = 0x1000_000f;
    c.g[V1][0] = 0x6c03_8009;
    c.g[V1][0] = c.g[V1][0] << 32;
    c.g[A1][0] = 0x6c0a_800f;
    c.g[A1][0] = c.g[A1][0] << 32;
    let dl = c.g[T0][0] as u32;
    c.sd(dl, c.g[V0][0]);
    c.g[T1][0] = 9;
    c.sd(at(dl, 0x18), c.g[V1][0]);
    c.sd(at(dl, 8), 0);
    c.sd(at(dl, 0x10), 0);
    let model = c.g[A2][0] as u32;
    c.lq(2, model);
    c.sq(2, at(dl, 0x20));
    c.lq(3, at(model, 0x10));
    c.sq(3, at(dl, 0x30));
    c.lq(2, at(model, 0x20));
    c.sd(at(dl, 0x58), c.g[A1][0]);
    c.sq(2, at(dl, 0x40));
    c.sd(at(dl, 0x50), 0);
    c.g[T0][0] = sx(at(dl, 0x60));
    c.g[V1][0] = c.g[T0][0];
    loop {
        c.cycles += 7;
        let src = c.g[A0][0] as u32;
        c.lq(2, src);
        c.g[T1][0] = sx(at(c.g[T1][0] as u32, 0xffff_ffff));
        c.g[A0][0] = sx(at(src, 0x10));
        c.sq(2, c.g[V1][0] as u32);
        let more = (c.g[T1][0] as i64) >= 0;
        c.g[V1][0] = sx(at(c.g[V1][0] as u32, 0x10));
        if !more {
            c.cycles += 3;
            c.g[T0][0] = sx(at(c.g[T0][0] as u32, 0xa0));
            let ra = c.g[RA][0] as u32;
            c.sw(c.g[A3][0] as u32, c.g[T0][0]);
            c.pc = ra;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0037_e0f8);
        }
    }
}

// ---------------------------------------------------------------------------
// Memory-card manager states.

/// `void MCManager_State14(MCManager* self)` (0x23f150): poll the card slot
/// (`+0x434`), then either end the pending operation (`+0x6c`) or report the
/// completed one (`+0x84`).
pub fn mc_manager_state14(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        open(c, 0x20);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];
        c.g[A1][0] = lw_at(c, this, 0x434);
        let mgr = c.g[A1][0];
        c.g[V1][0] = lw_at(c, mgr, 0);
        let vt = c.g[V1][0];
        c.g[A0][0] = lh_at(c, vt, 0x58);
        c.g[V0][0] = lw_at(c, vt, 0x5c);
        let func = c.g[V0][0] as u32;
        c.g[A0][0] = sx(at(mgr as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0023_f178)?;
        if c.g[V0][0] != 0 {
            c.lq(16, at(sp0, 0x10));
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw_at(c, this, 0x6c);
            let func = c.g[V0][0] as u32;
            call_ptr(c, func, 0x0023_f18c)?;
            let busy = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            let mut finished = false;
            if busy {
                c.cycles += 4;
                c.g[A1][0] = 0;
                callee::MC_END_OPERATION.call(c, 0x0023_f19c)?;
                c.lq(16, at(sp0, 0x10));
                finished = true;
            }
            if !finished {
                c.cycles += 5;
                c.g[V0][0] = lw_at(c, this, 0x84);
                let func = c.g[V0][0] as u32;
                call_ptr(c, func, 0x0023_f1b0)?;
                let ok = c.g[V0][0] == 0;
                c.g[V0][0] = 1;
                if !ok {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.sw(at(this as u32, 0xbc), c.g[V0][0]);
                    callee::MC_ON_OPERATION_DONE.call(c, 0x0023_f1c4)?;
                    c.sw(at(this as u32, 0x440), 0);
                }
                c.cycles += 1;
                c.lq(16, at(sp0, 0x10));
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x20));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE / UI constructors and helpers.

/// `cUIStateBuyPopup* cUIStateBuyPopup_ctor(self, owner, engine, uchar unk)`
/// (0x1caba8): construct the UI state, install the buy-popup vtable and reset
/// the popup parameters; when an owner is attached, propagate the player
/// index (`owner[0]`, a `FEModule*`) into the state's selection byte.
pub fn c_ui_state_buy_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        open(c, 0x40);
        let sp0 = spv(c);
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 24);
        c.sq(16, at(sp0, 0x30));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, at(sp0, 0x20));
        c.sq(18, at(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[S1][0] = sx((((c.g[A3][0] as u32) as i32) >> 24) as u32);
        callee::UI_STATE_CTOR_2.call(c, 0x001c_abcc)?;
        let this = c.g[S0][0];
        c.g[S2][0] = sx(at(this as u32, 0x48));
        c.g[V1][0] = 0x470000;
        c.g[A0][0] = c.g[S2][0];
        c.g[V1][0] = sx(at(c.g[V1][0] as u32, 0xffff_cb08));
        c.sw(at(this as u32, 8), c.g[V1][0]);
        callee::BUY_POPUP_RESET.call(c, 0x001c_abe4)?;
        c.g[V1][0] = 7;
        c.sb(at(this as u32, 0x44), c.g[S1][0]);
        c.sw(at(this as u32, 0xc), c.g[V1][0]);
        c.g[V0][0] = lw_at(c, this, 0x10);
        let owner = c.g[V0][0];
        c.g[A0][0] = lw_at(c, owner, 0);
        let no_owner = c.g[A0][0] == 0;
        if !no_owner {
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x001a_1cd0, 0x001c_ac08)?;
            c.sb(at(this as u32, 0x15), c.g[V0][0]);
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S2][0];
        callee::BUY_POPUP_RESET.call(c, 0x001c_ac14)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, at(sp0, 0x20));
        c.lq(16, at(sp0, 0x30));
        c.lq(18, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `int CameraTrigger_ReadActionType2(CameraTrigger* self, u32* out)`
/// (0x1715f8): read five serialized action-type words into `out[3..7]`
/// (`out[0] = 2`) and return their sum.
pub fn camera_trigger_read_action_type2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        open(c, 0x40);
        let sp0 = spv(c);
        c.g[V0][0] = 2;
        c.sq(17, at(sp0, 0x20));
        c.sq(18, at(sp0, 0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sq(16, at(sp0, 0x30));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        let out = c.g[S1][0] as u32;
        let reader = c.g[S2][0];
        c.g[A1][0] = sx(at(out, 0xc));
        c.sw(out, c.g[V0][0]);
        call_ptr(c, callee::SERIALIZE_READ_U32, 0x0017_1624)?;
        c.g[A0][0] = reader;
        c.g[A1][0] = sx(at(out, 0x10));
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, callee::SERIALIZE_READ_U32, 0x0017_1634)?;
        c.g[A0][0] = reader;
        c.g[A1][0] = sx(at(out, 0x14));
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V0][0] as u32));
        call_ptr(c, callee::SERIALIZE_READ_U32, 0x0017_1644)?;
        c.g[A0][0] = reader;
        c.g[A1][0] = sx(at(out, 0x18));
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V0][0] as u32));
        call_ptr(c, callee::SERIALIZE_READ_U32, 0x0017_1654)?;
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V0][0] as u32));
        c.g[A0][0] = reader;
        c.g[A1][0] = sx(at(out, 0x1c));
        call_ptr(c, callee::SERIALIZE_READ_U32, 0x0017_1664)?;
        c.g[V0][0] = sx(at(c.g[S0][0] as u32, c.g[V0][0] as u32));
        c.lq(17, at(sp0, 0x20));
        c.lq(16, at(sp0, 0x30));
        c.lq(18, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `int WScriptMan_CollectSetMarkItem(WScriptMan* self, int item)`
/// (0x30c3e0): find `item` in the collect-item table at `+0xc` and, when
/// found, mark it collected through the mission interface.
pub fn wscript_man_collect_set_mark_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.sq(17, at(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        let this = c.g[S1][0];
        c.g[V0][0] = lw_at(c, this, 8);
        let n = c.g[V0][0] as i64;
        c.g[S0][0] = 0;
        let mut found = false;
        if n > 0 {
            c.cycles += 2;
            c.g[V1][0] = sx(at(this as u32, 0xc));
            loop {
                c.cycles += 3;
                let ent = c.g[V1][0] as u32;
                c.g[V0][0] = lw_at(c, ent as u64, 0);
                if c.g[V0][0] != c.g[A1][0] {
                    c.g[V0][0] = lw_at(c, this, 8);
                } else {
                    c.cycles += 12;
                    call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0030_c41c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xa;
                    call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0030_c428)?;
                    let iface = c.g[V0][0];
                    c.g[A3][0] = lw_at(c, this, 0);
                    c.g[A0][0] = iface;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, callee::BE_MISSION_SET_COLLECTED, 0x0030_c43c)?;
                    c.lq(16, at(sp0, 0x20));
                    found = true;
                    break;
                }
                c.cycles += 4;
                c.g[S0][0] = sx(at(c.g[S0][0] as u32, 1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx(at(c.g[V1][0] as u32, 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_c408);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.lq(16, at(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `RiderModel* RiderModel_ctor(RiderModel* self)` (0x30d4b8): clear the
/// render fields and the four model-slot pointers at `+0x3c`, `+0x44`, ...
pub fn rider_model_ctor(c: &mut Cpu) {
    c.cycles += 24;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    open(c, 0x10);
    let sp0 = spv(c);
    let this = c.g[A0][0] as u32;
    c.sw(at(this, 0x38), 0);
    c.g[A1][0] = 3;
    c.swc1(0, at(sp0, 0xc));
    c.g[V1][0] = sx(at(this, 0x5c));
    c.swc1(0, sp0);
    c.swc1(0, at(sp0, 4));
    c.swc1(0, at(sp0, 8));
    c.sw(at(this, 0x30), 0);
    c.lq(2, sp0);
    c.sw(at(this, 0x34), 0);
    c.sq(2, at(this, 0x140));
    c.sw(at(this, 0x2c), 0);
    c.sw(at(this, 0xc), 0);
    c.sw(at(this, 0x1c), 0);
    c.sw(at(this, 0x3c), 0);
    c.sw(at(this, 8), 0);
    c.sw(at(this, 0x12c), 0);
    c.sw(at(this, 0x130), 0);
    c.sw(at(this, 0x24), 0);
    c.sw(at(this, 0x28), 0);
    c.sw(at(this, 0x20), 0);
    loop {
        c.cycles += 7;
        let slot = c.g[V1][0] as u32;
        c.sw(slot, 0);
        c.g[A1][0] = sx(at(c.g[A1][0] as u32, 0xffff_ffff));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx(at(slot, 0xffff_fff8));
        if !more {
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0];
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx(at(sp0, 0x10));
            c.pc = ra;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0030_d518);
        }
    }
}

/// `AvaSplineModifier* AvaSplineModifier_ctor_2(AvaSplineModifier* self,
/// RenderInstance* inst)` (0x357798): install the vtable, take the instance's
/// id and transform through its vtable, and snapshot the avalanche matrix.
pub fn ava_spline_modifier_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        open(c, 0x40);
        let sp0 = spv(c);
        c.g[V0][0] = 0x490000;
        c.sq(17, at(sp0, 0x20));
        c.g[V0][0] = sx(at(c.g[V0][0] as u32, 0xffff_f338));
        c.sq(16, at(sp0, 0x30));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, at(sp0, 0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        let this = c.g[S1][0];
        c.g[S2][0] = sx(at(this as u32, 0x10));
        c.sw(this as u32, c.g[V0][0]);
        let inst = c.g[S0][0];
        c.g[V1][0] = lw_at(c, inst, 0);
        c.g[A0][0] = lh_at(c, c.g[V1][0], 0x18);
        c.g[V0][0] = lw_at(c, c.g[V1][0], 0x1c);
        let func = c.g[V0][0] as u32;
        c.g[A0][0] = sx(at(inst as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0035_77d8)?;
        c.sw(at(this as u32, 0x80), c.g[V0][0]);
        c.g[A2][0] = 0x30;
        c.g[A1][0] = c.g[S2][0];
        c.g[V0][0] = lw_at(c, inst, 0);
        c.g[A0][0] = lh_at(c, c.g[V0][0], 0x10);
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0x14);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at(inst as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0035_77f8)?;
        c.g[A0][0] = lw_at(c, this, 0x80);
        c.g[A1][0] = sx(at(this as u32, 0x40));
        callee::AVALANCHE_GET_OBJECT_MATRIX.call(c, 0x0035_7804)?;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, at(sp0, 0x30));
        c.lq(17, at(sp0, 0x20));
        c.lq(18, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `void Rider_UpdateFromScript(Rider* self, ScriptState* script,
/// float, float, int)` (0x123da8): gather the script camera state, then apply
/// the script track and its sounds.
pub fn rider_update_from_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        open(c, 0x70);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x50));
        c.sq(17, at(sp0, 0x40));
        c.g[S0][0] = c.g[A0][0];
        c.swc1(21, at(sp0, 0x68));
        c.g[A0][0] = c.g[A1][0];
        c.swc1(20, at(sp0, 0x60));
        c.mov_s(21, 13);
        c.sd(at(sp0, 0x30), c.g[RA][0]);
        c.mov_s(20, 12);
        let this = c.g[S0][0];
        c.g[V0][0] = lw_at(c, this, 0xac4);
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A3][0];
        if !none {
            c.cycles += 14;
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, callee::RIDER_GATHER_CAM_STATE, 0x0012_3de4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            c.mov_s(12, 20);
            c.mov_s(13, 21);
            c.g[A2][0] = c.g[S1][0];
            callee::RIDER_APPLY_SCRIPT_TRACK.call(c, 0x0012_3dfc)?;
            c.g[A0][0] = c.g[S0][0];
            c.mov_s(12, 20);
            c.mov_s(13, 21);
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            callee::RIDER_UPDATE_SCRIPT_SOUNDS.call(c, 0x0012_3e14)?;
        }
        c.cycles += 7;
        c.lq(16, at(sp0, 0x50));
        c.lq(17, at(sp0, 0x40));
        c.g[RA][0] = c.ld(at(sp0, 0x30));
        c.lwc1(21, at(sp0, 0x68));
        c.lwc1(20, at(sp0, 0x60));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x70));
        c.pc = ra;
        Ok(())
    })
}

/// `void SndBankMan_InitLoadRequest(SndBankMan* self, u32 bank, u32 async,
/// void* cb)` (0x28b930): ask the bank's slot for the loading screen, mark
/// the request pending and start it.
pub fn snd_bank_man_init_load_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        open(c, 0x50);
        let sp0 = spv(c);
        let g = gpa(c, 0x524);
        c.g[V0][0] = lw_at(c, g as u64, 0);
        c.sq(16, at(sp0, 0x40));
        c.sq(17, at(sp0, 0x30));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, at(sp0, 0x20));
        c.g[S1][0] = c.g[A2][0];
        c.sq(19, at(sp0, 0x10));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S3][0] = c.g[A3][0];
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0x1d8);
        c.g[A1][0] = lw_at(c, c.g[V1][0], 0);
        c.g[V0][0] = lw_at(c, c.g[A1][0], 0);
        c.g[A0][0] = lh_at(c, c.g[V0][0], 0x38);
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0x3c);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at(c.g[A1][0] as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0028_b978)?;
        let pending = c.g[S1][0] != 0;
        let this = c.g[S2][0];
        c.sw(at(this as u32, 4), c.g[S0][0]);
        if !pending {
            c.cycles += 3;
            c.g[V0][0] = 1;
            c.sw(this as u32, c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.sw(this as u32, 0);
        }
        c.cycles += 10;
        c.g[A0][0] = sx(at(this as u32, 0x20));
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0041_66f4, 0x0028_b99c)?;
        c.lq(16, at(sp0, 0x40));
        c.lq(17, at(sp0, 0x30));
        c.lq(18, at(sp0, 0x20));
        c.lq(19, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x50));
        c.pc = ra;
        Ok(())
    })
}

/// `void SSXAudioSystem_OnEventSelectExit(SSXAudioSystem* self, int resume)`
/// (0x28f678): drop the event-select audio mode, stop the loading-screen
/// loop and (when asked) start the event music.
pub fn ssx_audio_system_on_event_select_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.sq(17, at(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        let this = c.g[S0][0];
        c.g[V0][0] = lw_at(c, this, 0x6088);
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 3;
            call_ptr(c, callee::SSX_AUDIO_RESUME_SFX, 0x0028_f6a0)?;
            c.sw(at(this as u32, 0x6088), 0);
        }
        c.cycles += 3;
        c.g[V0][0] = lw_at(c, this, 0x62a8);
        if (c.g[V0][0] as i64) < 0 {
            c.lq(16, at(sp0, 0x20));
        } else {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S0][0];
            callee::STOP_LOADING_SCREEN_LOOP.call(c, 0x0028_f6c0)?;
            let skip = c.g[S1][0] == 0;
            if skip {
                c.lq(16, at(sp0, 0x20));
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::SSX_AUDIO_IS_TRACK_GROUP5, 0x0028_f6d0)?;
                let not_group5 = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !not_group5 {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = u64::MAX;
                    c.g[T0][0] = 0;
                    callee::SSX_AUDIO_PLAY_MUSIC.call(c, 0x0028_f6ec)?;
                }
                c.cycles += 1;
                c.lq(16, at(sp0, 0x20));
            }
        }
        c.cycles += 4;
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void MemCardStateBase_PlayWidgetAnim(MemCardStateBase* self, char* name,
/// char* anim)` (0x1d93e8): find the widget named `name` and play the named
/// animation on it (animations come from the anim set at
/// `self->screen->something + 0x50`).
pub fn mem_card_state_base_play_widget_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        open(c, 0x40);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x30));
        c.sq(18, at(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, at(sp0, 0x20));
        c.g[A0][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, callee::STR_HASH, 0x001d_940c)?;
        let this = c.g[S0][0];
        c.g[A0][0] = lw_at(c, this, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x001d_9418)?;
        c.g[S1][0] = c.g[V0][0];
        let found = c.g[S1][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if found {
            c.cycles += 8;
            c.g[S0][0] = lw_at(c, this, 0x10);
            c.g[S0][0] = sx(at(c.g[S0][0] as u32, 0x50));
            call_ptr(c, callee::STR_HASH, 0x001d_9430)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_ANIM_SET_CREATE_ANIM, 0x001d_943c)?;
            let anim = c.g[V0][0];
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if !none {
                c.cycles += 5;
                c.g[A1][0] = anim;
                c.g[A2][0] = 3;
                c.g[A3][0] = 0;
                c.g[T0][0] = 1;
                call_ptr(c, callee::UI_OBJECT_SET_ANIM, 0x001d_9458)?;
            }
        }
        c.cycles += 6;
        c.lq(16, at(sp0, 0x30));
        c.lq(17, at(sp0, 0x20));
        c.lq(18, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `float ComputerActorRider_GetPathClassWeight(Rider* self, int a1, int a2,
/// int a3)` (0x10d870): map the rider's path class (`s16` at `+0xe00`) and the
/// caller flags to a weight, scaled by the `gp-0x7ce8` factor. Returns 0 for
/// the class/flags combinations that select "no weight".
pub fn computer_actor_rider_get_path_class_weight(c: &mut Cpu) {
    c.cycles += 3;
    let this = c.g[A0][0];
    c.g[V1][0] = lh_at(c, this, 0xe00);
    let kind = c.g[V1][0];
    let (a1, a2, a3) = (c.g[A1][0], c.g[A2][0], c.g[A3][0]);
    c.g[V0][0] = 1;
    let weighted;
    if kind != 0 {
        // class 1 (or anything else): the first dispatch charges its block.
        c.cycles += 2;
        if kind == 1 {
            c.g[V1][0] = 3;
            if a2 != 0 {
                weighted = true;
            } else {
                c.cycles += 2;
                c.cycles += 2;
                weighted = false;
            }
        } else {
            c.g[V0][0] = 2;
            c.cycles += 2;
            if kind != 2 {
                c.g[V0][0] = a1 ^ a2;
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
                c.g[V1][0] = 3;
                weighted = c.g[V0][0] == a3;
            } else {
                c.g[V1][0] = 3;
                if a3 != 0 {
                    weighted = true;
                } else {
                    c.cycles += 2;
                    c.cycles += 2;
                    weighted = false;
                }
            }
        }
    } else {
        c.cycles += 2;
        c.g[V1][0] = 3;
        weighted = a1 != 0;
    }
    if !weighted {
        c.cycles += 3;
        c.mtc1(0, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 11;
    c.g[V0][0] = 4;
    if a1 != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    let a = gpa(c, -0x7ce8);
    ldf(c, 1, a);
    c.g[V1][0] = sx(at(c.g[V0][0] as u32, 0xffff_ffff));
    if a2 != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[A0][0] = sx(at(c.g[V0][0] as u32, 0xffff_ffff));
    if a3 != 0 {
        c.g[V0][0] = c.g[A0][0];
    }
    c.mtc1(2, 0);
    c.cvt_s_w(0, 0);
    c.mul_s(0, 0, 1);
    abi::ret(c);
}

/// `void Rider_v15(Rider* self, float)` (0x10e830): when the rider's main
/// state is 1 or 4, record the best jump and give the +4 boost, then fire the
/// multiplier sound event.
pub fn rider_v15(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x10));
        c.swc1(20, at(sp0, 0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        c.mov_s(20, 12);
        callee::RIDER_GET_MAIN_STATE.call(c, 0x0010_e84c)?;
        c.g[V1][0] = 1;
        let this = c.g[S0][0];
        'body: {
            if c.g[V0][0] != c.g[V1][0] {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::RIDER_GET_MAIN_STATE.call(c, 0x0010_e860)?;
                c.g[V1][0] = 4;
                if c.g[V0][0] != c.g[V1][0] {
                    break 'body;
                }
                c.cycles += 1;
            }
            c.g[A0][0] = lw_at(c, this, 0x790);
            c.cycles += 14;
            c.mov_s(12, 20);
            callee::RIDER_RECORD_BEST_JUMP.call(c, 0x0010_e878)?;
            c.mov_s(12, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 4;
            callee::RIDER_ADD_BOOST.call(c, 0x0010_e888)?;
            call_ptr(c, callee::SSX_AUDIO_GET_INSTANCE, 0x0010_e890)?;
            let audio = c.g[V0][0];
            c.g[A0][0] = audio;
            c.g[A2][0] = c.g[S0][0];
            c.mov_s(12, 20);
            c.g[A1][0] = 2;
            call_ptr(c, 0x0029_ced8, 0x0010_e8a4)?;
        }
        c.lq(16, at(sp0, 0x10));
        c.cycles += 4;
        c.g[RA][0] = c.ld(sp0);
        c.lwc1(20, at(sp0, 0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `BERewardInt* BERewardInt_NewRewardsCtor(BERewardInt* self)` (0x15a488):
/// clear the ten new-reward entries and point the string fields at the
/// in-object buffers.
pub fn be_reward_int_new_rewards_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        open(c, 0x20);
        let sp0 = spv(c);
        c.g[V1][0] = 9;
        c.sq(16, at(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        let this = c.g[S0][0];
        c.g[V0][0] = sx(at(this as u32, 0x198));
        loop {
            c.cycles += 7;
            let slot = c.g[V0][0] as u32;
            c.sw(slot, 0);
            c.g[V1][0] = sx(at(c.g[V1][0] as u32, 0xffff_ffff));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(at(slot, 0xffff_fffc));
            if !more {
                c.cycles += 21;
                c.g[V0][0] = sx(at(this as u32, 0x146));
                c.g[V1][0] = sx(at(this as u32, 0x170));
                c.g[A1][0] = sx(at(this as u32, 0x147));
                c.g[A2][0] = sx(at(this as u32, 0x14d));
                c.g[A3][0] = sx(at(this as u32, 0x15c));
                c.g[T0][0] = sx(at(this as u32, 0x169));
                c.g[T1][0] = sx(at(this as u32, 0x16d));
                c.sw(at(this as u32, 0x174), c.g[V0][0]);
                c.sw(at(this as u32, 0x17c), c.g[V1][0]);
                c.g[A0][0] = c.g[S0][0];
                c.sw(at(this as u32, 0x180), c.g[A1][0]);
                c.sw(at(this as u32, 0x184), c.g[A2][0]);
                c.sw(at(this as u32, 0x188), c.g[A3][0]);
                c.sw(at(this as u32, 0x190), c.g[T0][0]);
                c.sw(at(this as u32, 0x194), c.g[T1][0]);
                callee::CLEAR_NEW_REWARDS.call(c, 0x0015_a4fc)?;
                c.g[V0][0] = c.g[S0][0];
                c.g[RA][0] = c.ld(sp0);
                c.lq(16, at(sp0, 0x10));
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at(sp0, 0x20));
                c.pc = ra;
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_a4a0);
            }
        }
    })
}

/// `void cDynamicSprayFX_SetupEmitters(cDynamicSprayFX* self)` (0x2df448):
/// initialise nine spray emitters from the effect's emitter list.
pub fn c_dynamic_spray_fx_setup_emitters(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        open(c, 0x50);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x40));
        c.sq(17, at(sp0, 0x30));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, at(sp0, 0x20));
        c.g[S1][0] = 9;
        c.sq(19, at(sp0, 0x10));
        c.g[S2][0] = 0;
        c.sd(sp0, c.g[RA][0]);
        c.g[S3][0] = 0;
        let this = c.g[S0][0];
        c.g[A0][0] = lw_at(c, this, 0x28);
        loop {
            c.cycles += 14;
            c.g[S1][0] = sx(at(c.g[S1][0] as u32, 0xffff_ffff));
            c.g[A1][0] = lw_at(c, this, 0x2c);
            c.g[A0][0] = sx(at(c.g[S2][0] as u32, c.g[A0][0] as u32));
            let rec = c.g[A0][0];
            c.g[A2][0] = lw_at(c, rec, 0x1f8);
            c.g[A1][0] = sx(at(c.g[A1][0] as u32, c.g[S3][0] as u32));
            c.lwc1(12, at(rec as u32, 0x58));
            c.g[S3][0] = sx(at(c.g[S3][0] as u32, 0xe8));
            c.g[V1][0] = lh_at(c, c.g[A2][0], 0x18);
            c.g[S2][0] = sx(at(c.g[S2][0] as u32, 0x210));
            c.g[V0][0] = lw_at(c, c.g[A2][0], 0x1c);
            let func = c.g[V0][0] as u32;
            c.g[A0][0] = sx(at(rec as u32, c.g[V1][0] as u32));
            call_ptr(c, func, 0x002d_f4a8)?;
            if (c.g[S1][0] as i64) < 0 {
                c.cycles += 8;
                c.sw(at(this as u32, 0x60), 0);
                c.lq(16, at(sp0, 0x40));
                c.lq(17, at(sp0, 0x30));
                c.lq(18, at(sp0, 0x20));
                c.lq(19, at(sp0, 0x10));
                c.g[RA][0] = c.ld(sp0);
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at(sp0, 0x50));
                c.pc = ra;
                return Ok(());
            }
            c.g[A0][0] = lw_at(c, this, 0x28);
            if event_due(c) {
                deliver_event_here(c, 0x002d_f478);
            }
        }
    })
}

/// `void SnowFall_dtor(SnowFall* self, int flags)` (0x2e5d18): destroy the
/// eleven snow boxes and the base process.
pub fn snow_fall_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        open(c, 0x50);
        let sp0 = spv(c);
        c.g[V0][0] = 0x480000;
        c.sq(18, at(sp0, 0x20));
        c.g[V0][0] = sx(at(c.g[V0][0] as u32, 0x7fd8));
        c.sq(16, at(sp0, 0x40));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, at(sp0, 0x30));
        c.g[S0][0] = sx(at(c.g[S2][0] as u32, 0x10));
        c.sq(19, at(sp0, 0x10));
        c.g[S1][0] = 0xb;
        c.sd(sp0, c.g[RA][0]);
        c.g[S3][0] = c.g[A1][0];
        c.sw(at(c.g[S2][0] as u32, 0xc), c.g[V0][0]);
        call_ptr(c, callee::MEM_REPORT_STATS_LOCKED, 0x002e_5d50)?;
        loop {
            c.cycles += 3;
            c.g[A0][0] = lw_at(c, c.g[S0][0], 0);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = 3;
                callee::SNOW_BOX_DTOR.call(c, 0x002e_5d64)?;
            }
            c.g[S1][0] = sx(at(c.g[S1][0] as u32, 0xffff_ffff));
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(at(c.g[S0][0] as u32, 4));
            if !more {
                c.cycles += 12;
                call_ptr(c, callee::MEM_REPORT_STATS_LOCKED, 0x002e_5d78)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                callee::GAME_PROCESS_DTOR.call(c, 0x002e_5d84)?;
                c.lq(16, at(sp0, 0x40));
                c.lq(17, at(sp0, 0x30));
                c.lq(18, at(sp0, 0x20));
                c.lq(19, at(sp0, 0x10));
                c.g[RA][0] = c.ld(sp0);
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at(sp0, 0x50));
                c.pc = ra;
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x002e_5d50);
            }
        }
    })
}

/// Zero ten score/time table entries at `0x536708` and `0x536730`, then sort
/// both with `qsort` (element size 4, comparator `cmp`).
fn game_mode_man_sort(c: &mut Cpu, head: u32, ret_first: u32, ret_second: u32, cmp: u32) -> Flow {
    c.cycles += 10;
    open(c, 0x30);
    let sp0 = spv(c);
    c.g[A0][0] = 0x530000;
    c.sq(16, at(sp0, 0x20));
    c.g[V1][0] = 0;
    c.sq(17, at(sp0, 0x10));
    c.g[S0][0] = c.g[A1][0];
    c.g[S1][0] = 0x530000;
    c.sd(sp0, c.g[RA][0]);
    c.g[A2][0] = sx(at(c.g[A0][0] as u32, 0x6708));
    c.g[A1][0] = sx(at(c.g[S1][0] as u32, 0x6730));
    loop {
        c.cycles += 7;
        let a1 = c.g[A1][0] as u32;
        let a2 = c.g[A2][0] as u32;
        c.sw(a1, c.g[V1][0]);
        c.sw(a2, c.g[V1][0]);
        c.g[A1][0] = sx(at(a1, 4));
        c.g[V1][0] = sx(at(c.g[V1][0] as u32, 1));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx(at(a2, 4));
        if !more {
            c.cycles += 17;
            c.g[A3][0] = sx(cmp);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0x6708));
            c.g[A2][0] = 4;
            call_ptr(c, callee::QSORT, ret_first)?;
            c.g[A3][0] = 0x245ec0;
            c.g[A0][0] = sx(at(c.g[S1][0] as u32, 0x6730));
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 4;
            call_ptr(c, callee::QSORT, ret_second)?;
            c.lq(16, at(sp0, 0x20));
            c.lq(17, at(sp0, 0x10));
            c.g[RA][0] = c.ld(sp0);
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx(at(sp0, 0x30));
            c.pc = ra;
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void GameModeMan_SortByScore(GameModeMan* self, void* cmp)` (0x238b70).
pub fn game_mode_man_sort_by_score(c: &mut Cpu) {
    guest(c, |c| {
        game_mode_man_sort(c, 0x0023_8b98, 0x0023_8bcc, 0x0023_8be4, 0x0024_5e30)
    })
}

/// `void GameModeMan_SortByTime(GameModeMan* self, void* cmp)` (0x238bf8).
pub fn game_mode_man_sort_by_time(c: &mut Cpu) {
    guest(c, |c| {
        game_mode_man_sort(c, 0x0023_8c20, 0x0023_8c54, 0x0023_8c6c, 0x0024_5e78)
    })
}

/// `int PS2GraphicsMan_WaitForRenderIdle(PS2GraphicsMan* self)` (0x369c28):
/// wait until the renderer reports idle, servicing timers and yielding the
/// EE thread between polls. Dispatcher-shaped: an event unwind leaves `pc`
/// at the poll loop head and the trampoline re-enters there.
pub fn ps2_graphics_man_wait_for_render_idle(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        if pc == 0x0036_9c28 {
            c.cycles += 5;
            open(c, 0x20);
            let sp0 = spv(c);
            c.sq(16, at(sp0, 0x10));
            c.sd(sp0, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            pc = 0x0036_9c88;
        }
        loop {
            if pc == 0x0036_9c88 {
                c.cycles += 7;
                let this = c.g[S0][0];
                c.g[V1][0] = lw_at(c, this, 0x10d8);
                c.g[A0][0] = lh_at(c, c.g[V1][0], 0x90);
                c.g[V0][0] = lw_at(c, c.g[V1][0], 0x94);
                let func = c.g[V0][0] as u32;
                c.g[A0][0] = sx(at(this as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x0036_9c9c)?;
                let idle = c.g[V0][0] == 0;
                let sp0 = spv(c);
                c.g[RA][0] = c.ld(sp0);
                if idle {
                    if event_due(c) && deliver_event(c, 0x0036_9c40) {
                        return Err(crate::call::Unwound);
                    }
                    pc = 0x0036_9c40;
                } else {
                    c.cycles += 3;
                    c.lq(16, at(sp0, 0x10));
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(at(sp0, 0x20));
                    c.pc = ra;
                    return Ok(());
                }
            } else if pc == 0x0036_9c40 {
                c.cycles += 18;
                let g = gpa(c, 0x2a74);
                c.g[V0][0] = lw_at(c, g as u64, 0);
                c.g[A1][0] = lw_at(c, c.g[V0][0], 8);
                c.g[V1][0] = lw_at(c, c.g[A1][0], 8);
                c.g[A0][0] = lh_at(c, c.g[V1][0], 0x58);
                c.g[V0][0] = lw_at(c, c.g[V1][0], 0x5c);
                let func = c.g[V0][0] as u32;
                c.g[A0][0] = sx(at(c.g[A1][0] as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x0036_9c5c)?;
                c.g[A0][0] = 0;
                call_ptr(c, callee::EA_TIMER_SERVICE, 0x0036_9c64)?;
                c.g[A0][0] = 0;
                call_ptr(c, callee::EA_THREAD_YIELD, 0x0036_9c6c)?;
                let g = gpa(c, 0x2a74);
                c.g[V0][0] = lw_at(c, g as u64, 0);
                c.g[A1][0] = lw_at(c, c.g[V0][0], 8);
                c.g[V1][0] = lw_at(c, c.g[A1][0], 8);
                c.g[A0][0] = lh_at(c, c.g[V1][0], 0x60);
                c.g[V0][0] = lw_at(c, c.g[V1][0], 0x64);
                let func = c.g[V0][0] as u32;
                c.g[A0][0] = sx(at(c.g[A1][0] as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x0036_9c88)?;
                pc = 0x0036_9c88;
            } else if pc == 0x0036_9c3c {
                c.cycles += 1;
                pc = 0x0036_9c40;
            } else {
                c.jump_out(pc, 0x0036_9c28, 0x0036_9cb0);
                return Ok(());
            }
        }
    })
}

/// `void cFEStateTrophyRoom_ShowTrophyInfo(cFEStateTrophyRoom* self, int
/// which)` (0x1d4890): select the trophy list entry (`self+0x4c` -> list,
/// `+8` -> item) and refresh the event stat text.
pub fn c_fe_state_trophy_room_show_trophy_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.sq(17, at(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        let this = c.g[S0][0];
        c.g[A2][0] = lw_at(c, this, 0x4c);
        let list = c.g[A2][0];
        let empty = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            c.cycles += 2;
            let which = c.g[S1][0] != 0;
            c.g[V0][0] = lw_at(c, list, 8);
            let item = c.g[V0][0];
            if which {
                c.cycles += 12;
                c.g[A1][0] = 1;
                c.g[A0][0] = lh_at(c, item, 0x48);
                c.g[V1][0] = lw_at(c, item, 0x4c);
                let func = c.g[V1][0] as u32;
                c.g[A0][0] = sx(at(list as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x001d_48e8)?;
                c.g[T0][0] = lw_at(c, this, 0x58);
                c.g[T1][0] = sx(at(c.g[S1][0] as u32, 0xffff_ffff));
                c.g[A0][0] = lw_at(c, this, 0x4c);
                c.g[A1][0] = lb_at(c, this, 0x44);
                c.g[A2][0] = lw_at(c, this, 0x50);
                c.g[A3][0] = lw_at(c, this, 0x54);
                callee::UI_TEXT_SET_EVENT_STAT.call(c, 0x001d_4904)?;
                c.cycles += 1;
                c.lq(16, at(sp0, 0x20));
            } else {
                c.cycles += 7;
                c.g[A1][0] = 0;
                c.g[A0][0] = lh_at(c, item, 0x48);
                c.g[V1][0] = lw_at(c, item, 0x4c);
                let func = c.g[V1][0] as u32;
                c.g[A0][0] = sx(at(list as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x001d_48cc)?;
                c.lq(16, at(sp0, 0x20));
            }
        } else {
            c.cycles += 1;
            c.lq(16, at(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void TagFieldSetToken(TagField* self, u32 id, char* str, u32 token)`
/// (0x3ec0f8): copy the current tag into a stack buffer, append the token as
/// up to three characters (big-endian order) and set the field.
pub fn tag_field_set_token(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        open(c, 0x160);
        let sp0 = spv(c);
        c.sq(17, at(sp0, 0x140));
        c.sq(18, at(sp0, 0x130));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, at(sp0, 0x150));
        c.g[S2][0] = c.g[A1][0];
        c.sd(at(sp0, 0x120), c.g[RA][0]);
        c.g[S0][0] = c.g[A3][0];
        c.g[A1][0] = c.g[SP][0];
        callee::TAG_FIELD_COPY.call(c, 0x003e_c120)?;
        c.g[V1][0] = c.g[V0][0];
        let token = c.g[S0][0];
        if token != 0 {
            c.cycles += 4;
            c.g[A0][0] = 0x20ff_ffff;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            loop {
                c.cycles += 2;
                let skip = c.g[V0][0] == 0;
                c.g[V0][0] = sx((((c.g[S0][0] as u32) as i32) >> 24) as u32);
                if !skip {
                    c.cycles += 2;
                    c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                    c.g[V1][0] = sx(at(c.g[V1][0] as u32, 1));
                }
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S0][0] as u32) << 8);
                let more = c.g[S0][0] != 0;
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003e_c138);
                }
            }
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.sb(c.g[V1][0] as u32, 0);
        c.g[A2][0] = c.g[SP][0];
        callee::TAG_FIELD_SET_RAW.call(c, 0x003e_c168)?;
        c.lq(16, at(sp0, 0x150));
        c.lq(17, at(sp0, 0x140));
        c.lq(18, at(sp0, 0x130));
        c.g[RA][0] = c.ld(at(sp0, 0x120));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x160));
        c.pc = ra;
        Ok(())
    })
}

/// `int <X>_Render(void)` (0x232488, 0x232650, 0x232820, 0x2328a8): when the
/// FE screen is active, push its view, draw the loading indicator and the
/// screen's HUD slot; returns 1 when a screen was drawn.
fn fe_load_screen_render(c: &mut Cpu, entry: u32) -> Flow {
    c.cycles += 10;
    open(c, 0x20);
    let sp0 = spv(c);
    let g = gpa(c, -0x854);
    c.g[A1][0] = lw_at(c, g as u64, 0);
    c.sd(at(sp0, 0x10), c.g[RA][0]);
    let manager = c.g[A1][0];
    c.g[V1][0] = lw_at(c, manager, 0x10d8);
    c.g[A0][0] = lh_at(c, c.g[V1][0], 0x88);
    c.g[V0][0] = lw_at(c, c.g[V1][0], 0x8c);
    let func = c.g[V0][0] as u32;
    c.g[A0][0] = sx(at(manager as u32, c.g[A0][0] as u32));
    call_ptr(c, func, entry + 0x20)?;
    let active = c.g[V0][0] != 0;
    let g = gpa(c, -0x854);
    c.g[V0][0] = lw_at(c, g as u64, 0);
    if active {
        c.cycles += 20;
        let mgr = c.g[V0][0];
        c.g[V1][0] = lw_at(c, mgr, 0x10d8);
        c.g[A0][0] = lh_at(c, c.g[V1][0], 0x78);
        c.sw(sp0, 0);
        c.sw(at(sp0, 4), 0);
        c.g[A0][0] = sx(at(mgr as u32, c.g[A0][0] as u32));
        c.sw(at(sp0, 8), 0);
        c.g[V0][0] = lw_at(c, c.g[V1][0], 0x7c);
        let func = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, func, entry + 0x4c)?;
        let g = gpa(c, -0x848);
        c.g[V1][0] = lw_at(c, g as u64, 0);
        c.g[A0][0] = lw_at(c, c.g[V1][0], 0xa4);
        callee::LOAD_INDICATOR_RENDER.call(c, entry + 0x58)?;
        let g = gpa(c, -0x854);
        c.g[A1][0] = lw_at(c, g as u64, 0);
        let mgr = c.g[A1][0];
        c.g[V0][0] = lw_at(c, mgr, 0x10d8);
        c.g[A0][0] = lh_at(c, c.g[V0][0], 0xa0);
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0xa4);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at(mgr as u32, c.g[A0][0] as u32));
        call_ptr(c, func, entry + 0x70)?;
        c.g[V0][0] = 1;
    } else {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld(at(sp0, 0x10));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(sp0, 0x20));
    c.pc = ra;
    Ok(())
}

/// `int PreStartScreen_Render(void)` (0x232488).
pub fn pre_start_screen_render(c: &mut Cpu) {
    guest(c, |c| fe_load_screen_render(c, 0x0023_2488))
}

/// `int InitLoadScreen_Render(void)` (0x232650).
pub fn init_load_screen_render(c: &mut Cpu) {
    guest(c, |c| fe_load_screen_render(c, 0x0023_2650))
}

/// `int cPreGameLoadScreen_Render(void)` (0x232820).
pub fn c_pre_game_load_screen_render(c: &mut Cpu) {
    guest(c, |c| fe_load_screen_render(c, 0x0023_2820))
}

/// `int cPreFELoadScreen_Render(void)` (0x2328a8).
pub fn c_pre_fe_load_screen_render(c: &mut Cpu) {
    guest(c, |c| fe_load_screen_render(c, 0x0023_28a8))
}

/// `void cUITextScroll_AddLine(cUITextScroll* self, char16* line)`
/// (0x3a1bf8): grow the line array, duplicate the string and append it.
pub fn c_ui_text_scroll_add_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        open(c, 0x50);
        let sp0 = spv(c);
        c.sq(19, at(sp0, 0x10));
        c.sq(18, at(sp0, 0x20));
        c.g[S3][0] = c.g[A0][0];
        c.sq(16, at(sp0, 0x40));
        c.sq(17, at(sp0, 0x30));
        c.sd(sp0, c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        callee::GROW_LINES.call(c, 0x003a_1c1c)?;
        let this = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, callee::USTR_LEN, 0x003a_1c28)?;
        c.g[V0][0] = sx(at(c.g[V0][0] as u32, 1));
        c.g[A1][0] = 0x490000;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[A1][0] = sx(at(c.g[A1][0] as u32, 0x40b8));
        call_ptr(c, callee::OPERATOR_NEW_NAMED, 0x003a_1c44)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::USTR_CPY, 0x003a_1c54)?;
        c.g[V0][0] = lw_at(c, this, 0xbc);
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.lq(18, at(sp0, 0x20));
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V0][0] as u32));
        c.lq(19, at(sp0, 0x10));
        c.sw(c.g[S0][0] as u32, c.g[S1][0]);
        c.lq(16, at(sp0, 0x40));
        c.lq(17, at(sp0, 0x30));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x50));
        c.pc = ra;
        Ok(())
    })
}

/// `UIObject* cUIGroup_FindById(cUIGroup* self, int id)` (0x3986b0): search
/// the group's own objects, then walk the sub-list looking for an object with
/// the id.
pub fn c_ui_group_find_by_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.sq(17, at(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        callee::UI_OBJECT_FIND_BY_ID.call(c, 0x0039_86cc)?;
        let hit = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S0][0];
        // The generated code charges the final +1 on every path except the
        // in-list hit, which leaves through the outer block.
        let mut charge_one = true;
        if !hit {
            c.cycles += 5;
            let this = c.g[S0][0];
            c.g[A0][0] = sx(at(this as u32, 0x74));
            callee::UI_LIST_FIRST.call(c, 0x0039_86dc)?;
            c.g[S0][0] = c.g[V0][0];
            let empty = c.g[S0][0] == 0;
            c.g[V0][0] = 0;
            if !empty {
                c.cycles += 2;
                c.g[V1][0] = lw_at(c, c.g[S0][0], 8);
                loop {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = lh_at(c, c.g[V1][0], 0x70);
                    c.g[V0][0] = lw_at(c, c.g[V1][0], 0x74);
                    let func = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(at(c.g[S0][0] as u32, c.g[A0][0] as u32));
                    call_ptr(c, func, 0x0039_8704)?;
                    if c.g[V0][0] != 0 {
                        c.lq(16, at(sp0, 0x20));
                        charge_one = false;
                        break;
                    }
                    c.cycles += 5;
                    c.g[S0][0] = lw_at(c, c.g[S0][0], 4);
                    c.g[A0][0] = c.g[S0][0];
                    callee::UI_LIST_NODE_SENTINEL.call(c, 0x0039_8718)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                        break;
                    }
                    c.g[V1][0] = lw_at(c, c.g[S0][0], 8);
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_86f0);
                    }
                }
            }
        }
        if charge_one {
            c.cycles += 1;
            c.lq(16, at(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void EventSelect_SetMapTabMode(EventSelect* self, int mode)` (0x203cb8):
/// store the map-tab mode and label the tab from the new mode.
pub fn event_select_set_map_tab_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, at(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[A0][0] = 0x470000;
        let this = c.g[S0][0];
        c.sw(this as u32, c.g[A1][0]);
        c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0xab8));
        call_ptr(c, callee::STR_HASH, 0x0020_3cdc)?;
        c.g[A0][0] = lw_at(c, this, 0x2e8);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x0020_3ce8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = lw_at(c, this, 0);
        let mode_zero = c.g[V0][0] == 0;
        c.g[A0][0] = 0x470000;
        if mode_zero {
            c.cycles += 6;
            c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0xad8));
            call_ptr(c, callee::STR_HASH, 0x0020_3d20)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_TEXT_SET_TEXT_ID, 0x0020_3d2c)?;
            c.lq(16, at(sp0, 0x20));
        } else {
            c.cycles += 8;
            c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0xac8));
            call_ptr(c, callee::STR_HASH, 0x0020_3d04)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_TEXT_SET_TEXT_ID, 0x0020_3d10)?;
            c.lq(16, at(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void FloatRail_ResetPosFromInstance(FloatRail* self, Instance* inst)`
/// (0x349220): look the instance's node up in the world table and copy its
/// node matrix into the rail's transform.
pub fn float_rail_reset_pos_from_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        open(c, 0x70);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x60));
        c.sd(at(sp0, 0x50), c.g[RA][0]);
        c.g[S0][0] = c.g[A1][0];
        callee::WORLD_GET_INSTANCE.call(c, 0x0034_9234)?;
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0);
        let inst = c.g[S0][0];
        c.g[A1][0] = lw_at(c, inst, 0x40);
        c.g[A0][0] = lw_at(c, c.g[V1][0], 8);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx(at(c.g[V0][0] as u32, c.g[A0][0] as u32));
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0);
        let none = c.g[V1][0] == 0;
        c.g[A0][0] = 0;
        if !none {
            c.cycles += 8;
            c.g[A0][0] = lw_at(c, c.g[V1][0], 0x1c);
            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx(at(c.g[V0][0] as u32, c.g[A0][0] as u32));
            c.g[V1][0] = lw_at(c, c.g[V0][0], 0);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let none2 = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            if !none2 {
                c.cycles += 1;
                c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
            }
        }
        c.cycles += 11;
        c.g[A1][0] = lw_at(c, inst, 0x44);
        c.g[A2][0] = c.g[SP][0];
        callee::INSTANCE_GET_NODE_MATRIX.call(c, 0x0034_9288)?;
        c.lq(2, at(sp0, 0x30));
        c.g[RA][0] = c.ld(at(sp0, 0x50));
        c.sq(2, at(inst as u32, 0x20));
        c.sq(2, at(sp0, 0x40));
        c.sq(2, at(inst as u32, 0x30));
        c.lq(16, at(sp0, 0x60));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x70));
        c.pc = ra;
        Ok(())
    })
}

/// `void cOVStateOverlay_PlayObjectAnim(cOVStateOverlay* self, char* widget,
/// char* anim)` (0x20a6f0): find the widget and play the named animation on
/// it (the overlay counterpart of `MemCardStateBase_PlayWidgetAnim`).
pub fn c_ov_state_overlay_play_object_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        open(c, 0x40);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x30));
        c.sq(18, at(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, at(sp0, 0x20));
        c.g[A0][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, callee::STR_HASH, 0x0020_a714)?;
        let this = c.g[S0][0];
        c.g[A0][0] = lw_at(c, this, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x0020_a720)?;
        c.g[S1][0] = c.g[V0][0];
        let found = c.g[S1][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if found {
            c.cycles += 8;
            c.g[S0][0] = lw_at(c, this, 0x10);
            c.g[S0][0] = sx(at(c.g[S0][0] as u32, 0x50));
            call_ptr(c, callee::STR_HASH, 0x0020_a738)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_ANIM_SET_CREATE_ANIM, 0x0020_a744)?;
            let anim = c.g[V0][0];
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if !none {
                c.cycles += 5;
                c.g[A1][0] = anim;
                c.g[A2][0] = 3;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                call_ptr(c, callee::UI_OBJECT_SET_ANIM, 0x0020_a760)?;
            }
        }
        c.cycles += 6;
        c.lq(16, at(sp0, 0x30));
        c.lq(17, at(sp0, 0x20));
        c.lq(18, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `void BEBAGTInt_SetUberTrickButton(BEBAGTInt* self, int player, u32 id,
/// u8 button)` (0x14fd80): write the button byte into the per-character row
/// of the bag of tricks (`0x52eec0 + profile*0x13ec + player*6 + id`).
pub fn bebagt_int_set_uber_trick_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        open(c, 0x50);
        let sp0 = spv(c);
        c.sq(18, at(sp0, 0x20));
        c.g[S2][0] = c.g[A1][0];
        c.sq(16, at(sp0, 0x40));
        c.sq(19, at(sp0, 0x10));
        c.g[S0][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        c.sq(17, at(sp0, 0x30));
        c.sd(sp0, c.g[RA][0]);
        c.g[A0][0] = c.g[S2][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0014_fdac)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[S1][0] = c.g[V0][0];
        callee::BE_GET_RIDER_CHARACTER.call(c, 0x0014_fdb8)?;
        c.g[V1][0] = 0x1fe;
        c.g[A0][0] = 6;
        c.mult(0, 5, 2, 3);
        c.mult(0, 16, 16, 4);
        c.g[V0][0] = 0x13ec;
        c.g[V1][0] = 0x530000;
        c.mult(0, 17, 17, 2);
        c.g[V1][0] = sx(at(c.g[V1][0] as u32, 0xec0));
        c.g[V0][0] = 1;
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[A1][0] as u32));
        c.g[RA][0] = c.ld(sp0);
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[S1][0] as u32));
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V1][0] as u32));
        c.lq(18, at(sp0, 0x20));
        c.sb(at(c.g[S0][0] as u32, 1), c.g[S3][0]);
        c.lq(17, at(sp0, 0x30));
        c.lq(16, at(sp0, 0x40));
        c.lq(19, at(sp0, 0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x50));
        c.pc = ra;
        Ok(())
    })
}

/// `void SectionMan_FinishEventSwitch(SectionMan* self)` (0x22dff0):
/// reselect peak lighting for the track, drop the pending event switch and
/// request the game state change.
pub fn section_man_finish_event_switch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        open(c, 0x20);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0022_e004)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0022_e010)?;
        c.g[A0][0] = c.g[V0][0];
        callee::BE_GET_TRACK_INFO.call(c, 0x0022_e018)?;
        c.g[A1][0] = lw_at(c, c.g[V0][0], 0);
        let this = c.g[S0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::SECTION_SELECT_PEAK_LIGHTING.call(c, 0x0022_e024)?;
        c.g[V0][0] = lw_at(c, this, 0x1cc);
        let none = c.g[V0][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 9;
            c.sw(at(this as u32, 0x1cc), 0);
            let g = gpa(c, -0x848);
            c.g[V0][0] = lw_at(c, g as u64, 0);
            c.g[V1][0] = lw_at(c, c.g[V0][0], 0x84);
            c.g[A2][0] = lw_at(c, c.g[V1][0], 0xc);
            c.g[V0][0] = lw_at(c, c.g[A2][0], 0xcc);
            c.g[A0][0] = lh_at(c, c.g[V0][0], 0x28);
            c.g[V1][0] = lw_at(c, c.g[V0][0], 0x2c);
            let func = c.g[V1][0] as u32;
            c.g[A0][0] = sx(at(c.g[A2][0] as u32, c.g[A0][0] as u32));
            call_ptr(c, func, 0x0022_e054)?;
        }
        c.cycles += 9;
        c.g[A0][0] = lw_at(c, this, 0x1b0);
        c.g[A1][0] = 0xa;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, callee::GAME_MODULE_REQUEST_STATE, 0x0022_e068)?;
        c.lq(16, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `int cStreamMan_QueueRead(cStreamMan* self, u32 offset, u32 size, u32 tag)`
/// (0x3a6f88): queue a read of the stream file (tag "DNEC"); returns 1 when a
/// read was queued.
pub fn c_stream_man_queue_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x40);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x30));
        c.sq(17, at(sp0, 0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, at(sp0, 0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S2][0] = c.g[A2][0];
        let this = c.g[S0][0];
        c.g[V0][0] = lw_at(c, this, 0x90);
        let busy = c.g[V0][0] != 0;
        c.g[A2][0] = c.g[A3][0];
        let mut queued = false;
        if !busy {
            c.cycles += 7;
            c.g[A0][0] = lw_at(c, this, 0x88);
            c.g[A3][0] = 0x444e0000;
            c.g[A1][0] = sx(at(this as u32, 4));
            c.g[A3][0] = c.g[A3][0] | 0x4543;
            call_ptr(c, callee::STREAM_FILE_QUEUE_RANGE, 0x003a_6fc8)?;
            let ok = c.g[V0][0] == 0;
            c.sw(at(this as u32, 0x9c), c.g[V0][0]);
            if !ok {
                c.cycles += 9;
                c.g[V0][0] = 1;
                c.sw(at(this as u32, 0x94), c.g[S1][0]);
                c.sw(at(this as u32, 0x90), c.g[V0][0]);
                c.sw(at(this as u32, 0x98), c.g[S2][0]);
                c.sw(at(this as u32, 0xe0), 0);
                c.sw(at(this as u32, 0xa0), 0);
                c.sw(at(this as u32, 0xc4), 0);
                c.sw(at(this as u32, 0xe4), 0);
                queued = true;
            }
        }
        if !queued {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        c.lq(16, at(sp0, 0x30));
        c.lq(17, at(sp0, 0x20));
        c.lq(18, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `UIAnim* UIAnimSet_CreateAnim(UIAnimSet* self, u32 id)` (0x397140): find
/// the animation entry with `id`, allocate it and run the animation
/// constructor. 0 when there is no such entry.
pub fn ui_anim_set_create_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        open(c, 0x20);
        let sp0 = spv(c);
        c.g[A3][0] = c.g[A0][0];
        c.sq(16, at(sp0, 0x10));
        c.g[A2][0] = 0;
        c.sd(sp0, c.g[RA][0]);
        c.g[V0][0] = lw_at(c, c.g[A3][0], 0);
        let set = c.g[V0][0];
        c.g[V1][0] = lw_at(c, set, 0);
        let empty = c.g[V1][0] == 0;
        c.g[S0][0] = sx(at(set as u32, 4));
        let mut made = false;
        if !empty {
            c.cycles += 1;
            c.g[T0][0] = 0x4a0000;
            loop {
                c.cycles += 3;
                let ent = c.g[S0][0];
                c.g[V0][0] = lw_at(c, ent, 8);
                if c.g[V0][0] != c.g[A1][0] {
                    c.g[A0][0] = lw_at(c, c.g[A3][0], 0);
                } else {
                    c.cycles += 10;
                    c.g[A1][0] = sx(at(c.g[T0][0] as u32, 0x4768));
                    c.g[A0][0] = 0x14;
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0;
                    call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0039_7188)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    callee::UI_ANIM_CTOR.call(c, 0x0039_7194)?;
                    c.lq(16, at(sp0, 0x10));
                    made = true;
                    break;
                }
                c.cycles += 6;
                c.g[A2][0] = sx(at(c.g[A2][0] as u32, 1));
                c.g[V1][0] = lw_at(c, c.g[S0][0], 0xc);
                c.g[V0][0] = lw_at(c, c.g[A0][0], 0);
                c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_7168);
                }
            }
        }
        if !made {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.lq(16, at(sp0, 0x10));
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `void FEAsyncManager_RequestPeakMap(FEAsyncManager* self, int index,
/// u32 map, int state)` (0x1a3768): format the peak-map path into the request
/// slot and arm the async state machine.
pub fn fe_async_manager_request_peak_map(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        open(c, 0x50);
        let sp0 = spv(c);
        c.g[V0][0] = 0x440000;
        c.sq(16, at(sp0, 0x40));
        c.g[V0][0] = sx(at(c.g[V0][0] as u32, 0x1258));
        c.sq(19, at(sp0, 0x10));
        c.g[S0][0] = 0x11c;
        c.g[S3][0] = c.g[A2][0];
        c.sq(17, at(sp0, 0x30));
        c.mult(0, 16, 19, 16);
        c.sq(18, at(sp0, 0x20));
        c.sd(sp0, c.g[RA][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A1][0] = sx(at(c.g[A1][0] as u32, c.g[V0][0] as u32));
        c.g[S2][0] = c.g[A3][0];
        c.g[A2][0] = lw_at(c, c.g[A1][0], 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x4a0000;
        c.g[A3][0] = 0x4a0000;
        c.g[A0][0] = sx(at(c.g[S1][0] as u32, c.g[S0][0] as u32));
        c.g[A3][0] = sx(at(c.g[A3][0] as u32, 0x1a00));
        c.g[A1][0] = sx(at(c.g[A1][0] as u32, 0x19f8));
        call_ptr(c, callee::MSG_PRINTF, 0x001a_37c0)?;
        let rec = sx(at(c.g[S1][0] as u32, c.g[S0][0] as u32));
        c.g[S0][0] = rec;
        c.g[A0][0] = c.g[S1][0];
        c.sw(at(rec as u32, 0x108), c.g[S2][0]);
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = 1;
        callee::FE_ASYNC_SET_STATE.call(c, 0x001a_37d8)?;
        c.lq(16, at(sp0, 0x40));
        c.lq(17, at(sp0, 0x30));
        c.lq(18, at(sp0, 0x20));
        c.lq(19, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x50));
        c.pc = ra;
        Ok(())
    })
}

/// `void cFEStateNetMessenger_SetRowName(cFEStateNetMessenger* self, u32 row,
/// char* text, u32 id)` (0x1a6618): label a net-messenger list row.
pub fn c_fe_state_net_messenger_set_row_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x60);
        let sp0 = spv(c);
        c.g[V1][0] = c.g[A1][0];
        c.sq(16, at(sp0, 0x50));
        c.sq(17, at(sp0, 0x40));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, at(sp0, 0x30));
        c.g[S1][0] = c.g[A2][0];
        c.sd(at(sp0, 0x20), c.g[RA][0]);
        c.g[V0][0] = lw_at(c, c.g[S0][0], 0x40);
        let nolist = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A3][0];
        if !nolist {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < 0xfu64) as u64;
            let bad_row = c.g[V0][0] == 0;
            c.g[A1][0] = 0x460000;
            if !bad_row {
                c.cycles += 12;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[V1][0];
                c.g[A1][0] = sx(at(c.g[A1][0] as u32, 0x15a0));
                call_ptr(c, callee::MSG_PRINTF, 0x001a_6660)?;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, callee::STR_HASH, 0x001a_6668)?;
                c.g[A0][0] = lw_at(c, c.g[S0][0], 0x40);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x001a_6674)?;
                c.g[S0][0] = c.g[V0][0];
                let nohit = c.g[S0][0] == 0;
                c.g[A1][0] = c.g[S1][0];
                if !nohit {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, callee::UI_TEXT_SET_STRING, 0x001a_6688)?;
                    c.sw(at(c.g[S0][0] as u32, 0x18), c.g[S2][0]);
                }
            }
        }
        c.cycles += 6;
        c.lq(16, at(sp0, 0x50));
        c.lq(17, at(sp0, 0x40));
        c.lq(18, at(sp0, 0x30));
        c.g[RA][0] = c.ld(at(sp0, 0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x60));
        c.pc = ra;
        Ok(())
    })
}

/// `void NetFEState_ShowPopup(NetFEState* self, Popup* popup)` (0x1a9bc0):
/// raise the popup's owning state or push the popup and rebuild its confirm
/// widget.
pub fn net_fe_state_show_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.sq(17, at(sp0, 0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = lw_at(c, c.g[S1][0], 0x40);
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0xd0);
        c.g[A0][0] = lw_at(c, c.g[V1][0], 0x10);
        c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0x18));
        call_ptr(c, callee::UI_LIST_FIRST_WRAPPER, 0x001a_9bec)?;
        c.g[V1][0] = c.g[V0][0];
        let popup = c.g[S0][0];
        c.g[V0][0] = lw_at(c, popup, 0x2b8);
        let owned = c.g[V0][0] != 0;
        // The final `+1; lq16` runs on every path except the push.
        let mut charge_one = true;
        if owned {
            c.g[V0][0] = lw_at(c, popup, 0x1c);
        } else {
            c.cycles += 2;
            let same = popup == c.g[V1][0];
            c.g[A1][0] = c.g[S0][0];
            if !same {
                c.cycles += 7;
                c.g[V0][0] = lw_at(c, c.g[S1][0], 0x40);
                c.g[V1][0] = lw_at(c, c.g[V0][0], 0xd0);
                c.g[A0][0] = lw_at(c, c.g[V1][0], 0x10);
                c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0x18));
                call_ptr(c, callee::UI_STATE_MGR_PUSH_POPUP, 0x001a_9c18)?;
                c.lq(16, at(sp0, 0x20));
                charge_one = false;
            }
        }
        if owned {
            c.cycles += 6;
            c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x500;
            c.sw(at(popup as u32, 0x1c), c.g[V0][0]);
            c.g[A0][0] = lw_at(c, c.g[S1][0], 0x6a0);
            callee::POPUP_CONFIRM_REBUILD.call(c, 0x001a_9c38)?;
        }
        if charge_one {
            c.cycles += 1;
            c.lq(16, at(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `int cFEStateCreateStep1_IsEmailFormatValid(cFEStateCreateStep1* self,
/// char* email)` (0x1b9cd8): true when the address has both an `@` and a
/// `.` (the game's own, lax check).
pub fn c_fe_state_create_step1_is_email_format_valid(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        open(c, 0x40);
        let sp0 = spv(c);
        c.sq(17, at(sp0, 0x20));
        c.g[S1][0] = c.g[A1][0];
        c.sq(16, at(sp0, 0x30));
        c.sq(18, at(sp0, 0x10));
        c.g[A0][0] = c.g[S1][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S2][0] = 0;
        call_ptr(c, callee::STRLEN, 0x001b_9cfc)?;
        c.g[S0][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        let empty = (c.g[A0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !empty {
            c.cycles += 3;
            c.g[A2][0] = 0x40;
            c.g[A1][0] = 1;
            c.g[V0][0] = sx(at(c.g[S1][0] as u32, c.g[V1][0] as u32));
            loop {
                c.cycles += 3;
                let at_p = c.g[V0][0] as u32;
                c.g[V0][0] = lb_at(c, at_p as u64, 0);
                let not_at = c.g[V0][0] != c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] ^ 0x2e;
                if !not_at {
                    // An `@`: mark it and skip the `.` test for this byte.
                    c.cycles += 2;
                    c.g[S2][0] = 1;
                } else {
                    c.cycles += 1;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[A1][0];
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = sx(at(c.g[V1][0] as u32, 1));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx(at(c.g[S1][0] as u32, c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001b_9d18);
                }
            }
        }
        c.cycles += 2;
        let no_at = c.g[S2][0] == 0;
        c.g[V0][0] = 0;
        if !no_at {
            c.cycles += 1;
            c.g[V0][0] = (0u64 < c.g[S0][0]) as u64;
        }
        c.cycles += 6;
        c.lq(16, at(sp0, 0x30));
        c.lq(17, at(sp0, 0x20));
        c.lq(18, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `void cFEStateCreateStep2_SetYearSelection(cFEStateCreateStep2* self)`
/// (0x1bb3b0): label the year list box with the current year and select the
/// rider's year.
pub fn c_fe_state_create_step2_set_year_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        open(c, 0x40);
        let sp0 = spv(c);
        c.sq(17, at(sp0, 0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, at(sp0, 0x30));
        c.g[A0][0] = 0x4a0000;
        c.sd(at(sp0, 0x10), c.g[RA][0]);
        c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0x1be8));
        call_ptr(c, callee::STR_HASH, 0x001b_b3d0)?;
        c.g[A0][0] = lw_at(c, c.g[S1][0], 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x001b_b3dc)?;
        c.g[S0][0] = c.g[V0][0];
        let nohit = c.g[S0][0] == 0;
        c.g[A0][0] = c.g[SP][0];
        if !nohit {
            c.cycles += 16;
            c.g[V0][0] = lw_at(c, c.g[S1][0], 0x6d0);
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx(at(c.g[A1][0] as u32, 0x12c0));
            c.g[A2][0] = lw_at(c, c.g[V0][0], 0x30);
            call_ptr(c, callee::MSG_PRINTF, 0x001b_b3fc)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            c.g[A3][0] = 0;
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, callee::UI_LISTBOX_SET_ITEM_TEXT, 0x001b_b410)?;
            c.g[V0][0] = lw_at(c, c.g[S1][0], 0x6d0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = lbu_at(c, c.g[V0][0], 0x30);
            c.g[A1][0] = sx(at(c.g[A1][0] as u32, 0xffff_ff94));
            c.g[A1][0] = c.g[A1][0] & 0xff;
            call_ptr(c, callee::SET_ITEM_YEAR, 0x001b_b428)?;
        }
        c.cycles += 5;
        c.lq(16, at(sp0, 0x30));
        c.lq(17, at(sp0, 0x20));
        c.g[RA][0] = c.ld(at(sp0, 0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `TexMan* TexMan_ctor(TexMan* self)` (0x366fe8): reset the table, the free
/// list and the two VRAM heaps.
pub fn tex_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(17, at(sp0, 0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, at(sp0, 0x20));
        c.sd(sp0, c.g[RA][0]);
        let this = c.g[S1][0];
        c.sw(at(this as u32, 0x1f58), 0);
        callee::TEX_MAN_INIT_TABLE.call(c, 0x0036_7004)?;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 1;
        c.g[V0][0] = sx(at(this as u32, 0x1f50));
        loop {
            c.cycles += 7;
            let slot = c.g[V0][0] as u32;
            c.sw(slot, c.g[A0][0]);
            c.g[V1][0] = sx(at(c.g[V1][0] as u32, 0xffff_ffff));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(at(slot, 0xffff_fffc));
            if !more {
                c.cycles += 18;
                c.g[S0][0] = sx(at(this as u32, 8));
                c.g[V0][0] = u64::MAX;
                c.sw(at(this as u32, 0x1f54), c.g[V0][0]);
                c.g[A0][0] = sx(at(this as u32, 0x1f60));
                c.g[A3][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x400000;
                callee::VRAM_HEAP_INIT.call(c, 0x0036_704c)?;
                c.g[A0][0] = sx(at(this as u32, 0x4350));
                c.g[A3][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x1000;
                callee::VRAM_HEAP_INIT.call(c, 0x0036_7060)?;
                c.lq(16, at(sp0, 0x20));
                c.lq(17, at(sp0, 0x10));
                c.g[RA][0] = c.ld(sp0);
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at(sp0, 0x30));
                c.pc = ra;
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_7010);
            }
        }
    })
}

/// `void LobbyMan_PeekRoom(LobbyMan* self, int room)` (0x25d770): build the
/// room name and send the "peek" lobby request.
pub fn lobby_man_peek_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x130);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x120));
        c.sq(17, at(sp0, 0x110));
        c.g[S0][0] = c.g[A0][0];
        c.sd(at(sp0, 0x100), c.g[RA][0]);
        let this = c.g[S0][0];
        c.g[V0][0] = lw_at(c, this, 0);
        c.g[V1][0] = 0x4a0000;
        let nolist = c.g[V0][0] == 0;
        if !nolist {
            // The two name sources: the room name (or the empty name).
            c.cycles += 4;
            c.sb(sp0, 0);
            c.g[V0][0] = u64::MAX;
            let unnamed = c.g[A1][0] == c.g[V0][0];
            c.g[S1][0] = sx(at(c.g[V1][0] as u32, 0x2f58));
            if !unnamed {
                c.cycles += 4;
                callee::LOBBY_GET_ROOM_NAME.call(c, 0x0025_d7a8)?;
                c.g[A2][0] = c.g[S1][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x4a0000;
                c.g[V0][0] = sx(at(c.g[V0][0] as u32, 0x2f40));
                c.g[A2][0] = c.g[S1][0];
            }
            c.cycles += 11;
            c.g[A3][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_d7cc)?;
            c.g[A1][0] = 0x70650000;
            c.g[A0][0] = lw_at(c, this, 0x54);
            c.g[A1][0] = c.g[A1][0] | 0x656b;
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, callee::LOBBY_API_SEND, 0x0025_d7e8)?;
        }
        c.cycles += 5;
        c.lq(16, at(sp0, 0x120));
        c.lq(17, at(sp0, 0x110));
        c.g[RA][0] = c.ld(at(sp0, 0x100));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x130));
        c.pc = ra;
        Ok(())
    })
}

/// `bool BuddyMan_IsOnlineAt(BuddyMan* self, int index)` (0x263338): the
/// roster entry's online flag (`+0xa7` bit 0) or its online bit (`+0xa8`).
pub fn buddy_man_is_online_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.sq(17, at(sp0, 0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        let idx = c.g[S0][0];
        c.g[S1][0] = c.g[A0][0];
        let mut v1: u64 = 0;
        let mut to_entry = false;
        if (idx as i64) < 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 5;
            callee::BUDDY_GET_ROSTER_COUNT.call(c, 0x0026_335c)?;
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            let bad = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if bad {
                // returns 0
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                callee::BUDDY_GET_ROSTER_ENTRY.call(c, 0x0026_3374)?;
                v1 = c.g[V0][0];
                c.g[V1][0] = v1;
                if v1 != 0 {
                    c.g[V0][0] = lb_at(c, v1, 0xa7);
                    to_entry = true;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                }
            }
        }
        if to_entry {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] & 1;
            let bit0 = c.g[V0][0] != 0;
            c.g[A0][0] = 0;
            if bit0 {
                c.cycles += 2;
                c.g[A0][0] = 1;
                c.g[V0][0] = c.g[A0][0];
            } else {
                c.cycles += 5;
                c.g[V0][0] = lw_at(c, v1, 0xa8);
                c.g[V1][0] = 0x100000;
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                let zero = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[A0][0];
                if !zero {
                    c.cycles += 2;
                    c.g[A0][0] = 1;
                    c.g[V0][0] = c.g[A0][0];
                }
            }
        }
        c.cycles += 5;
        c.lq(16, at(sp0, 0x20));
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void cSetupMan_SetLocalCharacter(cSetupMan* self, int character)`
/// (0x264cf0): set the local player's character in the race setup and its
/// profile record.
pub fn c_setup_man_set_local_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(17, at(sp0, 0x10));
        c.sq(16, at(sp0, 0x20));
        c.sd(sp0, c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0026_4d08)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_4d14)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::BE_SET_CHARACTER_BY_SLOT.call(c, 0x0026_4d28)?;
        c.g[V0][0] = lw_at(c, c.g[S0][0], 0xc);
        c.g[A0][0] = lh_at(c, c.g[V0][0], 8);
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0xc);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at(c.g[S0][0] as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0026_4d3c)?;
        let g = gpa(c, -0x848);
        c.g[V0][0] = lw_at(c, g as u64, 0);
        c.g[A1][0] = 7;
        c.g[A0][0] = lw_at(c, c.g[V0][0], 0x78);
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_4d4c)?;
        c.g[A0][0] = 0x530000;
        c.sw(at(c.g[A0][0] as u32, 0x4e58), c.g[S1][0]);
        c.g[V1][0] = lw_at(c, c.g[V0][0], 0xc);
        c.g[A0][0] = lh_at(c, c.g[V1][0], 8);
        c.g[A1][0] = lw_at(c, c.g[V1][0], 0xc);
        let func = c.g[A1][0] as u32;
        c.g[A0][0] = sx(at(c.g[V0][0] as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0026_4d68)?;
        c.lq(16, at(sp0, 0x20));
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, FAKE, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA, GAME_GP, RA as RET};
    use ssx3_ee as ee;

    /// Call a guest function with `args` in a0.. and return `v0`.
    fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = RET as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, RET, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The ELF string hash of the string at `s`.
    fn str_hash(c: &mut Cpu, s: u32) -> u32 {
        run(c, 0x0031_7670, ee::s10::f_00317670, &[s as u64]) as u32
    }

    /// A NUL-terminated byte string at `addr`.
    fn put_str(c: &mut Cpu, addr: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(addr + i as u32, *b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// The lazily built BE singleton at `gp-0x1f04` (a zeroed dummy), the
    /// current interface index at `gp-0x1efc`, and interface 0 of the
    /// `0x5308a8` table pointing at a test object whose `+8` vtable slot calls
    /// the fake. Returns the interface object.
    fn be_library(c: &mut Cpu, r: &mut Rng) -> u32 {
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
        let obj = DATA + 0x20_0000;
        let vt = vtable(c, r, &[0x8]);
        c.bus.write32(obj + 4, vt);
        c.bus.write32(0x0053_08a8, obj);
        obj
    }

    /// `gp-0x848` -> a dummy world.
    fn world(c: &mut Cpu, w: u32) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
    }

    /// A UIScreen at `screen` whose only child (`+0x38` -> count cell, `+0x3c`
    /// -> array) has id `id`; the child is a UI text object whose `+0x8` slot
    /// `+0x28` calls the fake. Returns the child.
    fn screen_with(c: &mut Cpu, r: &mut Rng, screen: u32, id: u32) -> u32 {
        let node = screen + 0x100;
        let vt = vtable(c, r, &[0x28]);
        c.bus.write32(screen + 0x38, screen + 0x10);
        c.bus.write32(screen + 0x3c, screen + 0x20);
        c.bus.write32(screen + 0x10, 1);
        c.bus.write32(screen + 0x20, node);
        c.bus.write32(node + 0x8, vt);
        c.bus.write32(node + 0x38, id);
        node
    }

    /// The shared camera-algorithm fixture: `this` at DATA, the rider target at
    /// DATA+0x100, the rider object at DATA+0x200 and the notify object at
    /// DATA+0x300, all slots pointing at the fake. `equal` picks the branch
    /// where the rider's current algorithm id still answers `rider`.
    fn camera_case(c: &mut Cpu, r: &mut Rng, equal: bool, rider: u64) {
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = rider;
        let vt_target = vtable(c, r, &[0x20]);
        let vt_rider = vtable(c, r, &[0x38]);
        c.bus.write32(DATA + 0x30, DATA + 0x100);
        c.bus.write32(DATA + 0x100, vt_target);
        c.bus.write32(DATA + 0x104, DATA + 0x200);
        c.bus.write32(DATA + 0x200 + 0x6c0, vt_rider);
        c.bus.write32(DATA + 0x10, DATA + 0x300);
        c.bus.write16(DATA + 0x300 + 0x28, 0);
        c.bus.write32(DATA + 0x300 + 0x2c, FAKE);
        c.bus.write32(
            LOG_RESULTS,
            if equal {
                rider as u32
            } else {
                rider as u32 ^ 1
            },
        );
        // ResetForRider's own current-id check answers a different id, so it
        // bails out before touching the rider.
        c.bus.write32(LOG_RESULTS + 4, 0x55);
    }

    macro_rules! camera_test {
        ($name:ident, $generated:path, $hand:path, $salt:expr) => {
            #[test]
            fn $name() {
                check($generated, $hand, 40, &|c, seed| {
                    let mut r = machine(c, seed, $salt, 4);
                    camera_case(c, &mut r, seed % 2 == 0, 3);
                });
            }
        };
    }

    camera_test!(
        mid_left_chase_camera_reset_for_rider_matches,
        ee::s02::f_00168f60__generated,
        mid_left_chase_camera_reset_for_rider,
        0x168f60
    );
    camera_test!(
        default_2_chase_camera_reset_for_rider_matches,
        ee::s02::f_00176ce0__generated,
        default_2_chase_camera_reset_for_rider,
        0x176ce0
    );
    camera_test!(
        default_3_chase_camera_reset_for_rider_matches,
        ee::s02::f_00176fe0__generated,
        default_3_chase_camera_reset_for_rider,
        0x176fe0
    );
    camera_test!(
        default_4_chase_camera_reset_for_rider_matches,
        ee::s02::f_001772e0__generated,
        default_4_chase_camera_reset_for_rider,
        0x1772e0
    );
    camera_test!(
        direction_ne_chase_camera_reset_for_rider_matches,
        ee::s02::f_00177588__generated,
        direction_ne_chase_camera_reset_for_rider,
        0x177588
    );
    camera_test!(
        post_race_1_chase_camera_reset_for_rider_matches,
        ee::s02::f_00178e90__generated,
        post_race_1_chase_camera_reset_for_rider,
        0x178e90
    );

    #[test]
    fn memitter_reset_particles_matches() {
        check(
            ee::s12::f_00371600__generated,
            memitter_reset_particles,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x371600, 0);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x178, seed % 4);
                c.bus.write32(DATA + 0x204, DATA + 0x2000);
                c.bus.write32(DATA + 0x1a0, DATA + 0x3000);
                c.bus.write32(DATA + 0x1a4, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn ps2_graphics_man_emit_model_lighting_matches() {
        check(
            ee::s12::f_0037e098__generated,
            ps2_graphics_man_emit_model_lighting,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x37e098, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = sx(DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn mc_manager_state14_matches() {
        check(ee::s06::f_0023f150__generated, mc_manager_state14, 40, &|c, seed| {
            let _r = machine(c, seed, 0x23f150, 4);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 0x434, DATA + 0x1000);
            c.bus.write32(DATA + 0x1000, DATA + 0x1100);
            c.bus.write16(DATA + 0x1100 + 0x58, 0);
            c.bus.write32(DATA + 0x84, FAKE);
            c.bus.write32(LOG_RESULTS, 0x11);
            match seed % 4 {
                // Card absent, no pending operation: report the completion.
                0 => {
                    c.bus.write32(DATA + 0x1100 + 0x5c, 0x002d_1bd8);
                    c.bus.write32(DATA + 0x6c, 0x002d_1bd8);
                    c.bus.write32(GAME_GP + 0x16c8, 0);
                }
                // Card absent, pending operation: end it.
                2 => {
                    c.bus.write32(DATA + 0x1100 + 0x5c, 0x002d_1bd8);
                    c.bus.write32(DATA + 0x6c, FAKE);
                    c.bus.write32(GAME_GP + 0x16c8, 0);
                }
                // Card present: the poll returns nonzero and the state exits.
                _ => {
                    c.bus.write32(DATA + 0x1100 + 0x5c, FAKE);
                    c.bus.write32(DATA + 0x6c, FAKE);
                }
            }
        });
    }

    fn c_ui_state_buy_popup_ctor_matches() {
        check(
            ee::s04::f_001caba8__generated,
            c_ui_state_buy_popup_ctor,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x1caba8, 2);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x200);
                // The owner's first word is the FE module (or absent).
                c.bus
                    .write32(DATA + 0x100, if seed % 3 == 0 { 0 } else { DATA + 0x1800 });
            },
        );
    }

    #[test]
    fn camera_trigger_read_action_type2_matches() {
        check(
            ee::s02::f_001715f8__generated,
            camera_trigger_read_action_type2,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1715f8, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA, DATA + 0x100);
                for k in 0..20u32 {
                    c.bus.write8(DATA + 0x100 + k, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn wscript_man_collect_set_mark_item_matches() {
        check(
            ee::s10::f_0030c3e0__generated,
            wscript_man_collect_set_mark_item,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30c3e0, 3);
                be_library(c, &mut r);
                c.bus.write32(LOG_RESULTS, DATA + 0x2100);
                c.g[A0][0] = sx(DATA);
                let n = 1 + seed % 3;
                c.bus.write32(DATA + 8, n);
                for i in 0..3u32 {
                    c.bus
                        .write32(DATA + 0xc + 4 * i, if i + 1 == n { 7 } else { 100 + i });
                }
                c.g[A1][0] = if seed % 2 == 0 { 7 } else { 200 };
            },
        );
    }

    #[test]
    fn rider_model_ctor_matches() {
        check(ee::s10::f_0030d4b8__generated, rider_model_ctor, 30, &|c, seed| {
            let _r = machine(c, seed, 0x30d4b8, 0);
            c.g[A0][0] = sx(DATA);
        });
    }

        #[test]
    fn rider_update_from_script_matches() {
        check(
            ee::s00::f_00123da8__generated,
            rider_update_from_script,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x123da8, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A3][0] = (seed % 5) as u64;
                // No script state: the body returns after the prologue.
                c.bus.write32(DATA + 0xac4, 0);
            },
        );
    }

    #[test]
    fn snd_bank_man_init_load_request_matches() {
        check(
            ee::s07::f_0028b930__generated,
            snd_bank_man_init_load_request,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x28b930, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.g[A3][0] = sx(DATA + 0x500);
                put_str(c, DATA + 0x500, b"bank");
                let m = DATA + 0x1000;
                c.bus.write32(GAME_GP + 0x524, m);
                c.bus.write32(m + 0x1d8, DATA + 0x1010);
                c.bus.write32(DATA + 0x1010, DATA + 0x1020);
                c.bus.write32(DATA + 0x1020, DATA + 0x1030);
                c.bus.write16(DATA + 0x1030 + 0x38, 0);
                c.bus.write32(DATA + 0x1030 + 0x3c, FAKE);
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_event_select_exit_matches() {
        check(
            ee::s07::f_0028f678__generated,
            ssx_audio_system_on_event_select_exit,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28f678, 3);
                be_library(c, &mut r);
                c.bus.write32(LOG_RESULTS, DATA + 0x2100);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(DATA + 0x6088, (seed % 3 == 0) as u32);
                // `+0x62a8` < 0 takes the short path; otherwise the loading-screen
                // loop runs (and its own guard is the same field).
                c.bus
                    .write32(DATA + 0x62a8, if seed % 4 == 0 { 0xffff_ffff } else { 0 });
                c.bus.write32(DATA + 0x118, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, 0);
            },
        );
    }

    /// The engine's size-class pool allocator (see `oc57`): short free lists
    /// behind `gp + 0x2a60..0x2a84`, enabled by the flag at `gp + 0xdd8`.
    fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }

    #[test]
    fn mem_card_state_base_play_widget_anim_matches() {
        check(
            ee::s04::f_001d93e8__generated,
            mem_card_state_base_play_widget_anim,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d93e8, 2);
                put_str(c, DATA + 0x400, b"widget");
                let h = str_hash(c, DATA + 0x400);
                let screen = DATA + 0x1000;
                screen_with(c, &mut r, screen, h);
                // The animation set at `*(self+0x10)+0x50` has no entries, so
                // CreateAnim answers 0.
                c.bus.write32(DATA + 0x10, DATA + 0x2000);
                c.bus.write32(DATA + 0x2050, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x500);
                c.bus
                    .write32(DATA + 0x40, if seed % 4 == 0 { 0 } else { screen });
            },
        );
    }

        #[test]
    fn rider_v15_matches() {
        check(ee::s00::f_0010e830__generated, rider_v15, 40, &|c, seed| {
            let _r = machine(c, seed, 0x10e830, 3);
            c.g[A0][0] = sx(DATA);
            c.f[12] = 1.0f32.to_bits();
            // The rider base and its main state (1 = Air, 4 = Railslide).
            c.bus.write32(DATA + 0x77c, DATA + 0x1000);
            let state = [0u32, 1, 4, 2][seed as usize % 4];
            c.bus.write32(DATA + 0x1000 + 0xde0, state);
            c.bus.write32(DATA + 0x790, DATA + 0x2000);
        });
    }

    #[test]
    fn be_reward_int_new_rewards_ctor_matches() {
        check(
            ee::s01::f_0015a488__generated,
            be_reward_int_new_rewards_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x15a488, 1);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn c_dynamic_spray_fx_setup_emitters_matches() {
        check(
            ee::s09::f_002df448__generated,
            c_dynamic_spray_fx_setup_emitters,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x2df448, 2);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x28, DATA + 0x1000);
                c.bus.write32(DATA + 0x2c, DATA + 0x2000);
                // Each of the nine records has an emitter object with a vtable slot
                // at `+0x1f8`; point them all at one fake-slot table.
                for i in 0..9u32 {
                    let rec = DATA + 0x1000 + 0x210 * i;
                    c.bus.write32(rec + 0x1f8, DATA + 0x3000);
                }
                c.bus.write16(DATA + 0x3000 + 0x18, 0);
                c.bus.write32(DATA + 0x3000 + 0x1c, FAKE);
            },
        );
    }

    #[test]
    fn snow_fall_dtor_matches() {
        check(ee::s09::f_002e5d18__generated, snow_fall_dtor, 40, &|c, seed| {
            let _r = machine(c, seed, 0x2e5d18, 4);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = 0;
            for i in 0..11u32 {
                c.bus.write32(
                    DATA + 0x10 + 4 * i,
                    if seed % 3 == 0 && i == 2 {
                        DATA + 0x2000 + 0x100 * i
                    } else {
                        0
                    },
                );
            }
        });
    }

    #[test]
    fn game_mode_man_sort_by_score_matches() {
        check(
            ee::s06::f_00238b70__generated,
            game_mode_man_sort_by_score,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x238b70, 2);
                c.g[A1][0] = (1 + seed % 5) as u64; // the entry count handed to qsort
                                                    // The score table lives in RAM at fixed addresses; give it values.
                for i in 0..10u32 {
                    c.bus.write32(0x0053_6708 + 4 * i, (seed * 7 + i) % 13);
                    c.bus.write32(0x0053_6730 + 4 * i, (seed * 5 + i) % 11);
                }
            },
        );
    }

    #[test]
    fn game_mode_man_sort_by_time_matches() {
        check(
            ee::s06::f_00238bf8__generated,
            game_mode_man_sort_by_time,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x238bf8, 2);
                c.g[A1][0] = (1 + seed % 5) as u64; // the entry count handed to qsort
                for i in 0..10u32 {
                    c.bus.write32(0x0053_6708 + 4 * i, (seed * 3 + i) % 17);
                    c.bus.write32(0x0053_6730 + 4 * i, (seed * 11 + i) % 19);
                }
            },
        );
    }

    /// `gp-0x854` -> a dummy FE manager and `gp-0x848` -> a dummy world, with
    /// the slots the render helpers read pointed at the fake.
    fn fe_render_fixture(c: &mut Cpu, r: &mut Rng) -> u32 {
        let m = DATA + 0x1000;
        let o = DATA + 0x1100;
        c.bus.write32(GAME_GP.wrapping_sub(0x854), m);
        c.bus.write32(m + 0x10d8, o);
        for off in [0x78u32, 0x88, 0xa0, 0xa8, 0xf8] {
            c.bus.write16(o + off, 0);
            c.bus.write32(o + off + 4, FAKE);
        }
        c.bus.write32(m + 0xe84, DATA + 0x1200);
        let w = DATA + 0x1300;
        world(c, w);
        c.bus.write32(w + 0xa4, DATA + 0x1400);
        let _ = r;
        o
    }

    #[test]
    fn ps2_graphics_man_wait_for_render_idle_matches() {
        check(
            ee::s12::f_00369c28__generated,
            ps2_graphics_man_wait_for_render_idle,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x369c28, 4);
                c.pc = 0x0036_9c28;
                let o = fe_render_fixture(c, &mut r);
                c.g[A0][0] = sx(DATA);
                // Busy: the fake answers nonzero and the poll returns at once.
                // Idle: a real zero returner sends the body through the flush
                // block, whose first virtual call unwinds the thread so the
                // poll loop stops after one iteration.
                if seed % 2 == 0 {
                    c.bus.write32(o + 0x94, 0x002d_1bd8);
                    c.bus.write32(GAME_GP + 0x16c8, 0);
                    c.bus.write32(crate::difftest::fakes::LOG_UNWIND_AT, 0);
                } else {
                    c.bus.write32(o + 0x94, FAKE);
                    c.bus.write32(LOG_RESULTS, 0x11);
                }
                c.bus.write32(GAME_GP + 0x2a74, 0);
            },
        );
    }

    fn c_fe_state_trophy_room_show_trophy_info_matches() {
        check(
            ee::s04::f_001d4890__generated,
            c_fe_state_trophy_room_show_trophy_info,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d4890, 3);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 3) as u64;
                let list = DATA + 0x1000;
                let item = DATA + 0x1100;
                let vt = DATA + 0x1200;
                c.bus
                    .write32(DATA + 0x4c, if seed % 4 == 0 { 0 } else { list });
                c.bus.write32(list + 8, item);
                c.bus.write16(vt + 0x48, 0);
                c.bus.write32(vt + 0x4c, FAKE);
                c.bus.write32(DATA + 0x58, 0x1111);
                c.bus.write8(DATA + 0x44, 5);
                c.bus.write32(DATA + 0x50, 0x2222);
                c.bus.write32(DATA + 0x54, 0x3333);
            },
        );
    }

    #[test]
    fn tag_field_set_token_matches() {
        check(ee::s14::f_003ec0f8__generated, tag_field_set_token, 40, &|c, seed| {
            let _r = machine(c, seed, 0x3ec0f8, 2);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A3][0] = [0u64, 1, 0x414243, u32::MAX as u64][seed as usize % 4];
        });
    }

    macro_rules! render_test {
        ($name:ident, $generated:path, $hand:path, $salt:expr) => {
            #[test]
            fn $name() {
                check($generated, $hand, 40, &|c, seed| {
                    let mut r = machine(c, seed, $salt, 4);
                    fe_render_fixture(c, &mut r);
                    // Active screen only every fourth seed.
                    c.bus
                        .write32(LOG_RESULTS, if seed % 4 == 0 { 0x11 } else { 0 });
                });
            }
        };
    }

    render_test!(
        pre_start_screen_render_matches,
        ee::s06::f_00232488__generated,
        pre_start_screen_render,
        0x232488
    );
    render_test!(
        init_load_screen_render_matches,
        ee::s06::f_00232650__generated,
        init_load_screen_render,
        0x232650
    );
    render_test!(
        c_pre_game_load_screen_render_matches,
        ee::s06::f_00232820__generated,
        c_pre_game_load_screen_render,
        0x232820
    );
    render_test!(
        c_pre_fe_load_screen_render_matches,
        ee::s06::f_002328a8__generated,
        c_pre_fe_load_screen_render,
        0x2328a8
    );

    #[test]
    fn c_ui_text_scroll_add_line_matches() {
        check(
            ee::s13::f_003a1bf8__generated,
            c_ui_text_scroll_add_line,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a1bf8, 3);
                pool_allocator(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                // A short UTF-16 line.
                let n = 1 + seed % 4;
                for i in 0..n {
                    c.bus.write16(DATA + 0x400 + 2 * i, b'a' as u16 + i as u16);
                }
                c.bus.write16(DATA + 0x400 + 2 * n, 0);
                // An empty line table: GrowLines allocates it.
                c.bus.write32(DATA + 0xbc, 0);
                c.bus.write16(DATA + 0xc8, 0);
            },
        );
    }

    #[test]
    fn c_ui_group_find_by_id_matches() {
        check(
            ee::s13::f_003986b0__generated,
            c_ui_group_find_by_id,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3986b0, 2);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 5) as u64;
                // The group's sub-list at `+0x74` is empty.
                c.bus.write32(DATA + 0x74, 0);
                let node = DATA + 0x1000;
                let vt = vtable(c, &mut r, &[0x70]);
                c.bus.write32(node + 8, vt);
                c.bus.write32(node + 4, DATA + 0x1100);
                c.bus
                    .write32(DATA + 0x74, if seed % 3 == 0 { 0 } else { DATA + 0x1200 });
            },
        );
    }

    #[test]
    fn event_select_set_map_tab_mode_matches() {
        check(
            ee::s05::f_00203cb8__generated,
            event_select_set_map_tab_mode,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x203cb8, 4);
                let screen = DATA + 0x1000;
                let node = screen_with(c, &mut r, screen, 0);
                c.bus.write32(LOG_RESULTS, 0x11);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus
                    .write32(DATA + 0x2e8, if seed % 4 == 0 { 0 } else { screen });
                let _ = node;
            },
        );
    }

        #[test]
    fn c_ov_state_overlay_play_object_anim_matches() {
        check(
            ee::s05::f_0020a6f0__generated,
            c_ov_state_overlay_play_object_anim,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20a6f0, 2);
                put_str(c, DATA + 0x500, b"overlay");
                let h = str_hash(c, DATA + 0x500);
                let screen = DATA + 0x1000;
                screen_with(c, &mut r, screen, h);
                c.bus.write32(DATA + 0x10, DATA + 0x2000);
                c.bus.write32(DATA + 0x2050, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = sx(DATA + 0x600);
                c.bus
                    .write32(DATA + 0x40, if seed % 4 == 0 { 0 } else { screen });
            },
        );
    }

    #[test]
    fn bebagt_int_set_uber_trick_button_matches() {
        check(
            ee::s01::f_0014fd80__generated,
            bebagt_int_set_uber_trick_button,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x14fd80, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 20) as u64;
                c.g[A2][0] = sx(DATA);
                c.g[A3][0] = (seed % 256) as u64;
                // The race setup table: player 0 in slot 0, whose record has a
                // profile word of 0 and character 0.
                c.bus.write32(0x0053_05b0, 0);
                c.bus.write32(0x0053_5b20 + 0xc, 0);
                c.bus.write8(0x0053_5b20 + 0x11, 0);
            },
        );
    }

    #[test]
    fn section_man_finish_event_switch_matches() {
        check(
            ee::s05::f_0022dff0__generated,
            section_man_finish_event_switch,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22dff0, 4);
                be_library(c, &mut r);
                c.bus.write32(LOG_RESULTS, 0x11);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x1cc, (seed % 2) as u32);
                c.bus.write32(DATA + 0x1b0, 0);
                // A minimal world -> race manager -> object with a `+0xcc` slot.
                let w = DATA + 0x1000;
                world(c, w);
                c.bus.write32(w + 0x84, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100 + 0xc, DATA + 0x1200);
                c.bus.write32(DATA + 0x1200 + 0xcc, DATA + 0x1300);
                c.bus.write16(DATA + 0x1300 + 0x28, 0);
                c.bus.write32(DATA + 0x1300 + 0x2c, FAKE);
                // The track info's first word (a track id), and the lighting table
                // entry so the dispatcher takes a real arm.
                c.bus.write32(0x0043_d950, 0);
                c.bus.write32(0x0047_b4e0, 0x0022_e1ac);
            },
        );
    }

    #[test]
    fn c_stream_man_queue_read_matches() {
        check(
            ee::s13::f_003a6f88__generated,
            c_stream_man_queue_read,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a6f88, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 0x1000) as u64;
                c.g[A2][0] = (seed % 0x800) as u64;
                c.g[A3][0] = 0x444e4543;
                c.bus.write32(DATA + 0x90, (seed % 2) as u32);
                c.bus.write32(DATA + 0x88, DATA + 0x1000);
            },
        );
    }

    #[test]
    fn ui_anim_set_create_anim_matches() {
        check(
            ee::s13::f_00397140__generated,
            ui_anim_set_create_anim,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x397140, 3);
                pool_allocator(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 6) as u64;
                // An empty animation set.
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus
                    .write32(DATA + 0x1000, if seed % 3 == 0 { 0 } else { 4 });
                if seed % 3 != 0 {
                    // A single entry with the sought id and stride 0x14.
                    c.bus.write32(DATA + 0x1004 + 8, seed % 6);
                    c.bus.write32(DATA + 0x1004 + 0xc, 0x14);
                    c.bus.write32(DATA + 0x1004 + 4, 0);
                }
            },
        );
    }

    #[test]
    fn fe_async_manager_request_peak_map_matches() {
        check(
            ee::s03::f_001a3768__generated,
            fe_async_manager_request_peak_map,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x1a3768, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 0x40) as u64;
                c.g[A2][0] = (seed % 0x100) as u64;
                c.g[A3][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_net_messenger_set_row_name_matches() {
        check(
            ee::s03::f_001a6618__generated,
            c_fe_state_net_messenger_set_row_name,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a6618, 3);
                put_str(c, DATA + 0x400, b"row");
                let h = str_hash(c, DATA + 0x400);
                let screen = DATA + 0x1000;
                screen_with(c, &mut r, screen, h);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 0x12) as u64;
                c.g[A2][0] = sx(DATA + 0x400);
                c.g[A3][0] = (seed % 0x100) as u64;
                c.bus
                    .write32(DATA + 0x40, if seed % 4 == 0 { 0 } else { screen });
            },
        );
    }

    #[test]
    fn net_fe_state_show_popup_matches() {
        check(
            ee::s03::f_001a9bc0__generated,
            net_fe_state_show_popup,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a9bc0, 3);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                let screen = DATA + 0x2000;
                c.bus.write32(DATA + 0x40, screen);
                c.bus.write32(screen + 0xd0, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 0x10, DATA + 0x2200);
                c.bus.write32(DATA + 0x2200 + 0x18, 0);
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(
                    LOG_RESULTS,
                    if seed % 3 == 0 {
                        DATA + 0x2200
                    } else {
                        DATA + 0x2300
                    },
                );
                c.bus.write32(DATA + 0x1000 + 0x2b8, (seed % 2) as u32);
                c.bus.write32(DATA + 0x1000 + 0x1c, 0x1234);
                c.bus.write32(DATA + 0x6a0, 0);
                let _ = vt;
            },
        );
    }

    #[test]
    fn c_fe_state_create_step1_is_email_format_valid_matches() {
        check(
            ee::s03::f_001b9cd8__generated,
            c_fe_state_create_step1_is_email_format_valid,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1b9cd8, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                let n = seed % 8;
                for i in 0..n {
                    let ch = match (seed + i) % 5 {
                        0 => b'@',
                        1 => b'.',
                        2 => b'a',
                        3 => b'1',
                        _ => b'x',
                    };
                    c.bus.write8(DATA + 0x400 + i, ch);
                }
                c.bus.write8(DATA + 0x400 + n, 0);
            },
        );
    }

    #[test]
    fn c_fe_state_create_step2_set_year_selection_matches() {
        check(
            ee::s03::f_001bb3b0__generated,
            c_fe_state_create_step2_set_year_selection,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1bb3b0, 3);
                let screen = DATA + 0x1000;
                let node = screen_with(c, &mut r, screen, 0);
                c.bus.write32(LOG_RESULTS, 0x11);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x40, if seed % 4 == 0 { 0 } else { screen });
                // `*(self+0x6d0)+0x30` holds the year index.
                c.bus.write32(DATA + 0x6d0, DATA + 0x2000);
                c.bus.write8(DATA + 0x2000 + 0x30, 0x6c + (seed % 8) as u8);
                let _ = node;
            },
        );
    }

    #[test]
    fn tex_man_ctor_matches() {
        check(ee::s12::f_00366fe8__generated, tex_man_ctor, 20, &|c, seed| {
            let _r = machine(c, seed, 0x366fe8, 0);
            c.g[A0][0] = sx(DATA);
            // The two VramHeap base pointers `VramHeap_Init` reads.
            c.bus.write32(DATA + 0x3f50, DATA + 0x8000);
            c.bus.write32(DATA + 0x6340, DATA + 0x9000);
        });
    }

    #[test]
    fn lobby_man_peek_room_matches() {
        check(ee::s06::f_0025d770__generated, lobby_man_peek_room, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x25d770, 2);
            be_library(c, &mut r);
            c.bus.write32(LOG_RESULTS, 0x11);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 3 == 0 {
                u64::MAX
            } else {
                (seed % 8) as u64
            };
            c.bus.write32(DATA, if seed % 4 == 0 { 0 } else { 1 });
            c.bus.write32(DATA + 0x54, DATA + 0x1000);
        });
    }

    #[test]
    fn buddy_man_is_online_at_matches() {
        check(
            ee::s07::f_00263338__generated,
            buddy_man_is_online_at,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x263338, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 4 == 0 {
                    u64::MAX
                } else {
                    (seed % 4) as u64
                };
                // An empty roster: count 0, no entries.
                c.bus
                    .write32(DATA + 0xcc, if seed % 3 == 0 { 0 } else { DATA + 0x1000 });
                c.bus.write32(DATA + 0x1000, 0);
            },
        );
    }

    #[test]
    fn c_setup_man_set_local_character_matches() {
        check(
            ee::s07::f_00264cf0__generated,
            c_setup_man_set_local_character,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x264cf0, 4);
                let iface = be_library(c, &mut r);
                c.bus.write32(LOG_RESULTS, DATA + 0x3000);
                c.bus.write32(LOG_RESULTS + 4, DATA + 0x4000);
                c.bus.write16(DATA + 0x3000 + 0xc, 0);
                c.bus.write32(DATA + 0x3000 + 0xc, DATA + 0x3100);
                c.bus.write16(DATA + 0x3100 + 8, 0);
                c.bus.write32(DATA + 0x3100 + 0xc, FAKE);
                c.bus.write32(DATA + 0x4000 + 0xc, DATA + 0x4100);
                c.bus.write16(DATA + 0x4100 + 8, 0);
                c.bus.write32(DATA + 0x4100 + 0xc, FAKE);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 16) as u64;
                let _ = iface;
                // `gp-0x848` -> a world whose `+0x78` is the interface slot.
                let w = DATA + 0x5000;
                world(c, w);
                c.bus.write32(w + 0x78, DATA + 0x5100);
            },
        );
    }
}
