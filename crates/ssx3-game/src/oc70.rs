//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for fifty small generated functions of wave 14 — memcard
//! request wrappers, the five `MenuSys*Item_Measure` trampolines, camera
//! algorithm constructors, the `cFEState*` constructors, trigger/sphere-tree
//! helpers and a handful of leaf bit and index helpers. Each has a
//! differential test against the generated body.
//!
//! The wrappers follow [`crate::call`] exactly: `Frame::open`/`save`/`save_ra`
//! for the stack frame, `Callee::call` for the in-shard `jal`s the generated
//! bodies make and `call_ptr` for the cross-shard ones, `?` for the unwind
//! check (a callee that did not come back to the expected address makes the
//! caller return at once, leaving every register as it is) and
//! `Frame::close` for `jr ra`. `v0`/`v1` and the other scratch registers are
//! left exactly as the generated code leaves them, including the `-1` in
//! `SphereTree_ctor` and the `0x460000 - vtable` constants the constructors
//! store.
//!
//! The four loops that check events at their back edge
//! (`WorldTriggerManager_FlagAllActive`, `UITextureSet_Find`,
//! `RCMP_IsMpegChunkTag`, `BEMissionInt_GetTotalCollectibles`, and the callers
//! of the first) use [`crate::events::event_due`] +
//! [`crate::events::deliver_event_here`] with the loop head as `pc`, as
//! documented there.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_6a68, world_trigger_manager_flag_all_active, "WorldTriggerManager_FlagAllActive"),
    (0x002c_6810, c_ps2_mem_card_request_list_dir_all, "cPS2MemCard_RequestListDirAll"),
    (0x002c_6968, c_ps2_mem_card_request_open, "cPS2MemCard_RequestOpen"),
    (0x002c_6a28, c_ps2_mem_card_do_close, "cPS2MemCard_DoClose"),
    (0x002c_6c50, c_ps2_mem_card_request_mkdir, "cPS2MemCard_RequestMkdir"),
    (0x002c_d240, menu_sys_callback_item_measure, "MenuSysCallbackItem_Measure"),
    (0x002c_d7b8, menu_sys_int8_item_measure, "MenuSysInt8Item_Measure"),
    (0x002c_dbf0, menu_sys_int_item_measure, "MenuSysIntItem_Measure"),
    (0x002c_dfa0, menu_sys_float_item_measure, "MenuSysFloatItem_Measure"),
    (0x002c_e368, menu_sys_float_range_item_measure, "MenuSysFloatRangeItem_Measure"),
    (0x002e_4540, c_transition_man_start_captured, "cTransitionMan_StartCaptured"),
    (0x002e_f378, world_painter_set_irradiance_override, "WorldPainter_SetIrradianceOverride"),
    (0x0030_8f38, w_script_man_delete_all_processes, "WScriptMan_DeleteAllProcesses"),
    (0x0031_0ee0, rider_model_load_state, "RiderModel_LoadState"),
    (0x0032_c508, sphere_tree_ctor, "SphereTree_ctor"),
    (0x0032_fe40, coll_sphere_collide_aabb, "CollSphere_CollideAABB"),
    (0x0033_06d8, coll_sphere_tree_get_diameter, "CollSphereTree_GetDiameter"),
    (0x0033_4850, nash_destroy_fwd_diff_cache, "Nash_DestroyFwdDiffCache"),
    (0x0034_ef08, floating_free_bound_box, "Floating_FreeBoundBox"),
    (0x0035_ed90, roller_modifier_translate, "RollerModifier_Translate"),
    (0x0036_9690, ps2_graphics_man_set_texture_name_cache, "PS2GraphicsMan_SetTextureNameCache"),
    (0x0039_8380, ui_texture_set_find, "UITextureSet_Find"),
    (0x0039_9730, ui_icon_set_texture, "UIIcon_SetTexture"),
    (0x003b_11f0, rcmp_is_mpeg_chunk_tag, "RCMP_IsMpegChunkTag"),
    (0x0012_a490, ai_path_data_init_if_early_section, "AI_PathDataInitIfEarlySection"),
    (0x0014_7580, be_new_player_int_get_slot_flag3, "BENewPlayerInt_GetSlotFlag3"),
    (0x0014_de28, be_reset_player_profile, "BE_ResetPlayerProfile"),
    (0x0014_f7a8, be_option_int_get_player_handicap, "BEOptionInt_GetPlayerHandicap"),
    (0x0015_3350, be_mission_int_get_total_collectibles, "BEMissionInt_GetTotalCollectibles"),
    (0x0015_a358, be_reward_int_set_granted_bit, "BERewardInt_SetGrantedBit"),
    (0x0015_a5b0, be_reward_int_new_rewards_set_bit, "BERewardInt_NewRewardsSetBit"),
    (0x0015_cee8, manual_cam_controller_reset, "ManualCamController_Reset"),
    (0x0015_fd08, manual_cam_algo_ctor, "ManualCamAlgo_ctor"),
    (0x0015_ffb0, editor_cam_algo_ctor, "EditorCamAlgo_ctor"),
    (0x0016_0438, manual_chase_camera_algorithm_ctor, "ManualChaseCameraAlgorithm_ctor"),
    (0x0016_16a8, idle_chase_camera_algorithm_ctor, "IdleChaseCameraAlgorithm_ctor"),
    (0x0016_2218, chase_cam_controller_exit_spoke_mode, "ChaseCamController_ExitSpokeMode"),
    (0x0016_82b8, lazy_chase_camera_algorithm_ctor, "LazyChaseCameraAlgorithm_ctor"),
    (0x0016_8650, look_back_chase_camera_algorithm_ctor, "LookBackChaseCameraAlgorithm_ctor"),
    (0x0016_89c8, mid_right_chase_camera_algorithm_ctor, "MidRightChaseCameraAlgorithm_ctor"),
    (0x0016_8cd8, mid_left_chase_camera_algorithm_ctor, "MidLeftChaseCameraAlgorithm_ctor"),
    (0x0016_c098, camera_trigger_man_destroy, "CameraTriggerMan_Destroy"),
    (0x0017_6a68, default_2_chase_camera_algorithm_ctor, "Default_2ChaseCameraAlgorithm_ctor"),
    (0x0017_6d68, default_3_chase_camera_algorithm_ctor, "Default_3ChaseCameraAlgorithm_ctor"),
    (0x0017_7068, default_4_chase_camera_algorithm_ctor, "Default_4ChaseCameraAlgorithm_ctor"),
    (0x0017_7e50, spoke_chase_camera_algorithm_ctor, "SpokeChaseCameraAlgorithm_ctor"),
    (0x0018_8ee8, c_fe_state_options_game_ctor, "cFEStateOptionsGame_ctor"),
    (0x0018_f278, c_fe_state_profile_load_v26, "cFEStateProfileLoad_v26"),
    (0x0019_1e08, c_fe_state_rule_select_v21, "cFEStateRuleSelect_v21"),
    (0x0019_4738, c_fe_state_title_ctor, "cFEStateTitle_ctor"),
];

// ---------------------------------------------------------------------------
// Callees, under the names the generated bodies give them.

mod callee {
    use super::{ee, Callee};

