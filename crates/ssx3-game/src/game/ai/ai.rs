//! `src/ai/ai.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_fcc;
use crate::abi::set_fpr;
use crate::abi::set_reg;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::rider::rate_step_60;
pub(crate) use crate::game::camera::camera::gp;
pub(crate) use crate::game::camera::camera::gp_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_5;
pub(crate) use crate::game::dirtysock::tags2::gp_8 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_2;
pub(crate) use crate::game::fe::feasyncfile::gp_12 as gp_4;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp_5;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_6;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw;
pub(crate) use crate::game::fe::feasyncfile::sw_6 as sw_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_7;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::fe::splashmanager::gpo;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::gp as gp_3;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_3b48, ai_select_race_state, "AI_SelectRaceState"),
    (
        0x0011_75b8,
        rider_metrix_reset_stats,
        "RiderMetrix_ResetStats",
    ),
    (0x0012_87b8, ai_dtor, "AI_dtor"),
    (
        0x0012_8958,
        ai_setup_computer_riders_2,
        "AI_SetupComputerRiders",
    ),
    (0x0012_8998, ai_setup_actor_riders, "AI_SetupActorRiders"),
    (
        0x0012_8a10,
        ai_reset_riders_and_order,
        "AI_ResetRidersAndOrder",
    ),
    (0x0012_9160, ai_update_rider_poses, "AI_UpdateRiderPoses"),
    (0x0012_96f8, ai_load_next_rider, "AI_LoadNextRider"),
    (0x0012_9768, ai_restart, "AI_Restart"),
    (0x0012_97c8, ai_reset_riders, "AI_ResetRiders"),
    (
        0x0012_a490,
        ai_path_data_init_if_early_section,
        "AI_PathDataInitIfEarlySection",
    ),
    (
        0x0012_ab20,
        ai_reset_computer_render_slots,
        "AI_ResetComputerRenderSlots",
    ),
    (
        0x0012_abd0,
        ai_computer_riders_have_render_slots,
        "AI_ComputerRidersHaveRenderSlots",
    ),
    (
        0x0012_ac48,
        ai_reset_actor_render_slots,
        "AI_ResetActorRenderSlots",
    ),
    (
        0x0012_acf8,
        ai_actor_riders_have_render_slots,
        "AI_ActorRidersHaveRenderSlots",
    ),
    (
        0x0012_ae38,
        ai_destroy_computer_riders,
        "AI_DestroyComputerRiders",
    ),
    (
        0x0012_aed0,
        ai_destroy_actor_riders,
        "AI_DestroyActorRiders",
    ),
    (
        0x0012_af68,
        ai_destroy_mission_riders,
        "AI_DestroyMissionRiders",
    ),
    (0x0012_b000, ai_remove_actor_riders, "AI_RemoveActorRiders"),
    (
        0x0012_b030,
        ai_remove_computer_riders,
        "AI_RemoveComputerRiders",
    ),
    (
        0x0012_b060,
        ai_remove_mission_riders,
        "AI_RemoveMissionRiders",
    ),
    (0x0012_b090, ai_destroy_all_riders, "AI_DestroyAllRiders"),
    (0x0012_b180, ai_reset_rider_stats, "AI_ResetRiderStats"),
    (0x0012_b200, ai_free_anim_sources, "AI_FreeAnimDataSources"),
    (0x0012_b340, ai_force_finish_riders, "AI_ForceFinishRiders"),
    (0x0012_b420, ai_get_sync_checksum, "AI_GetSyncChecksum"),
    (
        0x0012_b698,
        ai_apply_computer_rider_settings,
        "AI_ApplyComputerRiderSettings",
    ),
    (
        0x0012_b788,
        ai_refresh_rider_overlaps,
        "AI_RefreshRiderOverlaps",
    ),
    (0x0012_bac0, ai_set_sync_words, "AI_SetSyncWords"),
    (0x0012_baf0, ai_get_sync_words, "AI_GetSyncWords"),
    (
        0x0012_c028,
        ridergate_sub_is_start_time_reached,
        "RiderGateSub_IsStartTimeReached",
    ),
    (
        0x0012_c078,
        rider_gate_sub_can_start,
        "RiderGateSub_CanStart",
    ),
    (
        0x0012_c0c0,
        rider_gate_sub_update_wait_push,
        "RiderGateSub_UpdateWaitPush",
    ),
    (
        0x0012_c130,
        rider_gate_sub_update_push,
        "RiderGateSub_UpdatePush",
    ),
    (
        0x0012_c408,
        rider_gate_sub_update_launch,
        "RiderGateSub_UpdateLaunch",
    ),
    (
        0x0012_c590,
        rider_gate_sub_load_state,
        "RiderGateSub_LoadState",
    ),
    (
        0x0012_c5c8,
        rider_finish_line_sub_enter,
        "RiderFinishLineSub_Enter",
    ),
    (
        0x0012_c9f8,
        rider_finish_line_sub_load_state,
        "RiderFinishLineSub_LoadState",
    ),
    (
        0x0012_d9d8,
        rider_wipeout_sub_update_phase4,
        "RiderWipeoutSub_UpdatePhase4",
    ),
    (
        0x0012_da88,
        rider_wipeout_sub_select_tumble_anim,
        "RiderWipeoutSub_SelectTumbleAnim",
    ),
    (
        0x0012_dd98,
        rider_wipeout_sub_switch_to_anim_set_a,
        "RiderWipeoutSub_SwitchToAnimSetA",
    ),
    (
        0x0012_df48,
        rider_wipeout_sub_play_end_anim_a,
        "RiderWipeoutSub_PlayEndAnimA",
    ),
    (
        0x0012_e740,
        rider_wipeout_sub_load_state,
        "RiderWipeoutSub_LoadState",
    ),
    (0x0012_e9b0, rider_prewind_sub_exit, "RiderPrewindSub_Exit"),
    (0x0012_f230, rider_reset_sub_enter, "RiderResetSub_Enter"),
    (
        0x0012_f588,
        rider_reset_sub_is_local_player,
        "RiderResetSub_IsLocalPlayer",
    ),
    (
        0x0012_f5e8,
        rider_reset_sub_load_state,
        "RiderResetSub_LoadState",
    ),
    (
        0x0012_fc28,
        rider_natural_air_sub_load_state,
        "RiderNaturalAirSub_LoadState",
    ),
    (
        0x0012_fc60,
        rider_board_press_sub_enter,
        "RiderBoardPressSub_Enter",
    ),
    (
        0x0012_fec8,
        rider_board_press_sub_flip_stance,
        "RiderBoardPressSub_FlipStance",
    ),
    (
        0x0013_04d0,
        rider_board_press_sub_update_phase2,
        "RiderBoardPressSub_UpdatePhase2",
    ),
    (
        0x0013_1200,
        rider_board_press_sub_check_release,
        "RiderBoardPressSub_CheckRelease",
    ),
    (
        0x0013_1348,
        rider_board_press_sub_cancel,
        "RiderBoardPressSub_Cancel",
    ),
    (
        0x0013_13a8,
        rider_board_press_sub_settle_slew,
        "RiderBoardPressSub_SettleSlew",
    ),
    (
        0x0013_15d0,
        rider_board_press_sub_load_state,
        "RiderBoardPressSub_LoadState",
    ),
    (0x0013_1608, rider_cruise_sub_enter, "RiderCruiseSub_Enter"),
    (
        0x0013_2048,
        rider_railslide_sub_exit,
        "RiderRailslideSub_Exit",
    ),
    (
        0x0013_2620,
        rider_railslide_sub_set_rail,
        "RiderRailslideSub_SetRail",
    ),
    (
        0x0013_2878,
        rider_railslide_sub_load_state,
        "RiderRailslideSub_LoadState",
    ),
    (
        0x0013_2f98,
        rider_handplant_sub_exit,
        "RiderHandplantSub_Exit",
    ),
    (
        0x0013_30f0,
        rider_handplant_sub_load_state,
        "RiderHandplantSub_LoadState",
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
    (
        0x0013_5f70,
        rider_spin_sub_get_hud_dir_angle,
        "RiderSpinSub_GetHUDDirAngle",
    ),
    (
        0x0013_6100,
        rider_spin_sub_get_spin_hud_blend,
        "RiderSpinSub_GetSpinHUDBlend",
    ),
    (
        0x0013_6230,
        rider_spin_sub_load_state,
        "RiderSpinSub_LoadState",
    ),
    (
        0x0013_6920,
        rider_uber_grind_sub_load_state,
        "RiderUberGrindSub_LoadState",
    ),
    (
        0x0013_3308,
        rider_spin_sub_handle_input,
        "RiderSpinSub_HandleInput",
    ),
    (
        0x0013_4dd0,
        rider_spin_sub_apply_pose_rotation,
        "RiderSpinSub_ApplyPoseRotation",
    ),
    (0x0012_91e0, ai_render, "AI_Render"),
    (
        0x0012_e9b8,
        rider_prewind_sub_handle_input,
        "RiderPrewindSub_HandleInput",
    ),
    (
        0x0013_1620,
        rider_cruise_sub_handle_input,
        "RiderCruiseSub_HandleInput",
    ),
    (
        0x0013_52a8,
        rider_spin_sub_update_trick,
        "RiderSpinSub_UpdateTrick",
    ),
    (
        0x0012_bb20,
        ai_update_race_time_limit,
        "AI_UpdateRaceTimeLimit",
    ),
    (
        0x0012_ee30,
        rider_prewind_sub_play_directional_anim,
        "RiderPrewindSub_PlayDirectionalAnim",
    ),
    (
        0x0013_5180,
        rider_spin_sub_get_spin_factor,
        "RiderSpinSub_GetSpinFactor",
    ),
    (
        0x0012_f118,
        rider_prewind_sub_get_hud_angle,
        "RiderPrewindSub_GetHUDAngle",
    ),
    (
        0x0013_5cb0,
        rider_spin_sub_get_flip_progress,
        "RiderSpinSub_GetFlipProgress",
    ),
    (
        0x0013_5db0,
        rider_spin_sub_get_spin_progress,
        "RiderSpinSub_GetSpinProgress",
    ),
    (
        0x0012_cb68,
        rider_wipeout_sub_handle_input,
        "RiderWipeoutSub_HandleInput",
    ),
    (
        0x0012_cd20,
        rider_wipeout_sub_update_phase0,
        "RiderWipeoutSub_UpdatePhase0",
    ),
    (
        0x0012_d848,
        rider_wipeout_sub_update_recover,
        "RiderWipeoutSub_UpdateRecover",
    ),
    (
        0x0012_e528,
        rider_wipeout_sub_update_anim_speed,
        "RiderWipeoutSub_UpdateAnimSpeed",
    ),
    (
        0x0012_e778,
        rider_bump_sub_handle_input,
        "RiderBumpSub_HandleInput",
    ),
    (
        0x0013_5be0,
        rider_spin_sub_try_reverse,
        "RiderSpinSub_TryReverse",
    ),
    (0x0012_b7f0, ai_save_state, "AI_SaveState"),
    (
        0x0012_c558,
        rider_gate_sub_save_state,
        "RiderGateSub_SaveState",
    ),
    (
        0x0012_c9c0,
        rider_finish_line_sub_save_state,
        "RiderFinishLineSub_SaveState",
    ),
    (
        0x0012_e708,
        rider_wipeout_sub_save_state,
        "RiderWipeoutSub_SaveState",
    ),
    (
        0x0012_f5b0,
        rider_reset_sub_save_state,
        "RiderResetSub_SaveState",
    ),
    (
        0x0012_fbf0,
        rider_natural_air_sub_save_state,
        "RiderNaturalAirSub_SaveState",
    ),
    (
        0x0013_1598,
        rider_board_press_sub_save_state,
        "RiderBoardPressSub_SaveState",
    ),
    (
        0x0013_2840,
        rider_railslide_sub_save_state,
        "RiderRailslideSub_SaveState",
    ),
    (
        0x0013_30b8,
        rider_handplant_sub_save_state,
        "RiderHandplantSub_SaveState",
    ),
    (0x0013_3128, rider_spin_sub_enter, "RiderSpinSub_Enter"),
    (0x0013_4cb0, rider_spin_sub_exit, "RiderSpinSub_Exit"),
    (
        0x0013_61f8,
        rider_spin_sub_save_state,
        "RiderSpinSub_SaveState",
    ),
    (
        0x0013_68e8,
        rider_uber_grind_sub_save_state,
        "RiderUberGrindSub_SaveState",
    ),
    (0x0013_1c30, rider_cruise_sub_exit, "RiderCruiseSub_Exit"),
    (
        0x0012_f620,
        rider_natural_air_sub_enter,
        "RiderNaturalAirSub_Enter",
    ),
    (
        0x0013_5b30,
        rider_spin_sub_set_spin_direction,
        "RiderSpinSub_SetSpinDirection",
    ),
];

/// `gp`-relative globals used here.
pub(crate) mod g {}

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::ANIM_GET_LAYER_SEQUENCE;

    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_CTOR;

    pub(crate) use crate::game::dirtysock::tags2::callee::GET_TRANSITION_MAN;

    pub(crate) use crate::game::dirtysock::tags2::callee::IS_LOCAL_PLAYER;

    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_STOP;

    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;

    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;

    pub(crate) use crate::game::fe::fepopup::callee::C00113F88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115AB0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011FE78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011FE98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011FEC8;

    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;

    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;

    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;

    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;

    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;

    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS as RIDER_METRIX_RESET_STATS_2;

    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST as RIDER_ADD_BOOST_2;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;

    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_HAS_RENDER_SLOT;

    /// `IntcHandler_SaveRegs` (0x362120).
    pub const INTC_SAVE_REGS: Callee = Callee {
        addr: 0x0036_2120,
        f: ee::s12::f_00362120,
    };

    /// `IntcHandler_RestoreRegs` (0x362230).
    pub const INTC_RESTORE_REGS: Callee = Callee {
        addr: 0x0036_2230,
        f: ee::s12::f_00362230,
    };

    /// `Irradiance_FindIndex` (0x38ac50).
    pub const IRRADIANCE_FIND_INDEX: Callee = Callee {
        addr: 0x0038_ac50,
        f: ee::s12::f_0038ac50,
    };

    /// `Irradiance_GetEntry` (0x38abf8).
    pub const IRRADIANCE_GET_ENTRY: Callee = Callee {
        addr: 0x0038_abf8,
        f: ee::s12::f_0038abf8,
    };

    /// `cWorldCache_Free` (0x3a7e98).
    pub const WORLD_CACHE_FREE: Callee = Callee {
        addr: 0x003a_7e98,
        f: ee::s13::f_003a7e98,
    };

    /// `MPEG2_Flush_Buffer` (0x3b1290).
    pub const MPEG2_FLUSH_BUFFER: Callee = Callee {
        addr: 0x003b_1290,
        f: ee::s13::f_003b1290,
    };

    /// `MPEG2_Show_Bits` (0x3b1258).
    pub const MPEG2_SHOW_BITS: Callee = Callee {
        addr: 0x003b_1258,
        f: ee::s13::f_003b1258,
    };

    /// `MPEG2_Flush_Buffer32` (0x3b1340).
    pub const MPEG2_FLUSH_BUFFER32: Callee = Callee {
        addr: 0x003b_1340,
        f: ee::s13::f_003b1340,
    };

    /// `AI_ResetRiders` (0x1297c8).
    pub const AI_RESET_RIDERS: Callee = Callee {
        addr: 0x0012_97c8,
        f: ee::s00::f_001297c8,
    };

    /// `AI_SetRiderOrderMode` (0x128a48).
    pub const AI_SET_RIDER_ORDER_MODE: Callee = Callee {
        addr: 0x0012_8a48,
        f: ee::s00::f_00128a48,
    };

    /// `Rider_FindNearbyLights` (0x122088).
    pub const RIDER_FIND_NEARBY_LIGHTS: Callee = Callee {
        addr: 0x0012_2088,
        f: ee::s00::f_00122088,
    };

    /// `BE_CopyProfileCharacters` (0x156858).
    pub const BE_COPY_PROFILE_CHARACTERS: Callee = Callee {
        addr: 0x0015_6858,
        f: ee::s01::f_00156858,
    };

    pub const EA_FILE_FREE_FILE_HANDLE: Callee = Callee {
        addr: 0x003d_e8c0,
        f: ee::s14::EAFile_FreeFileHandle,
    };

    pub const EA_FILE_FREE_REQUEST: Callee = Callee {
        addr: 0x003d_e7b0,
        f: ee::s14::EAFile_FreeRequest,
    };

    pub const SPCH_FIND_BANK_TYPE: Callee = Callee {
        addr: 0x003d_8330,
        f: ee::s14::SPCH_FindBankType,
    };

    pub const RIDER_WIPEOUT_UPDATE_ANIM_SPEED: Callee = Callee {
        addr: 0x0012_e528,
        f: ee::s00::RiderWipeoutSub_UpdateAnimSpeed,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE00: Callee = Callee {
        addr: 0x0010_4ca0,
        f: ee::s00::RiderAnim_SeqEndType00,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE01: Callee = Callee {
        addr: 0x0010_4a40,
        f: ee::s00::RiderAnim_SeqEndType01,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE02: Callee = Callee {
        addr: 0x0010_4a60,
        f: ee::s00::RiderAnim_SeqEndType02,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE03: Callee = Callee {
        addr: 0x0010_4b78,
        f: ee::s00::RiderAnim_SeqEndType03,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE04: Callee = Callee {
        addr: 0x0010_4c18,
        f: ee::s00::RiderAnim_SeqEndType04,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE05: Callee = Callee {
        addr: 0x0010_4b48,
        f: ee::s00::RiderAnim_SeqEndType05,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE06: Callee = Callee {
        addr: 0x0010_4c38,
        f: ee::s00::RiderAnim_SeqEndType06,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE07: Callee = Callee {
        addr: 0x0010_4b98,
        f: ee::s00::RiderAnim_SeqEndType07,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE08: Callee = Callee {
        addr: 0x0010_4bb8,
        f: ee::s00::RiderAnim_SeqEndType08,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE09: Callee = Callee {
        addr: 0x0010_4bd8,
        f: ee::s00::RiderAnim_SeqEndType09,
    };

    pub const RIDER_ANIM_SEQ_END_TYPE0A: Callee = Callee {
        addr: 0x0010_4c80,
        f: ee::s00::RiderAnim_SeqEndType0A,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {}

/// `bool RiderResetSub_IsLocalPlayer(RiderResetSub *self)` (0x12f588): false
/// unless the object (+0x8 -> +0x87c) is present, else whether the player id at
/// +0x870 is non-negative.
pub fn rider_reset_sub_is_local_player(c: &mut Cpu) {
    c.cycles += 4;
    let obj = Addr(abi::arg(c, 0));
    let handle = obj.off(0x8).lw(c);
    c.g[V1][0] = handle;
    let present = Addr(handle as u32).off(0x87c).lw(c);
    let mut result = 0u64;
    if present != 0 {
        c.cycles += 3;
        let id = Addr(handle as u32).off(0x870).lw(c);
        result = ((id as i64) >= 0) as u64;
    }
    c.cycles += 2;
    c.g[A0][0] = result;
    c.g[V0][0] = result;
    abi::ret(c);
}

/// `void RiderBoardPressSub_UpdatePhase2(RiderBoardPressSub *self)`
/// (0x1304d0): set the phase field at +0 to 1.
pub fn rider_board_press_sub_update_phase2(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    this.sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Rider sub-states.

/// `void RiderBoardPressSub_Enter(RiderBoardPressSub *self)` (0x12fc60):
/// reset the sub-state and its rider's board-press words.
pub fn rider_board_press_sub_enter(c: &mut Cpu) {
    c.cycles += 8;
    let this = Addr(abi::arg(c, 0));
    let rider = this.off(0x14).lw(c);
    c.g[V0][0] = rider;
    this.sw(c, 0);
    Addr((rider as u32).wrapping_add(0x360)).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `void RiderCruiseSub_Enter(RiderCruiseSub *self)` (0x131608): clear the
/// two board-press words of the rider referenced at +0.
pub fn rider_cruise_sub_enter(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    let handle = this.lw(c);
    c.g[V0][0] = handle;
    Addr((handle as u32).wrapping_add(0x35c)).sw(c, 0);
    let handle = this.lw(c);
    c.g[V1][0] = handle;
    Addr((handle as u32).wrapping_add(0x360)).sw(c, 0);
    abi::ret(c);
}

/// `void RiderRailslideSub_Exit(RiderRailslideSub *self)` (0x132048): clear
/// the railslide word and restore its pitch constant.
pub fn rider_railslide_sub_exit(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    let rider = this.off(0x8).lw(c);
    c.g[V0][0] = rider;
    let pitch = Addr((c.g[GP][0] as u32).wrapping_sub(0x7594)).read_fpu(c);
    c.f[0] = pitch.0;
    Addr((rider as u32).wrapping_add(0x240)).sw(c, 0);
    Addr((rider as u32).wrapping_add(0x23c)).write_fpu(c, pitch);
    abi::ret(c);
}

/// `void RiderHandplantSub_Exit(RiderHandplantSub *self)` (0x132f98): clear
/// the handplant words and restore its pitch constant.
pub fn rider_handplant_sub_exit(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    let rider = this.off(0x14).lw(c);
    c.g[V0][0] = rider;
    let pitch = Addr((c.g[GP][0] as u32).wrapping_sub(0x754c)).read_fpu(c);
    c.f[0] = pitch.0;
    Addr((rider as u32).wrapping_add(0x32c)).sw(c, 0);
    let rider = this.off(0x14).lw(c);
    c.g[V1][0] = rider;
    Addr((rider as u32).wrapping_add(0x248)).write_fpu(c, pitch);
    Addr((rider as u32).wrapping_add(0x24c)).sw(c, 0);
    abi::ret(c);
}

pub(crate) const AI_SET_RACE_STATE: Callee = Callee {
    addr: 0x0011_3b10,
    f: ssx3_ee::s00::f_00113b10,
};

/// `void AI_SelectRaceState(AI *ai, int state)` (0x113b48): look up the
/// entry point for `state` in the dispatch table at `0x450000 + 0x6ca0`
/// (`(state-1) * 4`) and store the state's handler address (+0xa0), mode
/// (+0) and label pointer (+0x98); out-of-range states zero those fields.
/// The `__entries` twin handles the table's mid-function targets.
pub fn ai_select_race_state(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = sx(abi::arg(c, 1).wrapping_sub(1));
    c.g[V0][0] = (c.g[V1][0] < 7) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[A2][0] = c.g[A0][0];
    if out_of_range {
        c.cycles += 4;
        Addr(c.g[A2][0] as u32).sw(c, 0);
        Addr(c.g[A2][0] as u32).off(0x98).sw(c, 0);
        Addr(c.g[A2][0] as u32).off(0xa0).sw(c, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[V0][0] = 0x0045_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca0));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let target = c.g[A0][0] as u32;
    // (mode, handler address, label offset).
    let (mode, handler, label): (u64, u32, u32) = match target {
        0x0011_3b74 => (1, 0x0045_0000 + 0x6c58, 0xac),
        0x0011_3b94 => (2, 0x0045_0000 + 0x6c68, 0xb0),
        0x0011_3ba8 => (3, 0x004a_0000 + 0xe90, 0xb4),
        0x0011_3bbc => (4, 0x0045_0000 + 0x6c78, 0xbc),
        0x0011_3bd0 => (5, 0x004a_0000 + 0xe98, 0xc0),
        0x0011_3be4 => (6, 0x004a_0000 + 0xea0, 0xc4),
        0x0011_3bf8 => (7, 0x0045_0000 + 0x6c88, 0xc8),
        _ => {
            c.jump_out(target, 0x0011_3b48, 0x0011_3c1c);
            return;
        }
    };
    c.cycles += if mode == 1 { 4 } else { 5 };
    c.g[A0][0] = mode;
    c.g[V0][0] = sx(handler);
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(label));
    c.cycles += 4;
    Addr(c.g[A2][0] as u32).off(0x98).sw(c, c.g[V1][0]);
    Addr(c.g[A2][0] as u32).sw(c, c.g[A0][0]);
    Addr(c.g[A2][0] as u32).off(0xa0).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// AI manager: rider setup/teardown wrappers. The destroy/reset callees take
// their short path when the AI object's rider counts are zero.

pub(crate) const AI_DESTROY_MISSION_RIDERS: Callee = Callee {
    addr: 0x0012_af68,
    f: ssx3_ee::s00::f_0012af68,
};

pub(crate) const AI_DESTROY_ACTOR_RIDERS: Callee = Callee {
    addr: 0x0012_aed0,
    f: ssx3_ee::s00::f_0012aed0,
};

pub(crate) const AI_DESTROY_COMPUTER_RIDERS: Callee = Callee {
    addr: 0x0012_ae38,
    f: ssx3_ee::s00::f_0012ae38,
};

pub(crate) const AI_DESTROY_ALL_RIDERS: Callee = Callee {
    addr: 0x0012_b090,
    f: ssx3_ee::s00::f_0012b090,
};

pub(crate) const AI_CREATE_COMPUTER_RIDERS: Callee = Callee {
    addr: 0x0012_9e20,
    f: ssx3_ee::s00::f_00129e20,
};

pub(crate) const AI_CREATE_MISSION_RIDER: Callee = Callee {
    addr: 0x0012_9c00,
    f: ssx3_ee::s00::f_00129c00,
};

pub(crate) const AI_RESET_RIDERS: Callee = Callee {
    addr: 0x0012_97c8,
    f: ssx3_ee::s00::f_001297c8,
};

pub(crate) const AI_RESET_RIDER_RELATIONS: Callee = Callee {
    addr: 0x0010_f3b8,
    f: ssx3_ee::s00::f_0010f3b8,
};

pub(crate) const AI_FREE_ANIM_SOURCES: Callee = Callee {
    addr: 0x0012_b200,
    f: ssx3_ee::s00::f_0012b200,
};

/// `void AI_SetupComputerRiders(AI *ai)` (0x128958): tear down any mission
/// riders, then recreate the computer riders.
pub fn ai_setup_computer_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(c.g[S0][0] as u32).off(0x94).sw(c, 0);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x88).lw(c);
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            AI_DESTROY_MISSION_RIDERS.call(c, 0x0012_8980)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        AI_CREATE_COMPUTER_RIDERS.call(c, 0x0012_8988)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_SetupMissionRider(AI *ai)` (0x128ac0): create the mission rider
/// and reset all rider records.
pub fn ai_setup_mission_rider(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        AI_CREATE_MISSION_RIDER.call(c, 0x0012_8ad4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        AI_RESET_RIDERS.call(c, 0x0012_8ae0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_RemoveActorRiders(AI *ai)` (0x12b000): destroy the actor riders
/// and refresh the rider relations.
pub fn ai_remove_actor_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        AI_DESTROY_ACTOR_RIDERS.call(c, 0x0012_b014)?;
        c.g[A0][0] = c.g[S0][0];
        AI_RESET_RIDER_RELATIONS.call(c, 0x0012_b01c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_RemoveComputerRiders(AI *ai)` (0x12b030): destroy the computer
/// riders, refresh relations and clear the actor state.
pub fn ai_remove_computer_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        AI_DESTROY_COMPUTER_RIDERS.call(c, 0x0012_b044)?;
        c.g[A0][0] = c.g[S0][0];
        AI_RESET_RIDER_RELATIONS.call(c, 0x0012_b04c)?;
        Addr(c.g[S0][0] as u32).off(0x10).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_RemoveMissionRiders(AI *ai)` (0x12b060): destroy the mission
/// riders, refresh relations and clear the actor state.
pub fn ai_remove_mission_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        AI_DESTROY_MISSION_RIDERS.call(c, 0x0012_b074)?;
        c.g[A0][0] = c.g[S0][0];
        AI_RESET_RIDER_RELATIONS.call(c, 0x0012_b07c)?;
        Addr(c.g[S0][0] as u32).off(0x10).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_dtor(AI *ai, int free_memory)` (0x1287b8): destroy all riders,
/// delete the focus/danger managers, and (when `free_memory` is odd) the AI
/// itself.
pub fn ai_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7b78));
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Addr(c.g[S1][0] as u32).off(0xcc).sw(c, c.g[V0][0]);
        AI_DESTROY_ALL_RIDERS.call(c, 0x0012_87e0)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0xa4).lw(c);
        call::call_ptr(c, 0x0031_7e50, 0x0012_87e8)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0xa8).lw(c);
        call::call_ptr(c, 0x0031_7e50, 0x0012_87f0)?;
        c.g[S0][0] &= 1;
        let free_memory = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if free_memory {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call::call_ptr(c, 0x0031_7e50, 0x0012_8804)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of `AI_Destroy{Computer,Actor,Mission}Riders`: iterate the
/// `AIController` pointer array at `base_off` (count at `count_off`), call
/// each rider's vtable destructor (`rider[0x18] -> +0x6c0` slot), clear the
/// slot, then fold the count into the total at +0x78 and clear +`clear_off`.
pub(crate) fn ai_destroy_riders(
    c: &mut Cpu,
    count_off: u32,
    base_off: u32,
    clear_off: u32,
    head: u32,
    ret: u32,
) -> call::Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(count_off).lw(c);
    c.g[S2][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 1;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(base_off));
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = 0x3;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lw(c);
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
                c.g[A2][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = c.g[A2][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                call::call_ptr(c, target, ret)?;
            }
            c.cycles += 6;
            Addr(c.g[S0][0] as u32).sw(c, 0);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(count_off).lw(c);
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                crate::events::deliver_event_here(c, head);
            }
        }
    }
    c.cycles += 12;
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x78).lw(c);
    c.g[V1][0] = Addr(c.g[S1][0] as u32).off(count_off).lw(c);
    Addr(c.g[S1][0] as u32).off(clear_off).sw(c, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    Addr(c.g[S1][0] as u32).off(count_off).sw(c, 0);
    Addr(c.g[S1][0] as u32).off(0x78).sw(c, c.g[V0][0]);
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void AI_DestroyComputerRiders(AI *ai)` (0x12ae38).
pub fn ai_destroy_computer_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        ai_destroy_riders(c, 0x80, 0x48, 0x14, 0x0012_ae60, 0x0012_ae88)
    })
}

/// `void AI_DestroyActorRiders(AI *ai)` (0x12aed0).
pub fn ai_destroy_actor_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        ai_destroy_riders(c, 0x84, 0x5c, 0x8c, 0x0012_aef8, 0x0012_af20)
    })
}

/// `void AI_DestroyMissionRiders(AI *ai)` (0x12af68).
pub fn ai_destroy_mission_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        ai_destroy_riders(c, 0x88, 0x70, 0x90, 0x0012_af90, 0x0012_afb8)
    })
}

/// `void AI_DestroyAllRiders(AI *ai)` (0x12b090): destroy the record and
/// actor riders (vtable destructors), the mission/computer/actor rider sets,
/// free the animation data sources and clear the counts.
pub fn ai_destroy_all_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x78).lw(c);
        c.g[S0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x28));
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
            loop {
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
                c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x790).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xfc));
                call::call_ptr(c, 0x0015_4a58, 0x0012_b0d8)?;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x78).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_b0c0);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x7c).lw(c);
        c.g[S2][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A1][0] = 0x3;
                if c.g[V0][0] != 0 {
                    c.cycles += 7;
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lw(c);
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
                    c.g[A2][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                    let target = c.g[A2][0] as u32;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call::call_ptr(c, target, 0x0012_b120)?;
                }
                c.cycles += 6;
                Addr(c.g[S0][0] as u32).sw(c, 0);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x7c).lw(c);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_b0f8);
                }
            }
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S1][0];
        AI_DESTROY_MISSION_RIDERS.call(c, 0x0012_b140)?;
        c.g[A0][0] = c.g[S1][0];
        AI_DESTROY_COMPUTER_RIDERS.call(c, 0x0012_b148)?;
        c.g[A0][0] = c.g[S1][0];
        AI_DESTROY_ACTOR_RIDERS.call(c, 0x0012_b150)?;
        Addr(c.g[S1][0] as u32).off(0x7c).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr(c.g[S1][0] as u32).off(0x78).sw(c, 0);
        AI_FREE_ANIM_SOURCES.call(c, 0x0012_b160)?;
        Addr(c.g[S1][0] as u32).off(0x74).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_FreeAnimDataSources(AI *ai)` (0x12b200): release animation data
/// source slots (the original walks a global source table; the loop only
/// ever visits slot 0).
pub fn ai_free_anim_sources(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[S2][0] & 0xff;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[GP][0] as u32).off(0xd08).lw(c);
            c.g[S1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A1][0] = c.g[S0][0];
            if c.g[S0][0] != 0 {
                c.cycles += 9;
                c.g[A0][0] = Addr(c.g[GP][0] as u32).off(0xd8c).lw(c);
                call::call_ptr(c, 0x0031_4fe8, 0x0012_b244)?;
                c.g[A0][0] = Addr(c.g[GP][0] as u32).off(0xd08).lw(c);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                Addr(c.g[V0][0] as u32).sw(c, 0);
                call::call_ptr(c, 0x0031_1110, 0x0012_b254)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x3;
                call::call_ptr(c, 0x0031_12c8, 0x0012_b260)?;
            }
            c.cycles += 2;
            let wrap = (c.g[S2][0] as i64) <= 0;
            c.g[V1][0] = c.g[S2][0] & 0xff;
            if !wrap {
                break;
            }
            if event_due(c) {
                crate::events::deliver_event_here(c, 0x0012_b220);
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

pub(crate) const AI_RESET_RIDER_RELATIONS_2: Callee = Callee {
    addr: 0x0010_f398,
    f: ssx3_ee::s00::f_0010f398,
};

pub(crate) const RIDER_RESET: Callee = Callee {
    addr: 0x0011_d390,
    f: ssx3_ee::s00::f_0011d390,
};

pub(crate) const FOCUSMAN_RESET: Callee = Callee {
    addr: 0x0010_3358,
    f: ssx3_ee::s00::f_00103358,
};

pub(crate) const DANGERMAN_RESET: Callee = Callee {
    addr: 0x0010_1688,
    f: ssx3_ee::s00::f_00101688,
};

/// `void AI_ResetRiders(AI *ai, int clear)` (0x1297c8): reset relations and
/// zero the counters (+8, +0xc when `clear`), then reset every record
/// rider (via its vtable query) and every mission rider.
pub fn ai_reset_riders(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        AI_RESET_RIDER_RELATIONS_2.call(c, 0x0012_97ec)?;
        let clear = c.g[S3][0] != 0;
        Addr(c.g[S2][0] as u32).off(0x8).sw(c, 0);
        if clear {
            c.cycles += 1;
            Addr(c.g[S2][0] as u32).off(0xc).sw(c, 0);
        }
        c.cycles += 6;
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
        Addr(c.g[S2][0] as u32).off(0x18).sw(c, u64::MAX);
        c.g[S1][0] = 0;
        let none = (c.g[V1][0] as i64) <= 0;
        Addr(c.g[S2][0] as u32).off(0x1c).sw(c, 0);
        if !none {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
            loop {
                c.cycles += 9;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call::call_ptr(c, target, 0x0012_9834)?;
                let reset = if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    true
                } else {
                    c.cycles += 2;
                    if c.g[S3][0] != 0 {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        true
                    } else {
                        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
                        false
                    }
                };
                if reset {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x790).lw(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xfc));
                    call::call_ptr(c, 0x0015_4a58, 0x0012_9858)?;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    RIDER_RESET.call(c, 0x0012_9860)?;
                    c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_9818);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x88).lw(c);
        let no_missions = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !no_missions {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            loop {
                c.cycles += 8;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lw(c);
                RIDER_RESET.call(c, 0x0012_9898)?;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x88).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_9888);
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

/// `void AI_Restart(AI *ai, int state)` (0x129768): reset the focus and
/// danger managers, clear the path pointer and reselect the race state.
pub fn ai_restart(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xa4).lw(c);
        FOCUSMAN_RESET.call(c, 0x0012_9788)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xa8).lw(c);
        DANGERMAN_RESET.call(c, 0x0012_9790)?;
        Addr(c.g[S0][0] as u32).off(0x98).sw(c, 0);
        c.g[A1][0] = c.g[S1][0];
        Addr(c.g[S0][0] as u32).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        AI_SET_RACE_STATE.call(c, 0x0012_97a4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        AI_RESET_RIDERS.call(c, 0x0012_97b0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const RIDER_REFRESH_OVERLAPS: Callee = Callee {
    addr: 0x0012_0e50,
    f: ssx3_ee::s00::f_00120e50,
};

/// `void AI_RefreshRiderOverlaps(AI *ai)` (0x12b788): refresh the overlap
/// sets of every record rider.
pub fn ai_refresh_rider_overlaps(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
        c.g[S1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            loop {
                c.cycles += 7;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                RIDER_REFRESH_OVERLAPS.call(c, 0x0012_b7c4)?;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x78).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    crate::events::deliver_event_here(c, 0x0012_b7b8);
                }
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

/// `void AI_SetSyncWords(AI* self, int* src)` (0x12bac0): copy two words into
/// the AI's sync block at `+0x20`.
pub fn ai_set_sync_words(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x20));
    c.g[V1][0] = 1;
    loop {
        c.cycles += 7;
        let src = c.g[A1][0] as u32;
        c.g[V0][0] = Addr(src).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[A1][0] = sx(src.wrapping_add(4));
        let dst = c.g[A0][0] as u32;
        let v = c.g[V0][0];
        Addr(dst).sw(c, v);
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A0][0] = sx(dst.wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0012_bac8);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void AI_GetSyncWords(AI* self, int* dst)` (0x12baf0): copy the AI's sync
/// block at `+0x20` into `dst`.
pub fn ai_get_sync_words(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x20));
    c.g[V1][0] = 1;
    loop {
        c.cycles += 7;
        let src = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(src).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[A0][0] = sx(src.wrapping_add(4));
        let dst = c.g[A1][0] as u32;
        let v = c.g[V0][0];
        Addr(dst).sw(c, v);
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A1][0] = sx(dst.wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0012_baf8);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// The "tail-call virtual slot 2 of an interface object" shape.

/// `vt = *(VTable**)obj; vt[2].fn((char*)obj + vt[2].delta, self, tag)` — the
/// shared body of the twenty interface forwarders below. `obj` is the second
/// argument (`a1`), which the compiler keeps in `a2` (or `a3` when it also has
/// to materialize the extra constant argument in `a2`); `self` (the first
/// argument) becomes the method's second argument.
pub(crate) fn virtual_call_slot(c: &mut Cpu, ret: u32, tag: Option<u32>) -> Flow {
    c.cycles += if tag.is_some() { 13 } else { 12 };
    let frame = Frame::open(c, 0x10);
    let obj;
    if let Some(k) = tag {
        c.g[A3][0] = c.g[A1][0];
        obj = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = sx(k);
    } else {
        c.g[A2][0] = c.g[A1][0];
        obj = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
    }
    c.g[V1][0] = Addr(obj as u32).lw(c);
    let slot = VSlot::read(c, c.g[V1][0], 0x10);
    c.g[A0][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(obj);
    call_ptr(c, slot.func as u32, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `RiderGateSub_LoadState(self, iface)` (0x12c590): `iface->v02(self, 0x14)`.
pub fn rider_gate_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_c5b8, Some(0x14)))
}

/// `RiderFinishLineSub_LoadState(self, iface)` (0x12c9f8): `0x4`.
pub fn rider_finish_line_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_ca20, Some(0x4)))
}

/// `RiderWipeoutSub_LoadState(self, iface)` (0x12e740): `0x80`.
pub fn rider_wipeout_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_e768, Some(0x80)))
}

/// `RiderResetSub_LoadState(self, iface)` (0x12f5e8): `0x8`.
pub fn rider_reset_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_f610, Some(0x8)))
}

/// `RiderNaturalAirSub_LoadState(self, iface)` (0x12fc28): `0x14`.
pub fn rider_natural_air_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_fc50, Some(0x14)))
}

/// `RiderBoardPressSub_LoadState(self, iface)` (0x1315d0): `0x14`.
pub fn rider_board_press_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_15f8, Some(0x14)))
}

/// `RiderRailslideSub_LoadState(self, iface)` (0x132878): `0x8`.
pub fn rider_railslide_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_28a0, Some(0x8)))
}

/// `RiderHandplantSub_LoadState(self, iface)` (0x1330f0): `0x14`.
pub fn rider_handplant_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_3118, Some(0x14)))
}

/// `RiderSpinSub_LoadState(self, iface)` (0x136230): `0x58`.
pub fn rider_spin_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_6258, Some(0x58)))
}

/// `RiderUberGrindSub_LoadState(self, iface)` (0x136920): `0x8`.
pub fn rider_uber_grind_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_6948, Some(0x8)))
}

/// `void AI_ResetRidersAndOrder(AI* ai)` (0x128a10): reset every rider and
/// then put the order mode back to 0.
pub fn ai_reset_riders_and_order(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::AI_RESET_RIDERS.call(c, 0x0012_8a28)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::AI_SET_RIDER_ORDER_MODE.call(c, 0x0012_8a34)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SectionMan_FindLogicalSection` (0x22e0e0).
pub(crate) const FIND_LOGICAL_SECTION: u32 = 0x0022_e0e0;

/// `PathData_EmptyHook` (0x26ada0).
pub(crate) const PATH_DATA_EMPTY_HOOK: u32 = 0x0026_ada0;

// ---------------------------------------------------------------------------
// AI_PathDataInitIfEarlySection.

/// `void AI_PathDataInitIfEarlySection(void* p, uchar id)` (0x12a490) — the
/// reference has no name for it: when the logical section that the world's
/// rider manager names at `+0x78` is one of the first 22, initialise the path
/// data.
pub fn ai_path_data_init_if_early_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        let world = gp(c, -0x848);
        c.g[V0][0] = Addr(world).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x78).lw(c);
        call_ptr(c, FIND_LOGICAL_SECTION, 0x0012_a4a8)?;
        c.g[V0][0] = ((c.g[V0][0] as i64) < 22i64) as u64;
        let early = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if early {
            c.cycles += 4;
            c.g[A0][0] = 0x004d_0000;
            c.g[A0][0] = sx(0x004d_33a0);
            call_ptr(c, PATH_DATA_EMPTY_HOOK, 0x0012_a4c0)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
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

/// `void AI_SetupComputerRiders(cAI* ai)` (0x128958): kill the previous
/// computer riders when there were any, then create a fresh set.
pub fn ai_setup_computer_riders_2(c: &mut Cpu) {
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
pub(crate) fn rider_spin_sub_is_trick_anim(c: &mut Cpu, ret: u32, kind: u64, trick: u64) {
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

/// `bool RiderGateSub_CanStart(GateSub* g)` (0x12c078): false only when the
/// race is running (mode != 5) and the gate's timer (`g+0x14 -> +0x470`) is
/// still negative.
pub fn rider_gate_sub_can_start(c: &mut Cpu) {
    c.cycles += 8;
    let world = c.lw(gp(c, -0x848)) as u32;
    c.g[A3][0] = c.g[A0][0];
    c.g[A2][0] = 5;
    c.g[V0][0] = c.lw(world.wrapping_add(0x84));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
    c.g[A0][0] = c.lw(r32(c, V1));
    let running = c.g[A0][0] != c.g[A2][0];
    c.g[V0][0] = 1;
    if running {
        c.cycles += 6;
        c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(0x14));
        c.f[1] = 0;
        let timer = Addr(r32(c, V1).wrapping_add(0x470)).read_fpu(c);
        c.f[0] = timer.0;
        let ready = FpuF32::ZERO.le(timer);
        abi::set_fcc(c, ready);
        if !ready {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// The FPU constant the rider-gate timing multiplies the race clock by
/// (1/60: seconds per frame), at `gp_2 - 0x7840`.
pub(crate) const FRAME_SECONDS_GP: i32 = -0x7840;

/// The game-module global (`D_004A28A8`, the world/app pointer).
pub(crate) const GAME_MODULE_GP: i32 = -0x848;

// ---------------------------------------------------------------------------
// Rider gate timing.

/// `bool RiderGateSub_IsStartTimeReached(RiderGateSub* gate)` (0x12c028):
/// only while the race manager's state is 5, true when the race clock
/// (frames × 1/60) has reached the gate's start time (`gate->+0x14 ->
/// +0xb30`). 8 (+10 when racing) + 2 cycles.
pub fn ridergate_sub_is_start_time_reached(c: &mut Cpu) {
    c.cycles += 8;
    let module = c.lw(gp_2(c, GAME_MODULE_GP));
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
        let seconds = Addr(gp_2(c, FRAME_SECONDS_GP)).read_fpu(c);
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

/// `void AI_SetupActorRiders(AI* ai, int n)` (0x128998): clear the BE
/// interface, and unless the "computer players" flag is set, store `n` and
/// spawn the computer actor riders.
pub fn ai_setup_actor_riders(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    if call_ptr(c, 0x0014_dc80, 0x0012_89b4).is_err() {
        return;
    }
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    if call_ptr(c, 0x0014_dd58, 0x0012_89c0).is_err() {
        return;
    }
    c.g[V1][0] = 0x0053_0000;
    c.g[V0][0] = Addr(p_2(c, V1, 0x5c11)).lb(c);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.sw(p_2(c, S0, 0x94), c.g[S1][0]);
        c.g[A0][0] = c.g[S0][0];
        if callee::AI_CREATE_ACTOR_RIDERS.call(c, 0x0012_89dc).is_err() {
            return;
        }
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void RiderBoardPressSub_Cancel(RiderBoardPress* self)` (0x131348): cancel
/// a board press, finishing the metric and granting the boost when the press
/// had been held for one or two frames.
pub fn rider_board_press_sub_cancel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0013_135c);
        c.g[S0][0] = c.g[A0][0];
        callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW.call(c, 0x0013_135c)?;
        c.g[RA][0] = sx(0x0013_1364);
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS.call(c, 0x0013_1364)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(0x14).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x330).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            c.g[RA][0] = sx(0x0013_1384);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x790).lw(c);
            callee::RIDER_METRIX_FINISH_BOARD_PRESS.call(c, 0x0013_1384)?;
            c.g[A0][0] = Addr(r32(c, S0)).off(0x14).lw(c);
            c.mov_s(12, 0);
            c.g[RA][0] = sx(0x0013_1394);
            c.g[A1][0] = 1;
            callee::RIDER_ADD_BOOST_2.call(c, 0x0013_1394)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float RiderSpinSub_GetSpinHUDBlend(RiderSpinSub* self)` (0x136100): spin
/// amount in turns, clamped to [0, 1], eased by the square root curve.
pub fn rider_spin_sub_get_spin_hud_blend(c: &mut Cpu) {
    c.cycles += 11;
    c.lwc1(3, p(c.g[A0][0] as u32, 0x18));
    c.lwc1(0, gp(c, -0x7304));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 4);
    c.mul_s(0, 3, 0);
    c.add_s(0, 0, 4);
    c.cvt_w_s(1, 0);
    c.cvt_s_w(1, 1);
    c.c_lt_s(0, 1);
    if c.fcc() {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.sub_s(1, 1, 0);
    }
    c.cycles += 11;
    c.lwc1(0, gp(c, -0x7300));
    c.lwc1(2, gp(c, -0x72fc));
    c.mul_s(0, 1, 0);
    c.mtc1(0, 1);
    c.sub_s(0, 3, 0);
    c.abs_s(0, 0);
    c.mul_s(0, 0, 2);
    c.sub_s(0, 0, 4);
    c.max_s(0, 0, 1);
    c.add_s(0, 0, 0);
    c.pc = c.g[RA][0] as u32;
}

pub(crate) const ANIM_PLAYER_GET_ANIM_FIELD: u32 = 0x0031_2aa0;

pub(crate) const ANIM_CONTROLLER_SET_ANIM: u32 = 0x0031_28e8;

/// `void RiderPrewindSub_Exit(RiderPrewindSub* s)` (0x12e9b0): nothing to tear
/// down.
pub fn rider_prewind_sub_exit(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// RiderWipeoutSub animation selectors: look the current/next animation id up
// in a `.rodata` table and, when it actually changes, restart it through the
// rider's anim controller and rescale its speed.

/// The `RiderWipeoutSub` animation-id table (`.rodata 0x4580a0`): case block to
/// the id it selects.
pub(crate) const SWITCH_TO_ANIM_SET_A_IDS: [(u32, u64); 10] = [
    (0x0012_ddf8, 0x16a),
    (0x0012_de00, 0x16b),
    (0x0012_de08, 0x16c),
    (0x0012_de10, 0x16d),
    (0x0012_de18, 0x16e),
    (0x0012_de20, 0x16f),
    (0x0012_de28, 0x170),
    (0x0012_de30, 0x171),
    (0x0012_de38, 0x172),
    (0x0012_de40, 0x173),
];

/// `void RiderWipeoutSub_SwitchToAnimSetA(RiderWipeoutSub* w)` (0x12dd98):
/// pick animation set A's id for the rider's current animation and, if that is
/// not what is playing, restart it at speed -1 and update the anim speed.
pub fn rider_wipeout_sub_switch_to_anim_set_a(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0012_dd98 => {
                    c.cycles += 14;
                    let frame = Frame::open(c, 0x20);
                    c.g[A1][0] = 0x2;
                    frame.save(c, 16, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_PLAYER_GET_ANIM_FIELD, 0x0012_ddb8)?;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x150).lw(c);
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let same = c.g[V0][0] == 0;
                    c.g[A2][0] = c.g[A1][0];
                    pc = if same { 0x0012_de48 } else { 0x0012_ddd0 };
                }
                0x0012_ddd0 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x175));
                    c.g[V0][0] = (c.g[V1][0] < 0xfu64) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x46_0000;
                    pc = if out { 0x0012_de4c } else { 0x0012_dde0 };
                }
                0x0012_dde0 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f60));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match SWITCH_TO_ANIM_SET_A_IDS.iter().find(|e| e.0 == target) {
                        Some(e) => {
                            c.cycles += 2;
                            c.g[A1][0] = e.1;
                            0x0012_de4c
                        }
                        // The table's 0x17f entry keeps the id unchanged.
                        None if target == 0x0012_de4c => 0x0012_de4c,
                        None => {
                            c.jump_out(target, 0x0012_dd98, 0x0012_de80);
                            return Ok(());
                        }
                    };
                }
                0x0012_ddf8 | 0x0012_de00 | 0x0012_de08 | 0x0012_de10 | 0x0012_de18
                | 0x0012_de20 | 0x0012_de28 | 0x0012_de30 | 0x0012_de38 | 0x0012_de40 => {
                    c.cycles += 2;
                    let id = SWITCH_TO_ANIM_SET_A_IDS
                        .iter()
                        .find(|e| e.0 == pc)
                        .unwrap()
                        .1;
                    c.g[A1][0] = id;
                    pc = 0x0012_de4c;
                }
                0x0012_de48 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x174;
                    pc = 0x0012_de4c;
                }
                0x0012_de4c => {
                    c.cycles += 2;
                    let same = c.g[A1][0] == c.g[A2][0];
                    c.g[A2][0] = 0;
                    pc = if same { 0x0012_de70 } else { 0x0012_de54 };
                }
                0x0012_de54 => {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 12);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_CONTROLLER_SET_ANIM, 0x0012_de68)?;
                    c.g[A0][0] = c.g[S0][0];
                    callee::RIDER_WIPEOUT_UPDATE_ANIM_SPEED.call(c, 0x0012_de70)?;
                    pc = 0x0012_de70;
                }
                0x0012_de70 => {
                    c.cycles += 4;
                    let frame = Frame { size: 0x20 };
                    frame.restore(c, 16, 0x10);
                    frame.restore_ra(c, 0);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0012_dd98, 0x0012_de80);
                    return Ok(());
                }
            }
        }
    })
}

/// `RiderWipeoutSub_PlayEndAnimA` (0x12df48): pick the end-animation id for the
/// rider's current animation (table `.rodata 0x458110`) and, if it changes,
/// restart it at speed -1.
pub fn rider_wipeout_sub_play_end_anim_a(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0012_df48 => {
                    c.cycles += 13;
                    let frame = Frame::open(c, 0x20);
                    c.g[A1][0] = 0x2;
                    frame.save(c, 16, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_PLAYER_GET_ANIM_FIELD, 0x0012_df68)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x16a));
                    c.g[V0][0] = (c.g[V1][0] < 0x1au64) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[A2][0] = c.g[A1][0];
                    pc = if out { 0x0012_dfe4 } else { 0x0012_df7c };
                }
                0x0012_df7c => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x46_0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ef0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match target {
                        0x0012_df98 => 0x0012_df98,
                        0x0012_dfa0 => 0x0012_dfa0,
                        0x0012_dfa8 => 0x0012_dfa8,
                        0x0012_dfb0 => 0x0012_dfb0,
                        0x0012_dfb8 => 0x0012_dfb8,
                        0x0012_dfc0 => 0x0012_dfc0,
                        0x0012_dfc8 => 0x0012_dfc8,
                        0x0012_dfd0 => 0x0012_dfd0,
                        0x0012_dfd8 => 0x0012_dfd8,
                        0x0012_dfe0 => 0x0012_dfe0,
                        0x0012_dfe4 => 0x0012_dfe4,
                        _ => {
                            c.jump_out(target, 0x0012_df48, 0x0012_e010);
                            return Ok(());
                        }
                    };
                }
                0x0012_df98 | 0x0012_dfa0 | 0x0012_dfa8 | 0x0012_dfb0 | 0x0012_dfb8
                | 0x0012_dfc0 | 0x0012_dfc8 | 0x0012_dfd0 | 0x0012_dfd8 => {
                    c.cycles += 2;
                    let id = match pc {
                        0x0012_df98 => 0x185,
                        0x0012_dfa0 => 0x186,
                        0x0012_dfa8 => 0x187,
                        0x0012_dfb0 => 0x188,
                        0x0012_dfb8 => 0x189,
                        0x0012_dfc0 => 0x18a,
                        0x0012_dfc8 => 0x18b,
                        0x0012_dfd0 => 0x18c,
                        _ => 0x18d,
                    };
                    c.g[A1][0] = id;
                    pc = 0x0012_dfe4;
                }
                0x0012_dfe0 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x18e;
                    pc = 0x0012_dfe4;
                }
                0x0012_dfe4 => {
                    c.cycles += 2;
                    let same = c.g[A1][0] == c.g[A2][0];
                    c.g[A2][0] = 0;
                    pc = if same { 0x0012_e000 } else { 0x0012_dfec };
                }
                0x0012_dfec => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x80).lw(c);
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 12);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_CONTROLLER_SET_ANIM, 0x0012_e000)?;
                    pc = 0x0012_e000;
                }
                0x0012_e000 => {
                    c.cycles += 4;
                    let frame = Frame { size: 0x20 };
                    frame.restore(c, 16, 0x10);
                    frame.restore_ra(c, 0);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0012_df48, 0x0012_e010);
                    return Ok(());
                }
            }
        }
    })
}

/// The tumble-animation table (`.rodata 0x457fa0`): case block, animation id,
/// its cycle charge and the alternate table entry the ground flag selects.
pub(crate) const TUMBLE_ANIM_IDS: [(u32, u64, u64, u64); 33] = [
    (0x0012_dae8, 0x16a, 3, 0x175),
    (0x0012_daf4, 0x16b, 3, 0x176),
    (0x0012_db00, 0x16c, 3, 0x177),
    (0x0012_db0c, 0x170, 3, 0x17b),
    (0x0012_db18, 0x16e, 3, 0x179),
    (0x0012_db24, 0x16f, 3, 0x17a),
    (0x0012_db30, 0x172, 3, 0x17d),
    (0x0012_db3c, 0x173, 3, 0x17e),
    (0x0012_db48, 0x16d, 3, 0x178),
    (0x0012_db54, 0x171, 3, 0x17c),
    (0x0012_db60, 0x16a, 3, 0x175),
    (0x0012_db6c, 0x16b, 3, 0x176),
    (0x0012_db78, 0x16c, 3, 0x177),
    (0x0012_db84, 0x170, 3, 0x17b),
    (0x0012_db90, 0x16e, 3, 0x179),
    (0x0012_db9c, 0x173, 3, 0x17e),
    (0x0012_dba8, 0x171, 3, 0x17c),
    (0x0012_dbb4, 0x16d, 3, 0x178),
    (0x0012_dbc0, 0x16f, 3, 0x17a),
    (0x0012_dbcc, 0x172, 3, 0x17d),
    (0x0012_dbd8, 0x170, 3, 0x17b),
    (0x0012_dbe4, 0x171, 3, 0x17c),
    (0x0012_dbf0, 0x16c, 3, 0x177),
    (0x0012_dbfc, 0x16e, 3, 0x179),
    (0x0012_dc08, 0x173, 3, 0x17e),
    (0x0012_dc14, 0x170, 3, 0x17b),
    (0x0012_dc20, 0x170, 3, 0x17b),
    (0x0012_dc2c, 0x172, 3, 0x17d),
    (0x0012_dc38, 0x16b, 3, 0x176),
    (0x0012_dc44, 0x16a, 3, 0x175),
    (0x0012_dc50, 0x170, 3, 0x17b),
    (0x0012_dc5c, 0x172, 3, 0x17d),
    (0x0012_dc68, 0x174, 2, 0x17f),
];

/// `RiderWipeoutSub_SelectTumbleAnim` (0x12da88): choose the tumble animation
/// for the rider's current animation (table `.rodata 0x457fa0`) and, when it
/// changes, restart it at speed -1 and update the anim speed. `s0` records
/// whether the rider was on the ground (animation index 0), which selects the
/// second table column.
pub fn rider_wipeout_sub_select_tumble_anim(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x0012_da88 => {
                    c.cycles += 17;
                    let frame = Frame::open(c, 0x30);
                    c.g[A1][0] = 0x2;
                    frame.save(c, 16, 0x20);
                    frame.save(c, 17, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A0][0];
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x80).lw(c);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x784).lw(c);
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    c.g[S0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    call_ptr(c, ANIM_PLAYER_GET_ANIM_FIELD, 0x0012_dab8)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x148));
                    c.g[V0][0] = (c.g[V1][0] < 0x22u64) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[A2][0] = c.g[A1][0];
                    pc = if out { 0x0012_dc74 } else { 0x0012_dacc };
                }
                0x0012_dacc => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x45_0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7fa0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    if let Some(e) = TUMBLE_ANIM_IDS.iter().find(|e| e.0 == target) {
                        c.cycles += e.2;
                        c.g[A1][0] = e.1;
                        c.g[V0][0] = e.3;
                        pc = 0x0012_dc70;
                    } else {
                        c.jump_out(target, 0x0012_da88, 0x0012_dcac);
                        return Ok(());
                    }
                }
                0x0012_dae8 | 0x0012_daf4 | 0x0012_db00 | 0x0012_db0c | 0x0012_db18
                | 0x0012_db24 | 0x0012_db30 | 0x0012_db3c | 0x0012_db48 | 0x0012_db54
                | 0x0012_db60 | 0x0012_db6c | 0x0012_db78 | 0x0012_db84 | 0x0012_db90
                | 0x0012_db9c | 0x0012_dba8 | 0x0012_dbb4 | 0x0012_dbc0 | 0x0012_dbcc
                | 0x0012_dbd8 | 0x0012_dbe4 | 0x0012_dbf0 | 0x0012_dbfc | 0x0012_dc08
                | 0x0012_dc14 | 0x0012_dc20 | 0x0012_dc2c | 0x0012_dc38 | 0x0012_dc44
                | 0x0012_dc50 | 0x0012_dc5c | 0x0012_dc68 => {
                    let e = TUMBLE_ANIM_IDS.iter().find(|e| e.0 == pc).unwrap();
                    c.cycles += e.2;
                    c.g[A1][0] = e.1;
                    c.g[V0][0] = e.3;
                    pc = 0x0012_dc70;
                }
                0x0012_dc70 => {
                    c.cycles += 1;
                    if c.g[S0][0] != 0 {
                        c.g[A1][0] = c.g[V0][0];
                    }
                    pc = 0x0012_dc74;
                }
                0x0012_dc74 => {
                    c.cycles += 2;
                    let same = c.g[A1][0] == c.g[A2][0];
                    c.g[A2][0] = 0;
                    pc = if same { 0x0012_dc98 } else { 0x0012_dc7c };
                }
                0x0012_dc7c => {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x80).lw(c);
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 12);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x784).lw(c);
                    call_ptr(c, ANIM_CONTROLLER_SET_ANIM, 0x0012_dc90)?;
                    c.g[A0][0] = c.g[S1][0];
                    callee::RIDER_WIPEOUT_UPDATE_ANIM_SPEED.call(c, 0x0012_dc98)?;
                    pc = 0x0012_dc98;
                }
                0x0012_dc98 => {
                    c.cycles += 5;
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, 16, 0x20);
                    frame.restore(c, 17, 0x10);
                    frame.restore_ra(c, 0);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0012_da88, 0x0012_dcac);
                    return Ok(());
                }
            }
        }
    })
}

/// `AI_LoadNextRider(self)` (0x1296f8): find the first roster rider that is
/// not loaded and load its step. A `match pc` dispatcher re-enterable at its
/// loop head (0x129720) after an unwind.
pub fn ai_load_next_rider(c: &mut Cpu) {
    const ENTRY: u32 = 0x0012_96f8;
    const END: u32 = 0x0012_9764;
    const AT_HEAD: u32 = 0x0012_9720;
    const AT_DEAD: u32 = 0x0012_971c;
    guest(c, |c| {
        let mut stage = match c.pc {
            ENTRY => 0u8,
            AT_HEAD => 1,
            AT_DEAD => 6,
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        };
        // The frame is opened at the entry stage; resume stages reuse it.
        let mut frame = Frame { size: 0x40 };
        loop {
            match stage {
                0 => {
                    c.cycles += 9;
                    frame = Frame::open(c, 0x40);
                    frame.save(c, S1, 0x20);
                    frame.save(c, S0, 0x30);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S2, 0x10);
                    c.g[S0][0] = sx(p(c.g[S1][0] as u32, 0x28));
                    frame.save_ra(c, 0);
                    c.g[S2][0] = 0;
                    stage = 2;
                }
                2 => {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x78)).lw(c);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        frame.restore(c, S0, 0x30);
                        stage = 5;
                    } else {
                        stage = 3;
                    }
                }
                3 => {
                    c.cycles += 4;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    call_ptr(c, 0x0011_d640, 0x0012_973c)?;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                        if back_edge_unwinds(c, AT_HEAD) {
                            return Ok(());
                        }
                        stage = 1;
                    } else {
                        stage = 4;
                    }
                }
                4 => {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    call_ptr(c, 0x0011_c298, 0x0012_974c)?;
                    frame.restore(c, S0, 0x30);
                    stage = 5;
                }
                1 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    stage = 2;
                }
                6 => {
                    c.cycles += 1;
                    stage = 1;
                }
                _ => {
                    c.cycles += 5;
                    let frame = Frame { size: 0x40 };
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
    })
}

/// `void RiderGateSub_UpdateWaitPush(RiderGateSub* g, AiBits* bits)`
/// (0x12c0c0): when the signed 6-bit field at `bits+0` scaled by 0.032258 is
/// positive, play the landing animation (id 2) and set `g->state = 3`.
pub fn rider_gate_sub_update_wait_push(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.lwc1(2, gp(c, -0x783c));
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.mtc1(0, 1);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 14);
        let v = c.g[V0][0] as u32;
        c.g[V0][0] = sx(((v as i32) >> 26) as u32);
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(0, 0, 2);
        c.c_lt_s(1, 0);
        let not_positive = !c.fcc();
        c.g[S0][0] = c.g[A0][0];
        if !not_positive {
            c.cycles += 9;
            c.g[V0][0] = Addr(p_3(r32(c, S0), 0x14)).lw(c);
            c.g[A1][0] = 2;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            c.g[A0][0] = Addr(p_3(c.g[V0][0] as u32, 0x784)).lw(c);
            call::call_ptr(c, 0x0031_28e8, 0x0012_c118)?;
            c.g[V1][0] = 3;
            Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
        }
        c.cycles += 4;
        lqsp(c, 16, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `AI_ComputerRidersHaveRenderSlots` (0x12ABD0): true when every computer
/// rider in the rider list has a render slot.
pub fn ai_computer_riders_have_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, S2, 0x80);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 1;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = low(c, S2).wrapping_add(0x48);
            loop {
                c.cycles += 9;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                c.g[S1][0] = low(c, S1).wrapping_add(1);
                c.g[S0][0] = low(c, S0).wrapping_add(4);
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x18)).lw(c);
                callee::RIDER_HAS_RENDER_SLOT.call(c, 0x0012_ac14)?;
                c.g[V1][0] = lw_at(c, S2, 0x80);
                c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                if c.g[V0][0] == 0 {
                    c.g[S3][0] = 0;
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_ac00);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AI_ActorRidersHaveRenderSlots` (0x12ACF8): the same query for the actor
/// rider list (count at +0x84, riders at +0x5c).
pub fn ai_actor_riders_have_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, S2, 0x84);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 1;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = low(c, S2).wrapping_add(0x5c);
            loop {
                c.cycles += 9;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                c.g[S1][0] = low(c, S1).wrapping_add(1);
                c.g[S0][0] = low(c, S0).wrapping_add(4);
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x18)).lw(c);
                callee::RIDER_HAS_RENDER_SLOT.call(c, 0x0012_ad3c)?;
                c.g[V1][0] = lw_at(c, S2, 0x84);
                c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                if c.g[V0][0] == 0 {
                    c.g[S3][0] = 0;
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_ad28);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AI_GetSyncChecksum(self)` (0x12b420): start from the AI's random value
/// and add the sync checksum of each rider in the array at `+0x28`.
pub fn ai_get_sync_checksum(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x50));
        c.sq(19, sp_2(c).wrapping_add(0x10));
        c.sq(17, sp_2(c).wrapping_add(0x30));
        c.g[S3][0] = c.g[A0][0];
        c.sq(18, sp_2(c).wrapping_add(0x20));
        c.sq(16, sp_2(c).wrapping_add(0x40));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S2][0] = 0;
        call_ptr(c, callee::AI_RAND, 0x0012_b444)?;
        c.g[V1][0] = lw_r(c, S3, 0x78);
        let empty = (c.g[V1][0] as i64) <= 0;
        c.g[S1][0] = c.g[V0][0];
        if !empty {
            c.cycles += 2;
            c.g[S0][0] = sx(low_2(c, S3).wrapping_add(0x28));
            loop {
                c.cycles += 8;
                c.g[A0][0] = Addr(low_2(c, S0)).lw(c);
                c.g[S2][0] = sx(low_2(c, S2).wrapping_add(1));
                c.g[S0][0] = sx(low_2(c, S0).wrapping_add(4));
                callee::RIDER_GET_SYNC_CHECKSUM.call(c, 0x0012_b468)?;
                c.g[V1][0] = lw_r(c, S3, 0x78);
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                c.g[S1][0] = sx(low_2(c, S1).wrapping_add(low_2(c, V0)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0012_b458);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        c.lq(16, sp_2(c).wrapping_add(0x40));
        c.lq(17, sp_2(c).wrapping_add(0x30));
        c.lq(18, sp_2(c).wrapping_add(0x20));
        c.lq(19, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low_2(c, RA);
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x50));
        c.pc = t;
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
                callee::RIDER_METRIX_RESET_STATS_2.call(c, 0x0012_b1d4)?;
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
            c.lwc1(1, gp_3(c, -0x75f8));
            // The slew's step is 0.025 per 60 Hz tick (times the time scale).
            c.f[1] = rate_step_60(c, FpuF32(c.f[1])).0;
            Addr(at(c, V0, 0x288)).sw(c, 0);
            c.mul_s(0, 0, 1);
            let ra = word(c, RA);
            c.swc1(0, at(c, V0, 0x284));
            c.pc = ra;
            return;
        }
        c.cycles += 5;
        c.lwc1(1, gp_3(c, -0x75fc));
        // 0.025 per 60 Hz tick.
        c.f[1] = rate_step_60(c, FpuF32(c.f[1])).0;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        c.mul_s(0, 0, 1);
    } else {
        c.cycles += 5;
        c.lwc1(0, at(c, V0, 0x300));
        c.lwc1(1, gp_3(c, -0x7600));
        // 0.025 per 60 Hz tick.
        c.f[1] = rate_step_60(c, FpuF32(c.f[1])).0;
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

/// `void AI_UpdateRiderPoses(World* w)` (0x00129160): tick, update the
/// skeleton and rebuild the bone matrices for the world's first `+0x78`
/// riders (the rider pointers start at `+0x28`).
pub fn ai_update_rider_poses(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x78));
        c.g[S1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            loop {
                c.cycles += 14;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 12);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, 0x0011_eb60, 0x0012_91a0)?; // Rider_TickAnimation
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                call_ptr(c, 0x0011_eb98, 0x0012_91a8)?; // Rider_UpdateSkeleton
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x780));
                call_ptr(c, 0x0031_03f0, 0x0012_91b8)?; // RiderModel_BuildBoneMatrices
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x78));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x0012_9190);
                }
            }
        }
        c.cycles += 6;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `float RiderSpinSub_GetFlipHUDBlend(SpinSub* self)` (0x136168): blend of
/// the rider's spin angle folded into a quarter turn, then scaled.
pub fn rider_spin_sub_get_flip_hud_blend(c: &mut Cpu) {
    c.cycles += 11;
    c.lwc1(3, at(c, A0, 0x1c));
    c.lwc1(0, gp_4(c, -0x72f8));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 1);
    c.mul_s(0, 3, 0);
    c.add_s(0, 0, 1);
    c.cvt_w_s(1, 0);
    c.cvt_s_w(1, 1);
    c.c_lt_s(0, 1);
    if !c.fcc() {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.sub_s(1, 1, 0);
    }
    c.cycles += 11;
    c.lwc1(0, gp_4(c, -0x72f4));
    c.lwc1(2, gp_4(c, -0x72f0));
    c.mul_s(0, 1, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.sub_s(0, 3, 0);
    c.abs_s(0, 0);
    c.mul_s(2, 0, 2);
    c.c_lt_s(1, 2);
    if !c.fcc() {
        c.cycles += 3;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(AT, 0);
        c.sub_s(2, 2, 0);
    }
    c.cycles += 8;
    c.g[AT][0] = 0x3e80_0000;
    c.mtc1(AT, 0);
    c.mtc1(ZERO, 1);
    c.sub_s(2, 2, 0);
    c.lwc1(0, gp_4(c, -0x72ec));
    c.max_s(2, 2, 1);
    let ra = r32(c, RA);
    c.mul_s(0, 2, 0);
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Rider.

/// `int RiderRailslideSub_SetRail(self, int id)` (0x132620): remember the
/// rail id, ask the rider's rail state to play its enter animation when the
/// rail flag is set, and return 1 in that case.
pub fn rider_railslide_sub_set_rail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0xffff_ffff);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let none = c.g[S1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if none {
            c.cycles += 1;
            Addr(p(r32(c, S0), 4)).sw(c, c.g[S1][0]);
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.mtc1(0, 1);
            c.lwc1(0, p(r32(c, V1), 0x2f0));
            c.c_lt_s(1, 0);
            if !c.fcc() {
                c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, V1), 0xb2c)).lw(c);
                c.g[V0][0] = sx(((r32(c, V0) as i32) >> 1) as u32);
                c.g[V0][0] = c.g[V0][0] & 1;
                let rail_off = c.g[V0][0] == 0;
                c.g[A1][0] = 0xc;
                if !rail_off {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(p(r32(c, V1), 0x77c)).lw(c);
                    Addr(p(r32(c, V0), 0x394)).sw(c, c.g[S1][0]);
                    c.g[A0][0] = Addr(p(r32(c, S0), 8)).lw(c);
                    call_ptr(c, 0x0011_fec8, 0x00132680)?;
                    c.g[V0][0] = 1;
                    c.cycles += 5;
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 1;
                c.g[V0][0] = Addr(p(r32(c, S0), 4)).lw(c);
            }
            c.cycles += 2;
            if c.g[S1][0] == c.g[V0][0] {
                Addr(p(r32(c, S0), 4)).sw(c, c.g[S1][0]);
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 5;
                call_ptr(c, 0x0028_b180, 0x0013269c)?;
                c.g[A1][0] = Addr(p(r32(c, S0), 8)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0029_9b70, 0x001326a8)?;
                c.cycles += 1;
                Addr(p(r32(c, S0), 4)).sw(c, c.g[S1][0]);
                c.cycles += 1;
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

/// `void RiderRailslideSub_PlayEnterAnim(self, int alt)` (0x1326c8): play
/// the railslide enter animation matching the rider's state and side.
pub fn rider_railslide_sub_play_enter_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 4;
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, A0), 8)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x328)).lw(c);
        let not4 = c.g[A0][0] != c.g[V0][0];
        c.g[V0][0] = 3;
        // 1: the `alt` entry (b_132748), 2: the shared 0x12 entry (b_132730).
        let tail: u32;
        if not4 {
            c.cycles += 2;
            let not3 = c.g[A0][0] != c.g[V0][0];
            if not3 {
                c.cycles += 2;
                let alt0 = c.g[A1][0] == 0;
                c.g[A1][0] = 0x44;
                if alt0 {
                    tail = 1;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                    tail = 2;
                }
            } else {
                c.cycles += 2;
                let alt0 = c.g[A1][0] == 0;
                c.g[A1][0] = 0x46;
                if alt0 {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                    c.g[A1][0] = 0x13;
                    tail = 2;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                    tail = 2;
                }
            }
        } else {
            c.cycles += 2;
            let alt0 = c.g[A1][0] == 0;
            c.g[A1][0] = 0x45;
            if alt0 {
                c.cycles += 3;
                c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                c.g[A1][0] = 0x14;
                tail = 2;
            } else {
                c.cycles += 2;
                c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
                tail = 2;
            }
        }
        if tail == 1 {
            c.cycles += 6;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_28e8, 0x00132740)?;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        } else {
            c.cycles += 7;
            c.g[A0][0] = Addr(p(r32(c, V1), 0x784)).lw(c);
            c.g[A1][0] = 0x12;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_28e8, 0x00132760)?;
            c.g[RA][0] = Addr(sp(c)).ld(c);
        }
        c.cycles += 2;
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
            c.g[V0][0] = lw_3(c, at_2(c.g[S0][0], 4));
            c.g[RA][0] = sx(0x0012_c5e8);
            c.g[A0][0] = lw_3(c, at_2(c.g[V0][0], 0x790));
            callee::RIDER_METRIX_ON_RIDER_RESET.call(c, 0x0012_c5e8)?;
            c.g[V1][0] = lw_3(c, at_2(c.g[S0][0], 4));
            c.lwc1(1, gp_5(c, 0x1b9c));
            c.lqc2(3, at_2(c.g[V1][0], 0x1e0));
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
            c.lwc1(1, at_2(c.g[V1][0], 0x2dc));
            c.mtc1(0, 0);
            c.c_eq_s(1, 0);
            if !c.fcc() {
                Addr(c.g[S0][0] as u32).sw(c, 0);
                break 'end;
            }
            'check: {
                c.cycles += 6;
                c.lwc1(1, at_2(c.g[V1][0], 0x378));
                c.lwc1(0, gp_5(c, -0x77fc));
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
                c.lwc1(0, at_2(c.g[S0][0], 0x70));
                c.c_le_s(1, 0);
                let t = c.fcc();
                c.g[S1][0] = 0;
                if t {
                    break 'restart;
                }
                {
                    c.cycles += 7;
                    c.g[V0][0] = lw_3(c, at_2(c.g[S0][0], 0x80));
                    c.mtc1(0, 1);
                    c.lwc1(0, at_2(c.g[V0][0], 0x470));
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
                    c.g[A0][0] = lw_3(c, at_2(c.g[V0][0], 0x784));
                    break 'first;
                }
                break 'restart;
            }
            c.cycles += 3;
            c.g[V0][0] = lw_3(c, at_2(c.g[S0][0], 0x80));
            c.g[S1][0] = 1;
            c.g[A0][0] = lw_3(c, at_2(c.g[V0][0], 0x784));
            break 'first;
        }
        'second: {
            'play: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0012_da44);
                c.g[A1][0] = 2;
                callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED.call(c, 0x0012_da44)?;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = lw_3(c, at_2(c.g[S0][0], 0x80));
                    break 'play;
                }
                c.cycles += 2;
                if c.g[S1][0] == 0 {
                    frame.restore(c, S0, 0x20);
                    break 'second;
                }
                c.cycles += 1;
                c.g[A0][0] = lw_3(c, at_2(c.g[S0][0], 0x80));
                break 'play;
            }
            c.cycles += 7;
            c.g[RA][0] = sx(0x0012_da60);
            c.g[A1][0] = c.g[S1][0];
            callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST.call(c, 0x0012_da60)?;
            c.g[A0][0] = lw_3(c, at_2(c.g[S0][0], 0x80));
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
// `AI_ResetComputerRenderSlots`/`AI_ResetActorRenderSlots`: clear the "render
// slot taken" word of every rider of the AI's list, release the slot in the
// global graphics manager and store the slot number + 4.
pub(crate) fn ai_reset_render_slots(
    c: &mut Cpu,
    zero_off: u32,
    count_off: u32,
    items_off: u32,
    ret: u32,
) -> crate::call::Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    frame.save(c, S2, 0x30);
    frame.save(c, S3, 0x20);
    frame.save(c, S4, 0x10);
    frame.save_ra(c, 0);
    c.g[S3][0] = c.g[A0][0];
    c.sw((c.g[S3][0] as u32).wrapping_add(zero_off), 0u64);
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(count_off));
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[S2][0] = 0u64;
    if !none {
        c.cycles += 2;
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(items_off));
        loop {
            c.cycles += 5;
            c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x884));
            if c.g[V1][0] != 0 {
                c.cycles += 13;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x888), c.g[S4][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x884), 0u64);
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[S0][0] = c.lw(c.g[S1][0] as u32);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x390));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x394));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, ret)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x888), c.g[V0][0]);
            }
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(count_off));
            c.cycles += 4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, ret - 0x38);
            }
        }
    }
    c.cycles += 8;
    c.lq(16, frame.at(c, 0x50));
    c.lq(17, frame.at(c, 0x40));
    c.lq(18, frame.at(c, 0x30));
    c.lq(19, frame.at(c, 0x20));
    c.lq(20, frame.at(c, 0x10));
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `AI_ResetComputerRenderSlots` (0x12ab20): free every computer rider's
/// render slot.
pub fn ai_reset_computer_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        ai_reset_render_slots(c, 0x14, 0x80, 0x48, 0x0012_ab90)
    })
}

/// `AI_ResetActorRenderSlots` (0x12ac48): free every actor rider's render slot.
pub fn ai_reset_actor_render_slots(c: &mut Cpu) {
    guest(c, |c| {
        ai_reset_render_slots(c, 0x8c, 0x84, 0x5c, 0x0012_acb8)
    })
}

// ---------------------------------------------------------------------------
// AI_ForceFinishRiders (0x000012b340).

/// `AI_ForceFinishRiders`: when the AI race state is 5, force every rider that has not finished to the end of the course.
pub fn ai_force_finish_riders(c: &mut Cpu) {
    guest(c, |c| {
        'b_12b400: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V1][0] = 0x5;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = c.lw(c.g[S3][0] as u32);
            let t = c.g[V0][0] != c.g[V1][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_12b400;
            } else {
                'b_12b3fc: {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    c.g[S2][0] = 0u64;
                    call_ptr(c, 0x00258950, 0x0012b380)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x7c));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S4][0] = 0x1;
                    if t {
                        break 'b_12b3fc;
                    } else {
                        c.cycles += 3;
                        c.g[AT][0] = 0x41200000;
                        c.mtc1(1, 20);
                        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x40));
                        'l_12b398: loop {
                            'b_12b3ec: {
                                c.cycles += 5;
                                c.g[S0][0] = c.lw(c.g[S1][0] as u32);
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x87c));
                                if c.g[V0][0] != 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x7c));
                                    break 'b_12b3ec;
                                } else {
                                    'b_12b3c4: {
                                        c.cycles += 5;
                                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x470));
                                        c.c_le_s(20, 0);
                                        let t = c.fcc();
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_12b3c4;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] = 0u64;
                                            break 'b_12b3c4;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_12b3fc;
                                    } else {
                                        c.cycles += 8;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                                        call_ptr(c, 0x0026fa50, 0x0012b3d8)?;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x480), c.g[S4][0]);
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x00125108, 0x0012b3e8)?;
                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x7c));
                                        break 'b_12b3ec;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0012b398);
                                }
                                continue 'l_12b398;
                            } else {
                                break 'b_12b3fc;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_12b400;
            }
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

/// `sra` (arithmetic), then `andi 1`.
#[inline(always)]
pub(crate) fn bit(v: u64, n: u32) -> u64 {
    (((v as u32) as i32 >> n) as u32 & 1) as u64
}

/// `srl`, then `andi 1`.
#[inline(always)]
pub(crate) fn ubit(v: u64, n: u32) -> u64 {
    ((v as u32) >> n & 1) as u64
}

/// `AI_ApplyComputerRiderSettings(self, rider)` (0x12b698): apply the AI
/// options to a computer rider. The boost setting comes from option bits 3,
/// 0 and 5 (3 disables, 0 zeroes, 5 lowers it, else 1); then the boost meter
/// is reset, the boost time and boost enabling follow bits 3 and 1, and the
/// rider's handicap is stored last.
pub fn ai_apply_computer_rider_settings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let rider = abi::arg(c, 1);
        c.g[V0][0] = 0x53_0000;
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        c.g[S1][0] = c.g[V0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
        c.g[S0][0] = c.g[A1][0];
        c.g[V1][0] = lw_5(c, p(c.g[V0][0] as u32, 0x18));
        c.g[V0][0] = bit(c.g[V1][0], 3) as u64;
        c.g[A1][0] = 1;
        if c.g[V0][0] == 0 {
            // option bit 3 clear: test bit 0 of the same word
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & 1;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[V1][0] = 2;
                c.g[V0][0] = ubit(lw_5(c, p(c.g[S1][0] as u32, 0x8b8)), 5);
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = c.g[V1][0];
                }
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0;
            }
        } else {
            c.cycles += 2;
            c.g[A1][0] = 3;
        }
        c.cycles += 8;
        let state = lw_5(c, p(rider, 0x18));
        c.g[A0][0] = state;
        c.sw(p(state as u32, 0x304), c.g[A1][0]);
        Callee {
            addr: 0x0012_0d58,
            f: ee::s00::Rider_ResetBoostMeter,
        }
        .call(c, 0x0012_b704)?;
        c.g[V0][0] = ubit(lw_5(c, p(c.g[S1][0] as u32, 0x8b8)), 3);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8b8));
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = lw_5(c, p(rider, 0x18));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.swc1(0, p(c.g[V0][0] as u32, 0xb24));
        }
        c.cycles += 5;
        c.g[V0][0] = lw_5(c, p(c.g[A0][0] as u32, 0x18));
        c.g[V0][0] = bit(c.g[V0][0], 4) as u64;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = 2;
        if !t {
            c.cycles += 2;
            c.g[V1][0] = lw_5(c, p(rider, 0x18));
            c.sw(p(c.g[V1][0] as u32, 0xb28), c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[V0][0] = lw_5(c, p(c.g[A0][0] as u32, 0x18));
        c.g[V0][0] = bit(c.g[V0][0], 1) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = lw_5(c, p(rider, 0x18));
            c.g[A1][0] = 1;
            Callee {
                addr: 0x0012_5958,
                f: ee::s00::Rider_SetBoostEnabled,
            }
            .call(c, 0x0012_b764)?;
        }
        c.cycles += 9;
        call_ptr(c, 0x0014_f7e8, 0x0012_b76c)?;
        c.g[V1][0] = lw_5(c, p(rider, 0x18));
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        c.sw(p(c.g[V1][0] as u32, 0xb30), c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `RiderGateSub_UpdatePush(self)` (0x12c130): the rider-gate push
/// sub-state. Snap the slew targets to 1/0, then either the gate animation
/// finished (go to the sub-state after the push) or, while the rider is in
/// main state 3, wait for the push animation to pass halfway before jumping
/// to main state 1 and sub-state 0.
pub fn rider_gate_sub_update_push(c: &mut Cpu) {
    const SET_SLEW_TARGETS: Callee = Callee {
        addr: 0x0011_3f88,
        f: ee::s00::Rider_SetSlew3And4Targets,
    };
    const GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };
    const SET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe78,
        f: ee::s00::Rider_SetMainState,
    };
    const SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::Rider_SetSubState,
    };
    const ANIM_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;
    const ANIM_GET_LAYER_PROGRESS: u32 = 0x0031_2ab0;

    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12); // f12 = 1.0
        frame.save(c, 16, 0x40);
        frame.save_ra(c, 0x30);
        c.g[S0][0] = c.g[A0][0];
        c.mtc1(0, 13); // f13 = 0.0
        c.g[A0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
        SET_SLEW_TARGETS.call(c, 0x0012_c154)?;
        c.g[V0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
        c.g[A1][0] = 2;
        c.g[A0][0] = lw_6(c, p(c.g[V0][0] as u32, 0x784));
        call_ptr(c, ANIM_GET_LAYER_ANIM_ID, 0x0012_c164)?;
        c.g[V1][0] = 2;
        let anim_done = c.g[V0][0] != c.g[V1][0];
        if anim_done {
            // leave the push: main state 1 -> sub-state 4; else sub-state 0
            c.cycles += 5;
            c.g[A0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
            GET_MAIN_STATE.call(c, 0x0012_c1f8)?;
            c.g[V1][0] = 1;
            let not_state_one = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
            if !not_state_one {
                c.cycles += 4;
                c.g[A1][0] = 4;
                SET_SUB_STATE.call(c, 0x0012_c20c)?;
                sw_2(c, c.g[S0][0] as u32, 0);
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0;
                SET_SUB_STATE.call(c, 0x0012_c21c)?;
                sw_2(c, c.g[S0][0] as u32, 0);
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x40));
        } else {
            c.cycles += 5;
            c.g[A0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
            GET_MAIN_STATE.call(c, 0x0012_c178)?;
            c.g[V1][0] = 3;
            if c.g[V0][0] != c.g[V1][0] {
                c.lq(16, frame.at(c, 0x40));
            } else {
                c.cycles += 10;
                c.g[V0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
                c.g[A1][0] = 2;
                c.g[A0][0] = lw_6(c, p(c.g[V0][0] as u32, 0x784));
                call_ptr(c, ANIM_GET_LAYER_PROGRESS, 0x0012_c194)?;
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 1); // f1 = 0.5
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    // progress < 0.5: keep pushing
                    c.lq(16, frame.at(c, 0x40));
                } else {
                    c.cycles += 17;
                    c.g[V1][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
                    c.g[A1][0] = 0;
                    c.lwc1(0, gp(c, -0x7838));
                    c.lq(2, p(c.g[V1][0] as u32, 0x1b0));
                    c.sq(2, frame.at(c, 0x10));
                    c.mfc1(4, 0);
                    c.lqc2(4, frame.at(c, 0x10));
                    c.qmtc2(4, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x20));
                    c.lq(2, frame.at(c, 0x20));
                    c.sq(2, frame.at(c, 0));
                    c.sq(2, p(c.g[V1][0] as u32, 0x1e0));
                    c.g[A0][0] = lw_6(c, p(c.g[S0][0] as u32, 0x14));
                    SET_MAIN_STATE.call(c, 0x0012_c1e8)?;
                    c.lq(16, frame.at(c, 0x40));
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `int RiderBoardPressSub_CheckRelease(Ai* self, const AiStick* stick)`
/// (0x131200): hold-to-charge release check: accumulate the press timer
/// while the stick is neutral (|y| < 0.5), scale it by the rider's rate, and
/// report a release (return 1) when the timer, the rider's lean state or the
/// current charge says so.
pub fn rider_board_press_sub_check_release(c: &mut Cpu) {
    'b_131280: {
        'b_13127c: {
            c.cycles += 16;
            c.g[V1][0] = c.ldw_2(A1, 0x4);
            c.lwc1(3, c.pa(GP, -0x7614));
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 26);
            c.mtc1(0, 2);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 20);
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 26) as u32);
            c.mtc1(3, 0);
            c.cvt_s_w(0, 0);
            c.mul_s(4, 1, 3);
            c.c_eq_s(4, 2);
            let t = !c.fcc();
            c.mul_s(0, 0, 3);
            if t {
                break 'b_13127c;
            } else {
                c.cycles += 7;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 1);
                c.abs_s(0, 0);
                c.c_lt_s(0, 1);
                if !c.fcc() {
                    c.stw_2(A0, 0xc, 0u64);
                    break 'b_131280;
                } else {
                    c.cycles += 8;
                    c.g[V0][0] = c.ldw_2(A0, 0x14);
                    c.lwc1(2, c.pa(GP, -0x7610));
                    c.lwc1(0, c.pa(V0, 0x300));
                    c.lwc1(1, c.pa(A0, 0xc));
                    c.mul_s(0, 0, 2);
                    c.add_s(1, 1, 0);
                    let t = true;
                    c.swc1(1, c.pa(A0, 0xc));
                    if t {
                        break 'b_131280;
                    } else {
                        break 'b_13127c;
                    }
                }
            }
        }
        c.cycles += 1;
        c.stw_2(A0, 0xc, 0u64);
        break 'b_131280;
    }
    'b_131320: {
        'b_1312e4: {
            'b_1312e0: {
                c.cycles += 7;
                c.lwc1(1, c.pa(A0, 0xc));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 0);
                c.c_lt_s(0, 1);
                let t = c.fcc();
                c.g[V1][0] = c.ldw_2(A0, 0x14);
                if t {
                    break 'b_1312e0;
                } else {
                    'b_1312c4: {
                        c.cycles += 4;
                        c.g[A1][0] = c.ldw_2(V1, 0x330);
                        c.g[V0][0] = 0x2;
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_1312c4;
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, c.pa(GP, -0x760c));
                            c.c_lt_s(0, 4);
                            if c.fcc() {
                                c.lwc1(0, c.pa(V1, 0x300));
                                break 'b_1312e4;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = 0x1;
                                break 'b_1312c4;
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[A1][0] != c.g[V0][0] {
                        c.g[V1][0] = c.ldw_2(A0, 0x14);
                        break 'b_131320;
                    } else {
                        c.cycles += 5;
                        c.lwc1(0, c.pa(GP, -0x7608));
                        c.c_lt_s(4, 0);
                        if !c.fcc() {
                            c.g[V1][0] = c.ldw_2(A0, 0x14);
                            break 'b_131320;
                        } else {
                            break 'b_1312e0;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lwc1(0, c.pa(V1, 0x300));
            break 'b_1312e4;
        }
        c.cycles += 13;
        c.lwc1(1, c.pa(GP, -0x7604));
        c.add_s(0, 0, 0);
        c.stw_2(V1, 0x27c, 0u64);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 2);
        c.mul_s(0, 0, 1);
        c.swc1(0, c.pa(V1, 0x278));
        c.g[V0][0] = c.ldw_2(A0, 0x14);
        c.lwc1(0, c.pa(V0, 0x274));
        c.c_lt_s(0, 2);
        if !c.fcc() {
            c.g[V1][0] = c.ldw_2(A0, 0x14);
            break 'b_131320;
        } else {
            c.cycles += 2;
            let ra = c.g[RA][0] as u32;
            c.g[V0][0] = 0x1;
            c.pc = ra;
            return;
        }
    }
    'b_13133c: {
        c.cycles += 6;
        c.mtc1(0, 1);
        c.lwc1(0, c.pa(V1, 0x274));
        c.c_eq_s(0, 1);
        let t = c.fcc();
        c.g[V0][0] = 0x1;
        if t {
            break 'b_13133c;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_13133c;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
    return;
}

/// `float RiderSpinSub_GetHUDDirAngle(self, int* out)` (0x135f70): the HUD
/// ring direction: 0 when both signs are zero (and `*out = 1`), else the
/// angle of (y, x) from `Math_Atan2`, wrapped into [-pi, pi] (or
/// [-0.5pi, 1.5pi]) depending on the sign combination.
pub fn rider_spin_sub_get_hud_dir_angle(c: &mut Cpu) {
    'b_135fc4: {
        'b_135fc0: {
            'b_135fb8: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.mtc1(0, 12);
                c.std(SP, 0, c.g[RA][0]);
                c.swc1(22, c.pa(SP, 0x20));
                c.swc1(21, c.pa(SP, 0x18));
                c.swc1(20, c.pa(SP, 0x10));
                c.lwc1(0, c.pa(A0, 0x18));
                c.c_le_s(12, 0);
                let t = !c.fcc();

                if t {
                    break 'b_135fb8;
                } else {
                    c.cycles += 4;
                    c.c_lt_s(12, 0);
                    let t = !c.fcc();
                    c.mov_s(21, 12);
                    if t {
                        break 'b_135fc0;
                    } else {
                        c.cycles += 4;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 21);
                        let t = true;
                        c.lwc1(1, c.pa(A0, 0x1c));
                        if t {
                            break 'b_135fc4;
                        } else {
                            break 'b_135fb8;
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[AT][0] = 0xffffffffbf800000;
            c.mtc1(1, 21);
            break 'b_135fc0;
        }
        c.cycles += 1;
        c.lwc1(1, c.pa(A0, 0x1c));
        break 'b_135fc4;
    }
    'b_136000: {
        'b_135ffc: {
            'b_135ff4: {
                c.cycles += 4;
                c.mtc1(0, 0);
                c.c_le_s(0, 1);
                let t = !c.fcc();

                if t {
                    break 'b_135ff4;
                } else {
                    c.cycles += 4;
                    c.c_lt_s(0, 1);
                    let t = !c.fcc();
                    c.mov_s(22, 0);
                    if t {
                        break 'b_135ffc;
                    } else {
                        c.cycles += 4;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 22);
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_136000;
                        } else {
                            break 'b_135ff4;
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[AT][0] = 0xffffffffbf800000;
            c.mtc1(1, 22);
            break 'b_135ffc;
        }
        c.cycles += 1;
        c.g[V0][0] = 0u64;
        break 'b_136000;
    }
    'b_13601c: {
        c.cycles += 3;
        c.c_eq_s(21, 0);
        let t = !c.fcc();

        if t {
            break 'b_13601c;
        } else {
            c.cycles += 4;
            c.c_eq_s(22, 0);
            if c.fcc() {
                c.g[V0][0] = 0x1;
                break 'b_13601c;
            } else {
                break 'b_13601c;
            }
        }
    }
    'b_1360b4: {
        'b_1360b0: {
            'b_136030: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.stw_2(A1, 0, c.g[V0][0]);
                if t {
                    break 'b_136030;
                } else {
                    c.cycles += 3;
                    c.mtc1(0, 0);
                    let t = true;
                    c.g[RA][0] = c.ldd(SP, 0);
                    if t {
                        break 'b_1360b0;
                    } else {
                        break 'b_136030;
                    }
                }
            }
            'b_1360ac: {
                'b_13606c: {
                    c.cycles += 4;
                    c.mtc1(0, 20);
                    c.c_eq_s(22, 20);
                    let t = !c.fcc();

                    if t {
                        break 'b_13606c;
                    } else {
                        c.cycles += 4;
                        c.c_eq_s(21, 20);
                        let t = c.fcc();
                        c.mov_s(0, 21);
                        if t {
                            break 'b_1360ac;
                        } else {
                            c.cycles += 4;
                            c.c_le_s(20, 21);
                            c.lwc1(0, c.pa(GP, -0x7318));
                            let t = !c.fcc();
                            c.g[RA][0] = c.ldd(SP, 0);
                            if t {
                                break 'b_1360ac;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, c.pa(GP, -0x7314));
                                let t = true;
                                c.lwc1(22, c.pa(SP, 0x20));
                                if t {
                                    break 'b_1360b4;
                                } else {
                                    break 'b_13606c;
                                }
                            }
                        }
                    }
                }
                'b_1360a8: {
                    c.cycles += 7;
                    c.div_s(12, 21, 22);
                    c.g[RA][0] = sx(0x00136078);
                    c.pc = 0x0031c228;
                    call_indirect(c, 0x0031c228);
                    if c.pc != 0x00136078 {
                        return;
                    }
                    c.c_lt_s(22, 20);
                    let t = !c.fcc();
                    c.mov_s(12, 0);
                    if t {
                        break 'b_1360a8;
                    } else {
                        'b_1360a0: {
                            c.cycles += 3;
                            c.c_lt_s(20, 21);
                            let t = !c.fcc();

                            if t {
                                break 'b_1360a0;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, c.pa(GP, -0x7310));
                                let t = true;
                                c.add_s(12, 12, 0);
                                if t {
                                    break 'b_1360a8;
                                } else {
                                    break 'b_1360a0;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.lwc1(0, c.pa(GP, -0x730c));
                        c.sub_s(12, 12, 0);
                        break 'b_1360a8;
                    }
                }
                c.cycles += 1;
                c.mov_s(0, 12);
                break 'b_1360ac;
            }
            c.cycles += 1;
            c.g[RA][0] = c.ldd(SP, 0);
            break 'b_1360b0;
        }
        c.cycles += 1;
        c.lwc1(22, c.pa(SP, 0x20));
        break 'b_1360b4;
    }
    c.cycles += 4;
    c.lwc1(21, c.pa(SP, 0x18));
    c.lwc1(20, c.pa(SP, 0x10));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = ra;
    return;
}

/// `void RiderBoardPressSub_FlipStance(Ai* self)` (0x12fec8): the board-press
/// sub-state's stance flip: toggle the rider's stance bit, build the
/// stance-scaled leg spread from `sin/cos` of the board angle, mirror the
/// rider's two roll axes, and notify the board-press handler.
pub fn rider_board_press_sub_flip_stance(c: &mut Cpu) {
    'b_12ff10: {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.mtc1(0, 0);
        c.sq(19, c.pa(SP, 0x30));
        c.sq(16, c.pa(SP, 0x60));
        c.g[S3][0] = c.g[A0][0];
        c.sq(17, c.pa(SP, 0x50));
        c.sq(18, c.pa(SP, 0x40));
        c.std(SP, 0x20, c.g[RA][0]);
        c.g[V0][0] = c.ldw_2(S3, 0x14);
        c.g[V1][0] = c.ldw_2(V0, 0x320);
        c.g[V1][0] = c.g[V1][0] ^ 0x1;
        c.stw_2(V0, 0x320, c.g[V1][0]);
        c.g[V1][0] = c.ldw_2(S3, 0x14);
        c.g[V0][0] = c.ldw_2(V1, 0x320);
        if c.g[V0][0] == 0u64 {
            c.neg_s(0, 0);
            break 'b_12ff10;
        } else {
            c.cycles += 2;
            c.lwc1(0, c.pa(GP, -0x76cc));
            c.neg_s(0, 0);
            break 'b_12ff10;
        }
    }
    'b_12ffb0: {
        'b_12ffa8: {
            c.cycles += 36;
            c.g[V0][0] = 0x500000;
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 12);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
            c.g[S2][0] = c.ldw_2(V1, 0x784);
            c.g[S1][0] = 0x500000;
            c.lq(3, c.pa(V0, 0));
            c.mul_s(12, 0, 12);
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
            c.sq(3, c.pa(S2, 0x30));
            c.g[RA][0] = sx(0x0012ff44);
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xea0));
            c.pc = 0x0031be50;
            call_indirect(c, 0x0031be50);
            if c.pc != 0x0012ff44 {
                return;
            }
            c.lwc1(1, c.pa(S0, 0x8));
            c.g[A1][0] = 0x1;
            c.lwc1(0, c.pa(SP, 0x10));
            c.lwc1(2, c.pa(S1, -0xea0));
            c.lwc1(4, c.pa(S0, 0x4));
            c.mul_s(1, 0, 1);
            c.mul_s(2, 0, 2);
            c.lwc1(3, c.pa(SP, 0x14));
            c.mul_s(0, 0, 4);
            c.swc1(3, c.pa(SP, 0xc));
            c.swc1(1, c.pa(SP, 0x8));
            c.swc1(2, c.pa(SP, 0));
            c.swc1(0, c.pa(SP, 0x4));
            c.lq(2, c.pa(SP, 0));
            c.sq(2, c.pa(S2, 0x40));
            c.g[V1][0] = c.ldw_2(S3, 0x14);
            c.g[A0][0] = c.ldw_2(V1, 0x320);
            c.g[V0][0] = c.ldw_2(V1, 0x784);
            c.stw_2(V0, 0x18, c.g[A0][0]);
            c.g[V1][0] = c.ldw_2(S3, 0x14);
            c.g[A0][0] = c.ldw_2(V1, 0x328);
            let t = c.g[A0][0] != c.g[A1][0];
            c.g[V0][0] = 0x2;
            if t {
                break 'b_12ffa8;
            } else {
                c.cycles += 2;
                let t = true;
                c.stw_2(V1, 0x328, c.g[V0][0]);
                if t {
                    break 'b_12ffb0;
                } else {
                    break 'b_12ffa8;
                }
            }
        }
        c.cycles += 2;
        if c.g[A0][0] == c.g[V0][0] {
            c.stw_2(V1, 0x328, c.g[A1][0]);
            break 'b_12ffb0;
        } else {
            break 'b_12ffb0;
        }
    }
    c.cycles += 17;
    c.g[V0][0] = c.ldw_2(S3, 0x14);
    c.lwc1(0, c.pa(V0, 0x280));
    c.neg_s(0, 0);
    c.swc1(0, c.pa(V0, 0x280));
    c.g[V1][0] = c.ldw_2(S3, 0x14);
    c.lwc1(0, c.pa(V1, 0x288));
    c.neg_s(0, 0);
    c.swc1(0, c.pa(V1, 0x288));
    c.g[RA][0] = sx(0x0012ffd8);
    c.g[A0][0] = c.ldw_2(S3, 0x14);
    c.pc = 0x00116930;
    call_indirect(c, 0x00116930);
    if c.pc != 0x0012ffd8 {
        return;
    }
    c.lq(16, c.pa(SP, 0x60));
    c.lq(17, c.pa(SP, 0x50));
    c.lq(18, c.pa(SP, 0x40));
    c.lq(19, c.pa(SP, 0x30));
    c.g[RA][0] = c.ldd(SP, 0x20);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    c.pc = ra;
    return;
}

/// `void func_0012C408(void* gate, void* a1)` (0x12c408): per-frame launch
/// gate charge update. Resets the rider's gate-charge fields to the charge
/// angle, ramps the charge at `+0x10` towards 1 by 0.015 (clamped), sets the
/// slew targets, and when the rider is launched (state 3, charge >= 0.62)
/// scales the rider's rotation and re-enters the idle state; at exactly 1.0
/// it plays the gate animation and switches the sub-state.
pub fn rider_gate_sub_update_launch(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    'b_12c46c: {
        'b_12c450: {
            c.cycles += 16;
            frame.save(c, S0, 0x40);
            frame.save_ra(c, 0x30);
            c.g[S0][0] = c.g[A0][0];
            c.g[RA][0] = sx(0x0012c420);
            c.g[A0][0] = lw_7(c, at(c, S0, 0x14));
            if callee::C00115AB0.call(c, 0x0012c420).is_err() {
                return;
            }
            c.g[V0][0] = lw_7(c, at(c, S0, 0x14));
            c.lwc1(1, r32(c, GP).wrapping_sub(0x780c));
            c.swc1(0, at(c, V0, 0x2c8));
            sw(c, at(c, V0, 0x2cc), 0u64);
            c.swc1(0, at(c, V0, 0x2d0));
            c.lwc1(3, r32(c, GP).wrapping_sub(0x7808));
            // The launch's progress steps 0.015 per 60 Hz tick (not a 1/60
            // entry: the scan missed it). At 1.0 the rider goes back to
            // riding; 4x early at 240 Hz that is still in the air off the
            // gate, where a held stick starts a flip and the landing crashes.
            crate::game::ai::rider::per_tick_60(c, 3);
            c.lwc1(2, at(c, S0, 0x10));
            c.c_lt_s(1, 2);
            let t = !c.fcc();

            if t {
                break 'b_12c450;
            } else {
                c.cycles += 2;
                let t = true;
                c.sub_s(1, 2, 3);
                if t {
                    break 'b_12c46c;
                } else {
                    break 'b_12c450;
                }
            }
        }
        c.cycles += 7;
        c.lwc1(0, r32(c, GP).wrapping_sub(0x7804));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 1);
        c.c_lt_s(2, 0);
        if c.fcc() {
            c.add_s(1, 2, 3);
            break 'b_12c46c;
        } else {
            break 'b_12c46c;
        }
    }
    'b_12c4e8: {
        c.cycles += 11;
        c.g[A0][0] = lw_7(c, at(c, S0, 0x14));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 12);
        c.mtc1(0, 13);
        c.g[RA][0] = sx(0x0012c484);
        c.swc1(1, at(c, S0, 0x10));
        if callee::C00113F88.call(c, 0x0012c484).is_err() {
            return;
        }
        c.g[RA][0] = sx(0x0012c48c);
        c.g[A0][0] = lw_7(c, at(c, S0, 0x14));
        if callee::C0011FE98.call(c, 0x0012c48c).is_err() {
            return;
        }
        c.g[V1][0] = 0x3;
        let t = c.g[V0][0] != c.g[V1][0];
        c.lwc1(1, at(c, S0, 0x10));
        if t {
            break 'b_12c4e8;
        } else {
            c.cycles += 5;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x7800));
            c.c_le_s(0, 1);
            let t = !c.fcc();
            c.g[A1][0] = 0u64;
            if t {
                break 'b_12c4e8;
            } else {
                c.cycles += 15;
                c.g[V1][0] = lw_7(c, at(c, S0, 0x14));
                c.lwc1(0, at(c, S0, 0xc));
                lq(c, 2, at(c, V1, 0x1b0));
                sq(c, 2, at(c, SP, 0x10));
                c.mfc1(4, 0);
                c.lqc2(4, at(c, SP, 0x10));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, SP, 0x20));
                lq(c, 2, at(c, SP, 0x20));
                sq(c, 2, r32(c, SP));
                sq(c, 2, at(c, V1, 0x1e0));
                c.g[RA][0] = sx(0x0012c4e4);
                c.g[A0][0] = lw_7(c, at(c, S0, 0x14));
                if callee::C0011FE78.call(c, 0x0012c4e4).is_err() {
                    return;
                }
                c.lwc1(1, at(c, S0, 0x10));
                break 'b_12c4e8;
            }
        }
    }
    'b_12c548: {
        'b_12c544: {
            c.cycles += 6;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.c_eq_s(1, 0);
            let t = !c.fcc();
            c.g[A2][0] = 0u64;
            if t {
                break 'b_12c544;
            } else {
                'b_12c53c: {
                    c.cycles += 11;
                    c.g[V0][0] = lw_7(c, at(c, S0, 0x14));
                    c.g[A1][0] = 0x6;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 12);
                    c.g[RA][0] = sx(0x0012c518);
                    c.g[A0][0] = lw_7(c, at(c, V0, 0x784));
                    if call_ptr(c, 0x003128e8, 0x0012c518).is_err() {
                        return;
                    }
                    c.g[RA][0] = sx(0x0012c520);
                    c.g[A0][0] = lw_7(c, at(c, S0, 0x14));
                    if callee::C0011FE98.call(c, 0x0012c520).is_err() {
                        return;
                    }
                    c.g[V1][0] = 0x1;
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[A0][0] = lw_7(c, at(c, S0, 0x14));
                    if t {
                        break 'b_12c53c;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0012c534);
                        c.g[A1][0] = 0x4;
                        if callee::C0011FEC8.call(c, 0x0012c534).is_err() {
                            return;
                        }
                        let t = true;
                        frame.restore(c, S0, 0x40);
                        if t {
                            break 'b_12c548;
                        } else {
                            break 'b_12c53c;
                        }
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x0012c544);
                c.g[A1][0] = 0u64;
                if callee::C0011FEC8.call(c, 0x0012c544).is_err() {
                    return;
                }
                break 'b_12c544;
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x40);
        break 'b_12c548;
    }
    c.cycles += 3;
    c.g[RA][0] = ld(c, at(c, SP, 0x30));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x50));
    c.pc = t;
    return;
}

/// `void RiderResetSub_Enter(RiderResetSub *self)` (0x12f230): clear the
/// reset flag and the animation's +0x90 word; for the local player, stop a
/// running transition and start a white fade from two 1.0-alpha effects.
pub fn rider_reset_sub_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        c.g[A1][0] = 2;
        frame.save(c, 16, 0x70);
        frame.save(c, 20, 0x30);
        let this64 = abi::arg64(c, 0);
        c.g[S0][0] = this64;
        frame.save(c, 17, 0x60);
        let this = this64 as u32;
        c.g[S4][0] = sx(this.wrapping_add(4));
        frame.save(c, 18, 0x50);
        frame.save(c, 19, 0x40);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0x80);
        c.sw(this.wrapping_add(4), 0);
        c.g[V0][0] = c.lw(this.wrapping_add(8));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        callee::ANIM_GET_LAYER_SEQUENCE.call(c, 0x0012_f26c)?;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x90), 0);

        c.g[A0][0] = c.g[S0][0];
        callee::IS_LOCAL_PLAYER.call(c, 0x0012_f278)?;
        let not_local = c.g[V0][0] == 0;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if !not_local {
            c.cycles += 8;
            c.g[V0][0] = c.lw(this.wrapping_add(8));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x870));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            callee::GET_TRANSITION_MAN.call(c, 0x0012_f290)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x44));
            let idle = c.g[V0][0] == 0;
            if !idle {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                callee::TRANSITION_STOP.call(c, 0x0012_f2a8)?;
            }

            c.cycles += 51;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 20); // f20 = 1.0
            c.g[A0][0] = 0x14;
            callee::EFFECT_ALLOC.call(c, 0x0012_f2b8)?;
            c.g[S2][0] = c.g[V0][0];
            // The white colour {1, 1, 1, 1}.
            for off in [0u32, 4, 8, 0xc] {
                c.swc1(20, frame.at(c, off));
            }
            c.g[A0][0] = c.g[S2][0];
            callee::EFFECT_CTOR.call(c, 0x0012_f2d4)?;
            c.g[S0][0] = 0x0049_0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7f40));
            c.sw(c.g[S2][0] as u32, c.g[S0][0]);
            c.ldl(2, (c.g[SP][0] as u32).wrapping_add(7));
            c.ldr(2, c.g[SP][0] as u32);
            c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
            c.ldr(3, (c.g[SP][0] as u32).wrapping_add(8));
            c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0xb));
            c.sdr(2, (c.g[S2][0] as u32).wrapping_add(4));
            c.sdl(3, (c.g[S2][0] as u32).wrapping_add(0x13));
            c.sdr(3, (c.g[S2][0] as u32).wrapping_add(0xc));

            c.g[A0][0] = 0x14;
            callee::EFFECT_ALLOC.call(c, 0x0012_f308)?;
            c.g[S1][0] = c.g[V0][0];
            for off in [0x1c, 0x10, 0x14, 0x18] {
                c.swc1(20, frame.at(c, off));
            }
            c.g[A0][0] = c.g[S1][0];
            callee::EFFECT_CTOR.call(c, 0x0012_f324)?;

            // The start call: 0.5 fade, 0.0 delay, 0.5 out.
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S3][0];
            c.sw(c.g[S1][0] as u32, c.g[S0][0]);
            c.g[A2][0] = c.g[S2][0];
            c.mtc1(0, 13);
            c.g[A3][0] = c.g[S1][0];
            c.g[T1][0] = c.g[S4][0];
            c.g[A1][0] = 1;
            c.mov_s(14, 12);
            c.ldl(10, (c.g[SP][0] as u32).wrapping_add(0x17));
            c.ldr(10, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.ldl(11, (c.g[SP][0] as u32).wrapping_add(0x1f));
            c.ldr(11, (c.g[SP][0] as u32).wrapping_add(0x18));
            c.sdl(10, (c.g[S1][0] as u32).wrapping_add(0xb));
            c.sdr(10, (c.g[S1][0] as u32).wrapping_add(4));
            c.sdl(11, (c.g[S1][0] as u32).wrapping_add(0x13));
            c.sdr(11, (c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[T0][0] = 0;
            callee::TRANSITION_START.call(c, 0x0012_f374)?;
        }

        c.cycles += 9;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod spin_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    /// `Rider_CheckResetPath(rider, force, reason)`.
    pub const RIDER_CHECK_RESET_PATH: u32 = 0x0011_6120;

    /// `Rider_TryStartHandplant(rider, bool dir)`.
    pub const RIDER_TRY_START_HANDPLANT: u32 = 0x0010_7578;

    /// `Rider_TryStartRailslide(rider)`: jumps onto a rail found near the
    /// rider (`Rider_FindRail`); 1 when it did.
    pub const RIDER_TRY_START_RAILSLIDE: u32 = 0x0010_6848;

    /// `Rider_HandleBoostInput(rider, 0, 0)`.
    pub const RIDER_HANDLE_BOOST_INPUT: u32 = 0x0011_4130;

    /// `AnimController_GetLayerAnimId(ctrl, layer)`.
    pub const ANIM_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;

    /// `Input_QuantizeStickDirection(f12, f13, float* x, float* y)`.
    pub const INPUT_QUANTIZE_STICK_DIRECTION: u32 = 0x0011_58b8;

    /// `RiderMetrix_StartSpinTrick(metrix, bool switched)`: returns boost.
    pub const METRIX_START_SPIN_TRICK: u32 = 0x0011_9c98;

    /// `Rider_AddBoost(rider, f12 amount, 1)`.
    pub const RIDER_ADD_BOOST: u32 = 0x0010_e098;

    /// `Math_Atan(f12)`.
    pub const MATH_ATAN: u32 = 0x0031_c228;

    /// `RiderMetrix_SetSpinAmount(metrix, f12)`: returns boost.
    pub const METRIX_SET_SPIN_AMOUNT: u32 = 0x0011_9898;

    /// `RiderMetrix_SetFlipAmount(metrix, f12)`: returns boost.
    pub const METRIX_SET_FLIP_AMOUNT: u32 = 0x0011_98d8;

    /// `AnimController_GetLayerAnimEntry(ctrl, layer)`.
    pub const ANIM_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;

    /// `AnimController_IsLayerAnimFinished(ctrl, layer)`.
    pub const ANIM_IS_LAYER_ANIM_FINISHED: u32 = 0x0031_2ae8;

    /// `AnimController_GetCurrentAnimLength(ctrl, id)`.
    pub const ANIM_GET_CURRENT_ANIM_LENGTH: u32 = 0x0031_2790;

    /// `AnimController_PlayAnim(ctrl, id, f12 blend, 0)`.
    pub const ANIM_PLAY_ANIM: u32 = 0x0031_28e8;

    pub const UPDATE_TRICK: Callee = Callee {
        addr: 0x0013_52a8,
        f: ee::s01::RiderSpinSub_UpdateTrick,
    };

    pub const TRY_REVERSE: Callee = Callee {
        addr: 0x0013_5be0,
        f: ee::s01::RiderSpinSub_TryReverse,
    };

    pub const SET_SPIN_DIRECTION: Callee = Callee {
        addr: 0x0013_5b30,
        f: ee::s01::RiderSpinSub_SetSpinDirection,
    };

    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };

    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };

    pub const BE_CHAR_INT_GET_ATTRIB4_FACTOR: Callee = Callee {
        addr: 0x0014_95a8,
        f: ee::s01::BECharInt_GetAttrib4Factor,
    };
}

pub(crate) mod spin_sub {

    /// Spin-hold mode (int): 0 none, 2 hold button with the stick out, 1
    /// hold released, 3 parked while a trick plays.
    pub const MODE: u32 = 0x0;

    /// Spin phase (int): 3 idle, 0 started, 1 axes moving, 2 settling.
    pub const PHASE: u32 = 0xc;

    pub const FLIP_TARGET: u32 = 0x10;

    pub const SPIN_TARGET: u32 = 0x14;

    pub const FLIP: u32 = 0x18;

    pub const SPIN: u32 = 0x1c;

    /// Accumulated rotation (for the metrix and the HUD).
    pub const SPIN_TOTAL: u32 = 0x20;

    pub const FLIP_TOTAL: u32 = 0x24;

    /// The smoothed body lean from the stick (rider `+0x2a0` / `+0x294`).
    pub const LEAN_X: u32 = 0x28;

    pub const LEAN_Y: u32 = 0x2c;

    pub const SPIN_AMOUNT: u32 = 0x30;

    pub const FLIP_AMOUNT: u32 = 0x34;

    /// Rotation rates (rider `+0x2dc` / `+0x2e0` follow them).
    pub const SPIN_RATE: u32 = 0x38;

    pub const FLIP_RATE: u32 = 0x3c;

    /// Set (int 1) once a further half turn was queued.
    pub const TURNED: u32 = 0x44;

    /// How long the stick has held each axis.
    pub const SPIN_HELD: u32 = 0x48;

    pub const FLIP_HELD: u32 = 0x4c;

    /// The stick angle (`atan`, or a fixed angle on an axis).
    pub const STICK_ANGLE: u32 = 0x50;

    /// Time since the stick was released.
    pub const RELEASED: u32 = 0x54;

    /// The owning rider.
    pub const RIDER: u32 = 0x58;

    /// Rider: the animation controller (layer 2 is the trick layer;
    /// `+0x1c` its playback speed).
    pub const R_ANIM: u32 = 0x784;

    /// Rider: the animation state (`+0xac` kind, `+0x98`/`+0xa0` times).
    pub const R_ANIM_STATE: u32 = 0x788;

    pub const R_METRIX: u32 = 0x790;

    pub const R_28C: u32 = 0x28c;

    pub const R_LEAN_Y_BLEND: u32 = 0x290;

    pub const R_LEAN_Y: u32 = 0x294;

    pub const R_298: u32 = 0x298;

    pub const R_LEAN_X_BLEND: u32 = 0x29c;

    pub const R_LEAN_X: u32 = 0x2a0;

    pub const R_SPIN_SPEED: u32 = 0x2dc;

    pub const R_FLIP_SPEED: u32 = 0x2e0;

    /// Rider: positive scales both rotation speeds.
    pub const R_2EC: u32 = 0x2ec;

    /// Rider: stance (int, non-zero mirrors the spin direction) and the
    /// stance at take-off.
    pub const R_STANCE: u32 = 0x320;

    pub const R_STANCE_TAKEOFF: u32 = 0x324;

    /// Rider: lean direction from the stick (int 0, 1, 2).
    pub const R_LEAN_DIR: u32 = 0x330;

    /// Rider: the behaviour-engine player and character ids.
    pub const R_PLAYER: u32 = 0x86c;

    pub const R_CHARACTER: u32 = 0xb34;
}

pub(crate) mod spin_pad {

    pub const PAD_BUTTONS: u32 = 0x0;

    pub const PAD_STICKS: u32 = 0x4;
}

/// Stack scratch of [`rider_spin_sub_handle_input`]: the scaled stick
/// axes (`Input_QuantizeStickDirection` writes them back).
pub(crate) const STICK_FLIP: u32 = 0x0;

pub(crate) const STICK_SPIN: u32 = 0x4;

/// `reg + off` (a field of the object a register points at).
#[inline(always)]
pub(crate) fn spin_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `gp + off` (a `.lit4` constant or global).
#[inline(always)]
pub(crate) fn spin_gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `sp + off` (the stack scratch).
#[inline(always)]
pub(crate) fn spin_stack(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `void RiderSpinSub_HandleInput(RiderSpinSub* this, Pad* pad)`
/// (0x133308): the rider's in-air spin/flip input, once per tick.
///
/// - Reset (`Rider_CheckResetPath`, pad bit 12), handplant (bit 13 while
///   idle) and a rail jump (`Rider_TryStartRailslide`) take over and end
///   the function when they start.
/// - The stick axes (6-bit signed fields scaled by a `.lit4` constant)
///   and the spin-hold bit update the hold mode; `RiderSpinSub_UpdateTrick`
///   runs (it returns non-zero while a trick plays); the lean direction
///   goes to the rider; `Rider_HandleBoostInput`.
/// - Phase 3 (idle): a stick push starts a spin (unless the layer plays
///   anim 0x120 or `RiderSpinSub_TryReverse` takes it): quantized stick
///   direction, `RiderSpinSub_SetSpinDirection`, start rates,
///   `RiderMetrix_StartSpinTrick` and its boost.
/// - Phases 0..2 move the targets (`+0x10`/`+0x14`) by half turns while the
///   stick is held, count the hold times, and end the spin (phase 3) once
///   the rotation reached its targets with the stick released.
/// - The rotation speeds (rider `+0x2dc`/`+0x2e0`) come from the rates,
///   the distance to the target and the character's attribute factor
///   (`BECharInt_GetAttrib4Factor`); the lean is smoothed; the rotation
///   steps toward the targets; the amounts go to the metrix (boost).
/// - Finally picks the trick-layer animation (spin poses 0x10d..0x114 by
///   stick angle, flip poses 0x125..0x128, tweaks 0x129..0x130, landing
///   blends 0x132..0x139 from the current pose (a jump table), 0x131 /
///   0x11f) and plays it with `AnimController_PlayAnim` when it changed.
///
/// The control flow follows the original's blocks (labels are their guest
/// addresses); cycles are charged per block as the original does, and the
/// scratch registers (v0/v1/a*/f*) are left as it leaves them.
pub fn rider_spin_sub_handle_input(c: &mut Cpu) {
    use spin_pad::*;
    use spin_sub::*;
    let at = spin_at;
    let gp = spin_gp;
    let stack = spin_stack;
    guest(c, |c| {
        // The frame (0xb0): s0..s4, ra, f20..f27; sp+0 / sp+4 hold the
        // stick axes.
        let frame = Frame::open(c, 0xb0);
        'b_134c74: {
            c.cycles += 25;
            c.g[A2][0] = 0u64;
            frame.save(c, S0, 0x60);
            frame.save(c, S1, 0x50);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, S2, 0x40);
            c.g[S1][0] = c.g[A1][0];
            frame.save(c, S3, 0x30);
            frame.save(c, S4, 0x20);
            frame.save_ra(c, 0x10);
            frame.save_fpr(c, 27, 0xa8);
            frame.save_fpr(c, 26, 0xa0);
            frame.save_fpr(c, 25, 0x98);
            frame.save_fpr(c, 24, 0x90);
            frame.save_fpr(c, 23, 0x88);
            frame.save_fpr(c, 22, 0x80);
            frame.save_fpr(c, 21, 0x78);
            frame.save_fpr(c, 20, 0x70);
            c.g[A1][0] = c.lw(at(c, S1, PAD_BUTTONS));
            c.g[A0][0] = c.lw(at(c, S0, RIDER));
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 12) as u32);
            c.g[A1][0] = c.g[A1][0] & 0x1;
            call_ptr(c, spin_callee::RIDER_CHECK_RESET_PATH, 0x00133364)?;
            if c.g[V0][0] != 0u64 {
                frame.restore(c, S0, 0x60);
                break 'b_134c74;
            } else {
                'b_1333d0: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(at(c, S1, PAD_BUTTONS));
                    c.g[V0][0] = c.g[V0][0] & 0x2000;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        break 'b_1333d0;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw(at(c, S0, PHASE));
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_1333d0;
                        } else {
                            'b_1333c0: {
                                c.cycles += 13;
                                c.g[V0][0] = c.lw(at(c, S1, PAD_STICKS));
                                c.lwc1(2, gp(c, -0x7528));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
                                c.g[AT][0] = 0x3f000000;
                                c.mtc1(1, 1);
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.mul_s(0, 0, 2);
                                c.c_lt_s(1, 0);
                                let t = c.fcc();
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_1333c0;
                                } else {
                                    c.cycles += 1;
                                    c.g[A1][0] = 0u64;
                                    break 'b_1333c0;
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                            call_ptr(c, spin_callee::RIDER_TRY_START_HANDPLANT, 0x001333c8)?;
                            if c.g[V0][0] != 0u64 {
                                frame.restore(c, S0, 0x60);
                                break 'b_134c74;
                            } else {
                                break 'b_1333d0;
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.lw(at(c, S0, RIDER));
                call_ptr(c, spin_callee::RIDER_TRY_START_RAILSLIDE, 0x001333d8)?;
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x60);
                    break 'b_134c74;
                } else {
                    'b_1334bc: {
                        'b_1334b8: {
                            'b_1334b4: {
                                'b_13348c: {
                                    c.cycles += 28;
                                    c.g[A0][0] = c.lw(at(c, S1, PAD_STICKS));
                                    c.g[A2][0] = c.lw(at(c, S1, PAD_BUTTONS));
                                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 26);
                                    c.lwc1(0, gp(c, -0x7524));
                                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 26) as u32);
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                                    c.mtc1(3, 1);
                                    c.cvt_s_w(1, 1);
                                    c.mtc1(2, 2);
                                    c.cvt_s_w(2, 2);
                                    c.g[A1][0] = sx((c.g[A0][0] as u32) << 14);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 20);
                                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 26) as u32);
                                    c.mul_s(4, 1, 0);
                                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 26) as u32);
                                    c.mul_s(3, 2, 0);
                                    c.g[V0][0] = c.lw(at(c, S0, MODE));
                                    c.mtc1(4, 2);
                                    c.cvt_s_w(2, 2);
                                    c.mtc1(5, 1);
                                    c.cvt_s_w(1, 1);
                                    c.g[A0][0] = c.lb(at(c, S1, PAD_BUTTONS + 2));
                                    c.swc1(4, stack(c, STICK_FLIP));
                                    c.swc1(3, stack(c, STICK_SPIN));
                                    c.mul_s(23, 2, 0);
                                    let t = c.g[V0][0] != 0u64;
                                    c.mul_s(24, 1, 0);
                                    if t {
                                        break 'b_13348c;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.g[A2][0] & 0x8000;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V1][0] = c.lw(at(c, S0, MODE));
                                            break 'b_1334b8;
                                        } else {
                                            'b_133484: {
                                                c.cycles += 5;
                                                c.mtc1(0, 0);
                                                c.c_eq_s(24, 0);
                                                let t = !c.fcc();
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_133484;
                                                } else {
                                                    c.cycles += 4;
                                                    c.c_eq_s(23, 0);
                                                    if c.fcc() {
                                                        c.g[V1][0] = c.lw(at(c, S0, MODE));
                                                        break 'b_1334bc;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = 0x2;
                                                        break 'b_133484;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.sw(at(c, S0, MODE), c.g[V0][0]);
                                            break 'b_1334b4;
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.mtc1(0, 0);
                                c.c_eq_s(3, 0);
                                if !c.fcc() {
                                    c.sw(at(c, S0, MODE), 0u64);
                                    break 'b_1334b4;
                                } else {
                                    c.cycles += 4;
                                    c.c_eq_s(4, 0);
                                    if c.fcc() {
                                        c.g[V1][0] = c.lw(at(c, S0, MODE));
                                        break 'b_1334b8;
                                    } else {
                                        c.cycles += 1;
                                        c.sw(at(c, S0, MODE), 0u64);
                                        break 'b_1334b4;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V1][0] = c.lw(at(c, S0, MODE));
                            break 'b_1334b8;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = 0x2;
                        break 'b_1334bc;
                    }
                    'b_1334e4: {
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw(at(c, S0, MODE));
                            break 'b_1334e4;
                        } else {
                            'b_1334e0: {
                                'b_1334dc: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw(at(c, S1, PAD_BUTTONS));
                                    c.g[V0][0] = c.g[V0][0] & 0x8000;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_1334dc;
                                    } else {
                                        c.cycles += 2;
                                        c.g[A0][0] = 0xffffffffffffffff;
                                        break 'b_1334e0;
                                    }
                                }
                                c.cycles += 1;
                                c.sw(at(c, S0, MODE), c.g[V0][0]);
                                break 'b_1334e0;
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(at(c, S0, MODE));
                            break 'b_1334e4;
                        }
                    }
                    'b_133560: {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                        if c.g[V0][0] == 0u64 {
                            c.g[A2][0] = c.lw(at(c, S1, PAD_BUTTONS));
                            break 'b_133560;
                        } else {
                            'b_133528: {
                                c.cycles += 4;
                                c.mtc1(0, 0);
                                c.c_eq_s(24, 0);
                                let t = !c.fcc();

                                if t {
                                    break 'b_133528;
                                } else {
                                    c.cycles += 3;
                                    c.c_eq_s(23, 0);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_133528;
                                    } else {
                                        c.cycles += 6;
                                        c.lwc1(1, at(c, S0, RELEASED));
                                        c.lwc1(0, gp(c, -0x7520));
                                        c.c_lt_s(0, 1);
                                        if c.fcc() {
                                            c.sw(at(c, S0, MODE), 0u64);
                                            break 'b_133528;
                                        } else {
                                            break 'b_133528;
                                        }
                                    }
                                }
                            }
                            'b_133540: {
                                c.cycles += 6;
                                c.mtc1(0, 1);
                                c.lwc1(0, stack(c, STICK_SPIN));
                                c.c_eq_s(0, 1);
                                if c.fcc() {
                                    c.swc1(24, stack(c, STICK_SPIN));
                                    break 'b_133540;
                                } else {
                                    break 'b_133540;
                                }
                            }
                            'b_133554: {
                                c.cycles += 5;
                                c.lwc1(0, stack(c, STICK_FLIP));
                                c.c_eq_s(0, 1);
                                if c.fcc() {
                                    c.swc1(23, stack(c, STICK_FLIP));
                                    break 'b_133554;
                                } else {
                                    break 'b_133554;
                                }
                            }
                            c.cycles += 3;
                            c.mov_s(23, 1);
                            c.mov_s(24, 23);
                            c.g[A2][0] = c.lw(at(c, S1, PAD_BUTTONS));
                            break 'b_133560;
                        }
                    }
                    'b_133594: {
                        c.cycles += 8;
                        c.g[A1][0] = c.g[A0][0];
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 14) as u32);
                        c.g[A2][0] = c.g[A2][0] & 0x1;
                        spin_callee::UPDATE_TRICK.call(c, 0x00133574)?;
                        c.g[S2][0] = c.g[V0][0];
                        if c.g[S2][0] != 0u64 {
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            break 'b_133594;
                        } else {
                            'b_133590: {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw(at(c, S0, MODE));
                                c.g[V0][0] = 0x3;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.sw(at(c, S0, MODE), 0u64);
                                    break 'b_133590;
                                } else {
                                    break 'b_133590;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            break 'b_133594;
                        }
                    }
                    'b_1335d8: {
                        'b_1335d0: {
                            c.cycles += 7;
                            c.g[A0][0] = 0xffffffffffffffff;
                            c.sw(at(c, V0, R_LEAN_DIR), 0u64);
                            c.g[V1][0] = c.lw(at(c, S1, PAD_STICKS));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 12);
                            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 30) as u32);
                            let t = c.g[V1][0] == c.g[A0][0];
                            c.lwc1(27, gp(c, 0x1bac));
                            if t {
                                break 'b_1335d0;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = 0x1;
                                c.mtc1(0, 0);
                                let t = c.g[V1][0] != c.g[V0][0];

                                if t {
                                    break 'b_1335d8;
                                } else {
                                    c.cycles += 4;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 0);

                                    break 'b_1335d8;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[AT][0] = 0xffffffffbf800000;
                        c.mtc1(1, 0);
                        break 'b_1335d8;
                    }
                    'b_13363c: {
                        c.cycles += 5;
                        c.mtc1(0, 2);
                        c.c_eq_s(0, 2);
                        if c.fcc() {
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                            break 'b_13363c;
                        } else {
                            'b_133634: {
                                'b_133630: {
                                    'b_133624: {
                                        c.cycles += 6;
                                        c.lwc1(1, at(c, S0, LEAN_Y));
                                        c.mov_s(23, 0);
                                        c.c_lt_s(1, 2);
                                        let t = c.fcc();
                                        c.lwc1(27, gp(c, 0x1bb0));
                                        if t {
                                            break 'b_133624;
                                        } else {
                                            c.cycles += 4;
                                            c.c_eq_s(1, 2);
                                            if !c.fcc() {
                                                c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                                break 'b_133630;
                                            } else {
                                                c.cycles += 4;
                                                c.c_lt_s(23, 2);
                                                if !c.fcc() {
                                                    c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                                    break 'b_133630;
                                                } else {
                                                    break 'b_133624;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                    c.g[V0][0] = 0x2;
                                    break 'b_133634;
                                }
                                c.cycles += 1;
                                c.g[V0][0] = 0x1;
                                break 'b_133634;
                            }
                            c.cycles += 2;
                            c.sw(at(c, V1, R_LEAN_DIR), c.g[V0][0]);
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                            break 'b_13363c;
                        }
                    }
                    'b_133750: {
                        'b_13374c: {
                            'b_13366c: {
                                c.cycles += 5;
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0u64;
                                call_ptr(c, spin_callee::RIDER_HANDLE_BOOST_INPUT, 0x00133648)?;
                                let t = c.g[S2][0] == 0u64;
                                c.g[S3][0] = c.lw(at(c, S0, PHASE));
                                if t {
                                    break 'b_13366c;
                                } else {
                                    c.cycles += 4;
                                    c.mtc1(0, 23);
                                    c.g[V0][0] = c.lw(at(c, S0, MODE));
                                    let t = c.g[V0][0] != 0u64;
                                    c.mov_s(24, 23);
                                    if t {
                                        break 'b_13374c;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x3;
                                        c.sw(at(c, S0, MODE), c.g[V0][0]);
                                        break 'b_13374c;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = 0x3;
                            if c.g[S3][0] != c.g[V0][0] {
                                c.g[V1][0] = c.lw(at(c, S0, PHASE));
                                break 'b_133750;
                            } else {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                                call_ptr(c, spin_callee::ANIM_GET_LAYER_ANIM_ID, 0x00133688)?;
                                c.g[V1][0] = 0x120;
                                if c.g[V0][0] == c.g[V1][0] {
                                    c.g[V1][0] = c.lw(at(c, S0, PHASE));
                                    break 'b_133750;
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S0][0];
                                    spin_callee::TRY_REVERSE.call(c, 0x0013369c)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V1][0] = c.lw(at(c, S0, PHASE));
                                        break 'b_133750;
                                    } else {
                                        'b_1336cc: {
                                            c.cycles += 6;
                                            c.mtc1(0, 20);
                                            c.lwc1(0, stack(c, STICK_SPIN));
                                            c.c_eq_s(0, 20);
                                            let t = !c.fcc();
                                            c.lwc1(0, stack(c, STICK_FLIP));
                                            if t {
                                                break 'b_1336cc;
                                            } else {
                                                c.cycles += 4;
                                                c.c_eq_s(0, 20);
                                                if c.fcc() {
                                                    c.g[V1][0] = c.lw(at(c, S0, PHASE));
                                                    break 'b_133750;
                                                } else {
                                                    break 'b_1336cc;
                                                }
                                            }
                                        }
                                        c.cycles += 32;
                                        c.g[A0][0] = c.lw(at(c, S0, RIDER));
                                        c.g[A1][0] = c.g[SP][0]; // &stick_flip
                                        c.lwc1(12, gp(c, -0x751c));
                                        c.g[A2][0] = c.g[SP][0] | 0x4; // &stick_spin
                                        c.mov_s(13, 20);
                                        call_ptr(
                                            c,
                                            spin_callee::INPUT_QUANTIZE_STICK_DIRECTION,
                                            0x001336e4,
                                        )?;
                                        c.lwc1(12, stack(c, STICK_FLIP));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.lwc1(13, stack(c, STICK_SPIN));
                                        spin_callee::SET_SPIN_DIRECTION.call(c, 0x001336f4)?;
                                        c.lwc1(1, gp(c, -0x7518));
                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                        c.swc1(20, at(c, S0, FLIP_AMOUNT));
                                        c.swc1(20, at(c, S0, SPIN_AMOUNT));
                                        c.swc1(1, at(c, S0, SPIN_RATE));
                                        c.swc1(1, at(c, V0, R_SPIN_SPEED));
                                        c.lwc1(0, gp(c, -0x7514));
                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                        c.swc1(0, at(c, S0, FLIP_RATE));
                                        c.swc1(0, at(c, V0, R_FLIP_SPEED));
                                        c.sw(at(c, S0, PHASE), 0u64);
                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                        c.g[A1][0] = c.lw(at(c, V0, R_STANCE));
                                        c.g[V1][0] = c.lw(at(c, V0, R_STANCE_TAKEOFF));
                                        c.g[A0][0] = c.lw(at(c, V0, R_METRIX));
                                        c.g[A1][0] = c.g[A1][0] ^ c.g[V1][0];
                                        c.g[A1][0] = (0u64 < c.g[A1][0]) as u64;
                                        call_ptr(
                                            c,
                                            spin_callee::METRIX_START_SPIN_TRICK,
                                            0x0013373c,
                                        )?;
                                        c.g[A0][0] = c.lw(at(c, S0, RIDER));
                                        c.mov_s(12, 0);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, spin_callee::RIDER_ADD_BOOST, 0x0013374c)?;
                                        break 'b_13374c;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(at(c, S0, PHASE));
                        break 'b_133750;
                    }
                    'b_133c40: {
                        'b_13383c: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_13383c;
                            } else {
                                'b_1337b0: {
                                    c.cycles += 5;
                                    c.lwc1(0, at(c, S0, SPIN_TARGET));
                                    c.mtc1(0, 1);
                                    c.c_eq_s(0, 1);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_1337b0;
                                    } else {
                                        c.cycles += 4;
                                        c.lwc1(0, at(c, S0, FLIP_TARGET));
                                        c.c_eq_s(0, 1);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_1337b0;
                                        } else {
                                            c.cycles += 6;
                                            c.lwc1(0, at(c, S0, SPIN));
                                            c.swc1(1, stack(c, STICK_SPIN));
                                            c.c_eq_s(0, 1);
                                            let t = !c.fcc();
                                            c.swc1(1, stack(c, STICK_FLIP));
                                            if t {
                                                break 'b_133c40;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(0, at(c, S0, FLIP));
                                                c.c_eq_s(0, 1);
                                                let t = !c.fcc();
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_133c40;
                                                } else {
                                                    c.cycles += 2;
                                                    c.sw(at(c, S0, PHASE), c.g[V0][0]);
                                                    break 'b_133c40;
                                                }
                                            }
                                        }
                                    }
                                }
                                'b_133810: {
                                    c.cycles += 11;
                                    c.lwc1(0, gp(c, -0x7510));
                                    c.lwc1(3, gp(c, 0x1ba4));
                                    c.lwc1(1, at(c, S0, SPIN));
                                    c.lwc1(2, at(c, S0, SPIN_TARGET));
                                    c.sub_s(0, 0, 3);
                                    c.sub_s(1, 1, 2);
                                    c.abs_s(1, 1);
                                    c.c_lt_s(1, 0);
                                    let t = !c.fcc();
                                    c.lwc1(0, stack(c, STICK_SPIN));
                                    if t {
                                        break 'b_133810;
                                    } else {
                                        'b_13380c: {
                                            c.cycles += 10;
                                            c.lwc1(0, gp(c, -0x750c));
                                            c.lwc1(1, at(c, S0, FLIP));
                                            c.lwc1(2, at(c, S0, FLIP_TARGET));
                                            c.sub_s(0, 0, 3);
                                            c.sub_s(1, 1, 2);
                                            c.abs_s(1, 1);
                                            c.c_lt_s(1, 0);
                                            let t = !c.fcc();
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_13380c;
                                            } else {
                                                c.cycles += 2;
                                                c.sw(at(c, S0, PHASE), c.g[V0][0]);
                                                break 'b_133c40;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.lwc1(0, stack(c, STICK_SPIN));
                                        break 'b_133810;
                                    }
                                }
                                c.cycles += 5;
                                c.mtc1(0, 1);
                                c.c_eq_s(0, 1);
                                let t = !c.fcc();
                                c.lwc1(0, stack(c, STICK_FLIP));
                                if t {
                                    break 'b_133c40;
                                } else {
                                    c.cycles += 3;
                                    c.c_eq_s(0, 1);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_133c40;
                                    } else {
                                        c.cycles += 3;
                                        c.swc1(1, at(c, S0, FLIP_TARGET));
                                        c.swc1(1, at(c, S0, SPIN_TARGET));
                                        break 'b_133c40;
                                    }
                                }
                            }
                        }
                        'b_133b68: {
                            c.cycles += 2;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V0][0] = 0x2;
                                break 'b_133b68;
                            } else {
                                'b_133954: {
                                    'b_133950: {
                                        c.cycles += 6;
                                        c.lwc1(0, stack(c, STICK_SPIN));
                                        c.mtc1(0, 5);
                                        c.c_eq_s(0, 5);
                                        let t = !c.fcc();
                                        c.lwc1(0, stack(c, STICK_FLIP));
                                        if t {
                                            break 'b_133950;
                                        } else {
                                            c.cycles += 4;
                                            c.c_eq_s(0, 5);
                                            if !c.fcc() {
                                                c.sw(at(c, S0, RELEASED), 0u64);
                                                break 'b_133954;
                                            } else {
                                                'b_1338b0: {
                                                    c.cycles += 12;
                                                    c.lwc1(3, at(c, S0, SPIN_TARGET));
                                                    c.lwc1(0, at(c, S0, SPIN));
                                                    c.lwc1(1, at(c, S0, RELEASED));
                                                    c.lwc1(2, gp(c, -0x7508));
                                                    c.sub_s(0, 3, 0);
                                                    c.lwc1(4, gp(c, -0x7504));
                                                    c.abs_s(0, 0);
                                                    c.add_s(1, 1, 2);
                                                    c.c_lt_s(4, 0);
                                                    let t = !c.fcc();
                                                    c.swc1(1, at(c, S0, RELEASED));
                                                    if t {
                                                        break 'b_1338b0;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.c_lt_s(3, 5);
                                                        if !c.fcc() {
                                                            c.sub_s(3, 3, 4);
                                                            break 'b_1338b0;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.add_s(3, 3, 4);
                                                            break 'b_1338b0;
                                                        }
                                                    }
                                                }
                                                'b_1338f0: {
                                                    c.cycles += 9;
                                                    c.lwc1(2, at(c, S0, FLIP_TARGET));
                                                    c.lwc1(0, at(c, S0, FLIP));
                                                    c.lwc1(1, gp(c, -0x7500));
                                                    c.sub_s(0, 2, 0);
                                                    c.abs_s(0, 0);
                                                    c.c_lt_s(1, 0);
                                                    if !c.fcc() {
                                                        c.lwc1(1, at(c, S0, RELEASED));
                                                        break 'b_1338f0;
                                                    } else {
                                                        'b_1338ec: {
                                                            c.cycles += 5;
                                                            c.mtc1(0, 0);
                                                            c.c_lt_s(2, 0);
                                                            if !c.fcc() {
                                                                c.sub_s(2, 2, 1);
                                                                break 'b_1338ec;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.add_s(2, 2, 1);
                                                                break 'b_1338ec;
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.lwc1(1, at(c, S0, RELEASED));
                                                        break 'b_1338f0;
                                                    }
                                                }
                                                'b_133918: {
                                                    c.cycles += 6;
                                                    c.lwc1(0, gp(c, -0x74fc));
                                                    c.mtc1(0, 4);
                                                    c.c_lt_s(0, 1);
                                                    if !c.fcc() {
                                                        c.lwc1(0, at(c, S0, SPIN));
                                                        break 'b_133918;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.swc1(3, at(c, S0, SPIN_TARGET));
                                                        c.swc1(2, at(c, S0, FLIP_TARGET));
                                                        c.lwc1(4, gp(c, 0x1ba0));
                                                        c.lwc1(0, at(c, S0, SPIN));
                                                        break 'b_133918;
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.sub_s(0, 3, 0);
                                                c.abs_s(0, 0);
                                                c.c_le_s(0, 4);
                                                let t = !c.fcc();

                                                if t {
                                                    break 'b_133c40;
                                                } else {
                                                    c.cycles += 7;
                                                    c.lwc1(0, at(c, S0, FLIP));
                                                    c.sub_s(0, 2, 0);
                                                    c.abs_s(0, 0);
                                                    c.c_le_s(0, 4);
                                                    let t = !c.fcc();
                                                    c.g[V0][0] = 0x2;
                                                    if t {
                                                        break 'b_133c40;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.sw(at(c, S0, PHASE), c.g[V0][0]);
                                                        break 'b_133c40;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.sw(at(c, S0, RELEASED), 0u64);
                                    break 'b_133954;
                                }
                                'b_133980: {
                                    'b_133974: {
                                        c.cycles += 6;
                                        c.lwc1(0, stack(c, STICK_SPIN));
                                        c.lwc1(1, at(c, S0, RELEASED));
                                        c.c_eq_s(0, 1);
                                        if !c.fcc() {
                                            c.lwc1(0, at(c, S0, SPIN_HELD));
                                            break 'b_133974;
                                        } else {
                                            c.cycles += 2;
                                            c.swc1(1, at(c, S0, SPIN_HELD));
                                            break 'b_133980;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.lwc1(1, gp(c, -0x74f8));
                                    c.add_s(0, 0, 1);
                                    c.swc1(0, at(c, S0, SPIN_HELD));
                                    break 'b_133980;
                                }
                                'b_1339ac: {
                                    'b_1339a0: {
                                        c.cycles += 6;
                                        c.lwc1(0, stack(c, STICK_FLIP));
                                        c.mtc1(0, 1);
                                        c.c_eq_s(0, 1);
                                        if !c.fcc() {
                                            c.lwc1(0, at(c, S0, FLIP_HELD));
                                            break 'b_1339a0;
                                        } else {
                                            c.cycles += 2;
                                            c.swc1(1, at(c, S0, FLIP_HELD));
                                            break 'b_1339ac;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.lwc1(1, gp(c, -0x74f4));
                                    c.add_s(0, 0, 1);
                                    c.swc1(0, at(c, S0, FLIP_HELD));
                                    break 'b_1339ac;
                                }
                                'b_133a8c: {
                                    c.cycles += 10;
                                    c.lwc1(0, at(c, S0, SPIN));
                                    c.g[V1][0] = sx(at(c, S0, SPIN_TARGET));
                                    c.lwc1(2, at(c, S0, SPIN_TARGET));
                                    c.lwc1(1, gp(c, 0x1ba0));
                                    c.sub_s(0, 0, 2);
                                    c.abs_s(0, 0);
                                    c.c_le_s(0, 1);
                                    if !c.fcc() {
                                        c.lwc1(0, at(c, S0, FLIP));
                                        break 'b_133a8c;
                                    } else {
                                        'b_1339ec: {
                                            c.cycles += 5;
                                            c.mtc1(0, 1);
                                            c.c_eq_s(2, 1);
                                            let t = !c.fcc();
                                            c.mov_s(3, 1);
                                            if t {
                                                break 'b_1339ec;
                                            } else {
                                                c.cycles += 1;
                                                c.lwc1(3, gp(c, -0x74f0));
                                                break 'b_1339ec;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.lwc1(0, at(c, S0, SPIN_HELD));
                                        c.c_lt_s(3, 0);
                                        if !c.fcc() {
                                            c.lwc1(0, at(c, S0, FLIP));
                                            break 'b_133a8c;
                                        } else {
                                            'b_133a54: {
                                                'b_133a50: {
                                                    'b_133a30: {
                                                        c.cycles += 4;
                                                        c.c_eq_s(2, 1);
                                                        let t = !c.fcc();
                                                        c.lwc1(0, stack(c, STICK_SPIN));
                                                        if t {
                                                            break 'b_133a30;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[V0][0] = 0x1;
                                                            c.c_lt_s(0, 1);
                                                            c.lwc1(0, gp(c, -0x74ec));
                                                            let t = !c.fcc();
                                                            c.sw(at(c, S0, TURNED), c.g[V0][0]);
                                                            if t {
                                                                break 'b_133a50;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.lwc1(0, gp(c, -0x74e8));
                                                                c.swc1(0, at(c, V1, 0));
                                                                break 'b_133a54;
                                                            }
                                                        }
                                                    }
                                                    'b_133a48: {
                                                        c.cycles += 3;
                                                        c.c_lt_s(2, 1);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_133a48;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.lwc1(0, gp(c, -0x74e4));
                                                            c.sub_s(0, 2, 0);
                                                            break 'b_133a50;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    c.lwc1(0, gp(c, -0x74e0));
                                                    c.add_s(0, 2, 0);
                                                    break 'b_133a50;
                                                }
                                                c.cycles += 1;
                                                c.swc1(0, at(c, V1, 0));
                                                break 'b_133a54;
                                            }
                                            c.cycles += 7;
                                            c.lwc1(0, at(c, S0, SPIN_TARGET));
                                            c.lwc1(1, gp(c, 0x1bb4));
                                            c.abs_s(0, 0);
                                            c.c_lt_s(1, 0);
                                            if !c.fcc() {
                                                c.lwc1(0, at(c, S0, FLIP));
                                                break 'b_133a8c;
                                            } else {
                                                c.cycles += 7;
                                                c.lwc1(0, at(c, S0, SPIN_RATE));
                                                c.lwc1(2, gp(c, -0x74dc));
                                                c.lwc1(1, gp(c, -0x74d8));
                                                c.mul_s(0, 0, 2);
                                                c.max_s(0, 0, 1);
                                                c.swc1(0, at(c, S0, SPIN_RATE));
                                                c.lwc1(0, at(c, S0, FLIP));
                                                break 'b_133a8c;
                                            }
                                        }
                                    }
                                }
                                'b_133b58: {
                                    c.cycles += 8;
                                    c.g[V1][0] = sx(at(c, S0, FLIP_TARGET));
                                    c.lwc1(2, at(c, S0, FLIP_TARGET));
                                    c.lwc1(1, gp(c, 0x1ba0));
                                    c.sub_s(0, 0, 2);
                                    c.abs_s(0, 0);
                                    c.c_le_s(0, 1);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_133b58;
                                    } else {
                                        'b_133ac4: {
                                            c.cycles += 5;
                                            c.mtc1(0, 1);
                                            c.c_eq_s(2, 1);
                                            let t = !c.fcc();
                                            c.mov_s(3, 1);
                                            if t {
                                                break 'b_133ac4;
                                            } else {
                                                c.cycles += 1;
                                                c.lwc1(3, gp(c, -0x74d4));
                                                break 'b_133ac4;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.lwc1(0, at(c, S0, FLIP_HELD));
                                        c.c_lt_s(3, 0);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_133b58;
                                        } else {
                                            'b_133b28: {
                                                'b_133b24: {
                                                    'b_133b04: {
                                                        c.cycles += 4;
                                                        c.c_eq_s(2, 1);
                                                        let t = !c.fcc();
                                                        c.lwc1(0, stack(c, STICK_FLIP));
                                                        if t {
                                                            break 'b_133b04;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[V0][0] = 0x1;
                                                            c.c_lt_s(0, 1);
                                                            c.lwc1(0, gp(c, -0x74d0));
                                                            let t = !c.fcc();
                                                            c.sw(at(c, S0, TURNED), c.g[V0][0]);
                                                            if t {
                                                                break 'b_133b24;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.lwc1(0, gp(c, -0x74cc));
                                                                c.swc1(0, at(c, V1, 0));
                                                                break 'b_133b28;
                                                            }
                                                        }
                                                    }
                                                    'b_133b1c: {
                                                        c.cycles += 3;
                                                        c.c_lt_s(2, 1);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_133b1c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.lwc1(0, gp(c, -0x74c8));
                                                            c.sub_s(0, 2, 0);
                                                            break 'b_133b24;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    c.lwc1(0, gp(c, -0x74c4));
                                                    c.add_s(0, 2, 0);
                                                    break 'b_133b24;
                                                }
                                                c.cycles += 1;
                                                c.swc1(0, at(c, V1, 0));
                                                break 'b_133b28;
                                            }
                                            c.cycles += 6;
                                            c.lwc1(0, at(c, S0, FLIP_TARGET));
                                            c.lwc1(1, gp(c, 0x1bb8));
                                            c.abs_s(0, 0);
                                            c.c_lt_s(1, 0);
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_133b58;
                                            } else {
                                                c.cycles += 6;
                                                c.lwc1(0, at(c, S0, FLIP_RATE));
                                                c.lwc1(2, gp(c, -0x74c0));
                                                c.lwc1(1, gp(c, -0x74bc));
                                                c.mul_s(0, 0, 2);
                                                c.max_s(0, 0, 1);
                                                c.swc1(0, at(c, S0, FLIP_RATE));
                                                break 'b_133b58;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S2][0] == 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_133c40;
                                } else {
                                    c.cycles += 2;
                                    c.sw(at(c, S0, TURNED), c.g[V0][0]);
                                    break 'b_133c40;
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_133c40;
                        } else {
                            'b_133bc0: {
                                c.cycles += 7;
                                c.lwc1(1, at(c, S0, SPIN));
                                c.lwc1(0, at(c, S0, SPIN_TARGET));
                                c.sw(at(c, S0, MODE), 0u64);
                                c.c_eq_s(1, 0);
                                c.sw(stack(c, STICK_SPIN), 0u64);
                                let t = !c.fcc();
                                c.sw(stack(c, STICK_FLIP), 0u64);
                                if t {
                                    break 'b_133bc0;
                                } else {
                                    c.cycles += 6;
                                    c.lwc1(1, at(c, S0, FLIP));
                                    c.lwc1(0, at(c, S0, FLIP_TARGET));
                                    c.c_eq_s(1, 0);
                                    let t = !c.fcc();
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_133bc0;
                                    } else {
                                        c.cycles += 7;
                                        c.sw(at(c, S0, SPIN), 0u64);
                                        c.sw(at(c, S0, PHASE), c.g[V0][0]);
                                        c.sw(at(c, S0, SPIN_TARGET), 0u64);
                                        c.sw(at(c, S0, FLIP), 0u64);
                                        c.sw(at(c, S0, FLIP_TARGET), 0u64);
                                        c.sw(at(c, S0, TURNED), 0u64);
                                        break 'b_133c40;
                                    }
                                }
                            }
                            'b_133bf4: {
                                c.cycles += 2;
                                if c.g[S2][0] != 0u64 {
                                    c.lwc1(0, at(c, S0, FLIP_TARGET));
                                    break 'b_133bf4;
                                } else {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                    c.g[A1][0] = 0x2;
                                    c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                                    call_ptr(c, spin_callee::ANIM_GET_LAYER_ANIM_ID, 0x00133bd8)?;
                                    c.g[V1][0] = 0x120;
                                    if c.g[V0][0] == c.g[V1][0] {
                                        c.lwc1(0, at(c, S0, FLIP_TARGET));
                                        break 'b_133bf4;
                                    } else {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.g[S0][0];
                                        spin_callee::TRY_REVERSE.call(c, 0x00133bec)?;

                                        break 'b_133c40;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.lwc1(1, at(c, S0, FLIP));
                            c.c_eq_s(0, 1);
                            let t = c.fcc();

                            if t {
                                break 'b_133c40;
                            } else {
                                'b_133c24: {
                                    c.cycles += 4;
                                    c.mtc1(0, 0);
                                    c.c_lt_s(1, 0);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_133c24;
                                    } else {
                                        c.cycles += 4;
                                        c.c_lt_s(0, 23);
                                        if c.fcc() {
                                            c.neg_s(23, 23);
                                            break 'b_133c40;
                                        } else {
                                            break 'b_133c24;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.c_lt_s(0, 1);
                                let t = !c.fcc();

                                if t {
                                    break 'b_133c40;
                                } else {
                                    c.cycles += 4;
                                    c.c_lt_s(23, 0);
                                    if c.fcc() {
                                        c.neg_s(23, 23);
                                        break 'b_133c40;
                                    } else {
                                        break 'b_133c40;
                                    }
                                }
                            }
                        }
                    }
                    'b_133d70: {
                        'b_133cec: {
                            'b_133ce8: {
                                c.cycles += 34;
                                spin_callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x00133c48)?;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 20);
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x3;
                                spin_callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x00133c5c)?;
                                c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A2][0] = c.lw(at(c, V1, R_CHARACTER));
                                c.g[A1][0] = c.lw(at(c, V1, R_PLAYER));
                                spin_callee::BE_CHAR_INT_GET_ATTRIB4_FACTOR.call(c, 0x00133c70)?;
                                c.lwc1(1, gp(c, -0x74b8));
                                c.mul_s(0, 0, 1);
                                c.add_s(0, 0, 20);
                                c.div_s(25, 20, 0);
                                spin_callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x00133c88)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x3;
                                spin_callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x00133c94)?;
                                c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A2][0] = c.lw(at(c, V1, R_CHARACTER));
                                c.g[A1][0] = c.lw(at(c, V1, R_PLAYER));
                                spin_callee::BE_CHAR_INT_GET_ATTRIB4_FACTOR.call(c, 0x00133ca8)?;
                                c.lwc1(1, gp(c, -0x74b4));
                                c.mtc1(0, 3);
                                c.lwc1(2, stack(c, STICK_FLIP));
                                c.mul_s(0, 0, 1);
                                c.c_eq_s(2, 3);
                                let t = !c.fcc();
                                c.add_s(26, 0, 20);
                                if t {
                                    break 'b_133ce8;
                                } else {
                                    c.cycles += 5;
                                    c.lwc1(0, stack(c, STICK_SPIN));
                                    c.c_eq_s(0, 3);
                                    let t = !c.fcc();
                                    c.lwc1(20, stack(c, STICK_SPIN));
                                    if t {
                                        break 'b_133cec;
                                    } else {
                                        c.cycles += 3;
                                        c.lwc1(0, gp(c, -0x74b0));
                                        c.swc1(0, at(c, S0, STICK_ANGLE));
                                        break 'b_133d70;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.lwc1(20, stack(c, STICK_SPIN));
                            break 'b_133cec;
                        }
                        'b_133d6c: {
                            'b_133d2c: {
                                c.cycles += 5;
                                c.mtc1(0, 22);
                                c.c_eq_s(20, 22);
                                let t = !c.fcc();
                                c.lwc1(21, stack(c, STICK_FLIP));
                                if t {
                                    break 'b_133d2c;
                                } else {
                                    c.cycles += 4;
                                    c.c_eq_s(21, 22);
                                    let t = c.fcc();
                                    c.mov_s(0, 21);
                                    if t {
                                        break 'b_133d6c;
                                    } else {
                                        c.cycles += 4;
                                        c.c_le_s(22, 21);
                                        c.lwc1(0, gp(c, -0x74ac));
                                        if !c.fcc() {
                                            c.swc1(0, at(c, S0, STICK_ANGLE));
                                            break 'b_133d70;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(0, gp(c, -0x74a8));
                                            c.swc1(0, at(c, S0, STICK_ANGLE));
                                            break 'b_133d70;
                                        }
                                    }
                                }
                            }
                            'b_133d68: {
                                c.cycles += 7;
                                c.div_s(12, 21, 20);
                                call_ptr(c, spin_callee::MATH_ATAN, 0x00133d38)?;
                                c.c_lt_s(20, 22);
                                let t = !c.fcc();
                                c.mov_s(12, 0);
                                if t {
                                    break 'b_133d68;
                                } else {
                                    'b_133d60: {
                                        c.cycles += 3;
                                        c.c_lt_s(22, 21);
                                        let t = !c.fcc();

                                        if t {
                                            break 'b_133d60;
                                        } else {
                                            c.cycles += 3;
                                            c.lwc1(0, gp(c, -0x74a4));
                                            c.add_s(12, 12, 0);
                                            break 'b_133d68;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.lwc1(0, gp(c, -0x74a0));
                                    c.sub_s(12, 12, 0);
                                    break 'b_133d68;
                                }
                            }
                            c.cycles += 1;
                            c.mov_s(0, 12);
                            break 'b_133d6c;
                        }
                        c.cycles += 1;
                        c.swc1(0, at(c, S0, STICK_ANGLE));
                        break 'b_133d70;
                    }
                    'b_133eb0: {
                        'b_133eac: {
                            'b_133d94: {
                                c.cycles += 6;
                                c.lwc1(1, at(c, S0, SPIN_TARGET));
                                c.lwc1(4, at(c, S0, SPIN));
                                c.c_eq_s(1, 4);
                                let t = !c.fcc();
                                c.lwc1(2, stack(c, STICK_SPIN));
                                if t {
                                    break 'b_133d94;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                    c.sw(at(c, V0, R_SPIN_SPEED), 0u64);
                                    break 'b_133eac;
                                }
                            }
                            'b_133dc4: {
                                c.cycles += 5;
                                c.mtc1(0, 3);
                                c.mul_s(0, 2, 1);
                                c.c_lt_s(0, 3);
                                let t = !c.fcc();

                                if t {
                                    break 'b_133dc4;
                                } else {
                                    c.cycles += 7;
                                    c.lwc1(0, gp(c, -0x749c));
                                    c.lwc1(1, at(c, S0, SPIN_RATE));
                                    c.mul_s(0, 25, 0);
                                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                    c.min_s(1, 1, 0);
                                    c.swc1(1, at(c, V0, R_SPIN_SPEED));
                                    break 'b_133eac;
                                }
                            }
                            'b_133e70: {
                                'b_133de4: {
                                    c.cycles += 4;
                                    c.c_eq_s(2, 3);
                                    let t = c.fcc();
                                    c.lwc1(0, stack(c, STICK_FLIP));
                                    if t {
                                        break 'b_133de4;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                        c.lwc1(0, at(c, S0, SPIN_RATE));
                                        c.swc1(0, at(c, V0, R_SPIN_SPEED));
                                        break 'b_133e70;
                                    }
                                }
                                'b_133e28: {
                                    c.cycles += 5;
                                    c.sub_s(5, 4, 1);
                                    c.c_eq_s(0, 3);
                                    let t = !c.fcc();
                                    c.lwc1(4, at(c, S0, SPIN_RATE));
                                    if t {
                                        break 'b_133e28;
                                    } else {
                                        c.cycles += 12;
                                        c.lwc1(2, gp(c, -0x7498));
                                        c.abs_s(0, 5);
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 1);
                                        c.mul_s(0, 0, 2);
                                        c.lwc1(3, gp(c, -0x7494));
                                        c.lwc1(2, gp(c, -0x7490));
                                        c.sub_s(1, 1, 0);
                                        c.mul_s(0, 0, 3);
                                        c.mul_s(1, 1, 2);
                                        c.add_s(0, 0, 1);
                                        c.max_s(4, 0, 4);
                                        break 'b_133e28;
                                    }
                                }
                                'b_133e6c: {
                                    'b_133e68: {
                                        c.cycles += 10;
                                        c.lwc1(1, gp(c, 0x1ba0));
                                        c.abs_s(0, 5);
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 2);
                                        c.div_s(3, 0, 1);
                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                        c.c_le_s(3, 2);
                                        let t = !c.fcc();
                                        c.g[V0][0] = sx(at(c, V0, R_SPIN_SPEED));
                                        if t {
                                            break 'b_133e68;
                                        } else {
                                            c.cycles += 6;
                                            c.lwc1(0, gp(c, -0x748c));
                                            c.lwc1(1, gp(c, -0x7488));
                                            c.mul_s(0, 3, 0);
                                            c.add_s(0, 0, 1);
                                            c.mul_s(0, 4, 0);
                                            break 'b_133e6c;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.mov_s(0, 4);
                                    break 'b_133e6c;
                                }
                                c.cycles += 1;
                                c.swc1(0, at(c, V0, 0));
                                break 'b_133e70;
                            }
                            c.cycles += 11;
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.mtc1(0, 1);
                            c.lwc1(0, at(c, V0, R_SPIN_SPEED));
                            c.mul_s(0, 0, 26);
                            c.swc1(0, at(c, V0, R_SPIN_SPEED));
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.lwc1(0, at(c, V0, R_2EC));
                            c.c_lt_s(1, 0);
                            if !c.fcc() {
                                c.lwc1(1, at(c, S0, FLIP_TARGET));
                                break 'b_133eb0;
                            } else {
                                c.cycles += 4;
                                c.lwc1(0, at(c, V0, R_SPIN_SPEED));
                                c.lwc1(1, gp(c, -0x7484));
                                c.mul_s(0, 0, 1);
                                c.swc1(0, at(c, V0, R_SPIN_SPEED));
                                break 'b_133eac;
                            }
                        }
                        c.cycles += 1;
                        c.lwc1(1, at(c, S0, FLIP_TARGET));
                        break 'b_133eb0;
                    }
                    'b_133fe4: {
                        'b_133ed0: {
                            c.cycles += 5;
                            c.lwc1(4, at(c, S0, FLIP));
                            c.c_eq_s(1, 4);
                            let t = !c.fcc();
                            c.lwc1(2, stack(c, STICK_FLIP));
                            if t {
                                break 'b_133ed0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                c.sw(at(c, V0, R_FLIP_SPEED), 0u64);
                                break 'b_133fe4;
                            }
                        }
                        'b_133f00: {
                            c.cycles += 5;
                            c.mtc1(0, 3);
                            c.mul_s(0, 2, 1);
                            c.c_lt_s(0, 3);
                            let t = !c.fcc();

                            if t {
                                break 'b_133f00;
                            } else {
                                c.cycles += 7;
                                c.lwc1(0, gp(c, -0x7480));
                                c.lwc1(1, at(c, S0, FLIP_RATE));
                                c.mul_s(0, 25, 0);
                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                c.min_s(1, 1, 0);
                                c.swc1(1, at(c, V0, R_FLIP_SPEED));
                                break 'b_133fe4;
                            }
                        }
                        'b_133fac: {
                            'b_133f20: {
                                c.cycles += 4;
                                c.c_eq_s(2, 3);
                                let t = c.fcc();
                                c.lwc1(0, stack(c, STICK_SPIN));
                                if t {
                                    break 'b_133f20;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                    c.lwc1(0, at(c, S0, FLIP_RATE));
                                    c.swc1(0, at(c, V0, R_FLIP_SPEED));
                                    break 'b_133fac;
                                }
                            }
                            'b_133f64: {
                                c.cycles += 5;
                                c.sub_s(5, 4, 1);
                                c.c_eq_s(0, 3);
                                let t = !c.fcc();
                                c.lwc1(4, at(c, S0, FLIP_RATE));
                                if t {
                                    break 'b_133f64;
                                } else {
                                    c.cycles += 12;
                                    c.lwc1(2, gp(c, -0x747c));
                                    c.abs_s(0, 5);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 1);
                                    c.mul_s(0, 0, 2);
                                    c.lwc1(3, gp(c, -0x7478));
                                    c.lwc1(2, gp(c, -0x7474));
                                    c.sub_s(1, 1, 0);
                                    c.mul_s(0, 0, 3);
                                    c.mul_s(1, 1, 2);
                                    c.add_s(0, 0, 1);
                                    c.max_s(4, 0, 4);
                                    break 'b_133f64;
                                }
                            }
                            'b_133fa8: {
                                'b_133fa4: {
                                    c.cycles += 10;
                                    c.lwc1(1, gp(c, 0x1ba0));
                                    c.abs_s(0, 5);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 2);
                                    c.div_s(3, 0, 1);
                                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                    c.c_le_s(3, 2);
                                    let t = !c.fcc();
                                    c.g[V0][0] = sx(at(c, V0, R_FLIP_SPEED));
                                    if t {
                                        break 'b_133fa4;
                                    } else {
                                        c.cycles += 6;
                                        c.lwc1(0, gp(c, -0x7470));
                                        c.lwc1(1, gp(c, -0x746c));
                                        c.mul_s(0, 3, 0);
                                        c.add_s(0, 0, 1);
                                        c.mul_s(0, 4, 0);
                                        break 'b_133fa8;
                                    }
                                }
                                c.cycles += 1;
                                c.mov_s(0, 4);
                                break 'b_133fa8;
                            }
                            c.cycles += 1;
                            c.swc1(0, at(c, V0, 0));
                            break 'b_133fac;
                        }
                        c.cycles += 10;
                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                        c.mtc1(0, 1);
                        c.lwc1(0, at(c, V0, R_FLIP_SPEED));
                        c.mul_s(0, 0, 26);
                        c.swc1(0, at(c, V0, R_FLIP_SPEED));
                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                        c.lwc1(0, at(c, V0, R_2EC));
                        c.c_lt_s(1, 0);
                        let t = !c.fcc();

                        if t {
                            break 'b_133fe4;
                        } else {
                            c.cycles += 4;
                            c.lwc1(0, at(c, V0, R_FLIP_SPEED));
                            c.lwc1(1, gp(c, -0x7468));
                            c.mul_s(0, 0, 1);
                            c.swc1(0, at(c, V0, R_FLIP_SPEED));
                            break 'b_133fe4;
                        }
                    }
                    'b_133ffc: {
                        c.cycles += 5;
                        c.lwc1(1, gp(c, -0x7464));
                        c.abs_s(0, 24);
                        c.c_lt_s(0, 1);
                        let t = !c.fcc();

                        if t {
                            break 'b_133ffc;
                        } else {
                            c.cycles += 1;
                            c.mtc1(0, 24);
                            break 'b_133ffc;
                        }
                    }
                    'b_134010: {
                        c.cycles += 4;
                        c.abs_s(0, 23);
                        c.c_lt_s(0, 1);
                        let t = !c.fcc();

                        if t {
                            break 'b_134010;
                        } else {
                            c.cycles += 1;
                            c.mtc1(0, 23);
                            break 'b_134010;
                        }
                    }
                    'b_134064: {
                        'b_13403c: {
                            c.cycles += 6;
                            c.mtc1(0, 0);
                            c.lwc1(9, at(c, S0, SPIN));
                            c.c_lt_s(24, 0);
                            let t = !c.fcc();
                            c.lwc1(8, at(c, S0, FLIP));
                            if t {
                                break 'b_13403c;
                            } else {
                                c.cycles += 5;
                                c.lwc1(0, gp(c, 0x1ba8));
                                c.lwc1(4, at(c, S0, LEAN_X));
                                c.mov_s(5, 0);
                                c.neg_s(3, 0);
                                break 'b_134064;
                            }
                        }
                        'b_134058: {
                            c.cycles += 4;
                            c.c_le_s(24, 0);
                            let t = !c.fcc();
                            c.lwc1(5, gp(c, 0x1ba8));
                            if t {
                                break 'b_134058;
                            } else {
                                c.cycles += 3;
                                c.lwc1(3, at(c, S0, LEAN_X));
                                c.mov_s(4, 3);
                                break 'b_134064;
                            }
                        }
                        c.cycles += 3;
                        c.lwc1(3, gp(c, 0x1ba8));
                        c.lwc1(4, at(c, S0, LEAN_X));
                        c.mov_s(5, 3);
                        break 'b_134064;
                    }
                    'b_1340a0: {
                        c.cycles += 14;
                        c.g[AT][0] = 0x41200000;
                        c.mtc1(1, 1);
                        c.sub_s(0, 3, 4);
                        c.abs_s(0, 0);
                        c.lwc1(2, gp(c, -0x7460));
                        c.mul_s(0, 0, 1);
                        c.abs_s(6, 24);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 1);
                        c.div_s(0, 0, 5);
                        c.c_le_s(2, 0);
                        let t = !c.fcc();
                        c.mov_s(7, 2);
                        if t {
                            break 'b_1340a0;
                        } else {
                            c.cycles += 1;
                            c.min_s(7, 0, 1);
                            break 'b_1340a0;
                        }
                    }
                    'b_1340ec: {
                        'b_1340d8: {
                            c.cycles += 12;
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 2);
                            c.lwc1(1, gp(c, -0x745c));
                            c.mul_s(0, 6, 2);
                            c.mul_s(1, 7, 1);
                            c.add_s(0, 0, 2);
                            c.mul_s(1, 0, 1);
                            c.add_s(2, 3, 1);
                            c.c_lt_s(2, 4);
                            if !c.fcc() {
                                c.sub_s(0, 3, 1);
                                break 'b_1340d8;
                            } else {
                                c.cycles += 2;
                                c.sub_s(2, 4, 1);
                                break 'b_1340ec;
                            }
                        }
                        c.cycles += 4;
                        c.c_lt_s(4, 0);
                        let t = !c.fcc();
                        c.mov_s(2, 3);
                        if t {
                            break 'b_1340ec;
                        } else {
                            c.cycles += 1;
                            c.add_s(2, 4, 1);
                            break 'b_1340ec;
                        }
                    }
                    'b_134138: {
                        'b_134130: {
                            'b_134114: {
                                c.cycles += 6;
                                c.lwc1(0, at(c, S0, SPIN));
                                c.lwc1(1, at(c, S0, SPIN_TARGET));
                                c.c_lt_s(0, 1);
                                let t = !c.fcc();
                                c.swc1(2, at(c, S0, LEAN_X));
                                if t {
                                    break 'b_134114;
                                } else {
                                    c.cycles += 4;
                                    c.add_s(0, 0, 2);
                                    c.sub_s(0, 0, 4);
                                    c.min_s(0, 1, 0);
                                    break 'b_134130;
                                }
                            }
                            c.cycles += 4;
                            c.c_lt_s(1, 0);
                            if !c.fcc() {
                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                break 'b_134138;
                            } else {
                                c.cycles += 3;
                                c.add_s(0, 0, 2);
                                c.sub_s(0, 0, 4);
                                c.max_s(0, 1, 0);
                                break 'b_134130;
                            }
                        }
                        c.cycles += 2;
                        c.swc1(0, at(c, S0, SPIN));
                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                        break 'b_134138;
                    }
                    'b_134178: {
                        'b_134168: {
                            c.cycles += 10;
                            c.lwc1(1, gp(c, -0x7458));
                            c.lwc1(0, at(c, V0, R_SPIN_SPEED));
                            c.lwc1(2, at(c, S0, SPIN_TARGET));
                            c.mul_s(3, 0, 1);
                            c.lwc1(1, at(c, S0, SPIN));
                            c.add_s(0, 2, 3);
                            c.c_lt_s(0, 1);
                            if !c.fcc() {
                                c.sub_s(0, 2, 3);
                                break 'b_134168;
                            } else {
                                c.cycles += 2;
                                c.sub_s(2, 1, 3);
                                break 'b_134178;
                            }
                        }
                        c.cycles += 4;
                        c.c_lt_s(1, 0);
                        if c.fcc() {
                            c.add_s(2, 1, 3);
                            break 'b_134178;
                        } else {
                            break 'b_134178;
                        }
                    }
                    'b_1341c4: {
                        'b_1341a8: {
                            c.cycles += 9;
                            c.sub_s(1, 2, 1);
                            c.lwc1(0, at(c, S0, SPIN_TOTAL));
                            c.mtc1(0, 3);
                            c.swc1(2, at(c, S0, SPIN));
                            c.add_s(0, 0, 1);
                            c.c_lt_s(23, 3);
                            let t = !c.fcc();
                            c.swc1(0, at(c, S0, SPIN_TOTAL));
                            if t {
                                break 'b_1341a8;
                            } else {
                                c.cycles += 3;
                                c.neg_s(4, 27);
                                c.lwc1(5, at(c, S0, LEAN_Y));
                                break 'b_1341c4;
                            }
                        }
                        'b_1341c0: {
                            c.cycles += 4;
                            c.c_le_s(23, 3);
                            let t = !c.fcc();
                            c.lwc1(5, at(c, S0, LEAN_Y));
                            if t {
                                break 'b_1341c0;
                            } else {
                                c.cycles += 2;
                                c.mov_s(4, 5);
                                break 'b_1341c4;
                            }
                        }
                        c.cycles += 1;
                        c.mov_s(4, 27);
                        break 'b_1341c4;
                    }
                    'b_1341dc: {
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_lt_s(0, 4);
                        let t = !c.fcc();
                        c.lwc1(0, gp(c, 0x1bbc));
                        if t {
                            break 'b_1341dc;
                        } else {
                            c.cycles += 1;
                            c.min_s(4, 0, 4);
                            break 'b_1341dc;
                        }
                    }
                    'b_13421c: {
                        c.cycles += 15;
                        c.g[AT][0] = 0x41200000;
                        c.mtc1(1, 1);
                        c.sub_s(0, 4, 5);
                        c.abs_s(0, 0);
                        c.lwc1(3, gp(c, 0x1bac));
                        c.mul_s(0, 0, 1);
                        c.lwc1(2, gp(c, -0x7454));
                        c.abs_s(6, 23);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 1);
                        c.div_s(0, 0, 3);
                        c.c_le_s(2, 0);
                        let t = !c.fcc();
                        c.mov_s(7, 2);
                        if t {
                            break 'b_13421c;
                        } else {
                            c.cycles += 1;
                            c.min_s(7, 0, 1);
                            break 'b_13421c;
                        }
                    }
                    'b_134268: {
                        'b_134254: {
                            c.cycles += 12;
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 2);
                            c.lwc1(1, gp(c, -0x7450));
                            c.mul_s(0, 6, 2);
                            c.mul_s(1, 7, 1);
                            c.add_s(0, 0, 2);
                            c.mul_s(1, 0, 1);
                            c.add_s(2, 4, 1);
                            c.c_lt_s(2, 5);
                            if !c.fcc() {
                                c.sub_s(0, 4, 1);
                                break 'b_134254;
                            } else {
                                c.cycles += 2;
                                c.sub_s(2, 5, 1);
                                break 'b_134268;
                            }
                        }
                        c.cycles += 4;
                        c.c_lt_s(5, 0);
                        let t = !c.fcc();
                        c.mov_s(2, 4);
                        if t {
                            break 'b_134268;
                        } else {
                            c.cycles += 1;
                            c.add_s(2, 5, 1);
                            break 'b_134268;
                        }
                    }
                    'b_1342b4: {
                        'b_1342ac: {
                            'b_134290: {
                                c.cycles += 6;
                                c.lwc1(0, at(c, S0, FLIP));
                                c.lwc1(1, at(c, S0, FLIP_TARGET));
                                c.c_lt_s(0, 1);
                                let t = !c.fcc();
                                c.swc1(2, at(c, S0, LEAN_Y));
                                if t {
                                    break 'b_134290;
                                } else {
                                    c.cycles += 4;
                                    c.add_s(0, 0, 2);
                                    c.sub_s(0, 0, 5);
                                    c.min_s(0, 1, 0);
                                    break 'b_1342ac;
                                }
                            }
                            c.cycles += 4;
                            c.c_lt_s(1, 0);
                            if !c.fcc() {
                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                break 'b_1342b4;
                            } else {
                                c.cycles += 3;
                                c.add_s(0, 0, 2);
                                c.sub_s(0, 0, 5);
                                c.max_s(0, 1, 0);
                                break 'b_1342ac;
                            }
                        }
                        c.cycles += 2;
                        c.swc1(0, at(c, S0, FLIP));
                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                        break 'b_1342b4;
                    }
                    'b_1342f8: {
                        'b_1342e4: {
                            c.cycles += 10;
                            c.lwc1(1, gp(c, -0x744c));
                            c.lwc1(0, at(c, V0, R_FLIP_SPEED));
                            c.lwc1(4, at(c, S0, FLIP_TARGET));
                            c.mul_s(1, 0, 1);
                            c.lwc1(2, at(c, S0, FLIP));
                            c.add_s(0, 4, 1);
                            c.c_lt_s(0, 2);
                            if !c.fcc() {
                                c.sub_s(0, 4, 1);
                                break 'b_1342e4;
                            } else {
                                c.cycles += 2;
                                c.sub_s(6, 2, 1);
                                break 'b_1342f8;
                            }
                        }
                        c.cycles += 4;
                        c.c_lt_s(2, 0);
                        let t = !c.fcc();
                        c.mov_s(6, 4);
                        if t {
                            break 'b_1342f8;
                        } else {
                            c.cycles += 1;
                            c.add_s(6, 2, 1);
                            break 'b_1342f8;
                        }
                    }
                    'b_134340: {
                        c.cycles += 18;
                        c.lwc1(1, at(c, S0, SPIN));
                        c.sub_s(4, 6, 2);
                        c.sub_s(5, 6, 8);
                        c.lwc1(2, at(c, S0, FLIP_TOTAL));
                        c.sub_s(1, 1, 9);
                        c.lwc1(0, at(c, S0, SPIN_AMOUNT));
                        c.lwc1(3, at(c, S0, FLIP_AMOUNT));
                        c.add_s(2, 2, 4);
                        c.swc1(6, at(c, S0, FLIP));
                        c.add_s(0, 0, 1);
                        c.g[V1][0] = c.lw(at(c, S0, RIDER));
                        c.add_s(3, 3, 5);
                        c.swc1(2, at(c, S0, FLIP_TOTAL));
                        c.swc1(0, at(c, S0, SPIN_AMOUNT));
                        c.swc1(3, at(c, S0, FLIP_AMOUNT));
                        c.g[V0][0] = c.lw(at(c, V1, R_STANCE_TAKEOFF));
                        if c.g[V0][0] != 0u64 {
                            c.neg_s(0, 0);
                            break 'b_134340;
                        } else {
                            break 'b_134340;
                        }
                    }
                    'b_134c70: {
                        'b_134c3c: {
                            c.cycles += 39;
                            c.g[A0][0] = c.lw(at(c, V1, R_METRIX));
                            c.mov_s(12, 0);
                            call_ptr(c, spin_callee::METRIX_SET_SPIN_AMOUNT, 0x0013434c)?;
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                            c.mov_s(12, 0);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, spin_callee::RIDER_ADD_BOOST, 0x0013435c)?;
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.lwc1(12, at(c, S0, FLIP_AMOUNT));
                            c.g[A0][0] = c.lw(at(c, V0, R_METRIX));
                            call_ptr(c, spin_callee::METRIX_SET_FLIP_AMOUNT, 0x0013436c)?;
                            c.g[A0][0] = c.lw(at(c, S0, RIDER));
                            c.mov_s(12, 0);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, spin_callee::RIDER_ADD_BOOST, 0x0013437c)?;
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.g[A1][0] = 0x2;
                            c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                            call_ptr(c, spin_callee::ANIM_GET_LAYER_ANIM_ID, 0x0013438c)?;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 3);
                            c.g[S1][0] = c.g[V0][0];
                            c.lwc1(2, gp(c, 0x1ba8));
                            c.g[S4][0] = c.g[S1][0];
                            c.lwc1(1, at(c, S0, LEAN_X));
                            c.div_s(2, 3, 2);
                            c.lwc1(0, gp(c, 0x1bac));
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.div_s(3, 3, 0);
                            c.lwc1(4, gp(c, -0x7448));
                            c.swc1(4, at(c, V0, R_LEAN_X_BLEND));
                            c.mul_s(1, 1, 2);
                            c.swc1(1, at(c, V0, R_LEAN_X));
                            c.lwc1(0, at(c, S0, LEAN_Y));
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.mul_s(0, 0, 3);
                            c.swc1(4, at(c, V0, R_LEAN_Y_BLEND));
                            let t = c.g[S2][0] != 0u64;
                            c.swc1(0, at(c, V0, R_LEAN_Y));
                            if t {
                                break 'b_134c3c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0x120;
                                let t = c.g[S1][0] == c.g[V0][0];

                                if t {
                                    break 'b_134c3c;
                                } else {
                                    'b_134554: {
                                        'b_134418: {
                                            c.cycles += 6;
                                            c.lwc1(0, at(c, S0, LEAN_X));
                                            c.mtc1(0, 22);
                                            c.c_eq_s(0, 22);
                                            if !c.fcc() {
                                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                break 'b_134418;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(0, at(c, S0, LEAN_Y));
                                                c.c_eq_s(0, 22);
                                                let t = c.fcc();
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_134554;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                    break 'b_134418;
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = 0x2;
                                        c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                                        call_ptr(
                                            c,
                                            spin_callee::ANIM_LAYER_ANIM_ENTRY,
                                            0x00134424,
                                        )?;
                                        c.g[V1][0] = 0x9;
                                        let t = c.g[V0][0] == c.g[V1][0];

                                        if t {
                                            break 'b_134c3c;
                                        } else {
                                            'b_134444: {
                                                c.cycles += 4;
                                                c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                                c.g[V0][0] = c.lw(at(c, V1, R_STANCE));
                                                let t = c.g[V0][0] == 0u64;
                                                c.lwc1(20, at(c, V1, R_298));
                                                if t {
                                                    break 'b_134444;
                                                } else {
                                                    c.cycles += 1;
                                                    c.neg_s(20, 20);
                                                    break 'b_134444;
                                                }
                                            }
                                            'b_1344bc: {
                                                'b_134480: {
                                                    c.cycles += 4;
                                                    c.c_eq_s(20, 22);
                                                    let t = !c.fcc();
                                                    c.lwc1(21, at(c, V1, R_28C));
                                                    if t {
                                                        break 'b_134480;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.c_eq_s(21, 22);
                                                        let t = c.fcc();
                                                        c.mov_s(12, 21);
                                                        if t {
                                                            break 'b_1344bc;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.c_le_s(22, 21);
                                                            c.lwc1(12, gp(c, -0x7444));
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_1344bc;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.lwc1(12, gp(c, -0x7440));

                                                                break 'b_1344bc;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.div_s(12, 21, 20);
                                                call_ptr(c, spin_callee::MATH_ATAN, 0x0013448c)?;
                                                c.c_lt_s(20, 22);
                                                let t = !c.fcc();
                                                c.mov_s(12, 0);
                                                if t {
                                                    break 'b_1344bc;
                                                } else {
                                                    'b_1344b4: {
                                                        c.cycles += 3;
                                                        c.c_lt_s(22, 21);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_1344b4;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.lwc1(0, gp(c, -0x743c));
                                                            c.add_s(12, 12, 0);
                                                            break 'b_1344bc;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    c.lwc1(0, gp(c, -0x7438));
                                                    c.sub_s(12, 12, 0);
                                                    break 'b_1344bc;
                                                }
                                            }
                                            c.cycles += 6;
                                            c.lwc1(1, gp(c, -0x7434));
                                            c.abs_s(0, 12);
                                            c.c_lt_s(1, 0);
                                            let t = c.fcc();
                                            c.g[S1][0] = 0x129;
                                            if t {
                                                break 'b_134c3c;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(0, gp(c, -0x7430));
                                                c.c_lt_s(0, 12);
                                                let t = c.fcc();
                                                c.g[S1][0] = 0x12a;
                                                if t {
                                                    break 'b_134c3c;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(0, gp(c, -0x742c));
                                                    c.c_lt_s(0, 12);
                                                    let t = c.fcc();
                                                    c.g[S1][0] = 0x12b;
                                                    if t {
                                                        break 'b_134c3c;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.lwc1(0, gp(c, -0x7428));
                                                        c.c_lt_s(0, 12);
                                                        let t = c.fcc();
                                                        c.g[S1][0] = 0x12c;
                                                        if t {
                                                            break 'b_134c3c;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(0, gp(c, -0x7424));
                                                            c.c_lt_s(12, 0);
                                                            let t = c.fcc();
                                                            c.g[S1][0] = 0x130;
                                                            if t {
                                                                break 'b_134c3c;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.lwc1(0, gp(c, -0x7420));
                                                                c.c_lt_s(12, 0);
                                                                let t = c.fcc();
                                                                c.g[S1][0] = 0x12f;
                                                                if t {
                                                                    break 'b_134c3c;
                                                                } else {
                                                                    c.cycles += 5;
                                                                    c.lwc1(0, gp(c, -0x741c));
                                                                    c.c_lt_s(12, 0);
                                                                    let t = !c.fcc();
                                                                    c.g[S1][0] = 0x12d;
                                                                    if t {
                                                                        break 'b_134c3c;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.g[S1][0] = 0x12e;
                                                                        break 'b_134c3c;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    'b_134748: {
                                        c.cycles += 2;
                                        let t = c.g[S3][0] != c.g[V0][0];
                                        c.g[V1][0] = c.lw(at(c, S0, PHASE));
                                        if t {
                                            break 'b_134748;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
                                            let t = c.g[V0][0] == 0u64;

                                            if t {
                                                break 'b_134748;
                                            } else {
                                                'b_134650: {
                                                    'b_134648: {
                                                        'b_13463c: {
                                                            'b_134614: {
                                                                'b_1345f8: {
                                                                    'b_1345d4: {
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] =
                                                                            c.lw(at(c, S0, RIDER));
                                                                        c.g[V1][0] = c.lw(at(
                                                                            c, V0, R_STANCE,
                                                                        ));
                                                                        let t = c.g[V1][0] == 0u64;
                                                                        c.lwc1(
                                                                            0,
                                                                            stack(c, STICK_SPIN),
                                                                        );
                                                                        if t {
                                                                            break 'b_1345d4;
                                                                        } else {
                                                                            'b_1345a4: {
                                                                                c.cycles += 5;
                                                                                c.neg_s(20, 0);
                                                                                c.c_eq_s(20, 22);
                                                                                let t = !c.fcc();
                                                                                c.lwc1(
                                                                                    21,
                                                                                    stack(
                                                                                        c,
                                                                                        STICK_FLIP,
                                                                                    ),
                                                                                );
                                                                                if t {
                                                                                    break 'b_1345a4;
                                                                                } else {
                                                                                    c.cycles += 4;
                                                                                    c.c_eq_s(
                                                                                        21, 22,
                                                                                    );
                                                                                    let t = c.fcc();
                                                                                    c.mov_s(12, 21);
                                                                                    if t {
                                                                                        break 'b_134650;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;

                                                                                        break 'b_1345f8;
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 7;
                                                                            c.div_s(12, 21, 20);
                                                                            call_ptr(c, spin_callee::MATH_ATAN, 0x001345b0)?;
                                                                            c.c_lt_s(20, 22);
                                                                            let t = !c.fcc();
                                                                            c.mov_s(12, 0);
                                                                            if t {
                                                                                break 'b_134650;
                                                                            } else {
                                                                                c.cycles += 3;
                                                                                c.c_lt_s(22, 21);
                                                                                let t = c.fcc();

                                                                                if t {
                                                                                    break 'b_13463c;
                                                                                } else {
                                                                                    c.cycles += 2;

                                                                                    break 'b_134648;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 5;
                                                                    c.lwc1(
                                                                        20,
                                                                        stack(c, STICK_SPIN),
                                                                    );
                                                                    c.c_eq_s(20, 22);
                                                                    let t = !c.fcc();
                                                                    c.lwc1(
                                                                        21,
                                                                        stack(c, STICK_FLIP),
                                                                    );
                                                                    if t {
                                                                        break 'b_134614;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.c_eq_s(21, 22);
                                                                        let t = c.fcc();
                                                                        c.mov_s(12, 21);
                                                                        if t {
                                                                            break 'b_134650;
                                                                        } else {
                                                                            break 'b_1345f8;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.c_le_s(22, 21);
                                                                c.lwc1(12, gp(c, -0x7418));
                                                                let t = !c.fcc();

                                                                if t {
                                                                    break 'b_134650;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.lwc1(12, gp(c, -0x7414));

                                                                    break 'b_134650;
                                                                }
                                                            }
                                                            c.cycles += 7;
                                                            c.div_s(12, 21, 20);
                                                            call_ptr(
                                                                c,
                                                                spin_callee::MATH_ATAN,
                                                                0x00134620,
                                                            )?;
                                                            c.c_lt_s(20, 22);
                                                            let t = !c.fcc();
                                                            c.mov_s(12, 0);
                                                            if t {
                                                                break 'b_134650;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.c_lt_s(22, 21);
                                                                let t = !c.fcc();

                                                                if t {
                                                                    break 'b_134648;
                                                                } else {
                                                                    break 'b_13463c;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.lwc1(0, gp(c, -0x7410));
                                                        c.add_s(12, 12, 0);
                                                        break 'b_134650;
                                                    }
                                                    c.cycles += 2;
                                                    c.lwc1(0, gp(c, -0x740c));
                                                    c.sub_s(12, 12, 0);
                                                    break 'b_134650;
                                                }
                                                'b_1346e4: {
                                                    c.cycles += 6;
                                                    c.lwc1(1, gp(c, -0x7408));
                                                    c.abs_s(0, 12);
                                                    c.c_lt_s(1, 0);
                                                    let t = c.fcc();
                                                    c.g[S1][0] = 0x10e;
                                                    if t {
                                                        break 'b_1346e4;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.lwc1(0, gp(c, -0x7404));
                                                        c.c_lt_s(0, 12);
                                                        let t = c.fcc();
                                                        c.g[S1][0] = 0x112;
                                                        if t {
                                                            break 'b_1346e4;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(0, gp(c, -0x7400));
                                                            c.c_lt_s(0, 12);
                                                            let t = c.fcc();
                                                            c.g[S1][0] = 0x110;
                                                            if t {
                                                                break 'b_1346e4;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.lwc1(0, gp(c, -0x73fc));
                                                                c.c_lt_s(0, 12);
                                                                let t = c.fcc();
                                                                c.g[S1][0] = 0x111;
                                                                if t {
                                                                    break 'b_1346e4;
                                                                } else {
                                                                    c.cycles += 5;
                                                                    c.lwc1(0, gp(c, -0x73f8));
                                                                    c.c_lt_s(12, 0);
                                                                    let t = c.fcc();
                                                                    c.g[S1][0] = 0x114;
                                                                    if t {
                                                                        break 'b_1346e4;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.lwc1(0, gp(c, -0x73f4));
                                                                        c.c_lt_s(12, 0);
                                                                        let t = c.fcc();
                                                                        c.g[S1][0] = 0x10f;
                                                                        if t {
                                                                            break 'b_1346e4;
                                                                        } else {
                                                                            c.cycles += 5;
                                                                            c.lwc1(
                                                                                0,
                                                                                gp(c, -0x73f0),
                                                                            );
                                                                            c.c_lt_s(12, 0);
                                                                            let t = !c.fcc();
                                                                            c.g[S1][0] = 0x10d;
                                                                            if t {
                                                                                break 'b_1346e4;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.g[S1][0] = 0x113;
                                                                                break 'b_1346e4;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 25;
                                                spin_callee::BE_LIBRARY_GET_INSTANCE
                                                    .call(c, 0x001346ec)?;
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A1][0] = 0x3;
                                                spin_callee::BE_LIBRARY_GET_INTERFACE
                                                    .call(c, 0x001346f8)?;
                                                c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A2][0] = c.lw(at(c, V1, R_CHARACTER));
                                                c.g[A1][0] = c.lw(at(c, V1, R_PLAYER));
                                                spin_callee::BE_CHAR_INT_GET_ATTRIB4_FACTOR
                                                    .call(c, 0x0013470c)?;
                                                c.lwc1(1, gp(c, -0x73ec));
                                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                c.mul_s(0, 0, 1);
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 3);
                                                c.lwc1(1, at(c, V0, R_FLIP_SPEED));
                                                c.lwc1(2, at(c, V0, R_SPIN_SPEED));
                                                c.add_s(0, 0, 3);
                                                c.lwc1(4, gp(c, -0x73e8));
                                                c.max_s(1, 1, 2);
                                                c.g[V1][0] = c.lw(at(c, V0, R_ANIM));
                                                c.mul_s(0, 0, 1);
                                                c.mul_s(0, 0, 4);
                                                c.swc1(0, at(c, V1, 0x1c));
                                                break 'b_134c3c;
                                            }
                                        }
                                    }
                                    'b_13477c: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_13477c;
                                        } else {
                                            c.cycles += 5;
                                            c.lwc1(0, at(c, S0, SPIN_TARGET));
                                            c.mtc1(0, 1);
                                            c.c_eq_s(0, 1);
                                            let t = !c.fcc();

                                            if t {
                                                break 'b_13477c;
                                            } else {
                                                c.cycles += 5;
                                                c.lwc1(0, at(c, S0, FLIP_TARGET));
                                                c.c_eq_s(0, 1);
                                                if c.fcc() {
                                                    c.g[S1][0] = 0x11f;
                                                    break 'b_134c3c;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = 0x1;
                                                    break 'b_13477c;
                                                }
                                            }
                                        }
                                    }
                                    'b_1349d4: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_1349d4;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw(at(c, S0, TURNED));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x2;
                                            if t {
                                                break 'b_134c3c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                                                call_ptr(
                                                    c,
                                                    spin_callee::ANIM_LAYER_ANIM_ENTRY,
                                                    0x0013479c,
                                                )?;
                                                c.g[V1][0] = 0x2;
                                                let t = c.g[V0][0] != c.g[V1][0];

                                                if t {
                                                    break 'b_134c3c;
                                                } else {
                                                    'b_1347d8: {
                                                        c.cycles += 6;
                                                        c.lwc1(1, at(c, S0, SPIN));
                                                        c.lwc1(0, at(c, S0, SPIN_TARGET));
                                                        c.c_eq_s(1, 0);
                                                        if !c.fcc() {
                                                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                            break 'b_1347d8;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(1, at(c, S0, FLIP));
                                                            c.lwc1(0, at(c, S0, FLIP_TARGET));
                                                            c.c_eq_s(1, 0);
                                                            let t = c.fcc();

                                                            if t {
                                                                break 'b_134c3c;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                                break 'b_1347d8;
                                                            }
                                                        }
                                                    }
                                                    'b_1347fc: {
                                                        c.cycles += 5;
                                                        c.g[A1][0] = 0x2;
                                                        c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                                                        call_ptr(c, spin_callee::ANIM_IS_LAYER_ANIM_FINISHED, 0x001347e4)?;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.lwc1(1, at(c, S0, FLIP_TARGET));
                                                            break 'b_1347fc;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = 0x11f;
                                                            let t = c.g[S1][0] != c.g[V0][0];

                                                            if t {
                                                                break 'b_134c3c;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.lwc1(1, at(c, S0, FLIP_TARGET));
                                                                break 'b_1347fc;
                                                            }
                                                        }
                                                    }
                                                    'b_13483c: {
                                                        'b_134838: {
                                                            'b_134830: {
                                                                c.cycles += 5;
                                                                c.lwc1(0, at(c, S0, FLIP));
                                                                c.c_le_s(0, 1);
                                                                if !c.fcc() {
                                                                    c.lwc1(
                                                                        1,
                                                                        at(c, S0, SPIN_TARGET),
                                                                    );
                                                                    break 'b_134830;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.c_lt_s(0, 1);
                                                                    c.mtc1(0, 22);
                                                                    let t = !c.fcc();
                                                                    c.lwc1(
                                                                        1,
                                                                        at(c, S0, SPIN_TARGET),
                                                                    );
                                                                    if t {
                                                                        break 'b_134838;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[AT][0] = 0x3f800000;
                                                                        c.mtc1(1, 22);
                                                                        c.lwc1(0, at(c, S0, SPIN));
                                                                        break 'b_13483c;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            c.g[AT][0] = 0xffffffffbf800000;
                                                            c.mtc1(1, 22);
                                                            break 'b_134838;
                                                        }
                                                        c.cycles += 1;
                                                        c.lwc1(0, at(c, S0, SPIN));
                                                        break 'b_13483c;
                                                    }
                                                    'b_134878: {
                                                        'b_134874: {
                                                            'b_13486c: {
                                                                c.cycles += 4;
                                                                c.c_le_s(0, 1);
                                                                let t = !c.fcc();
                                                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                                if t {
                                                                    break 'b_13486c;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.c_lt_s(0, 1);
                                                                    c.mtc1(0, 21);
                                                                    let t = !c.fcc();

                                                                    if t {
                                                                        break 'b_134874;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[AT][0] = 0x3f800000;
                                                                        c.mtc1(1, 21);
                                                                        c.g[V1][0] = c.lw(at(
                                                                            c, V0, R_STANCE,
                                                                        ));
                                                                        break 'b_134878;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            c.g[AT][0] = 0xffffffffbf800000;
                                                            c.mtc1(1, 21);
                                                            break 'b_134874;
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V1][0] = c.lw(at(c, V0, R_STANCE));
                                                        break 'b_134878;
                                                    }
                                                    'b_134958: {
                                                        'b_134950: {
                                                            'b_134944: {
                                                                'b_13491c: {
                                                                    'b_134904: {
                                                                        'b_1348e0: {
                                                                            c.cycles += 2;
                                                                            let t =
                                                                                c.g[V1][0] == 0u64;

                                                                            if t {
                                                                                break 'b_1348e0;
                                                                            } else {
                                                                                'b_1348b0: {
                                                                                    c.cycles += 5;
                                                                                    c.neg_s(20, 21);
                                                                                    c.mtc1(0, 21);
                                                                                    c.c_eq_s(
                                                                                        20, 21,
                                                                                    );
                                                                                    let t =
                                                                                        !c.fcc();

                                                                                    if t {
                                                                                        break 'b_1348b0;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.c_eq_s(
                                                                                            22, 21,
                                                                                        );
                                                                                        let t =
                                                                                            c.fcc();
                                                                                        c.mov_s(
                                                                                            12, 22,
                                                                                        );
                                                                                        if t {
                                                                                            break 'b_134958;
                                                                                        } else {
                                                                                            c.cycles += 3;
                                                                                            c.c_le_s(21, 22);

                                                                                            break 'b_134904;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 7;
                                                                                c.div_s(12, 22, 20);
                                                                                call_ptr(c, spin_callee::MATH_ATAN, 0x001348bc)?;
                                                                                c.c_lt_s(20, 21);
                                                                                let t = !c.fcc();
                                                                                c.mov_s(12, 0);
                                                                                if t {
                                                                                    break 'b_134958;
                                                                                } else {
                                                                                    c.cycles += 3;
                                                                                    c.c_lt_s(
                                                                                        21, 22,
                                                                                    );
                                                                                    let t = c.fcc();

                                                                                    if t {
                                                                                        break 'b_134944;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;

                                                                                        break 'b_134950;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 4;
                                                                        c.mtc1(0, 20);
                                                                        c.c_eq_s(21, 20);
                                                                        let t = !c.fcc();

                                                                        if t {
                                                                            break 'b_13491c;
                                                                        } else {
                                                                            c.cycles += 4;
                                                                            c.c_eq_s(22, 20);
                                                                            let t = c.fcc();
                                                                            c.mov_s(12, 22);
                                                                            if t {
                                                                                break 'b_134958;
                                                                            } else {
                                                                                c.cycles += 1;
                                                                                c.c_le_s(20, 22);
                                                                                break 'b_134904;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 3;
                                                                    c.lwc1(12, gp(c, -0x73e4));
                                                                    let t = !c.fcc();

                                                                    if t {
                                                                        break 'b_134958;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.lwc1(12, gp(c, -0x73e0));

                                                                        break 'b_134958;
                                                                    }
                                                                }
                                                                c.cycles += 7;
                                                                c.div_s(12, 22, 21);
                                                                call_ptr(
                                                                    c,
                                                                    spin_callee::MATH_ATAN,
                                                                    0x00134928,
                                                                )?;
                                                                c.c_lt_s(21, 20);
                                                                let t = !c.fcc();
                                                                c.mov_s(12, 0);
                                                                if t {
                                                                    break 'b_134958;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.c_lt_s(20, 22);
                                                                    let t = !c.fcc();

                                                                    if t {
                                                                        break 'b_134950;
                                                                    } else {
                                                                        break 'b_134944;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.lwc1(0, gp(c, -0x73dc));
                                                            c.add_s(12, 12, 0);
                                                            break 'b_134958;
                                                        }
                                                        c.cycles += 2;
                                                        c.lwc1(0, gp(c, -0x73d8));
                                                        c.sub_s(12, 12, 0);
                                                        break 'b_134958;
                                                    }
                                                    c.cycles += 6;
                                                    c.lwc1(1, gp(c, -0x73d4));
                                                    c.abs_s(0, 12);
                                                    c.c_lt_s(1, 0);
                                                    let t = c.fcc();
                                                    c.g[S1][0] = 0x126;
                                                    if t {
                                                        break 'b_134c3c;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.lwc1(0, gp(c, -0x73d0));
                                                        c.c_lt_s(0, 12);
                                                        let t = c.fcc();

                                                        if t {
                                                            break 'b_134c3c;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.lwc1(0, gp(c, -0x73cc));
                                                            c.c_lt_s(0, 12);
                                                            let t = c.fcc();
                                                            c.g[S1][0] = 0x125;
                                                            if t {
                                                                break 'b_134c3c;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.lwc1(0, gp(c, -0x73c8));
                                                                c.c_lt_s(0, 12);
                                                                let t = c.fcc();

                                                                if t {
                                                                    break 'b_134c3c;
                                                                } else {
                                                                    c.cycles += 5;
                                                                    c.lwc1(0, gp(c, -0x73c4));
                                                                    c.c_lt_s(12, 0);
                                                                    let t = c.fcc();
                                                                    c.g[S1][0] = 0x128;
                                                                    if t {
                                                                        break 'b_134c3c;
                                                                    } else {
                                                                        c.cycles += 5;
                                                                        c.lwc1(0, gp(c, -0x73c0));
                                                                        c.c_lt_s(12, 0);
                                                                        if !c.fcc() {
                                                                            c.g[S1][0] = 0x127;
                                                                            break 'b_134c3c;
                                                                        } else {
                                                                            c.cycles += 2;

                                                                            break 'b_134c3c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    'b_134b80: {
                                        c.cycles += 2;
                                        if c.g[S3][0] == c.g[V0][0] {
                                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                            break 'b_134b80;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[V1][0] != c.g[V0][0] {
                                                c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                break 'b_134b80;
                                            } else {
                                                'b_134a50: {
                                                    'b_134a4c: {
                                                        c.cycles += 4;
                                                        c.g[V1][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_sub(0x10d));
                                                        c.g[V0][0] = (c.g[V1][0] < 0x1cu64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x460000;
                                                        if t {
                                                            break 'b_134a4c;
                                                        } else {
                                                            'b_134a44: {
                                                                'b_134a3c: {
                                                                    'b_134a34: {
                                                                        'b_134a2c: {
                                                                            'b_134a24: {
                                                                                'b_134a1c: {
                                                                                    'b_134a14: {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7e60));
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                        // The landing blend for the current pose: the original's jump table
                                                                                        // (.rodata 0x4581a0, indexed by `id - 0x10d`), read as it does.
                                                                                        c.g[A0][0] = c.lw(at(c, V1, 0));
                                                                                        let t = c.g
                                                                                            [A0][0]
                                                                                            as u32;
                                                                                        match t {
                                                                                            0x00134a0c => {
                                                                                                c.cycles += 2;
                                                                                                c.g[S1][0] = 0x132;
                                                                                                break 'b_134a50;
                                                                                            }
                                                                                            0x00134a14 => {
                                                                                                break 'b_134a14;
                                                                                            }
                                                                                            0x00134a1c => {
                                                                                                break 'b_134a1c;
                                                                                            }
                                                                                            0x00134a24 => {
                                                                                                break 'b_134a24;
                                                                                            }
                                                                                            0x00134a2c => {
                                                                                                break 'b_134a2c;
                                                                                            }
                                                                                            0x00134a34 => {
                                                                                                break 'b_134a34;
                                                                                            }
                                                                                            0x00134a3c => {
                                                                                                break 'b_134a3c;
                                                                                            }
                                                                                            0x00134a44 => {
                                                                                                break 'b_134a44;
                                                                                            }
                                                                                            0x00134a4c => {
                                                                                                break 'b_134a4c;
                                                                                            }
                                                                                            _ => {
                                                                                                c.jump_out(t, 0x00133308, 0x00134cb0);
                                                                                                return Ok(());
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 2;
                                                                                    c.g[S1][0] =
                                                                                        0x133;
                                                                                    break 'b_134a50;
                                                                                }
                                                                                c.cycles += 2;
                                                                                c.g[S1][0] = 0x134;
                                                                                break 'b_134a50;
                                                                            }
                                                                            c.cycles += 2;
                                                                            c.g[S1][0] = 0x135;
                                                                            break 'b_134a50;
                                                                        }
                                                                        c.cycles += 2;
                                                                        c.g[S1][0] = 0x136;
                                                                        break 'b_134a50;
                                                                    }
                                                                    c.cycles += 2;
                                                                    c.g[S1][0] = 0x137;
                                                                    break 'b_134a50;
                                                                }
                                                                c.cycles += 2;
                                                                c.g[S1][0] = 0x138;
                                                                break 'b_134a50;
                                                            }
                                                            c.cycles += 2;
                                                            c.g[S1][0] = 0x139;
                                                            break 'b_134a50;
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[S1][0] = 0x11f;
                                                    break 'b_134a50;
                                                }
                                                c.cycles += 2;
                                                let t = c.g[S1][0] == c.g[S4][0];
                                                c.g[A1][0] = c.g[S1][0];
                                                if t {
                                                    break 'b_134c70;
                                                } else {
                                                    'b_134aec: {
                                                        c.cycles += 37;
                                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 21);
                                                        c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                                                        call_ptr(c, spin_callee::ANIM_GET_CURRENT_ANIM_LENGTH, 0x00134a6c)?;
                                                        c.lwc1(1, at(c, S0, SPIN_TARGET));
                                                        c.mov_s(22, 0);
                                                        c.lwc1(5, at(c, S0, SPIN));
                                                        c.lwc1(2, at(c, S0, FLIP_TARGET));
                                                        c.lwc1(4, at(c, S0, FLIP));
                                                        c.sub_s(5, 5, 1);
                                                        c.lwc1(0, gp(c, -0x73bc));
                                                        c.abs_s(5, 5);
                                                        c.sub_s(4, 4, 2);
                                                        c.lwc1(9, gp(c, -0x73b8));
                                                        c.mul_s(1, 5, 0);
                                                        c.abs_s(4, 4);
                                                        c.mul_s(0, 4, 0);
                                                        c.lwc1(6, gp(c, -0x73b4));
                                                        c.lwc1(8, at(c, S0, SPIN_RATE));
                                                        c.sub_s(3, 21, 1);
                                                        c.lwc1(7, at(c, S0, FLIP_RATE));
                                                        c.sub_s(2, 21, 0);
                                                        c.mul_s(0, 0, 9);
                                                        c.mul_s(3, 3, 6);
                                                        c.mul_s(2, 2, 6);
                                                        c.mul_s(1, 1, 9);
                                                        c.add_s(0, 0, 2);
                                                        c.add_s(1, 1, 3);
                                                        c.max_s(0, 0, 7);
                                                        c.max_s(1, 1, 8);
                                                        c.div_s(4, 4, 0);
                                                        c.div_s(20, 5, 1);
                                                        c.c_lt_s(20, 4);
                                                        if c.fcc() {
                                                            c.mov_s(20, 4);
                                                            break 'b_134aec;
                                                        } else {
                                                            break 'b_134aec;
                                                        }
                                                    }
                                                    'b_134b0c: {
                                                        c.cycles += 6;
                                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                        c.mtc1(0, 1);
                                                        c.lwc1(0, at(c, V0, R_2EC));
                                                        c.c_lt_s(1, 0);
                                                        let t = !c.fcc();

                                                        if t {
                                                            break 'b_134b0c;
                                                        } else {
                                                            c.cycles += 2;
                                                            c.lwc1(0, gp(c, -0x73b0));
                                                            c.mul_s(20, 20, 0);
                                                            break 'b_134b0c;
                                                        }
                                                    }
                                                    'b_134b68: {
                                                        c.cycles += 22;
                                                        spin_callee::BE_LIBRARY_GET_INSTANCE
                                                            .call(c, 0x00134b14)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[A1][0] = 0x3;
                                                        spin_callee::BE_LIBRARY_GET_INTERFACE
                                                            .call(c, 0x00134b20)?;
                                                        c.g[V1][0] = c.lw(at(c, S0, RIDER));
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[A2][0] = c.lw(at(c, V1, R_CHARACTER));
                                                        c.g[A1][0] = c.lw(at(c, V1, R_PLAYER));
                                                        spin_callee::BE_CHAR_INT_GET_ATTRIB4_FACTOR
                                                            .call(c, 0x00134b34)?;
                                                        c.lwc1(1, gp(c, -0x73ac));
                                                        c.lwc1(2, gp(c, -0x73a8));
                                                        c.mul_s(0, 0, 1);
                                                        c.g[AT][0] = 0x40000000;
                                                        c.mtc1(1, 1);
                                                        c.add_s(0, 0, 21);
                                                        c.div_s(0, 21, 0);
                                                        c.mul_s(20, 20, 0);
                                                        c.c_lt_s(2, 20);
                                                        let t = !c.fcc();
                                                        c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                        if t {
                                                            break 'b_134b68;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.div_s(1, 22, 20);
                                                            break 'b_134b68;
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[AT][0] = 0x40000000;
                                                    c.mtc1(1, 0);
                                                    c.g[V1][0] = c.lw(at(c, V0, R_ANIM));
                                                    c.min_s(0, 1, 0);
                                                    c.swc1(0, at(c, V1, 0x1c));
                                                    break 'b_134c3c;
                                                }
                                            }
                                        }
                                    }
                                    'b_134ba8: {
                                        'b_134ba4: {
                                            c.cycles += 6;
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] = 0x1;
                                            c.g[V1][0] = c.lw(at(c, V0, R_ANIM_STATE));
                                            c.g[V1][0] = c.lw(at(c, V1, 0xac));
                                            let t = c.g[V1][0] == c.g[A0][0];
                                            c.g[A2][0] = c.g[V0][0];
                                            if t {
                                                break 'b_134ba4;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x3;
                                                let t = c.g[V1][0] != c.g[V0][0];

                                                if t {
                                                    break 'b_134ba8;
                                                } else {
                                                    break 'b_134ba4;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A1][0] = 0x1;
                                        break 'b_134ba8;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[A1][0] == 0u64;
                                    c.g[A1][0] = 0x131;
                                    if t {
                                        break 'b_134c3c;
                                    } else {
                                        'b_134c24: {
                                            'b_134c20: {
                                                c.cycles += 9;
                                                c.g[V0][0] = c.lw(at(c, A2, R_ANIM_STATE));
                                                c.g[A0][0] = c.lw(at(c, A2, R_ANIM));
                                                c.lwc1(1, at(c, V0, 0xa0));
                                                c.lwc1(0, at(c, V0, 0x98));
                                                c.sub_s(20, 0, 1);
                                                call_ptr(
                                                    c,
                                                    spin_callee::ANIM_GET_CURRENT_ANIM_LENGTH,
                                                    0x00134bc8,
                                                )?;
                                                c.g[V0][0] = 0x11f;
                                                let t = c.g[S1][0] != c.g[V0][0];
                                                c.mov_s(1, 0);
                                                if t {
                                                    break 'b_134c20;
                                                } else {
                                                    c.cycles += 4;
                                                    c.c_le_s(20, 1);
                                                    let t = !c.fcc();
                                                    c.g[V0][0] = 0x131;
                                                    if t {
                                                        break 'b_134c24;
                                                    } else {
                                                        'b_134c04: {
                                                            c.cycles += 7;
                                                            c.lwc1(0, gp(c, -0x73a4));
                                                            c.g[AT][0] = 0x3f800000;
                                                            c.mtc1(1, 2);
                                                            c.c_lt_s(0, 20);
                                                            let t = !c.fcc();
                                                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                                                            if t {
                                                                break 'b_134c04;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.div_s(2, 1, 20);
                                                                break 'b_134c04;
                                                            }
                                                        }
                                                        c.cycles += 7;
                                                        c.g[S1][0] = 0x131;
                                                        c.g[AT][0] = 0x40000000;
                                                        c.mtc1(1, 0);
                                                        c.g[V1][0] = c.lw(at(c, V0, R_ANIM));
                                                        c.min_s(0, 2, 0);
                                                        c.swc1(0, at(c, V1, 0x1c));
                                                        break 'b_134c3c;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] = 0x131;
                                            break 'b_134c24;
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S1][0] != c.g[V0][0];

                                        if t {
                                            break 'b_134c3c;
                                        } else {
                                            c.cycles += 4;
                                            c.c_lt_s(1, 20);
                                            if c.fcc() {
                                                c.g[S1][0] = 0x11f;
                                                break 'b_134c3c;
                                            } else {
                                                break 'b_134c3c;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S1][0] == c.g[S4][0];
                        c.g[A1][0] = c.g[S1][0];
                        if t {
                            break 'b_134c70;
                        } else {
                            c.cycles += 11;
                            c.g[V0][0] = c.lw(at(c, S0, RIDER));
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 12);
                            c.g[A2][0] = 0u64;
                            c.g[A0][0] = c.lw(at(c, V0, R_ANIM));
                            call_ptr(c, spin_callee::ANIM_PLAY_ANIM, 0x00134c5c)?;
                            c.g[V1][0] = c.lw(at(c, S0, RIDER));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.g[V0][0] = c.lw(at(c, V1, R_ANIM));
                            c.swc1(0, at(c, V0, 0x1c));
                            break 'b_134c70;
                        }
                    }
                    c.cycles += 1;
                    frame.restore(c, S0, 0x60);
                    break 'b_134c74;
                }
            }
        }
        c.cycles += 15;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 27, 0xa8);
        frame.restore_fpr(c, 26, 0xa0);
        frame.restore_fpr(c, 25, 0x98);
        frame.restore_fpr(c, 24, 0x90);
        frame.restore_fpr(c, 23, 0x88);
        frame.restore_fpr(c, 22, 0x80);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderSpinSub_ApplyPoseRotation (0x134dd0, ai.cpp; assembly only).

/// `void Math_SinCos(float* sin, float* cos, float angle)` (cross-shard).
pub(crate) const MATH_SIN_COS: u32 = 0x0031_be50;

/// `float RiderSpinSub_GetSpinFactor(RiderSpinSub*)` (same shard).
pub(crate) const RIDER_SPIN_SUB_GET_SPIN_FACTOR: Callee = Callee {
    addr: 0x0013_5180,
    f: ee::s01::RiderSpinSub_GetSpinFactor,
};

/// `void RiderSpinSub_ApplyPoseRotation(RiderSpinSub* self, Pose* pose)`
/// (0x134dd0): turns the rider's pose (`pose`: position at +0, rotation
/// quaternion at +0x10) for the spin sub-state. It
/// 1. scales the current path node's offset (path `+0x24`, node `+0x89c`,
///    16 bytes each) by the path's scale (`+0x140..+0x14c`), takes it
///    relative to the spin pivot (0x4ff130), rotates it by the pose
///    quaternion and adds it to the position;
/// 2. rotates the pose by the spin angle (`-(self+0x20 + self+0x28) / 2`
///    about world up, `Math_SinCos`);
/// 3. eases the lean factor `self+0x40` towards `GetSpinFactor()` by a step
///    proportional to the distance (`gp-0x7384`);
/// 4. rotates the pose by half of `self+0x24 + self+0x2c` about the axis
///    blended from 0x4ff150 and 0x4ff140 by the lean factor (normalised),
///    and moves the position back by the rotated pivot.
/// All vector work is the original's VU0 macro code (quaternion products
/// `opmula/opmsub`, rotations), issued in its order with its stack scratch.
pub fn rider_spin_sub_apply_pose_rotation(c: &mut Cpu) {
    guest(c, |c| {
        let me = c.g[A0][0] as u32;
        let pose = c.g[A1][0] as u32;
        // 1-2: offset to the pivot, rotated into the pose; spin angle.
        c.cycles += 116;
        let frame = Frame::open(c, 0xe0);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 12);
        c.sq(16, sp(c).wrapping_add(0xc0));
        c.sq(17, sp(c).wrapping_add(0xb0));
        c.g[S0][0] = 0x500000;
        c.sq(18, sp(c).wrapping_add(0xa0));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xea0));
        c.sq(19, sp(c).wrapping_add(0x90));
        c.g[S2][0] = c.g[A0][0];
        c.swc1(20, sp(c).wrapping_add(0xd0));
        c.g[S3][0] = c.g[A1][0];
        c.sd(sp(c).wrapping_add(0x80), c.g[RA][0]);
        c.g[A0][0] = sx(sp(c).wrapping_add(0x70));
        c.g[A1][0] = sx(sp(c).wrapping_add(0x74));
        c.g[A2][0] = c.lw(me.wrapping_add(0x58));
        c.lwc1(6, me.wrapping_add(0x28));
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x780));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x89c));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x14c));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x140));
        c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0xc));
        c.lwc1(1, c.g[V0][0] as u32);
        c.mul_s(4, 4, 0);
        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.mul_s(1, 1, 3);
        c.lwc1(5, (c.g[V1][0] as u32).wrapping_add(0x144));
        c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x148));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.mul_s(2, 2, 5);
        c.lwc1(7, me.wrapping_add(0x20));
        c.g[V0][0] = 0x500000;
        c.mul_s(0, 0, 3);
        c.swc1(4, sp(c).wrapping_add(0x1c));
        c.swc1(2, sp(c).wrapping_add(0x14));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
        c.swc1(1, sp(c).wrapping_add(0x10));
        c.add_s(7, 7, 6);
        c.swc1(0, sp(c).wrapping_add(0x18));
        c.lwc1(0, me.wrapping_add(0x2c));
        c.lwc1(1, me.wrapping_add(0x24));
        c.neg_s(7, 7);
        c.lqc2(3, sp(c).wrapping_add(0x10));
        c.lqc2(4, c.g[V0][0] as u32);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x20));
        c.lq(2, sp(c).wrapping_add(0x20));
        c.add_s(20, 1, 0);
        c.mul_s(12, 7, 12);
        c.sq(2, sp(c));
        c.lqc2(4, pose.wrapping_add(0x10));
        c.lqc2(5, sp(c));
        c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().opmula(4, 6);
        c.vu0().opmsub(7, 6, 4);
        c.vu0().mula(0b1110, 5, Src::Bc(0, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 7, Src::Bc(0, W));
        c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
        c.sqc2(8, sp(c).wrapping_add(0x20));
        c.lq(2, sp(c).wrapping_add(0x20));
        c.sq(2, sp(c).wrapping_add(0x10));
        c.lqc2(3, pose);
        c.lqc2(4, sp(c).wrapping_add(0x10));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, pose);
        call_ptr(c, MATH_SIN_COS, 0x0013_4ef8)?;
        // Spin quaternion (axis = world up x sin, w = cos) times the pose.
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.g[S2][0];
        c.lwc1(0, sp(c).wrapping_add(0x70));
        c.lwc1(2, (c.g[S0][0] as u32).wrapping_sub(0xea0));
        c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.lwc1(3, sp(c).wrapping_add(0x74));
        c.mul_s(0, 0, 4);
        c.swc1(3, sp(c).wrapping_add(0x4c));
        c.swc1(1, sp(c).wrapping_add(0x48));
        c.swc1(2, sp(c).wrapping_add(0x40));
        c.swc1(0, sp(c).wrapping_add(0x44));
        c.lq(2, sp(c).wrapping_add(0x40));
        c.sq(2, sp(c).wrapping_add(0x30));
        c.lqc2(4, pose.wrapping_add(0x10));
        c.lqc2(5, sp(c).wrapping_add(0x30));
        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
        c.sqc2(8, sp(c).wrapping_add(0x60));
        c.lq(2, sp(c).wrapping_add(0x60));
        c.sq(2, pose.wrapping_add(0x10));
        c.sq(2, sp(c).wrapping_add(0x50));
        RIDER_SPIN_SUB_GET_SPIN_FACTOR.call(c, 0x0013_4f74)?;
        // 3: ease the lean factor (self+0x40, f4) towards the spin factor
        // (f3) by step = |current - target| * k (f2).
        c.lwc1(4, me.wrapping_add(0x40));
        c.mov_s(3, 0);
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7384));
        c.sub_s(0, 4, 3);
        c.abs_s(0, 0);
        c.mul_s(2, 0, 2);
        c.add_s(1, 3, 2);
        c.c_lt_s(1, 4);
        if !c.fcc() {
            // target + step >= current: step up if current < target - step,
            // else snap to the target.
            c.sub_s(0, 3, 2);
            c.cycles += 4;
            c.c_lt_s(4, 0);
            let below = c.fcc();
            c.mov_s(1, 3);
            if below {
                c.cycles += 1;
                c.add_s(1, 4, 2);
            }
        } else {
            // current > target + step: step down.
            c.cycles += 2;
            c.sub_s(1, 4, 2);
        }
        // 4: the lean axis and its half-angle rotation.
        c.cycles += 112;
        c.g[V1][0] = 0x500000;
        c.swc1(1, me.wrapping_add(0x40));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xeb0));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.mfc1(2, 1);
        c.lqc2(4, c.g[V1][0] as u32);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp(c).wrapping_add(0x40));
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 12);
        c.lq(2, sp(c).wrapping_add(0x40));
        c.g[A0][0] = sx(sp(c).wrapping_add(0x78));
        c.lwc1(1, me.wrapping_add(0x40));
        c.mul_s(12, 20, 12);
        c.sq(2, sp(c).wrapping_add(0x30));
        c.g[A1][0] = sx(sp(c).wrapping_add(0x7c));
        c.g[V0][0] = 0x500000;
        c.sub_s(0, 0, 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
        c.mfc1(3, 0);
        c.lqc2(4, c.g[V0][0] as u32);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp(c).wrapping_add(0x50));
        c.lq(2, sp(c).wrapping_add(0x50));
        c.sq(2, sp(c).wrapping_add(0x40));
        c.lqc2(3, sp(c).wrapping_add(0x30));
        c.lqc2(4, sp(c).wrapping_add(0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x50));
        c.lq(2, sp(c).wrapping_add(0x50));
        c.sq(2, sp(c).wrapping_add(0x20));
        c.lqc2(3, sp(c).wrapping_add(0x20));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, sp(c).wrapping_add(0x50));
        c.lq(2, sp(c).wrapping_add(0x50));
        c.sq(2, sp(c).wrapping_add(0x10));
        call_ptr(c, MATH_SIN_COS, 0x0013_5078)?;
        c.lwc1(0, sp(c).wrapping_add(0x78));
        c.lwc1(1, sp(c).wrapping_add(0x10));
        c.lwc1(2, sp(c).wrapping_add(0x14));
        c.lwc1(4, sp(c).wrapping_add(0x18));
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.lwc1(3, sp(c).wrapping_add(0x7c));
        c.mul_s(0, 0, 4);
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 5);
        c.swc1(3, sp(c).wrapping_add(0x3c));
        c.swc1(1, sp(c).wrapping_add(0x30));
        c.swc1(2, sp(c).wrapping_add(0x34));
        c.swc1(0, sp(c).wrapping_add(0x38));
        c.lq(16, sp(c).wrapping_add(0xc0));
        c.lq(2, sp(c).wrapping_add(0x30));
        c.lq(17, sp(c).wrapping_add(0xb0));
        c.sq(2, sp(c).wrapping_add(0x20));
        c.lqc2(4, pose.wrapping_add(0x10));
        c.lqc2(5, sp(c).wrapping_add(0x20));
        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
        c.sqc2(8, sp(c).wrapping_add(0x50));
        c.lq(18, sp(c).wrapping_add(0xa0));
        c.lq(2, sp(c).wrapping_add(0x50));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x80));
        c.sq(2, pose.wrapping_add(0x10));
        c.sq(2, sp(c).wrapping_add(0x40));
        c.mfc1(3, 5);
        c.lqc2(4, sp(c));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp(c).wrapping_add(0x30));
        c.lwc1(20, sp(c).wrapping_add(0xd0));
        c.lq(2, sp(c).wrapping_add(0x30));
        c.sq(2, sp(c).wrapping_add(0x20));
        c.lqc2(4, pose.wrapping_add(0x10));
        c.lqc2(5, sp(c).wrapping_add(0x20));
        c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().opmula(4, 6);
        c.vu0().opmsub(7, 6, 4);
        c.vu0().mula(0b1110, 5, Src::Bc(0, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 6, Src::Bc(4, W));
        c.vu0().madda(0b1110, 7, Src::Bc(0, W));
        c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
        c.sqc2(8, sp(c).wrapping_add(0x40));
        c.lq(2, sp(c).wrapping_add(0x40));
        c.sq(2, sp(c).wrapping_add(0x30));
        c.lqc2(3, pose);
        c.lqc2(4, sp(c).wrapping_add(0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, pose);
        c.lq(19, sp(c).wrapping_add(0x90));
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod update_trick {
    use super::*;

    /// `RiderSpinSub` (the trick part): the grab-trick state (0 starting,
    /// 1 wind-up playing, 2 held pose reached, 3 holding, 4 hold pose
    /// looping, 5 releasing), the trick index being played, the rider.
    pub const STATE: u32 = 0x4;

    pub const TRICK: u32 = 0x8;

    pub const RIDER: u32 = 0x58;

    /// Rider fields: uber timer (float), uber level, animation controller,
    /// metrix, AI (player) index, uber flags.
    pub const UBER_TIMER: u32 = 0x2f0;

    pub const UBER_LEVEL: u32 = 0x2f4;

    pub const ANIM_CTRL: u32 = 0x784;

    pub const METRIX: u32 = 0x790;

    pub const AI_INDEX: u32 = 0x86c;

    pub const UBER_FLAGS: u32 = 0xb2c;

    /// Animation sequence (`AnimController_GetLayerSequence`): playback
    /// speed (float) and the 64-bit event flags (bit 0 started, 1 at the
    /// hold point, 2 looped, 3 finished).
    pub const SEQ_SPEED: u32 = 0x90;

    pub const SEQ_FLAGS: u32 = 0xb0;

    /// `.lit4` fade-out times (0.33 s on release / end / chain, 0.1 s when
    /// the hold starts).
    pub const FADE_ON_RELEASE: i32 = -0x735c;

    pub const FADE_ON_HOLD: i32 = -0x7358;

    pub const FADE_ON_END: i32 = -0x7354;

    pub const FADE_ON_CHAIN: i32 = -0x7350;

    pub const ANIM_CONTROLLER_FADE_OUT_LAYER: u32 = 0x0031_1e88;

    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;

    pub const ANIM_CONTROLLER_GET_LAYER_SEQUENCE: u32 = 0x0031_1b20;

    pub const ANIM_CONTROLLER_PLAY_ANIM: u32 = 0x0031_28e8;

    pub const BEBAGT_INT_GET_TRICK_ANIM2: Callee = Callee {
        addr: 0x0015_02c8,
        f: ee::s01::f_001502c8,
    };

    pub const BEBAGT_INT_GET_TRICK_ANIM: Callee = Callee {
        addr: 0x0015_0198,
        f: ee::s01::f_00150198,
    };

    pub const BEBAGT_INT_GET_TRICK_FIELD0: Callee = Callee {
        addr: 0x0015_00d8,
        f: ee::s01::f_001500d8,
    };

    pub const BEBAGT_INT_GET_TRICK_FIELD2: Callee = Callee {
        addr: 0x0015_00f8,
        f: ee::s01::f_001500f8,
    };

    pub const BEBAGT_INT_GET_TRICK_FIELD4: Callee = Callee {
        addr: 0x0015_0118,
        f: ee::s01::f_00150118,
    };

    pub const BEBAGT_INT_GET_TRICK_FIELD6: Callee = Callee {
        addr: 0x0015_0138,
        f: ee::s01::f_00150138,
    };

    pub const BEBAGT_INT_GET_TRICK_FIELD8: Callee = Callee {
        addr: 0x0015_0158,
        f: ee::s01::f_00150158,
    };

    pub const BEBAGT_INT_GET_TRICK_FIELD_A: Callee = Callee {
        addr: 0x0015_0178,
        f: ee::s01::f_00150178,
    };

    pub const BEBAGT_INT_GET_TRICK_VALUE: Callee = Callee {
        addr: 0x0015_03f8,
        f: ee::s01::f_001503f8,
    };

    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };

    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };

    pub const FLAGS64_TEST: Callee = Callee {
        addr: 0x0014_46a0,
        f: ee::s01::f_001446a0,
    };

    pub const RIDER_ADD_BOOST: u32 = 0x0010_e098;

    pub const RIDER_GET_TRICK_SKILL_FACTOR: u32 = 0x0012_0038;

    pub const RIDER_METRIX_ADD_TRICK: u32 = 0x0011_9708;

    pub const RIDER_METRIX_COMPLETE_TRICK: u32 = 0x0011_97d8;

    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;

    pub const SSX_AUDIO_SYSTEM_PLAY_GRAB_TRICK_SOUND: u32 = 0x0029_a530;
}

pub(crate) mod prewind_input {
    use super::*;

    /// `RiderPrewindSub`: the rider.
    pub const RIDER: u32 = 0x0;

    /// The pad: buttons (bit 12 reset, 13 prewind held, 14 boost; the
    /// trick stick's 6-bit axes at bits 15 and 21) and sticks (6-bit axes
    /// at bits 0 and 6).
    pub const PAD_BUTTONS: u32 = 0x0;

    pub const PAD_STICKS: u32 = 0x4;

    /// Rider fields: the launch speed, the lean (slew 0: value, rate,
    /// target) and pitch (slew 5) controls, the turn input, the prewind
    /// direction (0 lean, else pitch; 3 and 4 tilt the animation), the
    /// animation controller.
    pub const LAUNCH_SPEED: u32 = 0x220;

    pub const SLEW0_VALUE: u32 = 0x2a4;

    pub const SLEW0_RATE: u32 = 0x2a8;

    pub const SLEW0_TARGET: u32 = 0x2ac;

    pub const SLEW5_VALUE: u32 = 0x2b0;

    pub const SLEW5_RATE: u32 = 0x2b4;

    pub const SLEW5_TARGET: u32 = 0x2b8;

    pub const TURN_INPUT: u32 = 0x2dc;

    pub const PREWIND_DIR: u32 = 0x328;

    pub const ANIM_CTRL: u32 = 0x784;

    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;

    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;

    pub const ANIM_CONTROLLER_PLAY_ANIM: u32 = 0x0031_28e8;

    pub const INPUT_QUANTIZE_STICK_DIRECTION: Callee = Callee {
        addr: 0x0011_58b8,
        f: ee::s00::f_001158b8,
    };

    pub const MATH_SIN_COS: u32 = 0x0031_be50;

    pub const RIDER_CHECK_RESET_PATH: Callee = Callee {
        addr: 0x0011_6120,
        f: ee::s00::f_00116120,
    };

    pub const RIDER_EMPTY_STANCE_HOOK: Callee = Callee {
        addr: 0x0011_6930,
        f: ee::s00::f_00116930,
    };

    pub const RIDER_GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::f_0011fe98,
    };

    pub const RIDER_HANDLE_BOOST_INPUT: Callee = Callee {
        addr: 0x0011_4130,
        f: ee::s00::f_00114130,
    };

    pub const RIDER_LAUNCH_INTO_AIR: Callee = Callee {
        addr: 0x0011_4298,
        f: ee::s00::f_00114298,
    };

    pub const RIDER_PREWIND_SUB_PLAY_DIRECTIONAL_ANIM: Callee = Callee {
        addr: 0x0012_ee30,
        f: ee::s00::f_0012ee30,
    };

    pub const RIDER_SET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe78,
        f: ee::s00::f_0011fe78,
    };

    pub const RIDER_SET_SLEW0_TARGET: Callee = Callee {
        addr: 0x0011_3e80,
        f: ee::s00::f_00113e80,
    };

    pub const RIDER_SET_SLEW3_AND4_TARGETS: Callee = Callee {
        addr: 0x0011_3f88,
        f: ee::s00::f_00113f88,
    };

    pub const RIDER_SET_SLEW5_TARGET: Callee = Callee {
        addr: 0x0011_3f38,
        f: ee::s00::f_00113f38,
    };

    pub const RIDER_SET_SUB_STATE: Callee = Callee {
        addr: 0x0011_fec8,
        f: ee::s00::f_0011fec8,
    };

    pub const RIDER_TRY_ENTER_FINISH_LINE: Callee = Callee {
        addr: 0x0011_6378,
        f: ee::s00::f_00116378,
    };

    pub const RIDER_TRY_START_RAILSLIDE: Callee = Callee {
        addr: 0x0010_6848,
        f: ee::s00::f_00106848,
    };

    pub const RIDER_TURN_AROUND_IF_BACKWARD: Callee = Callee {
        addr: 0x0011_4cc0,
        f: ee::s00::f_00114cc0,
    };

    pub const SSX_AUDIO_SYSTEM_PLAY_TAKEOFF_SOUND: u32 = 0x0029_4170;
}

pub(crate) mod ai_render_fields {
    use super::*;

    /// The AI manager: the riders whose light slot is checked (count,
    /// array), whether only one view is drawn, the human actors (count,
    /// array of two), the viewed actor, the computer actors (shown?, count,
    /// array), the other actors (shown?, count, array). Actors point at
    /// their rider at +0x18.
    pub const SHOW_COMPUTERS: u32 = 0x14;

    pub const RENDER_LIST: u32 = 0x28;

    pub const HUMANS: u32 = 0x40;

    pub const COMPUTERS: u32 = 0x48;

    pub const OTHERS: u32 = 0x5c;

    pub const VIEWED: u32 = 0x70;

    pub const RENDER_COUNT: u32 = 0x78;

    pub const HUMAN_COUNT: u32 = 0x7c;

    pub const COMPUTER_COUNT: u32 = 0x80;

    pub const OTHER_COUNT: u32 = 0x84;

    pub const SINGLE_VIEW: u32 = 0x88;

    pub const SHOW_OTHERS: u32 = 0x8c;

    pub const ACTOR_RIDER: u32 = 0x18;

    /// Rider fields: its owner (`RiderBase`, the effects), visible flag,
    /// light slot (-1: none yet).
    pub const RIDER_OWNER: u32 = 0x77c;

    pub const RIDER_VISIBLE: u32 = 0x884;

    pub const RIDER_LIGHT_SLOT: u32 = 0x888;

    /// The owner's effects.
    pub const WAKE_FX: u32 = 0x3b0;

    pub const SPARK_FX: u32 = 0x470;

    pub const TRAIL_FX: u32 = 0x520;

    pub const SPEED_STREAMER_FX: u32 = 0x610;

    pub const POWER_UP_TRAIL: u32 = 0x9c0;

    pub const AIR_STREAMER_FX: u32 = 0xad0;

    pub const ANGER_FX: u32 = 0xaf0;

    pub const PULSE_FX: u32 = 0xb00;

    pub const SPRAY_FX: u32 = 0xb40;

    pub const ATTACK_FX: u32 = 0xc70;

    pub const CHEAT_CODE: u32 = 0xd20;

    /// The graphics manager (gp-0x854) and its vtable's light-slot
    /// allocator; `g_App` (gp-0x848) -> game module -> camera control (its
    /// mode at +0x70).
    pub const GP_GFX: i32 = -0x854;

    pub const GFX_VTABLE: u32 = 0x10d8;

    pub const GFX_ALLOC_LIGHT_SLOT: u32 = 0x390;

    pub const GP_APP: i32 = -0x848;

    pub const APP_MODULE: u32 = 0x84;

    pub const MODULE_CONTROL: u32 = 0x34;

    pub const CONTROL_MODE: u32 = 0x70;

    pub const C_AIR_STREAMER_FX_RENDER: u32 = 0x002e_f950;

    pub const C_ANGER_FX_RENDER: u32 = 0x002d_5048;

    pub const C_BOARD_SPARK_FX_RENDER: u32 = 0x002d_b478;

    pub const C_BOARD_TRAIL_FX_RENDER_BATCH: u32 = 0x002e_a538;

    pub const C_BOARD_WAKE_FX_RENDER: u32 = 0x002d_dab8;

    pub const C_DYNAMIC_SPRAY_FX_RENDER: u32 = 0x002e_24d0;

    pub const C_PLAYER_PULSE_FX_RENDER: u32 = 0x002e_3af8;

    pub const C_SPEED_BOOST_STREAMER_FX_RENDER: u32 = 0x002e_7a10;

    pub const RIDER_BASE_RENDER_ATTACK_FX: u32 = 0x002f_1510;

    pub const RIDER_BASE_RENDER_CHEAT_CODE: u32 = 0x002f_6908;

    pub const RIDER_BASE_RENDER_POWER_UP_TRAIL: u32 = 0x002e_b198;

    pub const RIDER_PREPARE_LIGHTING: Callee = Callee {
        addr: 0x0012_25c0,
        f: ee::s00::f_001225c0,
    };

    pub const RIDER_RENDER_ATTACHED_FX: Callee = Callee {
        addr: 0x0012_2638,
        f: ee::s00::f_00122638,
    };

    pub const RIDER_RENDER_MODEL: Callee = Callee {
        addr: 0x0012_2448,
        f: ee::s00::f_00122448,
    };

    pub const RIDER_RENDER_SHADOW: Callee = Callee {
        addr: 0x0012_25f0,
        f: ee::s00::f_001225f0,
    };

    pub const RIDER_UPDATE_VISIBILITY: Callee = Callee {
        addr: 0x0012_2278,
        f: ee::s00::f_00122278,
    };
}

pub(crate) mod cruise {

    /// `sub+0`: the rider.
    pub const RIDER: u32 = 0x0;

    /// `pad+0`: the button word (bit 12 reset, 13 handplant, 14 cheat jump,
    /// 15 jump, 16/17 boost, 18/19 tweak; bits 20..25 the steer axis, 26..31
    /// the lean axis); `pad+4`: the stick word (bits 0..5 the crouch axis,
    /// 6..11 the press axis).
    pub const PAD_BUTTONS: u32 = 0x0;

    pub const PAD_STICKS: u32 = 0x4;

    /// Rider: velocity, forward axis (dotted for "moving backwards").
    pub const VEL: u32 = 0x1e0;

    pub const FORWARD: u32 = 0x3a0;

    /// Rider: the slew targets the animation choice writes (rate, value).
    pub const SLEW0_RATE: u32 = 0x200;

    pub const SLEW0_VALUE: u32 = 0x204;

    pub const SLEW1_RATE: u32 = 0x218;

    pub const SLEW1_VALUE: u32 = 0x21c;

    pub const SLEW2_RATE: u32 = 0x224;

    pub const SLEW2_VALUE: u32 = 0x228;

    /// Rider: the steer input (slewed), the carve input, the lean.
    pub const STEER: u32 = 0x1f0;

    pub const CARVE: u32 = 0x1fc;

    pub const TURN: u32 = 0x214;

    pub const CROUCH: u32 = 0x220;

    pub const SPIN_SPEED: u32 = 0x2dc;

    pub const BOOST_INPUT: u32 = 0x2fc;

    pub const GOOFY: u32 = 0x320;

    pub const SLOPE: u32 = 0x3b8;

    pub const SURFACE: u32 = 0x438;

    pub const ANIM: u32 = 0x784;
}

/// `bool RiderSpinSub_UpdateTrick(RiderSpinSub* this, int trick, int held)`
/// (0x1352a8): the grab-trick state machine of an airborne rider, once per
/// tick. `trick` is the requested trick index (-1 none), `held` whether the
/// trick button is held.
///
/// - Reads the rider's trick skill factor (the playback speed of the
///   trick layer) and the BAGT trick interface; `hi` = uber level >= 5.
/// - State 0: a new trick starts its wind-up animation on layer 2 (on top
///   of a playing kind-0x12 trick: blended), no trick stops the layer
///   (anim 0x11f, speed -1); once the wind-up has started (flag 0), 1.
/// - State 1: at the hold point (flag 1) the trick is scored
///   (`RiderMetrix_AddTrick`, `Rider_AddBoost`), the layer is stopped and
///   the "held" animation, if any, starts on layer 1; 2.
/// - State 2: a different trick completes this one and fades layer 1
///   (5); holding fades both layers, plays the hold pose (the uber variant
///   when the uber timer runs and the flag allows), the grab sound; 3.
/// - State 3: still held at the hold point scores again and plays the
///   loop pose (kind 0x13: fields 8/A, else the uber value/anim2 tables);
///   4. Otherwise once the layer reached a hold, loop or end point the
///   trick is scored and completed; 5.
/// - State 4: released or changed: completed, layer 1 faded; 5.
/// - State 5: when layer 2 ends (anim kind 2 or flag 3), a held new trick
///   on a kind-0x14 layer chains (3), else back to 0 with no trick.
///
/// Returns whether layer 2 plays a trick animation (kind 0x12..0x14).
#[allow(unused_labels)]
pub fn rider_spin_sub_update_trick(c: &mut Cpu) {
    use update_trick::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0x90);
        'b_13541c: {
            'b_135418: {
                c.cycles += 28;
                frame.save(c, S1, 0x60);
                frame.save(c, S2, 0x50);
                c.g[S1][0] = c.g[A0][0];
                frame.save(c, S4, 0x30);
                frame.save(c, S5, 0x20);
                c.g[S4][0] = c.g[A1][0];
                frame.save(c, S6, 0x10);
                frame.save_fpr(c, 21, 0x88);
                c.g[S6][0] = c.g[A2][0];
                frame.save(c, S0, 0x70);
                frame.save(c, S3, 0x40);
                frame.save_ra(c, 0);
                frame.save_fpr(c, 20, 0x80);
                c.g[A0][0] = c.lw(at(c, S1, RIDER));
                call_ptr(c, RIDER_GET_TRICK_SKILL_FACTOR, 0x001352e8)?;
                c.mov_s(21, 0);
                BE_LIBRARY_GET_INSTANCE.call(c, 0x001352f0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x6;
                BE_LIBRARY_GET_INTERFACE.call(c, 0x001352fc)?;
                c.g[A0][0] = c.lw(at(c, S1, RIDER));
                c.g[S2][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(at(c, S1, STATE));
                c.g[V0][0] = c.lw(at(c, A0, UBER_LEVEL));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 5i64) as u64;
                let t = c.g[V1][0] != 0u64;
                c.g[S5][0] = c.g[V0][0] ^ 0x1;
                if t {
                    break 'b_135418;
                } else {
                    'b_135414: {
                        'b_135360: {
                            c.cycles += 3;
                            c.g[V0][0] = 0xffffffffffffffff;
                            let t = c.g[S4][0] != c.g[V0][0];
                            c.g[V0][0] = c.lw(at(c, S1, TRICK));
                            if t {
                                break 'b_135360;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == c.g[S4][0];
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_13541c;
                                } else {
                                    c.cycles += 13;
                                    c.g[A0][0] = c.lw(at(c, A0, ANIM_CTRL));
                                    c.g[A1][0] = 0x2;
                                    call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x00135338)?;
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 12);
                                    c.g[A1][0] = 0x11f;
                                    c.g[A2][0] = 0u64;
                                    c.swc1(12, at(c, V0, SEQ_SPEED));
                                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                    c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                    call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x00135358)?;
                                    c.sw(at(c, S1, TRICK), c.g[S4][0]);
                                    break 'b_135414;
                                }
                            }
                        }
                        'b_1353f0: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == c.g[S4][0];
                            c.g[A1][0] = 0x2;
                            if t {
                                break 'b_1353f0;
                            } else {
                                'b_1353dc: {
                                    'b_1353b0: {
                                        c.cycles += 6;
                                        c.sw(at(c, S1, TRICK), c.g[S4][0]);
                                        c.g[A0][0] = c.lw(at(c, A0, ANIM_CTRL));
                                        call_ptr(
                                            c,
                                            ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY,
                                            0x00135374,
                                        )?;
                                        c.g[V1][0] = 0x12;
                                        let t = c.g[V0][0] != c.g[V1][0];
                                        c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                        if t {
                                            break 'b_1353b0;
                                        } else {
                                            c.cycles += 12;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                            c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                            BEBAGT_INT_GET_TRICK_FIELD0.call(c, 0x00135390)?;
                                            c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[AT][0] = 0xffffffffbf800000;
                                            c.mtc1(1, 12);
                                            c.g[A2][0] = 0x1;
                                            call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x001353a8)?;
                                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                            break 'b_1353dc;
                                        }
                                    }
                                    c.cycles += 11;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                    c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                    BEBAGT_INT_GET_TRICK_FIELD0.call(c, 0x001353c0)?;
                                    c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 12);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x001353d8)?;
                                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                    break 'b_1353dc;
                                }
                                c.cycles += 5;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x001353e8)?;
                                c.swc1(21, at(c, V0, SEQ_SPEED));
                                break 'b_135414;
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = c.lw(at(c, A0, ANIM_CTRL));
                        c.g[A1][0] = 0x2;
                        call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x001353fc)?;
                        c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                        c.g[A1][0] = 0u64;
                        FLAGS64_TEST.call(c, 0x00135408)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_135414;
                        } else {
                            c.cycles += 1;
                            c.sw(at(c, S1, STATE), c.g[V0][0]);
                            break 'b_135414;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(at(c, S1, STATE));
                    break 'b_135418;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_13541c;
        }
        'b_1354f8: {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V1][0] = c.lw(at(c, S1, STATE));
                break 'b_1354f8;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                c.g[A1][0] = 0x2;
                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x00135434)?;
                c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                c.g[A1][0] = 0x1;
                FLAGS64_TEST.call(c, 0x00135440)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw(at(c, S1, STATE));
                    break 'b_1354f8;
                } else {
                    'b_1354f0: {
                        c.cycles += 25;
                        c.g[S0][0] = c.lw(at(c, S1, RIDER));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = c.lw(at(c, S1, TRICK));
                        c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                        BEBAGT_INT_GET_TRICK_FIELD4.call(c, 0x0013545c)?;
                        c.g[A0][0] = c.lw(at(c, S0, METRIX));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, RIDER_METRIX_ADD_TRICK, 0x00135468)?;
                        c.g[A0][0] = c.lw(at(c, S1, RIDER));
                        c.mov_s(12, 0);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, RIDER_ADD_BOOST, 0x00135478)?;
                        c.g[V0][0] = c.lw(at(c, S1, RIDER));
                        c.g[A1][0] = 0x2;
                        c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                        call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x00135488)?;
                        c.sw(at(c, V0, SEQ_SPEED), 0u64);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[V0][0] = c.lw(at(c, S1, RIDER));
                        c.g[A2][0] = c.lw(at(c, S1, TRICK));
                        c.g[A1][0] = c.lw(at(c, V0, AI_INDEX));
                        BEBAGT_INT_GET_TRICK_FIELD2.call(c, 0x001354a0)?;
                        c.g[V1][0] = 0x1b6;
                        let t = c.g[V0][0] == c.g[V1][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_1354f0;
                        } else {
                            c.cycles += 17;
                            c.g[S0][0] = c.lw(at(c, S1, RIDER));
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                            c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                            BEBAGT_INT_GET_TRICK_FIELD2.call(c, 0x001354c0)?;
                            c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                            c.g[A1][0] = c.g[V0][0];
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 12);
                            c.g[A2][0] = 0u64;
                            call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x001354d8)?;
                            c.g[V1][0] = c.lw(at(c, S1, RIDER));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lw(at(c, V1, ANIM_CTRL));
                            call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x001354e8)?;
                            c.swc1(21, at(c, V0, SEQ_SPEED));
                            c.g[V0][0] = 0x2;
                            break 'b_1354f0;
                        }
                    }
                    c.cycles += 2;
                    c.sw(at(c, S1, STATE), c.g[V0][0]);
                    c.g[V1][0] = c.lw(at(c, S1, STATE));
                    break 'b_1354f8;
                }
            }
        }
        'b_135674: {
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V1][0] = c.lw(at(c, S1, STATE));
                break 'b_135674;
            } else {
                'b_135670: {
                    'b_13566c: {
                        'b_135568: {
                            c.cycles += 3;
                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                            let t = c.g[S4][0] == c.g[A2][0];
                            c.g[A0][0] = c.g[S2][0];
                            if t {
                                break 'b_135568;
                            } else {
                                c.cycles += 22;
                                c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                BEBAGT_INT_GET_TRICK_FIELD4.call(c, 0x0013551c)?;
                                c.g[A0][0] = c.lw(at(c, S0, METRIX));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, RIDER_METRIX_COMPLETE_TRICK, 0x00135528)?;
                                c.g[A0][0] = c.lw(at(c, S1, RIDER));
                                c.mov_s(12, 0);
                                c.g[A1][0] = 0x1;
                                call_ptr(c, RIDER_ADD_BOOST, 0x00135538)?;
                                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                c.g[A1][0] = 0x1;
                                c.lwc1(12, gp(c, FADE_ON_RELEASE));
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_FADE_OUT_LAYER, 0x0013554c)?;
                                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x0013555c)?;
                                c.swc1(21, at(c, V0, SEQ_SPEED));
                                c.g[V0][0] = 0x5;
                                break 'b_13566c;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[S6][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_135670;
                        } else {
                            'b_135640: {
                                'b_135614: {
                                    'b_1355e4: {
                                        c.cycles += 17;
                                        c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                        c.lwc1(20, gp(c, FADE_ON_HOLD));
                                        c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                        c.mov_s(12, 20);
                                        call_ptr(c, ANIM_CONTROLLER_FADE_OUT_LAYER, 0x00135584)?;
                                        c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                        c.mov_s(12, 20);
                                        c.g[A1][0] = 0x2;
                                        c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                        call_ptr(c, ANIM_CONTROLLER_FADE_OUT_LAYER, 0x00135598)?;
                                        c.g[V1][0] = c.lw(at(c, S1, RIDER));
                                        c.mtc1(0, 1);
                                        c.lwc1(0, at(c, V1, UBER_TIMER));
                                        c.c_eq_s(0, 1);
                                        if c.fcc() {
                                            c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                            break 'b_1355e4;
                                        } else {
                                            'b_1355e0: {
                                                c.cycles += 4;
                                                c.g[V0][0] = c.lw(at(c, V1, UBER_FLAGS));
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = c.g[S2][0];
                                                if t {
                                                    break 'b_1355e0;
                                                } else {
                                                    c.cycles += 7;
                                                    c.g[A1][0] = c.lw(at(c, V1, AI_INDEX));
                                                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                                    c.g[A3][0] = c.g[S5][0];
                                                    BEBAGT_INT_GET_TRICK_ANIM
                                                        .call(c, 0x001355d4)?;
                                                    c.g[V1][0] = 0x1b6;
                                                    if c.g[V0][0] != c.g[V1][0] {
                                                        c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                                        break 'b_135614;
                                                    } else {
                                                        break 'b_1355e0;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                            break 'b_1355e4;
                                        }
                                    }
                                    c.cycles += 12;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                    c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                    BEBAGT_INT_GET_TRICK_FIELD6.call(c, 0x001355f4)?;
                                    c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 12);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x0013560c)?;

                                    break 'b_135640;
                                }
                                c.cycles += 11;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                c.g[A3][0] = c.g[S5][0];
                                c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                BEBAGT_INT_GET_TRICK_ANIM.call(c, 0x00135628)?;
                                c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[AT][0] = 0xffffffffbf800000;
                                c.mtc1(1, 12);
                                c.g[A2][0] = 0u64;
                                call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x00135640)?;
                                break 'b_135640;
                            }
                            c.cycles += 11;
                            call_ptr(c, SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x00135648)?;
                            c.g[A1][0] = c.lw(at(c, S1, RIDER));
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, SSX_AUDIO_SYSTEM_PLAY_GRAB_TRICK_SOUND, 0x00135654)?;
                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                            c.g[A1][0] = 0x2;
                            c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                            call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x00135664)?;
                            c.swc1(21, at(c, V0, SEQ_SPEED));
                            c.g[V0][0] = 0x3;
                            break 'b_13566c;
                        }
                    }
                    c.cycles += 1;
                    c.sw(at(c, S1, STATE), c.g[V0][0]);
                    break 'b_135670;
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw(at(c, S1, STATE));
                break 'b_135674;
            }
        }
        'b_135930: {
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V1][0] = c.lw(at(c, S1, STATE));
                break 'b_135930;
            } else {
                'b_135928: {
                    'b_1357e0: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, TRICK));
                        let t = c.g[S4][0] != c.g[V0][0];
                        c.g[V0][0] = c.lw(at(c, S1, RIDER));
                        if t {
                            break 'b_1357e0;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S6][0] == 0u64;

                            if t {
                                break 'b_1357e0;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x001356a0)?;
                                c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                                c.g[A1][0] = 0x1;
                                FLAGS64_TEST.call(c, 0x001356ac)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                if t {
                                    break 'b_1357e0;
                                } else {
                                    'b_1357d8: {
                                        'b_1357b0: {
                                            'b_135744: {
                                                c.cycles += 11;
                                                c.g[A1][0] = 0x2;
                                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                                call_ptr(
                                                    c,
                                                    ANIM_CONTROLLER_GET_LAYER_SEQUENCE,
                                                    0x001356c0,
                                                )?;
                                                c.sw(at(c, V0, SEQ_SPEED), 0u64);
                                                c.g[A1][0] = 0x2;
                                                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                                call_ptr(
                                                    c,
                                                    ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY,
                                                    0x001356d4,
                                                )?;
                                                c.g[V1][0] = 0x13;
                                                let t = c.g[V0][0] != c.g[V1][0];
                                                c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                                if t {
                                                    break 'b_135744;
                                                } else {
                                                    c.cycles += 19;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                                    c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                                    BEBAGT_INT_GET_TRICK_FIELD_A
                                                        .call(c, 0x001356f0)?;
                                                    c.g[A0][0] = c.lw(at(c, S0, METRIX));
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(
                                                        c,
                                                        RIDER_METRIX_ADD_TRICK,
                                                        0x001356fc,
                                                    )?;
                                                    c.g[A0][0] = c.lw(at(c, S1, RIDER));
                                                    c.mov_s(12, 0);
                                                    c.g[A1][0] = 0x1;
                                                    call_ptr(c, RIDER_ADD_BOOST, 0x0013570c)?;
                                                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                                    c.g[A0][0] = c.g[S2][0];
                                                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                                    c.g[A1][0] = c.lw(at(c, V0, AI_INDEX));
                                                    BEBAGT_INT_GET_TRICK_FIELD8
                                                        .call(c, 0x00135720)?;
                                                    c.g[V1][0] = 0x1b6;
                                                    let t = c.g[V0][0] == c.g[V1][0];
                                                    c.g[A0][0] = c.g[S2][0];
                                                    if t {
                                                        break 'b_1357d8;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                                        c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                                        c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                                        BEBAGT_INT_GET_TRICK_FIELD8
                                                            .call(c, 0x0013573c)?;
                                                        c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                                        break 'b_1357b0;
                                                    }
                                                }
                                            }
                                            c.cycles += 21;
                                            c.g[A3][0] = c.g[S5][0];
                                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                            BEBAGT_INT_GET_TRICK_VALUE.call(c, 0x00135758)?;
                                            c.g[A0][0] = c.lw(at(c, S0, METRIX));
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, RIDER_METRIX_ADD_TRICK, 0x00135764)?;
                                            c.g[A0][0] = c.lw(at(c, S1, RIDER));
                                            c.mov_s(12, 0);
                                            c.g[A1][0] = 0x1;
                                            call_ptr(c, RIDER_ADD_BOOST, 0x00135774)?;
                                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                            c.g[A3][0] = c.g[S5][0];
                                            c.g[A1][0] = c.lw(at(c, V0, AI_INDEX));
                                            BEBAGT_INT_GET_TRICK_ANIM2.call(c, 0x0013578c)?;
                                            c.g[V1][0] = 0x1b6;
                                            let t = c.g[V0][0] == c.g[V1][0];
                                            c.g[A3][0] = c.g[S5][0];
                                            if t {
                                                break 'b_1357d8;
                                            } else {
                                                c.cycles += 6;
                                                c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                                c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                                BEBAGT_INT_GET_TRICK_ANIM2.call(c, 0x001357ac)?;
                                                c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                                break 'b_1357b0;
                                            }
                                        }
                                        c.cycles += 10;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[AT][0] = 0xffffffffbf800000;
                                        c.mtc1(1, 12);
                                        c.g[A2][0] = 0u64;
                                        call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x001357c4)?;
                                        c.g[V1][0] = c.lw(at(c, S1, RIDER));
                                        c.g[A1][0] = 0x1;
                                        c.g[A0][0] = c.lw(at(c, V1, ANIM_CTRL));
                                        call_ptr(
                                            c,
                                            ANIM_CONTROLLER_GET_LAYER_SEQUENCE,
                                            0x001357d4,
                                        )?;
                                        c.swc1(21, at(c, V0, SEQ_SPEED));
                                        break 'b_1357d8;
                                    }
                                    c.cycles += 2;
                                    c.g[V0][0] = 0x4;
                                    break 'b_135928;
                                }
                            }
                        }
                    }
                    'b_135844: {
                        c.cycles += 8;
                        c.g[A1][0] = 0x2;
                        c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                        call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x001357ec)?;
                        c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                        c.g[A1][0] = 0x1;
                        FLAGS64_TEST.call(c, 0x001357f8)?;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw(at(c, S1, RIDER));
                        if t {
                            break 'b_135844;
                        } else {
                            c.cycles += 8;
                            c.g[A1][0] = 0x2;
                            c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                            call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x0013580c)?;
                            c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                            c.g[A1][0] = 0x3;
                            FLAGS64_TEST.call(c, 0x00135818)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                            if t {
                                break 'b_135844;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x0013582c)?;
                                c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                                c.g[A1][0] = 0x2;
                                FLAGS64_TEST.call(c, 0x00135838)?;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V1][0] = c.lw(at(c, S1, STATE));
                                    break 'b_135930;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                    break 'b_135844;
                                }
                            }
                        }
                    }
                    'b_1358c0: {
                        c.cycles += 6;
                        c.g[A1][0] = 0x2;
                        c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                        call_ptr(c, ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY, 0x00135850)?;
                        c.g[V1][0] = 0x13;
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[S0][0] = c.lw(at(c, S1, RIDER));
                        if t {
                            break 'b_1358c0;
                        } else {
                            c.cycles += 25;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                            c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                            BEBAGT_INT_GET_TRICK_FIELD_A.call(c, 0x0013586c)?;
                            c.g[A0][0] = c.lw(at(c, S0, METRIX));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, RIDER_METRIX_ADD_TRICK, 0x00135878)?;
                            c.g[A0][0] = c.lw(at(c, S1, RIDER));
                            c.mov_s(12, 0);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, RIDER_ADD_BOOST, 0x00135888)?;
                            c.g[S0][0] = c.lw(at(c, S1, RIDER));
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                            c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                            BEBAGT_INT_GET_TRICK_FIELD_A.call(c, 0x0013589c)?;
                            c.g[A0][0] = c.lw(at(c, S0, METRIX));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, RIDER_METRIX_COMPLETE_TRICK, 0x001358a8)?;
                            c.g[A0][0] = c.lw(at(c, S1, RIDER));
                            c.mov_s(12, 0);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, RIDER_ADD_BOOST, 0x001358b8)?;
                            c.g[V0][0] = 0x5;
                            break 'b_135928;
                        }
                    }
                    c.cycles += 26;
                    c.g[A3][0] = c.g[S5][0];
                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                    BEBAGT_INT_GET_TRICK_VALUE.call(c, 0x001358d4)?;
                    c.g[A0][0] = c.lw(at(c, S0, METRIX));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, RIDER_METRIX_ADD_TRICK, 0x001358e0)?;
                    c.g[A0][0] = c.lw(at(c, S1, RIDER));
                    c.mov_s(12, 0);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, RIDER_ADD_BOOST, 0x001358f0)?;
                    c.g[S0][0] = c.lw(at(c, S1, RIDER));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                    BEBAGT_INT_GET_TRICK_VALUE.call(c, 0x00135908)?;
                    c.g[A0][0] = c.lw(at(c, S0, METRIX));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, RIDER_METRIX_COMPLETE_TRICK, 0x00135914)?;
                    c.g[A0][0] = c.lw(at(c, S1, RIDER));
                    c.mov_s(12, 0);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, RIDER_ADD_BOOST, 0x00135924)?;
                    c.g[V0][0] = 0x5;
                    break 'b_135928;
                }
                c.cycles += 2;
                c.sw(at(c, S1, STATE), c.g[V0][0]);
                c.g[V1][0] = c.lw(at(c, S1, STATE));
                break 'b_135930;
            }
        }
        'b_135a00: {
            c.cycles += 3;
            c.g[V0][0] = 0x4;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                break 'b_135a00;
            } else {
                'b_135950: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, S1, TRICK));
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                    if t {
                        break 'b_135950;
                    } else {
                        c.cycles += 2;
                        let t = c.g[S6][0] != 0u64;

                        if t {
                            break 'b_135a00;
                        } else {
                            break 'b_135950;
                        }
                    }
                }
                'b_1359d0: {
                    'b_13599c: {
                        c.cycles += 6;
                        c.g[A1][0] = 0x2;
                        c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                        call_ptr(c, ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY, 0x0013595c)?;
                        c.g[V1][0] = 0x13;
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[S0][0] = c.lw(at(c, S1, RIDER));
                        if t {
                            break 'b_13599c;
                        } else {
                            c.cycles += 13;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = c.lw(at(c, S1, TRICK));
                            c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                            BEBAGT_INT_GET_TRICK_FIELD_A.call(c, 0x00135978)?;
                            c.g[A0][0] = c.lw(at(c, S0, METRIX));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, RIDER_METRIX_COMPLETE_TRICK, 0x00135984)?;
                            c.g[A0][0] = c.lw(at(c, S1, RIDER));
                            c.mov_s(12, 0);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, RIDER_ADD_BOOST, 0x00135994)?;
                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                            break 'b_1359d0;
                        }
                    }
                    c.cycles += 13;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = c.lw(at(c, S1, TRICK));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                    BEBAGT_INT_GET_TRICK_VALUE.call(c, 0x001359b0)?;
                    c.g[A0][0] = c.lw(at(c, S0, METRIX));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, RIDER_METRIX_COMPLETE_TRICK, 0x001359bc)?;
                    c.g[A0][0] = c.lw(at(c, S1, RIDER));
                    c.mov_s(12, 0);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, RIDER_ADD_BOOST, 0x001359cc)?;
                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                    break 'b_1359d0;
                }
                c.cycles += 12;
                c.g[A1][0] = 0x1;
                c.lwc1(12, gp(c, FADE_ON_END));
                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                call_ptr(c, ANIM_CONTROLLER_FADE_OUT_LAYER, 0x001359e0)?;
                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                c.g[A1][0] = 0x2;
                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x001359f0)?;
                c.swc1(21, at(c, V0, SEQ_SPEED));
                c.g[V0][0] = 0x5;
                c.sw(at(c, S1, STATE), c.g[V0][0]);
                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                break 'b_135a00;
            }
        }
        'b_135afc: {
            c.cycles += 8;
            c.g[A1][0] = 0x2;
            c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
            call_ptr(c, ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY, 0x00135a0c)?;
            c.g[S3][0] = c.g[V0][0];
            c.g[V1][0] = c.lw(at(c, S1, STATE));
            c.g[V0][0] = 0x5;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x12));
            if t {
                break 'b_135afc;
            } else {
                'b_135af8: {
                    'b_135a50: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x2;
                        let t = c.g[S3][0] == c.g[V0][0];
                        c.g[V0][0] = 0x14;
                        if t {
                            break 'b_135a50;
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                            c.g[A1][0] = 0x2;
                            c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                            call_ptr(c, ANIM_CONTROLLER_GET_LAYER_SEQUENCE, 0x00135a3c)?;
                            c.g[A0][0] = sx(at(c, V0, SEQ_FLAGS));
                            c.g[A1][0] = 0x3;
                            FLAGS64_TEST.call(c, 0x00135a48)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x14;
                            if t {
                                break 'b_135af8;
                            } else {
                                break 'b_135a50;
                            }
                        }
                    }
                    'b_135af4: {
                        'b_135af0: {
                            c.cycles += 2;
                            let t = c.g[S3][0] != c.g[V0][0];
                            c.g[V0][0] = 0xffffffffffffffff;
                            if t {
                                break 'b_135af0;
                            } else {
                                c.cycles += 2;
                                if c.g[S6][0] == 0u64 {
                                    c.sw(at(c, S1, STATE), 0u64);
                                    break 'b_135af4;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S4][0] == c.g[V0][0];
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'b_135af0;
                                    } else {
                                        'b_135aec: {
                                            c.cycles += 8;
                                            c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                            c.g[A2][0] = c.g[S4][0];
                                            c.g[A3][0] = c.g[S5][0];
                                            c.g[A1][0] = c.lw(at(c, V0, AI_INDEX));
                                            BEBAGT_INT_GET_TRICK_ANIM.call(c, 0x00135a7c)?;
                                            c.g[V1][0] = 0x1b6;
                                            let t = c.g[V0][0] == c.g[V1][0];
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_135aec;
                                            } else {
                                                c.cycles += 25;
                                                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                                                c.sw(at(c, S1, TRICK), c.g[S4][0]);
                                                c.lwc1(12, gp(c, FADE_ON_CHAIN));
                                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                                call_ptr(
                                                    c,
                                                    ANIM_CONTROLLER_FADE_OUT_LAYER,
                                                    0x00135a9c,
                                                )?;
                                                c.g[S0][0] = c.lw(at(c, S1, RIDER));
                                                c.g[A3][0] = c.g[S5][0];
                                                c.g[A2][0] = c.lw(at(c, S1, TRICK));
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[A1][0] = c.lw(at(c, S0, AI_INDEX));
                                                BEBAGT_INT_GET_TRICK_ANIM.call(c, 0x00135ab4)?;
                                                c.g[A0][0] = c.lw(at(c, S0, ANIM_CTRL));
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[AT][0] = 0xffffffffbf800000;
                                                c.mtc1(1, 12);
                                                c.g[A2][0] = 0x1;
                                                call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x00135acc)?;
                                                c.g[V1][0] = c.lw(at(c, S1, RIDER));
                                                c.g[A1][0] = 0x2;
                                                c.g[A0][0] = c.lw(at(c, V1, ANIM_CTRL));
                                                call_ptr(
                                                    c,
                                                    ANIM_CONTROLLER_GET_LAYER_SEQUENCE,
                                                    0x00135adc,
                                                )?;
                                                c.swc1(21, at(c, V0, SEQ_SPEED));
                                                c.g[V0][0] = 0x3;
                                                c.sw(at(c, S1, STATE), c.g[V0][0]);
                                                break 'b_135af8;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        break 'b_135af0;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(at(c, S1, STATE), 0u64);
                        break 'b_135af4;
                    }
                    c.cycles += 1;
                    c.sw(at(c, S1, TRICK), c.g[V0][0]);
                    break 'b_135af8;
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x12));
                break 'b_135afc;
            }
        }
        c.cycles += 13;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x88);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderPrewindSub_HandleInput(RiderPrewindSub* this, Pad* pad)`
/// (0x12e9b8): the input of a rider crouching for a jump (prewind), once
/// per tick.
///
/// - Crossing the finish line, a path reset (pad bit 12) or a rail jump
///   end the function.
/// - Released (bit 13 clear): the stick direction is quantized into the
///   lean / pitch slews; prewind directions 3 and 4 reset the animation
///   tilt (`Math_SinCos` of -0); the release animation (table 0x43d840);
///   direction 0; unless already airborne (main state 1) the rider
///   launches (`Rider_LaunchIntoAir` at `+0x220`, main state 1, take-off
///   sound); slews zeroed, sub-state 5.
/// - Held: boost input (bit 14), slews 3/4 to (1, 0); the stick axes
///   (scaled, clamped to ±0.5) lean or pitch. On a kind-10 layer-2
///   animation or anim id 0x15 the slews are parked (rate 1/12); a centred
///   rider turning around plays anim 0x15; otherwise the trick stick's
///   axes (±0.2 dead zone, to -1/0/1) set the slew targets with rates by
///   the distance (5 per second), and the directional animation (table
///   0x43d788) plays.
#[allow(unused_labels)]
pub fn rider_prewind_sub_handle_input(c: &mut Cpu) {
    use prewind_input::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0x80);
        'b_12ee0c: {
            c.cycles += 14;
            frame.save(c, S0, 0x50);
            frame.save(c, S2, 0x30);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S1, 0x40);
            c.g[S2][0] = c.g[A0][0];
            frame.save_ra(c, 0x20);
            frame.save_fpr(c, 22, 0x70);
            frame.save_fpr(c, 21, 0x68);
            frame.save_fpr(c, 20, 0x60);
            c.g[A0][0] = c.lw(at(c, S2, RIDER));
            RIDER_TRY_ENTER_FINISH_LINE.call(c, 0x0012e9e8)?;
            if c.g[V0][0] != 0u64 {
                frame.restore(c, S0, 0x50);
                break 'b_12ee0c;
            } else {
                c.cycles += 8;
                c.g[A1][0] = c.lw(at(c, S0, PAD_BUTTONS));
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.lw(at(c, S2, RIDER));
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 12) as u32);
                c.g[A1][0] = c.g[A1][0] & 0x1;
                RIDER_CHECK_RESET_PATH.call(c, 0x0012ea08)?;
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x50);
                    break 'b_12ee0c;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(at(c, S2, RIDER));
                    RIDER_TRY_START_RAILSLIDE.call(c, 0x0012ea18)?;
                    if c.g[V0][0] != 0u64 {
                        frame.restore(c, S0, 0x50);
                        break 'b_12ee0c;
                    } else {
                        'b_12eb50: {
                            c.cycles += 4;
                            c.g[A1][0] = c.lw(at(c, S0, PAD_BUTTONS));
                            c.g[V0][0] = c.g[A1][0] & 0x2000;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 14) as u32);
                            if t {
                                break 'b_12eb50;
                            } else {
                                'b_12eac4: {
                                    c.cycles += 12;
                                    c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                    c.lwc1(12, gp(c, -0x7790));
                                    c.lwc1(13, gp(c, -0x778c));
                                    c.g[A1][0] = sx(at(c, A0, SLEW5_VALUE));
                                    c.g[A2][0] = sx(at(c, A0, SLEW0_VALUE));
                                    INPUT_QUANTIZE_STICK_DIRECTION.call(c, 0x0012ea48)?;
                                    c.g[V1][0] = c.lw(at(c, S2, RIDER));
                                    c.g[V0][0] = c.lw(at(c, V1, PREWIND_DIR));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x500000;
                                    if t {
                                        break 'b_12eac4;
                                    } else {
                                        c.cycles += 25;
                                        c.mtc1(0, 12);
                                        c.g[S1][0] = c.lw(at(c, V1, ANIM_CTRL));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xed0));
                                        c.lq(3, r32(c, V0));
                                        c.neg_s(12, 12);
                                        c.g[A0][0] = sx(at(c, SP, 0x10));
                                        c.g[A1][0] = sx(at(c, SP, 0x14));
                                        c.sq(3, at(c, S1, 0x30));
                                        c.g[S0][0] = 0x500000;
                                        call_ptr(c, MATH_SIN_COS, 0x0012ea88)?;
                                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xea0));
                                        c.lwc1(0, frame.at(c, 0x10));
                                        c.lwc1(1, at(c, V0, 0x8));
                                        c.lwc1(2, (c.g[S0][0] as u32).wrapping_sub(0xea0));
                                        c.lwc1(4, at(c, V0, 0x4));
                                        c.mul_s(1, 0, 1);
                                        c.mul_s(2, 0, 2);
                                        c.lwc1(3, frame.at(c, 0x14));
                                        c.mul_s(0, 0, 4);
                                        c.swc1(3, frame.at(c, 0xc));
                                        c.swc1(1, frame.at(c, 0x8));
                                        c.swc1(2, frame.at(c, 0));
                                        c.swc1(0, frame.at(c, 0x4));
                                        c.lq(2, frame.at(c, 0));
                                        c.sq(2, at(c, S1, 0x40));
                                        break 'b_12eac4;
                                    }
                                }
                                'b_12eb1c: {
                                    c.cycles += 13;
                                    c.g[A1][0] = 0x440000;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x27c0));
                                    RIDER_PREWIND_SUB_PLAY_DIRECTIONAL_ANIM.call(c, 0x0012ead4)?;
                                    c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                    c.sw(at(c, V0, PREWIND_DIR), 0u64);
                                    c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                    RIDER_EMPTY_STANCE_HOOK.call(c, 0x0012eae4)?;
                                    c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                    RIDER_GET_MAIN_STATE.call(c, 0x0012eaec)?;
                                    c.g[V1][0] = 0x1;
                                    let t = c.g[V0][0] == c.g[V1][0];

                                    if t {
                                        break 'b_12eb1c;
                                    } else {
                                        c.cycles += 9;
                                        c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                        c.lwc1(12, at(c, A0, LAUNCH_SPEED));
                                        RIDER_LAUNCH_INTO_AIR.call(c, 0x0012eb04)?;
                                        c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                        c.g[A1][0] = 0x1;
                                        RIDER_SET_MAIN_STATE.call(c, 0x0012eb10)?;
                                        c.g[A0][0] = c.lw(gp(c, 0x410));
                                        c.g[A1][0] = c.lw(at(c, S2, RIDER));
                                        call_ptr(
                                            c,
                                            SSX_AUDIO_SYSTEM_PLAY_TAKEOFF_SOUND,
                                            0x0012eb1c,
                                        )?;
                                        break 'b_12eb1c;
                                    }
                                }
                                c.cycles += 13;
                                c.mtc1(0, 20);
                                c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                c.mov_s(12, 20);
                                RIDER_SET_SLEW0_TARGET.call(c, 0x0012eb2c)?;
                                c.mov_s(12, 20);
                                c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                c.mov_s(13, 12);
                                RIDER_SET_SLEW3_AND4_TARGETS.call(c, 0x0012eb3c)?;
                                c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                c.g[A1][0] = 0x5;
                                RIDER_SET_SUB_STATE.call(c, 0x0012eb48)?;
                                frame.restore(c, S0, 0x50);
                                break 'b_12ee0c;
                            }
                        }
                        'b_12ebb8: {
                            c.cycles += 25;
                            c.g[A0][0] = c.lw(at(c, S2, RIDER));
                            c.g[A1][0] = c.g[A1][0] & 0x1;
                            c.g[A2][0] = 0u64;
                            RIDER_HANDLE_BOOST_INPUT.call(c, 0x0012eb60)?;
                            c.mtc1(0, 20);
                            c.g[A0][0] = c.lw(at(c, S2, RIDER));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 12);
                            c.mov_s(13, 20);
                            RIDER_SET_SLEW3_AND4_TARGETS.call(c, 0x0012eb78)?;
                            c.g[V1][0] = c.lw(at(c, S0, PAD_STICKS));
                            c.lwc1(3, gp(c, -0x7788));
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 26);
                            c.g[AT][0] = 0xffffffffbf000000;
                            c.mtc1(1, 2);
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 1);
                            c.mtc1(2, 0);
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 3);
                            c.c_le_s(2, 0);
                            let t = !c.fcc();
                            c.mov_s(22, 2);
                            if t {
                                break 'b_12ebb8;
                            } else {
                                c.cycles += 1;
                                c.min_s(22, 0, 1);
                                break 'b_12ebb8;
                            }
                        }
                        'b_12ebe0: {
                            c.cycles += 9;
                            c.g[V0][0] = sx((c.g[V1][0] as u32) << 20);
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                            c.mtc1(2, 0);
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 3);
                            c.c_le_s(2, 0);
                            let t = !c.fcc();
                            c.mov_s(21, 2);
                            if t {
                                break 'b_12ebe0;
                            } else {
                                c.cycles += 1;
                                c.min_s(21, 0, 1);
                                break 'b_12ebe0;
                            }
                        }
                        'b_12ee08: {
                            'b_12ec44: {
                                c.cycles += 7;
                                c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY, 0x0012ebf0)?;
                                c.g[V1][0] = 0xa;
                                if c.g[V0][0] != c.g[V1][0] {
                                    c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                    break 'b_12ec44;
                                } else {
                                    'b_12ec28: {
                                        'b_12ec1c: {
                                            c.cycles += 4;
                                            c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                            c.g[V0][0] = c.lw(at(c, A0, PREWIND_DIR));
                                            let t = c.g[V0][0] != 0u64;

                                            if t {
                                                break 'b_12ec1c;
                                            } else {
                                                c.cycles += 4;
                                                c.mov_s(12, 22);
                                                RIDER_SET_SLEW0_TARGET.call(c, 0x0012ec14)?;
                                                c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                                break 'b_12ec28;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.mov_s(12, 21);
                                        RIDER_SET_SLEW5_TARGET.call(c, 0x0012ec24)?;
                                        c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                        break 'b_12ec28;
                                    }
                                    c.cycles += 7;
                                    c.lwc1(0, gp(c, -0x7784));
                                    c.sw(at(c, V0, SLEW0_TARGET), 0u64);
                                    c.swc1(0, at(c, V0, SLEW0_RATE));
                                    c.g[V1][0] = c.lw(at(c, S2, RIDER));
                                    c.swc1(0, at(c, V1, SLEW5_RATE));
                                    c.sw(at(c, V1, SLEW5_TARGET), 0u64);
                                    break 'b_12ee08;
                                }
                            }
                            'b_12ec84: {
                                c.cycles += 6;
                                c.g[A1][0] = 0x2;
                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                call_ptr(c, ANIM_CONTROLLER_GET_LAYER_ANIM_ID, 0x0012ec50)?;
                                c.g[V1][0] = 0x15;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                if t {
                                    break 'b_12ec84;
                                } else {
                                    c.cycles += 10;
                                    c.mov_s(12, 20);
                                    RIDER_SET_SLEW0_TARGET.call(c, 0x0012ec64)?;
                                    c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                    c.lwc1(0, gp(c, -0x7780));
                                    c.swc1(20, at(c, V0, SLEW0_TARGET));
                                    c.swc1(0, at(c, V0, SLEW0_RATE));
                                    c.g[V1][0] = c.lw(at(c, S2, RIDER));
                                    c.swc1(0, at(c, V1, SLEW5_RATE));
                                    c.swc1(20, at(c, V1, SLEW5_TARGET));
                                    break 'b_12ee08;
                                }
                            }
                            'b_12ecd8: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(at(c, A0, PREWIND_DIR));
                                if c.g[V0][0] != 0u64 {
                                    c.g[V1][0] = c.lw(at(c, S2, RIDER));
                                    break 'b_12ecd8;
                                } else {
                                    c.cycles += 5;
                                    c.lwc1(0, at(c, A0, TURN_INPUT));
                                    c.c_eq_s(0, 20);
                                    if !c.fcc() {
                                        c.g[V1][0] = c.lw(at(c, S2, RIDER));
                                        break 'b_12ecd8;
                                    } else {
                                        'b_12ecd4: {
                                            c.cycles += 4;
                                            RIDER_TURN_AROUND_IF_BACKWARD.call(c, 0x0012ecac)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x15;
                                            if t {
                                                break 'b_12ecd4;
                                            } else {
                                                c.cycles += 8;
                                                c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                                c.g[AT][0] = 0xffffffffbf800000;
                                                c.mtc1(1, 12);
                                                c.g[A2][0] = 0u64;
                                                c.g[A0][0] = c.lw(at(c, V0, ANIM_CTRL));
                                                call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM, 0x0012eccc)?;
                                                frame.restore(c, S0, 0x50);
                                                break 'b_12ee0c;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V1][0] = c.lw(at(c, S2, RIDER));
                                        break 'b_12ecd8;
                                    }
                                }
                            }
                            'b_12ecec: {
                                c.cycles += 4;
                                c.lwc1(5, gp(c, -0x777c));
                                c.g[V0][0] = c.lw(at(c, V1, PREWIND_DIR));
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.g[V1][0];
                                if t {
                                    break 'b_12ecec;
                                } else {
                                    c.cycles += 1;
                                    c.lwc1(5, gp(c, -0x7778));
                                    break 'b_12ecec;
                                }
                            }
                            'b_12ed58: {
                                'b_12ed3c: {
                                    c.cycles += 16;
                                    c.g[V1][0] = c.lw(at(c, S0, PAD_BUTTONS));
                                    c.lwc1(3, gp(c, -0x7774));
                                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 11);
                                    c.lwc1(2, gp(c, -0x7770));
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                    c.mtc1(2, 1);
                                    c.cvt_s_w(1, 1);
                                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 26) as u32);
                                    c.mtc1(3, 0);
                                    c.cvt_s_w(0, 0);
                                    c.mul_s(4, 1, 3);
                                    c.c_lt_s(4, 2);
                                    let t = !c.fcc();
                                    c.mul_s(3, 0, 3);
                                    if t {
                                        break 'b_12ed3c;
                                    } else {
                                        c.cycles += 4;
                                        c.g[AT][0] = 0xffffffffbf800000;
                                        c.mtc1(1, 4);

                                        break 'b_12ed58;
                                    }
                                }
                                c.cycles += 5;
                                c.lwc1(0, gp(c, -0x776c));
                                c.c_lt_s(0, 4);
                                c.mtc1(0, 4);
                                let t = !c.fcc();

                                if t {
                                    break 'b_12ed58;
                                } else {
                                    c.cycles += 2;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 4);
                                    break 'b_12ed58;
                                }
                            }
                            'b_12ed94: {
                                'b_12ed78: {
                                    c.cycles += 4;
                                    c.lwc1(0, gp(c, -0x7768));
                                    c.c_lt_s(3, 0);
                                    let t = !c.fcc();

                                    if t {
                                        break 'b_12ed78;
                                    } else {
                                        c.cycles += 4;
                                        c.g[AT][0] = 0xffffffffbf800000;
                                        c.mtc1(1, 3);
                                        c.lwc1(0, at(c, A0, SLEW0_VALUE));
                                        break 'b_12ed94;
                                    }
                                }
                                c.cycles += 5;
                                c.lwc1(0, gp(c, -0x7764));
                                c.c_lt_s(0, 3);
                                c.mtc1(0, 3);
                                let t = !c.fcc();
                                c.lwc1(0, at(c, A0, SLEW0_VALUE));
                                if t {
                                    break 'b_12ed94;
                                } else {
                                    c.cycles += 2;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 3);
                                    break 'b_12ed94;
                                }
                            }
                            'b_12edfc: {
                                'b_12edf0: {
                                    c.cycles += 19;
                                    c.lwc1(1, gp(c, -0x7760));
                                    c.sub_s(0, 4, 0);
                                    c.swc1(4, at(c, A0, SLEW0_TARGET));
                                    c.abs_s(0, 0);
                                    c.mul_s(0, 5, 0);
                                    c.mul_s(0, 0, 1);
                                    c.swc1(0, at(c, A0, SLEW0_RATE));
                                    c.g[V0][0] = c.lw(at(c, S2, RIDER));
                                    c.lwc1(0, at(c, V0, SLEW5_VALUE));
                                    c.swc1(3, at(c, V0, SLEW5_TARGET));
                                    c.sub_s(0, 3, 0);
                                    c.abs_s(0, 0);
                                    c.mul_s(0, 5, 0);
                                    c.mul_s(0, 0, 1);
                                    c.swc1(0, at(c, V0, SLEW5_RATE));
                                    c.g[A0][0] = c.lw(at(c, S2, RIDER));
                                    c.g[V0][0] = c.lw(at(c, A0, PREWIND_DIR));
                                    let t = c.g[V0][0] != 0u64;

                                    if t {
                                        break 'b_12edf0;
                                    } else {
                                        c.cycles += 4;
                                        c.mov_s(12, 22);
                                        RIDER_SET_SLEW0_TARGET.call(c, 0x0012ede8)?;
                                        c.g[A1][0] = 0x440000;
                                        break 'b_12edfc;
                                    }
                                }
                                c.cycles += 3;
                                c.mov_s(12, 21);
                                RIDER_SET_SLEW5_TARGET.call(c, 0x0012edf8)?;
                                c.g[A1][0] = 0x440000;
                                break 'b_12edfc;
                            }
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2878));
                            RIDER_PREWIND_SUB_PLAY_DIRECTIONAL_ANIM.call(c, 0x0012ee08)?;
                            break 'b_12ee08;
                        }
                        c.cycles += 1;
                        frame.restore(c, S0, 0x50);
                        break 'b_12ee0c;
                    }
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 22, 0x70);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_Render(AI* this)` (0x1291e0): draws the riders of a frame.
///
/// - Riders without a light slot (not visible, slot -1) get one from the
///   graphics manager (+4).
/// - The riders drawn (up to 8, on the stack): in a single view with
///   humans, the first human's rider (camera control mode 0) or the viewed
///   actor's; else every human's, and the computers' and others' when
///   shown (null actors give null riders).
/// - Per rider, in passes: visibility (`Rider_UpdateVisibility`, kept on
///   the stack with the owner), lighting, model and shadow of the visible
///   ones, the board trails as one batch, the owner's pulse, cheat code,
///   wake, air streamer, speed streamer and power-up trail effects; the
///   sparks, anger, spray and attack effects of the visible ones; the
///   attached effects of all.
#[allow(unused_labels)]
pub fn ai_render(c: &mut Cpu) {
    use ai_render_fields::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0x100);
        'b_12926c: {
            c.cycles += 12;
            frame.save(c, S1, 0xe0);
            frame.save(c, S2, 0xd0);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S4, 0xb0);
            c.g[S2][0] = 0u64;
            frame.save(c, S0, 0xf0);
            frame.save(c, S3, 0xc0);
            frame.save_ra(c, 0xa0);
            c.g[V0][0] = c.lw(at(c, S1, RENDER_COUNT));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S4][0] = sx(at(c, SP, 0x60));
            if t {
                break 'b_12926c;
            } else {
                c.cycles += 2;
                c.g[S3][0] = 0xffffffffffffffff;
                c.g[S0][0] = sx(at(c, S1, RENDER_LIST));
                'l_129218: loop {
                    'b_12925c: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(r32(c, S0));
                        c.g[V0][0] = c.lw(at(c, V1, RIDER_VISIBLE));
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw(at(c, S1, RENDER_COUNT));
                            break 'b_12925c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(at(c, V1, RIDER_LIGHT_SLOT));
                            if c.g[V0][0] != c.g[S3][0] {
                                c.g[V0][0] = c.lw(at(c, S1, RENDER_COUNT));
                                break 'b_12925c;
                            } else {
                                c.cycles += 10;
                                c.g[A1][0] = c.lw(gp(c, GP_GFX));
                                c.g[V1][0] = c.lw(at(c, A1, GFX_VTABLE));
                                c.g[A0][0] = c.lh(at(c, V1, GFX_ALLOC_LIGHT_SLOT));
                                c.g[V0][0] = c.lw(at(c, V1, GFX_ALLOC_LIGHT_SLOT + 4));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(r32(c, A0)));
                                call_ptr(c, t, 0x0012924c)?;
                                c.g[V1][0] = c.lw(r32(c, S0));
                                c.g[V0][0] = sx(at(c, V0, 0x4));
                                c.sw(at(c, V1, RIDER_LIGHT_SLOT), c.g[V0][0]);
                                c.g[V0][0] = c.lw(at(c, S1, RENDER_COUNT));
                                break 'b_12925c;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[S2][0] = sx(at(c, S2, 0x1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx(at(c, S0, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129218);
                        }
                        continue 'l_129218;
                    } else {
                        break 'b_12926c;
                    }
                }
            }
        }
        'b_1293c0: {
            'b_1292dc: {
                c.cycles += 4;
                c.g[V0][0] = c.lw(at(c, S1, SINGLE_VIEW));
                c.g[S2][0] = 0u64;
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[V1][0] = c.lw(at(c, S1, HUMAN_COUNT));
                if t {
                    break 'b_1292dc;
                } else {
                    c.cycles += 2;
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.g[V0][0] = c.lw(gp(c, GP_APP));
                    if t {
                        break 'b_1292dc;
                    } else {
                        'b_1292bc: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw(at(c, V0, APP_MODULE));
                            c.g[V0][0] = c.lw(at(c, V1, MODULE_CONTROL));
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                                break 'b_1292bc;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(at(c, V0, CONTROL_MODE));
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                                if t {
                                    break 'b_1292bc;
                                } else {
                                    'b_1292b4: {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lw(at(c, S1, HUMANS));
                                        c.g[S2][0] = 0x1;
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1292b4;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] = c.lw(at(c, A0, ACTOR_RIDER));
                                            break 'b_1292b4;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.sw(r32(c, SP), c.g[V0][0]);
                                    break 'b_1293c0;
                                }
                            }
                        }
                        'b_1292d4: {
                            c.cycles += 5;
                            c.g[A0][0] = c.lw(at(c, S1, VIEWED));
                            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(r32(c, V0)));
                            c.g[S2][0] = 0x1;
                            let t = c.g[A0][0] == 0u64;
                            c.g[V1][0] = 0u64;
                            if t {
                                break 'b_1292d4;
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = c.lw(at(c, A0, ACTOR_RIDER));
                                break 'b_1292d4;
                            }
                        }
                        c.cycles += 2;
                        c.sw(r32(c, V0), c.g[V1][0]);
                        break 'b_1293c0;
                    }
                }
            }
            'b_129320: {
                c.cycles += 2;
                let t = (c.g[V1][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                if t {
                    break 'b_129320;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4));
                    c.g[S0][0] = c.lw(at(c, S1, HUMAN_COUNT));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(r32(c, V0)));
                    c.g[A1][0] = sx(at(c, S1, HUMANS));
                    'l_1292f8: loop {
                        'b_129310: {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw(r32(c, A1));
                            c.g[V1][0] = sx(at(c, V1, 0x4));
                            c.g[S2][0] = sx(at(c, S2, 0x1));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = 0u64;
                            if t {
                                break 'b_129310;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = c.lw(at(c, V0, ACTOR_RIDER));
                                break 'b_129310;
                            }
                        }
                        c.cycles += 4;
                        c.sw(r32(c, V1), c.g[A0][0]);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        let t = c.g[S0][0] != 0u64;
                        c.g[A1][0] = sx(at(c, A1, 0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001292f8);
                            }
                            continue 'l_1292f8;
                        } else {
                            break 'b_129320;
                        }
                    }
                }
            }
            'b_129374: {
                c.cycles += 3;
                c.g[V0][0] = c.lw(at(c, S1, SHOW_COMPUTERS));
                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = c.lw(at(c, S1, SHOW_OTHERS));
                    break 'b_129374;
                } else {
                    'b_129370: {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw(at(c, S1, COMPUTER_COUNT));
                        let t = (c.g[A0][0] as i64) <= 0;
                        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                        if t {
                            break 'b_129370;
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(r32(c, V0)));
                            c.g[S0][0] = c.g[A0][0];
                            c.g[A1][0] = sx(at(c, S1, COMPUTERS));
                            'l_129348: loop {
                                'b_129360: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw(r32(c, A1));
                                    c.g[V1][0] = sx(at(c, V1, 0x4));
                                    c.g[S2][0] = sx(at(c, S2, 0x1));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = 0u64;
                                    if t {
                                        break 'b_129360;
                                    } else {
                                        c.cycles += 1;
                                        c.g[A0][0] = c.lw(at(c, V0, ACTOR_RIDER));
                                        break 'b_129360;
                                    }
                                }
                                c.cycles += 4;
                                c.sw(r32(c, V1), c.g[A0][0]);
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                let t = c.g[S0][0] != 0u64;
                                c.g[A1][0] = sx(at(c, A1, 0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00129348);
                                    }
                                    continue 'l_129348;
                                } else {
                                    break 'b_129370;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S1, SHOW_OTHERS));
                    break 'b_129374;
                }
            }
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_1293c0;
            } else {
                c.cycles += 3;
                c.g[A1][0] = c.lw(at(c, S1, OTHER_COUNT));
                let t = (c.g[A1][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                if t {
                    break 'b_1293c0;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(r32(c, V0)));
                    c.g[A0][0] = sx(at(c, S1, OTHERS));
                    c.g[S0][0] = c.g[A1][0];
                    'l_129398: loop {
                        'b_1293b0: {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw(r32(c, A0));
                            c.g[V1][0] = sx(at(c, V1, 0x4));
                            c.g[S2][0] = sx(at(c, S2, 0x1));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_1293b0;
                            } else {
                                c.cycles += 1;
                                c.g[A1][0] = c.lw(at(c, V0, ACTOR_RIDER));
                                break 'b_1293b0;
                            }
                        }
                        c.cycles += 4;
                        c.sw(r32(c, V1), c.g[A1][0]);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                        let t = c.g[S0][0] != 0u64;
                        c.g[A0][0] = sx(at(c, A0, 0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00129398);
                            }
                            continue 'l_129398;
                        } else {
                            break 'b_1293c0;
                        }
                    }
                }
            }
        }
        'b_1293f4: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_1293f4;
            } else {
                c.cycles += 2;
                c.g[S1][0] = c.g[SP][0];
                c.g[S0][0] = c.g[S2][0];
                'l_1293d0: loop {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw(r32(c, S1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    RIDER_UPDATE_VISIBILITY.call(c, 0x001293dc)?;
                    c.sw(at(c, S1, 0x40), c.g[V0][0]);
                    c.g[V0][0] = c.lw(r32(c, S1));
                    c.g[V1][0] = c.lw(at(c, V0, RIDER_OWNER));
                    c.sw(at(c, S1, 0x20), c.g[V1][0]);
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001293d0);
                        }
                        continue 'l_1293d0;
                    } else {
                        break 'b_1293f4;
                    }
                }
            }
        }
        'b_129428: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_129428;
            } else {
                c.cycles += 3;
                c.g[S1][0] = c.g[SP][0];
                c.g[S0][0] = c.g[S2][0];
                'l_129408: loop {
                    'b_129420: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x40));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129420;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            RIDER_PREPARE_LIGHTING.call(c, 0x0012941c)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129420;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129408);
                        }
                        continue 'l_129408;
                    } else {
                        break 'b_129428;
                    }
                }
            }
        }
        'b_129458: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_129458;
            } else {
                c.cycles += 2;
                c.g[S1][0] = c.g[SP][0];
                c.g[S0][0] = c.g[S2][0];
                'l_129438: loop {
                    'b_129450: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x40));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129450;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            RIDER_RENDER_MODEL.call(c, 0x0012944c)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129450;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129438);
                        }
                        continue 'l_129438;
                    } else {
                        break 'b_129458;
                    }
                }
            }
        }
        'b_1294b8: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;
            c.g[A1][0] = c.g[S4][0];
            if t {
                break 'b_1294b8;
            } else {
                c.cycles += 2;
                c.g[S1][0] = c.g[SP][0];
                c.g[S0][0] = c.g[S2][0];
                'l_129468: loop {
                    'b_129480: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x40));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129480;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            RIDER_RENDER_SHADOW.call(c, 0x0012947c)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129480;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129468);
                        }
                        continue 'l_129468;
                    } else {
                        c.cycles += 2;
                        let t = (c.g[S2][0] as i64) <= 0;
                        c.g[A1][0] = c.g[S4][0];
                        if t {
                            break 'b_1294b8;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = sx(at(c, SP, 0x20));
                            c.g[S0][0] = c.g[S2][0];
                            'l_129498: loop {
                                c.cycles += 8;
                                c.g[V1][0] = c.lw(r32(c, A0));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                c.g[A0][0] = sx(at(c, A0, 0x4));
                                c.g[V0][0] = sx(at(c, V1, TRAIL_FX));
                                if c.g[V1][0] == 0 {
                                    c.g[V0][0] = 0u64;
                                }
                                c.sw(r32(c, A1), c.g[V0][0]);
                                let t = c.g[S0][0] != 0u64;
                                c.g[A1][0] = sx(at(c, A1, 0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x00129498);
                                    }
                                    continue 'l_129498;
                                } else {
                                    break 'b_1294b8;
                                }
                            }
                        }
                    }
                }
            }
        }
        'b_1294f0: {
            c.cycles += 5;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, C_BOARD_TRAIL_FX_RENDER_BATCH, 0x001294c4)?;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_1294f0;
            } else {
                c.cycles += 3;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_1294d8: loop {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    c.g[A0][0] = sx(at(c, A0, PULSE_FX));
                    call_ptr(c, C_PLAYER_PULSE_FX_RENDER, 0x001294e8)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x001294d8);
                        }
                        continue 'l_1294d8;
                    } else {
                        break 'b_1294f0;
                    }
                }
            }
        }
        'b_129520: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_129520;
            } else {
                c.cycles += 4;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_129508: loop {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    c.g[A0][0] = sx(at(c, A0, CHEAT_CODE));
                    call_ptr(c, RIDER_BASE_RENDER_CHEAT_CODE, 0x00129518)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x00129508);
                        }
                        continue 'l_129508;
                    } else {
                        break 'b_129520;
                    }
                }
            }
        }
        'b_129550: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_129550;
            } else {
                c.cycles += 4;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_129538: loop {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    c.g[A0][0] = sx(at(c, A0, WAKE_FX));
                    call_ptr(c, C_BOARD_WAKE_FX_RENDER, 0x00129548)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x00129538);
                        }
                        continue 'l_129538;
                    } else {
                        break 'b_129550;
                    }
                }
            }
        }
        'b_129580: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_129580;
            } else {
                c.cycles += 4;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_129568: loop {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    c.g[A0][0] = sx(at(c, A0, AIR_STREAMER_FX));
                    call_ptr(c, C_AIR_STREAMER_FX_RENDER, 0x00129578)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x00129568);
                        }
                        continue 'l_129568;
                    } else {
                        break 'b_129580;
                    }
                }
            }
        }
        'b_1295b0: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_1295b0;
            } else {
                c.cycles += 4;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_129598: loop {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    c.g[A0][0] = sx(at(c, A0, SPEED_STREAMER_FX));
                    call_ptr(c, C_SPEED_BOOST_STREAMER_FX_RENDER, 0x001295a8)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x00129598);
                        }
                        continue 'l_129598;
                    } else {
                        break 'b_1295b0;
                    }
                }
            }
        }
        'b_1295e0: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_1295e0;
            } else {
                c.cycles += 4;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_1295c8: loop {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    c.g[A0][0] = sx(at(c, A0, POWER_UP_TRAIL));
                    call_ptr(c, RIDER_BASE_RENDER_POWER_UP_TRAIL, 0x001295d8)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x001295c8);
                        }
                        continue 'l_1295c8;
                    } else {
                        break 'b_1295e0;
                    }
                }
            }
        }
        'b_129614: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_129614;
            } else {
                c.cycles += 2;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                'l_1295f0: loop {
                    'b_12960c: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x20));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_12960c;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            c.g[A0][0] = sx(at(c, A0, SPARK_FX));
                            call_ptr(c, C_BOARD_SPARK_FX_RENDER, 0x00129608)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_12960c;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001295f0);
                        }
                        continue 'l_1295f0;
                    } else {
                        break 'b_129614;
                    }
                }
            }
        }
        'b_12964c: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;

            if t {
                break 'b_12964c;
            } else {
                c.cycles += 3;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                c.g[S0][0] = c.g[S2][0];
                'l_129628: loop {
                    'b_129644: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x20));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129644;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            c.g[A0][0] = sx(at(c, A0, ANGER_FX));
                            call_ptr(c, C_ANGER_FX_RENDER, 0x00129640)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129644;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129628);
                        }
                        continue 'l_129628;
                    } else {
                        break 'b_12964c;
                    }
                }
            }
        }
        'b_12967c: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;
            c.g[S0][0] = c.g[S2][0];
            if t {
                break 'b_12967c;
            } else {
                c.cycles += 1;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                'l_129658: loop {
                    'b_129674: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x20));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129674;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            c.g[A0][0] = sx(at(c, A0, SPRAY_FX));
                            call_ptr(c, C_DYNAMIC_SPRAY_FX_RENDER, 0x00129670)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_129674;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129658);
                        }
                        continue 'l_129658;
                    } else {
                        break 'b_12967c;
                    }
                }
            }
        }
        'b_1296ac: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;
            c.g[S0][0] = c.g[S2][0];
            if t {
                break 'b_1296ac;
            } else {
                c.cycles += 1;
                c.g[S1][0] = sx(at(c, SP, 0x20));
                'l_129688: loop {
                    'b_1296a4: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, S1, 0x20));
                        if c.g[V0][0] == 0u64 {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_1296a4;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw(r32(c, S1));
                            c.g[A0][0] = sx(at(c, A0, ATTACK_FX));
                            call_ptr(c, RIDER_BASE_RENDER_ATTACK_FX, 0x001296a0)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            break 'b_1296a4;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] != 0u64;
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00129688);
                        }
                        continue 'l_129688;
                    } else {
                        break 'b_1296ac;
                    }
                }
            }
        }
        'b_1296d4: {
            c.cycles += 2;
            let t = (c.g[S2][0] as i64) <= 0;
            c.g[S0][0] = c.g[S2][0];
            if t {
                break 'b_1296d4;
            } else {
                c.cycles += 3;
                c.g[S1][0] = c.g[SP][0];
                c.g[A0][0] = c.lw(r32(c, S1));
                'l_1296c0: loop {
                    c.cycles += 5;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    c.g[S1][0] = sx(at(c, S1, 0x4));
                    RIDER_RENDER_ATTACHED_FX.call(c, 0x001296cc)?;
                    if c.g[S0][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, S1));
                        if event_due(c) {
                            deliver_event_here(c, 0x001296c0);
                        }
                        continue 'l_1296c0;
                    } else {
                        break 'b_1296d4;
                    }
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderCruiseSub_HandleInput(RiderCruiseSub* this, Pad* pad)`
/// (0x131620): the ground cruise sub-state's input, once per tick.
///
/// - The stick axes become steer/press/crouch inputs (6-bit signed fields
///   times 1/31); moving backwards (velocity . forward < 0) drops the crouch.
/// - The finish line, a reset (pad bit 12), leaving the ground, a handplant
///   (bit 13, otherwise a rail jump), a jump (`Rider_TryEnterPrewind`) and a
///   board press each take over and end the function when they start; a
///   handplant attempt or a tweak clamps the steer to +-0.5.
/// - Boost input, the slew targets, the timed-event and idle-look
///   animations.
/// - Then the layer-2 (body) animation: kept on 0x16 (pushing off) and 0x15
///   (turn around), on kinds 10 and 5; otherwise picked from the speed, the
///   turn, the surface, the crouch and the carve (0x5..0x11), with the
///   slew targets set to match, and played when it changed.
///
/// The blocks follow the original's (labels are their guest addresses);
/// the scratch registers are left as it leaves them.
pub fn rider_cruise_sub_handle_input(c: &mut Cpu) {
    use cruise::*;
    guest(c, |c| {
        // The frame (0x60): ra, s0..s2, f20..f23.
        let frame = Frame::open(c, 0x60);
        {
            c.cycles += 41;
            c.lwc1(0, gpo(c, -0x75f4)); // 1/31
            frame.save(c, S0, 0x30);
            frame.save(c, S1, 0x20);
            c.g[S0][0] = c.g[A1][0];
            frame.save_fpr(c, 23, 0x58);
            c.g[S1][0] = c.g[A0][0];
            frame.save_fpr(c, 22, 0x50);
            frame.save_fpr(c, 21, 0x48);
            frame.save_fpr(c, 20, 0x40);
            frame.save(c, S2, 0x10);
            frame.save_ra(c, 0);
            c.mtc1(0, 23); // f23 = 0
            c.g[A0][0] = c.lw(at(c, S0, PAD_BUTTONS));
            c.g[V0][0] = c.lw(at(c, S0, PAD_STICKS));
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 6);
            c.g[A2][0] = c.lw(at(c, S1, RIDER));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 26);
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 26) as u32);
            c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 26) as u32);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
            // velocity . forward
            c.lqc2(3, at(c, A2, VEL));
            c.lqc2(5, at(c, A2, FORWARD));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(5, 4);
            c.mtc1(5, 2);
            c.mtc1(3, 3);
            c.cvt_s_w(3, 3);
            c.mtc1(4, 4);
            c.cvt_s_w(4, 4);
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.c_lt_s(2, 23);
            c.mul_s(21, 3, 0); // f21 steer
            c.mul_s(20, 1, 0); // f20 crouch
            let backwards = c.fcc();
            c.mul_s(22, 4, 0); // f22 press
            if backwards {
                c.cycles += 1;
                c.mov_s(20, 23);
            }
        }
        'b_131c0c: {
            c.cycles += 4;
            c.g[A0][0] = c.g[A2][0];
            call_ptr(c, 0x0011_6378, 0x0013_16d0)?; // Rider_TryEnterFinishLine
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'b_131c0c;
            }
            c.cycles += 8;
            c.g[A1][0] = c.lw(at(c, S0, PAD_BUTTONS));
            c.g[A2][0] = 0;
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 12) as u32);
            c.g[A1][0] &= 0x1;
            call_ptr(c, 0x0011_6120, 0x0013_16f0)?; // Rider_CheckResetPath
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'b_131c0c;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0013_1cc0, 0x0013_1700)?; // RiderCruiseSub_CheckAirborne
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'b_131c0c;
            }
            'b_131780: {
                c.cycles += 4;
                c.g[V0][0] = c.lw(at(c, S0, PAD_BUTTONS)) & 0x2000;
                if c.g[V0][0] != 0 {
                    // Handplant button: try a handplant (a1 = press > 0.5).
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f00_0000;
                    c.mtc1(1, 20);
                    c.c_lt_s(20, 22);
                    let hard = c.fcc();
                    c.g[A1][0] = 0x1;
                    if !hard {
                        c.cycles += 1;
                        c.g[A1][0] = 0;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(at(c, S1, RIDER));
                    call_ptr(c, 0x0010_7578, 0x0013_173c)?; // Rider_TryStartHandplant
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x30);
                        break 'b_131c0c;
                    }
                    // steer = clamp(steer, -0.5, 0.5); press 1, crouch 0.
                    c.cycles += 6;
                    c.g[AT][0] = 0xffff_ffff_bf00_0000;
                    c.mtc1(1, 0);
                    c.c_le_s(0, 21);
                    if c.fcc() {
                        c.min_s(0, 21, 20);
                    }
                    c.cycles += 5;
                    c.mov_s(21, 0);
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 22);
                    c.mov_s(20, 23);
                    break 'b_131780;
                }
                c.cycles += 4;
                c.g[A0][0] = c.lw(at(c, S1, RIDER));
                call_ptr(c, 0x0010_6848, 0x0013_1778)?; // Rider_TryStartRailslide
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'b_131c0c;
                }
            }
            c.cycles += 9;
            c.g[A1][0] = c.lw(at(c, S0, PAD_BUTTONS));
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 14) as u32);
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 15) as u32);
            c.g[A2][0] &= 0x1;
            c.g[A1][0] &= 0x1;
            call_ptr(c, 0x0011_62c8, 0x0013_179c)?; // Rider_TryEnterPrewind
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'b_131c0c;
            }
            c.cycles += 4;
            let jump = c.lw(at(c, S0, PAD_BUTTONS)) & 0x8000;
            c.g[V0][0] = jump;
            c.g[V0][0] = c.lw(at(c, S0, PAD_STICKS));
            if jump != 0 {
                // Jump held: press 1, crouch 0.
                c.cycles += 3;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 22);
                c.mtc1(0, 20);
            }
            c.cycles += 10;
            c.lwc1(0, gpo(c, -0x75f0)); // 1/31
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
            c.mtc1(2, 12);
            c.cvt_s_w(12, 12);
            c.mul_s(12, 12, 0);
            call_ptr(c, 0x0011_61d0, 0x0013_17e0)?; // Rider_TryEnterBoardPress
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'b_131c0c;
            }
            c.cycles += 9;
            c.g[A1][0] = c.lw(at(c, S0, PAD_BUTTONS));
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 19) as u32);
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 18) as u32);
            c.g[A2][0] &= 0x1;
            c.g[A1][0] &= 0x1;
            call_ptr(c, 0x0011_63b0, 0x0013_1804)?; // Rider_UpdateTweakLayerAnim
            if c.g[V0][0] != 0 {
                // A tweak plays: steer = clamp(steer, -0.5, 0.5).
                c.cycles += 8;
                c.g[AT][0] = 0xffff_ffff_bf00_0000;
                c.mtc1(1, 0);
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 1);
                c.c_le_s(0, 21);
                if c.fcc() {
                    c.min_s(0, 21, 1);
                }
                c.cycles += 2;
                c.mov_s(21, 0);
            }
            c.g[A1][0] = c.lw(at(c, S0, PAD_BUTTONS));
            cruise_body_anim(c)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x58);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `RiderCruiseSub_HandleInput` from 0x131834: boost, slews, the timed and
/// idle animations, then the body-layer animation choice (s1 = this, a1 =
/// the pad's buttons, f20..f22 crouch/steer/press). Ends with the final
/// 1-cycle block (before the `lq s0`).
pub(crate) fn cruise_body_anim(c: &mut Cpu) -> crate::call::Flow {
    use cruise::*;
    'b_131be8: {
        'b_1318c4: {
            c.cycles += 25;
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 16) as u32);
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
            c.g[A2][0] &= 0x1;
            c.g[A1][0] &= 0x1;
            call_ptr(c, 0x0011_4130, 0x0013_184c)?; // Rider_HandleBoostInput
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.mov_s(12, 21);
            call_ptr(c, 0x0011_3e80, 0x0013_1858)?; // Rider_SetSlew0Target
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            c.mov_s(13, 20);
            c.mov_s(12, 22);
            call_ptr(c, 0x0011_3f88, 0x0013_1868)?; // Rider_SetSlew3And4Targets
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            call_ptr(c, 0x0011_5b58, 0x0013_1870)?; // Rider_UpdateTimedEventAnim
            c.g[A0][0] = c.lw(at(c, S1, RIDER));
            call_ptr(c, 0x0011_5d48, 0x0013_1878)?; // Rider_UpdateIdleLookAnim
            c.g[V0][0] = c.lw(at(c, S1, RIDER));
            c.g[A1][0] = 0x2;
            c.g[A0][0] = c.lw(at(c, V0, ANIM));
            call_ptr(c, 0x0031_2aa0, 0x0013_1888)?; // AnimController_GetLayerAnimId
                                                    // s0 = the anim to play, s2 = the current one.
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = 0x16;
            c.g[S2][0] = c.g[S0][0];
            if c.g[S0][0] != c.g[V0][0] {
                break 'b_1318c4;
            }
            // 0x16 (pushing off): fixed slew rates (1/30), values 0.
            c.cycles += 11;
            c.g[V1][0] = c.lw(at(c, S1, RIDER));
            c.lwc1(0, gpo(c, -0x75ec));
            c.sw(at(c, V1, SLEW0_VALUE), 0);
            c.swc1(0, at(c, V1, SLEW0_RATE));
            c.g[V0][0] = c.lw(at(c, S1, RIDER));
            c.swc1(0, at(c, V0, SLEW1_RATE));
            c.sw(at(c, V0, SLEW1_VALUE), 0);
            c.g[V1][0] = c.lw(at(c, S1, RIDER));
            c.swc1(0, at(c, V1, SLEW2_RATE));
            c.sw(at(c, V1, SLEW2_VALUE), 0);
            break 'b_131be8;
        }
        'b_13191c: {
            'b_1318fc: {
                c.cycles += 3;
                c.g[V0][0] = 0x15;
                if c.g[S0][0] == c.g[V0][0] {
                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                    break 'b_1318fc;
                }
                c.cycles += 7;
                c.g[A0][0] = c.lw(at(c, S1, RIDER));
                c.mtc1(0, 1);
                c.lwc1(0, at(c, A0, SPIN_SPEED));
                c.c_eq_s(0, 1);
                if !c.fcc() {
                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                    break 'b_13191c;
                }
                c.cycles += 4;
                call_ptr(c, 0x0011_4cc0, 0x0013_18f4)?; // Rider_TurnAroundIfBackward
                let turned = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                if !turned {
                    break 'b_13191c;
                }
            }
            // 0x15 (turn around).
            c.cycles += 8;
            c.g[S0][0] = 0x15;
            c.lwc1(0, gpo(c, -0x75e8));
            c.sw(at(c, V0, SLEW0_VALUE), 0);
            c.swc1(0, at(c, V0, SLEW0_RATE));
            c.g[V1][0] = c.lw(at(c, S1, RIDER));
            c.swc1(0, at(c, V1, SLEW1_RATE));
            c.sw(at(c, V1, SLEW1_VALUE), 0);
            break 'b_131be8;
        }
        'b_131944: {
            c.cycles += 6;
            c.g[A1][0] = 0x2;
            c.g[A0][0] = c.lw(at(c, V0, ANIM));
            call_ptr(c, 0x0031_1ae8, 0x0013_1928)?; // AnimController_GetLayerAnimEntry
            c.g[V1][0] = 0xa;
            let other = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = c.lw(at(c, S1, RIDER));
            if other {
                break 'b_131944;
            }
            // Kind 10: steer slew only.
            c.cycles += 4;
            c.lwc1(0, gpo(c, -0x75e4));
            c.sw(at(c, V0, SLEW0_VALUE), 0);
            c.swc1(0, at(c, V0, SLEW0_RATE));
            break 'b_131be8;
        }
        'b_131980: {
            c.cycles += 6;
            c.g[A1][0] = 0x2;
            c.g[A0][0] = c.lw(at(c, V0, ANIM));
            call_ptr(c, 0x0031_1ae8, 0x0013_1950)?; // AnimController_GetLayerAnimEntry
            c.g[V1][0] = 0x5;
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V1][0] = c.lw(at(c, S1, RIDER));
                break 'b_131980;
            }
            // Kind 5: steer slews toward the steer input.
            c.cycles += 9;
            c.g[V0][0] = c.lw(at(c, S1, RIDER));
            c.lwc1(1, gpo(c, -0x75e0));
            c.lwc1(0, at(c, V0, STEER));
            c.swc1(1, at(c, V0, SLEW0_RATE));
            c.swc1(0, at(c, V0, SLEW0_VALUE));
            c.g[V1][0] = c.lw(at(c, S1, RIDER));
            c.swc1(1, at(c, V1, SLEW1_RATE));
            c.sw(at(c, V1, SLEW1_VALUE), 0);
            break 'b_131be8;
        }
        // The free choice: f20 = speed (|velocity|), v1 = rider.
        'b_131b6c: {
            c.cycles += 16;
            c.mtc1(0, 3);
            c.lqc2(3, at(c, V1, VEL));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 20);
            c.lwc1(0, at(c, V1, TURN));
            c.c_eq_s(0, 3);
            let turning = !c.fcc();
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
            if turning {
                break 'b_131b6c;
            }
            'b_131a04: {
                c.cycles += 7;
                c.lwc1(0, gpo(c, -0x75dc));
                c.lwc1(2, at(c, V1, STEER));
                c.lwc1(1, gpo(c, -0x75d8));
                c.c_lt_s(20, 0);
                c.swc1(2, at(c, V1, SLEW0_VALUE));
                let slow = c.fcc();
                c.swc1(1, at(c, V1, SLEW0_RATE));
                if !slow {
                    break 'b_131a04;
                }
                c.cycles += 4;
                c.c_lt_s(3, 22);
                if c.fcc() {
                    c.g[S0][0] = 0x16;
                    break 'b_131be8;
                }
                c.cycles += 6;
                c.lwc1(1, gpo(c, -0x75d4));
                c.abs_s(0, 21);
                c.c_lt_s(1, 0);
                let steering = c.fcc();
                c.g[S0][0] = 0x16;
                if steering {
                    break 'b_131be8;
                }
            }
            'b_131a7c: {
                c.cycles += 5;
                c.g[A0][0] = c.lw(at(c, S1, RIDER));
                c.g[V1][0] = 0x4;
                c.g[V0][0] = c.lw(at(c, A0, SURFACE));
                if c.g[V0][0] != c.g[V1][0] {
                    c.lwc1(0, at(c, A0, CARVE));
                    break 'b_131a7c;
                }
                // Surface 4.
                c.cycles += 13;
                c.lwc1(1, at(c, A0, STEER));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.abs_s(3, 1);
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 2);
                c.sub_s(0, 0, 3);
                c.lwc1(1, at(c, A0, CROUCH));
                c.mul_s(0, 0, 2);
                c.c_le_s(1, 0);
                let low = c.fcc();
                c.g[S0][0] = 0xe;
                if low {
                    break 'b_131be8;
                }
                c.cycles += 6;
                c.lwc1(1, at(c, A0, BOOST_INPUT));
                c.mtc1(0, 0);
                c.c_lt_s(0, 1);
                let boosting = c.fcc();
                c.g[S0][0] = 0xf;
                if !boosting {
                    break 'b_131be8;
                }
                c.cycles += 4;
                c.lwc1(0, gpo(c, -0x75d0));
                c.c_lt_s(3, 0);
                if !c.fcc() {
                    break 'b_131be8;
                }
                c.cycles += 2;
                c.g[S0][0] = 0x8;
                break 'b_131be8;
            }
            'b_131af0: {
                c.cycles += 6;
                c.lwc1(1, gpo(c, -0x75cc));
                c.abs_s(0, 0);
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                    break 'b_131af0;
                }
                c.cycles += 7;
                c.lwc1(0, at(c, A0, SLOPE));
                c.lwc1(1, gpo(c, -0x75c8));
                c.abs_s(0, 0);
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    c.g[V0][0] = c.lw(at(c, S1, RIDER));
                    break 'b_131af0;
                }
                c.cycles += 5;
                call_ptr(c, 0x0031_7810, 0x0013_1ab8)?; // Rand_NextGame
                c.g[V0][0] &= 0x1;
                let odd = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(at(c, S1, RIDER));
                if odd {
                    break 'b_131af0;
                }
                // A carve: 0x10 / 0x11 by side (flipped when goofy).
                c.cycles += 3;
                c.g[V1][0] = c.lw(at(c, V0, GOOFY));
                c.lwc1(1, at(c, V0, CARVE));
                if c.g[V1][0] != 0 {
                    c.cycles += 1;
                    c.neg_s(1, 1);
                }
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_lt_s(1, 0);
                let left = c.fcc();
                c.g[S0][0] = 0x11;
                if !left {
                    break 'b_131be8;
                }
                c.cycles += 2;
                c.g[S0][0] = 0x10;
                break 'b_131be8;
            }
            c.cycles += 13;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            c.lwc1(0, at(c, V0, STEER));
            c.abs_s(0, 0);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 3);
            c.sub_s(1, 1, 0);
            c.lwc1(2, at(c, V0, CROUCH));
            c.mul_s(1, 1, 3);
            c.c_le_s(2, 1);
            let low = c.fcc();
            c.g[S0][0] = 0x5;
            if low {
                break 'b_131be8;
            }
            c.cycles += 6;
            c.lwc1(1, at(c, V0, BOOST_INPUT));
            c.mtc1(0, 0);
            c.c_lt_s(0, 1);
            let boosting = c.fcc();
            c.g[S0][0] = 0x8;
            if boosting {
                break 'b_131be8;
            }
            c.cycles += 5;
            c.lwc1(0, gpo(c, -0x75c4));
            c.c_lt_s(20, 0);
            let slow = c.fcc();
            c.g[S0][0] = 0x7;
            if slow {
                break 'b_131be8;
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw(at(c, V0, SURFACE));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
            c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = 0x6;
                break 'b_131be8;
            }
            c.cycles += 2;
            break 'b_131be8;
        }
        // Turning: 0xb, or 0xc / 0xd by the turn side (and stance).
        {
            c.cycles += 7;
            c.lwc1(0, gpo(c, -0x75c0));
            c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
            c.swc1(3, at(c, V1, SLEW0_VALUE));
            c.swc1(0, at(c, V1, SLEW0_RATE));
            c.lwc1(0, gpo(c, -0x75bc));
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.lwc1(0, gpo(c, -0x75b8));
            }
        }
        c.cycles += 4;
        c.c_lt_s(0, 20);
        if c.fcc() {
            c.g[S0][0] = 0xb;
            break 'b_131be8;
        }
        c.cycles += 8;
        c.g[V0][0] = c.lw(at(c, S1, RIDER));
        c.lwc1(1, gpo(c, -0x75b4));
        c.lwc1(2, at(c, V0, TURN));
        c.abs_s(0, 2);
        c.c_lt_s(0, 1);
        let gentle = c.fcc();
        c.g[S0][0] = 0xb;
        if gentle {
            break 'b_131be8;
        }
        'b_131be4: {
            c.cycles += 4;
            c.c_lt_s(3, 2);
            let right = c.fcc();
            c.g[S0][0] = 0xd;
            if right {
                c.cycles += 3;
                c.g[V1][0] = c.lw(at(c, V0, GOOFY));
                c.g[V0][0] = 0xc;
                break 'b_131be4;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(at(c, V0, GOOFY));
            c.g[S0][0] = 0xc;
            c.g[V0][0] = 0xd;
        }
        c.cycles += 1;
        if c.g[V1][0] != 0 {
            c.g[S0][0] = c.g[V0][0];
        }
    }
    // Play the chosen animation when it changed.
    c.cycles += 2;
    c.g[A1][0] = c.g[S0][0];
    if c.g[S0][0] != c.g[S2][0] {
        c.cycles += 6;
        c.g[V0][0] = c.lw(at(c, S1, RIDER));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 12);
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lw(at(c, V0, ANIM));
        call_ptr(c, 0x0031_28e8, 0x0013_1c08)?; // AnimController_PlayAnim
    }
    c.cycles += 1;
    Ok(())
}

pub(crate) mod prewind {

    /// Stick x / y (floats).
    pub const STICK_X: u32 = 0x2a4;

    pub const STICK_Y: u32 = 0x2b0;

    /// Non-zero: the rider rides switch (x is mirrored).
    pub const SWITCH: u32 = 0x320;

    /// Column of the animation table (stance/variant).
    pub const ANIM_COLUMN: u32 = 0x328;

    /// The rider's animation controller.
    pub const ANIM_CTL: u32 = 0x784;
}

/// `Math_Atan(f12)` (0x31c228).
pub(crate) const MATH_ATAN: u32 = 0x0031_c228;

/// `AnimController_GetLayerAnimId(ctl, layer)` (0x312aa0).
pub(crate) const ANIM_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;

/// `AnimController_PlayAnim(ctl, anim, flags, f12 speed)` (0x3128e8).
pub(crate) const ANIM_PLAY_ANIM: u32 = 0x0031_28e8;

/// `void RiderPrewindSub_PlayDirectionalAnim(RiderPrewindSub* self, int (*table)[5])`
/// (0x12ee30): pick the prewind animation from the stick direction and play
/// it on layer 2 if it is not already playing. The stick angle
/// `atan2(-y, ±x)` (x mirrored when riding switch) selects one of nine rows
/// of the table (row 0: stick centred; then by angle sector: back, three
/// left-ish, three right-ish, forward), the rider's column selects the
/// entry. Plays with flags 0 and speed -1.
pub fn rider_prewind_sub_play_directional_anim(c: &mut Cpu) {
    use prewind::*;
    guest(c, |c| {
        let gp = r32(c, GP);
        let lit = |c: &mut Cpu, off: u32| Addr(gp.wrapping_sub(off)).read_fpu(c);
        let zero = FpuF32::ZERO;

        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let sub = Addr(r32(c, A0));
        let table = r32(c, A1);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 23, 0x48);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        let rider = sub.lw(c);
        c.g[V0][0] = rider;
        let rider = Addr(rider as u32);
        let x = rider.off(STICK_X).read_fpu(c);
        let y = rider.off(STICK_Y).read_fpu(c);
        c.f[21] = x.0;
        c.f[12] = y.0;
        let (ax, ay) = (x.abs(), y.abs());
        c.f[1] = ax.0;
        c.f[0] = ay.0;
        let x_major = ay.lt(ax);
        abi::set_fcc(c, x_major);
        // Stick magnitude (max norm) in f23.
        let mag = if x_major {
            c.cycles += 1;
            ax
        } else {
            ay
        };
        c.f[23] = mag.0;

        // The angle into f20. `stage` is where the shared tail is entered:
        // 0: f20 = f12 (atan result), 1: v0 = *self, 2: a1 = 2, 3: done.
        c.cycles += 3;
        let switch = rider.off(SWITCH).lw(c);
        c.g[V0][0] = switch;
        c.f[22] = 0;
        // Some((num >= 0)) when the denominator is zero.
        let mut axis: Option<bool> = None;
        let mut stage = 0;
        let atan = |c: &mut Cpu, num: FpuF32, den: FpuF32, back: u32| -> Flow {
            c.cycles += 7;
            c.f[12] = num.div(den).0;
            call_ptr(c, MATH_ATAN, back)?;
            let den_neg = den.lt(zero);
            abi::set_fcc(c, den_neg);
            c.f[12] = c.f[0];
            Ok(())
        };
        if switch != 0 {
            c.cycles += 6;
            let (num, den) = (-y, -x);
            c.f[20] = den.0;
            let den_zero = den.eq_s(zero);
            abi::set_fcc(c, den_zero);
            c.f[21] = num.0;
            if den_zero {
                c.cycles += 4;
                let num_zero = num.eq_s(zero);
                abi::set_fcc(c, num_zero);
                c.f[20] = num.0;
                if num_zero {
                    stage = 1;
                } else {
                    c.cycles += 3;
                    axis = Some(zero.le(num));
                }
            } else {
                atan(c, num, den, 0x0012_eecc)?;
                if den.lt(zero) {
                    c.cycles += 3;
                    let num_pos = zero.lt(num);
                    abi::set_fcc(c, num_pos);
                    if num_pos {
                        rider_prewind_add_pi(c, gp);
                    } else {
                        c.cycles += 2;
                        rider_prewind_sub_pi(c, gp);
                    }
                }
            }
        } else {
            c.cycles += 5;
            let (num, den) = (-y, x);
            let den_zero = den.eq_s(zero);
            abi::set_fcc(c, den_zero);
            c.f[20] = num.0;
            if den_zero {
                c.cycles += 4;
                let num_zero = num.eq_s(zero);
                abi::set_fcc(c, num_zero);
                if num_zero {
                    c.g[V0][0] = sub.lw(c);
                    stage = 2;
                } else {
                    c.cycles += 1;
                    axis = Some(zero.le(num));
                }
            } else {
                atan(c, num, den, 0x0012_ef3c)?;
                if den.lt(zero) {
                    c.cycles += 3;
                    let num_pos = zero.lt(num);
                    abi::set_fcc(c, num_pos);
                    if num_pos {
                        rider_prewind_add_pi(c, gp);
                    } else {
                        rider_prewind_sub_pi(c, gp);
                    }
                }
            }
        }
        if let Some(num_nonneg) = axis {
            abi::set_fcc(c, num_nonneg);
            // Straight up or down: +-pi/2 from the literal pool.
            c.cycles += 3;
            c.f[20] = lit(c, 0x775c).0;
            c.g[V0][0] = sub.lw(c);
            if num_nonneg {
                c.cycles += 3;
                c.f[20] = lit(c, 0x7758).0;
                c.g[A1][0] = 2;
                stage = 3;
            } else {
                stage = 2;
            }
        }
        if stage <= 0 {
            c.cycles += 1;
            c.f[20] = c.f[12];
        }
        if stage <= 1 {
            c.cycles += 1;
            c.g[V0][0] = sub.lw(c);
        }
        if stage <= 2 {
            c.cycles += 1;
            c.g[A1][0] = 2;
        }

        // The animation playing on layer 2.
        c.cycles += 7;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(ANIM_CTL)).lw(c);
        call_ptr(c, ANIM_GET_LAYER_ANIM_ID, 0x0012_ef80)?;
        c.f[0] = 0;
        let centred = mag.eq_s(zero);
        abi::set_fcc(c, centred);
        c.g[A0][0] = c.g[V0][0];

        // table[row][rider->column] into a1 (v1 = rider, v0 = entry address).
        let lookup = |c: &mut Cpu, reload: bool, row_off: u32| {
            if reload {
                c.g[V1][0] = sub.lw(c);
            }
            let column = Addr((c.g[V1][0] as u32).wrapping_add(ANIM_COLUMN)).lw(c);
            c.g[V0][0] = sx((column as u32) << 2);
            c.g[V0][0] = sx(((column as u32) << 2).wrapping_add(table));
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(row_off)).lw(c);
        };
        let angle = FpuF32(c.f[20]);
        if centred {
            c.cycles += 6;
            lookup(c, true, 0);
        } else {
            c.cycles += 5;
            let back = lit(c, 0x774c);
            c.f[1] = back.0;
            let a = angle.abs();
            c.f[0] = a.0;
            let behind = back.lt(a);
            abi::set_fcc(c, behind);
            if behind {
                c.cycles += 6;
                lookup(c, true, 0x14);
            } else {
                // (literal, angle above it (else below), row offset)
                const SECTORS: [(u32, bool, u32); 5] = [
                    (0x7748, true, 0x28),
                    (0x7744, true, 0x3c),
                    (0x7740, true, 0x50),
                    (0x773c, false, 0x64),
                    (0x7738, false, 0x78),
                ];
                let mut found = false;
                for &(off, above, row) in SECTORS.iter() {
                    c.cycles += 4;
                    let edge = lit(c, off);
                    c.f[0] = edge.0;
                    let hit = if above {
                        edge.lt(angle)
                    } else {
                        angle.lt(edge)
                    };
                    abi::set_fcc(c, hit);
                    if hit {
                        c.cycles += 6;
                        lookup(c, true, row);
                        found = true;
                        break;
                    }
                }
                if !found {
                    c.cycles += 5;
                    let edge = lit(c, 0x7734);
                    c.f[0] = edge.0;
                    let hit = angle.lt(edge);
                    abi::set_fcc(c, hit);
                    c.g[V1][0] = sub.lw(c);
                    if hit {
                        c.cycles += 5;
                        lookup(c, false, 0x8c);
                    } else {
                        c.cycles += 4;
                        lookup(c, false, 0xa0);
                    }
                }
            }
        }

        c.cycles += 2;
        let differs = c.g[A1][0] != c.g[A0][0];
        frame.restore(c, S0, 0x20);
        if differs {
            c.cycles += 6;
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(ANIM_CTL)).lw(c);
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.f[12] = 0xbf80_0000;
            c.g[A2][0] = 0;
            call_ptr(c, ANIM_PLAY_ANIM, 0x0012_f0f4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x48);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// atan2 quadrant fix-up: `f12 += pi` (literal at gp-0x7754).
pub(crate) fn rider_prewind_add_pi(c: &mut Cpu, gp: u32) {
    c.cycles += 3;
    let pi = Addr(gp.wrapping_sub(0x7754)).read_fpu(c);
    c.f[0] = pi.0;
    c.f[12] = (FpuF32(c.f[12]) + pi).0;
}

/// atan2 quadrant fix-up: `f12 -= pi` (literal at gp-0x7750).
pub(crate) fn rider_prewind_sub_pi(c: &mut Cpu, gp: u32) {
    c.cycles += 2;
    let pi = Addr(gp.wrapping_sub(0x7750)).read_fpu(c);
    c.f[0] = pi.0;
    c.f[12] = (FpuF32(c.f[12]) - pi).0;
}

// ---------------------------------------------------------------------------
// AI_UpdateRaceTimeLimit (0x12bb20)

/// AI manager fields.
pub(crate) const AI_STATE: u32 = 0x0;

// 5: racing, 3: waiting for the session
pub(crate) const AI_TIMER: u32 = 0x8;

// race ticks
pub(crate) const AI_COUNTDOWN: u32 = 0x18;

// ticks left after the first finisher; -1 done
pub(crate) const AI_RIDERS: u32 = 0x40;

// actor pointers
pub(crate) const AI_RIDER_COUNT: u32 = 0x7c;

/// Actor +0x18 is the rider; rider +0x470 the finish time (>= 0 when
/// finished), +0x480 the "forced finish" flag.
pub(crate) const ACTOR_RIDER: u32 = 0x18;

pub(crate) const RIDER_FINISH_TIME: u32 = 0x470;

pub(crate) const RIDER_FORCED_FINISH: u32 = 0x480;

/// World (gp-0x848): +0x78 the BE interface, +0x84 the session (+0x214
/// its state, +0x28 the replay).
pub(crate) const GP_WORLD: u32 = 0x848;

pub(crate) const GP_GAME_COMM: u32 = 0x204;

// gp-0x204
pub(crate) const SESSION_STATE: u32 = 0x214;

pub(crate) const SESSION_REPLAY: u32 = 0x28;

/// Globals: race active flag and race mode byte.
pub(crate) const RACE_ACTIVE: u32 = 0x0053_4b30;

pub(crate) const RACE_MODE: u32 = 0x0053_5c12;

/// The last tick at which a first finisher starts a countdown, the hard
/// time limit, and the longest countdown (2 minutes).
pub(crate) const LAST_COUNTDOWN_START: i64 = 0xd32b;

pub(crate) const TIME_LIMIT: u64 = 0xd3a4;

pub(crate) const MAX_COUNTDOWN: u64 = 0x1c20;

pub(crate) const BE_GET_INTERFACE: u32 = 0x0014_dd58;

// BELibrary_GetInterface (table)
pub(crate) const BE_GET_INSTANCE: u32 = 0x0014_dc80;

// BELibrary_GetInstance (table)
pub(crate) const REPLAY_STOP_RECORDING: u32 = 0x0026_fa50;

// (table)
pub(crate) const GAME_COMM_SEND_SYNC_CHECK: u32 = 0x0025_8f20;

// (table)
pub(crate) const AI_GET_SYNC_CHECKSUM: Callee = Callee {
    addr: 0x0012_b420,
    f: ee::s00::AI_GetSyncChecksum,
};

pub(crate) const RIDER_ON_FINISH_RACE: Callee = Callee {
    addr: 0x0012_5108,
    f: ee::s00::Rider_OnFinishRace,
};

pub(crate) const AI_ALL_PLAYERS_FINISHED: Callee = Callee {
    addr: 0x0012_a250,
    f: ee::s00::AI_AllPlayersFinished,
};

/// Force every rider that has not finished (finish time < 0) to finish
/// (`Rider_OnFinishRace` with the forced flag set). s2 counts, s0 walks the
/// actors, s3 = 1, f20 = 0. Loop head `head`, call return `ret`.
pub(crate) fn ai_force_finish_all(c: &mut Cpu, head: u32, ret: u32) -> crate::call::Flow {
    let this = Addr(c.g[S1][0] as u32);
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(ACTOR_RIDER).lw(c);
        let t = Addr(c.g[V0][0] as u32).off(RIDER_FINISH_TIME).read_fpu(c);
        abi::set_fpr(c, 0, t);
        let finished = FpuF32(c.f[20]).le(t);
        abi::set_fcc(c, finished);
        c.g[V1][0] = 1;
        if !finished {
            c.cycles += 1;
            c.g[V1][0] = 0;
        }
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.cycles += 5;
            Addr(c.g[V0][0] as u32)
                .off(RIDER_FORCED_FINISH)
                .sw(c, c.g[S3][0]);
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(ACTOR_RIDER).lw(c);
            RIDER_ON_FINISH_RACE.call(c, ret)?;
        }
        c.g[V0][0] = this.off(AI_RIDER_COUNT).lw(c);
        c.cycles += 4;
        c.g[S2][0] = addu(c.g[S2][0], 1);
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S0][0] = addu(c.g[S0][0], 4);
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// What AI_UpdateRaceTimeLimit's time-limit checks end with.
#[derive(Clone, Copy)]
pub(crate) enum Limit {
    /// The unfinished riders were forced to finish: stop the replay.
    Stop,
    /// Nothing more to do (one join cycle).
    Join,
    /// Straight to the replay check.
    Tail,
}

/// `void AI_UpdateRaceTimeLimit(AI* self)` (0x12bb20): the race time limit.
/// Every second (60 ticks) a sync checksum is taken. While racing (state 5,
/// or 3 with the session in state 3) in race modes 0 and 4: once someone
/// finishes before tick 0xd32b a countdown starts (up to 2 minutes, never
/// past the limit); when it runs out, or at the hard limit 0xd3a4 (if not
/// everyone has finished), every unfinished rider is forced to finish, the
/// replay stops recording and the countdown is marked done (-1). Then,
/// unless the replay is in states 1-9, the sync check is sent every second.
pub fn ai_update_race_time_limit(c: &mut Cpu) {
    /// How the function leaves: through the s0 and s1 reloads (one cycle
    /// each), the s1 reload only (s0 was reloaded on the way), or neither.
    enum Exit {
        Full,
        S1Only,
        Plain,
    }
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x70);
        let gp = c.g[GP][0] as u32;
        let world_slot = Addr(gp.wrapping_sub(GP_WORLD));
        c.g[V0][0] = world_slot.lw(c);
        frame.save(c, S1, 0x40);
        c.g[A1][0] = 7;
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x60);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x78).lw(c);
        call_ptr(c, BE_GET_INTERFACE, 0x0012_bb54)?;
        let this = Addr(c.g[S1][0] as u32);
        c.g[V1][0] = 0x53_0000;
        c.g[V0][0] = Addr(RACE_ACTIVE).lw(c);
        let active = c.g[V0][0] != 0;
        c.g[V0][0] = 0x3c;
        let exit = 'body: {
            if !active {
                break 'body Exit::Full;
            }
            c.cycles += 3;
            c.g[A0][0] = this.off(AI_TIMER).lw(c);
            c.cycles += 4;
            c.div(0, A0, V0);
            c.g[V1][0] = c.hi[0];
            c.g[S4][0] = 0; // sync checksum
            if c.g[V1][0] == 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                AI_GET_SYNC_CHECKSUM.call(c, 0x0012_bb88)?;
                c.g[S4][0] = c.g[V0][0];
            }
            c.cycles += 4;
            c.g[A1][0] = this.off(AI_STATE).lw(c);
            c.g[V0][0] = 5;
            let racing = c.g[A1][0] == 5;
            c.g[V0][0] = 3;
            if !racing {
                c.cycles += 2;
                let waiting = c.g[A1][0] == 3;
                frame.restore(c, S0, 0x50);
                if !waiting {
                    break 'body Exit::S1Only;
                }
                c.cycles += 5;
                c.g[V0][0] = world_slot.lw(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(SESSION_STATE).lw(c);
                if c.g[A0][0] != c.g[A1][0] {
                    frame.restore(c, S1, 0x40);
                    break 'body Exit::Plain;
                }
            }
            c.cycles += 9;
            call_ptr(c, BE_GET_INSTANCE, 0x0012_bbc0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, BE_GET_INTERFACE, 0x0012_bbcc)?;
            c.g[V1][0] = 0x53_0000;
            c.g[V1][0] = Addr(RACE_MODE).lb(c);
            let mut timed_mode = c.g[V1][0] == 0;
            c.g[V0][0] = 4;
            if !timed_mode {
                c.cycles += 2;
                timed_mode = c.g[V1][0] == 4;
                c.g[V0][0] = world_slot.lw(c);
            }
            let limit = if !timed_mode {
                Limit::Tail
            } else {
                c.cycles += 3;
                c.g[V0][0] = this.off(AI_COUNTDOWN).lw(c);
                if (c.g[V0][0] as i64) >= 0 {
                    ai_countdown(c, this)?
                } else {
                    ai_time_limit(c, this, gp)?
                }
            };
            match limit {
                Limit::Tail => {}
                Limit::Stop | Limit::Join => {
                    if let Limit::Stop = limit {
                        // Time is up: stop the replay, countdown done.
                        c.cycles += 5;
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(SESSION_REPLAY).lw(c);
                        call_ptr(c, REPLAY_STOP_RECORDING, 0x0012_bda4)?;
                        c.g[V0][0] = u64::MAX;
                        this.off(AI_COUNTDOWN).sw(c, c.g[V0][0]);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = world_slot.lw(c);
                }
            }
            // Send the sync check every second unless the replay is busy
            // (states 1-9).
            c.cycles += 5;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(SESSION_REPLAY).lw(c);
            c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[A1][0] = 0;
            if c.g[A0][0] != 0 {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10) as u64;
            }
            c.cycles += 2;
            let busy = c.g[A1][0] != 0;
            frame.restore(c, S0, 0x50);
            if busy {
                break 'body Exit::S1Only;
            }
            c.cycles += 4;
            c.g[A1][0] = this.off(AI_TIMER).lw(c);
            c.g[V0][0] = 0x3c;
            c.cycles += 4;
            c.div(0, A1, V0);
            c.g[V1][0] = c.hi[0];
            frame.restore(c, S0, 0x50);
            if c.g[V1][0] != 0 {
                break 'body Exit::S1Only;
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(gp.wrapping_sub(GP_GAME_COMM)).lw(c);
            c.g[A2][0] = c.g[S4][0];
            call_ptr(c, GAME_COMM_SEND_SYNC_CHECK, 0x0012_bdfc)?;
            Exit::Full
        };
        match exit {
            Exit::Full => {
                c.cycles += 1;
                frame.restore(c, S0, 0x50);
                c.cycles += 1;
                frame.restore(c, S1, 0x40);
            }
            Exit::S1Only => {
                c.cycles += 1;
                frame.restore(c, S1, 0x40);
            }
            Exit::Plain => {}
        }
        c.cycles += 7;
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// The running countdown (v0 = countdown >= 0): count it down; at zero
/// force everyone to finish.
pub(crate) fn ai_countdown(c: &mut Cpu, this: Addr) -> Result<Limit, crate::call::Unwound> {
    c.cycles += 2;
    let idle = (c.g[V0][0] as i64) <= 0;
    c.g[V0][0] = addu(c.g[V0][0], (-1i64) as u64);
    if idle {
        return Ok(Limit::Join);
    }
    c.cycles += 2;
    let left = (c.g[V0][0] as i64) > 0;
    this.off(AI_COUNTDOWN).sw(c, c.g[V0][0]);
    if left {
        return Ok(Limit::Join);
    }
    c.cycles += 3;
    c.g[V0][0] = this.off(AI_RIDER_COUNT).lw(c);
    c.g[S2][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 3;
        c.f[20] = 0;
        c.g[S3][0] = 1;
        c.g[S0][0] = addu(c.g[S1][0], AI_RIDERS as u64);
        ai_force_finish_all(c, 0x0012_bc18, 0x0012_bc50)?;
        c.cycles += 2;
    } else {
        c.cycles += 1;
    }
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_sub(GP_WORLD)).lw(c);
    Ok(Limit::Stop)
}

/// No countdown yet (v0 < 0): start one when someone has finished before
/// tick 0xd32b; at the hard limit force everyone to finish.
pub(crate) fn ai_time_limit(
    c: &mut Cpu,
    this: Addr,
    gp: u32,
) -> Result<Limit, crate::call::Unwound> {
    let world_slot = Addr(gp.wrapping_sub(GP_WORLD));
    c.g[V1][0] = this.off(AI_TIMER).lw(c);
    c.cycles += 4;
    c.g[V0][0] = (LAST_COUNTDOWN_START < c.g[V1][0] as i64) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        AI_ALL_PLAYERS_FINISHED.call(c, 0x0012_bc84)?;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = this.off(AI_RIDER_COUNT).lw(c);
            if (c.g[V0][0] as i64) > 0 {
                ai_start_countdown(c, this, gp);
                c.cycles += 1;
            }
        }
        c.g[V1][0] = this.off(AI_TIMER).lw(c);
    }
    // The hard limit.
    c.cycles += 4;
    c.g[V0][0] = ((TIME_LIMIT as i64 - 1) < c.g[V1][0] as i64) as u64;
    let over = c.g[V0][0] != 0;
    c.g[V0][0] = world_slot.lw(c);
    if !over {
        return Ok(Limit::Tail);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    AI_ALL_PLAYERS_FINISHED.call(c, 0x0012_bd28)?;
    let all = c.g[V0][0] != 0;
    c.g[V0][0] = world_slot.lw(c);
    if all {
        return Ok(Limit::Tail);
    }
    c.cycles += 3;
    c.g[V0][0] = this.off(AI_RIDER_COUNT).lw(c);
    c.g[S2][0] = 0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 3;
        c.f[20] = 0;
        c.g[S3][0] = 1;
        c.g[S0][0] = addu(c.g[S1][0], AI_RIDERS as u64);
        ai_force_finish_all(c, 0x0012_bd48, 0x0012_bd80)?;
    }
    c.cycles += 1;
    c.g[V0][0] = world_slot.lw(c);
    Ok(Limit::Stop)
}

/// For each finished rider (while the game comm is not in a menu or
/// paused, +0x8c/+0x48 both 0) set the countdown to the time left until the
/// limit, at most 2 minutes. v0 = rider count.
pub(crate) fn ai_start_countdown(c: &mut Cpu, this: Addr, gp: u32) {
    c.cycles += 6;
    c.f[1] = 0;
    c.g[A2][0] = c.g[V0][0];
    c.g[A3][0] = Addr(gp.wrapping_sub(GP_GAME_COMM)).lw(c);
    c.g[T1][0] = TIME_LIMIT;
    c.g[T0][0] = MAX_COUNTDOWN;
    c.g[A1][0] = addu(c.g[S1][0], AI_RIDERS as u64);
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(ACTOR_RIDER).lw(c);
        let t = Addr(c.g[V1][0] as u32).off(RIDER_FINISH_TIME).read_fpu(c);
        abi::set_fpr(c, 0, t);
        let finished = FpuF32(c.f[1]).le(t);
        abi::set_fcc(c, finished);
        c.g[A0][0] = 1;
        if !finished {
            c.cycles += 1;
            c.g[A0][0] = 0;
        }
        c.cycles += 2;
        c.g[A2][0] = addu(c.g[A2][0], (-1i64) as u64);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            let comm = Addr(c.g[A3][0] as u32);
            c.g[V0][0] = comm.off(0x8c).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = comm.off(0x48).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 5;
                    c.g[V0][0] = this.off(AI_TIMER).lw(c);
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = ((c.g[V0][0] as i64) < 7201) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = c.g[T0][0];
                    }
                    this.off(AI_COUNTDOWN).sw(c, c.g[V0][0]);
                }
            }
        }
        c.cycles += 2;
        let more = c.g[A2][0] != 0;
        c.g[A1][0] = addu(c.g[A1][0], 4);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0012_bcb0);
        }
    }
}

pub(crate) mod spin_lit {

    /// 1/7: stick magnitude scale.
    pub const MAG_SCALE: i32 = -0x7380;

    /// atan2(y, 0): the value for y < 0, then the one for y >= 0.
    pub const ATAN_X0_NEG: i32 = -0x737c;

    pub const ATAN_X0_POS: i32 = -0x7378;

    /// atan2 quadrant fix-ups (+pi / -pi).
    pub const PI_ADD: i32 = -0x7374;

    pub const PI_SUB: i32 = -0x7370;

    /// Dead zone 0.3 and range 0.7 of the magnitude.
    pub const DEAD_ZONE: i32 = -0x736c;

    pub const RANGE: i32 = -0x7368;

    /// pi/4 (the diagonal) and 1/0.7.
    pub const DIAGONAL: i32 = -0x7364;

    pub const RANGE_INV: i32 = -0x7360;
}

/// `float RiderSpinSub_GetSpinFactor(RiderSpinSub *this)` (0x135180): how
/// far the rider's stick (input at `this->input` +0x58, x at +0x2dc, y at
/// +0x2e0) is pushed along the diagonal: `clamp(|stick|/7 - 0.3, 0, 0.7) /
/// 0.7 * (1 - |atan2(|y|, |x|) - pi/4| / (pi/4))` (atan2 inlined around
/// `Math_Atan`).
pub fn rider_spin_sub_get_spin_factor(c: &mut Cpu) {
    use spin_lit::*;
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.lwc1(2, gp_at(c, MAG_SCALE));
        frame.save_fpr(c, 23, 0x28);
        frame.save_fpr(c, 22, 0x20);
        frame.save_fpr(c, 21, 0x18);
        frame.save_fpr(c, 20, 0x10);
        frame.save_ra(c, 0);
        c.mtc1(0, 22); // f22 = 0
        c.g[V0][0] = Addr(abi::arg(c, 0).wrapping_add(0x58)).lw(c);
        let input = r32(c, V0);
        c.lwc1(1, input.wrapping_add(0x2e0));
        c.lwc1(0, input.wrapping_add(0x2dc));
        c.abs_s(21, 1); // |y|
        c.abs_s(20, 0); // |x|
        c.mul_s(1, 1, 1);
        c.mul_s(0, 0, 0);
        c.c_eq_s(20, 22);
        c.add_s(0, 0, 1);
        c.sqrt_s(0, 0);
        let x_zero = c.fcc();
        c.mul_s(23, 0, 2); // magnitude / 7

        // f12 = atan2(|y|, |x|).
        if x_zero {
            c.cycles += 4;
            c.c_eq_s(21, 22);
            let y_zero = c.fcc();
            c.mov_s(12, 21);
            if !y_zero {
                c.cycles += 4;
                c.c_le_s(22, 21);
                c.lwc1(12, gp_at(c, ATAN_X0_NEG));
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(12, gp_at(c, ATAN_X0_POS));
                }
            }
        } else {
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, 0x0031_c228, 0x0013_5208)?; // Math_Atan(|y|/|x|)
            c.c_lt_s(20, 22);
            let x_neg = c.fcc();
            c.mov_s(1, 0);
            if x_neg {
                c.cycles += 3;
                c.c_lt_s(22, 21);
                if c.fcc() {
                    c.cycles += 3;
                    c.lwc1(0, gp_at(c, PI_ADD));
                    c.add_s(1, 1, 0);
                } else {
                    c.cycles += 2;
                    c.lwc1(0, gp_at(c, PI_SUB));
                    c.sub_s(1, 1, 0);
                }
            }
            c.cycles += 1;
            c.mov_s(12, 1);
        }

        // f4 = clamp(mag - 0.3, 0, 0.7).
        c.cycles += 8;
        c.lwc1(0, gp_at(c, DEAD_ZONE));
        c.mtc1(0, 1);
        c.sub_s(0, 23, 0);
        c.lwc1(2, gp_at(c, RANGE));
        c.c_le_s(1, 0);
        let above = c.fcc();
        c.mov_s(4, 1);
        if above {
            c.cycles += 1;
            c.min_s(4, 0, 2);
        }

        // f0 = f4 / 0.7 * (1 - |angle - pi/4| / (pi/4)).
        c.cycles += 17;
        c.lwc1(3, gp_at(c, DIAGONAL));
        c.lwc1(2, gp_at(c, RANGE_INV));
        c.sub_s(1, 12, 3);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.abs_s(1, 1);
        c.mul_s(2, 4, 2);
        c.div_s(1, 1, 3);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x28);
        frame.restore_fpr(c, 22, 0x20);
        frame.restore_fpr(c, 21, 0x18);
        frame.restore_fpr(c, 20, 0x10);
        c.sub_s(0, 0, 1);
        c.mul_s(0, 2, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderSpinSub_GetSpinProgress (0x135db0)

/// RiderSpinSub: spin start angle.
pub(crate) const SPIN_START: u32 = 0x10;

/// Current spin angle.
pub(crate) const SPIN_ANGLE: u32 = 0x18;

/// Spin rate (radians per tick).
pub(crate) const SPIN_RATE: u32 = 0x3c;

/// The rider.
pub(crate) const SPIN_RIDER: u32 = 0x58;

/// Rider: a float that, when positive, halves the spin progress scale.
pub(crate) const RIDER_SPIN_HALVE: u32 = 0x2ec;

/// Rider: player index and character (for the behaviour attribute).
pub(crate) const RIDER_PLAYER: u32 = 0x86c;

pub(crate) const RIDER_CHARACTER: u32 = 0xb34;

/// `.lit4`: 2*pi, ~0.5 (0.50006247), the attribute factor scale (~0.5003).
pub(crate) const GP_SPIN_TWO_PI: u32 = 0x7324;

pub(crate) const GP_SPIN_HALF: u32 = 0x7320;

pub(crate) const GP_SPIN_ATTRIB_SCALE: u32 = 0x731c;

pub(crate) const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_dc80,
    f: ee::s01::BELibrary_GetInstance,
};

pub(crate) const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
    addr: 0x0014_dd58,
    f: ee::s01::BELibrary_GetInterface,
};

pub(crate) const BE_CHAR_INT_GET_ATTRIB4_FACTOR: Callee = Callee {
    addr: 0x0014_95a8,
    f: ee::s01::BECharInt_GetAttrib4Factor,
};

/// `void RiderSpinSub::GetSpinProgress(RiderSpinSub* spin, float* done,
/// float* full)` (0x135db0): how far the current spin has turned in ticks
/// (`|angle - start|`, less one turn past 2*pi, over the rate) and how many
/// ticks a full turn takes (2*pi over the rate); both halved (x0.50006) when
/// the rider's +0x2ec is positive, then divided by `1 + 0.50026 *` the
/// character's spin attribute (`BECharInt_GetAttrib4Factor`).
pub fn rider_spin_sub_get_spin_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        let gp = r32(c, GP);
        let two_pi = Addr(gp.wrapping_sub(GP_SPIN_TWO_PI)).read_fpu(c);
        set_fpr(c, 3, two_pi);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        set_reg(c, S0, reg(c, A0));
        frame.save(c, S2, 0x10);
        set_reg(c, S1, reg(c, A1));
        frame.save_ra(c, 0);
        set_reg(c, S2, reg(c, A2));
        let spin = r32(c, S0);
        let (done, full) = (Addr(r32(c, S1)), Addr(r32(c, S2)));
        let start = Addr(spin.wrapping_add(SPIN_START)).read_fpu(c);
        let angle = Addr(spin.wrapping_add(SPIN_ANGLE)).read_fpu(c);
        let diff = angle - start;
        set_fpr(c, 0, diff);
        let mut turned = diff.abs();
        let over = two_pi.lt(turned);
        set_fcc(c, over);
        if over {
            turned = turned - two_pi;
        }
        set_fpr(c, 1, turned);

        c.cycles += 12;
        let rate = Addr(spin.wrapping_add(SPIN_RATE)).read_fpu(c);
        let zero = FpuF32::ZERO;
        set_fpr(c, 2, zero);
        let d = turned.div(rate);
        set_fpr(c, 0, d);
        done.write_fpu(c, d);
        let rate = Addr(spin.wrapping_add(SPIN_RATE)).read_fpu(c);
        let f = two_pi.div(rate);
        set_fpr(c, 1, f);
        full.write_fpu(c, f);
        let rider = Addr(spin.wrapping_add(SPIN_RIDER)).lw(c);
        set_reg(c, V0, rider);
        let halve = Addr((rider as u32).wrapping_add(RIDER_SPIN_HALVE)).read_fpu(c);
        set_fpr(c, 0, halve);
        let pos = zero.lt(halve);
        set_fcc(c, pos);
        if pos {
            c.cycles += 6;
            let h = Addr(gp.wrapping_sub(GP_SPIN_HALF)).read_fpu(c);
            set_fpr(c, 2, h);
            let f = f * h;
            set_fpr(c, 1, f);
            full.write_fpu(c, f);
            let d = done.read_fpu(c) * h;
            set_fpr(c, 0, d);
            done.write_fpu(c, d);
        }

        // The character's spin attribute.
        c.cycles += 28;
        BE_LIBRARY_GET_INSTANCE.call(c, 0x0013_5e44)?;
        set_reg(c, A0, reg(c, V0));
        set_reg(c, A1, 3);
        BE_LIBRARY_GET_INTERFACE.call(c, 0x0013_5e50)?;
        let rider = Addr(spin.wrapping_add(SPIN_RIDER)).lw(c);
        set_reg(c, V1, rider);
        set_reg(c, A0, reg(c, V0));
        let character = Addr((rider as u32).wrapping_add(RIDER_CHARACTER)).lw(c);
        set_reg(c, A2, character);
        let player = Addr((rider as u32).wrapping_add(RIDER_PLAYER)).lw(c);
        set_reg(c, A1, player);
        BE_CHAR_INT_GET_ATTRIB4_FACTOR.call(c, 0x0013_5e64)?;

        // Both divided by 1 + attribute * scale.
        let scale = Addr(gp.wrapping_sub(GP_SPIN_ATTRIB_SCALE)).read_fpu(c);
        set_fpr(c, 3, scale);
        set_reg(c, AT, 0x3f80_0000);
        let attrib = FpuF32(c.f[0]) * scale;
        let f = full.read_fpu(c);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0);
        let div = attrib + FpuF32::ONE;
        set_fpr(c, 0, div);
        let m = FpuF32::ONE.div(div);
        set_fpr(c, 2, m);
        let f = f * m;
        set_fpr(c, 1, f);
        full.write_fpu(c, f);
        frame.restore(c, S2, 0x10);
        let d = done.read_fpu(c) * m;
        set_fpr(c, 0, d);
        done.write_fpu(c, d);
        frame.restore(c, S1, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderPrewindSub_GetHUDAngle (0x12f118)

/// Rider (through the sub-state's first word): the stick's y and x.
pub(crate) const RIDER_STICK_Y: u32 = 0x2b0;

pub(crate) const RIDER_STICK_X: u32 = 0x2a4;

/// `.lit4`: the dead zone (0.1), the angles for x == 0 (y < 0, y >= 0), the
/// corrections for x < 0 (y > 0: added; else subtracted).
pub(crate) const GP_HUD_DEAD_ZONE: u32 = 0x7730;

pub(crate) const GP_HUD_DOWN: u32 = 0x772c;

pub(crate) const GP_HUD_UP: u32 = 0x7728;

pub(crate) const GP_HUD_ADD_PI: u32 = 0x7724;

pub(crate) const GP_HUD_SUB_PI: u32 = 0x7720;

/// `Math_Atan` (cross-shard, through the table): f0 = atan(f12).
pub(crate) const MATH_ATAN_2: u32 = 0x0031_c228;

/// `float RiderPrewindSub::GetHUDAngle(RiderPrewindSub* sub, int* centred)`
/// (0x12f118): the stick direction for the prewind HUD. Each stick axis
/// under 0.1 counts as 0; with both 0, `*centred = 1` and 0 is returned.
/// Otherwise `*centred = 0` and the angle is `atan2(y, x)` built from
/// `Math_Atan(y / x)` with the quadrant corrections (x == 0: the up or down
/// angle).
pub fn rider_prewind_sub_get_hud_angle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let rider = Addr(r32(c, A0)).lw(c);
        set_reg(c, V0, rider);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x20);
        frame.save_fpr(c, 21, 0x18);
        frame.save_fpr(c, 20, 0x10);
        let gp = r32(c, GP);
        let lit = |c: &mut Cpu, o: u32| Addr(gp.wrapping_sub(o)).read_fpu(c);
        let dead = lit(c, GP_HUD_DEAD_ZONE);
        set_fpr(c, 1, dead);
        let rider = rider as u32;
        let mut y = Addr(rider.wrapping_add(RIDER_STICK_Y)).read_fpu(c);
        set_fpr(c, 0, y.abs());
        let small = abi::c_lt_s(c, y.abs(), dead);
        let mut x = Addr(rider.wrapping_add(RIDER_STICK_X)).read_fpu(c);
        let zero = FpuF32::ZERO;
        if small {
            c.cycles += 1;
            y = zero;
        }
        c.cycles += 4;
        set_fpr(c, 0, x.abs());
        if abi::c_lt_s(c, x.abs(), dead) {
            c.cycles += 1;
            x = zero;
        }
        set_fpr(c, 20, y);
        set_fpr(c, 21, x);

        // Centred: both axes zero.
        c.cycles += 5;
        set_fpr(c, 0, zero);
        let mut centred = 0u64;
        let y_zero = y.eq_s(zero);
        set_fcc(c, y_zero);
        if y_zero {
            c.cycles += 4;
            let x_zero = x.eq_s(zero);
            set_fcc(c, x_zero);
            if x_zero {
                centred = 1;
            }
        }
        set_reg(c, V0, centred);
        c.cycles += 2;
        Addr(r32(c, A1)).sw(c, centred);
        if centred != 0 {
            c.cycles += 3;
            set_fpr(c, 0, zero);
            frame.restore_ra(c, 0);
            c.cycles += 1;
            frame.restore_fpr(c, 22, 0x20);
        } else {
            c.cycles += 4;
            set_fpr(c, 22, zero);
            let x_zero = x.eq_s(zero);
            set_fcc(c, x_zero);
            if x_zero {
                // Straight up or down.
                c.cycles += 4;
                let y_zero = y.eq_s(zero);
                set_fcc(c, y_zero);
                set_fpr(c, 0, y);
                if y_zero {
                    c.cycles += 1;
                    frame.restore_ra(c, 0);
                    c.cycles += 1;
                    frame.restore_fpr(c, 22, 0x20);
                } else {
                    c.cycles += 4;
                    let up = zero.le(y);
                    set_fcc(c, up);
                    let down = lit(c, GP_HUD_DOWN);
                    set_fpr(c, 0, down);
                    frame.restore_ra(c, 0);
                    if !up {
                        c.cycles += 1;
                        frame.restore_ra(c, 0);
                        c.cycles += 1;
                        frame.restore_fpr(c, 22, 0x20);
                    } else {
                        c.cycles += 3;
                        let up = lit(c, GP_HUD_UP);
                        set_fpr(c, 0, up);
                        frame.restore_fpr(c, 22, 0x20);
                    }
                }
            } else {
                // atan(y / x), corrected into the left half-plane.
                c.cycles += 7;
                set_fpr(c, 12, y.div(x));
                call_ptr(c, MATH_ATAN_2, 0x0012_f1e4)?;
                let left = abi::c_lt_s(c, x, zero);
                let mut angle = FpuF32(c.f[0]);
                set_fpr(c, 12, angle);
                if left {
                    c.cycles += 3;
                    if abi::c_lt_s(c, zero, y) {
                        c.cycles += 3;
                        let k = lit(c, GP_HUD_ADD_PI);
                        set_fpr(c, 0, k);
                        angle = angle + k;
                    } else {
                        c.cycles += 2;
                        let k = lit(c, GP_HUD_SUB_PI);
                        set_fpr(c, 0, k);
                        angle = angle - k;
                    }
                    set_fpr(c, 12, angle);
                }
                c.cycles += 1;
                set_fpr(c, 0, angle);
                c.cycles += 1;
                frame.restore_ra(c, 0);
                c.cycles += 1;
                frame.restore_fpr(c, 22, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore_fpr(c, 21, 0x18);
        frame.restore_fpr(c, 20, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderSpinSub_GetFlipProgress(RiderSpinSub* self, float* done,
/// float* total)` (0x135cb0): how far the current flip has turned. `done`
/// is the flip angle covered (`|end - start|`, folded below pi) over the
/// flip rate, `total` pi over the rate; both halved (times ~0.5) when the
/// rider's flip boost (+0x2ec) is positive, then scaled by
/// `1 / (1 + attrib4 * ~0.5)` from the rider's character attribute 4.
pub fn rider_spin_sub_get_flip_progress(c: &mut Cpu) {
    /// `RiderSpinSub` fields.
    const FLIP_START: u32 = 0x14;
    const FLIP_END: u32 = 0x1c;
    const FLIP_RATE: u32 = 0x38;
    const RIDER: u32 = 0x58;
    /// Rider fields.
    const RIDER_FLIP_BOOST: u32 = 0x2ec;
    const RIDER_INDEX: u32 = 0x86c;
    const RIDER_ATTRIB4: u32 = 0xb34;
    /// `.lit4` constants: pi, the boost scale, the attribute scale.
    const PI_GP: u32 = 0x7330;
    const BOOST_GP: u32 = 0x732c;
    const ATTRIB_GP: u32 = 0x7328;
    /// `BELibrary` interface id of the character interface.
    const CHAR_INTERFACE: u64 = 3;
    let lit = |c: &Cpu, off: u32| r32(c, GP).wrapping_sub(off);
    let fpr = |c: &Cpu, n: usize| FpuF32(c.f[n]);
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        let pi = Addr(lit(c, PI_GP)).read_fpu(c);
        c.f[3] = pi.0;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        let done_out = r32(c, S1);
        let total_out = r32(c, S2);
        let start = Addr(this).off(FLIP_START).read_fpu(c);
        let end = Addr(this).off(FLIP_END).read_fpu(c);
        let diff = end - start;
        c.f[0] = diff.0;
        let mut angle = diff.abs();
        let wrapped = pi.lt(angle);
        abi::set_fcc(c, wrapped);
        if wrapped {
            angle = angle - pi;
        }
        c.f[1] = angle.0;

        c.cycles += 12;
        let rate = Addr(this).off(FLIP_RATE).read_fpu(c);
        c.f[2] = 0;
        let done = angle.div(rate);
        c.f[0] = done.0;
        Addr(done_out).write_fpu(c, done);
        let rate = Addr(this).off(FLIP_RATE).read_fpu(c);
        let total = pi.div(rate);
        c.f[1] = total.0;
        Addr(total_out).write_fpu(c, total);
        let rider = Addr(this).off(RIDER).lw(c);
        c.g[V0][0] = rider;
        let boost = Addr(rider as u32).off(RIDER_FLIP_BOOST).read_fpu(c);
        c.f[0] = boost.0;
        let boosted = FpuF32(0).lt(boost);
        abi::set_fcc(c, boosted);
        if boosted {
            c.cycles += 6;
            let half = Addr(lit(c, BOOST_GP)).read_fpu(c);
            c.f[2] = half.0;
            let total = fpr(c, 1) * half;
            c.f[1] = total.0;
            Addr(total_out).write_fpu(c, total);
            let done = Addr(done_out).read_fpu(c) * half;
            c.f[0] = done.0;
            Addr(done_out).write_fpu(c, done);
        }

        c.cycles += 28;
        BE_LIBRARY_GET_INSTANCE.call(c, 0x0013_5d44)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = CHAR_INTERFACE;
        BE_LIBRARY_GET_INTERFACE.call(c, 0x0013_5d50)?;
        let rider = Addr(this).off(RIDER).lw(c);
        c.g[V1][0] = rider;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = Addr(rider as u32).off(RIDER_ATTRIB4).lw(c);
        c.g[A1][0] = Addr(rider as u32).off(RIDER_INDEX).lw(c);
        BE_CHAR_INT_GET_ATTRIB4_FACTOR.call(c, 0x0013_5d64)?;
        let k = Addr(lit(c, ATTRIB_GP)).read_fpu(c);
        c.f[3] = k.0;
        c.g[AT][0] = 0x3f80_0000;
        let one = FpuF32(0x3f80_0000);
        let scaled = fpr(c, 0) * k;
        let total = Addr(total_out).read_fpu(c);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0);
        let denom = scaled + one;
        c.f[0] = denom.0;
        let m = one.div(denom);
        c.f[2] = m.0;
        let total = total * m;
        c.f[1] = total.0;
        Addr(total_out).write_fpu(c, total);
        frame.restore(c, S2, 0x10);
        let done = Addr(done_out).read_fpu(c) * m;
        c.f[0] = done.0;
        Addr(done_out).write_fpu(c, done);
        frame.restore(c, S1, 0x20);
        frame.close(c);
        Ok(())
    });
}

pub(crate) mod wipeout_input {
    use super::*;

    pub const RIDER_HANDLE_BOOST_INPUT: Callee = Callee {
        addr: 0x0011_4130,
        f: ee::s00::f_00114130,
    };

    pub const RIDER_CHECK_RESET_PATH: Callee = Callee {
        addr: 0x0011_6120,
        f: ee::s00::f_00116120,
    };

    pub const UPDATE_PHASE0: Callee = Callee {
        addr: 0x0012_cd20,
        f: ee::s00::f_0012cd20,
    };

    pub const UPDATE_PHASE1: Callee = Callee {
        addr: 0x0012_d4e8,
        f: ee::s00::f_0012d4e8,
    };

    pub const UPDATE_PHASE2: Callee = Callee {
        addr: 0x0012_d160,
        f: ee::s00::f_0012d160,
    };

    pub const UPDATE_RECOVER: Callee = Callee {
        addr: 0x0012_d848,
        f: ee::s00::f_0012d848,
    };

    pub const UPDATE_PHASE4: Callee = Callee {
        addr: 0x0012_d9d8,
        f: ee::s00::f_0012d9d8,
    };

    /// `SSXAudioSystem_GetInstance` (table call).
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;

    /// `SSXAudioSystem_PlayWipeoutSlideSfx(audio, rider)` (table call).
    pub const AUDIO_SLIDE_SFX: u32 = 0x0029_e970;

    /// `SSXAudioSystem_PlayWipeoutImpactSfx(audio, rider, amount)` (table call).
    pub const AUDIO_IMPACT_SFX: u32 = 0x0029_e590;

    /// Wipeout sub-state: the phase (0..4; 3 = recovering).
    pub const PHASE: u32 = 0x0;

    /// Wipeout sub-state: the slide amount (decays toward 0, the slide
    /// button pushes it up).
    pub const SLIDE: u32 = 0x70;

    /// Wipeout sub-state: the rider.
    pub const RIDER: u32 = 0x80;

    /// Rider: the wipeout kind (0, 2 or other) that picks the slide rate.
    pub const RIDER_WIPEOUT_KIND: u32 = 0xb20;

    /// The pad's slide button.
    pub const SLIDE_BUTTON: u64 = 0x2000;

    /// The phase switch's jump table (5 entries).
    pub const PHASE_TABLE: u32 = 0x0045_7f80;

    /// `.lit4` literals (gp-relative): slide decay per tick, 0, the slide
    /// rates (default, kind 0, kind 2) and the tick length.
    pub const GP_DECAY: u32 = 0x77dc;

    pub const GP_ZERO: u32 = 0x77d8;

    pub const GP_RATE: u32 = 0x77d4;

    pub const GP_RATE_KIND0: u32 = 0x77d0;

    pub const GP_RATE_KIND2: u32 = 0x77cc;

    pub const GP_TICK: u32 = 0x77c8;
}

/// `void RiderWipeoutSub_HandleInput(RiderWipeoutSub* this, Pad* pad)`
/// (0x12cb68): per-tick input of a wiped-out rider. Handles the boost input
/// (no boost), lets the reset button reset the rider onto the path (then
/// returns), otherwise (unless recovering, phase 3) decays the slide amount
/// toward 0 and, while the slide button is held, pushes it up at the wipeout
/// kind's rate and plays the slide sound (once it reaches 1) or the impact
/// sound; finally runs the current phase's update.
pub fn rider_wipeout_sub_handle_input(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    use wipeout_input::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        // Rider_HandleBoostInput(rider, 0, 0).
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(RIDER)).lw(c);
        RIDER_HANDLE_BOOST_INPUT.call(c, 0x0012_cb90)?;
        // Rider_CheckResetPath(rider, pad bit 12, 0).
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A2][0] = 0;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(RIDER)).lw(c);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 12) as u32) & 0x1;
        RIDER_CHECK_RESET_PATH.call(c, 0x0012_cba8)?;
        'epilogue: {
            if c.g[V0][0] != 0 {
                // Reset onto the path: nothing else this tick.
                frame.restore(c, S0, 0x20);
                break 'epilogue;
            }
            c.cycles += 4;
            let gp = r32(c, GP);
            let lit = |off: u32| Addr(gp.wrapping_sub(off));
            let this = r32(c, S0);
            c.g[V0][0] = Addr(this.wrapping_add(PHASE)).lw(c) ^ 0x3;
            if c.g[V0][0] != 0 {
                // Decay the slide amount toward 0 by one step.
                c.cycles += 5;
                let slide = Addr(this.wrapping_add(SLIDE)).read_fpu(c);
                let step = lit(GP_DECAY).read_fpu(c);
                abi::set_fpr(c, 2, slide);
                abi::set_fpr(c, 3, step);
                if abi::c_lt_s(c, step, slide) {
                    c.cycles += 2;
                    abi::set_fpr(c, 1, slide - step);
                } else {
                    c.cycles += 6;
                    let zero = lit(GP_ZERO).read_fpu(c);
                    abi::set_fpr(c, 0, zero);
                    abi::set_fpr(c, 1, FpuF32::ZERO);
                    if abi::c_lt_s(c, slide, zero) {
                        abi::set_fpr(c, 1, slide + step);
                    }
                }
                c.cycles += 5;
                let decayed = FpuF32(c.f[1]);
                Addr(this.wrapping_add(SLIDE)).write_fpu(c, decayed);
                c.g[V0][0] = Addr(r32(c, S1)).lw(c) & SLIDE_BUTTON;
                if c.g[V0][0] != 0 {
                    // The slide rate by wipeout kind.
                    c.cycles += 5;
                    c.g[V0][0] = Addr(this.wrapping_add(RIDER)).lw(c);
                    let mut rate = lit(GP_RATE).read_fpu(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(RIDER_WIPEOUT_KIND)).lw(c);
                    c.g[V0][0] = 0x2;
                    if c.g[V1][0] == 0 {
                        c.cycles += 3;
                        rate = lit(GP_RATE_KIND0).read_fpu(c);
                    } else if c.g[V1][0] == c.g[V0][0] {
                        c.cycles += 3;
                        rate = lit(GP_RATE_KIND2).read_fpu(c);
                    } else {
                        c.cycles += 4;
                    }
                    abi::set_fpr(c, 2, rate);
                    // slide += rate * tick.
                    c.cycles += 10;
                    let tick = lit(GP_TICK).read_fpu(c);
                    let slide = Addr(this.wrapping_add(SLIDE)).read_fpu(c);
                    let push = rate * tick;
                    abi::set_fpr(c, 0, push);
                    c.g[AT][0] = 0x3f80_0000;
                    abi::set_fpr(c, 2, FpuF32::ONE);
                    let pushed = slide + push;
                    abi::set_fpr(c, 1, pushed);
                    let full = FpuF32::ONE.le(pushed);
                    abi::set_fcc(c, full);
                    Addr(this.wrapping_add(SLIDE)).write_fpu(c, pushed);
                    c.cycles += 7;
                    if full {
                        call_ptr(c, AUDIO_GET_INSTANCE, 0x0012_cc6c)?;
                        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(RIDER)).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, AUDIO_SLIDE_SFX, 0x0012_cc78)?;
                    } else {
                        call_ptr(c, AUDIO_GET_INSTANCE, 0x0012_cc88)?;
                        let this = r32(c, S0);
                        c.g[A1][0] = Addr(this.wrapping_add(RIDER)).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        c.lwc1(12, this.wrapping_add(SLIDE));
                        call_ptr(c, AUDIO_IMPACT_SFX, 0x0012_cc98)?;
                    }
                }
            }
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(PHASE)).lw(c);
            // switch (phase).
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < 0x5) as u64;
            let in_range = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0045_0000;
            if !in_range {
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
                break 'epilogue;
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(PHASE_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(PHASE_TABLE));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            let (update, ret) = match target {
                0x0012_ccc0 => (UPDATE_PHASE0, 0x0012_ccc8),
                0x0012_ccd0 => (UPDATE_PHASE1, 0x0012_ccd8),
                0x0012_cce0 => (UPDATE_PHASE2, 0x0012_cce8),
                0x0012_ccf0 => (UPDATE_RECOVER, 0x0012_ccf8),
                0x0012_cd00 => (UPDATE_PHASE4, 0x0012_cd08),
                _ => {
                    c.jump_out(target, 0x0012_cb68, 0x0012_cd1c);
                    return Ok(());
                }
            };
            c.cycles += if target == 0x0012_cd00 { 2 } else { 4 };
            c.g[A0][0] = c.g[S0][0];
            update.call(c, ret)?;
            if target == 0x0012_cd00 {
                c.cycles += 1;
            }
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderBumpSub_HandleInput (0x12e778)

/// Bump sub-state: the rider (`*this`).
pub(crate) const BUMP_RIDER: u32 = 0x0;

/// Pad: button / axis word (bit 12 reset path, 13/14 boost input, bits
/// 21..26 and 15..20 the 6-bit stick axes).
pub(crate) const BUMP_PAD_BUTTONS: u32 = 0x0;

/// Rider: velocity and facing vectors (their dot product picks the lean
/// sign).
pub(crate) const BUMP_RIDER_VELOCITY: u32 = 0x1e0;

pub(crate) const BUMP_RIDER_FACING: u32 = 0x3a0;

/// Rider floats: slew-1 target and rate.
pub(crate) const BUMP_RIDER_SLEW1_TARGET: u32 = 0x204;

pub(crate) const BUMP_RIDER_SLEW1_RATE: u32 = 0x200;

/// Rider: the owner (`RiderBase`) and the animation controller.
pub(crate) const BUMP_RIDER_OWNER: u32 = 0x77c;

pub(crate) const BUMP_RIDER_ANIM_CTRL: u32 = 0x784;

/// `gp`-relative `.lit4`: 1/31 (axis scale), again for the lean, 1/30.
pub(crate) const BUMP_AXIS_SCALE_GP: u32 = 0x77a0;

pub(crate) const BUMP_LEAN_SCALE_GP: u32 = 0x779c;

pub(crate) const BUMP_SLEW1_RATE_GP: u32 = 0x7798;

pub(crate) const BUMP_TRY_ENTER_FINISH_LINE: Callee = Callee {
    addr: 0x0011_6378,
    f: ee::s00::Rider_TryEnterFinishLine,
};

pub(crate) const BUMP_CHECK_RESET_PATH: Callee = Callee {
    addr: 0x0011_6120,
    f: ee::s00::Rider_CheckResetPath,
};

pub(crate) const BUMP_TRY_START_RAILSLIDE: Callee = Callee {
    addr: 0x0010_6848,
    f: ee::s00::Rider_TryStartRailslide,
};

pub(crate) const BUMP_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::Rider_GetMainState,
};

pub(crate) const BUMP_HANDLE_BOOST_INPUT: Callee = Callee {
    addr: 0x0011_4130,
    f: ee::s00::Rider_HandleBoostInput,
};

pub(crate) const BUMP_SET_SLEW0_TARGET: Callee = Callee {
    addr: 0x0011_3e80,
    f: ee::s00::Rider_SetSlew0Target,
};

pub(crate) const BUMP_SET_SLEW5_TARGET: Callee = Callee {
    addr: 0x0011_3f38,
    f: ee::s00::Rider_SetSlew5Target,
};

pub(crate) const BUMP_SET_SLEW3_AND4_TARGETS: Callee = Callee {
    addr: 0x0011_3f88,
    f: ee::s00::Rider_SetSlew3And4Targets,
};

pub(crate) const BUMP_RESET_GRIND_ORIENTATION: Callee = Callee {
    addr: 0x0011_5640,
    f: ee::s00::Rider_ResetGrindOrientation,
};

pub(crate) const BUMP_SET_SUB_STATE: Callee = Callee {
    addr: 0x0011_fec8,
    f: ee::s00::Rider_SetSubState,
};

/// `AnimController_IsLayerFinished(ctrl, layer)` (cross-shard).
pub(crate) const BUMP_ANIM_LAYER_DONE: u32 = 0x0031_2ae8;

/// The owner's land-animation helper (`owner + 0x2b0`, cross-shard).
pub(crate) const BUMP_PLAY_LAND_ANIM: u32 = 0x0013_26c8;

/// `void RiderBumpSub_HandleInput(RiderBumpSub* this, Pad* pad)` (0x12e778):
/// the input of a rider recovering from a bump, once per tick.
///
/// - Crossing the finish line, a path reset (pad bit 12) or a rail start
///   end the function.
/// - Airborne (main state 1): no boost, slew 0 to 0. Otherwise boost input
///   (bits 14/13); in main state 4 slew 5 follows the axis in bits 21..26
///   (/31), else slew 0 follows the axis in bits 15..20 (/31), negated when
///   the rider moves backwards (velocity . facing < 0).
/// - Slews 3/4 to 0, slew 1 to 0 at rate 1/30.
/// - When the layer-2 animation has finished: main state 4 plays the land
///   animation and enters sub-state 7; otherwise the grind orientation is
///   reset and sub-state 4 (airborne) or 0 follows.
pub fn rider_bump_sub_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        let this = Addr(r32(c, S0));
        let pad = Addr(r32(c, S1));
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        BUMP_TRY_ENTER_FINISH_LINE.call(c, 0x0012_e79c)?;
        if c.g[V0][0] != 0 {
            return bump_exit(c, frame);
        }
        c.cycles += 8;
        let buttons = pad.off(BUMP_PAD_BUTTONS).lw(c) as u32;
        c.g[A2][0] = 0;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        c.g[A1][0] = sx(((buttons as i32) >> 12) as u32) & 0x1;
        BUMP_CHECK_RESET_PATH.call(c, 0x0012_e7bc)?;
        if c.g[V0][0] != 0 {
            return bump_exit(c, frame);
        }
        c.cycles += 4;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        BUMP_TRY_START_RAILSLIDE.call(c, 0x0012_e7cc)?;
        if c.g[V0][0] != 0 {
            return bump_exit(c, frame);
        }

        c.cycles += 5;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        BUMP_GET_MAIN_STATE.call(c, 0x0012_e7dc)?;
        c.g[V1][0] = 0x1;
        if c.g[V0][0] == 1 {
            // Airborne: no boost, no lean.
            c.cycles += 9;
            c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            BUMP_HANDLE_BOOST_INPUT.call(c, 0x0012_e7f8)?;
            c.f[12] = 0;
            c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
            BUMP_SET_SLEW0_TARGET.call(c, 0x0012_e804)?;
        } else {
            let buttons = pad.off(BUMP_PAD_BUTTONS).lw(c) as u32;
            c.cycles += 11;
            c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
            c.g[A2][0] = sx(((buttons as i32) >> 13) as u32) & 0x1;
            c.g[A1][0] = sx(((buttons as i32) >> 14) as u32) & 0x1;
            BUMP_HANDLE_BOOST_INPUT.call(c, 0x0012_e824)?;
            c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
            BUMP_GET_MAIN_STATE.call(c, 0x0012_e82c)?;
            c.g[V1][0] = 0x4;
            let state4 = c.g[V0][0] == 4;
            let buttons = pad.off(BUMP_PAD_BUTTONS).lw(c) as u32;
            if state4 {
                // Slew 5 follows the axis in bits 21..26.
                c.cycles += 10;
                let gp = r32(c, GP);
                let scale = Addr(gp.wrapping_sub(BUMP_AXIS_SCALE_GP)).read_fpu(c);
                c.f[0] = scale.0;
                c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
                let axis = (((buttons << 5) as i32) >> 26) as u32;
                c.g[V0][0] = sx(axis);
                c.f[12] = axis;
                c.cvt_s_w(12, 12);
                c.mul_s(12, 12, 0);
                BUMP_SET_SLEW5_TARGET.call(c, 0x0012_e858)?;
            } else {
                // Slew 0 follows the axis in bits 15..20, signed by the
                // direction of travel.
                c.cycles += 20;
                let rider = this.off(BUMP_RIDER).lw(c);
                c.g[A0][0] = rider;
                let axis = (((buttons << 11) as i32) >> 26) as u32;
                c.f[3] = 0;
                c.g[V0][0] = sx(axis);
                let rider = rider as u32;
                c.lqc2(3, rider.wrapping_add(BUMP_RIDER_VELOCITY));
                c.lqc2(5, rider.wrapping_add(BUMP_RIDER_FACING));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(3, 4);
                c.f[1] = c.g[V1][0] as u32;
                c.f[2] = axis;
                c.cvt_s_w(2, 2);
                let gp = r32(c, GP);
                let scale = Addr(gp.wrapping_sub(BUMP_LEAN_SCALE_GP)).read_fpu(c);
                c.f[0] = scale.0;
                c.c_lt_s(1, 3);
                c.mul_s(12, 2, 0);
                if c.fcc() {
                    c.cycles += 1;
                    c.neg_s(12, 12);
                }
                c.cycles += 2;
                BUMP_SET_SLEW0_TARGET.call(c, 0x0012_e8bc)?;
            }
        }

        // Slews 3/4 and 1 to zero.
        c.cycles += 15;
        c.f[20] = 0;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        c.f[13] = c.f[20];
        c.f[12] = c.f[20];
        BUMP_SET_SLEW3_AND4_TARGETS.call(c, 0x0012_e8d0)?;
        let rider_word = this.off(BUMP_RIDER).lw(c);
        c.g[V1][0] = rider_word;
        let rider = Addr(rider_word as u32);
        c.g[A1][0] = 0x2;
        let gp = r32(c, GP);
        let rate = Addr(gp.wrapping_sub(BUMP_SLEW1_RATE_GP)).read_fpu(c);
        c.f[0] = rate.0;
        let zero = FpuF32(c.f[20]);
        rider.off(BUMP_RIDER_SLEW1_TARGET).write_fpu(c, zero);
        rider.off(BUMP_RIDER_SLEW1_RATE).write_fpu(c, rate);
        let rider = this.off(BUMP_RIDER).lw(c);
        c.g[V0][0] = rider;
        c.g[A0][0] = Addr((rider as u32).wrapping_add(BUMP_RIDER_ANIM_CTRL)).lw(c);
        call_ptr(c, BUMP_ANIM_LAYER_DONE, 0x0012_e8f0)?;
        if c.g[V0][0] == 0 {
            return bump_exit(c, frame);
        }

        // The bump animation has finished: leave the sub-state.
        c.cycles += 5;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        BUMP_GET_MAIN_STATE.call(c, 0x0012_e900)?;
        c.g[V1][0] = 0x4;
        c.g[A1][0] = 0;
        if c.g[V0][0] == 4 {
            c.cycles += 9;
            let rider = this.off(BUMP_RIDER).lw(c);
            c.g[V0][0] = rider;
            let owner = Addr((rider as u32).wrapping_add(BUMP_RIDER_OWNER)).lw(c) as u32;
            c.g[A0][0] = sx(owner.wrapping_add(0x2b0));
            call_ptr(c, BUMP_PLAY_LAND_ANIM, 0x0012_e91c)?;
            c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
            c.g[A1][0] = 0x7;
            BUMP_SET_SUB_STATE.call(c, 0x0012_e928)?;
            return bump_exit(c, frame);
        }
        c.cycles += 7;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        BUMP_RESET_GRIND_ORIENTATION.call(c, 0x0012_e938)?;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        BUMP_GET_MAIN_STATE.call(c, 0x0012_e940)?;
        c.g[V1][0] = 0x1;
        let airborne = c.g[V0][0] == 1;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        if airborne {
            c.cycles += 4;
            c.g[A1][0] = 0x4;
            BUMP_SET_SUB_STATE.call(c, 0x0012_e954)?;
        } else {
            c.cycles += 3;
            c.g[A1][0] = 0;
            BUMP_SET_SUB_STATE.call(c, 0x0012_e964)?;
        }
        bump_exit(c, frame)
    });
}

/// `RiderBumpSub_HandleInput`'s exit (`lq s0` in each branch, then the
/// shared 5-cycle tail).
pub(crate) fn bump_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    frame.restore(c, S0, 0x20);
    c.cycles += 5;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x30);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// RiderWipeoutSub_UpdateRecover (ai.cpp).

/// `SSXAudioSystem_GetInstance` (0x28b180; through the table).
pub(crate) const SSX_AUDIO_GET_INSTANCE_207: u32 = 0x0028_b180;

/// `SSXAudioSystem_PlayLandingSound` (0x2948d0; through the table).
pub(crate) const SSX_AUDIO_PLAY_LANDING_SOUND_207: u32 = 0x0029_48d0;

/// `AnimController_GetLayerProgress` (0x312ab0; through the table).
pub(crate) const ANIM_CONTROLLER_GET_LAYER_PROGRESS_207: u32 = 0x0031_2ab0;

/// `AnimController_IsLayerAnimFinished` (0x312ae8; through the table).
pub(crate) const ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED_207: u32 = 0x0031_2ae8;

/// `AnimController_PlayAnim` (0x3128e8; through the table).
pub(crate) const ANIM_CONTROLLER_PLAY_ANIM_207: u32 = 0x0031_28e8;

pub(crate) const RIDER_SET_SUB_STATE_207: Callee = Callee {
    addr: 0x0011_fec8,
    f: ee::s00::Rider_SetSubState,
};

pub(crate) const RIDER_SET_MAIN_STATE_207: Callee = Callee {
    addr: 0x0011_fe78,
    f: ee::s00::Rider_SetMainState,
};

pub(crate) const RIDER_METRIX_INTERRUPT_TRICK_207: Callee = Callee {
    addr: 0x0011_9e38,
    f: ee::s00::RiderMetrix_InterruptTrick,
};

/// `void RiderWipeoutSub_UpdateRecover(RiderWipeoutSub* self)` (0x12d848):
/// the wipeout's recovery phase.
///
/// - On the first update (`self+0x54` set): clear the flag, pass the length
///   of the impact vector (`self+0x60`, VU0 `sqrt` of the xyz dot product)
///   to the rider's vtable slot 0x88, then play the landing sound
///   (`SSXAudioSystem_PlayLandingSound(instance, rider, 0.0)`).
/// - A human rider (owner `+0x77c` → `+0x30` zero) feeds
///   `2 * (1 - progress of anim layer 2)` to vtable slot 0x90.
/// - When layer 2's animation has finished: a human rider plays anim 5 and
///   goes to sub-state 0 / main state 0; otherwise the trick is interrupted
///   (`RiderMetrix_InterruptTrick(metrix, +0x320 != +0x324, 0)`), anim 0x11f
///   plays and the rider goes to sub-state 5 / main state 1.
pub fn rider_wipeout_sub_update_recover(c: &mut Cpu) {
    /// The "recovery just started" flag.
    const STARTED: u32 = 0x54;
    /// The impact vector (xyz).
    const IMPACT: u32 = 0x60;
    const RIDER: u32 = 0x80;
    // Rider fields.
    const VTABLE: u32 = 0x6c0;
    const OWNER: u32 = 0x77c;
    const ANIM_CTL: u32 = 0x784;
    const METRIX: u32 = 0x790;
    /// Owner: non-zero for a computer rider.
    const OWNER_AI: u32 = 0x30;
    /// -1.0f (anim blend).
    const MINUS_ONE: u64 = 0xffff_ffff_bf80_0000;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S2)).off(STARTED).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 24;
            let me = r32(c, S2);
            c.sw(me.wrapping_add(STARTED), 0);
            c.g[A1][0] = Addr(me).off(RIDER).lw(c);
            c.lqc2(3, me.wrapping_add(IMPACT));
            // |impact| (xyz) into f12.
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(V0, 22);
            c.mtc1(V0, 12);
            c.g[V0][0] = Addr(r32(c, A1)).off(VTABLE).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x88).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x8c).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0012_d8b0)?;
            call_ptr(c, SSX_AUDIO_GET_INSTANCE_207, 0x0012_d8b8)?;
            c.g[A1][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
            c.mtc1(0, 12); // f12 = 0.0
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, SSX_AUDIO_PLAY_LANDING_SOUND_207, 0x0012_d8c8)?;
        }
        c.g[A2][0] = Addr(r32(c, S2)).off(RIDER).lw(c);

        // A human rider: feed the recovery progress to slot 0x90.
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, A2)).off(OWNER).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(OWNER_AI).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 14;
            let rider = r32(c, A2);
            c.g[S1][0] = Addr(rider).off(VTABLE).lw(c);
            c.g[A1][0] = 0x2;
            c.g[A0][0] = Addr(rider).off(ANIM_CTL).lw(c);
            c.g[S0][0] = Addr(r32(c, S1)).off(0x90).lh(c);
            c.g[S0][0] = sx(rider.wrapping_add(r32(c, S0)));
            call_ptr(c, ANIM_CONTROLLER_GET_LAYER_PROGRESS_207, 0x0012_d8f4)?;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 12); // f12 = 1.0
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = Addr(r32(c, S1)).off(0x94).lw(c);
            c.f[12] = (FpuF32(c.f[12]) - FpuF32(c.f[0])).0;
            let target = r32(c, V0);
            c.f[12] = (FpuF32(c.f[12]) + FpuF32(c.f[12])).0;
            call_ptr(c, target, 0x0012_d910)?;
        }
        c.g[V0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);

        // Layer 2 finished: leave the wipeout.
        c.cycles += 5;
        c.g[A1][0] = 0x2;
        c.g[A0][0] = Addr(r32(c, V0)).off(ANIM_CTL).lw(c);
        call_ptr(c, ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED_207, 0x0012_d920)?;
        let finished = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x30);
        if finished {
            c.cycles += 5;
            c.g[A0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
            c.g[V0][0] = Addr(r32(c, A0)).off(OWNER).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(OWNER_AI).lw(c);
            if c.g[V1][0] == 0 {
                c.cycles += 14;
                c.g[A0][0] = Addr(r32(c, A0)).off(ANIM_CTL).lw(c);
                c.g[A1][0] = 0x5;
                c.g[AT][0] = MINUS_ONE;
                c.mtc1(AT, 12);
                c.g[A2][0] = 0;
                call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM_207, 0x0012_d954)?;
                c.g[A0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
                c.g[A1][0] = 0;
                RIDER_SET_SUB_STATE_207.call(c, 0x0012_d960)?;
                c.g[A0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
                c.g[A1][0] = 0;
                RIDER_SET_MAIN_STATE_207.call(c, 0x0012_d96c)?;
            } else {
                let rider = r32(c, A0);
                c.g[V0][0] = Addr(rider).off(0x324).lw(c);
                c.cycles += 20;
                c.g[A2][0] = 0;
                c.g[A1][0] = Addr(rider).off(0x320).lw(c);
                c.g[A0][0] = Addr(rider).off(METRIX).lw(c);
                c.g[A1][0] ^= c.g[V0][0];
                c.g[A1][0] = (c.g[A1][0] != 0) as u64;
                RIDER_METRIX_INTERRUPT_TRICK_207.call(c, 0x0012_d98c)?;
                c.g[V0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
                c.g[A1][0] = 0x11f;
                c.g[AT][0] = MINUS_ONE;
                c.mtc1(AT, 12);
                c.g[A2][0] = 0;
                c.g[A0][0] = Addr(r32(c, V0)).off(ANIM_CTL).lw(c);
                call_ptr(c, ANIM_CONTROLLER_PLAY_ANIM_207, 0x0012_d9a8)?;
                c.g[A0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
                c.g[A1][0] = 0x5;
                RIDER_SET_SUB_STATE_207.call(c, 0x0012_d9b4)?;
                c.g[A0][0] = Addr(r32(c, S2)).off(RIDER).lw(c);
                c.g[A1][0] = 0x1;
                RIDER_SET_MAIN_STATE_207.call(c, 0x0012_d9c0)?;
            }
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutSub_UpdatePhase0 (0x12cd20)

/// `RiderWipeoutSub + 0x80`: the rider.
pub(crate) const WP0_RIDER: u32 = 0x80;

/// Rider fields: model (+0x780, its `+0x2c` the root-node pose array, 0x20
/// bytes per node: position, rotation quaternion), anim controller (+0x784),
/// wipeout state (+0x77c), the two tracked node indices.
pub(crate) const WP0_RIDER_STATE: u32 = 0x77c;

pub(crate) const WP0_RIDER_MODEL: u32 = 0x780;

pub(crate) const WP0_RIDER_ANIM: u32 = 0x784;

pub(crate) const WP0_RIDER_NODE1: u32 = 0x89c;

pub(crate) const WP0_RIDER_NODE2: u32 = 0x8a4;

/// Anim queries on track 2: `AnimController_GetTrackPhase` (f0),
/// `..._IsTrackDone`, `..._GetTrackSeq`, `..._GetTrackAnimType`.
pub(crate) const WP0_TRACK_PHASE: u32 = 0x0031_2ab0;

pub(crate) const WP0_TRACK_DONE: u32 = 0x0031_2ae8;

pub(crate) const WP0_TRACK_SEQ: u32 = 0x0031_1b20;

pub(crate) const WP0_TRACK_TYPE: u32 = 0x0031_1ae8;

/// `RiderWipeoutState_SetVelocity(state + 0x30, rider + 0x1e0, &angvel)`.
pub(crate) const WP0_SET_VELOCITY: u32 = 0x0013_6de0;

/// `RiderWipeoutState_EnableRootPoseOverride(state + 0x30, &vel, &angvel, node2)`.
pub(crate) const WP0_ROOT_OVERRIDE: u32 = 0x0013_6d40;

/// `RiderWipeoutSub_SelectTumbleAnim(self)`.
pub(crate) const WP0_SELECT_TUMBLE: Callee = Callee {
    addr: 0x0012_da88,
    f: ee::s00::f_0012da88,
};

/// `SSXAudioSystem_GetInstance()`, `..._StartWipeoutLoop(audio, self)`,
/// `..._StartWipeoutLoop2(audio, self)`.
pub(crate) const WP0_AUDIO_INSTANCE: u32 = 0x0028_b180;

pub(crate) const WP0_AUDIO_LOOP: u32 = 0x0029_6868;

pub(crate) const WP0_AUDIO_LOOP2: u32 = 0x0029_6e80;

/// `gp - 0x77c4`: 0.95, the phase up to which the start poses are recorded.
pub(crate) const WP0_RECORD_LIMIT_GP: u32 = 0x77c4;

/// Anim types that end the tumble phase specially.
pub(crate) const WP0_TYPE_ROOT_OVERRIDE: u64 = 0x17;

pub(crate) const WP0_TYPE_LOOP_A: u64 = 0x1d;

pub(crate) const WP0_TYPE_LOOP_B: u64 = 0x19;

/// `v = vf[src] * f[f].x` through the stack: `mfc1 v1, f; lqc2 vf4, src(sp);
/// qmtc2 v1, vf3; vmulx vf5, vf4, vf3; sqc2 vf5, dst(sp); lq v0, dst(sp)`.
pub(crate) fn wp0_scale(c: &mut Cpu, f: usize, src: u32, dst: u32) {
    let s = sp(c);
    c.mfc1(V1, f);
    c.lqc2(4, s.wrapping_add(src));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, s.wrapping_add(dst));
    c.lq(V0, s.wrapping_add(dst));
}

/// `vf5 = [a] - [b]` stored at `dst(sp)`.
pub(crate) fn wp0_sub(c: &mut Cpu, a: u32, b: u32, dst: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    let s = sp(c);
    c.sqc2(5, s.wrapping_add(dst));
}

/// The quaternion product `vf4 * vf5` (xyz cross/dot form) into vf8.
pub(crate) fn wp0_quat_mul(c: &mut Cpu) {
    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
}

/// The conjugate of the node quaternion at `node + 0x10` written to
/// `conj(sp)..+0xc` (`(-x, -y, -z, w)`, stored in the original's order).
pub(crate) fn wp0_store_conj(c: &mut Cpu, node: u32, conj: u32) {
    let s = sp(c).wrapping_add(conj);
    let w = Addr(node.wrapping_add(0x1c)).read_fpu(c);
    abi::set_fpr(c, 0, w);
    Addr(s.wrapping_add(0xc)).write_fpu(c, w);
    let x = Addr(node.wrapping_add(0x10)).read_fpu(c);
    abi::set_fpr(c, 2, x);
    Addr(s).write_fpu(c, x);
    let y = Addr(node.wrapping_add(0x14)).read_fpu(c);
    abi::set_fpr(c, 1, y);
    abi::set_fpr(c, 2, -x);
    Addr(s.wrapping_add(0x4)).write_fpu(c, y);
    abi::set_fpr(c, 1, -y);
    let z = Addr(node.wrapping_add(0x18)).read_fpu(c);
    abi::set_fpr(c, 0, z);
    Addr(s).write_fpu(c, -x);
    abi::set_fpr(c, 0, -z);
    Addr(s.wrapping_add(0x4)).write_fpu(c, -y);
    Addr(s.wrapping_add(0x8)).write_fpu(c, -z);
}

/// Copy the xyz of the quadword at `src(sp)` to `dst(sp)` through f0..f2.
pub(crate) fn wp0_copy_xyz(c: &mut Cpu, src: u32, dst: u32) {
    let s = sp(c);
    c.lwc1(0, s.wrapping_add(src));
    c.lwc1(1, s.wrapping_add(src + 4));
    c.lwc1(2, s.wrapping_add(src + 8));
    c.swc1(0, s.wrapping_add(dst));
    c.swc1(1, s.wrapping_add(dst + 4));
    c.swc1(2, s.wrapping_add(dst + 8));
}

/// `void RiderWipeoutSub_UpdatePhase0(RiderWipeoutSub* self)` (0x12cd20):
/// the tumble start. While the track-2 animation plays, record its phase and
/// the two tracked root-node poses (until phase 0.95). When it is done, turn
/// the pose change since the record into a linear and angular velocity for
/// the wipeout state (and, for anim type 0x17, a root-pose override from the
/// second node), then select the tumble animation: phase 1 if the state is in
/// mode 1, else phase 2 and start the wipeout audio loop (type 0x1d/0x19 or
/// the other one).
pub fn rider_wipeout_sub_update_phase0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0xe0);
        c.g[A1][0] = 2;
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0xa0);
        frame.save_fpr(c, 20, 0xd0);
        frame.save(c, S3, 0x90);
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 21, 0xd8);
        let rider = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
        c.g[V1][0] = rider;
        let rider = rider as u32;
        c.g[A0][0] = Addr(rider.wrapping_add(WP0_RIDER_MODEL)).lw(c);
        c.g[A2][0] = Addr(rider.wrapping_add(WP0_RIDER_NODE1)).lw(c);
        c.g[V0][0] = Addr(rider.wrapping_add(WP0_RIDER_NODE2)).lw(c);
        c.g[A3][0] = Addr(r32(c, A0).wrapping_add(0x2c)).lw(c); // nodes
        c.g[A2][0] = sx(r32(c, A2) << 5);
        c.g[V0][0] = sx(r32(c, V0) << 5);
        c.g[A0][0] = Addr(rider.wrapping_add(WP0_RIDER_ANIM)).lw(c);
        c.g[S2][0] = sx(r32(c, A3).wrapping_add(r32(c, V0))); // node 2
        c.g[S1][0] = sx(r32(c, A3).wrapping_add(r32(c, A2))); // node 1
        call_ptr(c, WP0_TRACK_PHASE, 0x0012_cd74)?;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
        c.f[20] = c.f[0]; // phase
        c.g[A1][0] = 2;
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(WP0_RIDER_ANIM)).lw(c);
        call_ptr(c, WP0_TRACK_DONE, 0x0012_cd88)?;
        let done = c.g[V0][0] != 0;
        c.g[A1][0] = 2;

        if !done {
            // Still playing: record the phase and both node poses up to 0.95.
            c.cycles += 5;
            let limit = Addr(r32(c, GP).wrapping_sub(WP0_RECORD_LIMIT_GP)).read_fpu(c);
            abi::set_fpr(c, 0, limit);
            let record = FpuF32(c.f[20]).lt(limit);
            abi::set_fcc(c, record);
            if record {
                c.cycles += 10;
                let (this, n1, n2) = (r32(c, S0), r32(c, S1), r32(c, S2));
                c.swc1(20, this.wrapping_add(0x50));
                c.lq(V0, n1);
                c.sq(V0, this.wrapping_add(0x10));
                c.lq(V1, n1.wrapping_add(0x10));
                c.sq(V1, this.wrapping_add(0x20));
                c.lq(V0, n2);
                c.sq(V0, this.wrapping_add(0x30));
                c.lq(V1, n2.wrapping_add(0x10));
                c.sq(V1, this.wrapping_add(0x40));
            }
        } else {
            c.cycles += 96;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(WP0_RIDER_ANIM)).lw(c);
            call_ptr(c, WP0_TRACK_SEQ, 0x0012_cd9c)?;
            let seq = r32(c, V0);
            let (this, n1) = (r32(c, S0), r32(c, S1));
            let length = Addr(seq.wrapping_add(0x10)).read_fpu(c);
            abi::set_fpr(c, 0, length);
            c.g[S3][0] = sx(sp(c).wrapping_add(0x30));
            let rate = Addr(seq.wrapping_add(0x90)).read_fpu(c);
            abi::set_fpr(c, 2, rate);
            c.g[A2][0] = c.g[S3][0];
            let start = Addr(this.wrapping_add(0x50)).read_fpu(c);
            abi::set_fpr(c, 1, start);
            // Linear: (node1.pos - start pos) * anim scale * s.
            wp0_sub(c, n1, this.wrapping_add(0x10), 0x30);
            let s0 = sp(c);
            c.lq(V0, s0.wrapping_add(0x30));
            let elapsed = FpuF32(c.f[20]) - start;
            c.g[V1][0] = Addr(this.wrapping_add(WP0_RIDER)).lw(c);
            c.sq(V0, s0.wrapping_add(0x20));
            let denom = elapsed * length;
            abi::set_fpr(c, 1, denom);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(WP0_RIDER_ANIM)).lw(c);
            let scale = Addr(r32(c, V0).wrapping_add(0x1c)).read_fpu(c);
            abi::set_fpr(c, 0, scale);
            let speed = rate.div(denom); // s
            abi::set_fpr(c, 20, speed);
            wp0_scale(c, 0, 0x20, 0x30);
            c.sq(V0, s0.wrapping_add(0x10));
            wp0_scale(c, 20, 0x10, 0x30);
            abi::set_fpr(c, 21, speed + speed);
            c.sq(V0, s0);
            // Angular: (node1.rot - start rot) * conj(node1.rot), xyz, * 2s.
            wp0_sub(c, n1.wrapping_add(0x10), this.wrapping_add(0x20), 0x20);
            c.lq(V0, s0.wrapping_add(0x20));
            c.sq(V0, s0.wrapping_add(0x10));
            wp0_store_conj(c, n1, 0x40);
            c.lq(V0, s0.wrapping_add(0x40));
            c.sq(V0, s0.wrapping_add(0x30));
            c.lqc2(4, s0.wrapping_add(0x10));
            c.lqc2(5, s0.wrapping_add(0x30));
            wp0_quat_mul(c);
            c.sqc2(8, s0.wrapping_add(0x40));
            c.lq(V0, s0.wrapping_add(0x40));
            Addr(s0.wrapping_add(0x4c)).sw(c, 0);
            c.sq(V0, s0.wrapping_add(0x20));
            wp0_copy_xyz(c, 0x20, 0x40);
            wp0_scale(c, 21, 0x40, 0x50);
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
            c.sq(V0, s0.wrapping_add(0x30));
            c.g[A1][0] = sx(r32(c, V1).wrapping_add(0x1e0));
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(WP0_RIDER_STATE)).lw(c);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x30));
            call_ptr(c, WP0_SET_VELOCITY, 0x0012_cef4)?;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
            c.g[A1][0] = 2;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(WP0_RIDER_ANIM)).lw(c);
            call_ptr(c, WP0_TRACK_TYPE, 0x0012_cf04)?;
            c.g[V1][0] = WP0_TYPE_ROOT_OVERRIDE;
            if c.g[V0][0] == c.g[V1][0] {
                // The same from node 2, handed to the root-pose override.
                c.cycles += 84;
                let (this, n2) = (r32(c, S0), r32(c, S2));
                let s0 = sp(c);
                wp0_sub(c, n2, this.wrapping_add(0x30), 0x70);
                c.g[A2][0] = c.g[S3][0];
                c.lq(V1, s0.wrapping_add(0x70));
                c.g[A3][0] = c.g[S2][0];
                c.g[A0][0] = Addr(this.wrapping_add(WP0_RIDER)).lw(c);
                c.g[A1][0] = c.g[SP][0];
                c.sq(V1, s0.wrapping_add(0x60));
                c.g[V0][0] = Addr(r32(c, A0).wrapping_add(WP0_RIDER_ANIM)).lw(c);
                let scale = Addr(r32(c, V0).wrapping_add(0x1c)).read_fpu(c);
                abi::set_fpr(c, 0, scale);
                wp0_scale(c, 0, 0x60, 0x70);
                c.sq(V0, s0.wrapping_add(0x50));
                wp0_scale(c, 20, 0x50, 0x70);
                c.sq(V0, s0.wrapping_add(0x40));
                c.sq(V0, s0);
                wp0_sub(c, n2.wrapping_add(0x10), this.wrapping_add(0x40), 0x50);
                c.lq(V0, s0.wrapping_add(0x50));
                c.sq(V0, s0.wrapping_add(0x10));
                c.sq(V0, s0.wrapping_add(0x40));
                wp0_store_conj(c, n2, 0x60);
                c.lq(V0, s0.wrapping_add(0x60));
                c.sq(V0, s0.wrapping_add(0x50));
                c.lqc2(4, s0.wrapping_add(0x10));
                c.lqc2(5, s0.wrapping_add(0x50));
                wp0_quat_mul(c);
                c.sqc2(8, s0.wrapping_add(0x60));
                c.lq(V0, s0.wrapping_add(0x60));
                Addr(s0.wrapping_add(0x5c)).sw(c, 0);
                c.sq(V0, s0.wrapping_add(0x20));
                c.sq(V0, s0.wrapping_add(0x40));
                wp0_copy_xyz(c, 0x20, 0x50);
                wp0_scale(c, 21, 0x50, 0x60);
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
                c.sq(V0, s0.wrapping_add(0x30));
                c.sq(V0, s0.wrapping_add(0x40));
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(WP0_RIDER_STATE)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x30));
                call_ptr(c, WP0_ROOT_OVERRIDE, 0x0012_d05c)?;
            }
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);

            // Next phase: 1 when the wipeout state is in mode 1, else 2.
            c.cycles += 5;
            c.g[A0][0] = 1;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(WP0_RIDER_STATE)).lw(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x30)).lw(c);
            if c.g[V0][0] == c.g[A0][0] {
                c.cycles += 5;
                let phase = c.g[V0][0];
                Addr(r32(c, S0)).sw(c, phase);
                c.g[A0][0] = c.g[S0][0];
                WP0_SELECT_TUMBLE.call(c, 0x0012_d080)?;
            } else {
                c.g[V0][0] = 2;
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                Addr(r32(c, S0)).sw(c, 2);
                WP0_SELECT_TUMBLE.call(c, 0x0012_d094)?;
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
                c.g[A1][0] = 2;
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(WP0_RIDER_ANIM)).lw(c);
                call_ptr(c, WP0_TRACK_TYPE, 0x0012_d0a4)?;
                c.g[V1][0] = WP0_TYPE_LOOP_A;
                let mut loop_a = c.g[V0][0] == c.g[V1][0];
                c.g[A1][0] = 2;
                if !loop_a {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(WP0_RIDER)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(WP0_RIDER_ANIM)).lw(c);
                    call_ptr(c, WP0_TRACK_TYPE, 0x0012_d0bc)?;
                    c.g[V1][0] = WP0_TYPE_LOOP_B;
                    loop_a = c.g[V0][0] == c.g[V1][0];
                }
                c.cycles += 7;
                let (ret1, start, ret2) = if loop_a {
                    (0x0012_d0d0, WP0_AUDIO_LOOP, 0x0012_d0dc)
                } else {
                    (0x0012_d0ec, WP0_AUDIO_LOOP2, 0x0012_d0f8)
                };
                call_ptr(c, WP0_AUDIO_INSTANCE, ret1)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, start, ret2)?;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 21, 0xd8);
        frame.restore_fpr(c, 20, 0xd0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutSub_UpdateAnimSpeed (0x12e528)

/// Wipeout sub-state fields: the tumble timer (+0x70) and the rider (+0x80).
pub(crate) const WIPEOUT_TIMER: u32 = 0x70;

pub(crate) const WIPEOUT_RIDER: u32 = 0x80;

/// Rider fields: progress along the course (+0x470), the motion state
/// (+0x77c, its velocity at +0x60) and the animation controller (+0x784).
pub(crate) const WIPEOUT_RIDER_PROGRESS: u32 = 0x470;

pub(crate) const WIPEOUT_RIDER_MOTION: u32 = 0x77c;

pub(crate) const WIPEOUT_MOTION_VELOCITY: u32 = 0x60;

pub(crate) const WIPEOUT_RIDER_ANIM: u32 = 0x784;

/// Animation sequence: playback speed (+0x10) and speed scale (+0x90).
pub(crate) const WIPEOUT_SEQ_SPEED: u32 = 0x10;

pub(crate) const WIPEOUT_SEQ_SCALE: u32 = 0x90;

/// The animation layer the wipeout plays on, and the two animation ids
/// that always play at normal speed.
pub(crate) const WIPEOUT_LAYER: u64 = 2;

pub(crate) const WIPEOUT_ANIM_FIXED_A: u64 = 0x1d;

pub(crate) const WIPEOUT_ANIM_FIXED_B: u64 = 0x19;

/// `1 / (2 pi)` (gp-0x77a4).
pub(crate) const WIPEOUT_INV_TWO_PI_GP: u32 = 0x77a4;

pub(crate) const WIPEOUT_HALF: u32 = 0x3f00_0000;

pub(crate) const WIPEOUT_ONE: u32 = 0x3f80_0000;

pub(crate) const WIPEOUT_TWO: u32 = 0x4000_0000;

/// `AnimController_GetLayerSequence` (0x311b20) and
/// `AnimController_GetLayerAnimEntry` (0x311ae8), through the table.
pub(crate) const WIPEOUT_GET_LAYER_SEQUENCE: u32 = 0x0031_1b20;

pub(crate) const WIPEOUT_GET_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;

/// `float RiderWipeoutSub_UpdateAnimSpeed(RiderWipeoutSub *self)`
/// (0x12e528): scales the tumble animation (layer 2) with the rider's
/// speed: `|velocity| * sequence speed / (2 pi)` clamped to [0.5, 2] (a
/// NaN gives 0.5). Animations 0x1d and 0x19 play at 1; after the tumble
/// timer reaches 1 or while the rider is not behind the start (progress
/// >= 0) the scale is at least 2; otherwise the clamped value. Returns the
/// clamped value. Frame 0x40 (ra at 0, s1 at 0x10, s0 at 0x20, f20 at
/// 0x30).
pub fn rider_wipeout_sub_update_anim_speed(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = WIPEOUT_LAYER;
        frame.save(c, S1, 0x10);
        frame.save_fpr(c, 20, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let rider = Addr(r32(c, S1)).off(WIPEOUT_RIDER).lw(c);
        c.g[V0][0] = rider;
        let rider = Addr(rider as u32);
        c.g[A0][0] = rider.off(WIPEOUT_RIDER_ANIM).lw(c);
        c.g[S0][0] = rider.off(WIPEOUT_RIDER_MOTION).lw(c);
        call_ptr(c, WIPEOUT_GET_LAYER_SEQUENCE, 0x0012_e554)?;

        // speed = |velocity| (VU0 macro mode, as the original).
        let seq_speed = Addr(r32(c, V0)).off(WIPEOUT_SEQ_SPEED).read_fpu(c);
        c.f[1] = seq_speed.0;
        let vel = r32(c, S0).wrapping_add(WIPEOUT_MOTION_VELOCITY);
        c.lqc2(3, vel);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V1, 22);
        let speed = FpuF32(c.g[V1][0] as u32);
        let inv_two_pi = Addr(gp.wrapping_sub(WIPEOUT_INV_TWO_PI_GP)).read_fpu(c);
        c.f[3] = inv_two_pi.0;
        let half = FpuF32(WIPEOUT_HALF);
        let two = FpuF32(WIPEOUT_TWO);
        c.f[2] = half.0;
        c.f[4] = two.0;
        c.g[AT][0] = WIPEOUT_TWO as u64;
        let rate = speed * seq_speed * inv_two_pi;
        c.f[0] = rate.0;
        let above = half.le(rate);
        abi::set_fcc(c, above);
        let clamped = if above {
            c.cycles += 1;
            rate.min(two)
        } else {
            half
        };
        c.f[20] = clamped.0;

        // The animation playing on the layer decides the scale.
        c.cycles += 7;
        let anim = |c: &mut Cpu| {
            let rider = Addr(r32(c, S1)).off(WIPEOUT_RIDER).lw(c);
            c.g[V0][0] = rider;
            c.g[A0][0] = Addr(rider as u32).off(WIPEOUT_RIDER_ANIM).lw(c);
        };
        c.g[A1][0] = WIPEOUT_LAYER;
        anim(c);
        call_ptr(c, WIPEOUT_GET_LAYER_ANIM_ENTRY, 0x0012_e5c0)?;
        c.g[V1][0] = WIPEOUT_ANIM_FIXED_A;
        c.g[A1][0] = WIPEOUT_LAYER;
        let mut fixed = c.g[V0][0] == c.g[V1][0];
        if !fixed {
            c.cycles += 6;
            anim(c);
            call_ptr(c, WIPEOUT_GET_LAYER_ANIM_ENTRY, 0x0012_e5d8)?;
            c.g[V1][0] = WIPEOUT_ANIM_FIXED_B;
            fixed = c.g[V0][0] == c.g[V1][0];
        }
        let one = FpuF32(WIPEOUT_ONE);
        if fixed {
            c.cycles += 8;
            c.g[A1][0] = WIPEOUT_LAYER;
            anim(c);
            call_ptr(c, WIPEOUT_GET_LAYER_SEQUENCE, 0x0012_e5f4)?;
            c.g[AT][0] = WIPEOUT_ONE as u64;
            c.f[0] = one.0;
            Addr(r32(c, V0)).off(WIPEOUT_SEQ_SCALE).write_fpu(c, one);
        } else {
            let timer = Addr(r32(c, S1)).off(WIPEOUT_TIMER).read_fpu(c);
            c.f[1] = timer.0;
            c.cycles += 6;
            c.g[AT][0] = WIPEOUT_ONE as u64;
            c.f[0] = one.0;
            let timer_done = one.le(timer);
            abi::set_fcc(c, timer_done);
            let rider = Addr(r32(c, S1)).off(WIPEOUT_RIDER).lw(c);
            c.g[A0][0] = rider;
            let mut at_least_two = timer_done;
            if !timer_done {
                c.cycles += 7;
                c.g[V0][0] = rider;
                c.f[1] = 0;
                let progress = Addr(rider as u32).off(WIPEOUT_RIDER_PROGRESS).read_fpu(c);
                c.f[0] = progress.0;
                let ahead = FpuF32(0).le(progress);
                abi::set_fcc(c, ahead);
                c.g[V1][0] = 1;
                if !ahead {
                    c.cycles += 1;
                    c.g[V1][0] = 0;
                }
                c.cycles += 2;
                at_least_two = ahead;
            }
            if at_least_two {
                c.cycles += 8;
                c.g[A0][0] = Addr(r32(c, A0)).off(WIPEOUT_RIDER_ANIM).lw(c);
                c.g[A1][0] = WIPEOUT_LAYER;
                call_ptr(c, WIPEOUT_GET_LAYER_SEQUENCE, 0x0012_e650)?;
                c.g[AT][0] = WIPEOUT_TWO as u64;
                let scale = FpuF32(c.f[20]).max(two);
                c.f[0] = scale.0;
                Addr(r32(c, V0)).off(WIPEOUT_SEQ_SCALE).write_fpu(c, scale);
            } else {
                c.g[A0][0] = Addr(r32(c, V0)).off(WIPEOUT_RIDER_ANIM).lw(c);
                c.cycles += 3;
                c.g[A1][0] = WIPEOUT_LAYER;
                call_ptr(c, WIPEOUT_GET_LAYER_SEQUENCE, 0x0012_e66c)?;
                let scale = FpuF32(c.f[20]);
                Addr(r32(c, V0)).off(WIPEOUT_SEQ_SCALE).write_fpu(c, scale);
            }
        }
        c.cycles += 7;
        c.f[0] = c.f[20];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderSpinSub_TryReverse (0x135be0)

/// `Rider_CanReverseSpin`-style query on the rider (0x114db8).
pub(crate) const SPIN_CAN_REVERSE: u32 = 0x0011_4db8;

/// `cRiderAnimBase::play(anim, id, flags, blend)` (0x3128e8).
pub(crate) const SPIN_ANIM_PLAY: u32 = 0x0031_28e8;

/// Spin sub-state fields.
pub(crate) const SPIN_SUB_10: u32 = 0x10;

pub(crate) const SPIN_SUB_18: u32 = 0x18;

pub(crate) const SPIN_SUB_ANGLE: u32 = 0x20;

pub(crate) const SPIN_SUB_RATE: u32 = 0x24;

pub(crate) const SPIN_SUB_2C: u32 = 0x2c;

pub(crate) const SPIN_SUB_RIDER: u32 = 0x58;

pub(crate) const RIDER_ANIM: u32 = 0x784;

/// The reverse-spin animation.
pub(crate) const SPIN_REVERSE_ANIM: u64 = 0x120;

/// `bool RiderSpinSub_TryReverse(RiderSpinSub *self)` (0x135be0): if the
/// rider may reverse (0x114db8), turn the spin around: the angle plus pi
/// wrapped back into one turn (`a - 2pi * floor(a / 2pi + 0.5)`), the rate
/// and the three other signed fields negated, and the reverse animation
/// (0x120) played with blend -1. Returns whether it reversed.
pub fn rider_spin_sub_try_reverse(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).off(SPIN_SUB_RIDER).lw(c);
        call_ptr(c, SPIN_CAN_REVERSE, 0x0013_5bf8)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            let sub = Addr(r32(c, S0));
            let gp = r32(c, GP);
            c.cycles += 13;
            let pi = Addr(gp.wrapping_sub(0x733c)).read_fpu(c);
            let angle = sub.off(SPIN_SUB_ANGLE).read_fpu(c);
            let inv_two_pi = Addr(gp.wrapping_sub(0x7338)).read_fpu(c);
            let turned = angle + pi;
            let half = FpuF32::from_f32(0.5);
            c.g[AT][0] = 0x3f00_0000;
            let turns = turned * inv_two_pi + half;
            let mut whole = FpuF32::from_i32(turns.to_i32());
            c.f[3] = angle.0;
            c.f[5] = turned.0;
            c.f[2] = half.0;
            c.f[0] = turns.0;
            if abi::c_lt_s(c, turns, whole) {
                c.cycles += 3;
                c.g[AT][0] = 0x3f80_0000;
                c.f[0] = FpuF32::ONE.0;
                whole = whole - FpuF32::ONE;
            }
            c.f[1] = whole.0;

            c.cycles += 24;
            let two_pi = Addr(gp.wrapping_sub(0x7334)).read_fpu(c);
            c.g[A1][0] = SPIN_REVERSE_ANIM;
            let rate = -sub.off(SPIN_SUB_RATE).read_fpu(c);
            c.g[A2][0] = 0;
            let f2c = -sub.off(SPIN_SUB_2C).read_fpu(c);
            let f18 = -sub.off(SPIN_SUB_18).read_fpu(c);
            let f10 = -sub.off(SPIN_SUB_10).read_fpu(c);
            let wrapped = turned - whole * two_pi;
            c.g[V0][0] = sub.off(SPIN_SUB_RIDER).lw(c);
            sub.off(SPIN_SUB_RATE).write_fpu(c, rate);
            sub.off(SPIN_SUB_2C).write_fpu(c, f2c);
            sub.off(SPIN_SUB_ANGLE).write_fpu(c, wrapped);
            sub.off(SPIN_SUB_18).write_fpu(c, f18);
            sub.off(SPIN_SUB_10).write_fpu(c, f10);
            c.f[3] = rate.0;
            c.f[0] = f2c.0;
            c.f[4] = f18.0;
            c.f[1] = f10.0;
            c.f[2] = wrapped.0;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.f[12] = 0xbf80_0000; // blend -1.0
            c.g[A0][0] = Addr(r32(c, V0)).off(RIDER_ANIM).lw(c);
            call_ptr(c, SPIN_ANIM_PLAY, 0x0013_5c9c)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the rider sub-state `SaveState` forwarders:
/// `vt = *(VTable**)stream; vt[1].fn(stream + vt[1].delta, self, size)` — the
/// state stream's write method (vtable slot 1, at +8) saves `size` bytes of
/// `self`. `stream` (a1) is kept in a3, `self` becomes the second argument.
pub(crate) fn rider_sub_save_state_call(
    c: &mut Cpu,
    ret_addr: u32,
    size: u32,
) -> crate::call::Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    c.g[A3][0] = c.g[A1][0];
    let stream = c.g[A3][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = c.g[A0][0];
    c.g[A2][0] = sx(size);
    c.g[V1][0] = Addr(stream as u32).lw(c);
    let slot = crate::call::VSlot::read(c, c.g[V1][0], 0x8);
    c.g[A0][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(stream);
    call_ptr(c, slot.func as u32, ret_addr)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void RiderSpinSub_SaveState(RiderSpinSub* self, StateStream* s)`
/// (0x1361f8): `s->Write(self, 0x58)`.
pub fn rider_spin_sub_save_state(c: &mut Cpu) {
    guest(c, |c| rider_sub_save_state_call(c, 0x0013_6220, 0x58))
}

/// `void RiderGateSub_SaveState(RiderGateSub* self, StateStream* s)`
/// (0x12c558): `s->Write(self, 0x14)`.
pub fn rider_gate_sub_save_state(c: &mut Cpu) {
    guest(c, |c| rider_sub_save_state_call(c, 0x0012_c580, 0x14))
}

/// The shape of the small `SaveState` methods: `s->Write(this + off, size)`
/// through the stream's vtable (+8), from a 0x10 frame returning to `ret`.
/// 13 cycles.
pub(crate) fn stream_write_member(c: &mut Cpu, off: u32, size: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        let stream = c.g[A1][0];
        c.g[A3][0] = stream;
        frame.save_ra(c, 0);
        c.g[A1][0] = if off == 0 {
            c.g[A0][0]
        } else {
            sx((c.g[A0][0] as u32).wrapping_add(off))
        };
        c.g[A2][0] = size;
        let vt = c.lw(stream as u32);
        c.g[V1][0] = vt;
        let delta = c.lh((vt as u32).wrapping_add(0x8));
        let write = c.lw((vt as u32).wrapping_add(0xc));
        c.g[V0][0] = write;
        c.g[A0][0] = sx((stream as u32).wrapping_add(delta as u32));
        call_ptr(c, write as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderHandplantSub::SaveState(cStream* s)` (0x1330b8): writes the
/// 0x14-byte state.
pub fn rider_handplant_sub_save_state(c: &mut Cpu) {
    stream_write_member(c, 0, 0x14, 0x0013_30e0);
}

// ---------------------------------------------------------------------------
// RiderSpinSub_Enter (0x133128)

/// RiderSpinSub: state (1 spinning, 0 not), next state (-1 none) at +0x8,
/// the exit state at +0xc.
pub(crate) const SPIN_ENTER_STATE: u32 = 0x0;

pub(crate) const SPIN_ENTER_NEXT: u32 = 0x8;

pub(crate) const SPIN_ENTER_EXIT: u32 = 0xc;

/// Spin and flip rates (radians per tick), the spin factor.
pub(crate) const SPIN_ENTER_SPIN_RATE: u32 = 0x38;

pub(crate) const SPIN_ENTER_FLIP_RATE: u32 = 0x3c;

pub(crate) const SPIN_ENTER_FACTOR: u32 = 0x40;

/// The rider.
pub(crate) const SPIN_ENTER_RIDER: u32 = 0x58;

/// Rider: stick x and y at the jump, the spin and flip rates the
/// animation reads, the animation controller, player and character.
pub(crate) const SPIN_ENTER_STICK_X: u32 = 0x2a4;

pub(crate) const SPIN_ENTER_STICK_Y: u32 = 0x2b0;

pub(crate) const SPIN_ENTER_RIDER_SPIN_RATE: u32 = 0x2dc;

pub(crate) const SPIN_ENTER_RIDER_FLIP_RATE: u32 = 0x2e0;

pub(crate) const SPIN_ENTER_ANIM_CTRL: u32 = 0x784;

pub(crate) const SPIN_ENTER_PLAYER: u32 = 0x86c;

pub(crate) const SPIN_ENTER_CHARACTER: u32 = 0xb34;

/// Animation sequence: playback speed.
pub(crate) const SPIN_ENTER_SEQ_SPEED: u32 = 0x90;

pub(crate) const SPIN_ENTER_GET_SPIN_FACTOR: Callee = Callee {
    addr: 0x0013_5180,
    f: ee::s01::RiderSpinSub_GetSpinFactor,
};

pub(crate) const SPIN_ENTER_SET_SPIN_DIRECTION: Callee = Callee {
    addr: 0x0013_5b30,
    f: ee::s01::RiderSpinSub_SetSpinDirection,
};

/// `void RiderSpinSub::Enter(RiderSpinSub* this, int)` (0x133128): start
/// the air-spin sub-state. Clears the spin bookkeeping (+0x4..+0x54, rates
/// at the minimum 4.4997), and if the stick held at the jump (rider
/// +0x2a4/+0x2b0) is pushed at all: spin rate = max(7 * the larger stick
/// axis, min) when x is pushed, flip rate = that * 2/3 when y is pushed
/// (both mirrored into the rider), the spin factor and direction, and the
/// spin animation's speed = (1 + 0.5 * the character's spin attribute) *
/// max(flip, spin rate) / 7; state 1. Otherwise state 0, exit state 3.
pub fn rider_spin_sub_enter(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 39;
        let frame = Frame::open(c, 0x30);
        c.lwc1(5, gp.wrapping_sub(0x7544)); // minimum rate
        frame.save(c, S0, 0x10);
        c.g[V0][0] = u64::MAX;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x20);
        let this = r32(c, S0);
        Addr(this.wrapping_add(SPIN_ENTER_NEXT)).sw(c, u64::MAX);
        c.g[V1][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        for off in [
            0x4, 0x18, 0x1c, 0x10, 0x14, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34,
        ] {
            Addr(this.wrapping_add(off)).sw(c, 0);
        }
        c.swc1(5, this.wrapping_add(SPIN_ENTER_SPIN_RATE));
        c.swc1(5, this.wrapping_add(SPIN_ENTER_FLIP_RATE));
        Addr(this.wrapping_add(SPIN_ENTER_FACTOR)).sw(c, 0);
        Addr(this.wrapping_add(0x44)).sw(c, 0);
        let rider = r32(c, V1);
        Addr(rider.wrapping_add(SPIN_ENTER_RIDER_SPIN_RATE)).sw(c, 0);
        c.lwc1(0, gp.wrapping_sub(0x7540)); // 1e30
        c.g[V0][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        Addr(r32(c, V0).wrapping_add(SPIN_ENTER_RIDER_FLIP_RATE)).sw(c, 0);
        Addr(this.wrapping_add(0x48)).sw(c, 0);
        c.swc1(0, this.wrapping_add(0x50));
        c.g[V0][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        Addr(this.wrapping_add(0x4c)).sw(c, 0);
        Addr(this.wrapping_add(0x54)).sw(c, 0);
        c.lwc1(4, this.wrapping_add(0x48)); // f4 = 0
        let rider = r32(c, V0);
        c.lwc1(0, rider.wrapping_add(SPIN_ENTER_STICK_X));
        c.c_eq_s(0, 4);
        if c.fcc() {
            c.cycles += 5;
            c.lwc1(0, rider.wrapping_add(SPIN_ENTER_STICK_Y));
            c.c_eq_s(0, 4);
            if c.fcc() {
                // The stick was centred: no spin.
                c.g[V0][0] = 3;
                c.cycles += 2;
                Addr(this.wrapping_add(SPIN_ENTER_STATE)).sw(c, 0);
                Addr(this.wrapping_add(SPIN_ENTER_EXIT)).sw(c, 3);
                return spin_enter_epilogue(c, frame);
            }
            c.cycles += 1;
        }
        Addr(this.wrapping_add(SPIN_ENTER_EXIT)).sw(c, 0);

        // f2 = the larger of |x| and |y|.
        c.cycles += 8;
        c.lwc1(3, rider.wrapping_add(SPIN_ENTER_STICK_X));
        c.lwc1(0, rider.wrapping_add(SPIN_ENTER_STICK_Y));
        c.abs_s(1, 3);
        c.abs_s(0, 0);
        c.c_lt_s(0, 1);
        let x_larger = c.fcc();
        c.mov_s(2, 0);
        if x_larger {
            c.cycles += 1;
            c.mov_s(2, 1);
        }

        // Spin rate from x.
        c.cycles += 4;
        c.c_eq_s(3, 4);
        if !c.fcc() {
            c.cycles += 6;
            c.lwc1(0, gp.wrapping_sub(0x753c)); // 7
            c.mul_s(0, 2, 0);
            c.max_s(0, 0, 5);
            c.swc1(0, this.wrapping_add(SPIN_ENTER_SPIN_RATE));
            c.swc1(0, rider.wrapping_add(SPIN_ENTER_RIDER_SPIN_RATE));
        }
        c.g[V0][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        let rider = r32(c, V0);

        // Flip rate from y.
        c.cycles += 4;
        c.lwc1(0, rider.wrapping_add(SPIN_ENTER_STICK_Y));
        c.c_eq_s(0, 4);
        if !c.fcc() {
            c.cycles += 7;
            c.lwc1(0, gp.wrapping_sub(0x7538)); // 7
            c.lwc1(1, gp.wrapping_sub(0x7534)); // 2/3
            c.mul_s(0, 2, 0);
            c.max_s(0, 0, 5);
            c.mul_s(0, 0, 1);
            c.swc1(0, this.wrapping_add(SPIN_ENTER_FLIP_RATE));
            c.swc1(0, rider.wrapping_add(SPIN_ENTER_RIDER_FLIP_RATE));
        }

        c.cycles += 37;
        c.g[A0][0] = c.g[S0][0];
        SPIN_ENTER_GET_SPIN_FACTOR.call(c, 0x0013_325c)?;
        let this = r32(c, S0);
        c.swc1(0, this.wrapping_add(SPIN_ENTER_FACTOR));
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        let rider = r32(c, V0);
        c.lwc1(13, rider.wrapping_add(SPIN_ENTER_STICK_X));
        c.lwc1(12, rider.wrapping_add(SPIN_ENTER_STICK_Y));
        SPIN_ENTER_SET_SPIN_DIRECTION.call(c, 0x0013_3274)?;
        crate::game::ai::ai::BE_LIBRARY_GET_INSTANCE.call(c, 0x0013_327c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        crate::game::ai::ai::BE_LIBRARY_GET_INTERFACE.call(c, 0x0013_3288)?;
        let this = r32(c, S0);
        c.g[V1][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        let rider = r32(c, V1);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(rider.wrapping_add(SPIN_ENTER_PLAYER)).lw(c);
        c.g[A2][0] = Addr(rider.wrapping_add(SPIN_ENTER_CHARACTER)).lw(c);
        crate::game::ai::ai::BE_CHAR_INT_GET_ATTRIB4_FACTOR.call(c, 0x0013_329c)?;

        // Animation speed = (1 + attrib * 0.5) * max(flip, spin) / 7.
        let this = r32(c, S0);
        c.lwc1(1, gp.wrapping_sub(0x7530));
        c.g[A1][0] = 2;
        c.g[V0][0] = Addr(this.wrapping_add(SPIN_ENTER_RIDER)).lw(c);
        let rider = r32(c, V0);
        c.mul_s(20, 0, 1);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 2);
        c.lwc1(0, rider.wrapping_add(SPIN_ENTER_RIDER_FLIP_RATE));
        c.lwc1(1, rider.wrapping_add(SPIN_ENTER_RIDER_SPIN_RATE));
        c.add_s(20, 20, 2);
        c.lwc1(3, gp.wrapping_sub(0x752c));
        c.max_s(0, 0, 1);
        c.g[A0][0] = Addr(rider.wrapping_add(SPIN_ENTER_ANIM_CTRL)).lw(c);
        c.mul_s(20, 20, 0);
        c.mul_s(20, 20, 3);
        call_ptr(c, 0x0031_1b20, 0x0013_32d8)?; // AnimController_GetLayerSequence(ctrl, 2)
        c.g[V1][0] = 1;
        let seq = r32(c, V0);
        c.swc1(20, seq.wrapping_add(SPIN_ENTER_SEQ_SPEED));
        let this = r32(c, S0);
        Addr(this.wrapping_add(SPIN_ENTER_STATE)).sw(c, 1);
        spin_enter_epilogue(c, frame)
    })
}

/// RiderSpinSub_Enter's epilogue (5 cycles).
pub(crate) fn spin_enter_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 5;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x20);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Rider sub-state SaveState: write the raw state block to a replay stream.

/// The stream's `Write(this, data, size)` virtual (vtable slot +0x8).
pub(crate) const SAVE_STATE_STREAM_WRITE: u32 = 0x8;

/// `stream->Write(self, size)` in a 0x10 frame, returning to `ret`: the
/// shape of the riders' plain `SaveState(self, stream)` methods (13
/// cycles).
pub(crate) fn rider_save_state_raw(c: &mut Cpu, size: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0]; // stream
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0]; // self
        c.g[A2][0] = size;
        let stream = c.g[A3][0];
        c.g[V1][0] = Addr(stream as u32).lw(c);
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, SAVE_STATE_STREAM_WRITE);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(stream);
        call_ptr(c, slot.func as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderRailslideSub::SaveState(RiderRailslideSub* this, Stream* s)`
/// (0x132840): writes the sub-state's 8-byte block to the replay stream.
pub fn rider_railslide_sub_save_state(c: &mut Cpu) {
    rider_save_state_raw(c, 0x8, 0x0013_2868);
}

// ---------------------------------------------------------------------------
// RiderSpinSub_Exit (0x134cb0)

/// RiderSpinSub: spin start and current angle (+0x14 / +0x1c compared).
pub(crate) const SPIN_EXIT_START: u32 = 0x14;

pub(crate) const SPIN_EXIT_ANGLE: u32 = 0x1c;

/// Rider: the spin input block cleared on exit, the two input timers
/// (+0x290/+0x29c, reset to 1/60), the pose matrix.
pub(crate) const SPIN_EXIT_RIDER_POSE: u32 = 0x110;

pub(crate) const SPIN_EXIT_TIMER_A: u32 = 0x290;

pub(crate) const SPIN_EXIT_TIMER_B: u32 = 0x29c;

pub(crate) const SPIN_EXIT_APPLY_POSE_ROTATION: Callee = Callee {
    addr: 0x0013_4dd0,
    f: ee::s01::RiderSpinSub_ApplyPoseRotation,
};

/// `this->rider` (`lw (s0 + 0x58)`).
pub(crate) fn spin_exit_rider(c: &mut Cpu) -> u64 {
    Addr(r32(c, S0).wrapping_add(SPIN_ENTER_RIDER)).lw(c)
}

/// `void RiderSpinSub::Exit(RiderSpinSub* this, int)` (0x134cb0): leave the
/// air spin. Fades out animation layer 1 (0.33 s), clears the rider's stick
/// snapshot (+0x2a4..+0x2b8), resets its input timers to 1/60, applies the
/// final pose rotation and rebuilds the world matrix; then scales the
/// rider's spin rate (+0x2dc) by 0.75 and snaps it to the quarter turn
/// above (pi/2, 3pi/2 or 5pi/2; 0 if not positive), negated when the spin
/// went backwards (angle +0x1c past start +0x14); flip rate +0x2e0 = 0.
pub fn rider_spin_sub_exit(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 42;
        let frame = Frame::open(c, 0x20);
        c.lwc1(12, gp.wrapping_sub(0x73a0)); // 0.33
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 1;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = spin_exit_rider(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(SPIN_ENTER_ANIM_CTRL)).lw(c);
        call_ptr(c, 0x0031_1e88, 0x0013_4cd4)?; // AnimController_FadeOutLayer(ctrl, 1, 0.33)

        c.g[V1][0] = spin_exit_rider(c);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(0, gp.wrapping_sub(0x739c)); // 1/60
        let rider = r32(c, V1);
        for off in [0x2a8, 0x2ac, 0x2a4] {
            Addr(rider.wrapping_add(off)).sw(c, 0);
        }
        c.g[V0][0] = spin_exit_rider(c);
        let rider = r32(c, V0);
        for off in [0x2b4, 0x2b8, 0x2b0] {
            Addr(rider.wrapping_add(off)).sw(c, 0);
        }
        c.g[V1][0] = spin_exit_rider(c);
        let rider = r32(c, V1);
        c.swc1(0, rider.wrapping_add(SPIN_EXIT_TIMER_A));
        Addr(rider.wrapping_add(SPIN_EXIT_TIMER_A + 4)).sw(c, 0);
        c.g[V0][0] = spin_exit_rider(c);
        let rider = r32(c, V0);
        c.swc1(0, rider.wrapping_add(SPIN_EXIT_TIMER_B));
        Addr(rider.wrapping_add(SPIN_EXIT_TIMER_B + 4)).sw(c, 0);
        let rider = spin_exit_rider(c) as u32;
        c.g[A1][0] = sx(rider.wrapping_add(SPIN_EXIT_RIDER_POSE));
        SPIN_EXIT_APPLY_POSE_ROTATION.call(c, 0x0013_4d20)?;
        c.g[A0][0] = spin_exit_rider(c);
        call_ptr(c, 0x0011_e098, 0x0013_4d28)?; // Rider_BuildWorldMatrix

        // Spin rate *= 0.75, snapped up to the next quarter turn.
        let gp = r32(c, GP);
        c.g[V0][0] = spin_exit_rider(c);
        let rider = r32(c, V0);
        let rate = rider.wrapping_add(SPIN_ENTER_RIDER_SPIN_RATE);
        c.lwc1(1, gp.wrapping_sub(0x7398)); // 0.75
        c.lwc1(0, rate);
        c.lwc1(2, gp.wrapping_sub(0x7394)); // 3pi/2
        c.mul_s(0, 0, 1);
        c.swc1(0, rate);
        c.g[V0][0] = spin_exit_rider(c);
        let rate = r32(c, V0).wrapping_add(SPIN_ENTER_RIDER_SPIN_RATE);
        c.lwc1(1, rate);
        c.c_lt_s(2, 1);
        c.lwc1(0, gp.wrapping_sub(0x7390)); // 5pi/2
        if c.fcc() {
            c.swc1(0, rate);
        } else {
            c.cycles += 4;
            c.lwc1(0, gp.wrapping_sub(0x738c)); // pi
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.cycles += 2;
                c.swc1(2, rate);
            } else {
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.cycles += 2;
                    c.lwc1(0, gp.wrapping_sub(0x7388)); // pi/2
                }
                c.swc1(0, rate);
            }
        }

        // Flip rate cleared; a backwards spin turns the other way.
        c.cycles += 8;
        c.g[V0][0] = spin_exit_rider(c);
        Addr(r32(c, V0).wrapping_add(SPIN_ENTER_RIDER_FLIP_RATE)).sw(c, 0);
        let this = r32(c, S0);
        c.lwc1(1, this.wrapping_add(SPIN_EXIT_ANGLE));
        c.lwc1(0, this.wrapping_add(SPIN_EXIT_START));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.cycles += 5;
            c.g[V0][0] = spin_exit_rider(c);
            let rate = r32(c, V0).wrapping_add(SPIN_ENTER_RIDER_SPIN_RATE);
            c.lwc1(0, rate);
            c.neg_s(0, 0);
            c.swc1(0, rate);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0013c5e0 RiderRailslideState_SaveState, 0x00131598
// RiderBoardPressSub_SaveState

/// The common body of the small rider-state `SaveState`s: `stream->write(
/// self, size)` (a0 = self, a1 = stream; the write is vtable slot 1 at
/// `+0x8`) returning to `ret`.
pub(crate) fn rider_state_save_self_bytes(c: &mut Cpu, size: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = size;
        c.g[V1][0] = c.lw(c.g[A3][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let write = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, write, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderBoardPressSub_SaveState(RiderBoardPressSub* self, cStream*
/// stream)` (0x131598): write the sub-state's first 0x14 bytes to the
/// stream.
pub fn rider_board_press_sub_save_state(c: &mut Cpu) {
    rider_state_save_self_bytes(c, 0x14, 0x0013_15c0);
}

// ---------------------------------------------------------------------------
// Rider state SaveState forwarders (0x13acb0, 0x12fbf0)

/// The shape of the rider states' `SaveState(self, stream)`: `stream->Write(
/// self, size)` through the stream's vtable slot 8 (`{delta, fn}`), in a
/// 0x10 frame holding `ra`; `ret` is the return address of the call.
pub(crate) fn rider_state_save_forward(c: &mut Cpu, ret: u32, size: u64) -> crate::call::Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    c.g[A3][0] = c.g[A1][0];
    let stream = c.g[A3][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = c.g[A0][0];
    c.g[A2][0] = size;
    c.g[V1][0] = c.lw(stream as u32);
    let vt = c.g[V1][0];
    let slot = crate::call::VSlot::read(c, vt, 0x8);
    c.g[A0][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(stream);
    call_ptr(c, slot.func as u32, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void RiderNaturalAirSub_SaveState(RiderNaturalAirSub* self, cPbStream*
/// stream)` (0x12fbf0): write the sub-state's 0x14 bytes.
pub fn rider_natural_air_sub_save_state(c: &mut Cpu) {
    guest(c, |c| rider_state_save_forward(c, 0x0012_fc18, 0x14))
}

pub(crate) const RESET_SUB_SIZE: u64 = 0x8;

/// `void RiderResetSub_SaveState(RiderResetSub* self, Stream* s)` (0x12f5b0): writes the
/// reset sub-state's 8 bytes to the save stream.
pub fn rider_reset_sub_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = RESET_SUB_SIZE;
        // s->Write(self, 8): vtable slot +0x8 `{delta, fn}`.
        let stream = r32(c, A3);
        c.g[V1][0] = Addr(stream).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(0x8).lh(c);
        c.g[V0][0] = Addr(vt).off(0xc).lw(c);
        let write = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, write, 0x0012_f5d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `stream->Write(self, size)`: the shared body of the small `SaveState`
/// methods. The stream's vtable slot 1 (`{delta, fn}` at +8) is called with
/// `(stream + delta, self, size)` in a 0x10 frame holding `ra`; `ret` is
/// the return address after the call.
pub(crate) fn save_state_write(c: &mut Cpu, size: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = size;
        let stream = c.g[A3][0] as u32;
        c.g[V1][0] = Addr(stream).lw(c);
        let vt = Addr(c.g[V1][0] as u32);
        c.g[A0][0] = vt.off(8).lh(c);
        c.g[V0][0] = vt.off(0xc).lw(c);
        let write = c.g[V0][0] as u32;
        c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, write, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderWipeoutSub_SaveState(RiderWipeoutSub* self, Stream* s)`
/// (0x12e708): writes the 0x80-byte state to the stream.
pub fn rider_wipeout_sub_save_state(c: &mut Cpu) {
    save_state_write(c, 0x80, 0x0012_e730);
}

/// `AI` object lists: players (inline pointer array at +0x40, count at
/// +0x7c), mission riders (+0x48, +0x80) and actors (+0x5c, +0x84).
pub(crate) const AI_SAVE_PLAYERS: u32 = 0x40;

pub(crate) const AI_SAVE_MISSION: u32 = 0x48;

pub(crate) const AI_SAVE_ACTORS: u32 = 0x5c;

pub(crate) const AI_SAVE_NUM_PLAYERS: u32 = 0x7c;

pub(crate) const AI_SAVE_NUM_MISSION: u32 = 0x80;

pub(crate) const AI_SAVE_NUM_ACTORS: u32 = 0x84;

/// Replay frame vtable: `Write(frame, src, size)` at slot 0x8.
pub(crate) const AI_SAVE_FRAME_WRITE: u32 = 0x8;

/// Rider -> object (+0x18) -> replay object (+0x77c); its vtable (+0xde8)
/// has `Save(obj, frame)` at slot 0x10.
pub(crate) const AI_SAVE_RIDER_OBJ: u32 = 0x18;

pub(crate) const AI_SAVE_OBJ_REPLAY: u32 = 0x77c;

pub(crate) const AI_SAVE_REPLAY_VT: u32 = 0xde8;

pub(crate) const AI_SAVE_REPLAY_SAVE: u32 = 0x10;

/// `WorldPainter_SaveViewState(frame)`.
pub(crate) const AI_SAVE_VIEW_STATE: u32 = 0x002e_f248;

/// One object loop of `AI_SaveState`, entered with a positive count: for
/// each entry of the inline list at `self + list`, call its replay
/// object's `Save(frame)` (vtable slot 0x10). `self` is s2, the frame s3,
/// the index s1 and the list cursor s0; the back edge (loop head `head`)
/// checks events. Leaves v0 = 0 (the failed `i < count` test).
pub(crate) fn ai_save_state_objs(
    c: &mut Cpu,
    list: u32,
    count: u32,
    head: u32,
    back: u32,
) -> crate::call::Flow {
    c.cycles += 3;
    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(list));
    c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
    loop {
        c.cycles += 14;
        c.g[A1][0] = c.g[S3][0];
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        let rider = c.g[V1][0] as u32;
        let obj = Addr(rider.wrapping_add(AI_SAVE_RIDER_OBJ)).lw(c);
        c.g[V0][0] = obj;
        let replay = Addr((obj as u32).wrapping_add(AI_SAVE_OBJ_REPLAY)).lw(c);
        c.g[A3][0] = replay;
        let vt = Addr((replay as u32).wrapping_add(AI_SAVE_REPLAY_VT)).lw(c);
        c.g[V1][0] = vt;
        let delta = Addr((vt as u32).wrapping_add(AI_SAVE_REPLAY_SAVE)).lh(c);
        let save = Addr((vt as u32).wrapping_add(AI_SAVE_REPLAY_SAVE + 4)).lw(c);
        c.g[A2][0] = save;
        c.g[A0][0] = sx((replay as u32).wrapping_add(delta as u32));
        call_ptr(c, save as u32, back)?;
        let n = Addr((c.g[S2][0] as u32).wrapping_add(count)).lw(c);
        let more = (c.g[S1][0] as i64) < (n as i64);
        c.g[V0][0] = more as u64;
        if !more {
            return Ok(());
        }
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void AI_SaveState(AI* self, cReplayFrame* frame)` (0x12b7f0): write
/// the AI block's first 0x28 bytes into the replay frame (its virtual
/// `Write`), let every player, mission rider and actor save its replay
/// state, then save the world painter's view state.
pub fn ai_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x28;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S2][0];
        let replay_frame = c.g[S3][0] as u32;
        let vt = Addr(replay_frame).lw(c);
        c.g[V0][0] = vt;
        let delta = Addr((vt as u32).wrapping_add(AI_SAVE_FRAME_WRITE)).lh(c);
        let write = Addr((vt as u32).wrapping_add(AI_SAVE_FRAME_WRITE + 4)).lw(c);
        c.g[V1][0] = write;
        c.g[A0][0] = sx(replay_frame.wrapping_add(delta as u32));
        call_ptr(c, write as u32, 0x0012_b830)?;

        // Players.
        let n = Addr((c.g[S2][0] as u32).wrapping_add(AI_SAVE_NUM_PLAYERS)).lw(c);
        if (n as i64) > 0 {
            ai_save_state_objs(
                c,
                AI_SAVE_PLAYERS,
                AI_SAVE_NUM_PLAYERS,
                0x0012_b848,
                0x0012_b870,
            )?;
            c.cycles += 1;
        }
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(AI_SAVE_NUM_MISSION)).lw(c);

        // Mission riders.
        c.cycles += 2;
        c.g[S1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            ai_save_state_objs(
                c,
                AI_SAVE_MISSION,
                AI_SAVE_NUM_MISSION,
                0x0012_b898,
                0x0012_b8c0,
            )?;
        }

        // Actors.
        c.cycles += 3;
        let n = Addr((c.g[S2][0] as u32).wrapping_add(AI_SAVE_NUM_ACTORS)).lw(c);
        c.g[V0][0] = n;
        c.g[S1][0] = 0;
        if (n as i64) > 0 {
            ai_save_state_objs(
                c,
                AI_SAVE_ACTORS,
                AI_SAVE_NUM_ACTORS,
                0x0012_b8e8,
                0x0012_b910,
            )?;
        }

        c.cycles += 9;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, AI_SAVE_VIEW_STATE, 0x0012_b928)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The replay stream interface's `Write(stream, src, size)` slot.
pub(crate) const SUB_SAVE_STATE_WRITE: u32 = 0x8;

/// The shared body of the rider sub-state `SaveState(self, stream)`
/// forwarders: `stream->Write(self, size)` through vtable slot 0x8 in a
/// 0x10 frame (the stream kept in a3, `self` passed as the source).
pub(crate) fn sub_save_state_write(c: &mut Cpu, size: u32, back: u32) -> crate::call::Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    c.g[A3][0] = c.g[A1][0];
    let stream = c.g[A3][0];
    frame.save_ra(c, 0);
    c.g[A1][0] = c.g[A0][0];
    c.g[A2][0] = sx(size);
    let vt = Addr(stream as u32).lw(c);
    c.g[V1][0] = vt;
    let delta = Addr((vt as u32).wrapping_add(SUB_SAVE_STATE_WRITE)).lh(c);
    let write = Addr((vt as u32).wrapping_add(SUB_SAVE_STATE_WRITE + 4)).lw(c);
    c.g[V0][0] = write;
    c.g[A0][0] = sx((stream as u32).wrapping_add(delta as u32));
    call_ptr(c, write as u32, back)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void RiderUberGrindSub_SaveState(RiderUberGrindSub* self, Stream* s)`
/// (0x1368e8): write the sub-state's 8 bytes into the replay stream.
pub fn rider_uber_grind_sub_save_state(c: &mut Cpu) {
    guest(c, |c| sub_save_state_write(c, 0x8, 0x0013_6910))
}

/// `void RiderFinishLineSub_SaveState(RiderFinishLineSub* self, Stream* s)`
/// (0x12c9c0): write the sub-state's 4 bytes into the replay stream.
pub fn rider_finish_line_sub_save_state(c: &mut Cpu) {
    guest(c, |c| sub_save_state_write(c, 0x4, 0x0012_c9e8))
}

// ---------------------------------------------------------------------------
// 0x00131c30 RiderCruiseSub_Exit

/// The rider's animation controller (`rider + 0x784`).
pub(crate) const RCSE_ANIM: u32 = 0x784;

/// Layer-1 anim types that the cruise state fades out on exit.
pub(crate) const RCSE_FADE_TYPES: [u64; 2] = [3, 0xd];

/// `void RiderCruiseSub_Exit(RiderSubState* self, int next)` (0x131c30):
/// unless the next sub-state is 4, when animation layer 1 plays a cruise
/// anim (type 3 or 13) fade layer 0 out over 0.1 s and set layer 1's
/// sequence weight (+0x90) to 1.
pub fn rider_cruise_sub_exit(c: &mut Cpu) {
    let frame = Frame { size: 0x20 };
    guest(c, |c| {
        c.cycles += 6;
        Frame::open(c, 0x20);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let to_4 = c.g[A1][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        let mut late = true;
        if !to_4 {
            // Layer 1's anim type (asked again for the second type).
            let mut fade = false;
            for (i, (cycles, ret_to)) in [(7u64, 0x0013_1c58u32), (6, 0x0013_1c70)]
                .into_iter()
                .enumerate()
            {
                c.cycles += cycles;
                let rider = Addr(r32(c, S0)).lw(c);
                c.g[V0][0] = rider;
                if i == 0 {
                    c.g[A1][0] = 1;
                }
                c.g[A0][0] = Addr(rider as u32).off(RCSE_ANIM).lw(c);
                call_ptr(c, 0x0031_1ae8, ret_to)?; // AnimController_GetLayerAnimEntry
                c.g[V1][0] = RCSE_FADE_TYPES[i];
                let hit = c.g[V0][0] == c.g[V1][0];
                if i == 0 {
                    c.g[A1][0] = 1;
                }
                if hit {
                    fade = true;
                    break;
                }
            }
            if fade {
                c.cycles += 12;
                let rider = Addr(r32(c, S0)).lw(c);
                c.g[V0][0] = rider;
                c.g[A1][0] = 0;
                c.lwc1(12, r32(c, GP).wrapping_sub(0x75b0)); // 0.1f
                c.g[A0][0] = Addr(rider as u32).off(RCSE_ANIM).lw(c);
                call_ptr(c, 0x0031_1e88, 0x0013_1c90)?; // AnimController_FadeOutLayer
                let rider = Addr(r32(c, S0)).lw(c);
                c.g[V0][0] = rider;
                c.g[A1][0] = 1;
                c.g[A0][0] = Addr(rider as u32).off(RCSE_ANIM).lw(c);
                call_ptr(c, 0x0031_1b20, 0x0013_1ca0)?; // AnimController_GetLayerSequence
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 0);
                c.swc1(0, r32(c, V0).wrapping_add(0x90));
            } else {
                late = false;
            }
        }
        if late {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod spin_direction {

    /// RiderSpinSub: +0x10 spin rate (±2π or 0), +0x14 flip rate (±π or 0),
    /// +0x18/+0x1c/+0x20/+0x24 accumulated angles, +0x28/+0x2c pending
    /// spin/flip deltas (folded in and cleared).
    pub const SPIN_RATE: u32 = 0x10;

    pub const FLIP_RATE: u32 = 0x14;

    pub const FLIP_ACC: u32 = 0x18;

    pub const SPIN_ACC: u32 = 0x1c;

    pub const SPIN_ACC2: u32 = 0x20;

    pub const FLIP_ACC2: u32 = 0x24;

    pub const PENDING_SPIN: u32 = 0x28;

    pub const PENDING_FLIP: u32 = 0x2c;

    /// `gp-0x734c` π, `gp-0x7348` -π, `gp-0x7344` 2π, `gp-0x7340` -2π.
    pub const GP_PI: u32 = 0x734c;

    pub const GP_NEG_PI: u32 = 0x7348;

    pub const GP_TWO_PI: u32 = 0x7344;

    pub const GP_NEG_TWO_PI: u32 = 0x7340;
}

/// `void RiderSpinSub_SetSpinDirection(RiderSpinSub* self, float spin,
/// float flip)` (0x135b30): set the flip rate from the sign of `flip`
/// (π, -π or 0) and the spin rate from the sign of `spin` (2π, -2π or 0),
/// then fold the pending spin/flip deltas into the accumulated angles and
/// clear them.
pub fn rider_spin_sub_set_spin_direction(c: &mut Cpu) {
    use spin_direction as k;
    let this = abi::arg(c, 0);
    let gp = r32(c, GP);
    // the sign of f13 (flip) picks the flip rate, f12 (spin) the spin rate
    for (arg, field, pos, neg) in [
        (13, k::FLIP_RATE, k::GP_PI, k::GP_NEG_PI),
        (12, k::SPIN_RATE, k::GP_TWO_PI, k::GP_NEG_TWO_PI),
    ] {
        c.cycles += 4;
        c.mtc1(0, 0);
        c.c_lt_s(0, arg);
        if c.fcc() {
            c.cycles += 3;
            c.lwc1(0, gp.wrapping_sub(pos));
        } else {
            c.cycles += 4;
            c.c_lt_s(arg, 0);
            if c.fcc() {
                c.cycles += 2;
                c.lwc1(0, gp.wrapping_sub(neg));
            }
        }
        c.swc1(0, this.wrapping_add(field));
    }
    c.cycles += 17;
    c.lwc1(4, this.wrapping_add(k::PENDING_SPIN));
    c.lwc1(5, this.wrapping_add(k::PENDING_FLIP));
    c.lwc1(1, this.wrapping_add(k::SPIN_ACC));
    c.lwc1(2, this.wrapping_add(k::FLIP_ACC));
    c.lwc1(3, this.wrapping_add(k::SPIN_ACC2));
    c.add_s(1, 1, 4);
    c.lwc1(0, this.wrapping_add(k::FLIP_ACC2));
    c.add_s(2, 2, 5);
    c.add_s(3, 3, 4);
    c.sw(this.wrapping_add(k::PENDING_FLIP), 0);
    c.add_s(0, 0, 5);
    c.swc1(1, this.wrapping_add(k::SPIN_ACC));
    c.swc1(2, this.wrapping_add(k::FLIP_ACC));
    c.swc1(3, this.wrapping_add(k::SPIN_ACC2));
    c.swc1(0, this.wrapping_add(k::FLIP_ACC2));
    c.sw(this.wrapping_add(k::PENDING_SPIN), 0);
    ret(c);
}

/// `RiderNaturalAirSub` fields: heading angle +0x0, speed +0x4, two
/// flags +0x8/+0xc (set to 1), a timer +0x10 (set to -1), the rider +0x14.
pub(crate) const NAT_AIR_ANGLE: u32 = 0x0;

pub(crate) const NAT_AIR_SPEED: u32 = 0x4;

pub(crate) const NAT_AIR_FLAG_A: u32 = 0x8;

pub(crate) const NAT_AIR_FLAG_B: u32 = 0xc;

pub(crate) const NAT_AIR_TIMER: u32 = 0x10;

pub(crate) const NAT_AIR_RIDER: u32 = 0x14;

/// Rider fields: the horizontal (x) and vertical (y) air velocity
/// components, and two (step, counter) pairs reset on entry.
pub(crate) const NAT_AIR_RIDER_VX: u32 = 0x1f0;

pub(crate) const NAT_AIR_RIDER_VY: u32 = 0x220;

pub(crate) const NAT_AIR_RIDER_STEP_A: u32 = 0x2a8;

pub(crate) const NAT_AIR_RIDER_COUNT_A: u32 = 0x2ac;

pub(crate) const NAT_AIR_RIDER_STEP_B: u32 = 0x2b4;

pub(crate) const NAT_AIR_RIDER_COUNT_B: u32 = 0x2b8;

/// gp-relative `.lit4` constants: -pi/2 (x = 0, y < 0), pi/2 (x = 0,
/// y >= 0), the +pi / -pi quadrant fixes for x < 0, and the step 1/30.
pub(crate) const NAT_AIR_GP_DOWN: u32 = 0x7718;

pub(crate) const NAT_AIR_GP_UP: u32 = 0x7714;

pub(crate) const NAT_AIR_GP_PLUS_PI: u32 = 0x7710;

pub(crate) const NAT_AIR_GP_MINUS_PI: u32 = 0x770c;

pub(crate) const NAT_AIR_GP_STEP: u32 = 0x7708;

/// `Math_Atan(x)` (0x31c228, through the function table).
pub(crate) const NAT_AIR_MATH_ATAN: u32 = 0x0031_c228;

/// A gp-relative float constant at gp - `off`.
pub(crate) fn nat_air_gp_const(c: &mut Cpu, off: u32) -> FpuF32 {
    Addr(r32(c, GP).wrapping_sub(off)).read_fpu(c)
}

/// `void RiderNaturalAirSub_Enter(RiderNaturalAirSub *self)` (0x12f620):
/// entering the natural-air sub-state. Speed = max(|vx|, vy), heading =
/// atan2(vy, vx) (computed with `Math_Atan` and quadrant fixes), both flags
/// 1, the timer -1, and the rider's two (step, counter) pairs reset to
/// (1/30, 0).
pub fn rider_natural_air_sub_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 22, 0x30);
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        let this = r32(c, S0);
        let rider = Addr(this.wrapping_add(NAT_AIR_RIDER)).lw(c);
        c.g[V0][0] = rider;
        let rider = rider as u32;
        let vy = Addr(rider.wrapping_add(NAT_AIR_RIDER_VY)).read_fpu(c);
        c.f[1] = vy.0;
        let mut speed = Addr(rider.wrapping_add(NAT_AIR_RIDER_VX)).read_fpu(c).abs();
        if abi::c_lt_s(c, speed, vy) {
            speed = vy;
        }
        c.f[0] = speed.0;

        c.cycles += 7;
        Addr(this.wrapping_add(NAT_AIR_SPEED)).write_fpu(c, speed);
        let zero = FpuF32(0);
        c.f[22] = 0;
        let vx = Addr(rider.wrapping_add(NAT_AIR_RIDER_VX)).read_fpu(c);
        c.f[20] = vx.0;
        let vertical = vx.eq_s(zero);
        abi::set_fcc(c, vertical);
        let vy = Addr(rider.wrapping_add(NAT_AIR_RIDER_VY)).read_fpu(c);
        c.f[21] = vy.0;

        // Where the original rejoins: whether a0 (the rider) still has to be
        // reloaded, and whether v1 = 1 is still to be set.
        let (reload_rider, set_flag) = if vertical {
            // Straight up or down (or no velocity at all).
            c.cycles += 4;
            let still = vy.eq_s(zero);
            abi::set_fcc(c, still);
            c.f[12] = vy.0;
            if still {
                (true, true)
            } else {
                c.cycles += 4;
                let up = zero.le(vy);
                abi::set_fcc(c, up);
                c.f[12] = nat_air_gp_const(c, NAT_AIR_GP_DOWN).0;
                c.g[A0][0] = Addr(this.wrapping_add(NAT_AIR_RIDER)).lw(c);
                if !up {
                    (false, true)
                } else {
                    c.cycles += 3;
                    c.f[12] = nat_air_gp_const(c, NAT_AIR_GP_UP).0;
                    c.g[V1][0] = 1;
                    (false, false)
                }
            }
        } else {
            c.cycles += 7;
            c.f[12] = vy.div(vx).0;
            call_ptr(c, NAT_AIR_MATH_ATAN, 0x0012_f6b0)?;
            let (vx, zero, vy) = (FpuF32(c.f[20]), FpuF32(c.f[22]), FpuF32(c.f[21]));
            let backwards = abi::c_lt_s(c, vx, zero);
            c.f[12] = c.f[0];
            if backwards {
                c.cycles += 3;
                let fix = if abi::c_lt_s(c, zero, vy) {
                    c.cycles += 3;
                    let pi = nat_air_gp_const(c, NAT_AIR_GP_PLUS_PI);
                    c.f[0] = pi.0;
                    FpuF32(c.f[12]) + pi
                } else {
                    c.cycles += 2;
                    let pi = nat_air_gp_const(c, NAT_AIR_GP_MINUS_PI);
                    c.f[0] = pi.0;
                    FpuF32(c.f[12]) - pi
                };
                c.f[12] = fix.0;
            }
            (true, true)
        };
        if reload_rider {
            c.cycles += 1;
            c.g[A0][0] = Addr(this.wrapping_add(NAT_AIR_RIDER)).lw(c);
        }
        if set_flag {
            c.cycles += 1;
            c.g[V1][0] = 1;
        }

        c.cycles += 18;
        let this = r32(c, S0);
        let step = nat_air_gp_const(c, NAT_AIR_GP_STEP);
        c.f[0] = step.0;
        c.g[V0][0] = u64::MAX;
        Addr(this.wrapping_add(NAT_AIR_TIMER)).sw(c, u64::MAX);
        let angle = FpuF32(c.f[12]);
        Addr(this.wrapping_add(NAT_AIR_ANGLE)).write_fpu(c, angle);
        let one = c.g[V1][0];
        Addr(this.wrapping_add(NAT_AIR_FLAG_B)).sw(c, one);
        Addr(this.wrapping_add(NAT_AIR_FLAG_A)).sw(c, one);
        let rider = r32(c, A0);
        Addr(rider.wrapping_add(NAT_AIR_RIDER_STEP_A)).write_fpu(c, step);
        Addr(rider.wrapping_add(NAT_AIR_RIDER_COUNT_A)).sw(c, 0);
        frame.restore_ra(c, 0);
        let rider = Addr(this.wrapping_add(NAT_AIR_RIDER)).lw(c);
        c.g[V0][0] = rider;
        frame.restore(c, S0, 0x10);
        frame.restore_fpr(c, 22, 0x30);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        let rider = rider as u32;
        Addr(rider.wrapping_add(NAT_AIR_RIDER_STEP_B)).write_fpu(c, step);
        Addr(rider.wrapping_add(NAT_AIR_RIDER_COUNT_B)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;

    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::control::handplantcontrol::rider_air_state_load_state;
    pub(crate) use crate::game::ai::control::handplantcontrol::rider_ground_state_load_state;
    pub(crate) use crate::game::ai::control::handplantcontrol::rider_handplant_state_load_state;
    pub(crate) use crate::game::ai::control::handplantcontrol::rider_railslide_state_load_state;
    pub(crate) use crate::game::ai::motion::wipeoutmotion::rider_wipeout_state_load_state;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_5;
    pub(crate) use crate::game::bx::bxstring2::tests::slot_2 as slot_4;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6;

    pub(crate) use crate::game::dirtysock::tags2::tests::belib_3 as belib;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;

    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_17 as SEEDS_7;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_8;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2 as globals;

    pub(crate) use crate::game::lib::kernel::tests::SEEDS;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;

    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::render::particle::tests::clear as clear_2;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_4;
    use ssx3_ee as ee;
    use ssx3_ee::s00;

    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn rider_reset_sub_is_local_player_matches() {
        check(
            ssx3_ee::s00::f_0012f588__generated,
            rider_reset_sub_is_local_player,
            300,
            &|c, seed| {
                let mut r = Rng(0x7171_7171 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let handle = DATA + 0x100;
                c.bus.write32(DATA + 0x8, handle);
                c.bus
                    .write32(handle + 0x87c, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(
                    handle + 0x870,
                    if seed % 5 == 0 {
                        0x8000_0000
                    } else {
                        r.next_u32() & 0x7fff_ffff
                    },
                );
            },
        );
    }

    #[test]
    fn rider_board_press_sub_update_phase2_matches() {
        check(
            ssx3_ee::s00::f_001304d0__generated,
            rider_board_press_sub_update_phase2,
            50,
            &|c, seed| {
                let mut r = Rng(0x8181_8181 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_board_press_sub_enter_matches() {
        check(
            ssx3_ee::s00::f_0012fc60,
            rider_board_press_sub_enter,
            200,
            &|c, seed| {
                let mut r = Rng(0x1001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x14, DATA + 0x400);
            },
        );
    }

    #[test]
    fn rider_cruise_sub_enter_matches() {
        check(
            ssx3_ee::s00::f_00131608,
            rider_cruise_sub_enter,
            200,
            &|c, seed| {
                let mut r = Rng(0x1002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, DATA + 0x400);
            },
        );
    }

    #[test]
    fn rider_railslide_sub_exit_matches() {
        check(
            ssx3_ee::s01::f_00132048,
            rider_railslide_sub_exit,
            200,
            &|c, seed| {
                let mut r = Rng(0x1003 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x7594), r.next_u32());
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x8, DATA + 0x400);
            },
        );
    }

    #[test]
    fn rider_handplant_sub_exit_matches() {
        check(
            ssx3_ee::s01::f_00132f98,
            rider_handplant_sub_exit,
            200,
            &|c, seed| {
                let mut r = Rng(0x1004 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x754c), r.next_u32());
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x14, DATA + 0x400);
            },
        );
    }

    // -------------------------------------------------------- AI race state

    /// The dispatch table at 0x450000 + 0x6ca0, valid or pointed at an
    /// unknown address.
    pub(crate) fn race_state_table(c: &mut Cpu, seed: u32) {
        const VALID: [u32; 7] = [
            0x0011_3b74,
            0x0011_3b94,
            0x0011_3ba8,
            0x0011_3bbc,
            0x0011_3bd0,
            0x0011_3be4,
            0x0011_3bf8,
        ];
        for (i, v) in VALID.iter().enumerate() {
            let val = if seed % 3 == 0 { 0xdead_beef } else { *v };
            c.bus.write32(0x0045_0000 + 0x6ca0 + 4 * i as u32, val);
        }
    }

    #[test]
    fn ai_select_race_state_matches() {
        check(
            ssx3_ee::s00::f_00113b48,
            ai_select_race_state,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 73, 0);
                race_state_table(c, seed);
                c.pc = 0x0011_3b48;
                let ai = DATA + 0x400;
                c.bus.write32(ai, 1 + r.next_u32() % 8);
                c.bus.write32(ai + 0x98, r.next_u32());
                c.g[A0][0] = ai as u64;
                c.g[A1][0] = (seed % 10) as u64;
            },
        );
    }

    // ---------------------------------------------------- AI setup/teardown

    /// An AI object at `DATA + 0x400` with the rider counts zeroed (so the
    /// destroy/create/reset callees take their short paths), plus noise.
    pub(crate) fn ai_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        let ai = DATA + 0x400;
        fill(c, &mut r, ai + 0x70, 0x30);
        for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x94] {
            c.bus.write32(ai + off, 0);
        }
        c.bus.write32(ai + 0xa4, DATA + 0x1000);
        c.bus.write32(ai + 0xa8, DATA + 0x2000);
        c.g[A0][0] = ai as u64;
        r
    }

    #[test]
    fn ai_remove_actor_riders_matches() {
        check(
            ssx3_ee::s00::f_0012b000,
            ai_remove_actor_riders,
            SEEDS,
            &|c, seed| {
                ai_setup(c, seed, 77, 2);
            },
        );
    }

    #[test]
    fn ai_remove_computer_riders_matches() {
        check(
            ssx3_ee::s00::f_0012b030,
            ai_remove_computer_riders,
            SEEDS,
            &|c, seed| {
                ai_setup(c, seed, 78, 2);
            },
        );
    }

    #[test]
    fn ai_remove_mission_riders_matches() {
        check(
            ssx3_ee::s00::f_0012b060,
            ai_remove_mission_riders,
            SEEDS,
            &|c, seed| {
                ai_setup(c, seed, 79, 2);
            },
        );
    }

    #[test]
    fn ai_dtor_matches() {
        check(ssx3_ee::s00::f_001287b8, ai_dtor, SEEDS, &|c, seed| {
            let mut r = ai_setup(c, seed, 80, 3);
            c.g[A1][0] = (r.next_u32() & 1) as u64;
        });
    }

    /// Build `count` rider pointers at `ai + base_off`, each with the vtable
    /// chain the destructor walks; the destructor target is an unregistered
    /// address that lands in the fake-callee hook.
    pub(crate) fn ai_rider_entries(c: &mut Cpu, base_off: u32, count: u32) {
        let ai = DATA + 0x400;
        for i in 0..count {
            let rider = DATA + 0x2000 + i * 0x100;
            let vtobj = DATA + 0x8000 + i * 0x100;
            let vt = DATA + 0x9000;
            c.bus.write32(ai + base_off + 4 * i, rider);
            c.bus.write32(rider + 0x18, vtobj);
            c.bus.write32(vtobj + 0x6c0, vt);
            c.bus.write16(vt + 8, 0);
            c.bus.write32(vt + 0xc, 0x0000_5000);
        }
    }

    #[test]
    fn ai_destroy_computer_riders_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 81, 1);
            let ai = DATA + 0x400;
            let count = 1 + r.next_u32() % 8;
            c.bus.write32(ai + 0x80, count);
            c.bus.write32(ai + 0x78, count + (r.next_u32() % 4));
            c.bus.write32(ai + 0x14, r.next_u32());
            ai_rider_entries(c, 0x48, count);
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_ae38)
        };
        check_events(
            ssx3_ee::s00::f_0012ae38,
            ai_destroy_computer_riders,
            SEEDS,
            &setup,
        );
    }

    #[test]
    fn ai_destroy_actor_riders_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 82, 1);
            let ai = DATA + 0x400;
            let count = 1 + r.next_u32() % 6;
            c.bus.write32(ai + 0x84, count);
            c.bus.write32(ai + 0x78, count + (r.next_u32() % 4));
            c.bus.write32(ai + 0x8c, r.next_u32());
            ai_rider_entries(c, 0x5c, count);
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_aed0)
        };
        check_events(
            ssx3_ee::s00::f_0012aed0,
            ai_destroy_actor_riders,
            SEEDS,
            &setup,
        );
    }

    #[test]
    fn ai_destroy_mission_riders_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 83, 1);
            let ai = DATA + 0x400;
            let count = 1 + r.next_u32() % 4;
            c.bus.write32(ai + 0x88, count);
            c.bus.write32(ai + 0x78, count + (r.next_u32() % 4));
            c.bus.write32(ai + 0x90, r.next_u32());
            ai_rider_entries(c, 0x70, count);
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_af68)
        };
        check_events(
            ssx3_ee::s00::f_0012af68,
            ai_destroy_mission_riders,
            SEEDS,
            &setup,
        );
    }

    #[test]
    fn ai_destroy_all_riders_matches() {
        check(
            ssx3_ee::s00::f_0012b090,
            ai_destroy_all_riders,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 84, 4);
                let ai = DATA + 0x400;
                fill(c, &mut r, ai, 0x60);
                for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88] {
                    c.bus.write32(ai + off, 0);
                }
                c.bus.write32(DATA + 0x100, r.next_u32()); // free-anim slot 0 = null
                c.bus.write32(GAME_GP + 0xd08, 0);
                c.g[A0][0] = ai as u64;
            },
        );
    }

    #[test]
    fn ai_free_anim_sources_matches() {
        check(
            ssx3_ee::s00::f_0012b200,
            ai_free_anim_sources,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 85, 3);
                c.bus.write32(GAME_GP + 0xd08, 0);
                c.g[A0][0] = (DATA + 0x400) as u64;
            },
        );
    }

    #[test]
    fn ai_reset_riders_matches() {
        check(
            ssx3_ee::s00::f_001297c8,
            ai_reset_riders,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 87, 2);
                let ai = DATA + 0x400;
                fill(c, &mut r, ai, 0x100);
                c.bus.write32(ai + 0x78, 0);
                c.bus.write32(ai + 0x88, 0);
                c.g[A0][0] = ai as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn ai_restart_matches() {
        check(ssx3_ee::s00::f_00129768, ai_restart, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 88, 3);
            let ai = DATA + 0x400;
            fill(c, &mut r, ai, 0x100);
            c.bus.write32(ai + 0x78, 0);
            c.bus.write32(ai + 0x88, 0);
            // Focus and danger managers with zero counts / in-range fields.
            let focus = DATA + 0x3000;
            let danger = DATA + 0x2000;
            fill(c, &mut r, focus, 0x120);
            fill(c, &mut r, danger, 0xc00);
            c.bus.write32(focus + 0xd8, 0);
            c.bus.write32(ai + 0xa4, focus);
            c.bus.write32(ai + 0xa8, danger);
            c.g[A0][0] = ai as u64;
            c.g[A1][0] = (r.next_u32() % 10) as u64;
        });
    }

    #[test]
    fn ai_refresh_rider_overlaps_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 89, 1);
            let ai = DATA + 0x400;
            let count = r.next_u32() % 4;
            c.bus.write32(ai + 0x78, count);
            for i in 0..count {
                let rider = DATA + 0x2000 + i * 0x100;
                c.bus.write32(ai + 0x28 + 4 * i, rider);
            }
            c.g[A0][0] = ai as u64;
            schedule(c, &mut r, seed, 0x0012_b788)
        };
        check_events(
            ssx3_ee::s00::f_0012b788,
            ai_refresh_rider_overlaps,
            SEEDS,
            &setup,
        );
    }

    /// The song instance / process / block a wrapper loads from its object.
    pub(crate) const CHILD: u32 = DATA + 0x1000;

    #[test]
    fn ai_set_sync_words_matches() {
        check(
            s00::f_0012bac0__generated,
            ai_set_sync_words,
            200,
            &|c, seed| {
                let mut r = Rng(0x12ba_c0aa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(CHILD);
                clear(c, OBJ, 0x40);
                c.bus.write32(CHILD, r.next_u32());
                c.bus.write32(CHILD + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn ai_get_sync_words_matches() {
        check(
            s00::f_0012baf0__generated,
            ai_get_sync_words,
            200,
            &|c, seed| {
                let mut r = Rng(0x12ba_f0aa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(CHILD);
                clear(c, OBJ, 0x40);
                clear(c, CHILD, 0x40);
                c.bus.write32(OBJ + 0x20, r.next_u32());
                c.bus.write32(OBJ + 0x24, r.next_u32());
            },
        );
    }

    pub(crate) const SEEDS_2: u32 = 40;

    /// The lazily built BELibrary singleton (`gp - 0x1f04`): a zeroed dummy so
    /// the file/allocator paths fail fast instead of waiting on the disc.
    pub(crate) fn be_library(c: &mut Cpu) {
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }

    /// The shared interface-forwarder fixture: `a1` is an object whose first
    /// word is a vtable built by [`fakes::vtable`] (slot 0x10 -> a `call_ptr`
    /// target in empty RAM, so it lands in the fake hook), `a0` the receiver.
    pub(crate) fn virtual_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32) {
        let mut r = machine(c, seed, salt, 2);
        be_library(c);
        let obj = DATA + 0x800;
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(obj);
        let vt = fakes::vtable(c, &mut r, &[slot]);
        c.bus.write32(obj, vt);
    }

    #[test]
    fn ai_reset_riders_and_order_matches() {
        check(
            ee::s00::f_00128a10__generated,
            ai_reset_riders_and_order,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x128a_10, 2);
                be_library(c);
                // No riders (+0x78) and no missions (+0x88): both reset walks skip.
                c.g[A0][0] = sx(DATA + 0x400);
            },
        );
    }

    macro_rules! load_state_tests {
        ($(($test:ident, $func:ident, $shard:ident, $addr:ident, $salt:expr)),* $(,)?) => {
            $(
                #[test]
                fn $test() {
                    check(ee::$shard::$addr, $func, SEEDS_2, &|c, seed| {
                        virtual_setup(c, seed, $salt, 0x10);
                    });
                }
            )*
        };
    }

    load_state_tests! {
        (rider_gate_sub_load_state_matches, rider_gate_sub_load_state, s00, f_0012c590, 0x12c5_90),
        (rider_finish_line_sub_load_state_matches, rider_finish_line_sub_load_state, s00, f_0012c9f8, 0x12c9_f8),
        (rider_wipeout_sub_load_state_matches, rider_wipeout_sub_load_state, s00, f_0012e740, 0x12e7_40),
        (rider_reset_sub_load_state_matches, rider_reset_sub_load_state, s00, f_0012f5e8, 0x12f5_e8),
        (rider_natural_air_sub_load_state_matches, rider_natural_air_sub_load_state, s00, f_0012fc28, 0x12fc_28),
        (rider_board_press_sub_load_state_matches, rider_board_press_sub_load_state, s00, f_001315d0, 0x1315_d0),
        (rider_railslide_sub_load_state_matches, rider_railslide_sub_load_state, s01, f_00132878, 0x1328_78),
        (rider_handplant_sub_load_state_matches, rider_handplant_sub_load_state, s01, f_001330f0, 0x1330_f0),
        (rider_spin_sub_load_state_matches, rider_spin_sub_load_state, s01, f_00136230, 0x1362_30),
        (rider_uber_grind_sub_load_state_matches, rider_uber_grind_sub_load_state, s01, f_00136920, 0x1369_20),
        (rider_wipeout_state_load_state_matches, rider_wipeout_state_load_state, s01, f_00138b10, 0x138b_10),
        (rider_handplant_state_load_state_matches, rider_handplant_state_load_state, s01, f_001399a8, 0x1399_a8),
        (rider_air_state_load_state_matches, rider_air_state_load_state, s01, f_0013ace8, 0x13ac_e8),
        (rider_railslide_state_load_state_matches, rider_railslide_state_load_state, s01, f_0013c618, 0x13c6_18),
        (rider_ground_state_load_state_matches, rider_ground_state_load_state, s01, f_0013f880, 0x13f8_80),
    }

    // -----------------------------------------------------------------------
    // AI_PathDataInitIfEarlySection.

    #[test]
    fn ai_path_data_init_if_early_section_matches() {
        check(
            ssx3_ee::s00::f_0012a490__generated,
            ai_path_data_init_if_early_section,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12a4_90, 0);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x1000;
                let manager = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, manager);
                // The logical-section table at 0x442168 (all zero in a fresh
                // machine): entry 0 matches a zero id, nothing matches a
                // non-zero one (the search then reports "not found", 0x32).
                let section = if seed % 2 == 0 { 0 } else { 0x9999 };
                c.bus.write32(manager + 0x78, section);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    (r.next_u32() | 1) as u64
                };
            },
        );
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
    fn ai_setup_computer_riders_matches() {
        check(
            ssx3_ee::s00::f_00128958__generated,
            ai_setup_computer_riders_2,
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
    pub(crate) fn spin_sub_test(generated: GuestFn, hand: GuestFn, kind: u32, salt: u64) {
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

    #[test]
    fn rider_gate_sub_can_start_matches() {
        check(
            ee::s00::f_0012c078__generated,
            rider_gate_sub_can_start,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12_c078, 0);
                let world = DATA + 0x4000;
                let mgr = DATA + 0x5000;
                let race = DATA + 0x6000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, mgr);
                c.bus.write32(mgr + 0xc, race);
                c.bus
                    .write32(race, if seed % 3 == 0 { 5 } else { r.next_u32() % 8 });
                let gate = DATA + 0x1000;
                let timer = DATA + 0x2000;
                c.bus.write32(gate + 0x14, timer);
                c.bus.write32(
                    timer + 0x470,
                    match seed % 4 {
                        0 => 0xbf80_0000,
                        1 => 0,
                        _ => r.next_u32(),
                    },
                );
                c.g[A0][0] = sx(gate);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    // ----------------------------------------------------------- rider gate

    #[test]
    fn ridergate_sub_is_start_time_reached_matches() {
        check(
            ee::s00::f_0012c028__generated,
            ridergate_sub_is_start_time_reached,
            80,
            &|c, seed| {
                let mut r = Rng(0x12c0_2844 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let (module, world, race) = (DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                let gate = DATA + 0x4000;
                let rider = DATA + 0x5000;
                c.bus.write32(gp_at(-0x848), module);
                c.bus.write32(module + 0x84, world);
                c.bus.write32(world + 0xc, race);
                c.bus
                    .write32(race, if seed % 3 == 0 { 5 } else { seed % 7 });
                c.bus.write32(race + 0xc, r.next_u32() % 100_000);
                c.bus.write32(gp_at(-0x7840), 0.016_666_668f32.to_bits());
                c.bus.write32(gate + 0x14, rider);
                c.bus.write32(rider + 0xb30, r.next_u32() % 2000);
                c.g[A0][0] = sx(gate);
            },
        );
    }

    /// The BELibrary singleton (gp-0x1f04) already exists; its interface slot
    /// 0 (`gp-0x1efc`) is an object whose vtable slot at `+8` is a fake.
    pub(crate) fn belibrary(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x1000);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        let vt = vtable(c, r, &[8]);
        c.bus.write32(0x0053_08a8, DATA + 0x1100);
        c.bus.write32(DATA + 0x1104, vt);
    }

    #[test]
    fn ai_setup_actor_riders_matches() {
        check(
            ssx3_ee::s00::f_00128998__generated,
            ai_setup_actor_riders,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x128998, 2);
                belibrary(c, &mut r);
                c.bus.write8(0x0053_5c11, (seed % 2) as u8);
                let ai = DATA;
                // No new riders to spawn: `AI_CreateActorRiders` breaks at once.
                c.bus.write32(ai + 0x84, 0x10);
                c.bus.write32(ai + 0x94, 0x10);
                c.bus.write32(ai + 0x78, 0);
                c.g[A0][0] = ai as u64;
                c.g[A1][0] = 0x10;
            },
        );
    }

    #[test]
    fn rider_board_press_sub_cancel_matches() {
        check(
            ee::s00::f_00131348__generated,
            rider_board_press_sub_cancel,
            SEEDS_3,
            &|c, seed| {
                let mut r = common(c, seed, 0x131348, 4);
                let this = DATA + 0x100;
                let rider = DATA + 0x2000;
                clear_2(c, this, 0x100);
                clear_2(c, rider, 0x800 / 4);
                c.bus.write32(this + 0x14, rider);
                c.bus.write32(rider + 0x330, (seed % 4) as u32);
                c.bus.write32(rider + 0x790, DATA + 0x3000);
                c.g[A0][0] = sx(this);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn rider_spin_sub_get_spin_hud_blend_matches() {
        check(
            ee::s01::f_00136100__generated,
            rider_spin_sub_get_spin_hud_blend,
            200,
            &|c, seed| {
                let mut r = Rng(0x1361_00d1 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x18, r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x7304), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x7300), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x72fc), r.edge_f32());
            },
        );
    }

    pub(crate) fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 8) ^ salt)
    }

    /// Scrambled registers, game `gp`, a zeroed data area, and four pointers
    /// into it. Enough for the pure leaves (loads/stores within `DATA`).
    pub(crate) fn leaf_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = rng(seed, salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        for i in 0..0x400 {
            c.bus.write32(DATA + i * 4, r.next_u32());
        }
        for i in 0..4 {
            c.g[[A0, A1, A2, A3][i]][0] = (DATA + (r.next_u32() & 0x3f0)) as u64;
        }
    }

    pub(crate) fn leaf(gen: GuestFn, hand: GuestFn, salt: u64) {
        check(gen, hand, SEEDS, &|c, seed| leaf_setup(c, seed, salt));
    }

    /// A test machine with fake call/jump-out/syscall hooks.
    pub(crate) fn fake_check(
        gen: GuestFn,
        hand: GuestFn,
        salt: u64,
        setup: &dyn Fn(&mut Cpu, u32, &mut Rng),
    ) {
        check(gen, hand, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            setup(c, seed, &mut r);
        });
    }

    /// [`check`] plus soft state and LO/HI: the replacement reproduces every
    /// register write of the generated body, so the caller-saved temporaries
    /// and the multiply pipeline must match too.
    pub(crate) fn strict_check(
        gen: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(gen, hand, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard {:#?} soft {:#?} lohi {:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    #[test]
    fn rider_prewind_sub_exit_matches() {
        leaf(
            ssx3_ee::s00::f_0012e9b0__generated,
            rider_prewind_sub_exit,
            12,
        );
    }

    // ------------------------------------------ RiderWipeoutSub anim selectors

    pub(crate) fn anim_object(c: &mut Cpu, id: u32) {
        c.g[ssx3_ee_rt::cpu::reg::SP][0] = (STACK - 0x100) as u64;
        c.bus.write32(DATA + 0x80, DATA + 0x200);
        c.bus.write32(DATA + 0x984, DATA + 0x300);
        c.bus.write32(DATA + 0x308, id);
        c.bus.write32(DATA, 0);
    }

    #[test]
    fn rider_wipeout_sub_play_end_anim_a_matches() {
        fake_check(
            ssx3_ee::s00::f_0012df48__generated,
            rider_wipeout_sub_play_end_anim_a,
            48,
            &|c, seed, _r| {
                let arms = [
                    0x0012_df98,
                    0x0012_dfa0,
                    0x0012_dfa8,
                    0x0012_dfb0,
                    0x0012_dfb8,
                    0x0012_dfc0,
                    0x0012_dfc8,
                    0x0012_dfd0,
                    0x0012_dfd8,
                    0x0012_dfe0,
                ];
                let ids = [
                    0x185, 0x186, 0x187, 0x188, 0x189, 0x18a, 0x18b, 0x18c, 0x18d, 0x18e,
                ];
                match seed % 3 {
                    0 => {
                        // In range; the (empty) end-anim table sends it to jump_out.
                        anim_object(c, 0x16a);
                        c.g[A0][0] = DATA as u64;
                    }
                    1 => {
                        // Out of table range and equal ids: straight to epilogue.
                        anim_object(c, 0);
                        c.g[A0][0] = DATA as u64;
                    }
                    _ => {
                        // Resume at a case block with the same id already playing.
                        let k = (seed as usize / 3) % arms.len();
                        c.pc = arms[k];
                        c.g[A2][0] = ids[k];
                    }
                }
            },
        );
    }

    #[test]
    fn rider_wipeout_sub_select_tumble_anim_matches() {
        fake_check(
            ssx3_ee::s00::f_0012da88__generated,
            rider_wipeout_sub_select_tumble_anim,
            49,
            &|c, seed, _r| {
                let arm = [
                    0x0012_dae8,
                    0x0012_daf4,
                    0x0012_db00,
                    0x0012_db0c,
                    0x0012_db18,
                    0x0012_db24,
                    0x0012_db30,
                    0x0012_db3c,
                    0x0012_db48,
                    0x0012_db54,
                    0x0012_db60,
                    0x0012_db6c,
                    0x0012_db78,
                    0x0012_db84,
                    0x0012_db90,
                    0x0012_db9c,
                    0x0012_dba8,
                    0x0012_dbb4,
                    0x0012_dbc0,
                    0x0012_dbcc,
                    0x0012_dbd8,
                    0x0012_dbe4,
                    0x0012_dbf0,
                    0x0012_dbfc,
                    0x0012_dc08,
                    0x0012_dc14,
                    0x0012_dc20,
                    0x0012_dc2c,
                    0x0012_dc38,
                    0x0012_dc44,
                    0x0012_dc50,
                    0x0012_dc5c,
                    0x0012_dc68,
                ];
                let a1 = [
                    0x16a, 0x16b, 0x16c, 0x170, 0x16e, 0x16f, 0x172, 0x173, 0x16d, 0x171, 0x16a,
                    0x16b, 0x16c, 0x170, 0x16e, 0x173, 0x171, 0x16d, 0x16f, 0x172, 0x170, 0x171,
                    0x16c, 0x16e, 0x173, 0x170, 0x170, 0x172, 0x16b, 0x16a, 0x170, 0x172, 0x174,
                ];
                match seed % 3 {
                    0 => {
                        // Anim id 0 has a non-empty alternate column: dispatch to
                        // the (empty) table -> jump_out.
                        anim_object(c, 0x148);
                        c.g[A0][0] = DATA as u64;
                    }
                    1 => {
                        // Out of table range and equal ids: straight to epilogue.
                        anim_object(c, 0);
                        c.g[A0][0] = DATA as u64;
                    }
                    _ => {
                        let k = (seed as usize / 3) % arm.len();
                        c.pc = arm[k];
                        c.g[S0][0] = 0; // keep the primary id
                        c.g[A2][0] = a1[k];
                    }
                }
            },
        );
    }

    #[test]
    fn rider_wipeout_sub_switch_to_anim_set_a_matches() {
        strict_check(
            ssx3_ee::s00::f_0012dd98__generated,
            rider_wipeout_sub_switch_to_anim_set_a,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 51, 1);
                c.pc = 0x0012_dd98;
                // `w + 0x80` -> anim player, its `+0x784` the layer controller and
                // its `+0x150` the "this animation is already playing" flag.
                c.bus
                    .write32(DATA + 0x350, if seed % 3 == 0 { 1 } else { 0 });
                let id = match seed % 4 {
                    0 => 0x174,
                    1 => 0x175 + (seed as u32 % 0xf),
                    2 => 0x148,
                    _ => r.next_u32() % 0x200,
                };
                anim_object(c, id);
                c.g[A0][0] = sx(DATA);
                // The animation-id table the dispatch reads (`.rodata 0x4580a0`):
                // ten id blocks, then the "keep the id" entry for 0x17f and the
                // duplicate blocks for 0x180..0x183.
                put_words(
                    c,
                    0x0045_80a0,
                    &[
                        0x0012_ddf8,
                        0x0012_de00,
                        0x0012_de08,
                        0x0012_de10,
                        0x0012_de18,
                        0x0012_de20,
                        0x0012_de28,
                        0x0012_de30,
                        0x0012_de38,
                        0x0012_de40,
                        0x0012_de4c,
                        0x0012_de28,
                        0x0012_de30,
                        0x0012_de38,
                        0x0012_de40,
                    ],
                );
                // A restart calls AnimController_PlayAnim, whose first act is the
                // layer getter through the controller's vtable (a fake). Unwinding
                // there keeps the animation cache and the DLHeap out of the test,
                // so a free/consolidate can never run on fixture data.
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(DATA + 0x300 + 0x58, vt);
                c.bus.write32(LOG_UNWIND_AT, 0);
            },
        );
    }

    #[test]
    fn ai_load_next_rider_matches() {
        check(
            ee::s00::f_001296f8__generated,
            ai_load_next_rider,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x0012_96f8, 3);
                c.g[A0][0] = OBJ as u64;
                let count = seed % 4;
                w(c, OBJ + 0x78, count);
                for i in 0..count {
                    let rider = DATA + 0x3000 + 0x900 * i;
                    w(c, OBJ + 0x28 + 4 * i, rider);
                    let loaded = if seed % 2 == 0 { 7 } else { 0 };
                    w(c, rider + 0x880, loaded);
                }
            },
        );
    }

    /// `ScriptList_GetFieldB0`: returns `*(a0 + 0xb0)`; used to hand a vtable
    /// method a controlled non-zero result cheaply.
    pub(crate) const LOAD_B0: u32 = 0x0027_72b8;

    pub(crate) fn w16(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }

    /// `{ short delta; short; void (*fn)(); }` at `vt + off`.
    pub(crate) fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16, func: u32) {
        w16(c, vt + off, delta as u16);
        w32(c, vt + off + 4, func);
    }

    // ------------------------------------------------------------------
    // Wave 36, batch 2.

    #[test]
    fn rider_gate_sub_update_wait_push_matches() {
        check(
            ssx3_ee::s00::f_0012c0c0__generated,
            rider_gate_sub_update_wait_push,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12c0c0, 0);
                let g = DATA + 0x1000;
                let bits = DATA + 0x2000;
                c.g[A0][0] = g as u64;
                c.g[A1][0] = bits as u64;
                w32(c, GAME_GP.wrapping_sub(0x783c), 0.032_258_064f32.to_bits());
                let signed = (seed as i32 % 17) - 8;
                let raw = ((signed as u32) & 0x3f) << 12 | (r.next_u32() & 0xfff);
                w32(c, bits, raw);
                let model = DATA + 0x3000;
                let anim = DATA + 0x4000;
                w32(c, g + 0x14, model);
                w32(c, model + 0x784, anim);
                let vt = DATA + 0x5000;
                w32(c, anim + 0x58, vt);
                slot(c, vt, 0x20, 0, LOAD_B0);
                w32(c, anim + 0xb0, 0x207);
            },
        );
    }

    #[test]
    fn ai_computer_riders_have_render_slots_matches() {
        check(
            ee::s00::f_0012abd0__generated,
            ai_computer_riders_have_render_slots,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x12_abd0, 3);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                let n = (seed % 4) as u32;
                w(c, DATA + 0x2000 + 0x80, n);
                for i in 0..4u32 {
                    let rider = DATA + 0x2400 + 0x40 * i;
                    w(c, DATA + 0x2000 + 0x48 + 4 * i, rider);
                    w(c, rider + 0x18, DATA + 0x2600 + 0x40 * i);
                    w(
                        c,
                        DATA + 0x2600 + 0x40 * i + 0x4,
                        if i % 2 == 0 { 1 } else { 0 },
                    );
                }
                c.pc = 0x0012_abd0;
            },
        );
    }

    #[test]
    fn ai_actor_riders_have_render_slots_matches() {
        check(
            ee::s00::f_0012acf8__generated,
            ai_actor_riders_have_render_slots,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x12_acf8, 3);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                let n = (seed % 4) as u32;
                w(c, DATA + 0x2000 + 0x84, n);
                for i in 0..4u32 {
                    let rider = DATA + 0x2400 + 0x40 * i;
                    w(c, DATA + 0x2000 + 0x5c + 4 * i, rider);
                    w(c, rider + 0x18, DATA + 0x2600 + 0x40 * i);
                    w(
                        c,
                        DATA + 0x2600 + 0x40 * i + 0x4,
                        if i % 2 == 0 { 1 } else { 0 },
                    );
                }
                c.pc = 0x0012_acf8;
            },
        );
    }

    #[test]
    fn ai_get_sync_checksum_matches() {
        check(
            ee::s00::f_0012b420__generated,
            ai_get_sync_checksum,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x12b420, 0);
                be_library_2(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                let n = seed % 4;
                w(c, obj + 0x78, n);
                for i in 0..n {
                    w(c, obj + 0x28 + 4 * i, DATA + 0x1000 + 0x40 * i);
                    w(c, DATA + 0x1000 + 0x40 * i + 4, seed + i);
                }
            },
        );
    }

    #[test]
    fn ai_reset_rider_stats_matches() {
        check(
            ee::s00::f_0012b180__generated,
            ai_reset_rider_stats,
            24,
            &|c, seed| {
                let _ = base_2(c, seed, 0x12b1_80, 2);
                let self_ = DATA;
                let n = (seed % 4) as u32;
                w(c, self_ + 0x78, n);
                for i in 0..n {
                    let rider = DATA + 0x1000 + 0x100 * i;
                    w(c, self_ + 0x28 + 4 * i, rider);
                    w(c, rider + 0x790, DATA + 0x2000 + 0x100 * i);
                }
                c.g[A0][0] = self_ as u64;
            },
        );
    }

    #[test]
    fn rider_board_press_sub_settle_slew_matches() {
        use crate::difftest::put_words;
        check(
            ee::s00::f_001313a8__generated,
            rider_board_press_sub_settle_slew,
            24,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1313_a8, 0);
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

    /// `RiderBoardPressSub_SettleSlew` writes the slew's `maxStep` from the
    /// rider's time scale and 0.025 **per tick**: at 120/240 Hz the step is
    /// halved/quartered, so the board press settles in the same real time.
    #[test]
    fn rider_board_press_sub_settle_slew_follows_the_rate() {
        use crate::tick;
        const STATE: u32 = 0x0080_0000;
        const RIDER: u32 = 0x0080_1000;
        // The game's 0.025 at the three sites (gp-0x7600/-0x75fc/-0x75f8).
        const STEP_60: u32 = 0x3ccc_ccce;
        let step = |hz: u32, amount: f32| -> (u32, u32) {
            let mut c = crate::difftest::fresh();
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = STATE as u64;
            c.bus.write32(STATE + 0x14, RIDER);
            c.bus.write32(RIDER + 0x280, amount.to_bits());
            c.bus.write32(RIDER + 0x300, 1.0f32.to_bits()); // time scale
            for off in [0x7600u32, 0x75fc, 0x75f8] {
                c.bus.write32(GAME_GP.wrapping_sub(off), STEP_60);
            }
            tick::set_for_test(&mut c, hz);
            rider_board_press_sub_settle_slew(&mut c);
            (c.bus.read32(RIDER + 0x284), c.bus.read32(RIDER + 0x288))
        };
        // 60 Hz: the step as stored, and the snap direction of each branch.
        assert_eq!(step(60, 0.2), (STEP_60, 0), "below");
        assert_eq!(step(60, 0.8), (STEP_60, 0x3f80_0000), "above");
        // Below -0.5 (the clamp) it snaps down; exactly -0.5 takes the
        // `|slew| < 0.5` branch and leaves the snap direction at 0.
        assert_eq!(step(60, -0.5), (STEP_60, 0), "at the clamp");
        assert_eq!(step(60, -0.6), (STEP_60, 0xbf80_0000), "negative");
        for (hz, want) in [(120u32, 0x3c4c_ccceu32), (240, 0x3bcc_ccce)] {
            for amount in [0.2f32, 0.8, -0.6] {
                let (s, _) = step(hz, amount);
                assert_eq!(s, want, "{hz} Hz, amount {amount}");
            }
        }
    }

    #[test]
    fn ai_update_rider_poses_matches() {
        check(
            ee::s00::f_00129160__generated,
            ai_update_rider_poses,
            40,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1291, 3);
                let wld = DATA;
                let n = (seed % 3) as u32;
                w(c, wld + 0x78, n);
                for i in 0..3u32 {
                    w(
                        c,
                        wld + 0x28 + 4 * i,
                        if i < n { DATA + 0x1000 + 0x800 * i } else { 0 },
                    );
                }
                fill(c, &mut r, DATA + 0x1000, 0x40);
                fill(c, &mut r, DATA + 0x1800, 0x40);
                c.g[A0][0] = sx(wld);
            },
        );
    }

    pub(crate) const RIDER: u32 = DATA + 0x14000;

    pub(crate) fn wf(c: &mut Cpu, a: u32, f: f32) {
        c.bus.write32(a, f.to_bits());
    }

    // ---------------------------------------------------------------- rider

    #[test]
    fn rider_railslide_sub_set_rail_matches() {
        check(
            ee::s01::f_00132620__generated,
            rider_railslide_sub_set_rail,
            10,
            &|c, seed| {
                let _ = base(c, seed, 0x132620, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => u64::MAX,
                    1 => 7,
                    _ => (seed % 5) as u64,
                };
                w(c, OBJ + 8, RIDER);
                w(c, OBJ + 4, (seed % 4) as u32);
                wf(c, RIDER + 0x2f0, if seed % 2 == 0 { 0.0 } else { 1.5 });
                w(c, RIDER + 0xb2c, if seed % 3 == 0 { 2 } else { 0 });
                w(c, RIDER + 0x77c, OBJ2);
            },
        );
    }

    pub(crate) fn wf_2(c: &mut Cpu, a: u32, v: f32) {
        c.bus.write32(a, v.to_bits());
    }

    // ------------------------------------------------------ rider finish line

    #[test]
    fn rider_finish_line_sub_enter_matches() {
        check(
            ee::s00::f_0012c5c8__generated,
            rider_finish_line_sub_enter,
            SEEDS_6,
            &|c, seed| {
                let _ = base_4(c, seed, 0x12c5_c8, 4);
                let obj = SCRATCH;
                let rider = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 4, rider);
                w(c, rider + 0x790, rider + 0x2000);
                // The finish-line normal and the two test values.
                for i in 0..4u32 {
                    wf_2(
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
                wf_2(c, rider + 0x2dc, if seed % 2 == 0 { 0.0 } else { 0.5 });
                wf_2(c, rider + 0x378, if seed % 3 == 0 { 0.0 } else { 1.0 });
                wf_2(
                    c,
                    GAME_GP.wrapping_add(0x1b9c),
                    if seed % 2 == 0 { 0.0 } else { 2.0 },
                );
                wf_2(
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
            SEEDS_6,
            &|c, seed| {
                let _ = base_4(c, seed, 0x12d9_d8, 6);
                let obj = SCRATCH;
                let rider = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x80, rider);
                w(c, rider + 0x784, rider + 0x2000);
                wf_2(
                    c,
                    obj + 0x70,
                    match seed % 4 {
                        0 => 0.0,
                        1 => 2.0,
                        _ => 1.0,
                    },
                );
                wf_2(c, rider + 0x470, if seed % 5 == 0 { 2.0 } else { 0.0 });
                w(c, obj + 0x40, 0);
                w(c, obj + 0xc, 0);
                w(c, obj, 0);
            },
        );
    }

    #[test]
    fn ai_reset_computer_render_slots_matches() {
        check(
            ee::s00::f_0012ab20__generated,
            ai_reset_computer_render_slots,
            60,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x12ab);
                c.g[A0][0] = sx(DATA);
                let n = (seed % 3) as u32;
                c.bus.write32(DATA + 0x80, n);
                c.bus.write32(gp_at(-0x854), DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x10d8, DATA + 0x4000);
                slot_4(c, DATA + 0x4000, 0x390, 1);
                for i in 0..3u32 {
                    let rider = DATA + 0x2000 + 0x100 * i;
                    c.bus.write32(DATA + 0x48 + 4 * i, rider);
                    c.bus.write32(rider + 0x18, DATA + 0x1000 + 0x100 * i);
                    c.bus
                        .write32(DATA + 0x1000 + 0x100 * i + 0x884, (i + 1) as u32);
                }
                fill(c, &mut r, DATA + 0x1000, 0x300);
            },
        );
    }

    #[test]
    fn ai_reset_actor_render_slots_matches() {
        check(
            ee::s00::f_0012ac48__generated,
            ai_reset_actor_render_slots,
            60,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x12ac);
                c.g[A0][0] = sx(DATA);
                let n = (seed % 3) as u32;
                c.bus.write32(DATA + 0x84, n);
                c.bus.write32(gp_at(-0x854), DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x10d8, DATA + 0x4000);
                slot_4(c, DATA + 0x4000, 0x390, 1);
                for i in 0..3u32 {
                    let rider = DATA + 0x2000 + 0x100 * i;
                    c.bus.write32(DATA + 0x5c + 4 * i, rider);
                    c.bus.write32(rider + 0x18, DATA + 0x1000 + 0x100 * i);
                    c.bus
                        .write32(DATA + 0x1000 + 0x100 * i + 0x884, (i + 1) as u32);
                }
                fill(c, &mut r, DATA + 0x1000, 0x300);
            },
        );
    }

    /// A float bit pattern in [-range, range).
    pub(crate) fn uf(r: &mut Rng, range: f32) -> u32 {
        r.f32(range)
    }

    #[test]
    fn ai_force_finish_riders_matches() {
        diff(
            ee::s00::f_0012b340__generated,
            ai_force_finish_riders,
            40,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x12_b340);
                c.g[A0][0] = sx(DATA);
                w(c, DATA, if seed % 5 == 0 { r.next_u32() } else { 5 });
                w(c, DATA + 0x7c, 2);
                for i in 0..2u32 {
                    w(c, DATA + 0x40 + 4 * i, DATA + 0x1000 + 0x100 * i);
                }
                for i in 0..2u32 {
                    let rider = DATA + 0x1000 + 0x100 * i;
                    let r_ = DATA + 0x2000 + 0x100 * i;
                    w(c, rider + 0x18, r_);
                    w(c, r_ + 0x87c, if seed % 2 == 0 { 0 } else { 1 });
                    w(
                        c,
                        r_ + 0x470,
                        if i == 0 {
                            0.0f32.to_bits()
                        } else {
                            uf(&mut r, 100.0)
                        },
                    );
                    w(c, r_ + 0x480, 0);
                }
                w(c, GAME_GP.wrapping_sub(0x204), DATA + 0x3000);
            },
        );
    }

    /// Every compared state must match: hard, caller-saved (`soft`) and LO/HI.
    pub(crate) fn strict(g: GuestFn, h: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(g, h, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    #[test]
    fn ai_apply_computer_rider_settings_matches() {
        strict(
            ee::s00::f_0012b698__generated,
            ai_apply_computer_rider_settings,
            SEEDS,
            &|c, seed| {
                let mut r = base_6(c, seed, 0xb698, 0);
                let rider = DATA + 0x100;
                let state = DATA + 0x2000;
                w(c, rider + 0x18, state);
                w(
                    c,
                    0x0053_08d0,
                    if seed % 3 == 0 {
                        8
                    } else if seed % 3 == 1 {
                        1
                    } else {
                        r.next_u32() & !9
                    },
                );
                w(c, 0x0053_08b8, r.next_u32());
                w(c, 0x0053_08d4, r.next_u32() % 5);
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(rider);
            },
        );
    }

    #[test]
    fn rider_gate_sub_update_push_matches() {
        strict(
            ssx3_ee::s00::f_0012c130__generated,
            rider_gate_sub_update_push,
            SEEDS_7,
            &|c, seed| {
                let mut r = base_7(c, seed, 0xc130);
                let gate = DATA + 0x1000;
                let rider = DATA + 0x2000;
                let base_7 = DATA + 0x3000;
                let anim = DATA + 0x4000;
                w(c, gate + 0x14, rider);
                w(c, rider + 0x77c, base_7);
                w(c, base_7 + 0xde0, [1u32, 3, 7, 8][(seed % 4) as usize]);
                w(c, rider + 0x784, anim);
                // Layer 2's animation id: 2 keeps pushing, otherwise done.
                w(
                    c,
                    anim + 8,
                    if seed % 2 == 0 { 2 } else { r.next_u32() % 8 },
                );
                // A two-entry sequence list whose last entry is at 1.0.
                let list = DATA + 0x5000;
                let n0 = DATA + 0x5100;
                let n1 = DATA + 0x5200;
                w(c, anim + 0x50, list);
                w(c, list + 4, n0);
                w(c, n0 + 0xc8, n1);
                w(c, n1 + 8, 0x3f80_0000);
                w(c, n1 + 0x10, 0x3f80_0000);
                c.g[A0][0] = sx(gate);
            },
        );
    }

    /// Generic fixture: scramble the register file, point every argument at a
    /// writable object, and fill those objects with small values (counts,
    /// modes) plus a few pointers into the data area at the offsets where
    /// structures usually hold a vtable or a buffer. Also builds the library
    /// singletons the game's callees expect.
    pub(crate) fn generic(c: &mut Cpu, seed: u32, words: u32) {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        globals_8(c, &mut r);
        for i in 0..8u32 {
            let base = DATA + 0x400 * (i + 1);
            c.g[[A0, A1, A2, A3, T0, T1, T2, T3][i as usize]][0] = base as u64;
            for k in 0..words {
                c.bus.write32(base + 4 * k, r.next_u32() % 8);
            }
            for k in 0..4u32 {
                c.bus
                    .write32(base + 4 * k, DATA + 0x1000 + 0x40 * (i * 4 + k));
            }
        }
    }

    /// The game's singletons in a test machine: the BELibrary dummy, the
    /// world (with the race-manager chain) and the frame-rate structure.
    pub(crate) fn globals_8(c: &mut Cpu, r: &mut Rng) {
        let world = DATA + 0x9000;
        let game = DATA + 0x9400;
        let race = DATA + 0x9800;
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x9c00);
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(GAME_GP - 0x854, DATA + 0x9200);
        c.bus.write32(world + 0x84, game);
        c.bus.write32(game + 0xc, race);
        c.bus.write32(race + 0x7c, 0);
        let hz = DATA + 0x9300;
        c.bus.write32(GAME_GP + 0x2a74, hz);
        c.bus.write32(hz + 0x10, 30 + (r.next_u32() % 120));
        // The audio system pointer some sound code follows.
        c.bus.write32(GAME_GP + 0x608, DATA + 0xa000);
    }

    #[test]
    fn rider_board_press_sub_check_release_matches() {
        same(
            ssx3_ee::s00::f_00131200__generated,
            rider_board_press_sub_check_release,
            200,
            &|c, seed| {
                let mut r = Rng(0x131200 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let self_at = DATA + 0x100;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.bus.write32(DATA + 0x300, r.next_u32());
                c.bus.write32(DATA + 0x304, r.next_u32());
                let rider = DATA + 0x200;
                c.bus.write32(self_at + 0x14, rider);
                let timer = if seed % 5 == 0 {
                    0.6f32.to_bits()
                } else {
                    r.f32(1.0)
                };
                c.bus.write32(self_at + 0xc, timer);
                for off in [0x274u32, 0x278, 0x27c, 0x300, 0x330] {
                    c.bus.write32(rider + off, r.next_u32());
                }
                for off in [-0x7614i32, -0x7610, -0x760c, -0x7608, -0x7604] {
                    c.bus
                        .write32(GAME_GP.wrapping_add(off as u32), r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn rider_spin_sub_get_hud_dir_angle_matches() {
        same(
            ssx3_ee::s01::f_00135f70__generated,
            rider_spin_sub_get_hud_dir_angle,
            200,
            &|c, seed| {
                let mut r = Rng(0x135f70 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.bus
                    .write32(DATA + 0x118, if seed % 4 == 0 { 0 } else { r.f32(1.0) });
                c.bus
                    .write32(DATA + 0x11c, if seed % 3 == 0 { 0 } else { r.f32(1.0) });
                for off in [-0x7318i32, -0x7314, -0x7310, -0x730c] {
                    c.bus
                        .write32(GAME_GP.wrapping_add(off as u32), r.edge_f32());
                }
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn rider_board_press_sub_flip_stance_matches() {
        same(
            ssx3_ee::s00::f_0012fec8__generated,
            rider_board_press_sub_flip_stance,
            200,
            &|c, seed| {
                let mut r = Rng(0x12fec8 ^ seed as u64);
                scramble(c, &mut r);
                generic(c, seed, 0x20);
                let self_at = DATA + 0x100;
                let rider = DATA + 0x200;
                let anim = DATA + 0x300;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x14, rider);
                c.bus.write32(rider + 0x320, (seed % 2) as u32);
                c.bus.write32(rider + 0x328, (seed % 4) as u32);
                c.bus.write32(rider + 0x784, anim);
                c.bus.write32(rider + 0x280, r.edge_f32());
                c.bus.write32(rider + 0x288, r.edge_f32());
                put_vec(
                    c,
                    anim + 0x30,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    anim + 0x40,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_words(
                    c,
                    0x004f_f160,
                    &[r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                for off in [0u32, 4, 8, 0xc] {
                    c.bus.write32(0x004f_f130 + off, r.edge_f32());
                }
            },
        );
    }

    pub(crate) const GATE: u32 = DATA + 0x1_0000;

    pub(crate) const GATE_RIDER: u32 = DATA + 0x1_1000;

    pub(crate) const GATE_RIDERBASE: u32 = DATA + 0x1_2000;

    #[test]
    fn rider_gate_sub_update_launch_matches() {
        check_all(
            ee::s00::f_0012c408__generated,
            rider_gate_sub_update_launch,
            30,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x12c4_08, 10);
                c.g[A0][0] = sx(GATE);
                c.g[A1][0] = sx(DATA + 0x1_3000);
                c.bus.write32(GATE + 0x14, GATE_RIDER);
                c.bus.write32(GATE + 0x10, r.f32(0.05)); // charge around 1.0
                c.bus.write32(GATE + 0xc, r.f32(1.0));
                c.bus.write32(GATE_RIDER + 0x77c, GATE_RIDERBASE);
                c.bus.write32(GATE_RIDER + 0x86c, r.next_u32() % 0x100);
                c.bus.write32(GATE_RIDER + 0x784, 0);
                c.bus.write32(
                    GATE_RIDERBASE + 0xde0,
                    [0u32, 1, 3, 0xff][seed as usize % 4],
                );
                put_words(c, GATE_RIDER + 0x1b0, &[0]);
            },
        );
    }

    /// `RiderGateSub_UpdateLaunch` steps the launch's progress (`+0x10`)
    /// 0.015 per 60 Hz tick: four ticks at 240 Hz move it as far as one at
    /// 60, so the rider leaves the gate state after the same real time.
    #[test]
    fn rider_gate_sub_update_launch_follows_the_rate() {
        let run = |hz: u32, start: f32| -> f32 {
            let mut c = crate::difftest::fresh();
            let _ = base_8(&mut c, 1, 0x12c4_08, 10);
            c.bus.write32(GATE + 0x14, GATE_RIDER);
            c.bus.write32(GATE + 0xc, 1.0f32.to_bits());
            c.bus.write32(GATE_RIDER + 0x77c, GATE_RIDERBASE);
            c.bus.write32(GATE_RIDER + 0x784, 0);
            c.bus.write32(GATE_RIDERBASE + 0xde0, 0); // not main state 3: no launch
            put_words(&mut c, GATE_RIDER + 0x1b0, &[0]);
            c.bus.write32(GATE + 0x10, start.to_bits());
            // The game's constants (gp-0x780c..-0x7800: 1.015, 0.015, 0.985,
            // 0.62).
            c.g[GP][0] = GAME_GP as u64;
            for (off, v) in [
                (0x780cu32, 0x3f81_eb85u32),
                (0x7808, 0x3c75_c291),
                (0x7804, 0x3f7c_28f6),
                (0x7800, 0x3f1e_b852),
            ] {
                c.bus.write32(GAME_GP.wrapping_sub(off), v);
            }
            crate::tick::set_for_test(&mut c, hz);
            for _ in 0..hz / 60 {
                c.g[A0][0] = sx(GATE);
                rider_gate_sub_update_launch(&mut c);
            }
            f32::from_bits(c.bus.read32(GATE + 0x10))
        };
        for start in [0.31f32, 0.62] {
            let want = run(60, start);
            assert!(
                (want - (start + 0.015)).abs() < 1e-6,
                "60 Hz from {start}: {want}"
            );
            for hz in [120, 240] {
                let got = run(hz, start);
                assert!(
                    (got - want).abs() < 1e-5,
                    "{hz} Hz from {start}: {got} vs {want}"
                );
            }
        }
    }

    /// The dummy world (and its nested objects).
    pub(crate) const WORLD_7: u32 = DATA + 0x8_0000;

    #[test]
    fn rider_reset_sub_enter_matches() {
        diff(
            ee::s00::f_0012f230__generated,
            rider_reset_sub_enter,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12_f230, 4);
                belib(c);
                let this = DATA + 0x1_0000;
                let anim = DATA + 0x2_0000;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 8, anim);
                c.bus.write32(anim + 0x784, DATA + 0x2_1000);
                // The layer sequence object's +0x90 is written.
                c.bus.write32(DATA + 0x2_1000, DATA + 0x2_2000);
                // IsLocalPlayer: present and id >= 0 on two seeds in three.
                c.bus
                    .write32(anim + 0x87c, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(anim + 0x870, if seed % 6 < 3 { 0x100 } else { 0xffff_ffff });
                c.bus.write32(GAME_GP - 0x848, WORLD_7);
                let game = WORLD_7 + 0x1000;
                c.bus.write32(WORLD_7 + 0x84, game);
                let transition = DATA + 0x4_0000;
                c.bus.write32(game + 0xc, transition);
                c.bus
                    .write32(transition + 0x44, if seed % 2 == 0 { 0 } else { 1 });
                let _ = r.next_u32();
            },
        );
    }

    // ------------------------------------------------------------------
    // RiderSpinSub_ApplyPoseRotation (0x134dd0).

    /// Random spin state, rider path node and pose (unit-ish quaternion);
    /// the lean factor below, near and above the spin factor so all three
    /// easing branches run. `Math_SinCos`, `GetSpinFactor` (and its
    /// `Math_Atan`) are real, with the game's `.lit4` constants.
    #[test]
    fn rider_spin_sub_apply_pose_rotation_matches() {
        use crate::difftest::fakes::machine;
        const SELF: u32 = DATA + 0x100;
        const RIDER: u32 = DATA + 0x1000;
        const PATH: u32 = DATA + 0x2000;
        const NODES: u32 = DATA + 0x3000;
        const POSE: u32 = DATA + 0x4000;
        crate::difftest::check(
            ee::s01::f_00134dd0__generated,
            rider_spin_sub_apply_pose_rotation,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0013_4dd0, 0);
                c.bus.write32(SELF + 0x58, RIDER);
                for off in [0x20u32, 0x24, 0x28, 0x2c] {
                    c.bus.write32(SELF + off, r.f32(3.2));
                }
                // Lean factor: anywhere in [-2, 2] (the spin factor is atan-based).
                let lean = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(2.0)
                };
                c.bus.write32(SELF + 0x40, lean);
                c.bus.write32(RIDER + 0x2dc, r.f32(20.0));
                c.bus.write32(RIDER + 0x2e0, r.f32(20.0));
                c.bus.write32(RIDER + 0x780, PATH);
                let node = r.next_u32() % 4;
                c.bus.write32(RIDER + 0x89c, node);
                c.bus.write32(PATH + 0x24, NODES);
                for off in [0x140u32, 0x144, 0x148, 0x14c] {
                    c.bus.write32(PATH + off, r.f32(2.0));
                }
                for i in 0..4 {
                    put_vec(
                        c,
                        NODES + 0x10 * i,
                        [r.f32(10.0), r.f32(10.0), r.f32(10.0), r.f32(1.0)],
                    );
                }
                // The spin pivot, the two lean axes and world up.
                for a in [0x004f_f130u32, 0x004f_f140, 0x004f_f150] {
                    put_vec(c, a, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                }
                put_vec(c, 0x004f_f160, [0, 0, 1.0f32.to_bits(), 0]);
                put_vec(
                    c,
                    POSE,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), 1.0f32.to_bits()],
                );
                put_vec(
                    c,
                    POSE + 0x10,
                    [r.f32(0.7), r.f32(0.7), r.f32(0.7), r.f32(1.0)],
                );
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(POSE);
                c.pc = 0x0013_4dd0;
            },
        );
    }

    use crate::difftest::check_paths;

    /// Prewind animation pick: stick x/y from {+0, -0, random} (both
    /// axes zero, one axis zero, every angle sector), switch on/off, a
    /// random column; the playing animation equals the chosen one (whole
    /// column the same) or not, so PlayAnim is reached. PlayAnim's virtual
    /// call is a fake forced to return 0x207 (its early exit) and sometimes
    /// unwinds.
    #[test]
    fn rider_prewind_sub_play_directional_anim_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        covered(
            ee::s00::f_0012ee30__generated,
            rider_prewind_sub_play_directional_anim,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0xee30, 1);
                c.pc = 0x0012_ee30;
                let sub = DATA;
                let rider = DATA + 0x1000;
                let table = DATA + 0x2000;
                let ctl = DATA + 0x3000;
                let vt = DATA + 0x4000;
                c.bus.write32(sub, rider);
                let pick = |r: &mut Rng, k: u32| match k {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => r.f32(2.0),
                };
                let x = pick(&mut r, seed % 5);
                let y = pick(&mut r, (seed / 5) % 5);
                c.bus.write32(rider + prewind::STICK_X, x);
                c.bus.write32(rider + prewind::STICK_Y, y);
                let switch = if (seed / 25) % 2 == 0 {
                    0
                } else {
                    r.next_u32() | 1
                };
                c.bus.write32(rider + prewind::SWITCH, switch);
                let column = r.next_u32() % 5;
                c.bus.write32(rider + prewind::ANIM_COLUMN, column);
                c.bus.write32(rider + prewind::ANIM_CTL, ctl);
                fill(c, &mut r, table, 45);
                let playing = r.next_u32() % 0x300;
                if seed % 3 == 0 {
                    for row in 0..9 {
                        c.bus.write32(table + row * 0x14 + column * 4, playing);
                    }
                }
                // Layer 2's animation id (ctl + 2 * 4).
                c.bus.write32(ctl + 8, playing);
                // PlayAnim: vtable at +0x58, its slot {delta at +0x20, fn at +0x24}.
                c.bus.write32(ctl + 0x58, vt);
                c.bus.write16(vt + 0x20, 0x10);
                c.bus.write32(vt + 0x24, FAKE);
                c.bus.write32(LOG_RESULTS, 0x207);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | sub as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | table as u64;
            },
        );
    }

    /// AI_UpdateRaceTimeLimit: inactive race, states 5/3/other (session
    /// state matching or not), race modes 0/4/other, countdown idle,
    /// running, expiring, not started (start it, hard limit, all finished),
    /// replay busy or not, every-second ticks. BELibrary_GetInterface's
    /// method is a fake; the other callees are real on short paths.
    #[test]
    fn ai_update_race_time_limit_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        covered(
            ee::s00::f_0012bb20__generated,
            ai_update_race_time_limit,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12bb_20, 2);
                let gp = GAME_GP;
                let this = DATA;
                let world = DATA + 0x6000;
                let be_obj = DATA + 0x7000;
                let be_vt = DATA + 0x7100;
                let comm = DATA + 0x8000;
                let sess = DATA + 0x9000;
                let replay = DATA + 0x9400;
                c.g[A0][0] = sx(this);
                c.bus.write32(gp - GP_WORLD, world);
                c.bus.write32(world + 0x78, be_obj);
                c.bus.write32(world + 0x84, sess);
                c.bus.write32(gp - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, be_obj);
                c.bus.write32(be_obj + 4, be_vt);
                c.bus.write16(be_vt + 8, 0);
                c.bus.write32(be_vt + 0xc, FAKE);
                c.bus.write32(gp - 0x1f04, DATA + 0x7800); // BELibrary instance
                c.bus.write32(gp - GP_GAME_COMM, comm);
                c.bus.write32(comm + 0x8c, (r.next_u32() % 4 == 0) as u32);
                c.bus.write32(comm + 0x48, (r.next_u32() % 4 == 0) as u32);
                c.bus.write32(sess + 0xc, sess + 0x500);
                c.bus.write32(sess + 0x500, 0); // session idle: no sync send
                c.bus.write32(
                    sess + SESSION_STATE,
                    if r.next_u32() % 2 == 0 { 3 } else { 4 },
                );
                c.bus.write32(sess + SESSION_REPLAY, replay);
                c.bus
                    .write32(replay, [0, 5, 14, 20][(r.next_u32() % 4) as usize]);
                c.bus.write32(RACE_ACTIVE, (seed % 13 != 0) as u32);
                c.bus
                    .write8(RACE_MODE, [0u8, 4, 0, 4, 2][(r.next_u32() % 5) as usize]);
                c.bus.write32(
                    this + AI_STATE,
                    [5, 5, 5, 3, 1][(r.next_u32() % 5) as usize],
                );
                let timer = match r.next_u32() % 5 {
                    0 => 60 * (1 + r.next_u32() % 800),
                    1 => 0xd32b - (r.next_u32() % 3),
                    2 => 0xd3a4 + (r.next_u32() % 3) * 60,
                    3 => 0xd32c + r.next_u32() % 0x70,
                    _ => r.next_u32() % 0xd3c0,
                };
                c.bus.write32(this + AI_TIMER, timer);
                let countdown = match seed % 5 {
                    0 | 1 => u32::MAX,
                    2 => 0,
                    3 => 1,
                    _ => 2 + r.next_u32() % 100,
                };
                c.bus.write32(this + AI_COUNTDOWN, countdown);
                c.bus.write32(this + 0x78, 0); // no riders for the sync checksum
                let count = r.next_u32() % 4;
                c.bus.write32(this + AI_RIDER_COUNT, count);
                for i in 0..4 {
                    let actor = DATA + 0x100 + 0x10 * i;
                    let rider = DATA + 0x1000 + 0x1000 * i;
                    c.bus.write32(this + AI_RIDERS + 4 * i, actor);
                    c.bus.write32(actor + ACTOR_RIDER, rider);
                    let t = if r.next_u32() % 2 == 0 {
                        (-1.0f32).to_bits()
                    } else {
                        r.f32(100.0) & 0x7fff_ffff
                    };
                    c.bus.write32(rider + RIDER_FINISH_TIME, t);
                    c.bus.write32(rider + RIDER_FORCED_FINISH, 0);
                }
            },
        );
    }

    /// RiderSpinSub_GetSpinFactor: stick x zero (y zero / positive /
    /// negative) or not (x negative with y positive / not, x positive);
    /// magnitudes below, inside and above the dead-zone range. Math_Atan
    /// runs for real on the game's constants (ELF image).
    #[test]
    fn rider_spin_sub_get_spin_factor_matches() {
        check(
            ee::s01::f_00135180__generated,
            rider_spin_sub_get_spin_factor,
            300,
            &|c, seed| {
                let mut r = Rng(0x135180 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0013_5180;
                let (this, input) = (DATA, DATA + 0x100);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x58, input);
                let pick = |r: &mut Rng, k: u32| match k {
                    0 => 0,
                    1 => 0x8000_0000,
                    2 => r.edge_f32(),
                    _ => r.f32(8.0),
                };
                let x = pick(&mut r, seed % 5);
                let y = pick(&mut r, (seed / 5) % 5);
                c.bus.write32(input + 0x2dc, x);
                c.bus.write32(input + 0x2e0, y);
            },
        );
    }

    /// `RiderSpinSub_GetSpinProgress`: spin angles in [-10, 10] (the turn
    /// is past 2*pi or not), rates 0.05..0.5 or edge values, the rider's
    /// halving float positive, zero or negative, `done` and `full` distinct
    /// or the same pointer. The behaviour library exists (gp-0x1f04); its
    /// interface getter is a fake (sometimes unwinding);
    /// `BECharInt_GetAttrib4Factor` runs for real on the game image
    /// (player 0, characters 1..5).
    #[test]
    fn rider_spin_sub_get_spin_progress_matches() {
        const SPIN: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const OUT: u32 = DATA + 0x3000;
        const BE_LIBRARY: u32 = DATA + 0x5000;
        const BE_OBJ: u32 = DATA + 0x5100;
        const BE_VT: u32 = DATA + 0x5200;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        covered(
            ee::s01::f_00135db0__generated,
            rider_spin_sub_get_spin_progress,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x135db0, 1);
                c.pc = 0x0013_5db0;
                crate::difftest::elf_image(c);
                c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(BE_INTERFACES, BE_OBJ);
                c.bus.write32(BE_OBJ + 4, BE_VT);
                c.bus.write32(BE_VT + 0xc, FAKE);
                c.bus.write8(0x0053_08e2, 10);
                c.bus.write8(0x0053_08e4, 10);
                c.bus.write32(SPIN + SPIN_START, r.f32(10.0));
                c.bus.write32(SPIN + SPIN_ANGLE, r.f32(10.0));
                let rate = if seed % 9 == 0 {
                    r.edge_f32()
                } else {
                    (0.05 + (r.next_u32() % 450) as f32 / 1000.0).to_bits()
                };
                c.bus.write32(SPIN + SPIN_RATE, rate);
                c.bus.write32(SPIN + SPIN_RIDER, RIDER);
                let halve = match seed % 3 {
                    0 => 0,
                    1 => 1.0f32.to_bits(),
                    _ => r.f32(2.0),
                };
                c.bus.write32(RIDER + RIDER_SPIN_HALVE, halve);
                c.bus.write32(RIDER + RIDER_PLAYER, 0);
                c.bus.write32(RIDER + RIDER_CHARACTER, 1 + r.next_u32() % 5);
                c.g[A0][0] = sx(SPIN);
                c.g[A1][0] = sx(OUT);
                c.g[A2][0] = sx(if seed % 7 == 0 { OUT } else { OUT + 4 });
            },
        );
    }

    /// `RiderPrewindSub_GetHUDAngle`: each stick axis zero, inside the dead
    /// zone (either sign), or anywhere in [-2, 2] (or an edge value), so
    /// every quadrant, the x == 0 up/down cases and the centred case occur;
    /// the real `Math_Atan` runs on the game image's constants.
    #[test]
    fn rider_prewind_sub_get_hud_angle_matches() {
        const SUB: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const OUT: u32 = DATA + 0x2000;
        crate::difftest::check(
            ee::s00::f_0012f118__generated,
            rider_prewind_sub_get_hud_angle,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12f118, 0);
                crate::difftest::elf_image(c);
                c.pc = 0x0012_f118;
                let axis = |r: &mut Rng, k: u32| match k % 6 {
                    0 => 0,
                    1 => r.f32(0.099),
                    2 => 0x8000_0000,
                    3 => r.edge_f32(),
                    _ => r.f32(2.0),
                };
                let y = axis(&mut r, seed);
                let x = axis(&mut r, seed / 6);
                c.bus.write32(SUB, RIDER);
                c.bus.write32(RIDER + RIDER_STICK_Y, y);
                c.bus.write32(RIDER + RIDER_STICK_X, x);
                fill(c, &mut r, OUT, 1);
                c.g[A0][0] = sx(SUB);
                c.g[A1][0] = sx(OUT);
            },
        );
    }

    // --------------------------------------------- RiderSpinSub_GetFlipProgress

    /// The BE tables `BECharInt_GetAttrib4Factor` reads (as in `ai`'s
    /// `be_tables`): attribute and character-max noise, profile ids, the
    /// player slots (human or CPU), rider records and the BE singletons.
    pub(crate) fn be_attrib_tables(c: &mut Cpu, r: &mut Rng) {
        const ATTRIBS: u32 = 0x0053_5538;
        const CHARACTER_MAX: u32 = 0x0053_08d8;
        const PLAYERS: u32 = 0x0053_4fe0;
        fill(c, r, ATTRIBS - 0x400, 0x400);
        fill(c, r, CHARACTER_MAX - 0x800, 0x400);
        for p in 0..8 {
            c.bus.write32(0x0053_05b0 + 4 * p, r.next_u32() % 8);
        }
        fill(c, r, PLAYERS, 8 * 0x1c / 4);
        for s in 0..8 {
            let pad = if r.next_u32() % 2 == 0 {
                r.next_u32() | 0x8000_0000
            } else {
                r.next_u32() % 4
            };
            c.bus.write32(PLAYERS + s * 0x1c + 0xc, pad);
        }
        fill(c, r, 0x0053_5b20, 16 * 0x1c / 4);
        for s in 0..16 {
            let rec = 0x0053_5b20 + s * 0x1c;
            if r.next_u32() % 2 == 0 {
                c.bus.write32(rec + 0xc, u32::MAX);
            }
            c.bus.write8(rec + 0x11, (r.next_u32() % 10) as u8);
        }
        for ch in 0..10 {
            for n in 0..7 {
                if r.next_u32() % 6 == 0 {
                    c.bus.write8(CHARACTER_MAX + ch * 15 + 8 + n, 0);
                }
            }
        }
        c.bus.write32(0x0053_5c08, r.next_u32() % 40);
        fill(c, r, 0x0043_d950, 40 * 0x64 / 4);
        for t in 0..40 {
            c.bus
                .write32(0x0043_d950 + t * 0x64 + 0x54, r.next_u32() % 4);
        }
        // NEW_RACE_INT, NEW_PLAYER_INT, AGGRESSION_INT, CHAR_INT exist.
        for g in [0x1f3c, 0x1f30, 0x1ea4, 0x1f2c] {
            c.bus.write32(GAME_GP - g, DATA + 0xe_0000 + g * 0x10);
        }
    }

    /// `RiderSpinSub_GetFlipProgress`: flip angles below and above pi,
    /// edge-case rates, boost positive / zero / negative; the BELibrary
    /// singleton exists and its interface lookup is a fake (some unwind);
    /// the real attribute-4 factor runs on random BE tables.
    #[test]
    fn rider_spin_sub_get_flip_progress_matches() {
        use crate::difftest::fakes::vtable;
        covered(
            ee::s01::f_00135cb0__generated,
            rider_spin_sub_get_flip_progress,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0013_5cb0, 1);
                elf_image(c);
                c.pc = 0x0013_5cb0;
                // The BELibrary singleton; interface slot 0 is an object
                // whose vtable slot +8 is a fake.
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x1000);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(0x0053_08a8, DATA + 0x1100);
                c.bus.write32(DATA + 0x1104, vt);
                be_attrib_tables(c, &mut r);
                let this = DATA + 0x30_0000;
                let rider = DATA + 0x30_1000;
                let done = DATA + 0x30_2000;
                let total = DATA + 0x30_2010;
                let pi = std::f32::consts::PI;
                let a = r.f32(4.0);
                let b = match seed % 3 {
                    0 => r.f32(1.0),
                    1 => (f32::from_bits(a) + if seed % 2 == 0 { 4.0 } else { -4.0 }).to_bits(),
                    _ => r.edge_f32(),
                };
                c.bus.write32(this + 0x14, a);
                c.bus.write32(this + 0x1c, b);
                let rate = if seed % 7 == 0 {
                    r.edge_f32()
                } else {
                    (pi * 2.0).to_bits()
                };
                c.bus.write32(this + 0x38, rate);
                c.bus.write32(this + 0x58, rider);
                let boost = match seed % 4 {
                    0 => 0,
                    1 => 1.0f32.to_bits(),
                    2 => (-1.0f32).to_bits(),
                    _ => r.edge_f32(),
                };
                c.bus.write32(rider + 0x2ec, boost);
                c.bus.write32(rider + 0x86c, r.next_u32() % 8);
                c.bus.write32(
                    rider + 0xb34,
                    match r.next_u32() % 3 {
                        0 => 0,
                        1 => r.next_u32() % 40,
                        _ => r.next_u32(),
                    },
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(done);
                c.g[A2][0] = sx(total);
            },
        );
    }

    // -- 0x0012cb68 RiderWipeoutSub_HandleInput --------------------------------

    /// `RiderWipeoutSub_HandleInput` against the generated body: every phase
    /// (recovering phase 3 skips the slide update; 5+ and negative ones fall
    /// out of the switch), slide amounts around the decay step and around 1,
    /// the slide button on and off, the three wipeout kinds. The real boost
    /// input, reset-path check (reset button never pressed) and audio callees
    /// run on their short paths (no boost, not in game, rider without a sound
    /// slot). The phase jump table is empty RAM (each phase update is a fake
    /// through `jump_out`, some unwinding), except that some phase-4 seeds put
    /// the real entry there and run the real `UpdatePhase4` (timer not
    /// expired, animation not finished).
    #[test]
    fn rider_wipeout_sub_handle_input_matches() {
        use super::wipeout_input::*;
        use crate::difftest::fakes::{covered, machine};
        use crate::difftest::{DATA, GAME_GP};
        const THIS: u32 = DATA + 0x6_0000;
        const PAD: u32 = DATA + 0x6_0100;
        const RIDER_OBJ: u32 = DATA + 0x6_1000;
        const CTRL: u32 = DATA + 0x6_3000;
        const SEQS: u32 = DATA + 0x6_3100;
        const SEQ_NODE: u32 = DATA + 0x6_3200;
        const WORLD: u32 = DATA + 0x6_4000;
        const AUDIO: u32 = DATA + 0x6_5000;
        const BE_LIBRARY: u32 = DATA + 0x6_6000;
        covered(
            ee::s00::f_0012cb68__generated,
            rider_wipeout_sub_handle_input,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0012_cb68, 1);
                c.pc = 0x0012_cb68;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                let f = |x: f32| x.to_bits();
                // Globals: the behaviour library, the world (not in game:
                // the audio calls stop early), the audio system.
                w(c, GAME_GP - 0x1f04, BE_LIBRARY);
                w(c, GAME_GP - 0x848, WORLD);
                w(c, WORLD + 0x84, 0);
                w(c, GAME_GP + 0x410, AUDIO);
                // The literals.
                w(c, GAME_GP - GP_DECAY, f(0.004_169_862_7));
                w(c, GAME_GP - GP_ZERO, 0);
                w(c, GAME_GP - GP_RATE, f(17.002_617));
                w(c, GAME_GP - GP_RATE_KIND0, f(8.001_495));
                w(c, GAME_GP - GP_RATE_KIND2, f(34.002_766));
                w(c, GAME_GP - GP_TICK, f(1.0 / 60.0));
                // The phase.
                let phase = match seed % 8 {
                    7 => 0xffff_fff0 | r.next_u32() % 16,
                    p => p,
                };
                let real_phase4 = phase == 4 && r.next_u32() % 2 == 0;
                if real_phase4 {
                    w(c, PHASE_TABLE + 4 * 4, 0x0012_cd00);
                }
                // The slide amount: around the decay step, around 1, or
                // small enough to stay below 1 (the real phase 4).
                let slide = if real_phase4 {
                    r.f32(0.4)
                } else {
                    match r.next_u32() % 4 {
                        0 => r.f32(0.01),
                        1 => f(1.0 - (r.next_u32() % 64) as f32 * 0.01),
                        2 => 0,
                        _ => r.f32(1.5),
                    }
                };
                w(c, THIS + PHASE, phase);
                w(c, THIS + SLIDE, slide);
                w(c, THIS + RIDER, RIDER_OBJ);
                // The pad: the reset button (bit 12) never pressed, the
                // slide button at random.
                w(c, PAD, r.next_u32() & !0x1000);
                // The rider: no boost (+0x2fc), a wipeout kind, no sound slot
                // (+0x874); for phase 4 the timer (+0x470) not expired and an
                // animation controller whose layer 2 is not finished.
                w(c, RIDER_OBJ + 0x2fc, 0);
                w(
                    c,
                    RIDER_OBJ + RIDER_WIPEOUT_KIND,
                    [0, 2, 1, 5][(r.next_u32() % 4) as usize],
                );
                w(c, RIDER_OBJ + 0x874, 0);
                w(c, RIDER_OBJ + 0x470, f(-1.0));
                w(c, RIDER_OBJ + 0x784, CTRL);
                w(c, CTRL + 0x50, SEQS);
                w(c, SEQS + 0x14, SEQ_NODE);
                w(c, SEQ_NODE + 0xc0, 0);
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(PAD);
            },
        );
    }

    // (fakes imported above)

    /// The `RiderBumpSub_HandleInput` fixture: the prewind input fixture's
    /// rider (snd.rs `prewind_input_fixture`) with a bump pad, every main
    /// state the function branches on and a finished / running layer-2
    /// animation.
    pub(crate) mod bump_input_fixture {
        use super::super::*;
        use crate::difftest::fakes::{fake_call, machine, FAKE};
        use crate::difftest::{elf_image, put_vec, Rng, DATA, GAME_GP};

        const THIS: u32 = DATA + 0x7_0000;
        const PAD: u32 = DATA + 0x7_0100;
        const RIDER: u32 = DATA + 0x7_1000;
        const OWNER: u32 = DATA + 0x7_2000;
        const CTRL: u32 = DATA + 0x7_3000;
        const CTRL_VT: u32 = DATA + 0x7_3100;
        const SEQ_LISTS: u32 = DATA + 0x7_3200;
        const SEQ: u32 = DATA + 0x7_3300;
        const MODEL: u32 = DATA + 0x7_4000;
        const BONES: u32 = DATA + 0x7_4100;
        const LIST: u32 = DATA + 0x7_5000;
        const LIST_TREE: u32 = DATA + 0x7_5400;
        const METRIX: u32 = DATA + 0x7_6000;
        const AUDIO: u32 = DATA + 0x7_7000;
        const BE_LIBRARY: u32 = DATA + 0x7_8000;
        const BE_OBJ: u32 = DATA + 0x7_8100;
        const BE_VT: u32 = DATA + 0x7_8200;
        const ANIM_TABLE: u32 = 0x0044_6990;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        /// `RiderBase_EnterSubState`'s jump table (zeroed: the enter
        /// handlers become fakes, which unwind the call chain).
        const ENTER_SUB_STATE_TABLE: u32 = 0x0045_6bd0;
        /// The controller's "current animation kind" method: answers 0x207
        /// (`PlayAnim` stops there).
        const FAKE_ANIM_KIND: u32 = FAKE + 0x10;

        fn bump_miss(c: &mut Cpu, target: u32) {
            fake_call(c, target);
            if target == FAKE_ANIM_KIND {
                c.g[V0][0] = 0x207;
            }
        }

        /// A 6-bit signed axis: centred, small, negative or random.
        fn bump_axis(r: &mut Rng) -> u32 {
            match r.next_u32() % 4 {
                0 => 0,
                1 => 3,
                2 => 0x3d,
                _ => r.next_u32() & 0x3f,
            }
        }

        pub(super) fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0012_e778, 12);
            let r = &mut r;
            c.hooks.miss = bump_miss;
            elf_image(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            for i in 0..14 {
                w(c, ENTER_SUB_STATE_TABLE + 4 * i, 0);
            }
            // The .lit4 scales (the game's or edge cases).
            if seed % 7 == 0 {
                for off in [BUMP_AXIS_SCALE_GP, BUMP_LEAN_SCALE_GP, BUMP_SLEW1_RATE_GP] {
                    let v = r.edge_f32();
                    w(c, GAME_GP - off, v);
                }
            }

            // The back end: the library exists, interface getters are fakes.
            w(c, GAME_GP - 0x1f04, BE_LIBRARY);
            w(c, GAME_GP - 0x1efc, 0);
            w(c, BE_INTERFACES, BE_OBJ);
            w(c, BE_OBJ + 4, BE_VT);
            w(c, BE_VT + 0xc, FAKE);
            c.bus.write8(0x0053_08e2, 10);
            c.bus.write8(0x0053_08e4, 10);
            w(c, GAME_GP + 0x410, AUDIO);

            // The pad: never the reset bit; random boost bits; both axes.
            let mut buttons = r.next_u32() & !0x0000_1000;
            buttons &= !(0x3f << 15 | 0x3f << 21);
            buttons |= bump_axis(r) << 15 | bump_axis(r) << 21;
            w(c, PAD, buttons);
            w(c, PAD + 4, r.next_u32());

            w(c, THIS, RIDER);
            // The rider: racing (finished on every 16th seed); main state
            // 1 (airborne), 4, 2 or 0; no sub-state to exit.
            w(
                c,
                RIDER + 0x470,
                if seed % 16 == 15 {
                    1.0f32.to_bits()
                } else {
                    (-1.0f32).to_bits()
                },
            );
            w(c, RIDER + 0x77c, OWNER);
            w(c, OWNER + 0xde0, [1, 4, 2, 0][((seed / 2) % 4) as usize]);
            w(c, OWNER + 0xde4, 0x20);
            w(c, OWNER + 0x2b8, RIDER);
            w(c, RIDER + 0x328, 0);
            w(c, RIDER + 0x304, 3);
            w(c, RIDER + 0x780, MODEL);
            w(c, MODEL + 0x2c, BONES);
            w(c, RIDER + 0x860, LIST);
            w(c, LIST, LIST_TREE);
            w(c, RIDER + 0x790, METRIX);
            w(c, METRIX + 0x1ac, RIDER);
            for off in [0x220, 0x2a4, 0x2b0] {
                w(c, RIDER + off, r.f32(2.0));
            }
            w(
                c,
                RIDER + 0x2dc,
                if r.next_u32() % 2 == 0 { 0 } else { r.f32(1.0) },
            );
            put_vec(
                c,
                RIDER + BUMP_RIDER_VELOCITY,
                [r.f32(5.0), r.f32(5.0), r.f32(5.0), 0],
            );
            put_vec(
                c,
                RIDER + BUMP_RIDER_FACING,
                [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0],
            );
            for off in [0x100, 0x110, 0x120, 0x130] {
                put_vec(
                    c,
                    RIDER + off,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), 1.0f32.to_bits()],
                );
            }

            // The animation controller: layer 2's sequence finished on
            // every other seed.
            w(c, RIDER + BUMP_RIDER_ANIM_CTRL, CTRL);
            w(c, CTRL + 0x50, SEQ_LISTS);
            w(c, CTRL + 0x58, CTRL_VT);
            w(c, CTRL_VT + 0x24, FAKE_ANIM_KIND);
            for k in 0..4 {
                w(c, SEQ_LISTS + 8 * k + 4, SEQ);
            }
            w(c, SEQ + 0xc0, if seed % 2 == 0 { 0 } else { 1 });
            for id in [0x12, 0x13, 0x14, 0x44, 0x45, 0x46] {
                w(c, ANIM_TABLE + id * 0x1c, 0);
            }
            w(c, CTRL + 8, 0x31);

            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = sx(PAD);
            c.pc = 0x0012_e778;
        }
    }

    /// `RiderBumpSub_HandleInput`: finished or not, main states 1 / 4 /
    /// other, forwards and backwards travel, the layer-2 animation finished
    /// (land animation + sub-state 7, or grind reset + sub-state 4/0) or
    /// running; real callees, fakes that unwind.
    #[test]
    fn rider_bump_sub_handle_input_matches() {
        check(
            ee::s00::f_0012e778__generated,
            rider_bump_sub_handle_input,
            300,
            &bump_input_fixture::setup,
        );
    }

    // ------------------------------------------------------ rider wipeout

    /// RiderWipeoutSub_UpdateRecover: first update or not (random impact
    /// vector), human or computer rider, layer 2 finished or not, equal or
    /// different +0x320/+0x324. The rider's vtable slots (0x88, 0x90, and
    /// 0x48/0x28 used by the landing sound) and the anim controller's slot
    /// are fakes (forced result 0x207: `AnimController_PlayAnim`'s early
    /// out). `PlayLandingSound` takes its replay path: the world's game
    /// module says replay (`*(module+0x28) == 1`), the race man has no
    /// riders (`FindRider` → 0) and no views (`FindViewInRange` → -1). The
    /// trick metrix is idle (`+0x70` set: `InterruptTrick` makes no calls).
    /// Owner states as in the board-press test (main >= 6, sub 0xe..0x15).
    #[test]
    fn rider_wipeout_sub_update_recover_matches() {
        const SELF: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const OWNER: u32 = DATA + 0x2000;
        const VT: u32 = DATA + 0x3000;
        const CTL: u32 = DATA + 0x4000;
        const CTL_VT: u32 = DATA + 0x5000;
        const SEQS: u32 = DATA + 0x6000;
        const SEQ: u32 = DATA + 0x6100;
        const METRIX: u32 = DATA + 0x7000;
        const WORLD: u32 = DATA + 0x8000;
        const MODULE: u32 = DATA + 0x8100;
        const REPLAY: u32 = DATA + 0x8200;
        const RACE_MAN: u32 = DATA + 0x8300;
        check(
            ee::s00::f_0012d848__generated,
            rider_wipeout_sub_update_recover,
            240,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0012_d848, 6);
                c.pc = 0x0012_d848;
                c.g[A0][0] = sx(SELF);
                let started = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(SELF + 0x54, started);
                for k in 0..4 {
                    let v = r.f32(10.0);
                    c.bus.write32(SELF + 0x60 + 4 * k, v);
                }
                c.bus.write32(SELF + 0x80, RIDER);
                c.bus.write32(RIDER + 0x6c0, VT);
                for (i, off) in [0x88u32, 0x90, 0x48, 0x28].into_iter().enumerate() {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(VT + off, delta as u16);
                    c.bus.write32(VT + off + 4, FAKE + 0x10 * i as u32);
                }
                c.bus.write32(RIDER + 0x77c, OWNER);
                let ai = if (seed / 3) % 2 == 0 {
                    0
                } else {
                    r.next_u32() | 1
                };
                c.bus.write32(OWNER + 0x30, ai);
                c.bus.write32(OWNER + 0xde0, 6 + r.next_u32() % 4);
                c.bus.write32(OWNER + 0xde4, 0xe + r.next_u32() % 8);
                c.bus.write32(RIDER + 0x784, CTL);
                c.bus.write32(CTL + 0x50, SEQS);
                c.bus.write32(SEQS + 0x14, SEQ);
                let (t, len) = (r.f32(4.0), r.f32(8.0) | 0x3f80_0000);
                c.bus.write32(SEQ + 0x8, t);
                c.bus.write32(SEQ + 0x10, len & 0x7fff_ffff);
                let finished = if (seed / 6) % 2 == 0 {
                    0
                } else {
                    r.next_u32() | 1
                };
                c.bus.write32(SEQ + 0xc0, finished);
                c.bus.write32(CTL + 0x58, CTL_VT);
                c.bus.write16(CTL_VT + 0x20, 0x10);
                c.bus.write32(CTL_VT + 0x24, FAKE + 0x40);
                for n in 0..8 {
                    c.bus.write32(fakes::LOG_RESULTS + 4 * n, 0x207);
                }
                let a = r.next_u32();
                let b = if (seed / 12) % 2 == 0 {
                    a
                } else {
                    r.next_u32()
                };
                c.bus.write32(RIDER + 0x320, a);
                c.bus.write32(RIDER + 0x324, b);
                c.bus.write32(RIDER + 0x790, METRIX);
                c.bus.write32(METRIX + 0x70, 1);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
                c.bus.write32(WORLD + 0x84, MODULE);
                c.bus.write32(MODULE + 0x28, REPLAY);
                c.bus.write32(REPLAY, 1);
                c.bus.write32(MODULE + 0xc, RACE_MAN);
            },
        );
    }

    pub(crate) fn oc258_w32(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// RiderWipeoutSub_UpdatePhase0: the track-2 anim still playing (phase
    /// below or above 0.95) or done; done with anim types 0x17 (root-pose
    /// override), 0x1d, 0x19 or another; the wipeout state in mode 1 or not.
    /// The real anim queries, velocity setter (and its air-predictor prime),
    /// root-pose override, tumble selection (an anim id outside its table:
    /// straight to its epilogue) and audio loops (the rider's loop handles
    /// are non-negative: their early exit) run.
    #[test]
    fn rider_wipeout_sub_update_phase0_matches() {
        check(
            ee::s00::f_0012cd20__generated,
            rider_wipeout_sub_update_phase0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0012_cd20, 0);
                c.pc = 0x0012_cd20;
                let this = DATA + 0x100;
                let rider = DATA + 0x1000;
                let model = DATA + 0x3000;
                let nodes = DATA + 0x3100;
                let anim = DATA + 0x4000;
                let lists = DATA + 0x4100;
                let seq = DATA + 0x4200;
                let state = DATA + 0x5000;
                let predictor = DATA + 0x6000;
                c.g[A0][0] = sx(this);
                // The recorded start: phase at +0x50, node poses at +0x10..+0x50.
                for k in 0..0x10u32 {
                    oc258_w32(c, this + 0x10 + 4 * k, r.f32(2.0));
                }
                oc258_w32(c, this + 0x50, r.f32(0.5));
                oc258_w32(c, this, r.next_u32() % 4);
                oc258_w32(c, this + WP0_RIDER, rider);
                // Rider: model nodes, anim controller, wipeout state.
                oc258_w32(c, rider + WP0_RIDER_MODEL, model);
                oc258_w32(c, model + 0x2c, nodes);
                for k in 0..0x20u32 {
                    oc258_w32(c, nodes + 4 * k, r.f32(2.0));
                }
                oc258_w32(c, rider + WP0_RIDER_NODE1, r.next_u32() % 4);
                oc258_w32(c, rider + WP0_RIDER_NODE2, r.next_u32() % 4);
                oc258_w32(c, rider + WP0_RIDER_ANIM, anim);
                oc258_w32(c, rider + WP0_RIDER_STATE, state);
                oc258_w32(c, rider + 0x774, 0);
                oc258_w32(c, rider + 0x778, 0);
                oc258_w32(c, rider + 0x788, predictor);
                for k in 0..4u32 {
                    oc258_w32(c, rider + 0x370 + 4 * k, r.f32(1.0));
                    oc258_w32(c, rider + 0x1e0 + 4 * k, r.f32(10.0));
                }
                // Anim controller: track-2 sequence list -> the playing sequence.
                oc258_w32(c, anim + 0x50, lists);
                oc258_w32(c, lists + 0x10 + 4, seq);
                oc258_w32(c, anim + 0x1c, r.f32(2.0));
                let length = 0.5 + (r.next_u32() % 100) as f32 / 50.0;
                let phase = match seed % 3 {
                    0 => 0.97,
                    _ => (r.next_u32() % 100) as f32 / 100.0,
                };
                oc258_w32(c, seq + 0x10, length.to_bits());
                oc258_w32(c, seq + 0x8, (length * phase).to_bits());
                oc258_w32(c, seq + 0x90, r.f32(3.0));
                oc258_w32(c, seq + 0xc0, if seed % 4 == 0 { 0 } else { 1 });
                // Track-2 anim id and its type in the anim table (0x446990, 0x1c each).
                let id = 1 + seed % 7;
                oc258_w32(c, anim + 0x8, id);
                let kind = [0x17u32, 0x1d, 0x19, 0x5][(seed / 4 % 4) as usize];
                oc258_w32(c, 0x0044_6990 + 0x1c * id, kind);
                // Wipeout state: mode at +0x30 (1 or not), its rider at +0x70.
                oc258_w32(c, state + 0x30, if seed / 16 % 2 == 0 { 1 } else { 0 });
                oc258_w32(c, state + 0x70, rider);
                oc258_w32(
                    c,
                    GAME_GP.wrapping_sub(WP0_RECORD_LIMIT_GP),
                    0.95f32.to_bits(),
                );
            },
        );
    }

    /// RiderWipeoutSub_UpdateAnimSpeed: random rider velocities and
    /// sequence speeds (rates below 0.5, inside, above 2, edge floats);
    /// the layer playing animation 0x1d, 0x19, another id or the "none"
    /// id 0x1b6 (the real AnimController getters and the anim table); the
    /// tumble timer below or at least 1; progress negative or not.
    #[test]
    fn rider_wipeout_sub_update_anim_speed_matches() {
        check(
            ee::s00::f_0012e528__generated,
            rider_wipeout_sub_update_anim_speed,
            300,
            &|c, seed| {
                let mut r = Rng(0x0012_e528 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0012_e528;
                let (this, rider, ctrl, motion) =
                    (DATA, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                let (lists, seq) = (DATA + 0x4000, DATA + 0x5000);
                c.bus
                    .write32(GAME_GP - WIPEOUT_INV_TWO_PI_GP, 0.159_154_94f32.to_bits());
                c.bus.write32(this + WIPEOUT_RIDER, rider);
                let timer = if seed % 3 == 0 {
                    1.0 + (r.next_u32() % 4) as f32
                } else {
                    (r.next_u32() % 100) as f32 / 100.0
                };
                c.bus.write32(this + WIPEOUT_TIMER, timer.to_bits());
                c.bus.write32(rider + WIPEOUT_RIDER_ANIM, ctrl);
                c.bus.write32(rider + WIPEOUT_RIDER_MOTION, motion);
                let progress = r.f32(500.0);
                c.bus.write32(rider + WIPEOUT_RIDER_PROGRESS, progress);
                for i in 0..4 {
                    let v = if seed % 11 == 5 {
                        r.edge_f32()
                    } else {
                        r.f32(40.0)
                    };
                    c.bus.write32(motion + WIPEOUT_MOTION_VELOCITY + 4 * i, v);
                }
                // Layer 2's sequence list (+0x50 + 2 * 8) and its first sequence.
                c.bus.write32(ctrl + 0x50, lists);
                c.bus.write32(lists + 0x10 + 4, seq);
                let speed = if seed % 7 == 3 {
                    r.edge_f32()
                } else {
                    r.f32(1.0)
                };
                c.bus.write32(seq + WIPEOUT_SEQ_SPEED, speed);
                c.bus.write32(seq + WIPEOUT_SEQ_SCALE, r.next_u32());
                // Layer 2's animation index and its anim table entry.
                let index = if seed % 9 == 8 {
                    0x1b6
                } else {
                    1 + r.next_u32() % 0x100
                };
                c.bus.write32(ctrl + 8, index);
                let id = [0x1d, 0x19, 0x1c, r.next_u32() % 0x40][(seed % 4) as usize];
                c.bus.write32(0x0044_6990 + index * 0x1c, id);
                c.g[A0][0] = sx(this);
            },
        );
    }

    // (fakes imported above)

    // --------------------------------------------------- RiderSpinSub_TryReverse

    pub(crate) const SPIN_CTRL: u32 = DATA + 0x5_a000;

    pub(crate) const SPIN_MAGIC: u32 = 0x135b_e0a5;

    /// The reverse query: a logged fake whose answer (0 or 1) the fixture
    /// picks at `SPIN_CTRL + 4`.
    pub(crate) fn spin_fake_can_reverse(c: &mut Cpu) {
        if c.bus.read32(SPIN_CTRL) != SPIN_MAGIC {
            return ee::s00::f_00114db8__generated(c);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, SPIN_CAN_REVERSE, ra);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(SPIN_CTRL + 4) as u64;
        }
    }

    pub(crate) fn spin_fake_play(c: &mut Cpu) {
        if c.bus.read32(SPIN_CTRL) != SPIN_MAGIC {
            return ee::s10::f_003128e8__generated(c);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, SPIN_ANIM_PLAY, ra);
    }

    pub(crate) fn spin_install_fakes() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| unsafe {
            ee::s00::OVERRIDE_00114DB8 = spin_fake_can_reverse;
            ee::s10::OVERRIDE_003128E8 = spin_fake_play;
        });
    }

    /// No reverse, or a reverse of angles around +-pi (the floor's
    /// correction when the rounding goes up), with edge floats in the other
    /// fields; the game's pi constants from the image.
    #[test]
    fn rider_spin_sub_try_reverse_matches() {
        spin_install_fakes();
        covered(
            ee::s01::f_00135be0__generated,
            rider_spin_sub_try_reverse,
            240,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0013_5be0, 2);
                c.pc = 0x0013_5be0;
                c.bus.write32(SPIN_CTRL, SPIN_MAGIC);
                c.bus.write32(SPIN_CTRL + 4, (seed % 4 != 0) as u32);
                let sub = DATA + 0x5_0000;
                let rider = DATA + 0x5_1000;
                c.g[A0][0] = sx(sub);
                c.bus.write32(sub + SPIN_SUB_RIDER, rider);
                c.bus.write32(rider + RIDER_ANIM, r.next_u32());
                let angles = [
                    0.0f32, 3.1415927, -3.1415927, 6.2831855, -6.2831855, 9.424778, -9.424778, 1.0,
                    -2.5, 100.0,
                ];
                let angle = if seed % 5 == 4 {
                    r.edge_f32()
                } else {
                    angles[(seed as usize / 4) % angles.len()].to_bits()
                };
                c.bus.write32(sub + SPIN_SUB_ANGLE, angle);
                for off in [SPIN_SUB_10, SPIN_SUB_18, SPIN_SUB_RATE, SPIN_SUB_2C] {
                    c.bus.write32(sub + off, r.edge_f32());
                }
            },
        );
    }

    /// A rider sub-state `SaveState` forwarder: `self` at DATA+0x400, the
    /// stream at DATA+0x800 whose vtable slot 1 is a fake (which unwinds for
    /// some seeds).
    pub(crate) fn rider_sub_save_state_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        use crate::difftest::fakes;
        let mut r = fakes::machine(c, seed, salt, 1);
        c.pc = entry;
        let stream = DATA + 0x800;
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(stream);
        let vt = fakes::vtable(c, &mut r, &[0x8]);
        c.bus.write32(stream, vt);
    }

    #[test]
    fn rider_spin_sub_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s01::f_001361f8__generated,
            rider_spin_sub_save_state,
            200,
            &|c, seed| {
                rider_sub_save_state_setup(c, seed, 0x1361_f8, 0x0013_61f8);
            },
        );
    }

    #[test]
    fn rider_gate_sub_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s00::f_0012c558__generated,
            rider_gate_sub_save_state,
            200,
            &|c, seed| {
                rider_sub_save_state_setup(c, seed, 0x12c5_58, 0x0012_c558);
            },
        );
    }

    /// A stream whose vtable `Write` (+8) is a fake (logs the registers,
    /// sometimes unwinds); `this` comes with junk upper bits on some seeds.
    pub(crate) fn stream_save_setup(c: &mut Cpu, seed: u32, entry: u32) {
        const ME: u32 = DATA;
        const STREAM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        let mut r = machine(c, seed, entry as u64, 1);
        c.pc = entry;
        c.bus.write32(STREAM, VT);
        c.bus.write16(VT + 8, (r.next_u32() % 0x20) as u16 & !3);
        c.bus.write32(VT + 0xc, FAKE);
        c.g[A0][0] = if seed % 4 == 0 {
            (r.next_u32() as u64) << 32 | ME as u64
        } else {
            sx(ME)
        };
        c.g[A1][0] = sx(STREAM);
    }

    /// `RiderHandplantSub_SaveState`.
    #[test]
    fn rider_handplant_sub_save_state_matches() {
        covered(
            ee::s01::f_001330b8__generated,
            rider_handplant_sub_save_state,
            200,
            &|c, seed| stream_save_setup(c, seed, 0x0013_30b8),
        );
    }

    /// `RiderSpinSub_Enter`: the stick x and y held at the jump each zero,
    /// -0, an edge value or in [-1.5, 1.5], so the centred early exit, x
    /// only, y only and both (either axis larger) occur. The real
    /// GetSpinFactor (with Math_Atan), SetSpinDirection,
    /// BECharInt_GetAttrib4Factor (player 0, characters 1..5) and
    /// AnimController_GetLayerSequence run on the game image; the BE
    /// interface getter is a fake (sometimes unwinding).
    #[test]
    fn rider_spin_sub_enter_matches() {
        const THIS: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const CTRL: u32 = DATA + 0x3000;
        const LAYERS: u32 = DATA + 0x3100;
        const SEQ: u32 = DATA + 0x3200;
        const BE_LIBRARY: u32 = DATA + 0x5000;
        const BE_OBJ: u32 = DATA + 0x5100;
        const BE_VT: u32 = DATA + 0x5200;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        covered(
            ee::s01::f_00133128__generated,
            rider_spin_sub_enter,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0013_3128, 1);
                elf_image(c);
                c.pc = 0x0013_3128;
                c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(BE_INTERFACES, BE_OBJ);
                c.bus.write32(BE_OBJ + 4, BE_VT);
                c.bus.write32(BE_VT + 0xc, FAKE);
                c.bus.write8(0x0053_08e2, 10);
                c.bus.write8(0x0053_08e4, 10);
                fill(c, &mut r, THIS, 0x60 / 4);
                c.bus.write32(THIS + SPIN_ENTER_RIDER, RIDER);
                let axis = |r: &mut Rng, k: u32| match k % 4 {
                    0 => 0,
                    1 => 0x8000_0000,
                    2 => r.edge_f32(),
                    _ => r.f32(1.5),
                };
                let x = axis(&mut r, seed);
                let y = axis(&mut r, seed / 4);
                c.bus.write32(RIDER + SPIN_ENTER_STICK_X, x);
                c.bus.write32(RIDER + SPIN_ENTER_STICK_Y, y);
                c.bus
                    .write32(RIDER + SPIN_ENTER_RIDER_SPIN_RATE, r.next_u32());
                c.bus
                    .write32(RIDER + SPIN_ENTER_RIDER_FLIP_RATE, r.next_u32());
                c.bus.write32(RIDER + SPIN_ENTER_ANIM_CTRL, CTRL);
                c.bus.write32(RIDER + SPIN_ENTER_PLAYER, 0);
                c.bus
                    .write32(RIDER + SPIN_ENTER_CHARACTER, 1 + r.next_u32() % 5);
                // Layer 2 of the controller: a one-entry sequence list.
                c.bus.write32(CTRL + 0x50, LAYERS);
                c.bus.write32(LAYERS + 2 * 8 + 4, SEQ);
                c.bus.write32(SEQ + SPIN_ENTER_SEQ_SPEED, r.next_u32());
                c.g[A0][0] = sx(THIS);
            },
        );
    }

    /// A replay stream whose Write virtual is a fake (sometimes unwinding),
    /// `self` anywhere in the data area.
    pub(crate) fn rider_save_state_setup(c: &mut Cpu, seed: u32, entry: u32) {
        use crate::difftest::fakes::{ptr, vtable};
        let mut r = machine(c, seed, entry as u64, 1);
        c.pc = entry;
        let vt = vtable(c, &mut r, &[SAVE_STATE_STREAM_WRITE]);
        let stream = DATA + 0x20_0000 + (r.next_u32() & 0xfff0);
        c.bus.write32(stream, vt);
        c.g[A0][0] = sx(ptr(&mut r));
        c.g[A1][0] = sx(stream);
    }

    /// `RiderRailslideSub_SaveState`: one stream write of 8 bytes.
    #[test]
    fn rider_railslide_sub_save_state_matches() {
        covered(
            ee::s01::f_00132840__generated,
            rider_railslide_sub_save_state,
            200,
            &|c, seed| rider_save_state_setup(c, seed, 0x0013_2840),
        );
    }

    /// `RiderSpinSub_Exit`: spin rates in [-10, 10] (after the 0.75 scale
    /// above 3pi/2, above pi, positive, or not), start/current angles in
    /// either order. The real AnimController_FadeOutLayer (empty layer),
    /// RiderSpinSub_ApplyPoseRotation (with GetSpinFactor, Math_Atan and
    /// the sin/cos helper) and Rider_BuildWorldMatrix run on random floats.
    #[test]
    fn rider_spin_sub_exit_matches() {
        const THIS: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const CTRL: u32 = DATA + 0x3000;
        const LAYERS: u32 = DATA + 0x3100;
        const BOARD: u32 = DATA + 0x5000;
        const BONES: u32 = DATA + 0x6000;
        check(
            ee::s01::f_00134cb0__generated,
            rider_spin_sub_exit,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0013_4cb0, 0);
                elf_image(c);
                c.pc = 0x0013_4cb0;
                for i in 0..0x60 / 4 {
                    c.bus.write32(THIS + 4 * i, r.f32(8.0));
                }
                c.bus.write32(THIS + SPIN_ENTER_RIDER, RIDER);
                for i in 0..0xc00 / 4 {
                    c.bus.write32(RIDER + 4 * i, r.f32(4.0));
                }
                c.bus
                    .write32(RIDER + SPIN_ENTER_RIDER_SPIN_RATE, r.f32(10.0));
                c.bus.write32(RIDER + SPIN_ENTER_ANIM_CTRL, CTRL);
                c.bus.write32(RIDER + 0x780, BOARD);
                c.bus.write32(RIDER + 0x89c, r.next_u32() % 8);
                for i in 0..0x200 / 4 {
                    c.bus.write32(BOARD + 4 * i, r.f32(2.0));
                    c.bus.write32(BONES + 4 * i, r.f32(2.0));
                }
                c.bus.write32(BOARD + 0x24, BONES);
                // Layer 1 of the controller: an empty sequence list.
                c.bus.write32(CTRL + 0x50, LAYERS);
                c.bus.write32(LAYERS + 8 + 4, 0);
                c.g[A0][0] = sx(THIS);
            },
        );
    }

    /// The stream setup of the rider-state SaveState tests: `this` at DATA,
    /// the stream's write is a fake (some unwind).
    pub(crate) fn rider_state_save_setup(c: &mut Cpu, seed: u32, entry: u32) {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::DATA;
        let mut r = machine(c, seed, entry as u64, 1);
        c.pc = entry;
        let stream = DATA + 0x800;
        let vt = DATA + 0x900;
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(stream);
        c.bus.write32(stream, vt);
        c.bus.write16(vt + 0x8, (r.next_u32() & 0xfffc) as u16);
        c.bus.write32(vt + 0xc, FAKE);
    }

    /// RiderBoardPressSub_SaveState: write(self, 0x14) through a fake.
    #[test]
    fn rider_board_press_sub_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s00::f_00131598__generated,
            rider_board_press_sub_save_state,
            64,
            &|c, seed| rider_state_save_setup(c, seed, 0x0013_1598),
        );
    }

    /// The rider-state SaveState forwarders: `a1` a stream whose vtable slot
    /// 8 is a fake (random delta; some calls unwind), `a0` the state.
    pub(crate) fn rider_state_save_setup_2(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        use crate::difftest::fakes::{machine, vtable};
        use crate::difftest::DATA;
        let mut r = machine(c, seed, salt, 1);
        c.pc = entry;
        let stream = DATA + 0x800;
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(stream);
        let vt = vtable(c, &mut r, &[8]);
        c.bus.write32(stream, vt);
    }

    #[test]
    fn rider_natural_air_sub_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s00::f_0012fbf0__generated,
            rider_natural_air_sub_save_state,
            60,
            &|c, seed| rider_state_save_setup_2(c, seed, 0x12_fbf0, 0x0012_fbf0),
        );
    }

    /// A state at DATA+0x800 saved to a stream at DATA+0x10 whose write
    /// method (slot +0x8, random `this` delta) is a fake that may unwind.
    pub(crate) fn oc286_save_block_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        use crate::difftest::fakes::vtable;
        let mut r = machine(c, seed, salt, 1);
        let stream = DATA + 0x10;
        let vt = vtable(c, &mut r, &[0x8]);
        c.bus.write32(stream, vt);
        c.g[A0][0] = sx(DATA + 0x800);
        c.g[A1][0] = sx(stream);
        c.pc = entry;
    }

    #[test]
    fn rider_reset_sub_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s00::f_0012f5b0__generated,
            rider_reset_sub_save_state,
            200,
            &|c, seed| oc286_save_block_setup(c, seed, 0x12_f5b0, 0x0012_f5b0),
        );
    }

    /// A stream at a random address whose vtable's slot 1 is a fake (which
    /// unwinds in some seeds), and a random `self`.
    pub(crate) fn save_state_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        use crate::difftest::fakes::{ptr, vtable};
        let mut r = machine(c, seed, salt, 1);
        let stream = ptr(&mut r);
        let vt = vtable(c, &mut r, &[8]);
        c.bus.write32(stream, vt);
        c.g[A0][0] = sx(ptr(&mut r));
        c.g[A1][0] = sx(stream);
        c.g[RA][0] = SENTINEL as u64;
    }

    #[test]
    fn rider_wipeout_sub_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s00::f_0012e708__generated,
            rider_wipeout_sub_save_state,
            300,
            &|c, seed| {
                save_state_fixture(c, seed, 0x0012_e708);
                c.pc = 0x0012_e708;
            },
        );
    }

    /// The world painter's six view states (globals at 0x4fa370, stride
    /// 0xf0, field +0x20 -> holder -> object with a vtable at +4 whose
    /// slot 0x200 is a fake): `WorldPainter_SaveViewState` makes 12 fake
    /// calls.
    pub(crate) fn ai_save_view_state_fixture(c: &mut Cpu) {
        use crate::difftest::fakes::FAKE;
        let holder = DATA + 0x30000;
        let obj = DATA + 0x30100;
        let vt = DATA + 0x30200;
        for k in 0..6 {
            c.bus.write32(0x004f_a370 + 0x20 + k * 0xf0, holder);
        }
        c.bus.write32(holder, obj);
        c.bus.write32(obj + 4, vt);
        c.bus.write16(vt + 0x200, 0);
        c.bus.write32(vt + 0x204, FAKE + 0x800);
    }

    /// An AI block with 0..2 players, 0..5 mission riders and 0..8 actors
    /// (or negative counts), each rider's replay object saving through a
    /// fake; a replay frame whose `Write` is a fake.
    pub(crate) fn ai_save_state_fixture(c: &mut Cpu, seed: u32) {
        use crate::difftest::fakes::{machine, FAKE};
        let mut r = Rng(0x12b7_f0ff ^ (seed as u64) << 4);
        let mut pick = |max: u32| -> u32 {
            match r.next_u32() % 5 {
                0 => 0,
                1 => (-(1 + (r.next_u32() % 2) as i32)) as u32,
                _ => 1 + r.next_u32() % max,
            }
        };
        let counts = [pick(2), pick(5), pick(8)];
        let calls: u32 = 1
            + 12
            + counts
                .iter()
                .map(|&n| if (n as i32) > 0 { n } else { 0 })
                .sum::<u32>();
        let mut r = machine(c, seed, 0x12b7_f000, calls);
        let ai = DATA;
        let frame = DATA + 0x200;
        let frame_vt = DATA + 0x300;
        c.bus.write32(frame, frame_vt);
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(frame_vt + AI_SAVE_FRAME_WRITE, delta as u16);
        c.bus.write32(frame_vt + AI_SAVE_FRAME_WRITE + 4, FAKE);
        let lists = [AI_SAVE_PLAYERS, AI_SAVE_MISSION, AI_SAVE_ACTORS];
        let count_offs = [AI_SAVE_NUM_PLAYERS, AI_SAVE_NUM_MISSION, AI_SAVE_NUM_ACTORS];
        let mut k = 0;
        for l in 0..3 {
            c.bus.write32(ai + count_offs[l], counts[l]);
            let n = if (counts[l] as i32) > 0 { counts[l] } else { 0 };
            for i in 0..n {
                let rider = DATA + 0x1000 + k * 0x100;
                let obj = DATA + 0x4000 + k * 0x800;
                let replay = DATA + 0x10000 + k * 0x1000;
                let vt = DATA + 0x20000 + k * 0x40;
                c.bus.write32(ai + lists[l] + 4 * i, rider);
                c.bus.write32(rider + AI_SAVE_RIDER_OBJ, obj);
                c.bus.write32(obj + AI_SAVE_OBJ_REPLAY, replay);
                c.bus.write32(replay + AI_SAVE_REPLAY_VT, vt);
                let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                c.bus.write16(vt + AI_SAVE_REPLAY_SAVE, delta as u16);
                c.bus
                    .write32(vt + AI_SAVE_REPLAY_SAVE + 4, FAKE + 0x10 * (k + 1));
                k += 1;
            }
        }
        ai_save_view_state_fixture(c);
        c.g[A0][0] = sx(ai);
        c.g[A1][0] = sx(frame);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x0012_b7f0;
    }

    /// `AI_SaveState`: empty and filled lists, fakes unwinding at any call.
    #[test]
    fn ai_save_state_matches() {
        check(
            ee::s00::f_0012b7f0__generated,
            ai_save_state,
            300,
            &ai_save_state_fixture,
        );
    }

    /// A sub-state at DATA+0x400 and a stream whose `Write` slot (0x8) is a
    /// fake that may unwind.
    pub(crate) fn sub_save_state_fixture(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        use crate::difftest::fakes::{machine, vtable};
        let mut r = machine(c, seed, salt, 1);
        let stream = DATA + 0x800;
        let vt = vtable(c, &mut r, &[SUB_SAVE_STATE_WRITE]);
        c.bus.write32(stream, vt);
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(stream);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = entry;
    }

    /// `RiderUberGrindSub_SaveState`: the stream write, returning or
    /// unwinding.
    #[test]
    fn rider_uber_grind_sub_save_state_matches() {
        check(
            ee::s01::f_001368e8__generated,
            rider_uber_grind_sub_save_state,
            200,
            &|c, seed| sub_save_state_fixture(c, seed, 0x1368_e800, 0x0013_68e8),
        );
    }

    /// `RiderFinishLineSub_SaveState`: the stream write, returning or
    /// unwinding.
    #[test]
    fn rider_finish_line_sub_save_state_matches() {
        check(
            ee::s00::f_0012c9c0__generated,
            rider_finish_line_sub_save_state,
            200,
            &|c, seed| sub_save_state_fixture(c, seed, 0x12c9_c000, 0x0012_c9c0),
        );
    }

    /// `RiderCruiseSub_Exit`: next state 4 (nothing), layer 1 playing an
    /// anim of type 3, 13, another type or none (id 0x1b6); the fade runs
    /// the real `FadeOutLayer` (layer 0's sequence list empty) and
    /// `GetLayerSequence`.
    #[test]
    fn rider_cruise_sub_exit_matches() {
        const SUB: u32 = DATA + 0x20_0000;
        const RIDER: u32 = DATA + 0x20_1000;
        const CTRL: u32 = DATA + 0x20_2000;
        const LISTS: u32 = DATA + 0x20_3000;
        const SEQ: u32 = DATA + 0x20_4000;
        check(
            ee::s00::f_00131c30__generated,
            rider_cruise_sub_exit,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x13_1c30, 0);
                c.pc = 0x0013_1c30;
                c.g[A0][0] = sx(SUB);
                c.g[A1][0] = if seed % 6 == 0 {
                    4
                } else {
                    (r.next_u32() % 8) as u64
                };
                c.bus.write32(SUB, RIDER);
                c.bus.write32(RIDER + RCSE_ANIM, CTRL);
                c.bus.write32(GAME_GP - 0x75b0, 0x3dcc_cccd);
                // Layer anim ids 0..3 (or none) and their types.
                let id = if seed % 7 == 1 { 0x1b6 } else { seed / 2 % 4 };
                c.bus.write32(CTRL, r.next_u32() % 4);
                c.bus.write32(CTRL + 4, id);
                for (k, ty) in [3u32, 0xd, 5, 0].into_iter().enumerate() {
                    c.bus.write32(0x0044_6990 + 0x1c * k as u32, ty);
                }
                // Sequence lists: layer 0 empty; layer 1's head sequence.
                c.bus.write32(CTRL + 0x50, LISTS);
                c.bus.write32(LISTS + 4, 0);
                c.bus.write32(LISTS + 8 + 4, SEQ);
                c.bus.write32(SEQ + 0xc8, 0);
                c.bus.write32(SEQ + 0x90, r.next_u32());
            },
        );
    }

    /// Spin and flip of every sign (and zero, and the odd edge values), the
    /// rate literals at their gp slots, random angles and pending deltas.
    #[test]
    fn rider_spin_sub_set_spin_direction_matches() {
        use crate::difftest::GAME_GP;
        use spin_direction as k;
        check_paths(
            ee::s01::f_00135b30__generated,
            rider_spin_sub_set_spin_direction,
            300,
            &[
                4 + 3 + 4 + 3 + 17,
                4 + 4 + 2 + 4 + 4 + 17,
                4 + 4 + 4 + 4 + 17,
            ],
            &|c, seed| {
                let mut r = Rng(0x135b_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0013_5b30;
                c.g[GP][0] = sx(GAME_GP);
                for (slot, v) in [
                    (k::GP_PI, 0x4049_0fdbu32),
                    (k::GP_NEG_PI, 0xc049_0fdb),
                    (k::GP_TWO_PI, 0x40c9_0fdb),
                    (k::GP_NEG_TWO_PI, 0xc0c9_0fdb),
                ] {
                    c.bus.write32(GAME_GP - slot, v);
                }
                let sign = |r: &mut Rng, m: u32| match m % 4 {
                    0 => r.f32(10.0) & 0x7fff_ffff,
                    1 => r.f32(10.0) | 0x8000_0000,
                    2 => 0,
                    _ => r.edge_f32(),
                };
                c.f[12] = sign(&mut r, seed);
                c.f[13] = sign(&mut r, seed / 4);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                for off in (0..0x30).step_by(4) {
                    c.bus.write32(this + off, r.f32(100.0));
                }
            },
        );
    }

    #[test]
    fn rider_natural_air_sub_enter_matches() {
        check(
            ee::s00::f_0012f620__generated,
            rider_natural_air_sub_enter,
            300,
            &|c, seed| {
                let mut r = Rng(0x12f6_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0012_f620;
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                fill(c, &mut r, crate::difftest::SP - 0x80, 0x20);
                let gp = crate::difftest::GAME_GP;
                for (off, v) in [
                    (NAT_AIR_GP_DOWN, -std::f32::consts::FRAC_PI_2),
                    (NAT_AIR_GP_UP, std::f32::consts::FRAC_PI_2),
                    (NAT_AIR_GP_PLUS_PI, std::f32::consts::PI),
                    (NAT_AIR_GP_MINUS_PI, std::f32::consts::PI),
                    (NAT_AIR_GP_STEP, 1.0 / 30.0),
                ] {
                    c.bus.write32(gp - off, v.to_bits());
                }
                let this = DATA;
                let rider = DATA + 0x1000;
                fill(c, &mut r, this, 0x8);
                fill(c, &mut r, rider + 0x1e0, 0x40);
                c.bus.write32(this + NAT_AIR_RIDER, rider);
                // Zero (both signs), edge cases, or ordinary velocities.
                let comp = |r: &mut Rng, k: u32| match k % 4 {
                    0 => (r.next_u32() & 1) << 31,
                    1 => r.edge_f32(),
                    _ => r.f32(50.0),
                };
                let vx = comp(&mut r, seed);
                let vy = comp(&mut r, seed / 4);
                c.bus.write32(rider + NAT_AIR_RIDER_VX, vx);
                c.bus.write32(rider + NAT_AIR_RIDER_VY, vy);
                c.g[A0][0] = this as u64;
            },
        );
    }
}