    /// `MenuSysItem_MeasureRow` (0x2ca4c8).
    pub const MEASURE_ROW: Callee = Callee { addr: 0x002c_a4c8, f: ee::s09::f_002ca4c8 };
    /// `cPS2MemCard_SetState` (0x2c48c0): the card state machine.
    pub const MEMCARD_SET_STATE: Callee = Callee { addr: 0x002c_48c0, f: ee::s08::f_002c48c0 };
    /// `cTransitionMan_Start` (0x2e4370).
    pub const TRANSITION_MAN_START: Callee = Callee { addr: 0x002e_4370, f: ee::s09::f_002e4370 };
    /// `WScriptProcessList_DeleteAll` (0x30b1b8).
    pub const WSCRIPT_DELETE_ALL: Callee = Callee { addr: 0x0030_b1b8, f: ee::s10::f_0030b1b8 };
    /// `SphereTree_ResetTransform` (0x32c540).
    pub const SPHERE_TREE_RESET_TRANSFORM: Callee =
        Callee { addr: 0x0032_c540, f: ee::s10::f_0032c540 };
    /// `CamController_Reset` (0x15ccf0).
    pub const CAM_CONTROLLER_RESET: Callee = Callee { addr: 0x0015_ccf0, f: ee::s01::f_0015ccf0 };
    /// `ManualCamAlgoBase_ctor` (0x15f938).
    pub const MANUAL_CAM_ALGO_BASE_CTOR: Callee =
        Callee { addr: 0x0015_f938, f: ee::s01::f_0015f938 };
    /// `ChaseCameraAlgorithm_ctor` (0x162318).
    pub const CHASE_CAMERA_ALGORITHM_CTOR: Callee =
        Callee { addr: 0x0016_2318, f: ee::s01::f_00162318 };
    /// `ChaseCamController_SetAlgorithm` (0x15d078).
    pub const CHASE_CAM_SET_ALGORITHM: Callee =
        Callee { addr: 0x0015_d078, f: ee::s01::f_0015d078 };
    /// `CameraTriggerMan_FreeTriggers` (0x16bf40).
    pub const CAMERA_TRIGGER_FREE_TRIGGERS: Callee =
        Callee { addr: 0x0016_bf40, f: ee::s02::f_0016bf40 };
    /// `CameraTriggerMan_ResetHeader` (0x16c390).
    pub const CAMERA_TRIGGER_RESET_HEADER: Callee =
        Callee { addr: 0x0016_c390, f: ee::s02::f_0016c390 };
    /// `UITextureSet_Find` (0x398380).
    pub const UI_TEXTURE_SET_FIND: Callee = Callee { addr: 0x0039_8380, f: ee::s13::f_00398380 };
    /// `UIState_ctor` (0x39e2a0).
    pub const UI_STATE_CTOR: Callee = Callee { addr: 0x0039_e2a0, f: ee::s13::f_0039e2a0 };
    /// `BENewPlayerInt_GetSlot` (0x147798; dispatches through its override slot).
    pub const BENEWPLAYER_GET_SLOT: Callee = Callee { addr: 0x0014_7798, f: ee::s01::f_00147798 };
    /// `BELibrary_ResetPlayerProfile` (0x149a88).
    pub const BE_LIBRARY_RESET_PROFILE: Callee =
        Callee { addr: 0x0014_9a88, f: ee::s01::f_00149a88 };
    /// `BEBAGTInt_ResetProfile` (0x14fc40).
    pub const BEBAGT_RESET_PROFILE: Callee = Callee { addr: 0x0014_fc40, f: ee::s01::f_0014fc40 };
    /// `BEOptionInt_GetMaxHandicap` (0x14f870).
    pub const BE_OPTION_GET_MAX_HANDICAP: Callee =
        Callee { addr: 0x0014_f870, f: ee::s01::f_0014f870 };
    /// `cFEStateProfileLoad_RefreshProfileList` (0x18f2b8).
    pub const FE_REFRESH_PROFILE_LIST: Callee =
        Callee { addr: 0x0018_f2b8, f: ee::s02::f_0018f2b8 };
    /// `cFEStateProfileLoad_RefreshSlots` (0x18f9d8).
    pub const FE_REFRESH_SLOTS: Callee = Callee { addr: 0x0018_f9d8, f: ee::s02::f_0018f9d8 };
}

/// `strcpy` (0x4166f4) — reached through the function table.
const STRLEN_AND_STRCPY: u32 = 0x0041_66f4;
/// `sceMcClose` (0x409e70).
const SCE_MC_CLOSE: u32 = 0x0040_9e70;
/// `__builtin_delete` (0x317e50).
const DELETE: u32 = 0x0031_7e50;
/// `EA_MemFree` (0x317e98).
const MEM_FREE: u32 = 0x0031_7e98;
/// `EA_MemCopy` (0x3e6574).
const MEM_COPY: u32 = 0x003e_6574;
/// `StrHash` (0x317670).
const STR_HASH: u32 = 0x0031_7670;
/// `SectionMan_FindLogicalSection` (0x22e0e0).
const FIND_LOGICAL_SECTION: u32 = 0x0022_e0e0;
/// `PathData_EmptyHook` (0x26ada0).
const PATH_DATA_EMPTY_HOOK: u32 = 0x0026_ada0;
/// `SphereTree_GetRadius` (0x32c590).
const SPHERE_TREE_GET_RADIUS: u32 = 0x0032_c590;
/// `SphereTree_Translate` (0x32c630).
const SPHERE_TREE_TRANSLATE: u32 = 0x0032_c630;
/// `AIFwdDiffCache_dtor` (0x327828).
const AI_FWD_DIFF_CACHE_DTOR: u32 = 0x0032_7828;
/// `SphereTree_SweepSphereAABB` (0x329590).
const SPHERE_TREE_SWEEP_AABB: u32 = 0x0032_9590;

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// WorldTriggerManager_FlagAllActive.

/// `void WorldTriggerManager_FlagAllActive(WorldTriggerManager* self)`
/// (0x2b6a68): for all 40 trigger slots (0x70 bytes each, the array at
/// `*self`), set the `+0x34` flag of every slot whose first word is 1.
pub fn world_trigger_manager_flag_all_active(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = 1;
    c.g[A2][0] = 0;
    c.g[A1][0] = 0x27;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    loop {
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        let not_active = c.g[V0][0] != c.g[A3][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x70));
        if !not_active {
            c.cycles += 1;
            let slot = (c.g[V1][0] as u32).wrapping_add(0x34);
            let one = c.g[V0][0];
            c.sw(slot, one);
        }
        c.cycles += 2;
        if (c.g[A1][0] as i64) >= 0 {
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x002b_6a78);
            }
            continue;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
}

// ---------------------------------------------------------------------------
// cPS2MemCard request wrappers.

/// `void cPS2MemCard_RequestListDirAll(this, const char* name)` (0x2c6810):
/// clear the result field at `+0x54`, copy `name` to the work buffer at
/// `+0x139` and enter state 0x3c.
pub fn c_ps2_mem_card_request_list_dir_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.sw((this as u32).wrapping_add(0x54), 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x139));
        call_ptr(c, STRLEN_AND_STRCPY, 0x002c_682c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x3c;
        callee::MEMCARD_SET_STATE.call(c, 0x002c_6838)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestOpen(this, const char* name, int mode)`
/// (0x2c6968): store `mode` at `+0x14e0`, copy the name to `+0xf9` and enter
/// state 0x3f.
pub fn c_ps2_mem_card_request_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let mode = c.g[A2][0];
        c.sw((this as u32).wrapping_add(0x14e0), mode);
        c.g[A0][0] = sx((this as u32).wrapping_add(0xf9));
        call_ptr(c, STRLEN_AND_STRCPY, 0x002c_6984)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x3f;
        callee::MEMCARD_SET_STATE.call(c, 0x002c_6990)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoClose(this)` (0x2c6a28): close the file descriptor at
/// `+0x18`, keep the result in the `gp + 0x850` slot and enter state 0x4a.
pub fn c_ps2_mem_card_do_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A0][0] = Addr((this as u32).wrapping_add(0x18)).lw(c);
        call_ptr(c, SCE_MC_CLOSE, 0x002c_6a40)?;
        c.g[A0][0] = c.g[S0][0];
        let result = c.g[V0][0];
        let slot = gp(c, 0x850);
        c.sw(slot, result);
        c.g[A1][0] = 0x4a;
        callee::MEMCARD_SET_STATE.call(c, 0x002c_6a50)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestMkdir(this, const char* name, int mode)`
/// (0x2c6c50): store `mode` at `+0x20`, copy the name to `+0xf9` and enter
/// state 0x4b.
pub fn c_ps2_mem_card_request_mkdir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let mode = c.g[A2][0];
        c.sw((this as u32).wrapping_add(0x20), mode);
        c.g[A0][0] = sx((this as u32).wrapping_add(0xf9));
        call_ptr(c, STRLEN_AND_STRCPY, 0x002c_6c6c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x4b;
        callee::MEMCARD_SET_STATE.call(c, 0x002c_6c78)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MenuSys*Item_Measure trampolines.
//
// Each is the same 7-instruction wrapper: `t0` receives the static row layout
// table, `a2` the item's value/count field at `+0x14`, `a3` zero, then
// `MenuSysItem_MeasureRow(row, item)` lays the row out; the item is returned.

/// `void* menu_measure(this, item, table, ret)`.
fn menu_measure(c: &mut Cpu, table: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[T0][0] = 0x0048_0000;
        frame.save(c, S0, 0x10);
        c.g[T0][0] = sx(table);
        frame.save_ra(c, 0);
        let row = c.g[A0][0];
        c.g[S0][0] = row;
        c.g[A3][0] = 0;
        c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
        callee::MEASURE_ROW.call(c, ret)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void* MenuSysCallbackItem_Measure(this, item)` (0x2cd240): lay the item
/// out with the callback-item descriptor table.
pub fn menu_sys_callback_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66c8, 0x002c_d264);
}

/// `void* MenuSysInt8Item_Measure(this, item)` (0x2cd7b8): lay the item out
/// with the integer-item descriptor table.
pub fn menu_sys_int8_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66e0, 0x002c_d7dc);
}

/// `void* MenuSysIntItem_Measure(this, item)` (0x2cdbf0): lay the item out
/// with the integer-item descriptor table.
pub fn menu_sys_int_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66e0, 0x002c_dc14);
}

/// `void* MenuSysFloatItem_Measure(this, item)` (0x2cdfa0): lay the item out
/// with the float-item descriptor table.
pub fn menu_sys_float_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66f0, 0x002c_dfc4);
}

/// `void* MenuSysFloatRangeItem_Measure(this, item)` (0x2ce368): lay the item
/// out with the float-item descriptor table.
pub fn menu_sys_float_range_item_measure(c: &mut Cpu) {
    menu_measure(c, 0x0048_66f0, 0x002c_e38c);
}

// ---------------------------------------------------------------------------
// cTransitionMan_StartCaptured.

/// `void cTransitionMan_StartCaptured(this, a1, a2, a3, float duration)`
/// (0x2e4540): forward to `cTransitionMan_Start` with a zeroed destination
/// mode, the zero/fraction pair `(0, 0)`, `t0 = 1` and the caller's `a3` in
/// `t1`; the capture duration moves from `f12` to `f14`.
pub fn c_transition_man_start_captured(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.mtc1(0, 0);
        c.mov_s(14, 12);
        c.g[T1][0] = c.g[A3][0];
        let frame = Frame::open(c, 0x10);
        c.mov_s(12, 0);
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.mov_s(13, 12);
        c.g[T0][0] = 1;
        callee::TRANSITION_MAN_START.call(c, 0x002e_456c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldPainter_SetIrradianceOverride.

/// `void WorldPainter_SetIrradianceOverride(const char* name)` (0x2ef378):
/// copy `name` into the world painter's override string at `gp + 0x24c0` and
/// mark the override active; a null name just clears the flag.
pub fn world_painter_set_irradiance_override(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let named = c.g[A0][0] != 0;
        frame.save_ra(c, 0);
        let flag = gp(c, 0xa74);
        if !named {
            c.cycles += 2;
            c.sw(flag, 0);
        } else {
            c.cycles += 6;
            let buffer = gp(c, 0x24c0);
            c.g[A1][0] = c.g[A0][0];
            c.g[A0][0] = sx(buffer);
            call_ptr(c, STRLEN_AND_STRCPY, 0x002e_f39c)?;
            c.g[V1][0] = 1;
            let one = c.g[V1][0];
            c.sw(flag, one);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptMan_DeleteAllProcesses.

/// `void WScriptMan_DeleteAllProcesses(this)` (0x308f38): empty the three
/// script process lists at `+0x2b8`, `+0x2bc` and `+0x2b4`.
pub fn w_script_man_delete_all_processes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        for (off, ret) in [
            (0x2b8u32, 0x0030_8f50u32),
            (0x2bc, 0x0030_8f58),
            (0x2b4, 0x0030_8f60),
        ] {
            c.g[A0][0] = sx((this as u32).wrapping_add(off));
            callee::WSCRIPT_DELETE_ALL.call(c, ret)?;
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderModel_LoadState.

/// `void RiderModel_LoadState(RiderModel* self, cAmStream* stream)`
/// (0x310ee0): hand the bone array (`+0x2c`, `numBones << 5` bytes, count at
/// `+0x10`) to the stream's `write` slot.
pub fn rider_model_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        let stream = c.g[A1][0];
        c.g[T0][0] = stream;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(stream as u32).lw(c);
        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x2c)).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
        c.g[A3][0] = Addr(c.g[V0][0] as u32).off(0x14).lw(c);
        let target = c.g[A3][0] as u32;
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0031_0f0c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sphere tree helpers.

/// `SphereTree* SphereTree_ctor(SphereTree* self)` (0x32c508): clear the
/// count at `+0x98` and reset the root transform; returns `self`.
pub fn sphere_tree_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 7;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.sw((this as u32).wrapping_add(0x98), 0);
        c.g[A0][0] = c.g[S0][0];
        callee::SPHERE_TREE_RESET_TRANSFORM.call(c, 0x0032_c52c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int CollSphere_CollideAABB(coll, a, b, box)` (0x32fe40): test the sphere
/// at `self + 0x60` (radius at `+0x70`) against the AABB in `box` (`+0x0`,
/// `+0x10`, `+0x40`); the callee's result is returned as a bool.
pub fn coll_sphere_collide_aabb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
        c.lwc1(12, (c.g[V0][0] as u32).wrapping_add(0x70));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x60));
        c.g[T1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
        call_ptr(c, SPHERE_TREE_SWEEP_AABB, 0x0032_fe68)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `float CollSphereTree_GetDiameter(tree)` (0x3306d8): twice the radius of
/// the root sphere, or -1 when the tree is empty.
pub fn coll_sphere_tree_get_diameter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 0);
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x60)).lw(c);
            call_ptr(c, SPHERE_TREE_GET_RADIUS, 0x0033_06f4)?;
            c.add_s(0, 0, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Nash_DestroyFwdDiffCache(this)` (0x334850): destroy the forward
/// difference cache at `+0xa4` (when there is one) and clear the field.
pub fn nash_destroy_fwd_diff_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A0][0] = Addr((this as u32).wrapping_add(0xa4)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, AI_FWD_DIFF_CACHE_DTOR, 0x0033_4874)?;
        }
        c.sw((this as u32).wrapping_add(0xa4), 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Floating_FreeBoundBox(this)` (0x34ef08): delete the bounding box at
/// `+0x78` and clear the pointer.
pub fn floating_free_bound_box(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A0][0] = Addr((this as u32).wrapping_add(0x78)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, DELETE, 0x0034_ef2c)?;
            c.sw((this as u32).wrapping_add(0x78), 0);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RollerModifier_Translate(sRmBody* self, const sRmVec4* b)`
/// (0x35ed90): add `*b` to the body position at `+0x30` (VU0, keeping the MAC
/// flags) and translate the body's bounding volume at `+0xe0` as well.
pub fn roller_modifier_translate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A1][0];
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe0));
        c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.lqc2(4, c.g[V1][0] as u32);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, (c.g[V0][0] as u32).wrapping_add(0x30));
        call_ptr(c, SPHERE_TREE_TRANSLATE, 0x0035_edbc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2GraphicsMan_SetTextureNameCache(this, count, const void* src)`
/// (0x369690): store `count` at `+0xe80` and copy `count` 12-byte entries
/// into the array at `+0x280`.
pub fn ps2_graphics_man_set_texture_name_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[A3][0] = c.g[A1][0];
        c.g[V1][0] = 0xc;
        c.g[A1][0] = c.g[A2][0];
        let frame = Frame::open(c, 0x10);
        c.mult(0, 0, 7, 3);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[V0][0] = this;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x280));
        c.g[A2][0] = c.lo[0];
        let count = c.g[A3][0];
        c.sw((this as u32).wrapping_add(0xe80), count);
        call_ptr(c, MEM_COPY, 0x0036_96bc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UITextureSet_Find / UIIcon_SetTexture.

/// `Texture* UITextureSet_Find(UITextureSet* set, int id)` (0x398380): linear
/// search of the `count` (`+0x10`) 0x20-byte entries at `*(set + 0xc)` for the
/// one whose id (`+0x1c`) matches; null when there is none.
pub fn ui_texture_set_find(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = c.g[A0][0];
    c.g[A2][0] = Addr((self_ as u32).wrapping_add(0x10)).lw(c);
    let count = c.g[A2][0];
    c.g[V1][0] = 0;
    if (count as i64) > 0 {
        c.cycles += 1;
        c.g[A0][0] = Addr((self_ as u32).wrapping_add(0xc)).lw(c);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).lw(c);
            let mismatch = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if mismatch {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_8390);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void UIIcon_SetTexture(this, int id)` (0x399730): look the texture up in
/// the icon's texture set (via `+0x5c` -> `+0xd0` -> `+0x10`) and store it at
/// `+0x7c`.
pub fn ui_icon_set_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = Addr((this as u32).wrapping_add(0x5c)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xd0).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58));
        callee::UI_TEXTURE_SET_FIND.call(c, 0x0039_9754)?;
        let found = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0x7c), found);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int RCMP_IsMpegChunkTag(int tag)` (0x3b11f0): whether `tag` is one of the
/// two words of the MPEG chunk-tag table at `0x495448`.
pub fn rcmp_is_mpeg_chunk_tag(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0049_0000;
    c.g[V1][0] = 0;
    c.g[A1][0] = sx(0x0049_5448);
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let mismatch = c.g[V0][0] != c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if !mismatch {
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = (c.g[V1][0] < 2u64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003b_1200);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
}

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
// BE getters/setters.

/// `int BENewPlayerInt_GetSlotFlag3(this, int index)` (0x147580): bit 3 of
/// the 32-byte player-character record at `&D_00534FE0 + id * 0x1c + 0x10`,
/// where `id` is the slot's player id.
pub fn be_new_player_int_get_slot_flag3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        callee::BENEWPLAYER_GET_SLOT.call(c, 0x0014_7590)?;
        c.g[A0][0] = 0x1c;
        c.g[V1][0] = 0x0053_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx(0x0053_4fe0);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
        c.g[V0][0] = c.g[V0][0] & 1;
        frame.close(c);
        Ok(())
    })
}

/// `void BE_ResetPlayerProfile(int player)` (0x14de28): reset the BE library's
/// profile (when there is one) and the BAGT profile of player `player`.
pub fn be_reset_player_profile(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save(c, S0, 0x10);
        let library = gp(c, -0x1ef8);
        c.g[A0][0] = Addr(library).lw(c);
        c.g[S0][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        let present = c.g[A0][0] != 0;
        frame.save_ra(c, 0);
        if present {
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            callee::BE_LIBRARY_RESET_PROFILE.call(c, 0x0014_de4c)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::BEBAGT_RESET_PROFILE.call(c, 0x0014_de54)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BEOptionInt_GetPlayerHandicap(int i)` (0x14f7a8): the maximum handicap
/// minus the player's stored handicap, both from the option table at
/// `D_005308B8`.
pub fn be_option_int_get_player_handicap(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[V1][0] = 0xc;
        let frame = Frame::open(c, 0x20);
        c.mult(0, A0, A0, V1);
        c.g[V0][0] = 0x0053_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0053_08b8);
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lw(c);
        callee::BE_OPTION_GET_MAX_HANDICAP.call(c, 0x0014_f7d0)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int BEMissionInt_GetTotalCollectibles(this, int start)` (0x153350): the
/// signed sum of the collectible-count bytes from `&D_0043FA70 + start`
/// (the reference's loop runs once: the bound is a constant 0).
pub fn be_mission_int_get_total_collectibles(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x0044_0000;
    c.g[A0][0] = 0;
    c.g[V0][0] = sx(0x0043_fa70);
    c.g[V1][0] = c.g[A0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lb(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0015_3368);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        abi::ret(c);
        return;
    }
}

/// `void ber_reward_set_bit(bits, int bit)` — the shared body of the two
/// reward-flag setters: `bits[bit >> 3] |= 1 << (bit & 7)`.
fn reward_set_bit(c: &mut Cpu, bits: u64, bit: u64) {
    c.cycles += 15;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] = sx((bit as u32).wrapping_add(7));
    c.g[V0][0] = ((c.g[V0][0] as i64) < (bit as i64)) as u64;
    c.g[A0][0] = sx((bit as u32) >> 3);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = bit;
    }
    c.g[A2][0] = sx((bits as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A3][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
    c.g[V0][0] = 1;
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 3);
    c.g[V1][0] = Addr(c.g[A2][0] as u32).lbu(c);
    c.g[A3][0] = sx((bit as u32).wrapping_sub(c.g[A3][0] as u32));
    let shift = (c.g[A3][0] as u32) & 31;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << shift);
    c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
    let value = c.g[V1][0];
    c.sb(c.g[A2][0] as u32, value);
    abi::ret(c);
}

/// `void BERewardInt_SetGrantedBit(this, int bit, uchar* bits)` (0x15a358):
/// set the granted bit.
pub fn be_reward_int_set_granted_bit(c: &mut Cpu) {
    let bits = c.g[A2][0];
    let bit = c.g[A1][0];
    reward_set_bit(c, bits, bit);
}

/// `void BERewardInt_NewRewardsSetBit(this, int bit, uchar* bits)` (0x15a5b0):
/// set the "new reward" bit (the generated body is the same as the granted
/// setter, inlined).
pub fn be_reward_int_new_rewards_set_bit(c: &mut Cpu) {
    let bits = c.g[A2][0];
    let bit = c.g[A1][0];
    reward_set_bit(c, bits, bit);
}

// ---------------------------------------------------------------------------
// Camera controller / algorithm constructors.

/// `void ManualCamController_Reset(this)` (0x15cee8): reset the controller and
/// then each algorithm in its list via the list's vtable slot at `+0x24`.
pub fn manual_cam_controller_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A0][0] = this;
        callee::CAM_CONTROLLER_RESET.call(c, 0x0015_cefc)?;
        c.g[A1][0] = Addr((this as u32).wrapping_add(0x18)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0015_cf14)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared shape of the camera-algorithm constructors: 15 cycles, a `0x20`
/// frame saving `s0` and `ra`, the base constructor, then the algorithm tag at
/// `+0xc` and the class vtable at `+0x10`; returns `self`.
fn camera_algorithm_ctor(c: &mut Cpu, base: Callee, tag: u64, vtable: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        base.call(c, ret)?;
        c.g[V0][0] = tag;
        c.g[V1][0] = 0x0046_0000;
        c.sw((this as u32).wrapping_add(0xc), tag);
        c.g[V1][0] = sx(vtable);
        let vt = c.g[V1][0];
        c.sw((this as u32).wrapping_add(0x10), vt);
        c.g[V0][0] = this;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ManualCamAlgo* ManualCamAlgo_ctor(this)` (0x15fd08): tag 0x51, vtable
/// `D_0045BD10`.
pub fn manual_cam_algo_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::MANUAL_CAM_ALGO_BASE_CTOR, 0x51, 0x0045_bd10, 0x0015_fd1c);
}

/// `EditorCamAlgo* EditorCamAlgo_ctor(this)` (0x15ffb0): tag 0x50, vtable
/// `D_0045BCA8`.
pub fn editor_cam_algo_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::MANUAL_CAM_ALGO_BASE_CTOR, 0x50, 0x0045_bca8, 0x0015_ffc4);
}

/// `ManualChaseCameraAlgorithm* ctor(this)` (0x160438): tag 0xb, vtable
/// `D_0045BAD0`.
pub fn manual_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0xb, 0x0045_bad0, 0x0016_044c);
}

/// `IdleChaseCameraAlgorithm* ctor(this)` (0x1616a8): tag 7, vtable
/// `D_0045BB48`.
pub fn idle_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0x7, 0x0045_bb48, 0x0016_16bc);
}

/// `LazyChaseCameraAlgorithm* ctor(this)` (0x1682b8): tag 0xd, vtable
/// `D_0045C628`.
pub fn lazy_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0xd, 0x0045_c628, 0x0016_82cc);
}

/// `LookBackChaseCameraAlgorithm* ctor(this)` (0x168650): tag 0x20, vtable
/// `D_0045C6A0`.
pub fn look_back_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0x20, 0x0045_c6a0, 0x0016_8664);
}

/// `MidRightChaseCameraAlgorithm* ctor(this)` (0x1689c8): tag 9, vtable
/// `D_0045C538`.
pub fn mid_right_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0x9, 0x0045_c538, 0x0016_89dc);
}

/// `MidLeftChaseCameraAlgorithm* ctor(this)` (0x168cd8): tag 9, vtable
/// `D_0045C5B0`.
pub fn mid_left_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0x9, 0x0045_c5b0, 0x0016_8cec);
}

/// `Default_2ChaseCameraAlgorithm* ctor(this)` (0x176a68): tag 0x3c, vtable
/// `D_0045CAB0`.
pub fn default_2_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0x3c, 0x0045_cab0, 0x0017_6a7c);
}

/// `Default_3ChaseCameraAlgorithm* ctor(this)` (0x176d68): tag 0x3d, vtable
/// `D_0045CA38`.
pub fn default_3_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0x3d, 0x0045_ca38, 0x0017_6d7c);
}

/// `Default_4ChaseCameraAlgorithm* ctor(this)` (0x177068): tag 0x3e, vtable
/// `D_0045C9C0`.
pub fn default_4_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0x3e, 0x0045_c9c0, 0x0017_707c);
}

/// `SpokeChaseCameraAlgorithm* ctor(this)` (0x177e50): tag 0x42, vtable
/// `D_0045CCC8`.
pub fn spoke_chase_camera_algorithm_ctor(c: &mut Cpu) {
    camera_algorithm_ctor(c, callee::CHASE_CAMERA_ALGORITHM_CTOR, 0x42, 0x0045_ccc8, 0x0017_7e64);
}

// ---------------------------------------------------------------------------
// ChaseCamController_ExitSpokeMode.

/// `void ChaseCamController_ExitSpokeMode(this)` (0x162218): clear the
/// spoke-mode bits 0 and 1 of the flags at `+0x30`; when neither was set,
/// blend back to the stored algorithm `+0x24` over one 150th of a second.
pub fn chase_cam_controller_exit_spoke_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0xffff_ffff_ffff_fffb;
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[V0][0] = Addr((this as u32).wrapping_add(0x30)).lw(c);
        c.g[V1][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] & 3;
        let in_spoke_mode = c.g[V0][0] != 0;
        let cleared = c.g[V1][0];
        c.sw((this as u32).wrapping_add(0x30), cleared);
        if !in_spoke_mode {
            c.cycles += 4;
            let rate = gp(c, -0x6b14);
            c.lwc1(12, rate);
            c.g[A2][0] = 0;
            c.g[A1][0] = Addr((this as u32).wrapping_add(0x24)).lw(c);
            callee::CHASE_CAM_SET_ALGORITHM.call(c, 0x0016_2248)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CameraTriggerMan_Destroy.

/// `void CameraTriggerMan_Destroy(this)` (0x16c098): free the trigger array,
/// then free the manager itself and clear the caller's pointer.
pub fn camera_trigger_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::CAMERA_TRIGGER_FREE_TRIGGERS.call(c, 0x0016_c0ac)?;
        c.g[A0][0] = Addr(this as u32).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, MEM_FREE, 0x0016_c0c0)?;
            c.sw(this as u32, 0);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFEState constructors.

/// `cFEStateOptionsGame* ctor(this)` (0x188ee8): base `UIState_ctor`, then the
/// state vtable at `+0x8` (`D_0046C6F0`) and id 0x1c at `+0xc`.
pub fn c_fe_state_options_game_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::UI_STATE_CTOR.call(c, 0x0018_8efc)?;
        c.g[V0][0] = 0x1c;
        c.g[V1][0] = 0x0047_0000;
        let id = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0xc), id);
        c.g[V1][0] = sx(0x0046_c6f0);
        let vt = c.g[V1][0];
        c.sw((this as u32).wrapping_add(0x8), vt);
        c.g[V0][0] = this;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateProfileLoad_v26(this)` (0x18f278): refresh the profile list
/// when the state at `+0x214` is 3, the slot list otherwise.
pub fn c_fe_state_profile_load_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 3;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x214)).lw(c);
        let not_three = c.g[V0][0] != c.g[V1][0];
        if not_three {
            c.cycles += 3;
            callee::FE_REFRESH_SLOTS.call(c, 0x0018_f2a8)?;
        } else {
            c.cycles += 4;
            callee::FE_REFRESH_PROFILE_LIST.call(c, 0x0018_f298)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateRuleSelect_v21(this, msg)` (0x191e08): 0x101 when the message
/// id at `+0x38` is the hash of `D_004A13A0`, 0x100 otherwise.
pub fn c_fe_state_rule_select_v21(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = 0x004a_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x004a_13a0);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x38)).lw(c);
        call_ptr(c, STR_HASH, 0x0019_1e24)?;
        c.g[S0][0] = c.g[S0][0] ^ c.g[V0][0];
        c.g[V1][0] = 0x100;
        c.g[V0][0] = 0x101;
        frame.restore_ra(c, 0);
        if c.g[S0][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateTitle* ctor(this)` (0x194738): base `UIState_ctor`, vtable
/// `D_0046BAB8` at `+0x8`, zero the id at `+0xc` and the field at `+0x48`.
pub fn c_fe_state_title_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::UI_STATE_CTOR.call(c, 0x0019_474c)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(0x0046_bab8);
        let vt = c.g[V1][0];
        c.sw((this as u32).wrapping_add(0x48), 0);
        c.sw((this as u32).wrapping_add(0x8), vt);
        c.sw((this as u32).wrapping_add(0xc), 0);
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
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{machine, ptr, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    /// A NUL-terminated `s` at `addr`.
    fn put_str(c: &mut Cpu, addr: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// The engine's size-class pool allocator (`MEM_PoolAlloc`): the pools must
    /// be marked ready and have free lists, or allocations grow the (zeroed)
    /// `DLHeap` and the frees walk a corrupt chain forever.
    fn pool_allocator(c: &mut Cpu) {
        let gp = GAME_GP;
        c.bus.write32(gp + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = 0x008c_0000 + 0x1000 * i;
            let base = 0x008b_0000 + 0x100 * i;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(gp + slot, pool);
        }
    }

    // -----------------------------------------------------------------------
    // Looping helpers.

    fn trigger_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x2b6a_68 ^ seed as u64);
        c.g[GP][0] = GAME_GP as u64;
        let array = DATA + 0x1000;
        c.bus.write32(DATA, array);
        c.g[A0][0] = DATA as u64;
        for i in 0..40u32 {
            let slot = array + i * 0x70;
            let active = match seed % 3 {
                0 => 1,
                1 => 2,
                _ => r.next_u32() % 3,
            };
            c.bus.write32(slot, active);
            c.bus.write32(slot + 0x34, r.next_u32());
        }
    }

    driven!(gen_flag_all_active, ssx3_ee::s08::f_002b6a68__generated);
    driven!(hand_flag_all_active, world_trigger_manager_flag_all_active);

    #[test]
    fn world_trigger_manager_flag_all_active_matches() {
        check(ssx3_ee::s08::f_002b6a68__generated, world_trigger_manager_flag_all_active, 48, &trigger_setup);
        check_events(gen_flag_all_active, hand_flag_all_active, 48, &|c, seed| {
            trigger_setup(c, seed);
            let mut r = Rng(0x6a68 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x002b_6a68);
            // The trigger sweep is 40 slots long, so a back edge is reached
            // within a few cycles whatever the event schedule says.
            c.next_event = c.cycles + 5;
            mode
        });
    }

    // -----------------------------------------------------------------------
    // Memcard request wrappers.

    fn memcard_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        let name = DATA + 0x800;
        match seed % 3 {
            0 => put_str(c, name, ""),
            1 => put_str(c, name, "BESLES-51234"),
            _ => put_str(c, name, "a much longer memory card file name"),
        }
        c.g[A1][0] = name as u64;
        c.g[A2][0] = (r.next_u32() % 4) as u64;
        // The state machine's dispatch table is zeroed: it lands in the
        // `jump_out` hook, which the test machine turns into a fake call.
        c.bus.write32(DATA + 0x18, 0x100);
    }

    #[test]
    fn c_ps2_mem_card_request_list_dir_all_matches() {
        check(ssx3_ee::s08::f_002c6810__generated, c_ps2_mem_card_request_list_dir_all, 48, &|c, seed| {
            memcard_setup(c, seed, 0x2c68_10);
        });
    }

    #[test]
    fn c_ps2_mem_card_request_open_matches() {
        check(ssx3_ee::s08::f_002c6968__generated, c_ps2_mem_card_request_open, 48, &|c, seed| {
            memcard_setup(c, seed, 0x2c69_68);
        });
    }

    #[test]
    fn c_ps2_mem_card_do_close_matches() {
        check(ssx3_ee::s08::f_002c6a28__generated, c_ps2_mem_card_do_close, 48, &|c, seed| {
            // A zero `sceMcClose` global makes the firmware call take its
            // "not initialised" path and return at once.
            memcard_setup(c, seed, 0x2c6a_28);
        });
    }

    #[test]
    fn c_ps2_mem_card_request_mkdir_matches() {
        check(ssx3_ee::s08::f_002c6c50__generated, c_ps2_mem_card_request_mkdir, 48, &|c, seed| {
            memcard_setup(c, seed, 0x2c6c_50);
        });
    }

    // -----------------------------------------------------------------------
    // MenuSys*Item_Measure trampolines.

    /// A zeroed item and row: `*(item + 0x14)` is 0, so `MenuSysItem_MeasureRow`
    /// takes its cheapest path and calls nothing.
    fn menu_item_setup(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (DATA + 0x800) as u64;
    }

    #[test]
    fn menu_sys_callback_item_measure_matches() {
        check(ssx3_ee::s09::f_002cd240__generated, menu_sys_callback_item_measure, 32, &|c, seed| {
            menu_item_setup(c, seed, 0x2cd2_40);
        });
    }

    #[test]
    fn menu_sys_int8_item_measure_matches() {
        check(ssx3_ee::s09::f_002cd7b8__generated, menu_sys_int8_item_measure, 32, &|c, seed| {
            menu_item_setup(c, seed, 0x2cd7_b8);
        });
    }

    #[test]
    fn menu_sys_int_item_measure_matches() {
        check(ssx3_ee::s09::f_002cdbf0__generated, menu_sys_int_item_measure, 32, &|c, seed| {
            menu_item_setup(c, seed, 0x2cdb_f0);
        });
    }

    #[test]
    fn menu_sys_float_item_measure_matches() {
        check(ssx3_ee::s09::f_002cdfa0__generated, menu_sys_float_item_measure, 32, &|c, seed| {
            menu_item_setup(c, seed, 0x2cdf_a0);
        });
    }

    #[test]
    fn menu_sys_float_range_item_measure_matches() {
        check(ssx3_ee::s09::f_002ce368__generated, menu_sys_float_range_item_measure, 32, &|c, seed| {
            menu_item_setup(c, seed, 0x2ce3_68);
        });
    }

    // -----------------------------------------------------------------------
    // cTransitionMan_StartCaptured.

    #[test]
    fn c_transition_man_start_captured_matches() {
        check(ssx3_ee::s09::f_002e4540__generated, c_transition_man_start_captured, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x2e45_40, 0);
            // A zeroed manager: `+0x44` is clear (no Stop), `t0` is 1 and the
            // incoming `a2` is 0, so no vtable is reached.
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ptr(&mut r) as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = ptr(&mut r) as u64;
            c.f[12] = r.edge_f32();
        });
    }

    // -----------------------------------------------------------------------
    // WorldPainter_SetIrradianceOverride.

    #[test]
    fn world_painter_set_irradiance_override_matches() {
        check(ssx3_ee::s09::f_002ef378__generated, world_painter_set_irradiance_override, 48, &|c, seed| {
            let mut r = machine(c, seed, 0x2ef3_78, 0);
            c.g[GP][0] = GAME_GP as u64;
            if seed % 2 == 0 {
                c.g[A0][0] = 0;
            } else {
                let name = DATA + 0x800;
                match seed % 3 {
                    0 => put_str(c, name, ""),
                    1 => put_str(c, name, "sunset"),
                    _ => {
                        c.bus.write32(name, r.next_u32());
                        c.bus.write8(name + 4, 0);
                    }
                }
                c.g[A0][0] = name as u64;
            }
            c.bus.write32(GAME_GP + 0xa74, r.next_u32());
        });
    }

    // -----------------------------------------------------------------------
    // WScriptMan_DeleteAllProcesses.

    #[test]
    fn w_script_man_delete_all_processes_matches() {
        check(ssx3_ee::s10::f_00308f38__generated, w_script_man_delete_all_processes, 32, &|c, seed| {
            let mut r = machine(c, seed, 0x308f_38, 0);
            c.g[A0][0] = DATA as u64;
            // Zeroed process lists: PopFront returns null at once.
            c.bus.write32(DATA + 0x2b8, 0);
            c.bus.write32(DATA + 0x2bc, 0);
            c.bus.write32(DATA + 0x2b4, 0);
            c.bus.write32(DATA + 0x10, r.next_u32());
        });
    }

    // -----------------------------------------------------------------------
    // RiderModel_LoadState.

    #[test]
    fn rider_model_load_state_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x310e_e0, 1);
            let this = DATA;
            let stream = DATA + 0x800;
            let vt = DATA + 0x900;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            c.bus.write32(stream, vt);
            c.bus.write16(vt + 0x10, (r.next_u32() & 7) as u16);
            c.bus.write32(vt + 0x14, FAKE);
            c.bus.write32(this + 0x10, r.next_u32() % 8);
            c.bus.write32(this + 0x2c, ptr(&mut r));
        };
        check(ssx3_ee::s10::f_00310ee0__generated, rider_model_load_state, 64, &setup);
        crate::difftest::fakes::covered(ssx3_ee::s10::f_00310ee0__generated, rider_model_load_state, 64, &setup);
    }

    // -----------------------------------------------------------------------
    // Sphere tree helpers.

    #[test]
    fn sphere_tree_ctor_matches() {
        check(ssx3_ee::s10::f_0032c508__generated, sphere_tree_ctor, 32, &|c, seed| {
            let mut r = machine(c, seed, 0x32c5_08, 0);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x98, r.next_u32());
            for i in 0..8u32 {
                c.bus.write32(DATA + i * 16, r.next_u32());
            }
        });
    }

    #[test]
    fn coll_sphere_collide_aabb_matches() {
        check(ssx3_ee::s11::f_0032fe40__generated, coll_sphere_collide_aabb, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x32fe_40, 0);
            let this = DATA;
            let box_ = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x2000) as u64;
            c.g[A2][0] = (DATA + 0x3000) as u64;
            c.g[A3][0] = box_ as u64;
            for i in 0..0x60u32 / 4 {
                c.bus.write32(this + i * 4, r.next_u32());
                c.bus.write32(box_ + i * 4, r.edge_f32());
            }
        });
    }

    #[test]
    fn coll_sphere_tree_get_diameter_matches() {
        check(ssx3_ee::s11::f_003306d8__generated, coll_sphere_tree_get_diameter, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x3306_d8, 0);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, (seed % 3 != 0) as u32);
            c.bus.write32(DATA + 0x60, DATA + 0x200);
            c.bus.write32(DATA + 0x200 + 0x98, DATA + 0x300);
            c.bus.write32(DATA + 0x300 + 0x20, DATA + 0x400);
            c.bus.write32(DATA + 0x200 + 0x90, r.f32(100.0));
            c.bus.write32(DATA + 0x400, r.f32(100.0));
        });
    }

    /// The cache pointer is small, so `MEM_IsPoolPointer` sees it inside the
    /// (zeroed) 8-byte pool and `MEM_PoolFree` is two stores — no heap walk.
    fn small_pointer(seed: u32) -> u32 {
        0x100 + (seed % 0x700)
    }

    #[test]
    fn nash_destroy_fwd_diff_cache_matches() {
        check(ssx3_ee::s11::f_00334850__generated, nash_destroy_fwd_diff_cache, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x3348_50, 0);
            c.g[A0][0] = DATA as u64;
            let p = if seed % 3 == 0 { 0 } else { small_pointer(seed) };
            c.bus.write32(DATA + 0xa4, p);
            c.bus.write32(DATA + 0x10, r.next_u32());
        });
    }

    #[test]
    fn floating_free_bound_box_matches() {
        check(ssx3_ee::s11::f_0034ef08__generated, floating_free_bound_box, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x34ef_08, 0);
            c.g[A0][0] = DATA as u64;
            let p = if seed % 3 == 0 { 0 } else { small_pointer(seed) };
            c.bus.write32(DATA + 0x78, p);
            c.bus.write32(DATA + 0xc, r.next_u32());
        });
    }

    #[test]
    fn roller_modifier_translate_matches() {
        check(ssx3_ee::s11::f_0035ed90__generated, roller_modifier_translate, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x35ed_90, 1);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            for i in 0..0x30u32 / 4 {
                c.bus.write32(DATA + 0x30 + i * 4, r.edge_f32());
                c.bus.write32(DATA + 0x80 + i * 4, r.edge_f32());
                c.bus.write32(DATA + 0x800 + i * 4, r.edge_f32());
            }
            for reg in 1..32 {
                let v = [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()];
                c.bus.plat.set_vf(reg, v);
            }
        });
    }

    #[test]
    fn ps2_graphics_man_set_texture_name_cache_matches() {
        check(ssx3_ee::s12::f_00369690__generated, ps2_graphics_man_set_texture_name_cache, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x3696_90, 0);
            c.g[A0][0] = DATA as u64;
            let count = r.next_u32() % 8;
            c.g[A1][0] = count as u64;
            let src = DATA + 0x4000;
            c.g[A2][0] = src as u64;
            for i in 0..count * 3 {
                c.bus.write32(src + i * 4, r.next_u32());
            }
            for i in 0..0x100u32 / 4 {
                c.bus.write32(DATA + 0x280 + i * 4, r.next_u32());
            }
            c.bus.write32(DATA + 0xe80, r.next_u32());
        });
    }

    // -----------------------------------------------------------------------
    // UITextureSet_Find / UIIcon_SetTexture.

    /// A texture set at `set`: `count` entries of 0x20 bytes with ids at
    /// `+0x1c`; `seed % 4 == 0` asks for an id that is not there.
    fn texture_set(c: &mut Cpu, r: &mut Rng, set: u32, count: u32) -> u32 {
        let entries = DATA + 0x4000;
        c.bus.write32(set + 0x10, count);
        c.bus.write32(set + 0xc, entries);
        for i in 0..8u32 {
            c.bus.write32(entries + i * 0x20, r.next_u32());
            c.bus.write32(entries + i * 0x20 + 0x1c, 0x1000 + i);
        }
        let n = r.next_u32() % (count + 1);
        if n < count {
            0x1000 + n
        } else {
            0x9000
        }
    }

    fn find_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3983_80, 0);
        c.g[A0][0] = DATA as u64;
        let count = 1 + r.next_u32() % 6;
        let id = texture_set(c, &mut r, DATA, count);
        c.g[A1][0] = id as u64;
    }

    driven!(gen_texture_find, ssx3_ee::s13::f_00398380__generated);
    driven!(hand_texture_find, ui_texture_set_find);

    #[test]
    fn ui_texture_set_find_matches() {
        check(ssx3_ee::s13::f_00398380__generated, ui_texture_set_find, 64, &find_setup);
        check_events(gen_texture_find, hand_texture_find, 64, &|c, seed| {
            find_setup(c, seed);
            let mut r = Rng(0x3983_80 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x0039_8380);
            // A short search may not reach the scheduled event: make the
            // first back edge due, identically for both bodies.
            c.next_event = c.cycles + 4;
            mode
        });
    }

    #[test]
    fn ui_icon_set_texture_matches() {
        check(ssx3_ee::s13::f_00399730__generated, ui_icon_set_texture, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x3997_30, 0);
            let this = DATA;
            let icon = DATA + 0x800;
            let scene = DATA + 0x1000;
            let set = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.bus.write32(this + 0x5c, icon);
            c.bus.write32(icon + 0xd0, scene);
            // The texture set lives at `*(scene + 0x10) + 0x58`.
            c.bus.write32(scene + 0x10, set);
            c.bus.write32(this + 0x7c, r.next_u32());
            let count = 1 + r.next_u32() % 4;
            let id = texture_set(c, &mut r, set + 0x58, count);
            c.g[A1][0] = id as u64;
        });
    }

    // -----------------------------------------------------------------------
    // RCMP_IsMpegChunkTag.

    fn chunk_tag_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3b11_f0, 0);
        let tags = 0x0049_5448u32;
        let a = r.next_u32();
        let b = r.next_u32();
        c.bus.write32(tags, a);
        c.bus.write32(tags + 4, b);
        c.g[A0][0] = match seed % 3 {
            0 => a as u64,
            1 => b as u64,
            _ => (a ^ 0x1000) as u64,
        };
    }

    driven!(gen_chunk_tag, ssx3_ee::s13::f_003b11f0__generated);
    driven!(hand_chunk_tag, rcmp_is_mpeg_chunk_tag);

    #[test]
    fn rcmp_is_mpeg_chunk_tag_matches() {
        check(ssx3_ee::s13::f_003b11f0__generated, rcmp_is_mpeg_chunk_tag, 48, &chunk_tag_setup);
        check_events(gen_chunk_tag, hand_chunk_tag, 48, &|c, seed| {
            chunk_tag_setup(c, seed);
            let mut r = Rng(0x3b11_f0 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x003b_11f0);
            // The table has two entries: make the (single) back edge due.
            c.next_event = c.cycles + 5;
            mode
        });
    }

    // -----------------------------------------------------------------------
    // AI_PathDataInitIfEarlySection.

    #[test]
    fn ai_path_data_init_if_early_section_matches() {
        check(ssx3_ee::s00::f_0012a490__generated, ai_path_data_init_if_early_section, 64, &|c, seed| {
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
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { (r.next_u32() | 1) as u64 };
        });
    }

    // -----------------------------------------------------------------------
    // BE getters/setters.

    #[test]
    fn be_new_player_int_get_slot_flag3_matches() {
        check(ssx3_ee::s01::f_00147580__generated, be_new_player_int_get_slot_flag3, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x1475_80, 0);
            let slot = r.next_u32() % 4;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = slot as u64;
            let id = r.next_u32() % 4;
            c.bus.write32(0x0053_05b0 + slot * 4, id);
            for i in 0..4u32 {
                c.bus.write32(0x0053_4fe0 + i * 0x1c + 0x10, if seed % 3 == 0 { 0 } else { r.next_u32() });
            }
        });
    }

    #[test]
    fn be_reset_player_profile_matches() {
        check(ssx3_ee::s01::f_0014de28__generated, be_reset_player_profile, 48, &|c, seed| {
            let mut r = machine(c, seed, 0x14de_28, 0);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A1][0] = r.next_u32() as u64;
            // No BE library: the first call is skipped for every second seed.
            let library = if seed % 2 == 0 { 0 } else { DATA + 0x6000 };
            c.bus.write32(GAME_GP.wrapping_sub(0x1ef8), library);
        });
    }

    #[test]
    fn be_option_int_get_player_handicap_matches() {
        check(ssx3_ee::s01::f_0014f7a8__generated, be_option_int_get_player_handicap, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x14f7_a8, 0);
            let table = 0x0053_08b8u32;
            let i = r.next_u32() % 4;
            c.g[A0][0] = i as u64;
            for k in 0..4u32 {
                c.bus.write32(table + k * 0xc + 8, r.next_u32() % 20);
            }
            c.bus.write32(table + 0x1c, r.next_u32() % 20);
            c.bus.write32(table + 0x8, r.next_u32() % 20);
            c.bus.write32(table + 0x14, r.next_u32() % 20);
        });
    }

    #[test]
    fn be_mission_int_get_total_collectibles_matches() {
        check(ssx3_ee::s01::f_00153350__generated, be_mission_int_get_total_collectibles, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x1533_50, 0);
            let table = 0x0043_fa70u32;
            let start = r.next_u32() % 8;
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = start as u64;
            for k in 0..10u32 {
                c.bus.write8(table + k, r.next_u32() as u8);
            }
        });
    }

    fn reward_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x15a3_58, 0);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (r.next_u32() % 64) as u64;
        let bits = DATA + 0x800;
        c.g[A2][0] = bits as u64;
        for k in 0..8u32 {
            c.bus.write8(bits + k, r.next_u32() as u8);
        }
    }

    #[test]
    fn be_reward_int_set_granted_bit_matches() {
        check(ssx3_ee::s01::f_0015a358__generated, be_reward_int_set_granted_bit, 64, &reward_setup);
    }

    #[test]
    fn be_reward_int_new_rewards_set_bit_matches() {
        check(ssx3_ee::s01::f_0015a5b0__generated, be_reward_int_new_rewards_set_bit, 64, &reward_setup);
    }

    // -----------------------------------------------------------------------
    // Camera controller / algorithm constructors.

    #[test]
    fn manual_cam_controller_reset_matches() {
        check(ssx3_ee::s01::f_0015cee8__generated, manual_cam_controller_reset, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x15ce_e8, 0);
            let this = DATA;
            let list = DATA + 0x100;
            let vt = DATA + 0x200;
            c.g[A0][0] = this as u64;
            // `CamController_Reset` walks `*(this + 8)` (null: nothing to do)
            // and resets the camera at `+0x10` (null: the splash manager's
            // rider chain is zeroed too).
            c.bus.write32(this + 0x8, list);
            c.bus.write32(this + 0x10, 0);
            c.bus.write32(this + 0x18, DATA + 0x300);
            c.bus.write32(DATA + 0x300 + 0x10, vt);
            c.bus.write16(vt + 0x20, (r.next_u32() & 7) as u16);
            c.bus.write32(vt + 0x24, FAKE);
        });
    }

    /// A zeroed algorithm object at `DATA`; all the vtable slots the base
    /// constructors reach are real functions.
    fn algo_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        for i in 0..0x100u32 {
            c.bus.write32(DATA + i * 4, r.next_u32());
        }
        for i in 0..0x100u32 {
            c.bus.write32(DATA + 0x400 + i * 4, r.next_u32());
        }
    }

    #[test]
    fn manual_cam_algo_ctor_matches() {
        check(ssx3_ee::s01::f_0015fd08__generated, manual_cam_algo_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x15fd_08);
        });
    }

    #[test]
    fn editor_cam_algo_ctor_matches() {
        check(ssx3_ee::s01::f_0015ffb0__generated, editor_cam_algo_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x15ff_b0);
        });
    }

    #[test]
    fn manual_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s01::f_00160438__generated, manual_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x1604_38);
        });
    }

    #[test]
    fn idle_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s01::f_001616a8__generated, idle_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x1616_a8);
        });
    }

    #[test]
    fn lazy_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s02::f_001682b8__generated, lazy_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x1682_b8);
        });
    }

    #[test]
    fn look_back_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s02::f_00168650__generated, look_back_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x1686_50);
        });
    }

    #[test]
    fn mid_right_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s02::f_001689c8__generated, mid_right_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x1689_c8);
        });
    }

    #[test]
    fn mid_left_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s02::f_00168cd8__generated, mid_left_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x168c_d8);
        });
    }

    #[test]
    fn default_2_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s02::f_00176a68__generated, default_2_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x176a_68);
        });
    }

    #[test]
    fn default_3_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s02::f_00176d68__generated, default_3_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x176d_68);
        });
    }

    #[test]
    fn default_4_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s02::f_00177068__generated, default_4_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x1770_68);
        });
    }

    #[test]
    fn spoke_chase_camera_algorithm_ctor_matches() {
        check(ssx3_ee::s02::f_00177e50__generated, spoke_chase_camera_algorithm_ctor, 32, &|c, seed| {
            algo_setup(c, seed, 0x177e_50);
        });
    }

    // -----------------------------------------------------------------------
    // ChaseCamController_ExitSpokeMode.

    #[test]
    fn chase_cam_controller_exit_spoke_mode_matches() {
        check(ssx3_ee::s01::f_00162218__generated, chase_cam_controller_exit_spoke_mode, 32, &|c, seed| {
            let mut r = machine(c, seed, 0x1622_18, 0);
            pool_allocator(c);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.g[GP][0] = GAME_GP as u64;
            let flags = match seed % 3 {
                0 => 0,
                1 => 1 + r.next_u32() % 3,
                _ => r.next_u32(),
            };
            c.bus.write32(this + 0x30, flags);
            // `SetAlgorithm` with an out-of-range algorithm id takes its
            // "no change" path; the algorithm object itself stays zeroed.
            c.bus.write32(this + 0x24, 0x100);
            c.bus.write32(this + 0x20, 0);
            c.bus.write32(this + 0x8, 0);
            c.bus.write32(this + 0x18, 0);
        });
    }

    // -----------------------------------------------------------------------
    // CameraTriggerMan_Destroy.

    /// The freed pointer is small, so `MEM_IsPoolPointer` sees it inside the
    /// (zeroed) 8-byte pool — `MEM_PoolFree` is two stores, never a heap walk.
    #[test]
    fn camera_trigger_man_destroy_matches() {
        check(ssx3_ee::s02::f_0016c098__generated, camera_trigger_man_destroy, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x16c0_98, 0);
            let this = DATA;
            c.g[A0][0] = this as u64;
            // An empty trigger list (count at `+4` is 0) and, for most seeds,
            // a small "pool" pointer to free.
            c.bus.write32(this + 0x4, 0);
            c.bus.write32(this + 0x0, if seed % 3 == 0 { 0 } else { small_pointer(seed) });
            c.bus.write32(this + 0x10, r.next_u32());
        });
    }

    // -----------------------------------------------------------------------
    // cFEState constructors.

    fn fe_state_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        for i in 0..0x120u32 {
            c.bus.write32(DATA + i * 4, r.next_u32());
        }
        // A zeroed class descriptor (the base ctor installs a real one): the
        // refresh helpers then read a null vtable slot and reach the fake.
        c.bus.write32(DATA + 0x8, 0);
        c.bus.write32(DATA + 0x214, match seed % 3 {
            0 => 3,
            1 => r.next_u32(),
            _ => 2,
        });
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
        c.bus.write32(DATA + 0x1000 + 0x84, DATA + 0x2000);
        c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x3000);
    }

    #[test]
    fn c_fe_state_options_game_ctor_matches() {
        check(ssx3_ee::s02::f_00188ee8__generated, c_fe_state_options_game_ctor, 24, &|c, seed| {
            fe_state_setup(c, seed, 0x188e_e8);
        });
    }

    #[test]
    fn c_fe_state_profile_load_v26_matches() {
        check(ssx3_ee::s02::f_0018f278__generated, c_fe_state_profile_load_v26, 24, &|c, seed| {
            fe_state_setup(c, seed, 0x18f2_78);
        });
    }

    #[test]
    fn c_fe_state_rule_select_v21_matches() {
        check(ssx3_ee::s02::f_00191e08__generated, c_fe_state_rule_select_v21, 48, &|c, seed| {
            let mut r = machine(c, seed, 0x191e_08, 0);
            c.g[GP][0] = GAME_GP as u64;
            let msg = DATA + 0x800;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = msg as u64;
            c.bus.write32(msg + 0x38, r.next_u32());
        });
    }

    #[test]
    fn c_fe_state_title_ctor_matches() {
        check(ssx3_ee::s02::f_00194738__generated, c_fe_state_title_ctor, 24, &|c, seed| {
            fe_state_setup(c, seed, 0x1947_38);
        });
    }
}
